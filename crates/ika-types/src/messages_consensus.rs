// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::crypto::AuthorityName;
use crate::messages_checkpoint::{CheckpointSequenceNumber, CheckpointSignatureMessage};
use crate::supported_protocol_versions::{
    Chain, SupportedProtocolVersions, SupportedProtocolVersionsWithHashes,
};
use byteorder::{BigEndian, ReadBytesExt};
use fastcrypto::error::FastCryptoResult;
use fastcrypto::groups::bls12381;
use fastcrypto_tbls::dkg_v1;
use fastcrypto_zkp::bn254::zk_login::{JwkId, JWK};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use sui_types::base_types::{
    ConciseableName, ObjectID, ObjectRef, SequenceNumber, TransactionDigest,
};
use sui_types::digests::ConsensusCommitDigest;
pub use sui_types::messages_consensus::{AuthorityIndex, TimestampMs, TransactionIndex};

// todo(omersadika): remove that and import from sui_types::messages_consensus once it u64
/// Consensus round number.
pub type Round = u64;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConsensusTransaction {
    /// Encodes an u64 unique tracking id to allow us trace a message between Ika and consensus.
    /// Use an byte array instead of u64 to ensure stable serialization.
    pub tracking_id: [u8; 8],
    pub kind: ConsensusTransactionKind,
}

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub enum ConsensusTransactionKey {
    CheckpointSignature(AuthorityName, CheckpointSequenceNumber),
    InitiateProcessMidEpoch(AuthorityName),
    EndOfPublish(AuthorityName),
    CapabilityNotification(AuthorityName, u64 /* generation */),

    TestMessage(AuthorityName, u64),
}

impl Debug for ConsensusTransactionKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CheckpointSignature(name, seq) => {
                write!(f, "CheckpointSignature({:?}, {:?})", name.concise(), seq)
            }
            Self::InitiateProcessMidEpoch(name) => {
                write!(f, "InitiateProcessMidEpoch({:?})", name.concise())
            }
            Self::EndOfPublish(name) => write!(f, "EndOfPublish({:?})", name.concise()),
            Self::CapabilityNotification(name, generation) => write!(
                f,
                "CapabilityNotification({:?}, {:?})",
                name.concise(),
                generation
            ),
            Self::TestMessage(name, num) => {
                write!(f, "TestMessage({:?}, {})", name.concise(), num)
            }
        }
    }
}

pub type MovePackageDigest = [u8; 32];

/// Used to advertise capabilities of each authority via consensus. This allows validators to
/// negotiate the creation of the AdvanceEpoch transaction.
#[derive(Serialize, Deserialize, Clone, Hash)]
pub struct AuthorityCapabilitiesV1 {
    /// Originating authority - must match transaction source authority from consensus.
    pub authority: AuthorityName,
    /// Generation number set by sending authority. Used to determine which of multiple
    /// AuthorityCapabilities messages from the same authority is the most recent.
    ///
    /// (Currently, we just set this to the current time in milliseconds since the epoch, but this
    /// should not be interpreted as a timestamp.)
    pub generation: u64,

    /// ProtocolVersions that the authority supports.
    pub supported_protocol_versions: SupportedProtocolVersionsWithHashes,

    /// A list of package id to move package digest to
    /// determine whether to do a protocol upgrade on sui.
    pub available_move_packages: Vec<(ObjectID, MovePackageDigest)>,
}

impl Debug for AuthorityCapabilitiesV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthorityCapabilities")
            .field("authority", &self.authority.concise())
            .field("generation", &self.generation)
            .field(
                "supported_protocol_versions",
                &self.supported_protocol_versions,
            )
            .field("available_move_packages", &self.available_move_packages)
            .finish()
    }
}

impl AuthorityCapabilitiesV1 {
    pub fn new(
        authority: AuthorityName,
        chain: Chain,
        supported_protocol_versions: SupportedProtocolVersions,
        available_move_packages: Vec<(ObjectID, MovePackageDigest)>,
    ) -> Self {
        let generation = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Ika did not exist prior to 1970")
            .as_millis()
            .try_into()
            .expect("This build of ika is not supported in the year 500,000,000");
        Self {
            authority,
            generation,
            supported_protocol_versions:
                SupportedProtocolVersionsWithHashes::from_supported_versions(
                    supported_protocol_versions,
                    chain,
                ),
            available_move_packages,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ConsensusTransactionKind {
    CheckpointSignature(Box<CheckpointSignatureMessage>),
    InitiateProcessMidEpoch(AuthorityName),
    EndOfPublish(AuthorityName),

    CapabilityNotificationV1(AuthorityCapabilitiesV1),
    // Test message for checkpoints.
    TestMessage(AuthorityName, u64),
}

impl ConsensusTransaction {
    pub fn new_checkpoint_signature_message(data: CheckpointSignatureMessage) -> Self {
        let mut hasher = DefaultHasher::new();
        data.checkpoint_message
            .auth_sig()
            .signature
            .hash(&mut hasher);
        let tracking_id = hasher.finish().to_le_bytes();
        Self {
            tracking_id,
            kind: ConsensusTransactionKind::CheckpointSignature(Box::new(data)),
        }
    }

    pub fn new_initiate_process_mid_epoch(authority: AuthorityName) -> Self {
        let mut hasher = DefaultHasher::new();
        authority.hash(&mut hasher);
        let tracking_id = hasher.finish().to_le_bytes();
        Self {
            tracking_id,
            kind: ConsensusTransactionKind::InitiateProcessMidEpoch(authority),
        }
    }

    pub fn new_end_of_publish(authority: AuthorityName) -> Self {
        let mut hasher = DefaultHasher::new();
        authority.hash(&mut hasher);
        let tracking_id = hasher.finish().to_le_bytes();
        Self {
            tracking_id,
            kind: ConsensusTransactionKind::EndOfPublish(authority),
        }
    }

    pub fn new_capability_notification_v1(capabilities: AuthorityCapabilitiesV1) -> Self {
        let mut hasher = DefaultHasher::new();
        capabilities.hash(&mut hasher);
        let tracking_id = hasher.finish().to_le_bytes();
        Self {
            tracking_id,
            kind: ConsensusTransactionKind::CapabilityNotificationV1(capabilities),
        }
    }

    pub fn new_test_message(authority: AuthorityName, num: u64) -> Self {
        let mut hasher = DefaultHasher::new();
        authority.hash(&mut hasher);
        hasher.write_u64(num);
        let tracking_id = hasher.finish().to_le_bytes();
        Self {
            tracking_id,
            kind: ConsensusTransactionKind::TestMessage(authority, num),
        }
    }

    pub fn get_tracking_id(&self) -> u64 {
        (&self.tracking_id[..])
            .read_u64::<BigEndian>()
            .unwrap_or_default()
    }

    pub fn key(&self) -> ConsensusTransactionKey {
        match &self.kind {
            ConsensusTransactionKind::CheckpointSignature(data) => {
                ConsensusTransactionKey::CheckpointSignature(
                    data.checkpoint_message.auth_sig().authority,
                    data.checkpoint_message.sequence_number,
                )
            }
            ConsensusTransactionKind::InitiateProcessMidEpoch(authority) => {
                ConsensusTransactionKey::InitiateProcessMidEpoch(*authority)
            }
            ConsensusTransactionKind::EndOfPublish(authority) => {
                ConsensusTransactionKey::EndOfPublish(*authority)
            }
            ConsensusTransactionKind::CapabilityNotificationV1(cap) => {
                ConsensusTransactionKey::CapabilityNotification(cap.authority, cap.generation)
            }
            ConsensusTransactionKind::TestMessage(authority, num) => {
                ConsensusTransactionKey::TestMessage(*authority, *num)
            }
        }
    }

    pub fn is_end_of_publish(&self) -> bool {
        matches!(self.kind, ConsensusTransactionKind::EndOfPublish(_))
    }
}
