use std::{str::FromStr, sync::Arc};

use anyhow::Result;
use axum::{extract::Query, routing::get, Json, Router};
use sui_json_rpc_types::SuiTransactionBlockResponseOptions;
use sui_sdk::SuiClientBuilder;

use anyhow::{anyhow, Ok};
use serde::{Deserialize, Serialize};

use axum::{http::StatusCode, response::IntoResponse};
use sui_rest_api::Client;
use sui_types::digests::TransactionDigest;

// const SUI_FULLNODE_URL : &str = "https://fullnode.devnet.sui.io:443";
const SUI_FULLNODE_URL: &str = "http://usw1a-tnt-rpc-0-3a5838e.testnet.sui.io:9000";

#[tokio::main]
async fn main() -> Result<()> {
    let server_url = "0.0.0.0:6920";

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let app = Router::new().route("/gettxdata", get(get_tx_data));

    println!("Starting server on address {}", server_url);

    let handle = tokio::spawn(async move {
        println!("Listening WS and HTTP on address {}", server_url);
        axum::Server::bind(&server_url.parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    tokio::join!(handle).0?;
    Ok(())
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TxDataRequest {
    pub tx_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TxDataResponse {
    pub ckp_epoch_id: u64,
    pub checkpoint_summary_bytes: Vec<u8>,
    pub checkpoint_contents_bytes: Vec<u8>,
    pub transaction_bytes: Vec<u8>,
}

pub async fn get_tx_data(payload: Query<TxDataRequest>) -> impl IntoResponse {
    let tid = TransactionDigest::from_str(&payload.tx_id).unwrap();

    // TOOD don't hardcode
    let sui_client: Arc<sui_sdk::SuiClient> = Arc::new(
        SuiClientBuilder::default()
            .build(SUI_FULLNODE_URL)
            .await
            .unwrap(),
    );

    let options = SuiTransactionBlockResponseOptions::new();
    let seq = sui_client
        .read_api()
        .get_transaction_with_options(tid, options)
        .await
        .unwrap()
        .checkpoint
        .ok_or(anyhow!("Transaction not found"))
        .unwrap();

    let rest_client: Client = Client::new(format!("{}/rest", SUI_FULLNODE_URL));
    let full_checkpoint = rest_client.get_full_checkpoint(seq).await.unwrap();

    let (matching_tx, _) = full_checkpoint
        .transactions
        .iter()
        .zip(full_checkpoint.checkpoint_contents.iter())
        // Note that we get the digest of the effects to ensure this is
        // indeed the correct effects that are authenticated in the contents.
        .find(|(tx, digest)| {
            tx.effects.execution_digests() == **digest && digest.transaction == tid
        })
        .ok_or(anyhow!("Transaction not found in checkpoint contents"))
        .unwrap();

    let res = TxDataResponse {
        ckp_epoch_id: full_checkpoint.checkpoint_summary.data().epoch,
        checkpoint_summary_bytes: bcs::to_bytes(&full_checkpoint.checkpoint_summary).unwrap(),
        checkpoint_contents_bytes: bcs::to_bytes(&full_checkpoint.checkpoint_contents).unwrap(),
        transaction_bytes: bcs::to_bytes(&matching_tx).unwrap(),
    };

    (StatusCode::OK, Json(res)).into_response()
}
