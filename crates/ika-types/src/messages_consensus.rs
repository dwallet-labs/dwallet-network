// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::crypto::AuthorityName;
use crate::messages_checkpoint::{CheckpointSequenceNumber, CheckpointSignatureMessage};
use crate::messages_dwallet_mpc::{
    DWalletMPCMessage, DWalletMPCMessageKey, MaliciousReport, SessionInfo,
};
use crate::supported_protocol_versions::SupportedProtocolVersionsWithHashes;
use byteorder::{BigEndian, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
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
    CheckpointSignature(AuthorityName, CheckpointSequenceNumber),
    CapabilityNotification(AuthorityName, u64 /* generation */),
    /// The message sent between MPC parties in a dwallet MPC session.
    DWalletMPCMessage(DWalletMPCMessageKey),
    /// The output of a dwallet MPC session.
    /// The [`Vec<u8>`] is the data, the [`ObjectID`] is the session ID and the [`PeraAddress`] is the
    /// address of the initiating user.
    DWalletMPCOutput(Vec<u8>, ObjectID, AuthorityName),
    DWalletMPCSessionFailedWithMalicious(AuthorityName, MaliciousReport),
}

impl Debug for ConsensusTransactionKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CheckpointSignature(name, seq) => {
                write!(f, "CheckpointSignature({:?}, {:?})", name.concise(), seq)
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ConsensusTransactionKind {
    CheckpointSignature(Box<CheckpointSignatureMessage>),
    CapabilityNotificationV1(AuthorityCapabilitiesV1),
    DWalletMPCMessage(DWalletMPCMessage),
    DWalletMPCOutput(AuthorityName, SessionInfo, Vec<u8>),
    /// Sending Authority and its MaliciousReport.
    DWalletMPCSessionFailedWithMalicious(AuthorityName, MaliciousReport),
}

impl ConsensusTransaction {
    /// Create a new consensus transaction with the message to be sent to the other MPC parties.
    pub fn new_dwallet_mpc_message(
        authority: AuthorityName,
        message: Vec<u8>,
        session_id: ObjectID,
        round_number: usize,
        session_sequence_number: u64,
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
            kind: ConsensusTransactionKind::DWalletMPCOutput(authority, session_info, output),
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
            kind: ConsensusTransactionKind::DWalletMPCSessionFailedWithMalicious(authority, report),
        }
    }

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
            ConsensusTransactionKind::CapabilityNotificationV1(cap) => {
                ConsensusTransactionKey::CapabilityNotification(cap.authority, cap.generation)
            }
            ConsensusTransactionKind::DWalletMPCMessage(message) => {
                ConsensusTransactionKey::DWalletMPCMessage(DWalletMPCMessageKey {
                    authority: message.authority.clone(),
                    session_id: message.session_id.clone(),
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
            ConsensusTransactionKind::DWalletMPCSessionFailedWithMalicious(authority, report) => {
                ConsensusTransactionKey::DWalletMPCSessionFailedWithMalicious(
                    *authority,
                    report.clone(),
                )
            }
        }
    }
}
