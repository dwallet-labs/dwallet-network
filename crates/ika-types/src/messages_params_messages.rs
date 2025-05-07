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

pub use crate::digests::ParamsMessageContentsDigest;
pub use crate::digests::ParamsMessageDigest;

pub type ParamsMessageSequenceNumber = u64;
pub type ParamsMessageTimestamp = u64;

// The constituent parts of params_messages, signed and certified

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ParamsMessageKind {
    NextConfigVersion(ProtocolVersion),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ParamsMessage {
    pub epoch: EpochId,
    pub sequence_number: ParamsMessageSequenceNumber,
    /// Timestamp of the params_message - number of milliseconds from the Unix epoch
    /// ParamsMessage timestamps are monotonic, but not strongly monotonic - subsequent
    /// params_messages can have same timestamp if they originate from the same underlining consensus commit
    pub timestamp_ms: ParamsMessageTimestamp,
    // todo : check with omer if it is okay to remove the vector
    pub messages: Vec<ParamsMessageKind>,
}

impl Message for ParamsMessage {
    type DigestType = ParamsMessageDigest;
    const SCOPE: IntentScope = IntentScope::ParamsMessage;

    fn digest(&self) -> Self::DigestType {
        ParamsMessageDigest::new(default_hash(self))
    }
}

impl ParamsMessage {
    pub fn new(
        epoch: EpochId,
        sequence_number: ParamsMessageSequenceNumber,
        messages: Vec<ParamsMessageKind>,
        timestamp_ms: ParamsMessageTimestamp,
    ) -> ParamsMessage {
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

    pub fn sequence_number(&self) -> &ParamsMessageSequenceNumber {
        &self.sequence_number
    }

    pub fn timestamp(&self) -> SystemTime {
        UNIX_EPOCH + Duration::from_millis(self.timestamp_ms)
    }

    pub fn report_params_message_age(&self, metrics: &Histogram) {
        SystemTime::now()
            .duration_since(self.timestamp())
            .map(|latency| {
                metrics.observe(latency.as_secs_f64());
            })
            .tap_err(|err| {
                warn!(
                    params_message_seq = self.sequence_number,
                    "unable to compute params_message age: {}", err
                )
            })
            .ok();
    }
}

impl Display for ParamsMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ParamsMessageSummary {{ epoch: {:?}, seq: {:?}",
            self.epoch, self.sequence_number,
        )
    }
}

// ParamsMessages are signed by an authority and 2f+1 form a
// certificate that others can use to catch up. The actual
// content of the digest must at the very least commit to
// the set of transactions contained in the certificate but
// we might extend this to contain roots of merkle trees,
// or other authenticated data structures to support light
// clients and more efficient sync protocols.

pub type ParamsMessageEnvelope<S> = Envelope<ParamsMessage, S>;
pub type CertifiedParamsMessage = ParamsMessageEnvelope<AuthorityStrongQuorumSignInfo>;
pub type SignedParamsMessage = ParamsMessageEnvelope<AuthoritySignInfo>;

pub type VerifiedParamsMessage = VerifiedEnvelope<ParamsMessage, AuthorityStrongQuorumSignInfo>;
pub type TrustedParamsMessage = TrustedEnvelope<ParamsMessage, AuthorityStrongQuorumSignInfo>;

impl CertifiedParamsMessage {
    pub fn verify_authority_signatures(&self, committee: &Committee) -> IkaResult {
        self.data().verify_epoch(self.auth_sig().epoch)?;
        self.auth_sig().verify_secure(
            self.data(),
            Intent::ika_app(IntentScope::ParamsMessage),
            committee,
        )
    }

    pub fn try_into_verified(self, committee: &Committee) -> IkaResult<VerifiedParamsMessage> {
        self.verify_authority_signatures(committee)?;
        Ok(VerifiedParamsMessage::new_from_verified(self))
    }

    pub fn into_summary_and_sequence(self) -> (ParamsMessageSequenceNumber, ParamsMessage) {
        let summary = self.into_data();
        (summary.sequence_number, summary)
    }

    pub fn get_validator_signature(self) -> AggregateAuthoritySignature {
        self.auth_sig().signature.clone()
    }
}

impl SignedParamsMessage {
    pub fn verify_authority_signatures(&self, committee: &Committee) -> IkaResult {
        self.data().verify_epoch(self.auth_sig().epoch)?;
        self.auth_sig().verify_secure(
            self.data(),
            Intent::ika_app(IntentScope::ParamsMessage),
            committee,
        )
    }

    pub fn try_into_verified(
        self,
        committee: &Committee,
    ) -> IkaResult<VerifiedEnvelope<ParamsMessage, AuthoritySignInfo>> {
        self.verify_authority_signatures(committee)?;
        Ok(VerifiedEnvelope::<ParamsMessage, AuthoritySignInfo>::new_from_verified(self))
    }
}

impl VerifiedParamsMessage {
    pub fn into_summary_and_sequence(self) -> (ParamsMessageSequenceNumber, ParamsMessage) {
        self.into_inner().into_summary_and_sequence()
    }
}

/// This is a message validators publish to consensus in order to sign params_message
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParamsMessageSignatureMessage {
    pub params_message: SignedParamsMessage,
}

impl ParamsMessageSignatureMessage {
    pub fn verify(&self, committee: &Committee) -> IkaResult {
        self.params_message.verify_authority_signatures(committee)
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
    fn test_signed_params_message() {
        let mut rng = StdRng::from_seed(RNG_SEED);
        let (keys, committee) = make_committee_key(&mut rng);
        let (_, committee2) = make_committee_key(&mut rng);

        let set = ParamsMessageContents::new_with_digests_only_for_tests([MessageDigest::random()]);

        // TODO: duplicated in a test below.

        let signed_params_messages: Vec<_> = keys
            .iter()
            .map(|k| {
                let name = k.public().into();

                SignedParamsMessage::new(
                    committee.epoch,
                    ParamsMessage::new(
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

        signed_params_messages.iter().for_each(|c| {
            c.verify_authority_signatures(&committee)
                .expect("signature ok")
        });

        // fails when not signed by member of committee
        signed_params_messages
            .iter()
            .for_each(|c| assert!(c.verify_authority_signatures(&committee2).is_err()));
    }

    #[test]
    fn test_certified_params_message() {
        let mut rng = StdRng::from_seed(RNG_SEED);
        let (keys, committee) = make_committee_key(&mut rng);

        let set = ParamsMessageContents::new_with_digests_only_for_tests([MessageDigest::random()]);

        let summary = ParamsMessage::new(
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

                SignedParamsMessage::sign(committee.epoch, &summary, k, name)
            })
            .collect();

        let params_message_cert =
            CertifiedParamsMessage::new(summary, sign_infos, &committee).expect("Cert is OK");

        // Signature is correct on proposal, and with same transactions
        assert!(params_message_cert
            .verify_with_contents(&committee, Some(&set))
            .is_ok());

        // Make a bad proposal
        let signed_params_messages: Vec<_> = keys
            .iter()
            .map(|k| {
                let name = k.public().into();
                let set = ParamsMessageContents::new_with_digests_only_for_tests([
                    MessageDigest::random(),
                ]);

                SignedParamsMessage::new(
                    committee.epoch,
                    ParamsMessage::new(
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

        let summary = signed_params_messages[0].data().clone();
        let sign_infos = signed_params_messages
            .into_iter()
            .map(|v| v.into_sig())
            .collect();
        assert!(CertifiedParamsMessage::new(summary, sign_infos, &committee)
            .unwrap()
            .verify_authority_signatures(&committee)
            .is_err())
    }

    // Generate a ParamsMessageSummary from the input transaction digest. All the other fields in the generated
    // ParamsMessageSummary will be the same. The generated ParamsMessageSummary can be used to test how input
    // transaction digest affects ParamsMessageSummary.
    fn generate_test_params_message_summary_from_digest(digest: MessageDigest) -> ParamsMessage {
        ParamsMessage::new(
            &ProtocolConfig::get_for_max_version_UNSAFE(),
            1,
            2,
            10,
            &ParamsMessageContents::new_with_digests_only_for_tests([MessageDigest::new(digest)]),
            None,
            GasCostSummary::default(),
            None,
            100,
            Vec::new(),
        )
    }

    // Tests that ConsensusCommitPrologue with different consensus commit digest will result in different params_message content.
    #[test]
    fn test_params_message_summary_with_different_consensus_digest() {
        // First, tests that same consensus commit digest will produce the same params_message content.
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
            let c1 = generate_test_params_message_summary_from_digest(*t1.digest());
            let c2 = generate_test_params_message_summary_from_digest(*t2.digest());
            assert_eq!(c1.digest(), c2.digest());
        }

        // Next, tests that different consensus commit digests will produce the different params_message contents.
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
            let c1 = generate_test_params_message_summary_from_digest(*t1.digest());
            let c2 = generate_test_params_message_summary_from_digest(*t2.digest());
            assert_ne!(c1.digest(), c2.digest());
        }
    }
}
