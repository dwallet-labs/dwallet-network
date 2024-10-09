use crate::base_types::{ObjectID, PeraAddress};
use crate::committee::EpochId;
use crate::crypto::{default_hash, AuthoritySignInfo};
use crate::digests::{SignatureMPCOutputDigest, SignatureMPCMessageDigest};
use crate::message_envelope::{Envelope, Message};
use serde::{Deserialize, Serialize};
use shared_crypto::intent::IntentScope;
use std::fmt::{Display, Formatter};

impl Message for SignatureMPCMessageSummary {
    type DigestType = SignatureMPCMessageDigest;
    const SCOPE: IntentScope = IntentScope::SignatureMPCMessage;

    fn digest(&self) -> Self::DigestType {
        SignatureMPCMessageDigest::new(default_hash(self))
    }
}

impl Display for SignatureMPCMessageSummary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SignatureMPCMessage {{ epoch: {:?}, message: {:?}, session_id: {:?}}}",
            self.epoch, self.message, self.session_id,
        )
    }
}

pub type SignedSignatureMPCMessageSummary = SignatureMPCMessageSummaryEnvelope<AuthoritySignInfo>;

pub type SignatureMPCMessageSummaryEnvelope<S> = Envelope<SignatureMPCMessageSummary, S>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignatureMPCMessageSummary {
    pub epoch: EpochId,
    pub message: Vec<u8>,
    pub session_id: SignatureMPCSessionID,
}

/// The session ID of the MPC is working on.
#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignatureMPCSessionID(pub [u8; SESSION_ID_LENGTH]);
const SESSION_ID_LENGTH: usize = 32;
pub type SignatureMPCRound = u64;
pub type SignatureMPCMessageKind = u64;

/// The message validators sending to each other during the signature MPC flow.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignatureMPCMessage {
    pub summary: SignedSignatureMPCMessageSummary,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SignatureMPCOutput {
    pub session_id: ObjectID,
    pub sender_address: PeraAddress,
    pub value: Vec<Vec<u8>>,
}

impl Message for SignatureMPCOutput {
    type DigestType = SignatureMPCOutputDigest;
    const SCOPE: IntentScope = IntentScope::SignatureMPCOutput;

    fn digest(&self) -> Self::DigestType {
        SignatureMPCOutputDigest::new(default_hash(self))
    }
}

pub type SignatureMPCOutputEnvelope<S> = Envelope<SignatureMPCOutput, S>;
pub type SignedSignatureMPCOutput = SignatureMPCOutputEnvelope<AuthoritySignInfo>;
