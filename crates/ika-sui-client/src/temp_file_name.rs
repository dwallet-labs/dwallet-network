use anyhow::bail;
use dwallet_classgroups_types::ClassGroupsEncryptionKeyAndProof;
use fastcrypto::traits::ToFromBytes;
use ika_config::validator_info::ValidatorInfo;
use ika_types::sui::{
    ClassGroupsPublicKeyAndProof, ClassGroupsPublicKeyAndProofBuilder,
    REQUEST_ADD_VALIDATOR_CANDIDATE_FUNCTION_NAME, SYSTEM_MODULE_NAME,
};
use move_core_types::ident_str;
use shared_crypto::intent::Intent;
use sui::client_commands::{
    estimate_gas_budget_from_gas_cost, execute_dry_run, SuiClientCommandResult,
};
use sui_json_rpc_types::SuiTransactionBlockEffectsAPI;
use sui_json_rpc_types::{ObjectChange, SuiTransactionBlockResponse};
use sui_keys::keystore::AccountKeystore;
use sui_sdk::wallet_context::WalletContext;
use sui_sdk::SuiClient;
use sui_types::base_types::{ObjectID, ObjectRef, SequenceNumber, SuiAddress};
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::TransactionDataAPI;
use sui_types::transaction::{
    Argument, CallArg, ObjectArg, SenderSignedData, Transaction, TransactionKind,
};

async fn create_class_groups_public_key_and_proof_builder_object(
    publisher_address: SuiAddress,
    context: &mut WalletContext,
    client: &SuiClient,
    ika_system_package_id: ObjectID,
) -> anyhow::Result<ObjectRef> {
    let mut ptb = ProgrammableTransactionBuilder::new();
    ptb.move_call(
        ika_system_package_id,
        ident_str!("class_groups_public_key_and_proof").into(),
        ident_str!("empty").into(),
        vec![],
        vec![],
    )?;
    ptb.transfer_arg(publisher_address, Argument::Result(0));
    let tx_kind = TransactionKind::ProgrammableTransaction(ptb.finish());

    let response = execute_sui_transaction(publisher_address, tx_kind, context).await?;

    let object_changes = response.object_changes.unwrap();

    let builder_id = object_changes
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
        .unwrap()
        .clone();

    let builder_ref = client
        .transaction_builder()
        .get_object_ref(builder_id)
        .await?;

    Ok(builder_ref)
}

pub async fn create_class_groups_public_key_and_proof_object(
    publisher_address: SuiAddress,
    context: &mut WalletContext,
    ika_system_package_id: ObjectID,
    class_groups_public_key_and_proof_bytes: Vec<u8>,
) -> anyhow::Result<ObjectRef> {
    let client = context.get_client().await?;
    let mut builder_object_ref = create_class_groups_public_key_and_proof_builder_object(
        publisher_address,
        context,
        &client,
        ika_system_package_id,
    )
    .await?;

    let class_groups_public_key_and_proof: Box<ClassGroupsEncryptionKeyAndProof> =
        Box::new(bcs::from_bytes(&class_groups_public_key_and_proof_bytes)?);
    for pubkey_and_proof in class_groups_public_key_and_proof.iter() {
        let mut ptb = ProgrammableTransactionBuilder::new();
        let pubkey_and_proof = bcs::to_bytes(pubkey_and_proof)?;
        ptb.move_call(
            ika_system_package_id,
            ident_str!("class_groups_public_key_and_proof").into(),
            ident_str!("add_public_key_and_proof").into(),
            vec![],
            vec![
                CallArg::Object(ObjectArg::ImmOrOwnedObject(builder_object_ref)),
                /// Sui limits the size of a single call argument to 16KB.
                CallArg::Pure(bcs::to_bytes(&pubkey_and_proof[0..10_000])?),
                CallArg::Pure(bcs::to_bytes(&pubkey_and_proof[10_000..])?),
            ],
        )?;
        let tx_kind = TransactionKind::ProgrammableTransaction(ptb.finish());

        let response = execute_sui_transaction(publisher_address, tx_kind, context).await?;
        let object_changes = response
            .object_changes
            .clone()
            .ok_or(anyhow::Error::msg("Failed to get object changes"))?;
        let builder_id = object_changes
            .iter()
            .filter_map(|o| match o {
                ObjectChange::Mutated {
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
            .unwrap()
            .clone();

        builder_object_ref = client
            .transaction_builder()
            .get_object_ref(builder_id)
            .await?;
    }

    let mut ptb = ProgrammableTransactionBuilder::new();
    ptb.move_call(
        ika_system_package_id,
        ident_str!("class_groups_public_key_and_proof").into(),
        ident_str!("finish").into(),
        vec![],
        vec![CallArg::Object(ObjectArg::ImmOrOwnedObject(
            builder_object_ref,
        ))],
    )?;
    ptb.transfer_arg(publisher_address, Argument::Result(0));
    let tx_kind = TransactionKind::ProgrammableTransaction(ptb.finish());

    let response = execute_sui_transaction(publisher_address, tx_kind, context).await?;

    let object_changes = response
        .object_changes
        .ok_or(anyhow::Error::msg("Failed to get object changes"))?;

    let obj_id = object_changes
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
        .unwrap()
        .clone();

    let pubkey_and_proof_obj_ref = client.transaction_builder().get_object_ref(obj_id).await?;

    Ok(pubkey_and_proof_obj_ref)
}

pub async fn create_sui_transaction(
    signer: SuiAddress,
    tx_kind: TransactionKind,
    context: &mut WalletContext,
) -> Result<Transaction, anyhow::Error> {
    let gas_price = context.get_reference_gas_price().await?;

    let client = context.get_client().await?;

    //let gas_budget = max_gas_budget(&client).await?;
    let gas_budget =
        estimate_gas_budget(context, signer, tx_kind.clone(), gas_price, None, None).await?;

    let tx_data = client
        .transaction_builder()
        .tx_data(signer, tx_kind, gas_budget, gas_price, vec![], None)
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

pub async fn execute_sui_transaction(
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

pub async fn request_add_validator_candidate(
    validator_address: SuiAddress,
    context: &mut WalletContext,
    validator_initialization_metadata: &ValidatorInfo,
    ika_system_package_id: ObjectID,
    system_id: ObjectID,
    init_system_shared_version: SequenceNumber,
    class_groups_pubkey_and_proof_obj_ref: ObjectRef,
) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let mut ptb = ProgrammableTransactionBuilder::new();

    ptb.move_call(
        ika_system_package_id,
        SYSTEM_MODULE_NAME.into(),
        REQUEST_ADD_VALIDATOR_CANDIDATE_FUNCTION_NAME.into(),
        vec![],
        vec![
            CallArg::Object(ObjectArg::SharedObject {
                id: system_id,
                initial_shared_version: init_system_shared_version,
                mutable: true,
            }),
            CallArg::Pure(bcs::to_bytes(
                &validator_initialization_metadata
                    .protocol_public_key
                    .as_bytes()
                    .to_vec(),
            )?),
            CallArg::Pure(bcs::to_bytes(
                &validator_initialization_metadata
                    .network_public_key
                    .as_bytes()
                    .to_vec(),
            )?),
            CallArg::Pure(bcs::to_bytes(
                &validator_initialization_metadata
                    .consensus_public_key
                    .as_bytes()
                    .to_vec(),
            )?),
            CallArg::Object(ObjectArg::ImmOrOwnedObject(
                class_groups_pubkey_and_proof_obj_ref,
            )),
            CallArg::Pure(bcs::to_bytes(
                &validator_initialization_metadata
                    .proof_of_possession
                    .as_ref()
                    .to_vec(),
            )?),
            CallArg::Pure(bcs::to_bytes(
                validator_initialization_metadata.name.as_bytes(),
            )?),
            CallArg::Pure(bcs::to_bytes(
                validator_initialization_metadata.name.as_bytes(),
            )?),
            CallArg::Pure(bcs::to_bytes(String::new().as_bytes())?),
            CallArg::Pure(bcs::to_bytes(String::new().as_bytes())?),
            CallArg::Pure(bcs::to_bytes(
                &validator_initialization_metadata.network_address.clone(),
            )?),
            CallArg::Pure(bcs::to_bytes(
                &validator_initialization_metadata.p2p_address.clone(),
            )?),
            CallArg::Pure(bcs::to_bytes(
                &validator_initialization_metadata.consensus_address.clone(),
            )?),
            CallArg::Pure(bcs::to_bytes(
                &validator_initialization_metadata.computation_price,
            )?),
            CallArg::Pure(bcs::to_bytes(
                &validator_initialization_metadata.commission_rate,
            )?),
        ],
    )?;

    let tx_kind = TransactionKind::ProgrammableTransaction(ptb.finish());

    execute_sui_transaction(validator_address, tx_kind, context).await
}
