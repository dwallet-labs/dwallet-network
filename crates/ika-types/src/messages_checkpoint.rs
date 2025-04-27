// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::committee::EpochId;
use crate::crypto::{
    default_hash, AggregateAuthoritySignature, AuthoritySignInfo, AuthoritySignInfoTrait,
    AuthorityStrongQuorumSignInfo,
};
use crate::error::IkaResult;
use crate::intent::{Intent, IntentScope};
use crate::message_envelope::{Envelope, Message, TrustedEnvelope, VerifiedEnvelope};
use crate::{committee::Committee, error::IkaError};
use prometheus::Histogram;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tap::TapFallible;
use tracing::warn;

pub use crate::digests::CheckpointContentsDigest;
pub use crate::digests::CheckpointMessageDigest;
use crate::message::MessageKind;

pub type CheckpointSequenceNumber = u64;
pub type CheckpointTimestamp = u64;

// The constituent parts of checkpoints, signed and certified

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CheckpointMessage {
    pub epoch: EpochId,
    pub sequence_number: CheckpointSequenceNumber,
    /// Timestamp of the checkpoint - number of milliseconds from the Unix epoch
    /// Checkpoint timestamps are monotonic, but not strongly monotonic - subsequent
    /// checkpoints can have same timestamp if they originate from the same underlining consensus commit
    pub timestamp_ms: CheckpointTimestamp,
    pub messages: Vec<MessageKind>,
}

impl Message for CheckpointMessage {
    type DigestType = CheckpointMessageDigest;
    const SCOPE: IntentScope = IntentScope::CheckpointMessage;

    fn digest(&self) -> Self::DigestType {
        CheckpointMessageDigest::new(default_hash(self))
    }
}

impl CheckpointMessage {
    pub fn new(
        epoch: EpochId,
        sequence_number: CheckpointSequenceNumber,
        messages: Vec<MessageKind>,
        timestamp_ms: CheckpointTimestamp,
    ) -> CheckpointMessage {
        Self {
            epoch,
            sequence_number,
            messages,
            timestamp_ms,
        }
    }

    pub fn verify_epoch(&self, epoch: EpochId) -> IkaResult {
        fp_ensure!(
            self.epoch == epoch,
            IkaError::WrongEpoch {
                expected_epoch: epoch,
                actual_epoch: self.epoch,
            }
        );
        Ok(())
    }

    pub fn sequence_number(&self) -> &CheckpointSequenceNumber {
        &self.sequence_number
    }

    pub fn timestamp(&self) -> SystemTime {
        UNIX_EPOCH + Duration::from_millis(self.timestamp_ms)
    }

    pub fn report_checkpoint_age(&self, metrics: &Histogram) {
        SystemTime::now()
            .duration_since(self.timestamp())
            .map(|latency| {
                metrics.observe(latency.as_secs_f64());
            })
            .tap_err(|err| {
                warn!(
                    checkpoint_seq = self.sequence_number,
                    "unable to compute checkpoint age: {}", err
                )
            })
            .ok();
    }
}

impl Display for CheckpointMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CheckpointSummary {{ epoch: {:?}, seq: {:?}",
            self.epoch, self.sequence_number,
        )
    }
}

// Checkpoints are signed by an authority and 2f+1 form a
// certificate that others can use to catch up. The actual
// content of the digest must at the very least commit to
// the set of transactions contained in the certificate but
// we might extend this to contain roots of merkle trees,
// or other authenticated data structures to support light
// clients and more efficient sync protocols.

pub type CheckpointMessageEnvelope<S> = Envelope<CheckpointMessage, S>;
pub type CertifiedCheckpointMessage = CheckpointMessageEnvelope<AuthorityStrongQuorumSignInfo>;
pub type SignedCheckpointMessage = CheckpointMessageEnvelope<AuthoritySignInfo>;

pub type VerifiedCheckpointMessage =
    VerifiedEnvelope<CheckpointMessage, AuthorityStrongQuorumSignInfo>;
pub type TrustedCheckpointMessage =
    TrustedEnvelope<CheckpointMessage, AuthorityStrongQuorumSignInfo>;

impl CertifiedCheckpointMessage {
    pub fn verify_authority_signatures(&self, committee: &Committee) -> IkaResult {
        self.data().verify_epoch(self.auth_sig().epoch)?;
        self.auth_sig().verify_secure(
            self.data(),
            Intent::ika_app(IntentScope::CheckpointMessage),
            committee,
        )
    }

    pub fn try_into_verified(self, committee: &Committee) -> IkaResult<VerifiedCheckpointMessage> {
        self.verify_authority_signatures(committee)?;
        Ok(VerifiedCheckpointMessage::new_from_verified(self))
    }

    pub fn into_summary_and_sequence(self) -> (CheckpointSequenceNumber, CheckpointMessage) {
        let summary = self.into_data();
        (summary.sequence_number, summary)
    }

    pub fn get_validator_signature(self) -> AggregateAuthoritySignature {
        self.auth_sig().signature.clone()
    }
}

impl SignedCheckpointMessage {
    pub fn verify_authority_signatures(&self, committee: &Committee) -> IkaResult {
        self.data().verify_epoch(self.auth_sig().epoch)?;
        self.auth_sig().verify_secure(
            self.data(),
            Intent::ika_app(IntentScope::CheckpointMessage),
            committee,
        )
    }

    pub fn try_into_verified(
        self,
        committee: &Committee,
    ) -> IkaResult<VerifiedEnvelope<CheckpointMessage, AuthoritySignInfo>> {
        self.verify_authority_signatures(committee)?;
        Ok(VerifiedEnvelope::<CheckpointMessage, AuthoritySignInfo>::new_from_verified(self))
    }
}

impl VerifiedCheckpointMessage {
    pub fn into_summary_and_sequence(self) -> (CheckpointSequenceNumber, CheckpointMessage) {
        self.into_inner().into_summary_and_sequence()
    }
}

/// This is a message validators publish to consensus in order to sign checkpoint
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CheckpointSignatureMessage {
    pub checkpoint_message: SignedCheckpointMessage,
}

impl CheckpointSignatureMessage {
    pub fn verify(&self, committee: &Committee) -> IkaResult {
        self.checkpoint_message
            .verify_authority_signatures(committee)
    }
}
