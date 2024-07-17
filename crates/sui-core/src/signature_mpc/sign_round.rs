// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::collections::HashSet;
use std::mem;

use futures::StreamExt;
use rand::rngs::OsRng;

use signature_mpc::twopc_mpc_protocols::decrypt_signature::{
    decrypt_signature_decentralized_party_sign, PartialDecryptionProof,
};
use signature_mpc::twopc_mpc_protocols::{
    AdditivelyHomomorphicDecryptionKeyShare, DecentralizedPartyPresign, DecryptionPublicParameters,
    DKGDecentralizedPartyOutput, Hash
    , initiate_decentralized_party_sign, message_digest, PaillierModulusSizedNumber, PartyID
    , ProtocolContext, PublicNonceEncryptedPartialSignatureAndProof, Result,
    SecretKeyShareSizedNumber, SignatureThresholdDecryptionParty,
};

use crate::signature_mpc::identifiable_abort;
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
            decryption_shares.clone(),
        ))
    }

    /// Tries to decrypt the signatures and return them.
    /// In case one or more of the signatures is invalid, it will generate proofs, one for every message in the batch, that it behaved honestly, and will return them and
    /// more information needed to launch the identifiable abort flow.
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
                        Ok(SignRoundCompletion::SignatureOutput(signatures))
                    }
                    Err(decryption_error) => {
                        let proofs_tuples =
                            identifiable_abort::generate_proofs(&state, &decryption_error.failed_messages_indices);
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
}

pub(crate) enum SignRoundCompletion {
    SignatureOutput(Vec<Vec<u8>>),
    ProofsMessage(Vec<PartialDecryptionProof>, Vec<usize>, Vec<PartyID>),
    MaliciousPartiesOutput(HashSet<PartyID>),
    None,
}

