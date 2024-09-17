// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::path::PathBuf;
use std::{collections::HashSet, sync::Arc};

use move_binary_format::CompiledModule;
use move_vm_config::verifier::{MeterConfig, VerifierConfig};
use pera_protocol_config::ProtocolConfig;
use pera_types::{
    base_types::{ObjectRef, PeraAddress, TxContext},
    committee::EpochId,
    digests::TransactionDigest,
    effects::TransactionEffects,
    error::{ExecutionError, PeraError, PeraResult},
    execution::{ExecutionResult, TypeLayoutStore},
    gas::PeraGasStatus,
    inner_temporary_store::InnerTemporaryStore,
    layout_resolver::LayoutResolver,
    metrics::{BytecodeVerifierMetrics, LimitsMetrics},
    transaction::{CheckedInputObjects, ProgrammableTransaction, TransactionKind},
};

use move_bytecode_verifier_meter::Meter;
use move_vm_runtime_v2::move_vm::MoveVM;
use pera_adapter_v2::adapter::{new_move_vm, run_metered_move_bytecode_verifier};
use pera_adapter_v2::execution_engine::{
    execute_genesis_state_update, execute_transaction_to_effects,
};
use pera_adapter_v2::execution_mode;
use pera_adapter_v2::type_layout_resolver::TypeLayoutResolver;
use pera_move_natives_v2::all_natives;
use pera_types::storage::BackingStore;
use pera_verifier_v2::meter::PeraVerifierMeter;

use crate::executor;
use crate::verifier;

pub(crate) struct Executor(Arc<MoveVM>);

pub(crate) struct Verifier<'m> {
    config: VerifierConfig,
    metrics: &'m Arc<BytecodeVerifierMetrics>,
}

impl Executor {
    pub(crate) fn new(
        protocol_config: &ProtocolConfig,
        silent: bool,
        enable_profiler: Option<PathBuf>,
    ) -> Result<Self, PeraError> {
        Ok(Executor(Arc::new(new_move_vm(
            all_natives(silent),
            protocol_config,
            enable_profiler,
        )?)))
    }
}

impl<'m> Verifier<'m> {
    pub(crate) fn new(config: VerifierConfig, metrics: &'m Arc<BytecodeVerifierMetrics>) -> Self {
        Verifier { config, metrics }
    }
}

impl executor::Executor for Executor {
    fn execute_transaction_to_effects(
        &self,
        store: &dyn BackingStore,
        protocol_config: &ProtocolConfig,
        metrics: Arc<LimitsMetrics>,
        enable_expensive_checks: bool,
        certificate_deny_set: &HashSet<TransactionDigest>,
        epoch_id: &EpochId,
        epoch_timestamp_ms: u64,
        input_objects: CheckedInputObjects,
        gas_coins: Vec<ObjectRef>,
        gas_status: PeraGasStatus,
        transaction_kind: TransactionKind,
        transaction_signer: PeraAddress,
        transaction_digest: TransactionDigest,
    ) -> (
        InnerTemporaryStore,
        PeraGasStatus,
        TransactionEffects,
        Result<(), ExecutionError>,
    ) {
        execute_transaction_to_effects::<execution_mode::Normal>(
            store,
            input_objects,
            gas_coins,
            gas_status,
            transaction_kind,
            transaction_signer,
            transaction_digest,
            &self.0,
            epoch_id,
            epoch_timestamp_ms,
            protocol_config,
            metrics,
            enable_expensive_checks,
            certificate_deny_set,
        )
    }

    fn dev_inspect_transaction(
        &self,
        store: &dyn BackingStore,
        protocol_config: &ProtocolConfig,
        metrics: Arc<LimitsMetrics>,
        enable_expensive_checks: bool,
        certificate_deny_set: &HashSet<TransactionDigest>,
        epoch_id: &EpochId,
        epoch_timestamp_ms: u64,
        input_objects: CheckedInputObjects,
        gas_coins: Vec<ObjectRef>,
        gas_status: PeraGasStatus,
        transaction_kind: TransactionKind,
        transaction_signer: PeraAddress,
        transaction_digest: TransactionDigest,
        skip_all_checks: bool,
    ) -> (
        InnerTemporaryStore,
        PeraGasStatus,
        TransactionEffects,
        Result<Vec<ExecutionResult>, ExecutionError>,
    ) {
        if skip_all_checks {
            execute_transaction_to_effects::<execution_mode::DevInspect<true>>(
                store,
                input_objects,
                gas_coins,
                gas_status,
                transaction_kind,
                transaction_signer,
                transaction_digest,
                &self.0,
                epoch_id,
                epoch_timestamp_ms,
                protocol_config,
                metrics,
                enable_expensive_checks,
                certificate_deny_set,
            )
        } else {
            execute_transaction_to_effects::<execution_mode::DevInspect<false>>(
                store,
                input_objects,
                gas_coins,
                gas_status,
                transaction_kind,
                transaction_signer,
                transaction_digest,
                &self.0,
                epoch_id,
                epoch_timestamp_ms,
                protocol_config,
                metrics,
                enable_expensive_checks,
                certificate_deny_set,
            )
        }
    }

    fn update_genesis_state(
        &self,
        store: &dyn BackingStore,
        protocol_config: &ProtocolConfig,
        metrics: Arc<LimitsMetrics>,
        tx_context: &mut TxContext,
        input_objects: CheckedInputObjects,
        pt: ProgrammableTransaction,
    ) -> Result<InnerTemporaryStore, ExecutionError> {
        execute_genesis_state_update(
            store,
            protocol_config,
            metrics,
            &self.0,
            tx_context,
            input_objects,
            pt,
        )
    }

    fn type_layout_resolver<'r, 'vm: 'r, 'store: 'r>(
        &'vm self,
        store: Box<dyn TypeLayoutStore + 'store>,
    ) -> Box<dyn LayoutResolver + 'r> {
        Box::new(TypeLayoutResolver::new(&self.0, store))
    }
}

impl<'m> verifier::Verifier for Verifier<'m> {
    fn meter(&self, config: MeterConfig) -> Box<dyn Meter> {
        Box::new(PeraVerifierMeter::new(config))
    }

    fn meter_compiled_modules(
        &mut self,
        _protocol_config: &ProtocolConfig,
        modules: &[CompiledModule],
        meter: &mut dyn Meter,
    ) -> PeraResult<()> {
        run_metered_move_bytecode_verifier(modules, &self.config, meter, self.metrics)
    }
}
