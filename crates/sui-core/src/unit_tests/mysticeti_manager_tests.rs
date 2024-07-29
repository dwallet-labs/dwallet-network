// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::authority::test_authority_builder::TestAuthorityBuilder;
use crate::checkpoints::CheckpointServiceNoop;
use crate::consensus_handler::ConsensusHandlerInitializer;
use crate::consensus_manager::mysticeti_manager::MysticetiManager;
use crate::consensus_manager::narwhal_manager::narwhal_manager_tests::{
    checkpoint_service_for_testing, signature_mpc_service_for_testing,
};
use crate::consensus_manager::ConsensusManagerMetrics;
use crate::consensus_manager::ConsensusManagerTrait;
use crate::consensus_validator::{SuiTxValidator, SuiTxValidatorMetrics};
use crate::mysticeti_adapter::LazyMysticetiClient;
use fastcrypto::traits::KeyPair;
use mysten_metrics::RegistryService;
use prometheus::Registry;
use std::sync::Arc;
use std::time::Duration;
use sui_swarm_config::network_config_builder::ConfigBuilder;
use sui_types::messages_signature_mpc::{
    config_signature_mpc_secret_for_network_for_testing, PartyID,
};
use tokio::time::sleep;

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn test_mysticeti_manager() {
    // GIVEN
    let configs = ConfigBuilder::new_with_temp_dir()
        .committee_size(1.try_into().unwrap())
        .build();

    let (decryption_key_share_public_parameters, decryption_key_shares) =
        config_signature_mpc_secret_for_network_for_testing(4 as PartyID);

    for i in 0..3 {
        let config = &configs.validator_configs()[0];

        let consensus_config = config.consensus_config().unwrap();
        let registry_service = RegistryService::new(Registry::new());
        let secret = Arc::pin(config.protocol_key_pair().copy());
        let genesis = config.genesis().unwrap();

        let state = TestAuthorityBuilder::new()
            .with_genesis_and_keypair(genesis, &secret)
            .build()
            .await;

        let metrics = ConsensusManagerMetrics::new(&Registry::new());
        let epoch_store = state.epoch_store_for_testing();
        let client = Arc::new(LazyMysticetiClient::default());

        let manager = MysticetiManager::new(
            config.protocol_key_pair().copy(),
            config.network_key_pair().copy(),
            consensus_config.db_path().to_path_buf(),
            metrics,
            registry_service,
            client,
        );

        let consensus_handler_initializer = ConsensusHandlerInitializer::new_for_testing(
            state.clone(),
            checkpoint_service_for_testing(state.clone()),
            signature_mpc_service_for_testing(
                decryption_key_share_public_parameters.clone(),
                decryption_key_shares[&((i + 1) as PartyID)],
                state.clone(),
            ),
        );

        // WHEN start mysticeti
        manager
            .start(
                config,
                epoch_store.clone(),
                consensus_handler_initializer,
                SuiTxValidator::new(
                    epoch_store.clone(),
                    Arc::new(CheckpointServiceNoop {}),
                    state.transaction_manager().clone(),
                    SuiTxValidatorMetrics::new(&Registry::new()),
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
    }
}
