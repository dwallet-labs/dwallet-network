// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use futures::StreamExt;
use rand::rngs::OsRng;
use signature_mpc::decrypt::{
    decrypt_signature_decentralized_party_sign, DecryptionError, DecryptionShare, PartialDecryptionProof,
};
use signature_mpc::twopc_mpc_protocols::{
    generate_proof, identify_malicious_parties, initiate_decentralized_party_sign, message_digest,
    AdditivelyHomomorphicDecryptionKeyShare, DKGDecentralizedPartyOutput, DecentralizedPartyPresign,
    DecryptionKeyShare, DecryptionPublicParameters, Hash, PaillierModulusSizedNumber, PartyID,
    ProofParty, ProtocolContext, PublicNonceEncryptedPartialSignatureAndProof, Result,
    SecretKeyShareSizedNumber, SignatureThresholdDecryptionParty,
};
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::mem;
use sui_types::base_types::EpochId;
use sui_types::messages_signature_mpc::SignatureMPCSessionID;

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
        for ((i, message_index), (proof, party)) in state
            .clone()
            .failed_messages_indices
            .unwrap()
            .into_iter()
            .enumerate()
            .zip(proof_results.into_iter())
        {
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
                _, // TODO: Parse involved decryption shares from decryption shares
                _,
                state.tiresias_public_parameters.clone(),
                a,
                state.involved_parties.clone(),
            )
            .iter()
            .for_each(|party_id| {
                malicious_parties.insert(*party_id);
            });
        }
        println!("malicious parties: {:?}", malicious_parties);
        Ok(SignRoundCompletion::MaliciousPartiesOutput(malicious_parties))
    }
}

pub(crate) enum SignRoundCompletion {
    SignatureOutput(Vec<Vec<u8>>),
    ProofsMessage(Vec<PartialDecryptionProof>, Vec<usize>, Vec<PartyID>),
    MaliciousPartiesOutput(HashSet<PartyID>),
    None,
}

#[derive(Clone)]
pub(crate) struct SignState {
    epoch: EpochId,
    pub party_id: PartyID,
    pub parties: HashSet<PartyID>,
    aggregator_party_id: PartyID,
    tiresias_public_parameters: DecryptionPublicParameters,
    tiresias_key_share_decryption_key_share: SecretKeyShareSizedNumber,
    messages: Option<Vec<Vec<u8>>>,
    public_nonce_encrypted_partial_signature_and_proofs:
        Option<Vec<PublicNonceEncryptedPartialSignatureAndProof<ProtocolContext>>>,
    presigns: Option<Vec<DecentralizedPartyPresign>>,
    decryption_shares: HashMap<PartyID, Vec<(PaillierModulusSizedNumber, PaillierModulusSizedNumber)>>,
    pub proofs: Option<HashMap<PartyID, Vec<(PartialDecryptionProof)>>>,
    pub failed_messages_indices: Option<Vec<usize>>,
    pub involved_parties: Vec<PartyID>,
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
            involved_parties: Vec::new(),
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
        party_id: PartyID,
        message: Vec<(PaillierModulusSizedNumber, PaillierModulusSizedNumber)>,
    ) -> Result<()> {
        let _ = self.decryption_shares.insert(party_id, message);
        Ok(())
    }

    pub(crate) fn insert_proofs(&mut self, party_id: PartyID, new_proofs: Vec<PartialDecryptionProof>) {
        if let Some(proofs_map) = &mut self.proofs {
            proofs_map.insert(party_id, new_proofs);
        } else {
            let mut proofs_map = HashMap::from([(party_id, new_proofs)]);
            self.proofs = Some(proofs_map);
        }
    }

    pub(crate) fn ready_for_complete_first_round(&self, round: &SignRound) -> bool {
        match round {
            SignRound::FirstRound { .. }
                if self.decryption_shares.len() == self.parties.len()
                    && self.party_id == self.aggregator_party_id =>
            {
                true
            }
            _ => false,
        }
    }

    pub(crate) fn receieved_all_decryption_shares(&self) -> bool {
        return self.decryption_shares.len() == self.parties.len()
            && self.party_id == self.aggregator_party_id;
    }
}
