use anyhow::{anyhow, Result};
use move_core_types::identifier::Identifier;

use sui_json_rpc_types::ObjectChange;
use sui_json_rpc_types::{SuiObjectDataOptions, SuiRawData};
use sui_sdk::wallet_context::WalletContext;
use sui_types::base_types::{ObjectID, ObjectRef};
use sui_types::object::Owner;
use sui_types::transaction::SharedInputObject;

/// Get a bcs serialized object by its ID.
pub async fn get_object_bcs_by_id(
    context: &mut WalletContext,
    object_id: ObjectID,
) -> Result<SuiRawData> {
    context
        .get_client()
        .await?
        .read_api()
        .get_object_with_options(
            object_id,
            SuiObjectDataOptions::default().with_bcs().with_owner(),
        )
        .await?
        .data
        .and_then(|data| data.bcs)
        .ok_or_else(|| anyhow!("could not find an object with ID: {:?}", object_id))
}

/// Retrieves a shared object by its ID for use as an input to a transaction.
///
/// This function is used when you need to send a shared object as an input to a transaction.
/// It retrieves the object and checks if it is shared, then returns the `initial_shared_version`
/// of the object.
/// The `initial_shared_version` refers to the version of the object when it was
/// first shared.
pub async fn get_shared_object_input_by_id(
    context: &mut WalletContext,
    object_id: ObjectID,
) -> Result<SharedInputObject> {
    let object_resp = context
        .get_client()
        .await?
        .read_api()
        .get_object_with_options(object_id, SuiObjectDataOptions::default().with_owner())
        .await?;

    let owner = object_resp
        .owner()
        .ok_or_else(|| anyhow!("missing object owner for an object with ID: {object_id}"))?;

    let initial_shared_version = match owner {
        Owner::Shared {
            initial_shared_version,
        } => initial_shared_version,
        _ => return Err(anyhow!("object with ID {object_id} is not shared")),
    };

    Ok(SharedInputObject {
        id: object_id,
        initial_shared_version,
        mutable: true,
    })
}

pub async fn get_object_ref_by_id(
    context: &mut WalletContext,
    object_id: ObjectID,
) -> Result<ObjectRef> {
    let object_resp = context
        .get_client()
        .await?
        .read_api()
        .get_object_with_options(object_id, SuiObjectDataOptions::default().with_owner())
        .await?
        .into_object();

    Ok(object_resp?.object_ref())
}

pub fn get_object_from_transaction_changes(
    object_changes: Vec<ObjectChange>,
    module: Identifier,
    type_name: Identifier,
) -> Result<ObjectID> {
    object_changes
        .into_iter()
        .find_map(|oc| {
            if let ObjectChange::Created {
                object_id,
                object_type,
                ..
            } = oc
            {
                if object_type.module == module && object_type.name == type_name {
                    return Some(object_id.clone());
                }
            }
            None
        })
        .ok_or_else(|| anyhow!("can't get latest state object ID."))
}
