use anyhow::bail;
use fastcrypto::traits::ToFromBytes;
use ika_config::validator_info::ValidatorInfo;
use ika_types::committee::ClassGroupsEncryptionKeyAndProof;
use ika_types::sui::system_inner_v1::ValidatorCapV1;
use ika_types::sui::{
    ADD_PAIR_TO_CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_FUNCTION_NAME,
    CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_MODULE_NAME,
    CREATE_CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_BUILDER_FUNCTION_NAME, ClassGroupsPublicKeyAndProof,
    ClassGroupsPublicKeyAndProofBuilder, FINISH_CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_FUNCTION_NAME,
    NEW_VALIDATOR_METADATA_FUNCTION_NAME, REQUEST_ADD_STAKE_FUNCTION_NAME,
    REQUEST_ADD_VALIDATOR_CANDIDATE_FUNCTION_NAME, REQUEST_ADD_VALIDATOR_FUNCTION_NAME,
    REQUEST_REMOVE_VALIDATOR_FUNCTION_NAME, SYSTEM_MODULE_NAME, VALIDATOR_CAP_MODULE_NAME,
    VALIDATOR_CAP_STRUCT_NAME, VALIDATOR_METADATA_MODULE_NAME,
};
use move_core_types::identifier::IdentStr;
use move_core_types::language_storage::StructTag;
use shared_crypto::intent::Intent;
use sui::fire_drill::get_gas_obj_ref;
use sui_json_rpc_types::{ObjectChange, SuiTransactionBlockResponse};
use sui_json_rpc_types::{SuiObjectDataOptions, SuiTransactionBlockResponseOptions};
use sui_keys::keystore::AccountKeystore;
use sui_sdk::SuiClient;
use sui_sdk::wallet_context::WalletContext;
use sui_types::base_types::{ObjectID, ObjectRef, SuiAddress};
use sui_types::object::Owner;
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::TransactionData;
use sui_types::transaction::{Argument, CallArg, Command, ObjectArg, Transaction, TransactionKind};

/// Create a ClassGroupsPublicKeyAndProofBuilder object
async fn create_class_groups_public_key_and_proof_builder_object(
    publisher_address: SuiAddress,
    context: &mut WalletContext,
    client: &SuiClient,
    ika_common_package_id: ObjectID,
    gas_budget: u64,
) -> anyhow::Result<ObjectRef> {
    let mut ptb = ProgrammableTransactionBuilder::new();
    ptb.move_call(
        ika_common_package_id,
        CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_MODULE_NAME.into(),
        CREATE_CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_BUILDER_FUNCTION_NAME.into(),
        vec![],
        vec![],
    )?;
    ptb.transfer_arg(publisher_address, Argument::Result(0));

    let tx_data = construct_unsigned_txn(context, publisher_address, gas_budget, ptb).await?;
    let response = execute_transaction(context, tx_data).await?;
    let object_changes = response
        .object_changes
        .ok_or(anyhow::Error::msg("failed to get object changes"))?;
    let builder_id = *object_changes
        .iter()
        .filter_map(|o| match o {
            ObjectChange::Created {
                object_id,
                object_type,
                ..
            } if ClassGroupsPublicKeyAndProofBuilder::type_(ika_common_package_id.into())
                == *object_type =>
            {
                Some(*object_id)
            }
            _ => None,
        })
        .collect::<Vec<_>>()
        .first()
        .ok_or(anyhow::Error::msg(
            "failed to get the class groups builder object id",
        ))?;

    let builder_ref = client
        .transaction_builder()
        .get_object_ref(builder_id)
        .await?;

    Ok(builder_ref)
}

/// Create a ClassGroupsPublicKeyAndProof object, using the ClassGroupsPublicKeyAndProofBuilder object
pub async fn create_class_groups_public_key_and_proof_object(
    publisher_address: SuiAddress,
    context: &mut WalletContext,
    ika_common_package_id: ObjectID,
    class_groups_public_key_and_proof_bytes: ClassGroupsEncryptionKeyAndProof,
    gas_budget: u64,
) -> anyhow::Result<ObjectRef> {
    let client = context.get_client().await?;
    let mut builder_object_ref = create_class_groups_public_key_and_proof_builder_object(
        publisher_address,
        context,
        &client,
        ika_common_package_id,
        gas_budget,
    )
    .await?;

    let class_groups_public_key_and_proof: Box<ClassGroupsEncryptionKeyAndProof> =
        Box::new(class_groups_public_key_and_proof_bytes);
    for pubkey_and_proof in class_groups_public_key_and_proof.iter() {
        let mut ptb = ProgrammableTransactionBuilder::new();
        let pubkey_and_proof = bcs::to_bytes(pubkey_and_proof)?;

        ptb.move_call(
            ika_common_package_id,
            CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_MODULE_NAME.into(),
            ADD_PAIR_TO_CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_FUNCTION_NAME.into(),
            vec![],
            vec![
                CallArg::Object(ObjectArg::ImmOrOwnedObject(builder_object_ref)),
                CallArg::Pure(bcs::to_bytes(&pubkey_and_proof[0..10_000])?),
                CallArg::Pure(bcs::to_bytes(&pubkey_and_proof[10_000..])?),
            ],
        )?;

        let tx_data = construct_unsigned_txn(context, publisher_address, gas_budget, ptb).await?;

        let response = execute_transaction(context, tx_data).await?;
        let object_changes = response
            .object_changes
            .clone()
            .ok_or(anyhow::Error::msg("Failed to get object changes"))?;
        let builder_id = *object_changes
            .iter()
            .filter_map(|o| match o {
                ObjectChange::Mutated {
                    object_id,
                    object_type,
                    ..
                } if ClassGroupsPublicKeyAndProofBuilder::type_(ika_common_package_id.into())
                    == *object_type =>
                {
                    Some(*object_id)
                }
                _ => None,
            })
            .collect::<Vec<_>>()
            .first()
            .ok_or(anyhow::Error::msg(
                "failed to get ClassGroupsPublicKeyAndProofBuilder object id",
            ))?;

        builder_object_ref = client
            .transaction_builder()
            .get_object_ref(builder_id)
            .await?;
    }

    let mut ptb = ProgrammableTransactionBuilder::new();
    ptb.move_call(
        ika_common_package_id,
        CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_MODULE_NAME.into(),
        FINISH_CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_FUNCTION_NAME.into(),
        vec![],
        vec![CallArg::Object(ObjectArg::ImmOrOwnedObject(
            builder_object_ref,
        ))],
    )?;
    ptb.transfer_arg(publisher_address, Argument::Result(0));

    let tx_data = construct_unsigned_txn(context, publisher_address, gas_budget, ptb).await?;
    let response = execute_transaction(context, tx_data).await?;
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
            } if ClassGroupsPublicKeyAndProof::type_(ika_common_package_id.into())
                == *object_type =>
            {
                Some(*object_id)
            }
            _ => None,
        })
        .collect::<Vec<_>>()
        .first()
        .ok_or(anyhow::Error::msg(
            "failed to get ClassGroupsPublicKeyAndProof object id",
        ))?;

    let pubkey_and_proof_obj_ref = client.transaction_builder().get_object_ref(obj_id).await?;

    Ok(pubkey_and_proof_obj_ref)
}

/// Request to add a validator candidate transaction
pub async fn request_add_validator_candidate(
    context: &mut WalletContext,
    validator_initialization_metadata: &ValidatorInfo,
    ika_system_package_id: ObjectID,
    ika_system_object_id: ObjectID,
    class_groups_pubkey_and_proof_obj_ref: ObjectRef,
    gas_budget: u64,
) -> Result<(SuiTransactionBlockResponse, ObjectID, ObjectID), anyhow::Error> {
    let mut ptb = ProgrammableTransactionBuilder::new();
    let name = ptb.input(CallArg::Pure(bcs::to_bytes(
        validator_initialization_metadata.name.as_str(),
    )?))?;
    let empty_str = ptb.input(CallArg::Pure(bcs::to_bytes(String::new().as_str())?))?;

    let Some(Owner::Shared {
        initial_shared_version,
    }) = context
        .get_client()
        .await?
        .read_api()
        .get_object_with_options(
            ika_system_object_id,
            SuiObjectDataOptions::new().with_owner(),
        )
        .await?
        .data
        .ok_or(anyhow::Error::msg("failed to get object data"))?
        .owner
    else {
        bail!("Failed to get owner of object")
    };

    let system_ref = ptb.input(CallArg::Object(ObjectArg::SharedObject {
        id: ika_system_object_id,
        initial_shared_version,
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

    let sender = context.active_address()?;

    ptb.transfer_args(
        sender,
        vec![
            Argument::NestedResult(1, 0),
            Argument::NestedResult(1, 1),
            Argument::NestedResult(1, 2),
        ],
    );

    let tx = construct_unsigned_txn(context, sender, gas_budget, ptb).await?;

    let response = execute_transaction(context, tx).await?;

    let object_changes = response
        .object_changes
        .clone()
        .ok_or(anyhow::Error::msg("failed to get object changes"))?;

    if !response.errors.is_empty() {
        println!("{:?}", response.errors);
        panic!("Become-candidate failed")
    }

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
        .ok_or(anyhow::Error::msg("failed to get validator cap object id"))?;

    let validator_cap = context
        .get_client()
        .await?
        .read_api()
        .get_move_object_bcs(validator_cap_id)
        .await?;
    let validator_cap: ValidatorCapV1 = bcs::from_bytes(&validator_cap)?;

    Ok((response, validator_cap.validator_id, validator_cap_id))
}

pub async fn stake_ika(
    context: &mut WalletContext,
    ika_system_package_id: ObjectID,
    ika_system_object_id: ObjectID,
    ika_supply_id: ObjectID,
    validator_id: ObjectID,
    stake_amount: u64,
    gas_budget: u64,
) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let mut ptb = ProgrammableTransactionBuilder::new();
    let client = context.get_client().await?;
    let ika_supply_ref = client
        .transaction_builder()
        .get_object_ref(ika_supply_id)
        .await?;

    let ika_supply_id_arg =
        ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(ika_supply_ref)))?;
    let stake_amount = ptb.input(CallArg::Pure(bcs::to_bytes(&stake_amount)?))?;

    let stake = ptb.command(sui_types::transaction::Command::SplitCoins(
        ika_supply_id_arg,
        vec![stake_amount],
    ));
    let validator = ptb.input(CallArg::Pure(bcs::to_bytes(&validator_id)?))?;
    let call_args = vec![stake, validator];

    let sender = context.active_address()?;

    add_ika_system_command_to_ptb(
        context,
        REQUEST_ADD_STAKE_FUNCTION_NAME,
        call_args,
        ika_system_object_id,
        ika_system_package_id,
        &mut ptb,
    )
    .await?;

    ptb.transfer_args(sender, vec![Argument::NestedResult(1, 0)]);

    let tx_data = construct_unsigned_txn(context, sender, gas_budget, ptb).await?;

    execute_transaction(context, tx_data).await
}

pub async fn request_add_validator(
    context: &mut WalletContext,
    ika_system_package_id: ObjectID,
    ika_system_object_id: ObjectID,
    validator_cap_id: ObjectID,
    gas_budget: u64,
) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let client = context.get_client().await?;
    let validator_cap_ref = client
        .transaction_builder()
        .get_object_ref(validator_cap_id)
        .await?;

    let mut ptb = ProgrammableTransactionBuilder::new();
    let call_args = vec![ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(
        validator_cap_ref,
    )))?];

    let sender = context.active_address()?;

    add_ika_system_command_to_ptb(
        context,
        REQUEST_ADD_VALIDATOR_FUNCTION_NAME,
        call_args,
        ika_system_object_id,
        ika_system_package_id,
        &mut ptb,
    )
    .await?;

    let tx_data = construct_unsigned_txn(context, sender, gas_budget, ptb).await?;

    execute_transaction(context, tx_data).await
}

pub async fn request_remove_validator(
    context: &mut WalletContext,
    ika_system_package_id: ObjectID,
    ika_system_object_id: ObjectID,
    validator_cap_id: ObjectID,
    gas_budget: u64,
) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let client = context.get_client().await?;
    let validator_cap_ref = client
        .transaction_builder()
        .get_object_ref(validator_cap_id)
        .await?;

    let mut ptb = ProgrammableTransactionBuilder::new();
    let call_args = vec![ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(
        validator_cap_ref,
    )))?];

    call_ika_system(
        context,
        REQUEST_REMOVE_VALIDATOR_FUNCTION_NAME,
        call_args,
        gas_budget,
        ika_system_object_id,
        ika_system_package_id,
        ptb,
    )
    .await
}

async fn construct_unsigned_ika_system_txn(
    context: &mut WalletContext,
    sender: SuiAddress,
    function: &'static IdentStr,
    call_args: Vec<Argument>,
    gas_budget: u64,
    ika_system_object_id: ObjectID,
    ika_system_package_id: ObjectID,
    mut ptb: ProgrammableTransactionBuilder,
) -> anyhow::Result<TransactionData> {
    add_ika_system_command_to_ptb(
        context,
        function,
        call_args,
        ika_system_object_id,
        ika_system_package_id,
        &mut ptb,
    )
    .await?;

    construct_unsigned_txn(context, sender, gas_budget, ptb).await
}

async fn add_ika_system_command_to_ptb(
    context: &mut WalletContext,
    function: &IdentStr,
    call_args: Vec<Argument>,
    ika_system_object_id: ObjectID,
    ika_system_package_id: ObjectID,
    ptb: &mut ProgrammableTransactionBuilder,
) -> anyhow::Result<()> {
    let Some(Owner::Shared {
        initial_shared_version,
    }) = context
        .get_client()
        .await?
        .read_api()
        .get_object_with_options(
            ika_system_object_id,
            SuiObjectDataOptions::new().with_owner(),
        )
        .await?
        .data
        .ok_or(anyhow::Error::msg("failed to get object data"))?
        .owner
    else {
        bail!("Failed to get owner of object")
    };

    let mut args = vec![ptb.input(CallArg::Object(ObjectArg::SharedObject {
        id: ika_system_object_id,
        initial_shared_version,
        mutable: true,
    }))?];

    args.extend(call_args);

    ptb.command(sui_types::transaction::Command::move_call(
        ika_system_package_id,
        SYSTEM_MODULE_NAME.into(),
        function.to_owned(),
        vec![],
        args,
    ));
    Ok(())
}

async fn construct_unsigned_txn(
    context: &mut WalletContext,
    sender: SuiAddress,
    gas_budget: u64,
    ptb: ProgrammableTransactionBuilder,
) -> anyhow::Result<TransactionData> {
    let sui_client = context.get_client().await?;
    let gas_price = context.get_reference_gas_price().await?;

    let tx = ptb.finish();
    let tx_kind = TransactionKind::ProgrammableTransaction(tx.clone());
    let gas_budget =
        sui::client_commands::estimate_gas_budget(context, sender, tx_kind, gas_price, None, None)
            .await
            .unwrap_or(gas_budget);

    let rgp = sui_client
        .governance_api()
        .get_reference_gas_price()
        .await?;

    let gas_obj_ref = get_gas_obj_ref(sender, &sui_client, gas_budget).await?;

    Ok(TransactionData::new_programmable(
        sender,
        vec![gas_obj_ref],
        tx,
        gas_budget,
        rgp,
    ))
}

pub async fn execute_transaction(
    context: &mut WalletContext,
    tx_data: TransactionData,
) -> anyhow::Result<SuiTransactionBlockResponse> {
    let sender = context.active_address()?;

    let signature =
        context
            .config
            .keystore
            .sign_secure(&sender, &tx_data, Intent::sui_transaction())?;
    let transaction = Transaction::from_data(tx_data, vec![signature]);
    let sui_client = context.get_client().await?;
    sui_client
        .quorum_driver_api()
        .execute_transaction_block(
            transaction,
            SuiTransactionBlockResponseOptions::new()
                .with_input()
                .with_effects()
                .with_object_changes(),
            Some(sui_types::quorum_driver_types::ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await
        .map_err(|err| anyhow::anyhow!(err.to_string()))
}

pub async fn call_ika_system(
    context: &mut WalletContext,
    function: &'static IdentStr,
    call_args: Vec<Argument>,
    gas_budget: u64,
    ika_system_object_id: ObjectID,
    ika_system_package_id: ObjectID,
    ptb: ProgrammableTransactionBuilder,
) -> anyhow::Result<SuiTransactionBlockResponse> {
    let sender = context.active_address()?;
    let tx_data = construct_unsigned_ika_system_txn(
        context,
        sender,
        function,
        call_args,
        gas_budget,
        ika_system_object_id,
        ika_system_package_id,
        ptb,
    )
    .await?;
    execute_transaction(context, tx_data).await
}
