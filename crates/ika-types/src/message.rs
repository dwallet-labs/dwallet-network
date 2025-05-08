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
    pub output: Vec<u8>,
    pub encrypted_centralized_secret_share_and_proof: Vec<u8>,
    pub encryption_key_address: Vec<u8>,
    pub rejected: bool,
    pub session_sequence_number: u64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct PresignOutput {
    pub dwallet_id: Vec<u8>,
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

// Note: the order of these fields, and the number must correspond to the Move code in
// `dwallet_2pc_mpc_secp256k1_inner.move`.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, IntoStaticStr)]
pub enum DwalletCheckpointMessageKind {
    DwalletDKGFirstRoundOutput(DKGFirstRoundOutput),
    DwalletDKGSecondRoundOutput(DKGSecondRoundOutput),
    DwalletEncryptedUserShare(EncryptedUserShareOutput),
    DwalletSign(SignOutput),
    DwalletPresign(PresignOutput),
    DwalletPartialSignatureVerificationOutput(PartialSignatureVerificationOutput),
    DwalletMPCNetworkDKGOutput(Secp256K1NetworkKeyPublicOutputSlice),
    DwalletMPCNetworkReshareOutput(Secp256K1NetworkKeyPublicOutputSlice),
}

// Note: the order of these fields, and the number must correspond to the Move code in
// `dwallet_2pc_mpc_secp256k1_inner.move`.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, IntoStaticStr)]
pub enum SystemCheckpointMessageKind {
    ParamsMessage, // or rename to protocol message
}

impl DwalletCheckpointMessageKind {
    pub fn name(&self) -> &'static str {
        match self {
            DwalletCheckpointMessageKind::DwalletMPCNetworkDKGOutput(_) => {
                "DwalletMPCNetworkDKGOutput"
            }
            DwalletCheckpointMessageKind::DwalletDKGFirstRoundOutput(_) => {
                "DwalletDKGFirstRoundOutput"
            }
            DwalletCheckpointMessageKind::DwalletDKGSecondRoundOutput(_) => {
                "DwalletDKGSecondRoundOutput"
            }
            DwalletCheckpointMessageKind::DwalletPresign(_) => "DwalletPresign",
            DwalletCheckpointMessageKind::DwalletSign(_) => "DwalletSign",
            DwalletCheckpointMessageKind::DwalletEncryptedUserShare(_) => {
                "DwalletEncryptedUserShare"
            }
            DwalletCheckpointMessageKind::DwalletPartialSignatureVerificationOutput(_) => {
                "DwalletPartialSignatureVerificationOutput"
            }
            DwalletCheckpointMessageKind::DwalletMPCNetworkReshareOutput(_) => {
                "DwalletMPCNetworkReshareOutput"
            }
        }
    }

    pub fn digest(&self) -> MessageDigest {
        MessageDigest::new(default_hash(self))
    }
}

impl Display for DwalletCheckpointMessageKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();
        match &self {
            DwalletCheckpointMessageKind::DwalletMPCNetworkDKGOutput(output) => {
                writeln!(
                    writer,
                    "MessageKind : DwalletMPCNetworkDKGOutput {:?}",
                    output
                )?;
            }
            DwalletCheckpointMessageKind::DwalletDKGFirstRoundOutput(_) => {
                writeln!(writer, "MessageKind : DwalletDKGFirstRoundOutput")?;
            }
            DwalletCheckpointMessageKind::DwalletDKGSecondRoundOutput(_) => {
                writeln!(writer, "MessageKind : DwalletDKGSecondRoundOutput")?;
            }
            DwalletCheckpointMessageKind::DwalletPresign(_) => {
                writeln!(writer, "MessageKind : DwalletPresign")?;
            }
            DwalletCheckpointMessageKind::DwalletSign(_) => {
                writeln!(writer, "MessageKind : DwalletSign")?;
            }
            DwalletCheckpointMessageKind::DwalletEncryptedUserShare(_) => {
                writeln!(writer, "MessageKind : DwalletEncryptedUserShare")?;
            }
            DwalletCheckpointMessageKind::DwalletPartialSignatureVerificationOutput(_) => {
                writeln!(
                    writer,
                    "MessageKind : DwalletPartialSignatureVerificationOutput"
                )?;
            }
            DwalletCheckpointMessageKind::DwalletMPCNetworkReshareOutput(_) => {
                writeln!(writer, "MessageKind : DwalletMPCNetworkReshareOutput")?;
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
