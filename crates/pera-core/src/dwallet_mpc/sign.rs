use crate::dwallet_mpc::mpc_party::AsyncProtocol;
use pera_types::error::PeraResult;
use std::collections::HashMap;
use twopc_mpc::dkg::Protocol;

pub(super) type SignFirstParty =
    <AsyncProtocol as twopc_mpc::sign::Protocol>::SignDecentralizedParty;
pub(super) type SignPublicInput =
    <AsyncProtocol as twopc_mpc::sign::Protocol>::SignDecentralizedPartyPublicInput;

/// A trait for generating the public input for decentralized Sign round in the MPC protocol.
///
/// This trait is implemented to resolve compiler type ambiguities that arise in the 2PC-MPC library
/// when accessing `mpc::Party::PublicInput`.
pub(super) trait SignPartyPublicInputGenerator: mpc::Party {
    fn generate_public_input(
        dkg_output: Vec<u8>,
        hashed_message: Vec<u8>,
        presign_first_round_output: Vec<u8>,
        presign_second_round_output: Vec<u8>,
        centralized_signed_message: Vec<u8>,
        decryption_key_share_public_parameters: <AsyncProtocol as twopc_mpc::sign::Protocol>::DecryptionKeySharePublicParameters,
    ) -> PeraResult<Vec<u8>>;
}

impl SignPartyPublicInputGenerator for SignFirstParty {
    fn generate_public_input(
        dkg_output: Vec<u8>,
        hashed_message: Vec<u8>,
        presign_first_round_output: Vec<u8>,
        presign_second_round_output: Vec<u8>,
        centralized_signed_message: Vec<u8>,
        decryption_key_share_public_parameters: <AsyncProtocol as twopc_mpc::sign::Protocol>::DecryptionKeySharePublicParameters,
    ) -> PeraResult<Vec<u8>> {
        let presign_first_round_output: <AsyncProtocol as twopc_mpc::presign::Protocol>::EncryptionOfMaskAndMaskedNonceShare = bcs::from_bytes(&presign_first_round_output)?;
        let presign_second_round_output: (<AsyncProtocol as twopc_mpc::presign::Protocol>::NoncePublicShareAndEncryptionOfMaskedNonceSharePart, <AsyncProtocol as twopc_mpc::presign::Protocol>::NoncePublicShareAndEncryptionOfMaskedNonceSharePart) = bcs::from_bytes(&presign_second_round_output)?;
        let presign: <AsyncProtocol as twopc_mpc::presign::Protocol>::Presign =
            (presign_first_round_output, presign_second_round_output).into();

        let auxiliary = SignPublicInput::from((
            class_groups_constants::protocol_public_parameters(),
            bcs::from_bytes::<<AsyncProtocol as twopc_mpc::sign::Protocol>::Message>(
                &hashed_message,
            )?,
            bcs::from_bytes::<<AsyncProtocol as Protocol>::DecentralizedPartyDKGOutput>(
                &dkg_output,
            )?,
            presign,
            bcs::from_bytes::<<AsyncProtocol as twopc_mpc::sign::Protocol>::SignMessage>(
                &centralized_signed_message,
            )?,
            decryption_key_share_public_parameters,
        ));

        Ok(bcs::to_bytes(&auxiliary)?)
    }
}

/// A struct to hold the batched sign session data.
pub struct BatchedSignSession {
    /// A map that contains the ready signatures, indexed by their hashed message. When this map contains all the hashed messages, the batched sign session is ready to be written to the chain.
    pub hashed_msg_to_signature: HashMap<Vec<u8>, Vec<u8>>,
    /// A list of all the messages that need to be signed, in the order they were received. The output list of signatures will be written to the chain in the same order.
    pub ordered_messages: Vec<Vec<u8>>,
}
