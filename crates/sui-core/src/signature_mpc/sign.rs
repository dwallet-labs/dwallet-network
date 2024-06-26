// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use futures::StreamExt;
use rand::rngs::OsRng;
use signature_mpc::decrypt::{
    decrypt_signature_decentralized_party_sign, PartialDecryptionProof,
};
use signature_mpc::twopc_mpc_protocols::{
    AdditivelyHomomorphicDecryptionKeyShare, DecentralizedPartyPresign, DecryptionKeyShare, DecryptionPublicParameters,
    DKGDecentralizedPartyOutput, generate_proof, Hash,
    identify_malicious_parties, initiate_decentralized_party_sign, message_digest, PaillierModulusSizedNumber, PartyID,
    ProofParty, ProtocolContext, PublicNonceEncryptedPartialSignatureAndProof, Result,
    SecretKeyShareSizedNumber, SignatureThresholdDecryptionParty,
};
use std::collections::{HashMap, HashSet};
use std::mem;
use sui_types::base_types::EpochId;
use sui_types::messages_signature_mpc::SignatureMPCSessionID;
use crate::signature_mpc::sign_state::SignState;

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
        epoch: EpochId,
        party_id: PartyID,
        parties: HashSet<PartyID>,
        session_id: SignatureMPCSessionID,
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
            //epoch,
            party_id,
            parties.clone(),
            //session_id,
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

        let mut v = decryption_shares.clone();
        if party_id == 1 || party_id == 2 {
            v[0] = (
                PaillierModulusSizedNumber::from_u16(200),
                PaillierModulusSizedNumber::from_u16(200),
            );
        }
        Ok((
            SignRound::FirstRound {
                signature_threshold_decryption_round_parties,
            },
            v,
        ))
    }

    pub fn generate_proofs(
        state: &SignState,
        failed_messages_indices: &Vec<usize>,
    ) -> Vec<(PartialDecryptionProof, ProofParty)> {
        let decryption_key_share = DecryptionKeyShare::new(
            state.party_id,
            state.tiresias_key_share_decryption_key_share,
            &state.tiresias_public_parameters,
        )
        .unwrap();

        failed_messages_indices
            .iter()
            .map(|index| {
                generate_proof(
                    state.tiresias_public_parameters.clone(),
                    decryption_key_share.clone(),
                    state.party_id,
                    state.presigns.clone().unwrap().get(*index).unwrap().clone(),
                    state
                        .tiresias_public_parameters
                        .encryption_scheme_public_parameters
                        .clone(),
                    state
                        .public_nonce_encrypted_partial_signature_and_proofs
                        .clone()
                        .unwrap()
                        .get(*index)
                        .unwrap()
                        .clone(),
                )
            })
            .collect()
    }

    pub(crate) fn complete_round(&mut self, state: SignState) -> Result<SignRoundCompletion> {
        let round = mem::take(self);
        match round {
            SignRound::FirstRound {
                signature_threshold_decryption_round_parties,
            } => {
                let decrypt_result = decrypt_signature_decentralized_party_sign(
                    state.messages.clone().unwrap(),
                    state.tiresias_public_parameters.clone(),
                    state.decryption_shares.clone(),
                    state
                        .public_nonce_encrypted_partial_signature_and_proofs
                        .clone()
                        .unwrap(),
                    signature_threshold_decryption_round_parties,
                );
                match decrypt_result {
                    Ok(signatures) => {
                        return Ok(SignRoundCompletion::SignatureOutput(signatures));
                    }
                    Err(decryption_error) => {
                        let proofs_tuples =
                            Self::generate_proofs(&state, &decryption_error.failed_messages_indices);
                        let proofs = proofs_tuples.iter().map(|(proof, _)| proof.clone()).collect();
                        Ok(SignRoundCompletion::ProofsMessage(
                            proofs,
                            decryption_error.failed_messages_indices,
                            decryption_error.involved_parties,
                        ))
                    }
                }
            }
            _ => Ok(SignRoundCompletion::None),
        }
    }

    pub(crate) fn identify_malicious(state: &SignState) -> Result<SignRoundCompletion> {
        let proof_results =
            Self::generate_proofs(&state, &state.failed_messages_indices.clone().unwrap());

        let mut malicious_parties = HashSet::new();
        let involved_shares: HashMap<
            PartyID,
            Vec<(PaillierModulusSizedNumber, PaillierModulusSizedNumber)>,
        > = state
            .clone()
            .decryption_shares
            .into_iter()
            .filter(|(party_id, _)| state.involved_parties.contains(party_id))
            .collect();

        for ((i, message_index), (proof, party)) in state
            .clone()
            .failed_messages_indices
            .unwrap()
            .into_iter()
            .enumerate()
            .zip(proof_results.into_iter())
        {
            let shares = involved_shares
                .clone()
                .into_iter()
                .map(|(party_id, shares)| (party_id, shares[message_index].0.clone())).collect();
            let masked_shares = involved_shares
                .clone()
                .into_iter()
                .map(|(party_id, shares)| (party_id, shares[message_index].1.clone())).collect();

            let a: HashMap<PartyID, _> = state
                .proofs
                .clone()
                .unwrap()
                .into_iter()
                .map(|(party_id, proofs)| (party_id, proofs[i].clone()))
                .collect();

            // TODO: make sure the proof is valid
            identify_malicious_parties(
                party,
                shares, // TODO: Parse involved decryption shares from decryption shares
                masked_shares,
                state.tiresias_public_parameters.clone(),
                a,
                state.involved_parties.clone(),
            )
            .iter()
            .for_each(|party_id| {
                malicious_parties.insert(*party_id);
            });
        }
        println!("found by {}", state.party_id);
        Ok(SignRoundCompletion::MaliciousPartiesOutput(malicious_parties))
    }
}

pub(crate) enum SignRoundCompletion {
    SignatureOutput(Vec<Vec<u8>>),
    ProofsMessage(Vec<PartialDecryptionProof>, Vec<usize>, Vec<PartyID>),
    MaliciousPartiesOutput(HashSet<PartyID>),
    None,
}
