// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::{sync::Arc, time::Duration};

use fastcrypto::traits::KeyPair;
use mysten_metrics::RegistryService;
use pera_swarm_config::network_config_builder::ConfigBuilder;
use prometheus::Registry;
use tokio::time::sleep;

use crate::{
    authority::test_authority_builder::TestAuthorityBuilder,
    checkpoints::CheckpointServiceNoop,
    consensus_handler::ConsensusHandlerInitializer,
    consensus_manager::{
        mysticeti_manager::MysticetiManager,
        narwhal_manager::narwhal_manager_tests::checkpoint_service_for_testing,
        ConsensusManagerMetrics, ConsensusManagerTrait,
    },
    consensus_validator::{PeraTxValidator, PeraTxValidatorMetrics},
    mysticeti_adapter::LazyMysticetiClient,
};

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn test_mysticeti_manager() {
    // GIVEN
    let configs = ConfigBuilder::new_with_temp_dir()
        .committee_size(1.try_into().unwrap())
        .build();

    let config = &configs.validator_configs()[0];

    let consensus_config = config.consensus_config().unwrap();
    let registry_service = RegistryService::new(Registry::new());
    let secret = Arc::pin(config.protocol_key_pair().copy());
    let genesis = config.genesis().unwrap();

    let state = TestAuthorityBuilder::new()
        .with_genesis_and_keypair(genesis, &secret)
        .build()
        .await;

    let metrics = Arc::new(ConsensusManagerMetrics::new(&Registry::new()));
    let epoch_store = state.epoch_store_for_testing();
    let client = Arc::new(LazyMysticetiClient::default());

    let manager = MysticetiManager::new(
        config.worker_key_pair().copy(),
        config.network_key_pair().copy(),
        consensus_config.db_path().to_path_buf(),
        registry_service,
        metrics,
        client,
    );

    let boot_counter = *manager.boot_counter.lock().await;
    assert_eq!(boot_counter, 0);

    for i in 1..=3 {
        let consensus_handler_initializer = ConsensusHandlerInitializer::new_for_testing(
            state.clone(),
            checkpoint_service_for_testing(state.clone()),
        );

        // WHEN start mysticeti
        manager
            .start(
                config,
                epoch_store.clone(),
                consensus_handler_initializer,
                PeraTxValidator::new(
                    epoch_store.clone(),
                    Arc::new(CheckpointServiceNoop {}),
                    state.transaction_manager().clone(),
                    PeraTxValidatorMetrics::new(&Registry::new()),
                ),
            )
            .await;

        // THEN
        assert!(manager.is_running().await);

        // Now try to shut it down
        sleep(Duration::from_secs(1)).await;

        // WHEN
        manager.shutdown().await;

        // THEN
        assert!(!manager.is_running().await);

        let boot_counter = *manager.boot_counter.lock().await;
        assert_eq!(boot_counter, i);
    }
}