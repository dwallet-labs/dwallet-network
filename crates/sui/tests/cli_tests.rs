// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::collections::BTreeSet;
use std::io::Read;
use std::os::unix::prelude::FileExt;
use std::str::FromStr;
use std::{fmt::Write, fs::read_dir, path::PathBuf, str, thread, time::Duration};

use expect_test::expect;
use move_package::BuildConfig as MoveBuildConfig;
use serde_json::json;
use sui::key_identity::{get_identity_address, KeyIdentity};
use sui_test_transaction_builder::batch_make_transfer_transactions;
use sui_types::object::Owner;
use sui_types::transaction::{
    TEST_ONLY_GAS_UNIT_FOR_GENERIC, TEST_ONLY_GAS_UNIT_FOR_OBJECT_BASICS,
    TEST_ONLY_GAS_UNIT_FOR_PUBLISH, TEST_ONLY_GAS_UNIT_FOR_SPLIT_COIN,
    TEST_ONLY_GAS_UNIT_FOR_TRANSFER,
};
use tokio::time::sleep;

use sui::client_commands::SwitchResponse;
use sui::{
    client_commands::{SuiClientCommandResult, SuiClientCommands},
    sui_commands::SuiCommand,
};
use sui_config::{
    PersistedConfig, SUI_CLIENT_CONFIG, SUI_FULLNODE_CONFIG, SUI_GENESIS_FILENAME,
    SUI_KEYSTORE_ALIASES_FILENAME, SUI_KEYSTORE_FILENAME, SUI_NETWORK_CONFIG,
};
use sui_json::SuiJsonValue;
use sui_json_rpc_types::{
    OwnedObjectRef, SuiObjectData, SuiObjectDataFilter, SuiObjectDataOptions, SuiObjectResponse,
    SuiObjectResponseQuery, SuiTransactionBlockEffects, SuiTransactionBlockEffectsAPI,
};
use sui_keys::keystore::AccountKeystore;
use sui_macros::sim_test;
use sui_move_build::{BuildConfig, SuiPackageHooks};
use sui_sdk::sui_client_config::SuiClientConfig;
use sui_sdk::wallet_context::WalletContext;
use sui_swarm_config::genesis_config::{AccountConfig, GenesisConfig};
use sui_swarm_config::network_config::NetworkConfig;
use sui_types::base_types::SuiAddress;
use sui_types::crypto::{
    Ed25519SuiSignature, Secp256k1SuiSignature, SignatureScheme, SuiKeyPair, SuiSignatureInner,
};
use sui_types::error::SuiObjectResponseError;
use sui_types::{base_types::ObjectID, crypto::get_key_pair, gas_coin::GasCoin};
use test_cluster::TestClusterBuilder;

const TEST_DATA_DIR: &str = "tests/data/";

#[sim_test]
async fn test_genesis() -> Result<(), anyhow::Error> {
    let temp_dir = tempfile::tempdir()?;
    let working_dir = temp_dir.path();
    let config = working_dir.join(SUI_NETWORK_CONFIG);

    // Start network without authorities
    let start = SuiCommand::Start {
        config: Some(config),
        no_full_node: false,
    }
    .execute()
    .await;
    assert!(matches!(start, Err(..)));
    // Genesis
    SuiCommand::Genesis {
        working_dir: Some(working_dir.to_path_buf()),
        write_config: None,
        force: false,
        from_config: None,
        epoch_duration_ms: None,
        benchmark_ips: None,
        with_faucet: false,
    }
    .execute()
    .await?;

    // Get all the new file names
    let files = read_dir(working_dir)?
        .flat_map(|r| r.map(|file| file.file_name().to_str().unwrap().to_owned()))
        .collect::<Vec<_>>();

    assert_eq!(10, files.len());
    assert!(files.contains(&SUI_CLIENT_CONFIG.to_string()));
    assert!(files.contains(&SUI_NETWORK_CONFIG.to_string()));
    assert!(files.contains(&SUI_FULLNODE_CONFIG.to_string()));
    assert!(files.contains(&SUI_GENESIS_FILENAME.to_string()));
    assert!(files.contains(&SUI_KEYSTORE_FILENAME.to_string()));
    assert!(files.contains(&SUI_KEYSTORE_ALIASES_FILENAME.to_string()));

    // Check network config
    let network_conf =
        PersistedConfig::<NetworkConfig>::read(&working_dir.join(SUI_NETWORK_CONFIG))?;
    assert_eq!(4, network_conf.validator_configs().len());

    // Check wallet config
    let wallet_conf =
        PersistedConfig::<SuiClientConfig>::read(&working_dir.join(SUI_CLIENT_CONFIG))?;

    assert!(!wallet_conf.envs.is_empty());

    assert_eq!(5, wallet_conf.keystore.addresses().len());

    // Genesis 2nd time should fail
    let result = SuiCommand::Genesis {
        working_dir: Some(working_dir.to_path_buf()),
        write_config: None,
        force: false,
        from_config: None,
        epoch_duration_ms: None,
        benchmark_ips: None,
        with_faucet: false,
    }
    .execute()
    .await;
    assert!(matches!(result, Err(..)));

    temp_dir.close()?;
    Ok(())
}

#[tokio::test]
async fn test_addresses_command() -> Result<(), anyhow::Error> {
    let test_cluster = TestClusterBuilder::new().build().await;
    let mut context = test_cluster.wallet;

    // Add 3 accounts
    for _ in 0..3 {
        context
            .config
            .keystore
            .add_key(None, SuiKeyPair::Ed25519(get_key_pair().1))?;
    }

    // Print all addresses
    SuiClientCommands::Addresses
        .execute(&mut context)
        .await
        .unwrap()
        .print(true);

    Ok(())
}

#[sim_test]
async fn test_objects_command() -> Result<(), anyhow::Error> {
    let mut test_cluster = TestClusterBuilder::new().build().await;
    let address = test_cluster.get_address_0();
    let context = &mut test_cluster.wallet;
    let alias = context
        .config
        .keystore
        .get_alias_by_address(&address)
        .unwrap();
    // Print objects owned by `address`
    SuiClientCommands::Objects {
        address: Some(KeyIdentity::Address(address)),
    }
    .execute(context)
    .await?
    .print(true);
    // Print objects owned by `address`, passing its alias
    SuiClientCommands::Objects {
        address: Some(KeyIdentity::Alias(alias)),
    }
    .execute(context)
    .await?
    .print(true);
    let client = context.get_client().await?;
    let _object_refs = client
        .read_api()
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_previous_transaction(),
            )),
            None,
            None,
        )
        .await?;

    Ok(())
}

// fixing issue https://github.com/MystenLabs/sui/issues/6546
#[tokio::test]
async fn test_regression_6546() -> Result<(), anyhow::Error> {
    let mut test_cluster = TestClusterBuilder::new().build().await;
    let address = test_cluster.get_address_0();
    let context = &mut test_cluster.wallet;

    let SuiClientCommandResult::Objects(coins) = SuiClientCommands::Objects {
        address: Some(KeyIdentity::Address(address)),
    }
    .execute(context)
    .await?
    else {
        panic!()
    };
    let config_path = test_cluster.swarm.dir().join(SUI_CLIENT_CONFIG);

    test_with_sui_binary(&[
        "client",
        "--client.config",
        config_path.to_str().unwrap(),
        "call",
        "--package",
        "0x2",
        "--module",
        "dwallet",
        "--function",
        "transfer",
        "--args",
        &coins.first().unwrap().object()?.object_id.to_string(),
        &test_cluster.get_address_1().to_string(),
        "--gas-budget",
        "100000000",
    ])
    .await
}

#[sim_test]
async fn test_custom_genesis() -> Result<(), anyhow::Error> {
    // Create and save genesis config file
    // Create 4 authorities, 1 account with 1 gas object with custom id

    let mut config = GenesisConfig::for_local_testing();
    config.accounts.clear();
    config.accounts.push(AccountConfig {
        address: None,
        gas_amounts: vec![500],
    });
    let mut cluster = TestClusterBuilder::new()
        .set_genesis_config(config)
        .build()
        .await;
    let address = cluster.get_address_0();
    let context = cluster.wallet_mut();

    assert_eq!(1, context.config.keystore.addresses().len());

    // Print objects owned by `address`
    SuiClientCommands::Objects {
        address: Some(KeyIdentity::Address(address)),
    }
    .execute(context)
    .await?
    .print(true);

    Ok(())
}

#[sim_test]
async fn test_object_info_get_command() -> Result<(), anyhow::Error> {
    let mut test_cluster = TestClusterBuilder::new().build().await;

    let address = test_cluster.get_address_0();
    let context = &mut test_cluster.wallet;
    let client = context.get_client().await?;

    let object_refs = client
        .read_api()
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::new(),
            )),
            None,
            None,
        )
        .await?
        .data;

    // Check log output contains all object ids.
    let object_id = object_refs.first().unwrap().object().unwrap().object_id;

    SuiClientCommands::Object {
        id: object_id,
        bcs: false,
    }
    .execute(context)
    .await?
    .print(true);

    SuiClientCommands::Object {
        id: object_id,
        bcs: true,
    }
    .execute(context)
    .await?
    .print(true);

    Ok(())
}

#[sim_test]
async fn test_gas_command() -> Result<(), anyhow::Error> {
    let mut test_cluster = TestClusterBuilder::new().build().await;
    let rgp = test_cluster.get_reference_gas_price().await;
    let address = test_cluster.get_address_0();
    let context = &mut test_cluster.wallet;
    let alias = context
        .config
        .keystore
        .get_alias_by_address(&address)
        .unwrap();

    let client = context.get_client().await?;
    let object_refs = client
        .read_api()
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::full_content(),
            )),
            None,
            None,
        )
        .await?;

    let object_id = object_refs
        .data
        .first()
        .unwrap()
        .object()
        .unwrap()
        .object_id;
    let object_to_send = object_refs.data.get(1).unwrap().object().unwrap().object_id;

    SuiClientCommands::Gas {
        address: Some(KeyIdentity::Address(address)),
    }
    .execute(context)
    .await?
    .print(true);

    tokio::time::sleep(Duration::from_millis(100)).await;

    // Send an object
    SuiClientCommands::Transfer {
        to: KeyIdentity::Address(SuiAddress::random_for_testing_only()),
        object_id: object_to_send,
        gas: Some(object_id),
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_TRANSFER,
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await?;

    // Fetch gas again, and use the alias instead of the address
    SuiClientCommands::Gas {
        address: Some(KeyIdentity::Alias(alias)),
    }
    .execute(context)
    .await?
    .print(true);

    Ok(())
}

#[sim_test]
async fn test_move_call_args_linter_command() -> Result<(), anyhow::Error> {
    let mut test_cluster = TestClusterBuilder::new().build().await;
    let rgp = test_cluster.get_reference_gas_price().await;
    let address1 = test_cluster.get_address_0();
    let context = &mut test_cluster.wallet;

    let address2 = SuiAddress::random_for_testing_only();

    let client = context.get_client().await?;
    // publish the object basics package
    let object_refs = client
        .read_api()
        .get_owned_objects(
            address1,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::full_content(),
            )),
            None,
            None,
        )
        .await?
        .data;
    let gas_obj_id = object_refs.first().unwrap().object().unwrap().object_id;
    let mut package_path = PathBuf::from(TEST_DATA_DIR);
    package_path.push("move_call_args_linter");
    let build_config = BuildConfig::new_for_testing().config;
    let resp = SuiClientCommands::Publish {
        package_path,
        build_config,
        gas: Some(gas_obj_id),
        gas_budget: TEST_ONLY_GAS_UNIT_FOR_PUBLISH * rgp,
        skip_dependency_verification: false,
        with_unpublished_dependencies: false,
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await?;

    let package = if let SuiClientCommandResult::Publish(response) = resp {
        assert!(
            response.status_ok().unwrap(),
            "Command failed: {:?}",
            response
        );
        response
            .effects
            .unwrap()
            .created()
            .iter()
            .find(
                |OwnedObjectRef {
                     owner,
                     reference: _,
                 }| matches!(owner, Owner::Immutable),
            )
            .unwrap()
            .reference
            .object_id
    } else {
        unreachable!("Invalid response");
    };

    // Print objects owned by `address1`
    SuiClientCommands::Objects {
        address: Some(KeyIdentity::Address(address1)),
    }
    .execute(context)
    .await?
    .print(true);
    tokio::time::sleep(Duration::from_millis(2000)).await;
    let client = context.get_client().await?;
    let object_refs = client
        .read_api()
        .get_owned_objects(
            address1,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_previous_transaction(),
            )),
            None,
            None,
        )
        .await?
        .data;

    // Create an object for address1 using Move call

    // Certain prep work
    // Get a gas object
    let coins: Vec<_> = object_refs
        .iter()
        .filter(|object_ref| object_ref.object().unwrap().is_gas_coin())
        .collect();
    let gas = coins.first().unwrap().object()?.object_id;
    let obj = coins.get(1).unwrap().object()?.object_id;

    // Create the args
    let args = vec![
        SuiJsonValue::new(json!("123"))?,
        SuiJsonValue::new(json!(address1))?,
    ];

    // Test case with no gas specified
    let resp = SuiClientCommands::Call {
        package,
        module: "object_basics".to_string(),
        function: "create".to_string(),
        type_args: vec![],
        args,
        gas: None,
        gas_budget: TEST_ONLY_GAS_UNIT_FOR_OBJECT_BASICS * rgp,
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await?;
    resp.print(true);

    // Get the created object
    let created_obj: ObjectID = if let SuiClientCommandResult::Call(resp) = resp {
        resp.effects
            .unwrap()
            .created()
            .first()
            .unwrap()
            .reference
            .object_id
    } else {
        panic!();
    };

    // Try a bad argument: decimal
    let args_json = json!([0.3f32, address1]);
    assert!(SuiJsonValue::new(args_json.as_array().unwrap().get(0).unwrap().clone()).is_err());

    // Try a bad argument: too few args
    let args_json = json!([300usize]);
    let mut args = vec![];
    for a in args_json.as_array().unwrap() {
        args.push(SuiJsonValue::new(a.clone()).unwrap());
    }

    let resp = SuiClientCommands::Call {
        package,
        module: "object_basics".to_string(),
        function: "create".to_string(),
        type_args: vec![],
        args: args.to_vec(),
        gas: Some(gas),
        gas_budget: TEST_ONLY_GAS_UNIT_FOR_OBJECT_BASICS * rgp,
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await;

    assert!(resp.is_err());

    let err_string = format!("{} ", resp.err().unwrap());
    assert!(err_string.contains("Expected 2 args, found 1"));

    // Try a transfer
    // This should fail due to mismatch of object being sent
    let args = vec![
        SuiJsonValue::new(json!(obj))?,
        SuiJsonValue::new(json!(address2))?,
    ];

    let resp = SuiClientCommands::Call {
        package,
        module: "object_basics".to_string(),
        function: "transfer".to_string(),
        type_args: vec![],
        args: args.to_vec(),
        gas: Some(gas),
        gas_budget: TEST_ONLY_GAS_UNIT_FOR_OBJECT_BASICS * rgp,
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await;

    assert!(resp.is_err());

    // FIXME: uncomment once we figure out what is going on with `resolve_and_type_check`
    // let err_string = format!("{} ", resp.err().unwrap());
    // let framework_addr = SUI_FRAMEWORK_ADDRESS.to_hex_literal();
    // let package_addr = package.to_hex_literal();
    // assert!(err_string.contains(&format!("Expected argument of type {package_addr}::object_basics::Object, but found type {framework_addr}::coin::Coin<{framework_addr}::sui::SUI>")));

    // Try a proper transfer
    let args = vec![
        SuiJsonValue::new(json!(created_obj))?,
        SuiJsonValue::new(json!(address2))?,
    ];

    SuiClientCommands::Call {
        package,
        module: "object_basics".to_string(),
        function: "transfer".to_string(),
        type_args: vec![],
        args: args.to_vec(),
        gas: Some(gas),
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_OBJECT_BASICS,
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await?;

    Ok(())
}

#[sim_test]
async fn test_package_publish_command() -> Result<(), anyhow::Error> {
    let mut test_cluster = TestClusterBuilder::new().build().await;
    let rgp = test_cluster.get_reference_gas_price().await;
    let address = test_cluster.get_address_0();
    let context = &mut test_cluster.wallet;

    let client = context.get_client().await?;
    let object_refs = client
        .read_api()
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_previous_transaction(),
            )),
            None,
            None,
        )
        .await?
        .data;

    // Check log output contains all object ids.
    let gas_obj_id = object_refs.first().unwrap().object().unwrap().object_id;

    // Provide path to well formed package sources
    let mut package_path = PathBuf::from(TEST_DATA_DIR);
    package_path.push("dummy_modules_publish");
    let build_config = BuildConfig::new_for_testing().config;
    let resp = SuiClientCommands::Publish {
        package_path,
        build_config,
        gas: Some(gas_obj_id),
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_PUBLISH,
        skip_dependency_verification: false,
        with_unpublished_dependencies: false,
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await?;

    // Print it out to CLI/logs
    resp.print(true);

    let obj_ids = if let SuiClientCommandResult::Publish(response) = resp {
        response
            .effects
            .as_ref()
            .unwrap()
            .created()
            .iter()
            .map(|refe| refe.reference.object_id)
            .collect::<Vec<_>>()
    } else {
        unreachable!("Invalid response");
    };

    // Check the objects
    for obj_id in obj_ids {
        get_parsed_object_assert_existence(obj_id, context).await;
    }

    Ok(())
}

#[sim_test]
async fn test_delete_shared_object() -> Result<(), anyhow::Error> {
    let mut test_cluster = TestClusterBuilder::new().build().await;
    let rgp = test_cluster.get_reference_gas_price().await;
    let address = test_cluster.get_address_0();
    let context = &mut test_cluster.wallet;

    let client = context.get_client().await?;
    let object_refs = client
        .read_api()
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_previous_transaction(),
            )),
            None,
            None,
        )
        .await?
        .data;

    let gas_obj_id = object_refs.first().unwrap().object().unwrap().object_id;

    // Provide path to well formed package sources
    let mut package_path = PathBuf::from(TEST_DATA_DIR);
    package_path.push("sod");
    let build_config = BuildConfig::new_for_testing().config;
    let resp = SuiClientCommands::Publish {
        package_path,
        build_config,
        gas: Some(gas_obj_id),
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_PUBLISH,
        skip_dependency_verification: false,
        with_unpublished_dependencies: false,
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await?;

    let owned_obj_ids = if let SuiClientCommandResult::Publish(response) = resp {
        let x = response.effects.unwrap();
        x.created().to_vec()
    } else {
        unreachable!("Invalid response");
    };

    // Check the objects
    for OwnedObjectRef { reference, .. } in &owned_obj_ids {
        get_parsed_object_assert_existence(reference.object_id, context).await;
    }

    let package_id = owned_obj_ids
        .into_iter()
        .find(|OwnedObjectRef { owner, .. }| owner == &Owner::Immutable)
        .expect("Must find published package ID")
        .reference;

    // Start and then receive the object
    let start_call_result = SuiClientCommands::Call {
        package: (*package_id.object_id).into(),
        module: "sod".to_string(),
        function: "start".to_string(),
        type_args: vec![],
        gas: None,
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_PUBLISH,
        args: vec![],
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await?;

    let shared_id = if let SuiClientCommandResult::Call(response) = start_call_result {
        response.effects.unwrap().created().to_vec()[0]
            .reference
            .object_id
    } else {
        unreachable!("Invalid response");
    };

    let delete_result = SuiClientCommands::Call {
        package: (*package_id.object_id).into(),
        module: "sod".to_string(),
        function: "delete".to_string(),
        type_args: vec![],
        gas: None,
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_PUBLISH,
        args: vec![SuiJsonValue::from_str(&shared_id.to_string()).unwrap()],
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await?;

    if let SuiClientCommandResult::Call(response) = delete_result {
        assert!(response.effects.unwrap().into_status().is_ok());
    } else {
        unreachable!("Invalid response");
    };

    Ok(())
}

#[sim_test]
async fn test_receive_argument() -> Result<(), anyhow::Error> {
    let mut test_cluster = TestClusterBuilder::new().build().await;
    let rgp = test_cluster.get_reference_gas_price().await;
    let address = test_cluster.get_address_0();
    let context = &mut test_cluster.wallet;

    let client = context.get_client().await?;
    let object_refs = client
        .read_api()
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_previous_transaction(),
            )),
            None,
            None,
        )
        .await?
        .data;

    let gas_obj_id = object_refs.first().unwrap().object().unwrap().object_id;

    // Provide path to well formed package sources
    let mut package_path = PathBuf::from(TEST_DATA_DIR);
    package_path.push("tto");
    let build_config = BuildConfig::new_for_testing().config;
    let resp = SuiClientCommands::Publish {
        package_path,
        build_config,
        gas: Some(gas_obj_id),
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_PUBLISH,
        skip_dependency_verification: false,
        with_unpublished_dependencies: false,
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await?;

    let owned_obj_ids = if let SuiClientCommandResult::Publish(response) = resp {
        let x = response.effects.unwrap();
        x.created().to_vec()
    } else {
        unreachable!("Invalid response");
    };

    // Check the objects
    for OwnedObjectRef { reference, .. } in &owned_obj_ids {
        get_parsed_object_assert_existence(reference.object_id, context).await;
    }

    let package_id = owned_obj_ids
        .into_iter()
        .find(|OwnedObjectRef { owner, .. }| owner == &Owner::Immutable)
        .expect("Must find published package ID")
        .reference;

    // Start and then receive the object
    let start_call_result = SuiClientCommands::Call {
        package: (*package_id.object_id).into(),
        module: "tto".to_string(),
        function: "start".to_string(),
        type_args: vec![],
        gas: None,
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_PUBLISH,
        args: vec![],
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await?;

    let (parent, child) = if let SuiClientCommandResult::Call(response) = start_call_result {
        let created = response.effects.unwrap().created().to_vec();
        let owners: BTreeSet<ObjectID> = created
            .iter()
            .flat_map(|refe| {
                refe.owner
                    .get_address_owner_address()
                    .ok()
                    .map(|x| x.into())
            })
            .collect();
        let child = created
            .iter()
            .find(|refe| !owners.contains(&refe.reference.object_id))
            .unwrap();
        let parent = created
            .iter()
            .find(|refe| owners.contains(&refe.reference.object_id))
            .unwrap();
        (parent.reference.clone(), child.reference.clone())
    } else {
        unreachable!("Invalid response");
    };

    let receive_result = SuiClientCommands::Call {
        package: (*package_id.object_id).into(),
        module: "tto".to_string(),
        function: "receiver".to_string(),
        type_args: vec![],
        gas: None,
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_PUBLISH,
        args: vec![
            SuiJsonValue::from_str(&parent.object_id.to_string()).unwrap(),
            SuiJsonValue::from_str(&child.object_id.to_string()).unwrap(),
        ],
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await?;

    if let SuiClientCommandResult::Call(response) = receive_result {
        assert!(response.effects.unwrap().into_status().is_ok());
    } else {
        unreachable!("Invalid response");
    };

    Ok(())
}

#[sim_test]
async fn test_receive_argument_by_immut_ref() -> Result<(), anyhow::Error> {
    let mut test_cluster = TestClusterBuilder::new().build().await;
    let rgp = test_cluster.get_reference_gas_price().await;
    let address = test_cluster.get_address_0();
    let context = &mut test_cluster.wallet;

    let client = context.get_client().await?;
    let object_refs = client
        .read_api()
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_previous_transaction(),
            )),
            None,
            None,
        )
        .await?
        .data;

    let gas_obj_id = object_refs.first().unwrap().object().unwrap().object_id;

    // Provide path to well formed package sources
    let mut package_path = PathBuf::from(TEST_DATA_DIR);
    package_path.push("tto");
    let build_config = BuildConfig::new_for_testing().config;
    let resp = SuiClientCommands::Publish {
        package_path,
        build_config,
        gas: Some(gas_obj_id),
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_PUBLISH,
        skip_dependency_verification: false,
        with_unpublished_dependencies: false,
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await?;

    let owned_obj_ids = if let SuiClientCommandResult::Publish(response) = resp {
        let x = response.effects.unwrap();
        x.created().to_vec()
    } else {
        unreachable!("Invalid response");
    };

    // Check the objects
    for OwnedObjectRef { reference, .. } in &owned_obj_ids {
        get_parsed_object_assert_existence(reference.object_id, context).await;
    }

    let package_id = owned_obj_ids
        .into_iter()
        .find(|OwnedObjectRef { owner, .. }| owner == &Owner::Immutable)
        .expect("Must find published package ID")
        .reference;

    // Start and then receive the object
    let start_call_result = SuiClientCommands::Call {
        package: (*package_id.object_id).into(),
        module: "tto".to_string(),
        function: "start".to_string(),
        type_args: vec![],
        gas: None,
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_PUBLISH,
        args: vec![],
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await?;

    let (parent, child) = if let SuiClientCommandResult::Call(response) = start_call_result {
        let created = response.effects.unwrap().created().to_vec();
        let owners: BTreeSet<ObjectID> = created
            .iter()
            .flat_map(|refe| {
                refe.owner
                    .get_address_owner_address()
                    .ok()
                    .map(|x| x.into())
            })
            .collect();
        let child = created
            .iter()
            .find(|refe| !owners.contains(&refe.reference.object_id))
            .unwrap();
        let parent = created
            .iter()
            .find(|refe| owners.contains(&refe.reference.object_id))
            .unwrap();
        (parent.reference.clone(), child.reference.clone())
    } else {
        unreachable!("Invalid response");
    };

    let receive_result = SuiClientCommands::Call {
        package: (*package_id.object_id).into(),
        module: "tto".to_string(),
        function: "invalid_call_immut_ref".to_string(),
        type_args: vec![],
        gas: None,
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_PUBLISH,
        args: vec![
            SuiJsonValue::from_str(&parent.object_id.to_string()).unwrap(),
            SuiJsonValue::from_str(&child.object_id.to_string()).unwrap(),
        ],
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await?;

    if let SuiClientCommandResult::Call(response) = receive_result {
        assert!(response.effects.unwrap().into_status().is_ok());
    } else {
        unreachable!("Invalid response");
    };

    Ok(())
}

#[sim_test]
async fn test_receive_argument_by_mut_ref() -> Result<(), anyhow::Error> {
    let mut test_cluster = TestClusterBuilder::new().build().await;
    let rgp = test_cluster.get_reference_gas_price().await;
    let address = test_cluster.get_address_0();
    let context = &mut test_cluster.wallet;

    let client = context.get_client().await?;
    let object_refs = client
        .read_api()
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_previous_transaction(),
            )),
            None,
            None,
        )
        .await?
        .data;

    let gas_obj_id = object_refs.first().unwrap().object().unwrap().object_id;

    // Provide path to well formed package sources
    let mut package_path = PathBuf::from(TEST_DATA_DIR);
    package_path.push("tto");
    let build_config = BuildConfig::new_for_testing().config;
    let resp = SuiClientCommands::Publish {
        package_path,
        build_config,
        gas: Some(gas_obj_id),
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_PUBLISH,
        skip_dependency_verification: false,
        with_unpublished_dependencies: false,
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await?;

    let owned_obj_ids = if let SuiClientCommandResult::Publish(response) = resp {
        let x = response.effects.unwrap();
        x.created().to_vec()
    } else {
        unreachable!("Invalid response");
    };

    // Check the objects
    for OwnedObjectRef { reference, .. } in &owned_obj_ids {
        get_parsed_object_assert_existence(reference.object_id, context).await;
    }

    let package_id = owned_obj_ids
        .into_iter()
        .find(|OwnedObjectRef { owner, .. }| owner == &Owner::Immutable)
        .expect("Must find published package ID")
        .reference;

    // Start and then receive the object
    let start_call_result = SuiClientCommands::Call {
        package: (*package_id.object_id).into(),
        module: "tto".to_string(),
        function: "start".to_string(),
        type_args: vec![],
        gas: None,
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_PUBLISH,
        args: vec![],
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await?;

    let (parent, child) = if let SuiClientCommandResult::Call(response) = start_call_result {
        let created = response.effects.unwrap().created().to_vec();
        let owners: BTreeSet<ObjectID> = created
            .iter()
            .flat_map(|refe| {
                refe.owner
                    .get_address_owner_address()
                    .ok()
                    .map(|x| x.into())
            })
            .collect();
        let child = created
            .iter()
            .find(|refe| !owners.contains(&refe.reference.object_id))
            .unwrap();
        let parent = created
            .iter()
            .find(|refe| owners.contains(&refe.reference.object_id))
            .unwrap();
        (parent.reference.clone(), child.reference.clone())
    } else {
        unreachable!("Invalid response");
    };

    let receive_result = SuiClientCommands::Call {
        package: (*package_id.object_id).into(),
        module: "tto".to_string(),
        function: "invalid_call_mut_ref".to_string(),
        type_args: vec![],
        gas: None,
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_PUBLISH,
        args: vec![
            SuiJsonValue::from_str(&parent.object_id.to_string()).unwrap(),
            SuiJsonValue::from_str(&child.object_id.to_string()).unwrap(),
        ],
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await?;

    if let SuiClientCommandResult::Call(response) = receive_result {
        assert!(response.effects.unwrap().into_status().is_ok());
    } else {
        unreachable!("Invalid response");
    };

    Ok(())
}

#[sim_test]
async fn test_package_publish_command_with_unpublished_dependency_succeeds(
) -> Result<(), anyhow::Error> {
    let with_unpublished_dependencies = true; // Value under test, results in successful response.

    let mut test_cluster = TestClusterBuilder::new().build().await;
    let rgp = test_cluster.get_reference_gas_price().await;
    let address = test_cluster.get_address_0();
    let context = &mut test_cluster.wallet;

    let client = context.get_client().await?;
    let object_refs = client
        .read_api()
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_previous_transaction(),
            )),
            None,
            None,
        )
        .await?
        .data;

    let gas_obj_id = object_refs.first().unwrap().object()?.object_id;

    let mut package_path = PathBuf::from(TEST_DATA_DIR);
    package_path.push("module_publish_with_unpublished_dependency");
    let build_config = BuildConfig::new_for_testing().config;
    let resp = SuiClientCommands::Publish {
        package_path,
        build_config,
        gas: Some(gas_obj_id),
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_PUBLISH,
        skip_dependency_verification: false,
        with_unpublished_dependencies,
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await?;

    // Print it out to CLI/logs
    resp.print(true);

    let obj_ids = if let SuiClientCommandResult::Publish(response) = resp {
        response
            .effects
            .as_ref()
            .unwrap()
            .created()
            .iter()
            .map(|refe| refe.reference.object_id)
            .collect::<Vec<_>>()
    } else {
        unreachable!("Invalid response");
    };

    // Check the objects
    for obj_id in obj_ids {
        get_parsed_object_assert_existence(obj_id, context).await;
    }

    Ok(())
}

#[sim_test]
async fn test_package_publish_command_with_unpublished_dependency_fails(
) -> Result<(), anyhow::Error> {
    let with_unpublished_dependencies = false; // Value under test, results in error response.

    let mut test_cluster = TestClusterBuilder::new().build().await;
    let rgp = test_cluster.get_reference_gas_price().await;
    let address = test_cluster.get_address_0();
    let context = &mut test_cluster.wallet;
    let client = context.get_client().await?;
    let object_refs = client
        .read_api()
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_previous_transaction(),
            )),
            None,
            None,
        )
        .await?
        .data;

    let gas_obj_id = object_refs.first().unwrap().object().unwrap().object_id;

    let mut package_path = PathBuf::from(TEST_DATA_DIR);
    package_path.push("module_publish_with_unpublished_dependency");
    let build_config = BuildConfig::new_for_testing().config;
    let result = SuiClientCommands::Publish {
        package_path,
        build_config,
        gas: Some(gas_obj_id),
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_PUBLISH,
        skip_dependency_verification: false,
        with_unpublished_dependencies,
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await;

    let expect = expect![[r#"
        Err(
            ModulePublishFailure {
                error: "Package dependency \"Unpublished\" does not specify a published address (the Move.toml manifest for \"Unpublished\" does not contain a published-at field).\nIf this is intentional, you may use the --with-unpublished-dependencies flag to continue publishing these dependencies as part of your package (they won't be linked against existing packages on-chain).",
            },
        )
    "#]];
    expect.assert_debug_eq(&result);
    Ok(())
}

#[sim_test]
async fn test_package_publish_command_non_zero_unpublished_dep_fails() -> Result<(), anyhow::Error>
{
    let with_unpublished_dependencies = true; // Value under test, incompatible with dependencies that specify non-zero address.

    let mut test_cluster = TestClusterBuilder::new().build().await;
    let rgp = test_cluster.get_reference_gas_price().await;
    let address = test_cluster.get_address_0();
    let context = &mut test_cluster.wallet;

    let client = context.get_client().await?;
    let object_refs = client
        .read_api()
        .get_owned_objects(address, None, None, None)
        .await?
        .data;

    let gas_obj_id = object_refs.first().unwrap().object().unwrap().object_id;

    let mut package_path = PathBuf::from(TEST_DATA_DIR);
    package_path.push("module_publish_with_unpublished_dependency_with_non_zero_address");
    let build_config = BuildConfig::new_for_testing().config;
    let result = SuiClientCommands::Publish {
        package_path,
        build_config,
        gas: Some(gas_obj_id),
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_PUBLISH,
        skip_dependency_verification: false,
        with_unpublished_dependencies,
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await;

    let expect = expect![[r#"
        Err(
            ModulePublishFailure {
                error: "The following modules in package dependencies set a non-zero self-address:\n - 0000000000000000000000000000000000000000000000000000000000000bad::non_zero in dependency UnpublishedNonZeroAddress\nIf these packages really are unpublished, their self-addresses should be set to \"0x0\" in the [addresses] section of the manifest when publishing. If they are already published, ensure they specify the address in the `published-at` of their Move.toml manifest.",
            },
        )
    "#]];
    expect.assert_debug_eq(&result);
    Ok(())
}

#[sim_test]
async fn test_package_publish_command_failure_invalid() -> Result<(), anyhow::Error> {
    let with_unpublished_dependencies = true; // Invalid packages should fail to publish, even if we allow unpublished dependencies.

    let mut test_cluster = TestClusterBuilder::new().build().await;
    let rgp = test_cluster.get_reference_gas_price().await;
    let address = test_cluster.get_address_0();
    let context = &mut test_cluster.wallet;

    let client = context.get_client().await?;
    let object_refs = client
        .read_api()
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_previous_transaction(),
            )),
            None,
            None,
        )
        .await?
        .data;

    let gas_obj_id = object_refs.first().unwrap().object().unwrap().object_id;

    let mut package_path = PathBuf::from(TEST_DATA_DIR);
    package_path.push("module_publish_failure_invalid");
    let build_config = BuildConfig::new_for_testing().config;
    let result = SuiClientCommands::Publish {
        package_path,
        build_config,
        gas: Some(gas_obj_id),
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_PUBLISH,
        skip_dependency_verification: false,
        with_unpublished_dependencies,
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await;

    let expect = expect![[r#"
        Err(
            ModulePublishFailure {
                error: "Package dependency \"Invalid\" does not specify a valid published address: could not parse value \"mystery\" for published-at field.",
            },
        )
    "#]];
    expect.assert_debug_eq(&result);
    Ok(())
}

#[sim_test]
async fn test_package_publish_nonexistent_dependency() -> Result<(), anyhow::Error> {
    let mut test_cluster = TestClusterBuilder::new().build().await;
    let rgp = test_cluster.get_reference_gas_price().await;
    let address = test_cluster.get_address_0();
    let context = &mut test_cluster.wallet;
    let client = context.get_client().await?;
    let object_refs = client
        .read_api()
        .get_owned_objects(address, None, None, None)
        .await?
        .data;

    let gas_obj_id = object_refs.first().unwrap().object().unwrap().object_id;

    let mut package_path = PathBuf::from(TEST_DATA_DIR);
    package_path.push("module_publish_with_nonexistent_dependency");
    let build_config = BuildConfig::new_for_testing().config;
    let result = SuiClientCommands::Publish {
        package_path,
        build_config,
        gas: Some(gas_obj_id),
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_PUBLISH,
        skip_dependency_verification: false,
        with_unpublished_dependencies: false,
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await;

    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("Dependency object does not exist or was deleted"),
        "{}",
        err
    );
    Ok(())
}

#[sim_test]
async fn test_package_publish_test_flag() -> Result<(), anyhow::Error> {
    let mut test_cluster = TestClusterBuilder::new().build().await;
    let rgp = test_cluster.get_reference_gas_price().await;
    let address = test_cluster.get_address_0();
    let context = &mut test_cluster.wallet;
    let client = context.get_client().await?;
    let object_refs = client
        .read_api()
        .get_owned_objects(address, None, None, None)
        .await?
        .data;

    let gas_obj_id = object_refs.first().unwrap().object().unwrap().object_id;

    let mut package_path = PathBuf::from(TEST_DATA_DIR);
    package_path.push("module_publish_with_nonexistent_dependency");
    let mut build_config: MoveBuildConfig = BuildConfig::new_for_testing().config;
    // this would have been the result of calling `sui client publish --test`
    build_config.test_mode = true;

    let result = SuiClientCommands::Publish {
        package_path,
        build_config,
        gas: Some(gas_obj_id),
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_PUBLISH,
        skip_dependency_verification: false,
        with_unpublished_dependencies: false,
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await;

    let expect = expect![[r#"
        Err(
            ModulePublishFailure {
                error: "The `publish` subcommand should not be used with the `--test` flag\n\nCode in published packages must not depend on test code.\nIn order to fix this and publish the package without `--test`, remove any non-test dependencies on test-only code.\nYou can ensure all test-only dependencies have been removed by compiling the package normally with `sui move build`.",
            },
        )
    "#]];
    expect.assert_debug_eq(&result);
    Ok(())
}

#[sim_test]
async fn test_package_upgrade_command() -> Result<(), anyhow::Error> {
    move_package::package_hooks::register_package_hooks(Box::new(SuiPackageHooks));
    let mut test_cluster = TestClusterBuilder::new().build().await;
    let rgp = test_cluster.get_reference_gas_price().await;
    let address = test_cluster.get_address_0();
    let context = &mut test_cluster.wallet;
    let client = context.get_client().await?;
    let object_refs = client
        .read_api()
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_previous_transaction(),
            )),
            None,
            None,
        )
        .await?
        .data;

    // Check log output contains all object ids.
    let gas_obj_id = object_refs.first().unwrap().object().unwrap().object_id;

    // Provide path to well formed package sources
    let mut package_path = PathBuf::from(TEST_DATA_DIR);
    package_path.push("dummy_modules_upgrade");
    let build_config = BuildConfig::new_for_testing().config;
    let resp = SuiClientCommands::Publish {
        package_path: package_path.clone(),
        build_config,
        gas: Some(gas_obj_id),
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_PUBLISH,
        skip_dependency_verification: false,
        with_unpublished_dependencies: false,
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await?;

    // Print it out to CLI/logs
    resp.print(true);

    let SuiClientCommandResult::Publish(response) = resp else {
        unreachable!("Invalid response");
    };

    let SuiTransactionBlockEffects::V1(effects) = response.effects.unwrap();

    assert!(effects.status.is_ok());
    let package = effects
        .created()
        .iter()
        .find(|refe| matches!(refe.owner, Owner::Immutable))
        .unwrap();

    let cap = effects
        .created()
        .iter()
        .find(|refe| matches!(refe.owner, Owner::AddressOwner(_)))
        .unwrap();

    // Hacky for now: we need to add the correct `published-at` field to the Move toml file.
    // In the future once we have automated address management replace this logic!
    let tmp_dir = tempfile::tempdir().unwrap();
    fs_extra::dir::copy(
        &package_path,
        tmp_dir.path(),
        &fs_extra::dir::CopyOptions::default(),
    )
    .unwrap();
    let mut upgrade_pkg_path = tmp_dir.path().to_path_buf();
    upgrade_pkg_path.extend(["dummy_modules_upgrade", "Move.toml"]);
    let mut move_toml = std::fs::File::options()
        .read(true)
        .write(true)
        .open(&upgrade_pkg_path)
        .unwrap();
    upgrade_pkg_path.pop();

    let mut buf = String::new();
    move_toml.read_to_string(&mut buf).unwrap();

    // Add a `published-at = "0x<package_object_id>"` to the Move manifest.
    let mut lines: Vec<String> = buf.split('\n').map(|x| x.to_string()).collect();
    let idx = lines.iter().position(|s| s == "[package]").unwrap();
    lines.insert(
        idx + 1,
        format!(
            "published-at = \"{}\"",
            package.reference.object_id.to_hex_uncompressed()
        ),
    );
    let new = lines.join("\n");
    move_toml.write_at(new.as_bytes(), 0).unwrap();

    // Now run the upgrade
    let build_config = BuildConfig::new_for_testing().config;
    let resp = SuiClientCommands::Upgrade {
        package_path: upgrade_pkg_path,
        upgrade_capability: cap.reference.object_id,
        build_config,
        gas: Some(gas_obj_id),
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_PUBLISH,
        skip_dependency_verification: false,
        with_unpublished_dependencies: false,
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await?;

    resp.print(true);

    let SuiClientCommandResult::Upgrade(response) = resp else {
        unreachable!("Invalid upgrade response");
    };
    let SuiTransactionBlockEffects::V1(effects) = response.effects.unwrap();

    assert!(effects.status.is_ok());

    let obj_ids = effects
        .created()
        .iter()
        .map(|refe| refe.reference.object_id)
        .collect::<Vec<_>>();

    // Check the objects
    for obj_id in obj_ids {
        get_parsed_object_assert_existence(obj_id, context).await;
    }

    Ok(())
}

#[sim_test]
async fn test_native_transfer() -> Result<(), anyhow::Error> {
    let mut test_cluster = TestClusterBuilder::new().build().await;
    let rgp = test_cluster.get_reference_gas_price().await;
    let address = test_cluster.get_address_0();
    let context = &mut test_cluster.wallet;
    let recipient = SuiAddress::random_for_testing_only();
    let client = context.get_client().await?;
    let object_refs = client
        .read_api()
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_previous_transaction(),
            )),
            None,
            None,
        )
        .await?
        .data;

    // Check log output contains all object ids.
    let gas_obj_id = object_refs.first().unwrap().object().unwrap().object_id;
    let obj_id = object_refs.get(1).unwrap().object().unwrap().object_id;

    let resp = SuiClientCommands::Transfer {
        gas: Some(gas_obj_id),
        to: KeyIdentity::Address(recipient),
        object_id: obj_id,
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_TRANSFER,
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await?;

    // Print it out to CLI/logs
    resp.print(true);

    // Get the mutated objects
    let (mut_obj1, mut_obj2) = if let SuiClientCommandResult::Transfer(response) = resp {
        assert!(
            response.status_ok().unwrap(),
            "Command failed: {:?}",
            response
        );
        (
            response
                .effects
                .as_ref()
                .unwrap()
                .mutated()
                .get(0)
                .unwrap()
                .reference
                .object_id,
            response
                .effects
                .as_ref()
                .unwrap()
                .mutated()
                .get(1)
                .unwrap()
                .reference
                .object_id,
        )
    } else {
        panic!()
    };

    // Check the objects
    let resp = SuiClientCommands::Object {
        id: mut_obj1,
        bcs: false,
    }
    .execute(context)
    .await?;
    let mut_obj1 = if let SuiClientCommandResult::Object(resp) = resp {
        if let Some(obj) = resp.data {
            obj
        } else {
            panic!()
        }
    } else {
        panic!();
    };

    let resp2 = SuiClientCommands::Object {
        id: mut_obj2,
        bcs: false,
    }
    .execute(context)
    .await?;
    let mut_obj2 = if let SuiClientCommandResult::Object(resp2) = resp2 {
        if let Some(obj) = resp2.data {
            obj
        } else {
            panic!()
        }
    } else {
        panic!();
    };

    let (gas, obj) = if mut_obj1.owner.unwrap().get_owner_address().unwrap() == address {
        (mut_obj1, mut_obj2)
    } else {
        (mut_obj2, mut_obj1)
    };

    assert_eq!(gas.owner.unwrap().get_owner_address().unwrap(), address);
    assert_eq!(obj.owner.unwrap().get_owner_address().unwrap(), recipient);

    let object_refs = client
        .read_api()
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_previous_transaction(),
            )),
            None,
            None,
        )
        .await?;

    // Check log output contains all object ids.
    let obj_id = object_refs.data.get(1).unwrap().object().unwrap().object_id;

    let resp = SuiClientCommands::Transfer {
        gas: None,
        to: KeyIdentity::Address(recipient),
        object_id: obj_id,
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_TRANSFER,
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await?;

    // Print it out to CLI/logs
    resp.print(true);

    // Get the mutated objects
    let (_mut_obj1, _mut_obj2) = if let SuiClientCommandResult::Transfer(response) = resp {
        (
            response
                .effects
                .as_ref()
                .unwrap()
                .mutated()
                .get(0)
                .unwrap()
                .reference
                .object_id,
            response
                .effects
                .as_ref()
                .unwrap()
                .mutated()
                .get(1)
                .unwrap()
                .reference
                .object_id,
        )
    } else {
        panic!()
    };

    Ok(())
}

#[test]
// Test for issue https://github.com/MystenLabs/sui/issues/1078
fn test_bug_1078() {
    let read = SuiClientCommandResult::Object(SuiObjectResponse::new_with_error(
        SuiObjectResponseError::NotExists {
            object_id: ObjectID::random(),
        },
    ));
    let mut writer = String::new();
    // fmt ObjectRead should not fail.
    write!(writer, "{}", read).unwrap();
    write!(writer, "{:?}", read).unwrap();
}

#[sim_test]
async fn test_switch_command() -> Result<(), anyhow::Error> {
    let mut cluster = TestClusterBuilder::new().build().await;
    let addr2 = cluster.get_address_1();
    let context = cluster.wallet_mut();

    // Get the active address
    let addr1 = context.active_address()?;

    // Run a command with address omitted
    let os = SuiClientCommands::Objects { address: None }
        .execute(context)
        .await?;

    let mut cmd_objs = if let SuiClientCommandResult::Objects(v) = os {
        v
    } else {
        panic!("Command failed")
    };

    // Check that we indeed fetched for addr1
    let client = context.get_client().await?;
    let mut actual_objs = client
        .read_api()
        .get_owned_objects(
            addr1,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::full_content(),
            )),
            None,
            None,
        )
        .await
        .unwrap()
        .data;
    cmd_objs.sort();
    actual_objs.sort();
    assert_eq!(cmd_objs, actual_objs);

    // Switch the address
    let resp = SuiClientCommands::Switch {
        address: Some(KeyIdentity::Address(addr2)),
        env: None,
    }
    .execute(context)
    .await?;
    assert_eq!(addr2, context.active_address()?);
    assert_ne!(addr1, context.active_address()?);
    assert_eq!(
        format!("{resp}"),
        format!(
            "{}",
            SuiClientCommandResult::Switch(SwitchResponse {
                address: Some(addr2.to_string()),
                env: None
            })
        )
    );

    // Wipe all the address info
    context.config.active_address = None;

    // Create a new address
    let os = SuiClientCommands::NewAddress {
        key_scheme: SignatureScheme::ED25519,
        alias: None,
        derivation_path: None,
        word_length: None,
    }
    .execute(context)
    .await?;
    let new_addr = if let SuiClientCommandResult::NewAddress(x) = os {
        x.address
    } else {
        panic!("Command failed")
    };

    // Check that we can switch to this address
    // Switch the address
    let resp = SuiClientCommands::Switch {
        address: Some(KeyIdentity::Address(new_addr)),
        env: None,
    }
    .execute(context)
    .await?;
    assert_eq!(new_addr, context.active_address()?);
    assert_eq!(
        format!("{resp}"),
        format!(
            "{}",
            SuiClientCommandResult::Switch(SwitchResponse {
                address: Some(new_addr.to_string()),
                env: None
            })
        )
    );
    Ok(())
}

#[sim_test]
async fn test_new_address_command_by_flag() -> Result<(), anyhow::Error> {
    let mut cluster = TestClusterBuilder::new().build().await;
    let context = cluster.wallet_mut();

    // keypairs loaded from config are Ed25519
    assert_eq!(
        context
            .config
            .keystore
            .keys()
            .iter()
            .filter(|k| k.flag() == Ed25519SuiSignature::SCHEME.flag())
            .count(),
        5
    );

    SuiClientCommands::NewAddress {
        key_scheme: SignatureScheme::Secp256k1,
        alias: None,
        derivation_path: None,
        word_length: None,
    }
    .execute(context)
    .await?;

    // new keypair generated is Secp256k1
    assert_eq!(
        context
            .config
            .keystore
            .keys()
            .iter()
            .filter(|k| k.flag() == Secp256k1SuiSignature::SCHEME.flag())
            .count(),
        1
    );

    Ok(())
}

#[sim_test]
async fn test_active_address_command() -> Result<(), anyhow::Error> {
    let mut cluster = TestClusterBuilder::new().build().await;
    let context = cluster.wallet_mut();

    // Get the active address
    let addr1 = context.active_address()?;

    // Run a command with address omitted
    let os = SuiClientCommands::ActiveAddress {}.execute(context).await?;

    let a = if let SuiClientCommandResult::ActiveAddress(Some(v)) = os {
        v
    } else {
        panic!("Command failed")
    };
    assert_eq!(a, addr1);

    let addr2 = context.config.keystore.addresses().get(1).cloned().unwrap();
    let resp = SuiClientCommands::Switch {
        address: Some(KeyIdentity::Address(addr2)),
        env: None,
    }
    .execute(context)
    .await?;
    assert_eq!(
        format!("{resp}"),
        format!(
            "{}",
            SuiClientCommandResult::Switch(SwitchResponse {
                address: Some(addr2.to_string()),
                env: None
            })
        )
    );

    // switch back to addr1 by using its alias
    let alias1 = context
        .config
        .keystore
        .get_alias_by_address(&addr1)
        .unwrap();
    let resp = SuiClientCommands::Switch {
        address: Some(KeyIdentity::Alias(alias1)),
        env: None,
    }
    .execute(context)
    .await?;
    assert_eq!(
        format!("{resp}"),
        format!(
            "{}",
            SuiClientCommandResult::Switch(SwitchResponse {
                address: Some(addr1.to_string()),
                env: None
            })
        )
    );

    Ok(())
}

fn get_gas_value(o: &SuiObjectData) -> u64 {
    GasCoin::try_from(o).unwrap().value()
}

async fn get_object(id: ObjectID, context: &WalletContext) -> Option<SuiObjectData> {
    let client = context.get_client().await.unwrap();
    let response = client
        .read_api()
        .get_object_with_options(id, SuiObjectDataOptions::full_content())
        .await
        .unwrap();
    response.data
}

async fn get_parsed_object_assert_existence(
    object_id: ObjectID,
    context: &WalletContext,
) -> SuiObjectData {
    get_object(object_id, context)
        .await
        .expect("Object {object_id} does not exist.")
}

#[sim_test]
async fn test_merge_coin() -> Result<(), anyhow::Error> {
    let mut test_cluster = TestClusterBuilder::new().build().await;
    let rgp = test_cluster.get_reference_gas_price().await;
    let address = test_cluster.get_address_0();
    let context = &mut test_cluster.wallet;

    let client = context.get_client().await?;
    let object_refs = client
        .read_api()
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_previous_transaction(),
            )),
            None,
            None,
        )
        .await?
        .data;

    // Check log output contains all object ids.
    let gas = object_refs.first().unwrap().object().unwrap().object_id;
    let primary_coin = object_refs.get(1).unwrap().object().unwrap().object_id;
    let coin_to_merge = object_refs.get(2).unwrap().object().unwrap().object_id;

    let total_value = get_gas_value(&get_object(primary_coin, context).await.unwrap())
        + get_gas_value(&get_object(coin_to_merge, context).await.unwrap());

    // Test with gas specified
    let resp = SuiClientCommands::MergeCoin {
        primary_coin,
        coin_to_merge,
        gas: Some(gas),
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_GENERIC,
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await?;
    let g = if let SuiClientCommandResult::MergeCoin(r) = resp {
        assert!(r.status_ok().unwrap(), "Command failed: {:?}", r);
        let object_id = r
            .effects
            .as_ref()
            .unwrap()
            .mutated_excluding_gas()
            .into_iter()
            .next()
            .unwrap()
            .reference
            .object_id;
        get_parsed_object_assert_existence(object_id, context).await
    } else {
        panic!("Command failed")
    };

    // Check total value is expected
    assert_eq!(get_gas_value(&g), total_value);

    // Check that old coin is deleted
    assert_eq!(get_object(coin_to_merge, context).await, None);

    let object_refs = client
        .read_api()
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_previous_transaction(),
            )),
            None,
            None,
        )
        .await?;

    let primary_coin = object_refs.data.get(1).unwrap().object()?.object_id;
    let coin_to_merge = object_refs.data.get(2).unwrap().object()?.object_id;

    let total_value = get_gas_value(&get_object(primary_coin, context).await.unwrap())
        + get_gas_value(&get_object(coin_to_merge, context).await.unwrap());

    // Test with no gas specified
    let resp = SuiClientCommands::MergeCoin {
        primary_coin,
        coin_to_merge,
        gas: None,
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_GENERIC,
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await?;

    let g = if let SuiClientCommandResult::MergeCoin(r) = resp {
        let object_id = r
            .effects
            .as_ref()
            .unwrap()
            .mutated_excluding_gas()
            .into_iter()
            .next()
            .unwrap()
            .reference
            .object_id;
        get_parsed_object_assert_existence(object_id, context).await
    } else {
        panic!("Command failed")
    };

    // Check total value is expected
    assert_eq!(get_gas_value(&g), total_value);

    // Check that old coin is deleted
    assert_eq!(get_object(coin_to_merge, context).await, None);

    Ok(())
}

#[sim_test]
async fn test_split_coin() -> Result<(), anyhow::Error> {
    let mut test_cluster = TestClusterBuilder::new().build().await;
    let rgp = test_cluster.get_reference_gas_price().await;
    let address = test_cluster.get_address_0();
    let context = &mut test_cluster.wallet;
    let client = context.get_client().await?;
    let object_refs = client
        .read_api()
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_previous_transaction(),
            )),
            None,
            None,
        )
        .await?;

    // Check log output contains all object ids.
    let gas = object_refs.data.first().unwrap().object()?.object_id;
    let mut coin = object_refs.data.get(1).unwrap().object()?.object_id;

    let orig_value = get_gas_value(&get_object(coin, context).await.unwrap());

    // Test with gas specified
    let resp = SuiClientCommands::SplitCoin {
        gas: Some(gas),
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_SPLIT_COIN,
        coin_id: coin,
        amounts: Some(vec![1000, 10]),
        count: None,
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await?;

    let (updated_coin, new_coins) = if let SuiClientCommandResult::SplitCoin(r) = resp {
        assert!(r.status_ok().unwrap(), "Command failed: {:?}", r);
        let updated_object_id = r
            .effects
            .as_ref()
            .unwrap()
            .mutated_excluding_gas()
            .into_iter()
            .next()
            .unwrap()
            .reference
            .object_id;
        let updated_obj = get_parsed_object_assert_existence(updated_object_id, context).await;
        let new_object_refs = r.effects.unwrap().created().to_vec();
        let mut new_objects = Vec::with_capacity(new_object_refs.len());
        for obj_ref in new_object_refs {
            new_objects.push(
                get_parsed_object_assert_existence(obj_ref.reference.object_id, context).await,
            );
        }
        (updated_obj, new_objects)
    } else {
        panic!("Command failed")
    };

    // Check values expected
    assert_eq!(get_gas_value(&updated_coin) + 1000 + 10, orig_value);
    assert!((get_gas_value(&new_coins[0]) == 1000) || (get_gas_value(&new_coins[0]) == 10));
    assert!((get_gas_value(&new_coins[1]) == 1000) || (get_gas_value(&new_coins[1]) == 10));
    let client = context.get_client().await?;
    let object_refs = client
        .read_api()
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_previous_transaction(),
            )),
            None,
            None,
        )
        .await?
        .data;

    // Get another coin
    for c in object_refs {
        let coin_data = c.into_object().unwrap();
        if get_gas_value(&get_object(coin_data.object_id, context).await.unwrap()) > 2000 {
            coin = coin_data.object_id;
        }
    }
    let orig_value = get_gas_value(&get_object(coin, context).await.unwrap());

    // Test split coin into equal parts
    let resp = SuiClientCommands::SplitCoin {
        gas: None,
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_SPLIT_COIN,
        coin_id: coin,
        amounts: None,
        count: Some(3),
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await?;

    let (updated_coin, new_coins) = if let SuiClientCommandResult::SplitCoin(r) = resp {
        assert!(r.status_ok().unwrap(), "Command failed: {:?}", r);
        let updated_object_id = r
            .effects
            .as_ref()
            .unwrap()
            .mutated_excluding_gas()
            .into_iter()
            .next()
            .unwrap()
            .reference
            .object_id;
        let updated_obj = get_parsed_object_assert_existence(updated_object_id, context).await;
        let new_object_refs = r.effects.unwrap().created().to_vec();
        let mut new_objects = Vec::with_capacity(new_object_refs.len());
        for obj_ref in new_object_refs {
            new_objects.push(
                get_parsed_object_assert_existence(obj_ref.reference.object_id, context).await,
            );
        }
        (updated_obj, new_objects)
    } else {
        panic!("Command failed")
    };

    // Check values expected
    assert_eq!(
        get_gas_value(&updated_coin),
        orig_value / 3 + orig_value % 3
    );
    assert_eq!(get_gas_value(&new_coins[0]), orig_value / 3);
    assert_eq!(get_gas_value(&new_coins[1]), orig_value / 3);

    let object_refs = client
        .read_api()
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_previous_transaction(),
            )),
            None,
            None,
        )
        .await?
        .data;

    // Get another coin
    for c in object_refs {
        let coin_data = c.into_object().unwrap();
        if get_gas_value(&get_object(coin_data.object_id, context).await.unwrap()) > 2000 {
            coin = coin_data.object_id;
        }
    }
    let orig_value = get_gas_value(&get_object(coin, context).await.unwrap());

    // Test with no gas specified
    let resp = SuiClientCommands::SplitCoin {
        gas: None,
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_SPLIT_COIN,
        coin_id: coin,
        amounts: Some(vec![1000, 10]),
        count: None,
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await?;

    let (updated_coin, new_coins) = if let SuiClientCommandResult::SplitCoin(r) = resp {
        assert!(r.status_ok().unwrap(), "Command failed: {:?}", r);
        let updated_object_id = r
            .effects
            .as_ref()
            .unwrap()
            .mutated_excluding_gas()
            .into_iter()
            .next()
            .unwrap()
            .reference
            .object_id;
        let updated_obj = get_parsed_object_assert_existence(updated_object_id, context).await;
        let new_object_refs = r.effects.unwrap().created().to_vec();
        let mut new_objects = Vec::with_capacity(new_object_refs.len());
        for obj_ref in new_object_refs {
            new_objects.push(
                get_parsed_object_assert_existence(obj_ref.reference.object_id, context).await,
            );
        }
        (updated_obj, new_objects)
    } else {
        panic!("Command failed")
    };

    // Check values expected
    assert_eq!(get_gas_value(&updated_coin) + 1000 + 10, orig_value);
    assert!((get_gas_value(&new_coins[0]) == 1000) || (get_gas_value(&new_coins[0]) == 10));
    assert!((get_gas_value(&new_coins[1]) == 1000) || (get_gas_value(&new_coins[1]) == 10));
    Ok(())
}

#[sim_test]
async fn test_signature_flag() -> Result<(), anyhow::Error> {
    let res = SignatureScheme::from_flag("0");
    assert!(res.is_ok());
    assert_eq!(res.unwrap().flag(), SignatureScheme::ED25519.flag());

    let res = SignatureScheme::from_flag("1");
    assert!(res.is_ok());
    assert_eq!(res.unwrap().flag(), SignatureScheme::Secp256k1.flag());

    let res = SignatureScheme::from_flag("2");
    assert!(res.is_ok());
    assert_eq!(res.unwrap().flag(), SignatureScheme::Secp256r1.flag());

    let res = SignatureScheme::from_flag("something");
    assert!(res.is_err());
    Ok(())
}

#[sim_test]
async fn test_execute_signed_tx() -> Result<(), anyhow::Error> {
    let mut test_cluster = TestClusterBuilder::new().build().await;
    let context = &mut test_cluster.wallet;
    let mut txns = batch_make_transfer_transactions(context, 1).await;
    let txn = txns.swap_remove(0);

    let (tx_data, signatures) = txn.to_tx_bytes_and_signatures();
    SuiClientCommands::ExecuteSignedTx {
        tx_bytes: tx_data.encoded(),
        signatures: signatures.into_iter().map(|s| s.encoded()).collect(),
    }
    .execute(context)
    .await?;
    Ok(())
}

#[sim_test]
async fn test_serialize_tx() -> Result<(), anyhow::Error> {
    let mut test_cluster = TestClusterBuilder::new().build().await;
    let rgp = test_cluster.get_reference_gas_price().await;
    let address = test_cluster.get_address_0();
    let address1 = test_cluster.get_address_1();
    let context = &mut test_cluster.wallet;
    let alias1 = context
        .config
        .keystore
        .get_alias_by_address(&address1)
        .unwrap();
    let client = context.get_client().await?;
    let object_refs = client
        .read_api()
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_previous_transaction(),
            )),
            None,
            None,
        )
        .await?
        .data;
    let coin = object_refs.get(1).unwrap().object().unwrap().object_id;

    SuiClientCommands::TransferSui {
        to: KeyIdentity::Address(address1),
        sui_coin_object_id: coin,
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_TRANSFER,
        amount: Some(1),
        serialize_unsigned_transaction: true,
        serialize_signed_transaction: false,
    }
    .execute(context)
    .await?;

    SuiClientCommands::TransferSui {
        to: KeyIdentity::Address(address1),
        sui_coin_object_id: coin,
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_TRANSFER,
        amount: Some(1),
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: true,
    }
    .execute(context)
    .await?;

    // use alias for transfer
    SuiClientCommands::TransferSui {
        to: KeyIdentity::Alias(alias1),
        sui_coin_object_id: coin,
        gas_budget: rgp * TEST_ONLY_GAS_UNIT_FOR_TRANSFER,
        amount: Some(1),
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: true,
    }
    .execute(context)
    .await?;
    Ok(())
}

#[tokio::test]
async fn test_stake_with_none_amount() -> Result<(), anyhow::Error> {
    let mut test_cluster = TestClusterBuilder::new().build().await;
    let address = test_cluster.get_address_0();
    let context = &mut test_cluster.wallet;

    let client = context.get_client().await?;
    let coins = client
        .coin_read_api()
        .get_coins(address, None, None, None)
        .await?
        .data;

    let config_path = test_cluster.swarm.dir().join(SUI_CLIENT_CONFIG);
    let validator_addr = client
        .governance_api()
        .get_latest_sui_system_state()
        .await?
        .active_validators[0]
        .sui_address;

    test_with_sui_binary(&[
        "client",
        "--client.config",
        config_path.to_str().unwrap(),
        "call",
        "--package",
        "0x3",
        "--module",
        "dwallet_system",
        "--function",
        "request_add_stake_mul_coin",
        "--args",
        "0x5",
        &format!("[{}]", coins.first().unwrap().coin_object_id),
        "[]",
        &validator_addr.to_string(),
        "--gas-budget",
        "1000000000",
    ])
    .await?;

    let stake = client.governance_api().get_stakes(address).await?;

    assert_eq!(1, stake.len());
    assert_eq!(
        coins.first().unwrap().balance,
        stake.first().unwrap().stakes.first().unwrap().principal
    );
    Ok(())
}

#[tokio::test]
async fn test_stake_with_u64_amount() -> Result<(), anyhow::Error> {
    let mut test_cluster = TestClusterBuilder::new().build().await;
    let address = test_cluster.get_address_0();
    let context = &mut test_cluster.wallet;

    let client = context.get_client().await?;
    let coins = client
        .coin_read_api()
        .get_coins(address, None, None, None)
        .await?
        .data;

    let config_path = test_cluster.swarm.dir().join(SUI_CLIENT_CONFIG);
    let validator_addr = client
        .governance_api()
        .get_latest_sui_system_state()
        .await?
        .active_validators[0]
        .sui_address;

    test_with_sui_binary(&[
        "client",
        "--client.config",
        config_path.to_str().unwrap(),
        "call",
        "--package",
        "0x3",
        "--module",
        "dwallet_system",
        "--function",
        "request_add_stake_mul_coin",
        "--args",
        "0x5",
        &format!("[{}]", coins.first().unwrap().coin_object_id),
        "[1000000000]",
        &validator_addr.to_string(),
        "--gas-budget",
        "1000000000",
    ])
    .await?;

    let stake = client.governance_api().get_stakes(address).await?;

    assert_eq!(1, stake.len());
    assert_eq!(
        1000000000,
        stake.first().unwrap().stakes.first().unwrap().principal
    );
    Ok(())
}

async fn test_with_sui_binary(args: &[&str]) -> Result<(), anyhow::Error> {
    let mut cmd = assert_cmd::Command::cargo_bin("dwallet").unwrap();
    let args = args.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    // test cluster will not response if this call is in the same thread
    let out = thread::spawn(move || cmd.args(args).assert());
    while !out.is_finished() {
        sleep(Duration::from_millis(100)).await;
    }
    out.join().unwrap().success();
    Ok(())
}

#[sim_test]
async fn test_get_owned_objects_owned_by_address_and_check_pagination() -> Result<(), anyhow::Error>
{
    let mut test_cluster = TestClusterBuilder::new().build().await;
    let address = test_cluster.get_address_0();
    let context = &mut test_cluster.wallet;

    let client = context.get_client().await?;
    let object_responses = client
        .read_api()
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new(
                Some(SuiObjectDataFilter::StructType(GasCoin::type_())),
                Some(
                    SuiObjectDataOptions::new()
                        .with_type()
                        .with_owner()
                        .with_previous_transaction(),
                ),
            )),
            None,
            None,
        )
        .await?;

    // assert that all the objects_returned are owned by the address
    for resp in &object_responses.data {
        let obj_owner = resp.object().unwrap().owner.unwrap();
        assert_eq!(
            obj_owner.get_owner_address().unwrap().to_string(),
            address.to_string()
        )
    }
    // assert that has next page is false
    assert!(!object_responses.has_next_page);

    // Pagination check
    let mut has_next = true;
    let mut cursor = None;
    let mut response_data: Vec<SuiObjectResponse> = Vec::new();
    while has_next {
        let object_responses = client
            .read_api()
            .get_owned_objects(
                address,
                Some(SuiObjectResponseQuery::new(
                    Some(SuiObjectDataFilter::StructType(GasCoin::type_())),
                    Some(
                        SuiObjectDataOptions::new()
                            .with_type()
                            .with_owner()
                            .with_previous_transaction(),
                    ),
                )),
                cursor,
                Some(1),
            )
            .await?;

        response_data.push(object_responses.data.first().unwrap().clone());

        if object_responses.has_next_page {
            cursor = object_responses.next_cursor;
        } else {
            has_next = false;
        }
    }

    assert_eq!(&response_data, &object_responses.data);

    Ok(())
}

#[tokio::test]
async fn test_linter_suppression_stats() -> Result<(), anyhow::Error> {
    let mut cmd = assert_cmd::Command::cargo_bin("dwallet").unwrap();
    let args = vec!["move", "test", "--path", "tests/data/linter"];
    let output = cmd
        .args(&args)
        .output()
        .expect("failed to run 'sui move test'");
    let out_str = str::from_utf8(&output.stderr).unwrap();
    assert!(
        out_str.contains("Total number of linter warnings suppressed: 5 (filtered categories: 3)")
    );
    Ok(())
}

#[tokio::test]
async fn key_identity_test() {
    let mut test_cluster = TestClusterBuilder::new().build().await;
    let address = test_cluster.get_address_0();
    let context = &mut test_cluster.wallet;
    let alias = context
        .config
        .keystore
        .get_alias_by_address(&address)
        .unwrap();

    // by alias
    assert_eq!(
        address,
        get_identity_address(Some(KeyIdentity::Alias(alias)), context).unwrap()
    );
    // by address
    assert_eq!(
        address,
        get_identity_address(Some(KeyIdentity::Address(address)), context).unwrap()
    );
    // alias does not exist
    assert!(get_identity_address(Some(KeyIdentity::Alias("alias".to_string())), context).is_err());

    // get active address instead when no alias/address is given
    assert_eq!(
        context.active_address().unwrap(),
        get_identity_address(None, context).unwrap()
    );
}
