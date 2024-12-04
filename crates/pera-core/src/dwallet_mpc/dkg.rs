//! This module provides a wrapper around the DKG protocol from the 2PC-MPC library.
//!
//! It integrates both DKG parties (each representing a round in the DKG protocol) and
use crate::dwallet_mpc::mpc_party::AsyncProtocol;
use mpc::Party;
use twopc_mpc::dkg::Protocol;

pub(super) type DKGFirstParty = <AsyncProtocol as Protocol>::EncryptionOfSecretKeyShareRoundParty;
pub(super) type DKGSecondParty = <AsyncProtocol as Protocol>::ProofVerificationRoundParty;

/// A trait for generating the public input for the initial round of the DKG protocol.
///
/// This trait is implemented to resolve compiler type ambiguities that arise in the 2PC-MPC library
/// when accessing `mpc::Party::PublicInput`.
pub(super) trait DKGFirstPartyPublicInputGenerator: Party {
    /// Generates the public input required for the first round of the DKG protocol.
    fn generate_public_input() -> Vec<u8>;
}

/// A trait for generating the public input for the last round of the DKG protocol.
///
/// This trait is implemented to resolve compiler type ambiguities that arise in the 2PC-MPC library
/// when accessing `mpc::Party::PublicInput`.
pub(super) trait DKGSecondPartyPublicInputGenerator: Party {
    /// Generates the public input required for the second round of the DKG protocol.
    fn generate_public_input(
        first_round_output: Vec<u8>,
        centralized_party_public_key_share: Vec<u8>,
    ) -> Vec<u8>;
}

impl DKGFirstPartyPublicInputGenerator for DKGFirstParty {
    fn generate_public_input() -> Vec<u8> {
        let input: Self::PublicInput = class_groups_constants::protocol_public_parameters();
        bcs::to_bytes(&input).unwrap()
    }
}

impl DKGSecondPartyPublicInputGenerator for DKGSecondParty {
    fn generate_public_input(
        first_round_output_buf: Vec<u8>,
        centralized_party_public_key_share_buf: Vec<u8>,
    ) -> Vec<u8> {
        let first_round_output: <DKGFirstParty as Party>::PublicOutput =
            bcs::from_bytes(&first_round_output_buf).unwrap();
        let centralized_party_public_key_share: <AsyncProtocol as Protocol>::PublicKeyShareAndProof =
            bcs::from_bytes(&centralized_party_public_key_share_buf).unwrap();

        let input: Self::PublicInput = (
            class_groups_constants::protocol_public_parameters(),
            first_round_output,
            centralized_party_public_key_share,
        )
            .into();
        bcs::to_bytes(&input).unwrap()
    }
}
