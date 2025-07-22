use anyhow::bail;
use fastcrypto::traits::ToFromBytes;
use ika_config::validator_info::ValidatorInfo;
use ika_types::committee::ClassGroupsEncryptionKeyAndProof;
use ika_types::sui::system_inner_v1::ValidatorCapV1;
use ika_types::sui::{
    ADD_PAIR_TO_CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_FUNCTION_NAME,
    CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_MODULE_NAME, COLLECT_COMMISSION_FUNCTION_NAME,
    CREATE_CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_BUILDER_FUNCTION_NAME, ClassGroupsPublicKeyAndProof,
    ClassGroupsPublicKeyAndProofBuilder, DROP_OPRIONAL_TABLE_VEC_FUNC_NAME,
    DWALLET_2PC_MPC_COORDINATOR_MODULE_NAME,
    FINISH_CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_FUNCTION_NAME, NEW_VALIDATOR_METADATA_FUNCTION_NAME,
    PricingInfoKey, PricingInfoValue, REPORT_VALIDATOR_FUNCTION_NAME,
    REQUEST_ADD_STAKE_FUNCTION_NAME, REQUEST_ADD_VALIDATOR_CANDIDATE_FUNCTION_NAME,
    REQUEST_ADD_VALIDATOR_FUNCTION_NAME, REQUEST_REMOVE_VALIDATOR_CANDIDATE_FUNCTION_NAME,
    REQUEST_REMOVE_VALIDATOR_FUNCTION_NAME, REQUEST_WITHDRAW_STAKE_FUNCTION_NAME,
    ROTATE_COMMISSION_CAP_FUNCTION_NAME, ROTATE_OPERATION_CAP_FUNCTION_NAME,
    SET_NEXT_COMMISSION_FUNCTION_NAME,
    SET_NEXT_EPOCH_CLASS_GROUPS_PUBKEY_AND_PROOF_BYTES_FUNCTION_NAME,
    SET_NEXT_EPOCH_CONSENSUS_ADDRESS_FUNCTION_NAME,
    SET_NEXT_EPOCH_CONSENSUS_PUBKEY_BYTES_FUNCTION_NAME,
    SET_NEXT_EPOCH_NETWORK_ADDRESS_FUNCTION_NAME,
    SET_NEXT_EPOCH_NETWORK_PUBKEY_BYTES_FUNCTION_NAME, SET_NEXT_EPOCH_P2P_ADDRESS_FUNCTION_NAME,
    SET_NEXT_EPOCH_PROTOCOL_PUBKEY_BYTES_FUNCTION_NAME, SET_PRICING_VOTE_FUNCTION_NAME,
    SET_VALIDATOR_METADATA_FUNCTION_NAME, SET_VALIDATOR_NAME_FUNCTION_NAME, SYSTEM_MODULE_NAME,
    UNDO_REPORT_VALIDATOR_FUNCTION_NAME, VALIDATOR_CAP_MODULE_NAME, VALIDATOR_CAP_STRUCT_NAME,
    VALIDATOR_COMMISSION_STRUCT_NAME, VALIDATOR_METADATA_FUNCTION_NAME,
    VALIDATOR_METADATA_MODULE_NAME, VALIDATOR_OPERATION_STRUCT_NAME,
    VERIFY_COMMISSION_CAP_FUNCTION_NAME, VERIFY_OPERATION_CAP_FUNCTION_NAME,
    VERIFY_VALIDATOR_CAP_FUNCTION_NAME, WITHDRAW_STAKE_FUNCTION_NAME,
};
use move_core_types::ident_str;
use move_core_types::identifier::IdentStr;
use move_core_types::language_storage::StructTag;
use serde::Serialize;
use shared_crypto::intent::Intent;
use sui::fire_drill::get_gas_obj_ref;
use sui_json_rpc_types::{ObjectChange, SuiTransactionBlockResponse};
use sui_json_rpc_types::{SuiObjectDataOptions, SuiTransactionBlockResponseOptions};
use sui_keys::keystore::AccountKeystore;
use sui_sdk::SuiClient;
use sui_sdk::wallet_context::WalletContext;
use sui_types::base_types::{ObjectID, ObjectRef, SuiAddress};
use sui_types::collection_types::Entry;
use sui_types::object::Owner;
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::TransactionData;
use sui_types::transaction::{Argument, CallArg, Command, ObjectArg, Transaction, TransactionKind};

#[derive(Serialize)]
pub struct BecomeCandidateValidatorData {
    pub validator_id: ObjectID,
    pub validator_cap_id: ObjectID,
    pub validator_operation_cap_id: ObjectID,
    pub validator_commission_cap_id: ObjectID,
}

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
) -> Result<(SuiTransactionBlockResponse, BecomeCandidateValidatorData), anyhow::Error> {
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

    let validator_operation_cap_type = StructTag {
        address: ika_system_package_id.into(),
        module: VALIDATOR_CAP_MODULE_NAME.into(),
        name: VALIDATOR_OPERATION_STRUCT_NAME.into(),
        type_params: vec![],
    };

    let validator_operation_cap_id = *object_changes
        .iter()
        .filter_map(|o| match o {
            ObjectChange::Created {
                object_id,
                object_type,
                ..
            } if validator_operation_cap_type == *object_type => Some(*object_id),
            _ => None,
        })
        .collect::<Vec<_>>()
        .first()
        .ok_or(anyhow::Error::msg("failed to get validator cap object id"))?;

    let validator_commission_cap_type = StructTag {
        address: ika_system_package_id.into(),
        module: VALIDATOR_CAP_MODULE_NAME.into(),
        name: VALIDATOR_COMMISSION_STRUCT_NAME.into(),
        type_params: vec![],
    };

    let validator_commission_cap_id = *object_changes
        .iter()
        .filter_map(|o| match o {
            ObjectChange::Created {
                object_id,
                object_type,
                ..
            } if validator_commission_cap_type == *object_type => Some(*object_id),
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

    Ok((
        response,
        BecomeCandidateValidatorData {
            validator_id: validator_cap.validator_id,
            validator_cap_id,
            validator_operation_cap_id,
            validator_commission_cap_id,
        },
    ))
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

/// Request to remove a validator candidate transaction
pub async fn request_remove_validator_candidate(
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
        REQUEST_REMOVE_VALIDATOR_CANDIDATE_FUNCTION_NAME,
        call_args,
        ika_system_object_id,
        ika_system_package_id,
        &mut ptb,
    )
    .await?;

    let tx_data = construct_unsigned_txn(context, sender, gas_budget, ptb).await?;

    execute_transaction(context, tx_data).await
}

/// Set next commission rate for a validator
pub async fn set_next_commission(
    context: &mut WalletContext,
    ika_system_package_id: ObjectID,
    ika_system_object_id: ObjectID,
    validator_operation_cap_id: ObjectID,
    new_commission_rate: u16,
    gas_budget: u64,
) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let client = context.get_client().await?;
    let validator_operation_cap_ref = client
        .transaction_builder()
        .get_object_ref(validator_operation_cap_id)
        .await?;

    let mut ptb = ProgrammableTransactionBuilder::new();
    let new_commission_rate = ptb.input(CallArg::Pure(bcs::to_bytes(&new_commission_rate)?))?;
    let call_args = vec![
        new_commission_rate,
        ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(
            validator_operation_cap_ref,
        )))?,
    ];

    let sender = context.active_address()?;

    add_ika_system_command_to_ptb(
        context,
        SET_NEXT_COMMISSION_FUNCTION_NAME,
        call_args,
        ika_system_object_id,
        ika_system_package_id,
        &mut ptb,
    )
    .await?;

    let tx_data = construct_unsigned_txn(context, sender, gas_budget, ptb).await?;

    execute_transaction(context, tx_data).await
}

/// Withdraw stake from a validator's staking pool
pub async fn withdraw_stake(
    context: &mut WalletContext,
    ika_system_package_id: ObjectID,
    ika_system_object_id: ObjectID,
    staked_ika_id: ObjectID,
    gas_budget: u64,
) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let client = context.get_client().await?;
    let staked_ika_ref = client
        .transaction_builder()
        .get_object_ref(staked_ika_id)
        .await?;

    let mut ptb = ProgrammableTransactionBuilder::new();
    let call_args = vec![ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(staked_ika_ref)))?];

    let sender = context.active_address()?;

    add_ika_system_command_to_ptb(
        context,
        WITHDRAW_STAKE_FUNCTION_NAME,
        call_args,
        ika_system_object_id,
        ika_system_package_id,
        &mut ptb,
    )
    .await?;

    ptb.transfer_args(sender, vec![Argument::NestedResult(0, 0)]);

    let tx_data = construct_unsigned_txn(context, sender, gas_budget, ptb).await?;

    execute_transaction(context, tx_data).await
}

/// Request to withdraw stake from a validator's staking pool
pub async fn request_withdraw_stake(
    context: &mut WalletContext,
    ika_system_package_id: ObjectID,
    ika_system_object_id: ObjectID,
    staked_ika_id: ObjectID,
    gas_budget: u64,
) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let client = context.get_client().await?;
    let staked_ika_ref = client
        .transaction_builder()
        .get_object_ref(staked_ika_id)
        .await?;

    let mut ptb = ProgrammableTransactionBuilder::new();
    let call_args = vec![ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(staked_ika_ref)))?];

    let sender = context.active_address()?;

    add_ika_system_command_to_ptb(
        context,
        REQUEST_WITHDRAW_STAKE_FUNCTION_NAME,
        call_args,
        ika_system_object_id,
        ika_system_package_id,
        &mut ptb,
    )
    .await?;

    let tx_data = construct_unsigned_txn(context, sender, gas_budget, ptb).await?;

    execute_transaction(context, tx_data).await
}

/// Report a validator as a bad or non-performant actor
pub async fn report_validator(
    context: &mut WalletContext,
    ika_system_package_id: ObjectID,
    ika_system_object_id: ObjectID,
    validator_operation_cap_id: ObjectID,
    reportee_id: ObjectID,
    gas_budget: u64,
) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let client = context.get_client().await?;
    let validator_operation_cap_ref = client
        .transaction_builder()
        .get_object_ref(validator_operation_cap_id)
        .await?;

    let mut ptb = ProgrammableTransactionBuilder::new();
    let reportee_id = ptb.input(CallArg::Pure(bcs::to_bytes(&reportee_id)?))?;
    let call_args = vec![
        ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(
            validator_operation_cap_ref,
        )))?,
        reportee_id,
    ];

    let sender = context.active_address()?;

    add_ika_system_command_to_ptb(
        context,
        REPORT_VALIDATOR_FUNCTION_NAME,
        call_args,
        ika_system_object_id,
        ika_system_package_id,
        &mut ptb,
    )
    .await?;

    let tx_data = construct_unsigned_txn(context, sender, gas_budget, ptb).await?;

    execute_transaction(context, tx_data).await
}

/// Undo a report_validator action
pub async fn undo_report_validator(
    context: &mut WalletContext,
    ika_system_package_id: ObjectID,
    ika_system_object_id: ObjectID,
    validator_operation_cap_id: ObjectID,
    reportee_id: ObjectID,
    gas_budget: u64,
) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let client = context.get_client().await?;
    let validator_operation_cap_ref = client
        .transaction_builder()
        .get_object_ref(validator_operation_cap_id)
        .await?;

    let mut ptb = ProgrammableTransactionBuilder::new();
    let reportee_id = ptb.input(CallArg::Pure(bcs::to_bytes(&reportee_id)?))?;
    let call_args = vec![
        ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(
            validator_operation_cap_ref,
        )))?,
        reportee_id,
    ];

    let sender = context.active_address()?;

    add_ika_system_command_to_ptb(
        context,
        UNDO_REPORT_VALIDATOR_FUNCTION_NAME,
        call_args,
        ika_system_object_id,
        ika_system_package_id,
        &mut ptb,
    )
    .await?;

    let tx_data = construct_unsigned_txn(context, sender, gas_budget, ptb).await?;

    execute_transaction(context, tx_data).await
}

/// Rotate operation cap for a validator
pub async fn rotate_operation_cap(
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
        ROTATE_OPERATION_CAP_FUNCTION_NAME,
        call_args,
        ika_system_object_id,
        ika_system_package_id,
        &mut ptb,
    )
    .await?;

    ptb.transfer_args(sender, vec![Argument::NestedResult(0, 0)]);

    let tx_data = construct_unsigned_txn(context, sender, gas_budget, ptb).await?;

    execute_transaction(context, tx_data).await
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
) -> anyhow::Result<Argument> {
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

    let return_arg = ptb.command(sui_types::transaction::Command::move_call(
        ika_system_package_id,
        SYSTEM_MODULE_NAME.into(),
        function.to_owned(),
        vec![],
        args,
    ));
    Ok(return_arg)
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
    let gas_budget = sui::client_commands::estimate_gas_budget(
        context,
        sender,
        tx_kind,
        gas_price,
        vec![],
        None,
    )
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

/// Rotate commission cap for a validator
pub async fn rotate_commission_cap(
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
        ROTATE_COMMISSION_CAP_FUNCTION_NAME,
        call_args,
        ika_system_object_id,
        ika_system_package_id,
        &mut ptb,
    )
    .await?;

    ptb.transfer_args(sender, vec![Argument::NestedResult(0, 0)]);

    let tx_data = construct_unsigned_txn(context, sender, gas_budget, ptb).await?;

    execute_transaction(context, tx_data).await
}

/// Collect commission from a validator
pub async fn collect_commission(
    context: &mut WalletContext,
    ika_system_package_id: ObjectID,
    ika_system_object_id: ObjectID,
    validator_commission_cap_id: ObjectID,
    amount: Option<u64>,
    gas_budget: u64,
) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let client = context.get_client().await?;
    let validator_commission_cap_ref = client
        .transaction_builder()
        .get_object_ref(validator_commission_cap_id)
        .await?;

    let mut ptb = ProgrammableTransactionBuilder::new();
    let amount = ptb.input(CallArg::Pure(bcs::to_bytes(&amount)?))?;
    let call_args = vec![
        ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(
            validator_commission_cap_ref,
        )))?,
        amount,
    ];

    let sender = context.active_address()?;

    add_ika_system_command_to_ptb(
        context,
        COLLECT_COMMISSION_FUNCTION_NAME,
        call_args,
        ika_system_object_id,
        ika_system_package_id,
        &mut ptb,
    )
    .await?;

    ptb.transfer_args(sender, vec![Argument::NestedResult(0, 0)]);

    let tx_data = construct_unsigned_txn(context, sender, gas_budget, ptb).await?;

    execute_transaction(context, tx_data).await
}

/// Set validator name
pub async fn set_validator_name(
    context: &mut WalletContext,
    ika_system_package_id: ObjectID,
    ika_system_object_id: ObjectID,
    validator_operation_cap_id: ObjectID,
    name: String,
    gas_budget: u64,
) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let client = context.get_client().await?;
    let validator_operation_cap_ref = client
        .transaction_builder()
        .get_object_ref(validator_operation_cap_id)
        .await?;

    let mut ptb = ProgrammableTransactionBuilder::new();
    let name = ptb.input(CallArg::Pure(bcs::to_bytes(&name)?))?;
    let call_args = vec![
        name,
        ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(
            validator_operation_cap_ref,
        )))?,
    ];

    let sender = context.active_address()?;

    add_ika_system_command_to_ptb(
        context,
        SET_VALIDATOR_NAME_FUNCTION_NAME,
        call_args,
        ika_system_object_id,
        ika_system_package_id,
        &mut ptb,
    )
    .await?;

    let tx_data = construct_unsigned_txn(context, sender, gas_budget, ptb).await?;

    execute_transaction(context, tx_data).await
}

/// Get validator metadata
pub async fn validator_metadata(
    context: &mut WalletContext,
    ika_system_package_id: ObjectID,
    ika_system_object_id: ObjectID,
    validator_id: ObjectID,
    gas_budget: u64,
) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let mut ptb = ProgrammableTransactionBuilder::new();
    let validator_id = ptb.input(CallArg::Pure(bcs::to_bytes(&validator_id)?))?;
    let call_args = vec![validator_id];

    let sender = context.active_address()?;

    add_ika_system_command_to_ptb(
        context,
        VALIDATOR_METADATA_FUNCTION_NAME,
        call_args,
        ika_system_object_id,
        ika_system_package_id,
        &mut ptb,
    )
    .await?;

    let tx_data = construct_unsigned_txn(context, sender, gas_budget, ptb).await?;

    execute_transaction(context, tx_data).await
}

/// Set validator metadata
pub async fn set_validator_metadata(
    context: &mut WalletContext,
    ika_system_package_id: ObjectID,
    ika_system_object_id: ObjectID,
    validator_operation_cap_id: ObjectID,
    metadata: String,
    gas_budget: u64,
) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let client = context.get_client().await?;
    let validator_operation_cap_ref = client
        .transaction_builder()
        .get_object_ref(validator_operation_cap_id)
        .await?;

    let mut ptb = ProgrammableTransactionBuilder::new();
    let metadata = ptb.input(CallArg::Pure(bcs::to_bytes(&metadata)?))?;
    let call_args = vec![
        ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(
            validator_operation_cap_ref,
        )))?,
        metadata,
    ];

    let sender = context.active_address()?;

    add_ika_system_command_to_ptb(
        context,
        SET_VALIDATOR_METADATA_FUNCTION_NAME,
        call_args,
        ika_system_object_id,
        ika_system_package_id,
        &mut ptb,
    )
    .await?;

    let tx_data = construct_unsigned_txn(context, sender, gas_budget, ptb).await?;

    execute_transaction(context, tx_data).await
}

/// Set next epoch network address
pub async fn set_next_epoch_network_address(
    context: &mut WalletContext,
    ika_system_package_id: ObjectID,
    ika_system_object_id: ObjectID,
    validator_operation_cap_id: ObjectID,
    network_address: String,
    gas_budget: u64,
) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let client = context.get_client().await?;
    let validator_operation_cap_ref = client
        .transaction_builder()
        .get_object_ref(validator_operation_cap_id)
        .await?;

    let mut ptb = ProgrammableTransactionBuilder::new();
    let network_address = ptb.input(CallArg::Pure(bcs::to_bytes(&network_address)?))?;
    let call_args = vec![
        network_address,
        ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(
            validator_operation_cap_ref,
        )))?,
    ];

    let sender = context.active_address()?;

    add_ika_system_command_to_ptb(
        context,
        SET_NEXT_EPOCH_NETWORK_ADDRESS_FUNCTION_NAME,
        call_args,
        ika_system_object_id,
        ika_system_package_id,
        &mut ptb,
    )
    .await?;

    let tx_data = construct_unsigned_txn(context, sender, gas_budget, ptb).await?;

    execute_transaction(context, tx_data).await
}

/// Set next epoch p2p address
pub async fn set_next_epoch_p2p_address(
    context: &mut WalletContext,
    ika_system_package_id: ObjectID,
    ika_system_object_id: ObjectID,
    validator_operation_cap_id: ObjectID,
    p2p_address: String,
    gas_budget: u64,
) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let client = context.get_client().await?;
    let validator_operation_cap_ref = client
        .transaction_builder()
        .get_object_ref(validator_operation_cap_id)
        .await?;

    let mut ptb = ProgrammableTransactionBuilder::new();
    let p2p_address = ptb.input(CallArg::Pure(bcs::to_bytes(&p2p_address)?))?;
    let call_args = vec![
        p2p_address,
        ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(
            validator_operation_cap_ref,
        )))?,
    ];

    let sender = context.active_address()?;

    add_ika_system_command_to_ptb(
        context,
        SET_NEXT_EPOCH_P2P_ADDRESS_FUNCTION_NAME,
        call_args,
        ika_system_object_id,
        ika_system_package_id,
        &mut ptb,
    )
    .await?;

    let tx_data = construct_unsigned_txn(context, sender, gas_budget, ptb).await?;

    execute_transaction(context, tx_data).await
}

/// Set next epoch consensus address
pub async fn set_next_epoch_consensus_address(
    context: &mut WalletContext,
    ika_system_package_id: ObjectID,
    ika_system_object_id: ObjectID,
    validator_operation_cap_id: ObjectID,
    consensus_address: String,
    gas_budget: u64,
) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let client = context.get_client().await?;
    let validator_operation_cap_ref = client
        .transaction_builder()
        .get_object_ref(validator_operation_cap_id)
        .await?;

    let mut ptb = ProgrammableTransactionBuilder::new();
    let consensus_address = ptb.input(CallArg::Pure(bcs::to_bytes(&consensus_address)?))?;
    let call_args = vec![
        consensus_address,
        ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(
            validator_operation_cap_ref,
        )))?,
    ];

    let sender = context.active_address()?;

    add_ika_system_command_to_ptb(
        context,
        SET_NEXT_EPOCH_CONSENSUS_ADDRESS_FUNCTION_NAME,
        call_args,
        ika_system_object_id,
        ika_system_package_id,
        &mut ptb,
    )
    .await?;

    let tx_data = construct_unsigned_txn(context, sender, gas_budget, ptb).await?;

    execute_transaction(context, tx_data).await
}

/// Set next epoch protocol pubkey bytes
pub async fn set_next_epoch_protocol_pubkey_bytes(
    context: &mut WalletContext,
    ika_system_package_id: ObjectID,
    ika_system_object_id: ObjectID,
    validator_operation_cap_id: ObjectID,
    protocol_pubkey: Vec<u8>,
    proof_of_possession_bytes: Vec<u8>,
    gas_budget: u64,
) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let client = context.get_client().await?;
    let validator_operation_cap_ref = client
        .transaction_builder()
        .get_object_ref(validator_operation_cap_id)
        .await?;

    let mut ptb = ProgrammableTransactionBuilder::new();
    let protocol_pubkey = ptb.input(CallArg::Pure(bcs::to_bytes(&protocol_pubkey)?))?;
    let proof_of_possession_bytes =
        ptb.input(CallArg::Pure(bcs::to_bytes(&proof_of_possession_bytes)?))?;
    let call_args = vec![
        protocol_pubkey,
        proof_of_possession_bytes,
        ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(
            validator_operation_cap_ref,
        )))?,
    ];

    let sender = context.active_address()?;

    add_ika_system_command_to_ptb(
        context,
        SET_NEXT_EPOCH_PROTOCOL_PUBKEY_BYTES_FUNCTION_NAME,
        call_args,
        ika_system_object_id,
        ika_system_package_id,
        &mut ptb,
    )
    .await?;

    let tx_data = construct_unsigned_txn(context, sender, gas_budget, ptb).await?;

    execute_transaction(context, tx_data).await
}

/// Set next epoch network pubkey bytes
pub async fn set_next_epoch_network_pubkey_bytes(
    context: &mut WalletContext,
    ika_system_package_id: ObjectID,
    ika_system_object_id: ObjectID,
    validator_operation_cap_id: ObjectID,
    network_pubkey: Vec<u8>,
    gas_budget: u64,
) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let client = context.get_client().await?;
    let validator_operation_cap_ref = client
        .transaction_builder()
        .get_object_ref(validator_operation_cap_id)
        .await?;

    let mut ptb = ProgrammableTransactionBuilder::new();
    let network_pubkey = ptb.input(CallArg::Pure(bcs::to_bytes(&network_pubkey)?))?;
    let call_args = vec![
        network_pubkey,
        ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(
            validator_operation_cap_ref,
        )))?,
    ];

    let sender = context.active_address()?;

    add_ika_system_command_to_ptb(
        context,
        SET_NEXT_EPOCH_NETWORK_PUBKEY_BYTES_FUNCTION_NAME,
        call_args,
        ika_system_object_id,
        ika_system_package_id,
        &mut ptb,
    )
    .await?;

    let tx_data = construct_unsigned_txn(context, sender, gas_budget, ptb).await?;

    execute_transaction(context, tx_data).await
}

/// Set next epoch consensus pubkey bytes
pub async fn set_next_epoch_consensus_pubkey_bytes(
    context: &mut WalletContext,
    ika_system_package_id: ObjectID,
    ika_system_object_id: ObjectID,
    validator_operation_cap_id: ObjectID,
    consensus_pubkey_bytes: Vec<u8>,
    gas_budget: u64,
) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let client = context.get_client().await?;
    let validator_operation_cap_ref = client
        .transaction_builder()
        .get_object_ref(validator_operation_cap_id)
        .await?;

    let mut ptb = ProgrammableTransactionBuilder::new();
    let consensus_pubkey_bytes =
        ptb.input(CallArg::Pure(bcs::to_bytes(&consensus_pubkey_bytes)?))?;
    let call_args = vec![
        consensus_pubkey_bytes,
        ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(
            validator_operation_cap_ref,
        )))?,
    ];

    let sender = context.active_address()?;

    add_ika_system_command_to_ptb(
        context,
        SET_NEXT_EPOCH_CONSENSUS_PUBKEY_BYTES_FUNCTION_NAME,
        call_args,
        ika_system_object_id,
        ika_system_package_id,
        &mut ptb,
    )
    .await?;

    let tx_data = construct_unsigned_txn(context, sender, gas_budget, ptb).await?;

    execute_transaction(context, tx_data).await
}

/// Verify validator cap
pub async fn verify_validator_cap(
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
        VERIFY_VALIDATOR_CAP_FUNCTION_NAME,
        call_args,
        ika_system_object_id,
        ika_system_package_id,
        &mut ptb,
    )
    .await?;

    let tx_data = construct_unsigned_txn(context, sender, gas_budget, ptb).await?;

    execute_transaction(context, tx_data).await
}

/// Verify operation cap
pub async fn verify_operation_cap(
    context: &mut WalletContext,
    ika_system_package_id: ObjectID,
    ika_system_object_id: ObjectID,
    validator_operation_cap_id: ObjectID,
    gas_budget: u64,
) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let client = context.get_client().await?;
    let validator_operation_cap_ref = client
        .transaction_builder()
        .get_object_ref(validator_operation_cap_id)
        .await?;

    let mut ptb = ProgrammableTransactionBuilder::new();
    let call_args = vec![ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(
        validator_operation_cap_ref,
    )))?];

    let sender = context.active_address()?;

    add_ika_system_command_to_ptb(
        context,
        VERIFY_OPERATION_CAP_FUNCTION_NAME,
        call_args,
        ika_system_object_id,
        ika_system_package_id,
        &mut ptb,
    )
    .await?;

    let tx_data = construct_unsigned_txn(context, sender, gas_budget, ptb).await?;

    execute_transaction(context, tx_data).await
}

/// Verify commission cap
pub async fn verify_commission_cap(
    context: &mut WalletContext,
    ika_system_package_id: ObjectID,
    ika_system_object_id: ObjectID,
    validator_commission_cap_id: ObjectID,
    gas_budget: u64,
) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let client = context.get_client().await?;
    let validator_commission_cap_ref = client
        .transaction_builder()
        .get_object_ref(validator_commission_cap_id)
        .await?;

    let mut ptb = ProgrammableTransactionBuilder::new();
    let call_args = vec![ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(
        validator_commission_cap_ref,
    )))?];

    let sender = context.active_address()?;

    add_ika_system_command_to_ptb(
        context,
        VERIFY_COMMISSION_CAP_FUNCTION_NAME,
        call_args,
        ika_system_object_id,
        ika_system_package_id,
        &mut ptb,
    )
    .await?;

    let tx_data = construct_unsigned_txn(context, sender, gas_budget, ptb).await?;

    execute_transaction(context, tx_data).await
}

/// Set next epoch class groups pubkey and proof bytes
pub async fn set_next_epoch_class_groups_pubkey_and_proof_bytes(
    context: &mut WalletContext,
    ika_system_package_id: ObjectID,
    ika_system_object_id: ObjectID,
    ika_common_package_id: ObjectID,
    validator_operation_cap_id: ObjectID,
    class_groups_pubkey_and_proof_obj_ref: ObjectRef,
    gas_budget: u64,
) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let client = context.get_client().await?;
    let validator_operation_cap_ref = client
        .transaction_builder()
        .get_object_ref(validator_operation_cap_id)
        .await?;

    let mut ptb = ProgrammableTransactionBuilder::new();
    let call_args = vec![
        ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(
            class_groups_pubkey_and_proof_obj_ref,
        )))?,
        ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(
            validator_operation_cap_ref,
        )))?,
    ];

    let sender = context.active_address()?;

    let optional_tablevec_to_delete = add_ika_system_command_to_ptb(
        context,
        SET_NEXT_EPOCH_CLASS_GROUPS_PUBKEY_AND_PROOF_BYTES_FUNCTION_NAME,
        call_args,
        ika_system_object_id,
        ika_system_package_id,
        &mut ptb,
    )
    .await?;

    ptb.command(Command::move_call(
        ika_common_package_id,
        CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_MODULE_NAME.into(),
        DROP_OPRIONAL_TABLE_VEC_FUNC_NAME.to_owned(),
        vec![],
        vec![optional_tablevec_to_delete],
    ));

    let tx_data = construct_unsigned_txn(context, sender, gas_budget, ptb).await?;

    execute_transaction(context, tx_data).await
}

/// Set pricing vote for DWallet operations
pub async fn set_pricing_vote(
    context: &mut WalletContext,
    ika_system_package_id: ObjectID,
    ika_system_object_id: ObjectID,
    ika_dwallet_2pc_mpc_package_id: ObjectID,
    ika_dwallet_2pc_mpc_coordinator_object_id: ObjectID,
    validator_operation_cap_id: ObjectID,
    new_value: Vec<Entry<PricingInfoKey, PricingInfoValue>>,
    gas_budget: u64,
) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let mut ptb = ProgrammableTransactionBuilder::new();
    let client = context.get_client().await?;
    let validator_operation_cap_ref = client
        .transaction_builder()
        .get_object_ref(validator_operation_cap_id)
        .await?;

    let call_args = vec![ptb.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(
        validator_operation_cap_ref,
    )))?];

    let verified_validator_operation_cap = add_ika_system_command_to_ptb(
        context,
        VERIFY_OPERATION_CAP_FUNCTION_NAME,
        call_args,
        ika_system_object_id,
        ika_system_package_id,
        &mut ptb,
    )
    .await?;

    let sender = context.active_address()?;

    let dwallet_2pc_mpc_coordinator = ptb.input(
        get_dwallet_2pc_mpc_coordinator_call_arg(
            context,
            ika_dwallet_2pc_mpc_coordinator_object_id,
        )
        .await?,
    )?;

    let pricing_info = ptb.command(Command::move_call(
        ika_dwallet_2pc_mpc_package_id,
        ident_str!("pricing").into(),
        ident_str!("empty").into(),
        vec![],
        vec![],
    ));
    let none_bcs = bcs::to_bytes(&None::<u32>)?;

    for entry in new_value {
        let curve = ptb.input(CallArg::Pure(bcs::to_bytes(&entry.key.curve)?))?;
        let signature_algo_bcs = match &entry.key.signature_algorithm {
            None => none_bcs.clone(),
            Some(signature_algo) => bcs::to_bytes(&Some(*signature_algo))?,
        };
        let signature_algo = ptb.input(CallArg::Pure(signature_algo_bcs))?;
        let protocol = ptb.input(CallArg::Pure(bcs::to_bytes(&entry.key.protocol)?))?;
        let fee_ika = ptb.input(CallArg::Pure(bcs::to_bytes(&entry.value.fee_ika)?))?;
        let gas_fee_reimbursement_sui = ptb.input(CallArg::Pure(bcs::to_bytes(
            &entry.value.gas_fee_reimbursement_sui,
        )?))?;
        let gas_fee_reimbursement_sui_for_system_calls = ptb.input(CallArg::Pure(
            bcs::to_bytes(&entry.value.gas_fee_reimbursement_sui_for_system_calls)?,
        ))?;
        let args = vec![
            pricing_info,
            curve,
            signature_algo,
            protocol,
            fee_ika,
            gas_fee_reimbursement_sui,
            gas_fee_reimbursement_sui_for_system_calls,
        ];
        ptb.command(Command::move_call(
            ika_dwallet_2pc_mpc_package_id,
            ident_str!("pricing").into(),
            ident_str!("insert_or_update_pricing").into(),
            vec![],
            args,
        ));
    }

    let args = vec![
        dwallet_2pc_mpc_coordinator,
        pricing_info,
        verified_validator_operation_cap,
    ];
    ptb.command(Command::move_call(
        ika_dwallet_2pc_mpc_package_id,
        DWALLET_2PC_MPC_COORDINATOR_MODULE_NAME.into(),
        SET_PRICING_VOTE_FUNCTION_NAME.to_owned(),
        vec![],
        args,
    ));

    let tx_data = construct_unsigned_txn(context, sender, gas_budget, ptb).await?;

    execute_transaction(context, tx_data).await
}

async fn get_dwallet_2pc_mpc_coordinator_call_arg(
    context: &mut WalletContext,
    ika_dwallet_2pc_mpc_coordinator_object_id: ObjectID,
) -> anyhow::Result<CallArg> {
    let Some(Owner::Shared {
        initial_shared_version,
    }) = context
        .get_client()
        .await?
        .read_api()
        .get_object_with_options(
            ika_dwallet_2pc_mpc_coordinator_object_id,
            SuiObjectDataOptions::new().with_owner(),
        )
        .await?
        .data
        .ok_or(anyhow::Error::msg("failed to get object data"))?
        .owner
    else {
        bail!("Failed to get owner of object")
    };

    Ok(CallArg::Object(ObjectArg::SharedObject {
        id: ika_dwallet_2pc_mpc_coordinator_object_id,
        initial_shared_version,
        mutable: true,
    }))
}
