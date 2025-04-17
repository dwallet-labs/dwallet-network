// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
use std::{path::PathBuf, sync::Arc};

use arc_swap::ArcSwapOption;
use async_trait::async_trait;
use consensus_config::{Committee, NetworkKeyPair, Parameters, ProtocolKeyPair};
use consensus_core::{CommitConsumer, CommitConsumerMonitor, CommitIndex, ConsensusAuthority};
use fastcrypto::ed25519;
use ika_config::NodeConfig;
use ika_types::{committee::EpochId, sui::epoch_start_system::EpochStartSystemTrait};
use mysten_metrics::{RegistryID, RegistryService};
use prometheus::Registry;
use sui_protocol_config::ConsensusNetwork;
use tokio::sync::Mutex;
use tracing::info;

use crate::{
    authority::authority_per_epoch_store::AuthorityPerEpochStore,
    consensus_handler::{ConsensusHandlerInitializer, MysticetiConsensusHandler},
    consensus_manager::{
        ConsensusManagerMetrics, ConsensusManagerTrait, Running, RunningLockGuard,
    },
    consensus_validator::IkaTxValidator,
    mysticeti_adapter::LazyMysticetiClient,
};

pub struct MysticetiManager {
    protocol_keypair: ProtocolKeyPair,
    network_keypair: NetworkKeyPair,
    storage_base_path: PathBuf,
    // TODO: switch to parking_lot::Mutex.
    running: Mutex<Running>,
    metrics: Arc<ConsensusManagerMetrics>,
    registry_service: RegistryService,
    authority: ArcSwapOption<(ConsensusAuthority, RegistryID)>,
    boot_counter: Mutex<u64>,
    // Use a shared lazy mysticeti client so we can update the internal mysticeti
    // client that gets created for every new epoch.
    client: Arc<LazyMysticetiClient>,
    // TODO: switch to parking_lot::Mutex.
    consensus_handler: Mutex<Option<MysticetiConsensusHandler>>,
    consumer_monitor: ArcSwapOption<CommitConsumerMonitor>,
}

impl MysticetiManager {
    /// NOTE: Mysticeti protocol key uses Ed25519 instead of BLS.
    /// But for security, the protocol keypair must be different from the network keypair.
    pub fn new(
        protocol_keypair: ed25519::Ed25519KeyPair,
        network_keypair: ed25519::Ed25519KeyPair,
        storage_base_path: PathBuf,
        registry_service: RegistryService,
        metrics: Arc<ConsensusManagerMetrics>,
        client: Arc<LazyMysticetiClient>,
    ) -> Self {
        Self {
            protocol_keypair: ProtocolKeyPair::new(protocol_keypair),
            network_keypair: NetworkKeyPair::new(network_keypair),
            storage_base_path,
            running: Mutex::new(Running::False),
            metrics,
            registry_service,
            authority: ArcSwapOption::empty(),
            client,
            consensus_handler: Mutex::new(None),
            boot_counter: Mutex::new(0),
            consumer_monitor: ArcSwapOption::empty(),
        }
    }

    fn get_store_path(&self, epoch: EpochId) -> PathBuf {
        let mut store_path = self.storage_base_path.clone();
        store_path.push(format!("{}", epoch));
        store_path
    }
}

#[async_trait]
impl ConsensusManagerTrait for MysticetiManager {
    async fn start(
        &self,
        config: &NodeConfig,
        epoch_store: Arc<AuthorityPerEpochStore>,
        consensus_handler_initializer: ConsensusHandlerInitializer,
        tx_validator: IkaTxValidator,
    ) {
        let system_state = epoch_store.epoch_start_state();
        let committee: Committee = system_state.get_consensus_committee();
        let epoch = epoch_store.epoch();
        let protocol_config = epoch_store.protocol_config();

        let Some(_guard) = RunningLockGuard::acquire_start(
            &self.metrics,
            &self.running,
            epoch,
            protocol_config.version,
        )
        .await
        else {
            return;
        };

        let consensus_config = config
            .consensus_config()
            .expect("consensus_config should exist");

        let parameters = Parameters {
            db_path: self.get_store_path(epoch),
            ..consensus_config.parameters.clone().unwrap_or_default()
        };

        let own_protocol_key = self.protocol_keypair.public();
        let (own_index, _) = committee
            .authorities()
            .find(|(_, a)| a.protocol_key == own_protocol_key)
            .expect("Own authority should be among the consensus authorities!");

        let registry = Registry::new_custom(Some("consensus".to_string()), None).unwrap();

        let consensus_handler = consensus_handler_initializer.new_consensus_handler();
        let (commit_consumer, commit_receiver, _) =
            CommitConsumer::new(consensus_handler.last_processed_subdag_index() as CommitIndex);
        let monitor = commit_consumer.monitor();

        // If there is a previous consumer monitor, it indicates that the consensus engine has been restarted, due to an epoch change. However, that on its
        // own doesn't tell us much whether it participated on an active epoch or an old one. We need to check if it has handled any commits to determine this.
        // If indeed any commits did happen, then we assume that node did participate on previous run.
        let participated_on_previous_run =
            if let Some(previous_monitor) = self.consumer_monitor.swap(Some(monitor.clone())) {
                previous_monitor.highest_handled_commit() > 0
            } else {
                false
            };

        // Increment the boot counter only if the consensus successfully participated in the previous run.
        // This is typical during normal epoch changes, where the node restarts as expected, and the boot counter is incremented to prevent amnesia recovery on the next start.
        // If the node is recovering from a restore process and catching up across multiple epochs, it won't handle any commits until it reaches the last active epoch.
        // In this scenario, we do not increment the boot counter, as we need amnesia recovery to run.
        let mut boot_counter = self.boot_counter.lock().await;
        if participated_on_previous_run {
            *boot_counter += 1;
        } else {
            info!(
                "Node has not participated in previous run. Boot counter will not increment {}",
                *boot_counter
            );
        }

        // This can only be changed for all validators together at the same epoch
        let mut protocol_config = if epoch >= 0 {
            sui_protocol_config::ProtocolConfig::get_for_version(
                sui_protocol_config::ProtocolVersion::new(70),
                sui_protocol_config::Chain::Mainnet,
            )
        } else {
            sui_protocol_config::ProtocolConfig::get_for_version(
                sui_protocol_config::ProtocolVersion::new(70),
                sui_protocol_config::Chain::Mainnet,
            )
        };

        // TODO (#873): Implement a production grade configuration upgrade mechanism
        // We use the `_for_testing` functions because they are currently the only way
        // to modify Sui's protocol configuration from external crates.
        // I have opened an [issue](https://github.com/MystenLabs/sui/issues/21891)
        // in the Sui repository to address this limitation.
        protocol_config.set_consensus_max_transaction_size_bytes_for_testing(500 * 1024 * 1024);
        protocol_config
            .set_consensus_max_transactions_in_block_bytes_for_testing(500 * 1024 * 1024);
        let authority = ConsensusAuthority::start(
            protocol_config.consensus_network(),
            own_index,
            committee.clone(),
            parameters.clone(),
            protocol_config,
            self.protocol_keypair.clone(),
            self.network_keypair.clone(),
            Arc::new(tx_validator.clone()),
            commit_consumer,
            registry.clone(),
            *boot_counter,
        )
        .await;
        let client = authority.transaction_client();

        let registry_id = self.registry_service.add(registry.clone());

        let registered_authority = Arc::new((authority, registry_id));
        self.authority.swap(Some(registered_authority.clone()));

        // Initialize the client to send transactions to this Mysticeti instance.
        self.client.set(client);

        let handler = MysticetiConsensusHandler::new(consensus_handler, commit_receiver, monitor);

        let mut consensus_handler = self.consensus_handler.lock().await;
        *consensus_handler = Some(handler);

        // Wait until all locally available commits have been processed
        registered_authority.0.replay_complete().await;
    }

    async fn shutdown(&self) {
        let Some(_guard) = RunningLockGuard::acquire_shutdown(&self.metrics, &self.running).await
        else {
            return;
        };

        // Stop consensus submissions.
        self.client.clear();

        // swap with empty to ensure there is no other reference to authority and we can safely do Arc unwrap
        let r = self.authority.swap(None).unwrap();
        let Ok((authority, registry_id)) = Arc::try_unwrap(r) else {
            panic!("Failed to retrieve the mysticeti authority");
        };

        // shutdown the authority and wait for it
        authority.stop().await;

        // drop the old consensus handler to force stop any underlying task running.
        let mut consensus_handler = self.consensus_handler.lock().await;
        if let Some(mut handler) = consensus_handler.take() {
            handler.abort().await;
        }

        // unregister the registry id
        self.registry_service.remove(registry_id);
    }

    async fn is_running(&self) -> bool {
        Running::False != *self.running.lock().await
    }
}
