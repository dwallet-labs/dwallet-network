// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use rand::rngs::OsRng;
use signature_mpc::twopc_mpc_protocols::{
    decrypt_signature_decentralized_party_sign, initiate_decentralized_party_sign, message_digest,
    AdditivelyHomomorphicDecryptionKeyShare, AdjustedLagrangeCoefficientSizedNumber, Commitment,
    DKGDecentralizedPartyOutput, DecentralizedPartyPresign, DecryptionKeyShare,
    DecryptionPublicParameters, GroupElement, Hash, PaillierModulusSizedNumber, PartyID,
    ProtocolContext, PublicNonceEncryptedPartialSignatureAndProof, Result,
    SecretKeyShareSizedNumber, SignatureThresholdDecryptionParty, Value,
};
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::mem;
use sui_types::base_types::{EpochId, ObjectRef};
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

        Ok((
            SignRound::FirstRound {
                signature_threshold_decryption_round_parties,
            },
            decryption_shares,
        ))
    }

    pub(crate) fn complete_round(&mut self, state: SignState) -> Result<SignRoundCompletion> {
        let round = mem::take(self);
        match round {
            SignRound::FirstRound {
                signature_threshold_decryption_round_parties,
            } => {
                let signatures_s = decrypt_signature_decentralized_party_sign(
                    state.messages.unwrap(),
                    state.tiresias_public_parameters.clone(),
                    state.decryption_shares.clone(),
                    state
                        .public_nonce_encrypted_partial_signature_and_proofs
                        .clone()
                        .unwrap(),
                    signature_threshold_decryption_round_parties,
                )?;

                Ok(SignRoundCompletion::Output(signatures_s))
            }
            _ => Ok(SignRoundCompletion::None),
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
    public_nonce_encrypted_partial_signature_and_proofs:
        Option<Vec<PublicNonceEncryptedPartialSignatureAndProof<ProtocolContext>>>,

    decryption_shares:
        HashMap<PartyID, Vec<(PaillierModulusSizedNumber, PaillierModulusSizedNumber)>>,
}

impl SignState {
    pub(crate) fn new(
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
        }
    }

    pub(crate) fn set(
        &mut self,
        messages: Vec<Vec<u8>>,
        public_nonce_encrypted_partial_signature_and_proofs: Vec<
            PublicNonceEncryptedPartialSignatureAndProof<ProtocolContext>,
        >,
    ) {
        self.messages = Some(messages);
        self.public_nonce_encrypted_partial_signature_and_proofs =
            Some(public_nonce_encrypted_partial_signature_and_proofs);
    }

    pub(crate) fn insert_first_round(
        &mut self,
        party_id: PartyID,
        message: Vec<(PaillierModulusSizedNumber, PaillierModulusSizedNumber)>,
    ) -> Result<()> {
        let _ = self.decryption_shares.insert(party_id, message);
        Ok(())
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
}
