// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use dashmap::DashMap;
use futures::FutureExt;
use itertools::Itertools;
use move_symbol_pool::static_symbols;
use rand::rngs::OsRng;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use tap::TapFallible;
use tokio::{
    sync::{Notify, watch},
    time::timeout,
};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::WatchStream;
use tracing::{debug, error, info, instrument, warn};

use dkg::DKGState;
use identifiable_abort::{generate_proofs, identify_malicious};
use mysten_metrics::{monitored_scope, MonitoredFutureExt, spawn_monitored_task};
use sign_state::SignState;
use signature_mpc::decrypt::{DecryptionShare, PartialDecryptionProof};
use signature_mpc::twopc_mpc_protocols::{
    Commitment, DecommitmentProofVerificationRoundParty, DecryptionPublicParameters,
    initiate_decentralized_party_dkg, PartyID, ProtocolContext, PublicNonceEncryptedPartialSignatureAndProof,
    SecretKeyShareEncryptionAndProof, SecretKeyShareSizedNumber,
};
use sui_types::base_types::{ConciseableName, ObjectRef};
use sui_types::base_types::{AuthorityName, EpochId, TransactionDigest};
use sui_types::effects::{TransactionEffects, TransactionEffectsAPI};
use sui_types::error::{SuiError, SuiResult};
use sui_types::message_envelope::Message;
use sui_types::messages_signature_mpc::{
    InitiateSignatureMPCProtocol, SignatureMPCMessage, SignatureMPCMessageProtocols,
    SignatureMPCMessageSummary, SignatureMPCOutput, SignatureMPCSessionID,
};
use sui_types::sui_system_state::{SuiSystemState, SuiSystemStateTrait};
use sui_types::transaction::{TransactionDataAPI, TransactionKind};
use typed_store::Map;
use typed_store::traits::{TableSummary, TypedStoreDebug};

use crate::authority::{AuthorityState, EffectsNotifyRead};
use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::authority_client::AuthorityAPI;
use crate::signature_mpc::dkg::{DKGRound, DKGRoundCompletion};
use crate::signature_mpc::identifiable_abort::spawn_generate_proof;
pub use crate::signature_mpc::metrics::SignatureMPCMetrics;
use crate::signature_mpc::presign::{PresignRound, PresignRoundCompletion, PresignState};
use crate::signature_mpc::sign_round::{SignRound, SignRoundCompletion};
use crate::signature_mpc::signature_mpc_subscriber::SignatureMpcSubscriber;
use crate::signature_mpc::submit_to_consensus::SubmitSignatureMPC;
pub use crate::signature_mpc::submit_to_consensus::SubmitSignatureMPCToConsensus;

mod aggregate;
mod dkg;
mod metrics;
mod presign;
mod sign_round;
mod sign_state;
mod signature_mpc_subscriber;
mod submit_to_consensus;
mod identifiable_abort;

pub trait SignatureMPCServiceNotify {
    fn notify_signature_mpc_message(
        &self,
        epoch_store: &AuthorityPerEpochStore,
        message: &SignatureMPCMessage,
    ) -> SuiResult;
}

pub const MAX_MESSAGES_IN_PROGRESS: usize = 1000;

pub struct SignatureMPCAggregator {
    epoch: EpochId,
    epoch_store: Arc<AuthorityPerEpochStore>,
    party_id: PartyID,
    parties: HashSet<PartyID>,
    tiresias_public_parameters: DecryptionPublicParameters,
    tiresias_key_share_decryption_key_share: SecretKeyShareSizedNumber,
    submit: Arc<dyn SubmitSignatureMPC>,
    metrics: Arc<SignatureMPCMetrics>,
    exit: watch::Receiver<()>,
    /// Channel to receive protocols initiation for signature mpc from the state.
    rx_initiate_signature_mpc_protocol_sender: mpsc::Receiver<InitiateSignatureMPCProtocol>,
    rx_signature_mpc_protocol_message_sender: mpsc::Receiver<SignatureMPCMessage>,

    session_refs: Arc<DashMap<SignatureMPCSessionID, ObjectRef>>,

    dkg_session_rounds: Arc<DashMap<SignatureMPCSessionID, DKGRound>>,
    dkg_session_states: Arc<DashMap<SignatureMPCSessionID, DKGState>>,
    presign_session_rounds: Arc<DashMap<SignatureMPCSessionID, PresignRound>>,
    presign_session_states: Arc<DashMap<SignatureMPCSessionID, PresignState>>,
    sign_session_rounds: Arc<DashMap<SignatureMPCSessionID, SignRound>>,
    sign_session_states: Arc<DashMap<SignatureMPCSessionID, SignState>>,
}

impl SignatureMPCAggregator {
    fn new(
        epoch: EpochId,
        epoch_store: Arc<AuthorityPerEpochStore>,
        party_id: PartyID,
        parties: HashSet<PartyID>,
        tiresias_public_parameters: DecryptionPublicParameters,
        tiresias_key_share_decryption_key_share: SecretKeyShareSizedNumber,
        submit: Arc<dyn SubmitSignatureMPC>,
        metrics: Arc<SignatureMPCMetrics>,
        exit: watch::Receiver<()>,
        rx_initiate_signature_mpc_protocol_sender: mpsc::Receiver<InitiateSignatureMPCProtocol>,
        rx_signature_mpc_protocol_message_sender: mpsc::Receiver<SignatureMPCMessage>,
    ) -> Self {
        Self {
            epoch,
            epoch_store,
            party_id,
            parties,
            tiresias_public_parameters,
            tiresias_key_share_decryption_key_share,
            submit,
            metrics,
            exit,
            rx_initiate_signature_mpc_protocol_sender,
            rx_signature_mpc_protocol_message_sender,

            session_refs: Arc::new(DashMap::new()),

            dkg_session_rounds: Arc::new(DashMap::new()),
            dkg_session_states: Arc::new(DashMap::new()),
            presign_session_rounds: Arc::new(DashMap::new()),
            presign_session_states: Arc::new(DashMap::new()),
            sign_session_rounds: Arc::new(DashMap::new()),
            sign_session_states: Arc::new(DashMap::new()),
        }
    }

    async fn run(mut self) {
        info!("Starting SignatureMPCService");

        loop {
            tokio::select! {
                biased;

                _ = self.exit.changed().boxed() => {
                    // return on exit signal
                    info!("Shutting down SignatureMPCService");
                    return;
                }
                Some(
                    signature_mpc_protocol_message
                ) = self.rx_signature_mpc_protocol_message_sender.recv() => {
                    let epoch_store = self.epoch_store.clone();
                    let parties = self.parties.clone();
                    let tiresias_public_parameters = self.tiresias_public_parameters.clone();
                    let tiresias_key_share_decryption_key_share = self.tiresias_key_share_decryption_key_share.clone();
                    let submit = self.submit.clone();

                    let session_refs = self.session_refs.clone();

                    let dkg_session_rounds = self.dkg_session_rounds.clone();
                    let dkg_session_states = self.dkg_session_states.clone();
                    let presign_session_rounds = self.presign_session_rounds.clone();
                    let presign_session_states = self.presign_session_states.clone();
                    let sign_session_rounds = self.sign_session_rounds.clone();
                    let sign_session_states = self.sign_session_states.clone();
                    spawn_monitored_task!(Self::insert_message(
                        self.epoch,
                        epoch_store,
                        self.party_id,
                        parties,
                        tiresias_public_parameters,
                        tiresias_key_share_decryption_key_share,
                        submit,
                        session_refs,
                        dkg_session_rounds,
                        dkg_session_states,
                        presign_session_rounds,
                        presign_session_states,
                        sign_session_rounds,
                        sign_session_states,
                        signature_mpc_protocol_message
                    ));
                }

                Some(
                    initiate_signature_mpc_protocol
                ) = self.rx_initiate_signature_mpc_protocol_sender.recv() => {
                    let epoch_store = self.epoch_store.clone();
                    let parties = self.parties.clone();
                    let tiresias_public_parameters = self.tiresias_public_parameters.clone();
                    let tiresias_key_share_decryption_key_share = self.tiresias_key_share_decryption_key_share.clone();
                    let submit = self.submit.clone();

                    let session_refs = self.session_refs.clone();
                    let dkg_session_rounds = self.dkg_session_rounds.clone();
                    let dkg_session_states = self.dkg_session_states.clone();
                    let presign_session_rounds = self.presign_session_rounds.clone();
                    let presign_session_states = self.presign_session_states.clone();
                    let sign_session_rounds = self.sign_session_rounds.clone();
                    let sign_session_states = self.sign_session_states.clone();

                    spawn_monitored_task!(Self::initiate_protocol(
                        self.epoch,
                        epoch_store,
                        self.party_id,
                        parties,
                        tiresias_public_parameters,
                        tiresias_key_share_decryption_key_share,
                        submit,
                        session_refs,
                        dkg_session_rounds,
                        dkg_session_states,
                        presign_session_rounds,
                        presign_session_states,
                        sign_session_rounds,
                        sign_session_states,
                        initiate_signature_mpc_protocol
                    ));
                }
            }
        }
    }

    async fn insert_message(
        epoch: EpochId,
        epoch_store: Arc<AuthorityPerEpochStore>,
        party_id: PartyID,
        parties: HashSet<PartyID>,
        tiresias_public_parameters: DecryptionPublicParameters,
        tiresias_key_share_decryption_key_share: SecretKeyShareSizedNumber,
        submit: Arc<dyn SubmitSignatureMPC>,
        session_refs: Arc<DashMap<SignatureMPCSessionID, ObjectRef>>,
        dkg_session_rounds: Arc<DashMap<SignatureMPCSessionID, DKGRound>>,
        dkg_session_states: Arc<DashMap<SignatureMPCSessionID, DKGState>>,
        presign_session_rounds: Arc<DashMap<SignatureMPCSessionID, PresignRound>>,
        presign_session_states: Arc<DashMap<SignatureMPCSessionID, PresignState>>,
        sign_session_rounds: Arc<DashMap<SignatureMPCSessionID, SignRound>>,
        sign_session_states: Arc<DashMap<SignatureMPCSessionID, SignState>>,
        message: SignatureMPCMessage,
    ) {
        let session_id = message.summary.session_id;
        let sender_party_id = (epoch_store
            .committee()
            .authority_index(&message.summary.auth_sig().authority)
            .unwrap()
            + 1) as PartyID;

        let Some(session_ref) = session_refs.get(&session_id) else {
            return;
        };
        let session_ref = session_ref.clone();
        match &message.summary.message {
            // Q: Where is the message.summary.message field defined?
            SignatureMPCMessageProtocols::DKG(m) => {
                let mut state = dkg_session_states
                    .entry(message.summary.session_id)
                    .or_insert_with(|| DKGState::new(epoch, party_id, parties.clone()));
                let _ = state.insert_first_round(sender_party_id, m.clone());

                if let Some(r) = dkg_session_rounds.get_mut(&session_id) {
                    if state.ready_for_complete_first_round(&r) {
                        drop(r);
                        let state = state.clone();
                        Self::spawn_complete_dkg_round(
                            epoch,
                            epoch_store.clone(),
                            party_id,
                            session_id,
                            session_ref,
                            state,
                            dkg_session_rounds.clone(),
                            dkg_session_states.clone(),
                            submit.clone(),
                        );
                    }
                }
            }
            SignatureMPCMessageProtocols::PresignFirstRound(m) => {
                let mut state = presign_session_states.entry(session_id).or_insert_with(|| {
                    PresignState::new(
                        tiresias_public_parameters
                            .encryption_scheme_public_parameters
                            .clone(),
                        epoch,
                        party_id,
                        parties,
                        session_id,
                    )
                });

                let _ = state.insert_first_round(sender_party_id, m.clone());

                if let Some(r) = presign_session_rounds.get_mut(&session_id) {
                    if state.ready_for_complete_first_round(&r) {
                        drop(r);
                        Self::spawn_complete_presign_first_round(
                            epoch,
                            epoch_store.clone(),
                            party_id,
                            session_id,
                            session_ref,
                            state.clone(),
                            presign_session_rounds.clone(),
                            presign_session_states.clone(),
                            submit.clone(),
                        );
                    }
                }
            }
            SignatureMPCMessageProtocols::PresignSecondRound(m) => {
                let mut state = presign_session_states.entry(session_id).or_insert_with(|| {
                    PresignState::new(
                        tiresias_public_parameters
                            .encryption_scheme_public_parameters
                            .clone(),
                        epoch,
                        party_id,
                        parties,
                        session_id,
                    )
                });
                let _ = state.insert_second_round(sender_party_id, m.clone());

                if let Some(r) = presign_session_rounds.get_mut(&session_id) {
                    if state.ready_for_complete_second_round(&r) {
                        drop(r);
                        Self::spawn_complete_presign_second_round(
                            epoch,
                            epoch_store.clone(),
                            party_id,
                            session_id,
                            session_ref,
                            state.clone(),
                            presign_session_rounds.clone(),
                            presign_session_states.clone(),
                            submit.clone(),
                        );
                    }
                }
            }
            SignatureMPCMessageProtocols::Sign(m) => {
                let mut state = sign_session_states.entry(session_id).or_insert_with(|| {
                    SignState::new(
                        tiresias_key_share_decryption_key_share,
                        tiresias_public_parameters.clone(),
                        epoch,
                        party_id,
                        parties.clone(),
                        session_id,
                    )
                });

                let _ = state.insert_first_round(sender_party_id, m.clone());

                if let Some(proofs) = state.clone().proofs {
                    if state.should_identify_malicious_actors() {
                        println!("run IA from received last share id {}", state.party_id);
                        let _ = identify_malicious(&state);
                        return;
                    }
                }
                if let Some(r) = sign_session_rounds.get_mut(&session_id) {
                    if state.ready_for_complete_first_round(&r) {
                        drop(r);
                        Self::spawn_complete_sign_round(
                            epoch,
                            epoch_store.clone(),
                            party_id,
                            session_id,
                            session_ref,
                            sign_session_rounds.clone(),
                            sign_session_states.clone(),
                            submit.clone(),
                        );
                    }
                }
            }
            SignatureMPCMessageProtocols::SignProofs(
                prover_party_id,
                new_proofs,
                message_indices,
                involved_parties,
            ) => {
                println!("recv proof from {}", prover_party_id);
                let mut state = sign_session_states.entry(session_id).or_insert_with(|| {
                    SignState::new(
                        tiresias_key_share_decryption_key_share,
                        tiresias_public_parameters.clone(),
                        epoch,
                        party_id,
                        parties.clone(),
                        session_id,
                    )
                });
                state.failed_messages_indices = Some(message_indices.clone());
                state.involved_parties = involved_parties.clone();
                state.insert_proofs(prover_party_id.clone(), new_proofs.clone());
                println!("proofs len: {}", state.clone().proofs.unwrap().len());
                if state.should_identify_malicious_actors() {
                    println!(
                        "received all proofs, run IA from received proof, id {}",
                        state.party_id
                    );
                    let _ = identify_malicious(&state);
                }
                if state.clone().proofs.unwrap().contains_key(&party_id) {
                    return;
                }
                spawn_generate_proof(
                    epoch,
                    epoch_store.clone(),
                    party_id,
                    session_id,
                    session_ref,
                    sign_session_rounds.clone(),
                    sign_session_states.clone(),
                    submit.clone(),
                    message_indices.clone(),
                    involved_parties.clone(),
                );
            }
            _ => {}
        }
    }

    fn spawn_complete_dkg_round(
        epoch: EpochId,
        epoch_store: Arc<AuthorityPerEpochStore>,
        party_id: PartyID,
        session_id: SignatureMPCSessionID,
        session_ref: ObjectRef,
        state: DKGState,
        dkg_session_rounds: Arc<DashMap<SignatureMPCSessionID, DKGRound>>,
        dkg_session_states: Arc<DashMap<SignatureMPCSessionID, DKGState>>,
        submit: Arc<dyn SubmitSignatureMPC>,
    ) {
        spawn_monitored_task!(async move {
            let m = {
                if let Some(mut round) = dkg_session_rounds.get_mut(&session_id) {
                    round.complete_round(state.clone()).ok()
                } else {
                    None
                }
            };
            if let Some(m) = m {
                match m {
                    DKGRoundCompletion::Message(m) => {
                        // if let Some(mut s) = dkg_session_states.get_mut(&session_id) {
                        //     let _ = s.insert_first_round(party_id, m.clone());
                        //     drop(s);
                        // }
                        let _ = submit
                            .sign_and_submit_message(
                                &SignatureMPCMessageSummary::new(
                                    epoch,
                                    SignatureMPCMessageProtocols::DKG(m),
                                    session_id,
                                ),
                                &epoch_store,
                            )
                            .await;
                    }
                    DKGRoundCompletion::Output(secret_key_share_encryption_and_proof) => {
                        let _ = submit
                            .sign_and_submit_output(
                                &SignatureMPCOutput::new_dkg(
                                    epoch,
                                    session_id,
                                    session_ref,
                                    state
                                        .get_commitment_to_centralized_party_secret_key_share()
                                        .unwrap(),
                                    secret_key_share_encryption_and_proof,
                                )
                                .unwrap(),
                                &epoch_store,
                            )
                            .await;
                    }
                    DKGRoundCompletion::None => {}
                }
            }
        });
    }

    fn spawn_complete_presign_first_round(
        epoch: EpochId,
        epoch_store: Arc<AuthorityPerEpochStore>,
        party_id: PartyID,
        session_id: SignatureMPCSessionID,
        session_ref: ObjectRef,
        state: PresignState,
        presign_session_rounds: Arc<DashMap<SignatureMPCSessionID, PresignRound>>,
        presign_session_states: Arc<DashMap<SignatureMPCSessionID, PresignState>>,
        submit: Arc<dyn SubmitSignatureMPC>,
    ) {
        spawn_monitored_task!(async move {
            let m = {
                if let Some(mut round) = presign_session_rounds.get_mut(&session_id) {
                    round.complete_round(state.clone()).ok()
                } else {
                    None
                }
            };
            if let Some(m) = m {
                match m {
                    PresignRoundCompletion::Message(m) => {
                        let _ = submit
                            .sign_and_submit_message(
                                &SignatureMPCMessageSummary::new(
                                    epoch,
                                    SignatureMPCMessageProtocols::PresignFirstRound(m),
                                    session_id,
                                ),
                                &epoch_store,
                            )
                            .await;
                    }
                    PresignRoundCompletion::FirstRoundOutput((
                        output,
                        message_to_submit,
                        individual_encrypted_nonce_shares_and_public_shares,
                    )) => {
                        {
                            if let Some(mut s) = presign_session_states.get_mut(&session_id) {
                                let _ = s.set_individual_encrypted_nonce_shares_and_public_shares(
                                    individual_encrypted_nonce_shares_and_public_shares,
                                );
                            }
                        }
                        let _ = submit
                            .sign_and_submit_message(
                                &SignatureMPCMessageSummary::new(
                                    epoch,
                                    SignatureMPCMessageProtocols::PresignSecondRound(message_to_submit),
                                    session_id,
                                ),
                                &epoch_store,
                            )
                            .await;
                        let _ = submit
                            .sign_and_submit_output(
                                &SignatureMPCOutput::new_presign_output(
                                    epoch,
                                    session_id,
                                    session_ref,
                                    output,
                                )
                                .unwrap(),
                                &epoch_store,
                            )
                            .await;
                    }
                    PresignRoundCompletion::SecondRoundOutput(_) => {
                        // TODO: should never happen, add error
                    }
                    PresignRoundCompletion::None => {}
                }
            }
        });
    }

    fn spawn_complete_presign_second_round(
        epoch: EpochId,
        epoch_store: Arc<AuthorityPerEpochStore>,
        party_id: PartyID,
        session_id: SignatureMPCSessionID,
        session_ref: ObjectRef,
        state: PresignState,
        presign_session_rounds: Arc<DashMap<SignatureMPCSessionID, PresignRound>>,
        presign_session_states: Arc<DashMap<SignatureMPCSessionID, PresignState>>,
        submit: Arc<dyn SubmitSignatureMPC>,
    ) {
        spawn_monitored_task!(async move {
            let m = {
                if let Some(mut round) = presign_session_rounds.get_mut(&session_id) {
                    round.complete_round(state.clone()).ok()
                } else {
                    None
                }
            };
            if let Some(m) = m {
                match m {
                    PresignRoundCompletion::Message(m) => {
                        let _ = submit
                            .sign_and_submit_message(
                                &SignatureMPCMessageSummary::new(
                                    epoch,
                                    SignatureMPCMessageProtocols::PresignSecondRound(m),
                                    session_id,
                                ),
                                &epoch_store,
                            )
                            .await;
                    }
                    PresignRoundCompletion::FirstRoundOutput(_) => {
                        // TODO: should never happen, add error
                    }
                    PresignRoundCompletion::SecondRoundOutput(presigns) => {
                        let _ = submit
                            .sign_and_submit_output(
                                &SignatureMPCOutput::new_presign(
                                    epoch,
                                    session_id,
                                    session_ref,
                                    presigns,
                                )
                                .unwrap(),
                                &epoch_store,
                            )
                            .await;
                    }
                    PresignRoundCompletion::None => {}
                }
            }
        });
    }

    fn spawn_complete_sign_round(
        epoch: EpochId,
        epoch_store: Arc<AuthorityPerEpochStore>,
        party_id: PartyID,
        session_id: SignatureMPCSessionID,
        session_ref: ObjectRef,
        sign_session_rounds: Arc<DashMap<SignatureMPCSessionID, SignRound>>,
        sign_session_states: Arc<DashMap<SignatureMPCSessionID, SignState>>,
        submit: Arc<dyn SubmitSignatureMPC>,
    ) {
        spawn_monitored_task!(async move {
            let mut mut_state = sign_session_states.get_mut(&session_id).unwrap();
            let state = mut_state.clone();
            let m = match sign_session_rounds.get_mut(&session_id) {
                Some(mut round) => match round.complete_round(state.clone()) {
                    Ok(result) => Some(result),
                    Err(_) => None,
                },
                None => {
                    println!("Failed to find session round");
                    None
                }
            };

            if let Some(m) = m {
                match m {
                    SignRoundCompletion::ProofsMessage(proofs, message_indices, involved_parties) => {
                        mut_state.failed_messages_indices = Some(message_indices.clone());
                        mut_state.involved_parties = involved_parties.clone();
                        let _ = mut_state.insert_proofs(state.party_id, proofs.clone());

                        let _ = submit
                            .sign_and_submit_message(
                                &SignatureMPCMessageSummary::new(
                                    epoch,
                                    SignatureMPCMessageProtocols::SignProofs(
                                        state.party_id,
                                        proofs.clone(),
                                        message_indices,
                                        involved_parties,
                                    ),
                                    session_id,
                                ),
                                &epoch_store,
                            )
                            .await;
                    }
                    SignRoundCompletion::SignatureOutput(sigs) => {
                        let _ = submit
                            .sign_and_submit_output(
                                &SignatureMPCOutput::new_sign(epoch, session_id, session_ref, sigs)
                                    .unwrap(),
                                &epoch_store,
                            )
                            .await;
                    }
                    SignRoundCompletion::None => {}
                    SignRoundCompletion::MaliciousPartiesOutput(_) => {}
                }
            }
        });
    }


    async fn initiate_protocol(
        epoch: EpochId,
        epoch_store: Arc<AuthorityPerEpochStore>,
        party_id: PartyID,
        parties: HashSet<PartyID>,
        tiresias_public_parameters: DecryptionPublicParameters,
        tiresias_key_share_decryption_key_share: SecretKeyShareSizedNumber,
        submit: Arc<dyn SubmitSignatureMPC>,
        session_refs: Arc<DashMap<SignatureMPCSessionID, ObjectRef>>,
        dkg_session_rounds: Arc<DashMap<SignatureMPCSessionID, DKGRound>>,
        dkg_session_states: Arc<DashMap<SignatureMPCSessionID, DKGState>>,
        presign_session_rounds: Arc<DashMap<SignatureMPCSessionID, PresignRound>>,
        presign_session_states: Arc<DashMap<SignatureMPCSessionID, PresignState>>,
        sign_session_rounds: Arc<DashMap<SignatureMPCSessionID, SignRound>>,
        sign_session_states: Arc<DashMap<SignatureMPCSessionID, SignState>>,
        initiate_signature_mpc_protocol: InitiateSignatureMPCProtocol,
    ) {
        match initiate_signature_mpc_protocol {
            InitiateSignatureMPCProtocol::DKG {
                session_id,
                session_ref,
                commitment_to_centralized_party_secret_key_share,
            } => {
                session_refs.insert(session_id, session_ref);
                if let Ok((round, message)) = DKGRound::new(
                    tiresias_public_parameters,
                    epoch,
                    party_id,
                    parties.clone(),
                    session_id,
                    commitment_to_centralized_party_secret_key_share.clone(),
                ) {
                    let mut state = dkg_session_states
                        .entry(session_id)
                        .or_insert_with(|| DKGState::new(epoch, party_id, parties.clone()));

                    state.set(commitment_to_centralized_party_secret_key_share);

                    dkg_session_rounds.insert(session_id, round);

                    let summary = SignatureMPCMessageSummary::new(
                        epoch,
                        SignatureMPCMessageProtocols::DKG(message),
                        session_id,
                    );
                    // TODO: Handle error
                    let _ = submit.sign_and_submit_message(&summary, &epoch_store).await;
                }
            }
            InitiateSignatureMPCProtocol::Presign {
                session_id,
                session_ref,
                dkg_output,
                commitments_and_proof_to_centralized_party_nonce_shares,
            } => {
                session_refs.insert(session_id, session_ref);
                if let Ok((round, message)) = PresignRound::new(
                    tiresias_public_parameters.clone(),
                    epoch,
                    party_id,
                    parties.clone(),
                    session_id,
                    dkg_output,
                    commitments_and_proof_to_centralized_party_nonce_shares.clone(),
                ) {
                    let mut state = presign_session_states.entry(session_id).or_insert_with(|| {
                        PresignState::new(
                            tiresias_public_parameters
                                .encryption_scheme_public_parameters
                                .clone(),
                            epoch,
                            party_id,
                            parties,
                            session_id,
                        )
                    });

                    state.set(commitments_and_proof_to_centralized_party_nonce_shares);

                    presign_session_rounds.insert(session_id, round);

                    let summary = SignatureMPCMessageSummary::new(
                        epoch,
                        SignatureMPCMessageProtocols::PresignFirstRound(message),
                        session_id,
                    );
                    // TODO: Handle error
                    let _ = submit.sign_and_submit_message(&summary, &epoch_store).await;
                }
            }
            InitiateSignatureMPCProtocol::Sign {
                session_id,
                session_ref,
                messages,
                dkg_output,
                public_nonce_encrypted_partial_signature_and_proofs,
                presigns,
                hash,
            } => {
                session_refs.insert(session_id, session_ref);
                if let Ok((round, message)) = SignRound::new(
                    tiresias_public_parameters.clone(),
                    tiresias_key_share_decryption_key_share,
                    party_id,
                    parties.clone(),
                    messages.clone(),
                    dkg_output,
                    public_nonce_encrypted_partial_signature_and_proofs.clone(),
                    presigns.clone(),
                    hash.into(),
                ) {
                    let mut state = sign_session_states.entry(session_id).or_insert_with(|| {
                        SignState::new(
                            tiresias_key_share_decryption_key_share,
                            tiresias_public_parameters,
                            epoch,
                            party_id,
                            parties,
                            session_id,
                        )
                    });

                    state.set(
                        messages,
                        public_nonce_encrypted_partial_signature_and_proofs,
                        presigns.clone(),
                    );

                    sign_session_rounds.insert(session_id, round);

                    let summary = SignatureMPCMessageSummary::new(
                        epoch,
                        SignatureMPCMessageProtocols::Sign(message),
                        session_id,
                    );
                    // TODO: Handle error
                    let result = submit.sign_and_submit_message(&summary, &epoch_store).await;
                    match result {
                        Ok(_) => {}
                        Err(e) => {
                            error!("Failed to submit sign message: {:?}", e);
                        }
                    }
                }
            }
        }
    }
}

/// This is a service used to communicate with other pieces of sui(for ex. authority)
pub struct SignatureMPCService {
    tx_signature_mpc_protocol_message_sender: mpsc::Sender<SignatureMPCMessage>,
}

impl SignatureMPCService {
    pub fn spawn(
        tiresias_public_parameters: DecryptionPublicParameters,
        tiresias_key_share_decryption_key_share: SecretKeyShareSizedNumber,
        state: Arc<AuthorityState>,
        epoch_store: Arc<AuthorityPerEpochStore>,
        submit: Arc<dyn SubmitSignatureMPC>,
        metrics: Arc<SignatureMPCMetrics>,
    ) -> (Arc<Self>, watch::Sender<()> /* The exit sender */) {
        info!("Starting signature mpc service.");

        let (tx_signature_mpc_protocol_message_sender, rx_signature_mpc_protocol_message_sender) =
            mpsc::channel(MAX_MESSAGES_IN_PROGRESS);

        let (exit_snd, exit_rcv) = watch::channel(());

        // TODO: remove unwrap
        let party_id = (epoch_store.committee().authority_index(&state.name).unwrap() + 1) as PartyID;

        let epoch = epoch_store.epoch();

        let rx_initiate_signature_mpc_protocol_sender =
            SignatureMpcSubscriber::new(epoch_store.clone(), exit_rcv.clone());

        let parties = HashSet::from_iter(
            epoch_store
                .committee()
                .authority_indexes()
                .into_iter()
                .map(|p| (p + 1) as PartyID),
        );

        let aggregator = SignatureMPCAggregator::new(
            epoch,
            epoch_store,
            party_id,
            parties,
            tiresias_public_parameters,
            tiresias_key_share_decryption_key_share,
            submit,
            metrics,
            exit_rcv,
            rx_initiate_signature_mpc_protocol_sender,
            rx_signature_mpc_protocol_message_sender,
        );

        spawn_monitored_task!(aggregator.run());

        let mut service = Arc::new(Self {
            tx_signature_mpc_protocol_message_sender,
        });

        (service, exit_snd)
    }
}

impl SignatureMPCServiceNotify for SignatureMPCService {
    fn notify_signature_mpc_message(
        &self,
        epoch_store: &AuthorityPerEpochStore,
        message: &SignatureMPCMessage,
    ) -> SuiResult {
        let _ = message
            .summary
            .verify_committee_sigs_only(epoch_store.committee())?;

        let message = message.clone();
        let sender = self.tx_signature_mpc_protocol_message_sender.clone();
        tokio::spawn(async move {
            sender
                .send(message)
                .await
                .tap_err(|e| warn!("Submit signature mpc message failed with {:?}", e))
                .expect("TODO: panic message");
        });
        //
        // self.tx_signature_mpc_protocol_message_sender.send(message.clone())
        //     .tap_err(|e| warn!("Submit signature mpc message failed with {:?}", e))
        //     .map_err(|e| SuiError::Unknown(format!("{:?}", e)))?;

        Ok(())
    }
}

// test helper
pub struct SignatureMPCServiceNoop {}

impl SignatureMPCServiceNotify for SignatureMPCServiceNoop {
    fn notify_signature_mpc_message(
        &self,
        _: &AuthorityPerEpochStore,
        _: &SignatureMPCMessage,
    ) -> SuiResult {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use either::Either;
    use tokio::sync::mpsc;

    use sui_types::error::SuiResult;
    use sui_types::messages_signature_mpc::{SignatureMPCMessageSummary, SignatureMPCOutput};

    use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
    use crate::signature_mpc::SubmitSignatureMPC;

    #[async_trait::async_trait]
    impl SubmitSignatureMPC for mpsc::Sender<Either<SignatureMPCMessageSummary, SignatureMPCOutput>> {
        async fn sign_and_submit_message(
            &self,
            summary: &SignatureMPCMessageSummary,
            epoch_store: &Arc<AuthorityPerEpochStore>,
        ) -> SuiResult {
            self.try_send(Either::Left(summary.clone())).unwrap();
            Ok(())
        }

        async fn sign_and_submit_output(
            &self,
            output: &SignatureMPCOutput,
            epoch_store: &Arc<AuthorityPerEpochStore>,
        ) -> SuiResult {
            self.try_send(Either::Right(output.clone())).unwrap();
            Ok(())
        }
    }
}
