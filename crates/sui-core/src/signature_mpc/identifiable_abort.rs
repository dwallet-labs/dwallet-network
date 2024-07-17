use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use dashmap::DashMap;

use mysten_metrics::spawn_monitored_task;
use signature_mpc::twopc_mpc_protocols;
use signature_mpc::twopc_mpc_protocols::{
    AdditivelyHomomorphicDecryptionKeyShare, DecryptionKeyShare, generate_proof,
    identify_message_malicious_parties, PaillierModulusSizedNumber, PartyID, SignaturePartialDecryptionProofParty,
    SignaturePartialDecryptionProofVerificationParty,
};
use signature_mpc::twopc_mpc_protocols::decrypt_signature::PartialDecryptionProof;
use sui_types::base_types::{EpochId, ObjectRef};
use sui_types::messages_signature_mpc::{
    SignatureMPCMessageProtocols, SignatureMPCMessageSummary, SignatureMPCSessionID, SignMessage,
};

use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::signature_mpc::sign_round::{SignRound, SignRoundCompletion};
use crate::signature_mpc::sign_state::SignState;
use crate::signature_mpc::submit_to_consensus::SubmitSignatureMPC;

/// Generate a list of proofs, one for every message in the messages batch that its decryption failed.
/// Each proof proves that the executing party id (state.party_id) while signing on that message.
pub fn generate_proofs(
    state: &SignState,
    failed_messages_indices: &Vec<usize>,
) -> Vec<(
    PartialDecryptionProof,
    SignaturePartialDecryptionProofVerificationParty,
)> {
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

/// Identify all the parties that behaved maliciously in this messages batch.
pub(crate) fn identify_batch_malicious_parties(
    state: &SignState,
) -> twopc_mpc_protocols::Result<SignRoundCompletion> {
    // Need to call [`generate_proofs`] to re-generate the SignaturePartialDecryptionProofVerificationParty objects,
    // that are necessary to call the [`identify_malicious_parties`] function.
    let failed_messages_parties = generate_proofs(&state, &state.failed_messages_indices.clone().unwrap());
    let mut malicious_parties = HashSet::new();
    let involved_shares = get_involved_shares(&state);
    for ((i, message_index), (_, party)) in state
        .clone()
        .failed_messages_indices
        .unwrap()
        .into_iter()
        .enumerate()
        .zip(failed_messages_parties.into_iter())
    {
        let (shares, masked_shares) = change_shares_type(&involved_shares, message_index);
        let involved_proofs = get_involved_proofs(&state, i);
        identify_message_malicious_parties(
            party,
            shares,
            masked_shares,
            state.tiresias_public_parameters.clone(),
            involved_proofs,
            state
                .involved_parties
                .as_deref()
                .unwrap_or(&Vec::new())
                .into(),
        )
        .iter()
        .for_each(|party_id| {
            malicious_parties.insert(*party_id);
        });
    }
    Ok(SignRoundCompletion::MaliciousPartiesOutput(
        malicious_parties,
    ))
}

/// Maps the decryption shares to the type expected by the identify_malicious_decrypters function from the 2pc-mpc repository.
fn change_shares_type(
    involved_shares: &HashMap<
        PartyID,
        Vec<(PaillierModulusSizedNumber, PaillierModulusSizedNumber)>,
    >,
    message_index: usize,
) -> (
    HashMap<PartyID, PaillierModulusSizedNumber>,
    HashMap<PartyID, PaillierModulusSizedNumber>,
) {
    involved_shares
        .iter()
        .map(|(party_id, shares)| {
            (
                (*party_id, shares[message_index].0.clone()),
                (*party_id, shares[message_index].1.clone()),
            )
        })
        .unzip()
}

fn get_involved_shares(
    state: &SignState,
) -> HashMap<PartyID, Vec<(PaillierModulusSizedNumber, PaillierModulusSizedNumber)>> {
    state
        .clone()
        .decryption_shares
        .into_iter()
        .filter(|(party_id, _)| {
            state
                .involved_parties
                .as_deref()
                .unwrap_or(&Vec::new())
                .contains(party_id)
        })
        .collect()
}

fn get_involved_proofs(state: &SignState, i: usize) -> HashMap<PartyID, PartialDecryptionProof> {
    state
        .proofs
        .clone()
        .unwrap()
        .into_iter()
        .map(|(party_id, proofs)| (party_id, proofs[i].clone()))
        .collect()
}

/// Generates proofs, one for evert message in the batch, that this party behaved honestly while
/// signing it. Then, sends the proofs to the other parties.
pub fn spawn_proof_generation(
    epoch: EpochId,
    epoch_store: Arc<AuthorityPerEpochStore>,
    party_id: PartyID,
    session_id: SignatureMPCSessionID,
    sign_session_states: Arc<DashMap<SignatureMPCSessionID, SignState>>,
    submit: Arc<dyn SubmitSignatureMPC>,
    failed_messages_indices: Vec<usize>,
    involved_parties: Vec<PartyID>,
    state: SignState,
) {
    spawn_monitored_task!(async move {
        if !state.clone().proofs.unwrap().contains_key(&party_id) {
            let proofs = generate_proofs(&state, &failed_messages_indices);
            let proofs: Vec<_> = proofs.iter().map(|(proof, _)| proof.clone()).collect();
            let _ = submit
                .sign_and_submit_message(
                    &SignatureMPCMessageSummary::new(
                        epoch,
                        SignatureMPCMessageProtocols::Sign(SignMessage::Proofs((
                            proofs,
                            failed_messages_indices,
                            involved_parties,
                        ))),
                        session_id,
                    ),
                    &epoch_store,
                )
                .await;
        }
    });
}
