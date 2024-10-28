use crate::base_types::{ObjectID, PeraAddress};
use crate::crypto::{default_hash, AuthoritySignInfo};
use crate::digests::SignatureMPCOutputDigest;
use crate::message_envelope::{Envelope, Message};
use serde::{Deserialize, Serialize};
use shared_crypto::intent::IntentScope;
use std::fmt::Display;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum MPCRound {
    DKGFirst,
    DKGSecond,
}

/// The content of the system transaction that stores the MPC session output on chain.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SignatureMPCOutput {
    /// The session ID of the MPC session.
    pub session_id: ObjectID,
    /// The address of the initiating user.
    pub sender_address: PeraAddress,
    pub dwallet_cap_id: ObjectID,
    /// The final value of the MPC session.
    pub value: Vec<u8>,
    pub mpc_round: MPCRound,
}

impl Message for SignatureMPCOutput {
    type DigestType = SignatureMPCOutputDigest;
    const SCOPE: IntentScope = IntentScope::SignatureMPCOutput;

    fn digest(&self) -> Self::DigestType {
        SignatureMPCOutputDigest::new(default_hash(self))
    }
}
