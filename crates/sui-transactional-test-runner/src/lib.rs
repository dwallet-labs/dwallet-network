// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//! This module contains the transactional test runner instantiation for the Sui adapter

pub mod args;
pub mod programmable_transaction_test_parser;
mod simulator_persisted_store;
pub mod test_adapter;

pub use move_transactional_test_runner::framework::run_test_impl;
use rand::rngs::StdRng;
use simulacrum::Simulacrum;
use simulacrum::SimulatorStore;
use simulator_persisted_store::PersistedStore;
use std::path::Path;
use std::sync::Arc;
use sui_core::authority::authority_test_utils::send_and_confirm_transaction_with_execution_error;
use sui_core::authority::AuthorityState;
use sui_json_rpc::authority_state::StateRead;
use sui_json_rpc_types::DevInspectResults;
use sui_json_rpc_types::EventFilter;
use sui_rest_api::node_state_getter::NodeStateGetter;
use sui_storage::key_value_store::TransactionKeyValueStore;
use sui_types::base_types::ObjectID;
use sui_types::base_types::SuiAddress;
use sui_types::base_types::VersionNumber;
use sui_types::digests::TransactionDigest;
use sui_types::digests::TransactionEventsDigest;
use sui_types::effects::TransactionEffects;
use sui_types::effects::TransactionEvents;
use sui_types::error::ExecutionError;
use sui_types::error::SuiError;
use sui_types::error::SuiResult;
use sui_types::event::Event;
use sui_types::messages_checkpoint::CheckpointContentsDigest;
use sui_types::messages_checkpoint::VerifiedCheckpoint;
use sui_types::object::Object;
use sui_types::storage::ObjectKey;
use sui_types::storage::ObjectStore;
use sui_types::sui_system_state::epoch_start_sui_system_state::EpochStartSystemStateTrait;
use sui_types::sui_system_state::SuiSystemStateTrait;
use sui_types::transaction::Transaction;
use sui_types::transaction::TransactionDataAPI;
use sui_types::transaction::TransactionKind;
use test_adapter::{SuiTestAdapter, PRE_COMPILED};

#[cfg_attr(not(msim), tokio::main)]
#[cfg_attr(msim, msim::main)]
pub async fn run_test(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    telemetry_subscribers::init_for_testing();
    run_test_impl::<SuiTestAdapter>(path, Some(&*PRE_COMPILED)).await?;
    Ok(())
}

pub struct ValidatorWithFullnode {
    pub validator: Arc<AuthorityState>,
    pub fullnode: Arc<AuthorityState>,
    pub kv_store: Arc<TransactionKeyValueStore>,
}

#[allow(unused_variables)]
/// TODO: better name?
#[async_trait::async_trait]
pub trait TransactionalAdapter: Send + Sync + ObjectStore + NodeStateGetter {
    async fn execute_txn(
        &mut self,
        transaction: Transaction,
    ) -> anyhow::Result<(TransactionEffects, Option<ExecutionError>)>;

    async fn create_checkpoint(&mut self) -> anyhow::Result<VerifiedCheckpoint>;

    async fn advance_clock(
        &mut self,
        duration: std::time::Duration,
    ) -> anyhow::Result<TransactionEffects>;

    async fn advance_epoch(&mut self, create_random_state: bool) -> anyhow::Result<()>;

    async fn request_gas(
        &mut self,
        address: SuiAddress,
        amount: u64,
    ) -> anyhow::Result<TransactionEffects>;

    async fn dev_inspect_transaction_block(
        &self,
        sender: SuiAddress,
        transaction_kind: TransactionKind,
        gas_price: Option<u64>,
    ) -> SuiResult<DevInspectResults>;

    async fn query_tx_events_asc(
        &self,
        tx_digest: &TransactionDigest,
        limit: usize,
    ) -> SuiResult<Vec<Event>>;

    async fn get_active_validator_addresses(&self) -> SuiResult<Vec<SuiAddress>>;
}

#[async_trait::async_trait]
impl TransactionalAdapter for ValidatorWithFullnode {
    async fn execute_txn(
        &mut self,
        transaction: Transaction,
    ) -> anyhow::Result<(TransactionEffects, Option<ExecutionError>)> {
        let with_shared = transaction
            .data()
            .intent_message()
            .value
            .contains_shared_object();
        let (_, effects, execution_error) = send_and_confirm_transaction_with_execution_error(
            &self.validator,
            Some(&self.fullnode),
            transaction,
            with_shared,
        )
        .await?;
        Ok((effects.into_data(), execution_error))
    }

    async fn dev_inspect_transaction_block(
        &self,
        sender: SuiAddress,
        transaction_kind: TransactionKind,
        gas_price: Option<u64>,
    ) -> SuiResult<DevInspectResults> {
        self.fullnode
            .dev_inspect_transaction_block(sender, transaction_kind, gas_price)
            .await
    }

    async fn query_tx_events_asc(
        &self,
        tx_digest: &TransactionDigest,
        limit: usize,
    ) -> SuiResult<Vec<Event>> {
        Ok(self
            .validator
            .query_events(
                &self.kv_store,
                EventFilter::Transaction(*tx_digest),
                None,
                limit,
                false,
            )
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|sui_event| sui_event.into())
            .collect())
    }

    async fn create_checkpoint(&mut self) -> anyhow::Result<VerifiedCheckpoint> {
        unimplemented!("create_checkpoint not supported")
    }

    async fn advance_clock(
        &mut self,
        _duration: std::time::Duration,
    ) -> anyhow::Result<TransactionEffects> {
        unimplemented!("advance_clock not supported")
    }

    async fn advance_epoch(&mut self, _create_random_state: bool) -> anyhow::Result<()> {
        unimplemented!("advance_epoch not supported")
    }

    async fn request_gas(
        &mut self,
        _address: SuiAddress,
        _amount: u64,
    ) -> anyhow::Result<TransactionEffects> {
        unimplemented!("request_gas not supported")
    }

    async fn get_active_validator_addresses(&self) -> SuiResult<Vec<SuiAddress>> {
        Ok(self
            .fullnode
            .get_system_state()
            .map_err(|e| {
                SuiError::SuiSystemStateReadError(format!(
                    "Failed to get system state from fullnode: {}",
                    e
                ))
            })?
            .into_sui_system_state_summary()
            .active_validators
            .iter()
            .map(|x| x.sui_address)
            .collect::<Vec<_>>())
    }
}

#[async_trait::async_trait]
impl NodeStateGetter for ValidatorWithFullnode {
    fn get_verified_checkpoint_by_sequence_number(
        &self,
        sequence_number: u64,
    ) -> SuiResult<VerifiedCheckpoint> {
        self.validator
            .get_verified_checkpoint_by_sequence_number(sequence_number)
    }

    fn get_latest_checkpoint_sequence_number(&self) -> SuiResult<u64> {
        self.validator.get_latest_checkpoint_sequence_number()
    }

    fn get_checkpoint_contents(
        &self,
        content_digest: CheckpointContentsDigest,
    ) -> SuiResult<sui_types::messages_checkpoint::CheckpointContents> {
        self.validator.get_checkpoint_contents(content_digest)
    }

    fn multi_get_transaction_blocks(
        &self,
        tx_digests: &[TransactionDigest],
    ) -> SuiResult<Vec<Option<sui_types::transaction::VerifiedTransaction>>> {
        self.validator.multi_get_transaction_blocks(tx_digests)
    }

    fn multi_get_executed_effects(
        &self,
        digests: &[TransactionDigest],
    ) -> SuiResult<Vec<Option<sui_types::effects::TransactionEffects>>> {
        self.validator.multi_get_executed_effects(digests)
    }

    fn multi_get_events(
        &self,
        event_digests: &[TransactionEventsDigest],
    ) -> SuiResult<Vec<Option<TransactionEvents>>> {
        self.validator.multi_get_events(event_digests)
    }

    fn multi_get_object_by_key(
        &self,
        object_keys: &[ObjectKey],
    ) -> Result<Vec<Option<Object>>, SuiError> {
        self.validator.multi_get_object_by_key(object_keys)
    }

    fn get_object_by_key(
        &self,
        object_id: &ObjectID,
        version: VersionNumber,
    ) -> Result<Option<Object>, SuiError> {
        self.validator.get_object_by_key(object_id, version)
    }

    fn get_object(&self, object_id: &ObjectID) -> Result<Option<Object>, SuiError> {
        self.validator.database.get_object(object_id)
    }
}

impl ObjectStore for ValidatorWithFullnode {
    fn get_object(&self, object_id: &ObjectID) -> Result<Option<Object>, SuiError> {
        self.validator.database.get_object(object_id)
    }

    fn get_object_by_key(
        &self,
        object_id: &ObjectID,
        version: VersionNumber,
    ) -> Result<Option<Object>, SuiError> {
        self.validator
            .database
            .get_object_by_key(object_id, version)
    }
}

#[async_trait::async_trait]
impl TransactionalAdapter for Simulacrum<StdRng, PersistedStore> {
    async fn execute_txn(
        &mut self,
        transaction: Transaction,
    ) -> anyhow::Result<(TransactionEffects, Option<ExecutionError>)> {
        Ok(self.execute_transaction(transaction)?)
    }

    async fn dev_inspect_transaction_block(
        &self,
        _sender: SuiAddress,
        _transaction_kind: TransactionKind,
        _gas_price: Option<u64>,
    ) -> SuiResult<DevInspectResults> {
        unimplemented!("dev_inspect_transaction_block not supported in simulator mode")
    }

    async fn query_tx_events_asc(
        &self,
        tx_digest: &TransactionDigest,
        _limit: usize,
    ) -> SuiResult<Vec<Event>> {
        Ok(self
            .store()
            .get_transaction_events_by_tx_digest(tx_digest)
            .map(|x| x.data)
            .unwrap_or_default())
    }

    async fn create_checkpoint(&mut self) -> anyhow::Result<VerifiedCheckpoint> {
        Ok(self.create_checkpoint())
    }

    async fn advance_clock(
        &mut self,
        duration: std::time::Duration,
    ) -> anyhow::Result<TransactionEffects> {
        Ok(self.advance_clock(duration))
    }

    async fn advance_epoch(&mut self, create_random_state: bool) -> anyhow::Result<()> {
        self.advance_epoch(create_random_state);
        Ok(())
    }

    async fn request_gas(
        &mut self,
        address: SuiAddress,
        amount: u64,
    ) -> anyhow::Result<TransactionEffects> {
        self.request_gas(address, amount)
    }

    async fn get_active_validator_addresses(&self) -> SuiResult<Vec<SuiAddress>> {
        // TODO: this is a hack to get the validator addresses. Currently using start state
        //       but we should have a better way to get this information after reconfig
        Ok(self.epoch_start_state().get_validator_addresses())
    }
}
