// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! The IkaBridgeStatus observable monitors whether the Ika Bridge is paused.

use crate::Observable;
use async_trait::async_trait;
use prometheus::IntGauge;
use std::sync::Arc;
use ika_bridge::ika_client::IkaBridgeClient;

use tokio::time::Duration;
use tracing::{error, info};

pub struct IkaBridgeStatus {
    ika_client: Arc<IkaBridgeClient>,
    metric: IntGauge,
}

impl IkaBridgeStatus {
    pub fn new(ika_client: Arc<IkaBridgeClient>, metric: IntGauge) -> Self {
        Self { ika_client, metric }
    }
}

#[async_trait]
impl Observable for IkaBridgeStatus {
    fn name(&self) -> &str {
        "IkaBridgeStatus"
    }

    async fn observe_and_report(&self) {
        let status = self.ika_client.is_bridge_paused().await;
        match status {
            Ok(status) => {
                self.metric.set(status as i64);
                info!("Ika Bridge Status: {:?}", status);
            }
            Err(e) => {
                error!("Error getting ika bridge status: {:?}", e);
            }
        }
    }

    fn interval(&self) -> Duration {
        Duration::from_secs(2)
    }
}
