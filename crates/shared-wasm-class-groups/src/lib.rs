mod constants;
pub mod message_digest;

use base64::{engine::general_purpose::STANDARD, Engine as _};
use class_groups::dkg::Secp256k1Party;
use class_groups::SecretKeyShareSizedInteger;
use constants::{DYCRPTION_SHARES, NETWORK_DKG_OUTPUT};
use group::PartyID;
use std::collections::HashMap;

pub fn network_dkg_final_output() -> Box<<Secp256k1Party as mpc::Party>::PublicOutput> {
    // Safe to unwrap as we're using a hardcoded constant.
    let protocol_public_parameters = STANDARD.decode(NETWORK_DKG_OUTPUT).unwrap();
    Box::new(
        bcs::from_bytes::<<Secp256k1Party as mpc::Party>::PublicOutput>(
            &protocol_public_parameters,
        )
        .unwrap(),
    )
}

pub fn decryption_key_shares(party_id: PartyID) -> HashMap<PartyID, SecretKeyShareSizedInteger> {
    let bytes = STANDARD.decode(DYCRPTION_SHARES).unwrap();
    let shares: HashMap<PartyID, HashMap<PartyID, SecretKeyShareSizedInteger>> =
        bcs::from_bytes(&bytes).unwrap();
    shares.get(&party_id).unwrap().clone()
}
