// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::{error::Error, types::execution_result::ExecutionResult};
use async_graphql::*;
use fastcrypto::encoding::Encoding;
use fastcrypto::{encoding::Base64, traits::ToFromBytes};
use sui_json_rpc_types::SuiTransactionBlockResponseOptions;
use sui_sdk::SuiClient;
use sui_types::quorum_driver_types::ExecuteTransactionRequestType;
use sui_types::{signature::GenericSignature, transaction::Transaction};

pub struct Mutation;

#[Object]
impl Mutation {
    /// Execute a transaction, committing its effects on chain.
    ///
    /// `txBytes` is a `TransactionData` struct that has been BCS-encoded
    ///     and then Base64-encoded.
    /// `signatures` are a list of `flag || signature || pubkey` bytes,
    ///     Base64-encoded.
    ///
    /// Waits until the transaction has been finalized on chain to return
    /// its transaction digest.  If the transaction could not be
    /// finalized, returns the errors that prevented it, instead.
    async fn execute_transaction_block(
        &self,
        ctx: &Context<'_>,
        tx_bytes: String,
        signatures: Vec<String>,
    ) -> Result<ExecutionResult> {
        let sui_sdk_client: &Option<SuiClient> = ctx
            .data()
            .map_err(|_| Error::Internal("Unable to fetch Sui SDK client".to_string()))
            .extend()?;
        let sui_sdk_client = sui_sdk_client
            .as_ref()
            .ok_or_else(|| Error::Internal("Sui SDK client not initialized".to_string()))
            .extend()?;
        let tx_data = bcs::from_bytes(
            &Base64::decode(&tx_bytes)
                .map_err(|e| {
                    Error::Client(format!(
                        "Unable to deserialize transaction bytes from Base64: {e}"
                    ))
                })
                .extend()?,
        )
        .map_err(|e| {
            Error::Client(format!(
                "Unable to deserialize transaction bytes as BCS: {e}"
            ))
        })
        .extend()?;

        let mut sigs = Vec::new();
        for sig in signatures {
            sigs.push(
                GenericSignature::from_bytes(
                    &Base64::decode(&sig)
                        .map_err(|e| {
                            Error::Client(format!(
                                "Unable to deserialize signature bytes {sig} from Base64: {e}"
                            ))
                        })
                        .extend()?,
                )
                .map_err(|e| Error::Client(format!("Unable to create signature from bytes: {e}")))
                .extend()?,
            );
        }
        let transaction = Transaction::from_generic_sig_data(tx_data, sigs);

        let result = sui_sdk_client
            .quorum_driver_api()
            .execute_transaction_block(
                transaction,
                SuiTransactionBlockResponseOptions::default(),
                Some(ExecuteTransactionRequestType::WaitForEffectsCert),
            )
            .await
            // TODO: use proper error type as this could be a client error or internal error
            // depending on the specific error returned
            .map_err(|e| Error::Internal(format!("Unable to execute transaction: {e}")))
            .extend()?;

        Ok(ExecutionResult {
            errors: if result.errors.is_empty() {
                None
            } else {
                Some(result.errors)
            },
            digest: result.digest.to_string(),
        })
    }
}
