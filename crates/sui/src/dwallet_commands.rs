// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::fmt::Debug;
use std::time::Duration;

use anyhow::anyhow;
use bip32::secp256k1::elliptic_curve::rand_core::OsRng;
use clap::*;
use fastcrypto::encoding::Base64;
use fastcrypto::encoding::Encoding;
use move_core_types::language_storage::TypeTag;
use serde_json::{Number, Value};
use tokio::time::sleep;

use shared_crypto::intent::Intent;
use signature_mpc::twopc_mpc_protocols::{
    initiate_centralized_party_dkg, initiate_centralized_party_presign,
    initiate_centralized_party_sign, message_digest, verify_signature,
    PresignDecentralizedPartyOutput, ProtocolContext, SecretKeyShareEncryptionAndProof,
};
use sui_json::SuiJsonValue;
use sui_json_rpc_types::{
    ObjectChange, SuiData, SuiObjectDataFilter, SuiObjectResponseQuery,
    SuiTransactionBlockEffectsAPI,
};
use sui_json_rpc_types::{SuiExecutionStatus, SuiObjectDataOptions};
use sui_keys::keystore::AccountKeystore;
use sui_sdk::sui_client_config::DWalletSecretShare;
use sui_sdk::wallet_context::WalletContext;
use sui_types::base_types::ObjectRef;
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::signature_mpc::{
    DKGSessionOutput, DWallet, Presign, PresignSessionOutput, SignData, SignOutput,
    APPROVE_MESSAGES_FUNC_NAME, CREATE_DKG_SESSION_FUNC_NAME, CREATE_DWALLET_FUNC_NAME,
    CREATE_PARTIAL_USER_SIGNED_MESSAGES_FUNC_NAME, CREATE_PRESIGN_SESSION_FUNC_NAME,
    DKG_SESSION_STRUCT_NAME, DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME, DWALLET_MODULE_NAME,
    DWALLET_STRUCT_NAME, PRESIGN_SESSION_STRUCT_NAME, SIGN_FUNC_NAME, SIGN_SESSION_STRUCT_NAME,
};
use sui_types::transaction::{Argument, CallArg, ObjectArg};
use sui_types::{
    base_types::ObjectID,
    transaction::{SenderSignedData, Transaction, TransactionDataAPI},
    SUI_SYSTEM_PACKAGE_ID,
};

use crate::client_commands::{
    construct_move_call_transaction, NewDWalletOutput, NewSignOutput, SuiClientCommandResult,
};
use crate::serialize_or_execute;

#[derive(ValueEnum, Clone, Debug)]
pub enum Hash {
    KECCAK256,
    SHA256,
}

impl From<Hash> for signature_mpc::twopc_mpc_protocols::Hash {
    fn from(value: Hash) -> Self {
        match value {
            Hash::KECCAK256 => Self::KECCAK256,
            Hash::SHA256 => Self::SHA256,
        }
    }
}

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum SuiDWalletCommands {
    /// Create a new dWallet.
    #[command(name = "create")]
    Create {
        #[clap(long)]
        alias: String,
        /// ID of the gas object for gas payment, in 20 bytes Hex string
        /// If not provided, a gas object with at least `gas_budget` value will be selected.
        #[clap(long)]
        gas: Option<ObjectID>,

        /// Gas budget for this transfer
        #[clap(long)]
        gas_budget: u64,

        /// Instead of executing the transaction, serialize the bcs bytes of the unsigned transaction data
        /// (TransactionData) using base64 encoding, and print out the string.
        #[clap(long, required = false)]
        serialize_unsigned_transaction: bool,

        /// Instead of executing the transaction, serialize the bcs bytes of the signed transaction data
        /// (SenderSignedData) using base64 encoding, and print out the string.
        #[clap(long, required = false)]
        serialize_signed_transaction: bool,
    },
}

impl SuiDWalletCommands {
    pub async fn execute(
        self,
        context: &mut WalletContext,
    ) -> Result<SuiClientCommandResult, anyhow::Error> {
        let ret = Ok(match self {
            SuiDWalletCommands::Create {
                alias,
                gas,
                gas_budget,
                serialize_unsigned_transaction,
                serialize_signed_transaction,
            } => {
                if context.config.dwallets.iter().any(|d| d.alias == alias) {
                    return Err(anyhow!(
                        "dWallet config with the name [{alias}] already exists."
                    ));
                }

                // TODO: handle Errors instead of `unwrap`
                let centralized_party_commitment_round_party =
                    initiate_centralized_party_dkg().unwrap();

                let (
                    commitment_to_centralized_party_secret_key_share,
                    centralized_party_decommitment_round_party,
                ) = centralized_party_commitment_round_party
                    .sample_commit_and_prove_secret_key_share(&mut OsRng)
                    .unwrap();

                let commitment_to_centralized_party_secret_key_share =
                    bcs::to_bytes(&commitment_to_centralized_party_secret_key_share).unwrap();

                let gas_owner = context.try_get_object_owner(&gas).await?;
                let sender = gas_owner.unwrap_or(context.active_address()?);

                let client = context.get_client().await?;

                let mut pt_builder = ProgrammableTransactionBuilder::new();

                let commitment_to_centralized_party_secret_key_share = pt_builder
                    .input(CallArg::from(
                        &commitment_to_centralized_party_secret_key_share,
                    ))
                    .unwrap();
                let cap = pt_builder.programmable_move_call(
                    SUI_SYSTEM_PACKAGE_ID,
                    DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.into(),
                    CREATE_DKG_SESSION_FUNC_NAME.into(),
                    Vec::new(),
                    Vec::from([commitment_to_centralized_party_secret_key_share]),
                );
                pt_builder.transfer_arg(sender, cap);

                let tx_data = client
                    .transaction_builder()
                    .finish_programmable_transaction(sender, pt_builder, gas, gas_budget)
                    .await?;

                let session_response = serialize_or_execute!(
                    tx_data,
                    serialize_unsigned_transaction,
                    serialize_signed_transaction,
                    context,
                    Call
                );

                let SuiClientCommandResult::Call(session) = session_response else {
                    return Err(anyhow!("Can't get create dkg session response."));
                };

                let session_id = session
                    .object_changes
                    .unwrap()
                    .iter()
                    .find_map(|o| {
                        if let ObjectChange::Created {
                            object_id,
                            object_type,
                            ..
                        } = o
                        {
                            if object_type.address == SUI_SYSTEM_PACKAGE_ID.into()
                                && object_type.module == DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.into()
                                && object_type.name == DKG_SESSION_STRUCT_NAME.into()
                            {
                            }
                            return Some(object_id);
                        }
                        None
                    })
                    .unwrap()
                    .clone();

                sleep(Duration::from_millis(500)).await;

                let mut cursor = None;
                let mut output: Option<DKGSessionOutput>;
                loop {
                    let client = context.get_client().await?;
                    let response = client
                        .read_api()
                        .get_owned_objects(
                            context.active_address()?,
                            Some(SuiObjectResponseQuery::new(
                                Some(SuiObjectDataFilter::StructType(DKGSessionOutput::type_())),
                                Some(SuiObjectDataOptions::bcs_lossless()),
                            )),
                            cursor,
                            None,
                        )
                        .await?;

                    output = response.data.iter().find_map(|o| {
                        let move_object = o.move_object_bcs().unwrap();
                        let output = DKGSessionOutput::from_bcs_bytes(move_object).unwrap();
                        if output.session_id.bytes == session_id {
                            Some(output)
                        } else {
                            None
                        }
                    });

                    if output.is_some() {
                        break;
                    } else if response.has_next_page {
                        cursor = response.next_cursor;
                    } else {
                        cursor = None;
                    }
                }
                let output = output.unwrap();

                // let mut stream = client.read_api().subscribe_transaction(TransactionFilter::ToAddress(context.active_address()?)).await?;
                //
                // let mut output: Option<DKGSessionOutput> = None;
                // while let Some(effects) = stream.next().await {
                //     if let SuiTransactionBlockEffects::V1(effects) = effects? {
                //         let obj_ref = &effects.created[0];
                //
                //         let response = client
                //             .read_api()
                //             .get_object_with_options(
                //                 obj_ref.object_id(),
                //                 SuiObjectDataOptions::bcs_lossless(),
                //             )
                //             .await?;
                //
                //         output = response.data.iter().find_map(|o| {
                //             if let Some(bcs_object) = &o.bcs {
                //                 let move_object = bcs_object.try_as_move();
                //                 let output = move_object.map(|o| DKGSessionOutput::from_bcs_bytes(&o.bcs_bytes).ok()).flatten();
                //                 output.filter(|o| o.session_id.bytes == session_id)
                //             } else {
                //                 None
                //             }
                //         });
                //         if output.is_some() {
                //             break;
                //         }
                //     }
                // }
                // let output = output.unwrap();

                let secret_key_share_encryption_and_proof =
                    bcs::from_bytes::<SecretKeyShareEncryptionAndProof<ProtocolContext>>(
                        &output.secret_key_share_encryption_and_proof,
                    )?;

                let (
                    centralized_party_public_key_share_decommitment_and_proof,
                    centralized_party_dkg_output,
                ) = centralized_party_decommitment_round_party
                    .decommit_proof_public_key_share(
                        secret_key_share_encryption_and_proof,
                        &mut OsRng,
                    )
                    .unwrap();

                let public_key_share_decommitment_and_proof =
                    bcs::to_bytes(&centralized_party_public_key_share_decommitment_and_proof)
                        .unwrap();

                let public_key_share_decommitment_and_proof =
                    public_key_share_decommitment_and_proof
                        .iter()
                        .map(|v| Value::Number(Number::from(*v)))
                        .collect();

                let centralized_party_public_key_share_decommitment_and_proofs =
                    SuiJsonValue::new(Value::Array(public_key_share_decommitment_and_proof))
                        .unwrap();

                let tx_data = construct_move_call_transaction(
                    SUI_SYSTEM_PACKAGE_ID,
                    DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.as_str(),
                    &CREATE_DWALLET_FUNC_NAME.as_str(),
                    Vec::new(),
                    gas,
                    gas_budget,
                    Vec::from([
                        SuiJsonValue::from_object_id(*output.id.object_id()),
                        centralized_party_public_key_share_decommitment_and_proofs,
                    ]),
                    context,
                )
                .await?;

                let dwallet_response = serialize_or_execute!(
                    tx_data,
                    serialize_unsigned_transaction,
                    serialize_signed_transaction,
                    context,
                    Call
                );

                let SuiClientCommandResult::Call(dwallet) = dwallet_response else {
                    return Err(anyhow!("Can't get response."));
                };

                let dwallet_id = dwallet
                    .object_changes
                    .unwrap()
                    .iter()
                    .find_map(|o| {
                        if let ObjectChange::Created {
                            object_id,
                            object_type,
                            ..
                        } = o
                        {
                            if object_type.address == SUI_SYSTEM_PACKAGE_ID.into()
                                && object_type.module == DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.into()
                                && object_type.name == DWALLET_STRUCT_NAME.into()
                            {
                            }
                            return Some(object_id);
                        }
                        None
                    })
                    .unwrap()
                    .clone();

                let resp = context
                    .get_client()
                    .await?
                    .read_api()
                    .get_object_with_options(
                        dwallet_id,
                        SuiObjectDataOptions::default().with_bcs().with_owner(),
                    )
                    .await?;

                let Some(data) = resp.data else {
                    return Err(anyhow!("Could not find dwallet at {dwallet_id}"));
                };

                let dwallet: DWallet = data
                    .bcs
                    .ok_or_else(|| anyhow!("Fetch dwallet object but no data was returned"))?
                    .try_as_move()
                    .ok_or_else(|| anyhow!("dwallet is not a Move Object"))?
                    .deserialize()?;

                context.config.add_dwallet(DWalletSecretShare {
                    alias: alias.clone(),
                    dwallet_id,
                    dwallet_cap_id: dwallet.dwallet_cap_id.bytes.clone(),
                    dkg_output: centralized_party_dkg_output,
                });
                context.config.save()?;

                SuiClientCommandResult::NewDWallet(NewDWalletOutput {
                    alias,
                    dwallet_id,
                    dwallet_cap_id: dwallet.dwallet_cap_id.bytes,
                    public_key: Base64::encode(dwallet.public_key),
                })
            }
        });

        ret
    }
}
