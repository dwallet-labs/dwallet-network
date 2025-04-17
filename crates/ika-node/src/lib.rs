// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use anemo::Network;
use anemo::PeerId;
use anemo_tower::callback::CallbackLayer;
use anemo_tower::trace::DefaultMakeSpan;
use anemo_tower::trace::DefaultOnFailure;
use anemo_tower::trace::TraceLayer;
use anyhow::anyhow;
use anyhow::Result;
use arc_swap::ArcSwap;
use fastcrypto_zkp::bn254::zk_login::JwkId;
use fastcrypto_zkp::bn254::zk_login::OIDCProvider;
use futures::TryFutureExt;
use mysten_network::server::SUI_TLS_SERVER_NAME;
use prometheus::Registry;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;
#[cfg(msim)]
use std::sync::atomic::Ordering;
use std::sync::{Arc, RwLock, Weak};
use std::time::Duration;

use ika_core::consensus_adapter::ConsensusClient;
use ika_core::consensus_manager::UpdatableConsensusClient;

use ika_types::digests::ChainIdentifier;
use ika_types::digests::CheckpointMessageDigest;
use ika_types::sui::SystemInner;
use sui_types::base_types::{random_object_ref, ConciseableName};
use sui_types::crypto::RandomnessRound;
use tap::tap::TapFallible;
use tokio::runtime::Handle;
use tokio::sync::{broadcast, mpsc, watch, Mutex};
use tokio::task::{JoinHandle, JoinSet};
use tower::ServiceBuilder;
use tracing::{debug, error, warn};
use tracing::{error_span, info, Instrument};

use fastcrypto_zkp::bn254::zk_login::JWK;
pub use handle::IkaNodeHandle;
use ika_archival::reader::ArchiveReaderBalancer;
use ika_archival::writer::ArchiveWriter;
use ika_config::node::RunWithRange;
use ika_config::node_config_metrics::NodeConfigMetrics;
use ika_config::object_storage_config::{ObjectStoreConfig, ObjectStoreType};
use ika_config::{ConsensusConfig, NodeConfig};
use ika_core::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use ika_core::authority::epoch_start_configuration::EpochStartConfigTrait;
use ika_core::authority::epoch_start_configuration::EpochStartConfiguration;
use ika_core::authority::AuthorityState;
use ika_core::checkpoints::{
    CheckpointMetrics, CheckpointService, CheckpointStore, SendCheckpointToStateSync,
    SubmitCheckpointToConsensus,
};
use ika_core::consensus_adapter::{
    CheckConnection, ConnectionMonitorStatus, ConsensusAdapter, ConsensusAdapterMetrics,
};
use ika_core::consensus_manager::{ConsensusManager, ConsensusManagerTrait};
use ika_core::consensus_throughput_calculator::{
    ConsensusThroughputCalculator, ConsensusThroughputProfiler, ThroughputProfileRanges,
};
use ika_core::consensus_validator::{IkaTxValidator, IkaTxValidatorMetrics};
use ika_core::epoch::committee_store::CommitteeStore;
use ika_core::epoch::consensus_store_pruner::ConsensusStorePruner;
use ika_core::epoch::epoch_metrics::EpochMetrics;
use ika_core::storage::RocksDbStore;
use mysten_metrics::{spawn_monitored_task, RegistryService};
use mysten_network::server::ServerBuilder;
use mysten_service::server_timing::server_timing_middleware;

use ika_network::discovery::TrustedPeerChangeEvent;
use ika_network::{discovery, state_sync};
use ika_protocol_config::{Chain, ProtocolConfig};
use sui_macros::fail_point;
use sui_macros::{fail_point_async, replay_log};
use sui_storage::{FileCompression, StorageFormat};
use sui_types::base_types::EpochId;

use ika_types::committee::Committee;
use ika_types::crypto::AuthorityName;
use ika_types::error::{IkaError, IkaResult};
use ika_types::messages_consensus::{AuthorityCapabilitiesV1, ConsensusTransaction};
use ika_types::quorum_driver_types::QuorumDriverEffectsQueueResult;
use ika_types::sui::epoch_start_system::EpochStartSystem;
use ika_types::sui::epoch_start_system::EpochStartSystemTrait;
use ika_types::sui::SystemInnerTrait;
use sui_types::crypto::KeypairTraits;

use ika_core::consensus_adapter::SubmitToConsensus;
use ika_types::supported_protocol_versions::SupportedProtocolVersions;
use typed_store::rocks::default_db_options;
use typed_store::DBMetrics;

use crate::metrics::{GrpcMetrics, IkaNodeMetrics};

pub mod admin;
mod handle;
pub mod metrics;

pub struct ValidatorComponents {
    consensus_manager: ConsensusManager,
    consensus_store_pruner: ConsensusStorePruner,
    consensus_adapter: Arc<ConsensusAdapter>,
    // Keeping the handle to the checkpoint service tasks to shut them down during reconfiguration.
    checkpoint_service_tasks: JoinSet<()>,
    checkpoint_metrics: Arc<CheckpointMetrics>,
    ika_tx_validator_metrics: Arc<IkaTxValidatorMetrics>,

    dwallet_mpc_service_exit: watch::Sender<()>,
}
pub struct P2pComponents {
    p2p_network: Network,
    known_peers: HashMap<PeerId, String>,
    discovery_handle: discovery::Handle,
    state_sync_handle: state_sync::Handle,
}

#[cfg(msim)]
mod simulator {
    use std::sync::atomic::AtomicBool;

    use super::*;
    pub(super) struct SimState {
        pub sim_node: ika_simulator::runtime::NodeHandle,
        pub sim_safe_mode_expected: AtomicBool,
        _leak_detector: ika_simulator::NodeLeakDetector,
    }

    impl Default for SimState {
        fn default() -> Self {
            Self {
                sim_node: ika_simulator::runtime::NodeHandle::current(),
                sim_safe_mode_expected: AtomicBool::new(false),
                _leak_detector: ika_simulator::NodeLeakDetector::new(),
            }
        }
    }

    type JwkInjector = dyn Fn(AuthorityName, &OIDCProvider) -> IkaResult<Vec<(JwkId, JWK)>>
        + Send
        + Sync
        + 'static;

    fn default_fetch_jwks(
        _authority: AuthorityName,
        _provider: &OIDCProvider,
    ) -> IkaResult<Vec<(JwkId, JWK)>> {
        use fastcrypto_zkp::bn254::zk_login::parse_jwks;
        // Just load a default Twitch jwk for testing.
        parse_jwks(
            ika_types::zk_login_util::DEFAULT_JWK_BYTES,
            &OIDCProvider::Twitch,
        )
        .map_err(|_| IkaError::JWKRetrievalError)
    }

    thread_local! {
        static JWK_INJECTOR: std::cell::RefCell<Arc<JwkInjector>> = std::cell::RefCell::new(Arc::new(default_fetch_jwks));
    }

    pub(super) fn get_jwk_injector() -> Arc<JwkInjector> {
        JWK_INJECTOR.with(|injector| injector.borrow().clone())
    }

    pub fn set_jwk_injector(injector: Arc<JwkInjector>) {
        JWK_INJECTOR.with(|cell| *cell.borrow_mut() = injector);
    }
}

use ika_core::authority::authority_perpetual_tables::AuthorityPerpetualTables;
use ika_core::consensus_handler::ConsensusHandlerInitializer;
use ika_core::dwallet_mpc::dwallet_mpc_service::DWalletMPCService;
use ika_core::dwallet_mpc::mpc_manager::DWalletMPCManager;
use ika_core::dwallet_mpc::mpc_outputs_verifier::DWalletMPCOutputsVerifier;
use ika_core::dwallet_mpc::network_dkg::{
    DwalletMPCNetworkKeys, ValidatorPrivateDecryptionKeyData,
};
use ika_core::sui_connector::metrics::SuiConnectorMetrics;
use ika_core::sui_connector::sui_executor::StopReason;
use ika_core::sui_connector::SuiConnectorService;
use ika_sui_client::metrics::SuiClientMetrics;
use ika_sui_client::{SuiBridgeClient, SuiClient};
use ika_types::messages_dwallet_mpc::IkaPackagesConfig;
#[cfg(msim)]
pub use simulator::set_jwk_injector;
#[cfg(msim)]
use simulator::*;
use sui_types::execution_config_utils::to_binary_config;

pub struct IkaNode {
    config: NodeConfig,
    validator_components: Mutex<Option<ValidatorComponents>>,

    state: Arc<AuthorityState>,
    registry_service: RegistryService,
    metrics: Arc<IkaNodeMetrics>,

    _discovery: discovery::Handle,
    _connection_monitor_handle: consensus_core::ConnectionMonitorHandle,
    state_sync_handle: state_sync::Handle,
    checkpoint_store: Arc<CheckpointStore>,
    connection_monitor_status: Arc<ConnectionMonitorStatus>,

    /// Broadcast channel to send the starting system state for the next epoch.
    end_of_epoch_channel: broadcast::Sender<SystemInner>,

    /// Broadcast channel to notify state-sync for new validator peers.
    trusted_peer_change_tx: watch::Sender<TrustedPeerChangeEvent>,

    #[cfg(msim)]
    sim_state: SimState,

    sui_connector_service: Arc<SuiConnectorService>,

    _state_archive_handle: Option<broadcast::Sender<()>>,

    shutdown_channel_tx: broadcast::Sender<Option<RunWithRange>>,
}

impl fmt::Debug for IkaNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("IkaNode")
            .field("name", &self.state.name.concise())
            .finish()
    }
}

impl IkaNode {
    pub async fn start(
        config: NodeConfig,
        registry_service: RegistryService,
        custom_rpc_runtime: Option<Handle>,
    ) -> Result<Arc<IkaNode>> {
        Self::start_async(config, registry_service, "unknown").await
    }

    pub async fn start_async(
        config: NodeConfig,
        registry_service: RegistryService,
        software_version: &'static str,
    ) -> Result<Arc<IkaNode>> {
        NodeConfigMetrics::new(&registry_service.default_registry()).record_metrics(&config);
        let mut config = config.clone();
        if config.supported_protocol_versions.is_none() {
            info!(
                "populating config.supported_protocol_versions with default {:?}",
                SupportedProtocolVersions::SYSTEM_DEFAULT
            );
            config.supported_protocol_versions = Some(SupportedProtocolVersions::SYSTEM_DEFAULT);
        }

        let prometheus_registry = registry_service.default_registry();

        info!(node =? config.protocol_public_key(),
            "Initializing ika-node listening on {}", config.network_address
        );

        // Initialize metrics to track db usage before creating any stores
        DBMetrics::init(&prometheus_registry);

        // Initialize Mysten metrics.
        mysten_metrics::init_metrics(&prometheus_registry);
        // Unsupported (because of the use of static variable) and unnecessary in simtests.
        #[cfg(not(msim))]
        mysten_metrics::thread_stall_monitor::start_thread_stall_monitor();

        let sui_client_metrics = SuiClientMetrics::new(&registry_service.default_registry());

        let sui_client = Arc::new(
            SuiClient::new(
                &config.sui_connector_config.sui_rpc_url,
                sui_client_metrics,
                config.sui_connector_config.ika_package_id,
                config.sui_connector_config.ika_system_package_id,
                config.sui_connector_config.ika_system_object_id,
            )
            .await?,
        );

        let latest_system_state = sui_client.get_system_inner_until_success().await;
        let epoch_start_system_state = sui_client
            .get_epoch_start_system_until_success(&latest_system_state)
            .await;

        let previous_epoch_last_checkpoint_sequence_number =
            latest_system_state.previous_epoch_last_checkpoint_sequence_number();

        let committee = Arc::new(epoch_start_system_state.get_ika_committee());

        let secret = Arc::pin(config.protocol_key_pair().copy());
        //let genesis_committee = genesis.committee()?;
        let committee_store = Arc::new(CommitteeStore::new(
            config.db_path().join("epochs"),
            &committee,
            None,
        ));
        let perpetual_tables_options = default_db_options().optimize_db_for_write_throughput(4);
        let perpetual_tables = Arc::new(AuthorityPerpetualTables::open(
            &config.db_path().join("store"),
            Some(perpetual_tables_options.options),
        ));

        //let cur_epoch = latest_system_state.epoch();
        // let committee = committee_store
        //     .get_committee(&cur_epoch)?
        //     .expect("Committee of the current epoch must exist");
        let chain_identifier = ChainIdentifier::from(config.sui_connector_config.ika_system_object_id);

        let epoch_start_configuration = EpochStartConfiguration::new(epoch_start_system_state)
            .expect("EpochStartConfiguration construction cannot fail");

        // let epoch_start_configuration = store
        //     .get_epoch_start_configuration()?
        //     .expect("EpochStartConfiguration of the current epoch must exist");

        let epoch_options = default_db_options().optimize_db_for_write_throughput(4);
        let packages_config = IkaPackagesConfig {
            ika_package_id: config.sui_connector_config.ika_package_id,
            ika_system_package_id: config.sui_connector_config.ika_system_package_id,
            ika_system_object_id: config.sui_connector_config.ika_system_object_id,
        };
        let epoch_store = AuthorityPerEpochStore::new(
            config.protocol_public_key(),
            committee.clone(),
            &config.db_path().join("store"),
            Some(epoch_options.options),
            EpochMetrics::new(&registry_service.default_registry()),
            epoch_start_configuration,
            chain_identifier.clone(),
            perpetual_tables.clone(),
            packages_config,
        );

        info!("created epoch store");

        replay_log!(
            "Beginning replay run. Epoch: {:?}, Protocol config: {:?}",
            epoch_store.epoch(),
            epoch_store.protocol_config()
        );

        let effective_buffer_stake = epoch_store.get_effective_buffer_stake_bps();
        let default_buffer_stake = epoch_store
            .protocol_config()
            .buffer_stake_for_protocol_upgrade_bps();
        if effective_buffer_stake != default_buffer_stake {
            warn!(
                ?effective_buffer_stake,
                ?default_buffer_stake,
                "buffer_stake_for_protocol_upgrade_bps is currently overridden"
            );
        }

        info!("creating checkpoint store");

        let checkpoint_store = CheckpointStore::new(&config.db_path().join("checkpoints"));
        // checkpoint_store.insert_genesis_checkpoint(
        //     genesis.checkpoint(),
        //     genesis.checkpoint_contents().clone(),
        //     &epoch_store,
        // );

        info!("creating state sync store");
        let state_sync_store = RocksDbStore::new(committee_store.clone(), checkpoint_store.clone());

        let sui_connector_metrics = SuiConnectorMetrics::new(&registry_service.default_registry());
        let is_validator = config.consensus_config.is_some();
        let dwallet_network_keys = if is_validator {
            let party_id = epoch_store.authority_name_to_party_id(&config.protocol_public_key())?;
            let validator_private_data = ValidatorPrivateDecryptionKeyData {
                party_id,
                class_groups_decryption_key: config
                    .class_groups_key_pair_and_proof
                    .class_groups_keypair()
                    .decryption_key(),
                validator_decryption_key_shares: RwLock::new(HashMap::new()),
            };
            let dwallet_network_keys = DwalletMPCNetworkKeys::new(validator_private_data);
            let dwallet_network_keys_arc = Arc::new(dwallet_network_keys);
            Some(dwallet_network_keys_arc)
        } else {
            None
        };

        let sui_connector_service = Arc::new(
            SuiConnectorService::new(
                perpetual_tables.clone(),
                checkpoint_store.clone(),
                sui_client.clone(),
                config.sui_connector_config.clone(),
                sui_connector_metrics,
                dwallet_network_keys.clone(),
                epoch_store.get_weighted_threshold_access_structure()?,
            )
            .await?,
        );

        info!("creating archive reader");
        // Create network
        // TODO only configure validators as seed/preferred peers for validators and not for
        // fullnodes once we've had a chance to re-work fullnode configuration generation.
        let archive_readers =
            ArchiveReaderBalancer::new(config.archive_reader_config(), &prometheus_registry)?;
        let (trusted_peer_change_tx, trusted_peer_change_rx) = watch::channel(Default::default());
        let P2pComponents {
            p2p_network,
            known_peers,
            discovery_handle,
            state_sync_handle,
        } = Self::create_p2p_network(
            &config,
            state_sync_store.clone(),
            chain_identifier,
            trusted_peer_change_rx,
            archive_readers.clone(),
            &prometheus_registry,
        )?;

        // We must explicitly send this instead of relying on the initial value to trigger
        // watch value change, so that state-sync is able to process it.
        send_trusted_peer_change(
            &config,
            &trusted_peer_change_tx,
            epoch_store.epoch_start_state(),
        )
        .expect("Initial trusted peers must be set");

        info!("start state archival");
        // Start archiving local state to remote store
        let state_archive_handle =
            Self::start_state_archival(&config, &prometheus_registry, state_sync_store.clone())
                .await?;

        let authority_name = config.protocol_public_key();

        info!("create authority state");
        let state = AuthorityState::new(
            authority_name,
            secret,
            config.supported_protocol_versions.unwrap(),
            perpetual_tables.clone(),
            epoch_store.clone(),
            committee_store.clone(),
            checkpoint_store.clone(),
            &prometheus_registry,
            config.clone(),
        )
        .await;

        info!("created authority state");

        let (end_of_epoch_channel, end_of_epoch_receiver) =
            broadcast::channel(config.end_of_epoch_broadcast_channel_capacity);

        let authority_names_to_peer_ids = epoch_store
            .epoch_start_state()
            .get_authority_names_to_peer_ids();

        let network_connection_metrics = consensus_core::QuinnConnectionMetrics::new(
            "ika",
            &registry_service.default_registry(),
        );

        let authority_names_to_peer_ids = ArcSwap::from_pointee(authority_names_to_peer_ids);

        let connection_monitor_handle = consensus_core::AnemoConnectionMonitor::spawn(
            p2p_network.downgrade(),
            Arc::new(network_connection_metrics),
            known_peers,
        );

        let connection_monitor_status = ConnectionMonitorStatus {
            connection_statuses: connection_monitor_handle.connection_statuses(),
            authority_names_to_peer_ids,
        };

        let connection_monitor_status = Arc::new(connection_monitor_status);
        let ika_node_metrics = Arc::new(IkaNodeMetrics::new(&registry_service.default_registry()));

        let validator_components = if state.is_validator(&epoch_store) {
            let components = Self::construct_validator_components(
                config.clone(),
                state.clone(),
                committee,
                epoch_store.clone(),
                checkpoint_store.clone(),
                state_sync_handle.clone(),
                connection_monitor_status.clone(),
                &registry_service,
                ika_node_metrics.clone(),
                previous_epoch_last_checkpoint_sequence_number,
                // Safe to unwrap() because the node is a Validator.
                dwallet_network_keys.clone().unwrap(),
                sui_client.clone(),
            )
            .await?;
            // This is only needed during cold start.
            components.consensus_adapter.submit_recovered(&epoch_store);

            Some(components)
        } else {
            None
        };

        // setup shutdown channel
        let (shutdown_channel, _) = broadcast::channel::<Option<RunWithRange>>(1);

        let node = Self {
            config,
            validator_components: Mutex::new(validator_components),
            state,
            registry_service,
            metrics: ika_node_metrics,

            _discovery: discovery_handle,
            _connection_monitor_handle: connection_monitor_handle,
            state_sync_handle,
            checkpoint_store,

            end_of_epoch_channel,
            connection_monitor_status,
            trusted_peer_change_tx,

            #[cfg(msim)]
            sim_state: Default::default(),

            sui_connector_service,
            _state_archive_handle: state_archive_handle,
            shutdown_channel_tx: shutdown_channel,
        };

        info!("IkaNode started!");
        let node = Arc::new(node);
        let node_copy = node.clone();
        let perpetual_tables_copy = perpetual_tables.clone();
        let sui_client_clone = sui_client.clone();
        spawn_monitored_task!(async move {
            let result = Self::monitor_reconfiguration(
                node_copy,
                perpetual_tables_copy,
                dwallet_network_keys.clone(),
                sui_client_clone,
            )
            .await;
            if let Err(error) = result {
                warn!("Reconfiguration finished with error {:?}", error);
            }
        });

        Ok(node)
    }

    pub fn subscribe_to_epoch_change(&self) -> broadcast::Receiver<SystemInner> {
        self.end_of_epoch_channel.subscribe()
    }

    pub fn subscribe_to_shutdown_channel(&self) -> broadcast::Receiver<Option<RunWithRange>> {
        self.shutdown_channel_tx.subscribe()
    }

    pub fn current_epoch_for_testing(&self) -> EpochId {
        self.state.current_epoch_for_testing()
    }

    pub fn db_checkpoint_path(&self) -> PathBuf {
        self.config.db_checkpoint_path()
    }

    pub fn clear_override_protocol_upgrade_buffer_stake(&self, epoch: EpochId) -> IkaResult {
        self.state
            .clear_override_protocol_upgrade_buffer_stake(epoch)
    }

    pub fn set_override_protocol_upgrade_buffer_stake(
        &self,
        epoch: EpochId,
        buffer_stake_bps: u64,
    ) -> IkaResult {
        self.state
            .set_override_protocol_upgrade_buffer_stake(epoch, buffer_stake_bps)
    }

    async fn start_state_archival(
        config: &NodeConfig,
        prometheus_registry: &Registry,
        state_sync_store: RocksDbStore,
    ) -> Result<Option<tokio::sync::broadcast::Sender<()>>> {
        if let Some(remote_store_config) = &config.state_archive_write_config.object_store_config {
            let local_store_config = ObjectStoreConfig {
                object_store: Some(ObjectStoreType::File),
                directory: Some(config.archive_path()),
                ..Default::default()
            };
            let archive_writer = ArchiveWriter::new(
                local_store_config,
                remote_store_config.clone(),
                FileCompression::Zstd,
                StorageFormat::Blob,
                Duration::from_secs(600),
                256 * 1024 * 1024,
                prometheus_registry,
            )
            .await?;
            Ok(Some(archive_writer.start(state_sync_store).await?))
        } else {
            Ok(None)
        }
    }

    fn create_p2p_network(
        config: &NodeConfig,
        state_sync_store: RocksDbStore,
        chain_identifier: ChainIdentifier,
        trusted_peer_change_rx: watch::Receiver<TrustedPeerChangeEvent>,
        archive_readers: ArchiveReaderBalancer,
        prometheus_registry: &Registry,
    ) -> Result<P2pComponents> {
        let (state_sync, state_sync_server) = state_sync::Builder::new()
            .config(config.p2p_config.state_sync.clone().unwrap_or_default())
            .store(state_sync_store)
            .archive_readers(archive_readers)
            .with_metrics(prometheus_registry)
            .build();

        let (discovery, discovery_server) = discovery::Builder::new(trusted_peer_change_rx)
            .config(config.p2p_config.clone())
            .build();

        let discovery_config = config.p2p_config.discovery.clone().unwrap_or_default();
        let known_peers: HashMap<PeerId, String> = discovery_config
            .allowlisted_peers
            .clone()
            .into_iter()
            .map(|ap| (ap.peer_id, "allowlisted_peer".to_string()))
            .chain(config.p2p_config.seed_peers.iter().filter_map(|peer| {
                peer.peer_id
                    .map(|peer_id| (peer_id, "seed_peer".to_string()))
            }))
            .collect();

        let p2p_network = {
            let routes = anemo::Router::new()
                .add_rpc_service(discovery_server)
                .add_rpc_service(state_sync_server);
            let inbound_network_metrics =
                consensus_core::NetworkRouteMetrics::new("ika", "inbound", prometheus_registry);
            let outbound_network_metrics =
                consensus_core::NetworkRouteMetrics::new("ika", "outbound", prometheus_registry);

            let service = ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_server_errors()
                        .make_span_with(DefaultMakeSpan::new().level(tracing::Level::INFO))
                        .on_failure(DefaultOnFailure::new().level(tracing::Level::WARN)),
                )
                .layer(CallbackLayer::new(
                    consensus_core::MetricsMakeCallbackHandler::new(
                        Arc::new(inbound_network_metrics),
                        config.p2p_config.excessive_message_size(),
                    ),
                ))
                .service(routes);

            let outbound_layer = ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_client_and_server_errors()
                        .make_span_with(DefaultMakeSpan::new().level(tracing::Level::INFO))
                        .on_failure(DefaultOnFailure::new().level(tracing::Level::WARN)),
                )
                .layer(CallbackLayer::new(
                    consensus_core::MetricsMakeCallbackHandler::new(
                        Arc::new(outbound_network_metrics),
                        config.p2p_config.excessive_message_size(),
                    ),
                ))
                .into_inner();

            let mut anemo_config = config.p2p_config.anemo_config.clone().unwrap_or_default();
            // Set the max_frame_size to be 1 GB to work around the issue of there being too many
            // staking events in the epoch change txn.
            anemo_config.max_frame_size = Some(1 << 30);

            // Set a higher default value for socket send/receive buffers if not already
            // configured.
            let mut quic_config = anemo_config.quic.unwrap_or_default();
            if quic_config.socket_send_buffer_size.is_none() {
                quic_config.socket_send_buffer_size = Some(20 << 20);
            }
            if quic_config.socket_receive_buffer_size.is_none() {
                quic_config.socket_receive_buffer_size = Some(20 << 20);
            }
            quic_config.allow_failed_socket_buffer_size_setting = true;

            // Set high-performance defaults for quinn transport.
            // With 200MiB buffer size and ~500ms RTT, max throughput ~400MiB/s.
            if quic_config.max_concurrent_bidi_streams.is_none() {
                quic_config.max_concurrent_bidi_streams = Some(500);
            }
            if quic_config.max_concurrent_uni_streams.is_none() {
                quic_config.max_concurrent_uni_streams = Some(500);
            }
            if quic_config.stream_receive_window.is_none() {
                quic_config.stream_receive_window = Some(100 << 20);
            }
            if quic_config.receive_window.is_none() {
                quic_config.receive_window = Some(200 << 20);
            }
            if quic_config.send_window.is_none() {
                quic_config.send_window = Some(200 << 20);
            }
            if quic_config.crypto_buffer_size.is_none() {
                quic_config.crypto_buffer_size = Some(1 << 20);
            }
            if quic_config.max_idle_timeout_ms.is_none() {
                quic_config.max_idle_timeout_ms = Some(30_000);
            }
            if quic_config.keep_alive_interval_ms.is_none() {
                quic_config.keep_alive_interval_ms = Some(5_000);
            }
            anemo_config.quic = Some(quic_config);

            let server_name = format!("ika-{}", chain_identifier);
            let network = Network::bind(config.p2p_config.listen_address)
                .server_name(&server_name)
                .private_key(config.network_key_pair().copy().private().0.to_bytes())
                .config(anemo_config)
                .outbound_request_layer(outbound_layer)
                .start(service)?;
            info!(
                server_name = server_name,
                "P2p network started on {}",
                network.local_addr()
            );

            network
        };

        let discovery_handle =
            discovery.start(p2p_network.clone(), config.network_key_pair().copy());
        let state_sync_handle = state_sync.start(p2p_network.clone());

        Ok(P2pComponents {
            p2p_network,
            known_peers,
            discovery_handle,
            state_sync_handle,
        })
    }

    async fn construct_validator_components(
        config: NodeConfig,
        state: Arc<AuthorityState>,
        committee: Arc<Committee>,
        epoch_store: Arc<AuthorityPerEpochStore>,
        checkpoint_store: Arc<CheckpointStore>,
        state_sync_handle: state_sync::Handle,
        connection_monitor_status: Arc<ConnectionMonitorStatus>,
        registry_service: &RegistryService,
        ika_node_metrics: Arc<IkaNodeMetrics>,
        previous_epoch_last_checkpoint_sequence_number: u64,
        network_keys: Arc<DwalletMPCNetworkKeys>,
        sui_client: Arc<SuiBridgeClient>,
    ) -> Result<ValidatorComponents> {
        let mut config_clone = config.clone();
        let consensus_config = config_clone
            .consensus_config
            .as_mut()
            .ok_or_else(|| anyhow!("Validator is missing consensus config"))?;

        let client = Arc::new(UpdatableConsensusClient::new());
        let consensus_adapter = Arc::new(Self::construct_consensus_adapter(
            &committee,
            consensus_config,
            state.name,
            connection_monitor_status.clone(),
            &registry_service.default_registry(),
            epoch_store.protocol_config().clone(),
            client.clone(),
        ));
        let consensus_manager =
            ConsensusManager::new(&config, consensus_config, registry_service, client);

        // This only gets started up once, not on every epoch. (Make call to remove every epoch.)
        let consensus_store_pruner = ConsensusStorePruner::new(
            consensus_manager.get_storage_base_path(),
            consensus_config.db_retention_epochs(),
            consensus_config.db_pruner_period(),
            &registry_service.default_registry(),
        );

        let checkpoint_metrics = CheckpointMetrics::new(&registry_service.default_registry());
        let ika_tx_validator_metrics =
            IkaTxValidatorMetrics::new(&registry_service.default_registry());

        Self::start_epoch_specific_validator_components(
            &config,
            state.clone(),
            consensus_adapter,
            checkpoint_store,
            epoch_store,
            state_sync_handle,
            consensus_manager,
            consensus_store_pruner,
            checkpoint_metrics,
            ika_node_metrics,
            ika_tx_validator_metrics,
            previous_epoch_last_checkpoint_sequence_number,
            network_keys,
            sui_client,
        )
        .await
    }

    async fn start_epoch_specific_validator_components(
        config: &NodeConfig,
        state: Arc<AuthorityState>,
        consensus_adapter: Arc<ConsensusAdapter>,
        checkpoint_store: Arc<CheckpointStore>,
        epoch_store: Arc<AuthorityPerEpochStore>,
        state_sync_handle: state_sync::Handle,
        consensus_manager: ConsensusManager,
        consensus_store_pruner: ConsensusStorePruner,
        checkpoint_metrics: Arc<CheckpointMetrics>,
        ika_node_metrics: Arc<IkaNodeMetrics>,
        ika_tx_validator_metrics: Arc<IkaTxValidatorMetrics>,
        previous_epoch_last_checkpoint_sequence_number: u64,
        network_keys: Arc<DwalletMPCNetworkKeys>,
        sui_client: Arc<SuiBridgeClient>,
    ) -> Result<ValidatorComponents> {
        let (checkpoint_service, checkpoint_service_tasks) = Self::start_checkpoint_service(
            config,
            consensus_adapter.clone(),
            checkpoint_store,
            epoch_store.clone(),
            state.clone(),
            state_sync_handle,
            checkpoint_metrics.clone(),
            previous_epoch_last_checkpoint_sequence_number,
        );

        let dwallet_mpc_service_exit =
            Self::start_dwallet_mpc_service(epoch_store.clone(), sui_client);

        // Start the dWallet MPC manager on epoch start.
        epoch_store.set_dwallet_mpc_network_keys(network_keys)?;
        // This verifier is in sync with the consensus,
        // used to verify outputs before sending a system TX to store them.
        epoch_store
            .set_dwallet_mpc_outputs_verifier(DWalletMPCOutputsVerifier::new(&epoch_store))?;

        epoch_store.set_dwallet_mpc_manager(DWalletMPCManager::try_new(
            Arc::new(consensus_adapter.clone()),
            Arc::clone(&epoch_store),
            epoch_store.epoch(),
            config.clone(),
        )?)?;

        // create a new map that gets injected into both the consensus handler and the consensus adapter
        // the consensus handler will write values forwarded from consensus, and the consensus adapter
        // will read the values to make decisions about which validator submits a transaction to consensus
        let low_scoring_authorities = Arc::new(ArcSwap::new(Arc::new(HashMap::new())));

        consensus_adapter.swap_low_scoring_authorities(low_scoring_authorities.clone());

        let throughput_calculator = Arc::new(ConsensusThroughputCalculator::new(
            None,
            state.metrics.clone(),
        ));

        let throughput_profiler = Arc::new(ConsensusThroughputProfiler::new(
            throughput_calculator.clone(),
            None,
            None,
            state.metrics.clone(),
            ThroughputProfileRanges::from_chain(epoch_store.get_chain_identifier()),
        ));

        consensus_adapter.swap_throughput_profiler(throughput_profiler);

        let consensus_handler_initializer = ConsensusHandlerInitializer::new(
            state.clone(),
            checkpoint_service.clone(),
            epoch_store.clone(),
            low_scoring_authorities,
            throughput_calculator,
        );

        consensus_manager
            .start(
                config,
                epoch_store.clone(),
                consensus_handler_initializer,
                IkaTxValidator::new(
                    state.clone(),
                    consensus_adapter.clone(),
                    checkpoint_service.clone(),
                    ika_tx_validator_metrics.clone(),
                ),
            )
            .await;

        Ok(ValidatorComponents {
            consensus_manager,
            consensus_store_pruner,
            consensus_adapter,
            checkpoint_service_tasks,
            checkpoint_metrics,
            ika_tx_validator_metrics,
            dwallet_mpc_service_exit,
        })
    }

    fn start_checkpoint_service(
        config: &NodeConfig,
        consensus_adapter: Arc<ConsensusAdapter>,
        checkpoint_store: Arc<CheckpointStore>,
        epoch_store: Arc<AuthorityPerEpochStore>,
        state: Arc<AuthorityState>,
        state_sync_handle: state_sync::Handle,
        checkpoint_metrics: Arc<CheckpointMetrics>,
        previous_epoch_last_checkpoint_sequence_number: u64,
    ) -> (Arc<CheckpointService>, JoinSet<()>) {
        let epoch_start_timestamp_ms = epoch_store.epoch_start_state().epoch_start_timestamp_ms();
        let epoch_duration_ms = epoch_store.epoch_start_state().epoch_duration_ms();

        debug!(
            "Starting checkpoint service with epoch start timestamp {}
            and epoch duration {}",
            epoch_start_timestamp_ms, epoch_duration_ms
        );

        let checkpoint_output = Box::new(SubmitCheckpointToConsensus {
            sender: consensus_adapter,
            signer: state.secret.clone(),
            authority: config.protocol_public_key(),
            metrics: checkpoint_metrics.clone(),
        });

        let certified_checkpoint_output = SendCheckpointToStateSync::new(state_sync_handle);
        let max_tx_per_checkpoint = max_tx_per_checkpoint(epoch_store.protocol_config());
        let max_checkpoint_size_bytes =
            epoch_store.protocol_config().max_checkpoint_size_bytes() as usize;

        CheckpointService::spawn(
            state.clone(),
            checkpoint_store,
            epoch_store,
            checkpoint_output,
            Box::new(certified_checkpoint_output),
            checkpoint_metrics,
            max_tx_per_checkpoint,
            max_checkpoint_size_bytes,
            previous_epoch_last_checkpoint_sequence_number,
        )
    }

    fn construct_consensus_adapter(
        committee: &Committee,
        consensus_config: &ConsensusConfig,
        authority: AuthorityName,
        connection_monitor_status: Arc<ConnectionMonitorStatus>,
        prometheus_registry: &Registry,
        protocol_config: ProtocolConfig,
        consensus_client: Arc<dyn ConsensusClient>,
    ) -> ConsensusAdapter {
        let ca_metrics = ConsensusAdapterMetrics::new(prometheus_registry);
        // The consensus adapter allows the authority to send user certificates through consensus.

        ConsensusAdapter::new(
            consensus_client,
            authority,
            connection_monitor_status,
            consensus_config.max_pending_transactions(),
            consensus_config.max_pending_transactions() * 2 / committee.num_members(),
            consensus_config.max_submit_position,
            consensus_config.submit_delay_step_override(),
            ca_metrics,
            protocol_config,
        )
    }

    pub fn state(&self) -> Arc<AuthorityState> {
        self.state.clone()
    }

    pub fn clone_committee_store(&self) -> Arc<CommitteeStore> {
        self.state.committee_store().clone()
    }

    /*
    pub fn clone_authority_store(&self) -> Arc<AuthorityStore> {
        self.state.db()
    }
    */

    /// This function awaits the completion of checkpoint execution of the current epoch,
    /// after which it iniitiates reconfiguration of the entire system.
    pub async fn monitor_reconfiguration(
        self: Arc<Self>,
        perpetual_tables: Arc<AuthorityPerpetualTables>,
        dwallet_network_keys: Option<Arc<DwalletMPCNetworkKeys>>,
        sui_client: Arc<SuiBridgeClient>,
    ) -> Result<()> {
        let sui_client_clone2 = sui_client.clone();
        loop {
            let run_with_range = self.config.run_with_range;

            let cur_epoch_store = self.state.load_epoch_store_one_call_per_task();

            let stop_condition = self
                .sui_connector_service
                .run_epoch(cur_epoch_store.epoch(), run_with_range)
                .await;

            let (latest_system_state, epoch_start_system_state) = match stop_condition {
                StopReason::EpochComplete(latest_system_state, epoch_start_system_state) => {
                    (latest_system_state, epoch_start_system_state)
                }
                StopReason::RunWithRangeCondition => {
                    IkaNode::shutdown(&self).await;
                    self.shutdown_channel_tx
                        .send(run_with_range)
                        .expect("RunWithRangeCondition met but failed to send shutdown message");
                    return Ok(());
                }
            };

            // // Safe to call because we are in the middle of reconfiguration.
            // let latest_system_state = self
            //     .state
            //     .get_object_cache_reader()
            //     .get_ika_system_state_object_unsafe()
            //     .expect("Read Ika System State object cannot fail");

            #[cfg(msim)]
            if !self
                .sim_state
                .sim_safe_mode_expected
                .load(Ordering::Relaxed)
            {
                debug_assert!(!latest_system_state.safe_mode());
            }

            if let Err(err) = self.end_of_epoch_channel.send(latest_system_state.clone()) {
                if self.state.is_fullnode(&cur_epoch_store) {
                    warn!(
                        "Failed to send end of epoch notification to subscriber: {:?}",
                        err
                    );
                }
            }

            let previous_epoch_last_checkpoint_sequence_number =
                latest_system_state.previous_epoch_last_checkpoint_sequence_number();

            let next_epoch_committee = epoch_start_system_state.get_ika_committee();
            let next_epoch = next_epoch_committee.epoch();
            assert_eq!(cur_epoch_store.epoch() + 1, next_epoch);

            info!(
                next_epoch,
                "Finished executing all checkpoints in epoch. About to reconfigure the system."
            );

            fail_point_async!("reconfig_delay");

            // We save the connection monitor status map regardless of validator / fullnode status
            // so that we don't need to restart the connection monitor every epoch.
            // Update the mappings that will be used by the consensus adapter if it exists or is
            // about to be created.
            let authority_names_to_peer_ids =
                epoch_start_system_state.get_authority_names_to_peer_ids();
            self.connection_monitor_status
                .update_mapping_for_epoch(authority_names_to_peer_ids);

            cur_epoch_store.record_epoch_reconfig_start_time_metric();

            let _ = send_trusted_peer_change(
                &self.config,
                &self.trusted_peer_change_tx,
                &epoch_start_system_state,
            );

            // The following code handles 4 different cases, depending on whether the node
            // was a validator in the previous epoch, and whether the node is a validator
            // in the new epoch.

            let new_validator_components = if let Some(ValidatorComponents {
                consensus_manager,
                consensus_store_pruner,
                consensus_adapter,
                mut checkpoint_service_tasks,
                checkpoint_metrics,
                ika_tx_validator_metrics,
                dwallet_mpc_service_exit,
            }) = self.validator_components.lock().await.take()
            {
                info!("Reconfiguring the validator.");
                // Cancel the old checkpoint service tasks.
                // Waiting for checkpoint builder to finish gracefully is not possible, because it
                // may wait on transactions while consensus on peers have already shut down.
                checkpoint_service_tasks.abort_all();
                drop(dwallet_mpc_service_exit);
                while let Some(result) = checkpoint_service_tasks.join_next().await {
                    if let Err(err) = result {
                        if err.is_panic() {
                            std::panic::resume_unwind(err.into_panic());
                        }
                        warn!("Error in checkpoint service task: {:?}", err);
                    }
                }
                info!("Checkpoint service has shut down.");

                consensus_manager.shutdown().await;
                info!("Consensus has shut down.");

                let new_epoch_store = self
                    .reconfigure_state(
                        &cur_epoch_store,
                        next_epoch_committee.clone(),
                        epoch_start_system_state,
                        perpetual_tables.clone(),
                    )
                    .await;
                info!("Epoch store finished reconfiguration.");

                consensus_store_pruner.prune(next_epoch).await;

                if self.state.is_validator(&new_epoch_store) {
                    // Only restart consensus if this node is still a validator in the new epoch.
                    Some(
                        Self::start_epoch_specific_validator_components(
                            &self.config,
                            self.state.clone(),
                            consensus_adapter,
                            self.checkpoint_store.clone(),
                            new_epoch_store.clone(),
                            self.state_sync_handle.clone(),
                            consensus_manager,
                            consensus_store_pruner,
                            checkpoint_metrics,
                            self.metrics.clone(),
                            ika_tx_validator_metrics,
                            previous_epoch_last_checkpoint_sequence_number,
                            // safe to unwrap because we are a validator
                            dwallet_network_keys.clone().unwrap(),
                            sui_client_clone2.clone(),
                        )
                        .await?,
                    )
                } else {
                    info!("This node is no longer a validator after reconfiguration");
                    None
                }
            } else {
                let new_epoch_store = self
                    .reconfigure_state(
                        &cur_epoch_store,
                        next_epoch_committee.clone(),
                        epoch_start_system_state,
                        perpetual_tables.clone(),
                    )
                    .await;

                if self.state.is_validator(&new_epoch_store) {
                    info!("Promoting the node from fullnode to validator, starting grpc server");

                    Some(
                        Self::construct_validator_components(
                            self.config.clone(),
                            self.state.clone(),
                            Arc::new(next_epoch_committee.clone()),
                            new_epoch_store.clone(),
                            self.checkpoint_store.clone(),
                            self.state_sync_handle.clone(),
                            self.connection_monitor_status.clone(),
                            &self.registry_service,
                            self.metrics.clone(),
                            previous_epoch_last_checkpoint_sequence_number,
                            // safe to unwrap because we are a validator
                            dwallet_network_keys.clone().unwrap(),
                            sui_client.clone(),
                        )
                        .await?,
                    )
                } else {
                    None
                }
            };
            *self.validator_components.lock().await = new_validator_components;

            // Force releasing current epoch store DB handle, because the
            // Arc<AuthorityPerEpochStore> may linger.
            cur_epoch_store.release_db_handles();

            info!("Reconfiguration finished");
        }
    }

    async fn shutdown(&self) {
        if let Some(validator_components) = &*self.validator_components.lock().await {
            validator_components.consensus_manager.shutdown().await;
        }
    }

    async fn reconfigure_state(
        &self,
        cur_epoch_store: &AuthorityPerEpochStore,
        next_epoch_committee: Committee,
        next_epoch_start_system_state: EpochStartSystem,
        perpetual_tables: Arc<AuthorityPerpetualTables>,
    ) -> Arc<AuthorityPerEpochStore> {
        let next_epoch = next_epoch_committee.epoch();

        let epoch_start_configuration = EpochStartConfiguration::new(next_epoch_start_system_state)
            .expect("EpochStartConfiguration construction cannot fail");

        let new_epoch_store = self
            .state
            .reconfigure(
                cur_epoch_store,
                self.config.supported_protocol_versions.unwrap(),
                next_epoch_committee,
                epoch_start_configuration,
                perpetual_tables,
            )
            .await
            .expect("Reconfigure authority state cannot fail");
        info!(next_epoch, "Node State has been reconfigured");
        assert_eq!(next_epoch, new_epoch_store.epoch());

        new_epoch_store
    }

    pub fn get_config(&self) -> &NodeConfig {
        &self.config
    }

    fn start_dwallet_mpc_service(
        epoch_store: Arc<AuthorityPerEpochStore>,
        sui_client: Arc<SuiBridgeClient>,
    ) -> watch::Sender<()> {
        let (exit_sender, exit_receiver) = watch::channel(());
        let mut service = DWalletMPCService::new(epoch_store.clone(), exit_receiver);
        spawn_monitored_task!(service.spawn(sui_client));

        exit_sender
    }
}

#[cfg(msim)]
impl IkaNode {
    pub fn get_sim_node_id(&self) -> ika_simulator::task::NodeId {
        self.sim_state.sim_node.id()
    }

    pub fn set_safe_mode_expected(&self, new_value: bool) {
        info!("Setting safe mode expected to {}", new_value);
        self.sim_state
            .sim_safe_mode_expected
            .store(new_value, Ordering::Relaxed);
    }

    #[allow(unused_variables)]
    async fn fetch_jwks(
        authority: AuthorityName,
        provider: &OIDCProvider,
    ) -> IkaResult<Vec<(JwkId, JWK)>> {
        get_jwk_injector()(authority, provider)
    }
}

/// Notify state-sync that a new list of trusted peers are now available.
fn send_trusted_peer_change(
    config: &NodeConfig,
    sender: &watch::Sender<TrustedPeerChangeEvent>,
    epoch_state_state: &EpochStartSystem,
) -> Result<(), watch::error::SendError<TrustedPeerChangeEvent>> {
    sender
        .send(TrustedPeerChangeEvent {
            new_peers: epoch_state_state.get_validator_as_p2p_peers(config.protocol_public_key()),
        })
        .tap_err(|err| {
            warn!(
                "Failed to send validator peer information to state sync: {:?}",
                err
            );
        })
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Threshold {
    pub threshold_seconds: Option<u32>,
}

async fn health_check_handler(
    axum::extract::Query(Threshold { threshold_seconds }): axum::extract::Query<Threshold>,
    axum::Extension(state): axum::Extension<Arc<AuthorityState>>,
) -> impl axum::response::IntoResponse {
    if let Some(threshold_seconds) = threshold_seconds {
        // Attempt to get the latest checkpoint
        let summary = match state
            .get_checkpoint_store()
            .get_highest_executed_checkpoint()
        {
            Ok(Some(summary)) => summary,
            Ok(None) => {
                warn!("Highest executed checkpoint not found");
                return (axum::http::StatusCode::SERVICE_UNAVAILABLE, "down");
            }
            Err(err) => {
                warn!("Failed to retrieve highest executed checkpoint: {:?}", err);
                return (axum::http::StatusCode::SERVICE_UNAVAILABLE, "down");
            }
        };

        // Calculate the threshold time based on the provided threshold_seconds
        let latest_chain_time = summary.timestamp();
        let threshold =
            std::time::SystemTime::now() - Duration::from_secs(threshold_seconds as u64);

        // Check if the latest checkpoint is within the threshold
        if latest_chain_time < threshold {
            warn!(
                ?latest_chain_time,
                ?threshold,
                "failing healthcheck due to checkpoint lag"
            );
            return (axum::http::StatusCode::SERVICE_UNAVAILABLE, "down");
        }
    }
    // if health endpoint is responding and no threshold is given, respond success
    (axum::http::StatusCode::OK, "up")
}

#[cfg(not(test))]
fn max_tx_per_checkpoint(protocol_config: &ProtocolConfig) -> usize {
    protocol_config.max_messages_per_checkpoint() as usize
}

#[cfg(test)]
fn max_tx_per_checkpoint(_: &ProtocolConfig) -> usize {
    2
}
