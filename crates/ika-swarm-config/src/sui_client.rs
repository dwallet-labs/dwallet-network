use std::collections::HashMap;
use anyhow::bail;
use shared_crypto::intent::Intent;
use sui::client_commands::{estimate_gas_budget_from_gas_cost, execute_dry_run, max_gas_budget, request_tokens_from_faucet, SuiClientCommandResult};
use sui_config::{sui_config_dir, SUI_CLIENT_CONFIG};
use sui_keys::keystore::{InMemKeystore, Keystore, AccountKeystore};
use sui_sdk::rpc_types::{ObjectChange, SuiTransactionBlockResponse, SuiTransactionBlockResponseOptions};
use sui_sdk::sui_client_config::{SuiClientConfig, SuiEnv};
use sui_sdk::SuiClient;
use sui_sdk::wallet_context::WalletContext;
use sui_types::base_types::{ObjectID, SequenceNumber, SuiAddress};
use sui_types::crypto::{SignatureScheme, SuiKeyPair};
use sui_types::{Identifier, SUI_FRAMEWORK_PACKAGE_ID};
use sui_types::coin::{TreasuryCap, COIN_MODULE_NAME};
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::{CallArg, ObjectArg, SenderSignedData, Transaction, TransactionDataAPI, TransactionKind};
use move_core_types::language_storage::StructTag;
use ika_move_packages::IkaMovePackage;
use ika_types::governance::{MIN_VALIDATOR_JOINING_STAKE_NIKA};
use ika_types::ika_coin::{IKACoin, IKA, TOTAL_SUPPLY_NIKA};
use ika_types::sui::ika_system_state::IkaSystemStateWrapper;
use ika_config::Config;
use fastcrypto::traits::ToFromBytes;
use sui_sdk::rpc_types::SuiTransactionBlockEffectsAPI;
use ika_config::initiation::InitiationParameters;
use crate::validator_initialization_config::{ValidatorInitializationConfig, ValidatorInitializationMetadata};

pub async fn init_ika_on_sui(
    validator_initialization_configs: &Vec<ValidatorInitializationConfig>,
    sui_fullnode_rpc_url: String,
    sui_faucet_url: String,
    initiation_parameters: InitiationParameters,
) -> Result<(ObjectID, ObjectID, ObjectID, SuiKeyPair), anyhow::Error> {
    //let config_dir = ika_config_dir()?;
    let config_dir = tempfile::tempdir()?.into_path();
    let config_path = config_dir.join(SUI_CLIENT_CONFIG);
    //let keystore_path = config_dir.join(SUI_KEYSTORE_FILENAME);
    //let mut keystore = Keystore::from(FileBasedKeystore::new(&keystore_path)?);
    let mut keystore = Keystore::InMem(InMemKeystore::default());
    let alias = "publisher";
    let _ = keystore.update_alias(alias, None);
    let (publisher_address, phrase, scheme) =
        keystore.generate_and_add_new_key(SignatureScheme::ED25519, Some(alias.to_string()), None, None)?;

    let publisher_keypair = keystore.get_key(&publisher_address)?.copy();
    
    println!(
        "Generated new keypair and alias for address with scheme {:?} [{alias}: {publisher_address}]",
        scheme.to_string()
    );
    println!("Secret Recovery Phrase : [{phrase}]");
    let active_env = "localnet";
    SuiClientConfig {
        keystore,
        envs: vec![SuiEnv {
            alias: active_env.to_string(),
            rpc: sui_fullnode_rpc_url.clone(),
            ws: None,
            basic_auth: None,
        }],
        active_address: Some(publisher_address),
        active_env: Some(active_env.to_string()),
    }
        .persisted(&config_path)
        .save()?;


    let mut context = WalletContext::new(&config_path, None, None)?;

    let client = context.get_client().await?;

    let mut request_tokens_from_faucet_futures = vec![request_tokens_from_faucet(publisher_address, sui_faucet_url.clone())];
    let mut validator_addresses = Vec::new();
    for validator_initialization_config in validator_initialization_configs {
        let alias = validator_initialization_config.name.clone().unwrap();
        context.add_account(Some(alias), validator_initialization_config.account_key_pair.copy());

        let validator_address: SuiAddress = (&validator_initialization_config.account_key_pair.public()).into();

        request_tokens_from_faucet_futures.push(request_tokens_from_faucet(validator_address, sui_faucet_url.clone()));

        validator_addresses.push(validator_address);
    }

    futures::future::join_all(request_tokens_from_faucet_futures).await.into_iter().collect::<Result<Vec<_>, _>>()?;

    let ika_package = ika_move_packages::BuiltInIkaMovePackages::get_package_by_name("ika");
    let ika_system_package = ika_move_packages::BuiltInIkaMovePackages::get_package_by_name("ika_system");

    let (ika_package_id, treasury_cap_id) = publish_ika_package_to_sui(publisher_address, &mut context, client.clone(), ika_package).await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    println!("Package `ika` published: ika_package_id: {ika_package_id} treasury_cap_id: {treasury_cap_id}");

    let (ika_system_package_id, init_cap_id) = publish_ika_system_package_to_sui(publisher_address, &mut context, client.clone(), ika_system_package, ika_package_id).await?;

    println!("Package `ika_system` published: ika_system_package_id: {ika_system_package_id} init_cap_id: {init_cap_id}");

    let init_id = initialize_ika_pre_launch(publisher_address, &mut context, client.clone(), ika_system_package_id, init_cap_id).await?;
    let (_, init_initial_shared_version, _) = client.transaction_builder().get_object_ref(init_id).await?;
    println!("Running `initialize_ika_pre_launch` done: init_id: {init_id}");

    for validator_initialization_config in validator_initialization_configs {
        let validator_address: SuiAddress = (&validator_initialization_config.account_key_pair.public()).into();

        let validator_initialization_metadata = validator_initialization_config.to_validator_initialization_metadata();
        request_add_validator_candidate(validator_address, &mut context, client.clone(), &validator_initialization_metadata, ika_system_package_id, init_id, init_initial_shared_version).await?;
        println!("Running `request_add_validator_candidate` done for validator {validator_address}");
    }

    let ika_supply_id = mint_ika(publisher_address, &mut context, client.clone(), ika_package_id, treasury_cap_id).await?;

    println!("Minting done: ika_supply_id: {ika_supply_id}");

    stake_ika(publisher_address, &mut context, ika_system_package_id, init_id, init_initial_shared_version, ika_supply_id, validator_addresses.clone()).await?;

    println!("Staking for all validators done.");

    for validator_address in validator_addresses {
        request_add_validator(validator_address, &mut context, client.clone(), ika_system_package_id, init_id, init_initial_shared_version).await?;
        println!("Running `request_add_validator` done for validator {validator_address}");
    }

    let ika_system_state_id = initialize_ika_launch(publisher_address, &mut context, client.clone(), ika_system_package_id, init_cap_id, init_id, init_initial_shared_version, treasury_cap_id, initiation_parameters).await?;

    println!("Running `initialize_ika_launch` done: IkaSystemState created - ika_system_state_id: {ika_system_state_id}");

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    Ok((ika_package_id, ika_system_package_id, ika_system_state_id, publisher_keypair))
}

async fn initialize_ika_launch(publisher_address: SuiAddress, context: &mut WalletContext, client: SuiClient, ika_system_package_id: ObjectID, init_cap_id: ObjectID, init_id: ObjectID, init_initial_shared_version: SequenceNumber, treasury_cap_id: ObjectID, initiation_parameters: InitiationParameters, ) -> Result<ObjectID, anyhow::Error> {
    let mut ptb = ProgrammableTransactionBuilder::new();

    let init_cap_ref = client.transaction_builder().get_object_ref(init_cap_id).await?;
    let treasury_cap_ref = client.transaction_builder().get_object_ref(treasury_cap_id).await?;
    
    ptb.move_call(
        ika_system_package_id,
        Identifier::new("init")?,
        Identifier::new("initialize_ika_launch")?,
        vec![],
        vec![
            CallArg::Object(ObjectArg::ImmOrOwnedObject(init_cap_ref)),
            CallArg::Object(ObjectArg::SharedObject {
                id: init_id,
                initial_shared_version: init_initial_shared_version,
                mutable: true,
            }),
            CallArg::Object(ObjectArg::ImmOrOwnedObject(treasury_cap_ref)),
            CallArg::Pure(bcs::to_bytes(&initiation_parameters.protocol_version)?),
            CallArg::Pure(bcs::to_bytes(&initiation_parameters.chain_start_timestamp_ms)?),
            CallArg::Pure(bcs::to_bytes(&initiation_parameters.epoch_duration_ms)?),
            CallArg::Pure(bcs::to_bytes(&initiation_parameters.stake_subsidy_start_epoch)?),
            CallArg::Pure(bcs::to_bytes(&initiation_parameters.stake_subsidy_rate)?),
            CallArg::Pure(bcs::to_bytes(&initiation_parameters.stake_subsidy_period_length)?),
            CallArg::Pure(bcs::to_bytes(&initiation_parameters.min_validator_count)?),
            CallArg::Pure(bcs::to_bytes(&initiation_parameters.max_validator_count)?),
            CallArg::Pure(bcs::to_bytes(&initiation_parameters.min_validator_joining_stake)?),
            CallArg::Pure(bcs::to_bytes(&initiation_parameters.validator_low_stake_threshold)?),
            CallArg::Pure(bcs::to_bytes(&initiation_parameters.validator_very_low_stake_threshold)?),
            CallArg::Pure(bcs::to_bytes(&initiation_parameters.validator_low_stake_grace_period)?),
            CallArg::Pure(bcs::to_bytes(&initiation_parameters.reward_slashing_rate)?),
        ]
    )?;

    let tx_kind = TransactionKind::ProgrammableTransaction(ptb.finish());

    let response = execute_sui_transaction_no_events(publisher_address, tx_kind, context).await?;

    let object_changes = response.object_changes.unwrap();

    let ika_system_state_id = object_changes.iter().filter_map(|o| match o {
        ObjectChange::Created {
            object_id,
            object_type,
            ..
        } if IkaSystemStateWrapper::type_(ika_system_package_id.into()) == *object_type => Some(*object_id),
        _ => None,
    }).collect::<Vec<_>>().first().unwrap().clone();

    Ok(ika_system_state_id)
}

async fn request_add_validator(validator_address: SuiAddress, context: &mut WalletContext, client: SuiClient, ika_system_package_id: ObjectID, init_id: ObjectID, init_initial_shared_version: SequenceNumber) -> Result<(), anyhow::Error> {
    let mut ptb = ProgrammableTransactionBuilder::new();

    ptb.move_call(
        ika_system_package_id,
        Identifier::new("init")?,
        Identifier::new("request_add_validator")?,
        vec![],
        vec![
            CallArg::Object(ObjectArg::SharedObject {
                id: init_id,
                initial_shared_version: init_initial_shared_version,
                mutable: true,
            }),
        ]
    )?;

    let tx_kind = TransactionKind::ProgrammableTransaction(ptb.finish());

    let _ = execute_sui_transaction(validator_address, tx_kind, context).await?;

    Ok(())
}


async fn stake_ika(publisher_address: SuiAddress, context: &mut WalletContext, ika_system_package_id: ObjectID, init_id: ObjectID, init_initial_shared_version: SequenceNumber, ika_supply_id: ObjectID, validator_addresses: Vec<SuiAddress>) -> Result<(), anyhow::Error> {
    let mut ptb = ProgrammableTransactionBuilder::new();

    let init_arg = ptb.input(CallArg::Object(ObjectArg::SharedObject {
        id: init_id,
        initial_shared_version: init_initial_shared_version,
        mutable: true,
    }))?;

    let mut client = context.get_client().await?;

    let ika_supply_ref = client.transaction_builder().get_object_ref(ika_supply_id).await?;

    let ika_supply_id_arg = ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(ika_supply_ref)))?;
    let stake_amount = ptb.input(CallArg::Pure(bcs::to_bytes(&MIN_VALIDATOR_JOINING_STAKE_NIKA)?))?;

    for validator_address in validator_addresses {
        let stake = ptb.command(sui_types::transaction::Command::SplitCoins(
            ika_supply_id_arg,
            vec![stake_amount],
        ));
        let validator_arg = ptb.input(CallArg::Pure(bcs::to_bytes(&validator_address)?))?;
        ptb.command(
            sui_types::transaction::Command::move_call(
                ika_system_package_id,
                Identifier::new("init")?,
                Identifier::new("request_add_stake")?,
                vec![],
                vec![
                    init_arg,
                    stake,
                    validator_arg,
                ]
            )
        );
    }

    let tx_kind = TransactionKind::ProgrammableTransaction(ptb.finish());

    let _ = execute_sui_transaction_no_events(publisher_address, tx_kind, context).await?;

    Ok(())
}


async fn mint_ika(publisher_address: SuiAddress, context: &mut WalletContext, client: SuiClient, ika_package_id: ObjectID, treasury_cap_id: ObjectID) -> Result<ObjectID, anyhow::Error> {
    let mut ptb = ProgrammableTransactionBuilder::new();

    let treasury_cap_ref = client.transaction_builder().get_object_ref(treasury_cap_id).await?;

    let treasury_cap_arg = ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(treasury_cap_ref)))?;
    let total_supply_arg = ptb.input(CallArg::Pure(bcs::to_bytes(&TOTAL_SUPPLY_NIKA)?))?;
    let publisher_address_arg = ptb.input(CallArg::Pure(bcs::to_bytes(&publisher_address)?))?;
    ptb.command(
        sui_types::transaction::Command::move_call(
            SUI_FRAMEWORK_PACKAGE_ID,
            COIN_MODULE_NAME.into(),
            Identifier::new("mint_and_transfer")?,
            vec![IKA::type_tag(ika_package_id.into())],
            vec![
                treasury_cap_arg,
                total_supply_arg,
                publisher_address_arg,
            ]
        )
    );

    let tx_kind = TransactionKind::ProgrammableTransaction(ptb.finish());

    let response = execute_sui_transaction(publisher_address, tx_kind, context).await?;

    let object_changes = response.object_changes.unwrap();

    let ika_supply_id = object_changes.iter().filter_map(|o| match o {
        ObjectChange::Created {
            object_id,
            object_type,
            ..
        } if IKACoin::type_(ika_package_id.into()) == *object_type => Some(*object_id),
        _ => None,
    }).collect::<Vec<_>>().first().unwrap().clone();


    Ok(ika_supply_id)
}

async fn request_add_validator_candidate(validator_address: SuiAddress, context: &mut WalletContext, client: SuiClient, validator_initialization_metadata: &ValidatorInitializationMetadata, ika_system_package_id: ObjectID, init_id: ObjectID, init_initial_shared_version: SequenceNumber) -> Result<(), anyhow::Error> {
    let mut ptb = ProgrammableTransactionBuilder::new();

    ptb.move_call(
        ika_system_package_id,
        Identifier::new("init")?,
        Identifier::new("request_add_validator_candidate")?,
        vec![],
        vec![
            CallArg::Object(ObjectArg::SharedObject {
                id: init_id,
                initial_shared_version: init_initial_shared_version,
                mutable: true,
            }),
            CallArg::Pure(bcs::to_bytes(&validator_initialization_metadata.protocol_public_key.as_bytes().to_vec())?),
            CallArg::Pure(bcs::to_bytes(&validator_initialization_metadata.network_public_key.as_bytes().to_vec())?),
            CallArg::Pure(bcs::to_bytes(&validator_initialization_metadata.worker_public_key.as_bytes().to_vec())?),
            CallArg::Pure(bcs::to_bytes(&validator_initialization_metadata.proof_of_possession.as_ref().to_vec())?),
            CallArg::Pure(bcs::to_bytes(validator_initialization_metadata.name.as_bytes())?),
            CallArg::Pure(bcs::to_bytes(validator_initialization_metadata.name.as_bytes())?),
            CallArg::Pure(bcs::to_bytes(String::new().as_bytes())?),
            CallArg::Pure(bcs::to_bytes(String::new().as_bytes())?),
            CallArg::Pure(bcs::to_bytes(&validator_initialization_metadata.consensus_address.clone())?),
            CallArg::Pure(bcs::to_bytes(&validator_initialization_metadata.consensus_address.clone())?),
            CallArg::Pure(bcs::to_bytes(&validator_initialization_metadata.consensus_address.clone())?),
            CallArg::Pure(bcs::to_bytes(&validator_initialization_metadata.consensus_address.clone())?),
            CallArg::Pure(bcs::to_bytes(&validator_initialization_metadata.computation_price)?),
            CallArg::Pure(bcs::to_bytes(&validator_initialization_metadata.commission_rate)?),
        ],
    )?;

    let tx_kind = TransactionKind::ProgrammableTransaction(ptb.finish());

    let response = execute_sui_transaction(validator_address, tx_kind, context).await?;

    Ok(())
}

async fn initialize_ika_pre_launch(publisher_address: SuiAddress, context: &mut WalletContext, client: SuiClient, ika_system_package_id: ObjectID, init_cap_id: ObjectID) -> Result<ObjectID, anyhow::Error> {
    let mut ptb = ProgrammableTransactionBuilder::new();

    let init_cap_ref = client.transaction_builder().get_object_ref(init_cap_id).await?;
    ptb.move_call(
        ika_system_package_id,
        Identifier::new("init")?,
        Identifier::new("initialize_ika_pre_launch")?,
        vec![],
        vec![
            CallArg::Object(ObjectArg::ImmOrOwnedObject(init_cap_ref))
        ]
    )?;

    let tx_kind = TransactionKind::ProgrammableTransaction(ptb.finish());

    let response = execute_sui_transaction(publisher_address, tx_kind, context).await?;

    let object_changes = response.object_changes.unwrap();

    let init_type = StructTag {
        address: ika_system_package_id.into(),
        module: Identifier::new("init")?,
        name: Identifier::new("Init")?,
        type_params: vec![],
    };

    let init_id = object_changes.iter().filter_map(|o| match o {
        ObjectChange::Created {
            object_id,
            object_type,
            ..
        } if init_type == *object_type => Some(*object_id),
        _ => None,
    }).collect::<Vec<_>>().first().unwrap().clone();

    Ok(init_id)
}

async fn publish_ika_system_package_to_sui(publisher_address: SuiAddress, context: &mut WalletContext, client: SuiClient, ika_system_package: &IkaMovePackage, ika_package_id: ObjectID) -> Result<(ObjectID, ObjectID), anyhow::Error> {
    let mut ika_system_package_dependencies = ika_system_package.dependencies.clone();
    ika_system_package_dependencies.push(ika_package_id);

    let bytes = ika_system_package.bytes_with_deps(HashMap::from([("ika".to_string(), ika_package_id)]))?;


    let object_changes = publish_package_to_sui(publisher_address, context, client, bytes, ika_system_package_dependencies).await?;
    let ika_system_package_id = object_changes.iter().filter_map(|o| match o {
        ObjectChange::Published {
            package_id, ..
        } => Some(*package_id),
        _ => None,
    }).collect::<Vec<_>>().first().unwrap().clone();

    let init_cap_type = StructTag {
        address: ika_system_package_id.into(),
        module: Identifier::new("init")?,
        name: Identifier::new("InitCap")?,
        type_params: vec![],
    };

    let init_cap_id = object_changes.iter().filter_map(|o| match o {
        ObjectChange::Created {
            object_id,
            object_type,
            ..
        } if init_cap_type == *object_type => Some(*object_id),
        _ => None,
    }).collect::<Vec<_>>().first().unwrap().clone();

    Ok((ika_system_package_id, init_cap_id))
}

async fn publish_ika_package_to_sui(publisher_address: SuiAddress, context: &mut WalletContext, client: SuiClient, ika_package: &IkaMovePackage) -> Result<(ObjectID, ObjectID), anyhow::Error> {
    let object_changes = publish_package_to_sui(publisher_address, context, client, ika_package.bytes.clone(), ika_package.dependencies.clone()).await?;
    let ika_package_id = object_changes.iter().filter_map(|o| match o {
        ObjectChange::Published {
            package_id, ..
        } => Some(*package_id),
        _ => None,
    }).collect::<Vec<_>>().first().unwrap().clone();

    let treasury_cap_id = object_changes.iter().filter_map(|o| match o {
        ObjectChange::Created {
            object_id,
            object_type,
            ..
        } if TreasuryCap::is_treasury_type(object_type) => Some(*object_id),
        _ => None,
    }).collect::<Vec<_>>().first().unwrap().clone();

    Ok((ika_package_id, treasury_cap_id))
}

async fn publish_package_to_sui(publisher_address: SuiAddress, context: &mut WalletContext, client: SuiClient, ika_move_package_bytes: Vec<Vec<u8>>, ika_move_package_dep_ids: Vec<ObjectID>,) -> Result<Vec<ObjectChange>, anyhow::Error> {
    let tx_kind = client
        .transaction_builder()
        .publish_tx_kind(
            publisher_address,
            ika_move_package_bytes,
            ika_move_package_dep_ids,
        )
        .await?;

    let response = execute_sui_transaction(publisher_address, tx_kind, context).await?;

    let object_changes = response.object_changes.unwrap();
    Ok(object_changes)
}

pub(crate) async fn create_sui_transaction(
    signer: SuiAddress,
    tx_kind: TransactionKind,
    context: &mut WalletContext,
) -> Result<Transaction, anyhow::Error> {
    let gas_price = context.get_reference_gas_price().await?;

    let client = context.get_client().await?;

    let gas_budget = max_gas_budget(&client).await?;
    // let gas_budget = estimate_gas_budget(
    //     context,
    //     signer,
    //     tx_kind.clone(),
    //     gas_price,
    //     None,
    //     None,
    // ).await?;

    let tx_data = client
        .transaction_builder()
        .tx_data(
            signer,
            tx_kind,
            gas_budget,
            gas_price,
            vec![],
            None,
        )
        .await?;

    let signature = context.config.keystore.sign_secure(
        &tx_data.sender(),
        &tx_data,
        Intent::sui_transaction(),
    )?;
    let sender_signed_data = SenderSignedData::new_from_sender_signature(tx_data, signature);

    let transaction = Transaction::new(sender_signed_data);

    Ok(transaction)
}

pub(crate) async fn execute_sui_transaction(
    signer: SuiAddress,
    tx_kind: TransactionKind,
    context: &mut WalletContext,
) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let transaction = create_sui_transaction(signer, tx_kind, context).await?;

    let response = context
        .execute_transaction_may_fail(transaction.clone())
        .await?;
    Ok(response)
}

// Right now execute_sui_transaction crashes for somme txs, this is an hack to make it work for now.
pub(crate) async fn execute_sui_transaction_no_events(
    signer: SuiAddress,
    tx_kind: TransactionKind,
    context: &mut WalletContext,
) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let transaction = create_sui_transaction(signer, tx_kind, context).await?;

    let client = context.get_client().await?;

    let response = client
        .quorum_driver_api()
        .execute_transaction_block(
            transaction,
            SuiTransactionBlockResponseOptions::new()
                .with_effects()
                .with_input()
                .with_object_changes()
                .with_balance_changes(),
            Some(sui_types::quorum_driver_types::ExecuteTransactionRequestType::WaitForEffectsCert)).await?;
    Ok(response)
}

pub async fn estimate_gas_budget(
    context: &mut WalletContext,
    signer: SuiAddress,
    kind: TransactionKind,
    gas_price: u64,
    gas_payment: Option<Vec<ObjectID>>,
    sponsor: Option<SuiAddress>,
) -> Result<u64, anyhow::Error> {
    let client = context.get_client().await?;
    let SuiClientCommandResult::DryRun(dry_run) =
        execute_dry_run(context, signer, kind, None, gas_price, gas_payment, sponsor).await?
    else {
        bail!("Wrong SuiClientCommandResult. Should be SuiClientCommandResult::DryRun.")
    };

    let rgp = client.read_api().get_reference_gas_price().await?;

    Ok(estimate_gas_budget_from_gas_cost(
        dry_run.effects.gas_cost_summary(),
        rgp,
    ))
}