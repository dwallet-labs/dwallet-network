use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use dashmap::DashMap;
use twopc_mpc::secp256k1::paillier::bulletproofs::PartialDecryptionProof;

use mysten_metrics::spawn_monitored_task;
use signature_mpc::twopc_mpc_protocols::{
    generate_proof, identify_message_malicious_parties, AdditivelyHomomorphicDecryptionKeyShare,
    DecryptionKeyShare, PaillierModulusSizedNumber, PartyID,
    SignaturePartialDecryptionProofVerificationParty,
};
use sui_types::base_types::EpochId;
use sui_types::messages_signature_mpc::{
    SignMessage, SignatureMPCMessageProtocols, SignatureMPCMessageSummary, SignatureMPCSessionID,
};

use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::signature_mpc::sign::SignState;
use crate::signature_mpc::submit_to_consensus::SubmitSignatureMPC;
use twopc_mpc::{Error, Result};

/// Generate a list of proofs, one for every message in the batch.
/// Each proof proves that the executing party ID
/// (state.party_id) behaved honestly while signing the message.
pub fn generate_proofs(
    state: &SignState,
) -> Result<
    Vec<(
        PartialDecryptionProof,
        SignaturePartialDecryptionProofVerificationParty,
    )>,
> {
    let decryption_key_share = DecryptionKeyShare::new(
        state.party_id,
        state.tiresias_key_share_decryption_key_share,
        &state.tiresias_public_parameters,
    )?;
    let presigns = state
        .presigns
        .clone()
        .ok_or(twopc_mpc::Error::InvalidParameters)?;
    let public_nonce_encrypted_partial_signature_and_proofs = state
        .public_nonce_encrypted_partial_signature_and_proofs
        .clone()
        .ok_or(twopc_mpc::Error::InvalidParameters)?;
    state
        .messages
        .iter()
        .enumerate()
        .map(|(index, _)| {
            let presign = presigns
                .get(index)
                .ok_or(twopc_mpc::Error::InvalidParameters)?;
            let public_nonce_encrypted_partial_signature_and_proof =
                public_nonce_encrypted_partial_signature_and_proofs
                    .get(index)
                    .ok_or(twopc_mpc::Error::InvalidParameters)?;
            generate_proof(
                &state.tiresias_public_parameters,
                &decryption_key_share,
                state.party_id,
                presign,
                &state
                    .tiresias_public_parameters
                    .encryption_scheme_public_parameters,
                public_nonce_encrypted_partial_signature_and_proof,
            )
        })
        .collect::<Result<Vec<_>>>()
}

/// Identify all the parties that behaved maliciously in this messages batch.
pub(crate) fn identify_batch_malicious_parties(state: &SignState) -> Result<HashSet<PartyID>> {
    // Need to call [`generate_proofs`] to
    // re-generate the [`SignaturePartialDecryptionProofVerificationParty`] objects,
    // that are necessary to call the [`identify_message_malicious_parties`] function.
    let parties_with_proofs = generate_proofs(state)?;
    let involved_shares = get_involved_shares(state);

    let malicious_parties = parties_with_proofs
        .into_iter()
        .enumerate()
        .map(|(i, (_, party))| {
            let (shares, masked_shares) =
                normalize_shares_for_identify_malicious_decrypters(&involved_shares, i)?;
            let party_to_msg_proof = state
                .proofs
                .iter()
                .map(|(party_id, proofs)| (*party_id, proofs[i].clone()))
                .collect();
            identify_message_malicious_parties(
                party,
                shares,
                masked_shares,
                &state.tiresias_public_parameters,
                party_to_msg_proof,
                &state.involved_parties,
            )
        })
        .collect::<Result<Vec<Vec<_>>>>()?
        .into_iter()
        .flatten()
        .collect::<HashSet<_>>();
    Ok(malicious_parties)
}

/// Maps the decryption shares to the type
/// expected by the identify_malicious_decrypters function from the `2PC-MPC` repository.
fn normalize_shares_for_identify_malicious_decrypters(
    involved_shares: &HashMap<
        PartyID,
        Vec<(PaillierModulusSizedNumber, PaillierModulusSizedNumber)>,
    >,
    message_index: usize,
) -> Result<(
    HashMap<PartyID, PaillierModulusSizedNumber>,
    HashMap<PartyID, PaillierModulusSizedNumber>,
)> {
    let (partial_signature_map, masked_nonce_map): (HashMap<_, _>, HashMap<_, _>) = involved_shares
        .iter()
        .map(|(party_id, shares)| {
            shares
                .get(message_index)
                .map(
                    |(partial_signature_decryption_share, masked_nonce_decryption_share)| {
                        (
                            (*party_id, *partial_signature_decryption_share),
                            (*party_id, *masked_nonce_decryption_share),
                        )
                    },
                )
                .ok_or(Error::InvalidParameters)
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .unzip();

    Ok((partial_signature_map, masked_nonce_map))
}

fn get_involved_shares(
    state: &SignState,
) -> HashMap<PartyID, Vec<(PaillierModulusSizedNumber, PaillierModulusSizedNumber)>> {
    state
        .decryption_shares
        .iter()
        .filter_map(|(party_id, data)| {
            if state.involved_parties.contains(party_id) {
                return Some((*party_id, data.clone()));
            }
            None
        })
        .collect()
}

/// Generates a proof that this party behaved honestly while signing a message.
/// One for every message in the batch,
/// Then, send the proofs to the other parties.
pub fn spawn_proof_generation(
    epoch: EpochId,
    epoch_store: Arc<AuthorityPerEpochStore>,
    party_id: PartyID,
    session_id: SignatureMPCSessionID,
    _sign_session_states: Arc<DashMap<SignatureMPCSessionID, SignState>>,
    submit: Arc<dyn SubmitSignatureMPC>,
    involved_parties: Vec<PartyID>,
    state: SignState,
) {
    spawn_monitored_task!(async move {
        if state.proofs.contains_key(&party_id) || !involved_parties.contains(&party_id) {
            return;
        }
        let proofs = generate_proofs(&state);
        if let Ok(proofs) = proofs {
            let proofs: Vec<_> = proofs.iter().map(|(proof, _)| proof.clone()).collect();
            let _ = submit
                .sign_and_submit_message(
                    &SignatureMPCMessageSummary::new(
                        epoch,
                        SignatureMPCMessageProtocols::Sign(SignMessage::IAProofs(proofs)),
                        session_id,
                    ),
                    &epoch_store,
                )
                .await;
        }
    });
}
