// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//! The SuiSyncer module handles synchronizing Events emitted
//! on the Sui blockchain from concerned modules of `ika_system` package.
use crate::sui_connector::metrics::SuiConnectorMetrics;
use ika_sui_client::{retry_with_max_elapsed_time, SuiClient, SuiClientInner};
use ika_types::error::IkaResult;
use ika_types::sui::SystemInner;
use mysten_metrics::spawn_logged_monitored_task;
use std::sync::Arc;
use sui_json_rpc_types::SuiEvent;
use sui_types::{event::EventID, Identifier};
use tokio::{
    sync::Notify,
    task::JoinHandle,
    time::{self, Duration},
};
use tracing::{debug, error, info};

pub struct SuiSyncer<C> {
    sui_client: Arc<SuiClient<C>>,
    // The last transaction that the syncer has fully processed.
    // Syncer will resume posting this transaction (i.e., exclusive) when it starts.
    modules: Vec<Identifier>,
    metrics: Arc<SuiConnectorMetrics>,
}

impl<C> SuiSyncer<C>
where
    C: SuiClientInner + 'static,
{
    pub fn new(
        sui_client: Arc<SuiClient<C>>,
        modules: Vec<Identifier>,
        metrics: Arc<SuiConnectorMetrics>,
    ) -> Self {
        Self {
            sui_client,
            modules,
            metrics,
        }
    }

    pub async fn run(
        self,
        query_interval: Duration,
        new_events_sender: tokio::sync::broadcast::Sender<Vec<SuiEvent>>,
    ) -> IkaResult<Vec<JoinHandle<()>>> {
        info!("Starting SuiSyncer");
        let mut task_handles = vec![];
        for module in self.modules {
            let metrics = self.metrics.clone();
            let sui_client_clone = self.sui_client.clone();
            let new_events_sender_clone = new_events_sender.clone();
            task_handles.push(spawn_logged_monitored_task!(
                Self::run_event_listening_task(
                    module,
                    sui_client_clone,
                    query_interval,
                    metrics,
                    new_events_sender_clone,
                )
            ));
        }
        Ok(task_handles)
    }

    async fn run_event_listening_task(
        // The module where interested events are defined.
        // Module is always of ika system package.
        module: Identifier,
        sui_client: Arc<SuiClient<C>>,
        query_interval: Duration,
        metrics: Arc<SuiConnectorMetrics>,
        new_events_sender: tokio::sync::broadcast::Sender<Vec<SuiEvent>>,
    ) {
        info!(?module, "Starting sui events listening task");
        let mut interval = time::interval(query_interval);
        interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

        // Create a task to update metrics
        let notify = Arc::new(Notify::new());
        let notify_clone = notify.clone();
        let sui_client_clone = sui_client.clone();
        let last_synced_sui_checkpoints_metric = metrics
            .last_synced_sui_checkpoints
            .with_label_values(&[&module.to_string()]);
        spawn_logged_monitored_task!(async move {
            loop {
                notify_clone.notified().await;
                let Ok(Ok(latest_checkpoint_sequence_number)) = retry_with_max_elapsed_time!(
                    sui_client_clone.get_latest_checkpoint_sequence_number(),
                    Duration::from_secs(120)
                ) else {
                    error!("failed to query the latest checkpoint sequence number from the sui client after retry");
                    continue;
                };
                last_synced_sui_checkpoints_metric.set(latest_checkpoint_sequence_number as i64);
            }
        });
        let mut cursor: Option<EventID> = None;
        let mut start_epoch_cursor: Option<EventID> = None;
        let mut loop_index: usize = 0;
        loop {
            // Fetching the epoch start TX digest less frequently
            // as it is unexpected to change often.
            if loop_index % 10 == 0 {
                debug!("Querying epoch start cursor from Sui");
                let SystemInner::V1(system_inner) = sui_client.must_get_system_inner_object().await;
                let Ok(epoch_start_tx_digest) = system_inner.epoch_start_tx_digest.try_into()
                else {
                    // This should not happen, but if it does, we need to know about it.
                    error!("cloud not parse `epoch_start_tx_digest` - wrong length");
                    continue;
                };
                let start_epoch_event = EventID::from((epoch_start_tx_digest, 0));
                if start_epoch_cursor != Some(start_epoch_event) {
                    start_epoch_cursor = Some(start_epoch_event);
                    cursor = start_epoch_cursor;
                }
            }
            loop_index += 1;

            interval.tick().await;
            let Ok(Ok(events)) = retry_with_max_elapsed_time!(
                sui_client.query_events_by_module(module.clone(), cursor),
                Duration::from_secs(120)
            ) else {
                error!("failed to query events from the sui client — retrying");
                continue;
            };

            let len = events.data.len();
            if len != 0 {
                if !events.has_next_page {
                    // If this is the last page, it means we have processed all
                    // events up to the latest checkpoint
                    // We can then update the latest checkpoint metric.
                    notify.notify_one();
                }
                if let Err(e) = new_events_sender.send(events.data) {
                    error!(error=?e, ?module, "failed to send new events to the channel");
                }

                if let Some(next) = events.next_cursor {
                    cursor = Some(next);
                }
                info!(
                    ?module,
                    ?cursor,
                    "Observed {len} new events from Sui network"
                );
            }
        }
    }
}
