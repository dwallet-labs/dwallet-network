// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::committee::{EpochId, ProtocolVersion, StakeUnit};
use crate::crypto::{
    default_hash, AggregateAuthoritySignature, AuthoritySignInfo, AuthoritySignInfoTrait,
    AuthorityStrongQuorumSignInfo,
};
use crate::error::IkaResult;
use crate::intent::{Intent, IntentScope};
use crate::message_envelope::{Envelope, Message, TrustedEnvelope, VerifiedEnvelope};
use crate::{committee::Committee, error::IkaError};
use ika_protocol_config::ProtocolConfig;
use prometheus::Histogram;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::fmt::{Debug, Display, Formatter};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use sui_types::effects::{TestEffectsBuilder, TransactionEffectsAPI};
use sui_types::storage::ReadStore;
use sui_types::sui_serde::BigInt;
use sui_types::transaction::{Transaction, TransactionData};
use tap::TapFallible;
use tracing::warn;

pub use crate::digests::IkaSystemCheckpointContentsDigest;
pub use crate::digests::IkaSystemCheckpointDigest;

pub type IkaSystemCheckpointSequenceNumber = u64;
pub type IkaSystemCheckpointTimestamp = u64;

// The constituent parts of ika_system_checkpoints, signed and certified

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum IkaSystemCheckpointKind {
    NextConfigVersion(ProtocolVersion),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct IkaSystemCheckpoint {
    pub epoch: EpochId,
    pub sequence_number: IkaSystemCheckpointSequenceNumber,
    /// Timestamp of the ika_system_checkpoint - number of milliseconds from the Unix epoch
    /// IkaSystemCheckpoint timestamps are monotonic, but not strongly monotonic - subsequent
    /// ika_system_checkpoints can have same timestamp if they originate from the same underlining consensus commit
    pub timestamp_ms: IkaSystemCheckpointTimestamp,
    pub messages: Vec<IkaSystemCheckpointKind>,
}

impl Message for IkaSystemCheckpoint {
    type DigestType = IkaSystemCheckpointDigest;
    const SCOPE: IntentScope = IntentScope::IkaSystemCheckpoint;

    fn digest(&self) -> Self::DigestType {
        IkaSystemCheckpointDigest::new(default_hash(self))
    }
}

impl IkaSystemCheckpoint {
    pub fn new(
        epoch: EpochId,
        sequence_number: IkaSystemCheckpointSequenceNumber,
        messages: Vec<IkaSystemCheckpointKind>,
        timestamp_ms: IkaSystemCheckpointTimestamp,
    ) -> IkaSystemCheckpoint {
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

    pub fn sequence_number(&self) -> &IkaSystemCheckpointSequenceNumber {
        &self.sequence_number
    }

    pub fn timestamp(&self) -> SystemTime {
        UNIX_EPOCH + Duration::from_millis(self.timestamp_ms)
    }

    pub fn report_ika_system_checkpoint_age(&self, metrics: &Histogram) {
        SystemTime::now()
            .duration_since(self.timestamp())
            .map(|latency| {
                metrics.observe(latency.as_secs_f64());
            })
            .tap_err(|err| {
                warn!(
                    ika_system_checkpoint_seq = self.sequence_number,
                    "unable to compute ika_system_checkpoint age: {}", err
                )
            })
            .ok();
    }
}

impl Display for IkaSystemCheckpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IkaSystemCheckpointSummary {{ epoch: {:?}, seq: {:?}",
            self.epoch, self.sequence_number,
        )
    }
}

// IkaSystemCheckpoints are signed by an authority and 2f+1 form a
// certificate that others can use to catch up. The actual
// content of the digest must at the very least commit to
// the set of transactions contained in the certificate, but
// we might extend this to contain roots of merkle trees,
// or other authenticated data structures to support light
// clients and more efficient sync protocols.

pub type IkaSystemCheckpointEnvelope<S> = Envelope<IkaSystemCheckpoint, S>;
pub type CertifiedIkaSystemCheckpoint = IkaSystemCheckpointEnvelope<AuthorityStrongQuorumSignInfo>;
pub type SignedIkaSystemCheckpoint = IkaSystemCheckpointEnvelope<AuthoritySignInfo>;

pub type VerifiedIkaSystemCheckpoint =
    VerifiedEnvelope<IkaSystemCheckpoint, AuthorityStrongQuorumSignInfo>;
pub type TrustedIkaSystemCheckpoint =
    TrustedEnvelope<IkaSystemCheckpoint, AuthorityStrongQuorumSignInfo>;

impl CertifiedIkaSystemCheckpoint {
    pub fn verify_authority_signatures(&self, committee: &Committee) -> IkaResult {
        self.data().verify_epoch(self.auth_sig().epoch)?;
        self.auth_sig().verify_secure(
            self.data(),
            Intent::ika_app(IntentScope::IkaSystemCheckpoint),
            committee,
        )
    }

    pub fn try_into_verified(
        self,
        committee: &Committee,
    ) -> IkaResult<VerifiedIkaSystemCheckpoint> {
        self.verify_authority_signatures(committee)?;
        Ok(VerifiedIkaSystemCheckpoint::new_from_verified(self))
    }

    pub fn into_summary_and_sequence(
        self,
    ) -> (IkaSystemCheckpointSequenceNumber, IkaSystemCheckpoint) {
        let summary = self.into_data();
        (summary.sequence_number, summary)
    }

    pub fn get_validator_signature(self) -> AggregateAuthoritySignature {
        self.auth_sig().signature.clone()
    }
}

impl SignedIkaSystemCheckpoint {
    pub fn verify_authority_signatures(&self, committee: &Committee) -> IkaResult {
        self.data().verify_epoch(self.auth_sig().epoch)?;
        self.auth_sig().verify_secure(
            self.data(),
            Intent::ika_app(IntentScope::IkaSystemCheckpoint),
            committee,
        )
    }

    pub fn try_into_verified(
        self,
        committee: &Committee,
    ) -> IkaResult<VerifiedEnvelope<IkaSystemCheckpoint, AuthoritySignInfo>> {
        self.verify_authority_signatures(committee)?;
        Ok(VerifiedEnvelope::<IkaSystemCheckpoint, AuthoritySignInfo>::new_from_verified(self))
    }
}

impl VerifiedIkaSystemCheckpoint {
    pub fn into_summary_and_sequence(
        self,
    ) -> (IkaSystemCheckpointSequenceNumber, IkaSystemCheckpoint) {
        self.into_inner().into_summary_and_sequence()
    }
}

/// This is a message validators publish to consensus in order to sign ika_system_checkpoint
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IkaSystemCheckpointSignatureMessage {
    pub ika_system_checkpoint: SignedIkaSystemCheckpoint,
}

impl IkaSystemCheckpointSignatureMessage {
    pub fn verify(&self, committee: &Committee) -> IkaResult {
        self.ika_system_checkpoint
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
    fn test_signed_ika_system_checkpoint() {
        let mut rng = StdRng::from_seed(RNG_SEED);
        let (keys, committee) = make_committee_key(&mut rng);
        let (_, committee2) = make_committee_key(&mut rng);

        let set =
            IkaSystemCheckpointContents::new_with_digests_only_for_tests([MessageDigest::random()]);

        // TODO: duplicated in a test below.

        let signed_ika_system_checkpoints: Vec<_> = keys
            .iter()
            .map(|k| {
                let name = k.public().into();

                SignedIkaSystemCheckpoint::new(
                    committee.epoch,
                    IkaSystemCheckpoint::new(
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

        signed_ika_system_checkpoints.iter().for_each(|c| {
            c.verify_authority_signatures(&committee)
                .expect("signature ok")
        });

        // fails when not signed by member of committee
        signed_ika_system_checkpoints
            .iter()
            .for_each(|c| assert!(c.verify_authority_signatures(&committee2).is_err()));
    }

    #[test]
    fn test_certified_ika_system_checkpoint() {
        let mut rng = StdRng::from_seed(RNG_SEED);
        let (keys, committee) = make_committee_key(&mut rng);

        let set =
            IkaSystemCheckpointContents::new_with_digests_only_for_tests([MessageDigest::random()]);

        let summary = IkaSystemCheckpoint::new(
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

                SignedIkaSystemCheckpoint::sign(committee.epoch, &summary, k, name)
            })
            .collect();

        let ika_system_checkpoint_cert =
            CertifiedIkaSystemCheckpoint::new(summary, sign_infos, &committee).expect("Cert is OK");

        // Signature is correct on proposal, and with same transactions
        assert!(ika_system_checkpoint_cert
            .verify_with_contents(&committee, Some(&set))
            .is_ok());

        // Make a bad proposal
        let signed_ika_system_checkpoints: Vec<_> = keys
            .iter()
            .map(|k| {
                let name = k.public().into();
                let set = IkaSystemCheckpointContents::new_with_digests_only_for_tests([
                    MessageDigest::random(),
                ]);

                SignedIkaSystemCheckpoint::new(
                    committee.epoch,
                    IkaSystemCheckpoint::new(
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

        let summary = signed_ika_system_checkpoints[0].data().clone();
        let sign_infos = signed_ika_system_checkpoints
            .into_iter()
            .map(|v| v.into_sig())
            .collect();
        assert!(
            CertifiedIkaSystemCheckpoint::new(summary, sign_infos, &committee)
                .unwrap()
                .verify_authority_signatures(&committee)
                .is_err()
        )
    }

    // Generate a IkaSystemCheckpointSummary from the input transaction digest. All the other fields in the generated
    // IkaSystemCheckpointSummary will be the same. The generated IkaSystemCheckpointSummary can be used to test how input
    // transaction digest affects IkaSystemCheckpointSummary.
    fn generate_test_ika_system_checkpoint_summary_from_digest(
        digest: MessageDigest,
    ) -> IkaSystemCheckpoint {
        IkaSystemCheckpoint::new(
            &ProtocolConfig::get_for_max_version_UNSAFE(),
            1,
            2,
            10,
            &IkaSystemCheckpointContents::new_with_digests_only_for_tests([MessageDigest::new(
                digest,
            )]),
            None,
            GasCostSummary::default(),
            None,
            100,
            Vec::new(),
        )
    }

    // Tests that ConsensusCommitPrologue with different consensus commit digest will result in different ika_system_checkpoint content.
    #[test]
    fn test_ika_system_checkpoint_summary_with_different_consensus_digest() {
        // First, tests that same consensus commit digest will produce the same ika_system_checkpoint content.
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
            let c1 = generate_test_ika_system_checkpoint_summary_from_digest(*t1.digest());
            let c2 = generate_test_ika_system_checkpoint_summary_from_digest(*t2.digest());
            assert_eq!(c1.digest(), c2.digest());
        }

        // Next, tests that different consensus commit digests will produce the different ika_system_checkpoint contents.
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
            let c1 = generate_test_ika_system_checkpoint_summary_from_digest(*t1.digest());
            let c2 = generate_test_ika_system_checkpoint_summary_from_digest(*t2.digest());
            assert_ne!(c1.digest(), c2.digest());
        }
    }
}
