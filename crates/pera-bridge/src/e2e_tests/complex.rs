// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::client::bridge_authority_aggregator::BridgeAuthorityAggregator;
use crate::e2e_tests::basic::initiate_bridge_eth_to_pera;
use crate::e2e_tests::basic::initiate_bridge_pera_to_eth;
use crate::e2e_tests::test_utils::BridgeTestClusterBuilder;
use crate::pera_transaction_builder::build_pera_transaction;
use crate::types::{BridgeAction, EmergencyAction};
use crate::types::{BridgeActionStatus, EmergencyActionType};
use ethers::types::Address as EthAddress;
use pera_json_rpc_types::PeraExecutionStatus;
use pera_json_rpc_types::PeraTransactionBlockEffectsAPI;
use pera_types::bridge::{BridgeChainId, TOKEN_ID_ETH};
use std::sync::Arc;
use tracing::info;

#[tokio::test(flavor = "multi_thread", worker_threads = 16)]
async fn test_pera_bridge_paused() {
    telemetry_subscribers::init_for_testing();

    // approve pause action in bridge nodes
    let pause_action = BridgeAction::EmergencyAction(EmergencyAction {
        nonce: 0,
        chain_id: BridgeChainId::PeraCustom,
        action_type: EmergencyActionType::Pause,
    });

    let unpause_action = BridgeAction::EmergencyAction(EmergencyAction {
        nonce: 1,
        chain_id: BridgeChainId::PeraCustom,
        action_type: EmergencyActionType::Unpause,
    });

    // Setup bridge test env
    let bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(true)
        .with_num_validators(4)
        .with_approved_governance_actions(vec![
            vec![pause_action.clone(), unpause_action.clone()],
            vec![unpause_action.clone()],
            vec![unpause_action.clone()],
            vec![],
        ])
        .build()
        .await;

    let bridge_client = bridge_test_cluster.bridge_client();
    let pera_address = bridge_test_cluster.pera_user_address();
    let pera_token_type_tags = bridge_client.get_token_id_map().await.unwrap();

    // verify bridge are not paused
    assert!(!bridge_client.get_bridge_summary().await.unwrap().is_frozen);

    // try bridge from eth and verify it works on pera
    initiate_bridge_eth_to_pera(&bridge_test_cluster, 10, 0)
        .await
        .unwrap();
    // verify Eth was transferred to Pera address
    let eth_coin_type = pera_token_type_tags.get(&TOKEN_ID_ETH).unwrap();
    let eth_coin = bridge_client
        .pera_client()
        .coin_read_api()
        .get_coins(pera_address, Some(eth_coin_type.to_string()), None, None)
        .await
        .unwrap()
        .data;
    assert_eq!(1, eth_coin.len());

    // get pause bridge signatures from committee
    let bridge_committee = Arc::new(bridge_client.get_bridge_committee().await.unwrap());
    let agg = BridgeAuthorityAggregator::new(bridge_committee);
    let certified_action = agg
        .request_committee_signatures(pause_action)
        .await
        .unwrap();

    // execute pause bridge on pera
    let gas = bridge_test_cluster
        .wallet()
        .get_one_gas_object_owned_by_address(pera_address)
        .await
        .unwrap()
        .unwrap();

    let tx = build_pera_transaction(
        pera_address,
        &gas,
        certified_action,
        bridge_client
            .get_mutable_bridge_object_arg_must_succeed()
            .await,
        &pera_token_type_tags,
        1000,
    )
    .unwrap();

    let response = bridge_test_cluster.sign_and_execute_transaction(&tx).await;
    assert_eq!(
        response.effects.unwrap().status(),
        &PeraExecutionStatus::Success
    );
    info!("Bridge paused");

    // verify bridge paused
    assert!(bridge_client.get_bridge_summary().await.unwrap().is_frozen);

    // Transfer from eth to pera should fail on Pera
    let eth_to_pera_bridge_action = initiate_bridge_eth_to_pera(&bridge_test_cluster, 10, 1).await;
    assert!(eth_to_pera_bridge_action.is_err());
    // message should not be recorded on Pera when the bridge is paused
    let res = bridge_test_cluster
        .bridge_client()
        .get_token_transfer_action_onchain_status_until_success(
            bridge_test_cluster.eth_chain_id() as u8,
            1,
        )
        .await;
    assert_eq!(BridgeActionStatus::NotFound, res);
    // Transfer from Pera to eth should fail
    let pera_to_eth_bridge_action = initiate_bridge_pera_to_eth(
        &bridge_test_cluster,
        EthAddress::random(),
        eth_coin.first().unwrap().object_ref(),
        0,
        10,
    )
    .await;
    assert!(pera_to_eth_bridge_action.is_err())
}
