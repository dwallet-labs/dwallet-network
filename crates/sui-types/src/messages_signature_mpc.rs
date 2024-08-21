// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use serde::{Deserialize, Serialize};
use shared_crypto::intent::IntentScope;
pub use signature_mpc::twopc_mpc_protocols::{
    config_signature_mpc_secret_for_network_for_testing, tiresias_deal_trusted_shares, Commitment,
    DKGDecentralizedPartyOutput, DecentralizedPartyPresign, DecryptionPublicParameters,
    EncDHCommitment, EncDHDecommitment, EncDHProofShare, EncDLCommitment, EncDLDecommitment,
    EncDLProofShare, LargeBiPrimeSizedNumber, PaillierModulusSizedNumber, PartyID,
    PresignDecentralizedPartyOutput, PublicNonceEncryptedPartialSignatureAndProof,
    SecretKeyShareEncryptionAndProof, SecretKeyShareSizedNumber,
    SignatureNonceSharesCommitmentsAndBatchedProof,
};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use twopc_mpc::secp256k1::paillier::bulletproofs::PartialDecryptionProof;

use crate::base_types::{ObjectID, ObjectRef};
use crate::committee::EpochId;
use crate::crypto::{default_hash, AuthoritySignInfo, AuthorityStrongQuorumSignInfo};
pub use crate::digests::CheckpointContentsDigest;
pub use crate::digests::CheckpointDigest;
use crate::digests::{SignatureMPCMessageDigest, SignatureMPCOutputDigest};
use crate::error::SuiResult;
use crate::message_envelope::{Envelope, Message, UnauthenticatedMessage};
use crate::{committee::Committee, error::SuiError};

pub type InitSignatureMPCProtocolSequenceNumber = u64;
pub type SignatureMPCRound = u64;
pub type SignatureMPCMessageKind = u64;
pub type SignatureMPCTimestamp = u64;

const SESSION_ID_LENGTH: usize = 32;

/// The session ID of the mpc is working on.
#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignatureMPCSessionID(pub [u8; SESSION_ID_LENGTH]);

// TODO: uncomment this and use a struct and not PhantomData
// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
// pub struct ProtocolContext {
//     epoch: EpochId,
//     party_id: PartyID,
//     number_of_parties: PartyID,
//     session_id: SignatureMPCSessionID,
// }

// TODO: remove this temp hack
pub type ProtocolContext = PhantomData<()>;

/// The messages that may be sent between validators while performing the sign flow.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SignMessage {
    /// A validator will send this message after decrypting a message. This message contains a vector of decryption
    /// shares, one for every message in the batch.
    DecryptionShares(Vec<(PaillierModulusSizedNumber, PaillierModulusSizedNumber)>),
    /// The aggregator only may send this message if calling [`twopc_mpc_protocols::decrypt_signatures`]
    /// returned an error. This message tells to each of the involved validators to generate a proof that
    /// he behaved honestly in the decryption.
    StartIAFlow(Vec<PartyID>),
    /// This message contains the proofs a validator may generate after receiving the [`SignMessage::StartIAFlow`] message.
    IAProofs(Vec<PartialDecryptionProof>),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SignatureMPCBulletProofAggregatesMessage {
    Commitment(
        (
            Vec<EncDHCommitment<ProtocolContext>>,
            Vec<EncDLCommitment<ProtocolContext>>,
        ),
    ),
    Decommitment(
        (
            Vec<EncDHDecommitment<ProtocolContext>>,
            Vec<EncDLDecommitment<ProtocolContext>>,
        ),
    ),
    ProofShare(
        (
            Vec<EncDHProofShare<ProtocolContext>>,
            Vec<EncDLProofShare<ProtocolContext>>,
        ),
    ),
}

impl SignatureMPCBulletProofAggregatesMessage {
    pub fn round(&self) -> SignatureMPCRound {
        match self {
            SignatureMPCBulletProofAggregatesMessage::Commitment(_) => 1,
            SignatureMPCBulletProofAggregatesMessage::Decommitment(_) => 2,
            SignatureMPCBulletProofAggregatesMessage::ProofShare(_) => 3,
        }
    }
}

impl SignMessage {
    pub fn round(&self) -> SignatureMPCRound {
        match self {
            SignMessage::DecryptionShares(_) => 1,
            SignMessage::IAProofs(_) => 2,
            SignMessage::StartIAFlow(_) => 3,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SignatureMPCMessageProtocols {
    DKG(SignatureMPCBulletProofAggregatesMessage),
    PresignFirstRound(SignatureMPCBulletProofAggregatesMessage),
    PresignSecondRound(SignatureMPCBulletProofAggregatesMessage),
    Sign(SignMessage),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignatureMPCMessageSummary {
    pub epoch: EpochId,
    pub message: SignatureMPCMessageProtocols,
    pub session_id: SignatureMPCSessionID,
}

impl Message for SignatureMPCMessageSummary {
    type DigestType = SignatureMPCMessageDigest;
    const SCOPE: IntentScope = IntentScope::SignatureMPCMessage;

    fn digest(&self) -> Self::DigestType {
        SignatureMPCMessageDigest::new(default_hash(self))
    }

    fn verify_user_input(&self) -> SuiResult {
        Ok(())
    }

    fn verify_epoch(&self, epoch: EpochId) -> SuiResult {
        fp_ensure!(
            self.epoch == epoch,
            SuiError::WrongEpoch {
                expected_epoch: epoch,
                actual_epoch: self.epoch,
            }
        );
        Ok(())
    }
}

impl UnauthenticatedMessage for SignatureMPCMessageSummary {}

impl SignatureMPCMessageSummary {
    pub fn new(
        epoch: EpochId,
        message: SignatureMPCMessageProtocols,
        session_id: SignatureMPCSessionID,
    ) -> SignatureMPCMessageSummary {
        Self {
            epoch,
            message,
            session_id,
        }
    }
}

impl Display for SignatureMPCMessageSummary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SignatureMPCMessage {{ epoch: {:?}, message: {:?}, session_id: {:?}}}",
            self.epoch, self.message, self.session_id,
        )
    }
}

pub type SignatureMPCMessageSummaryEnvelope<S> = Envelope<SignatureMPCMessageSummary, S>;
pub type SignedSignatureMPCMessageSummary = SignatureMPCMessageSummaryEnvelope<AuthoritySignInfo>;

/// This is a message validators publish to consensus to sign checkpoint.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignatureMPCMessage {
    pub summary: SignedSignatureMPCMessageSummary,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SignatureMPCOutputValue {
    DKG {
        commitment_to_centralized_party_secret_key_share: Vec<u8>,
        secret_key_share_encryption_and_proof: Vec<u8>,
    },
    PresignOutput(Vec<u8>),
    Presign(Vec<u8>),
    Sign {
        sigs: Vec<Vec<u8>>,
        /// Used to punish a malicious validator if it attempts to send an invalid signature.
        aggregator_public_key: Vec<u8>,
    },
}

impl Display for SignatureMPCOutputValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SignatureMPCOutputValue::DKG {
                commitment_to_centralized_party_secret_key_share,
                secret_key_share_encryption_and_proof,
            } => {
                write!(
                    f,
                    "DKGSignatureMPCOutputValue::DKG {{ commitment_to_centralized_party_secret_key_share: \
                    {:?}, secret_key_share_encryption_and_proof: {:?}}}",
                    commitment_to_centralized_party_secret_key_share,
                    secret_key_share_encryption_and_proof,
                )
            }
            SignatureMPCOutputValue::PresignOutput(output) => {
                write!(
                    f,
                    "DKGSignatureMPCOutputValue::PresignOutput {{ output: {:?}}}",
                    output,
                )
            }
            SignatureMPCOutputValue::Presign(presigns) => {
                write!(
                    f,
                    "DKGSignatureMPCOutputValue::Presign {{ presigns: {:?}}}",
                    presigns,
                )
            }
            SignatureMPCOutputValue::Sign { sigs, .. } => {
                write!(f, "DKGSignatureMPCOutputValue::Sign {{ sigs: {:?}}}", sigs,)
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SignatureMPCOutput {
    pub epoch: EpochId,
    pub session_id: SignatureMPCSessionID,
    pub session_ref: ObjectRef,
    pub value: SignatureMPCOutputValue,
}

impl Message for SignatureMPCOutput {
    type DigestType = SignatureMPCOutputDigest;
    const SCOPE: IntentScope = IntentScope::SignatureMPCOutput;

    fn digest(&self) -> Self::DigestType {
        SignatureMPCOutputDigest::new(default_hash(self))
    }

    fn verify_user_input(&self) -> SuiResult {
        Ok(())
    }

    fn verify_epoch(&self, epoch: EpochId) -> SuiResult {
        fp_ensure!(
            self.epoch == epoch,
            SuiError::WrongEpoch {
                expected_epoch: epoch,
                actual_epoch: self.epoch,
            }
        );
        Ok(())
    }
}

impl UnauthenticatedMessage for SignatureMPCOutput {}

impl SignatureMPCOutput {
    pub fn new_dkg(
        epoch: EpochId,
        session_id: SignatureMPCSessionID,
        session_ref: ObjectRef,
        commitment_to_centralized_party_secret_key_share: Commitment,
        secret_key_share_encryption_and_proof: SecretKeyShareEncryptionAndProof<ProtocolContext>,
    ) -> SuiResult<SignatureMPCOutput> {
        let commitment_to_centralized_party_secret_key_share =
            bcs::to_bytes(&commitment_to_centralized_party_secret_key_share).map_err(|e| {
                SuiError::ObjectSerializationError {
                    error: format!("{e}"),
                }
            })?;
        let secret_key_share_encryption_and_proof =
            bcs::to_bytes(&secret_key_share_encryption_and_proof).map_err(|e| {
                SuiError::ObjectSerializationError {
                    error: format!("{e}"),
                }
            })?;
        Ok(Self {
            epoch,
            session_id,
            session_ref,
            value: SignatureMPCOutputValue::DKG {
                commitment_to_centralized_party_secret_key_share,
                secret_key_share_encryption_and_proof,
            },
        })
    }
    pub fn new_presign_output(
        epoch: EpochId,
        session_id: SignatureMPCSessionID,
        session_ref: ObjectRef,
        output: PresignDecentralizedPartyOutput<ProtocolContext>,
    ) -> SuiResult<SignatureMPCOutput> {
        let output = bcs::to_bytes(&output).map_err(|e| SuiError::ObjectSerializationError {
            error: format!("{e}"),
        })?;
        Ok(Self {
            epoch,
            session_id,
            session_ref,
            value: SignatureMPCOutputValue::PresignOutput(output),
        })
    }
    pub fn new_presign(
        epoch: EpochId,
        session_id: SignatureMPCSessionID,
        session_ref: ObjectRef,
        presigns: Vec<DecentralizedPartyPresign>,
    ) -> SuiResult<SignatureMPCOutput> {
        let presigns =
            bcs::to_bytes(&presigns).map_err(|e| SuiError::ObjectSerializationError {
                error: format!("{e}"),
            })?;
        Ok(Self {
            epoch,
            session_id,
            session_ref,
            value: SignatureMPCOutputValue::Presign(presigns),
        })
    }
    pub fn new_sign(
        epoch: EpochId,
        session_id: SignatureMPCSessionID,
        session_ref: ObjectRef,
        sigs: Vec<Vec<u8>>,
    ) -> SuiResult<SignatureMPCOutput> {
        Ok(Self {
            epoch,
            session_id,
            session_ref,
            value: SignatureMPCOutputValue::Sign {
                sigs,
                aggregator_public_key: Vec::new(),
            },
        })
    }

    pub fn message_kind(&self) -> SignatureMPCMessageKind {
        match &self.value {
            SignatureMPCOutputValue::DKG { .. } => 1,
            SignatureMPCOutputValue::PresignOutput(_) => 2,
            SignatureMPCOutputValue::Presign(_) => 3,
            SignatureMPCOutputValue::Sign { .. } => 4,
        }
    }
}

impl Display for SignatureMPCOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DKGSignatureMPCAggregatedProofSharesMessage {{ epoch: {:?}, session_id: {:?}, value: {}}}",
            self.epoch,
            self.session_id,
            self.value,
        )
    }
}

pub type SignatureMPCOutputEnvelope<S> = Envelope<SignatureMPCOutput, S>;
pub type SignedSignatureMPCOutput = SignatureMPCOutputEnvelope<AuthoritySignInfo>;
pub type CertifiedSignatureMPCOutput = SignatureMPCOutputEnvelope<AuthorityStrongQuorumSignInfo>;

impl SignatureMPCMessage {
    pub fn verify(&self, committee: &Committee) -> SuiResult {
        self.summary.verify_authority_signatures(committee)
    }

    pub fn message_kind(&self) -> SignatureMPCMessageKind {
        match &self.summary.message {
            SignatureMPCMessageProtocols::DKG(_) => 1,
            SignatureMPCMessageProtocols::PresignFirstRound(_) => 2,
            SignatureMPCMessageProtocols::PresignSecondRound(_) => 3,
            SignatureMPCMessageProtocols::Sign(_) => 3,
        }
    }

    pub fn round(&self) -> SignatureMPCRound {
        match &self.summary.message {
            SignatureMPCMessageProtocols::DKG(m) => m.round(),
            SignatureMPCMessageProtocols::PresignFirstRound(m) => m.round(),
            SignatureMPCMessageProtocols::PresignSecondRound(m) => m.round(),
            SignatureMPCMessageProtocols::Sign(m) => m.round(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum InitiateSignatureMPCProtocol {
    DKG {
        session_id: SignatureMPCSessionID,
        session_ref: ObjectRef,
        commitment_to_centralized_party_secret_key_share: Commitment,
    },
    Presign {
        session_id: SignatureMPCSessionID,
        session_ref: ObjectRef,
        dkg_output: DKGDecentralizedPartyOutput,
        commitments_and_proof_to_centralized_party_nonce_shares:
            SignatureNonceSharesCommitmentsAndBatchedProof<ProtocolContext>,
    },
    Sign {
        session_id: SignatureMPCSessionID,
        session_ref: ObjectRef,
        messages: Vec<Vec<u8>>,
        dkg_output: DKGDecentralizedPartyOutput,
        public_nonce_encrypted_partial_signature_and_proofs:
            Vec<PublicNonceEncryptedPartialSignatureAndProof<ProtocolContext>>,
        presigns: Vec<DecentralizedPartyPresign>,
        hash: u8,
    },
}
