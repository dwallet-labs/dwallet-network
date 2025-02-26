// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::committee::{Committee, EpochId, ProtocolVersion};
use crate::crypto::{
    default_hash, AuthoritySignInfo, AuthoritySignInfoTrait, AuthoritySignature,
    AuthorityStrongQuorumSignInfo, DefaultHash, EmptySignInfo, Signer, ToFromBytes,
};
use crate::digests::MessageDigest;
use crate::messages_consensus::MovePackageDigest;
use crate::messages_dwallet_mpc::SessionInfo;
use dwallet_mpc_types::dwallet_mpc::{DWalletMPCNetworkKeyScheme, NetworkDecryptionKeyShares};
use enum_dispatch::enum_dispatch;
use fastcrypto::{encoding::Base64, hash::HashFunction};
use ika_protocol_config::ProtocolConfig;
use itertools::Either;
use move_core_types::{ident_str, identifier};
use move_core_types::{identifier::Identifier, language_storage::TypeTag};
use nonempty::{nonempty, NonEmpty};
use serde::{Deserialize, Serialize};
use shared_crypto::intent::{Intent, IntentMessage, IntentScope};
use std::fmt::Write;
use std::fmt::{Debug, Display, Formatter};
use std::iter::once;
use std::sync::Arc;
use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    hash::Hash,
    iter,
};
use strum::IntoStaticStr;
use sui_types::authenticator_state::ActiveJwk;
use sui_types::crypto::{RandomnessRound, Signature};
use sui_types::digests::ConsensusCommitDigest;
use sui_types::message_envelope::{Envelope, Message, TrustedEnvelope, VerifiedEnvelope};
use sui_types::messages_checkpoint::CheckpointTimestamp;
use sui_types::messages_consensus::TimestampMs;
use sui_types::{base_types::*, error::*};
use tap::Pipe;
use tracing::trace;

/// EndOfEpochMessageKind
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, IntoStaticStr)]
pub enum EndOfEpochMessageKind {
    AdvanceEpoch {
        /// The next (to become) epoch ID.
        epoch: EpochId,
        /// The protocol version in effect in the new epoch.
        protocol_version: ProtocolVersion,
        /// Unix timestamp when epoch started
        epoch_start_timestamp_ms: u64,
        /// Ika move packages (package id) to be upgraded and their
        /// move packages digest of the new version
        move_packages: Vec<(ObjectID, MovePackageDigest)>,
        // to version this struct, do not add new fields. Instead, add a AdvanceEpoch to
        // MessageKind.
    },
}

impl EndOfEpochMessageKind {
    pub fn new_advance_epoch(
        next_epoch: EpochId,
        protocol_version: ProtocolVersion,
        epoch_start_timestamp_ms: u64,
        move_packages: Vec<(ObjectID, MovePackageDigest)>,
    ) -> Self {
        Self::AdvanceEpoch {
            epoch: next_epoch,
            protocol_version,
            epoch_start_timestamp_ms,
            move_packages,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct DKGFirstRoundOutput {
    pub dwallet_id: Vec<u8>,
    pub output: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct DKGSecondRoundOutput {
    pub dwallet_id: Vec<u8>,
    pub output: Vec<u8>,
    pub encrypted_centralized_secret_share_and_proof: Vec<u8>,
    pub encryption_key_address: Vec<u8>,
    pub rejected: bool,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct PresignOutput {
    pub dwallet_id: Vec<u8>,
    pub session_id: Vec<u8>,
    pub presign: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct SignOutput {
    pub initiating_user_address: Vec<u8>,
    pub batch_session_id: Vec<u8>,
    pub signatures: Vec<u8>,
    pub dwallet_id: Vec<u8>,
    pub is_future_sign: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct EncryptedUserShareOutput {
    pub initiating_user_address: Vec<u8>,
    pub dwallet_id: Vec<u8>,
    pub encrypted_centralized_secret_share_and_proof: Vec<u8>,
    pub encryption_key_id: Vec<u8>,
    pub session_id: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct EncryptionKeyVerificationOutput {
    pub initiating_user_address: Vec<u8>,
    pub session_id: Vec<u8>,
    pub key_signer_public_key: Vec<u8>,
    pub encryption_key: Vec<u8>,
    pub encryption_key_signature: Vec<u8>,
    pub encryption_key_scheme: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct PartialSignatureVerificationOutput {
    pub initiating_user_address: Vec<u8>,
    pub session_id: Vec<u8>,
    pub dwallet_id: Vec<u8>,
    pub dwallet_decentralized_public_output: Vec<u8>,
    pub dwallet_cap_id: Vec<u8>,
    pub dwallet_mpc_network_decryption_key_version: Vec<u8>,
    pub signature_data: Vec<u8>,
    pub messages: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, IntoStaticStr)]
pub enum MessageKind {
    InitiateProcessMidEpoch,

    /// A list of message to be run at the
    /// end of the epoch.
    EndOfEpoch(Vec<EndOfEpochMessageKind>),

    /// Test message for checkpoints.
    TestMessage(u32, u64),
    // .. more action types go here
    DwalletDKGFirstRoundOutput(DKGFirstRoundOutput),
    DwalletDKGSecondRoundOutput(DKGSecondRoundOutput),
    DwalletSign(SignOutput),
    DwalletEncryptedUserShare(EncryptedUserShareOutput),
    DwalletEncryptionKeyVerification(EncryptionKeyVerificationOutput),
    DwalletPartialSignatureVerificationOutput(PartialSignatureVerificationOutput),
    DwalletMPCNetworkDKGOutput(DWalletMPCNetworkKeyScheme, NetworkDecryptionKeyShares),
    DwalletPresign(PresignOutput),
}

impl MessageKind {
    pub fn is_end_of_epoch_tx(&self) -> bool {
        matches!(self, MessageKind::EndOfEpoch(_))
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::InitiateProcessMidEpoch => "InitiateProcessMidEpoch",
            Self::EndOfEpoch(_) => "EndOfEpoch",
            Self::TestMessage(_, _) => "TestMessage",
            MessageKind::DwalletMPCNetworkDKGOutput(_, _) => "DwalletMPCNetworkDKGOutput",
            MessageKind::DwalletDKGFirstRoundOutput(_) => "DwalletDKGFirstRoundOutput",
            MessageKind::DwalletDKGSecondRoundOutput(_) => "DwalletDKGSecondRoundOutput",
            MessageKind::DwalletPresign(_) => "DwalletPresign",
            MessageKind::DwalletSign(_) => "DwalletSign",
            MessageKind::DwalletEncryptedUserShare(_) => "DwalletEncryptedUserShare",
            MessageKind::DwalletEncryptionKeyVerification(_) => "DwalletEncryptionKeyVerification",
            MessageKind::DwalletPartialSignatureVerificationOutput(_) => {
                "DwalletPartialSignatureVerificationOutput"
            }
        }
    }

    pub fn new_initiate_process_mid_epoch_message() -> Self {
        Self::InitiateProcessMidEpoch
    }

    pub fn new_end_of_epoch_message(messages: Vec<EndOfEpochMessageKind>) -> Self {
        Self::EndOfEpoch(messages)
    }

    pub fn digest(&self) -> MessageDigest {
        MessageDigest::new(default_hash(self))
    }
}

impl Display for MessageKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();
        match &self {
            Self::InitiateProcessMidEpoch => {
                writeln!(writer, "MessageKind : InitiateProcessMidEpoch")?;
            }
            Self::EndOfEpoch(_) => {
                writeln!(writer, "MessageKind : EndOfEpoch")?;
            }
            Self::TestMessage(authority, num) => {
                writeln!(
                    writer,
                    "MessageKind : TestMessage authority: {}, num: {}",
                    authority, num
                )?;
            }
            MessageKind::DwalletMPCNetworkDKGOutput(key_scheme, _) => {
                writeln!(
                    writer,
                    "MessageKind : DwalletMPCNetworkDKGOutput {:?}",
                    key_scheme
                )?;
            }
            MessageKind::DwalletDKGFirstRoundOutput(_) => {
                writeln!(writer, "MessageKind : DwalletDKGFirstRoundOutput")?;
            }
            MessageKind::DwalletDKGSecondRoundOutput(_) => {
                writeln!(writer, "MessageKind : DwalletDKGSecondRoundOutput")?;
            }
            MessageKind::DwalletPresign(_) => {
                writeln!(writer, "MessageKind : DwalletPresign")?;
            }
            MessageKind::DwalletSign(_) => {
                writeln!(writer, "MessageKind : DwalletSign")?;
            }
            MessageKind::DwalletEncryptedUserShare(_) => {
                writeln!(writer, "MessageKind : DwalletEncryptedUserShare")?;
            }
            MessageKind::DwalletEncryptionKeyVerification(_) => {
                writeln!(writer, "MessageKind : DwalletEncryptionKeyVerification")?;
            }
            MessageKind::DwalletPartialSignatureVerificationOutput(_) => {
                writeln!(
                    writer,
                    "MessageKind : DwalletPartialSignatureVerificationOutput"
                )?;
            }
        }
        write!(f, "{}", writer)
    }
}

// #[enum_dispatch(MessageDataAPI)]
// #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
// pub enum MessageKind {
//     V1(MessageDataV1),
//     // When new variants are introduced, it is important that we check version support
//     // in the validity_check function based on the protocol config.
// }
//
// #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
// pub struct MessageDataV1 {
//     pub kind: MessageKind,
//     // pub sender: IkaAddress,
//     // pub gas_data: GasData,
//     // pub expiration: TransactionExpiration,
// }
//
// impl MessageKind {
//     pub fn new(
//         kind: MessageKind
//     ) -> Self {
//         MessageKind::V1(MessageDataV1 {
//             kind,
//         })
//     }
//
//     pub fn new_initiate_process_mid_epoch_message() -> Self {
//         Self::new(MessageKind::InitiateProcessMidEpoch)
//     }
//
//     pub fn new_end_of_epoch_message(messages: Vec<EndOfEpochMessageKind>) -> Self {
//         Self::new(MessageKind::EndOfEpoch(messages))
//     }
//
//     pub fn kind(&self) -> &MessageKind {
//         match self {
//             MessageKind::V1(MessageDataV1 { kind }) => kind,
//         }
//     }
//
//     pub fn message_version(&self) -> u64 {
//         match self {
//             MessageKind::V1(_) => 1,
//         }
//     }
//
//     pub fn digest(&self) -> MessageDigest {
//         MessageDigest::new(default_hash(self))
//     }
// }
//
// #[enum_dispatch]
// pub trait MessageDataAPI {
//     // Note: this implies that SingleMessageKind itself must be versioned, so that it can be
//     // shared across versions. This will be easy to do since it is already an enum.
//     fn kind(&self) -> &MessageKind;
//
//     // Used by programmable_transaction_builder
//     fn kind_mut(&mut self) -> &mut MessageKind;
//
//     // kind is moved out of often enough that this is worth it to special case.
//     fn into_kind(self) -> MessageKind;
//
//     /// returns true if the transaction is one that is specially sequenced to run at the very end
//     /// of the epoch
//     fn is_end_of_epoch_tx(&self) -> bool;
// }
//
// impl MessageDataAPI for MessageDataV1 {
//     fn kind(&self) -> &MessageKind {
//         &self.kind
//     }
//
//     fn kind_mut(&mut self) -> &mut MessageKind {
//         &mut self.kind
//     }
//
//     fn into_kind(self) -> MessageKind {
//         self.kind
//     }
//
//     fn is_end_of_epoch_tx(&self) -> bool {
//         matches!(
//             self.kind,
//             MessageKind::EndOfEpoch(_)
//         )
//     }
// }
//
// impl MessageDataV1 {}
