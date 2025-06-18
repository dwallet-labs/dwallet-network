// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::crypto::default_hash;
use crate::digests::MessageDigest;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use strum::IntoStaticStr;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct DKGFirstRoundOutput {
    pub dwallet_id: Vec<u8>,
    pub output: Vec<u8>,
    pub rejected: bool,
    pub session_sequence_number: u64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct DKGSecondRoundOutput {
    pub dwallet_id: Vec<u8>,
    pub encrypted_secret_share_id: Vec<u8>,
    pub output: Vec<u8>,
    pub rejected: bool,
    pub session_sequence_number: u64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct PresignOutput {
    pub dwallet_id: Option<Vec<u8>>,
    pub presign_id: Vec<u8>,
    pub presign: Vec<u8>,
    pub rejected: bool,
    pub session_sequence_number: u64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct SignOutput {
    pub dwallet_id: Vec<u8>,
    pub sign_id: Vec<u8>,
    pub signature: Vec<u8>,
    pub is_future_sign: bool,
    pub rejected: bool,
    pub session_sequence_number: u64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct EncryptedUserShareOutput {
    pub dwallet_id: Vec<u8>,
    pub encrypted_user_secret_key_share_id: Vec<u8>,
    pub rejected: bool,
    pub session_sequence_number: u64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct PartialSignatureVerificationOutput {
    pub dwallet_id: Vec<u8>,
    pub partial_centralized_signed_message_id: Vec<u8>,
    pub rejected: bool,
    pub session_sequence_number: u64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct NetworkKeyPublicOutputSlice {
    pub session_id: Vec<u8>,
    pub dwallet_network_decryption_key_id: Vec<u8>,
    pub public_output: Vec<u8>,
    pub supported_curves: Vec<u32>,
    pub is_last: bool,
    pub rejected: bool,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct MakeDWalletUserSecretKeySharesPublicOutput {
    pub dwallet_id: Vec<u8>,
    pub public_user_secret_key_shares: Vec<u8>,
    pub rejected: bool,
    pub session_sequence_number: u64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct DWalletImportedKeyVerificationOutput {
    pub dwallet_id: Vec<u8>,
    pub public_output: Vec<u8>,
    pub encrypted_user_secret_key_share_id: Vec<u8>,
    pub rejected: bool,
    pub session_sequence_number: u64,
}

// Note: the order of these fields, and the number must correspond to the Move code in
// `dwallet_2pc_mpc_coordinator_inner.move`.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, IntoStaticStr)]
pub enum DWalletMessageKind {
    RespondDWalletDKGFirstRoundOutput(DKGFirstRoundOutput),
    RespondDWalletDKGSecondRoundOutput(DKGSecondRoundOutput),
    RespondDWalletEncryptedUserShare(EncryptedUserShareOutput),
    RespondMakeDWalletUserSecretKeySharesPublic(MakeDWalletUserSecretKeySharesPublicOutput),
    RespondDWalletImportedKeyVerificationOutput(DWalletImportedKeyVerificationOutput),
    RespondDWalletPresign(PresignOutput),
    RespondDWalletSign(SignOutput),
    RespondDWalletPartialSignatureVerificationOutput(PartialSignatureVerificationOutput),
    RespondDWalletMPCNetworkDKGOutput(NetworkKeyPublicOutputSlice),
    RespondDWalletMPCNetworkReconfigurationOutput(NetworkKeyPublicOutputSlice),
    SetMaxActiveSessionsBuffer(u64),
    SetGasFeeReimbursementSuiSystemCallValue(u64),
}

impl DWalletMessageKind {
    pub fn name(&self) -> &'static str {
        match self {
            DWalletMessageKind::RespondDWalletDKGFirstRoundOutput(_) => {
                "RespondDWalletDKGFirstRoundOutput"
            }
            DWalletMessageKind::RespondDWalletDKGSecondRoundOutput(_) => {
                "RespondDWalletDKGSecondRoundOutput"
            }
            DWalletMessageKind::RespondDWalletEncryptedUserShare(_) => {
                "RespondDWalletEncryptedUserShare"
            }
            DWalletMessageKind::RespondDWalletPresign(_) => "RespondDWalletPresign",
            DWalletMessageKind::RespondDWalletSign(_) => "RespondDWalletSign",
            DWalletMessageKind::RespondDWalletPartialSignatureVerificationOutput(_) => {
                "RespondDWalletPartialSignatureVerificationOutput"
            }
            DWalletMessageKind::RespondDWalletMPCNetworkDKGOutput(_) => {
                "RespondDWalletMPCNetworkDKGOutput"
            }
            DWalletMessageKind::RespondDWalletMPCNetworkReconfigurationOutput(_) => {
                "RespondDWalletMPCNetworkReconfigurationOutput"
            }
            DWalletMessageKind::RespondMakeDWalletUserSecretKeySharesPublic(_) => {
                "RespondMakeDWalletUserSecretKeySharesPublic"
            }
            DWalletMessageKind::RespondDWalletImportedKeyVerificationOutput(_) => {
                "RespondDWalletImportedKeyVerificationOutput"
            }
            DWalletMessageKind::SetMaxActiveSessionsBuffer(_) => "SetMaxActiveSessionsBuffer",
            DWalletMessageKind::SetGasFeeReimbursementSuiSystemCallValue(_) => {
                "SetGasFeeReimbursementSuiSystemCallValue"
            }
        }
    }

    pub fn digest(&self) -> MessageDigest {
        MessageDigest::new(default_hash(self))
    }
}

impl Display for DWalletMessageKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();
        match &self {
            DWalletMessageKind::RespondDWalletMPCNetworkDKGOutput(_) => {
                writeln!(writer, "MessageKind : RespondDwalletMPCNetworkDKGOutput")?;
            }
            DWalletMessageKind::RespondDWalletDKGFirstRoundOutput(_) => {
                writeln!(writer, "MessageKind : RespondDwalletDKGFirstRoundOutput")?;
            }
            DWalletMessageKind::RespondDWalletDKGSecondRoundOutput(_) => {
                writeln!(writer, "MessageKind : RespondDwalletDKGSecondRoundOutput")?;
            }
            DWalletMessageKind::RespondDWalletPresign(_) => {
                writeln!(writer, "MessageKind : RespondDwalletPresign")?;
            }
            DWalletMessageKind::RespondDWalletSign(_) => {
                writeln!(writer, "MessageKind : RespondDwalletSign")?;
            }
            DWalletMessageKind::RespondDWalletEncryptedUserShare(_) => {
                writeln!(writer, "MessageKind : RespondDwalletEncryptedUserShare")?;
            }
            DWalletMessageKind::RespondDWalletPartialSignatureVerificationOutput(_) => {
                writeln!(
                    writer,
                    "MessageKind : RespondDwalletPartialSignatureVerificationOutput"
                )?;
            }
            DWalletMessageKind::RespondDWalletMPCNetworkReconfigurationOutput(_) => {
                writeln!(
                    writer,
                    "MessageKind : RespondDWalletMPCNetworkReconfigurationOutput"
                )?;
            }
            DWalletMessageKind::RespondMakeDWalletUserSecretKeySharesPublic(_) => {
                writeln!(
                    writer,
                    "MessageKind : RespondMakeDWalletUserSecretKeySharesPublic"
                )?;
            }
            DWalletMessageKind::RespondDWalletImportedKeyVerificationOutput(_) => {
                writeln!(
                    writer,
                    "MessageKind : RespondDWalletImportedKeyVerificationOutput"
                )?;
            }
            DWalletMessageKind::SetMaxActiveSessionsBuffer(buffer_size) => {
                writeln!(
                    writer,
                    "MessageKind : SetMaxActiveSessionsBuffer({})",
                    buffer_size
                )?;
            }
            DWalletMessageKind::SetGasFeeReimbursementSuiSystemCallValue(value) => {
                writeln!(
                    writer,
                    "MessageKind : SetGasFeeReimbursementSuiSystemCallValue({})",
                    value
                )?;
            }
        }
        write!(f, "{}", writer)
    }
}

impl Debug for DWalletMessageKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();
        match &self {
            DWalletMessageKind::RespondDWalletMPCNetworkDKGOutput(_) => {
                writeln!(writer, "MessageKind : RespondDwalletMPCNetworkDKGOutput")?;
            }
            DWalletMessageKind::RespondDWalletDKGFirstRoundOutput(_) => {
                writeln!(writer, "MessageKind : RespondDwalletDKGFirstRoundOutput")?;
            }
            DWalletMessageKind::RespondDWalletDKGSecondRoundOutput(_) => {
                writeln!(writer, "MessageKind : RespondDwalletDKGSecondRoundOutput")?;
            }
            DWalletMessageKind::RespondDWalletPresign(_) => {
                writeln!(writer, "MessageKind : RespondDwalletPresign")?;
            }
            DWalletMessageKind::RespondDWalletSign(_) => {
                writeln!(writer, "MessageKind : RespondDwalletSign")?;
            }
            DWalletMessageKind::RespondDWalletEncryptedUserShare(_) => {
                writeln!(writer, "MessageKind : RespondDwalletEncryptedUserShare")?;
            }
            DWalletMessageKind::RespondDWalletPartialSignatureVerificationOutput(_) => {
                writeln!(
                    writer,
                    "MessageKind : RespondDwalletPartialSignatureVerificationOutput"
                )?;
            }
            DWalletMessageKind::RespondDWalletMPCNetworkReconfigurationOutput(_) => {
                writeln!(
                    writer,
                    "MessageKind : RespondDWalletMPCNetworkReconfigurationOutput"
                )?;
            }
            DWalletMessageKind::RespondMakeDWalletUserSecretKeySharesPublic(_) => {
                writeln!(
                    writer,
                    "MessageKind : RespondMakeDWalletUserSecretKeySharesPublic"
                )?;
            }
            DWalletMessageKind::RespondDWalletImportedKeyVerificationOutput(_) => {
                writeln!(
                    writer,
                    "MessageKind : RespondDWalletImportedKeyVerificationOutput"
                )?;
            }
            DWalletMessageKind::SetMaxActiveSessionsBuffer(buffer_size) => {
                writeln!(
                    writer,
                    "MessageKind : SetMaxActiveSessionsBuffer({})",
                    buffer_size
                )?;
            }
            DWalletMessageKind::SetGasFeeReimbursementSuiSystemCallValue(value) => {
                writeln!(
                    writer,
                    "MessageKind : SetGasFeeReimbursementSuiSystemCallValue({})",
                    value
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
