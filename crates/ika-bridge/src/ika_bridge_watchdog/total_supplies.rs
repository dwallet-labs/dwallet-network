// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! The IkaBridgeStatus observable monitors whether the Ika Bridge is paused.

use crate::ika_bridge_watchdog::Observable;
use async_trait::async_trait;
use prometheus::IntGaugeVec;
use std::{collections::BTreeMap, sync::Arc};
use ika_sdk::IkaClient;

use tokio::time::Duration;
use tracing::{error, info};

pub struct TotalSupplies {
    ika_client: Arc<IkaClient>,
    coins: BTreeMap<String, String>,
    metric: IntGaugeVec,
}

impl TotalSupplies {
    pub fn new(
        ika_client: Arc<IkaClient>,
        coins: BTreeMap<String, String>,
        metric: IntGaugeVec,
    ) -> Self {
        Self {
            ika_client,
            coins,
            metric,
        }
    }
}

#[async_trait]
impl Observable for TotalSupplies {
    fn name(&self) -> &str {
        "TotalSupplies"
    }

    async fn observe_and_report(&self) {
        for (coin_name, coin_type) in &self.coins {
            let resp = self
                .ika_client
                .coin_read_api()
                .get_total_supply(coin_type.clone())
                .await;
            match resp {
                Ok(supply) => {
                    self.metric
                        .with_label_values(&[coin_name])
                        .set(supply.value as i64);
                    info!("Total supply for {coin_type}: {}", supply.value);
                }
                Err(e) => {
                    error!("Error getting total supply for coin {coin_type}: {:?}", e);
                }
            }
        }
    }

    fn interval(&self) -> Duration {
        Duration::from_secs(10)
    }
}
