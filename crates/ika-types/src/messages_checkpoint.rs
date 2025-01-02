// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use sui_types::base_types::{
    random_object_ref, ExecutionData, VerifiedExecutionData
};
use crate::committee::{EpochId, ProtocolVersion, StakeUnit};
use crate::crypto::{
    default_hash, AccountKeyPair, AggregateAuthoritySignature, AuthoritySignInfo,
    AuthoritySignInfoTrait, AuthorityStrongQuorumSignInfo, AuthorityName
};
use sui_types::crypto::{
    get_key_pair, RandomnessRound
};
use crate::digests::{Digest, ActionDigest};
use sui_types::effects::{TestEffectsBuilder, TransactionEffectsAPI};
use crate::error::IkaResult;
use sui_types::gas::GasCostSummary;
use crate::message_envelope::{Envelope, Message, TrustedEnvelope, VerifiedEnvelope};
use sui_types::signature::GenericSignature;
use sui_types::storage::ReadStore;
use crate::ika_serde::AsProtocolVersion;
use sui_types::sui_serde::BigInt;
use sui_types::sui_serde::Readable;
use sui_types::transaction::{Transaction, TransactionData};
use crate::{committee::Committee, error::IkaError};
use anyhow::Result;
use fastcrypto::hash::MultisetHash;
use mysten_metrics::histogram::Histogram as MystenHistogram;
use once_cell::sync::OnceCell;
use prometheus::Histogram;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use crate::intent::{Intent, IntentMessage, IntentScope};
use std::fmt::{Debug, Display, Formatter};
use std::slice::Iter;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use ika_protocol_config::ProtocolConfig;
use tap::TapFallible;
use tracing::warn;

pub use crate::digests::CheckpointContentsDigest;
pub use crate::digests::CheckpointMessageDigest;
use crate::action::{ActionData, ActionKind};

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
    pub actions: Vec<ActionData>,
    
    /// CheckpointSummary is not a revolvable structure - it must be readable by any version of the
    /// code. Therefore, in order to allow extensions to be added to CheckpointSummary, we allow
    /// opaque data to be added to checkpoints which can be deserialized based on the current
    /// protocol version.
    ///
    /// This is implemented with BCS-serialized `CheckpointVersionSpecificData`.
    pub version_specific_data: Vec<u8>,
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
        protocol_config: &ProtocolConfig,
        epoch: EpochId,
        sequence_number: CheckpointSequenceNumber,
        messages: Vec<ActionData>,
        timestamp_ms: CheckpointTimestamp,
        randomness_rounds: Vec<RandomnessRound>,
    ) -> CheckpointMessage {
        let version_specific_data = match protocol_config
            .checkpoint_summary_version_specific_data_as_option()
        {
            None | Some(0) => Vec::new(),
            Some(1) => bcs::to_bytes(&CheckpointVersionSpecificData::V1(
                CheckpointVersionSpecificDataV1 { randomness_rounds },
            ))
            .expect("version specific data should serialize"),
            _ => unimplemented!("unrecognized version_specific_data version for CheckpointSummary"),
        };

        Self {
            epoch,
            sequence_number,
            actions: messages,
            timestamp_ms,
            version_specific_data,
            //checkpoint_commitments: Default::default(),
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

    pub fn report_checkpoint_age(&self, metrics: &Histogram, metrics_deprecated: &MystenHistogram) {
        SystemTime::now()
            .duration_since(self.timestamp())
            .map(|latency| {
                metrics.observe(latency.as_secs_f64());
                metrics_deprecated.report(latency.as_millis() as u64);
            })
            .tap_err(|err| {
                warn!(
                    checkpoint_seq = self.sequence_number,
                    "unable to compute checkpoint age: {}", err
                )
            })
            .ok();
    }

    pub fn is_last_checkpoint_of_epoch(&self) -> bool {
        self.actions.iter().any(|a| matches!(a.kind(), ActionKind::EndOfEpochTransaction(_)))
    }

    pub fn version_specific_data(
        &self,
        config: &ProtocolConfig,
    ) -> Result<Option<CheckpointVersionSpecificData>> {
        match config.checkpoint_summary_version_specific_data_as_option() {
            None | Some(0) => Ok(None),
            Some(1) => Ok(Some(bcs::from_bytes(&self.version_specific_data)?)),
            _ => unimplemented!("unrecognized version_specific_data version in CheckpointSummary"),
        }
    }
}

impl Display for CheckpointMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CheckpointSummary {{ epoch: {:?}, seq: {:?}",
            self.epoch,
            self.sequence_number,
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

pub type VerifiedCheckpointMessage = VerifiedEnvelope<CheckpointMessage, AuthorityStrongQuorumSignInfo>;
pub type TrustedCheckpointMessage = TrustedEnvelope<CheckpointMessage, AuthorityStrongQuorumSignInfo>;

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
        self.checkpoint_message.verify_authority_signatures(committee)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum CheckpointContents {
    V1(CheckpointContentsV1),
}

/// CheckpointContents are the transactions included in an upcoming checkpoint.
/// They must have already been causally ordered. Since the causal order algorithm
/// is the same among validators, we expect all honest validators to come up with
/// the same order for each checkpoint content.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CheckpointContentsV1 {
    #[serde(skip)]
    digest: OnceCell<CheckpointContentsDigest>,

    messages: Vec<ActionDigest>,
}

impl CheckpointContents {
    pub fn new_with_digests_and_signatures<T>(
        contents: T,
    ) -> Self
    where
        T: IntoIterator<Item =ActionDigest>,
    {
        let transactions: Vec<_> = contents.into_iter().collect();
        Self::V1(CheckpointContentsV1 {
            digest: Default::default(),
            messages: transactions,
        })
    }

    #[cfg(any(test, feature = "test-utils"))]
    pub fn new_with_digests_only_for_tests<T>(contents: T) -> Self
    where
        T: IntoIterator<Item =ActionDigest>,
    {
        let transactions: Vec<_> = contents.into_iter().collect();
        Self::V1(CheckpointContentsV1 {
            digest: Default::default(),
            messages: transactions,
        })
    }

    fn as_v1(&self) -> &CheckpointContentsV1 {
        match self {
            Self::V1(v) => v,
        }
    }

    fn into_v1(self) -> CheckpointContentsV1 {
        match self {
            Self::V1(v) => v,
        }
    }

    pub fn iter(&self) -> Iter<'_, ActionDigest> {
        self.as_v1().messages.iter()
    }

    pub fn into_inner(self) -> Vec<ActionDigest> {
        self.into_v1().messages
    }

    pub fn inner(&self) -> &[ActionDigest] {
        &self.as_v1().messages
    }

    pub fn size(&self) -> usize {
        self.as_v1().messages.len()
    }

    pub fn digest(&self) -> &CheckpointContentsDigest {
        self.as_v1()
            .digest
            .get_or_init(|| CheckpointContentsDigest::new(default_hash(self)))
    }
}

/// Same as CheckpointContents, but contains full contents of all Transactions and
/// TransactionEffects associated with the checkpoint.
// NOTE: This data structure is used for state sync of checkpoints. Therefore, we attempt
// to estimate its size in CheckpointBuilder in order to limit the maximum serialized
// size of a checkpoint sent over the network. If this struct is modified,
// CheckpointBuilder::split_checkpoint_chunks should also be updated accordingly.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FullCheckpointContents {
    messages: Vec<ActionData>,
}

impl FullCheckpointContents {
    pub fn iter(&self) -> Iter<'_, ActionData> {
        self.messages.iter()
    }

    /// Verifies that this checkpoint's digest matches the given digest, and that all internal
    /// Transaction and TransactionEffects digests are consistent.
    pub fn verify_digests(&self, digest: CheckpointContentsDigest) -> Result<()> {
        let self_digest = *self.checkpoint_contents().digest();
        fp_ensure!(
            digest == self_digest,
            anyhow::anyhow!(
                "checkpoint contents digest {self_digest} does not match expected digest {digest}"
            )
        );
        Ok(())
    }

    pub fn checkpoint_contents(&self) -> CheckpointContents {
        CheckpointContents::V1(CheckpointContentsV1 {
            digest: Default::default(),
            messages: self.messages.iter().map(|tx| tx.digest()).collect(),
        })
    }

    pub fn into_checkpoint_contents(self) -> CheckpointContents {
        CheckpointContents::V1(CheckpointContentsV1 {
            digest: Default::default(),
            messages: self
                .messages
                .into_iter()
                .map(|tx| tx.digest())
                .collect(),
        })
    }

    pub fn size(&self) -> usize {
        self.messages.len()
    }
}

impl IntoIterator for FullCheckpointContents {
    type Item = ActionData;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.messages.into_iter()
    }
}

/// Holds data in CheckpointSummary that is serialized into the `version_specific_data` field.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckpointVersionSpecificData {
    V1(CheckpointVersionSpecificDataV1),
}

impl CheckpointVersionSpecificData {
    pub fn as_v1(&self) -> &CheckpointVersionSpecificDataV1 {
        match self {
            Self::V1(v) => v,
        }
    }

    pub fn into_v1(self) -> CheckpointVersionSpecificDataV1 {
        match self {
            Self::V1(v) => v,
        }
    }

    pub fn empty_for_tests() -> CheckpointVersionSpecificData {
        CheckpointVersionSpecificData::V1(CheckpointVersionSpecificDataV1 {
            randomness_rounds: Vec::new(),
        })
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckpointVersionSpecificDataV1 {
    /// Lists the rounds for which RandomnessStateUpdate transactions are present in the checkpoint.
    pub randomness_rounds: Vec<RandomnessRound>,
}

#[cfg(test)]
#[cfg(feature = "test-utils")]
mod tests {
    use crate::digests::{ConsensusCommitDigest, ActionDigest, TransactionEffectsDigest};
    use crate::action::VerifiedTransaction;
    use fastcrypto::traits::KeyPair;
    use rand::prelude::StdRng;
    use rand::SeedableRng;

    use super::*;
    use crate::utils::make_committee_key;

    // TODO use the file name as a seed
    const RNG_SEED: [u8; 32] = [
        21, 23, 199, 200, 234, 250, 252, 178, 94, 15, 202, 178, 62, 186, 88, 137, 233, 192, 130,
        157, 179, 179, 65, 9, 31, 249, 221, 123, 225, 112, 199, 247,
    ];

    #[test]
    fn test_signed_checkpoint() {
        let mut rng = StdRng::from_seed(RNG_SEED);
        let (keys, committee) = make_committee_key(&mut rng);
        let (_, committee2) = make_committee_key(&mut rng);

        let set = CheckpointContents::new_with_digests_only_for_tests([ActionDigest::random()]);

        // TODO: duplicated in a test below.

        let signed_checkpoints: Vec<_> = keys
            .iter()
            .map(|k| {
                let name = k.public().into();

                SignedCheckpointMessage::new(
                    committee.epoch,
                    CheckpointMessage::new(
                        &ProtocolConfig::get_for_max_version_UNSAFE(),
                        committee.epoch,
                        1,
                        0,
                        &set,
                        None,
                        GasCostSummary::default(),
                        None,
                        0,
                        Vec::new(),
                    ),
                    k,
                    name,
                )
            })
            .collect();

        signed_checkpoints.iter().for_each(|c| {
            c.verify_authority_signatures(&committee)
                .expect("signature ok")
        });

        // fails when not signed by member of committee
        signed_checkpoints
            .iter()
            .for_each(|c| assert!(c.verify_authority_signatures(&committee2).is_err()));
    }

    #[test]
    fn test_certified_checkpoint() {
        let mut rng = StdRng::from_seed(RNG_SEED);
        let (keys, committee) = make_committee_key(&mut rng);

        let set = CheckpointContents::new_with_digests_only_for_tests([ActionDigest::random()]);

        let summary = CheckpointMessage::new(
            &ProtocolConfig::get_for_max_version_UNSAFE(),
            committee.epoch,
            1,
            0,
            &set,
            None,
            GasCostSummary::default(),
            None,
            0,
            Vec::new(),
        );

        let sign_infos: Vec<_> = keys
            .iter()
            .map(|k| {
                let name = k.public().into();

                SignedCheckpointMessage::sign(committee.epoch, &summary, k, name)
            })
            .collect();

        let checkpoint_cert =
            CertifiedCheckpointMessage::new(summary, sign_infos, &committee).expect("Cert is OK");

        // Signature is correct on proposal, and with same transactions
        assert!(checkpoint_cert
            .verify_with_contents(&committee, Some(&set))
            .is_ok());

        // Make a bad proposal
        let signed_checkpoints: Vec<_> = keys
            .iter()
            .map(|k| {
                let name = k.public().into();
                let set = CheckpointContents::new_with_digests_only_for_tests([
                    ActionDigest::random(),
                ]);

                SignedCheckpointMessage::new(
                    committee.epoch,
                    CheckpointMessage::new(
                        &ProtocolConfig::get_for_max_version_UNSAFE(),
                        committee.epoch,
                        1,
                        0,
                        &set,
                        None,
                        GasCostSummary::default(),
                        None,
                        0,
                        Vec::new(),
                    ),
                    k,
                    name,
                )
            })
            .collect();

        let summary = signed_checkpoints[0].data().clone();
        let sign_infos = signed_checkpoints
            .into_iter()
            .map(|v| v.into_sig())
            .collect();
        assert!(
            CertifiedCheckpointMessage::new(summary, sign_infos, &committee)
                .unwrap()
                .verify_authority_signatures(&committee)
                .is_err()
        )
    }

    // Generate a CheckpointSummary from the input transaction digest. All the other fields in the generated
    // CheckpointSummary will be the same. The generated CheckpointSummary can be used to test how input
    // transaction digest affects CheckpointSummary.
    fn generate_test_checkpoint_summary_from_digest(
        digest: ActionDigest,
    ) -> CheckpointMessage {
        CheckpointMessage::new(
            &ProtocolConfig::get_for_max_version_UNSAFE(),
            1,
            2,
            10,
            &CheckpointContents::new_with_digests_only_for_tests([ActionDigest::new(
                digest,
            )]),
            None,
            GasCostSummary::default(),
            None,
            100,
            Vec::new(),
        )
    }

    // Tests that ConsensusCommitPrologue with different consensus commit digest will result in different checkpoint content.
    #[test]
    fn test_checkpoint_summary_with_different_consensus_digest() {
        // First, tests that same consensus commit digest will produce the same checkpoint content.
        {
            let t1 = VerifiedTransaction::new_consensus_commit_prologue_v3(
                1,
                2,
                100,
                ConsensusCommitDigest::default(),
                Vec::new(),
            );
            let t2 = VerifiedTransaction::new_consensus_commit_prologue_v3(
                1,
                2,
                100,
                ConsensusCommitDigest::default(),
                Vec::new(),
            );
            let c1 = generate_test_checkpoint_summary_from_digest(*t1.digest());
            let c2 = generate_test_checkpoint_summary_from_digest(*t2.digest());
            assert_eq!(c1.digest(), c2.digest());
        }

        // Next, tests that different consensus commit digests will produce the different checkpoint contents.
        {
            let t1 = VerifiedTransaction::new_consensus_commit_prologue_v3(
                1,
                2,
                100,
                ConsensusCommitDigest::default(),
                Vec::new(),
            );
            let t2 = VerifiedTransaction::new_consensus_commit_prologue_v3(
                1,
                2,
                100,
                ConsensusCommitDigest::random(),
                Vec::new(),
            );
            let c1 = generate_test_checkpoint_summary_from_digest(*t1.digest());
            let c2 = generate_test_checkpoint_summary_from_digest(*t2.digest());
            assert_ne!(c1.digest(), c2.digest());
        }
    }
}
