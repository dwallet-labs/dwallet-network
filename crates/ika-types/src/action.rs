// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use sui_types::{base_types::*, error::*};
use sui_types::authenticator_state::ActiveJwk;
use crate::committee::{Committee, EpochId, ProtocolVersion};
use crate::crypto::{
    default_hash, AuthoritySignInfo, AuthoritySignInfoTrait, AuthoritySignature,
    AuthorityStrongQuorumSignInfo, DefaultHash, EmptySignInfo,
    Signer, ToFromBytes,
};
use sui_types::crypto::{
    RandomnessRound, Signature
};
use crate::digests::{ActionDigest};
use sui_types::message_envelope::{Envelope, Message, TrustedEnvelope, VerifiedEnvelope};
use sui_types::messages_checkpoint::CheckpointTimestamp;
use enum_dispatch::enum_dispatch;
use fastcrypto::{encoding::Base64, hash::HashFunction};
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
use sui_types::digests::ConsensusCommitDigest;
use sui_types::messages_consensus::TimestampMs;
use ika_protocol_config::ProtocolConfig;
use tap::Pipe;
use tracing::trace;
use crate::messages_consensus::MovePackageDigest;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct AdvanceEpoch {
    /// The next (to become) epoch ID.
    pub epoch: EpochId,
    /// The protocol version in effect in the new epoch.
    pub protocol_version: ProtocolVersion,
    /// Unix timestamp when epoch started
    pub epoch_start_timestamp_ms: u64,
    /// Ika move packages (package id) to be upgraded and their
    /// move packages digest of the new version
    pub move_packages: Vec<(ObjectID, MovePackageDigest)>,
    // to version this struct, do not add new fields. Instead, add a AdvanceEpoch to
    // MessageKind.
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, IntoStaticStr)]
pub enum ActionKind {
    // Test message for checkpoints.
    TestMessage(u32, u64),

    /// A list of message to be run at the
    /// end of the epoch.
    EndOfEpochTransaction(Vec<EndOfEpochMessageKind>),

    // .. more action types go here
}

/// EndOfEpochMessageKind
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, IntoStaticStr)]
pub enum EndOfEpochMessageKind {
    AdvanceEpoch(AdvanceEpoch),
}

impl EndOfEpochMessageKind {
    pub fn new_advance_epoch(
        next_epoch: EpochId,
        protocol_version: ProtocolVersion,
        epoch_start_timestamp_ms: u64,
        move_packages: Vec<(ObjectID, MovePackageDigest)>,
    ) -> Self {
        Self::AdvanceEpoch(AdvanceEpoch {
            epoch: next_epoch,
            protocol_version,
            epoch_start_timestamp_ms,
            move_packages,
        })
    }
}

impl ActionKind {
    pub fn is_end_of_epoch_tx(&self) -> bool {
        matches!(
            self,
            ActionKind::EndOfEpochTransaction(_)
        )
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::EndOfEpochTransaction(_) => "EndOfEpochTransaction",
            Self::TestMessage(_,_) => "TestMessage",
        }
    }
}

impl Display for ActionKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();
        match &self {
            Self::EndOfEpochTransaction(_) => {
                writeln!(writer, "Transaction Kind : End of Epoch Transaction")?;
            }
            Self::TestMessage(authority, num) => {
                writeln!(writer, "TestMessage authority: {}, num: {}", authority, num)?;
            }
        }
        write!(f, "{}", writer)
    }
}

#[enum_dispatch(ActionDataAPI)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum ActionData {
    V1(ActionDataV1),
    // When new variants are introduced, it is important that we check version support
    // in the validity_check function based on the protocol config.
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct ActionDataV1 {
    pub kind: ActionKind,
    // pub sender: IkaAddress,
    // pub gas_data: GasData,
    // pub expiration: TransactionExpiration,
}

impl ActionData {
    pub fn new(
        kind: ActionKind
    ) -> Self {
        ActionData::V1(ActionDataV1 {
            kind,
        })
    }

    pub fn new_end_of_epoch_message(messages: Vec<EndOfEpochMessageKind>) -> Self {
        Self::new(ActionKind::EndOfEpochTransaction(messages))
    }

    pub fn kind(&self) -> &ActionKind {
        match self {
            ActionData::V1(ActionDataV1 { kind }) => kind,
        }
    }

    pub fn message_version(&self) -> u64 {
        match self {
            ActionData::V1(_) => 1,
        }
    }
    
    pub fn digest(&self) -> ActionDigest {
        ActionDigest::new(default_hash(self))
    }
}

#[enum_dispatch]
pub trait ActionDataAPI {
    // Note: this implies that SingleMessageKind itself must be versioned, so that it can be
    // shared across versions. This will be easy to do since it is already an enum.
    fn kind(&self) -> &ActionKind;

    // Used by programmable_transaction_builder
    fn kind_mut(&mut self) -> &mut ActionKind;

    // kind is moved out of often enough that this is worth it to special case.
    fn into_kind(self) -> ActionKind;

    /// returns true if the transaction is one that is specially sequenced to run at the very end
    /// of the epoch
    fn is_end_of_epoch_tx(&self) -> bool;
}

impl ActionDataAPI for ActionDataV1 {
    fn kind(&self) -> &ActionKind {
        &self.kind
    }

    fn kind_mut(&mut self) -> &mut ActionKind {
        &mut self.kind
    }

    fn into_kind(self) -> ActionKind {
        self.kind
    }

    fn is_end_of_epoch_tx(&self) -> bool {
        matches!(
            self.kind,
            ActionKind::EndOfEpochTransaction(_)
        )
    }
}

impl ActionDataV1 {}
