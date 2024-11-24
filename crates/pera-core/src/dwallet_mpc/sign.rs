use group::PartyID;
use mpc::{Advance, Party, WeightedThresholdAccessStructure};
use twopc_mpc::dkg::Protocol;

use crate::dwallet_mpc::bytes_party::{AdvanceResult, AsyncProtocol, BytesParty, MPCParty};
use crate::dwallet_mpc::dkg::{DKGFirstParty, DKGFirstPartyAuxiliaryInputGenerator};
use crate::dwallet_mpc::mpc_manager::twopc_error_to_pera_error;
use pera_types::error::{PeraError, PeraResult};
use std::collections::HashMap;

pub type SignFirstParty = <AsyncProtocol as twopc_mpc::sign::Protocol>::SignDecentralizedParty;
pub type SignAuxiliaryInput =
    <AsyncProtocol as twopc_mpc::sign::Protocol>::SignDecentralizedPartyAuxiliaryInput;

impl SignBytesParty {
    pub(crate) fn generate_auxiliary_input(
        session_id: Vec<u8>,
        weighted_threshold_access_structure: WeightedThresholdAccessStructure,
        party_id: PartyID,
        dkg_output: Vec<u8>,
        hashed_message: Vec<u8>,
        presign: Vec<u8>,
        centralized_signed_message: Vec<u8>,
        decryption_key_share_public_parameters: <AsyncProtocol as twopc_mpc::sign::Protocol>::DecryptionKeySharePublicParameters,
    ) -> PeraResult<Vec<u8>> {
        let auxiliary_auxiliary_input = DKGFirstParty::generate_auxiliary_input(
            session_id.clone(),
            party_id,
            weighted_threshold_access_structure,
        );

        let auxiliary = SignAuxiliaryInput::from((
            auxiliary_auxiliary_input,
            bcs::from_bytes::<<AsyncProtocol as twopc_mpc::sign::Protocol>::Message>(
                &hashed_message,
            )?,
            bcs::from_bytes::<<AsyncProtocol as Protocol>::DecentralizedPartyDKGOutput>(
                &dkg_output,
            )?,
            bcs::from_bytes::<<AsyncProtocol as twopc_mpc::presign::Protocol>::Presign>(&presign)?,
            bcs::from_bytes::<<AsyncProtocol as twopc_mpc::sign::Protocol>::SignMessage>(
                &centralized_signed_message,
            )?,
            decryption_key_share_public_parameters,
        ));

        Ok(bcs::to_bytes(&auxiliary)?)
    }
}

/// A wrapper for the decentralized round of the Sign protocol.
///
/// This struct represents the only round of the Sign protocol.
pub struct SignBytesParty {
    pub party: SignFirstParty,
}

impl BytesParty for SignBytesParty {
    fn advance(
        self,
        messages: HashMap<PartyID, Vec<u8>>,
        auxiliary_input: Vec<u8>,
    ) -> PeraResult<AdvanceResult> {
        let mut auxiliary_input: SignAuxiliaryInput =
            // This is not a validator malicious behaviour, as the authority input is being sent by the initiating user.
            // In this case this MPC session should be cancelled.
            bcs::from_bytes(&auxiliary_input).map_err(|_| PeraError::DWalletMPCInvalidUserInput)?;

        let messages = messages
            .into_iter()
            .map(|(party_id, message)| {
                let message = bcs::from_bytes(&message).unwrap();
                (party_id, message)
            })
            .collect::<HashMap<PartyID, _>>();
        let mut rng = rand_core::OsRng;
        let result = self.party.advance(messages, &auxiliary_input, &mut rng);
        if result.is_err() {
            let result = twopc_error_to_pera_error(result.err().unwrap());
            return Err(result);
        }
        match result.map_err(twopc_error_to_pera_error)? {
            mpc::AdvanceResult::Advance((message, new_party)) => Ok(AdvanceResult::Advance((
                bcs::to_bytes(&message)?,
                MPCParty::SignBytesParty(Self { party: new_party }),
            ))),
            mpc::AdvanceResult::Finalize(output) => {
                Ok(AdvanceResult::Finalize(bcs::to_bytes(&output)?, vec![]))
            }
            mpc::AdvanceResult::FinalizeAsync(output) => Ok(AdvanceResult::Finalize(
                bcs::to_bytes(&output.output)?,
                output.malicious_parties,
            )),
        }
    }
}
