use crate::signature_mpc::dkg::{AuxiliaryFirst, AuxiliarySecond, DKGFirstParty, DKGSecondParty};

pub fn get_dkg_first_round_auxiliary_input(session_id: Vec<u8>) -> Vec<u8> {
    bcs::to_bytes(&DKGFirstParty::first_auxiliary_input(session_id)).unwrap()
}

pub fn get_dkg_second_round_auxiliary_input(
    first_round_output: Vec<u8>,
    centralized_party_public_key_share: Vec<u8>,
    session_id: Vec<u8>,
) -> Vec<u8> {
    // let first_round_auxiliary_input = get_dkg_first_round_auxiliary_input(session_id.clone());
    bcs::to_bytes(&DKGSecondParty::first_auxiliary_input(
        bcs::from_bytes(&first_round_output).unwrap(), // remove unwrap
        bcs::from_bytes(&centralized_party_public_key_share).unwrap(), // remove unwrap
        session_id,
    ))
    .unwrap()
}