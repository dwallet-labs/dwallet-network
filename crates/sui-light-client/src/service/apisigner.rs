use anyhow::Context;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use base64::prelude::*;
use csv::Writer;

use dashmap::DashMap;
use diesel::IntoSql;
use serde::{Deserialize, Serialize};
use serde_json::json;
use shared_crypto::intent::Intent;
use sui_keys::keystore::AccountKeystore;
use sui_sdk::{
    types::{
        base_types::SuiAddress,
        crypto::EncodeDecodeBase64,
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        transaction::{
            ProgrammableTransaction, TransactionData, TransactionDataAPI, TransactionKind,
        },
    },
    SUI_COIN_TYPE,
};
use types::{
    api::RegisterWorkspaceSignerRequest,
    axum::{AeonError, AeonResponse},
};

use crate::types::AppStateRc;
use tracing::{error, info, Level};

pub async fn register(
    State(app_state): State<AppStateRc>,
    payload: Json<RegisterWorkspaceSignerRequest>,
) -> Result<AeonResponse<()>, AeonError> {
    // Check if the workspace exists
    let workspace_map = app_state
        .user_shares
        .entry(payload.workspace_id.to_string())
        .or_insert_with(DashMap::new);

    // Check if the vault already has a user share
    if workspace_map.contains_key(&payload.vault_id) {
        return Err(AeonError::BadRequest(
            "user share already exists".to_string(),
        ));
    }

    // Insert the new user share into the vault
    workspace_map.insert(
        payload.vault_id.clone(),
        payload.encrypted_organization_share.clone(),
    );

    Ok(types::axum::AeonResponse(()))
}
