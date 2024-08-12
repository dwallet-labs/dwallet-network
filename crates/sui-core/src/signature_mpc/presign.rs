// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::signature_mpc::aggregate::{
    BulletProofAggregateRound, BulletProofAggregateRoundCompletion, BulletProofAggregateState,
};
use rand::rngs::OsRng;
use signature_mpc::twopc_mpc_protocols::{
    initiate_decentralized_party_presign, new_decentralized_party_presign_batch,
    DKGDecentralizedPartyOutput, DecentralizedPartyPresign, DecryptionPublicParameters,
    EncryptedMaskAndMaskedNonceShare, EncryptedMaskedNoncesRoundParty,
    EncryptedNonceShareAndPublicShare, EncryptionPublicParameters,
    EnhancedLanguageStatementAccessors, PartyID, PresignDecentralizedPartyOutput, ProtocolContext,
    Result, SignatureNonceSharesCommitmentsAndBatchedProof, Value,
};
use std::collections::{HashMap, HashSet};
use std::mem;
use sui_types::base_types::EpochId;
use sui_types::messages_signature_mpc::SignatureMPCBulletProofAggregatesMessage;
use sui_types::messages_signature_mpc::SignatureMPCSessionID;

#[derive(Default)]
pub(crate) enum PresignRound {
    FirstRound {
        bullet_proof_aggregates_round: BulletProofAggregateRound,
        decentralized_party_encrypted_masked_nonce_shares_round_party:
            EncryptedMaskedNoncesRoundParty<ProtocolContext>,
    },
    SecondRound {
        bullet_proof_aggregates_round: BulletProofAggregateRound,
        masks_and_encrypted_masked_key_shares: Vec<EncryptedMaskAndMaskedNonceShare>,
        encrypted_nonce_shares_and_public_shares: Vec<EncryptedNonceShareAndPublicShare>,
    },
    #[default]
    None,
}

impl PresignRound {
    pub(crate) fn new(
        tiresias_public_parameters: DecryptionPublicParameters,
        _epoch: EpochId,
        party_id: PartyID,
        parties: HashSet<PartyID>,
        _session_id: SignatureMPCSessionID,
        dkg_output: DKGDecentralizedPartyOutput,
        commitments_and_proof_to_centralized_party_nonce_shares: SignatureNonceSharesCommitmentsAndBatchedProof<ProtocolContext>,
    ) -> Result<(Self, SignatureMPCBulletProofAggregatesMessage)> {
        let decentralized_party_encrypted_masked_key_share_and_public_nonce_shares_party =
            initiate_decentralized_party_presign(
                tiresias_public_parameters,
                party_id,
                parties.clone(),
                dkg_output.clone(),
            )?;

        let (
            (
                decentralized_party_encrypted_masked_key_share_commitment_round_party,
                decentralized_party_public_nonce_shares_commitment_round_party,
            ),
            decentralized_party_encrypted_masked_nonce_shares_round_party,
        ) = decentralized_party_encrypted_masked_key_share_and_public_nonce_shares_party
            .sample_mask_and_nonce_shares_and_initialize_proof_aggregation(
                commitments_and_proof_to_centralized_party_nonce_shares.clone(),
                &mut OsRng,
            )?;

        let (bullet_proof_aggregates_round, message) = BulletProofAggregateRound::new(
            Vec::from([decentralized_party_encrypted_masked_key_share_commitment_round_party]),
            Vec::from([decentralized_party_public_nonce_shares_commitment_round_party]),
        )?;

        let round = PresignRound::FirstRound {
            bullet_proof_aggregates_round,
            decentralized_party_encrypted_masked_nonce_shares_round_party,
        };

        Ok((round, message))
    }

    pub(crate) fn complete_round(&mut self, state: PresignState) -> Result<PresignRoundCompletion> {
        let round = mem::take(self);
        match round {
            PresignRound::FirstRound {
                mut bullet_proof_aggregates_round,
                decentralized_party_encrypted_masked_nonce_shares_round_party,
            } => {
                let message = match bullet_proof_aggregates_round
                    .complete_round(state.first_round_bullet_proof_aggregates_state)?
                {
                    BulletProofAggregateRoundCompletion::Message(m) => {
                        *self = PresignRound::FirstRound {
                            bullet_proof_aggregates_round,
                            decentralized_party_encrypted_masked_nonce_shares_round_party,
                        };
                        PresignRoundCompletion::Message(m)
                    }
                    BulletProofAggregateRoundCompletion::Output((
                        (masked_key_share, public_nonce_shares),
                        (_, individual_encrypted_nonce_shares_and_public_shares),
                    )) => {
                        let (
                            masks_and_encrypted_masked_key_share_proof,
                            masks_and_encrypted_masked_key_share,
                        ) = masked_key_share.first().unwrap().clone();
                        let (
                            encrypted_nonce_shares_and_public_shares_proof,
                            encrypted_nonce_shares_and_public_shares,
                        ) = public_nonce_shares.first().unwrap().clone();

                        let output = PresignDecentralizedPartyOutput::new(
                            masks_and_encrypted_masked_key_share.clone(),
                            masks_and_encrypted_masked_key_share_proof,
                            encrypted_nonce_shares_and_public_shares.clone(),
                            encrypted_nonce_shares_and_public_shares_proof,
                        )
                        .unwrap();

                        let masks_and_encrypted_masked_key_shares: Vec<
                            EncryptedMaskAndMaskedNonceShare,
                        > = masks_and_encrypted_masked_key_share
                            .into_iter()
                            .map(|mask_and_encrypted_masked_key_share| {
                                mask_and_encrypted_masked_key_share
                                    .language_statement()
                                    .clone()
                            })
                            .collect();

                        let encrypted_nonce_shares_and_public_shares: Vec<
                            EncryptedNonceShareAndPublicShare,
                        > = encrypted_nonce_shares_and_public_shares
                            .into_iter()
                            .map(|encrypted_nonce_share_and_public_share| {
                                encrypted_nonce_share_and_public_share
                                    .language_statement()
                                    .clone()
                            })
                            .collect();

                        let decentralized_party_encrypted_masked_nonce_shares_commitment_round_party =
                            decentralized_party_encrypted_masked_nonce_shares_round_party
                                .initialize_proof_aggregation(
                                    masks_and_encrypted_masked_key_shares.clone(),
                                    encrypted_nonce_shares_and_public_shares.clone(),
                                    &mut OsRng,
                                )?;

                        let (bullet_proof_aggregates_round, message) =
                            BulletProofAggregateRound::new(
                                Vec::from(decentralized_party_encrypted_masked_nonce_shares_commitment_round_party),
                                Vec::new(),
                            ).unwrap();

                        *self = PresignRound::SecondRound {
                            bullet_proof_aggregates_round,
                            masks_and_encrypted_masked_key_shares,
                            encrypted_nonce_shares_and_public_shares,
                        };
                        PresignRoundCompletion::FirstRoundOutput((
                            output,
                            message,
                            individual_encrypted_nonce_shares_and_public_shares,
                        ))
                    }
                    BulletProofAggregateRoundCompletion::None => PresignRoundCompletion::None,
                };
                Ok(message)
            }
            PresignRound::SecondRound {
                mut bullet_proof_aggregates_round,
                masks_and_encrypted_masked_key_shares,
                encrypted_nonce_shares_and_public_shares,
            } => {
                let message = match bullet_proof_aggregates_round
                    .complete_round(state.second_round_bullet_proof_aggregates_state)?
                {
                    BulletProofAggregateRoundCompletion::Message(m) => {
                        *self = PresignRound::SecondRound {
                            bullet_proof_aggregates_round,
                            masks_and_encrypted_masked_key_shares,
                            encrypted_nonce_shares_and_public_shares,
                        };
                        PresignRoundCompletion::Message(m)
                    }
                    BulletProofAggregateRoundCompletion::Output((
                        (enc_dh_proof_aggregation_outputs, _),
                        (individual_encrypted_masked_nonce_shares, _),
                    )) => {
                        let encrypted_masked_nonce_shares = enc_dh_proof_aggregation_outputs
                            .into_iter()
                            .map(|(_, encrypted_masked_nonce_share)| encrypted_masked_nonce_share)
                            .collect::<Vec<_>>();
                        let encrypted_masked_nonce_shares: Vec<_> = encrypted_masked_nonce_shares
                            .into_iter()
                            .flatten()
                            .map(|encrypted_masked_nonce_share| {
                                encrypted_masked_nonce_share.language_statement().clone()
                            })
                            .collect();

                        let decentralized_party_presigns: Vec<_> =
                            new_decentralized_party_presign_batch(
                                state.parties,
                                state
                                    .commitments_and_proof_to_centralized_party_nonce_shares
                                    .clone()
                                    .unwrap(),
                                masks_and_encrypted_masked_key_shares,
                                state
                                    .individual_encrypted_nonce_shares_and_public_shares
                                    .unwrap(),
                                encrypted_nonce_shares_and_public_shares,
                                individual_encrypted_masked_nonce_shares,
                                encrypted_masked_nonce_shares,
                            )?;

                        PresignRoundCompletion::SecondRoundOutput(decentralized_party_presigns)
                    }
                    BulletProofAggregateRoundCompletion::None => PresignRoundCompletion::None,
                };
                Ok(message)
            }
            PresignRound::None => Ok(PresignRoundCompletion::None),
        }
    }
}

pub(crate) enum PresignRoundCompletion {
    Message(SignatureMPCBulletProofAggregatesMessage),
    FirstRoundOutput(
        (
            PresignDecentralizedPartyOutput<ProtocolContext>,
            SignatureMPCBulletProofAggregatesMessage,
            HashMap<PartyID, Vec<Value<EncryptedNonceShareAndPublicShare>>>,
        ),
    ),
    SecondRoundOutput(Vec<DecentralizedPartyPresign>),
    None,
}

#[derive(Clone)]
pub(crate) struct PresignState {
    party_id: PartyID,
    parties: HashSet<PartyID>,

    commitments_and_proof_to_centralized_party_nonce_shares:
        Option<SignatureNonceSharesCommitmentsAndBatchedProof<ProtocolContext>>,

    individual_encrypted_nonce_shares_and_public_shares:
        Option<HashMap<PartyID, Vec<Value<EncryptedNonceShareAndPublicShare>>>>,

    first_round_bullet_proof_aggregates_state: BulletProofAggregateState,
    second_round_bullet_proof_aggregates_state: BulletProofAggregateState,
}

impl PresignState {
    pub(crate) fn new(
        _tiresias_public_parameters: EncryptionPublicParameters,
        _epoch: EpochId,
        party_id: PartyID,
        parties: HashSet<PartyID>,
        _session_id: SignatureMPCSessionID,
    ) -> Self {
        Self {
            party_id,
            parties: parties.clone(),
            commitments_and_proof_to_centralized_party_nonce_shares: None,
            individual_encrypted_nonce_shares_and_public_shares: None,
            first_round_bullet_proof_aggregates_state: BulletProofAggregateState::new(
                party_id,
                parties.clone(),
            ),
            second_round_bullet_proof_aggregates_state: BulletProofAggregateState::new(
                party_id, parties,
            ),
        }
    }

    pub(crate) fn set(
        &mut self,
        commitments_and_proof_to_centralized_party_nonce_shares: SignatureNonceSharesCommitmentsAndBatchedProof<ProtocolContext>,
    ) {
        self.commitments_and_proof_to_centralized_party_nonce_shares =
            Some(commitments_and_proof_to_centralized_party_nonce_shares);
    }

    pub(crate) fn set_individual_encrypted_nonce_shares_and_public_shares(
        &mut self,
        individual_encrypted_nonce_shares_and_public_shares: HashMap<
            PartyID,
            Vec<Value<EncryptedNonceShareAndPublicShare>>,
        >,
    ) {
        self.individual_encrypted_nonce_shares_and_public_shares =
            Some(individual_encrypted_nonce_shares_and_public_shares);
    }

    pub(crate) fn insert_first_round(
        &mut self,
        party_id: PartyID,
        message: SignatureMPCBulletProofAggregatesMessage,
    ) -> Result<()> {
        self.first_round_bullet_proof_aggregates_state
            .insert(party_id, message)
    }

    pub(crate) fn insert_second_round(
        &mut self,
        party_id: PartyID,
        message: SignatureMPCBulletProofAggregatesMessage,
    ) -> Result<()> {
        self.second_round_bullet_proof_aggregates_state
            .insert(party_id, message)
    }

    pub(crate) fn ready_for_complete_first_round(&self, round: &PresignRound) -> bool {
        if let PresignRound::FirstRound {
            bullet_proof_aggregates_round,
            ..
        } = round
        {
            self.first_round_bullet_proof_aggregates_state
                .ready_for_complete_round(bullet_proof_aggregates_round)
        } else {
            false
        }
    }

    pub(crate) fn ready_for_complete_second_round(&self, round: &PresignRound) -> bool {
        if let PresignRound::SecondRound {
            bullet_proof_aggregates_round,
            ..
        } = round
        {
            self.second_round_bullet_proof_aggregates_state
                .ready_for_complete_round(bullet_proof_aggregates_round)
        } else {
            false
        }
    }
}
