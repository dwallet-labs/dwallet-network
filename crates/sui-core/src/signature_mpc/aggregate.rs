// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use futures::StreamExt;
use rand::rngs::OsRng;
use std::collections::{HashMap, HashSet};
use std::mem;

use signature_mpc::twopc_mpc_protocols::{
    CommitmentRoundParty, DecommitmentRoundParty, EncDHCommitment, EncDHCommitmentRoundParty,
    EncDHDecommitment, EncDHDecommitmentRoundParty, EncDHProofAggregationOutput,
    EncDHProofAggregationRoundParty, EncDHProofShare, EncDHProofShareRoundParty, EncDLCommitment,
    EncDLCommitmentRoundParty, EncDLDecommitment, EncDLDecommitmentRoundParty,
    EncDLProofAggregationOutput, EncDLProofAggregationRoundParty, EncDLProofShare,
    EncDLProofShareRoundParty, EncryptedMaskAndMaskedNonceShare, EncryptedNonceShareAndPublicShare,
    EnhancedLanguageStatementAccessors, Error, PartyID, ProofAggregationRoundParty,
    ProofShareRoundParty, ProtocolContext, Result, Value,
};
use sui_types::messages_signature_mpc::SignatureMPCBulletProofAggregatesMessage;

#[derive(Default)]
pub(crate) enum BulletProofAggregateRound {
    Decommitment {
        enc_dh: Vec<EncDHDecommitmentRoundParty<ProtocolContext>>,
        enc_dl: Vec<EncDLDecommitmentRoundParty<ProtocolContext>>,
    },
    ProofShare {
        enc_dh: Vec<EncDHProofShareRoundParty<ProtocolContext>>,
        enc_dl: Vec<EncDLProofShareRoundParty<ProtocolContext>>,
    },
    ProofAggregation {
        enc_dh: Vec<EncDHProofAggregationRoundParty<ProtocolContext>>,
        enc_dl: Vec<EncDLProofAggregationRoundParty<ProtocolContext>>,
    },
    #[default]
    None,
}

impl BulletProofAggregateRound {
    pub(crate) fn new(
        enc_dh_commitment_parties: Vec<EncDHCommitmentRoundParty<ProtocolContext>>,
        enc_dl_commitment_parties: Vec<EncDLCommitmentRoundParty<ProtocolContext>>,
    ) -> Result<(Self, SignatureMPCBulletProofAggregatesMessage)> {
        let enc_dh_result = enc_dh_commitment_parties
            .into_iter()
            .map(|p| {
                p.commit_statements_and_statement_mask(&mut OsRng)
                    .map_err(|e| Error::EnhancedMaurer(e))
            })
            .collect::<Result<Vec<(_, _)>>>()?;
        let (enc_dh_commitments, enc_dh_decommitment_round_parties): (Vec<_>, Vec<_>) =
            enc_dh_result.into_iter().unzip();

        let enc_dl_result = enc_dl_commitment_parties
            .into_iter()
            .map(|p| {
                p.commit_statements_and_statement_mask(&mut OsRng)
                    .map_err(|e| Error::EnhancedMaurer(e))
            })
            .collect::<Result<Vec<(_, _)>>>()?;
        let (enc_dl_commitments, enc_dl_decommitment_round_parties): (Vec<_>, Vec<_>) =
            enc_dl_result.into_iter().unzip();

        let commitments = (enc_dh_commitments, enc_dl_commitments);

        let message = SignatureMPCBulletProofAggregatesMessage::Commitment(commitments.clone());

        let round = BulletProofAggregateRound::Decommitment {
            enc_dh: enc_dh_decommitment_round_parties,
            enc_dl: enc_dl_decommitment_round_parties,
        };
        Ok((round, message))
    }

    pub(crate) fn complete_round(
        &mut self,
        state: BulletProofAggregateState,
    ) -> Result<BulletProofAggregateRoundCompletion> {
        let round = mem::take(self);
        match round {
            BulletProofAggregateRound::Decommitment { enc_dh, enc_dl }
                if state.commitments.len() == state.parties.len() =>
            {
                let enc_dh_commitments: Vec<HashMap<_, _>> = (0..enc_dh.len())
                    .map(|i| {
                        state
                            .commitments
                            .iter()
                            .map(|(party_id, (commitments, _))| (*party_id, commitments[i].clone()))
                            .collect()
                    })
                    .collect();

                let enc_dh_result = enc_dh
                    .into_iter()
                    .zip(enc_dh_commitments.into_iter())
                    .map(|(party, enc_dh_commitments)| {
                        party
                            .decommit_statements_and_statement_mask(enc_dh_commitments, &mut OsRng)
                            .map_err(|e| Error::EnhancedMaurer(e))
                    })
                    .collect::<Result<Vec<(_, _)>>>()?;
                let (enc_dh_decommitments, enc_dh_proof_share_round_parties): (Vec<_>, Vec<_>) =
                    enc_dh_result.into_iter().unzip();

                let enc_dl_commitments: Vec<HashMap<_, _>> = (0..enc_dl.len())
                    .map(|i| {
                        state
                            .commitments
                            .iter()
                            .map(|(party_id, (_, commitments))| (*party_id, commitments[i].clone()))
                            .collect()
                    })
                    .collect();

                let enc_dl_result = enc_dl
                    .into_iter()
                    .zip(enc_dl_commitments.into_iter())
                    .map(|(party, enc_dl_commitments)| {
                        party
                            .decommit_statements_and_statement_mask(enc_dl_commitments, &mut OsRng)
                            .map_err(|e| Error::EnhancedMaurer(e))
                    })
                    .collect::<Result<Vec<(_, _)>>>()?;
                let (enc_dl_decommitments, enc_dl_proof_share_round_parties): (Vec<_>, Vec<_>) =
                    enc_dl_result.into_iter().unzip();

                let decommitments = (enc_dh_decommitments, enc_dl_decommitments);

                let message =
                    SignatureMPCBulletProofAggregatesMessage::Decommitment(decommitments.clone());

                *self = BulletProofAggregateRound::ProofShare {
                    enc_dh: enc_dh_proof_share_round_parties,
                    enc_dl: enc_dl_proof_share_round_parties,
                };
                Ok(BulletProofAggregateRoundCompletion::Message(message))
            }
            BulletProofAggregateRound::ProofShare { enc_dh, enc_dl }
                if state.decommitments.len() == state.parties.len() =>
            {
                let enc_dh_decommitments: Vec<HashMap<_, _>> = (0..enc_dh.len())
                    .map(|i| {
                        state
                            .decommitments
                            .iter()
                            .map(|(party_id, (decommitments, _))| {
                                (*party_id, decommitments[i].clone())
                            })
                            .collect()
                    })
                    .collect();

                let enc_dh_result = enc_dh
                    .into_iter()
                    .zip(enc_dh_decommitments.into_iter())
                    .map(|(party, enc_dh_decommitments)| {
                        party
                            .generate_proof_share(enc_dh_decommitments, &mut OsRng)
                            .map_err(|e| Error::EnhancedMaurer(e))
                    })
                    .collect::<Result<Vec<(_, _)>>>()?;
                let (enc_dh_proof_shares, enc_dh_proof_aggregation_round_parties): (
                    Vec<_>,
                    Vec<_>,
                ) = enc_dh_result.into_iter().unzip();

                let enc_dl_decommitments: Vec<HashMap<_, _>> = (0..enc_dl.len())
                    .map(|i| {
                        state
                            .decommitments
                            .iter()
                            .map(|(party_id, (_, decommitments))| {
                                (*party_id, decommitments[i].clone())
                            })
                            .collect()
                    })
                    .collect();

                let enc_dl_result = enc_dl
                    .into_iter()
                    .zip(enc_dl_decommitments.into_iter())
                    .map(|(party, enc_dl_decommitments)| {
                        party
                            .generate_proof_share(enc_dl_decommitments, &mut OsRng)
                            .map_err(|e| Error::EnhancedMaurer(e))
                    })
                    .collect::<Result<Vec<(_, _)>>>()?;
                let (enc_dl_proof_shares, enc_dl_proof_aggregation_round_parties): (
                    Vec<_>,
                    Vec<_>,
                ) = enc_dl_result.into_iter().unzip();

                let proof_shares = (enc_dh_proof_shares, enc_dl_proof_shares);

                let message =
                    SignatureMPCBulletProofAggregatesMessage::ProofShare(proof_shares.clone());

                *self = BulletProofAggregateRound::ProofAggregation {
                    enc_dh: enc_dh_proof_aggregation_round_parties,
                    enc_dl: enc_dl_proof_aggregation_round_parties,
                };
                Ok(BulletProofAggregateRoundCompletion::Message(message))
            }
            BulletProofAggregateRound::ProofAggregation { enc_dh, enc_dl }
                if state.proof_shares.len() == state.parties.len() =>
            {
                let enc_dh_proof_shares: Vec<HashMap<_, _>> = (0..enc_dh.len())
                    .map(|i| {
                        state
                            .proof_shares
                            .iter()
                            .map(|(party_id, (proof_shares, _))| {
                                (*party_id, proof_shares[i].clone())
                            })
                            .collect()
                    })
                    .collect();

                let decommitments_vecs: HashMap<_, Vec<_>> = state
                    .decommitments
                    .iter()
                    .map(|(party_id, v)| {
                        (
                            *party_id,
                            v.0.iter()
                                .map(|(decommitment, _)| decommitment.clone())
                                .collect(),
                        )
                    })
                    .collect();

                let individual_encrypted_masked_nonce_shares = decommitments_vecs
                    .into_iter()
                    .map(|(party_id, decommitments)| {
                        (
                            party_id,
                            decommitments
                                .into_iter()
                                .flat_map(|maurer_decommitment| {
                                    maurer_decommitment.statements.into_iter().map(|statement| {
                                        let (_, language_statement) = statement.into();

                                        language_statement
                                    })
                                })
                                .collect(),
                        )
                    })
                    .collect();

                let enc_dh_output = enc_dh
                    .into_iter()
                    .zip(enc_dh_proof_shares.into_iter())
                    .map(|(party, enc_dh_proof_shares)| {
                        party
                            .aggregate_proof_shares(enc_dh_proof_shares, &mut OsRng)
                            .map_err(|e| Error::EnhancedMaurer(e))
                    })
                    .collect::<Result<Vec<(_, _)>>>()?;

                let enc_dl_proof_shares: Vec<HashMap<_, _>> = (0..enc_dl.len())
                    .map(|i| {
                        state
                            .proof_shares
                            .iter()
                            .map(|(party_id, (_, proof_shares))| {
                                (*party_id, proof_shares[i].clone())
                            })
                            .collect()
                    })
                    .collect();

                // let individual_encrypted_nonce_shares_and_public_shares: IndividualEncryptedNonceSharesAndPublicShares = enc_dl.iter()
                //     .map(|p| p.maurer_proof_aggregation_round_party.statements.clone().into_iter().map(|(id, s)| (id, s.clone().into_iter().map(|f| f.language_statement().clone()).collect::<Vec<_>>())).collect::<HashMap<_, _>>())
                //     .reduce(|i1, i2| i1.into_iter().map(|(id, s)| (id, i2.get(&id).unwrap().clone().into_iter().chain(s.into_iter()).collect::<Vec<_>>())).collect::<HashMap<_, _>>()).unwrap();

                let decommitments_vecs: HashMap<_, Vec<_>> = state
                    .decommitments
                    .iter()
                    .map(|(party_id, v)| {
                        (
                            *party_id,
                            v.1.iter()
                                .map(|(decommitment, _)| decommitment.clone())
                                .collect(),
                        )
                    })
                    .collect();

                let individual_encrypted_nonce_shares_and_public_shares = decommitments_vecs
                    .into_iter()
                    .map(|(party_id, decommitments)| {
                        (
                            party_id,
                            decommitments
                                .into_iter()
                                .flat_map(|maurer_decommitment| {
                                    maurer_decommitment.statements.into_iter().map(|statement| {
                                        let (_, language_statement) = statement.into();

                                        language_statement
                                    })
                                })
                                .collect(),
                        )
                    })
                    .collect();

                let enc_dl_output = enc_dl
                    .into_iter()
                    .zip(enc_dl_proof_shares.into_iter())
                    .map(|(party, enc_dl_proof_shares)| {
                        party
                            .aggregate_proof_shares(enc_dl_proof_shares, &mut OsRng)
                            .map_err(|e| Error::EnhancedMaurer(e))
                    })
                    .collect::<Result<Vec<(_, _)>>>()?;

                Ok(BulletProofAggregateRoundCompletion::Output((
                    (enc_dh_output, enc_dl_output),
                    (
                        individual_encrypted_masked_nonce_shares,
                        individual_encrypted_nonce_shares_and_public_shares,
                    ),
                )))
            }
            _ => Ok(BulletProofAggregateRoundCompletion::None),
        }
    }
}

pub(crate) enum BulletProofAggregateRoundCompletion {
    Message(SignatureMPCBulletProofAggregatesMessage),
    Output(
        (
            (
                Vec<EncDHProofAggregationOutput<ProtocolContext>>,
                Vec<EncDLProofAggregationOutput<ProtocolContext>>,
            ),
            (
                HashMap<PartyID, Vec<Value<EncryptedMaskAndMaskedNonceShare>>>,
                HashMap<PartyID, Vec<Value<EncryptedNonceShareAndPublicShare>>>,
            ),
        ),
    ),
    None,
}

#[derive(Clone)]
pub(crate) struct BulletProofAggregateState {
    party_id: PartyID,
    parties: HashSet<PartyID>,

    commitments: HashMap<
        PartyID,
        (
            Vec<EncDHCommitment<ProtocolContext>>,
            Vec<EncDLCommitment<ProtocolContext>>,
        ),
    >,
    decommitments: HashMap<
        PartyID,
        (
            Vec<EncDHDecommitment<ProtocolContext>>,
            Vec<EncDLDecommitment<ProtocolContext>>,
        ),
    >,
    proof_shares: HashMap<
        PartyID,
        (
            Vec<EncDHProofShare<ProtocolContext>>,
            Vec<EncDLProofShare<ProtocolContext>>,
        ),
    >,
}

impl BulletProofAggregateState {
    pub(crate) fn new(party_id: PartyID, parties: HashSet<PartyID>) -> Self {
        Self {
            party_id,
            parties,
            commitments: Default::default(),
            decommitments: Default::default(),
            proof_shares: Default::default(),
        }
    }

    pub(crate) fn insert(
        &mut self,
        party_id: PartyID,
        message: SignatureMPCBulletProofAggregatesMessage,
    ) -> Result<()> {
        // TODO: how to handle double message?
        match message {
            SignatureMPCBulletProofAggregatesMessage::Commitment(message) => {
                self.commitments.insert(party_id, message);
            }
            SignatureMPCBulletProofAggregatesMessage::Decommitment(message) => {
                self.decommitments.insert(party_id, message);
            }
            SignatureMPCBulletProofAggregatesMessage::ProofShare(message) => {
                self.proof_shares.insert(party_id, message);
            }
        };
        Ok(())
    }

    pub(crate) fn ready_for_complete_round(&self, round: &BulletProofAggregateRound) -> bool {
        match round {
            BulletProofAggregateRound::Decommitment { .. }
                if self.commitments.len() == self.parties.len() =>
            {
                true
            }
            BulletProofAggregateRound::ProofShare { .. }
                if self.decommitments.len() == self.parties.len() =>
            {
                true
            }
            BulletProofAggregateRound::ProofAggregation { .. }
                if self.proof_shares.len() == self.parties.len() =>
            {
                true
            }
            _ => false,
        }
    }
}
