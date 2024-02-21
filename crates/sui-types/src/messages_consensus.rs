// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::base_types::ConciseableName;
use crate::base_types::{AuthorityName, ObjectRef, TransactionDigest};
use crate::digests::ConsensusCommitDigest;
use crate::messages_checkpoint::{
    CheckpointSequenceNumber, CheckpointSignatureMessage, CheckpointTimestamp,
};
use crate::transaction::CertifiedTransaction;
use byteorder::{BigEndian, ReadBytesExt};
use fastcrypto_zkp::bn254::zk_login::{JwkId, JWK};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};
use sui_protocol_config::SupportedProtocolVersions;

/// Only commit_timestamp_ms is passed to the move call currently.
/// However we include epoch and round to make sure each ConsensusCommitPrologue has a unique tx digest.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct ConsensusCommitPrologue {
    /// Epoch of the commit prologue transaction
    pub epoch: u64,
    /// Consensus round of the commit
    pub round: u64,
    /// Unix timestamp from consensus
    pub commit_timestamp_ms: CheckpointTimestamp,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct ConsensusCommitPrologueV2 {
    /// Epoch of the commit prologue transaction
    pub epoch: u64,
    /// Consensus round of the commit
    pub round: u64,
    /// Unix timestamp from consensus
    pub commit_timestamp_ms: CheckpointTimestamp,
    /// Digest of consensus output
    pub consensus_commit_digest: ConsensusCommitDigest,
}

// In practice, JWKs are about 500 bytes of json each, plus a bit more for the ID.
// 4096 should give us plenty of space for any imaginable JWK while preventing DoSes.
static MAX_TOTAL_JWK_SIZE: usize = 4096;

pub fn check_total_jwk_size(id: &JwkId, jwk: &JWK) -> bool {
    id.iss.len() + id.kid.len() + jwk.kty.len() + jwk.alg.len() + jwk.e.len() + jwk.n.len()
        <= MAX_TOTAL_JWK_SIZE
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConsensusTransaction {
    /// Encodes an u64 unique tracking id to allow us trace a message between Sui and Narwhal.
    /// Use an byte array instead of u64 to ensure stable serialization.
    pub tracking_id: [u8; 8],
    pub kind: ConsensusTransactionKind,
}

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum ConsensusTransactionKey {
    Certificate(TransactionDigest),
    CheckpointSignature(AuthorityName, CheckpointSequenceNumber),
    EndOfPublish(AuthorityName),
    CapabilityNotification(AuthorityName, u64 /* generation */),
    // Key must include both id and jwk, because honest validators could be given multiple jwks for
    // the same id by malfunctioning providers.
    NewJWKFetched(Box<(AuthorityName, JwkId, JWK)>),
}

impl Debug for ConsensusTransactionKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Certificate(digest) => write!(f, "Certificate({:?})", digest),
            Self::CheckpointSignature(name, seq) => {
                write!(f, "CheckpointSignature({:?}, {:?})", name.concise(), seq)
            }
            Self::EndOfPublish(name) => write!(f, "EndOfPublish({:?})", name.concise()),
            Self::CapabilityNotification(name, generation) => write!(
                f,
                "CapabilityNotification({:?}, {:?})",
                name.concise(),
                generation
            ),
            Self::NewJWKFetched(key) => {
                let (authority, id, jwk) = &**key;
                write!(
                    f,
                    "NewJWKFetched({:?}, {:?}, {:?})",
                    authority.concise(),
                    id,
                    jwk
                )
            }
        }
    }
}

/// Used to advertise capabilities of each authority via narwhal. This allows validators to
/// negotiate the creation of the ChangeEpoch transaction.
#[derive(Serialize, Deserialize, Clone, Hash)]
pub struct AuthorityCapabilities {
    /// Originating authority - must match narwhal transaction source.
    pub authority: AuthorityName,
    /// Generation number set by sending authority. Used to determine which of multiple
    /// AuthorityCapabilities messages from the same authority is the most recent.
    ///
    /// (Currently, we just set this to the current time in milliseconds since the epoch, but this
    /// should not be interpreted as a timestamp.)
    pub generation: u64,

    /// ProtocolVersions that the authority supports.
    pub supported_protocol_versions: SupportedProtocolVersions,

    /// The ObjectRefs of all versions of system packages that the validator possesses.
    /// Used to determine whether to do a framework/movestdlib upgrade.
    pub available_system_packages: Vec<ObjectRef>,
}

impl Debug for AuthorityCapabilities {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthorityCapabilities")
            .field("authority", &self.authority.concise())
            .field("generation", &self.generation)
            .field(
                "supported_protocol_versions",
                &self.supported_protocol_versions,
            )
            .field("available_system_packages", &self.available_system_packages)
            .finish()
    }
}

impl AuthorityCapabilities {
    pub fn new(
        authority: AuthorityName,
        supported_protocol_versions: SupportedProtocolVersions,
        available_system_packages: Vec<ObjectRef>,
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
            supported_protocol_versions,
            available_system_packages,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ConsensusTransactionKind {
    UserTransaction(Box<CertifiedTransaction>),
    CheckpointSignature(Box<CheckpointSignatureMessage>),
    EndOfPublish(AuthorityName),
    CapabilityNotification(AuthorityCapabilities),
    NewJWKFetched(AuthorityName, JwkId, JWK),
    RandomnessStateUpdate(u64, Vec<u8>),
}

impl ConsensusTransaction {
    pub fn new_certificate_message(
        authority: &AuthorityName,
        certificate: CertifiedTransaction,
    ) -> Self {
        let mut hasher = DefaultHasher::new();
        let tx_digest = certificate.digest();
        tx_digest.hash(&mut hasher);
        authority.hash(&mut hasher);
        let tracking_id = hasher.finish().to_le_bytes();
        Self {
            tracking_id,
            kind: ConsensusTransactionKind::UserTransaction(Box::new(certificate)),
        }
    }

    pub fn new_checkpoint_signature_message(data: CheckpointSignatureMessage) -> Self {
        let mut hasher = DefaultHasher::new();
        data.summary.auth_sig().signature.hash(&mut hasher);
        let tracking_id = hasher.finish().to_le_bytes();
        Self {
            tracking_id,
            kind: ConsensusTransactionKind::CheckpointSignature(Box::new(data)),
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

    pub fn new_capability_notification(capabilities: AuthorityCapabilities) -> Self {
        let mut hasher = DefaultHasher::new();
        capabilities.hash(&mut hasher);
        let tracking_id = hasher.finish().to_le_bytes();
        Self {
            tracking_id,
            kind: ConsensusTransactionKind::CapabilityNotification(capabilities),
        }
    }

    pub fn new_mysticeti_certificate(
        round: u64,
        offset: u64,
        certificate: CertifiedTransaction,
    ) -> Self {
        let mut hasher = DefaultHasher::new();
        let tx_digest = certificate.digest();
        tx_digest.hash(&mut hasher);
        round.hash(&mut hasher);
        offset.hash(&mut hasher);
        let tracking_id = hasher.finish().to_le_bytes();
        Self {
            tracking_id,
            kind: ConsensusTransactionKind::UserTransaction(Box::new(certificate)),
        }
    }

    pub fn new_jwk_fetched(authority: AuthorityName, id: JwkId, jwk: JWK) -> Self {
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);
        let tracking_id = hasher.finish().to_le_bytes();
        Self {
            tracking_id,
            kind: ConsensusTransactionKind::NewJWKFetched(authority, id, jwk),
        }
    }

    pub fn get_tracking_id(&self) -> u64 {
        (&self.tracking_id[..])
            .read_u64::<BigEndian>()
            .unwrap_or_default()
    }

    pub fn key(&self) -> ConsensusTransactionKey {
        match &self.kind {
            ConsensusTransactionKind::UserTransaction(cert) => {
                ConsensusTransactionKey::Certificate(*cert.digest())
            }
            ConsensusTransactionKind::CheckpointSignature(data) => {
                ConsensusTransactionKey::CheckpointSignature(
                    data.summary.auth_sig().authority,
                    data.summary.sequence_number,
                )
            }
            ConsensusTransactionKind::EndOfPublish(authority) => {
                ConsensusTransactionKey::EndOfPublish(*authority)
            }
            ConsensusTransactionKind::CapabilityNotification(cap) => {
                ConsensusTransactionKey::CapabilityNotification(cap.authority, cap.generation)
            }
            ConsensusTransactionKind::NewJWKFetched(authority, id, key) => {
                ConsensusTransactionKey::NewJWKFetched(Box::new((
                    *authority,
                    id.clone(),
                    key.clone(),
                )))
            }
            ConsensusTransactionKind::RandomnessStateUpdate(_, _) => {
                unreachable!("there should never be a RandomnessStateUpdate with SequencedConsensusTransactionKind::External")
            }
        }
    }

    pub fn is_user_certificate(&self) -> bool {
        matches!(self.kind, ConsensusTransactionKind::UserTransaction(_))
    }

    pub fn is_end_of_publish(&self) -> bool {
        matches!(self.kind, ConsensusTransactionKind::EndOfPublish(_))
    }
}

#[test]
fn test_jwk_compatibility() {
    // Ensure that the JWK and JwkId structs in fastcrypto do not change formats.
    // If this test breaks DO NOT JUST UPDATE THE EXPECTED BYTES. Instead, add a local JWK or
    // JwkId struct that mirrors the fastcrypto struct, use it in AuthenticatorStateUpdate, and
    // add Into/From as necessary.
    let jwk = JWK {
        kty: "a".to_string(),
        e: "b".to_string(),
        n: "c".to_string(),
        alg: "d".to_string(),
    };

    let expected_jwk_bytes = vec![1, 97, 1, 98, 1, 99, 1, 100];
    let jwk_bcs = bcs::to_bytes(&jwk).unwrap();
    assert_eq!(jwk_bcs, expected_jwk_bytes);

    let id = JwkId {
        iss: "abc".to_string(),
        kid: "def".to_string(),
    };

    let expected_id_bytes = vec![3, 97, 98, 99, 3, 100, 101, 102];
    let id_bcs = bcs::to_bytes(&id).unwrap();
    assert_eq!(id_bcs, expected_id_bytes);
}
