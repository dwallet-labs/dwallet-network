// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::{
    fmt::{Debug, Display, Formatter, Write},
};
use std::time::Duration;

use anyhow::{anyhow};
use bip32::secp256k1::elliptic_curve::rand_core::OsRng;
use clap::*;
use fastcrypto::{
    encoding::{Base64},
    traits::ToFromBytes,
};
use fastcrypto::encoding::Encoding;
use move_core_types::language_storage::TypeTag;

use serde_json::{json, Number, Value};

use shared_crypto::intent::Intent;
use sui_json::SuiJsonValue;
use sui_json_rpc_types::{ObjectChange, RPCTransactionRequestParams, SuiData, SuiObjectData, SuiObjectDataFilter, SuiObjectResponse, SuiObjectResponseQuery, SuiParsedData, SuiRawData, SuiTransactionBlockEffects, SuiTransactionBlockEffectsAPI, SuiTransactionBlockResponse, SuiTransactionBlockResponseOptions, TransactionFilter};
use sui_json_rpc_types::{SuiExecutionStatus, SuiObjectDataOptions};
use sui_keys::keystore::AccountKeystore;
use sui_sdk::sui_client_config::{DWalletSecretShare, SuiClientConfig, SuiEnv};
use sui_sdk::wallet_context::WalletContext;
use sui_types::{base_types::{ObjectID,}, SUI_SYSTEM_PACKAGE_ID, transaction::{SenderSignedData, Transaction, TransactionData, TransactionDataAPI}};

use tokio::time::sleep;
use signature_mpc::twopc_mpc_protocols::{initiate_centralized_party_dkg, ProtocolContext, SecretKeyShareEncryptionAndProof, initiate_centralized_party_presign, PresignDecentralizedPartyOutput, initiate_centralized_party_sign, message_digest, verify_signature};
use sui_types::base_types::ObjectRef;
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::signature_mpc::{APPROVE_MESSAGES_FUNC_NAME, CREATE_DKG_SESSION_FUNC_NAME, CREATE_DWALLET_FUNC_NAME, CREATE_PRESIGN_SESSION_FUNC_NAME, DKG_SESSION_OUTPUT_STRUCT_NAME, DKG_SESSION_STRUCT_NAME, DKGSessionOutput, DWallet, DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME, DWALLET_MODULE_NAME, DWALLET_STRUCT_NAME, PRESIGN_SESSION_STRUCT_NAME, PresignSessionOutput, Presign, SignOutput, SIGN_SESSION_STRUCT_NAME, SIGN_MESSAGES_FUNC_NAME, CREATE_SIGN_MESSAGES_FUNC_NAME, SignData};
use sui_types::transaction::{Argument, CallArg, ObjectArg, TransactionKind};
use crate::client_commands::{construct_move_call_transaction, NewDWalletOutput, NewSignOutput, SuiClientCommandResult};
use crate::serialize_or_execute;

#[derive(ValueEnum, Clone, Debug)]
pub enum Hash {
    KECCAK256,
    SHA256
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
        /// If not provided, a gas object with at least gas_budget value will be selected
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

    /// Create a new dWallet.
    #[command(name = "sign")]
    Sign {

        /// A list of Base64 encoded messages to sign.
        #[clap(long)]
        messages: Vec<String>,

        /// The hash function, either "KECCAK256" (default) or "SHA256".
        #[clap(long, value_enum, default_value_t=Hash::KECCAK256)]
        hash: Hash,

        /// ID of the gas object for gas payment, in 20 bytes Hex string
        /// If not provided, a gas object with at least gas_budget value will be selected
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
                        "dWallet config with name [{alias}] already exists."
                    ));
                }

                // TODO: handle Errors instead of `unwrap`
                let centralized_party_commitment_round_party = initiate_centralized_party_dkg().unwrap();

                let (
                    commitment_to_centralized_party_secret_key_share,
                    centralized_party_decommitment_round_party,
                ) = centralized_party_commitment_round_party
                    .sample_commit_and_prove_secret_key_share(&mut OsRng)
                    .unwrap();

                let commitment_to_centralized_party_secret_key_share = bcs::to_bytes(&commitment_to_centralized_party_secret_key_share).unwrap();

                let gas_owner = context.try_get_object_owner(&gas).await?;
                let sender = gas_owner.unwrap_or(context.active_address()?);

                let client = context.get_client().await?;

                let mut pt_builder = ProgrammableTransactionBuilder::new();

                let commitment_to_centralized_party_secret_key_share = pt_builder.input(CallArg::from(&commitment_to_centralized_party_secret_key_share)).unwrap();
                let cap = pt_builder.programmable_move_call(
                    SUI_SYSTEM_PACKAGE_ID,
                    DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.into(),
                    CREATE_DKG_SESSION_FUNC_NAME.into(),
                    Vec::new(),
                    Vec::from([commitment_to_centralized_party_secret_key_share]),
                );
                pt_builder.transfer_arg(sender, cap);

                let tx_data = client
                    .transaction_builder().
                    finish_programmable_transaction(
                        sender,
                        pt_builder,
                        gas,
                        gas_budget
                    ).await?;

                let session_response = serialize_or_execute!(
                    tx_data,
                    serialize_unsigned_transaction,
                    serialize_signed_transaction,
                    context,
                    Call
                );

                let SuiClientCommandResult::Call(session) = session_response else {
                    return Err(anyhow!(
                            "Can't get create dkg session response."
                        ));
                };

                let session_id = session.object_changes.unwrap().iter().find_map(|o| {
                    if let ObjectChange::Created {
                        object_id,
                        object_type,
                        ..
                    } = o {
                        if object_type.address == SUI_SYSTEM_PACKAGE_ID.into() &&
                            object_type.module == DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.into() &&
                            object_type.name == DKG_SESSION_STRUCT_NAME.into() {
                        }
                        return Some(object_id)
                    }
                    None
                }).unwrap().clone();

                sleep(Duration::from_millis(500)).await;

                let mut cursor = None;
                let mut output: Option<DKGSessionOutput> = None;
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

                let secret_key_share_encryption_and_proof = bcs::from_bytes::<SecretKeyShareEncryptionAndProof<ProtocolContext>>(&output.secret_key_share_encryption_and_proof)?;

                let (
                    centralized_party_public_key_share_decommitment_and_proof,
                    centralized_party_dkg_output,
                ) = centralized_party_decommitment_round_party
                    .decommit_proof_public_key_share(secret_key_share_encryption_and_proof, &mut OsRng)
                    .unwrap();

                let public_key_share_decommitment_and_proof = bcs::to_bytes(&centralized_party_public_key_share_decommitment_and_proof).unwrap();

                let public_key_share_decommitment_and_proof = public_key_share_decommitment_and_proof.iter().map(|v| Value::Number(Number::from(*v))).collect();

                let centralized_party_public_key_share_decommitment_and_proofs = SuiJsonValue::new(Value::Array(public_key_share_decommitment_and_proof)).unwrap();

                let tx_data = construct_move_call_transaction(
                    SUI_SYSTEM_PACKAGE_ID, DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.as_str(), &CREATE_DWALLET_FUNC_NAME.as_str(), Vec::new(), gas, gas_budget, Vec::from([SuiJsonValue::from_object_id(*output.id.object_id()), centralized_party_public_key_share_decommitment_and_proofs]), context,
                ).await?;

                let dwallet_response = serialize_or_execute!(
                    tx_data,
                    serialize_unsigned_transaction,
                    serialize_signed_transaction,
                    context,
                    Call
                );

                let SuiClientCommandResult::Call(dwallet) = dwallet_response else {
                    return Err(anyhow!(
                            "Can't get response."
                        ));
                };

                let dwallet_id = dwallet.object_changes.unwrap().iter().find_map(|o| {
                    if let ObjectChange::Created {
                        object_id,
                        object_type,
                        ..
                    } = o {
                        if object_type.address == SUI_SYSTEM_PACKAGE_ID.into() &&
                            object_type.module == DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.into() &&
                            object_type.name == DWALLET_STRUCT_NAME.into() {
                        }
                        return Some(object_id)
                    }
                    None
                }).unwrap().clone();

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
                    return Err(anyhow!(
                        "Could not find dwallet at {dwallet_id}"
                    ));
                };

                let dwallet: DWallet = data
                    .bcs
                    .ok_or_else(|| {
                        anyhow!("Fetch dwallet object but no data was returned")
                    })?
                    .try_as_move()
                    .ok_or_else(|| anyhow!("dwallet is not a Move Object"))?
                    .deserialize()?;

                context.config.add_dwallet(
                    DWalletSecretShare {
                        alias: alias.clone(),
                        dwallet_id,
                        dwallet_cap_id: dwallet.dwallet_cap_id.bytes.clone(),
                        dkg_output: centralized_party_dkg_output,
                    }
                );
                context.config.save()?;

                SuiClientCommandResult::NewDWallet(NewDWalletOutput {
                    alias,
                    dwallet_id,
                    dwallet_cap_id: dwallet.dwallet_cap_id.bytes,
                    public_key: Base64::encode(dwallet.public_key)
                })
            }
            SuiDWalletCommands::Sign {
                messages,
                gas,
                gas_budget,
                serialize_unsigned_transaction,
                serialize_signed_transaction,
                hash
            } => {
                let hash: signature_mpc::twopc_mpc_protocols::Hash = hash.into();

                let DWalletSecretShare { alias: _, dkg_output, dwallet_id, dwallet_cap_id } = context.config.get_active_dwallet()?.clone();
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
                    return Err(anyhow!(
                        "Could not find dwallet at {dwallet_id}"
                    ));
                };

                let dwallet_ref = data.object_ref();

                let mut messages_vec = Vec::new();
                for m in messages {
                    messages_vec.push(
                        Base64::try_from(m)
                            .map_err(|e| anyhow!(e))?
                            .to_vec()
                            .map_err(|e| anyhow!(e))?,
                    );
                }

                let centralized_party_commitment_round_party = initiate_centralized_party_presign(dkg_output.clone()).unwrap();

                let (
                    centralized_party_nonce_shares_commitments_and_batched_proof,
                    centralized_party_proof_verification_round_party,
                ) = centralized_party_commitment_round_party
                    .sample_commit_and_prove_signature_nonce_share(messages_vec.len(), &mut OsRng)
                    .unwrap();

                let centralized_party_nonce_shares_commitments_and_batched_proof = bcs::to_bytes(&centralized_party_nonce_shares_commitments_and_batched_proof).unwrap();

                let centralized_party_nonce_shares_commitments_and_batched_proof = centralized_party_nonce_shares_commitments_and_batched_proof.iter().map(|v| Value::Number(Number::from(*v))).collect();

                let centralized_party_nonce_shares_commitments_and_batched_proof = SuiJsonValue::new(Value::Array(centralized_party_nonce_shares_commitments_and_batched_proof)).unwrap();

                let messages_vec_input = messages_vec.iter().map(|v| Value::Array(v.iter().map(|v| Value::Number(Number::from(*v))).collect::<Vec<_>>())).collect::<Vec<_>>();


                let messages_vec_input = SuiJsonValue::new(Value::Array(messages_vec_input)).unwrap();

                let gas_owner = context.try_get_object_owner(&gas).await?;
                let sender = gas_owner.unwrap_or(context.active_address()?);

                let client = context.get_client().await?;

                let hash_num: u8 = hash.clone().into();
                let mut pt_builder = ProgrammableTransactionBuilder::new();
                client.transaction_builder().single_move_call(
                    &mut pt_builder,
                    SUI_SYSTEM_PACKAGE_ID,
                    DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.as_str(),
                    CREATE_PRESIGN_SESSION_FUNC_NAME.as_str(),
                    Vec::new(),
                    Vec::from([SuiJsonValue::from_object_id(dwallet_id), messages_vec_input.clone(), centralized_party_nonce_shares_commitments_and_batched_proof, SuiJsonValue::new(Value::Number(Number::from(hash_num))).unwrap()])
                ).await?;

                // let dwallet_arg = pt_builder.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(dwallet_ref))).unwrap();
                // let centralized_party_nonce_shares_commitments_and_batched_proof_arg = pt_builder.input(CallArg::from(&centralized_party_nonce_shares_commitments_and_batched_proof)).unwrap();
                // let hash = pt_builder.input(CallArg::from(1u8)).unwrap();
                // pt_builder.programmable_move_call(
                //     SUI_SYSTEM_PACKAGE_ID,
                //     DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.into(),
                //     CREATE_PRESIGN_SESSION_FUNC_NAME.into(),
                //     Vec::new(),
                //     Vec::from([dwallet_arg, Argument::Result(0), centralized_party_nonce_shares_commitments_and_batched_proof_arg, hash]),
                // );

                let tx_data = client
                    .transaction_builder().
                    finish_programmable_transaction(
                        sender,
                        pt_builder,
                        gas,
                        gas_budget
                    ).await?;

                let session_response = serialize_or_execute!(
                    tx_data,
                    serialize_unsigned_transaction,
                    serialize_signed_transaction,
                    context,
                    Call
                );

                let SuiClientCommandResult::Call(session) = session_response else {
                    return Err(anyhow!(
                            "Can't get response."
                        ));
                };

                let (session_id, session_ref) = session.object_changes.unwrap().iter().find_map(|o| {
                    if let ObjectChange::Created {
                        object_id,
                        object_type,
                        ..
                    } = o {
                        if object_type.address == SUI_SYSTEM_PACKAGE_ID.into() &&
                            object_type.module == DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.into() &&
                            object_type.name == PRESIGN_SESSION_STRUCT_NAME.into() {
                        }
                        return Some((*object_id, o.object_ref()))
                    }
                    None
                }).unwrap().clone();

                sleep(Duration::from_millis(500)).await;

                let mut cursor = None;
                let mut output: Option<(PresignSessionOutput, ObjectRef)> = None;
                loop {
                    let client = context.get_client().await?;
                    let response = client
                        .read_api()
                        .get_owned_objects(
                            context.active_address()?,
                            Some(SuiObjectResponseQuery::new(
                                Some(SuiObjectDataFilter::StructType(PresignSessionOutput::type_())),
                                Some(SuiObjectDataOptions::bcs_lossless()),
                            )),
                            cursor,
                            None,
                        )
                        .await?;

                    output = response.data.iter().find_map(|o| {
                        let move_object = o.move_object_bcs().unwrap();
                        let output = PresignSessionOutput::from_bcs_bytes(move_object).unwrap();
                        if output.session_id.bytes == session_id {
                            Some((output, o.object_ref_if_exists().unwrap()))
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
                let (presign_output, presign_ref) = output.unwrap();

                let presign_output = bcs::from_bytes::<PresignDecentralizedPartyOutput<ProtocolContext>>(&presign_output.output)?;


                let centralized_party_presigns = centralized_party_proof_verification_round_party
                    .verify_presign_output(presign_output, &mut OsRng)
                    .unwrap();

                let centralized_party_sign_round_parties = initiate_centralized_party_sign(dkg_output.clone(), centralized_party_presigns).unwrap();
                let digests = messages_vec.iter().map(|message| message_digest(message, &hash)).collect::<Vec<_>>();
                let (public_nonce_encrypted_partial_signature_and_proofs, signature_verification_round_parties): (Vec<_>, Vec<_>) = digests.clone().into_iter().zip(centralized_party_sign_round_parties.into_iter()).map(|(m, party)| {
                    party
                        .evaluate_encrypted_partial_signature_prehash(m, &mut OsRng)
                        .unwrap()
                }).collect::<Vec<_>>().into_iter().unzip();

                let public_nonce_encrypted_partial_signature_and_proofs = bcs::to_bytes(&public_nonce_encrypted_partial_signature_and_proofs).unwrap();

                // let public_nonce_encrypted_partial_signature_and_proofs = public_nonce_encrypted_partial_signature_and_proofs.iter().map(|v| Value::Number(Number::from(*v))).collect();
                //
                // let public_nonce_encrypted_partial_signature_and_proofs = SuiJsonValue::new(Value::Array(public_nonce_encrypted_partial_signature_and_proofs)).unwrap();

                sleep(Duration::from_millis(500)).await;
                let mut cursor = None;
                let mut decentralized_presign: Option<(Presign, ObjectRef)> = None;
                loop {
                    let client = context.get_client().await?;
                    let response = client
                        .read_api()
                        .get_owned_objects(
                            context.active_address()?,
                            Some(SuiObjectResponseQuery::new(
                                Some(SuiObjectDataFilter::StructType(Presign::type_())),
                                Some(SuiObjectDataOptions::bcs_lossless()),
                            )),
                            cursor,
                            None,
                        )
                        .await?;

                    decentralized_presign = response.data.iter().find_map(|o| {
                        let move_object = o.move_object_bcs().unwrap();
                        let decentralized_presign = Presign::from_bcs_bytes(move_object).unwrap();
                        if decentralized_presign.session_id.bytes == session_id {

                            Some((decentralized_presign, o.object_ref_if_exists().unwrap()))
                        } else {
                            None
                        }
                    });

                    if decentralized_presign.is_some() {
                        break;
                    } else if response.has_next_page {
                        cursor = response.next_cursor;
                    } else {
                        cursor = None;
                    }
                }
                let (decentralized_presign, decentralized_presign_ref) = decentralized_presign.unwrap();

                let mut pt_builder = ProgrammableTransactionBuilder::new();
                client.transaction_builder().single_move_call(
                    &mut pt_builder,
                    SUI_SYSTEM_PACKAGE_ID,
                    DWALLET_MODULE_NAME.as_str(),
                    APPROVE_MESSAGES_FUNC_NAME.as_str(),
                    Vec::new(),
                    Vec::from([SuiJsonValue::from_object_id(dwallet_cap_id), messages_vec_input])
                ).await?;

                let dwallet_arg = pt_builder.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(dwallet_ref))).unwrap();
                let session_arg = pt_builder.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(session_ref))).unwrap();
                let presign_arg = pt_builder.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(presign_ref))).unwrap();
                let decentralized_presign_arg = pt_builder.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(decentralized_presign_ref))).unwrap();
                let public_nonce_encrypted_partial_signature_and_proofs = pt_builder.input(CallArg::from(&public_nonce_encrypted_partial_signature_and_proofs)).unwrap();
                pt_builder.programmable_move_call(
                    SUI_SYSTEM_PACKAGE_ID,
                    DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.into(),
                    CREATE_SIGN_MESSAGES_FUNC_NAME.into(),
                    Vec::new(),
                    Vec::from([dwallet_arg, session_arg, presign_arg, decentralized_presign_arg, public_nonce_encrypted_partial_signature_and_proofs]),
                );
                pt_builder.programmable_move_call(
                    SUI_SYSTEM_PACKAGE_ID,
                    DWALLET_MODULE_NAME.into(),
                    SIGN_MESSAGES_FUNC_NAME.into(),
                    Vec::from([TypeTag::Struct(Box::new(SignData::type_()))]),
                    Vec::from([Argument::Result(1), Argument::Result(0)]),
                );

                let tx_data = client
                    .transaction_builder().
                    finish_programmable_transaction(
                        sender,
                        pt_builder,
                        gas,
                        gas_budget
                    ).await?;

                let session_response = serialize_or_execute!(
                    tx_data,
                    serialize_unsigned_transaction,
                    serialize_signed_transaction,
                    context,
                    Call
                );

                let SuiClientCommandResult::Call(session) = session_response else {
                    return Err(anyhow!(
                            "Can't get response."
                        ));
                };

                let session_id = session.object_changes.unwrap().iter().find_map(|o| {
                    if let ObjectChange::Created {
                        object_id,
                        object_type,
                        ..
                    } = o {
                        if object_type.address == SUI_SYSTEM_PACKAGE_ID.into() &&
                            object_type.module == DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME.into() &&
                            object_type.name == SIGN_SESSION_STRUCT_NAME.into() {
                        }
                        return Some(object_id)
                    }
                    None
                }).unwrap().clone();


                sleep(Duration::from_millis(500)).await;
                let mut cursor = None;
                let mut sign_output: Option<SignOutput> = None;
                loop {
                    let client = context.get_client().await?;
                    let response = client
                        .read_api()
                        .get_owned_objects(
                            context.active_address()?,
                            Some(SuiObjectResponseQuery::new(
                                Some(SuiObjectDataFilter::StructType(SignOutput::type_())),
                                Some(SuiObjectDataOptions::bcs_lossless()),
                            )),
                            cursor,
                            None,
                        )
                        .await?;

                    sign_output = response.data.iter().find_map(|o| {
                        let move_object = o.move_object_bcs().unwrap();
                        let sign_output = SignOutput::from_bcs_bytes(move_object).unwrap();
                        if sign_output.session_id.bytes == session_id {
                            Some(sign_output)
                        } else {
                            None
                        }
                    });

                    if sign_output.is_some() {
                        break;
                    } else if response.has_next_page {
                        cursor = response.next_cursor;
                    } else {
                        cursor = None;
                    }
                }
                let sign_output = sign_output.unwrap();
                
                let is_valid = verify_signature(messages_vec, &hash, dkg_output.public_key.clone(), sign_output.signatures.clone());

                println!("is_valid: {}", is_valid);
                
                let signatures = sign_output.signatures.iter().map(|s| Base64::encode(s)).collect::<Vec<_>>();

                SuiClientCommandResult::NewSignOutput(NewSignOutput {
                    dwallet_id,
                    sign_output_id: sign_output.id.object_id().clone(),
                    signatures,
                })

            }
        });
        ret
    }
}
