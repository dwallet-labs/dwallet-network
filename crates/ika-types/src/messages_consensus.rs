// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::crypto::AuthorityName;
use crate::messages_checkpoint::{CheckpointSequenceNumber, CheckpointSignatureMessage};
use crate::messages_dwallet_mpc::{
    DWalletMPCMessage, DWalletMPCMessageKey, MaliciousReport, SessionInfo,
};
use crate::supported_protocol_versions::{
    Chain, SupportedProtocolVersions, SupportedProtocolVersionsWithHashes,
};
use byteorder::{BigEndian, ReadBytesExt};
use dwallet_mpc_types::dwallet_mpc::{MPCMessageBuilder, MPCMessageSlice, MessageState};
use fastcrypto::error::FastCryptoResult;
use fastcrypto::groups::bls12381;
use fastcrypto_tbls::dkg_v1;
use fastcrypto_zkp::bn254::zk_login::{JwkId, JWK};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use sui_types::base_types::{
    ConciseableName, EpochId, ObjectID, ObjectRef, SequenceNumber, SuiAddress, TransactionDigest,
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
    /// The message sent between MPC parties in a dwallet MPC session.
    DWalletMPCMessage(DWalletMPCMessageKey),
    /// The output of a dwallet MPC session.
    /// The [`Vec<u8>`] is the data, the [`ObjectID`] is the session ID and the [`PeraAddress`] is the
    /// address of the initiating user.
    DWalletMPCOutput(MPCMessageSlice, ObjectID, AuthorityName),
    DWalletMPCSessionFailedWithMalicious(AuthorityName, MaliciousReport),
}

impl Debug for ConsensusTransactionKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CheckpointSignature(name, seq) => {
                write!(f, "CheckpointSignature({:?}, {:?})", name.concise(), seq)
            }
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ConsensusTransactionKind {
    CheckpointSignature(Box<CheckpointSignatureMessage>),
    DWalletMPCMessage(DWalletMPCMessage),
    DWalletMPCOutput(AuthorityName, SessionInfo, MPCMessageSlice),
    /// Sending Authority and its MaliciousReport.
    DWalletMPCSessionFailedWithMalicious(AuthorityName, MaliciousReport),
}

impl ConsensusTransaction {
    /// Create new consensus transactions with the message to be sent to the other MPC parties.
    pub fn new_dwallet_mpc_messages(
        authority: AuthorityName,
        message: Vec<u8>,
        session_id: ObjectID,
        round_number: usize,
        session_sequence_number: u64,
    ) -> Vec<Self> {
        // This size is arbitrary and might be changed in the future.
        let messages = MPCMessageBuilder::split(message, 120 * 1024);
        let messages = match messages.messages {
            MessageState::Incomplete(messages) => messages,
            MessageState::Complete(_) => panic!("should never happen "),
        };

        messages
            .iter()
            .map(|(sequence_number, message)| {
                let mut hasher = DefaultHasher::new();
                message.fragment.hash(&mut hasher);
                let tracking_id = hasher.finish().to_le_bytes();
                Self {
                    tracking_id,
                    kind: ConsensusTransactionKind::DWalletMPCMessage(DWalletMPCMessage {
                        message: message.clone(),
                        authority,
                        round_number,
                        session_id: session_id.clone(),
                        session_sequence_number,
                    }),
                }
            })
            .collect()
    }

    /// Create new consensus transactions with the output of the MPC session to be sent to the parties.
    pub fn new_dwallet_mpc_output(
        authority: AuthorityName,
        output: Vec<u8>,
        session_info: SessionInfo,
    ) -> Vec<Self> {
        let messages = MPCMessageBuilder::split(output.clone(), 120 * 1024);
        let messages = match messages.messages {
            MessageState::Incomplete(messages) => messages,
            MessageState::Complete(_) => panic!("should never happen "),
        };

        messages
            .iter()
            .map(|(_, message)| {
                let mut hasher = DefaultHasher::new();
                message.fragment.hash(&mut hasher);
                let tracking_id = hasher.finish().to_le_bytes();
                Self {
                    tracking_id,
                    kind: ConsensusTransactionKind::DWalletMPCOutput(
                        authority,
                        session_info.clone(),
                        message.clone(),
                    ),
                }
            })
            .collect()
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
            ConsensusTransactionKind::DWalletMPCMessage(message) => {
                ConsensusTransactionKey::DWalletMPCMessage(DWalletMPCMessageKey {
                    message: message.message.clone(),
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

    pub fn is_end_of_publish(&self) -> bool {
        matches!(self.kind, ConsensusTransactionKind::EndOfPublish(_))
    }
}
