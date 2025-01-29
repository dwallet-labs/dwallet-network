// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

pub use checked::*;

#[pera_macros::with_checked_arithmetic]
mod checked {
    use crate::execution_mode::{self, ExecutionMode};
    use dwallet_mpc_types::dwallet_mpc::{
        MPCPublicOutput, DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME, DWALLET_MODULE_NAME,
    };
    use move_binary_format::CompiledModule;
    use move_vm_runtime::move_vm::MoveVM;
    use pera_types::balance::{
        BALANCE_CREATE_REWARDS_FUNCTION_NAME, BALANCE_DESTROY_REBATES_FUNCTION_NAME,
        BALANCE_MODULE_NAME,
    };
    use pera_types::gas_coin::GAS;
    use pera_types::messages_checkpoint::CheckpointTimestamp;
    use pera_types::metrics::LimitsMetrics;
    use pera_types::object::OBJECT_START_VERSION;
    use pera_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
    use pera_types::randomness_state::{
        RANDOMNESS_MODULE_NAME, RANDOMNESS_STATE_CREATE_FUNCTION_NAME,
        RANDOMNESS_STATE_UPDATE_FUNCTION_NAME,
    };
    use pera_types::{BRIDGE_ADDRESS, PERA_BRIDGE_OBJECT_ID, PERA_RANDOMNESS_STATE_OBJECT_ID};
    use std::{collections::HashSet, sync::Arc};
    use tracing::{info, instrument, trace, warn};

    use crate::adapter::new_move_vm;
    use crate::programmable_transactions;
    use crate::type_layout_resolver::TypeLayoutResolver;
    use crate::{gas_charger::GasCharger, temporary_store::TemporaryStore};
    use move_core_types::ident_str;
    use pera_move_natives::all_natives;
    use pera_protocol_config::{check_limit_by_meter, LimitThresholdCrossed, ProtocolConfig};
    use pera_types::authenticator_state::{
        AUTHENTICATOR_STATE_CREATE_FUNCTION_NAME, AUTHENTICATOR_STATE_EXPIRE_JWKS_FUNCTION_NAME,
        AUTHENTICATOR_STATE_MODULE_NAME, AUTHENTICATOR_STATE_UPDATE_FUNCTION_NAME,
    };
    use pera_types::base_types::SequenceNumber;
    use pera_types::bridge::BRIDGE_COMMITTEE_MINIMAL_VOTING_POWER;
    use pera_types::bridge::{
        BridgeChainId, BRIDGE_CREATE_FUNCTION_NAME, BRIDGE_INIT_COMMITTEE_FUNCTION_NAME,
        BRIDGE_MODULE_NAME,
    };
    use pera_types::clock::{CLOCK_MODULE_NAME, CONSENSUS_COMMIT_PROLOGUE_FUNCTION_NAME};
    use pera_types::committee::EpochId;
    use pera_types::deny_list_v1::{DENY_LIST_CREATE_FUNC, DENY_LIST_MODULE};
    use pera_types::digests::{
        get_mainnet_chain_identifier, get_testnet_chain_identifier, ChainIdentifier,
    };
    use pera_types::effects::TransactionEffects;
    use pera_types::error::{ExecutionError, ExecutionErrorKind};
    use pera_types::execution::is_certificate_denied;
    use pera_types::execution_config_utils::to_binary_config;
    use pera_types::execution_status::{CongestedObjects, ExecutionStatus};
    use pera_types::gas::GasCostSummary;
    use pera_types::gas::PeraGasStatus;
    use pera_types::id::UID;
    use pera_types::inner_temporary_store::InnerTemporaryStore;
    use pera_types::messages_dwallet_mpc::{
        DWalletMPCOutput, MPCProtocolInitData, SingleSignSessionData,
    };
    #[cfg(msim)]
    use pera_types::pera_system_state::advance_epoch_result_injection::maybe_modify_result;
    use pera_types::pera_system_state::{
        AdvanceEpochParams, ADVANCE_EPOCH_SAFE_MODE_FUNCTION_NAME,
    };
    use pera_types::storage::BackingStore;
    use pera_types::transaction::{
        Argument, AuthenticatorStateExpire, AuthenticatorStateUpdate, CallArg, ChangeEpoch,
        Command, EndOfEpochTransactionKind, GenesisTransaction, ObjectArg, ProgrammableTransaction,
        TransactionKind,
    };
    use pera_types::transaction::{CheckedInputObjects, RandomnessStateUpdate};
    use pera_types::{
        base_types::{ObjectID, ObjectRef, PeraAddress, TransactionDigest, TxContext},
        object::{Object, ObjectInner},
        pera_system_state::{ADVANCE_EPOCH_FUNCTION_NAME, PERA_SYSTEM_MODULE_NAME},
        PERA_AUTHENTICATOR_STATE_OBJECT_ID, PERA_FRAMEWORK_ADDRESS, PERA_FRAMEWORK_PACKAGE_ID,
        PERA_SYSTEM_PACKAGE_ID,
    };

    #[instrument(name = "tx_execute_to_effects", level = "debug", skip_all)]
    pub fn execute_transaction_to_effects<Mode: ExecutionMode>(
        store: &dyn BackingStore,
        input_objects: CheckedInputObjects,
        gas_coins: Vec<ObjectRef>,
        gas_status: PeraGasStatus,
        transaction_kind: TransactionKind,
        transaction_signer: PeraAddress,
        transaction_digest: TransactionDigest,
        move_vm: &Arc<MoveVM>,
        epoch_id: &EpochId,
        epoch_timestamp_ms: u64,
        protocol_config: &ProtocolConfig,
        metrics: Arc<LimitsMetrics>,
        enable_expensive_checks: bool,
        certificate_deny_set: &HashSet<TransactionDigest>,
    ) -> (
        InnerTemporaryStore,
        PeraGasStatus,
        TransactionEffects,
        Result<Mode::ExecutionResults, ExecutionError>,
    ) {
        let input_objects = input_objects.into_inner();
        let mutable_inputs = if enable_expensive_checks {
            input_objects.mutable_inputs().keys().copied().collect()
        } else {
            HashSet::new()
        };
        let shared_object_refs = input_objects.filter_shared_objects();
        let receiving_objects = transaction_kind.receiving_objects();
        let mut transaction_dependencies = input_objects.transaction_dependencies();
        let contains_deleted_input = input_objects.contains_deleted_objects();
        let cancelled_objects = input_objects.get_cancelled_objects();

        let mut temporary_store = TemporaryStore::new(
            store,
            input_objects,
            receiving_objects,
            transaction_digest,
            protocol_config,
            *epoch_id,
        );

        let mut gas_charger =
            GasCharger::new(transaction_digest, gas_coins, gas_status, protocol_config);

        let mut tx_ctx = TxContext::new_from_components(
            &transaction_signer,
            &transaction_digest,
            epoch_id,
            epoch_timestamp_ms,
        );

        let is_epoch_change = transaction_kind.is_end_of_epoch_tx();

        let deny_cert = is_certificate_denied(&transaction_digest, certificate_deny_set);

        let (gas_cost_summary, execution_result) = execute_transaction::<Mode>(
            &mut temporary_store,
            transaction_kind,
            &mut gas_charger,
            &mut tx_ctx,
            move_vm,
            protocol_config,
            metrics,
            enable_expensive_checks,
            deny_cert,
            contains_deleted_input,
            cancelled_objects,
        );

        let status = if let Err(error) = &execution_result {
            // Elaborate errors in logs if they are unexpected or their status is terse.
            use ExecutionErrorKind as K;
            match error.kind() {
                K::InvariantViolation | K::VMInvariantViolation => {
                    #[skip_checked_arithmetic]
                    tracing::error!(
                        kind = ?error.kind(),
                        tx_digest = ?transaction_digest,
                        "INVARIANT VIOLATION! Source: {:?}",
                        error.source(),
                    );
                }

                K::PeraMoveVerificationError | K::VMVerificationOrDeserializationError => {
                    #[skip_checked_arithmetic]
                    tracing::debug!(
                        kind = ?error.kind(),
                        tx_digest = ?transaction_digest,
                        "Verification Error. Source: {:?}",
                        error.source(),
                    );
                }

                K::PublishUpgradeMissingDependency | K::PublishUpgradeDependencyDowngrade => {
                    #[skip_checked_arithmetic]
                    tracing::debug!(
                        kind = ?error.kind(),
                        tx_digest = ?transaction_digest,
                        "Publish/Upgrade Error. Source: {:?}",
                        error.source(),
                    )
                }

                _ => (),
            };

            let (status, command) = error.to_execution_status();
            ExecutionStatus::new_failure(status, command)
        } else {
            ExecutionStatus::Success
        };

        #[skip_checked_arithmetic]
        trace!(
            tx_digest = ?transaction_digest,
            computation_gas_cost = gas_cost_summary.computation_cost,
            storage_gas_cost = gas_cost_summary.storage_cost,
            storage_gas_rebate = gas_cost_summary.storage_rebate,
            "Finished execution of transaction with status {:?}",
            status
        );

        // Genesis writes a special digest to indicate that an object was created during
        // genesis and not written by any normal transaction - remove that from the
        // dependencies
        transaction_dependencies.remove(&TransactionDigest::genesis_marker());

        if enable_expensive_checks && !Mode::allow_arbitrary_function_calls() {
            temporary_store
                .check_ownership_invariants(
                    &transaction_signer,
                    &mut gas_charger,
                    &mutable_inputs,
                    is_epoch_change,
                )
                .unwrap()
        } // else, in dev inspect mode and anything goes--don't check

        let (inner, effects) = temporary_store.into_effects(
            shared_object_refs,
            &transaction_digest,
            transaction_dependencies,
            gas_cost_summary,
            status,
            &mut gas_charger,
            *epoch_id,
        );

        (
            inner,
            gas_charger.into_gas_status(),
            effects,
            execution_result,
        )
    }

    pub fn execute_genesis_state_update(
        store: &dyn BackingStore,
        protocol_config: &ProtocolConfig,
        metrics: Arc<LimitsMetrics>,
        move_vm: &Arc<MoveVM>,
        tx_context: &mut TxContext,
        input_objects: CheckedInputObjects,
        pt: ProgrammableTransaction,
    ) -> Result<InnerTemporaryStore, ExecutionError> {
        let input_objects = input_objects.into_inner();
        let mut temporary_store = TemporaryStore::new(
            store,
            input_objects,
            vec![],
            tx_context.digest(),
            protocol_config,
            0,
        );
        let mut gas_charger = GasCharger::new_unmetered(tx_context.digest());
        let _ = programmable_transactions::execution::execute::<execution_mode::Genesis>(
            protocol_config,
            metrics,
            move_vm,
            &mut temporary_store,
            tx_context,
            &mut gas_charger,
            pt,
        );
        temporary_store.update_object_version_and_prev_tx();
        Ok(temporary_store.into_inner())
    }

    #[instrument(name = "tx_execute", level = "debug", skip_all)]
    fn execute_transaction<Mode: ExecutionMode>(
        temporary_store: &mut TemporaryStore<'_>,
        transaction_kind: TransactionKind,
        gas_charger: &mut GasCharger,
        tx_ctx: &mut TxContext,
        move_vm: &Arc<MoveVM>,
        protocol_config: &ProtocolConfig,
        metrics: Arc<LimitsMetrics>,
        enable_expensive_checks: bool,
        deny_cert: bool,
        contains_deleted_input: bool,
        cancelled_objects: Option<(Vec<ObjectID>, SequenceNumber)>,
    ) -> (
        GasCostSummary,
        Result<Mode::ExecutionResults, ExecutionError>,
    ) {
        gas_charger.smash_gas(temporary_store);

        // At this point no charges have been applied yet
        debug_assert!(
            gas_charger.no_charges(),
            "No gas charges must be applied yet"
        );

        let is_genesis_tx = matches!(transaction_kind, TransactionKind::Genesis(_));
        let advance_epoch_gas_summary = transaction_kind.get_advance_epoch_tx_gas_summary();

        // We must charge object read here during transaction execution, because if this fails
        // we must still ensure an effect is committed and all objects versions incremented

        let result = gas_charger.charge_input_objects(temporary_store);
        let mut result = result.and_then(|()| {
            let mut execution_result = if deny_cert {
                Err(ExecutionError::new(
                    ExecutionErrorKind::CertificateDenied,
                    None,
                ))
            } else if contains_deleted_input {
                Err(ExecutionError::new(
                    ExecutionErrorKind::InputObjectDeleted,
                    None,
                ))
            } else if let Some((cancelled_objects, reason)) = cancelled_objects {
                match reason {
                    SequenceNumber::CONGESTED => Err(ExecutionError::new(
                        ExecutionErrorKind::ExecutionCancelledDueToSharedObjectCongestion {
                            congested_objects: CongestedObjects(cancelled_objects),
                        },
                        None,
                    )),
                    SequenceNumber::RANDOMNESS_UNAVAILABLE => Err(ExecutionError::new(
                        ExecutionErrorKind::ExecutionCancelledDueToRandomnessUnavailable,
                        None,
                    )),
                    _ => panic!("invalid cancellation reason SequenceNumber: {reason}"),
                }
            } else {
                execution_loop::<Mode>(
                    temporary_store,
                    transaction_kind,
                    tx_ctx,
                    move_vm,
                    gas_charger,
                    protocol_config,
                    metrics.clone(),
                )
            };

            let meter_check = check_meter_limit(
                temporary_store,
                gas_charger,
                protocol_config,
                metrics.clone(),
            );
            if let Err(e) = meter_check {
                execution_result = Err(e);
            }

            if execution_result.is_ok() {
                let gas_check = check_written_objects_limit::<Mode>(
                    temporary_store,
                    gas_charger,
                    protocol_config,
                    metrics,
                );
                if let Err(e) = gas_check {
                    execution_result = Err(e);
                }
            }

            execution_result
        });

        let cost_summary = gas_charger.charge_gas(temporary_store, &mut result);
        // For advance epoch transaction, we need to provide epoch rewards and rebates as extra
        // information provided to check_pera_conserved, because we mint rewards, and burn
        // the rebates. We also need to pass in the unmetered_storage_rebate because storage
        // rebate is not reflected in the storage_rebate of gas summary. This is a bit confusing.
        // We could probably clean up the code a bit.
        // Put all the storage rebate accumulated in the system transaction
        // to the 0x5 object so that it's not lost.
        temporary_store.conserve_unmetered_storage_rebate(gas_charger.unmetered_storage_rebate());

        if let Err(e) = run_conservation_checks::<Mode>(
            temporary_store,
            gas_charger,
            tx_ctx,
            move_vm,
            protocol_config.simple_conservation_checks(),
            enable_expensive_checks,
            &cost_summary,
            is_genesis_tx,
            advance_epoch_gas_summary,
        ) {
            // FIXME: we cannot fail the transaction if this is an epoch change transaction.
            result = Err(e);
        }

        (cost_summary, result)
    }

    #[instrument(name = "run_conservation_checks", level = "debug", skip_all)]
    fn run_conservation_checks<Mode: ExecutionMode>(
        temporary_store: &mut TemporaryStore<'_>,
        gas_charger: &mut GasCharger,
        tx_ctx: &mut TxContext,
        move_vm: &Arc<MoveVM>,
        simple_conservation_checks: bool,
        enable_expensive_checks: bool,
        cost_summary: &GasCostSummary,
        is_genesis_tx: bool,
        advance_epoch_gas_summary: Option<(u64, u64)>,
    ) -> Result<(), ExecutionError> {
        let mut result: std::result::Result<(), pera_types::error::ExecutionError> = Ok(());
        if !is_genesis_tx && !Mode::skip_conservation_checks() {
            // ensure that this transaction did not create or destroy PERA, try to recover if the check fails
            let conservation_result = {
                temporary_store
                    .check_pera_conserved(simple_conservation_checks, cost_summary)
                    .and_then(|()| {
                        if enable_expensive_checks {
                            // ensure that this transaction did not create or destroy PERA, try to recover if the check fails
                            let mut layout_resolver =
                                TypeLayoutResolver::new(move_vm, Box::new(&*temporary_store));
                            temporary_store.check_pera_conserved_expensive(
                                cost_summary,
                                advance_epoch_gas_summary,
                                &mut layout_resolver,
                            )
                        } else {
                            Ok(())
                        }
                    })
            };
            if let Err(conservation_err) = conservation_result {
                // conservation violated. try to avoid panic by dumping all writes, charging for gas, re-checking
                // conservation, and surfacing an aborted transaction with an invariant violation if all of that works
                result = Err(conservation_err);
                gas_charger.reset(temporary_store);
                gas_charger.charge_gas(temporary_store, &mut result);
                // check conservation once more more
                if let Err(recovery_err) = {
                    temporary_store
                        .check_pera_conserved(simple_conservation_checks, cost_summary)
                        .and_then(|()| {
                            if enable_expensive_checks {
                                // ensure that this transaction did not create or destroy PERA, try to recover if the check fails
                                let mut layout_resolver =
                                    TypeLayoutResolver::new(move_vm, Box::new(&*temporary_store));
                                temporary_store.check_pera_conserved_expensive(
                                    cost_summary,
                                    advance_epoch_gas_summary,
                                    &mut layout_resolver,
                                )
                            } else {
                                Ok(())
                            }
                        })
                } {
                    // if we still fail, it's a problem with gas
                    // charging that happens even in the "aborted" case--no other option but panic.
                    // we will create or destroy PERA otherwise
                    panic!(
                        "PERA conservation fail in tx block {}: {}\nGas status is {}\nTx was ",
                        tx_ctx.digest(),
                        recovery_err,
                        gas_charger.summary()
                    )
                }
            }
        } // else, we're in the genesis transaction which mints the PERA supply, and hence does not satisfy PERA conservation, or
          // we're in the non-production dev inspect mode which allows us to violate conservation
        result
    }

    #[instrument(name = "check_meter_limit", level = "debug", skip_all)]
    fn check_meter_limit(
        temporary_store: &mut TemporaryStore<'_>,
        gas_charger: &mut GasCharger,
        protocol_config: &ProtocolConfig,
        metrics: Arc<LimitsMetrics>,
    ) -> Result<(), ExecutionError> {
        let effects_estimated_size = temporary_store.estimate_effects_size_upperbound();

        // Check if a limit threshold was crossed.
        // For metered transactions, there is not soft limit.
        // For system transactions, we allow a soft limit with alerting, and a hard limit where we terminate
        match check_limit_by_meter!(
            !gas_charger.is_unmetered(),
            effects_estimated_size,
            protocol_config.max_serialized_tx_effects_size_bytes(),
            protocol_config.max_serialized_tx_effects_size_bytes_system_tx(),
            metrics.excessive_estimated_effects_size
        ) {
            LimitThresholdCrossed::None => Ok(()),
            LimitThresholdCrossed::Soft(_, limit) => {
                warn!(
                    effects_estimated_size = effects_estimated_size,
                    soft_limit = limit,
                    "Estimated transaction effects size crossed soft limit",
                );
                Ok(())
            }
            LimitThresholdCrossed::Hard(_, lim) => Err(ExecutionError::new_with_source(
                ExecutionErrorKind::EffectsTooLarge {
                    current_size: effects_estimated_size as u64,
                    max_size: lim as u64,
                },
                "Transaction effects are too large",
            )),
        }
    }

    #[instrument(name = "check_written_objects_limit", level = "debug", skip_all)]
    fn check_written_objects_limit<Mode: ExecutionMode>(
        temporary_store: &mut TemporaryStore<'_>,
        gas_charger: &mut GasCharger,
        protocol_config: &ProtocolConfig,
        metrics: Arc<LimitsMetrics>,
    ) -> Result<(), ExecutionError> {
        if let (Some(normal_lim), Some(system_lim)) = (
            protocol_config.max_size_written_objects_as_option(),
            protocol_config.max_size_written_objects_system_tx_as_option(),
        ) {
            let written_objects_size = temporary_store.written_objects_size();

            match check_limit_by_meter!(
                !gas_charger.is_unmetered(),
                written_objects_size,
                normal_lim,
                system_lim,
                metrics.excessive_written_objects_size
            ) {
                LimitThresholdCrossed::None => (),
                LimitThresholdCrossed::Soft(_, limit) => {
                    warn!(
                        written_objects_size = written_objects_size,
                        soft_limit = limit,
                        "Written objects size crossed soft limit",
                    )
                }
                LimitThresholdCrossed::Hard(_, lim) => {
                    return Err(ExecutionError::new_with_source(
                        ExecutionErrorKind::WrittenObjectsTooLarge {
                            current_size: written_objects_size as u64,
                            max_size: lim as u64,
                        },
                        "Written objects size crossed hard limit",
                    ))
                }
            };
        }

        Ok(())
    }

    #[instrument(level = "debug", skip_all)]
    fn execution_loop<Mode: ExecutionMode>(
        temporary_store: &mut TemporaryStore<'_>,
        transaction_kind: TransactionKind,
        tx_ctx: &mut TxContext,
        move_vm: &Arc<MoveVM>,
        gas_charger: &mut GasCharger,
        protocol_config: &ProtocolConfig,
        metrics: Arc<LimitsMetrics>,
    ) -> Result<Mode::ExecutionResults, ExecutionError> {
        let result = match transaction_kind {
            TransactionKind::ChangeEpoch(change_epoch) => {
                let builder = ProgrammableTransactionBuilder::new();
                advance_epoch(
                    builder,
                    change_epoch,
                    temporary_store,
                    tx_ctx,
                    move_vm,
                    gas_charger,
                    protocol_config,
                    metrics,
                )?;
                Ok(Mode::empty_results())
            }
            TransactionKind::Genesis(GenesisTransaction { objects }) => {
                if tx_ctx.epoch() != 0 {
                    panic!("BUG: Genesis Transactions can only be executed in epoch 0");
                }

                for genesis_object in objects {
                    match genesis_object {
                        pera_types::transaction::GenesisObject::RawObject { data, owner } => {
                            let object = ObjectInner {
                                data,
                                owner,
                                previous_transaction: tx_ctx.digest(),
                                storage_rebate: 0,
                            };
                            temporary_store.create_object(object.into());
                        }
                    }
                }
                Ok(Mode::empty_results())
            }
            TransactionKind::ConsensusCommitPrologue(prologue) => {
                setup_consensus_commit(
                    prologue.commit_timestamp_ms,
                    temporary_store,
                    tx_ctx,
                    move_vm,
                    gas_charger,
                    protocol_config,
                    metrics,
                )
                .expect("ConsensusCommitPrologue cannot fail");
                Ok(Mode::empty_results())
            }
            TransactionKind::ConsensusCommitPrologueV2(prologue) => {
                setup_consensus_commit(
                    prologue.commit_timestamp_ms,
                    temporary_store,
                    tx_ctx,
                    move_vm,
                    gas_charger,
                    protocol_config,
                    metrics,
                )
                .expect("ConsensusCommitPrologueV2 cannot fail");
                Ok(Mode::empty_results())
            }
            TransactionKind::ConsensusCommitPrologueV3(prologue) => {
                setup_consensus_commit(
                    prologue.commit_timestamp_ms,
                    temporary_store,
                    tx_ctx,
                    move_vm,
                    gas_charger,
                    protocol_config,
                    metrics,
                )
                .expect("ConsensusCommitPrologueV3 cannot fail");
                Ok(Mode::empty_results())
            }
            TransactionKind::ProgrammableTransaction(pt) => {
                programmable_transactions::execution::execute::<Mode>(
                    protocol_config,
                    metrics,
                    move_vm,
                    temporary_store,
                    tx_ctx,
                    gas_charger,
                    pt,
                )
            }
            TransactionKind::EndOfEpochTransaction(txns) => {
                let mut builder = ProgrammableTransactionBuilder::new();
                let len = txns.len();
                for (i, tx) in txns.into_iter().enumerate() {
                    match tx {
                        EndOfEpochTransactionKind::ChangeEpoch(change_epoch) => {
                            assert_eq!(i, len - 1);
                            advance_epoch(
                                builder,
                                change_epoch,
                                temporary_store,
                                tx_ctx,
                                move_vm,
                                gas_charger,
                                protocol_config,
                                metrics,
                            )?;
                            return Ok(Mode::empty_results());
                        }
                        EndOfEpochTransactionKind::AuthenticatorStateCreate => {
                            assert!(protocol_config.enable_jwk_consensus_updates());
                            builder = setup_authenticator_state_create(builder);
                        }
                        EndOfEpochTransactionKind::AuthenticatorStateExpire(expire) => {
                            assert!(protocol_config.enable_jwk_consensus_updates());

                            // TODO: it would be nice if a failure of this function didn't cause
                            // safe mode.
                            builder = setup_authenticator_state_expire(builder, expire);
                        }
                        EndOfEpochTransactionKind::RandomnessStateCreate => {
                            assert!(protocol_config.random_beacon());
                            builder = setup_randomness_state_create(builder);
                        }
                        EndOfEpochTransactionKind::DenyListStateCreate => {
                            assert!(protocol_config.enable_coin_deny_list_v1());
                            builder = setup_coin_deny_list_state_create(builder);
                        }
                        EndOfEpochTransactionKind::BridgeStateCreate(chain_id) => {
                            assert!(protocol_config.enable_bridge());
                            builder = setup_bridge_create(builder, chain_id)
                        }
                        EndOfEpochTransactionKind::BridgeCommitteeInit(bridge_shared_version) => {
                            assert!(protocol_config.enable_bridge());
                            assert!(protocol_config.should_try_to_finalize_bridge_committee());
                            builder = setup_bridge_committee_update(builder, bridge_shared_version)
                        }
                    }
                }
                unreachable!("EndOfEpochTransactionKind::ChangeEpoch should be the last transaction in the list")
            }
            TransactionKind::AuthenticatorStateUpdate(auth_state_update) => {
                setup_authenticator_state_update(
                    auth_state_update,
                    temporary_store,
                    tx_ctx,
                    move_vm,
                    gas_charger,
                    protocol_config,
                    metrics,
                )?;
                Ok(Mode::empty_results())
            }
            TransactionKind::RandomnessStateUpdate(randomness_state_update) => {
                setup_randomness_state_update(
                    randomness_state_update,
                    temporary_store,
                    tx_ctx,
                    move_vm,
                    gas_charger,
                    protocol_config,
                    metrics,
                )?;
                Ok(Mode::empty_results())
            }
            TransactionKind::DWalletMPCOutput(data) => {
                let res = setup_and_execute_dwallet_mpc_output(
                    data,
                    temporary_store,
                    tx_ctx,
                    move_vm,
                    gas_charger,
                    protocol_config,
                    metrics,
                );


                res?;

                Ok(Mode::empty_results())
            }
            TransactionKind::LockNextCommittee(..) => {
                setup_and_execute_lock_next_epoch_committee(
                    temporary_store,
                    tx_ctx,
                    move_vm,
                    gas_charger,
                    protocol_config,
                    metrics,
                )?;
                Ok(Mode::empty_results())
            }
        }?;
        temporary_store.check_execution_results_consistency()?;
        Ok(result)
    }

    fn mint_epoch_rewards_in_pt(
        builder: &mut ProgrammableTransactionBuilder,
        params: &AdvanceEpochParams,
    ) -> (Argument, Argument) {
        // Create storage rewards.
        let storage_charge_arg = builder
            .input(CallArg::Pure(
                bcs::to_bytes(&params.storage_charge).unwrap(),
            ))
            .unwrap();
        let storage_rewards = builder.programmable_move_call(
            PERA_FRAMEWORK_PACKAGE_ID,
            BALANCE_MODULE_NAME.to_owned(),
            BALANCE_CREATE_REWARDS_FUNCTION_NAME.to_owned(),
            vec![GAS::type_tag()],
            vec![storage_charge_arg],
        );

        // Create computation rewards.
        let computation_charge_arg = builder
            .input(CallArg::Pure(
                bcs::to_bytes(&params.computation_charge).unwrap(),
            ))
            .unwrap();
        let computation_rewards = builder.programmable_move_call(
            PERA_FRAMEWORK_PACKAGE_ID,
            BALANCE_MODULE_NAME.to_owned(),
            BALANCE_CREATE_REWARDS_FUNCTION_NAME.to_owned(),
            vec![GAS::type_tag()],
            vec![computation_charge_arg],
        );
        (storage_rewards, computation_rewards)
    }

    pub fn construct_advance_epoch_pt(
        mut builder: ProgrammableTransactionBuilder,
        params: &AdvanceEpochParams,
    ) -> Result<ProgrammableTransaction, ExecutionError> {
        // Step 1: Create storage and computation rewards.
        let (storage_rewards, computation_rewards) = mint_epoch_rewards_in_pt(&mut builder, params);

        // Step 2: Advance the epoch.
        let mut arguments = vec![storage_rewards, computation_rewards];
        let call_arg_arguments = vec![
            CallArg::PERA_SYSTEM_MUT,
            CallArg::Pure(bcs::to_bytes(&params.epoch).unwrap()),
            CallArg::Pure(bcs::to_bytes(&params.next_protocol_version.as_u64()).unwrap()),
            CallArg::Pure(bcs::to_bytes(&params.storage_rebate).unwrap()),
            CallArg::Pure(bcs::to_bytes(&params.non_refundable_storage_fee).unwrap()),
            CallArg::Pure(bcs::to_bytes(&params.storage_fund_reinvest_rate).unwrap()),
            CallArg::Pure(bcs::to_bytes(&params.reward_slashing_rate).unwrap()),
            CallArg::Pure(bcs::to_bytes(&params.epoch_start_timestamp_ms).unwrap()),
        ]
        .into_iter()
        .map(|a| builder.input(a))
        .collect::<Result<_, _>>();

        assert_invariant!(
            call_arg_arguments.is_ok(),
            "Unable to generate args for advance_epoch transaction!"
        );

        arguments.append(&mut call_arg_arguments.unwrap());

        info!("Call arguments to advance_epoch transaction: {:?}", params);

        let storage_rebates = builder.programmable_move_call(
            PERA_SYSTEM_PACKAGE_ID,
            PERA_SYSTEM_MODULE_NAME.to_owned(),
            ADVANCE_EPOCH_FUNCTION_NAME.to_owned(),
            vec![],
            arguments,
        );

        // Step 3: Destroy the storage rebates.
        builder.programmable_move_call(
            PERA_FRAMEWORK_PACKAGE_ID,
            BALANCE_MODULE_NAME.to_owned(),
            BALANCE_DESTROY_REBATES_FUNCTION_NAME.to_owned(),
            vec![GAS::type_tag()],
            vec![storage_rebates],
        );
        Ok(builder.finish())
    }

    pub fn construct_advance_epoch_safe_mode_pt(
        params: &AdvanceEpochParams,
        protocol_config: &ProtocolConfig,
    ) -> Result<ProgrammableTransaction, ExecutionError> {
        let mut builder = ProgrammableTransactionBuilder::new();
        // Step 1: Create storage and computation rewards.
        let (storage_rewards, computation_rewards) = mint_epoch_rewards_in_pt(&mut builder, params);

        // Step 2: Advance the epoch.
        let mut arguments = vec![storage_rewards, computation_rewards];

        let mut args = vec![
            CallArg::PERA_SYSTEM_MUT,
            CallArg::Pure(bcs::to_bytes(&params.epoch).unwrap()),
            CallArg::Pure(bcs::to_bytes(&params.next_protocol_version.as_u64()).unwrap()),
            CallArg::Pure(bcs::to_bytes(&params.storage_rebate).unwrap()),
            CallArg::Pure(bcs::to_bytes(&params.non_refundable_storage_fee).unwrap()),
        ];

        if protocol_config.get_advance_epoch_start_time_in_safe_mode() {
            args.push(CallArg::Pure(
                bcs::to_bytes(&params.epoch_start_timestamp_ms).unwrap(),
            ));
        }

        let call_arg_arguments = args
            .into_iter()
            .map(|a| builder.input(a))
            .collect::<Result<_, _>>();

        assert_invariant!(
            call_arg_arguments.is_ok(),
            "Unable to generate args for advance_epoch transaction!"
        );

        arguments.append(&mut call_arg_arguments.unwrap());

        info!("Call arguments to advance_epoch transaction: {:?}", params);

        builder.programmable_move_call(
            PERA_SYSTEM_PACKAGE_ID,
            PERA_SYSTEM_MODULE_NAME.to_owned(),
            ADVANCE_EPOCH_SAFE_MODE_FUNCTION_NAME.to_owned(),
            vec![],
            arguments,
        );

        Ok(builder.finish())
    }

    fn advance_epoch(
        builder: ProgrammableTransactionBuilder,
        change_epoch: ChangeEpoch,
        temporary_store: &mut TemporaryStore<'_>,
        tx_ctx: &mut TxContext,
        move_vm: &Arc<MoveVM>,
        gas_charger: &mut GasCharger,
        protocol_config: &ProtocolConfig,
        metrics: Arc<LimitsMetrics>,
    ) -> Result<(), ExecutionError> {
        let params = AdvanceEpochParams {
            epoch: change_epoch.epoch,
            next_protocol_version: change_epoch.protocol_version,
            storage_charge: change_epoch.storage_charge,
            computation_charge: change_epoch.computation_charge,
            storage_rebate: change_epoch.storage_rebate,
            non_refundable_storage_fee: change_epoch.non_refundable_storage_fee,
            storage_fund_reinvest_rate: protocol_config.storage_fund_reinvest_rate(),
            reward_slashing_rate: protocol_config.reward_slashing_rate(),
            epoch_start_timestamp_ms: change_epoch.epoch_start_timestamp_ms,
        };
        let advance_epoch_pt = construct_advance_epoch_pt(builder, &params)?;
        let result = programmable_transactions::execution::execute::<execution_mode::System>(
            protocol_config,
            metrics.clone(),
            move_vm,
            temporary_store,
            tx_ctx,
            gas_charger,
            advance_epoch_pt,
        );

        #[cfg(msim)]
        let result = maybe_modify_result(result, change_epoch.epoch);

        if result.is_err() {
            tracing::error!(
            "Failed to execute advance epoch transaction. Switching to safe mode. Error: {:?}. Input objects: {:?}. Tx data: {:?}",
            result.as_ref().err(),
            temporary_store.objects(),
            change_epoch,
        );
            temporary_store.drop_writes();
            // Must reset the storage rebate since we are re-executing.
            gas_charger.reset_storage_cost_and_rebate();

            if protocol_config.get_advance_epoch_start_time_in_safe_mode() {
                temporary_store.advance_epoch_safe_mode(&params, protocol_config);
            } else {
                let advance_epoch_safe_mode_pt =
                    construct_advance_epoch_safe_mode_pt(&params, protocol_config)?;
                programmable_transactions::execution::execute::<execution_mode::System>(
                    protocol_config,
                    metrics.clone(),
                    move_vm,
                    temporary_store,
                    tx_ctx,
                    gas_charger,
                    advance_epoch_safe_mode_pt,
                )
                .expect("Advance epoch with safe mode must succeed");
            }
        }

        if protocol_config.fresh_vm_on_framework_upgrade() {
            let new_vm = new_move_vm(
                all_natives(/* silent */ true, protocol_config),
                protocol_config,
                /* enable_profiler */ None,
            )
            .expect("Failed to create new MoveVM");
            process_system_packages(
                change_epoch,
                temporary_store,
                tx_ctx,
                &new_vm,
                gas_charger,
                protocol_config,
                metrics,
            );
        } else {
            process_system_packages(
                change_epoch,
                temporary_store,
                tx_ctx,
                move_vm,
                gas_charger,
                protocol_config,
                metrics,
            );
        }
        Ok(())
    }

    fn process_system_packages(
        change_epoch: ChangeEpoch,
        temporary_store: &mut TemporaryStore<'_>,
        tx_ctx: &mut TxContext,
        move_vm: &MoveVM,
        gas_charger: &mut GasCharger,
        protocol_config: &ProtocolConfig,
        metrics: Arc<LimitsMetrics>,
    ) {
        let binary_config = to_binary_config(protocol_config);
        for (version, modules, dependencies) in change_epoch.system_packages.into_iter() {
            let deserialized_modules: Vec<_> = modules
                .iter()
                .map(|m| CompiledModule::deserialize_with_config(m, &binary_config).unwrap())
                .collect();

            if version == OBJECT_START_VERSION {
                let package_id = deserialized_modules.first().unwrap().address();
                info!("adding new system package {package_id}");

                let publish_pt = {
                    let mut b = ProgrammableTransactionBuilder::new();
                    b.command(Command::Publish(modules, dependencies));
                    b.finish()
                };

                programmable_transactions::execution::execute::<execution_mode::System>(
                    protocol_config,
                    metrics.clone(),
                    move_vm,
                    temporary_store,
                    tx_ctx,
                    gas_charger,
                    publish_pt,
                )
                .expect("System Package Publish must succeed");
            } else {
                let mut new_package = Object::new_system_package(
                    &deserialized_modules,
                    version,
                    dependencies,
                    tx_ctx.digest(),
                );

                info!(
                    "upgraded system package {:?}",
                    new_package.compute_object_reference()
                );

                // Decrement the version before writing the package so that the store can record the
                // version growing by one in the effects.
                new_package
                    .data
                    .try_as_package_mut()
                    .unwrap()
                    .decrement_version();

                // upgrade of a previously existing framework module
                temporary_store.upgrade_system_package(new_package);
            }
        }
    }

    /// Perform metadata updates in preparation for the transactions in the upcoming checkpoint:
    ///
    /// - Set the timestamp for the `Clock` shared object from the timestamp in the header from
    ///   consensus.
    fn setup_consensus_commit(
        consensus_commit_timestamp_ms: CheckpointTimestamp,
        temporary_store: &mut TemporaryStore<'_>,
        tx_ctx: &mut TxContext,
        move_vm: &Arc<MoveVM>,
        gas_charger: &mut GasCharger,
        protocol_config: &ProtocolConfig,
        metrics: Arc<LimitsMetrics>,
    ) -> Result<(), ExecutionError> {
        let pt = {
            let mut builder = ProgrammableTransactionBuilder::new();
            let res = builder.move_call(
                PERA_FRAMEWORK_ADDRESS.into(),
                CLOCK_MODULE_NAME.to_owned(),
                CONSENSUS_COMMIT_PROLOGUE_FUNCTION_NAME.to_owned(),
                vec![],
                vec![
                    CallArg::CLOCK_MUT,
                    CallArg::Pure(bcs::to_bytes(&consensus_commit_timestamp_ms).unwrap()),
                ],
            );
            assert_invariant!(
                res.is_ok(),
                "Unable to generate consensus_commit_prologue transaction!"
            );
            builder.finish()
        };
        programmable_transactions::execution::execute::<execution_mode::System>(
            protocol_config,
            metrics,
            move_vm,
            temporary_store,
            tx_ctx,
            gas_charger,
            pt,
        )
    }

    fn setup_authenticator_state_create(
        mut builder: ProgrammableTransactionBuilder,
    ) -> ProgrammableTransactionBuilder {
        builder
            .move_call(
                PERA_FRAMEWORK_ADDRESS.into(),
                AUTHENTICATOR_STATE_MODULE_NAME.to_owned(),
                AUTHENTICATOR_STATE_CREATE_FUNCTION_NAME.to_owned(),
                vec![],
                vec![],
            )
            .expect("Unable to generate authenticator_state_create transaction!");
        builder
    }

    fn setup_randomness_state_create(
        mut builder: ProgrammableTransactionBuilder,
    ) -> ProgrammableTransactionBuilder {
        builder
            .move_call(
                PERA_FRAMEWORK_ADDRESS.into(),
                RANDOMNESS_MODULE_NAME.to_owned(),
                RANDOMNESS_STATE_CREATE_FUNCTION_NAME.to_owned(),
                vec![],
                vec![],
            )
            .expect("Unable to generate randomness_state_create transaction!");
        builder
    }

    /// Executes the system transaction to store the final MPC output on-chain,
    /// making it accessible to the initiating user.
    /// Each validator executes this transaction locally,
    /// and if validators represent more than two-thirds of the voting power,
    /// "vote" to include this transaction by executing it,
    /// the transaction is added to the checkpoint.
    fn setup_and_execute_dwallet_mpc_output(
        data: DWalletMPCOutput,
        temporary_store: &mut TemporaryStore<'_>,
        tx_ctx: &mut TxContext,
        move_vm: &Arc<MoveVM>,
        gas_charger: &mut GasCharger,
        protocol_config: &ProtocolConfig,
        metrics: Arc<LimitsMetrics>,
    ) -> Result<(), ExecutionError> {
        let mut module_name = DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME;

        let (move_function_name, args) = match data.session_info.mpc_round {
            MPCProtocolInitData::DKGFirst => (
                "create_dkg_first_round_output",
                vec![
                    CallArg::Pure(data.session_info.session_id.to_vec()),
                    CallArg::Pure(bcs_to_bytes(&data.output)?),
                    CallArg::Pure(data.session_info.initiating_user_address.to_vec()),
                ],
            ),
            MPCProtocolInitData::DKGSecond(event_data, dwallet_network_key_version) => (
                "create_dkg_second_round_output",
                vec![
                    CallArg::Pure(data.session_info.initiating_user_address.to_vec()),
                    CallArg::Pure(data.session_info.session_id.to_vec()),
                    // decentralized_public_output
                    CallArg::Pure(bcs_to_bytes(&data.output)?),
                    CallArg::Pure(event_data.dwallet_cap_id.bytes.to_vec()),
                    CallArg::Pure(bcs_to_bytes(&dwallet_network_key_version)?),
                    CallArg::Pure(bcs_to_bytes(
                        &event_data.encrypted_centralized_secret_share_and_proof,
                    )?),
                    CallArg::Pure(event_data.encryption_key_id.bytes.to_vec()),
                    CallArg::Pure(bcs_to_bytes(
                        &event_data.centralized_public_output_signature,
                    )?),
                    CallArg::Pure(bcs_to_bytes(&event_data.initiator_public_key)?),
                    CallArg::Pure(bcs_to_bytes(&event_data.centralized_public_output)?),
                ],
            ),
            MPCProtocolInitData::PresignFirst(
                dwallet_id,
                dkg_output,
                batch_session_id,
                network_key_version,
            ) => (
                "launch_presign_second_round",
                vec![
                    CallArg::Pure(data.session_info.initiating_user_address.to_vec()),
                    CallArg::Pure(bcs_to_bytes(&dwallet_id)?),
                    CallArg::Pure(bcs_to_bytes(&dkg_output)?),
                    CallArg::Pure(bcs_to_bytes(&data.output)?),
                    CallArg::Pure(data.session_info.session_id.to_vec()),
                    CallArg::Pure(batch_session_id.to_vec()),
                    CallArg::Pure(bcs_to_bytes(&network_key_version)?),
                ],
            ),
            MPCProtocolInitData::PresignSecond(
                dwallet_id,
                _first_round_output,
                batch_session_id,
            ) => {
                let presigns: Vec<(ObjectID, MPCPublicOutput)> = bcs::from_bytes(&data.output)
                    .map_err(|e| {
                        ExecutionError::new(
                            ExecutionErrorKind::DeserializationFailed,
                            Some(
                                format!("Failed to deserialize PresignSecond output: {}", e).into(),
                            ),
                        )
                    })?;
                let first_round_session_ids: Vec<ObjectID> =
                    presigns.iter().map(|(k, _)| *k).collect();
                let presigns: Vec<MPCPublicOutput> = presigns.into_iter().map(|(_, v)| v).collect();
                (
                    "create_batched_presign_output",
                    vec![
                        CallArg::Pure(data.session_info.initiating_user_address.to_vec()),
                        CallArg::Pure(batch_session_id.to_vec()),
                        CallArg::Pure(bcs_to_bytes(&first_round_session_ids)?),
                        CallArg::Pure(bcs_to_bytes(&presigns).map_err(|e| {
                            ExecutionError::new(
                                ExecutionErrorKind::SerializationFailed,
                                Some(format!("Failed to serialize values for batch: {}", e).into()),
                            )
                        })?),
                        CallArg::Pure(bcs_to_bytes(&dwallet_id)?),
                    ],
                )
            }
            MPCProtocolInitData::Sign(SingleSignSessionData {
                batch_session_id,
                dwallet_id,
                is_future_sign,
                ..
            }) => {
                module_name = DWALLET_MODULE_NAME;
                (
                    "create_sign_output",
                    vec![
                        // Serialized Vector of Signatures.
                        CallArg::Pure(data.output),
                        // The Batch Session ID.
                        CallArg::Pure(bcs_to_bytes(&batch_session_id)?),
                        CallArg::Pure(data.session_info.initiating_user_address.to_vec()),
                        CallArg::Pure(bcs_to_bytes(&dwallet_id)?),
                        CallArg::Pure(bcs_to_bytes(&is_future_sign)?),
                    ],
                )
            }
            MPCProtocolInitData::NetworkDkg(key_type, new_key) => {
                let new_key = new_key.ok_or(ExecutionError::new(
                    ExecutionErrorKind::TypeArgumentError {
                        argument_idx: 0,
                        kind: pera_types::execution_status::TypeArgumentError::TypeNotFound,
                    },
                    None,
                ))?;
                module_name = PERA_SYSTEM_MODULE_NAME;
                (
                    "new_decryption_key_shares_version",
                    vec![
                        CallArg::PERA_SYSTEM_MUT,
                        CallArg::Pure(bcs_to_bytes(
                            &new_key.current_epoch_encryptions_of_shares_per_crt_prime,
                        )?),
                        CallArg::Pure(bcs_to_bytes(&new_key.encryption_scheme_public_parameters)?),
                        CallArg::Pure(bcs_to_bytes(
                            &new_key.decryption_key_share_public_parameters,
                        )?),
                        CallArg::Pure(bcs_to_bytes(&new_key.encryption_key)?),
                        CallArg::Pure(bcs_to_bytes(&new_key.reconstructed_commitments_to_sharing)?),
                        CallArg::Pure(bcs_to_bytes(&(key_type as u8))?),
                    ],
                )
            }
            MPCProtocolInitData::EncryptedShareVerification(verification_data) => (
                "create_encrypted_user_share",
                vec![
                    CallArg::Pure(verification_data.dwallet_id.bytes.to_vec()),
                    CallArg::Pure(bcs_to_bytes(
                        &verification_data.encrypted_centralized_secret_share_and_proof,
                    )?),
                    CallArg::Pure(verification_data.encryption_key_id.bytes.to_vec()),
                    CallArg::Pure(data.session_info.session_id.to_vec()),
                    CallArg::Pure(bcs_to_bytes(
                        &verification_data.centralized_public_output_signature,
                    )?),
                    CallArg::Pure(bcs_to_bytes(&verification_data.encryptor_ed25519_pubkey)?),
                    CallArg::Pure(verification_data.initiator.to_vec()),
                ],
            ),
            MPCProtocolInitData::EncryptionKeyVerification(verification_data) => {
                module_name = DWALLET_MODULE_NAME;
                (
                    "create_encryption_key",
                    vec![
                        CallArg::Pure(bcs_to_bytes(&verification_data.encryption_key)?),
                        CallArg::Pure(bcs_to_bytes(&verification_data.encryption_key_signature)?),
                        CallArg::Pure(bcs_to_bytes(&verification_data.key_singer_public_key)?),
                        CallArg::Pure(bcs_to_bytes(&verification_data.encryption_key_scheme)?),
                        CallArg::Pure(verification_data.initiator.to_vec()),
                        CallArg::Pure(data.session_info.session_id.to_vec()),
                    ],
                )
            }
            _ => {
                unreachable!(
                    "MPCRound {:?} is not supported for creating an on chain output",
                    data.session_info.mpc_round
                )
            }
        };

        let pt = {
            let mut builder = ProgrammableTransactionBuilder::new();
            let res = builder.move_call(
                PERA_SYSTEM_PACKAGE_ID.into(),
                module_name.to_owned(),
                ident_str!(move_function_name).to_owned(),
                vec![],
                args,
            );
            assert_invariant!(res.is_ok(), "Unable to generate mpc transaction!");
            builder.finish()
        };
        programmable_transactions::execution::execute::<execution_mode::System>(
            protocol_config,
            metrics,
            move_vm,
            temporary_store,
            tx_ctx,
            gas_charger,
            pt,
        )
    }

    fn bcs_to_bytes<T>(obj: &T) -> Result<Vec<u8>, ExecutionError>
    where
        T: serde::Serialize,
    {
        bcs::to_bytes(obj).map_err(|e| {
            ExecutionError::new(
                ExecutionErrorKind::SerializationFailed,
                Some(format!("Failed to serialize object: {}", e).into()),
            )
        })
    }

    fn setup_and_execute_lock_next_epoch_committee(
        temporary_store: &mut TemporaryStore<'_>,
        tx_ctx: &mut TxContext,
        move_vm: &Arc<MoveVM>,
        gas_charger: &mut GasCharger,
        protocol_config: &ProtocolConfig,
        metrics: Arc<LimitsMetrics>,
    ) -> Result<(), ExecutionError> {
        let pt = {
            let mut builder = ProgrammableTransactionBuilder::new();
            let res = builder.move_call(
                PERA_SYSTEM_PACKAGE_ID.into(),
                PERA_SYSTEM_MODULE_NAME.to_owned(),
                ident_str!("lock_next_epoch_committee").to_owned(),
                vec![],
                vec![CallArg::PERA_SYSTEM_MUT],
            );
            assert_invariant!(res.is_ok(), "Unable to generate mpc transaction!");
            builder.finish()
        };
        programmable_transactions::execution::execute::<execution_mode::System>(
            protocol_config,
            metrics,
            move_vm,
            temporary_store,
            tx_ctx,
            gas_charger,
            pt,
        )
    }

    fn setup_bridge_create(
        mut builder: ProgrammableTransactionBuilder,
        chain_id: ChainIdentifier,
    ) -> ProgrammableTransactionBuilder {
        let bridge_uid = builder
            .input(CallArg::Pure(
                UID::new(PERA_BRIDGE_OBJECT_ID).to_bcs_bytes(),
            ))
            .expect("Unable to create Bridge object UID!");

        let bridge_chain_id = if chain_id == get_mainnet_chain_identifier() {
            BridgeChainId::PeraMainnet as u8
        } else if chain_id == get_testnet_chain_identifier() {
            BridgeChainId::PeraTestnet as u8
        } else {
            // How do we distinguish devnet from other test envs?
            BridgeChainId::PeraCustom as u8
        };

        let bridge_chain_id = builder.pure(bridge_chain_id).unwrap();
        builder.programmable_move_call(
            BRIDGE_ADDRESS.into(),
            BRIDGE_MODULE_NAME.to_owned(),
            BRIDGE_CREATE_FUNCTION_NAME.to_owned(),
            vec![],
            vec![bridge_uid, bridge_chain_id],
        );
        builder
    }

    fn setup_bridge_committee_update(
        mut builder: ProgrammableTransactionBuilder,
        bridge_shared_version: SequenceNumber,
    ) -> ProgrammableTransactionBuilder {
        let bridge = builder
            .obj(ObjectArg::SharedObject {
                id: PERA_BRIDGE_OBJECT_ID,
                initial_shared_version: bridge_shared_version,
                mutable: true,
            })
            .expect("Unable to create Bridge object arg!");
        let system_state = builder
            .obj(ObjectArg::PERA_SYSTEM_MUT)
            .expect("Unable to create System State object arg!");

        let voting_power = builder.programmable_move_call(
            PERA_SYSTEM_PACKAGE_ID,
            PERA_SYSTEM_MODULE_NAME.to_owned(),
            ident_str!("validator_voting_powers").to_owned(),
            vec![],
            vec![system_state],
        );

        // Hardcoding min stake participation to 75.00%
        // TODO: We need to set a correct value or make this configurable.
        let min_stake_participation_percentage = builder
            .input(CallArg::Pure(
                bcs::to_bytes(&BRIDGE_COMMITTEE_MINIMAL_VOTING_POWER).unwrap(),
            ))
            .unwrap();

        builder.programmable_move_call(
            BRIDGE_ADDRESS.into(),
            BRIDGE_MODULE_NAME.to_owned(),
            BRIDGE_INIT_COMMITTEE_FUNCTION_NAME.to_owned(),
            vec![],
            vec![bridge, voting_power, min_stake_participation_percentage],
        );
        builder
    }

    fn setup_authenticator_state_update(
        update: AuthenticatorStateUpdate,
        temporary_store: &mut TemporaryStore<'_>,
        tx_ctx: &mut TxContext,
        move_vm: &Arc<MoveVM>,
        gas_charger: &mut GasCharger,
        protocol_config: &ProtocolConfig,
        metrics: Arc<LimitsMetrics>,
    ) -> Result<(), ExecutionError> {
        let pt = {
            let mut builder = ProgrammableTransactionBuilder::new();
            let res = builder.move_call(
                PERA_FRAMEWORK_ADDRESS.into(),
                AUTHENTICATOR_STATE_MODULE_NAME.to_owned(),
                AUTHENTICATOR_STATE_UPDATE_FUNCTION_NAME.to_owned(),
                vec![],
                vec![
                    CallArg::Object(ObjectArg::SharedObject {
                        id: PERA_AUTHENTICATOR_STATE_OBJECT_ID,
                        initial_shared_version: update.authenticator_obj_initial_shared_version,
                        mutable: true,
                    }),
                    CallArg::Pure(bcs::to_bytes(&update.new_active_jwks).unwrap()),
                ],
            );
            assert_invariant!(
                res.is_ok(),
                "Unable to generate authenticator_state_update transaction!"
            );
            builder.finish()
        };
        programmable_transactions::execution::execute::<execution_mode::System>(
            protocol_config,
            metrics,
            move_vm,
            temporary_store,
            tx_ctx,
            gas_charger,
            pt,
        )
    }

    fn setup_authenticator_state_expire(
        mut builder: ProgrammableTransactionBuilder,
        expire: AuthenticatorStateExpire,
    ) -> ProgrammableTransactionBuilder {
        builder
            .move_call(
                PERA_FRAMEWORK_ADDRESS.into(),
                AUTHENTICATOR_STATE_MODULE_NAME.to_owned(),
                AUTHENTICATOR_STATE_EXPIRE_JWKS_FUNCTION_NAME.to_owned(),
                vec![],
                vec![
                    CallArg::Object(ObjectArg::SharedObject {
                        id: PERA_AUTHENTICATOR_STATE_OBJECT_ID,
                        initial_shared_version: expire.authenticator_obj_initial_shared_version,
                        mutable: true,
                    }),
                    CallArg::Pure(bcs::to_bytes(&expire.min_epoch).unwrap()),
                ],
            )
            .expect("Unable to generate authenticator_state_expire transaction!");
        builder
    }

    fn setup_randomness_state_update(
        update: RandomnessStateUpdate,
        temporary_store: &mut TemporaryStore<'_>,
        tx_ctx: &mut TxContext,
        move_vm: &Arc<MoveVM>,
        gas_charger: &mut GasCharger,
        protocol_config: &ProtocolConfig,
        metrics: Arc<LimitsMetrics>,
    ) -> Result<(), ExecutionError> {
        let pt = {
            let mut builder = ProgrammableTransactionBuilder::new();
            let res = builder.move_call(
                PERA_FRAMEWORK_ADDRESS.into(),
                RANDOMNESS_MODULE_NAME.to_owned(),
                RANDOMNESS_STATE_UPDATE_FUNCTION_NAME.to_owned(),
                vec![],
                vec![
                    CallArg::Object(ObjectArg::SharedObject {
                        id: PERA_RANDOMNESS_STATE_OBJECT_ID,
                        initial_shared_version: update.randomness_obj_initial_shared_version,
                        mutable: true,
                    }),
                    CallArg::Pure(bcs::to_bytes(&update.randomness_round).unwrap()),
                    CallArg::Pure(bcs::to_bytes(&update.random_bytes).unwrap()),
                ],
            );
            assert_invariant!(
                res.is_ok(),
                "Unable to generate randomness_state_update transaction!"
            );
            builder.finish()
        };
        programmable_transactions::execution::execute::<execution_mode::System>(
            protocol_config,
            metrics,
            move_vm,
            temporary_store,
            tx_ctx,
            gas_charger,
            pt,
        )
    }

    fn setup_coin_deny_list_state_create(
        mut builder: ProgrammableTransactionBuilder,
    ) -> ProgrammableTransactionBuilder {
        builder
            .move_call(
                PERA_FRAMEWORK_ADDRESS.into(),
                DENY_LIST_MODULE.to_owned(),
                DENY_LIST_CREATE_FUNC.to_owned(),
                vec![],
                vec![],
            )
            .expect("Unable to generate coin_deny_list_create transaction!");
        builder
    }
}
