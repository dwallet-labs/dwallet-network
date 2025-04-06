//! The orchestrator for dWallet MPC cryptographic computations.
//!
//! The orchestrator manages a task queue for cryptographic computations and 
//! ensures efficient CPU resource utilization.
//! It tracks the number of available CPU cores and prevents launching 
//! tasks when all cores are occupied.
//! 
//! Key responsibilities:
//! - Manages a queue of pending cryptographic computations
//! - Tracks currently running sessions and available CPU cores
//! - Handles session spawning and completion notifications
//! - Implements special handling for aggregated sign operations
//! - Ensures computations don't become redundant based on received messages
//!
//! The orchestrator uses a channel-based notification system to track computation status:
//! - Sends `Started` notifications when computations begin
//! - Sends `Completed` notifications when computations finish
//! - Updates the running session count accordingly
use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::dwallet_mpc::mpc_session::DWalletMPCSession;
use crate::dwallet_mpc::sign::SIGN_LAST_ROUND_COMPUTATION_CONSTANT_SECONDS;
use dwallet_mpc_types::dwallet_mpc::MPCSessionStatus;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_dwallet_mpc::DWalletMPCLocalComputationMetadata;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use sui_types::base_types::{ObjectID, TransactionDigest};
use tokio::runtime::Handle;
use tokio::sync::mpsc::UnboundedSender;
use tracing::{error, info, warn};

/// Represents the state transitions of cryptographic computations in the orchestrator.
///
/// This enum is used for communication between Tokio and Rayon tasks via a channel.
/// In the aggregated sign flow, Rayon tasks are spawned from within Tokio tasks,
/// requiring explicit lifecycle tracking.
pub(crate) enum ComputationUpdate {
    /// A new computation has started.
    Started,

    /// A computation has been completed.
    Completed,
}

/// The orchestrator for DWallet MPC cryptographic computations.
///
/// The orchestrator manages cryptographic computation tasks and ensures efficient
///  CPU resource utilization.
/// It tracks available CPU cores and prevents launching tasks when all cores are occupied.
///
/// Key responsibilities:
/// - Manages a queue of pending cryptographic computations
/// - Tracks currently running sessions and available CPU cores
/// - Handles session spawning and completion notifications
/// - Implements special handling for aggregated sign operations
/// - Ensures computations don't become redundant based on received messages
pub(crate) struct CryptographicComputationsOrchestrator {
    /// The number of logical CPUs available for cryptographic computations on the validator's
    /// machine. Used to limit parallel task execution.
    available_cores_for_cryptographic_computations: usize,
    
    /// A channel sender to notify the manager about computation lifecycle events.
    /// Used to track when computations start and complete, allowing proper resource management.
    computation_channel_sender: UnboundedSender<ComputationUpdate>,
    
    /// The number of currently running cryptographic computations.
    /// Tracks tasks that have been spawned with [`rayon::spawn_fifo`] but haven't completed yet.
    /// Used to prevent exceeding available CPU cores.
    currently_running_sessions_count: usize,
    
    /// Reference to the epoch store, used for accessing validator state and configuration.
    epoch_store: Arc<AuthorityPerEpochStore>,
}

impl CryptographicComputationsOrchestrator {
    /// Creates a new orchestrator for cryptographic computations.
    pub(crate) fn try_new(epoch_store: &Arc<AuthorityPerEpochStore>) -> DwalletMPCResult<Self> {
        let completed_computation_channel_sender =
            Self::listen_for_completed_computations(&epoch_store);
        let available_cores_for_computations: usize = std::thread::available_parallelism()
            .map_err(|e| DwalletMPCError::FailedToGetAvailableParallelism(e.to_string()))?
            .into();
        if !(available_cores_for_computations > 0) {
            return Err(DwalletMPCError::InsufficientCPUCores);
        }

        Ok(CryptographicComputationsOrchestrator {
            available_cores_for_cryptographic_computations: available_cores_for_computations,
            computation_channel_sender: completed_computation_channel_sender,
            currently_running_sessions_count: 0,
            epoch_store: epoch_store.clone(),
        })
    }

    fn listen_for_completed_computations(
        epoch_store: &Arc<AuthorityPerEpochStore>,
    ) -> UnboundedSender<ComputationUpdate> {
        let (completed_computation_channel_sender, mut completed_computation_channel_receiver) =
            tokio::sync::mpsc::unbounded_channel();
        let epoch_store_for_channel = epoch_store.clone();
        tokio::spawn(async move {
            loop {
                match completed_computation_channel_receiver.recv().await {
                    None => {
                        break;
                    }
                    Some(updateValue) => match updateValue {
                        ComputationUpdate::Started => {
                            epoch_store_for_channel
                                .get_dwallet_mpc_manager()
                                .await
                                .cryptographic_computations_orchestrator
                                .currently_running_sessions_count += 1;
                        }
                        ComputationUpdate::Completed => {
                            epoch_store_for_channel
                                .get_dwallet_mpc_manager()
                                .await
                                .cryptographic_computations_orchestrator
                                .currently_running_sessions_count -= 1;
                        }
                    },
                }
            }
        });
        completed_computation_channel_sender
    }

    /// Checks if a new session can be spawned based on available CPU cores.	
    pub(crate) fn can_spawn_session(&self) -> bool {
        self.currently_running_sessions_count < self.available_cores_for_cryptographic_computations
    }

    pub(crate) fn spawn_session(&mut self, session: &DWalletMPCSession) -> DwalletMPCResult<()> {
        Self::spawn_session_static(self.computation_channel_sender.clone(), session)
    }

    pub(crate) fn spawn_session_static(
        finished_computation_sender: UnboundedSender<ComputationUpdate>,
        session: &DWalletMPCSession,
    ) -> DwalletMPCResult<()> {
        // Hook the tokio thread pool to the rayon thread pool.
        let handle = Handle::current();
        let session = session.clone();
        if let Err(err) = finished_computation_sender.send(ComputationUpdate::Started) {
            error!(
                "Failed to send a started computation message with error: {:?}",
                err
            );
        }
        rayon::spawn_fifo(move || {
            if let Err(err) = session.advance(&handle) {
                error!("failed to advance session with error: {:?}", err);
            }
            if let Err(err) = finished_computation_sender.send(ComputationUpdate::Completed) {
                error!(
                    "Failed to send a finished computation message with error: {:?}",
                    err
                );
            }
        });
        Ok(())
    }

    /// Deterministically decides by the session ID how long this validator should wait before
    /// running the last step of the sign protocol.
    /// If while waiting, the validator receives a valid signature for this session,
    /// it will not run the last step in the sign protocol, and save computation resources.
    fn get_validator_position(&self, session_id: &ObjectID) -> DwalletMPCResult<usize> {
        let session_id_as_32_bytes: [u8; 32] = session_id.into_bytes();
        let positions = self
            .epoch_store
            .committee()
            .shuffle_by_stake_from_tx_digest(&TransactionDigest::new(session_id_as_32_bytes));
        let authority_name = self.epoch_store.name;
        let position = positions
            .iter()
            .position(|&x| x == authority_name)
            .ok_or(DwalletMPCError::InvalidMPCPartyType)?;
        Ok(position)
    }

    pub(crate) fn spawn_aggregated_sign(
        &mut self,
        session: DWalletMPCSession,
    ) -> DwalletMPCResult<()> {
        let validator_position = self.get_validator_position(&session.session_id)?;
        let epoch_store = self.epoch_store.clone();
        let sender = self.computation_channel_sender.clone();
        tokio::spawn(async move {
            if validator_position > 0 {
                for _ in 1..validator_position {
                    let manager = epoch_store.get_dwallet_mpc_manager().await;
                    let Some(session) = manager.mpc_sessions.get(&session.session_id) else {
                        error!(
                    "failed to get session when checking if sign last round should get executed"
                );
                        return;
                    };
                    // If a malicious report has been received for the sign session, all the validators
                    // should execute the last step immediately.
                    if !session.session_specific_state.is_none() {
                        break;
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs(
                        SIGN_LAST_ROUND_COMPUTATION_CONSTANT_SECONDS as u64,
                    ))
                    .await;
                }
            }
            let manager = epoch_store.get_dwallet_mpc_manager().await;
            let Some(live_session) = manager.mpc_sessions.get(&session.session_id) else {
                error!(
                    "failed to get session when checking if sign last round should get executed"
                );
                return;
            };
            if live_session.status != MPCSessionStatus::Active
                && !live_session.is_verifying_sign_ia_report()
            {
                return;
            }
            info!(
                "running last sign cryptographic step for session_id: {:?}",
                session.session_id
            );
            let session = session.clone();
            if let Err(e) = Self::spawn_session_static(sender, &session) {
                error!("failed to spawn session with error: {:?}", e);
            }
        });
        Ok(())
    }
}
