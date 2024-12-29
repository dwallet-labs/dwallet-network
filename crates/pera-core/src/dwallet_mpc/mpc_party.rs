use crate::dwallet_mpc;
use crate::dwallet_mpc::dkg::{DKGFirstParty, DKGSecondParty};
use crate::dwallet_mpc::network_dkg::advance_network_dkg;
use crate::dwallet_mpc::presign::{PresignFirstParty, PresignSecondParty};
use crate::dwallet_mpc::sign::SignFirstParty;
use commitment::CommitmentSizedNumber;
use dwallet_mpc_types::dwallet_mpc::{DWalletMPCNetworkKey, MPCMessage, MPCPrivateOutput, MPCPublicInput};
use group::PartyID;
use mpc::WeightedThresholdAccessStructure;
use pera_types::base_types::ObjectID;
use pera_types::dwallet_mpc_error::DwalletMPCResult;
use std::collections::HashMap;
use twopc_mpc::sign::Protocol;

pub(super) type AsyncProtocol = twopc_mpc::secp256k1::class_groups::AsyncProtocol;

/// Enum representing the different parties used in the MPC manager.
#[derive(Clone, Debug)]
pub enum MPCParty {
    /// The party used in the first round of the DKG protocol.
    FirstDKGBytesParty,
    /// The party used in the second round of the DKG protocol.
    SecondDKGBytesParty,
    /// The party used in the first round of the presign protocol.
    FirstPresignBytesParty,
    /// The party used in the second round of the presign protocol.
    SecondPresignBytesParty,
    /// The party used in the sign protocol.
    SignBytesParty(HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>),
    /// The party used in the network DKG protocol.
    NetworkDkg(DWalletMPCNetworkKeyScheme),
}

impl MPCParty {
    /// Advances the party to the next round by processing incoming messages and public input.
    /// Returns the next [`MPCParty`] to use, or the final output if the protocol has completed.
    pub fn advance(
        &self,
        messages: Vec<HashMap<PartyID, MPCMessage>>,
        session_id: ObjectID,
        party_id: PartyID,
        access_threshold: &WeightedThresholdAccessStructure,
        public_input: MPCPublicInput,
    ) -> DwalletMPCResult<mpc::AsynchronousRoundResult<Vec<u8>, MPCPrivateOutput, Vec<u8>>> {
        let session_id = CommitmentSizedNumber::from_le_slice(session_id.to_vec().as_slice());
        let res = match &self {
            MPCParty::FirstDKGBytesParty => {
                let public_input = bcs::from_bytes(&public_input)?;
                dwallet_mpc::advance::<DKGFirstParty>(
                    session_id,
                    party_id,
                    access_threshold,
                    messages,
                    public_input,
                    (),
                )
            }
            MPCParty::SecondDKGBytesParty => {
                let public_input = bcs::from_bytes(&public_input)?;
                dwallet_mpc::advance::<DKGSecondParty>(
                    session_id,
                    party_id,
                    access_threshold,
                    messages,
                    public_input,
                    (),
                )
            }
            MPCParty::FirstPresignBytesParty => {
                let public_input = bcs::from_bytes(&public_input)?;
                dwallet_mpc::advance::<PresignFirstParty>(
                    session_id,
                    party_id,
                    access_threshold,
                    messages,
                    public_input,
                    (),
                )
            }
            MPCParty::SecondPresignBytesParty => {
                let public_input = bcs::from_bytes(&public_input)?;
                dwallet_mpc::advance::<PresignSecondParty>(
                    session_id,
                    party_id,
                    access_threshold,
                    messages,
                    public_input,
                    (),
                )
            }
            MPCParty::SignBytesParty(decryption_key_share) => {
                let public_input = bcs::from_bytes(&public_input)?;
                dwallet_mpc::advance::<SignFirstParty>(
                    session_id,
                    party_id,
                    access_threshold,
                    messages,
                    public_input,
                    decryption_key_share.clone(),
                )
            }
            MPCParty::NetworkDkg(key_type) => {
                return advance_network_dkg(
                    session_id,
                    access_threshold,
                    party_id,
                    &public_input,
                    key_type,
                    messages,
                );
            }
        }?;

        match res {
            mpc::AsynchronousRoundResult::Finalize {
                malicious_parties,
                private_output,
                public_output,
            } => Ok(mpc::AsynchronousRoundResult::Finalize {
                malicious_parties,
                private_output: MPCPrivateOutput::None,
                public_output,
            }),
            mpc::AsynchronousRoundResult::Advance {
                malicious_parties,
                message,
            } => Ok(mpc::AsynchronousRoundResult::Advance {
                malicious_parties,
                message,
            }),
        }
    }
}
