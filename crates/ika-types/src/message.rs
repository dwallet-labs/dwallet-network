// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::crypto::default_hash;
use crate::digests::MessageDigest;
use fastcrypto::hash::HashFunction;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use strum::IntoStaticStr;
use sui_types::base_types::ObjectID;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct DKGFirstRoundOutput {
    pub dwallet_id: Vec<u8>,
    pub output: Vec<u8>,
    pub session_sequence_number: u64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct DKGSecondRoundOutput {
    pub dwallet_id: Vec<u8>,
    pub session_id: Vec<u8>,
    pub encrypted_secret_share_id: Vec<u8>,
    pub output: Vec<u8>,
    pub rejected: bool,
    pub session_sequence_number: u64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct PresignOutput {
    pub dwallet_id: Option<Vec<u8>>,
    pub presign_id: Vec<u8>,
    pub session_id: Vec<u8>,
    pub presign: Vec<u8>,
    pub rejected: bool,
    pub session_sequence_number: u64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct SignOutput {
    pub dwallet_id: Vec<u8>,
    pub sign_id: Vec<u8>,
    pub session_id: Vec<u8>,
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
    pub session_id: Vec<u8>,
    pub dwallet_id: Vec<u8>,
    pub partial_centralized_signed_message_id: Vec<u8>,
    pub rejected: bool,
    pub session_sequence_number: u64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct Secp256K1NetworkKeyPublicOutputSlice {
    pub dwallet_network_decryption_key_id: Vec<u8>,
    pub public_output: Vec<u8>,
    pub is_last: bool,
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
    pub session_id: Vec<u8>,
    pub rejected: bool,
    pub session_sequence_number: u64,
}

// Note: the order of these fields, and the number must correspond to the Move code in
// `dwallet_2pc_mpc_coordinator_inner.move`.
#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize, IntoStaticStr)]
pub enum MessageKind {
    DwalletDKGFirstRoundOutput(DKGFirstRoundOutput),
    DwalletDKGSecondRoundOutput(DKGSecondRoundOutput),
    DwalletEncryptedUserShare(EncryptedUserShareOutput),
    DwalletSign(SignOutput),
    DwalletPresign(PresignOutput),
    DwalletPartialSignatureVerificationOutput(PartialSignatureVerificationOutput),
    DwalletMPCNetworkDKGOutput(Secp256K1NetworkKeyPublicOutputSlice),
    DwalletMPCNetworkReshareOutput(Secp256K1NetworkKeyPublicOutputSlice),
    MakeDWalletUserSecretKeySharesPublic(MakeDWalletUserSecretKeySharesPublicOutput),
    DWalletImportedKeyVerificationOutput(DWalletImportedKeyVerificationOutput),
}

impl MessageKind {
    pub fn name(&self) -> &'static str {
        match self {
            MessageKind::DwalletMPCNetworkDKGOutput(_) => "DwalletMPCNetworkDKGOutput",
            MessageKind::DwalletDKGFirstRoundOutput(_) => "DwalletDKGFirstRoundOutput",
            MessageKind::DwalletDKGSecondRoundOutput(_) => "DwalletDKGSecondRoundOutput",
            MessageKind::DwalletPresign(_) => "DwalletPresign",
            MessageKind::DwalletSign(_) => "DwalletSign",
            MessageKind::DwalletEncryptedUserShare(_) => "DwalletEncryptedUserShare",
            MessageKind::DwalletPartialSignatureVerificationOutput(_) => {
                "DwalletPartialSignatureVerificationOutput"
            }
            MessageKind::DwalletMPCNetworkReshareOutput(_) => "DwalletMPCNetworkReshareOutput",
            MessageKind::MakeDWalletUserSecretKeySharesPublic(_) => {
                "MakeDWalletUserSecretKeySharesPublic"
            }
            MessageKind::DWalletImportedKeyVerificationOutput(_) => {
                "DWalletImportedKeyVerificationOutput"
            }
        }
    }

    pub fn digest(&self) -> MessageDigest {
        MessageDigest::new(default_hash(self))
    }
}

impl Display for MessageKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();
        match &self {
            MessageKind::DwalletMPCNetworkDKGOutput(output) => {
                writeln!(
                    writer,
                    "MessageKind : DwalletMPCNetworkDKGOutput {:?}",
                    output
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
            MessageKind::DwalletPartialSignatureVerificationOutput(_) => {
                writeln!(
                    writer,
                    "MessageKind : DwalletPartialSignatureVerificationOutput"
                )?;
            }
            MessageKind::DwalletMPCNetworkReshareOutput(_) => {
                writeln!(writer, "MessageKind : DwalletMPCNetworkReshareOutput")?;
            }
            MessageKind::MakeDWalletUserSecretKeySharesPublic(_) => {
                writeln!(writer, "MessageKind : MakeDWalletUserSecretKeySharesPublic")?;
            }
            MessageKind::DWalletImportedKeyVerificationOutput(_) => {
                writeln!(writer, "MessageKind : DWalletImportedKeyVerificationOutput")?;
            }
        }
        write!(f, "{}", writer)
    }
}

impl Debug for MessageKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();
        match &self {
            MessageKind::DwalletMPCNetworkDKGOutput(output) => {
                writeln!(
                    writer,
                    "MessageKind : DwalletMPCNetworkDKGOutput {:?}",
                    output
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
            MessageKind::DwalletPartialSignatureVerificationOutput(_) => {
                writeln!(
                    writer,
                    "MessageKind : DwalletPartialSignatureVerificationOutput"
                )?;
            }
            MessageKind::DwalletMPCNetworkReshareOutput(_) => {
                writeln!(writer, "MessageKind : DwalletMPCNetworkReshareOutput")?;
            }
            MessageKind::MakeDWalletUserSecretKeySharesPublic(_) => {
                writeln!(writer, "MessageKind : MakeDWalletUserSecretKeySharesPublic")?;
            }
            MessageKind::DWalletImportedKeyVerificationOutput(_) => {
                writeln!(writer, "MessageKind : DWalletImportedKeyVerificationOutput")?;
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
