mod constants;
pub mod message_digest;

use base64::{engine::general_purpose::STANDARD, Engine as _};
use class_groups::dkg::Secp256k1Party;
use class_groups::SecretKeyShareSizedInteger;
use constants::{DECRYPTION_KEY_SHARE_PUBLIC_PARAMETERS, DYCRPTION_SHARES, NETWORK_DKG_OUTPUT};
use group::{secp256k1, PartyID};
use std::collections::HashMap;
use twopc_mpc::secp256k1::class_groups::{
    ProtocolPublicParameters, FUNDAMENTAL_DISCRIMINANT_LIMBS, NON_FUNDAMENTAL_DISCRIMINANT_LIMBS,
};

/// This module contains the secp256k1 constants for the class groups protocol.
/// NOTE: This is a temporary solution until the class groups encryption key DKG is complete.
/// Todo (#312): Remove this module and use the class groups DKG to generate the constants.
pub fn protocol_public_parameters() -> ProtocolPublicParameters {
    // Safe to unwrap as we're using a hardcoded constant.
    let encryption_scheme_pp = network_dkg_final_output()
        .default_encryption_scheme_public_parameters::<secp256k1::GroupElement>()
        .unwrap();

    ProtocolPublicParameters::new::<
        { secp256k1::SCALAR_LIMBS },
        { FUNDAMENTAL_DISCRIMINANT_LIMBS },
        { NON_FUNDAMENTAL_DISCRIMINANT_LIMBS },
        secp256k1::GroupElement,
    >(encryption_scheme_pp)
}

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

pub fn decryption_key_share_public_parameters() -> Vec<u8> {
    // Safe to unwrap as we're using a hardcoded constant.
    STANDARD
        .decode(DECRYPTION_KEY_SHARE_PUBLIC_PARAMETERS)
        .unwrap()
}

pub fn decryption_key_shares(party_id: PartyID) -> HashMap<PartyID, SecretKeyShareSizedInteger> {
    let bytes = STANDARD.decode(DYCRPTION_SHARES).unwrap();
    let shares: HashMap<PartyID, HashMap<PartyID, SecretKeyShareSizedInteger>> =
        bcs::from_bytes(&bytes).unwrap();
    shares.get(&party_id).unwrap().clone()
}
