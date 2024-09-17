// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::net::SocketAddr;
use std::sync::Arc;

use axum::routing::post;
use axum::{Extension, Router};
use once_cell::sync::Lazy;
use tracing::info;

use pera_sdk::PeraClient;

use crate::errors::Error;
use crate::state::{CheckpointBlockProvider, OnlineServerContext};
use crate::types::{Currency, PeraEnv};

/// This lib implements the Rosetta online and offline server defined by the [Rosetta API Spec](https://www.rosetta-api.org/docs/Reference.html)
mod account;
mod block;
mod construction;
mod errors;
mod network;
pub mod operations;
mod state;
pub mod types;

pub static PERA: Lazy<Currency> = Lazy::new(|| Currency {
    symbol: "PERA".to_string(),
    decimals: 9,
});

pub struct RosettaOnlineServer {
    env: PeraEnv,
    context: OnlineServerContext,
}

impl RosettaOnlineServer {
    pub fn new(env: PeraEnv, client: PeraClient) -> Self {
        let blocks = Arc::new(CheckpointBlockProvider::new(client.clone()));
        Self {
            env,
            context: OnlineServerContext::new(client, blocks),
        }
    }

    pub async fn serve(self, addr: SocketAddr) {
        // Online endpoints
        let app = Router::new()
            .route("/account/balance", post(account::balance))
            .route("/account/coins", post(account::coins))
            .route("/block", post(block::block))
            .route("/block/transaction", post(block::transaction))
            .route("/construction/submit", post(construction::submit))
            .route("/construction/metadata", post(construction::metadata))
            .route("/network/status", post(network::status))
            .route("/network/list", post(network::list))
            .route("/network/options", post(network::options))
            .layer(Extension(self.env))
            .with_state(self.context);

        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

        info!(
            "Pera Rosetta online server listening on {}",
            listener.local_addr().unwrap()
        );
        axum::serve(listener, app).await.unwrap();
    }
}

pub struct RosettaOfflineServer {
    env: PeraEnv,
}

impl RosettaOfflineServer {
    pub fn new(env: PeraEnv) -> Self {
        Self { env }
    }

    pub async fn serve(self, addr: SocketAddr) {
        // Online endpoints
        let app = Router::new()
            .route("/construction/derive", post(construction::derive))
            .route("/construction/payloads", post(construction::payloads))
            .route("/construction/combine", post(construction::combine))
            .route("/construction/preprocess", post(construction::preprocess))
            .route("/construction/hash", post(construction::hash))
            .route("/construction/parse", post(construction::parse))
            .route("/network/list", post(network::list))
            .route("/network/options", post(network::options))
            .layer(Extension(self.env));
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

        info!(
            "Pera Rosetta offline server listening on {}",
            listener.local_addr().unwrap()
        );
        axum::serve(listener, app).await.unwrap();
    }
}
