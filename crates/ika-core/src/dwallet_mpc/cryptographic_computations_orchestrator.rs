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
//! The orchestrator uses a channel-based notification system to track completed computation.
use crate::dwallet_mpc::dwallet_mpc_metrics::DWalletMPCMetrics;
use crate::dwallet_mpc::mpc_session::DWalletMPCSession;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_dwallet_mpc::SessionIdentifier;
use std::sync::Arc;
use std::time::Instant;
use tokio::runtime::Handle;
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::{error, info, warn};

/// Channel size for cryptographic computations state updates.
/// This channel should not reach a size even close to this.
/// But since this is critical to keep the computations running,
/// we are using a big buffer (this size of the data is small).
const COMPUTATION_UPDATE_CHANNEL_SIZE: usize = 10_000;

/// This struct is used for reporting completed cryptographic computation,
/// the report is sent via a channel for communication between Tokio and Rayon tasks.
/// In the aggregated sign flow, Rayon tasks are spawned from within Tokio tasks,
/// requiring explicit lifecycle tracking.
pub(crate) struct CompletedComputationReport {
    session_identifier: SessionIdentifier,
    round: usize,
    attempts_count: usize,
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

    /// A channel sender to notify the manager about completed computations,
    /// allowing proper resource management.
    completed_computation_sender: Sender<CompletedComputationReport>,
    completed_computation_receiver: Receiver<CompletedComputationReport>,

    /// The number of currently running cryptographic computations.
    /// Tracks tasks that have been spawned with [`rayon::spawn_fifo`] but haven't completed yet.
    /// Used to prevent exceeding available CPU cores.
    currently_running_sessions_count: usize,
}

impl CryptographicComputationsOrchestrator {
    /// Creates a new orchestrator for cryptographic computations.
    pub(crate) fn try_new() -> DwalletMPCResult<Self> {
        let (report_computation_completed_sender, report_computation_completed_receiver) =
            tokio::sync::mpsc::channel(COMPUTATION_UPDATE_CHANNEL_SIZE);
        let available_cores_for_computations: usize = std::thread::available_parallelism()
            .map_err(|e| DwalletMPCError::FailedToGetAvailableParallelism(e.to_string()))?
            .into();
        if available_cores_for_computations == 0 {
            error!(
                "failed to get available parallelism, no CPU cores available for cryptographic computations"
            );
            return Err(DwalletMPCError::InsufficientCPUCores);
        }
        // Note: Enable the feature to enforce a minimum of CPU cores.
        #[cfg(feature = "enforce-minimum-cpu")]
        {
            assert!(
                available_cores_for_computations >= 16,
                "Validator must have more at least 16 CPU cores for cryptographic computations"
            );
        }
        info!(
            available_cores_for_computations =? available_cores_for_computations,
            "Available CPU cores for Rayon cryptographic computations"
        );

        Ok(CryptographicComputationsOrchestrator {
            available_cores_for_cryptographic_computations: available_cores_for_computations,
            completed_computation_sender: report_computation_completed_sender,
            completed_computation_receiver: report_computation_completed_receiver,
            currently_running_sessions_count: 0,
        })
    }

    /// Check for completed computations, and sufficient CPU cores.
    pub(crate) fn has_available_cores_to_perform_computation(&mut self) -> bool {
        while let Ok(completed_session) = self.completed_computation_receiver.try_recv() {
            self.currently_running_sessions_count -= 1;
            info!(
                session_identifier=?completed_session.session_identifier,
                round=?completed_session.round,
                attempts_count=?completed_session.attempts_count,
                currently_running_sessions_count =? self.currently_running_sessions_count,
                "Completed cryptographic computation"
            );
        }

        self.currently_running_sessions_count < self.available_cores_for_cryptographic_computations
    }

    pub(super) async fn try_spawn_session(
        &mut self,
        session: &DWalletMPCSession,
        dwallet_mpc_metrics: Arc<DWalletMPCMetrics>,
    ) -> DwalletMPCResult<()> {
        if !self.has_available_cores_to_perform_computation() {
            warn!(
                session_id=?session.session_identifier,
                mpc_protocol=?session.mpc_event_data.as_ref().unwrap().request_input,
                "No available CPU cores to perform cryptographic computation"
            );
            return Err(DwalletMPCError::InsufficientCPUCores);
        }

        let handle = Handle::current();
        let mut session = session.clone();
        // Safe to unwrap here (event must exist before this).
        let request_input = session.mpc_event_data.clone().unwrap().request_input;

        dwallet_mpc_metrics.add_advance_call(&request_input, &session.current_round.to_string());

        let computation_channel_sender = self.completed_computation_sender.clone();
        rayon::spawn_fifo(move || {
            let advance_start_time = Instant::now();
            if let Err(err) = session.advance(&handle) {
                error!(
                    error=?err,
                    mpc_protocol=?request_input,
                    session_id=?session.session_identifier,
                    "failed to advance an MPC session"
                );
            } else {
                let elapsed = advance_start_time.elapsed();
                let elapsed_ms = elapsed.as_millis();
                info!(
                    mpc_protocol=?request_input,
                    session_id=?session.session_identifier,
                    duration_ms = elapsed_ms,
                    duration_seconds = elapsed_ms / 1000,
                    party_id = session.party_id,
                    current_round = session.current_round,
                    "MPC session advanced successfully"
                );

                dwallet_mpc_metrics
                    .add_advance_completion(&request_input, &session.current_round.to_string());
                dwallet_mpc_metrics.set_last_completion_duration(
                    &request_input,
                    &session.current_round.to_string(),
                    elapsed.as_millis() as i64,
                );
            }

            handle.spawn(async move {
                let start_send = Instant::now();
                if let Err(err) = computation_channel_sender
                    .send(CompletedComputationReport {
                        session_identifier: session.session_identifier,
                        round: session.current_round,
                        attempts_count: session.attempts_count,
                    })
                    .await
                {
                    error!(
                        ?err,
                        "failed to send a finished computation message with error"
                    );
                } else {
                    let elapsed_ms = start_send.elapsed().as_millis();
                    info!(
                        duration_ms = elapsed_ms,
                        mpc_protocol=?request_input,
                        duration_seconds = elapsed_ms / 1000,
                        "Computation update message sent"
                    );
                }
            });
        });

        self.currently_running_sessions_count += 1;

        Ok(())
    }
}
