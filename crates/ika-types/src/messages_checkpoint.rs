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

#[cfg(test)]
#[cfg(feature = "test-utils")]
mod tests {
    use crate::digests::{ConsensusCommitDigest, MessageDigest, TransactionEffectsDigest};
    use crate::message::VerifiedTransaction;
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

        let set = CheckpointContents::new_with_digests_only_for_tests([MessageDigest::random()]);

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

        let set = CheckpointContents::new_with_digests_only_for_tests([MessageDigest::random()]);

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
                let set =
                    CheckpointContents::new_with_digests_only_for_tests([MessageDigest::random()]);

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
    fn generate_test_checkpoint_summary_from_digest(digest: MessageDigest) -> CheckpointMessage {
        CheckpointMessage::new(
            &ProtocolConfig::get_for_max_version_UNSAFE(),
            1,
            2,
            10,
            &CheckpointContents::new_with_digests_only_for_tests([MessageDigest::new(digest)]),
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
