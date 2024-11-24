use crate::base_types::{ObjectID, PeraAddress};
use crate::crypto::default_hash;
use crate::digests::DWalletMPCOutputDigest;
use crate::message_envelope::Message;
use group::PartyID;
use serde::{Deserialize, Serialize};
use shared_crypto::intent::IntentScope;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MPCRound {
    /// The first round of the DKG protocol.
    DKGFirst,
    /// The second round of the DKG protocol.
    DKGSecond,
    /// The first round of the presign protocol.
    /// Contains the `ObjectId` of the dwallet object and the dkg decentralized output.
    PresignFirst(ObjectID, Vec<u8>),
    /// The second round of the presign protocol.
    /// Contains the `ObjectId` of the dwallet object and the presign first round output.
    PresignSecond(ObjectID, Vec<u8>),
    /// The first round of the sign protocol.
    /// Contains the party id associated with the decryption share.
    Sign(PartyID),
}

/// The content of the system transaction that stores the MPC session output on chain.
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

/// Holds information about the current MPC session.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct SessionInfo {
    /// Unique identifier for the MPC session.
    pub session_id: ObjectID,
    /// The address of the user that initiated this session.
    pub initiating_user_address: PeraAddress,
    /// The `DWalletCap` object's ID associated with the `DWallet`.
    pub dwallet_cap_id: ObjectID,
    /// The current MPC round in the protocol.
    /// Contains extra parameters if needed.
    pub mpc_round: MPCRound,
}
