// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::metrics::SuiClientMetrics;
use anyhow::anyhow;
use async_trait::async_trait;
use core::panic;
use dwallet_classgroups_types::{SingleEncryptionKeyAndProof, NUM_OF_CLASS_GROUPS_KEY_OBJECTS};
use ika_move_packages::BuiltInIkaMovePackages;
use ika_types::committee::ClassGroupsEncryptionKeyAndProof;
use ika_types::error::{IkaError, IkaResult};
use ika_types::messages_consensus::MovePackageDigest;
use ika_types::messages_dwallet_mpc::{
    DBSuiEvent, DWalletNetworkEncryptionKey, DWalletNetworkEncryptionKeyData,
    DWalletNetworkEncryptionKeyState,
};
use ika_types::sui::epoch_start_system::{EpochStartSystem, EpochStartValidatorInfoV1};
use ika_types::sui::staking::StakingPool;
use ika_types::sui::system_inner_v1::{DWalletCoordinatorInnerV1, SystemInnerV1};
use ika_types::sui::{
    DWalletCoordinator, DWalletCoordinatorInner, System, SystemInner, SystemInnerTrait, Validator,
};
use itertools::Itertools;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use sui_json_rpc_types::{EventFilter, Page, SuiEvent};
use sui_json_rpc_types::{
    EventPage, SuiObjectDataOptions, SuiTransactionBlockResponse,
    SuiTransactionBlockResponseOptions,
};
use sui_json_rpc_types::{SuiData, SuiObjectDataFilter, SuiObjectResponseQuery};
use sui_sdk::error::Error;
use sui_sdk::{SuiClient as SuiSdkClient, SuiClientBuilder};
use sui_types::base_types::{EpochId, ObjectRef};
use sui_types::clock::Clock;
use sui_types::collection_types::Table;
use sui_types::dynamic_field::Field;
use sui_types::gas_coin::GasCoin;
use sui_types::move_package::MovePackage;
use sui_types::object::Owner;
use sui_types::transaction::ObjectArg;
use sui_types::transaction::Transaction;
use sui_types::TypeTag;
use sui_types::{
    base_types::{ObjectID, SuiAddress},
    digests::TransactionDigest,
    event::EventID,
    Identifier,
};
use tokio::sync::OnceCell;
use tracing::{debug, error, info, warn};

pub mod ika_validator_transactions;
pub mod metrics;

#[macro_export]
macro_rules! retry_with_max_elapsed_time {
    ($func:expr, $max_elapsed_time:expr) => {{
        // The following delay sequence (in secs) will be used, applied with jitter
        // 0.4, 0.8, 1.6, 3.2, 6.4, 12.8, 25.6, 30, 60, 120, 120 ...
        let backoff = backoff::ExponentialBackoff {
            initial_interval: Duration::from_millis(400),
            randomization_factor: 0.1,
            multiplier: 2.0,
            max_interval: Duration::from_secs(120),
            max_elapsed_time: Some($max_elapsed_time),
            ..Default::default()
        };
        backoff::future::retry(backoff, || {
            let fut = async {
                let result = $func.await;
                match result {
                    Ok(_) => {
                        return Ok(result);
                    }
                    Err(err) => {
                        // For simplicity we treat every error as transient so we can retry until max_elapsed_time
                        error!(?err, "retrying with max elapsed time");
                        return Err(backoff::Error::transient(err));
                    }
                }
            };
            std::boxed::Box::pin(fut)
        })
        .await
    }};
}

pub struct SuiClient<P> {
    inner: P,
    sui_client_metrics: Arc<SuiClientMetrics>,
    ika_package_id: ObjectID,
    #[allow(dead_code)]
    ika_common_package_id: ObjectID,
    ika_dwallet_2pc_mpc_package_id: ObjectID,
    ika_system_package_id: ObjectID,
    ika_system_object_id: ObjectID,
    ika_dwallet_coordinator_object_id: ObjectID,
}

pub type SuiConnectorClient = SuiClient<SuiSdkClient>;

impl SuiConnectorClient {
    pub async fn new(
        rpc_url: &str,
        sui_client_metrics: Arc<SuiClientMetrics>,
        ika_package_id: ObjectID,
        ika_common_package_id: ObjectID,
        ika_dwallet_2pc_mpc_package_id: ObjectID,
        ika_system_package_id: ObjectID,
        ika_system_object_id: ObjectID,
        ika_dwallet_coordinator_object_id: ObjectID,
    ) -> anyhow::Result<Self> {
        let inner = SuiClientBuilder::default()
            .build(rpc_url)
            .await
            .map_err(|e| {
                anyhow!("Can't establish connection with Sui Rpc {rpc_url}. Error: {e}")
            })?;
        let self_ = Self {
            inner,
            sui_client_metrics,
            ika_package_id,
            ika_common_package_id,
            ika_dwallet_2pc_mpc_package_id,
            ika_system_package_id,
            ika_system_object_id,
            ika_dwallet_coordinator_object_id,
        };
        self_.describe().await?;
        Ok(self_)
    }

    pub fn sui_client(&self) -> &SuiSdkClient {
        &self.inner
    }
}

impl<P> SuiClient<P>
where
    P: SuiClientInner,
{
    pub async fn get_events_by_tx_digest(
        &self,
        tx_digest: TransactionDigest,
    ) -> anyhow::Result<Vec<SuiEvent>> {
        Ok(self.inner.get_events_by_tx_digest(tx_digest).await?)
    }

    /// Remaining sessions not processed during previous Epochs.
    pub async fn pull_dwallet_mpc_uncompleted_events(
        &self,
        epoch_id: EpochId,
    ) -> IkaResult<Vec<DBSuiEvent>> {
        loop {
            let dwallet_coordinator_inner = self.must_get_dwallet_coordinator_inner_v1().await;

            // Make sure we are synced with Sui to fetch the missed events.
            // If Sui's epoch number matches ours,
            // all the necessary missed events must be synced as well.
            // Note that we make sure that the coordinator's epoch number matches ours,
            // so that we know for sure that our Sui state is synced.
            if dwallet_coordinator_inner.current_epoch > epoch_id {
                return Err(IkaError::EpochEnded(epoch_id));
            }
            if dwallet_coordinator_inner.current_epoch != epoch_id {
                warn!(
                    sui_state_current_epoch=?dwallet_coordinator_inner.current_epoch,
                    our_current_epoch=?epoch_id,
                    "Sui's epoch number doesn't match ours "
                );
                tokio::time::sleep(Duration::from_secs(2)).await;
                continue;
            }

            let missed_events = self
                .inner
                .get_uncompleted_events(
                    dwallet_coordinator_inner
                        .session_management
                        .user_requested_sessions_events
                        .id
                        .id
                        .bytes,
                )
                .await
                .map_err(|e| {
                    error!("failed to get missed events: {e}");
                    IkaError::SuiClientInternalError(format!("failed to get missed events: {e}"))
                })?;

            if !missed_events.is_empty() {
                info!(
                    number_of_missed_events = missed_events.len(),
                    "retrieved missed events from Sui successfully"
                );
            } else {
                debug!("retrieved zero missed events from Sui");
            }

            return Ok(missed_events);
        }
    }

    pub fn new_for_testing(inner: P) -> Self {
        Self {
            inner,
            sui_client_metrics: SuiClientMetrics::new_for_testing(),
            // TODO(omersadika) fix that random
            ika_package_id: ObjectID::random(),
            ika_common_package_id: ObjectID::random(),
            ika_dwallet_2pc_mpc_package_id: ObjectID::random(),
            ika_system_package_id: ObjectID::random(),
            ika_system_object_id: ObjectID::random(),
            ika_dwallet_coordinator_object_id: ObjectID::random(),
        }
    }

    // TODO assert chain identifier
    async fn describe(&self) -> anyhow::Result<()> {
        let chain_id = self.inner.get_chain_identifier().await?;
        let checkpoint_sequence_number = self.inner.get_latest_checkpoint_sequence_number().await?;
        tracing::info!(
            "SuiClient is connected to chain {chain_id}, current checkpoint sequence number: {checkpoint_sequence_number}"
        );
        Ok(())
    }

    pub async fn get_dwallet_coordinator_inner(&self) -> IkaResult<DWalletCoordinatorInner> {
        let result = self
            .inner
            .get_dwallet_coordinator(self.ika_dwallet_coordinator_object_id)
            .await
            .map_err(|e| IkaError::SuiClientInternalError(format!("Can't get Coordinator: {e}")))?;
        let wrapper = bcs::from_bytes::<DWalletCoordinator>(&result).map_err(|e| {
            IkaError::SuiClientSerializationError(format!("Can't serialize Coordinator: {e}"))
        })?;

        match wrapper.version {
            1 => {
                let result = self
                    .inner
                    .get_dwallet_coordinator_inner(
                        self.ika_dwallet_coordinator_object_id,
                        wrapper.version,
                    )
                    .await
                    .map_err(|e| {
                        IkaError::SuiClientInternalError(format!(
                            "Can't get DWalletCoordinatorInner v1: {e}"
                        ))
                    })?;
                let dynamic_field_inner = bcs::from_bytes::<Field<u64, DWalletCoordinatorInnerV1>>(
                    &result,
                )
                .map_err(|e| {
                    IkaError::SuiClientSerializationError(format!(
                        "Can't serialize DWalletCoordinatorInner v1: {e}"
                    ))
                })?;
                let ika_system_state_inner = dynamic_field_inner.value;

                Ok(DWalletCoordinatorInner::V1(ika_system_state_inner))
            }
            _ => Err(IkaError::SuiClientInternalError(format!(
                "Unsupported DWalletCoordinatorInner version: {}",
                wrapper.version
            ))),
        }
    }

    pub async fn get_system_inner(&self) -> IkaResult<SystemInner> {
        let result = self
            .inner
            .get_system(self.ika_system_object_id)
            .await
            .map_err(|e| IkaError::SuiClientInternalError(format!("Can't get System: {e}")))?;
        let wrapper = bcs::from_bytes::<System>(&result).map_err(|e| {
            IkaError::SuiClientSerializationError(format!("Can't serialize System: {e}"))
        })?;

        match wrapper.version {
            1 => {
                let result = self
                    .inner
                    .get_system_inner(self.ika_system_object_id, wrapper.version)
                    .await
                    .map_err(|e| {
                        IkaError::SuiClientInternalError(format!("Can't get SystemInner v1: {e}"))
                    })?;
                let dynamic_field_inner = bcs::from_bytes::<Field<u64, SystemInnerV1>>(&result)
                    .map_err(|e| {
                        IkaError::SuiClientSerializationError(format!(
                            "Can't serialize SystemInner v1: {e}"
                        ))
                    })?;
                let ika_system_state_inner = dynamic_field_inner.value;

                Ok(SystemInner::V1(ika_system_state_inner))
            }
            _ => Err(IkaError::SuiClientInternalError(format!(
                "Unsupported SystemInner version: {}",
                wrapper.version
            ))),
        }
    }

    /// Retrieves Sui's System clock object.
    pub async fn get_clock(&self) -> IkaResult<Clock> {
        let sui_clock_address = "0x6";
        let result = self
            .inner
            .get_clock(ObjectID::from_hex_literal(sui_clock_address).unwrap())
            .await
            .map_err(|e| {
                IkaError::SuiClientInternalError(format!(
                    "Can't get the System clock from Sui: {e}"
                ))
            })?;
        bcs::from_bytes::<Clock>(&result).map_err(|e| {
            IkaError::SuiClientSerializationError(format!(
                "Can't deserialize Sui System clock: {e}"
            ))
        })
    }

    pub async fn get_class_groups_public_keys_and_proofs(
        &self,
        validators: &Vec<StakingPool>,
    ) -> IkaResult<HashMap<ObjectID, ClassGroupsEncryptionKeyAndProof>> {
        self.inner
            .get_class_groups_public_keys_and_proofs(validators)
            .await
            .map_err(|e| {
                IkaError::SuiClientInternalError(format!(
                    "Can't get_class_groups_public_keys_and_proofs: {e}"
                ))
            })
    }

    pub async fn get_epoch_start_system(
        &self,
        ika_system_state_inner: &SystemInner,
    ) -> IkaResult<EpochStartSystem> {
        match ika_system_state_inner {
            SystemInner::V1(ika_system_state_inner) => {
                let validator_ids = ika_system_state_inner
                    .validator_set
                    .active_committee
                    .members
                    .iter()
                    .map(|m| m.validator_id)
                    .collect::<Vec<_>>();

                let validators = self
                    .inner
                    .get_validators(validator_ids)
                    .await
                    .map_err(|e| {
                        IkaError::SuiClientInternalError(format!(
                            "Can't get_validators_from_object_table: {e}"
                        ))
                    })?;
                let validators = validators
                    .iter()
                    .map(|v| {
                        bcs::from_bytes::<StakingPool>(v).map_err(|e| {
                            IkaError::SuiClientSerializationError(format!(
                                "Can't serialize StakingPool: {e}"
                            ))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                let validators_class_groups_public_key_and_proof = self
                    .inner
                    .get_class_groups_public_keys_and_proofs(&validators)
                    .await
                    .map_err(|e| {
                        IkaError::SuiClientInternalError(format!(
                            "can't get_class_groups_public_keys_and_proofs: {e}"
                        ))
                    })?;

                let validators = ika_system_state_inner
                    .validator_set
                    .active_committee
                    .members
                    .iter()
                    .map(|m| {
                        let validator = validators.iter().find(|v| v.id == m.validator_id).ok_or(
                            IkaError::InvalidCommittee(format!(
                                "Validator with ID {} not found in the active committee",
                                m.validator_id
                            )),
                        )?;
                        let info = validator.verified_validator_info();
                        Ok(EpochStartValidatorInfoV1 {
                            validator_id: validator.id,
                            protocol_pubkey: info.protocol_pubkey.clone(),
                            network_pubkey: info.network_pubkey.clone(),
                            consensus_pubkey: info.consensus_pubkey.clone(),
                            class_groups_public_key_and_proof:
                                validators_class_groups_public_key_and_proof
                                    .get(&validator.id)
                                    .cloned(),
                            network_address: info.network_address.clone(),
                            p2p_address: info.p2p_address.clone(),
                            consensus_address: info.consensus_address.clone(),
                            voting_power: 1,
                            hostname: info.name.clone(),
                        })
                    })
                    .collect::<IkaResult<Vec<_>>>()?;

                let epoch_start_system_state = EpochStartSystem::new_v1(
                    ika_system_state_inner.epoch,
                    ika_system_state_inner.protocol_version,
                    ika_system_state_inner.epoch_start_timestamp_ms,
                    ika_system_state_inner.epoch_duration_ms(),
                    validators,
                    ika_system_state_inner
                        .validator_set
                        .active_committee
                        .quorum_threshold,
                    ika_system_state_inner
                        .validator_set
                        .active_committee
                        .validity_threshold,
                );

                Ok(epoch_start_system_state)
            }
        }
    }

    /// Get the validators' info by their IDs.
    pub async fn get_validators_info_by_ids(
        &self,
        validator_ids: Vec<ObjectID>,
    ) -> Result<Vec<StakingPool>, IkaError> {
        let validators = self
            .inner
            .get_validators(validator_ids)
            .await
            .map_err(|e| {
                IkaError::SuiClientInternalError(format!(
                    "failure in `get_validators_from_object_table()`: {e}"
                ))
            })?;
        validators
            .iter()
            .map(|v| {
                bcs::from_bytes::<StakingPool>(v).map_err(|e| {
                    IkaError::SuiClientSerializationError(format!(
                        "failed to de-serialize Validator info: {e}"
                    ))
                })
            })
            .collect::<Result<Vec<_>, _>>()
    }

    /// Get the mutable system object arg on chain.
    // We retry a few times in case of errors. If it fails eventually, we panic.
    // In general it's safe to call in the beginning of the program.
    // After the first call, the result is cached since the value should never change.
    pub async fn get_mutable_system_arg_must_succeed(&self) -> ObjectArg {
        static ARG: OnceCell<ObjectArg> = OnceCell::const_new();
        *ARG.get_or_init(|| async move {
            let Ok(Ok(system_arg)) = retry_with_max_elapsed_time!(
                self.inner.get_mutable_shared_arg(self.ika_system_object_id),
                Duration::from_secs(30)
            ) else {
                panic!("Failed to get system object arg after retries");
            };
            system_arg
        })
        .await
    }

    /// Get the clock object arg for the shared system object on the chain.
    pub async fn get_clock_arg_must_succeed(&self) -> ObjectArg {
        static ARG: OnceCell<ObjectArg> = OnceCell::const_new();
        *ARG.get_or_init(|| async move {
            let Ok(Ok(system_arg)) = retry_with_max_elapsed_time!(
                self.inner.get_shared_arg(ObjectID::from_single_byte(6)),
                Duration::from_secs(30)
            ) else {
                panic!("failed to get system object arg after retries");
            };
            system_arg
        })
        .await
    }

    /// Retrieves the dwallet_2pc_mpc_coordinator_id object arg from the Sui chain.
    pub async fn get_mutable_dwallet_2pc_mpc_coordinator_arg_must_succeed(&self) -> ObjectArg {
        static ARG: OnceCell<ObjectArg> = OnceCell::const_new();
        *ARG.get_or_init(|| async move {
            let Ok(Ok(system_arg)) = retry_with_max_elapsed_time!(
                self.inner
                    .get_mutable_shared_arg(self.ika_dwallet_coordinator_object_id),
                Duration::from_secs(30)
            ) else {
                panic!("Failed to get dwallet_2pc_mpc_coordinator_id object arg after retries");
            };
            system_arg
        })
        .await
    }

    pub async fn get_available_move_packages(
        &self,
    ) -> IkaResult<Vec<(ObjectID, MovePackageDigest)>> {
        self.inner
            .get_available_move_packages(self.ika_package_id, self.ika_system_package_id)
            .await
            .map_err(|e| {
                IkaError::SuiClientInternalError(format!("Can't get_available_move_packages: {e}"))
            })
    }

    /// Query emitted Events that are defined in the given Move Module.
    pub async fn query_events_by_module(
        &self,
        module: Identifier,
        // cursor is exclusive
        cursor: Option<EventID>,
    ) -> IkaResult<Page<SuiEvent, EventID>> {
        let filter = EventFilter::MoveEventModule {
            package: self.ika_dwallet_2pc_mpc_package_id,
            module: module.clone(),
        };
        let events = self
            .inner
            .query_events(filter.clone(), cursor)
            .await
            .map_err(|e| IkaError::SuiClientInternalError(format!("Can't query_events: {e}")))?;

        // Safeguard check that all events are emitted from requested package and module
        assert!(events.data.iter().all(|event| event.type_.address.as_ref()
            == self.ika_dwallet_2pc_mpc_package_id.as_ref()
            && event.type_.module == module));
        Ok(events)
    }

    pub async fn get_chain_identifier(&self) -> IkaResult<String> {
        self.inner.get_chain_identifier().await.map_err(|e| {
            IkaError::SuiClientInternalError(format!("Can't get_chain_identifier: {e}"))
        })
    }

    pub async fn get_reference_gas_price_until_success(&self) -> u64 {
        loop {
            let Ok(Ok(rgp)) = retry_with_max_elapsed_time!(
                self.inner.get_reference_gas_price(),
                Duration::from_secs(30)
            ) else {
                self.sui_client_metrics
                    .sui_rpc_errors
                    .with_label_values(&["get_reference_gas_price"])
                    .inc();
                error!("Failed to get gas price per unit size");
                continue;
            };
            return rgp;
        }
    }

    pub async fn get_latest_checkpoint_sequence_number(&self) -> IkaResult<u64> {
        self.inner
            .get_latest_checkpoint_sequence_number()
            .await
            .map_err(|e| {
                IkaError::SuiClientInternalError(format!(
                    "Can't get_latest_checkpoint_sequence_number: {e}"
                ))
            })
    }

    pub async fn execute_transaction_block_with_effects(
        &self,
        tx: Transaction,
    ) -> IkaResult<SuiTransactionBlockResponse> {
        self.inner.execute_transaction_block_with_effects(tx).await
    }

    pub async fn must_get_system_inner_object(&self) -> SystemInner {
        loop {
            match retry_with_max_elapsed_time!(self.get_system_inner(), Duration::from_secs(30)) {
                Ok(Ok(ika_system_state)) => return ika_system_state,
                Ok(Err(err)) => {
                    self.sui_client_metrics
                        .sui_rpc_errors
                        .with_label_values(&["must_get_system_inner_object"])
                        .inc();
                    warn!(
                        ?err,
                        "Received error from `get_system_inner()`. Retrying...",
                    );
                }
                Err(err) => {
                    self.sui_client_metrics
                        .sui_rpc_errors
                        .with_label_values(&["must_get_system_inner_object"])
                        .inc();
                    warn!(
                        ?err,
                        system_object_id=%self.ika_system_object_id,
                        "failed to get system inner object",
                    );
                }
            }
        }
    }

    pub async fn must_get_dwallet_coordinator_inner_v1(&self) -> DWalletCoordinatorInnerV1 {
        let DWalletCoordinatorInner::V1(inner_v1) = self.must_get_dwallet_coordinator_inner().await;
        inner_v1
    }

    pub async fn get_dwallet_mpc_network_keys(
        &self,
    ) -> IkaResult<HashMap<ObjectID, DWalletNetworkEncryptionKey>> {
        let dwallet_coordinator_inner = self.must_get_dwallet_coordinator_inner_v1().await;
        self.inner
            .get_network_encryption_keys(&dwallet_coordinator_inner)
            .await
            .map_err(|e| {
                IkaError::SuiClientInternalError(format!("can't get_network_encryption_keys: {e}"))
            })
    }

    pub async fn get_network_encryption_key_with_full_data_by_epoch(
        &self,
        network_decryption_key: &DWalletNetworkEncryptionKey,
        epoch: EpochId,
    ) -> IkaResult<DWalletNetworkEncryptionKeyData> {
        self.inner
            .get_network_encryption_key_with_full_data_by_epoch(network_decryption_key, epoch)
            .await
            .map_err(|e| {
                IkaError::SuiClientInternalError(format!(
                    "Can't get_network_encryption_key_with_full_data_by_epoch: {e}"
                ))
            })
    }

    pub async fn must_get_dwallet_coordinator_inner(&self) -> DWalletCoordinatorInner {
        loop {
            match retry_with_max_elapsed_time!(
                self.get_dwallet_coordinator_inner(),
                Duration::from_secs(30)
            ) {
                Ok(Ok(ika_system_state)) => return ika_system_state,
                Ok(Err(err)) => {
                    self.sui_client_metrics
                        .sui_rpc_errors
                        .with_label_values(&["must_get_dwallet_coordinator_inner"])
                        .inc();
                    warn!(
                        ?err,
                        "Received error from `get_dwallet_coordinator_inner()`. Retrying...",
                    );
                }
                Err(err) => {
                    self.sui_client_metrics
                        .sui_rpc_errors
                        .with_label_values(&["must_get_dwallet_coordinator_inner"])
                        .inc();
                    warn!(
                        ?err,
                        system_object_id=%self.ika_system_object_id,
                        "Failed to get dwallet coordinator inner object",
                    );
                }
            }
        }
    }

    pub async fn must_get_epoch_start_system(
        &self,
        system_inner: &SystemInner,
    ) -> EpochStartSystem {
        loop {
            match retry_with_max_elapsed_time!(
                self.get_epoch_start_system(system_inner),
                Duration::from_secs(30)
            ) {
                Ok(Ok(ika_system_state)) => return ika_system_state,
                Ok(Err(err)) => {
                    self.sui_client_metrics
                        .sui_rpc_errors
                        .with_label_values(&["must_get_epoch_start_system"])
                        .inc();
                    warn!(
                        ?err,
                        "Received error from `get_epoch_start_system()`. Retrying...",
                    );
                }
                Err(err) => {
                    self.sui_client_metrics
                        .sui_rpc_errors
                        .with_label_values(&["must_get_epoch_start_system"])
                        .inc();
                    warn!(
                        ?err,
                        "Received error from `get_epoch_start_system` retry wrapper. Retrying...",
                    );
                }
            }
        }
    }

    pub async fn get_gas_objects(&self, address: SuiAddress) -> Vec<ObjectRef> {
        self.inner.get_gas_objects(address).await
    }
}

/// Use a trait to abstract over the SuiSDKClient and SuiMockClient for testing.
#[async_trait]
pub trait SuiClientInner: Send + Sync {
    type Error: Into<anyhow::Error> + Send + Sync + std::error::Error + 'static;
    async fn query_events(
        &self,
        query: EventFilter,
        cursor: Option<EventID>,
    ) -> Result<EventPage, Self::Error>;

    async fn get_events_by_tx_digest(
        &self,
        tx_digest: TransactionDigest,
    ) -> Result<Vec<SuiEvent>, Self::Error>;

    async fn get_chain_identifier(&self) -> Result<String, Self::Error>;

    async fn get_reference_gas_price(&self) -> Result<u64, Self::Error>;

    async fn get_latest_checkpoint_sequence_number(&self) -> Result<u64, Self::Error>;

    async fn get_system(&self, ika_system_object_id: ObjectID) -> Result<Vec<u8>, Self::Error>;

    async fn get_clock(&self, clock_obj_id: ObjectID) -> Result<Vec<u8>, Self::Error>;

    async fn get_dwallet_coordinator(
        &self,
        dwallet_coordinator_id: ObjectID,
    ) -> Result<Vec<u8>, Self::Error>;

    #[allow(clippy::ptr_arg)]
    async fn get_class_groups_public_keys_and_proofs(
        &self,
        validators: &Vec<StakingPool>,
    ) -> Result<HashMap<ObjectID, ClassGroupsEncryptionKeyAndProof>, self::Error>;

    #[allow(clippy::ptr_arg)]
    async fn get_network_encryption_keys(
        &self,
        dwallet_coordinator_inner: &DWalletCoordinatorInnerV1,
    ) -> Result<HashMap<ObjectID, DWalletNetworkEncryptionKey>, self::Error>;

    async fn get_network_encryption_key_with_full_data_by_epoch(
        &self,
        network_decryption_key: &DWalletNetworkEncryptionKey,
        epoch: EpochId,
    ) -> Result<DWalletNetworkEncryptionKeyData, self::Error>;

    async fn get_current_reconfiguration_public_output(
        &self,
        epoch_id: EpochId,
        table_id: ObjectID,
    ) -> Result<ObjectID, Self::Error>;

    async fn read_table_vec_as_raw_bytes(&self, table_id: ObjectID)
        -> Result<Vec<u8>, self::Error>;

    async fn get_system_inner(
        &self,
        ika_system_object_id: ObjectID,
        version: u64,
    ) -> Result<Vec<u8>, Self::Error>;

    async fn get_dwallet_coordinator_inner(
        &self,
        dwallet_coordinator_id: ObjectID,
        version: u64,
    ) -> Result<Vec<u8>, Self::Error>;

    async fn get_validators(
        &self,
        validator_ids: Vec<ObjectID>,
    ) -> Result<Vec<Vec<u8>>, Self::Error>;

    async fn get_validator_inners(
        &self,
        validators: Vec<Validator>,
    ) -> Result<Vec<Vec<u8>>, Self::Error>;

    async fn get_mutable_shared_arg(
        &self,
        ika_system_object_id: ObjectID,
    ) -> Result<ObjectArg, Self::Error>;

    async fn get_shared_arg(&self, obj_id: ObjectID) -> Result<ObjectArg, Self::Error>;

    async fn get_available_move_packages(
        &self,
        //chain: sui_protocol_config::Chain,
        ika_package_id: ObjectID,
        ika_system_package_id: ObjectID,
    ) -> Result<Vec<(ObjectID, MovePackageDigest)>, Self::Error>;

    async fn execute_transaction_block_with_effects(
        &self,
        tx: Transaction,
    ) -> Result<SuiTransactionBlockResponse, IkaError>;

    async fn get_gas_objects(&self, address: SuiAddress) -> Vec<ObjectRef>;

    /// Fetch events for which no output was received (weren't completed.)
    /// Completed events are removed from the SessionManagement in Move,
    /// so querying all the values assures we query uncompleted events exclusively.
    async fn get_uncompleted_events(
        &self,
        events_bag_id: ObjectID,
    ) -> Result<Vec<DBSuiEvent>, self::Error>;
}

#[async_trait]
impl SuiClientInner for SuiSdkClient {
    type Error = sui_sdk::error::Error;

    async fn query_events(
        &self,
        query: EventFilter,
        cursor: Option<EventID>,
    ) -> Result<EventPage, Self::Error> {
        self.event_api()
            .query_events(query, cursor, None, false)
            .await
    }

    async fn get_events_by_tx_digest(
        &self,
        tx_digest: TransactionDigest,
    ) -> Result<Vec<SuiEvent>, Self::Error> {
        self.event_api().get_events(tx_digest).await
    }

    async fn get_chain_identifier(&self) -> Result<String, Self::Error> {
        self.read_api().get_chain_identifier().await
    }

    async fn get_reference_gas_price(&self) -> Result<u64, Self::Error> {
        self.governance_api().get_reference_gas_price().await
    }

    async fn get_latest_checkpoint_sequence_number(&self) -> Result<u64, Self::Error> {
        self.read_api()
            .get_latest_checkpoint_sequence_number()
            .await
    }

    async fn get_system(&self, ika_system_object_id: ObjectID) -> Result<Vec<u8>, Self::Error> {
        self.read_api()
            .get_move_object_bcs(ika_system_object_id)
            .await
    }

    async fn get_clock(&self, clock_obj_id: ObjectID) -> Result<Vec<u8>, Self::Error> {
        self.read_api().get_move_object_bcs(clock_obj_id).await
    }

    async fn get_dwallet_coordinator(
        &self,
        dwallet_coordinator_id: ObjectID,
    ) -> Result<Vec<u8>, Self::Error> {
        self.read_api()
            .get_move_object_bcs(dwallet_coordinator_id)
            .await
    }

    /// Fetch events for which no output was received (weren't completed.)
    async fn get_uncompleted_events(
        &self,
        coordinator_events_bag_id: ObjectID,
    ) -> Result<Vec<DBSuiEvent>, self::Error> {
        let mut events = vec![];
        let mut next_cursor = None;
        loop {
            let dynamic_fields = self
                .read_api()
                .get_dynamic_fields(coordinator_events_bag_id, next_cursor, None)
                .await?;
            for df in dynamic_fields.data.iter() {
                let object_id = df.object_id;
                let dynamic_field_response = self
                    .read_api()
                    .get_object_with_options(object_id, SuiObjectDataOptions::bcs_lossless())
                    .await?;
                let resp = dynamic_field_response.into_object().map_err(|e| {
                    Error::DataError(format!("can't get bcs of object {:?}: {:?}", object_id, e))
                })?;
                let move_object = resp.bcs.ok_or(Error::DataError(format!(
                    "object {:?} has no bcs data",
                    object_id
                )))?;
                let raw_move_obj = move_object.try_into_move().ok_or(Error::DataError(format!(
                    "object {:?} is not a MoveObject",
                    object_id
                )))?;

                let Some(TypeTag::Struct(event_tag)) = raw_move_obj.type_.type_params.get(1) else {
                    continue;
                };
                let event = DBSuiEvent {
                    type_: *event_tag.clone(),
                    contents: raw_move_obj.bcs_bytes,
                    pulled: true,
                };
                events.push(event);
            }
            if !dynamic_fields.has_next_page {
                break;
            }
            next_cursor = dynamic_fields.next_cursor;
        }

        Ok(events)
    }

    async fn get_class_groups_public_keys_and_proofs(
        &self,
        validators: &Vec<StakingPool>,
    ) -> Result<HashMap<ObjectID, ClassGroupsEncryptionKeyAndProof>, self::Error> {
        let mut class_groups_public_keys_and_proofs: HashMap<
            ObjectID,
            ClassGroupsEncryptionKeyAndProof,
        > = HashMap::new();
        for validator in validators {
            let info = validator.verified_validator_info();
            let dynamic_fields = self
                .read_api()
                .get_dynamic_fields(
                    info.class_groups_pubkey_and_proof_bytes.contents.id,
                    None,
                    None,
                )
                .await?;
            let mut validator_class_groups_public_key_and_proof_bytes: [Vec<u8>;
                NUM_OF_CLASS_GROUPS_KEY_OBJECTS] = Default::default();
            if dynamic_fields.data.len() != NUM_OF_CLASS_GROUPS_KEY_OBJECTS {
                warn!(
                    validator_id=?validator.id,
                    expected_num_of_class_groups_keys=NUM_OF_CLASS_GROUPS_KEY_OBJECTS,
                    dynamic_fields_count=dynamic_fields.data.len(),
                    "Validator class groups public key and proof length mismatch",
                );
                continue;
            }
            for df in dynamic_fields.data.iter() {
                let object_id = df.object_id;
                let dynamic_field_response = self
                    .read_api()
                    .get_object_with_options(object_id, SuiObjectDataOptions::bcs_lossless())
                    .await?;
                let resp = dynamic_field_response.into_object().map_err(|e| {
                    Error::DataError(format!("can't get bcs of object {:?}: {:?}", object_id, e))
                })?;
                let move_object = resp.bcs.ok_or(Error::DataError(format!(
                    "object {:?} has no bcs data",
                    object_id
                )))?;
                let raw_move_obj = move_object.try_into_move().ok_or(Error::DataError(format!(
                    "object {:?} is not a MoveObject",
                    object_id
                )))?;
                let key_slice = bcs::from_bytes::<Field<u64, Vec<u8>>>(&raw_move_obj.bcs_bytes)?;
                validator_class_groups_public_key_and_proof_bytes[key_slice.name as usize] =
                    key_slice.value.clone();
            }
            let validator_class_groups_public_key_and_proof: Result<
                Vec<SingleEncryptionKeyAndProof>,
                _,
            > = validator_class_groups_public_key_and_proof_bytes
                .into_iter()
                .map(|v| bcs::from_bytes::<SingleEncryptionKeyAndProof>(&v))
                .collect();

            match validator_class_groups_public_key_and_proof {
                Ok(validator_class_groups_public_key_and_proof) => {
                    class_groups_public_keys_and_proofs.insert(
                        validator.id,
                        validator_class_groups_public_key_and_proof
                            .try_into()
                            .map_err(|e| {
                                Error::DataError(format!(
                                    "class groups key from Sui is invalid: {:?}",
                                    e
                                ))
                            })?,
                    );
                }
                Err(e) => {
                    warn!(
                        validator_id=?validator.id,
                        error=?e,
                        "Failed to deserialize class groups public key and proof for a validator"
                    );
                    continue;
                }
            }
        }
        Ok(class_groups_public_keys_and_proofs)
    }

    async fn get_network_encryption_keys(
        &self,
        dwallet_coordinator_inner: &DWalletCoordinatorInnerV1,
    ) -> Result<HashMap<ObjectID, DWalletNetworkEncryptionKey>, self::Error> {
        let mut network_encryption_keys = HashMap::new();

        let mut cursor = None;
        loop {
            let dynamic_fields = self
                .read_api()
                .get_dynamic_fields(dwallet_coordinator_inner.dwallet_network_encryption_keys.id, cursor, None)
                .await
                .map_err(|e| {
                    Error::DataError(format!(
                        "can't get dynamic fields of `dwallet_coordinator_inner.dwallet_network_encryption_keys` table {:?}: {:?}",
                        dwallet_coordinator_inner.dwallet_network_encryption_keys.id, e
                    ))
                })?;
            let object_ids: Vec<ObjectID> =
                dynamic_fields.data.iter().map(|df| df.object_id).collect();
            let objects = self
                .read_api()
                .multi_get_object_with_options(object_ids, SuiObjectDataOptions::bcs_lossless())
                .await?;

            for resp in objects {
                if let Some(data) = resp.data {
                    let object_id = data.object_id;
                    let raw_data = data.bcs.ok_or(Error::DataError(format!(
                        "object {:?} has no bcs data",
                        object_id
                    )))?;
                    let raw_move_obj = raw_data.try_into_move().ok_or(Error::DataError(
                        format!("object {:?} is not a MoveObject", object_id),
                    ))?;
                    let value =
                        bcs::from_bytes::<DWalletNetworkEncryptionKey>(&raw_move_obj.bcs_bytes)?;
                    network_encryption_keys.insert(object_id, value);
                }
            }

            cursor = dynamic_fields.next_cursor;
            if !dynamic_fields.has_next_page {
                break;
            }
        }
        Ok(network_encryption_keys)
    }

    async fn get_network_encryption_key_with_full_data_by_epoch(
        &self,
        key: &DWalletNetworkEncryptionKey,
        epoch: EpochId,
    ) -> Result<DWalletNetworkEncryptionKeyData, self::Error> {
        let network_dkg_public_output = self
            .read_table_vec_as_raw_bytes(key.network_dkg_public_output.contents.id)
            .await?;

        // Note that if we try to read the reconfiguration public output during the first epoch,
        // where we only had NetworkDKG, `get_current_reconfiguration_public_output()` function will error.
        // In this case, the validator will be stuck in a loop where it can't process events
        // until the epoch is switched, since it will be endlessly waiting for the network key.
        let awaiting_first_reconfiguration_to_complete = key.state
            == (DWalletNetworkEncryptionKeyState::AwaitingNetworkReconfiguration {
                is_first: true,
            });
        let no_reconfiguration_key_data = key.reconfiguration_public_outputs.size == 0;
        let mut current_reconfiguration_public_output = vec![];

        if no_reconfiguration_key_data
            || key.state == DWalletNetworkEncryptionKeyState::AwaitingNetworkDKG
            || key.state == DWalletNetworkEncryptionKeyState::NetworkDKGCompleted
            || awaiting_first_reconfiguration_to_complete
        {
            info!(
                key_id = ?key.id,
                ?epoch,
                "Reconfiguration public output for key not is not ready for epoch",
            );
        } else {
            let current_reconfiguration_public_output_id = self
                .get_current_reconfiguration_public_output(
                    epoch,
                    key.reconfiguration_public_outputs.id,
                )
                .await?;
            current_reconfiguration_public_output = self
                .read_table_vec_as_raw_bytes(current_reconfiguration_public_output_id)
                .await?;
        };

        Ok(DWalletNetworkEncryptionKeyData {
            id: key.id,
            current_epoch: epoch,
            current_reconfiguration_public_output,
            network_dkg_public_output,
            state: key.state.clone(),
        })
    }

    async fn get_current_reconfiguration_public_output(
        &self,
        epoch_id: EpochId,
        table_id: ObjectID,
    ) -> Result<ObjectID, Self::Error> {
        let mut cursor = None;
        loop {
            let dynamic_fields = self
                .read_api()
                .get_dynamic_fields(table_id, cursor, None)
                .await
                .map_err(|e| {
                    Error::DataError(format!(
                        "can't get dynamic fields of table {:?}: {:?}",
                        table_id, e
                    ))
                })?;

            for df in dynamic_fields.data.iter() {
                let object_id = df.object_id;
                let dynamic_field_response = self
                    .read_api()
                    .get_object_with_options(object_id, SuiObjectDataOptions::bcs_lossless())
                    .await?;
                let resp = dynamic_field_response.into_object().map_err(|e| {
                    Error::DataError(format!("can't get bcs of object {:?}: {:?}", object_id, e))
                })?;
                let raw_data = resp.bcs.ok_or(Error::DataError(format!(
                    "object {:?} has no bcs data",
                    object_id
                )))?;
                let raw_move_obj = raw_data.try_into_move().ok_or(Error::DataError(format!(
                    "object {:?} is not a MoveObject",
                    object_id
                )))?;
                let reconfig_public_output =
                    bcs::from_bytes::<Field<u64, Table>>(&raw_move_obj.bcs_bytes)?;
                if reconfig_public_output.name == epoch_id {
                    return Ok(reconfig_public_output.value.id);
                }
            }

            cursor = dynamic_fields.next_cursor;
            if !dynamic_fields.has_next_page {
                break;
            }
        }
        Err(Error::DataError(format!(
            "Failed to load current reconfiguration public output for epoch {:?} from table {:?}",
            epoch_id, table_id
        )))
    }

    async fn read_table_vec_as_raw_bytes(
        &self,
        table_id: ObjectID,
    ) -> Result<Vec<u8>, Self::Error> {
        let mut full_output: HashMap<usize, Vec<u8>> = HashMap::new();
        let mut cursor = None;
        loop {
            let dynamic_fields = self
                .read_api()
                .get_dynamic_fields(table_id, cursor, None)
                .await
                .map_err(|e| {
                    Error::DataError(format!(
                        "can't get dynamic fields of table {:?}: {:?}",
                        table_id, e
                    ))
                })?;

            for df in dynamic_fields.data.iter() {
                let object_id = df.object_id;
                let dynamic_field_response = self
                    .read_api()
                    .get_object_with_options(object_id, SuiObjectDataOptions::bcs_lossless())
                    .await?;
                let resp = dynamic_field_response.into_object().map_err(|e| {
                    Error::DataError(format!("can't get bcs of object {:?}: {:?}", object_id, e))
                })?;
                let raw_data = resp.bcs.ok_or(Error::DataError(format!(
                    "object {:?} has no bcs data",
                    object_id
                )))?;
                let raw_move_obj = raw_data.try_into_move().ok_or(Error::DataError(format!(
                    "object {:?} is not a MoveObject",
                    object_id
                )))?;
                let bytes_chunk = bcs::from_bytes::<Field<u64, Vec<u8>>>(&raw_move_obj.bcs_bytes)?;
                full_output.insert(bytes_chunk.name as usize, bytes_chunk.value.clone());
            }

            cursor = dynamic_fields.next_cursor;
            if !dynamic_fields.has_next_page {
                break;
            }
        }

        Ok(full_output
            .into_iter()
            .sorted()
            .fold(Vec::new(), |mut acc, (_, mut v)| {
                acc.append(&mut v);
                acc
            }))
    }

    async fn get_system_inner(
        &self,
        ika_system_object_id: ObjectID,
        version: u64,
    ) -> Result<Vec<u8>, Self::Error> {
        let dynamic_fields = self
            .read_api()
            .get_dynamic_fields(ika_system_object_id, None, None)
            .await?;
        let dynamic_field = dynamic_fields.data.iter().find(|df| {
            df.name.type_ == TypeTag::U64
                && df
                    .name
                    .value
                    .as_str()
                    .map(|v| v == version.to_string().as_str())
                    .unwrap_or(false)
        });
        if let Some(dynamic_field) = dynamic_field {
            let result = self
                .read_api()
                .get_dynamic_field_object(ika_system_object_id, dynamic_field.name.clone())
                .await?;

            if let Some(dynamic_field) = result.data {
                let object_id = dynamic_field.object_id;
                let dynamic_field_response = self
                    .read_api()
                    .get_object_with_options(object_id, SuiObjectDataOptions::bcs_lossless())
                    .await?;
                let resp = dynamic_field_response.into_object().map_err(|e| {
                    Error::DataError(format!("Can't get bcs of object {:?}: {:?}", object_id, e))
                })?;
                // unwrap: requested bcs data
                let move_object = resp.bcs.unwrap();
                let raw_move_obj = move_object.try_into_move().ok_or(Error::DataError(format!(
                    "Object {:?} is not a MoveObject",
                    object_id
                )))?;
                return Ok(raw_move_obj.bcs_bytes);
            }
        }
        Err(Error::DataError(format!(
            "Failed to load ika system state inner object with ID {:?} and version {:?}",
            ika_system_object_id, version
        )))
    }

    async fn get_dwallet_coordinator_inner(
        &self,
        dwallet_coordinator_id: ObjectID,
        version: u64,
    ) -> Result<Vec<u8>, Self::Error> {
        let dynamic_fields = self
            .read_api()
            .get_dynamic_fields(dwallet_coordinator_id, None, None)
            .await?;
        let dynamic_field = dynamic_fields.data.iter().find(|df| {
            df.name.type_ == TypeTag::U64
                && df
                    .name
                    .value
                    .as_str()
                    .map(|v| v == version.to_string().as_str())
                    .unwrap_or(false)
        });
        if let Some(dynamic_field) = dynamic_field {
            let result = self
                .read_api()
                .get_dynamic_field_object(dwallet_coordinator_id, dynamic_field.name.clone())
                .await?;

            if let Some(dynamic_field) = result.data {
                let object_id = dynamic_field.object_id;
                let dynamic_field_response = self
                    .read_api()
                    .get_object_with_options(object_id, SuiObjectDataOptions::bcs_lossless())
                    .await?;
                let resp = dynamic_field_response.into_object().map_err(|e| {
                    Error::DataError(format!("Can't get bcs of object {:?}: {:?}", object_id, e))
                })?;
                // unwrap: requested bcs data
                let move_object = resp.bcs.unwrap();
                let raw_move_obj = move_object.try_into_move().ok_or(Error::DataError(format!(
                    "Object {:?} is not a MoveObject",
                    object_id
                )))?;
                return Ok(raw_move_obj.bcs_bytes);
            }
        }
        Err(Error::DataError(format!(
            "Failed to load DWalletCoordinatorInner object with ID {:?} and version {:?}",
            dwallet_coordinator_id, version
        )))
    }

    async fn get_validators(
        &self,
        validator_ids: Vec<ObjectID>,
    ) -> Result<Vec<Vec<u8>>, Self::Error> {
        let mut dynamic_fields_agg = Vec::new();
        // There is a limit in sui called "DEFAULT_RPC_QUERY_MAX_RESULT_LIMIT" which is set to 50.
        for chunk in validator_ids.chunks(50) {
            let objects = self
                .read_api()
                .multi_get_object_with_options(chunk.to_vec(), SuiObjectDataOptions::bcs_lossless())
                .await?;

            dynamic_fields_agg.extend(objects);
        }

        let mut validators = Vec::new();
        for (dynamic_field, object_id) in dynamic_fields_agg.iter().zip(validator_ids.iter()) {
            let resp = dynamic_field.object().map_err(|e| {
                Error::DataError(format!("Can't get bcs of object {:?}: {:?}", object_id, e))
            })?;
            // unwrap: requested bcs data
            let move_object = resp.bcs.as_ref().unwrap();
            let raw_move_obj =
                move_object
                    .clone()
                    .try_into_move()
                    .ok_or(Error::DataError(format!(
                        "Object {:?} is not a MoveObject",
                        object_id
                    )))?;
            validators.push(raw_move_obj.bcs_bytes);
        }
        Ok(validators)
    }

    async fn get_validator_inners(
        &self,
        validators: Vec<Validator>,
    ) -> Result<Vec<Vec<u8>>, Self::Error> {
        let mut validator_inners = Vec::new();

        for validator in validators {
            let dynamic_fields = self
                .read_api()
                .get_dynamic_fields(validator.inner.id.id.bytes, None, None)
                .await?;

            let dynamic_field = dynamic_fields.data.iter().find(|df| {
                df.name.type_ == TypeTag::U64
                    && df
                        .name
                        .value
                        .as_str()
                        .map(|v| v == validator.inner.version.to_string().as_str())
                        .unwrap_or(false)
            });

            if let Some(dynamic_field) = dynamic_field {
                let object_id = dynamic_field.object_id;
                let dynamic_field_response = self
                    .read_api()
                    .get_object_with_options(object_id, SuiObjectDataOptions::bcs_lossless())
                    .await?;
                let resp = dynamic_field_response.into_object().map_err(|e| {
                    Error::DataError(format!("Can't get bcs of object {:?}: {:?}", object_id, e))
                })?;
                // unwrap: requested bcs data
                let move_object = resp.bcs.unwrap();
                let raw_move_obj = move_object.try_into_move().ok_or(Error::DataError(format!(
                    "Object {:?} is not a MoveObject",
                    object_id
                )))?;
                validator_inners.push(raw_move_obj.bcs_bytes);
            }
        }
        Ok(validator_inners)
    }

    async fn get_mutable_shared_arg(
        &self,
        ika_system_object_id: ObjectID,
    ) -> Result<ObjectArg, Self::Error> {
        let response = self
            .read_api()
            .get_object_with_options(
                ika_system_object_id,
                SuiObjectDataOptions::new().with_owner(),
            )
            .await?;
        let Some(Owner::Shared {
            initial_shared_version,
        }) = response.owner()
        else {
            return Err(Self::Error::DataError(format!(
                "Failed to load ika system state owner {:?}",
                ika_system_object_id
            )));
        };
        Ok(ObjectArg::SharedObject {
            id: ika_system_object_id,
            initial_shared_version,
            mutable: true,
        })
    }

    /// Get the shared object arg for the shared system object on the chain.
    async fn get_shared_arg(&self, obj_id: ObjectID) -> Result<ObjectArg, Self::Error> {
        let response = self
            .read_api()
            .get_object_with_options(obj_id, SuiObjectDataOptions::new().with_owner())
            .await?;
        let Some(Owner::Shared {
            initial_shared_version,
        }) = response.owner()
        else {
            return Err(Self::Error::DataError(format!(
                "Failed to load ika system state owner {:?}",
                obj_id
            )));
        };
        Ok(ObjectArg::SharedObject {
            id: obj_id,
            initial_shared_version,
            mutable: false,
        })
    }

    async fn get_available_move_packages(
        &self,
        //chain: sui_protocol_config::Chain,
        ika_package_id: ObjectID,
        ika_system_package_id: ObjectID,
    ) -> Result<Vec<(ObjectID, MovePackageDigest)>, Self::Error> {
        let mut results = vec![];
        //let protocol_config_response = self.read_api().get_protocol_config(None).await?;
        //let protocol_config = sui_protocol_config::ProtocolConfig::get_for_version(protocol_config_response.protocol_version, chain);
        //let binary_config = sui_types::execution_config_utils::to_binary_config(&protocol_config);

        let ika_packages = vec![
            ("ika".to_string(), ika_package_id),
            ("ika_system".to_string(), ika_system_package_id),
        ];
        for (name, package_id) in ika_packages.clone() {
            //let object_response = self.read_api().get_object_with_options(package_id, SuiObjectDataOptions::full_content()).await?;
            //let object_data = object_response.data.expect("Package object should have data.");
            //let object: Object = object_data.try_into().map_err(|e: anyhow::Error| Self::Error::DataError(e.to_string()))?;
            let move_package = BuiltInIkaMovePackages::get_package_by_name(&name);
            //let modules = move_package.modules_with_deps(ika_packages.clone().into_iter().collect()).map_err(|e: anyhow::Error| Self::Error::DataError(e.to_string()))?;
            let bytes = move_package
                .bytes_with_deps(ika_packages.clone().into_iter().collect())
                .map_err(|e: anyhow::Error| Self::Error::DataError(e.to_string()))?;
            let full_deps = move_package
                .full_deps(ika_packages.clone().into_iter().collect())
                .map_err(|e: anyhow::Error| Self::Error::DataError(e.to_string()))?;
            let digest = MovePackage::compute_digest_for_modules_and_deps(
                bytes.iter(),
                full_deps.iter(),
                true,
            );
            results.push((package_id, digest))
        }

        Ok(results)
    }

    async fn execute_transaction_block_with_effects(
        &self,
        tx: Transaction,
    ) -> Result<SuiTransactionBlockResponse, IkaError> {
        match self.quorum_driver_api().execute_transaction_block(
            tx,
            SuiTransactionBlockResponseOptions::new().with_effects().with_events(),
            Some(sui_types::quorum_driver_types::ExecuteTransactionRequestType::WaitForEffectsCert),
        ).await {
            Ok(response) => Ok(response),
            Err(e) => Err(IkaError::SuiClientTxFailureGeneric(e.to_string())),
        }
    }

    async fn get_gas_objects(&self, address: SuiAddress) -> Vec<ObjectRef> {
        loop {
            let results = self
                .read_api()
                .get_owned_objects(
                    address,
                    Some(SuiObjectResponseQuery::new(
                        Some(SuiObjectDataFilter::StructType(GasCoin::type_())),
                        Some(SuiObjectDataOptions::full_content()),
                    )),
                    None,
                    None,
                )
                .await
                .map(|o| {
                    o.data
                        .into_iter()
                        .filter_map(|r| r.data.map(|o| o.object_ref()))
                        .collect::<Vec<_>>()
                });

            match results {
                Ok(gas_objs) => return gas_objs,
                Err(err) => {
                    warn!("can't get gas objects for address {}: {}", address, err);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }
}
