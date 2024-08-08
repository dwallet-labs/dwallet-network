// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use rand::rngs::OsRng;
use signature_mpc::twopc_mpc_protocols::{
    decrypt_signature_decentralized_party_sign, initiate_decentralized_party_sign, message_digest,
    AdditivelyHomomorphicDecryptionKeyShare, DKGDecentralizedPartyOutput,
    DecentralizedPartyPresign, DecryptionPublicParameters, Hash, PaillierModulusSizedNumber,
    PartyID, ProtocolContext, PublicNonceEncryptedPartialSignatureAndProof, Result,
    SecretKeyShareSizedNumber, SignatureThresholdDecryptionParty,
};
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::mem;
use sui_types::base_types::EpochId;
use sui_types::messages_signature_mpc::{SignMessage, SignatureMPCSessionID};
use twopc_mpc::secp256k1::paillier::bulletproofs::PartialDecryptionProof;

#[derive(Default)]
pub(crate) enum SignRound {
    FirstRound {
        signature_threshold_decryption_round_parties: Vec<SignatureThresholdDecryptionParty>,
    },
    #[default]
    None,
}

impl SignRound {
    pub(crate) fn new(
        tiresias_public_parameters: DecryptionPublicParameters,
        tiresias_key_share_decryption_key_share: SecretKeyShareSizedNumber,
        party_id: PartyID,
        parties: HashSet<PartyID>,
        messages: Vec<Vec<u8>>,
        dkg_output: DKGDecentralizedPartyOutput,
        public_nonce_encrypted_partial_signature_and_proofs: Vec<
            PublicNonceEncryptedPartialSignatureAndProof<ProtocolContext>,
        >,
        presigns: Vec<DecentralizedPartyPresign>,
        hash: Hash,
    ) -> Result<(
        Self,
        Vec<(PaillierModulusSizedNumber, PaillierModulusSizedNumber)>,
    )> {
        let sign_mpc_party_per_message = initiate_decentralized_party_sign(
            tiresias_key_share_decryption_key_share,
            tiresias_public_parameters.clone(),
            party_id,
            parties.clone(),
            dkg_output,
            presigns.clone(),
        )?;

        let (decryption_shares, signature_threshold_decryption_round_parties): (Vec<_>, Vec<_>) =
            messages
                .iter()
                .zip(sign_mpc_party_per_message.into_iter())
                .zip(
                    public_nonce_encrypted_partial_signature_and_proofs
                        .clone()
                        .into_iter(),
                )
                .map(
                    |((m, party), public_nonce_encrypted_partial_signature_and_proof)| {
                        let m = message_digest(m, &hash);
                        party.partially_decrypt_encrypted_signature_parts_prehash(
                            m,
                            public_nonce_encrypted_partial_signature_and_proof,
                            &mut OsRng,
                        )
                    },
                )
                .collect::<Result<
                    Vec<(
                        (PaillierModulusSizedNumber, PaillierModulusSizedNumber),
                        SignatureThresholdDecryptionParty,
                    )>,
                >>()?
                .into_iter()
                .unzip();
        Ok((
            SignRound::FirstRound {
                signature_threshold_decryption_round_parties,
            },
            decryption_shares,
        ))
    }

    /// Tries to decrypt the signatures and return them.
    /// If one or more of the signatures is invalid,
    /// it will return a [`SignRoundCompletion::StartIdentifiableAbortFlow`] to launch the IA flow.
    pub(crate) fn complete_round(&mut self, state: SignState) -> Result<SignRoundCompletion> {
        let round = mem::take(self);
        match round {
            // todo(zeev): remove this enum and handle the `mem` code.
            SignRound::FirstRound {
                signature_threshold_decryption_round_parties,
            } => {
                let decrypt_result = decrypt_signature_decentralized_party_sign(
                    state.tiresias_public_parameters.clone(),
                    state.decryption_shares.clone(),
                    state
                        .public_nonce_encrypted_partial_signature_and_proofs
                        .clone()
                        .unwrap(),
                    signature_threshold_decryption_round_parties,
                );
                match decrypt_result {
                    Ok(signatures) => Ok(SignRoundCompletion::SignatureOutput(signatures)),
                    Err(decryption_error) => Ok(SignRoundCompletion::StartIdentifiableAbortFlow(
                        decryption_error.failed_messages_indices,
                        decryption_error.decrypters,
                    )),
                }
            }
            _ => Ok(SignRoundCompletion::None),
        }
    }
}

pub(crate) enum SignRoundCompletion {
    SignatureOutput(Vec<Vec<u8>>),
    StartIdentifiableAbortFlow(Vec<usize>, Vec<PartyID>),
    None,
}

#[derive(Clone)]
pub(crate) struct SignState {
    epoch: EpochId,
    pub party_id: PartyID,
    parties: HashSet<PartyID>,
    aggregator_party_id: PartyID,
    pub tiresias_public_parameters: DecryptionPublicParameters,
    pub tiresias_key_share_decryption_key_share: SecretKeyShareSizedNumber,
    pub messages: Option<Vec<Vec<u8>>>,
    pub public_nonce_encrypted_partial_signature_and_proofs:
        Option<Vec<PublicNonceEncryptedPartialSignatureAndProof<ProtocolContext>>>,
    pub presigns: Option<Vec<DecentralizedPartyPresign>>,
    pub decryption_shares:
        HashMap<PartyID, Vec<(PaillierModulusSizedNumber, PaillierModulusSizedNumber)>>,
    pub proofs: Option<HashMap<PartyID, Vec<PartialDecryptionProof>>>,
    pub failed_messages_indices: Option<Vec<usize>>,
    pub involved_parties: Option<Vec<PartyID>>,
}

impl SignState {
    pub(crate) fn new(
        tiresias_key_share_decryption_key_share: SecretKeyShareSizedNumber,
        tiresias_public_parameters: DecryptionPublicParameters,
        epoch: EpochId,
        party_id: PartyID,
        parties: HashSet<PartyID>,
        session_id: SignatureMPCSessionID,
    ) -> Self {
        let aggregator_party_id = ((u64::from_be_bytes((&session_id.0[0..8]).try_into().unwrap())
            % parties.len() as u64)
            + 1) as PartyID;

        Self {
            epoch,
            party_id,
            parties,
            aggregator_party_id,
            tiresias_public_parameters,
            messages: None,
            public_nonce_encrypted_partial_signature_and_proofs: None,
            decryption_shares: HashMap::new(),
            tiresias_key_share_decryption_key_share,
            presigns: None,
            proofs: None,
            failed_messages_indices: None,
            involved_parties: None,
        }
    }

    pub(crate) fn set(
        &mut self,
        messages: Vec<Vec<u8>>,
        public_nonce_encrypted_partial_signature_and_proofs: Vec<
            PublicNonceEncryptedPartialSignatureAndProof<ProtocolContext>,
        >,
        presigns: Vec<DecentralizedPartyPresign>,
    ) {
        self.messages = Some(messages);
        self.public_nonce_encrypted_partial_signature_and_proofs =
            Some(public_nonce_encrypted_partial_signature_and_proofs);
        self.presigns = Some(presigns);
    }

    pub(crate) fn insert_first_round(
        &mut self,
        sender_id: PartyID,
        message: SignMessage,
    ) -> Result<()> {
        match message {
            SignMessage::DecryptionShares(shares) => {
                let _ = self.decryption_shares.insert(sender_id, shares);
            }
            SignMessage::StartIAFlow(failed_messages_indices, involved_parties) => {
                if self.failed_messages_indices.is_none() {
                    self.failed_messages_indices = Some(failed_messages_indices.clone());
                }
                if self.involved_parties.is_none() {
                    self.involved_parties = Some(involved_parties.clone());
                }
            }
            SignMessage::IAProofs(proofs) => {
                self.insert_proofs(sender_id, proofs.clone());
            }
        }
        Ok(())
    }

    fn insert_proofs(&mut self, party_id: PartyID, new_proofs: Vec<PartialDecryptionProof>) {
        if self.clone().involved_parties.unwrap().contains(&party_id) {
            self.proofs
                .get_or_insert(HashMap::new())
                .insert(party_id, new_proofs);
        }
    }

    pub(crate) fn ready_for_complete_first_round(&self, round: &SignRound) -> bool {
        match round {
            SignRound::FirstRound { .. } => {
                self.received_all_decryption_shares() && self.party_id == self.aggregator_party_id
            }
            _ => false,
        }
    }

    fn received_all_decryption_shares(&self) -> bool {
        self.decryption_shares.len() == self.parties.len()
    }

    pub(crate) fn should_identify_malicious_actors(&self) -> bool {
        // TODO: Handle the case a validator does not send its proof.
        if let Some(proofs) = self.clone().proofs {
            let threshold: usize = self.tiresias_public_parameters.threshold.into();
            return proofs.len() == threshold && self.received_all_decryption_shares();
        }
        false
    }
}
