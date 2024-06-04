// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::{
    fmt::{Debug, Display, Formatter, Write},
    path::PathBuf,
    sync::Arc,
};
use std::time::Duration;

use anyhow::{anyhow, bail, ensure};
use bip32::DerivationPath;
use bip32::secp256k1::elliptic_curve::rand_core::OsRng;
use clap::*;
use colored::Colorize;
use fastcrypto::{
    encoding::{Base64, Encoding},
    traits::ToFromBytes,
};

use json_to_table::json_to_table;
use move_core_types::language_storage::{StructTag, TypeTag};
use move_package::BuildConfig as MoveBuildConfig;
use prometheus::Registry;
use serde::Serialize;
use serde_json::{json, Number, Value};
use sui_move::build::resolve_lock_file_path;
use sui_protocol_config::ProtocolConfig;
use sui_source_validation::{BytecodeSourceVerifier, SourceMode};

use shared_crypto::intent::Intent;
use sui_execution::verifier::VerifierOverrides;
use sui_json::SuiJsonValue;
use sui_json_rpc_types::{DynamicFieldPage, MoveCallParams, ObjectChange, RPCTransactionRequestParams, SuiData, SuiObjectData, SuiObjectDataFilter, SuiObjectResponse, SuiObjectResponseQuery, SuiParsedData, SuiRawData, SuiTransactionBlockEffects, SuiTransactionBlockEffectsAPI, SuiTransactionBlockResponse, SuiTransactionBlockResponseOptions};
use sui_json_rpc_types::{SuiExecutionStatus, SuiObjectDataOptions};
use sui_keys::keystore::AccountKeystore;
use sui_move_build::{
    build_from_resolution_graph, check_invalid_dependencies, check_unpublished_dependencies,
    gather_published_ids, BuildConfig, CompiledPackage, PackageDependencies, PublishedAtError,
};
use sui_replay::ReplayToolCommand;
use sui_sdk::sui_client_config::{DWalletSecretShare, SuiClientConfig, SuiEnv};
use sui_sdk::wallet_context::WalletContext;
use sui_sdk::SuiClient;
use sui_types::{base_types::{ObjectID, SequenceNumber, SuiAddress}, crypto::SignatureScheme, digests::TransactionDigest, dynamic_field::DynamicFieldInfo, error::SuiError, gas_coin::GasCoin, metrics::BytecodeVerifierMetrics, move_package::UpgradeCap, object::Owner, parse_sui_type_tag, signature::GenericSignature, SUI_SYSTEM_PACKAGE_ID, transaction::{SenderSignedData, Transaction, TransactionData, TransactionDataAPI}};

use tabled::{
    builder::Builder as TableBuilder,
    settings::{
        object::Cell as TableCell, style::HorizontalLine, Border as TableBorder,
        Modify as TableModify, Panel as TablePanel, Style as TableStyle,
    },
};
use tokio::time::sleep;
use tracing::info;
use signature_mpc::twopc_mpc_protocols::{initiate_centralized_party_dkg, SecretKeyShareEncryptionAndProof, EncryptedDecentralizedPartySecretKeyShareValue, initiate_centralized_party_presign, DKGCentralizedPartyOutput, PresignDecentralizedPartyOutput, initiate_centralized_party_sign, message_digest};
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::signature_mpc::{APPROVE_MESSAGES_FUNC_NAME, CREATE_DKG_SESSION_FUNC_NAME, CREATE_DWALLET_FUNC_NAME, CREATE_PRESIGN_SESSION_FUNC_NAME, DKG_SESSION_OUTPUT_STRUCT_NAME, DKG_SESSION_STRUCT_NAME, DKGSessionOutput, DWallet, DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME, DWALLET_MODULE_NAME, DWALLET_STRUCT_NAME, PRESIGN_SESSION_STRUCT_NAME, PresignSessionOutput, Presign};
use sui_types::transaction::{Argument, CallArg, ObjectArg, TransactionKind};
use crate::dwallet_commands::SuiDWalletCommands;
use crate::eth_dwallet_commands::{create_eth_dwallet, eth_approve_message, init_ethereum_state};

use crate::key_identity::{get_identity_address, KeyIdentity};
use crate::sui_commands::SuiCommand;

#[macro_export]
macro_rules! serialize_or_execute {
    ($tx_data:expr, $serialize_unsigned:expr, $serialize_signed:expr, $context:expr, $result_variant:ident) => {{
        assert!(
            !$serialize_unsigned || !$serialize_signed,
            "Cannot specify both --serialize-unsigned-transaction and --serialize-signed-transaction"
        );
        if $serialize_unsigned {
            SuiClientCommandResult::SerializedUnsignedTransaction($tx_data)
        } else {
            let signature = $context.config.keystore.sign_secure(
                &$tx_data.sender(),
                &$tx_data,
                Intent::sui_transaction(),
            )?;
            let sender_signed_data = SenderSignedData::new_from_sender_signature(
                $tx_data,
                Intent::sui_transaction(),
                signature,
            );
            if $serialize_signed {
                SuiClientCommandResult::SerializedSignedTransaction(sender_signed_data)
            } else {
                let transaction = Transaction::new(sender_signed_data);
                let response = $context.execute_transaction_may_fail(transaction).await?;
                let effects = response.effects.as_ref().ok_or_else(|| {
                    anyhow!("Effects from SuiTransactionBlockResult should not be empty")
                })?;
                if matches!(effects.status(), SuiExecutionStatus::Failure { .. }) {
                    return Err(anyhow!(
                        "Error executing transaction: {:#?}",
                        effects.status()
                    ));
                }
                SuiClientCommandResult::$result_variant(response)
            }
        }
    }};
}

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum SuiClientCommands {
    /// Default address used for commands when none specified
    #[clap(name = "active-address")]
    ActiveAddress,
    /// Default environment used for commands when none specified
    #[clap(name = "active-env")]
    ActiveEnv,
    /// Obtain the Addresses managed by the client.
    #[clap(name = "addresses")]
    Addresses,

    /// Call Move function
    #[clap(name = "call")]
    Call {
        /// Object ID of the package, which contains the module
        #[clap(long)]
        package: ObjectID,
        /// The name of the module in the package
        #[clap(long)]
        module: String,
        /// Function name in module
        #[clap(long)]
        function: String,
        /// Type arguments to the generic function being called.
        /// All must be specified, or the call will fail.
        #[clap(
            long,
            value_parser = parse_sui_type_tag,
            num_args(1..),
        )]
        type_args: Vec<TypeTag>,
        /// Simplified ordered args like in the function syntax
        /// ObjectIDs, Addresses must be hex strings
        #[clap(long, num_args(1..))]
        args: Vec<SuiJsonValue>,
        /// ID of the gas object for gas payment, in 20 bytes Hex string
        #[clap(long)]
        /// If not provided, a gas object with at least gas_budget value will be selected
        #[clap(long)]
        gas: Option<ObjectID>,
        /// Gas budget for this call
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

    /// Query the chain identifier from the rpc endpoint.
    #[clap(name = "chain-identifier")]
    ChainIdentifier,

    /// Query a dynamic field by its address.
    #[clap(name = "dynamic-field")]
    DynamicFieldQuery {
        ///The ID of the parent object
        #[clap(name = "object_id")]
        id: ObjectID,
        /// Optional paging cursor
        #[clap(long)]
        cursor: Option<ObjectID>,
        /// Maximum item returned per page
        #[clap(long, default_value = "50")]
        limit: usize,
    },

    /// List all Sui environments
    Envs,

    /// Execute a Signed Transaction. This is useful when the user prefers to sign elsewhere and use this command to execute.
    ExecuteSignedTx {
        /// BCS serialized transaction data bytes without its type tag, as base-64 encoded string.
        #[clap(long)]
        tx_bytes: String,

        /// A list of Base64 encoded signatures `flag || signature || pubkey`.
        #[clap(long)]
        signatures: Vec<String>,
    },

    /// Obtain all gas objects owned by the address.
    /// An address' alias can be used instead of the address.
    #[clap(name = "gas")]
    Gas {
        /// Address (or its alias) owning the objects
        #[clap(name = "owner_address")]
        #[arg(value_parser)]
        address: Option<KeyIdentity>,
    },

    /// Merge two coin objects into one coin
    MergeCoin {
        /// The address of the coin to merge into.
        #[clap(long)]
        primary_coin: ObjectID,
        /// The address of the coin to be merged.
        #[clap(long)]
        coin_to_merge: ObjectID,
        /// The address of the gas object for gas payment.
        /// If not provided, a gas object with at least gas_budget value will be selected.
        #[clap(long)]
        gas: Option<ObjectID>,
        /// Gas budget for this call
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

    /// Generate new address and keypair with keypair scheme flag {ed25519 | secp256k1 | secp256r1}
    /// with optional derivation path, default to m/44'/784'/0'/0'/0' for ed25519 or
    /// m/54'/784'/0'/0/0 for secp256k1 or m/74'/784'/0'/0/0 for secp256r1. Word length can be
    /// { word12 | word15 | word18 | word21 | word24} default to word12 if not specified.
    #[clap(name = "new-address")]
    NewAddress {
        key_scheme: SignatureScheme,
        /// The alias must start with a letter and can contain only letters, digits, hyphens (-), or underscores (_).
        alias: Option<String>,
        word_length: Option<String>,
        derivation_path: Option<DerivationPath>,
    },

    /// Add new Sui environment.
    #[clap(name = "new-env")]
    NewEnv {
        #[clap(long)]
        alias: String,
        #[clap(long, value_hint = ValueHint::Url)]
        rpc: String,
        #[clap(long, value_hint = ValueHint::Url)]
        ws: Option<String>,
        #[clap(long, value_hint = ValueHint::Url)]
        eth_execution_rpc: Option<String>,
        #[clap(long, value_hint = ValueHint::Url)]
        eth_consensus_rpc: Option<String>,
        eth_chain_id: Option<u64>,
        eth_genesis_time: Option<u64>,
        eth_genesis_validators_root: Option<String>,
        state_object_id: Option<ObjectID>,
    },

    /// Get object info
    #[clap(name = "object")]
    Object {
        /// Object ID of the object to fetch
        #[clap(name = "object_id")]
        id: ObjectID,

        /// Return the bcs serialized version of the object
        #[clap(long)]
        bcs: bool,
    },
    /// Obtain all objects owned by the address. It also accepts an address by its alias.
    #[clap(name = "objects")]
    Objects {
        /// Address owning the object. If no address is provided, it will show all
        /// objects owned by `sui client active-address`.
        #[clap(name = "owner_address")]
        address: Option<KeyIdentity>,
    },
    /// Pay coins to recipients following specified amounts, with input coins.
    /// Length of recipients must be the same as that of amounts.
    #[clap(name = "pay")]
    Pay {
        /// The input coins to be used for pay recipients, following the specified amounts.
        #[clap(long, num_args(1..))]
        input_coins: Vec<ObjectID>,

        /// The recipient addresses, must be of same length as amounts
        /// Aliases of addresses are also accepted as input.
        #[clap(long, num_args(1..))]
        recipients: Vec<KeyIdentity>,

        /// The amounts to be paid, following the order of recipients.
        #[clap(long, num_args(1..))]
        amounts: Vec<u64>,

        /// ID of the gas object for gas payment, in 20 bytes Hex string
        /// If not provided, a gas object with at least gas_budget value will be selected
        #[clap(long)]
        gas: Option<ObjectID>,

        /// Gas budget for this transaction
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

    /// Pay all residual SUI coins to the recipient with input coins, after deducting the gas cost.
    /// The input coins also include the coin for gas payment, so no extra gas coin is required.
    PayAllSui {
        /// The input coins to be used for pay recipients, including the gas coin.
        #[clap(long, num_args(1..))]
        input_coins: Vec<ObjectID>,

        /// The recipient address (or its alias if it's an address in the keystore).
        #[clap(long)]
        recipient: KeyIdentity,

        /// Gas budget for this transaction
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

    /// Pay SUI coins to recipients following following specified amounts, with input coins.
    /// Length of recipients must be the same as that of amounts.
    /// The input coins also include the coin for gas payment, so no extra gas coin is required.
    PaySui {
        /// The input coins to be used for pay recipients, including the gas coin.
        #[clap(long, num_args(1..))]
        input_coins: Vec<ObjectID>,

        /// The recipient addresses, must be of same length as amounts.
        /// Aliases of addresses are also accepted as input.
        #[clap(long, num_args(1..))]
        recipients: Vec<KeyIdentity>,

        /// The amounts to be paid, following the order of recipients.
        #[clap(long, num_args(1..))]
        amounts: Vec<u64>,

        /// Gas budget for this transaction
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

    /// Publish Move modules
    #[clap(name = "publish")]
    Publish {
        /// Path to directory containing a Move package
        #[clap(name = "package_path", global = true, default_value = ".")]
        package_path: PathBuf,

        /// Package build options
        #[clap(flatten)]
        build_config: MoveBuildConfig,

        /// ID of the gas object for gas payment, in 20 bytes Hex string
        /// If not provided, a gas object with at least gas_budget value will be selected
        #[clap(long)]
        gas: Option<ObjectID>,

        /// Gas budget for running module initializers
        #[clap(long)]
        gas_budget: u64,

        /// Publish the package without checking whether compiling dependencies from source results
        /// in bytecode matching the dependencies found on-chain.
        #[clap(long)]
        skip_dependency_verification: bool,

        /// Also publish transitive dependencies that have not already been published.
        #[clap(long)]
        with_unpublished_dependencies: bool,

        /// Instead of executing the transaction, serialize the bcs bytes of the unsigned transaction data
        /// (TransactionData) using base64 encoding, and print out the string.
        #[clap(long, required = false)]
        serialize_unsigned_transaction: bool,

        /// Instead of executing the transaction, serialize the bcs bytes of the signed transaction data
        /// (SenderSignedData) using base64 encoding, and print out the string.
        #[clap(long, required = false)]
        serialize_signed_transaction: bool,
    },

    /// Split a coin object into multiple coins.
    #[clap(group(ArgGroup::new("split").required(true).args(&["amounts", "count"])))]
    SplitCoin {
        /// Coin to Split, in 20 bytes Hex string
        #[clap(long)]
        coin_id: ObjectID,
        /// Specific amounts to split out from the coin
        #[clap(long, num_args(1..))]
        amounts: Option<Vec<u64>>,
        /// Count of equal-size coins to split into
        #[clap(long)]
        count: Option<u64>,
        /// ID of the gas object for gas payment, in 20 bytes Hex string
        /// If not provided, a gas object with at least gas_budget value will be selected
        #[clap(long)]
        gas: Option<ObjectID>,
        /// Gas budget for this call
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

    /// Switch active address and network(e.g., devnet, local rpc server).
    #[clap(name = "switch")]
    Switch {
        /// An address to be used as the active address for subsequent
        /// commands. It accepts also the alias of the address.
        #[clap(long)]
        address: Option<KeyIdentity>,
        /// The RPC server URL (e.g., local rpc server, devnet rpc server, etc) to be
        /// used for subsequent commands.
        #[clap(long)]
        env: Option<String>,
        /// The dWallet alias to be used for subsequent commands.
        #[clap(long)]
        dwallet: Option<String>,
    },

    /// Get the effects of executing the given transaction block
    #[clap(name = "tx-block")]
    TransactionBlock {
        /// Digest of the transaction block
        #[clap(name = "digest")]
        digest: TransactionDigest,
    },

    /// Transfer object
    #[clap(name = "transfer")]
    Transfer {
        /// Recipient address (or its alias if it's an address in the keystore)
        #[clap(long)]
        to: KeyIdentity,

        /// Object to transfer, in 20 bytes Hex string
        #[clap(long)]
        object_id: ObjectID,

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

    /// Transfer SUI, and pay gas with the same SUI coin object.
    /// If amount is specified, only the amount is transferred; otherwise the entire object
    /// is transferred.
    #[clap(name = "transfer-dwlt")]
    TransferSui {
        /// Recipient address (or its alias if it's an address in the keystore)
        #[clap(long)]
        to: KeyIdentity,

        /// Sui coin object to transfer, ID in 20 bytes Hex string. This is also the gas object.
        #[clap(long)]
        sui_coin_object_id: ObjectID,

        /// Gas budget for this transfer
        #[clap(long)]
        gas_budget: u64,

        /// The amount to transfer, if not specified, the entire coin object will be transferred.
        #[clap(long)]
        amount: Option<u64>,

        /// Instead of executing the transaction, serialize the bcs bytes of the unsigned transaction data
        /// (TransactionData) using base64 encoding, and print out the string.
        #[clap(long, required = false)]
        serialize_unsigned_transaction: bool,

        /// Instead of executing the transaction, serialize the bcs bytes of the signed transaction data
        /// (SenderSignedData) using base64 encoding, and print out the string.
        #[clap(long, required = false)]
        serialize_signed_transaction: bool,
    },

    /// Upgrade Move modules
    #[clap(name = "upgrade")]
    Upgrade {
        /// Path to directory containing a Move package
        #[clap(name = "package_path", global = true, default_value = ".")]
        package_path: PathBuf,

        /// ID of the upgrade capability for the package being upgraded.
        #[clap(long)]
        upgrade_capability: ObjectID,

        /// Package build options
        #[clap(flatten)]
        build_config: MoveBuildConfig,

        /// ID of the gas object for gas payment, in 20 bytes Hex string
        /// If not provided, a gas object with at least gas_budget value will be selected
        #[clap(long)]
        gas: Option<ObjectID>,

        /// Gas budget for running module initializers
        #[clap(long)]
        gas_budget: u64,

        /// Publish the package without checking whether compiling dependencies from source results
        /// in bytecode matching the dependencies found on-chain.
        #[clap(long)]
        skip_dependency_verification: bool,

        /// Also publish transitive dependencies that have not already been published.
        #[clap(long)]
        with_unpublished_dependencies: bool,

        /// Instead of executing the transaction, serialize the bcs bytes of the unsigned transaction data
        /// (TransactionData) using base64 encoding, and print out the string.
        #[clap(long, required = false)]
        serialize_unsigned_transaction: bool,

        /// Instead of executing the transaction, serialize the bcs bytes of the signed transaction data
        /// (SenderSignedData) using base64 encoding, and print out the string.
        #[clap(long, required = false)]
        serialize_signed_transaction: bool,
    },

    /// Run the bytecode verifier on the package
    #[clap(name = "verify-bytecode-meter")]
    VerifyBytecodeMeter {
        /// Path to directory containing a Move package
        #[clap(name = "package_path", global = true, default_value = ".")]
        package_path: PathBuf,

        /// Package build options
        #[clap(flatten)]
        build_config: MoveBuildConfig,
    },

    /// Verify local Move packages against on-chain packages, and optionally their dependencies.
    #[clap(name = "verify-source")]
    VerifySource {
        /// Path to directory containing a Move package
        #[clap(name = "package_path", global = true, default_value = ".")]
        package_path: PathBuf,

        /// Package build options
        #[clap(flatten)]
        build_config: MoveBuildConfig,

        /// Verify on-chain dependencies.
        #[clap(long)]
        verify_deps: bool,

        /// Don't verify source (only valid if --verify-deps is enabled).
        #[clap(long)]
        skip_source: bool,

        /// If specified, override the addresses for the package's own modules with this address.
        /// Only works for unpublished modules (whose addresses are currently 0x0).
        #[clap(long)]
        address_override: Option<ObjectID>,
    },

    /// Replay a given transaction to view transaction effects. Set environment variable MOVE_VM_STEP=1 to debug.
    #[clap(name = "replay-transaction")]
    ReplayTransaction {
        /// The digest of the transaction to replay
        #[arg(long, short)]
        tx_digest: String,
    },

    /// Replay transactions listed in a file.
    #[clap(name = "replay-batch")]
    ReplayBatch {
        /// The path to the file of transaction digests to replay, with one digest per line
        #[arg(long, short)]
        path: PathBuf,

        /// If an error is encountered during a transaction, this specifies whether to terminate or continue
        #[arg(long, short)]
        terminate_early: bool,
    },

    /// Replay all transactions in a range of checkpoints.
    #[command(name = "replay-checkpoint")]
    ReplayCheckpoints {
        /// The starting checkpoint sequence number of the range of checkpoints to replay
        #[arg(long, short)]
        start: u64,

        /// The ending checkpoint sequence number of the range of checkpoints to replay
        #[arg(long, short)]
        end: u64,

        /// If an error is encountered during a transaction, this specifies whether to terminate or continue
        #[arg(long, short)]
        terminate_early: bool,
    },

    /// dWallet subcommands.
    #[command(name = "dwallet")]
    DWallet {
        #[clap(subcommand)]
        cmd: Option<SuiDWalletCommands>,
    },

    /// Connect dWallet to be controlled by Eth contract.
    #[command(name = "dwallet-connect-eth")]
    CreateEthDwallet {
        /// The ObjectID of the dWallet *cap*ability.
        #[clap(long)]
        dwallet_cap_id: ObjectID,
        /// The address of the contract.
        #[clap(long)]
        smart_contract_address: String,
        /// The slot of the Data structure that holds approved transactions in eth smart contract.
        #[clap(long)]
        // todo(zeev): change the clap name to something shorter.
        smart_contract_approved_tx_slot: u64,
        /// The address of the gas object for gas payment.
        #[clap(long)]
        gas: Option<ObjectID>,
        /// Gas budget for this call
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

    /// Approve a TX with Eth contract for a given dWallet.
    #[command(name = "dwallet-eth-verify")]
    EthApproveMessage {
        #[clap(long)]
        /// Object of a [EthDwalletCap]
        eth_dwallet_cap_id: ObjectID,
        /// The Message (TX) to verify.
        #[clap(long)]
        message: String,
        /// The DWallet ID
        // todo(zeev): in the future we can fetch the dwallet ID.
        #[clap(long)]
        dwallet_id: ObjectID,
        #[clap(long)]
        gas: Option<ObjectID>,
        /// Gas budget for this call
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

    /// Initiate the LatestEthereumState struct in the DWallet module.
    #[command(name = "init-eth-state")]
    InitEthState {
        #[clap(long)]
        checkpoint: String,
        #[clap(long)]
        gas: Option<ObjectID>,
        /// Gas budget for this call
        #[clap(long)]
        gas_budget: u64,
        /// Instead of executing the transaction, serialize the bcs bytes of the unsigned transaction data
        /// (TransactionData) using base64 encoding, and print out the string.
        #[clap(long, required = false)]
        serialize_unsigned_transaction: bool,
        /// Instead of executing the transaction, serialize the bcs bytes of the signed transaction data
        /// (SenderSignedData) using base64 encoding, and print out the string.
        #[clap(long, required = false)]
        serialize_signed_transaction: bool,    }
}

impl SuiClientCommands {
    pub async fn execute(
        self,
        context: &mut WalletContext,
    ) -> Result<SuiClientCommandResult, anyhow::Error> {
        let ret = Ok(match self {
            SuiClientCommands::ReplayTransaction { tx_digest } => {
                let cmd = ReplayToolCommand::ReplayTransaction {
                    tx_digest,
                    show_effects: true,
                    diag: false,
                    executor_version_override: None,
                    protocol_version_override: None,
                };
                let rpc = context.config.get_active_env()?.rpc.clone();
                let _command_result =
                    sui_replay::execute_replay_command(Some(rpc), false, false, None, cmd).await?;
                SuiClientCommandResult::ReplayTransaction
            }
            SuiClientCommands::ReplayBatch {
                path,
                terminate_early,
            } => {
                let cmd = ReplayToolCommand::ReplayBatch {
                    path,
                    terminate_early,
                    batch_size: 16,
                };
                let rpc = context.config.get_active_env()?.rpc.clone();
                let _command_result =
                    sui_replay::execute_replay_command(Some(rpc), false, false, None, cmd).await?;
                SuiClientCommandResult::ReplayBatch
            }
            SuiClientCommands::ReplayCheckpoints {
                start,
                end,
                terminate_early,
            } => {
                let cmd = ReplayToolCommand::ReplayCheckpoints {
                    start,
                    end,
                    terminate_early,
                    max_tasks: 16,
                };
                let rpc = context.config.get_active_env()?.rpc.clone();
                let _command_result =
                    sui_replay::execute_replay_command(Some(rpc), false, false, None, cmd).await?;
                SuiClientCommandResult::ReplayCheckpoints
            }
            SuiClientCommands::Addresses => {
                let active_address = context.active_address()?;
                let addresses = context
                    .config
                    .keystore
                    .addresses_with_alias()
                    .into_iter()
                    .map(|(address, alias)| (alias.alias.to_string(), *address))
                    .collect();

                let output = AddressesOutput {
                    active_address,
                    addresses,
                };
                SuiClientCommandResult::Addresses(output)
            }
            SuiClientCommands::DynamicFieldQuery { id, cursor, limit } => {
                let client = context.get_client().await?;
                let df_read = client
                    .read_api()
                    .get_dynamic_fields(id, cursor, Some(limit))
                    .await?;
                SuiClientCommandResult::DynamicFieldQuery(df_read)
            }

            SuiClientCommands::Upgrade {
                package_path,
                upgrade_capability,
                build_config,
                gas,
                gas_budget,
                skip_dependency_verification,
                with_unpublished_dependencies,
                serialize_unsigned_transaction,
                serialize_signed_transaction,
            } => {
                let sender = context.try_get_object_owner(&gas).await?;
                let sender = sender.unwrap_or(context.active_address()?);

                let client = context.get_client().await?;
                let (dependencies, compiled_modules, compiled_package, package_id) =
                    compile_package(
                        &client,
                        build_config,
                        package_path,
                        with_unpublished_dependencies,
                        skip_dependency_verification,
                    )
                    .await?;

                let package_id = package_id.map_err(|e| match e {
                    PublishedAtError::NotPresent => {
                        anyhow!("No 'published-at' field in manifest for package to be upgraded.")
                    }
                    PublishedAtError::Invalid(v) => anyhow!(
                        "Invalid 'published-at' field in manifest of package to be upgraded. \
                         Expected an on-chain address, but found: {v:?}"
                    ),
                })?;

                let resp = context
                    .get_client()
                    .await?
                    .read_api()
                    .get_object_with_options(
                        upgrade_capability,
                        SuiObjectDataOptions::default().with_bcs().with_owner(),
                    )
                    .await?;

                let Some(data) = resp.data else {
                    return Err(anyhow!(
                        "Could not find upgrade capability at {upgrade_capability}"
                    ));
                };

                let upgrade_cap: UpgradeCap = data
                    .bcs
                    .ok_or_else(|| {
                        anyhow!("Fetch upgrade capability object but no data was returned")
                    })?
                    .try_as_move()
                    .ok_or_else(|| anyhow!("Upgrade capability is not a Move Object"))?
                    .deserialize()?;
                // We keep the existing policy -- no fancy policies or changing the upgrade
                // policy at the moment. To change the policy you can call a Move function in the
                // `package` module to change this policy.
                let upgrade_policy = upgrade_cap.policy;
                let package_digest =
                    compiled_package.get_package_digest(with_unpublished_dependencies);

                let data = client
                    .transaction_builder()
                    .upgrade(
                        sender,
                        package_id,
                        compiled_modules,
                        dependencies.published.into_values().collect(),
                        upgrade_capability,
                        upgrade_policy,
                        package_digest.to_vec(),
                        gas,
                        gas_budget,
                    )
                    .await?;
                serialize_or_execute!(
                    data,
                    serialize_unsigned_transaction,
                    serialize_signed_transaction,
                    context,
                    Upgrade
                )
            }
            SuiClientCommands::Publish {
                package_path,
                gas,
                build_config,
                gas_budget,
                skip_dependency_verification,
                with_unpublished_dependencies,
                serialize_unsigned_transaction,
                serialize_signed_transaction,
            } => {
                if build_config.test_mode {
                    return Err(SuiError::ModulePublishFailure {
                        error:
                            "The `publish` subcommand should not be used with the `--test` flag\n\
                            \n\
                            Code in published packages must not depend on test code.\n\
                            In order to fix this and publish the package without `--test`, \
                            remove any non-test dependencies on test-only code.\n\
                            You can ensure all test-only dependencies have been removed by \
                            compiling the package normally with `sui move build`."
                                .to_string(),
                    }
                    .into());
                }

                let sender = context.try_get_object_owner(&gas).await?;
                let sender = sender.unwrap_or(context.active_address()?);

                let client = context.get_client().await?;
                let (dependencies, compiled_modules, _, _) = compile_package(
                    &client,
                    build_config,
                    package_path,
                    with_unpublished_dependencies,
                    skip_dependency_verification,
                )
                .await?;

                let data = client
                    .transaction_builder()
                    .publish(
                        sender,
                        compiled_modules,
                        dependencies.published.into_values().collect(),
                        gas,
                        gas_budget,
                    )
                    .await?;
                serialize_or_execute!(
                    data,
                    serialize_unsigned_transaction,
                    serialize_signed_transaction,
                    context,
                    Publish
                )
            }

            SuiClientCommands::VerifyBytecodeMeter {
                package_path,
                build_config,
            } => {
                let protocol_config = ProtocolConfig::get_for_max_version_UNSAFE();
                let registry = &Registry::new();
                let bytecode_verifier_metrics = Arc::new(BytecodeVerifierMetrics::new(registry));

                let package = compile_package_simple(build_config, package_path)?;
                let modules: Vec<_> = package.get_modules().cloned().collect();

                let mut verifier =
                    sui_execution::verifier(&protocol_config, true, &bytecode_verifier_metrics);
                let overrides = VerifierOverrides::new(None, None);
                println!("Running bytecode verifier for {} modules", modules.len());
                let verifier_values = verifier.meter_compiled_modules_with_overrides(
                    &modules,
                    &protocol_config,
                    &overrides,
                )?;
                SuiClientCommandResult::VerifyBytecodeMeter {
                    max_module_ticks: verifier_values
                        .max_per_mod_meter_current
                        .unwrap_or(u128::MAX),
                    max_function_ticks: verifier_values
                        .max_per_fun_meter_current
                        .unwrap_or(u128::MAX),
                    used_function_ticks: verifier_values.fun_meter_units_result,
                    used_module_ticks: verifier_values.mod_meter_units_result,
                }
            }

            SuiClientCommands::Object { id, bcs } => {
                // Fetch the object ref
                let client = context.get_client().await?;
                if !bcs {
                    let object_read = client
                        .read_api()
                        .get_object_with_options(id, SuiObjectDataOptions::full_content())
                        .await?;
                    SuiClientCommandResult::Object(object_read)
                } else {
                    let raw_object_read = client
                        .read_api()
                        .get_object_with_options(id, SuiObjectDataOptions::bcs_lossless())
                        .await?;
                    SuiClientCommandResult::RawObject(raw_object_read)
                }
            }

            SuiClientCommands::TransactionBlock { digest } => {
                let client = context.get_client().await?;
                let tx_read = client
                    .read_api()
                    .get_transaction_with_options(
                        digest,
                        SuiTransactionBlockResponseOptions {
                            show_input: true,
                            show_raw_input: false,
                            show_effects: true,
                            show_events: true,
                            show_object_changes: true,
                            show_balance_changes: false,
                        },
                    )
                    .await?;
                SuiClientCommandResult::TransactionBlock(tx_read)
            }

            SuiClientCommands::Call {
                package,
                module,
                function,
                type_args,
                gas,
                gas_budget,
                args,
                serialize_unsigned_transaction,
                serialize_signed_transaction,
            } => {
                let tx_data = construct_move_call_transaction(
                    package, &module, &function, type_args, gas, gas_budget, args, context,
                )
                .await?;
                serialize_or_execute!(
                    tx_data,
                    serialize_unsigned_transaction,
                    serialize_signed_transaction,
                    context,
                    Call
                )
            }

            SuiClientCommands::Transfer {
                to,
                object_id,
                gas,
                gas_budget,
                serialize_unsigned_transaction,
                serialize_signed_transaction,
            } => {
                let from = context.get_object_owner(&object_id).await?;
                let to = get_identity_address(Some(to), context)?;
                let client = context.get_client().await?;
                let data = client
                    .transaction_builder()
                    .transfer_object(from, object_id, gas, gas_budget, to)
                    .await?;
                serialize_or_execute!(
                    data,
                    serialize_unsigned_transaction,
                    serialize_signed_transaction,
                    context,
                    Transfer
                )
            }

            SuiClientCommands::TransferSui {
                to,
                sui_coin_object_id: object_id,
                gas_budget,
                amount,
                serialize_unsigned_transaction,
                serialize_signed_transaction,
            } => {
                let from = context.get_object_owner(&object_id).await?;
                let to = get_identity_address(Some(to), context)?;
                let client = context.get_client().await?;
                let data = client
                    .transaction_builder()
                    .transfer_sui(from, object_id, gas_budget, to, amount)
                    .await?;
                serialize_or_execute!(
                    data,
                    serialize_unsigned_transaction,
                    serialize_signed_transaction,
                    context,
                    TransferSui
                )
            }

            SuiClientCommands::Pay {
                input_coins,
                recipients,
                amounts,
                gas,
                gas_budget,
                serialize_unsigned_transaction,
                serialize_signed_transaction,
            } => {
                ensure!(
                    !input_coins.is_empty(),
                    "Pay transaction requires a non-empty list of input coins"
                );
                ensure!(
                    !recipients.is_empty(),
                    "Pay transaction requires a non-empty list of recipient addresses"
                );
                ensure!(
                    recipients.len() == amounts.len(),
                    format!(
                        "Found {:?} recipient addresses, but {:?} recipient amounts",
                        recipients.len(),
                        amounts.len()
                    ),
                );
                let recipients = recipients
                    .into_iter()
                    .map(|x| get_identity_address(Some(x), context))
                    .collect::<Result<Vec<SuiAddress>, anyhow::Error>>()
                    .map_err(|e| anyhow!("{e}"))?;
                let from = context.get_object_owner(&input_coins[0]).await?;
                let client = context.get_client().await?;
                let data = client
                    .transaction_builder()
                    .pay(from, input_coins, recipients, amounts, gas, gas_budget)
                    .await?;
                serialize_or_execute!(
                    data,
                    serialize_unsigned_transaction,
                    serialize_signed_transaction,
                    context,
                    Pay
                )
            }

            SuiClientCommands::PaySui {
                input_coins,
                recipients,
                amounts,
                gas_budget,
                serialize_unsigned_transaction,
                serialize_signed_transaction,
            } => {
                ensure!(
                    !input_coins.is_empty(),
                    "PaySui transaction requires a non-empty list of input coins"
                );
                ensure!(
                    !recipients.is_empty(),
                    "PaySui transaction requires a non-empty list of recipient addresses"
                );
                ensure!(
                    recipients.len() == amounts.len(),
                    format!(
                        "Found {:?} recipient addresses, but {:?} recipient amounts",
                        recipients.len(),
                        amounts.len()
                    ),
                );
                let recipients = recipients
                    .into_iter()
                    .map(|x| get_identity_address(Some(x), context))
                    .collect::<Result<Vec<SuiAddress>, anyhow::Error>>()
                    .map_err(|e| anyhow!("{e}"))?;
                let signer = context.get_object_owner(&input_coins[0]).await?;
                let client = context.get_client().await?;
                let data = client
                    .transaction_builder()
                    .pay_sui(signer, input_coins, recipients, amounts, gas_budget)
                    .await?;
                serialize_or_execute!(
                    data,
                    serialize_unsigned_transaction,
                    serialize_signed_transaction,
                    context,
                    PaySui
                )
            }

            SuiClientCommands::PayAllSui {
                input_coins,
                recipient,
                gas_budget,
                serialize_unsigned_transaction,
                serialize_signed_transaction,
            } => {
                ensure!(
                    !input_coins.is_empty(),
                    "PayAllSui transaction requires a non-empty list of input coins"
                );
                let recipient = get_identity_address(Some(recipient), context)?;
                let signer = context.get_object_owner(&input_coins[0]).await?;
                let client = context.get_client().await?;
                let data = client
                    .transaction_builder()
                    .pay_all_sui(signer, input_coins, recipient, gas_budget)
                    .await?;

                serialize_or_execute!(
                    data,
                    serialize_unsigned_transaction,
                    serialize_signed_transaction,
                    context,
                    PayAllSui
                )
            }

            SuiClientCommands::Objects { address } => {
                let address = get_identity_address(address, context)?;
                let client = context.get_client().await?;
                let mut objects: Vec<SuiObjectResponse> = Vec::new();
                let mut cursor = None;
                loop {
                    let response = client
                        .read_api()
                        .get_owned_objects(
                            address,
                            Some(SuiObjectResponseQuery::new_with_options(
                                SuiObjectDataOptions::full_content(),
                            )),
                            cursor,
                            None,
                        )
                        .await?;
                    objects.extend(response.data);

                    if response.has_next_page {
                        cursor = response.next_cursor;
                    } else {
                        break;
                    }
                }
                SuiClientCommandResult::Objects(objects)
            }

            SuiClientCommands::NewAddress {
                key_scheme,
                alias,
                derivation_path,
                word_length,
            } => {
                let (address, phrase, scheme) = context.config.keystore.generate_and_add_new_key(
                    key_scheme,
                    alias.clone(),
                    derivation_path,
                    word_length,
                )?;

                let alias = match alias {
                    Some(x) => x,
                    None => context.config.keystore.get_alias_by_address(&address)?,
                };

                SuiClientCommandResult::NewAddress(NewAddressOutput {
                    alias,
                    address,
                    key_scheme: scheme,
                    recovery_phrase: phrase,
                })
            }
            SuiClientCommands::Gas { address } => {
                let address = get_identity_address(address, context)?;
                let coins = context
                    .gas_objects(address)
                    .await?
                    .iter()
                    // Ok to unwrap() since `get_gas_objects` guarantees gas
                    .map(|(_val, object)| GasCoin::try_from(object).unwrap())
                    .collect();
                SuiClientCommandResult::Gas(coins)
            }
            SuiClientCommands::ChainIdentifier => {
                let ci = context
                    .get_client()
                    .await?
                    .read_api()
                    .get_chain_identifier()
                    .await?;
                SuiClientCommandResult::ChainIdentifier(ci)
            }
            SuiClientCommands::SplitCoin {
                coin_id,
                amounts,
                count,
                gas,
                gas_budget,
                serialize_unsigned_transaction,
                serialize_signed_transaction,
            } => {
                let signer = context.get_object_owner(&coin_id).await?;
                let client = context.get_client().await?;
                let data = match (amounts, count) {
                    (Some(amounts), None) => {
                        client
                            .transaction_builder()
                            .split_coin(signer, coin_id, amounts, gas, gas_budget)
                            .await?
                    }
                    (None, Some(count)) => {
                        if count == 0 {
                            return Err(anyhow!("Coin split count must be greater than 0"));
                        }
                        client
                            .transaction_builder()
                            .split_coin_equal(signer, coin_id, count, gas, gas_budget)
                            .await?
                    }
                    _ => {
                        return Err(anyhow!("Exactly one of `count` and `amounts` must be present for split-coin command."));
                    }
                };
                serialize_or_execute!(
                    data,
                    serialize_unsigned_transaction,
                    serialize_signed_transaction,
                    context,
                    SplitCoin
                )
            }
            SuiClientCommands::MergeCoin {
                primary_coin,
                coin_to_merge,
                gas,
                gas_budget,
                serialize_unsigned_transaction,
                serialize_signed_transaction,
            } => {
                let client = context.get_client().await?;
                let signer = context.get_object_owner(&primary_coin).await?;
                let data = client
                    .transaction_builder()
                    .merge_coins(signer, primary_coin, coin_to_merge, gas, gas_budget)
                    .await?;
                serialize_or_execute!(
                    data,
                    serialize_unsigned_transaction,
                    serialize_signed_transaction,
                    context,
                    MergeCoin
                )
            }
            SuiClientCommands::Switch { address, env , dwallet} => {
                let mut addr = None;

                if address.is_none() && env.is_none() {
                    return Err(anyhow!(
                        "No address, an alias, or env specified. Please specify one."
                    ));
                }

                if let Some(address) = address.clone() {
                    let address = get_identity_address(Some(address), context)?;
                    if !context.config.keystore.addresses().contains(&address) {
                        return Err(anyhow!("Address {} not managed by wallet", address));
                    }
                    context.config.active_address = Some(address);
                    addr = Some(address.to_string());
                }

                if let Some(ref env) = env {
                    Self::switch_env(&mut context.config, env)?;
                }

                if let Some(ref dwallet) = dwallet {
                    Self::switch_dwallet(&mut context.config, dwallet)?;
                }
                context.config.save()?;
                SuiClientCommandResult::Switch(SwitchResponse { address: addr, env })
            }
            SuiClientCommands::ActiveAddress => {
                SuiClientCommandResult::ActiveAddress(context.active_address().ok())
            }

            SuiClientCommands::ExecuteSignedTx {
                tx_bytes,
                signatures,
            } => {
                let data = bcs::from_bytes(
                    &Base64::try_from(tx_bytes)
                        .map_err(|e| anyhow!(e))?
                        .to_vec()
                        .map_err(|e| anyhow!(e))?,
                )?;

                let mut sigs = Vec::new();
                for sig in signatures {
                    sigs.push(
                        GenericSignature::from_bytes(
                            &Base64::try_from(sig)
                                .map_err(|e| anyhow!(e))?
                                .to_vec()
                                .map_err(|e| anyhow!(e))?,
                        )
                        .map_err(|e| anyhow!(e))?,
                    );
                }
                let transaction = Transaction::from_generic_sig_data(data, sigs);

                let response = context.execute_transaction_may_fail(transaction).await?;
                SuiClientCommandResult::ExecuteSignedTx(response)
            }
            SuiClientCommands::CreateEthDwallet {
                dwallet_cap_id,
                smart_contract_address,
                smart_contract_approved_tx_slot,
                gas,
                gas_budget,
                serialize_unsigned_transaction,
                serialize_signed_transaction,
            } => {
                create_eth_dwallet(
                    context,
                    dwallet_cap_id,
                    smart_contract_address,
                    smart_contract_approved_tx_slot,
                    gas,
                    gas_budget,
                    serialize_unsigned_transaction,
                    serialize_signed_transaction,
                )
                    .await?
            }

            SuiClientCommands::EthApproveMessage {
                eth_dwallet_cap_id,
                message,
                dwallet_id,
                gas,
                gas_budget,
                serialize_unsigned_transaction,
                serialize_signed_transaction,
            } => {
                eth_approve_message(
                    context,
                    eth_dwallet_cap_id,
                    message,
                    dwallet_id,
                    gas,
                    gas_budget,
                    serialize_unsigned_transaction,
                    serialize_signed_transaction,
                )
                    .await?
            }
            SuiClientCommands::InitEthState {checkpoint, gas, gas_budget, serialize_unsigned_transaction, serialize_signed_transaction } => {
                init_ethereum_state(checkpoint, context, gas, gas_budget, serialize_unsigned_transaction, serialize_signed_transaction).await?
            }
            SuiClientCommands::NewEnv {
                alias,
                rpc,
                ws,
                eth_execution_rpc,
                eth_consensus_rpc,
                eth_genesis_time,
                eth_chain_id,
                eth_genesis_validators_root,
                state_object_id,
            } => {
                if context.config.envs.iter().any(|env| env.alias == alias) {
                    return Err(anyhow!(
                        "Environment config with name [{alias}] already exists."
                    ));
                }
                let env = SuiEnv {
                    alias,
                    rpc,
                    ws,
                    eth_execution_rpc,
                    eth_consensus_rpc,
                    eth_genesis_time,
                    eth_chain_id,
                    eth_genesis_validators_root,
                    state_object_id,
                };

                // Check urls are valid and the server is reachable
                env.create_rpc_client(None, None).await?;
                context.config.envs.push(env.clone());
                context.config.save()?;
                SuiClientCommandResult::NewEnv(env)
            }
            SuiClientCommands::ActiveEnv => {
                SuiClientCommandResult::ActiveEnv(context.config.active_env.clone())
            }
            SuiClientCommands::Envs => SuiClientCommandResult::Envs(
                context.config.envs.clone(),
                context.config.active_env.clone(),
            ),
            SuiClientCommands::VerifySource {
                package_path,
                build_config,
                verify_deps,
                skip_source,
                address_override,
            } => {
                if skip_source && !verify_deps {
                    return Err(anyhow!(
                        "Source skipped and not verifying deps: Nothing to verify."
                    ));
                }

                let build_config =
                    resolve_lock_file_path(build_config, Some(package_path.clone()))?;
                let compiled_package = BuildConfig {
                    config: build_config,
                    run_bytecode_verifier: true,
                    print_diags_to_stderr: true,
                }
                .build(package_path)?;

                let client = context.get_client().await?;

                BytecodeSourceVerifier::new(client.read_api())
                    .verify_package(
                        &compiled_package,
                        verify_deps,
                        match (skip_source, address_override) {
                            (true, _) => SourceMode::Skip,
                            (false, None) => SourceMode::Verify,
                            (false, Some(addr)) => SourceMode::VerifyAt(addr.into()),
                        },
                    )
                    .await?;

                SuiClientCommandResult::VerifySource
            }
            SuiClientCommands::DWallet {
                cmd,
            } => {
                if let Some(cmd) = cmd {
                    cmd.execute(context).await?
                } else {
                    // Print help
                    let mut app: Command = SuiCommand::command();
                    app.build();
                    app.find_subcommand_mut("client").unwrap().find_subcommand_mut("dwallet").unwrap().print_help()?;
                    bail!(
                        "Wrong dwallet command."
                    );
                }
            }
        });
        ret
    }

    pub fn switch_env(config: &mut SuiClientConfig, env: &str) -> Result<(), anyhow::Error> {
        let env = Some(env.into());
        ensure!(config.get_env(&env).is_some(), "Environment config not found for [{env:?}], add new environment config using the `sui client new-env` command.");
        config.active_env = env;
        Ok(())
    }

    pub fn switch_dwallet(config: &mut SuiClientConfig, dwallet: &str) -> Result<(), anyhow::Error> {
        let dwallet = Some(dwallet.into());
        ensure!(config.get_env(&dwallet).is_some(), "dWallet config not found for [{dwallet:?}], create a new dWallet using the `sui client dwallet create` command.");
        config.active_dwallet = dwallet;
        Ok(())
    }
}

fn compile_package_simple(
    build_config: MoveBuildConfig,
    package_path: PathBuf,
) -> Result<CompiledPackage, anyhow::Error> {
    let config = BuildConfig {
        config: resolve_lock_file_path(build_config, Some(package_path.clone()))?,
        run_bytecode_verifier: false,
        print_diags_to_stderr: false,
    };
    let resolution_graph = config.resolution_graph(&package_path)?;

    Ok(build_from_resolution_graph(
        package_path,
        resolution_graph,
        false,
        false,
    )?)
}

async fn compile_package(
    client: &SuiClient,
    build_config: MoveBuildConfig,
    package_path: PathBuf,
    with_unpublished_dependencies: bool,
    skip_dependency_verification: bool,
) -> Result<
    (
        PackageDependencies,
        Vec<Vec<u8>>,
        CompiledPackage,
        Result<ObjectID, PublishedAtError>,
    ),
    anyhow::Error,
> {
    let config = resolve_lock_file_path(build_config, Some(package_path.clone()))?;
    let run_bytecode_verifier = true;
    let print_diags_to_stderr = true;
    let config = BuildConfig {
        config,
        run_bytecode_verifier,
        print_diags_to_stderr,
    };
    let resolution_graph = config.resolution_graph(&package_path)?;
    let (package_id, dependencies) = gather_published_ids(&resolution_graph);
    check_invalid_dependencies(&dependencies.invalid)?;
    if !with_unpublished_dependencies {
        check_unpublished_dependencies(&dependencies.unpublished)?;
    };
    let compiled_package = build_from_resolution_graph(
        package_path,
        resolution_graph,
        run_bytecode_verifier,
        print_diags_to_stderr,
    )?;
    if !compiled_package.is_system_package() {
        if let Some(already_published) = compiled_package.published_root_module() {
            return Err(SuiError::ModulePublishFailure {
                error: format!(
                    "Modules must all have 0x0 as their addresses. \
                     Violated by module {:?}",
                    already_published.self_id(),
                ),
            }
            .into());
        }
    }
    if with_unpublished_dependencies {
        compiled_package.verify_unpublished_dependencies(&dependencies.unpublished)?;
    }
    let compiled_modules = compiled_package.get_package_bytes(with_unpublished_dependencies);
    if !skip_dependency_verification {
        let verifier = BytecodeSourceVerifier::new(client.read_api());
        if let Err(e) = verifier.verify_package_deps(&compiled_package).await {
            return Err(SuiError::ModulePublishFailure {
                error: format!(
                    "[warning] {e}\n\
                     \n\
                     This may indicate that the on-chain version(s) of your package's dependencies \
                     may behave differently than the source version(s) your package was built \
                     against.\n\
                     \n\
                     Fix this by rebuilding your packages with source versions matching on-chain \
                     versions of dependencies, or ignore this warning by re-running with the \
                     --skip-dependency-verification flag."
                ),
            }
            .into());
        } else {
            eprintln!(
                "{}",
                "Successfully verified dependencies on-chain against source."
                    .bold()
                    .green(),
            );
        }
    } else {
        eprintln!("{}", "Skipping dependency verification".bold().yellow());
    }
    Ok((dependencies, compiled_modules, compiled_package, package_id))
}

impl Display for SuiClientCommandResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();
        match self {
            SuiClientCommandResult::Addresses(addresses) => {
                let mut builder = TableBuilder::default();
                builder.set_header(vec!["alias", "address", "active address"]);
                for (alias, address) in &addresses.addresses {
                    let active_address = if address == &addresses.active_address {
                        "*".to_string()
                    } else {
                        "".to_string()
                    };
                    builder.push_record([alias.to_string(), address.to_string(), active_address]);
                }
                let mut table = builder.build();
                let style = TableStyle::rounded();
                table.with(style);
                write!(f, "{}", table)?
            }
            SuiClientCommandResult::DynamicFieldQuery(df_refs) => {
                let df_refs = DynamicFieldOutput {
                    has_next_page: df_refs.has_next_page,
                    next_cursor: df_refs.next_cursor,
                    data: df_refs.data.clone(),
                };

                let json_obj = json!(df_refs);
                let mut table = json_to_table(&json_obj);
                let style = TableStyle::rounded().horizontals([]);
                table.with(style);
                write!(f, "{}", table)?
            }
            SuiClientCommandResult::Gas(gas_coins) => {
                let gas_coins = gas_coins
                    .iter()
                    .map(GasCoinOutput::from)
                    .collect::<Vec<_>>();
                if gas_coins.is_empty() {
                    write!(f, "No gas coins are owned by this address")?;
                    return Ok(());
                }

                let mut builder = TableBuilder::default();
                builder.set_header(vec!["gasCoinId", "gasBalance"]);
                for coin in &gas_coins {
                    builder.push_record(vec![
                        coin.gas_coin_id.to_string(),
                        coin.gas_balance.to_string(),
                    ]);
                }
                let mut table = builder.build();
                table.with(TableStyle::rounded());
                if gas_coins.len() > 10 {
                    table.with(TablePanel::header(format!(
                        "Showing {} gas coins and their balances.",
                        gas_coins.len()
                    )));
                    table.with(TablePanel::footer(format!(
                        "Showing {} gas coins and their balances.",
                        gas_coins.len()
                    )));
                    table.with(TableStyle::rounded().horizontals([
                        HorizontalLine::new(1, TableStyle::modern().get_horizontal()),
                        HorizontalLine::new(2, TableStyle::modern().get_horizontal()),
                        HorizontalLine::new(
                            gas_coins.len() + 2,
                            TableStyle::modern().get_horizontal(),
                        ),
                    ]));
                    table.with(tabled::settings::style::BorderSpanCorrection);
                }
                write!(f, "{}", table)?;
            }
            SuiClientCommandResult::NewAddress(new_address) => {
                let mut builder = TableBuilder::default();
                builder.push_record(vec!["alias", new_address.alias.as_str()]);
                builder.push_record(vec!["address", new_address.address.to_string().as_str()]);
                builder.push_record(vec![
                    "keyScheme",
                    new_address.key_scheme.to_string().as_str(),
                ]);
                builder.push_record(vec![
                    "recoveryPhrase",
                    new_address.recovery_phrase.to_string().as_str(),
                ]);

                let mut table = builder.build();
                table.with(TableStyle::rounded());
                table.with(TablePanel::header(
                    "Created new keypair and saved it to keystore.",
                ));

                table.with(
                    TableModify::new(TableCell::new(0, 0))
                        .with(TableBorder::default().corner_bottom_right('┬')),
                );
                table.with(
                    TableModify::new(TableCell::new(0, 0))
                        .with(TableBorder::default().corner_top_right('─')),
                );

                write!(f, "{}", table)?
            }
            SuiClientCommandResult::NewDWallet(new_dwallet) => {
                let mut builder = TableBuilder::default();
                builder.push_record(vec!["alias", new_dwallet.alias.as_str()]);
                builder.push_record(vec!["dwallet_id", new_dwallet.dwallet_id.to_string().as_str()]);
                builder.push_record(vec!["dwallet_cap_id", new_dwallet.dwallet_cap_id.to_string().as_str()]);

                let mut table = builder.build();
                table.with(TableStyle::rounded());
                table.with(TablePanel::header(
                    "Created new dwallet and saved its secret share.",
                ));

                table.with(
                    TableModify::new(TableCell::new(0, 0))
                        .with(TableBorder::default().corner_bottom_right('┬')),
                );
                table.with(
                    TableModify::new(TableCell::new(0, 0))
                        .with(TableBorder::default().corner_top_right('─')),
                );

                write!(f, "{}", table)?
            }
            SuiClientCommandResult::NewSignOutput(sign_output) => {
                let mut builder = TableBuilder::default();
                builder.push_record(vec!["dwallet_id", sign_output.dwallet_id.to_string().as_str()]);
                builder.push_record(vec!["sign_output_id", sign_output.sign_output_id.to_string().as_str()]);
                builder.push_record(vec!["signatures:", ""]);

                for signature in &sign_output.signatures {
                    builder.push_record(vec!["", signature.as_str()]);
                }

                let mut table = builder.build();
                table.with(TableStyle::rounded());
                table.with(TablePanel::header(
                    "MPC completed and sign output object was generated (signatures in base64).",
                ));

                table.with(
                    TableModify::new(TableCell::new(0, 0))
                        .with(TableBorder::default().corner_bottom_right('┬')),
                );
                table.with(
                    TableModify::new(TableCell::new(0, 0))
                        .with(TableBorder::default().corner_top_right('─')),
                );

                write!(f, "{}", table)?
            }
            SuiClientCommandResult::Object(object_read) => match object_read.object() {
                Ok(obj) => {
                    let object = ObjectOutput::from(obj);
                    let json_obj = json!(&object);
                    let mut table = json_to_table(&json_obj);
                    table.with(TableStyle::rounded().horizontals([]));
                    writeln!(f, "{}", table)?
                }
                Err(e) => writeln!(f, "Internal error, cannot read the object: {e}")?,
            },
            SuiClientCommandResult::Objects(object_refs) => {
                if object_refs.is_empty() {
                    writeln!(f, "This address has no owned objects.")?
                } else {
                    let objects = ObjectsOutput::from_vec(object_refs.to_vec());
                    match objects {
                        Ok(objs) => {
                            let json_obj = json!(objs);
                            let mut table = json_to_table(&json_obj);
                            table.with(TableStyle::rounded().horizontals([]));
                            writeln!(f, "{}", table)?
                        }
                        Err(e) => write!(f, "Internal error: {e}")?,
                    }
                }
            }
            SuiClientCommandResult::Upgrade(response)
            | SuiClientCommandResult::Publish(response) => {
                write!(writer, "{}", response)?;
            }
            SuiClientCommandResult::TransactionBlock(response) => {
                write!(writer, "{}", response)?;
            }
            SuiClientCommandResult::RawObject(raw_object_read) => {
                let raw_object = match raw_object_read.object() {
                    Ok(v) => match &v.bcs {
                        Some(SuiRawData::MoveObject(o)) => {
                            format!("{:?}\nNumber of bytes: {}", o.bcs_bytes, o.bcs_bytes.len())
                        }
                        Some(SuiRawData::Package(p)) => {
                            let mut temp = String::new();
                            let mut bcs_bytes = 0usize;
                            for m in &p.module_map {
                                temp.push_str(&format!("{:?}\n", m));
                                bcs_bytes += m.1.len()
                            }
                            format!("{}Number of bytes: {}", temp, bcs_bytes)
                        }
                        None => "Bcs field is None".to_string().red().to_string(),
                    },
                    Err(err) => format!("{err}").red().to_string(),
                };
                writeln!(writer, "{}", raw_object)?;
            }
            SuiClientCommandResult::Call(response) => {
                write!(writer, "{}", response)?;
            }
            SuiClientCommandResult::SerializedUnsignedTransaction(tx_data) => {
                writeln!(
                    writer,
                    "{}",
                    fastcrypto::encoding::Base64::encode(bcs::to_bytes(tx_data).unwrap())
                )?;
            }
            SuiClientCommandResult::SerializedSignedTransaction(sender_signed_tx) => {
                writeln!(
                    writer,
                    "{}",
                    fastcrypto::encoding::Base64::encode(bcs::to_bytes(sender_signed_tx).unwrap())
                )?;
            }
            SuiClientCommandResult::Transfer(response) => {
                write!(writer, "{}", response)?;
            }
            SuiClientCommandResult::TransferSui(response) => {
                write!(writer, "{}", response)?;
            }
            SuiClientCommandResult::Pay(response) => {
                write!(writer, "{}", response)?;
            }
            SuiClientCommandResult::PaySui(response) => {
                write!(writer, "{}", response)?;
            }
            SuiClientCommandResult::PayAllSui(response) => {
                write!(writer, "{}", response)?;
            }
            SuiClientCommandResult::SyncClientState => {
                writeln!(writer, "Client state sync complete.")?;
            }
            SuiClientCommandResult::ChainIdentifier(ci) => {
                writeln!(writer, "{}", ci)?;
            }
            SuiClientCommandResult::SplitCoin(response) => {
                write!(writer, "{}", response)?;
            }
            SuiClientCommandResult::MergeCoin(response) => {
                write!(writer, "{}", response)?;
            }
            SuiClientCommandResult::Switch(response) => {
                write!(writer, "{}", response)?;
            }
            SuiClientCommandResult::ActiveAddress(response) => {
                match response {
                    Some(r) => write!(writer, "{}", r)?,
                    None => write!(writer, "None")?,
                };
            }
            SuiClientCommandResult::ExecuteSignedTx(response) => {
                write!(writer, "{}", response)?;
            }
            SuiClientCommandResult::ActiveEnv(env) => {
                write!(writer, "{}", env.as_deref().unwrap_or("None"))?;
            }
            SuiClientCommandResult::NewEnv(env) => {
                writeln!(writer, "Added new dWallet env [{}] to config.", env.alias)?;
            }
            SuiClientCommandResult::Envs(envs, active) => {
                let mut builder = TableBuilder::default();
                builder.set_header(["alias", "url", "active"]);
                for env in envs {
                    builder.push_record(vec![env.alias.clone(), env.rpc.clone(), {
                        if Some(env.alias.as_str()) == active.as_deref() {
                            "*".to_string()
                        } else {
                            "".to_string()
                        }
                    }]);
                }
                let mut table = builder.build();
                table.with(TableStyle::rounded());
                write!(f, "{}", table)?
            }
            SuiClientCommandResult::VerifySource => {
                writeln!(writer, "Source verification succeeded!")?;
            }
            SuiClientCommandResult::VerifyBytecodeMeter {
                max_module_ticks,
                max_function_ticks,
                used_function_ticks,
                used_module_ticks,
            } => {
                let mut builder = TableBuilder::default();
                builder.set_header(vec!["", "Module", "Function"]);
                builder.push_record(vec![
                    "Max".to_string(),
                    max_module_ticks.to_string(),
                    max_function_ticks.to_string(),
                ]);
                builder.push_record(vec![
                    "Used".to_string(),
                    used_module_ticks.to_string(),
                    used_function_ticks.to_string(),
                ]);
                let mut table = builder.build();
                table.with(TableStyle::rounded());
                if (used_module_ticks > max_module_ticks)
                    || (used_function_ticks > max_function_ticks)
                {
                    table.with(TablePanel::header("Module will NOT pass metering check!"));
                } else {
                    table.with(TablePanel::header("Module will pass metering check!"));
                }
                table.with(tabled::settings::style::BorderSpanCorrection);
                writeln!(f, "{}", table)?;
            }
            SuiClientCommandResult::ReplayTransaction => {}
            SuiClientCommandResult::ReplayBatch => {}
            SuiClientCommandResult::ReplayCheckpoints => {}
        }
        write!(f, "{}", writer.trim_end_matches('\n'))
    }
}

pub(crate) async fn construct_move_call_transaction(
    package: ObjectID,
    module: &str,
    function: &str,
    type_args: Vec<TypeTag>,
    gas: Option<ObjectID>,
    gas_budget: u64,
    args: Vec<SuiJsonValue>,
    context: &mut WalletContext,
) -> Result<TransactionData, anyhow::Error> {
    // Convert all numeric input to String, this will allow number input from the CLI without failing SuiJSON's checks.
    let args = args
        .into_iter()
        .map(|value| SuiJsonValue::new(convert_number_to_string(value.to_json_value())))
        .collect::<Result<_, _>>()?;

    let type_args = type_args
        .into_iter()
        .map(|arg| arg.try_into())
        .collect::<Result<Vec<_>, _>>()?;
    let gas_owner = context.try_get_object_owner(&gas).await?;
    let sender = gas_owner.unwrap_or(context.active_address()?);

    let client = context.get_client().await?;
    client
        .transaction_builder()
        .move_call(
            sender, package, module, function, type_args, args, gas, gas_budget,
        )
        .await
}

fn convert_number_to_string(value: Value) -> Value {
    match value {
        Value::Number(n) => Value::String(n.to_string()),
        Value::Array(a) => Value::Array(a.into_iter().map(convert_number_to_string).collect()),
        Value::Object(o) => Value::Object(
            o.into_iter()
                .map(|(k, v)| (k, convert_number_to_string(v)))
                .collect(),
        ),
        _ => value,
    }
}

impl Debug for SuiClientCommandResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = unwrap_err_to_string(|| match self {
            SuiClientCommandResult::Gas(gas_coins) => {
                let gas_coins = gas_coins
                    .iter()
                    .map(GasCoinOutput::from)
                    .collect::<Vec<_>>();
                Ok(serde_json::to_string_pretty(&gas_coins)?)
            }
            SuiClientCommandResult::Object(object_read) => {
                let object = object_read.object()?;
                Ok(serde_json::to_string_pretty(&object)?)
            }
            SuiClientCommandResult::RawObject(raw_object_read) => {
                let raw_object = raw_object_read.object()?;
                Ok(serde_json::to_string_pretty(&raw_object)?)
            }
            _ => Ok(serde_json::to_string_pretty(self)?),
        });
        write!(f, "{}", s)
    }
}

fn unwrap_err_to_string<T: Display, F: FnOnce() -> Result<T, anyhow::Error>>(func: F) -> String {
    match func() {
        Ok(s) => format!("{s}"),
        Err(err) => format!("{err}").red().to_string(),
    }
}

impl SuiClientCommandResult {
    pub fn objects_response(&self) -> Option<Vec<SuiObjectResponse>> {
        use SuiClientCommandResult::*;
        match self {
            Object(o) | RawObject(o) => Some(vec![o.clone()]),
            Objects(o) => Some(o.clone()),
            _ => None,
        }
    }

    pub fn print(&self, pretty: bool) {
        let line = if pretty {
            format!("{self}")
        } else {
            format!("{:?}", self)
        };
        // Log line by line
        for line in line.lines() {
            // Logs write to a file on the side.  Print to stdout and also log to file, for tests to pass.
            println!("{line}");
            info!("{line}")
        }
    }

    pub fn tx_block_response(&self) -> Option<&SuiTransactionBlockResponse> {
        use SuiClientCommandResult::*;
        match self {
            Upgrade(b) | Publish(b) | TransactionBlock(b) | Call(b) | Transfer(b)
            | TransferSui(b) | Pay(b) | PaySui(b) | PayAllSui(b) | SplitCoin(b) | MergeCoin(b)
            | ExecuteSignedTx(b) => Some(b),
            _ => None,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressesOutput {
    pub active_address: SuiAddress,
    pub addresses: Vec<(String, SuiAddress)>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicFieldOutput {
    pub has_next_page: bool,
    pub next_cursor: Option<ObjectID>,
    pub data: Vec<DynamicFieldInfo>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewAddressOutput {
    pub alias: String,
    pub address: SuiAddress,
    pub key_scheme: SignatureScheme,
    pub recovery_phrase: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewDWalletOutput {
    pub alias: String,
    pub dwallet_id: ObjectID,
    pub dwallet_cap_id: ObjectID,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewSignOutput {
    pub dwallet_id: ObjectID,
    pub sign_output_id: ObjectID,
    pub signatures: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectOutput {
    pub object_id: ObjectID,
    pub version: SequenceNumber,
    pub digest: String,
    pub obj_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_tx: Option<TransactionDigest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_rebate: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<SuiParsedData>,
}

impl From<&SuiObjectData> for ObjectOutput {
    fn from(obj: &SuiObjectData) -> Self {
        let owner_type = match obj.owner {
            Some(Owner::AddressOwner(_)) => Some("AddressOwner".to_string()),
            Some(Owner::ObjectOwner(_)) => Some("ObjectOwner".to_string()),
            Some(Owner::Shared { .. }) => Some("Shared".to_string()),
            Some(Owner::Immutable) => Some("Immutable".to_string()),
            None => None,
        };
        let obj_type = match obj.type_.as_ref() {
            Some(x) => x.to_string(),
            None => "unknown".to_string(),
        };
        Self {
            object_id: obj.object_id,
            version: obj.version,
            digest: obj.digest.to_string(),
            obj_type,
            owner_type,
            prev_tx: obj.previous_transaction,
            storage_rebate: obj.storage_rebate,
            content: obj.content.clone(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GasCoinOutput {
    pub gas_coin_id: ObjectID,
    pub gas_balance: u64,
}

impl From<&GasCoin> for GasCoinOutput {
    fn from(gas_coin: &GasCoin) -> Self {
        Self {
            gas_coin_id: *gas_coin.id(),
            gas_balance: gas_coin.value(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectsOutput {
    pub object_id: ObjectID,
    pub version: SequenceNumber,
    pub digest: String,
    pub object_type: String,
}

impl ObjectsOutput {
    fn from(obj: SuiObjectResponse) -> Result<Self, anyhow::Error> {
        let obj = obj.into_object()?;
        // this replicates the object type display as in the sui explorer
        let object_type = match obj.type_ {
            Some(sui_types::base_types::ObjectType::Struct(x)) => {
                let address = x.address().to_string();
                // check if the address has length of 64 characters
                // otherwise, keep it as it is
                let address = if address.len() == 64 {
                    format!("0x{}..{}", &address[..4], &address[address.len() - 4..])
                } else {
                    address
                };
                format!("{}::{}::{}", address, x.module(), x.name(),)
            }
            Some(sui_types::base_types::ObjectType::Package) => "Package".to_string(),
            None => "unknown".to_string(),
        };
        Ok(Self {
            object_id: obj.object_id,
            version: obj.version,
            digest: Base64::encode(obj.digest),
            object_type,
        })
    }
    fn from_vec(objs: Vec<SuiObjectResponse>) -> Result<Vec<Self>, anyhow::Error> {
        objs.into_iter()
            .map(ObjectsOutput::from)
            .collect::<Result<Vec<_>, _>>()
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum SuiClientCommandResult {
    ActiveAddress(Option<SuiAddress>),
    ActiveEnv(Option<String>),
    Addresses(AddressesOutput),
    Call(SuiTransactionBlockResponse),
    ChainIdentifier(String),
    DynamicFieldQuery(DynamicFieldPage),
    Envs(Vec<SuiEnv>, Option<String>),
    ExecuteSignedTx(SuiTransactionBlockResponse),
    Gas(Vec<GasCoin>),
    MergeCoin(SuiTransactionBlockResponse),
    NewAddress(NewAddressOutput),
    NewDWallet(NewDWalletOutput),
    NewSignOutput(NewSignOutput),
    NewEnv(SuiEnv),
    Object(SuiObjectResponse),
    Objects(Vec<SuiObjectResponse>),
    Pay(SuiTransactionBlockResponse),
    PayAllSui(SuiTransactionBlockResponse),
    PaySui(SuiTransactionBlockResponse),
    Publish(SuiTransactionBlockResponse),
    RawObject(SuiObjectResponse),
    SerializedSignedTransaction(SenderSignedData),
    SerializedUnsignedTransaction(TransactionData),
    SplitCoin(SuiTransactionBlockResponse),
    Switch(SwitchResponse),
    SyncClientState,
    TransactionBlock(SuiTransactionBlockResponse),
    Transfer(SuiTransactionBlockResponse),
    TransferSui(SuiTransactionBlockResponse),
    Upgrade(SuiTransactionBlockResponse),
    VerifyBytecodeMeter {
        max_module_ticks: u128,
        max_function_ticks: u128,
        used_function_ticks: u128,
        used_module_ticks: u128,
    },
    VerifySource,
    ReplayTransaction,
    ReplayBatch,
    ReplayCheckpoints,
}

#[derive(Serialize, Clone)]
pub struct SwitchResponse {
    /// Active address
    pub address: Option<String>,
    pub env: Option<String>,
}

impl Display for SwitchResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();

        if let Some(addr) = &self.address {
            writeln!(writer, "Active address switched to {addr}")?;
        }
        if let Some(env) = &self.env {
            writeln!(writer, "Active environment switched to [{env}]")?;
        }
        write!(f, "{}", writer)
    }
}
