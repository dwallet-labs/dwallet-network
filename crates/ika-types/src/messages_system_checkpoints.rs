// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::committee::{EpochId, ProtocolVersion};
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

pub use crate::digests::SystemCheckpointContentsDigest;
pub use crate::digests::SystemCheckpointMessageDigest;

pub type SystemCheckpointSequenceNumber = u64;
pub type SystemCheckpointTimestamp = u64;

// The constituent parts of system checkpoints, signed and certified.
// Note: the order of these fields, and the number must correspond to the Move code in
// `system_inner.move`.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SystemCheckpointMessageKind {
    /// Set the next protocol version for the next epoch.
    SetNextConfigVersion(ProtocolVersion),
    /// Set a new epoch duration in milliseconds.
    SetEpochDurationMs(u64),
    /// Set a new stake subsidy start epoch.
    SetStakeSubsidyStartEpoch(EpochId),
    /// Set a new stake subsidy rate in basis points (1/100th of a percent).
    /// The distribution per period will be recalculated.
    SetStakeSubsidyRate(u16),
    /// Set a new length of the stake subsidy period.
    /// The distribution per period will be recalculated.
    SetStakeSubsidyPeriodLength(u64),
    /// Set a new minimum number of validators required to be active in the system.
    SetMinValidatorCount(u64),
    /// Set a new maximum number of validators allowed in the system.
    SetMaxValidatorCount(u64),
    /// Set a new minimum stake required for a validator to join the system.
    SetMinValidatorJoiningStake(u64),
    /// Set a new maximum number of validators that can change in a single epoch.
    SetMaxValidatorChangeCount(u64),
    /// Set a new rate at which rewards are slashed in basis points (1/100th of a percent).
    SetRewardSlashingRate(u64),
    /// The last checkpoint message of the epoch.
    /// After the Sui smart contract receives this message, it knows that no more system checkpoints will get created
    /// in this epoch, and it allows external calls to advance the epoch.
    EndOfPublish,
    /// Set an approved upgrade for a package.
    SetApprovedUpgrade {
        /// The ID of the package that is approved for upgrade.
        package_id: Vec<u8>,
        /// The version of the package that is approved for upgrade.
        /// if None, the upgrade approval will be deleted.
        digest: Option<Vec<u8>>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SystemCheckpointMessage {
    pub epoch: EpochId,
    pub sequence_number: SystemCheckpointSequenceNumber,
    /// Timestamp of the system checkpoint - number of milliseconds from the Unix epoch
    /// System checkpoint timestamps are monotonic, but not strongly monotonic - subsequent
    /// system checkpoints can have same timestamp if they originate from the same underlining consensus commit
    pub timestamp_ms: SystemCheckpointTimestamp,
    pub messages: Vec<SystemCheckpointMessageKind>,
}

impl Message for SystemCheckpointMessage {
    type DigestType = SystemCheckpointMessageDigest;
    const SCOPE: IntentScope = IntentScope::SystemCheckpointMessage;

    fn digest(&self) -> Self::DigestType {
        SystemCheckpointMessageDigest::new(default_hash(self))
    }
}

impl SystemCheckpointMessage {
    pub fn new(
        epoch: EpochId,
        sequence_number: SystemCheckpointSequenceNumber,
        messages: Vec<SystemCheckpointMessageKind>,
        timestamp_ms: SystemCheckpointTimestamp,
    ) -> SystemCheckpointMessage {
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

    pub fn sequence_number(&self) -> &SystemCheckpointSequenceNumber {
        &self.sequence_number
    }

    pub fn timestamp(&self) -> SystemTime {
        UNIX_EPOCH + Duration::from_millis(self.timestamp_ms)
    }

    pub fn report_system_checkpoint_age(&self, metrics: &Histogram) {
        SystemTime::now()
            .duration_since(self.timestamp())
            .map(|latency| {
                metrics.observe(latency.as_secs_f64());
            })
            .tap_err(|err| {
                warn!(
                    system_checkpoint_seq = self.sequence_number,
                    "unable to compute system checkpoint age: {}", err
                )
            })
            .ok();
    }
}

impl Display for SystemCheckpointMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SystemCheckpointSummary {{ epoch: {:?}, seq: {:?}",
            self.epoch, self.sequence_number,
        )
    }
}

// System checkpoints are signed by an authority and 2f+1 form a
// certificate that others can use to catch up. The actual
// content of the digest must at the very least commit to
// the set of transactions contained in the certificate but
// we might extend this to contain roots of merkle trees,
// or other authenticated data structures to support light
// clients and more efficient sync protocols.

pub type SystemCheckpointMessageEnvelope<S> = Envelope<SystemCheckpointMessage, S>;
pub type CertifiedSystemCheckpointMessage =
    SystemCheckpointMessageEnvelope<AuthorityStrongQuorumSignInfo>;
pub type SignedSystemCheckpointMessage = SystemCheckpointMessageEnvelope<AuthoritySignInfo>;

pub type VerifiedSystemCheckpointMessage =
    VerifiedEnvelope<SystemCheckpointMessage, AuthorityStrongQuorumSignInfo>;
pub type TrustedSystemCheckpointMessage =
    TrustedEnvelope<SystemCheckpointMessage, AuthorityStrongQuorumSignInfo>;

impl CertifiedSystemCheckpointMessage {
    pub fn verify_authority_signatures(&self, committee: &Committee) -> IkaResult {
        self.data().verify_epoch(self.auth_sig().epoch)?;
        self.auth_sig().verify_secure(
            self.data(),
            Intent::ika_app(IntentScope::SystemCheckpointMessage),
            committee,
        )
    }

    pub fn try_into_verified(
        self,
        committee: &Committee,
    ) -> IkaResult<VerifiedSystemCheckpointMessage> {
        self.verify_authority_signatures(committee)?;
        Ok(VerifiedSystemCheckpointMessage::new_from_verified(self))
    }

    pub fn into_summary_and_sequence(
        self,
    ) -> (SystemCheckpointSequenceNumber, SystemCheckpointMessage) {
        let summary = self.into_data();
        (summary.sequence_number, summary)
    }

    pub fn get_validator_signature(self) -> AggregateAuthoritySignature {
        self.auth_sig().signature.clone()
    }
}

impl SignedSystemCheckpointMessage {
    pub fn verify_authority_signatures(&self, committee: &Committee) -> IkaResult {
        self.data().verify_epoch(self.auth_sig().epoch)?;
        self.auth_sig().verify_secure(
            self.data(),
            Intent::ika_app(IntentScope::SystemCheckpointMessage),
            committee,
        )
    }

    pub fn try_into_verified(
        self,
        committee: &Committee,
    ) -> IkaResult<VerifiedEnvelope<SystemCheckpointMessage, AuthoritySignInfo>> {
        self.verify_authority_signatures(committee)?;
        Ok(VerifiedEnvelope::<SystemCheckpointMessage, AuthoritySignInfo>::new_from_verified(self))
    }
}

impl VerifiedSystemCheckpointMessage {
    pub fn into_summary_and_sequence(
        self,
    ) -> (SystemCheckpointSequenceNumber, SystemCheckpointMessage) {
        self.into_inner().into_summary_and_sequence()
    }
}

/// This is a message validators publish to consensus in order to sign system checkpoint
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemCheckpointSignatureMessage {
    pub checkpoint_message: SignedSystemCheckpointMessage,
}

impl SystemCheckpointSignatureMessage {
    pub fn verify(&self, committee: &Committee) -> IkaResult {
        self.checkpoint_message
            .verify_authority_signatures(committee)
    }
}
