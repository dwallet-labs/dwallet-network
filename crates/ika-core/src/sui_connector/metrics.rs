// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;
use mysten_common::metrics::{push_metrics, MetricsPushClient};
use mysten_metrics::RegistryService;
use prometheus::{
    register_histogram_vec_with_registry, register_int_counter_vec_with_registry,
    register_int_counter_with_registry, register_int_gauge_vec_with_registry,
    register_int_gauge_with_registry, HistogramVec, IntCounter, IntCounterVec, IntGauge,
    IntGaugeVec, Registry,
};
use std::time::Duration;
use sui_types::crypto::NetworkKeyPair;

const FINE_GRAINED_LATENCY_SEC_BUCKETS: &[f64] = &[
    0.001, 0.005, 0.01, 0.05, 0.1, 0.15, 0.2, 0.25, 0.3, 0.35, 0.4, 0.45, 0.5, 0.6, 0.7, 0.8, 0.9,
    1.0, 1.2, 1.4, 1.6, 1.8, 2.0, 2.5, 3.0, 3.5, 4.0, 5.0, 6.0, 6.5, 7.0, 7.5, 8.0, 8.5, 9.0, 9.5,
    10., 15., 20., 25., 30., 35., 40., 45., 50., 60., 70., 80., 90., 100., 120., 140., 160., 180.,
    200., 250., 300., 350., 400.,
];

#[derive(Clone, Debug)]
pub struct SuiConnectorMetrics {
    pub err_build_sui_transaction: IntCounter,
    pub err_signature_aggregation: IntCounter,
    pub err_signature_aggregation_too_many_failures: IntCounter,
    pub err_sui_transaction_submission: IntCounter,
    pub err_sui_transaction_submission_too_many_failures: IntCounter,
    pub err_sui_transaction_execution: IntCounter,
    pub requests_received: IntCounterVec,
    pub requests_ok: IntCounterVec,
    pub err_requests: IntCounterVec,
    pub requests_inflight: IntGaugeVec,

    pub last_synced_sui_checkpoints: IntGaugeVec,
    pub last_finalized_eth_block: IntGauge,
    pub last_synced_eth_blocks: IntGaugeVec,

    pub sui_watcher_received_events: IntCounter,
    pub sui_watcher_received_actions: IntCounter,
    pub sui_watcher_unrecognized_events: IntCounter,
    pub eth_watcher_received_events: IntCounter,
    pub eth_watcher_received_actions: IntCounter,
    pub eth_watcher_unrecognized_events: IntCounter,
    pub action_executor_already_processed_actions: IntCounter,
    pub action_executor_signing_queue_received_actions: IntCounter,
    pub action_executor_signing_queue_skipped_actions: IntCounter,
    pub action_executor_execution_queue_received_actions: IntCounter,
    pub action_executor_execution_queue_skipped_actions_due_to_pausing: IntCounter,

    pub last_observed_actions_seq_num: IntGaugeVec,

    pub signer_with_cache_hit: IntCounterVec,
    pub signer_with_cache_miss: IntCounterVec,

    pub eth_rpc_queries: IntCounterVec,
    pub eth_rpc_queries_latency: HistogramVec,

    pub gas_coin_balance: IntGauge,

    pub sui_rpc_errors: IntCounterVec,
    pub observed_governance_actions: IntCounterVec,
    pub current_sui_connector_voting_rights: IntGaugeVec,

    pub auth_agg_ok_responses: IntCounterVec,
    pub auth_agg_bad_responses: IntCounterVec,
}

impl SuiConnectorMetrics {
    pub fn new(registry: &Registry) -> Arc<Self> {
        let this = Self {
            err_build_sui_transaction: register_int_counter_with_registry!(
                "sui_connector_err_build_sui_transaction",
                "Total number of errors of building sui transactions",
                registry,
            )
                .unwrap(),
            err_signature_aggregation: register_int_counter_with_registry!(
                "sui_connector_err_signature_aggregation",
                "Total number of errors of aggregating validators signatures",
                registry,
            )
                .unwrap(),
            err_signature_aggregation_too_many_failures: register_int_counter_with_registry!(
                "sui_connector_err_signature_aggregation_too_many_failures",
                "Total number of continuous failures during validator signature aggregation",
                registry,
            )
                .unwrap(),
            err_sui_transaction_submission: register_int_counter_with_registry!(
                "sui_connector_err_sui_transaction_submission",
                "Total number of errors of submitting sui transactions",
                registry,
            )
                .unwrap(),
            err_sui_transaction_submission_too_many_failures: register_int_counter_with_registry!(
                "sui_connector_err_sui_transaction_submission_too_many_failures",
                "Total number of continuous failures to submitting sui transactions",
                registry,
            )
                .unwrap(),
            err_sui_transaction_execution: register_int_counter_with_registry!(
                "sui_connector_err_sui_transaction_execution",
                "Total number of failures of sui transaction execution",
                registry,
            )
                .unwrap(),
            requests_received: register_int_counter_vec_with_registry!(
                "sui_connector_requests_received",
                "Total number of requests received in Server, by request type",
                &["type"],
                registry,
            )
                .unwrap(),
            requests_ok: register_int_counter_vec_with_registry!(
                "sui_connector_requests_ok",
                "Total number of ok requests, by request type",
                &["type"],
                registry,
            )
                .unwrap(),
            err_requests: register_int_counter_vec_with_registry!(
                "sui_connector_err_requests",
                "Total number of erred requests, by request type",
                &["type"],
                registry,
            )
                .unwrap(),
            requests_inflight: register_int_gauge_vec_with_registry!(
                "sui_connector_requests_inflight",
                "Total number of inflight requests, by request type",
                &["type"],
                registry,
            )
                .unwrap(),
            sui_watcher_received_events: register_int_counter_with_registry!(
                "sui_connector_sui_watcher_received_events",
                "Total number of received events in sui watcher",
                registry,
            )
                .unwrap(),
            eth_watcher_received_events: register_int_counter_with_registry!(
                "sui_connector_eth_watcher_received_events",
                "Total number of received events in eth watcher",
                registry,
            )
                .unwrap(),
            sui_watcher_received_actions: register_int_counter_with_registry!(
                "sui_connector_sui_watcher_received_actions",
                "Total number of received actions in sui watcher",
                registry,
            )
                .unwrap(),
            eth_watcher_received_actions: register_int_counter_with_registry!(
                "sui_connector_eth_watcher_received_actions",
                "Total number of received actions in eth watcher",
                registry,
            )
                .unwrap(),
            sui_watcher_unrecognized_events: register_int_counter_with_registry!(
                "sui_connector_sui_watcher_unrecognized_events",
                "Total number of unrecognized events in sui watcher",
                registry,
            )
                .unwrap(),
            eth_watcher_unrecognized_events: register_int_counter_with_registry!(
                "sui_connector_eth_watcher_unrecognized_events",
                "Total number of unrecognized events in eth watcher",
                registry,
            )
                .unwrap(),
            action_executor_already_processed_actions: register_int_counter_with_registry!(
                "sui_connector_action_executor_already_processed_actions",
                "Total number of already processed actions action executor",
                registry,
            )
                .unwrap(),
            action_executor_signing_queue_received_actions: register_int_counter_with_registry!(
                "sui_connector_action_executor_signing_queue_received_actions",
                "Total number of received actions in action executor signing queue",
                registry,
            )
                .unwrap(),
            action_executor_signing_queue_skipped_actions: register_int_counter_with_registry!(
                "sui_connector_action_executor_signing_queue_skipped_actions",
                "Total number of skipped actions in action executor signing queue",
                registry,
            )
                .unwrap(),
            action_executor_execution_queue_received_actions: register_int_counter_with_registry!(
                "sui_connector_action_executor_execution_queue_received_actions",
                "Total number of received actions in action executor execution queue",
                registry,
            )
                .unwrap(),
            action_executor_execution_queue_skipped_actions_due_to_pausing: register_int_counter_with_registry!(
                "sui_connector_action_executor_execution_queue_skipped_actions_due_to_pausing",
                "Total number of skipped actions in action executor execution queue because of pausing",
                registry,
            )
                .unwrap(),
            gas_coin_balance: register_int_gauge_with_registry!(
                "sui_connector_gas_coin_balance",
                "Current balance of gas coin, in mist",
                registry,
            )
                .unwrap(),
            eth_rpc_queries: register_int_counter_vec_with_registry!(
                "sui_connector_eth_rpc_queries",
                "Total number of queries issued to eth provider, by request type",
                &["type"],
                registry,
            )
                .unwrap(),
            eth_rpc_queries_latency: register_histogram_vec_with_registry!(
                "sui_connector_eth_rpc_queries_latency",
                "Latency of queries issued to eth provider, by request type",
                &["type"],
                FINE_GRAINED_LATENCY_SEC_BUCKETS.to_vec(),
                registry,
            )
                .unwrap(),
            last_synced_sui_checkpoints: register_int_gauge_vec_with_registry!(
                "sui_connector_last_synced_sui_checkpoints",
                "The latest sui checkpoints synced for each module",
                &["module_name"],
                registry,
            )
                .unwrap(),
            last_synced_eth_blocks: register_int_gauge_vec_with_registry!(
                "sui_connector_last_synced_eth_blocks",
                "The latest synced eth blocks synced for each contract",
                &["contract_address"],
                registry,
            )
                .unwrap(),
            last_finalized_eth_block: register_int_gauge_with_registry!(
                "sui_connector_last_finalized_eth_block",
                "The latest finalized eth block observed",
                registry,
            )
                .unwrap(),
            last_observed_actions_seq_num: register_int_gauge_vec_with_registry!(
                "sui_connector_last_observed_actions_seq_num",
                "The latest observed action sequence number per chain_id and action_type",
                &["chain_id", "action_type"],
                registry,
            )
                .unwrap(),
            signer_with_cache_hit: register_int_counter_vec_with_registry!(
                "sui_connector_signer_with_cache_hit",
                "Total number of hit in signer's cache, by verifier type",
                &["type"],
                registry,
            )
                .unwrap(),
            signer_with_cache_miss: register_int_counter_vec_with_registry!(
                "sui_connector_signer_with_cache_miss",
                "Total number of miss in signer's cache, by verifier type",
                &["type"],
                registry,
            )
                .unwrap(),
            sui_rpc_errors: register_int_counter_vec_with_registry!(
                "sui_connector_sui_rpc_errors",
                "Total number of errors from sui RPC, by RPC method",
                &["method"],
                registry,
            )
                .unwrap(),
            observed_governance_actions: register_int_counter_vec_with_registry!(
                "sui_connector_observed_governance_actions",
                "Total number of observed governance actions",
                &["action_type", "chain_id"],
                registry,
            )
                .unwrap(),
            current_sui_connector_voting_rights: register_int_gauge_vec_with_registry!(
                "current_sui_connector_voting_rights",
                "Current voting power in the bridge committee",
                &["authority"],
                registry
            )
                .unwrap(),
            auth_agg_ok_responses: register_int_counter_vec_with_registry!(
                "sui_connector_auth_agg_ok_responses",
                "Total number of ok respones from auth agg",
                &["authority"],
                registry,
            )
                .unwrap(),
            auth_agg_bad_responses: register_int_counter_vec_with_registry!(
                "sui_connector_auth_agg_bad_responses",
                "Total number of bad respones from auth agg",
                &["authority"],
                registry,
            )
                .unwrap(),
        };
        Arc::new(this)
    }

    pub fn new_for_testing() -> Arc<Self> {
        let registry = Registry::new();
        Self::new(&registry)
    }
}
