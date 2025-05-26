use crate::validator_initialization_config::ValidatorInitializationConfig;
use anyhow::bail;
use dwallet_classgroups_types::ClassGroupsEncryptionKeyAndProof;
use fastcrypto::traits::ToFromBytes;
use ika_config::initiation::{InitiationParameters, MIN_VALIDATOR_JOINING_STAKE_NIKA};
use ika_config::validator_info::ValidatorInfo;
use ika_config::Config;
use ika_move_packages::IkaMovePackage;
use ika_types::ika_coin::IKACoin;
use ika_types::messages_dwallet_mpc::IkaPackagesConfig;
use ika_types::sui::system_inner_v1::ValidatorCapV1;
use ika_types::sui::{
    ClassGroupsPublicKeyAndProof, ClassGroupsPublicKeyAndProofBuilder, System,
    ADD_PAIR_TO_CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_FUNCTION_NAME,
    CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_MODULE_NAME,
    CREATE_CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_BUILDER_FUNCTION_NAME,
    DWALLET_2PC_MPC_SECP256K1_MODULE_NAME, DWALLET_COORDINATOR_STRUCT_NAME,
    FINISH_CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_FUNCTION_NAME, INITIALIZE_FUNCTION_NAME,
    INIT_CAP_STRUCT_NAME, INIT_MODULE_NAME, NEW_VALIDATOR_METADATA_FUNCTION_NAME,
    PROTOCOL_CAP_MODULE_NAME, PROTOCOL_CAP_STRUCT_NAME, REQUEST_ADD_STAKE_FUNCTION_NAME,
    REQUEST_ADD_VALIDATOR_CANDIDATE_FUNCTION_NAME, REQUEST_ADD_VALIDATOR_FUNCTION_NAME,
    REQUEST_DWALLET_NETWORK_DECRYPTION_KEY_DKG_BY_CAP_FUNCTION_NAME, SYSTEM_MODULE_NAME,
    VALIDATOR_CAP_MODULE_NAME, VALIDATOR_CAP_STRUCT_NAME, VALIDATOR_METADATA_MODULE_NAME,
};
use move_core_types::ident_str;
use move_core_types::language_storage::{StructTag, TypeTag};
use shared_crypto::intent::Intent;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use sui::client_commands::{
    estimate_gas_budget_from_gas_cost, execute_dry_run, request_tokens_from_faucet,
    SuiClientCommandResult,
};
use sui_config::SUI_CLIENT_CONFIG;
use sui_keys::keystore::{AccountKeystore, InMemKeystore, Keystore};
use sui_sdk::rpc_types::{ObjectChange, SuiObjectDataOptions, SuiTransactionBlockResponse};
use sui_sdk::rpc_types::{
    SuiObjectDataFilter, SuiObjectResponseQuery, SuiTransactionBlockEffectsAPI,
};
use sui_sdk::sui_client_config::{SuiClientConfig, SuiEnv};
use sui_sdk::wallet_context::WalletContext;
use sui_sdk::SuiClient;
use sui_types::base_types::{ObjectID, ObjectRef, SequenceNumber, SuiAddress};
use sui_types::coin::TreasuryCap;
use sui_types::crypto::{SignatureScheme, SuiKeyPair};
use sui_types::move_package::UpgradeCap;
use sui_types::object::Owner;
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::{
    Argument, CallArg, Command, ObjectArg, SenderSignedData, Transaction, TransactionDataAPI,
    TransactionKind,
};
use sui_types::{
    MOVE_STDLIB_PACKAGE_ID, SUI_CLOCK_OBJECT_ID, SUI_CLOCK_OBJECT_SHARED_VERSION,
    SUI_FRAMEWORK_PACKAGE_ID,
};

const DKG_FIRST_ROUND_PROTOCOL_FLAG: u32 = 0;
const DKG_SECOND_ROUND_PROTOCOL_FLAG: u32 = 1;
const RE_ENCRYPT_USER_SHARE_PROTOCOL_FLAG: u32 = 2;
const MAKE_DWALLET_USER_SECRET_KEY_SHARE_PUBLIC_PROTOCOL_FLAG: u32 = 3;
const IMPORTED_KEY_DWALLET_VERIFICATION_PROTOCOL_FLAG: u32 = 4;
const PRESIGN_PROTOCOL_FLAG: u32 = 5;
const SIGN_PROTOCOL_FLAG: u32 = 6;
const FUTURE_SIGN_PROTOCOL_FLAG: u32 = 7;
const SIGN_WITH_PARTIAL_USER_SIGNATURE_PROTOCOL_FLAG: u32 = 8;

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
    let (publisher_address, phrase, scheme) = keystore.generate_and_add_new_key(
        SignatureScheme::ED25519,
        Some(alias.to_string()),
        None,
        None,
    )?;

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

    let mut request_tokens_from_faucet_futures = vec![
        request_tokens_from_faucet(publisher_address, sui_faucet_url.clone()),
        request_tokens_from_faucet(publisher_address, sui_faucet_url.clone()),
    ];
    let mut validator_addresses = Vec::new();
    for validator_initialization_config in validator_initialization_configs {
        let alias = validator_initialization_config.name.clone().unwrap();
        context.add_account(
            Some(alias),
            validator_initialization_config.account_key_pair.copy(),
        );

        let validator_address: SuiAddress =
            (&validator_initialization_config.account_key_pair.public()).into();

        request_tokens_from_faucet_futures.push(request_tokens_from_faucet(
            validator_address,
            sui_faucet_url.clone(),
        ));

        validator_addresses.push(validator_address);
    }

    futures::future::join_all(request_tokens_from_faucet_futures)
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        // Temporary workaround for the faucet returning 200 OK, but it seems as an error
        // since the new faucet api changed
        // TODO: Remove this workaround when we move to the new sui version and new faucet api
        .or_else(|e| {
            if e.to_string().contains("200 OK") {
                Ok(vec![])
            } else {
                Err(e)
            }
        })?;

    let ika_package = ika_move_packages::BuiltInIkaMovePackages::get_package_by_name("ika");
    let ika_system_package =
        ika_move_packages::BuiltInIkaMovePackages::get_package_by_name("ika_system");

    let (ika_package_id, treasury_cap_id, ika_package_upgrade_cap_id) =
        publish_ika_package_to_sui(publisher_address, &mut context, client.clone(), ika_package)
            .await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    merge_coins(publisher_address, &mut context).await?;
    println!("Merge coins done, address {:?}", publisher_address);

    println!("Package `ika` published: ika_package_id: {ika_package_id} treasury_cap_id: {treasury_cap_id}");

    let (ika_system_package_id, init_cap_id, ika_system_package_upgrade_cap_id) =
        publish_ika_system_package_to_sui(
            publisher_address,
            &mut context,
            client.clone(),
            ika_system_package,
            ika_package_id,
        )
        .await?;

    println!("Package `ika_system` published: ika_system_package_id: {ika_system_package_id} init_cap_id: {init_cap_id}");

    let ika_supply_id = minted_ika(publisher_address, client.clone(), ika_package_id).await?;

    println!("Minting done: ika_supply_id: {ika_supply_id}");

    let (ika_system_object_id, protocol_cap_id, init_system_shared_version) = init_initialize(
        publisher_address,
        &mut context,
        client.clone(),
        ika_system_package_id,
        init_cap_id,
        ika_package_upgrade_cap_id,
        ika_system_package_upgrade_cap_id,
        treasury_cap_id,
        initiation_parameters,
    )
    .await?;

    println!("Running `init::initialize` done: ika_system_object_id: {ika_system_object_id} protocol_cap_id: {protocol_cap_id}");
    let ika_config = IkaPackagesConfig {
        ika_package_id,
        ika_system_package_id,
        ika_system_object_id,
    };
    let mut file = File::create("ika_config.json")?;
    let json = serde_json::to_string_pretty(&ika_config)?;
    file.write_all(json.as_bytes())?;

    let mut validator_ids = Vec::new();
    let mut validator_cap_ids = Vec::new();
    for validator_initialization_config in validator_initialization_configs {
        let validator_address: SuiAddress =
            (&validator_initialization_config.account_key_pair.public()).into();

        let validator_initialization_metadata = validator_initialization_config.to_validator_info();
        let (validator_id, validator_cap_id) = request_add_validator_candidate(
            validator_address,
            &mut context,
            client.clone(),
            &validator_initialization_metadata,
            ika_system_package_id,
            ika_system_object_id,
            init_system_shared_version,
        )
        .await?;
        validator_ids.push(validator_id);
        validator_cap_ids.push(validator_cap_id);
        println!("Running `system::request_add_validator_candidate` done for validator {validator_address}");
    }

    stake_ika(
        publisher_address,
        &mut context,
        ika_system_package_id,
        ika_system_object_id,
        init_system_shared_version,
        ika_supply_id,
        validator_ids.clone(),
    )
    .await?;

    println!("Staking for all validators done.");

    for (validator_address, validator_cap_id) in validator_addresses.iter().zip(validator_cap_ids) {
        request_add_validator(
            *validator_address,
            &mut context,
            client.clone(),
            ika_system_package_id,
            ika_system_object_id,
            init_system_shared_version,
            validator_cap_id,
        )
        .await?;
        println!("Running `system::request_add_validator` done for validator {validator_address}");
    }

    let (dwallet_2pc_mpc_coordinator_id, dwallet_2pc_mpc_coordinator_initial_shared_version) =
        ika_system_initialize(
            publisher_address,
            &mut context,
            client.clone(),
            ika_system_package_id,
            ika_system_object_id,
            init_system_shared_version,
            protocol_cap_id,
        )
        .await?;
    println!("Running `system::initialize` done.");

    ika_system_request_dwallet_network_encryption_key_dkg_by_cap(
        publisher_address,
        &mut context,
        client.clone(),
        ika_system_package_id,
        ika_system_object_id,
        init_system_shared_version,
        dwallet_2pc_mpc_coordinator_id,
        dwallet_2pc_mpc_coordinator_initial_shared_version,
        protocol_cap_id,
    )
    .await?;

    println!("Running `system::request_dwallet_network_encryption_key_dkg_by_cap` done.");

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    Ok((
        ika_package_id,
        ika_system_package_id,
        ika_system_object_id,
        publisher_keypair,
    ))
}

pub async fn ika_system_request_dwallet_network_encryption_key_dkg_by_cap(
    publisher_address: SuiAddress,
    context: &mut WalletContext,
    client: SuiClient,
    ika_system_package_id: ObjectID,
    ika_system_object_id: ObjectID,
    init_system_shared_version: SequenceNumber,
    dwallet_2pc_mpc_coordinator_id: ObjectID,
    dwallet_2pc_mpc_coordinator_initial_shared_version: SequenceNumber,
    protocol_cap_id: ObjectID,
) -> Result<(), anyhow::Error> {
    let mut ptb = ProgrammableTransactionBuilder::new();

    let protocol_cap_ref = client
        .transaction_builder()
        .get_object_ref(protocol_cap_id)
        .await?;

    ptb.move_call(
        ika_system_package_id,
        SYSTEM_MODULE_NAME.into(),
        REQUEST_DWALLET_NETWORK_DECRYPTION_KEY_DKG_BY_CAP_FUNCTION_NAME.into(),
        vec![],
        vec![
            CallArg::Object(ObjectArg::SharedObject {
                id: ika_system_object_id,
                initial_shared_version: init_system_shared_version,
                mutable: true,
            }),
            CallArg::Object(ObjectArg::SharedObject {
                id: dwallet_2pc_mpc_coordinator_id,
                initial_shared_version: dwallet_2pc_mpc_coordinator_initial_shared_version,
                mutable: true,
            }),
            CallArg::Object(ObjectArg::ImmOrOwnedObject(protocol_cap_ref)),
        ],
    )?;

    let tx_kind = TransactionKind::ProgrammableTransaction(ptb.finish());

    let _ = execute_sui_transaction(publisher_address, tx_kind, context, vec![]).await?;

    Ok(())
}

pub async fn ika_system_initialize(
    publisher_address: SuiAddress,
    context: &mut WalletContext,
    client: SuiClient,
    ika_system_package_id: ObjectID,
    ika_system_object_id: ObjectID,
    init_system_shared_version: SequenceNumber,
    protocol_cap_id: ObjectID,
) -> Result<(ObjectID, SequenceNumber), anyhow::Error> {
    let mut ptb = ProgrammableTransactionBuilder::new();

    let protocol_cap_ref = client
        .transaction_builder()
        .get_object_ref(protocol_cap_id)
        .await?;

    let zero_key = ptb.input(CallArg::Pure(bcs::to_bytes(&vec![0u32])?))?;
    let zero_and_one_value = ptb.input(CallArg::Pure(bcs::to_bytes(&vec![vec![0u32, 1u32]])?))?;
    let zero = ptb.input(CallArg::Pure(bcs::to_bytes(&0u32)?))?;
    let zero_option = ptb.input(CallArg::Pure(bcs::to_bytes(&Some(0u32))?))?;
    let none_option = ptb.input(CallArg::Pure(bcs::to_bytes(&None::<u32>)?))?;

    let dkg_first_round_protocol_flag = ptb.input(CallArg::Pure(bcs::to_bytes(
        &DKG_FIRST_ROUND_PROTOCOL_FLAG,
    )?))?;
    let dkg_second_round_protocol_flag = ptb.input(CallArg::Pure(bcs::to_bytes(
        &DKG_SECOND_ROUND_PROTOCOL_FLAG,
    )?))?;
    let re_encrypt_user_share_protocol_flag = ptb.input(CallArg::Pure(bcs::to_bytes(
        &RE_ENCRYPT_USER_SHARE_PROTOCOL_FLAG,
    )?))?;
    let make_dwallet_user_secret_key_share_public_protocol_flag = ptb.input(CallArg::Pure(
        bcs::to_bytes(&MAKE_DWALLET_USER_SECRET_KEY_SHARE_PUBLIC_PROTOCOL_FLAG)?,
    ))?;
    let imported_key_dwallet_verification_protocol_flag = ptb.input(CallArg::Pure(
        bcs::to_bytes(&IMPORTED_KEY_DWALLET_VERIFICATION_PROTOCOL_FLAG)?,
    ))?;
    let presign_protocol_flag = ptb.input(CallArg::Pure(bcs::to_bytes(&PRESIGN_PROTOCOL_FLAG)?))?;
    let sign_protocol_flag = ptb.input(CallArg::Pure(bcs::to_bytes(&SIGN_PROTOCOL_FLAG)?))?;
    let future_sign_protocol_flag =
        ptb.input(CallArg::Pure(bcs::to_bytes(&FUTURE_SIGN_PROTOCOL_FLAG)?))?;
    let sign_with_partial_user_signature_protocol_flag = ptb.input(CallArg::Pure(
        bcs::to_bytes(&SIGN_WITH_PARTIAL_USER_SIGNATURE_PROTOCOL_FLAG)?,
    ))?;

    let zero_price = ptb.input(CallArg::Pure(bcs::to_bytes(&0u64)?))?;

    let ika_system_arg = ptb.input(CallArg::Object(ObjectArg::SharedObject {
        id: ika_system_object_id,
        initial_shared_version: init_system_shared_version,
        mutable: true,
    }))?;

    let dwallet_pricing = ptb.programmable_move_call(
        ika_system_package_id,
        ident_str!("dwallet_pricing").into(),
        ident_str!("empty").into(),
        vec![],
        vec![],
    );

    ptb.programmable_move_call(
        ika_system_package_id,
        ident_str!("dwallet_pricing").into(),
        ident_str!("insert_or_update_dwallet_pricing").into(),
        vec![],
        vec![
            dwallet_pricing,
            zero,
            none_option,
            dkg_first_round_protocol_flag,
            zero_price,
            zero_price,
            zero_price,
            zero_price,
        ],
    );

    ptb.programmable_move_call(
        ika_system_package_id,
        ident_str!("dwallet_pricing").into(),
        ident_str!("insert_or_update_dwallet_pricing").into(),
        vec![],
        vec![
            dwallet_pricing,
            zero,
            none_option,
            dkg_second_round_protocol_flag,
            zero_price,
            zero_price,
            zero_price,
            zero_price,
        ],
    );

    ptb.programmable_move_call(
        ika_system_package_id,
        ident_str!("dwallet_pricing").into(),
        ident_str!("insert_or_update_dwallet_pricing").into(),
        vec![],
        vec![
            dwallet_pricing,
            zero,
            none_option,
            re_encrypt_user_share_protocol_flag,
            zero_price,
            zero_price,
            zero_price,
            zero_price,
        ],
    );

    ptb.programmable_move_call(
        ika_system_package_id,
        ident_str!("dwallet_pricing").into(),
        ident_str!("insert_or_update_dwallet_pricing").into(),
        vec![],
        vec![
            dwallet_pricing,
            zero,
            none_option,
            make_dwallet_user_secret_key_share_public_protocol_flag,
            zero_price,
            zero_price,
            zero_price,
            zero_price,
        ],
    );

    ptb.programmable_move_call(
        ika_system_package_id,
        ident_str!("dwallet_pricing").into(),
        ident_str!("insert_or_update_dwallet_pricing").into(),
        vec![],
        vec![
            dwallet_pricing,
            zero,
            none_option,
            imported_key_dwallet_verification_protocol_flag,
            zero_price,
            zero_price,
            zero_price,
            zero_price,
        ],
    );

    ptb.programmable_move_call(
        ika_system_package_id,
        ident_str!("dwallet_pricing").into(),
        ident_str!("insert_or_update_dwallet_pricing").into(),
        vec![],
        vec![
            dwallet_pricing,
            zero,
            zero_option,
            presign_protocol_flag,
            zero_price,
            zero_price,
            zero_price,
            zero_price,
        ],
    );

    ptb.programmable_move_call(
        ika_system_package_id,
        ident_str!("dwallet_pricing").into(),
        ident_str!("insert_or_update_dwallet_pricing").into(),
        vec![],
        vec![
            dwallet_pricing,
            zero,
            zero_option,
            sign_protocol_flag,
            zero_price,
            zero_price,
            zero_price,
            zero_price,
        ],
    );

    ptb.programmable_move_call(
        ika_system_package_id,
        ident_str!("dwallet_pricing").into(),
        ident_str!("insert_or_update_dwallet_pricing").into(),
        vec![],
        vec![
            dwallet_pricing,
            zero,
            zero_option,
            future_sign_protocol_flag,
            zero_price,
            zero_price,
            zero_price,
            zero_price,
        ],
    );

    ptb.programmable_move_call(
        ika_system_package_id,
        ident_str!("dwallet_pricing").into(),
        ident_str!("insert_or_update_dwallet_pricing").into(),
        vec![],
        vec![
            dwallet_pricing,
            zero,
            zero_option,
            sign_with_partial_user_signature_protocol_flag,
            zero_price,
            zero_price,
            zero_price,
            zero_price,
        ],
    );

    let supported_signature_algorithms_to_hash_schemes = ptb.programmable_move_call(
        SUI_FRAMEWORK_PACKAGE_ID,
        ident_str!("vec_map").into(),
        ident_str!("from_keys_values").into(),
        vec![TypeTag::U32, TypeTag::Vector(Box::new(TypeTag::U32))],
        vec![zero_key, zero_and_one_value],
    );

    let supported_signature_algorithms_to_hash_schemes_vec = ptb.programmable_move_call(
        MOVE_STDLIB_PACKAGE_ID,
        ident_str!("vector").into(),
        ident_str!("singleton").into(),
        vec![TypeTag::Struct(Box::new(StructTag {
            address: SUI_FRAMEWORK_PACKAGE_ID.into(),
            module: ident_str!("vec_map").into(),
            name: ident_str!("VecMap").into(),
            type_params: vec![TypeTag::U32, TypeTag::Vector(Box::new(TypeTag::U32))],
        }))],
        vec![supported_signature_algorithms_to_hash_schemes],
    );

    let supported_curves_to_signature_algorithms_to_hash_schemes = ptb.programmable_move_call(
        SUI_FRAMEWORK_PACKAGE_ID,
        ident_str!("vec_map").into(),
        ident_str!("from_keys_values").into(),
        vec![
            TypeTag::U32,
            TypeTag::Struct(Box::new(StructTag {
                address: SUI_FRAMEWORK_PACKAGE_ID.into(),
                module: ident_str!("vec_map").into(),
                name: ident_str!("VecMap").into(),
                type_params: vec![TypeTag::U32, TypeTag::Vector(Box::new(TypeTag::U32))],
            })),
        ],
        vec![zero_key, supported_signature_algorithms_to_hash_schemes_vec],
    );

    let protocol_cap_arg = ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(
        protocol_cap_ref,
    )))?;

    let clock_arg = ptb.input(CallArg::Object(ObjectArg::SharedObject {
        id: SUI_CLOCK_OBJECT_ID,
        initial_shared_version: SUI_CLOCK_OBJECT_SHARED_VERSION,
        mutable: false,
    }))?;

    ptb.programmable_move_call(
        ika_system_package_id,
        SYSTEM_MODULE_NAME.into(),
        INITIALIZE_FUNCTION_NAME.into(),
        vec![],
        vec![
            ika_system_arg,
            dwallet_pricing,
            supported_curves_to_signature_algorithms_to_hash_schemes,
            protocol_cap_arg,
            clock_arg,
        ],
    );

    let tx_kind = TransactionKind::ProgrammableTransaction(ptb.finish());

    let response = execute_sui_transaction(publisher_address, tx_kind, context, vec![]).await?;

    let object_changes = response.object_changes.unwrap();

    let dwallet_2pc_mpc_coordinator_type = StructTag {
        address: ika_system_package_id.into(),
        module: DWALLET_2PC_MPC_SECP256K1_MODULE_NAME.into(),
        name: DWALLET_COORDINATOR_STRUCT_NAME.into(),
        type_params: vec![],
    };

    let dwallet_2pc_mpc_coordinator_id = *object_changes
        .iter()
        .filter_map(|o| match o {
            ObjectChange::Created {
                object_id,
                object_type,
                ..
            } if dwallet_2pc_mpc_coordinator_type == *object_type => Some(*object_id),
            _ => None,
        })
        .collect::<Vec<_>>()
        .first()
        .unwrap();

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let response = client
        .read_api()
        .get_object_with_options(
            dwallet_2pc_mpc_coordinator_id,
            SuiObjectDataOptions::new().with_owner(),
        )
        .await?;

    let Some(Owner::Shared {
        initial_shared_version,
    }) = response.data.unwrap().owner
    else {
        return Err(anyhow::Error::msg("Owner does not exist"));
    };

    Ok((dwallet_2pc_mpc_coordinator_id, initial_shared_version))
}

pub async fn init_initialize(
    publisher_address: SuiAddress,
    context: &mut WalletContext,
    client: SuiClient,
    ika_system_package_id: ObjectID,
    init_cap_id: ObjectID,
    ika_package_upgrade_cap_id: ObjectID,
    ika_system_package_upgrade_cap_id: ObjectID,
    treasury_cap_id: ObjectID,
    initiation_parameters: InitiationParameters,
) -> Result<(ObjectID, ObjectID, SequenceNumber), anyhow::Error> {
    let mut ptb = ProgrammableTransactionBuilder::new();

    let init_cap_ref = client
        .transaction_builder()
        .get_object_ref(init_cap_id)
        .await?;
    let ika_package_upgrade_cap_ref = client
        .transaction_builder()
        .get_object_ref(ika_package_upgrade_cap_id)
        .await?;
    let ika_system_package_upgrade_cap_ref = client
        .transaction_builder()
        .get_object_ref(ika_system_package_upgrade_cap_id)
        .await?;
    let treasury_cap_ref = client
        .transaction_builder()
        .get_object_ref(treasury_cap_id)
        .await?;

    ptb.move_call(
        ika_system_package_id,
        INIT_MODULE_NAME.into(),
        INITIALIZE_FUNCTION_NAME.into(),
        vec![],
        vec![
            CallArg::Object(ObjectArg::ImmOrOwnedObject(init_cap_ref)),
            CallArg::Object(ObjectArg::ImmOrOwnedObject(ika_package_upgrade_cap_ref)),
            CallArg::Object(ObjectArg::ImmOrOwnedObject(
                ika_system_package_upgrade_cap_ref,
            )),
            CallArg::Object(ObjectArg::ImmOrOwnedObject(treasury_cap_ref)),
            CallArg::Pure(bcs::to_bytes(&initiation_parameters.protocol_version)?),
            CallArg::Pure(bcs::to_bytes(
                &initiation_parameters.chain_start_timestamp_ms,
            )?),
            CallArg::Pure(bcs::to_bytes(&initiation_parameters.epoch_duration_ms)?),
            CallArg::Pure(bcs::to_bytes(
                &initiation_parameters.stake_subsidy_start_epoch,
            )?),
            CallArg::Pure(bcs::to_bytes(&initiation_parameters.stake_subsidy_rate)?),
            CallArg::Pure(bcs::to_bytes(
                &initiation_parameters.stake_subsidy_period_length,
            )?),
            CallArg::Pure(bcs::to_bytes(&initiation_parameters.min_validator_count)?),
            CallArg::Pure(bcs::to_bytes(&initiation_parameters.max_validator_count)?),
            CallArg::Pure(bcs::to_bytes(
                &initiation_parameters.min_validator_joining_stake,
            )?),
            CallArg::Pure(bcs::to_bytes(&initiation_parameters.reward_slashing_rate)?),
            CallArg::Pure(bcs::to_bytes(&initiation_parameters.lock_active_committee)?),
        ],
    )?;

    ptb.transfer_arg(publisher_address, Argument::Result(0));

    let tx_kind = TransactionKind::ProgrammableTransaction(ptb.finish());

    let response = execute_sui_transaction(publisher_address, tx_kind, context, vec![]).await?;

    let object_changes = response.object_changes.unwrap();

    let ika_system_object_id = *object_changes
        .iter()
        .filter_map(|o| match o {
            ObjectChange::Created {
                object_id,
                object_type,
                ..
            } if System::type_(ika_system_package_id.into()) == *object_type => Some(*object_id),
            _ => None,
        })
        .collect::<Vec<_>>()
        .first()
        .unwrap();

    let protocol_cap_type = StructTag {
        address: ika_system_package_id.into(),
        module: PROTOCOL_CAP_MODULE_NAME.into(),
        name: PROTOCOL_CAP_STRUCT_NAME.into(),
        type_params: vec![],
    };

    let protocol_cap_id = *object_changes
        .iter()
        .filter_map(|o| match o {
            ObjectChange::Created {
                object_id,
                object_type,
                ..
            } if protocol_cap_type == *object_type => Some(*object_id),
            _ => None,
        })
        .collect::<Vec<_>>()
        .first()
        .unwrap();

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let response = client
        .read_api()
        .get_object_with_options(
            ika_system_object_id,
            SuiObjectDataOptions::new().with_owner(),
        )
        .await?;

    let Some(Owner::Shared {
        initial_shared_version,
    }) = response.data.unwrap().owner
    else {
        return Err(anyhow::Error::msg("Owner does not exist"));
    };

    Ok((
        ika_system_object_id,
        protocol_cap_id,
        initial_shared_version,
    ))
}

async fn request_add_validator(
    validator_address: SuiAddress,
    context: &mut WalletContext,
    client: SuiClient,
    ika_system_package_id: ObjectID,
    ika_system_object_id: ObjectID,
    init_system_shared_version: SequenceNumber,
    validator_cap_id: ObjectID,
) -> Result<(), anyhow::Error> {
    let mut ptb = ProgrammableTransactionBuilder::new();

    let validator_cap_ref = client
        .transaction_builder()
        .get_object_ref(validator_cap_id)
        .await?;

    ptb.move_call(
        ika_system_package_id,
        SYSTEM_MODULE_NAME.into(),
        REQUEST_ADD_VALIDATOR_FUNCTION_NAME.into(),
        vec![],
        vec![
            CallArg::Object(ObjectArg::SharedObject {
                id: ika_system_object_id,
                initial_shared_version: init_system_shared_version,
                mutable: true,
            }),
            CallArg::Object(ObjectArg::ImmOrOwnedObject(validator_cap_ref)),
        ],
    )?;

    let tx_kind = TransactionKind::ProgrammableTransaction(ptb.finish());

    let _ = execute_sui_transaction(validator_address, tx_kind, context, vec![]).await?;

    Ok(())
}

async fn merge_coins(
    publisher_address: SuiAddress,
    context: &mut WalletContext,
) -> Result<(), anyhow::Error> {
    let coins = context
        .get_all_gas_objects_owned_by_address(publisher_address)
        .await?;
    let mut ptb = ProgrammableTransactionBuilder::new();
    let gas_coin = coins.first().unwrap();
    let coins = coins
        .iter()
        .skip(1)
        .map(|c| {
            ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(*c)))
                // Safe to unwrap as this function is only being called at the swarm config.
                .unwrap()
        })
        .collect::<Vec<_>>();

    ptb.command(sui_types::transaction::Command::MergeCoins(
        // Safe to unwrap as this function is only being called at the swarm config.
        Argument::GasCoin,
        // Keep the gas object out
        coins.to_vec(),
    ));
    let tx_kind = TransactionKind::ProgrammableTransaction(ptb.finish());
    let _ = execute_sui_transaction(publisher_address, tx_kind, context, vec![gas_coin.0]).await?;

    Ok(())
}

async fn stake_ika(
    publisher_address: SuiAddress,
    context: &mut WalletContext,
    ika_system_package_id: ObjectID,
    ika_system_object_id: ObjectID,
    init_system_shared_version: SequenceNumber,
    ika_supply_id: ObjectID,
    validator_ids: Vec<ObjectID>,
) -> Result<(), anyhow::Error> {
    let mut ptb = ProgrammableTransactionBuilder::new();

    let init_arg = ptb.input(CallArg::Object(ObjectArg::SharedObject {
        id: ika_system_object_id,
        initial_shared_version: init_system_shared_version,
        mutable: true,
    }))?;

    let client = context.get_client().await?;

    let ika_supply_ref = client
        .transaction_builder()
        .get_object_ref(ika_supply_id)
        .await?;

    let ika_supply_id_arg =
        ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(ika_supply_ref)))?;
    let stake_amount = ptb.input(CallArg::Pure(bcs::to_bytes(
        &MIN_VALIDATOR_JOINING_STAKE_NIKA,
    )?))?;

    for validator_id in validator_ids {
        let stake = ptb.command(sui_types::transaction::Command::SplitCoins(
            ika_supply_id_arg,
            vec![stake_amount],
        ));
        let validator = ptb.input(CallArg::Pure(bcs::to_bytes(&validator_id).unwrap()))?;

        let staked = ptb.command(sui_types::transaction::Command::move_call(
            ika_system_package_id,
            SYSTEM_MODULE_NAME.into(),
            REQUEST_ADD_STAKE_FUNCTION_NAME.into(),
            vec![],
            vec![init_arg, stake, validator],
        ));

        ptb.transfer_arg(publisher_address, staked);
    }

    let tx_kind = TransactionKind::ProgrammableTransaction(ptb.finish());

    let _ = execute_sui_transaction(publisher_address, tx_kind, context, vec![]).await?;

    Ok(())
}

pub async fn minted_ika(
    publisher_address: SuiAddress,
    client: SuiClient,
    ika_package_id: ObjectID,
) -> Result<ObjectID, anyhow::Error> {
    let data = client
        .read_api()
        .get_owned_objects(
            publisher_address,
            Some(SuiObjectResponseQuery {
                filter: Some(SuiObjectDataFilter::StructType(IKACoin::type_(
                    ika_package_id.into(),
                ))),
                options: None,
            }),
            None,
            None,
        )
        .await?;

    let ika_supply_id = &data.data.first().unwrap().object_id()?;

    Ok(*ika_supply_id)
}

async fn request_add_validator_candidate(
    validator_address: SuiAddress,
    context: &mut WalletContext,
    client: SuiClient,
    validator_initialization_metadata: &ValidatorInfo,
    ika_system_package_id: ObjectID,
    ika_system_object_id: ObjectID,
    init_system_shared_version: SequenceNumber,
) -> Result<(ObjectID, ObjectID), anyhow::Error> {
    let mut ptb = ProgrammableTransactionBuilder::new();

    let class_groups_pubkey_and_proof_obj_ref = create_class_groups_public_key_and_proof_object(
        validator_address,
        context,
        &client,
        ika_system_package_id,
        validator_initialization_metadata
            .class_groups_public_key_and_proof
            .clone(),
    )
    .await?;

    let name = ptb.input(CallArg::Pure(bcs::to_bytes(
        validator_initialization_metadata.name.as_str(),
    )?))?;
    let empty_str = ptb.input(CallArg::Pure(bcs::to_bytes(String::new().as_str())?))?;

    let system_ref = ptb.input(CallArg::Object(ObjectArg::SharedObject {
        id: ika_system_object_id,
        initial_shared_version: init_system_shared_version,
        mutable: true,
    }))?;

    let protocol_public_key = ptb.input(CallArg::Pure(bcs::to_bytes(
        &validator_initialization_metadata
            .protocol_public_key
            .as_bytes()
            .to_vec(),
    )?))?;

    let network_public_key = ptb.input(CallArg::Pure(bcs::to_bytes(
        &validator_initialization_metadata
            .network_public_key
            .as_bytes()
            .to_vec(),
    )?))?;

    let consensus_public_key = ptb.input(CallArg::Pure(bcs::to_bytes(
        &validator_initialization_metadata
            .consensus_public_key
            .as_bytes()
            .to_vec(),
    )?))?;

    let class_groups_pubkey_and_proof_obj_ref = ptb.input(CallArg::Object(
        ObjectArg::ImmOrOwnedObject(class_groups_pubkey_and_proof_obj_ref),
    ))?;

    let proof_of_possession = ptb.input(CallArg::Pure(bcs::to_bytes(
        &validator_initialization_metadata
            .proof_of_possession
            .as_ref()
            .to_vec(),
    )?))?;

    let network_address = ptb.input(CallArg::Pure(bcs::to_bytes(
        &validator_initialization_metadata.network_address.clone(),
    )?))?;

    let p2p_address = ptb.input(CallArg::Pure(bcs::to_bytes(
        &validator_initialization_metadata.p2p_address.clone(),
    )?))?;

    let consensus_address = ptb.input(CallArg::Pure(bcs::to_bytes(
        &validator_initialization_metadata.consensus_address.clone(),
    )?))?;

    let commission_rate = ptb.input(CallArg::Pure(bcs::to_bytes(
        &validator_initialization_metadata.commission_rate,
    )?))?;

    let metadata = ptb.command(Command::move_call(
        ika_system_package_id,
        VALIDATOR_METADATA_MODULE_NAME.into(),
        NEW_VALIDATOR_METADATA_FUNCTION_NAME.into(),
        vec![],
        vec![name, empty_str, empty_str],
    ));

    ptb.command(Command::move_call(
        ika_system_package_id,
        SYSTEM_MODULE_NAME.into(),
        REQUEST_ADD_VALIDATOR_CANDIDATE_FUNCTION_NAME.into(),
        vec![],
        vec![
            system_ref,
            name,
            protocol_public_key,
            network_public_key,
            consensus_public_key,
            class_groups_pubkey_and_proof_obj_ref,
            proof_of_possession,
            network_address,
            p2p_address,
            consensus_address,
            commission_rate,
            metadata,
        ],
    ));

    ptb.transfer_args(
        validator_address,
        vec![
            Argument::NestedResult(1, 0),
            Argument::NestedResult(1, 1),
            Argument::NestedResult(1, 2),
        ],
    );

    let tx_kind = TransactionKind::ProgrammableTransaction(ptb.finish());

    let response = execute_sui_transaction(validator_address, tx_kind, context, vec![]).await?;

    let object_changes = response.object_changes.unwrap();

    let validator_cap_type = StructTag {
        address: ika_system_package_id.into(),
        module: VALIDATOR_CAP_MODULE_NAME.into(),
        name: VALIDATOR_CAP_STRUCT_NAME.into(),
        type_params: vec![],
    };

    let validator_cap_id = *object_changes
        .iter()
        .filter_map(|o| match o {
            ObjectChange::Created {
                object_id,
                object_type,
                ..
            } if validator_cap_type == *object_type => Some(*object_id),
            _ => None,
        })
        .collect::<Vec<_>>()
        .first()
        .unwrap();

    let validator_cap = context
        .get_client()
        .await?
        .read_api()
        .get_move_object_bcs(validator_cap_id)
        .await?;
    let validator_cap: ValidatorCapV1 = bcs::from_bytes(&validator_cap)?;

    Ok((validator_cap.validator_id, validator_cap_id))
}

pub async fn publish_ika_system_package_to_sui(
    publisher_address: SuiAddress,
    context: &mut WalletContext,
    client: SuiClient,
    ika_system_package: &IkaMovePackage,
    ika_package_id: ObjectID,
) -> Result<(ObjectID, ObjectID, ObjectID), anyhow::Error> {
    let mut ika_system_package_dependencies = ika_system_package.dependencies.clone();
    ika_system_package_dependencies.push(ika_package_id);

    let bytes =
        ika_system_package.bytes_with_deps(HashMap::from([("ika".to_string(), ika_package_id)]))?;

    let object_changes = publish_package_to_sui(
        publisher_address,
        context,
        client,
        bytes,
        ika_system_package_dependencies,
    )
    .await?;
    let ika_system_package_id = *object_changes
        .iter()
        .filter_map(|o| match o {
            ObjectChange::Published { package_id, .. } => Some(*package_id),
            _ => None,
        })
        .collect::<Vec<_>>()
        .first()
        .unwrap();

    let init_cap_type = StructTag {
        address: ika_system_package_id.into(),
        module: INIT_MODULE_NAME.into(),
        name: INIT_CAP_STRUCT_NAME.into(),
        type_params: vec![],
    };

    let init_cap_id = *object_changes
        .iter()
        .filter_map(|o| match o {
            ObjectChange::Created {
                object_id,
                object_type,
                ..
            } if init_cap_type == *object_type => Some(*object_id),
            _ => None,
        })
        .collect::<Vec<_>>()
        .first()
        .unwrap();

    let ika_system_package_upgrade_cap_id = *object_changes
        .iter()
        .filter_map(|o| match o {
            ObjectChange::Created {
                object_id,
                object_type,
                ..
            } if UpgradeCap::type_() == *object_type => Some(*object_id),
            _ => None,
        })
        .collect::<Vec<_>>()
        .first()
        .unwrap();

    Ok((
        ika_system_package_id,
        init_cap_id,
        ika_system_package_upgrade_cap_id,
    ))
}

async fn create_class_groups_public_key_and_proof_builder_object(
    publisher_address: SuiAddress,
    context: &mut WalletContext,
    client: &SuiClient,
    ika_system_package_id: ObjectID,
) -> anyhow::Result<ObjectRef> {
    let mut ptb = ProgrammableTransactionBuilder::new();
    ptb.move_call(
        ika_system_package_id,
        CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_MODULE_NAME.into(),
        CREATE_CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_BUILDER_FUNCTION_NAME.into(),
        vec![],
        vec![],
    )?;
    ptb.transfer_arg(publisher_address, Argument::Result(0));
    let tx_kind = TransactionKind::ProgrammableTransaction(ptb.finish());

    let response = execute_sui_transaction(publisher_address, tx_kind, context, vec![]).await?;

    let object_changes = response.object_changes.unwrap();

    let builder_id = *object_changes
        .iter()
        .filter_map(|o| match o {
            ObjectChange::Created {
                object_id,
                object_type,
                ..
            } if ClassGroupsPublicKeyAndProofBuilder::type_(ika_system_package_id.into())
                == *object_type =>
            {
                Some(*object_id)
            }
            _ => None,
        })
        .collect::<Vec<_>>()
        .first()
        .unwrap();

    let builder_ref = client
        .transaction_builder()
        .get_object_ref(builder_id)
        .await?;

    Ok(builder_ref)
}

async fn create_class_groups_public_key_and_proof_object(
    publisher_address: SuiAddress,
    context: &mut WalletContext,
    client: &SuiClient,
    ika_system_package_id: ObjectID,
    class_groups_public_key_and_proof_bytes: Vec<u8>,
) -> anyhow::Result<ObjectRef> {
    let builder_object_ref = create_class_groups_public_key_and_proof_builder_object(
        publisher_address,
        context,
        client,
        ika_system_package_id,
    )
    .await?;

    let class_groups_public_key_and_proof: Box<ClassGroupsEncryptionKeyAndProof> =
        Box::new(bcs::from_bytes(&class_groups_public_key_and_proof_bytes)?);

    add_public_keys_and_proofs_with_rng(
        publisher_address,
        context,
        client,
        ika_system_package_id,
        (0, 3),
        builder_object_ref.0,
        &class_groups_public_key_and_proof,
    )
    .await?;
    let builder_object_ref = client
        .transaction_builder()
        .get_object_ref(builder_object_ref.0)
        .await?;
    let mut ptb = ProgrammableTransactionBuilder::new();
    ptb.move_call(
        ika_system_package_id,
        CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_MODULE_NAME.into(),
        FINISH_CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_FUNCTION_NAME.into(),
        vec![],
        vec![CallArg::Object(ObjectArg::ImmOrOwnedObject(
            builder_object_ref,
        ))],
    )?;
    ptb.transfer_arg(publisher_address, Argument::Result(0));
    let tx_kind = TransactionKind::ProgrammableTransaction(ptb.finish());

    let response = execute_sui_transaction(publisher_address, tx_kind, context, vec![]).await?;

    let object_changes = response
        .object_changes
        .ok_or(anyhow::Error::msg("Failed to get object changes"))?;

    let obj_id = *object_changes
        .iter()
        .filter_map(|o| match o {
            ObjectChange::Created {
                object_id,
                object_type,
                ..
            } if ClassGroupsPublicKeyAndProof::type_(ika_system_package_id.into())
                == *object_type =>
            {
                Some(*object_id)
            }
            _ => None,
        })
        .collect::<Vec<_>>()
        .first()
        .unwrap();

    let pubkey_and_proof_obj_ref = client.transaction_builder().get_object_ref(obj_id).await?;

    Ok(pubkey_and_proof_obj_ref)
}

async fn add_public_keys_and_proofs_with_rng(
    publisher_address: SuiAddress,
    context: &mut WalletContext,
    client: &SuiClient,
    ika_system_package_id: ObjectID,
    range: (u8, u8),
    cg_builder_object_id: ObjectID,
    class_groups_public_key_and_proof: &ClassGroupsEncryptionKeyAndProof,
) -> anyhow::Result<()> {
    let mut first_ptb = ProgrammableTransactionBuilder::new();
    let builder_object_ref = client
        .transaction_builder()
        .get_object_ref(cg_builder_object_id)
        .await?;
    for i in range.0..range.1 {
        let pubkey_and_proof = bcs::to_bytes(&class_groups_public_key_and_proof[i as usize])?;
        let proof_builder = first_ptb.obj(ObjectArg::ImmOrOwnedObject(builder_object_ref))?;
        let first_proof_bytes_half = first_ptb.pure(pubkey_and_proof[0..10_000].to_vec())?;
        let second_proof_bytes_half = first_ptb.pure(pubkey_and_proof[10_000..].to_vec())?;
        first_ptb.programmable_move_call(
            ika_system_package_id,
            CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_MODULE_NAME.into(),
            ADD_PAIR_TO_CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_FUNCTION_NAME.into(),
            vec![],
            vec![
                proof_builder,
                // Sui limits the size of a single call argument to 16KB.
                first_proof_bytes_half,
                second_proof_bytes_half,
            ],
        );
    }
    let tx_kind = TransactionKind::ProgrammableTransaction(first_ptb.finish());
    execute_sui_transaction(publisher_address, tx_kind, context, vec![]).await?;
    Ok(())
}

pub async fn publish_ika_package_to_sui(
    publisher_address: SuiAddress,
    context: &mut WalletContext,
    client: SuiClient,
    ika_package: &IkaMovePackage,
) -> Result<(ObjectID, ObjectID, ObjectID), anyhow::Error> {
    let object_changes = publish_package_to_sui(
        publisher_address,
        context,
        client,
        ika_package.bytes.clone(),
        ika_package.dependencies.clone(),
    )
    .await?;
    let ika_package_id = *object_changes
        .iter()
        .filter_map(|o| match o {
            ObjectChange::Published { package_id, .. } => Some(*package_id),
            _ => None,
        })
        .collect::<Vec<_>>()
        .first()
        .unwrap();

    let treasury_cap_id = *object_changes
        .iter()
        .filter_map(|o| match o {
            ObjectChange::Created {
                object_id,
                object_type,
                ..
            } if TreasuryCap::is_treasury_type(object_type) => Some(*object_id),
            _ => None,
        })
        .collect::<Vec<_>>()
        .first()
        .unwrap();

    let ika_package_upgrade_cap_id = *object_changes
        .iter()
        .filter_map(|o| match o {
            ObjectChange::Created {
                object_id,
                object_type,
                ..
            } if UpgradeCap::type_() == *object_type => Some(*object_id),
            _ => None,
        })
        .collect::<Vec<_>>()
        .first()
        .unwrap();

    Ok((ika_package_id, treasury_cap_id, ika_package_upgrade_cap_id))
}

async fn publish_package_to_sui(
    publisher_address: SuiAddress,
    context: &mut WalletContext,
    client: SuiClient,
    ika_move_package_bytes: Vec<Vec<u8>>,
    ika_move_package_dep_ids: Vec<ObjectID>,
) -> Result<Vec<ObjectChange>, anyhow::Error> {
    let tx_kind = client
        .transaction_builder()
        .publish_tx_kind(
            publisher_address,
            ika_move_package_bytes,
            ika_move_package_dep_ids,
        )
        .await?;

    let response = execute_sui_transaction(publisher_address, tx_kind, context, vec![]).await?;

    let object_changes = response.object_changes.unwrap();
    Ok(object_changes)
}

pub(crate) async fn create_sui_transaction(
    signer: SuiAddress,
    tx_kind: TransactionKind,
    context: &mut WalletContext,
    gas_payment: Vec<ObjectID>,
) -> Result<Transaction, anyhow::Error> {
    let gas_price = context.get_reference_gas_price().await?;

    let client = context.get_client().await?;

    //let gas_budget = max_gas_budget(&client).await?;
    let gas_budget =
        estimate_gas_budget(context, signer, tx_kind.clone(), gas_price, None, None).await?;

    let tx_data = client
        .transaction_builder()
        .tx_data(signer, tx_kind, gas_budget, gas_price, gas_payment, None)
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
    gas_payment: Vec<ObjectID>,
) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let transaction = create_sui_transaction(signer, tx_kind, context, gas_payment).await?;

    let response = context
        .execute_transaction_may_fail(transaction.clone())
        .await?;
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
