// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//! This module contains the transactional test runner instantiation for the Pera adapter

pub mod args;
pub mod programmable_transaction_test_parser;
mod simulator_persisted_store;
pub mod test_adapter;

pub use move_transactional_test_runner::framework::run_test_impl;
use pera_core::authority::authority_test_utils::send_and_confirm_transaction_with_execution_error;
use pera_core::authority::AuthorityState;
use pera_json_rpc::authority_state::StateRead;
use pera_json_rpc_types::DevInspectResults;
use pera_json_rpc_types::EventFilter;
use pera_storage::key_value_store::TransactionKeyValueStore;
use pera_types::base_types::ObjectID;
use pera_types::base_types::PeraAddress;
use pera_types::base_types::VersionNumber;
use pera_types::committee::EpochId;
use pera_types::digests::TransactionDigest;
use pera_types::digests::TransactionEventsDigest;
use pera_types::effects::TransactionEffects;
use pera_types::effects::TransactionEvents;
use pera_types::error::ExecutionError;
use pera_types::error::PeraError;
use pera_types::error::PeraResult;
use pera_types::event::Event;
use pera_types::executable_transaction::{ExecutableTransaction, VerifiedExecutableTransaction};
use pera_types::messages_checkpoint::CheckpointContentsDigest;
use pera_types::messages_checkpoint::VerifiedCheckpoint;
use pera_types::object::Object;
use pera_types::pera_system_state::epoch_start_pera_system_state::EpochStartSystemStateTrait;
use pera_types::pera_system_state::PeraSystemStateTrait;
use pera_types::storage::ObjectStore;
use pera_types::storage::ReadStore;
use pera_types::transaction::InputObjects;
use pera_types::transaction::Transaction;
use pera_types::transaction::TransactionDataAPI;
use pera_types::transaction::TransactionKind;
use rand::rngs::StdRng;
use simulacrum::Simulacrum;
use simulacrum::SimulatorStore;
use simulator_persisted_store::PersistedStore;
use std::path::Path;
use std::sync::Arc;
use test_adapter::{PeraTestAdapter, PRE_COMPILED};

#[cfg_attr(not(msim), tokio::main)]
#[cfg_attr(msim, msim::main)]
pub async fn run_test(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let (_guard, _filter_handle) = telemetry_subscribers::TelemetryConfig::new()
        .with_env()
        .init();
    run_test_impl::<PeraTestAdapter>(path, Some(std::sync::Arc::new(PRE_COMPILED.clone()))).await?;
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
pub trait TransactionalAdapter: Send + Sync + ReadStore {
    async fn execute_txn(
        &mut self,
        transaction: Transaction,
    ) -> anyhow::Result<(TransactionEffects, Option<ExecutionError>)>;

    async fn read_input_objects(&self, transaction: Transaction) -> PeraResult<InputObjects>;

    fn prepare_txn(
        &self,
        transaction: Transaction,
        input_objects: InputObjects,
    ) -> anyhow::Result<(TransactionEffects, Option<ExecutionError>)>;

    async fn create_checkpoint(&mut self) -> anyhow::Result<VerifiedCheckpoint>;

    async fn advance_clock(
        &mut self,
        duration: std::time::Duration,
    ) -> anyhow::Result<TransactionEffects>;

    async fn advance_epoch(&mut self, create_random_state: bool) -> anyhow::Result<()>;

    async fn request_gas(
        &mut self,
        address: PeraAddress,
        amount: u64,
    ) -> anyhow::Result<TransactionEffects>;

    async fn dev_inspect_transaction_block(
        &self,
        sender: PeraAddress,
        transaction_kind: TransactionKind,
        gas_price: Option<u64>,
    ) -> PeraResult<DevInspectResults>;

    async fn query_tx_events_asc(
        &self,
        tx_digest: &TransactionDigest,
        limit: usize,
    ) -> PeraResult<Vec<Event>>;

    async fn get_active_validator_addresses(&self) -> PeraResult<Vec<PeraAddress>>;
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
            false,
        )
        .await?;
        Ok((effects.into_data(), execution_error))
    }

    async fn read_input_objects(&self, transaction: Transaction) -> PeraResult<InputObjects> {
        let tx = VerifiedExecutableTransaction::new_unchecked(
            ExecutableTransaction::new_from_data_and_sig(
                transaction.data().clone(),
                pera_types::executable_transaction::CertificateProof::Checkpoint(0, 0),
            ),
        );

        let epoch_store = self.validator.load_epoch_store_one_call_per_task().clone();
        self.validator.read_objects_for_execution(&tx, &epoch_store)
    }

    fn prepare_txn(
        &self,
        transaction: Transaction,
        input_objects: InputObjects,
    ) -> anyhow::Result<(TransactionEffects, Option<ExecutionError>)> {
        let tx = VerifiedExecutableTransaction::new_unchecked(
            ExecutableTransaction::new_from_data_and_sig(
                transaction.data().clone(),
                pera_types::executable_transaction::CertificateProof::Checkpoint(0, 0),
            ),
        );

        let epoch_store = self.validator.load_epoch_store_one_call_per_task().clone();
        let (_, effects, error) =
            self.validator
                .prepare_certificate_for_benchmark(&tx, input_objects, &epoch_store)?;
        Ok((effects, error))
    }

    async fn dev_inspect_transaction_block(
        &self,
        sender: PeraAddress,
        transaction_kind: TransactionKind,
        gas_price: Option<u64>,
    ) -> PeraResult<DevInspectResults> {
        self.fullnode
            .dev_inspect_transaction_block(
                sender,
                transaction_kind,
                gas_price,
                None,
                None,
                None,
                None,
                None,
            )
            .await
    }

    async fn query_tx_events_asc(
        &self,
        tx_digest: &TransactionDigest,
        limit: usize,
    ) -> PeraResult<Vec<Event>> {
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
            .map(|pera_event| pera_event.into())
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
        self.validator.reconfigure_for_testing().await;
        self.fullnode.reconfigure_for_testing().await;
        Ok(())
    }

    async fn request_gas(
        &mut self,
        _address: PeraAddress,
        _amount: u64,
    ) -> anyhow::Result<TransactionEffects> {
        unimplemented!("request_gas not supported")
    }

    async fn get_active_validator_addresses(&self) -> PeraResult<Vec<PeraAddress>> {
        Ok(self
            .fullnode
            .get_system_state()
            .map_err(|e| {
                PeraError::PeraSystemStateReadError(format!(
                    "Failed to get system state from fullnode: {}",
                    e
                ))
            })?
            .into_pera_system_state_summary()
            .active_validators
            .iter()
            .map(|x| x.pera_address)
            .collect::<Vec<_>>())
    }
}

impl ReadStore for ValidatorWithFullnode {
    fn get_committee(
        &self,
        _epoch: pera_types::committee::EpochId,
    ) -> pera_types::storage::error::Result<Option<Arc<pera_types::committee::Committee>>> {
        todo!()
    }

    fn get_latest_epoch_id(&self) -> pera_types::storage::error::Result<EpochId> {
        Ok(self.validator.epoch_store_for_testing().epoch())
    }

    fn get_latest_checkpoint(&self) -> pera_types::storage::error::Result<VerifiedCheckpoint> {
        let sequence_number = self
            .validator
            .get_latest_checkpoint_sequence_number()
            .unwrap();
        self.get_checkpoint_by_sequence_number(sequence_number)
            .map(|c| c.unwrap())
    }

    fn get_highest_verified_checkpoint(
        &self,
    ) -> pera_types::storage::error::Result<VerifiedCheckpoint> {
        todo!()
    }

    fn get_highest_synced_checkpoint(
        &self,
    ) -> pera_types::storage::error::Result<VerifiedCheckpoint> {
        todo!()
    }

    fn get_lowest_available_checkpoint(
        &self,
    ) -> pera_types::storage::error::Result<pera_types::messages_checkpoint::CheckpointSequenceNumber>
    {
        todo!()
    }

    fn get_checkpoint_by_digest(
        &self,
        _digest: &pera_types::messages_checkpoint::CheckpointDigest,
    ) -> pera_types::storage::error::Result<Option<VerifiedCheckpoint>> {
        todo!()
    }

    fn get_checkpoint_by_sequence_number(
        &self,
        sequence_number: pera_types::messages_checkpoint::CheckpointSequenceNumber,
    ) -> pera_types::storage::error::Result<Option<VerifiedCheckpoint>> {
        self.validator
            .get_checkpoint_store()
            .get_checkpoint_by_sequence_number(sequence_number)
            .map_err(pera_types::storage::error::Error::custom)
    }

    fn get_checkpoint_contents_by_digest(
        &self,
        digest: &CheckpointContentsDigest,
    ) -> pera_types::storage::error::Result<
        Option<pera_types::messages_checkpoint::CheckpointContents>,
    > {
        self.validator
            .get_checkpoint_store()
            .get_checkpoint_contents(digest)
            .map_err(pera_types::storage::error::Error::custom)
    }

    fn get_checkpoint_contents_by_sequence_number(
        &self,
        _sequence_number: pera_types::messages_checkpoint::CheckpointSequenceNumber,
    ) -> pera_types::storage::error::Result<
        Option<pera_types::messages_checkpoint::CheckpointContents>,
    > {
        todo!()
    }

    fn get_transaction(
        &self,
        tx_digest: &TransactionDigest,
    ) -> pera_types::storage::error::Result<Option<Arc<pera_types::transaction::VerifiedTransaction>>>
    {
        self.validator
            .get_transaction_cache_reader()
            .get_transaction_block(tx_digest)
            .map_err(pera_types::storage::error::Error::custom)
    }

    fn get_transaction_effects(
        &self,
        tx_digest: &TransactionDigest,
    ) -> pera_types::storage::error::Result<Option<TransactionEffects>> {
        self.validator
            .get_transaction_cache_reader()
            .get_executed_effects(tx_digest)
            .map_err(pera_types::storage::error::Error::custom)
    }

    fn get_events(
        &self,
        event_digest: &TransactionEventsDigest,
    ) -> pera_types::storage::error::Result<Option<TransactionEvents>> {
        self.validator
            .get_transaction_cache_reader()
            .get_events(event_digest)
            .map_err(pera_types::storage::error::Error::custom)
    }

    fn get_full_checkpoint_contents_by_sequence_number(
        &self,
        _sequence_number: pera_types::messages_checkpoint::CheckpointSequenceNumber,
    ) -> pera_types::storage::error::Result<
        Option<pera_types::messages_checkpoint::FullCheckpointContents>,
    > {
        todo!()
    }

    fn get_full_checkpoint_contents(
        &self,
        _digest: &CheckpointContentsDigest,
    ) -> pera_types::storage::error::Result<
        Option<pera_types::messages_checkpoint::FullCheckpointContents>,
    > {
        todo!()
    }
}

impl ObjectStore for ValidatorWithFullnode {
    fn get_object(
        &self,
        object_id: &ObjectID,
    ) -> Result<Option<Object>, pera_types::storage::error::Error> {
        self.validator.get_object_store().get_object(object_id)
    }

    fn get_object_by_key(
        &self,
        object_id: &ObjectID,
        version: VersionNumber,
    ) -> Result<Option<Object>, pera_types::storage::error::Error> {
        self.validator
            .get_object_store()
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

    async fn read_input_objects(&self, _transaction: Transaction) -> PeraResult<InputObjects> {
        unimplemented!("read_input_objects not supported in simulator mode")
    }

    fn prepare_txn(
        &self,
        _transaction: Transaction,
        _input_objects: InputObjects,
    ) -> anyhow::Result<(TransactionEffects, Option<ExecutionError>)> {
        unimplemented!("prepare_txn not supported in simulator mode")
    }

    async fn dev_inspect_transaction_block(
        &self,
        _sender: PeraAddress,
        _transaction_kind: TransactionKind,
        _gas_price: Option<u64>,
    ) -> PeraResult<DevInspectResults> {
        unimplemented!("dev_inspect_transaction_block not supported in simulator mode")
    }

    async fn query_tx_events_asc(
        &self,
        tx_digest: &TransactionDigest,
        _limit: usize,
    ) -> PeraResult<Vec<Event>> {
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
        address: PeraAddress,
        amount: u64,
    ) -> anyhow::Result<TransactionEffects> {
        self.request_gas(address, amount)
    }

    async fn get_active_validator_addresses(&self) -> PeraResult<Vec<PeraAddress>> {
        // TODO: this is a hack to get the validator addresses. Currently using start state
        //       but we should have a better way to get this information after reconfig
        Ok(self.epoch_start_state().get_validator_addresses())
    }
}
