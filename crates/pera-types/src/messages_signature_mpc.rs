use crate::base_types::{ObjectID, PeraAddress};
use crate::crypto::default_hash;
use crate::digests::DwalletMPCOutputDigest;
use crate::message_envelope::Message;
use serde::{Deserialize, Serialize};
use shared_crypto::intent::IntentScope;

/// The content of the system transaction that stores the MPC session output on the chain.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DwalletMPCOutput {
    /// The session ID of the MPC session.
    pub session_id: ObjectID,
    /// The address of the initiating user.
    pub initiating_address: PeraAddress,
    /// The final value of the MPC session.
    pub value: Vec<u8>,
}

impl Message for DwalletMPCOutput {
    type DigestType = DwalletMPCOutputDigest;
    const SCOPE: IntentScope = IntentScope::SignatureMPCOutput;

    fn digest(&self) -> Self::DigestType {
        DwalletMPCOutputDigest::new(default_hash(self))
    }
}
