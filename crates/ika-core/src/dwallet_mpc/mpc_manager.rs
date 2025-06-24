use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::consensus_adapter::SubmitToConsensus;
use ika_types::error::IkaResult;
use sui_types::base_types::ObjectID;

use crate::dwallet_mpc::cryptographic_computations_orchestrator::CryptographicComputationsOrchestrator;
use crate::dwallet_mpc::dwallet_mpc_metrics::DWalletMPCMetrics;
use crate::dwallet_mpc::malicious_handler::MaliciousHandler;
use crate::dwallet_mpc::mpc_protocols::network_dkg::{
    DwalletMPCNetworkKeys, ValidatorPrivateDecryptionKeyData,
};
use crate::dwallet_mpc::mpc_session::{DWalletMPCSession, MPCEventData};
use crate::dwallet_mpc::{mpc_session::session_input_from_event, party_ids_to_authority_names};
use crate::stake_aggregator::StakeAggregator;
use class_groups::Secp256k1DecryptionKeySharePublicParameters;
use dwallet_classgroups_types::ClassGroupsEncryptionKeyAndProof;
use dwallet_mpc_types::dwallet_mpc::{MPCSessionStatus, VersionedNetworkDkgOutput};
use group::PartyID;
use ika_config::NodeConfig;
use ika_types::committee::{Committee, EpochId};
use ika_types::crypto::AuthorityName;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_dwallet_mpc::{
    AsyncProtocol, DBSuiEvent, DWalletMPCEvent, DWalletMPCMessage, MPCProtocolInitData,
    MaliciousReport, SessionIdentifier, SessionInfo, SessionType, ThresholdNotReachedReport,
};
use mpc::WeightedThresholdAccessStructure;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Weak};
use tokio::sync::watch;
use tracing::{debug, error, info, warn};
use twopc_mpc::sign::Protocol;

/// The [`DWalletMPCManager`] manages MPC sessions:
/// — Keeping track of all MPC sessions,
/// — Executing all active sessions, and
/// — (De)activating sessions.
///
/// The correct way to use the manager is to create it along with all other Ika components
/// at the start of each epoch.
/// Ensuring it is destroyed when the epoch ends and providing a clean slate for each new epoch.
pub(crate) struct DWalletMPCManager {
    /// The party ID of the current authority. Based on the authority index in the committee.
    party_id: PartyID,
    /// MPC sessions that where created.
    pub(crate) mpc_sessions: HashMap<SessionIdentifier, DWalletMPCSession>,
    consensus_adapter: Arc<dyn SubmitToConsensus>,
    pub(super) node_config: NodeConfig,
    epoch_store: Weak<AuthorityPerEpochStore>,
    epoch_id: EpochId,
    pub(crate) weighted_threshold_access_structure: WeightedThresholdAccessStructure,
    pub(crate) validators_class_groups_public_keys_and_proofs:
        HashMap<PartyID, ClassGroupsEncryptionKeyAndProof>,
    pub(crate) cryptographic_computations_orchestrator: CryptographicComputationsOrchestrator,
    /// A struct for managing malicious actors in MPC protocols.
    /// This struct maintains a record of malicious actors reported by validators.
    /// An actor is deemed malicious if it is reported by a quorum of validators.
    /// Any message/output from these authorities will be ignored.
    /// This list is maintained during the Epoch.
    /// This happens automatically because the [`DWalletMPCManager`]
    /// is part of the [`AuthorityPerEpochStore`].
    pub(crate) malicious_handler: MaliciousHandler,
    /// The order of the sessions that are ready to get computed.
    pub(crate) pending_for_computation_order: VecDeque<DWalletMPCSession>,
    /// The order of the sessions that have received quorum for their current round, but we have not
    /// yet received an event for from Sui.
    pub(crate) pending_for_events_order: VecDeque<DWalletMPCSession>,
    pub(crate) last_session_to_complete_in_current_epoch: u64,
    pub(crate) recognized_self_as_malicious: bool,
    pub(crate) network_keys: Box<DwalletMPCNetworkKeys>,
    /// Events that wait for the network key to update.
    /// Once we get the network key, these events will continue.
    pub(crate) events_pending_for_network_key: Vec<DWalletMPCEvent>,
    pub(crate) next_epoch_committee_receiver: watch::Receiver<Committee>,
    pub(crate) dwallet_mpc_metrics: Arc<DWalletMPCMetrics>,
    pub(crate) threshold_not_reached_reports:
        HashMap<ThresholdNotReachedReport, StakeAggregator<(), true>>,
}

/// The messages that the [`DWalletMPCManager`] can receive and process asynchronously.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DWalletMPCDBMessage {
    /// An MPC message from another validator.
    Message(DWalletMPCMessage),
    /// Signal delivery of messages has ended,
    /// now the sessions that received a quorum of messages can advance.
    EndOfDelivery,
    /// A message indicating that an MPC session has failed.
    /// The advance failed, and the session needs to be restarted or marked as failed.
    MPCSessionFailed(ObjectID),
    /// A message to start processing the cryptographic computations.
    /// This message is being sent every five seconds by the dWallet MPC Service,
    /// to skip redundant advancements that have already been completed by other validators.
    PerformCryptographicComputations,

    /// A message that continas a [`MaliciousReport`] after an advance/finalize.
    /// AuthorityName is the name of the authority that reported the malicious parties.
    MaliciousReport(AuthorityName, MaliciousReport),
    /// A meesage indicating that some of the parteis were malicous,
    /// but we can still retry once we recieve more messages.
    ThresholdNotReachedReport(AuthorityName, ThresholdNotReachedReport),
}

struct ReadySessionsResponse {
    ready_sessions: Vec<DWalletMPCSession>,
    pending_for_event_sessions: Vec<DWalletMPCSession>,
    malicious_actors: Vec<PartyID>,
}

impl DWalletMPCManager {
    pub(crate) async fn must_create_dwallet_mpc_manager(
        consensus_adapter: Arc<dyn SubmitToConsensus>,
        epoch_store: Arc<AuthorityPerEpochStore>,
        next_epoch_committee_receiver: watch::Receiver<Committee>,
        node_config: NodeConfig,
        dwallet_mpc_metrics: Arc<DWalletMPCMetrics>,
    ) -> Self {
        Self::try_new(
            consensus_adapter.clone(),
            epoch_store.clone(),
            next_epoch_committee_receiver,
            node_config.clone(),
            dwallet_mpc_metrics,
        )
        .unwrap_or_else(|err| {
            error!(?err, "Failed to create DWalletMPCManager.");
            // We panic on purpose, this should not happen.
            panic!("DWalletMPCManager initialization failed: {:?}", err);
        })
    }

    pub fn try_new(
        consensus_adapter: Arc<dyn SubmitToConsensus>,
        epoch_store: Arc<AuthorityPerEpochStore>,
        next_epoch_committee_receiver: watch::Receiver<Committee>,
        node_config: NodeConfig,
        dwallet_mpc_metrics: Arc<DWalletMPCMetrics>,
    ) -> DwalletMPCResult<Self> {
        let weighted_threshold_access_structure =
            epoch_store.get_weighted_threshold_access_structure()?;
        let mpc_computations_orchestrator = CryptographicComputationsOrchestrator::try_new()?;
        let party_id = epoch_store.authority_name_to_party_id(&epoch_store.name)?;
        let validator_private_data = ValidatorPrivateDecryptionKeyData {
            party_id,
            class_groups_decryption_key: node_config
                .class_groups_key_pair_and_proof
                .clone()
                // Since this is a validator, we can unwrap
                // the `class_groups_key_pair_and_proof`.
                .expect("Class groups key pair and proof must be present")
                .class_groups_keypair()
                .decryption_key(),
            validator_decryption_key_shares: HashMap::new(),
        };
        let dwallet_network_keys = DwalletMPCNetworkKeys::new(validator_private_data);
        Ok(Self {
            mpc_sessions: HashMap::new(),
            consensus_adapter,
            party_id: epoch_store.authority_name_to_party_id(&epoch_store.name.clone())?,
            epoch_store: Arc::downgrade(&epoch_store),
            epoch_id: epoch_store.epoch(),
            node_config,
            weighted_threshold_access_structure,
            validators_class_groups_public_keys_and_proofs: epoch_store
                .get_validators_class_groups_public_keys_and_proofs()
                .map_err(|e| DwalletMPCError::MPCManagerError(e.to_string()))?,
            cryptographic_computations_orchestrator: mpc_computations_orchestrator,
            malicious_handler: MaliciousHandler::new(epoch_store.committee().clone()),
            pending_for_computation_order: VecDeque::new(),
            pending_for_events_order: Default::default(),
            last_session_to_complete_in_current_epoch: 0,
            recognized_self_as_malicious: false,
            network_keys: Box::new(dwallet_network_keys),
            next_epoch_committee_receiver,
            events_pending_for_network_key: vec![],
            dwallet_mpc_metrics,
            threshold_not_reached_reports: Default::default(),
        })
    }

    pub(crate) fn update_last_session_to_complete_in_current_epoch(
        &mut self,
        update_last_session_to_complete_in_current_epoch: u64,
    ) {
        if update_last_session_to_complete_in_current_epoch
            <= self.last_session_to_complete_in_current_epoch
        {
            return;
        }
        self.last_session_to_complete_in_current_epoch =
            update_last_session_to_complete_in_current_epoch;
    }

    pub(crate) async fn handle_dwallet_db_event(&mut self, event: DWalletMPCEvent) {
        if event.session_info.epoch != self.epoch_id {
            warn!(
                session_identifier=?event.session_info.session_identifier,
                event_type=?event.event,
                event_epoch=?event.session_info.epoch,
                "received an event for a different epoch, skipping"
            );
            return;
        }
        if let Err(err) = self
            .handle_event(event.event.clone(), event.session_info.clone())
            .await
        {
            match err {
                DwalletMPCError::WaitingForNetworkKey(key_id) => {
                    // This is not an error, we are waiting for the network key to be updated.
                    info!(
                        ?err,
                        session_info=?event.session_info,
                        type=?event.event.type_,
                        key_id=?key_id,
                        "Adding event to pending for the network key"
                    );
                    self.events_pending_for_network_key.push(event);
                }
                _ => {
                    error!(
                        ?err,
                        ?event.event.type_,
                        session_info=?event.session_info,
                        "failed to handle dWallet MPC event with error"
                    );
                }
            }
        }
    }

    pub(crate) async fn handle_dwallet_db_message(&mut self, message: DWalletMPCDBMessage) {
        match message {
            DWalletMPCDBMessage::PerformCryptographicComputations => {
                self.perform_cryptographic_computation().await;
            }
            DWalletMPCDBMessage::Message(message) => {
                if let Err(err) = self.handle_message(message.clone()) {
                    error!(
                        ?err,
                        session_identifier=?message.session_identifier,
                        from_authority=?message.authority,
                        "failed to handle an MPC message with error"
                    );
                }
            }
            DWalletMPCDBMessage::EndOfDelivery => {
                if let Err(err) = self.handle_end_of_delivery().await {
                    error!("failed to handle the end of delivery with error: {:?}", err);
                }
            }
            DWalletMPCDBMessage::MPCSessionFailed(session_id) => {
                error!(session_id=?session_id, "dwallet MPC session failed");
                // TODO (#524): Handle failed MPC sessions
            }
            DWalletMPCDBMessage::MaliciousReport(authority_name, report) => {
                if let Err(err) = self.handle_malicious_report(authority_name, report) {
                    error!(
                        ?err,
                        "dWallet MPC session failed with malicious parties with error",
                    );
                }
            }
            DWalletMPCDBMessage::ThresholdNotReachedReport(authority, report) => {
                if let Err(err) = self.handle_threshold_not_reached_report(report, authority) {
                    error!(
                        ?err,
                        "dWallet MPC session failed — threshold not reached with error",
                    );
                }
            }
        }
    }

    fn handle_threshold_not_reached_report(
        &mut self,
        report: ThresholdNotReachedReport,
        origin_authority: AuthorityName,
    ) -> DwalletMPCResult<()> {
        // Previously malicious actors are ignored.
        if self
            .malicious_handler
            .get_malicious_actors_names()
            .contains(&origin_authority)
        {
            return Ok(());
        }
        let committee = self.epoch_store()?.committee().clone();
        let current_voters_for_report = self
            .threshold_not_reached_reports
            .entry(report.clone())
            .or_insert(StakeAggregator::new(committee));
        // We already have a quorum for this report.
        if current_voters_for_report.has_quorum() {
            // Do nothing, quorum has already been reached.
            return Ok(());
        }
        if current_voters_for_report
            .insert_generic(origin_authority, ())
            .is_quorum_reached()
        {
            // Quorum has been reached, we can report the malicious actors.
            self.prepare_for_round_retry(report.session_identifier)?;
        }
        Ok(())
    }

    fn prepare_for_round_retry(
        &mut self,
        session_identifier: SessionIdentifier,
    ) -> DwalletMPCResult<()> {
        let epoch_store = self.epoch_store()?;
        if let Some(session) = self.mpc_sessions.get_mut(&session_identifier) {
            session.attempts_count += 1;
            // We got a `TWOPCMPCThresholdNotReached` error and a quorum agreement on it.
            // So all parties that sent a regular MPC Message for the last executed
            // round are malicious—as the round aborted with the error `TWOPCMPCThresholdNotReached`.
            // All honest parties should report that there is a quorum for `ThresholdNotReached`.
            // We must then remove these messages and mark the senders as malicious.
            // Note that the current round was already incremented
            // since we received the quorum for `ThresholdNotReached`
            // on the previous round,
            // but no messages were sent for the current round.
            self.malicious_handler
                .report_malicious_actors(&party_ids_to_authority_names(
                    &session
                        .serialized_full_messages
                        .get(&(session.current_round - 1))
                        .unwrap_or(&HashMap::new())
                        .keys()
                        .cloned()
                        .collect::<Vec<PartyID>>(),
                    &epoch_store,
                )?);
            session
                .serialized_full_messages
                .remove(&(session.current_round - 1));
            // Decrement the current round, as we are going to retry the previous round.
            session.current_round -= 1;
        }
        Ok(())
    }

    /// Advance all the MPC sessions that either received enough messages
    /// or perform the first step of the flow.
    /// We parallelize the advances with `Rayon` to speed up the process.
    pub async fn handle_end_of_delivery(&mut self) -> IkaResult {
        let ready_sessions_response = self.get_ready_to_advance_sessions()?;
        if !ready_sessions_response.malicious_actors.is_empty() {
            self.flag_parties_as_malicious(&ready_sessions_response.malicious_actors)?;
        }
        self.pending_for_computation_order
            .extend(ready_sessions_response.ready_sessions);
        self.pending_for_events_order
            .extend(ready_sessions_response.pending_for_event_sessions);
        Ok(())
    }

    fn handle_malicious_report(
        &mut self,
        reporting_authority: AuthorityName,
        report: MaliciousReport,
    ) -> DwalletMPCResult<()> {
        if self
            .malicious_handler
            .get_malicious_actors_names()
            .contains(&reporting_authority)
        {
            return Ok(());
        }
        self.malicious_handler
            .report_malicious_actor(report.clone(), reporting_authority)?;
        let epoch_store = self.epoch_store()?;
        if self.malicious_handler.is_malicious_actor(&epoch_store.name) {
            self.recognized_self_as_malicious = true;
            error!(
                authority=?epoch_store.name,
                reporting_authority=?reporting_authority,
                malicious_actors=?report.malicious_actors,
                session_identifier=?report.session_identifier,
                "node recognized itself as malicious"
            );
        }
        Ok(())
    }

    async fn handle_event(
        &mut self,
        event: DBSuiEvent,
        session_info: SessionInfo,
    ) -> DwalletMPCResult<()> {
        if let Some(session) = self.mpc_sessions.get(&session_info.session_identifier) {
            if session.mpc_event_data.is_none() {
                let mpc_event_data = self.new_mpc_event_data(event, &session_info).await?;
                if let Some(mut_session) =
                    self.mpc_sessions.get_mut(&session_info.session_identifier)
                {
                    mut_session.mpc_event_data = Some(mpc_event_data);
                }
            }
        } else {
            let mpc_event_data = self.new_mpc_event_data(event, &session_info).await?;
            self.dwallet_mpc_metrics
                .add_received_event_start(&mpc_event_data.init_protocol_data);
            self.push_new_mpc_session(&session_info.session_identifier, Some(mpc_event_data));
        }
        Ok(())
    }

    async fn new_mpc_event_data(
        &self,
        event: DBSuiEvent,
        session_info: &SessionInfo,
    ) -> Result<MPCEventData, DwalletMPCError> {
        let (public_input, private_input) = session_input_from_event(event, self).await?;
        let mpc_event_data = MPCEventData {
            session_type: session_info.session_type.clone(),
            init_protocol_data: session_info.mpc_round.clone(),
            private_input,
            decryption_shares: match session_info.mpc_round.clone() {
                MPCProtocolInitData::Sign(init_event) => self.get_decryption_key_shares(
                    &init_event.event_data.dwallet_network_decryption_key_id,
                )?,
                MPCProtocolInitData::NetworkEncryptionKeyReconfiguration(init_event) => self
                    .get_decryption_key_shares(
                        &init_event.event_data.dwallet_network_decryption_key_id,
                    )?,
                _ => HashMap::new(),
            },
            public_input,
        };
        Ok(mpc_event_data)
    }

    pub(crate) fn get_protocol_public_parameters(
        &self,
        key_id: &ObjectID,
    ) -> DwalletMPCResult<twopc_mpc::secp256k1::class_groups::ProtocolPublicParameters> {
        self.network_keys.get_protocol_public_parameters(key_id)
    }

    pub(super) fn get_decryption_key_share_public_parameters(
        &self,
        key_id: &ObjectID,
    ) -> DwalletMPCResult<Secp256k1DecryptionKeySharePublicParameters> {
        self.network_keys.get_decryption_public_parameters(key_id)
    }

    pub(super) async fn get_network_dkg_public_output(
        &self,
        key_id: &ObjectID,
    ) -> DwalletMPCResult<VersionedNetworkDkgOutput> {
        self.network_keys
            .get_network_dkg_public_output(key_id)
            .await
    }

    /// Retrieves the decryption share for the current authority.
    ///
    /// This function accesses the current epoch's store and determines the party ID for the
    /// authority using its name.
    /// It then retrieves the corresponding decryption share from
    /// the node configuration.
    /// The decryption share is combined with the public parameters
    /// to build a [`DecryptionKeyShare`].
    /// If any required data is missing or invalid, an
    /// appropriate error is returned.
    fn get_decryption_key_shares(
        &self,
        key_id: &ObjectID,
    ) -> DwalletMPCResult<HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>> {
        self.network_keys
            .validator_private_dec_key_data
            .validator_decryption_key_shares
            .get(key_id)
            .cloned()
            .ok_or(DwalletMPCError::WaitingForNetworkKey(*key_id))
    }

    /// Returns the sessions that can perform the next cryptographic round,
    /// and the list of malicious parties that has
    /// been detected while checking for such sessions.
    fn get_ready_to_advance_sessions(&mut self) -> DwalletMPCResult<ReadySessionsResponse> {
        let quorum_check_results: Vec<(DWalletMPCSession, Vec<PartyID>)> = self
            .mpc_sessions
            .iter_mut()
            .filter_map(|(_, ref mut session)| {
                let quorum_check_result = session.check_quorum_for_next_crypto_round().ok()?;
                if quorum_check_result.is_ready {
                    session.received_more_messages_since_last_advance = false;
                    // We must first clone the session, as we approve to advance the current session
                    // in the current round and then start waiting for the next round's messages
                    // until it is ready to advance or finalized.
                    let session_clone = session.clone();
                    session.current_round += 1;
                    Some((session_clone, quorum_check_result.malicious_parties))
                } else {
                    None
                }
            })
            .collect();

        let malicious_parties: Vec<PartyID> = quorum_check_results
            .clone()
            .into_iter()
            .flat_map(|(_, malicious_parties)| malicious_parties)
            .collect();
        let ready_to_advance_sessions: Vec<DWalletMPCSession> = quorum_check_results
            .into_iter()
            .map(|(session, _)| session)
            .collect();
        let (ready_sessions, pending_for_event_sessions): (Vec<_>, Vec<_>) =
            ready_to_advance_sessions
                .into_iter()
                .partition(|s| s.mpc_event_data.is_some());
        Ok(ReadySessionsResponse {
            ready_sessions,
            pending_for_event_sessions,
            malicious_actors: malicious_parties,
        })
    }

    /// Spawns all ready MPC cryptographic computations using Rayon.
    /// If no local CPUs are available, computations will execute as CPUs are freed.
    pub(crate) async fn perform_cryptographic_computation(&mut self) {
        for event in self
            .events_pending_for_network_key
            .drain(..)
            .collect::<Vec<_>>()
            .into_iter()
        {
            self.handle_dwallet_db_event(event.clone()).await;
        }
        for (index, pending_for_event_session) in
            self.pending_for_events_order.clone().iter().enumerate()
        {
            let Some(live_session) = self
                .mpc_sessions
                .get(&pending_for_event_session.session_identifier)
            else {
                // This should never happen
                continue;
            };
            if live_session.mpc_event_data.is_some() {
                let mpc_protocol = live_session
                    .mpc_event_data
                    .clone()
                    .unwrap()
                    .init_protocol_data;
                info!(
                    session_identifier=?pending_for_event_session.session_identifier,
                    mpc_protocol=?mpc_protocol,
                    "Received event data for a known session"
                );
                let mut ready_to_advance_session = pending_for_event_session.clone();
                ready_to_advance_session.mpc_event_data = live_session.mpc_event_data.clone();
                self.pending_for_computation_order
                    .push_back(ready_to_advance_session);
                self.pending_for_events_order.remove(index);
            }
        }
        let pending_for_computation = self.pending_for_computation_order.len();
        for _ in 0..pending_for_computation {
            if !self
                .cryptographic_computations_orchestrator
                .can_spawn_session()
            {
                warn!(
                    pending_for_computation=pending_for_computation,
                    avliable_cores=?self.cryptographic_computations_orchestrator.available_cores_for_cryptographic_computations,
                    currently_running_sessions_count=?self.cryptographic_computations_orchestrator.currently_running_sessions_count,
                    "No available CPUs for cryptographic computations, waiting for a free CPU"
                );
                return;
            }
            // Safe to unwrap, as we just checked that the queue is not empty.
            let oldest_pending_session = self.pending_for_computation_order.pop_front().unwrap();
            // Safe to unwarp since the session was ready to compute.
            let live_session = self
                .mpc_sessions
                .get(&oldest_pending_session.session_identifier)
                .unwrap();
            if live_session.status != MPCSessionStatus::Active {
                info!(
                    session_identifier=?oldest_pending_session.session_identifier,
                    "Session is not active, skipping"
                );
                continue;
            }
            let Some(mpc_event_data) = oldest_pending_session.mpc_event_data.clone() else {
                // This should never happen.
                error!(
                    session_identifier=?oldest_pending_session.session_identifier,
                    last_session_to_complete_in_current_epoch=?self.last_session_to_complete_in_current_epoch,
                    "session does not have event data, skipping"
                );
                continue;
            };

            let should_advance = match mpc_event_data.session_type {
                SessionType::User { sequence_number } => {
                    sequence_number <= self.last_session_to_complete_in_current_epoch
                }
                SessionType::System => true,
            };
            if !should_advance {
                debug!(
                    session_identifier=?oldest_pending_session.session_identifier,
                    last_session_to_complete_in_current_epoch=?self.last_session_to_complete_in_current_epoch,
                    "Session should not be computed yet, skipping"
                );
                self.pending_for_computation_order
                    .push_back(oldest_pending_session.clone());
                continue;
            }
            if let Err(err) = self
                .cryptographic_computations_orchestrator
                .spawn_session(&oldest_pending_session, self.dwallet_mpc_metrics.clone())
                .await
            {
                error!(
                    session_identifier=?oldest_pending_session.session_identifier,
                    last_session_to_complete_in_current_epoch=?self.last_session_to_complete_in_current_epoch,
                    mpc_protocol=?mpc_event_data.init_protocol_data,
                    error=?err,
                    "failed to spawn a cryptographic session"
                );
            }
        }
    }

    /// Returns the epoch store.
    /// Errors if the epoch was switched in the middle.
    pub(crate) fn epoch_store(&self) -> DwalletMPCResult<Arc<AuthorityPerEpochStore>> {
        self.epoch_store
            .upgrade()
            .ok_or(DwalletMPCError::EpochEnded(self.epoch_id))
    }

    /// Handles a message by forwarding it to the relevant MPC session.
    /// If the session does not exist, punish the sender.
    pub(crate) fn handle_message(&mut self, message: DWalletMPCMessage) -> DwalletMPCResult<()> {
        info!(
            session_identifier=?message.session_identifier,
            from_authority=?message.authority,
            receiving_authority=?self.epoch_store()?.name,
            crypto_round_number=?message.round_number,
            mpc_protocol=message.mpc_protocol,
            "Received an MPC message for session",
        );
        if self
            .malicious_handler
            .get_malicious_actors_names()
            .contains(&message.authority)
        {
            warn!(
                session_identifier=?message.session_identifier,
                from_authority=?message.authority,
                receiving_authority=?self.epoch_store()?.name,
                crypto_round_number=?message.round_number,
                mpc_protocol=?message.mpc_protocol,
                "Received a message for from malicious authority — ignoring",
            );
            // Ignore a malicious actor's messages.
            return Ok(());
        }

        let session = match self.mpc_sessions.entry(message.session_identifier) {
            Entry::Occupied(session) => session.into_mut(),
            Entry::Vacant(_) => {
                warn!(
                    session_identifier=?message.session_identifier,
                    from_authority=?message.authority,
                    receiving_authority=?self.epoch_store()?.name,
                    crypto_round_number=?message.round_number,
                    mpc_protocol=?message.mpc_protocol,
                    "received a message for an MPC session, which an event has not yet received for"
                );
                // This can happen if the session is not in the active sessions,
                // but we still want to store the message.
                // We will create a new session for it.
                self.push_new_mpc_session(&message.session_identifier, None);
                self.mpc_sessions
                    .get_mut(&message.session_identifier)
                    .unwrap()
            }
        };
        match session.store_message(&message) {
            Err(DwalletMPCError::MaliciousParties(malicious_parties)) => {
                error!(
                    session_identifier=?message.session_identifier,
                    from_authority=?message.authority,
                    receiving_authority=?self.epoch_store()?.name,
                    crypto_round_number=?message.round_number,
                    malicious_parties=?malicious_parties,
                    mpc_protocol=?message.mpc_protocol,
                    "Error storing message, malicious parties detected"
                );
                self.flag_parties_as_malicious(&malicious_parties)?;
                Ok(())
            }
            other => other,
        }
    }

    /// Convert the indices of the malicious parties to their addresses and store them
    /// in the malicious actors set.
    /// New messages from these parties will be ignored.
    /// Restarted for each epoch.
    fn flag_parties_as_malicious(&mut self, malicious_parties: &[PartyID]) -> DwalletMPCResult<()> {
        let malicious_parties_names =
            party_ids_to_authority_names(malicious_parties, &*self.epoch_store()?)?;
        warn!(
            "dWallet MPC flagged the following parties as malicious: {:?}",
            malicious_parties_names
        );

        self.malicious_handler
            .report_malicious_actors(&malicious_parties_names);
        Ok(())
    }

    /// Spawns a new MPC session if the number of active sessions is below the limit.
    /// Otherwise, add the session to the pending queue.
    pub(super) fn push_new_mpc_session(
        &mut self,
        session_identifier: &SessionIdentifier,
        mpc_event_data: Option<MPCEventData>,
    ) {
        info!(
            "Received start MPC flow event for session identifier {:?}",
            session_identifier
        );

        let new_session = DWalletMPCSession::new(
            self.epoch_store.clone(),
            self.consensus_adapter.clone(),
            self.epoch_id,
            MPCSessionStatus::Active,
            *session_identifier,
            self.party_id,
            self.weighted_threshold_access_structure.clone(),
            mpc_event_data,
            self.dwallet_mpc_metrics.clone(),
        );
        info!(
            // todo(zeev): add metadata.
            last_session_to_complete_in_current_epoch=?self.last_session_to_complete_in_current_epoch,
            "Adding MPC session to active sessions",
        );
        self.mpc_sessions.insert(*session_identifier, new_session);
    }

    pub(super) async fn must_get_next_active_committee(&self) -> Committee {
        self.next_epoch_committee_receiver
            .clone()
            .wait_for(|committee| committee.epoch == self.epoch_id + 1)
            .await
            .expect("next epoch committee channel got closed unexpectedly")
            .clone()
    }
}
