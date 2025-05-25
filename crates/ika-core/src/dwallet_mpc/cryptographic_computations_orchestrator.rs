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
use crate::dwallet_mpc::dwallet_mpc_metrics::DWalletMPCMetrics;
use crate::dwallet_mpc::mpc_session::DWalletMPCSession;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_dwallet_mpc::MPCProtocolInitData;
use std::sync::Arc;
use std::time::Instant;
use tokio::runtime::Handle;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tracing::{error, info};

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
    computation_channel_receiver: UnboundedReceiver<ComputationUpdate>,

    /// The number of currently running cryptographic computations.
    /// Tracks tasks that have been spawned with [`rayon::spawn_fifo`] but haven't completed yet.
    /// Used to prevent exceeding available CPU cores.
    currently_running_sessions_count: usize,
}

impl CryptographicComputationsOrchestrator {
    /// Creates a new orchestrator for cryptographic computations.
    pub(crate) fn try_new() -> DwalletMPCResult<Self> {
        let (completed_computation_channel_sender, completed_computation_channel_receiver) =
            tokio::sync::mpsc::unbounded_channel();
        let available_cores_for_computations: usize = std::thread::available_parallelism()
            .map_err(|e| DwalletMPCError::FailedToGetAvailableParallelism(e.to_string()))?
            .into();
        if !(available_cores_for_computations > 0) {
            error!(
                "failed to get available parallelism, no CPU cores available for cryptographic computations"
            );
            return Err(DwalletMPCError::InsufficientCPUCores);
        }
        info!(
            available_cores_for_computations =? available_cores_for_computations,
            "available CPU cores for Rayon cryptographic computations"
        );

        Ok(CryptographicComputationsOrchestrator {
            available_cores_for_cryptographic_computations: available_cores_for_computations,
            computation_channel_sender: completed_computation_channel_sender,
            computation_channel_receiver: completed_computation_channel_receiver,
            currently_running_sessions_count: 0,
        })
    }

    pub(crate) fn check_for_completed_computations(&mut self) {
        loop {
            match self.computation_channel_receiver.try_recv() {
                Ok(computation_update) => match computation_update {
                    ComputationUpdate::Started => {
                        info!(
                            currently_running_sessions_count =? self.currently_running_sessions_count,
                            "Started cryptographic computation, increasing count"
                        );
                        self.currently_running_sessions_count += 1;
                    }
                    ComputationUpdate::Completed => {
                        info!(
                            currently_running_sessions_count =? self.currently_running_sessions_count,
                            "Completed cryptographic computation, decreasing count"
                        );
                        self.currently_running_sessions_count -= 1;
                    }
                },
                Err(err) => match err {
                    TryRecvError::Empty => {
                        info!("no new completed computations");
                        return;
                    }
                    TryRecvError::Disconnected => {
                        error!("cryptographic computations channel got disconnected");
                        return;
                    }
                },
            }
        }
    }

    /// Checks if a new session can be spawned based on available CPU cores.
    pub(crate) fn can_spawn_session(&self) -> bool {
        self.currently_running_sessions_count < self.available_cores_for_cryptographic_computations
    }

    pub(super) fn spawn_session(
        &mut self,
        session: &DWalletMPCSession,
        dwallet_mpc_metrics: Arc<DWalletMPCMetrics>,
    ) -> DwalletMPCResult<()> {
        let handle = Handle::current();
        let session = session.clone();
        Self::update_started_computation_metric(
            // Safe to unwrap here (event must exist before this).
            &session.mpc_event_data.clone().unwrap().init_protocol_data,
            dwallet_mpc_metrics.clone(),
        );
        if let Err(err) = self
            .computation_channel_sender
            .send(ComputationUpdate::Started)
        {
            error!(
                "failed to send a started computation message with error: {:?}",
                err
            );
        }
        let computation_channel_sender = self.computation_channel_sender.clone();
        rayon::spawn_fifo(move || {
            let start_advance = Instant::now();
            if let Err(err) = session.advance(&handle) {
                error!("failed to advance session with error: {:?}", err);
            };
            let elapsed = start_advance.elapsed();
            Self::update_completed_computation_metric(
                // Safe to unwrap here (event must exist before this).
                &session.mpc_event_data.unwrap().init_protocol_data,
                dwallet_mpc_metrics.clone(),
                elapsed.as_millis(),
            );
            if let Err(err) = computation_channel_sender.send(ComputationUpdate::Completed) {
                error!(
                    "failed to send a finished computation message with error: {:?}",
                    err
                );
            }
        });
        Ok(())
    }

    fn update_started_computation_metric(
        mpc_protocol_init_data: &MPCProtocolInitData,
        dwallet_mpc_metrics: Arc<DWalletMPCMetrics>,
    ) {
        match &mpc_protocol_init_data {
            MPCProtocolInitData::DKGFirst(_) => {
                dwallet_mpc_metrics
                    .advance_calls_for_dwallet_dkg_first_round
                    .inc();
            }
            MPCProtocolInitData::DKGSecond(_) => {
                dwallet_mpc_metrics
                    .advance_calls_for_dwallet_dkg_second_round
                    .inc();
            }
            MPCProtocolInitData::Presign(_) => {
                dwallet_mpc_metrics.advance_calls_for_presign.inc();
            }
            MPCProtocolInitData::Sign(_) => {
                dwallet_mpc_metrics.advance_calls_for_sign.inc();
            }
            MPCProtocolInitData::NetworkDkg(_, _) => {
                dwallet_mpc_metrics.advance_calls_for_network_dkg.inc();
            }
            MPCProtocolInitData::EncryptedShareVerification(_) => {
                dwallet_mpc_metrics
                    .advance_calls_for_encrypted_share_verification
                    .inc();
            }
            MPCProtocolInitData::PartialSignatureVerification(_) => {
                dwallet_mpc_metrics
                    .advance_calls_for_partial_signature_verification
                    .inc();
            }
            MPCProtocolInitData::DecryptionKeyReshare(_) => {
                dwallet_mpc_metrics
                    .advance_calls_for_decryption_key_reshare
                    .inc();
            }
            MPCProtocolInitData::MakeDWalletUserSecretKeySharesPublicRequest(_) => {
                dwallet_mpc_metrics
                    .advance_calls_for_make_dwallet_user_secret_key_shares_public
                    .inc()
            }
            MPCProtocolInitData::DWalletImportedKeyVerificationRequest(_) => {
                dwallet_mpc_metrics
                    .advance_calls_for_import_dwallet_verification
                    .inc();
            }
            MPCProtocolInitData::MakeDWalletUserSecretKeySharesPublicRequest(_) => {
                dwallet_mpc_metrics
                    .advance_calls_for_make_dwallet_user_secret_key_shares_public
                    .inc()
            }
            MPCProtocolInitData::DWalletImportedKeyVerificationRequestEvent(_) => {
                dwallet_mpc_metrics
                    .advance_calls_for_import_dwallet_verification
                    .inc();
            }
        }
    }

    fn update_completed_computation_metric(
        mpc_protocol_init_data: &MPCProtocolInitData,
        dwallet_mpc_metrics: Arc<DWalletMPCMetrics>,
        computation_duration: u128,
    ) {
        match &mpc_protocol_init_data {
            MPCProtocolInitData::DKGFirst(_) => {
                dwallet_mpc_metrics
                    .advance_completions_for_dwallet_dkg_first_round
                    .inc();
                dwallet_mpc_metrics
                    .dwallet_dkg_first_round_completion_duration
                    .set(computation_duration as i64);
            }
            MPCProtocolInitData::DKGSecond(_) => {
                dwallet_mpc_metrics
                    .advance_completions_for_dwallet_dkg_second_round
                    .inc();
                dwallet_mpc_metrics
                    .dwallet_dkg_second_round_completion_duration
                    .set(computation_duration as i64);
            }
            MPCProtocolInitData::Presign(_) => {
                dwallet_mpc_metrics.advance_completions_for_presign.inc();
                dwallet_mpc_metrics
                    .presign_last_completion_duration
                    .set(computation_duration as i64);
            }
            MPCProtocolInitData::Sign(_) => {
                dwallet_mpc_metrics.advance_completions_for_sign.inc();
                dwallet_mpc_metrics
                    .sign_last_completion_duration
                    .set(computation_duration as i64);
            }
            MPCProtocolInitData::NetworkDkg(_, _) => {
                dwallet_mpc_metrics
                    .advance_completions_for_network_dkg
                    .inc();
                dwallet_mpc_metrics
                    .network_dkg_completion_duration
                    .set(computation_duration as i64);
            }
            MPCProtocolInitData::EncryptedShareVerification(_) => {
                dwallet_mpc_metrics
                    .advance_completions_for_encrypted_share_verification
                    .inc();
                dwallet_mpc_metrics
                    .encrypted_share_verification_completion_duration
                    .set(computation_duration as i64);
            }
            MPCProtocolInitData::PartialSignatureVerification(_) => {
                dwallet_mpc_metrics
                    .advance_completions_for_partial_signature_verification
                    .inc();
                dwallet_mpc_metrics
                    .partial_signature_verification_completion_duration
                    .set(computation_duration as i64);
            }
            MPCProtocolInitData::DecryptionKeyReshare(_) => {
                dwallet_mpc_metrics
                    .advance_completions_for_decryption_key_reshare
                    .inc();
                dwallet_mpc_metrics
                    .decryption_key_reshare_completion_duration
                    .set(computation_duration as i64);
            }
            MPCProtocolInitData::MakeDWalletUserSecretKeySharesPublicRequest(_) => {
                dwallet_mpc_metrics
                    .advance_completions_for_make_dwallet_user_secret_key_shares_public
                    .inc();
                dwallet_mpc_metrics
                    .make_dwallet_user_secret_key_shares_public_completion_duration
                    .set(computation_duration as i64);
            }
            MPCProtocolInitData::DWalletImportedKeyVerificationRequest(_) => {
                dwallet_mpc_metrics
                    .advance_completions_for_import_dwallet_verification
                    .inc();
                dwallet_mpc_metrics
                    .import_dwallet_verification_completion_duration
                    .set(computation_duration as i64);
            }
        }
    }
}
