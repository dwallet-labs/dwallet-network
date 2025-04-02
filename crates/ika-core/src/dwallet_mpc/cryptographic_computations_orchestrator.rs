//! The orchestrator for dWallet MPC cryptographic computations.
//!
//! The orchestrator's job is to manage a task queue for computations
//! and avoid launching tasks that cannot be parallelized at the moment
//! due to unavailable CPUs.
//! When a CPU core is freed, and before launching the Rayon task,
//! it ensures that the computation has not become redundant based on
//! messages received since it was added to the queue.
//! This approach reduces the read delay from the local DB without slowing down state sync.
use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::dwallet_mpc::mpc_session::DWalletMPCSession;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_dwallet_mpc::DWalletMPCLocalComputationMetadata;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::runtime::Handle;
use tokio::sync::mpsc::UnboundedSender;
use tracing::error;

/// The possible MPC computations update.
/// Needed to use a channel also for start messages because in the aggregated sign flow,
/// the Rayon task is being spawned from within a Tokio task.
pub(crate) enum ComputationUpdate {
    /// A new computation has started.
    Started,

    /// A computation has been completed.
    Completed,
}

/// The orchestrator for DWallet MPC cryptographic computations.
///
/// The orchestrator's job is to manage a task queue for computations
/// and avoid launching tasks that cannot be parallelized at the moment
/// due to unavailable CPUs.
/// When a CPU core is freed, and before launching the Rayon task,
/// it ensures that the computation has not become redundant based on
/// messages received since it was added to the queue. This approach
/// reduces the read delay from the local DB without slowing down state sync.
pub(crate) struct CryptographicComputationsOrchestrator {
    /// The number of logical CPUs available for cryptographic computations on the validator's
    /// machine.
    pub(crate) available_cores_for_cryptographic_computations: usize,
    /// A channel sender to notify the manager that a computation has been completed.
    /// This is needed to decrease the [`currently_running_sessions_count`] when a computation is
    /// done.
    pub(crate) computation_channel_sender: UnboundedSender<ComputationUpdate>,
    /// The number of currently running cryptographic computations — i.e.,
    /// computations we called [`rayon::spawn_fifo`] for,
    /// but we didn't receive a completion message for.
    pub(crate) currently_running_sessions_count: usize,
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

    fn spawn_session(&mut self, session: &DWalletMPCSession) -> DwalletMPCResult<()> {
        let session_id = session.session_id;
        // Hook the tokio thread pool to the rayon thread pool.
        let handle = Handle::current();
        let session = session.clone();
        let finished_computation_sender = self.computation_channel_sender.clone();
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
}
