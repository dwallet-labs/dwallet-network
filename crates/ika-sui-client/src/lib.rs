// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::metrics::SuiClientMetrics;
use anyhow::anyhow;
use async_trait::async_trait;
use core::panic;
use dwallet_classgroups_types::{
    ClassGroupsEncryptionKeyAndProof, SingleEncryptionKeyAndProof, NUM_OF_CLASS_GROUPS_KEYS,
};
use dwallet_mpc_types::dwallet_mpc::{
    NetworkDecryptionKeyOnChainOutput, NetworkDecryptionKeyShares,
};
use fastcrypto::traits::ToFromBytes;
use ika_move_packages::BuiltInIkaMovePackages;
use ika_types::error::{IkaError, IkaResult};
use ika_types::messages_consensus::MovePackageDigest;
use ika_types::messages_dwallet_mpc::{
    DBSuiEvent, DWalletNetworkDecryptionKey, DWalletNetworkDecryptionKeyState,
};
use ika_types::sui::epoch_start_system::{EpochStartSystem, EpochStartValidatorInfoV1};
use ika_types::sui::system_inner_v1::{
    DWalletCoordinatorInnerV1, DWalletNetworkDecryptionKeyCap, SystemInnerV1,
};
use ika_types::sui::validator_inner_v1::ValidatorInnerV1;
use ika_types::sui::{
    DWalletCoordinator, DWalletCoordinatorInner, System, SystemInner, SystemInnerTrait, Validator,
};
use itertools::Itertools;
use move_binary_format::binary_config::BinaryConfig;
use move_core_types::account_address::AccountAddress;
use move_core_types::annotated_value::MoveEnumLayout;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::from_utf8;
use std::sync::Arc;
use std::time::Duration;
use sui_json_rpc_api::BridgeReadApiClient;
use sui_json_rpc_types::{DevInspectResults, SuiData, SuiMoveValue};
use sui_json_rpc_types::{EventFilter, Page, SuiEvent};
use sui_json_rpc_types::{
    EventPage, SuiObjectDataOptions, SuiTransactionBlockResponse,
    SuiTransactionBlockResponseOptions,
};
use sui_sdk::error::Error;
use sui_sdk::{SuiClient as SuiSdkClient, SuiClientBuilder};
use sui_types::balance::Balance;
use sui_types::base_types::SequenceNumber;
use sui_types::base_types::{EpochId, ObjectRef};
use sui_types::clock::Clock;
use sui_types::collection_types::TableVec;
use sui_types::dynamic_field::Field;
use sui_types::gas_coin::GasCoin;
use sui_types::id::{ID, UID};
use sui_types::move_package::MovePackage;
use sui_types::object::{MoveObject, Object, Owner};
use sui_types::parse_sui_type_tag;
use sui_types::transaction::Argument;
use sui_types::transaction::CallArg;
use sui_types::transaction::Command;
use sui_types::transaction::ObjectArg;
use sui_types::transaction::ProgrammableTransaction;
use sui_types::transaction::Transaction;
use sui_types::transaction::TransactionKind;
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
                    Err(e) => {
                        // For simplicity we treat every error as transient so we can retry until max_elapsed_time
                        debug!("Retrying due to error: {:?}", e);
                        return Err(backoff::Error::transient(e));
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
    ika_system_package_id: ObjectID,
    system_id: ObjectID,
}

pub type SuiBridgeClient = SuiClient<SuiSdkClient>;

impl SuiBridgeClient {
    pub async fn new(
        rpc_url: &str,
        sui_client_metrics: Arc<SuiClientMetrics>,
        ika_package_id: ObjectID,
        ika_system_package_id: ObjectID,
        system_id: ObjectID,
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
            ika_system_package_id,
            system_id,
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
    pub async fn get_dwallet_mpc_missed_events(
        &self,
        epoch_id: EpochId,
    ) -> IkaResult<Vec<DBSuiEvent>> {
        let system_inner = self.get_system_inner_until_success().await;
        loop {
            if let Some(dwallet_state_id) = system_inner.dwallet_2pc_mpc_secp256k1_id() {
                let dwallet_coordinator_inner = self
                    .get_dwallet_coordinator_inner_until_success(dwallet_state_id)
                    .await;
                match dwallet_coordinator_inner {
                    DWalletCoordinatorInner::V1(dwallet_coordinator_inner_v1) => {
                        // Make sure we are synced with Sui in order to fetch the missed events
                        // If Sui's epoch number matches ours, all the needed missed events must be synced as well.
                        if dwallet_coordinator_inner_v1.current_epoch != epoch_id {
                            tokio::time::sleep(Duration::from_secs(2)).await;
                            continue;
                        }
                        let missed_events = self
                            .inner
                            .get_missed_events(
                                dwallet_coordinator_inner_v1
                                    .session_start_events
                                    .id
                                    .id
                                    .bytes,
                            )
                            .await
                            .map_err(|e| {
                                error!("failed to get missed events: {e}");
                                IkaError::SuiClientInternalError(format!(
                                    "failed to get missed events: {e}"
                                ))
                            })?;
                        info!("retrieved missed events from Sui successfully");
                        return Ok(missed_events);
                    }
                };
            }
        }
    }

    pub fn new_for_testing(inner: P) -> Self {
        Self {
            inner,
            sui_client_metrics: SuiClientMetrics::new_for_testing(),
            // TODO(omersadika) fix that random
            ika_package_id: ObjectID::random(),
            ika_system_package_id: ObjectID::random(),
            system_id: ObjectID::random(),
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

    pub async fn get_dwallet_coordinator_inner(
        &self,
        dwallet_coordinator_id: ObjectID,
    ) -> IkaResult<DWalletCoordinatorInner> {
        let result = self
            .inner
            .get_dwallet_coordinator(dwallet_coordinator_id)
            .await
            .map_err(|e| IkaError::SuiClientInternalError(format!("Can't get Coordinator: {e}")))?;
        let wrapper = bcs::from_bytes::<DWalletCoordinator>(&result).map_err(|e| {
            IkaError::SuiClientSerializationError(format!("Can't serialize Coordinator: {e}"))
        })?;

        match wrapper.version {
            1 => {
                let result = self
                    .inner
                    .get_dwallet_coordinator_inner(dwallet_coordinator_id, wrapper.version)
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
            .get_system(self.system_id)
            .await
            .map_err(|e| IkaError::SuiClientInternalError(format!("Can't get System: {e}")))?;
        let wrapper = bcs::from_bytes::<System>(&result).map_err(|e| {
            IkaError::SuiClientSerializationError(format!("Can't serialize System: {e}"))
        })?;

        match wrapper.version {
            1 => {
                let result = self
                    .inner
                    .get_system_inner(self.system_id, wrapper.version)
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

    pub async fn get_epoch_start_system(
        &self,
        ika_system_state_inner: &SystemInner,
    ) -> IkaResult<EpochStartSystem> {
        match ika_system_state_inner {
            SystemInner::V1(ika_system_state_inner) => {
                let validator_ids = ika_system_state_inner
                    .validators
                    .active_committee
                    .members
                    .iter()
                    .map(|m| m.validator_id)
                    .collect::<Vec<_>>();

                let validators = self
                    .inner
                    .get_validators_from_object_table(
                        ika_system_state_inner.validators.validators.id,
                        validator_ids,
                    )
                    .await
                    .map_err(|e| {
                        IkaError::SuiClientInternalError(format!(
                            "Can't get_validators_from_object_table: {e}"
                        ))
                    })?;
                let validators = validators
                    .iter()
                    .map(|v| {
                        bcs::from_bytes::<Validator>(&v).map_err(|e| {
                            IkaError::SuiClientSerializationError(format!(
                                "Can't serialize Validator: {e}"
                            ))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                let validators =
                    self.inner
                        .get_validator_inners(validators)
                        .await
                        .map_err(|e| {
                            IkaError::SuiClientInternalError(format!(
                                "Can't get_validator_inners: {e}"
                            ))
                        })?;
                let validators = validators
                    .iter()
                    .map(|v| {
                        bcs::from_bytes::<Field<u64, ValidatorInnerV1>>(&v).map_err(|e| {
                            IkaError::SuiClientSerializationError(format!(
                                "Can't serialize ValidatorInnerV1: {e}"
                            ))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                let validators = validators
                    .iter()
                    .map(|v| v.value.clone())
                    .collect::<Vec<_>>();

                let network_decryption_keys = self
                    .inner
                    .get_network_decryption_keys(
                        &ika_system_state_inner.dwallet_2pc_mpc_secp256k1_network_decryption_keys,
                    )
                    .await
                    .unwrap_or_default();

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
                    .validators
                    .active_committee
                    .members
                    .iter()
                    .map(|m| {
                        let validator = validators
                            .iter()
                            .find(|v| v.validator_id == m.validator_id)
                            .unwrap();
                        let metadata = validator.verified_metadata();
                        EpochStartValidatorInfoV1 {
                            validator_id: validator.validator_id,
                            protocol_pubkey: metadata.protocol_pubkey.clone(),
                            network_pubkey: metadata.network_pubkey.clone(),
                            consensus_pubkey: metadata.consensus_pubkey.clone(),
                            class_groups_public_key_and_proof: bcs::to_bytes(
                                &validators_class_groups_public_key_and_proof
                                    .get(&validator.validator_id)
                                    // Okay to `unwrap`
                                    // because we can't start the chain without the system state data.
                                    .expect("failed to get the validator class groups public key from Sui")
                                    .clone(),
                            )
                                .unwrap(),
                            network_address: metadata.network_address.clone(),
                            p2p_address: metadata.p2p_address.clone(),
                            consensus_address: metadata.consensus_address.clone(),
                            voting_power: m.voting_power,
                            hostname: metadata.name.clone(),
                        }
                    })
                    .collect::<Vec<_>>();

                let epoch_start_system_state = EpochStartSystem::new_v1(
                    ika_system_state_inner.epoch,
                    ika_system_state_inner.protocol_version,
                    ika_system_state_inner.epoch_start_timestamp_ms,
                    ika_system_state_inner.epoch_duration_ms(),
                    validators,
                    network_decryption_keys.clone(),
                );

                Ok(epoch_start_system_state)
            }
        }
    }

    /// Get the mutable system object arg on chain.
    // We retry a few times in case of errors. If it fails eventually, we panic.
    // In general it's safe to call in the beginning of the program.
    // After the first call, the result is cached since the value should never change.
    pub async fn get_mutable_system_arg_must_succeed(&self) -> ObjectArg {
        static ARG: OnceCell<ObjectArg> = OnceCell::const_new();
        *ARG.get_or_init(|| async move {
            let Ok(Ok(system_arg)) = retry_with_max_elapsed_time!(
                self.inner.get_mutable_shared_arg(self.system_id),
                Duration::from_secs(30)
            ) else {
                panic!("Failed to get system object arg after retries");
            };
            system_arg
        })
        .await
    }

    pub async fn get_clock_arg_must_succeed(&self) -> ObjectArg {
        static ARG: OnceCell<ObjectArg> = OnceCell::const_new();
        *ARG.get_or_init(|| async move {
            let Ok(Ok(system_arg)) = retry_with_max_elapsed_time!(
                self.inner.get_shared_arg(ObjectID::from_single_byte(6)),
                Duration::from_secs(30)
            ) else {
                panic!("Failed to get system object arg after retries");
            };
            system_arg
        })
        .await
    }

    /// Retrieves the dwallet_2pc_mpc_secp256k1_id object arg from the Sui chain.
    pub async fn get_mutable_dwallet_2pc_mpc_secp256k1_arg_must_succeed(
        &self,
        dwallet_2pc_mpc_secp256k1_id: ObjectID,
    ) -> ObjectArg {
        static ARG: OnceCell<ObjectArg> = OnceCell::const_new();
        *ARG.get_or_init(|| async move {
            let Ok(Ok(system_arg)) = retry_with_max_elapsed_time!(
                self.inner
                    .get_mutable_shared_arg(dwallet_2pc_mpc_secp256k1_id),
                Duration::from_secs(30)
            ) else {
                panic!("Failed to get dwallet_2pc_mpc_secp256k1_id object arg after retries");
            };
            system_arg
        })
        .await
    }

    pub async fn get_available_move_packages(
        &self,
    ) -> IkaResult<Vec<(ObjectID, MovePackageDigest)>> {
        Ok(self
            .inner
            .get_available_move_packages(self.ika_package_id, self.ika_system_package_id)
            .await
            .map_err(|e| {
                IkaError::SuiClientInternalError(format!("Can't get_available_move_packages: {e}"))
            })?)
    }

    /// Query emitted Events that are defined in the given Move Module.
    pub async fn query_events_by_module(
        &self,
        module: Identifier,
        // cursor is exclusive
        cursor: Option<EventID>,
    ) -> IkaResult<Page<SuiEvent, EventID>> {
        let filter = EventFilter::MoveEventModule {
            package: self.ika_system_package_id,
            module: module.clone(),
        };
        let events = self
            .inner
            .query_events(filter.clone(), cursor)
            .await
            .map_err(|e| IkaError::SuiClientInternalError(format!("Can't query_events: {e}")))?;

        // Safeguard check that all events are emitted from requested package and module
        assert!(events.data.iter().all(|event| event.type_.address.as_ref()
            == self.ika_system_package_id.as_ref()
            && event.type_.module == module));
        Ok(events)
    }

    pub async fn get_chain_identifier(&self) -> IkaResult<String> {
        Ok(self.inner.get_chain_identifier().await.map_err(|e| {
            IkaError::SuiClientInternalError(format!("Can't get_chain_identifier: {e}"))
        })?)
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
        Ok(self
            .inner
            .get_latest_checkpoint_sequence_number()
            .await
            .map_err(|e| {
                IkaError::SuiClientInternalError(format!(
                    "Can't get_latest_checkpoint_sequence_number: {e}"
                ))
            })?)
    }

    pub async fn execute_transaction_block_with_effects(
        &self,
        tx: sui_types::transaction::Transaction,
    ) -> IkaResult<SuiTransactionBlockResponse> {
        self.inner.execute_transaction_block_with_effects(tx).await
    }

    pub async fn get_system_inner_until_success(&self) -> SystemInner {
        loop {
            let Ok(Ok(ika_system_state)) =
                retry_with_max_elapsed_time!(self.get_system_inner(), Duration::from_secs(30))
            else {
                self.sui_client_metrics
                    .sui_rpc_errors
                    .with_label_values(&["get_system_inner_until_success"])
                    .inc();
                error!("Failed to get system inner until success");
                continue;
            };
            return ika_system_state;
        }
    }

    pub async fn get_dwallet_mpc_network_keys(
        &self,
    ) -> IkaResult<HashMap<ObjectID, NetworkDecryptionKeyShares>> {
        let system_inner = self.get_system_inner_until_success().await;
        Ok(self
            .inner
            .get_network_decryption_keys(
                system_inner
                    .into_init_version_for_tooling()
                    .dwallet_2pc_mpc_secp256k1_network_decryption_keys(),
            )
            .await
            .map_err(|e| {
                IkaError::SuiClientInternalError(format!("Can't get_network_decryption_keys: {e}"))
            })?)
    }

    pub async fn get_dwallet_coordinator_inner_until_success(
        &self,
        dwallet_state_id: ObjectID,
    ) -> DWalletCoordinatorInner {
        loop {
            let res = retry_with_max_elapsed_time!(
                self.get_dwallet_coordinator_inner(dwallet_state_id),
                Duration::from_secs(30)
            );
            let Ok(Ok(ika_system_state)) = res else {
                self.sui_client_metrics
                    .sui_rpc_errors
                    .with_label_values(&["get_dwallet_coordinator_inner_until_success"])
                    .inc();
                error!("Failed to get dwallet coordinator inner until success");
                error!(?res);
                continue;
            };
            return ika_system_state;
        }
    }

    pub async fn get_epoch_start_system_until_success(
        &self,
        system_inner: &SystemInner,
    ) -> EpochStartSystem {
        loop {
            let Ok(Ok(ika_system_state)) = retry_with_max_elapsed_time!(
                self.get_epoch_start_system(&system_inner),
                Duration::from_secs(30)
            ) else {
                self.sui_client_metrics
                    .sui_rpc_errors
                    .with_label_values(&["get_epoch_start_system_until_success"])
                    .inc();
                error!("Failed to get epoch start system until success");
                continue;
            };
            return ika_system_state;
        }
    }

    pub async fn get_gas_data_panic_if_not_gas(
        &self,
        gas_object_id: ObjectID,
    ) -> (GasCoin, ObjectRef, Owner) {
        self.inner
            .get_gas_data_panic_if_not_gas(gas_object_id)
            .await
    }

    pub async fn get_gas_data(&self, gas_object_id: ObjectID) -> (GasCoin, ObjectRef, Owner) {
        self.inner.get_gas_data(gas_object_id).await
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

    async fn get_system(&self, system_id: ObjectID) -> Result<Vec<u8>, Self::Error>;
    async fn get_clock(&self, system_id: ObjectID) -> Result<Vec<u8>, Self::Error>;
    async fn get_dwallet_coordinator(
        &self,
        dwallet_coordinator_id: ObjectID,
    ) -> Result<Vec<u8>, Self::Error>;

    async fn get_class_groups_public_keys_and_proofs(
        &self,
        validators: &Vec<ValidatorInnerV1>,
    ) -> Result<HashMap<ObjectID, ClassGroupsEncryptionKeyAndProof>, self::Error>;

    async fn get_network_decryption_keys(
        &self,
        network_decryption_caps: &Vec<DWalletNetworkDecryptionKeyCap>,
    ) -> Result<HashMap<ObjectID, NetworkDecryptionKeyShares>, self::Error>;

    async fn read_table_vec_as_raw_bytes(&self, table_id: ObjectID)
        -> Result<Vec<u8>, self::Error>;

    async fn get_system_inner(
        &self,
        system_id: ObjectID,
        version: u64,
    ) -> Result<Vec<u8>, Self::Error>;

    async fn get_dwallet_coordinator_inner(
        &self,
        dwallet_coordinator_id: ObjectID,
        version: u64,
    ) -> Result<Vec<u8>, Self::Error>;

    async fn get_validators_from_object_table(
        &self,
        validators_object_table_id: ObjectID,
        validator_ids: Vec<ObjectID>,
    ) -> Result<Vec<Vec<u8>>, Self::Error>;

    async fn get_validator_inners(
        &self,
        validators: Vec<Validator>,
    ) -> Result<Vec<Vec<u8>>, Self::Error>;

    async fn get_mutable_shared_arg(&self, system_id: ObjectID) -> Result<ObjectArg, Self::Error>;

    async fn get_shared_arg(&self, system_id: ObjectID) -> Result<ObjectArg, Self::Error>;

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

    async fn get_gas_data_panic_if_not_gas(
        &self,
        gas_object_id: ObjectID,
    ) -> (GasCoin, ObjectRef, Owner);
    async fn get_gas_data(&self, gas_object_id: ObjectID) -> (GasCoin, ObjectRef, Owner);
    async fn get_missed_events(
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

    async fn get_system(&self, system_id: ObjectID) -> Result<Vec<u8>, Self::Error> {
        self.read_api().get_move_object_bcs(system_id).await
    }

    async fn get_clock(&self, system_id: ObjectID) -> Result<Vec<u8>, Self::Error> {
        self.read_api().get_move_object_bcs(system_id).await
    }

    async fn get_dwallet_coordinator(
        &self,
        dwallet_coordinator_id: ObjectID,
    ) -> Result<Vec<u8>, Self::Error> {
        self.read_api()
            .get_move_object_bcs(dwallet_coordinator_id)
            .await
    }

    async fn get_missed_events(
        &self,
        events_bag_id: ObjectID,
    ) -> Result<Vec<DBSuiEvent>, self::Error> {
        let mut events = vec![];
        let mut next_cursor = None;
        loop {
            let dynamic_fields = self
                .read_api()
                .get_dynamic_fields(events_bag_id, next_cursor, None)
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
        validators: &Vec<ValidatorInnerV1>,
    ) -> Result<HashMap<ObjectID, ClassGroupsEncryptionKeyAndProof>, self::Error> {
        let mut class_groups_public_keys_and_proofs: HashMap<
            ObjectID,
            ClassGroupsEncryptionKeyAndProof,
        > = HashMap::new();
        for validator in validators {
            let metadata = validator.verified_metadata();
            let dynamic_fields = self
                .read_api()
                .get_dynamic_fields(
                    metadata.class_groups_public_key_and_proof.contents.id,
                    None,
                    None,
                )
                .await?;
            let mut validator_class_groups_public_key_and_proof_bytes: [Vec<u8>;
                NUM_OF_CLASS_GROUPS_KEYS] = Default::default();
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
                validator_class_groups_public_key_and_proof_bytes
                    [key_slice.name.clone() as usize] = key_slice.value.clone();
            }
            let validator_class_groups_public_key_and_proof: Result<
                Vec<SingleEncryptionKeyAndProof>,
                _,
            > = validator_class_groups_public_key_and_proof_bytes
                .into_iter()
                .map(|v| bcs::from_bytes::<SingleEncryptionKeyAndProof>(&v))
                .collect();

            class_groups_public_keys_and_proofs.insert(
                validator.validator_id,
                validator_class_groups_public_key_and_proof?
                    .try_into()
                    .map_err(|_| {
                        Error::DataError(
                            "class groups key from Sui has an invalid length".to_string(),
                        )
                    })?,
            );
        }
        Ok(class_groups_public_keys_and_proofs)
    }

    async fn get_network_decryption_keys(
        &self,
        network_decryption_caps: &Vec<DWalletNetworkDecryptionKeyCap>,
    ) -> Result<HashMap<ObjectID, NetworkDecryptionKeyShares>, self::Error> {
        let mut network_decryption_keys = HashMap::new();
        for cap in network_decryption_caps {
            let key_id = cap.dwallet_network_decryption_key_id;
            let dynamic_field_response = self
                .read_api()
                .get_object_with_options(key_id, SuiObjectDataOptions::bcs_lossless())
                .await?;
            let resp = dynamic_field_response.into_object().map_err(|e| {
                Error::DataError(format!("can't get bcs of object {:?}: {:?}", key_id, e))
            })?;
            let move_object = resp.bcs.ok_or(Error::DataError(format!(
                "object {:?} has no bcs data",
                key_id
            )))?;
            let raw_move_obj = move_object.try_into_move().ok_or(Error::DataError(format!(
                "object {:?} is not a MoveObject",
                key_id
            )))?;
            let key_obj = bcs::from_bytes::<DWalletNetworkDecryptionKey>(&raw_move_obj.bcs_bytes)
                .map_err(|e| {
                Error::DataError(format!("can't deserialize object {:?}: {:?}", key_id, e))
            })?;
            if DWalletNetworkDecryptionKeyState::NetworkDKGCompleted != key_obj.state {
                continue;
            }
            let public_output_bytes = self
                .read_table_vec_as_raw_bytes(key_obj.public_output.contents.id)
                .await?;
            let public_output =
                bcs::from_bytes::<NetworkDecryptionKeyOnChainOutput>(&public_output_bytes)?;
            let current_shares = self
                .read_table_vec_as_raw_bytes(key_obj.current_epoch_shares.contents.id)
                .await?;
            let key = NetworkDecryptionKeyShares {
                epoch: key_obj.current_epoch,
                current_epoch_encryptions_of_shares_per_crt_prime: current_shares,
                previous_epoch_encryptions_of_shares_per_crt_prime: vec![],
                encryption_scheme_public_parameters: public_output
                    .encryption_scheme_public_parameters,
                decryption_key_share_public_parameters: public_output
                    .decryption_key_share_public_parameters,
                encryption_key: public_output.encryption_key,
                public_verification_keys: public_output.public_verification_keys,
                setup_parameters_per_crt_prime: public_output.setup_parameters_per_crt_prime,
            };
            network_decryption_keys.insert(key_id, key);
        }
        Ok(network_decryption_keys)
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
            .fold(Vec::new(), |mut acc, (k, mut v)| {
                acc.append(&mut v);
                acc
            }))
    }

    async fn get_system_inner(
        &self,
        system_id: ObjectID,
        version: u64,
    ) -> Result<Vec<u8>, Self::Error> {
        let dynamic_fields = self
            .read_api()
            .get_dynamic_fields(system_id, None, None)
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
                .get_dynamic_field_object(system_id, dynamic_field.name.clone())
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
            system_id, version
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

    async fn get_validators_from_object_table(
        &self,
        validators_object_table_id: ObjectID,
        validator_ids: Vec<ObjectID>,
    ) -> Result<Vec<Vec<u8>>, Self::Error> {
        let mut validator_dynamic_ids = Vec::new();
        let mut cursor = None;
        loop {
            let dynamic_fields = self
                .read_api()
                .get_dynamic_fields(validators_object_table_id, cursor, None)
                .await?;

            for dynamic_field in &dynamic_fields.data {
                let name = &dynamic_field.name.value;

                let bytes = name.as_str().unwrap();

                let validator_id: ObjectID =
                    AccountAddress::from_hex_literal(bytes).unwrap().into();

                if validator_ids.contains(&validator_id) {
                    let result = self
                        .read_api()
                        .get_dynamic_field_object(
                            validators_object_table_id,
                            dynamic_field.name.clone(),
                        )
                        .await?;

                    if let Some(dynamic_field) = result.data {
                        validator_dynamic_ids.push(dynamic_field.object_id);
                    }
                }
            }

            cursor = dynamic_fields.next_cursor;
            if !dynamic_fields.has_next_page {
                break;
            }
        }

        let dynamic_field_response = self
            .read_api()
            .multi_get_object_with_options(
                validator_dynamic_ids.clone(),
                SuiObjectDataOptions::bcs_lossless(),
            )
            .await?;
        let mut validators = Vec::new();
        for (dynamic_field, object_id) in dynamic_field_response
            .iter()
            .zip(validator_dynamic_ids.iter())
        {
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

    async fn get_mutable_shared_arg(&self, system_id: ObjectID) -> Result<ObjectArg, Self::Error> {
        let response = self
            .read_api()
            .get_object_with_options(system_id, SuiObjectDataOptions::new().with_owner())
            .await?;
        let Some(Owner::Shared {
            initial_shared_version,
        }) = response.owner()
        else {
            return Err(Self::Error::DataError(format!(
                "Failed to load ika system state owner {:?}",
                system_id
            )));
        };
        Ok(ObjectArg::SharedObject {
            id: system_id,
            initial_shared_version,
            mutable: true,
        })
    }

    async fn get_shared_arg(&self, system_id: ObjectID) -> Result<ObjectArg, Self::Error> {
        let response = self
            .read_api()
            .get_object_with_options(system_id, SuiObjectDataOptions::new().with_owner())
            .await?;
        let Some(Owner::Shared {
            initial_shared_version,
        }) = response.owner()
        else {
            return Err(Self::Error::DataError(format!(
                "Failed to load ika system state owner {:?}",
                system_id
            )));
        };
        Ok(ObjectArg::SharedObject {
            id: system_id,
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

    async fn get_gas_data_panic_if_not_gas(
        &self,
        gas_object_id: ObjectID,
    ) -> (GasCoin, ObjectRef, Owner) {
        loop {
            match self
                .read_api()
                .get_object_with_options(
                    gas_object_id,
                    SuiObjectDataOptions::default().with_owner().with_content(),
                )
                .await
                .map(|resp| resp.data)
            {
                Ok(Some(gas_obj)) => {
                    let owner = gas_obj.owner.clone().expect("Owner is requested");
                    let gas_coin = GasCoin::try_from(&gas_obj)
                        .unwrap_or_else(|err| panic!("{} is not a gas coin: {err}", gas_object_id));
                    return (gas_coin, gas_obj.object_ref(), owner);
                }
                other => {
                    warn!("Can't get gas object: {:?}: {:?}", gas_object_id, other);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }

    async fn get_gas_data(&self, gas_object_id: ObjectID) -> (GasCoin, ObjectRef, Owner) {
        loop {
            if let Ok(Some(gas_obj)) = self
                .read_api()
                .get_object_with_options(
                    gas_object_id,
                    SuiObjectDataOptions::default().with_owner().with_content(),
                )
                .await
                .map(|resp| resp.data)
            {
                let owner = gas_obj.owner.clone().expect("Owner is requested");
                if let Ok(gas_coin) = GasCoin::try_from(&gas_obj) {
                    return (gas_coin, gas_obj.object_ref(), owner);
                }
            }
            warn!("Can't get gas object: {:?}", gas_object_id);
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}
