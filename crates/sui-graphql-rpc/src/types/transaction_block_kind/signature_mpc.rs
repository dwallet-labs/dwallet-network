// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::types::move_type::MoveType;
use crate::types::object_read::ObjectRead;
use crate::types::sui_address::SuiAddress;
use crate::types::{base64::Base64, epoch::Epoch};
use async_graphql::*;
use sui_types::messages_signature_mpc::{
    SignatureMPCOutput as NativeSignatureMPCOutput,
    SignatureMPCOutputValue as NativeSignatureMPCOutputValue,
};

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct SignatureMPCOutputTransaction(pub NativeSignatureMPCOutput);

#[derive(Union, Clone, Eq, PartialEq)]
enum SignatureMPCOutputValue {
    DKG(DKG),
    PresignOutput(PresignOutput),
    Presign(Presign),
    Sign(Sign),
}

#[derive(SimpleObject, Clone, Eq, PartialEq)]
struct DKG {
    commitment_to_centralized_party_secret_key_share: Vec<u8>,
    secret_key_share_encryption_and_proof: Vec<u8>,
}
#[derive(SimpleObject, Clone, Eq, PartialEq)]
struct PresignOutput {
    output: Vec<u8>,
}
#[derive(SimpleObject, Clone, Eq, PartialEq)]
struct Presign {
    presigns: Vec<u8>,
}
#[derive(SimpleObject, Clone, Eq, PartialEq)]
struct Sign {
    sigs: Vec<Vec<u8>>,
}

/// System transaction to store the output of signature mpc dkg on-chain.
#[Object]
impl SignatureMPCOutputTransaction {
    /// Epoch of the transaction.
    async fn epoch(&self, ctx: &Context<'_>) -> Result<Option<Epoch>> {
        Epoch::query(ctx.data_unchecked(), Some(self.0.epoch))
            .await
            .extend()
    }

    /// The session_id of the dkg.
    async fn session_id(&self) -> Base64 {
        Base64::from(self.0.session_id.0.as_slice())
    }

    /// The session_ref of the dkg.
    async fn session_ref(&self) -> ObjectRead {
        ObjectRead(self.0.session_ref)
    }
}

impl From<NativeSignatureMPCOutputValue>
    for crate::types::transaction_block_kind::signature_mpc::SignatureMPCOutputValue
{
    fn from(pt: NativeSignatureMPCOutputValue) -> Self {
        use crate::types::transaction_block_kind::signature_mpc::SignatureMPCOutputValue as P;
        use NativeSignatureMPCOutputValue as N;
        match pt {
            N::DKG {
                commitment_to_centralized_party_secret_key_share, secret_key_share_encryption_and_proof
            } => crate::types::transaction_block_kind::signature_mpc::SignatureMPCOutputValue::DKG(crate::types::transaction_block_kind::signature_mpc::DKG {
                commitment_to_centralized_party_secret_key_share, secret_key_share_encryption_and_proof
            }),

            N::PresignOutput(output) => crate::types::transaction_block_kind::signature_mpc::SignatureMPCOutputValue::PresignOutput(crate::types::transaction_block_kind::signature_mpc::PresignOutput { output }),
            N::Presign(presigns) => crate::types::transaction_block_kind::signature_mpc::SignatureMPCOutputValue::Presign(crate::types::transaction_block_kind::signature_mpc::Presign { presigns }),
            N::Sign{sigs, .. } => crate::types::transaction_block_kind::signature_mpc::SignatureMPCOutputValue::Sign(crate::types::transaction_block_kind::signature_mpc::Sign { sigs }),
        }
    }
}
