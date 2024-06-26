use signature_mpc::decrypt::PartialDecryptionProof;
use signature_mpc::twopc_mpc_protocols;
use signature_mpc::twopc_mpc_protocols::{AdditivelyHomomorphicDecryptionKeyShare, DecryptionKeyShare, generate_proof, identify_malicious_parties, PaillierModulusSizedNumber, PartyID, ProofParty};
use std::collections::{HashMap, HashSet};
use crate::signature_mpc::sign_round::SignRoundCompletion;
use crate::signature_mpc::sign_state::SignState;

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

pub(crate) fn identify_malicious(state: &SignState) -> twopc_mpc_protocols::Result<SignRoundCompletion> {
    let proof_results = generate_proofs(&state, &state.failed_messages_indices.clone().unwrap());
    let mut malicious_parties = HashSet::new();
    let involved_shares = get_involved_shares(&state);
    for ((i, message_index), (_, party)) in state
        .clone()
        .failed_messages_indices
        .unwrap()
        .into_iter()
        .enumerate()
        .zip(proof_results.into_iter())
    {
        let shares = extract_shares(&involved_shares, message_index, 0);
        let masked_shares = extract_shares(&involved_shares, message_index, 1);
        let involved_proofs = get_involved_proofs(&state, i);
        identify_malicious_parties(
            party,
            shares,
            masked_shares,
            state.tiresias_public_parameters.clone(),
            involved_proofs,
            state.involved_parties.clone(),
        )
            .iter()
            .for_each(|party_id| {
                malicious_parties.insert(*party_id);
            });
    }
    Ok(SignRoundCompletion::MaliciousPartiesOutput(malicious_parties))
}

fn extract_shares(
    involved_shares: &HashMap<PartyID, Vec<(PaillierModulusSizedNumber, PaillierModulusSizedNumber)>>,
    message_index: usize,
    element_index: usize,
) -> HashMap<PartyID, PaillierModulusSizedNumber> {
    involved_shares
        .iter()
        .map(|(party_id, shares)| {
            let element = match element_index {
                0 => shares[message_index].0.clone(),
                1 => shares[message_index].1.clone(),
                _ => panic!("Invalid element index"),
            };
            (*party_id, element)
        })
        .collect()
}

fn get_involved_shares(state: &SignState) -> HashMap<PartyID, Vec<(PaillierModulusSizedNumber, PaillierModulusSizedNumber)>> {
    state
        .clone()
        .decryption_shares
        .into_iter()
        .filter(|(party_id, _)| state.involved_parties.contains(party_id))
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