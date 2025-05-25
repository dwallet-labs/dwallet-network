// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::crypto::AuthorityName;
use crate::messages_dwallet_checkpoint::{
    DWalletCheckpointSequenceNumber, DWalletCheckpointSignatureMessage,
};
use crate::messages_dwallet_mpc::{
    DWalletMPCMessage, DWalletMPCMessageKey, MaliciousReport, SessionInfo,
    ThresholdNotReachedReport,
};
use crate::messages_system_checkpoints::{
    SystemCheckpointSequenceNumber, SystemCheckpointSignatureMessage,
};
use crate::supported_protocol_versions::{
    SupportedProtocolVersions, SupportedProtocolVersionsWithHashes,
};
use byteorder::{BigEndian, ReadBytesExt};
use ika_protocol_config::Chain;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};
use sui_types::base_types::{ConciseableName, ObjectID};
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
    DWalletCheckpointSignature(AuthorityName, DWalletCheckpointSequenceNumber),
    CapabilityNotification(AuthorityName, u64 /* generation */),
    /// The message sent between MPC parties in a dwallet MPC session.
    DWalletMPCMessage(DWalletMPCMessageKey),
    /// The output of a dwallet MPC session.
    /// The [`Vec<u8>`] is the data, the [`ObjectID`] is the session ID and the [`PeraAddress`] is the
    /// address of the initiating user.
    DWalletMPCOutput(Vec<u8>, ObjectID, AuthorityName),
    DWalletMPCSessionFailedWithMalicious(AuthorityName, MaliciousReport),
    DWalletMPCThresholdNotReached(AuthorityName, ThresholdNotReachedReport),
    SystemCheckpointSignature(AuthorityName, SystemCheckpointSequenceNumber),
}

impl Debug for ConsensusTransactionKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DWalletCheckpointSignature(name, seq) => {
                write!(
                    f,
                    "DWalletCheckpointSignature({:?}, {:?})",
                    name.concise(),
                    seq
                )
            }
            Self::CapabilityNotification(name, generation) => write!(
                f,
                "CapabilityNotification({:?}, {:?})",
                name.concise(),
                generation
            ),
            Self::DWalletMPCMessage(message) => {
                write!(f, "DWalletMPCMessage({:?})", message,)
            }
            Self::DWalletMPCOutput(value, session_id, authority) => {
                write!(
                    f,
                    "DWalletMPCOutput({:?}, {:?}, {:?})",
                    value, session_id, authority
                )
            }
            Self::DWalletMPCSessionFailedWithMalicious(authority, report) => {
                write!(
                    f,
                    "DWalletMPCSessionFailedWithMalicious({:?}, {:?})",
                    authority.concise(),
                    report,
                )
            }
            ConsensusTransactionKey::DWalletMPCThresholdNotReached(authority, report) => {
                write!(
                    f,
                    "DWalletMPCThresholdNotReached({:?}, {:?})",
                    authority.concise(),
                    report,
                )
            }
            ConsensusTransactionKey::SystemCheckpointSignature(name, seq) => {
                write!(
                    f,
                    "SystemCheckpointSignature({:?}, {:?})",
                    name.concise(),
                    seq
                )
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
    pub generation: u64,

    /// ProtocolVersions that the authority supports.
    pub supported_protocol_versions: SupportedProtocolVersionsWithHashes,

    /// A list of package id to move package digest to
    /// determine whether to do a protocol upgrade on sui.
    pub available_move_packages: Vec<(ObjectID, MovePackageDigest)>,
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
            .expect("Sui did not exist prior to 1970")
            .as_millis()
            .try_into()
            .expect("This build of sui is not supported in the year 500,000,000");
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ConsensusTransactionKind {
    DWalletCheckpointSignature(Box<DWalletCheckpointSignatureMessage>),
    CapabilityNotificationV1(AuthorityCapabilitiesV1),
    DWalletMPCMessage(DWalletMPCMessage),
    DWalletMPCOutput(AuthorityName, Box<SessionInfo>, Vec<u8>),
    /// Sending Authority and its MaliciousReport.
    DWalletMPCMaliciousReport(AuthorityName, MaliciousReport),
    DWalletMPCThresholdNotReached(AuthorityName, ThresholdNotReachedReport),
    SystemCheckpointSignature(Box<SystemCheckpointSignatureMessage>),
}

impl ConsensusTransaction {
    /// Create a new consensus transaction with the message to be sent to the other MPC parties.
    pub fn new_dwallet_mpc_message(
        authority: AuthorityName,
        message: Vec<u8>,
        session_id: ObjectID,
        round_number: usize,
    ) -> Self {
        let mut hasher = DefaultHasher::new();
        session_id.into_bytes().hash(&mut hasher);
        let tracking_id = hasher.finish().to_le_bytes();
        Self {
            tracking_id,
            kind: ConsensusTransactionKind::DWalletMPCMessage(DWalletMPCMessage {
                message,
                authority,
                round_number,
                session_id,
            }),
        }
    }

    /// Create a new consensus transaction with the output of the MPC session to be sent to the parties.
    pub fn new_dwallet_mpc_output(
        authority: AuthorityName,
        output: Vec<u8>,
        session_info: SessionInfo,
    ) -> Self {
        let mut hasher = DefaultHasher::new();
        output.hash(&mut hasher);
        let tracking_id = hasher.finish().to_le_bytes();
        Self {
            tracking_id,
            kind: ConsensusTransactionKind::DWalletMPCOutput(authority, Box::new(session_info), output),
        }
    }

    /// Create a new consensus transaction with the output of the MPC session to be sent to the parties.
    pub fn new_dwallet_mpc_session_failed_with_malicious(
        authority: AuthorityName,
        report: MaliciousReport,
    ) -> Self {
        let mut hasher = DefaultHasher::new();
        report.session_id.hash(&mut hasher);
        let tracking_id = hasher.finish().to_le_bytes();
        Self {
            tracking_id,
            kind: ConsensusTransactionKind::DWalletMPCMaliciousReport(authority, report),
        }
    }

    pub fn new_dwallet_mpc_session_threshold_not_reached(
        authority: AuthorityName,
        report: ThresholdNotReachedReport,
    ) -> Self {
        let mut hasher = DefaultHasher::new();
        report.session_id.hash(&mut hasher);
        let tracking_id = hasher.finish().to_le_bytes();
        Self {
            tracking_id,
            kind: ConsensusTransactionKind::DWalletMPCThresholdNotReached(authority, report),
        }
    }

    pub fn new_dwallet_checkpoint_signature_message(
        data: DWalletCheckpointSignatureMessage,
    ) -> Self {
        let mut hasher = DefaultHasher::new();
        data.dwallet_checkpoint_message
            .auth_sig()
            .signature
            .hash(&mut hasher);
        let tracking_id = hasher.finish().to_le_bytes();
        Self {
            tracking_id,
            kind: ConsensusTransactionKind::DWalletCheckpointSignature(Box::new(data)),
        }
    }

    pub fn new_system_checkpoint_signature_message(data: SystemCheckpointSignatureMessage) -> Self {
        let mut hasher = DefaultHasher::new();
        data.system_checkpoint
            .auth_sig()
            .signature
            .hash(&mut hasher);
        let tracking_id = hasher.finish().to_le_bytes();
        Self {
            tracking_id,
            kind: ConsensusTransactionKind::SystemCheckpointSignature(Box::new(data)),
        }
    }

    pub fn new_capability_notification_v1(data: AuthorityCapabilitiesV1) -> Self {
        let mut hasher = DefaultHasher::new();
        data.authority.hash(&mut hasher);
        let tracking_id = hasher.finish().to_le_bytes();
        Self {
            tracking_id,
            kind: ConsensusTransactionKind::CapabilityNotificationV1(data),
        }
    }

    pub fn get_tracking_id(&self) -> u64 {
        (&self.tracking_id[..])
            .read_u64::<BigEndian>()
            .unwrap_or_default()
    }

    pub fn key(&self) -> ConsensusTransactionKey {
        match &self.kind {
            ConsensusTransactionKind::DWalletCheckpointSignature(data) => {
                ConsensusTransactionKey::DWalletCheckpointSignature(
                    data.dwallet_checkpoint_message.auth_sig().authority,
                    data.dwallet_checkpoint_message.sequence_number,
                )
            }
            ConsensusTransactionKind::CapabilityNotificationV1(cap) => {
                ConsensusTransactionKey::CapabilityNotification(cap.authority, cap.generation)
            }
            ConsensusTransactionKind::DWalletMPCMessage(message) => {
                ConsensusTransactionKey::DWalletMPCMessage(DWalletMPCMessageKey {
                    authority: message.authority,
                    session_id: message.session_id,
                    round_number: message.round_number,
                })
            }
            ConsensusTransactionKind::DWalletMPCOutput(authority, session_info, output) => {
                ConsensusTransactionKey::DWalletMPCOutput(
                    output.clone(),
                    session_info.session_id,
                    *authority,
                )
            }
            ConsensusTransactionKind::DWalletMPCMaliciousReport(authority, report) => {
                ConsensusTransactionKey::DWalletMPCSessionFailedWithMalicious(
                    *authority,
                    report.clone(),
                )
            }
            ConsensusTransactionKind::DWalletMPCThresholdNotReached(authority, report) => {
                ConsensusTransactionKey::DWalletMPCThresholdNotReached(*authority, report.clone())
            }
            ConsensusTransactionKind::SystemCheckpointSignature(data) => {
                ConsensusTransactionKey::SystemCheckpointSignature(
                    data.system_checkpoint.auth_sig().authority,
                    data.system_checkpoint.sequence_number,
                )
            }
        }
    }
}
