// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::collections::{HashMap, HashSet};
use rand::rngs::OsRng;
use sui_types::base_types::{EpochId, ObjectRef};
use sui_types::messages_signature_mpc::{AdditivelyHomomorphicDecryptionKeyShare, PartyID, SignatureMPCSessionID, TwopcMPCResult, DecryptionPublicParameters, DKGSignatureMPCDecentralizedOutput, PresignSignatureMPCDecentralizedPartyPresign, initiate_decentralized_party_sign, SecretKeyShareSizedNumber, message_digest, SignSignatureMPCCentralizedPublicNonceEncryptedPartialSignatureAndProof, DecryptionKeyShare, AdjustedLagrangeCoefficientSizedNumber, decrypt_signature_decentralized_party_sign, PaillierModulusSizedNumber, DKGSignatureMPCCentralizedCommitment, Secp256k1GroupElementValue, SignSignatureMPCSignatureThresholdDecryptionRoundParty};
use std::convert::TryInto;
use std::mem;

#[derive(Default)]
pub(crate) enum SignRound {
    FirstRound {
        signature_threshold_decryption_round_parties: Vec<SignSignatureMPCSignatureThresholdDecryptionRoundParty>
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
        dkg_output: DKGSignatureMPCDecentralizedOutput,
        public_nonce_encrypted_partial_signature_and_proofs: Vec<SignSignatureMPCCentralizedPublicNonceEncryptedPartialSignatureAndProof>,
        presigns: Vec<PresignSignatureMPCDecentralizedPartyPresign>,
    ) -> TwopcMPCResult<(Self, Vec<(PaillierModulusSizedNumber, PaillierModulusSizedNumber)>)> {
        let sign_mpc_party_per_message = initiate_decentralized_party_sign(
            tiresias_key_share_decryption_key_share,
            tiresias_public_parameters.clone(),
            epoch,
            party_id,
            parties.clone(),
            session_id,
            dkg_output,
            presigns.clone(),
        )?;

        let (decryption_shares, signature_threshold_decryption_round_parties): (Vec<_>, Vec<_>) = messages.iter().zip(sign_mpc_party_per_message.into_iter()).zip(public_nonce_encrypted_partial_signature_and_proofs.clone().into_iter()).map(|((m, party), public_nonce_encrypted_partial_signature_and_proof)| {
            let m = message_digest(m);
            party
                .partially_decrypt_encrypted_signature_parts_prehash(
                    m,
                    public_nonce_encrypted_partial_signature_and_proof,
                    &mut OsRng,
                )
        }).collect::<TwopcMPCResult<Vec<((PaillierModulusSizedNumber, PaillierModulusSizedNumber), SignSignatureMPCSignatureThresholdDecryptionRoundParty)>>>()?.into_iter().unzip();

        Ok((
            SignRound::FirstRound {
                signature_threshold_decryption_round_parties
            },
            decryption_shares
        ))
    }

    pub(crate) fn complete_round(
        &mut self,
        state: SignState
    ) -> TwopcMPCResult<SignRoundCompletion> {
        let round = mem::take(self);
        match round {
            SignRound::FirstRound { signature_threshold_decryption_round_parties } => {
                let signatures_s = decrypt_signature_decentralized_party_sign(state.public_key.unwrap(), state.messages.unwrap(), state.tiresias_public_parameters.clone(), state.decryption_shares.clone(), state.public_nonce_encrypted_partial_signature_and_proofs.clone().unwrap(), signature_threshold_decryption_round_parties)?;

                Ok(SignRoundCompletion::Output(signatures_s))            }
            _ => Ok(SignRoundCompletion::None)
        }


    }
}


pub(crate) enum SignRoundCompletion {
    Output(Vec<Vec<u8>>),
    None,
}

#[derive(Clone)]
pub(crate) struct SignState {
    epoch: EpochId,
    party_id: PartyID,
    parties: HashSet<PartyID>,
    aggregator_party_id: PartyID,
    tiresias_public_parameters: DecryptionPublicParameters,

    messages: Option<Vec<Vec<u8>>>,
    public_nonce_encrypted_partial_signature_and_proofs: Option<Vec<SignSignatureMPCCentralizedPublicNonceEncryptedPartialSignatureAndProof>>,
    public_key: Option<Secp256k1GroupElementValue>,

    decryption_shares: HashMap<PartyID, Vec<(PaillierModulusSizedNumber, PaillierModulusSizedNumber)>>,
}

impl SignState {
    pub(crate) fn new(
        tiresias_public_parameters: DecryptionPublicParameters,
        epoch: EpochId,
        party_id: PartyID,
        parties: HashSet<PartyID>,
        session_id: SignatureMPCSessionID,
    ) -> Self {
        let aggregator_party_id = ((u64::from_be_bytes((&session_id.0[0..8]).try_into().unwrap()) % parties.len() as u64) + 1) as PartyID;

        Self {
            epoch,
            party_id,
            parties,
            aggregator_party_id,
            tiresias_public_parameters,
            messages: None,
            public_nonce_encrypted_partial_signature_and_proofs: None,
            public_key: None,
            decryption_shares: HashMap::new(),
        }
    }

    pub(crate) fn set(
        &mut self,
        messages: Vec<Vec<u8>>,
        public_nonce_encrypted_partial_signature_and_proofs: Vec<SignSignatureMPCCentralizedPublicNonceEncryptedPartialSignatureAndProof>,
        public_key: Secp256k1GroupElementValue
    ) {
        self.messages = Some(messages);
        self.public_nonce_encrypted_partial_signature_and_proofs = Some(public_nonce_encrypted_partial_signature_and_proofs);
        self.public_key = Some(public_key);
    }

    pub(crate) fn insert_first_round(
        &mut self,
        party_id: PartyID,
        message: Vec<(PaillierModulusSizedNumber, PaillierModulusSizedNumber)>,
    ) -> TwopcMPCResult<()> {
        let _ = self
            .decryption_shares
            .insert(party_id, message);
        Ok(())
    }

    pub(crate) fn ready_for_complete_first_round(&self, round: &SignRound) -> bool {
        match round {
            SignRound::FirstRound { .. } if self.decryption_shares.len() == self.parties.len() && self.party_id == self.aggregator_party_id => true,
            _ => false
        }
    }
}
