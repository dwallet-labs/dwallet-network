// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//! `BridgeOrchestrator` is the component that:
//! 1. monitors Pera and Ethereum events with the help of `PeraSyncer` and `EthSyncer`
//! 2. updates WAL table and cursor tables
//! 2. hands actions to `BridgeExecutor` for execution

use crate::abi::EthBridgeEvent;
use crate::action_executor::{
    submit_to_executor, BridgeActionExecutionWrapper, BridgeActionExecutorTrait,
};
use crate::error::BridgeError;
use crate::events::PeraBridgeEvent;
use crate::metrics::BridgeMetrics;
use crate::storage::BridgeOrchestratorTables;
use crate::pera_client::{PeraClient, PeraClientInner};
use crate::types::EthLog;
use ethers::types::Address as EthAddress;
use mysten_metrics::spawn_logged_monitored_task;
use std::sync::Arc;
use pera_json_rpc_types::PeraEvent;
use pera_types::Identifier;
use tokio::task::JoinHandle;
use tracing::{error, info};

pub struct BridgeOrchestrator<C> {
    _pera_client: Arc<PeraClient<C>>,
    pera_events_rx: mysten_metrics::metered_channel::Receiver<(Identifier, Vec<PeraEvent>)>,
    eth_events_rx: mysten_metrics::metered_channel::Receiver<(EthAddress, u64, Vec<EthLog>)>,
    store: Arc<BridgeOrchestratorTables>,
    monitor_tx: mysten_metrics::metered_channel::Sender<PeraBridgeEvent>,
    metrics: Arc<BridgeMetrics>,
}

impl<C> BridgeOrchestrator<C>
where
    C: PeraClientInner + 'static,
{
    pub fn new(
        pera_client: Arc<PeraClient<C>>,
        pera_events_rx: mysten_metrics::metered_channel::Receiver<(Identifier, Vec<PeraEvent>)>,
        eth_events_rx: mysten_metrics::metered_channel::Receiver<(EthAddress, u64, Vec<EthLog>)>,
        store: Arc<BridgeOrchestratorTables>,
        monitor_tx: mysten_metrics::metered_channel::Sender<PeraBridgeEvent>,
        metrics: Arc<BridgeMetrics>,
    ) -> Self {
        Self {
            _pera_client: pera_client,
            pera_events_rx,
            eth_events_rx,
            store,
            monitor_tx,
            metrics,
        }
    }

    pub async fn run(
        self,
        bridge_action_executor: impl BridgeActionExecutorTrait,
    ) -> Vec<JoinHandle<()>> {
        tracing::info!("Starting BridgeOrchestrator");
        let mut task_handles = vec![];
        let store_clone = self.store.clone();

        // Spawn BridgeActionExecutor
        let (handles, executor_sender) = bridge_action_executor.run();
        task_handles.extend(handles);
        let executor_sender_clone = executor_sender.clone();
        let metrics_clone = self.metrics.clone();
        task_handles.push(spawn_logged_monitored_task!(Self::run_pera_watcher(
            store_clone,
            executor_sender_clone,
            self.pera_events_rx,
            self.monitor_tx,
            metrics_clone,
        )));
        let store_clone = self.store.clone();

        // Re-submit pending actions to executor
        let actions = store_clone
            .get_all_pending_actions()
            .into_values()
            .collect::<Vec<_>>();
        for action in actions {
            submit_to_executor(&executor_sender, action)
                .await
                .expect("Submit to executor should not fail");
        }

        let metrics_clone = self.metrics.clone();
        task_handles.push(spawn_logged_monitored_task!(Self::run_eth_watcher(
            store_clone,
            executor_sender,
            self.eth_events_rx,
            metrics_clone,
        )));

        task_handles
    }

    async fn run_pera_watcher(
        store: Arc<BridgeOrchestratorTables>,
        executor_tx: mysten_metrics::metered_channel::Sender<BridgeActionExecutionWrapper>,
        mut pera_events_rx: mysten_metrics::metered_channel::Receiver<(Identifier, Vec<PeraEvent>)>,
        monitor_tx: mysten_metrics::metered_channel::Sender<PeraBridgeEvent>,
        metrics: Arc<BridgeMetrics>,
    ) {
        info!("Starting pera watcher task");
        while let Some((identifier, events)) = pera_events_rx.recv().await {
            if events.is_empty() {
                continue;
            }
            info!("Received {} Pera events: {:?}", events.len(), events);
            metrics
                .pera_watcher_received_events
                .inc_by(events.len() as u64);
            let bridge_events = events
                .iter()
                .filter_map(|pera_event| {
                    match PeraBridgeEvent::try_from_pera_event(pera_event) {
                        Ok(bridge_event) => Some(bridge_event),
                        // On testnet some early bridge transactions could have zero value (before we disallow it in Move)
                        Err(BridgeError::ZeroValueBridgeTransfer(_)) => {
                            error!("Zero value bridge transfer: {:?}", pera_event);
                            None
                        }
                        Err(e) => {
                            panic!(
                                "Pera Event could not be deserialzed to PeraBridgeEvent: {:?}",
                                e
                            );
                        }
                    }
                })
                .collect::<Vec<_>>();

            let mut actions = vec![];
            for (pera_event, opt_bridge_event) in events.iter().zip(bridge_events) {
                if opt_bridge_event.is_none() {
                    // TODO: we probably should not miss any events, log for now.
                    metrics.pera_watcher_unrecognized_events.inc();
                    error!("Pera event not recognized: {:?}", pera_event);
                    continue;
                }
                // Unwrap safe: checked above
                let bridge_event: PeraBridgeEvent = opt_bridge_event.unwrap();
                info!("Observed Pera bridge event: {:?}", bridge_event);

                // Send event to monitor
                monitor_tx
                    .send(bridge_event.clone())
                    .await
                    .expect("Sending event to monitor channel should not fail");

                if let Some(action) = bridge_event
                    .try_into_bridge_action(pera_event.id.tx_digest, pera_event.id.event_seq as u16)
                {
                    actions.push(action);
                }
            }

            if !actions.is_empty() {
                info!("Received {} actions from Pera: {:?}", actions.len(), actions);
                metrics
                    .pera_watcher_received_actions
                    .inc_by(actions.len() as u64);
                // Write action to pending WAL
                store
                    .insert_pending_actions(&actions)
                    .expect("Store operation should not fail");
                for action in actions {
                    submit_to_executor(&executor_tx, action)
                        .await
                        .expect("Submit to executor should not fail");
                }
            }

            // Unwrap safe: in the beginning of the loop we checked that events is not empty
            let cursor = events.last().unwrap().id;
            store
                .update_pera_event_cursor(identifier, cursor)
                .expect("Store operation should not fail");
        }
        panic!("Pera event channel was closed unexpectedly");
    }

    async fn run_eth_watcher(
        store: Arc<BridgeOrchestratorTables>,
        executor_tx: mysten_metrics::metered_channel::Sender<BridgeActionExecutionWrapper>,
        mut eth_events_rx: mysten_metrics::metered_channel::Receiver<(
            ethers::types::Address,
            u64,
            Vec<EthLog>,
        )>,
        metrics: Arc<BridgeMetrics>,
    ) {
        info!("Starting eth watcher task");
        while let Some((contract, end_block, logs)) = eth_events_rx.recv().await {
            if logs.is_empty() {
                store
                    .update_eth_event_cursor(contract, end_block)
                    .expect("Store operation should not fail");
                continue;
            }

            info!("Received {} Eth events", logs.len());
            metrics
                .eth_watcher_received_events
                .inc_by(logs.len() as u64);

            let bridge_events = logs
                .iter()
                .map(EthBridgeEvent::try_from_eth_log)
                .collect::<Vec<_>>();

            let mut actions = vec![];
            for (log, opt_bridge_event) in logs.iter().zip(bridge_events) {
                if opt_bridge_event.is_none() {
                    // TODO: we probably should not miss any events, log for now.
                    metrics.eth_watcher_unrecognized_events.inc();
                    error!("Eth event not recognized: {:?}", log);
                    continue;
                }
                // Unwrap safe: checked above
                let bridge_event = opt_bridge_event.unwrap();
                info!("Observed Eth bridge event: {:?}", bridge_event);

                match bridge_event.try_into_bridge_action(log.tx_hash, log.log_index_in_tx) {
                    Ok(Some(action)) => actions.push(action),
                    Ok(None) => {}
                    Err(e) => {
                        error!(eth_tx_hash=?log.tx_hash, eth_event_index=?log.log_index_in_tx, "Error converting EthBridgeEvent to BridgeAction: {:?}", e);
                    }
                }
                // TODO: handle non Action events
            }
            if !actions.is_empty() {
                info!("Received {} actions from Eth: {:?}", actions.len(), actions);
                metrics
                    .eth_watcher_received_actions
                    .inc_by(actions.len() as u64);
                // Write action to pending WAL
                store
                    .insert_pending_actions(&actions)
                    .expect("Store operation should not fail");
                // Execution will remove the pending actions from DB when the action is completed.
                for action in actions {
                    submit_to_executor(&executor_tx, action)
                        .await
                        .expect("Submit to executor should not fail");
                }
            }

            store
                .update_eth_event_cursor(contract, end_block)
                .expect("Store operation should not fail");
        }
        panic!("Eth event channel was closed");
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        test_utils::{get_test_eth_to_pera_bridge_action, get_test_log_and_action},
        types::BridgeActionDigest,
    };
    use ethers::types::{Address as EthAddress, TxHash};
    use prometheus::Registry;
    use std::str::FromStr;

    use super::*;
    use crate::events::init_all_struct_tags;
    use crate::test_utils::get_test_pera_to_eth_bridge_action;
    use crate::{events::tests::get_test_pera_event_and_action, pera_mock_client::PeraMockClient};

    #[tokio::test]
    async fn test_pera_watcher_task() {
        // Note: this test may fail because of the following reasons:
        // the PeraEvent's struct tag does not match the ones in events.rs

        let (
            pera_events_tx,
            pera_events_rx,
            _eth_events_tx,
            eth_events_rx,
            monitor_tx,
            _monitor_rx,
            pera_client,
            store,
        ) = setup();
        let (executor, mut executor_requested_action_rx) = MockExecutor::new();
        // start orchestrator
        let registry = Registry::new();
        let metrics = Arc::new(BridgeMetrics::new(&registry));
        let _handles = BridgeOrchestrator::new(
            Arc::new(pera_client),
            pera_events_rx,
            eth_events_rx,
            store.clone(),
            monitor_tx,
            metrics,
        )
        .run(executor)
        .await;

        let identifier = Identifier::from_str("test_pera_watcher_task").unwrap();
        let (pera_event, bridge_action) = get_test_pera_event_and_action(identifier.clone());
        pera_events_tx
            .send((identifier.clone(), vec![pera_event.clone()]))
            .await
            .unwrap();

        let start = std::time::Instant::now();
        // Executor should have received the action
        assert_eq!(
            executor_requested_action_rx.recv().await.unwrap(),
            bridge_action.digest()
        );
        loop {
            let actions = store.get_all_pending_actions();
            if actions.is_empty() {
                if start.elapsed().as_secs() > 5 {
                    panic!("Timed out waiting for action to be written to WAL");
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                continue;
            }
            assert_eq!(actions.len(), 1);
            let action = actions.get(&bridge_action.digest()).unwrap();
            assert_eq!(action, &bridge_action);
            assert_eq!(
                store.get_pera_event_cursors(&[identifier]).unwrap()[0].unwrap(),
                pera_event.id,
            );
            break;
        }
    }

    #[tokio::test]
    async fn test_eth_watcher_task() {
        // Note: this test may fail beacuse of the following reasons:
        // 1. Log and BridgeAction returned from `get_test_log_and_action` are not in sync
        // 2. Log returned from `get_test_log_and_action` is not parseable log (not abigen!, check abi.rs)

        let (
            _pera_events_tx,
            pera_events_rx,
            eth_events_tx,
            eth_events_rx,
            monitor_tx,
            _monitor_rx,
            pera_client,
            store,
        ) = setup();
        let (executor, mut executor_requested_action_rx) = MockExecutor::new();
        // start orchestrator
        let registry = Registry::new();
        let metrics = Arc::new(BridgeMetrics::new(&registry));
        let _handles = BridgeOrchestrator::new(
            Arc::new(pera_client),
            pera_events_rx,
            eth_events_rx,
            store.clone(),
            monitor_tx,
            metrics,
        )
        .run(executor)
        .await;
        let address = EthAddress::random();
        let (log, bridge_action) = get_test_log_and_action(address, TxHash::random(), 10);
        let log_index_in_tx = 10;
        let log_block_num = log.block_number.unwrap().as_u64();
        let eth_log = EthLog {
            log: log.clone(),
            tx_hash: log.transaction_hash.unwrap(),
            block_number: log_block_num,
            log_index_in_tx,
        };
        let end_block_num = log_block_num + 15;

        eth_events_tx
            .send((address, end_block_num, vec![eth_log.clone()]))
            .await
            .unwrap();

        // Executor should have received the action
        assert_eq!(
            executor_requested_action_rx.recv().await.unwrap(),
            bridge_action.digest()
        );
        let start = std::time::Instant::now();
        loop {
            let actions = store.get_all_pending_actions();
            if actions.is_empty() {
                if start.elapsed().as_secs() > 5 {
                    panic!("Timed out waiting for action to be written to WAL");
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                continue;
            }
            assert_eq!(actions.len(), 1);
            let action = actions.get(&bridge_action.digest()).unwrap();
            assert_eq!(action, &bridge_action);
            assert_eq!(
                store.get_eth_event_cursors(&[address]).unwrap()[0].unwrap(),
                end_block_num,
            );
            break;
        }
    }

    #[tokio::test]
    /// Test that when orchestrator starts, all pending actions are sent to executor
    async fn test_resume_actions_in_pending_logs() {
        let (
            _pera_events_tx,
            pera_events_rx,
            _eth_events_tx,
            eth_events_rx,
            monitor_tx,
            _monitor_rx,
            pera_client,
            store,
        ) = setup();
        let (executor, mut executor_requested_action_rx) = MockExecutor::new();

        let action1 = get_test_pera_to_eth_bridge_action(
            None,
            Some(0),
            Some(99),
            Some(10000),
            None,
            None,
            None,
        );

        let action2 = get_test_eth_to_pera_bridge_action(None, None, None, None);
        store
            .insert_pending_actions(&vec![action1.clone(), action2.clone()])
            .unwrap();

        // start orchestrator
        let registry = Registry::new();
        let metrics = Arc::new(BridgeMetrics::new(&registry));
        let _handles = BridgeOrchestrator::new(
            Arc::new(pera_client),
            pera_events_rx,
            eth_events_rx,
            store.clone(),
            monitor_tx,
            metrics,
        )
        .run(executor)
        .await;

        // Executor should have received the action
        let mut digests = std::collections::HashSet::new();
        digests.insert(executor_requested_action_rx.recv().await.unwrap());
        digests.insert(executor_requested_action_rx.recv().await.unwrap());
        assert!(digests.contains(&action1.digest()));
        assert!(digests.contains(&action2.digest()));
        assert_eq!(digests.len(), 2);
    }

    #[allow(clippy::type_complexity)]
    fn setup() -> (
        mysten_metrics::metered_channel::Sender<(Identifier, Vec<PeraEvent>)>,
        mysten_metrics::metered_channel::Receiver<(Identifier, Vec<PeraEvent>)>,
        mysten_metrics::metered_channel::Sender<(EthAddress, u64, Vec<EthLog>)>,
        mysten_metrics::metered_channel::Receiver<(EthAddress, u64, Vec<EthLog>)>,
        mysten_metrics::metered_channel::Sender<PeraBridgeEvent>,
        mysten_metrics::metered_channel::Receiver<PeraBridgeEvent>,
        PeraClient<PeraMockClient>,
        Arc<BridgeOrchestratorTables>,
    ) {
        telemetry_subscribers::init_for_testing();
        let registry = Registry::new();
        mysten_metrics::init_metrics(&registry);

        init_all_struct_tags();

        let temp_dir = tempfile::tempdir().unwrap();
        let store = BridgeOrchestratorTables::new(temp_dir.path());

        let mock_client = PeraMockClient::default();
        let pera_client = PeraClient::new_for_testing(mock_client.clone());

        let (eth_events_tx, eth_events_rx) = mysten_metrics::metered_channel::channel(
            100,
            &mysten_metrics::get_metrics()
                .unwrap()
                .channel_inflight
                .with_label_values(&["unit_test_eth_events_queue"]),
        );

        let (pera_events_tx, pera_events_rx) = mysten_metrics::metered_channel::channel(
            100,
            &mysten_metrics::get_metrics()
                .unwrap()
                .channel_inflight
                .with_label_values(&["unit_test_pera_events_queue"]),
        );
        let (monitor_tx, monitor_rx) = mysten_metrics::metered_channel::channel(
            10000,
            &mysten_metrics::get_metrics()
                .unwrap()
                .channel_inflight
                .with_label_values(&["monitor_queue"]),
        );
        (
            pera_events_tx,
            pera_events_rx,
            eth_events_tx,
            eth_events_rx,
            monitor_tx,
            monitor_rx,
            pera_client,
            store,
        )
    }

    /// A `BridgeActionExecutorTrait` implementation that only tracks the submitted actions.
    struct MockExecutor {
        requested_transactions_tx: tokio::sync::broadcast::Sender<BridgeActionDigest>,
    }

    impl MockExecutor {
        fn new() -> (Self, tokio::sync::broadcast::Receiver<BridgeActionDigest>) {
            let (tx, rx) = tokio::sync::broadcast::channel(100);
            (
                Self {
                    requested_transactions_tx: tx,
                },
                rx,
            )
        }
    }

    impl BridgeActionExecutorTrait for MockExecutor {
        fn run(
            self,
        ) -> (
            Vec<tokio::task::JoinHandle<()>>,
            mysten_metrics::metered_channel::Sender<BridgeActionExecutionWrapper>,
        ) {
            let (tx, mut rx) =
                mysten_metrics::metered_channel::channel::<BridgeActionExecutionWrapper>(
                    100,
                    &mysten_metrics::get_metrics()
                        .unwrap()
                        .channel_inflight
                        .with_label_values(&["unit_test_mock_executor"]),
                );

            let handles = tokio::spawn(async move {
                while let Some(action) = rx.recv().await {
                    self.requested_transactions_tx
                        .send(action.0.digest())
                        .unwrap();
                }
            });
            (vec![handles], tx)
        }
    }
}
