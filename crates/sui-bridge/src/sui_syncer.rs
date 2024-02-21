// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//! The SuiSyncer module is responsible for synchronizing Events emitted on Sui blockchain from
//! concerned bridge packages.

use crate::{
    error::BridgeResult,
    retry_with_max_delay,
    sui_client::{SuiClient, SuiClientInner},
};
use mysten_metrics::spawn_logged_monitored_task;
use std::{collections::HashMap, sync::Arc};
use sui_json_rpc_types::SuiEvent;
use sui_types::{
    base_types::ObjectID, digests::TransactionDigest, Identifier, SUI_SYSTEM_PACKAGE_ID,
};
use tokio::{
    task::JoinHandle,
    time::{self, Duration},
};
use tokio_retry::strategy::{jitter, ExponentialBackoff};
use tokio_retry::Retry;

// TODO: use the right package id
const PACKAGE_ID: ObjectID = SUI_SYSTEM_PACKAGE_ID;
const SUI_EVENTS_CHANNEL_SIZE: usize = 1000;

/// Map from contract address to their start block.
pub type SuiTargetModules = HashMap<Identifier, TransactionDigest>;

pub struct SuiSyncer<C> {
    sui_client: Arc<SuiClient<C>>,
    // The last transaction that the syncer has fully processed.
    // Syncer will resume post this transaction (i.e. exclusive), when it starts.
    cursors: SuiTargetModules,
}

impl<C> SuiSyncer<C>
where
    C: SuiClientInner + 'static,
{
    pub fn new(sui_client: Arc<SuiClient<C>>, cursors: SuiTargetModules) -> Self {
        Self {
            sui_client,
            cursors,
        }
    }

    pub async fn run(
        self,
        query_interval: Duration,
    ) -> BridgeResult<(
        Vec<JoinHandle<()>>,
        mysten_metrics::metered_channel::Receiver<(Identifier, Vec<SuiEvent>)>,
    )> {
        let (events_tx, events_rx) = mysten_metrics::metered_channel::channel(
            SUI_EVENTS_CHANNEL_SIZE,
            &mysten_metrics::get_metrics()
                .unwrap()
                .channels
                .with_label_values(&["sui_events_queue"]),
        );

        let mut task_handles = vec![];
        for (module, cursor) in self.cursors {
            let events_rx_clone = events_tx.clone();
            let sui_client_clone = self.sui_client.clone();
            task_handles.push(spawn_logged_monitored_task!(
                Self::run_event_listening_task(
                    module,
                    cursor,
                    events_rx_clone,
                    sui_client_clone,
                    query_interval
                )
            ));
        }
        Ok((task_handles, events_rx))
    }

    async fn run_event_listening_task(
        // The module where interested events are defined.
        // Moudle is always of bridge package 0x9.
        module: Identifier,
        mut next_cursor: TransactionDigest,
        events_sender: mysten_metrics::metered_channel::Sender<(Identifier, Vec<SuiEvent>)>,
        sui_client: Arc<SuiClient<C>>,
        query_interval: Duration,
    ) {
        tracing::info!(
            ?module,
            ?next_cursor,
            "Starting sui events listening task from tx_digest {next_cursor}"
        );
        let mut interval = time::interval(query_interval);
        interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);
        loop {
            interval.tick().await;
            let Ok(events) = retry_with_max_delay!(
                sui_client.query_events_by_module(PACKAGE_ID, module.clone(), next_cursor),
                Duration::from_secs(600)
            ) else {
                tracing::error!("Failed to query events from sui client after retry");
                continue;
            };

            let len = events.data.len();
            if len != 0 {
                // Note: it's extremely critical to make sure the SuiEvents we send via this channel
                // are complete per transaction level. Namely, we should never send a partial list
                // of events for a transaction. Otherwise, we may end up missing events.
                // See `sui_client.query_events_by_module` for how this is implemented.
                events_sender
                    .send((module.clone(), events.data))
                    .await
                    .expect("All Sui event channel receivers are closed");
                // Unwrap: `query_events_by_module` always returns Some `next_cursor`
                // If the events list is empty, `next_cursor` will be the same as `start_tx_digest`
                next_cursor = events.next_cursor.unwrap();
                tracing::info!(?module, ?next_cursor, "Observed {len} new Sui events");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{sui_client::SuiClient, sui_mock_client::SuiMockClient};
    use prometheus::Registry;
    use sui_json_rpc_types::EventPage;
    use sui_types::{digests::TransactionDigest, event::EventID, Identifier};
    use tokio::time::timeout;

    #[tokio::test]
    async fn test_sui_syncer_basic() -> anyhow::Result<()> {
        telemetry_subscribers::init_for_testing();
        let registry = Registry::new();
        mysten_metrics::init_metrics(&registry);

        let mock = SuiMockClient::default();
        let client = Arc::new(SuiClient::new_for_testing(mock.clone()));
        let module_foo = Identifier::new("Foo").unwrap();
        let module_bar = Identifier::new("Bar").unwrap();
        let empty_events = EventPage::empty();
        let cursor = TransactionDigest::random();
        add_event_response(&mock, module_foo.clone(), cursor, empty_events.clone());
        add_event_response(&mock, module_bar.clone(), cursor, empty_events.clone());

        let target_modules = HashMap::from_iter(vec![
            (module_foo.clone(), cursor),
            (module_bar.clone(), cursor),
        ]);
        let interval = Duration::from_millis(200);
        let (_handles, mut events_rx) = SuiSyncer::new(client, target_modules)
            .run(interval)
            .await
            .unwrap();

        // Initially there are no events
        assert_no_more_events(interval, &mut events_rx).await;

        // Module Foo has new events
        let event_1: SuiEvent = SuiEvent::random_for_testing();
        let module_foo_events_1: sui_json_rpc_types::Page<SuiEvent, EventID> = EventPage {
            data: vec![event_1.clone(), event_1.clone()],
            next_cursor: None,
            has_next_page: false,
        };
        add_event_response(
            &mock,
            module_foo.clone(),
            event_1.id.tx_digest,
            empty_events.clone(),
        );
        add_event_response(
            &mock,
            module_foo.clone(),
            cursor,
            module_foo_events_1.clone(),
        );

        let (identifier, received_events) = events_rx.recv().await.unwrap();
        assert_eq!(identifier, module_foo);
        assert_eq!(received_events.len(), 2);
        assert_eq!(received_events[0].id, event_1.id);
        assert_eq!(received_events[1].id, event_1.id);
        // No more
        assert_no_more_events(interval, &mut events_rx).await;

        // Module Bar has new events
        let event_2: SuiEvent = SuiEvent::random_for_testing();
        let module_bar_events_1 = EventPage {
            data: vec![event_2.clone()],
            next_cursor: None,
            has_next_page: false,
        };
        add_event_response(
            &mock,
            module_bar.clone(),
            event_2.id.tx_digest,
            empty_events.clone(),
        );

        add_event_response(&mock, module_bar.clone(), cursor, module_bar_events_1);

        let (identifier, received_events) = events_rx.recv().await.unwrap();
        assert_eq!(identifier, module_bar);
        assert_eq!(received_events.len(), 1);
        assert_eq!(received_events[0].id, event_2.id);
        // No more
        assert_no_more_events(interval, &mut events_rx).await;

        Ok(())
    }

    async fn assert_no_more_events(
        interval: Duration,
        events_rx: &mut mysten_metrics::metered_channel::Receiver<(Identifier, Vec<SuiEvent>)>,
    ) {
        match timeout(interval * 2, events_rx.recv()).await {
            Err(_e) => (),
            other => panic!("Should have timed out, but got: {:?}", other),
        };
    }

    fn add_event_response(
        mock: &SuiMockClient,
        module: Identifier,
        cursor: TransactionDigest,
        events: EventPage,
    ) {
        mock.add_event_response(
            PACKAGE_ID,
            module.clone(),
            EventID {
                tx_digest: cursor,
                event_seq: u16::MAX as u64,
            },
            events.clone(),
        );
    }
}
