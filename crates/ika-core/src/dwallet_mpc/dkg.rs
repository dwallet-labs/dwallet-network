//! This module provides a wrapper around the DKG protocol from the 2PC-MPC library.
//!
//! It integrates both DKG parties (each representing a round in the DKG protocol).
use ika_types::messages_dwallet_mpc::AsyncProtocol;
use dwallet_mpc_types::dwallet_mpc::{
    MPCPublicInput, SerializedWrappedMPCPublicOutput, VersionedCentralizedDKGPublicOutput,
    VersionedPublicKeyShareAndProof,
};
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use mpc::Party;
use twopc_mpc::dkg::Protocol;

/// This struct represents the initial round of the DKG protocol.
pub type DKGFirstParty = <AsyncProtocol as Protocol>::EncryptionOfSecretKeyShareRoundParty;
pub(super) type DWalletImportedKeyVerificationParty =
    <AsyncProtocol as Protocol>::TrustedDealerDKGDecentralizedParty;
/// This struct represents the final round of the DKG protocol.
pub(super) type DKGSecondParty = <AsyncProtocol as Protocol>::ProofVerificationRoundParty;

/// A trait for generating the public input for the initial round of the DKG protocol.
///
/// This trait is implemented to resolve compiler type ambiguities that arise in the 2PC-MPC library
/// when accessing [`Party::PublicInput`].
/// It defines the parameters and logic
/// necessary to initiate the first round of the DKG protocol,
/// preparing the party with the essential session information and other contextual data.
pub(super) trait DKGFirstPartyPublicInputGenerator: Party {
    /// Generates the public input required for the first round of the DKG protocol.
    fn generate_public_input(
        protocol_public_parameters: twopc_mpc::secp256k1::class_groups::ProtocolPublicParameters,
    ) -> DwalletMPCResult<<DKGFirstParty as mpc::Party>::PublicInput>;
}

/// A trait for generating the public input for the last round of the DKG protocol.
///
/// This trait is implemented to resolve compiler type ambiguities that arise in the 2PC-MPC library
/// when accessing [`Party::PublicInput`].
/// It defines the parameters and logic
/// necessary to initiate the second round of the DKG protocol,
/// preparing the party with the essential session information and other contextual data.
pub(super) trait DKGSecondPartyPublicInputGenerator: Party {
    /// Generates the public input required for the second round of the DKG protocol.
    fn generate_public_input(
        protocol_public_parameters: twopc_mpc::secp256k1::class_groups::ProtocolPublicParameters,
        first_round_output: SerializedWrappedMPCPublicOutput,
        centralized_party_public_key_share: SerializedWrappedMPCPublicOutput,
    ) -> DwalletMPCResult<<DKGSecondParty as mpc::Party>::PublicInput>;
}

impl DKGFirstPartyPublicInputGenerator for DKGFirstParty {
    fn generate_public_input(
        protocol_public_parameters: twopc_mpc::secp256k1::class_groups::ProtocolPublicParameters,
    ) -> DwalletMPCResult<<DKGFirstParty as mpc::Party>::PublicInput> {
        let input: Self::PublicInput = protocol_public_parameters.into();
        Ok(input)
    }
}

impl DKGSecondPartyPublicInputGenerator for DKGSecondParty {
    fn generate_public_input(
        protocol_public_parameters: twopc_mpc::secp256k1::class_groups::ProtocolPublicParameters,
        first_round_output_buf: SerializedWrappedMPCPublicOutput,
        centralized_party_public_key_share_buf: SerializedWrappedMPCPublicOutput,
    ) -> DwalletMPCResult<<DKGSecondParty as mpc::Party>::PublicInput> {
        let first_round_output_buf: VersionedCentralizedDKGPublicOutput =
            bcs::from_bytes(&first_round_output_buf).map_err(DwalletMPCError::BcsError)?;
        let centralized_party_public_key_share: VersionedPublicKeyShareAndProof =
            bcs::from_bytes(&centralized_party_public_key_share_buf)
                .map_err(DwalletMPCError::BcsError)?;
        match first_round_output_buf {
            VersionedCentralizedDKGPublicOutput::V1(first_round_output) => {
                let first_round_output: <DKGFirstParty as Party>::PublicOutput =
                    bcs::from_bytes(&first_round_output).map_err(DwalletMPCError::BcsError)?;
                let centralized_party_public_key_share = match centralized_party_public_key_share {
                    VersionedPublicKeyShareAndProof::V1(centralized_party_public_key_share) => {
                        bcs::from_bytes(&centralized_party_public_key_share)
                            .map_err(DwalletMPCError::BcsError)?
                    }
                };

                let input: Self::PublicInput = (
                    protocol_public_parameters,
                    first_round_output,
                    centralized_party_public_key_share,
                )
                    .into();
                Ok(input)
            }
        }
    }
}
