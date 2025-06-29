//! A module to verify the dWallet MPC outputs.
//! The module handles storing the outputs received for each session,
//! and deciding whether an output is valid
//! by checking if an authorized validator set voted for it.
//! Any validator that voted for a different output is considered malicious.

use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::dwallet_mpc::dwallet_mpc_metrics::DWalletMPCMetrics;
use crate::dwallet_mpc::mpc_session::MPCSessionLogger;
use crate::stake_aggregator::StakeAggregator;
use commitment::CommitmentSizedNumber;
use dwallet_mpc_types::dwallet_mpc::SerializedWrappedMPCPublicOutput;
use ika_types::crypto::AuthorityName;
use ika_types::dwallet_mpc_error::DwalletMPCResult;
use ika_types::error::IkaResult;
use ika_types::messages_dwallet_mpc::{SessionIdentifier, SessionInfo};
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{error, info};

/// Verify the DWallet MPC outputs.
///
/// Stores all the outputs received for each session,
/// and decides whether an output is valid
/// by checking if a validators with quorum of stake voted for it.
pub struct DWalletMPCOutputsVerifier {
    /// The outputs received for each MPC session.
    mpc_sessions_outputs: HashMap<SessionIdentifier, SessionOutputsData>,
    consensus_round_completed_sessions_sender: mpsc::UnboundedSender<SessionIdentifier>,
    dwallet_mpc_metrics: Arc<DWalletMPCMetrics>,
}

/// The data needed to manage the outputs of an MPC session.
struct SessionOutputsData {
    /// Maps session's output to the authorities that voted for it.
    /// The key must contain the session info, and the output to prevent
    /// malicious behavior, such as sending the correct output, but from a faulty session.
    session_output_to_voting_authorities:
        HashMap<(SerializedWrappedMPCPublicOutput, SessionInfo), StakeAggregator<(), true>>,
    /// Needed to make sure an authority does not send two outputs for the same session.
    authorities_that_sent_output: HashSet<AuthorityName>,
    current_result: OutputVerificationStatus,
}

impl SessionOutputsData {
    fn clear_data(&mut self) {
        self.session_output_to_voting_authorities.clear();
        self.authorities_that_sent_output.clear();
    }
}

/// The result of verifying an incoming output for an MPC session.
/// We need to differentiate between a duplicate and a malicious output,
/// as the output can be sent twice by honest parties.
#[derive(PartialOrd, PartialEq, Clone)]
pub enum OutputVerificationStatus {
    FirstQuorumReached(SerializedWrappedMPCPublicOutput),
    Malicious,
    /// We need more votes to decide if the output is valid or not.
    NotEnoughVotes,
    /// The output has already been verified and committed to the chain.
    /// This happens every time since all honest parties send the same output.
    AlreadyCommitted,
}

pub struct OutputVerificationResult {
    pub result: OutputVerificationStatus,
    pub malicious_actors: Vec<AuthorityName>,
}

impl DWalletMPCOutputsVerifier {
    pub fn new(
        dwallet_mpc_metrics: Arc<DWalletMPCMetrics>,
        consensus_round_completed_sessions_sender: mpsc::UnboundedSender<SessionIdentifier>,
    ) -> Self {
        DWalletMPCOutputsVerifier {
            mpc_sessions_outputs: HashMap::new(),
            dwallet_mpc_metrics,
            consensus_round_completed_sessions_sender,
        }
    }

    /// Stores the given MPC output, and checks if any of the received
    /// outputs already received a quorum of votes.
    /// If so, the output is returned along with a vector of malicious actors,
    /// i.e., parties that voted for other outputs.
    // TODO (#311): Make sure validator don't mark other validators as malicious
    // TODO (#311): or take any active action while syncing
    pub fn try_verify_output(
        &mut self,
        output: &[u8],
        session_info: &SessionInfo,
        origin_authority: AuthorityName,
        epoch_store: &AuthorityPerEpochStore,
    ) -> DwalletMPCResult<OutputVerificationResult> {
        // TODO (#876): Set the maximum message size to the smallest size possible.
        info!(
            mpc_protocol=?session_info.mpc_round,
            session_identifier=?session_info.session_identifier,
            from_authority=?origin_authority,
            receiving_authority=?epoch_store.name,
            output_size_bytes=?output.len(),
            "Received DWallet MPC output",
        );
        let committee = epoch_store.committee().clone();

        let session_output_data = self
            .mpc_sessions_outputs
            .entry(session_info.session_identifier)
            .or_insert(SessionOutputsData {
                session_output_to_voting_authorities: HashMap::new(),
                authorities_that_sent_output: HashSet::new(),
                current_result: OutputVerificationStatus::NotEnoughVotes,
            });
        if session_output_data.current_result == OutputVerificationStatus::AlreadyCommitted {
            return Ok(OutputVerificationResult {
                result: OutputVerificationStatus::AlreadyCommitted,
                malicious_actors: vec![],
            });
        }
        // Sent more than once.
        if session_output_data
            .authorities_that_sent_output
            .contains(&origin_authority)
        {
            // Duplicate.
            return Ok(OutputVerificationResult {
                result: OutputVerificationStatus::AlreadyCommitted,
                malicious_actors: vec![],
            });
        }
        session_output_data
            .authorities_that_sent_output
            .insert(origin_authority);

        if session_output_data
            .session_output_to_voting_authorities
            .entry((output.to_owned(), session_info.clone()))
            .or_insert(StakeAggregator::new(committee))
            .insert_generic(origin_authority, ())
            .is_quorum_reached()
        {
            session_output_data.current_result = OutputVerificationStatus::AlreadyCommitted;
            session_output_data.clear_data();
            if let Err(e) = self
                .consensus_round_completed_sessions_sender
                .send(session_info.session_identifier)
            {
                error!(
                    e=?e,
                    "error in sending completed session ID"
                );
            };
            let mpc_event_data = session_info.mpc_round.clone();
            self.dwallet_mpc_metrics.add_completion(&mpc_event_data);
            return Ok(OutputVerificationResult {
                result: OutputVerificationStatus::FirstQuorumReached(output.to_owned()),
                malicious_actors: vec![],
            });
        }
        Ok(OutputVerificationResult {
            result: OutputVerificationStatus::NotEnoughVotes,
            malicious_actors: vec![],
        })
    }

    /// Syncs the [`DWalletMPCOutputsVerifier`] from the epoch start.
    /// Fails only if the epoch switched in the middle of the state sync (in which case the process is exited and this struct would be re-initialized).
    pub fn bootstrap_from_storage(&mut self, epoch_store: &AuthorityPerEpochStore) -> IkaResult {
        info!("Bootstrapping MPC Outputs Verifier from Storage");
        for output in epoch_store.tables()?.get_all_dwallet_mpc_outputs()? {
            let party_to_authority_map = epoch_store.committee().party_to_authority_map();
            let mpc_protocol_name = output.session_info.mpc_round.to_string();

            // Create a base logger with common parameters.
            let base_logger = MPCSessionLogger::new()
                .with_protocol_name(mpc_protocol_name.clone())
                .with_party_to_authority_map(party_to_authority_map.clone());
            let session_identifier = CommitmentSizedNumber::from_le_slice(
                &output.session_info.session_identifier.into_bytes(),
            );
            base_logger.write_output_to_disk(
                session_identifier,
                epoch_store.authority_name_to_party_id(&epoch_store.name)?,
                epoch_store.authority_name_to_party_id(&output.authority)?,
                &output.output,
                &output.session_info,
            );
            if let Err(err) = self.try_verify_output(
                &output.output,
                &output.session_info,
                output.authority,
                epoch_store,
            ) {
                error!(
                    "failed to verify output from session {:?} and party {:?}: {:?}",
                    output.session_info.session_identifier, output.authority, err
                );
            }
        }

        Ok(())
    }
}
