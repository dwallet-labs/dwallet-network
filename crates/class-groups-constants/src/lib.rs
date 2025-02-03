mod constants;

use base64::{engine::general_purpose::STANDARD, Engine as _};
use class_groups::dkg::Secp256k1Party;
use class_groups::{
    SecretKeyShareSizedNumber, SECP256K1_FUNDAMENTAL_DISCRIMINANT_LIMBS, SECP256K1_SCALAR_LIMBS,
};
use constants::{DECRYPTION_KEY_SHARE_PUBLIC_PARAMETERS, DYCRPTION_SHARES, NETWORK_DKG_OUTPUT};
use group::{secp256k1, PartyID};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use twopc_mpc::secp256k1::class_groups::{
    ProtocolPublicParameters, FUNDAMENTAL_DISCRIMINANT_LIMBS, NON_FUNDAMENTAL_DISCRIMINANT_LIMBS,
};

/// Contains the public keys of the DWallet.
///
/// Being used to sign on with the Sui signature key when encrypting this DWallet to another user. The receiving user
/// can later verify the signature is valid and now this DWallet decentralized output
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, Hash)]
pub struct DWalletPublicKeys {
    pub centralized_public_share: Vec<u8>,
    pub decentralized_public_share: Vec<u8>,
    pub public_key: Vec<u8>,
}

type AsyncProtocol = twopc_mpc::secp256k1::class_groups::AsyncProtocol;

pub type DKGDecentralizedOutput =
    <AsyncProtocol as twopc_mpc::dkg::Protocol>::DecentralizedPartyDKGOutput;

/// This module contains the secp256k1 constants for the class groups protocol.
/// NOTE: This is a temporary solution until the class groups encryption key DKG is complete.
/// Todo (#312): Remove this module and use the class groups DKG to generate the constants.
pub fn protocol_public_parameters() -> ProtocolPublicParameters {
    // Safe to unwrap as we're using a hardcoded constant.
    let encryption_scheme_pp = network_dkg_final_output().default_encryption_scheme_public_parameters::<SECP256K1_SCALAR_LIMBS, SECP256K1_FUNDAMENTAL_DISCRIMINANT_LIMBS, secp256k1::GroupElement>().unwrap();
    ProtocolPublicParameters::new::<
        { secp256k1::SCALAR_LIMBS },
        { FUNDAMENTAL_DISCRIMINANT_LIMBS },
        { NON_FUNDAMENTAL_DISCRIMINANT_LIMBS },
        secp256k1::GroupElement,
    >(encryption_scheme_pp)
}

pub fn network_dkg_final_output() -> <Secp256k1Party as mpc::Party>::PublicOutput {
    // Safe to unwrap as we're using a hardcoded constant.
    let protocol_public_parameters = STANDARD.decode(&NETWORK_DKG_OUTPUT).unwrap();
    bcs::from_bytes(&protocol_public_parameters).unwrap()
}

pub fn decryption_key_share_public_parameters() -> Vec<u8> {
    // Safe to unwrap as we're using a hardcoded constant.
    STANDARD
        .decode(DECRYPTION_KEY_SHARE_PUBLIC_PARAMETERS)
        .unwrap()
}

pub fn decryption_key_share(party_id: PartyID) -> HashMap<PartyID, SecretKeyShareSizedNumber> {
    let bytes = STANDARD.decode(DYCRPTION_SHARES).unwrap();
    let shares: HashMap<PartyID, HashMap<PartyID, SecretKeyShareSizedNumber>> =
        bcs::from_bytes(&bytes).unwrap();
    shares.get(&party_id).unwrap().clone()
}

/// Derived [`DWalletPublicKeys`] from the given [`DKGDecentralizedOutput`].
// Can't use the TryFrom trait as it leads to conflicting implementations
pub fn public_keys_from_dkg_output(
    value: DKGDecentralizedOutput,
) -> anyhow::Result<DWalletPublicKeys> {
    Ok(DWalletPublicKeys {
        centralized_public_share: bcs::to_bytes(&value.centralized_party_public_key_share)?,
        decentralized_public_share: bcs::to_bytes(&value.public_key_share)?,
        public_key: bcs::to_bytes(&value.public_key)?,
    })
}
