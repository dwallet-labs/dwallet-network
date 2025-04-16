//! The orchestrator for dWallet MPC cryptographic computations.
//!
//! The orchestrator manages a task queue for cryptographic computations and
//! ensures efficient CPU resource utilization.
//! It tracks the number of available CPU cores and prevents launching
//! tasks when all cores are occupied.
//!
//! Key responsibilities:
//! — Manages a queue of pending cryptographic computations
//! — Tracks currently running sessions and available CPU cores
//! — Handles session spawning and completion notifications.
//! — Implements special handling for aggregated sign operations
//! — Ensures computations don't become redundant based on received messages
//!
//! The orchestrator uses a channel-based notification system to track computation status:
//! — Sends `Started` notifications when computations begin
//! — Sends `Completed` notifications when computations finish
//! — Updates the running sessions count accordingly
use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::dwallet_mpc::mpc_session::DWalletMPCSession;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use std::sync::Arc;
use tokio::runtime::Handle;
use tokio::sync::mpsc::UnboundedSender;
use tracing::error;

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
/// — Manages a queue of pending cryptographic computations
/// — Tracks currently running sessions and available CPU cores
/// — Handles session spawning and completion notifications
/// — Implements special handling for aggregated sign operations
/// — Ensures computations don't become redundant based on received messages
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
                    Some(update_value) => match update_value {
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

    pub(super) fn spawn_session(&mut self, session: &DWalletMPCSession) -> DwalletMPCResult<()> {
        let handle = Handle::current();
        let session = session.clone();
        if let Err(err) = self
            .computation_channel_sender
            .send(ComputationUpdate::Started)
        {
            error!(
                "Failed to send a started computation message with error: {:?}",
                err
            );
        }
        let computation_channel_sender = self.computation_channel_sender.clone();
        rayon::spawn_fifo(move || {
            if let Err(err) = session.advance(&handle) {
                error!("failed to advance session with error: {:?}", err);
            };
            if let Err(err) = computation_channel_sender.send(ComputationUpdate::Completed) {
                error!(
                    "Failed to send a finished computation message with error: {:?}",
                    err
                );
            }
        });
        Ok(())
    }
}
