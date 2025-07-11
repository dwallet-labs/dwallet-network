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

use std::collections::{HashMap, HashSet};
use crate::dwallet_mpc::dwallet_mpc_metrics::DWalletMPCMetrics;
use crate::dwallet_mpc::mpc_session::DWalletMPCSession;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_dwallet_mpc::{MPCRequestInput, SessionIdentifier};
use std::sync::Arc;
use std::time::Instant;
use group::PartyID;
use mpc::WeightedThresholdAccessStructure;
use tokio::runtime::Handle;
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::{debug, error, info, warn};
use dwallet_mpc_types::dwallet_mpc::{MPCMessage, MPCPrivateOutput, SerializedWrappedMPCPublicOutput};
use dwallet_rng::RootSeed;
use ika_types::committee::Committee;

/// Channel size for cryptographic computations state updates.
/// This channel should not reach a size even close to this.
/// But since this is critical to keep the computations running,
/// we are using a big buffer (this size of the data is small).
const COMPUTATION_UPDATE_CHANNEL_SIZE: usize = 10_000;

/// A unique key for a computation request.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) struct ComputationId {
    pub(crate) session_identifier: SessionIdentifier,
    pub(crate) mpc_round: usize,
    pub(crate) attempt_number: usize,
}

// TODO: move to file, together with the function
#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct ComputationRequest {
    pub(crate) party_id: PartyID,
    pub(crate) validator_name: AuthorityPublicKeyBytes,
    pub(crate) committee: Arc<Committee>,
    pub(crate) access_structure: WeightedThresholdAccessStructure,
    pub(crate) input: MPCRequestInput,
    pub(crate) messages: HashMap<usize, HashMap<PartyID, MPCMessage>>,
}

struct ComputationCompletionUpdate {
    computation_id: ComputationId,
    computation_result: DwalletMPCResult<
        mpc::AsynchronousRoundResult<MPCMessage, MPCPrivateOutput, SerializedWrappedMPCPublicOutput>,
    >
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
    completed_computation_sender: Sender<ComputationCompletionUpdate>,
    completed_computation_receiver: Receiver<ComputationCompletionUpdate>,

    /// The currently running cryptographic computations.
    /// Tracks tasks that have been spawned with [`rayon::spawn_fifo`] but haven't completed yet.
    /// Used to prevent exceeding available CPU cores.
    currently_running_cryptographic_computations: HashSet<ComputationId>,

    /// The root seed of this validator, used for deriving the per-round seed for advancing this session.
    /// SECURITY NOTICE: *MUST KEEP PRIVATE*.
    root_seed: RootSeed,
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
                "Validator must have at least 16 CPU cores"
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
            currently_running_cryptographic_computations: HashSet::new(),
        })
    }

    /// Check for completed computations, and sufficient CPU cores.
    pub(crate) fn receive_completed_computations(&mut self) -> HashMap<ComputationId, DwalletMPCResult<
        mpc::AsynchronousRoundResult<MPCMessage, MPCPrivateOutput, SerializedWrappedMPCPublicOutput>,
    >> {
        let mut completed_computation_results = HashMap::new();
        while let Ok(computation_update) = self.completed_computation_receiver.try_recv() {
            self.currently_running_cryptographic_computations.remove(&computation_update.computation_id);
            completed_computation_results.insert(computation_update.computation_id, computation_update.computation_result);

            debug!(
                session_id=?computation_update.computation_id.session_identifier,
                mpc_round=?computation_update.computation_id.mpc_round,
                attempt_number=?computation_update.computation_id.attempt_number,
                currently_running_sessions_count =? self.currently_running_cryptographic_computations.len(),
                "Received a cryptographic computation completed update"
            );
        }

        completed_computation_results
    }

    /// Check for completed computations, and sufficient CPU cores.
    pub fn has_available_cores_to_perform_computation(&mut self) -> bool {
        self.currently_running_cryptographic_computations.len() < self.available_cores_for_cryptographic_computations
    }

    /// Try to spawn a cryptographic `computation_request` to execute in a different thread,
    /// if a CPU core is available for it.
    ///
    /// Return `true` if the request is executing (whether it is newly executing or was already running),
    /// and `false` if no cores were available to execute it.
    pub(super) async fn try_spawn_cryptographic_computation(
        &mut self,
        computation_id: ComputationId,
        computation_request: ComputationRequest,
        dwallet_mpc_metrics: Arc<DWalletMPCMetrics>,
    ) -> bool {
        // TODO: save the outputs, add getter.

        if !self.has_available_cores_to_perform_computation() {
            info!(
                session_id=?computation_id.session_identifier,
                mpc_round=?computation_id.mpc_round,
                attempt_number=?computation_id.attempt_number,
                mpc_protocol=?computation_request.input,
                "No available CPU cores to perform cryptographic computation"
            );

            return false;
        }

        if self.currently_running_cryptographic_computations.contains(&computation_id) {
            // Don't run a task that's already running.
            // Return `true` to signal that this task is executing, we haven't failed spawning it.
            return true;
        }

        let handle = Handle::current();

        // TODO(Scaly): `dwallet_mpc_metrics` is Arc, how can we write to it?
        dwallet_mpc_metrics.add_advance_call(&computation_request.input, &computation_id.mpc_round.to_string());

        let computation_channel_sender = self.completed_computation_sender.clone();
        rayon::spawn_fifo(move || {
            let advance_start_time = Instant::now();

            let computation_result = computation_request.compute();
            if let Err(err) = &computation_result
                {
                   error!(
                session_id=?computation_id.session_identifier,
                mpc_round=?computation_id.mpc_round,
                attempt_number=?computation_id.attempt_number,
                mpc_protocol=?computation_request.input,
                error=?err,
                "Cryptographic computation failed"
            );
            } else {
                let elapsed = advance_start_time.elapsed();
                let elapsed_ms = elapsed.as_millis();

                info!(
                party_id=?computation_request.party_id,
                session_id=?computation_id.session_identifier,
                mpc_round=?computation_id.mpc_round,
                attempt_number=?computation_id.attempt_number,
                mpc_protocol=?computation_request.input,
                duration_ms = elapsed_ms,
                duration_seconds = elapsed_ms / 1000,
                "Cryptographic computation completed successfully"
            );

                dwallet_mpc_metrics
                    .add_advance_completion(&computation_request.input, &computation_id.mpc_round.to_string());

                dwallet_mpc_metrics.set_last_completion_duration(
                    &computation_request.input,
                    &computation_id.mpc_round.to_string(),
                    elapsed.as_millis() as i64,
                );
            }

            // TODO(Scaly): why is this from handle?
            handle.spawn(async move {
                if let Err(err) = computation_channel_sender
                    .send(ComputationCompletionUpdate {
                        computation_id,
                        computation_result
                    })
                    .await
                {
                    error!(
                        ?err,
                        "failed to send a computation completion update"
                    );
                }
            });
        });

        self.currently_running_cryptographic_computations.insert(computation_id);

        true
    }
}
