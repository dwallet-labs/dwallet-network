// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::committee::EpochId;
use crate::crypto::{AuthoritySignInfo, AuthorityStrongQuorumSignInfo, default_hash};
use crate::digests::{SignatureMPCMessageDigest, SignatureMPCOutputDigest};
use crate::error::SuiResult;
use crate::message_envelope::{Envelope, Message, UnauthenticatedMessage};
use crate::{committee::Committee, error::SuiError};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use shared_crypto::intent::IntentScope;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
pub use signature_mpc::decrypt::{DecryptionShare, PartialDecryptionProof};
pub use signature_mpc::twopc_mpc_protocols::{Commitment, DecentralizedPartyPresign, DKGDecentralizedPartyOutput, EncDHCommitment, EncDHDecommitment, EncDHProofShare, EncDLCommitment, EncDLDecommitment, EncDLProofShare, LargeBiPrimeSizedNumber, PaillierModulusSizedNumber, PresignDecentralizedPartyOutput, PublicNonceEncryptedPartialSignatureAndProof, SecretKeyShareEncryptionAndProof, SecretKeyShareSizedNumber, SignatureNonceSharesCommitmentsAndBatchedProof, tiresias_deal_trusted_shares, DecryptionPublicParameters, PartyID};
pub use crate::digests::CheckpointContentsDigest;
pub use crate::digests::CheckpointDigest;
use crate::base_types::ObjectRef;

pub type InitSignatureMPCProtocolSequenceNumber = u64;
pub type SignatureMPCRound = u64;
pub type SignatureMPCMessageKind = u64;
pub type SignatureMPCTimestamp = u64;

const SESSION_ID_LENGTH: usize = 32;

/// The session id of the mpc is working on.
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SignatureMPCBulletProofAggregatesMessage {
    Commitment((Vec<EncDHCommitment<ProtocolContext>>, Vec<EncDLCommitment<ProtocolContext>>)),
    Decommitment((Vec<EncDHDecommitment<ProtocolContext>>, Vec<EncDLDecommitment<ProtocolContext>>)),
    ProofShare((Vec<EncDHProofShare<ProtocolContext>>, Vec<EncDLProofShare<ProtocolContext>>)),
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SignatureMPCMessageProtocols {
    DKG(SignatureMPCBulletProofAggregatesMessage),
    PresignFirstRound(SignatureMPCBulletProofAggregatesMessage),
    PresignSecondRound(SignatureMPCBulletProofAggregatesMessage),
    Sign(Vec<(PaillierModulusSizedNumber, PaillierModulusSizedNumber)>),
    SignProofs(PartyID, Vec<(PartialDecryptionProof)>, Vec<usize>, Vec<PartyID>),
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

/// This is a message validators publish to consensus in order to sign checkpoint
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
    Sign(Vec<Vec<u8>>),
    // Q: What is the identifiable abort output?
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
                    "DKGSignatureMPCOutputValue::DKG {{ commitment_to_centralized_party_secret_key_share: {:?}, secret_key_share_encryption_and_proof: {:?}}}",
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
            SignatureMPCOutputValue::Sign(sigs) => {
                write!(
                    f,
                    "DKGSignatureMPCOutputValue::Sign {{ sigs: {:?}}}",
                    sigs,
                )
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
            value: SignatureMPCOutputValue::Sign(sigs),
        })
    }

    pub fn message_kind(&self) -> SignatureMPCMessageKind {
        match &self.value {
            SignatureMPCOutputValue::DKG { .. } => 1,
            SignatureMPCOutputValue::PresignOutput(_) => 2,
            SignatureMPCOutputValue::Presign(_) => 3,
            SignatureMPCOutputValue::Sign(_) => 4,
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
pub type CertifiedSignatureMPCOutput =
SignatureMPCOutputEnvelope<AuthorityStrongQuorumSignInfo>;

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
            SignatureMPCMessageProtocols::SignProofs(_, _, _, _) => 4,
        }
    }

    pub fn round(&self) -> SignatureMPCRound {
        match &self.summary.message {
            SignatureMPCMessageProtocols::DKG(m) => m.round(),
            SignatureMPCMessageProtocols::PresignFirstRound(m) => m.round(),
            SignatureMPCMessageProtocols::PresignSecondRound(m) => m.round(),
            SignatureMPCMessageProtocols::Sign(_) => 1,
            SignatureMPCMessageProtocols::SignProofs(_, _, _, _) => 1,
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
        public_nonce_encrypted_partial_signature_and_proofs: Vec<PublicNonceEncryptedPartialSignatureAndProof<ProtocolContext>>,
        presigns: Vec<DecentralizedPartyPresign>,
        hash: u8,
    },
}

pub fn config_signature_mpc_secret_for_network_for_testing(number_of_parties: PartyID) -> (DecryptionPublicParameters, HashMap<PartyID, SecretKeyShareSizedNumber>) {
    let t = (((number_of_parties * 2) / 3) + 1) as PartyID;

    pub const N: LargeBiPrimeSizedNumber = LargeBiPrimeSizedNumber::from_be_hex("97431848911c007fa3a15b718ae97da192e68a4928c0259f2d19ab58ed01f1aa930e6aeb81f0d4429ac2f037def9508b91b45875c11668cea5dc3d4941abd8fbb2d6c8750e88a69727f982e633051f60252ad96ba2e9c9204f4c766c1c97bc096bb526e4b7621ec18766738010375829657c77a23faf50e3a31cb471f72c7abecdec61bdf45b2c73c666aa3729add2d01d7d96172353380c10011e1db3c47199b72da6ae769690c883e9799563d6605e0670a911a57ab5efc69a8c5611f158f1ae6e0b1b6434bafc21238921dc0b98a294195e4e88c173c8dab6334b207636774daad6f35138b9802c1784f334a82cbff480bb78976b22bb0fb41e78fdcb8095");
    pub const SECRET_KEY: PaillierModulusSizedNumber = PaillierModulusSizedNumber::from_be_hex("19d698592b9ccb2890fb84be46cd2b18c360153b740aeccb606cf4168ee2de399f05273182bf468978508a5f4869cb867b340e144838dfaf4ca9bfd38cd55dc2837688aed2dbd76d95091640c47b2037d3d0ca854ffb4c84970b86f905cef24e876ddc8ab9e04f2a5f171b9c7146776c469f0d90908aa436b710cf4489afc73cd3ee38bb81e80a22d5d9228b843f435c48c5eb40088623a14a12b44e2721b56625da5d56d257bb27662c6975630d51e8f5b930d05fc5ba461a0e158cbda0f3266408c9bf60ff617e39ae49e707cbb40958adc512f3b4b69a5c3dc8b6d34cf45bc9597840057438598623fb65254869a165a6030ec6bec12fd59e192b3c1eefd33ef5d9336e0666aa8f36c6bd2749f86ea82290488ee31bf7498c2c77a8900bae00efcff418b62d41eb93502a245236b89c241ad6272724858122a2ebe1ae7ec4684b29048ba25b3a516c281a93043d58844cf3fa0c6f1f73db5db7ecba179652349dea8df5454e0205e910e0206736051ac4b7c707c3013e190423532e907af2e85e5bb6f6f0b9b58257ca1ec8b0318dd197f30352a96472a5307333f0e6b83f4f775fb302c1e10f21e1fcbfff17e3a4aa8bb6f553d9c6ebc2c884ae9b140dd66f21afc8610418e9f0ba2d14ecfa51ff08744a3470ebe4bb21bd6d65b58ac154630b8331ea620673ffbabb179a971a6577c407a076654a629c7733836c250000");
    pub const BASE: PaillierModulusSizedNumber = PaillierModulusSizedNumber::from_be_hex("03B4EFB895D3A85104F1F93744F9DB8924911747DE87ACEC55F1BF37C4531FD7F0A5B498A943473FFA65B89A04FAC2BBDF76FF14D81EB0A0DAD7414CF697E554A93C8495658A329A1907339F9438C1048A6E14476F9569A14BD092BCB2730DCE627566808FD686008F46A47964732DC7DCD2E6ECCE83F7BCCAB2AFDF37144ED153A118B683FF6A3C6971B08DE53DA5D2FEEF83294C21998FC0D1E219A100B6F57F2A2458EA9ABCFA8C5D4DF14B286B71BF5D7AD4FFEEEF069B64E0FC4F1AB684D6B2F20EAA235892F360AA2ECBF361357405D77E5023DF7BEDC12F10F6C35F3BE1163BC37B6C97D62616260A2862F659EB1811B1DDA727847E810D0C2FA120B18E99C9008AA4625CF1862460F8AB3A41E3FDB552187E0408E60885391A52EE2A89DD2471ECBA0AD922DEA0B08474F0BED312993ECB90C90C0F44EF267124A6217BC372D36F8231EB76B0D31DDEB183283A46FAAB74052A01F246D1C638BC00A47D25978D7DF9513A99744D8B65F2B32E4D945B0BA3B7E7A797604173F218D116A1457D20A855A52BBD8AC15679692C5F6AC4A8AF425370EF1D4184322F317203BE9678F92BFD25C7E6820D70EE08809424720249B4C58B81918DA02CFD2CAB3C42A02B43546E64430F529663FCEFA51E87E63F0813DA52F3473506E9E98DCD3142D830F1C1CDF6970726C190EAE1B5D5A26BC30857B4DF639797895E5D61A5EE");

    tiresias_deal_trusted_shares(t, number_of_parties, N, SECRET_KEY, BASE)
}