use crate::base_types::{ObjectID, PeraAddress};
use crate::crypto::default_hash;
use crate::digests::DWalletMPCOutputDigest;
use crate::message_envelope::Message;
use group::PartyID;
use pera_mpc_types::dwallet_mpc::{MPCMessage, MPCOutput};
use serde::{Deserialize, Serialize};
use shared_crypto::intent::IntentScope;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MPCRound {
    /// The first round of the DKG protocol.
    DKGFirst,
    /// The second round of the DKG protocol.
    DKGSecond,
    /// The first round of the Presign protocol.
    /// Contains the `ObjectId` of the dWallet object,
    /// and the DKG decentralized output.
    PresignFirst(ObjectID, Vec<u8>),
    /// The second round of the Presign protocol.
    /// Contains the `ObjectId` of the dWallet object,
    /// and the Presign first round output.
    PresignSecond(ObjectID, Vec<u8>),
    /// The first and only round of the Sign protocol.
    /// Contains the `PartyID` associated with the decryption share,
    /// the `ObjectID` of the batched sign session,
    /// and the hashed message being signed.
    Sign(ObjectID, Vec<u8>),
    /// A batched sign session, contains the list of messages that are being signed.
    BatchedSign(Vec<Vec<u8>>),
    /// The round of the network DKG protocol.
    NetworkDkg,
}

/// The content of the system transaction that stores the MPC session output on the chain.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DWalletMPCOutput {
    /// The session information of the MPC session.
    pub session_info: SessionInfo,
    /// The final value of the MPC session.
    pub output: Vec<u8>,
}

impl Message for DWalletMPCOutput {
    type DigestType = DWalletMPCOutputDigest;
    const SCOPE: IntentScope = IntentScope::DWalletMPCOutput;

    fn digest(&self) -> Self::DigestType {
        DWalletMPCOutputDigest::new(default_hash(self))
    }
}

// todo(zeev): rename to MPCSessionInfo.
/// Holds information about the current MPC session.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct SessionInfo {
    /// The session ID of the first round in the flow — e.g.,
    /// in Presign we have two rounds, so the session ID of the first.
    pub flow_session_id: ObjectID,
    /// Unique identifier for the MPC session.
    pub session_id: ObjectID,
    /// The address of the user that initiated this session.
    pub initiating_user_address: PeraAddress,
    /// The `DWalletCap` object's ID associated with the `DWallet`.
    // TODO (#365): Remove DWallet cap ID from the [`SessionInfo`] struct and move it to the DKG second [`MPCRound`]
    pub dwallet_cap_id: ObjectID,
    /// The current MPC round in the protocol.
    /// Contains extra parameters if needed.
    pub mpc_round: MPCRound,
}
