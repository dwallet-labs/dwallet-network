use anyhow::{anyhow, Error};
use move_core_types::identifier::Identifier;
use sui_json_rpc_types::ObjectChange;
use sui_json_rpc_types::{SuiObjectDataOptions, SuiRawData};
use sui_sdk::wallet_context::WalletContext;
use sui_types::base_types::ObjectID;
use sui_types::object::Owner;
use sui_types::transaction::SharedInputObject;

pub async fn get_object_bcs_by_id(
    context: &mut WalletContext,
    object_id: ObjectID,
) -> Result<SuiRawData, Error> {
    let object_resp = context
        .get_client()
        .await?
        .read_api()
        .get_object_with_options(
            object_id,
            SuiObjectDataOptions::default().with_bcs().with_owner(),
        )
        .await?;

    match object_resp.data {
        Some(data) => Ok(data.bcs.ok_or_else(|| anyhow!("missing object data"))?),
        None => Err(anyhow!("could not find an object with ID: {:?}", object_id)),
    }
}

pub async fn get_shared_object_input_by_id(
    context: &mut WalletContext,
    object_id: ObjectID,
) -> Result<SharedInputObject, Error> {
    let object_resp = context
        .get_client()
        .await?
        .read_api()
        .get_object_with_options(object_id, SuiObjectDataOptions::default().with_owner())
        .await?;

    match object_resp.data {
        Some(_) => {
            let owner = object_resp
                .owner()
                .ok_or_else(|| anyhow!("missing object owner"))?;
            let initial_shared_version = match owner {
                Owner::Shared {
                    initial_shared_version,
                } => initial_shared_version,
                _ => return Err(anyhow!("object is not shared")),
            };
            Ok(SharedInputObject {
                id: object_id,
                initial_shared_version,
                mutable: true,
            })
        }
        None => Err(anyhow!("could not find an object with ID: {:?}", object_id)),
    }
}

pub fn get_object_from_transaction_changes(
    object_changes: Vec<ObjectChange>,
    module: Identifier,
    type_name: Identifier,
) -> Result<ObjectID, Error> {
    let latest_state_object_id = object_changes
        .iter()
        .find_map(|oc| {
            if let ObjectChange::Created {
                object_id,
                object_type,
                ..
            } = oc
            {
                if object_type.module == module && object_type.name == type_name {
                    return Some(object_id);
                }
            }
            None
        })
        .ok_or_else(|| anyhow!("can't get latest state object ID."))?;
    Ok(latest_state_object_id.clone())
}
