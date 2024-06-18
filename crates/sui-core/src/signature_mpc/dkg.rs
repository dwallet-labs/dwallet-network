// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use sui_types::messages_signature_mpc::SignatureMPCSessionID;
use std::collections::HashSet;
use std::mem;
use crate::signature_mpc::aggregate::{BulletProofAggregateRound, BulletProofAggregateState, BulletProofAggregateRoundCompletion};
use rand::rngs::OsRng;
use sui_types::base_types::{EpochId};
use signature_mpc::twopc_mpc_protocols::{initiate_decentralized_party_dkg, Commitment, SecretKeyShareEncryptionAndProof, Result, Error, PartyID, ProtocolContext, EncryptionPublicParameters, DecryptionPublicParameters};
use sui_types::messages_signature_mpc::SignatureMPCBulletProofAggregatesMessage;

#[derive(Default)]
pub(crate) enum DKGRound {
    FirstRound {
        bullet_proof_aggregates_round: BulletProofAggregateRound,
    },
    #[default]
    None,
}

impl DKGRound {
    pub(crate) fn new(
        tiresias_public_parameters: DecryptionPublicParameters,
        epoch: EpochId,
        party_id: PartyID,
        parties: HashSet<PartyID>,
        session_id: SignatureMPCSessionID,
        commitment_to_centralized_party_secret_key_share: Commitment,
    ) -> Result<(Self, SignatureMPCBulletProofAggregatesMessage)> {
        let encryption_of_secret_key_share_round_party = initiate_decentralized_party_dkg(
            tiresias_public_parameters,
            //epoch,
            party_id,
            parties.clone(),
            //session_id,
        )?;

        let (encryption_of_secret_key_share_commitment_round_party, _) =
            encryption_of_secret_key_share_round_party
                .sample_secret_key_share_and_initialize_proof_aggregation(
                    commitment_to_centralized_party_secret_key_share,
                    &mut OsRng,
                )
                .unwrap();

        let (round, message) = BulletProofAggregateRound::new(
            Vec::new(),
            Vec::from([encryption_of_secret_key_share_commitment_round_party]),
        )?;
        Ok((DKGRound::FirstRound {
            bullet_proof_aggregates_round: round
        }, message))
    }

    pub(crate) fn complete_round(
        &mut self,
        state: DKGState,
    ) -> Result<DKGRoundCompletion> {
        let round = mem::take(self);
        match round {
            DKGRound::FirstRound {
                bullet_proof_aggregates_round: mut round
            } => {
                let message = match round.complete_round(state.bullet_proof_aggregate_state)?
                {
                    BulletProofAggregateRoundCompletion::Message(m) => {
                        *self = DKGRound::FirstRound {
                            bullet_proof_aggregates_round: round
                        };
                        DKGRoundCompletion::Message(m)
                    }
                    BulletProofAggregateRoundCompletion::Output(((_, o), _)) => {
                        let (secret_share_proof, secret_share) = o.first().unwrap().clone();
                        DKGRoundCompletion::Output(
                            SecretKeyShareEncryptionAndProof::<ProtocolContext>::new(
                                secret_share.first().unwrap().clone(),
                                secret_share_proof,
                            ),
                        )
                    }
                    BulletProofAggregateRoundCompletion::None => {
                        DKGRoundCompletion::None
                    }
                };
                Ok(message)
            }
            _ => Ok(DKGRoundCompletion::None)
        }
    }
}

pub(crate) enum DKGRoundCompletion {
    Message(SignatureMPCBulletProofAggregatesMessage),
    Output(SecretKeyShareEncryptionAndProof<ProtocolContext>),
    None,
}

#[derive(Clone)]
pub(crate) struct DKGState {
    epoch: EpochId,
    party_id: PartyID,
    parties: HashSet<PartyID>,
    commitment_to_centralized_party_secret_key_share: Option<Commitment>,

    bullet_proof_aggregate_state: BulletProofAggregateState,
}

impl DKGState {
    pub(crate) fn new(
        epoch: EpochId,
        party_id: PartyID,
        parties: HashSet<PartyID>,
    ) -> Self {
        Self {
            epoch,
            party_id,
            parties: parties.clone(),
            commitment_to_centralized_party_secret_key_share: None,

            bullet_proof_aggregate_state: BulletProofAggregateState::new(party_id, parties),
        }
    }

    pub(crate) fn set(
        &mut self,
        commitment_to_centralized_party_secret_key_share: Commitment,
    ) {
        self.commitment_to_centralized_party_secret_key_share = Some(commitment_to_centralized_party_secret_key_share);
    }

    pub(crate) fn get_commitment_to_centralized_party_secret_key_share(
        &self,
    ) -> Option<Commitment> {
        self.commitment_to_centralized_party_secret_key_share.clone()
    }

    pub(crate) fn insert_first_round(
        &mut self,
        party_id: PartyID,
        message: SignatureMPCBulletProofAggregatesMessage,
    ) -> Result<()> {
        self
            .bullet_proof_aggregate_state
            .insert(party_id, message)
    }

    pub(crate) fn ready_for_complete_first_round(&self, round: &DKGRound) -> bool {
        if let DKGRound::FirstRound { bullet_proof_aggregates_round: round } = round {
            self.bullet_proof_aggregate_state.ready_for_complete_round(round)
        } else {
            false
        }
    }

}
