mod event_handling;

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
use crate::dwallet_mpc::party_ids_to_authority_names;
use crate::stake_aggregator::StakeAggregator;
use class_groups::Secp256k1DecryptionKeySharePublicParameters;
use dwallet_classgroups_types::ClassGroupsKeyPairAndProof;
use dwallet_mpc_types::dwallet_mpc::{MPCSessionStatus, NetworkDecryptionKeyPublicData, VersionedNetworkDkgOutput};
use dwallet_rng::RootSeed;
use group::PartyID;
use ika_config::NodeConfig;
use ika_types::committee::ClassGroupsEncryptionKeyAndProof;
use ika_types::committee::{Committee, EpochId};
use ika_types::crypto::AuthorityName;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_dwallet_mpc::{
    DBSuiEvent, DWalletMPCEvent, DWalletMPCMessage, MaliciousReport, SessionIdentifier,
    SessionInfo, SessionType, ThresholdNotReachedReport,
};
use mpc::WeightedThresholdAccessStructure;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Weak};
use group::helpers::DeduplicateAndSort;
use itertools::Itertools;
use tokio::sync::watch;
use tokio::sync::watch::Receiver;
use tracing::{debug, error, info, warn};

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
    /// A map of all MPC sessions that start execution in this epoch.
    /// These include completed sessions, and they are never to be removed from this
    /// mapping until the epoch advances.
    pub(crate) mpc_sessions: HashMap<SessionIdentifier, DWalletMPCSession>,
    consensus_adapter: Arc<dyn SubmitToConsensus>,
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
    pub(crate) ordered_sessions_pending_for_computation: VecDeque<DWalletMPCSession>,
    /// The order of the sessions that have received quorum for their current round, but we have not
    /// yet received an event for from Sui.
    pub(crate) sessions_pending_for_events: VecDeque<DWalletMPCSession>,
    pub(crate) last_session_to_complete_in_current_epoch: u64,
    pub(crate) recognized_self_as_malicious: bool,
    pub(crate) network_keys: Box<DwalletMPCNetworkKeys>,
    network_keys_receiver: Receiver<Arc<HashMap<ObjectID, NetworkDecryptionKeyPublicData>>>,
    /// Events that wait for the network key to update.
    /// Once we get the network key, these events will continue.
    pub(crate) events_pending_for_network_key: HashMap<ObjectID, Vec<DWalletMPCEvent>>,
    pub(crate) events_pending_for_next_active_committee: Vec<DWalletMPCEvent>,
    pub(crate) next_epoch_committee_receiver: watch::Receiver<Committee>,
    pub(crate) next_active_committee: Option<Committee>,
    pub(crate) dwallet_mpc_metrics: Arc<DWalletMPCMetrics>,
    pub(crate) threshold_not_reached_reports:
        HashMap<ThresholdNotReachedReport, StakeAggregator<(), true>>,

    /// The root seed of this validator, used for deriving the session and round-specific seed for advancing MPC sessions.
    /// SECURITY NOTICE: *MUST KEEP PRIVATE*.
    root_seed: RootSeed,
}

/// The messages that the [`DWalletMPCManager`] can receive and process asynchronously.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DWalletMPCDBMessage {
    /// An MPC message from another validator.
    Message(DWalletMPCMessage),
    /// A message indicating that an MPC session has failed.
    /// The advance failed, and the session needs to be restarted or marked as failed.
    MPCSessionFailed(ObjectID),

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
        network_keys_receiver: Receiver<Arc<HashMap<ObjectID, NetworkDecryptionKeyPublicData>>>,
        next_epoch_committee_receiver: Receiver<Committee>,
        node_config: NodeConfig,
        dwallet_mpc_metrics: Arc<DWalletMPCMetrics>,
    ) -> Self {
        Self::try_new(
            consensus_adapter.clone(),
            epoch_store.clone(),
            network_keys_receiver,
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
        network_keys_receiver: Receiver<Arc<HashMap<ObjectID, NetworkDecryptionKeyPublicData>>>,
        next_epoch_committee_receiver: watch::Receiver<Committee>,
        node_config: NodeConfig,
        dwallet_mpc_metrics: Arc<DWalletMPCMetrics>,
    ) -> DwalletMPCResult<Self> {
        let weighted_threshold_access_structure =
            epoch_store.get_weighted_threshold_access_structure()?;
        let mpc_computations_orchestrator = CryptographicComputationsOrchestrator::try_new()?;
        let party_id = epoch_store.authority_name_to_party_id(&epoch_store.name)?;
        let root_seed = node_config
            .root_seed
            .clone()
            .ok_or(DwalletMPCError::MissingRootSeed)?
            .root_seed()
            .clone();
        let class_groups_decryption_key =
            ClassGroupsKeyPairAndProof::from_seed(&root_seed).decryption_key();

        // TODO(Scaly): It's weird that `validator_decryption_key_shares` is a hash-map that can be empty.
        // It should never be empty, and we should use an Option and None to describe a state where it doesn't exist yet.
        let validator_private_data = ValidatorPrivateDecryptionKeyData {
            party_id,
            class_groups_decryption_key,
            validator_decryption_key_shares: HashMap::new(),
        };
        let dwallet_network_keys = DwalletMPCNetworkKeys::new(validator_private_data);

        // Re-initialize the malicious handler every epoch. This is done intentionally:
        // we want to "forget" the malicious actors from the previous epoch and start from scratch.
        let malicious_handler = MaliciousHandler::new(epoch_store.committee().clone());
        Ok(Self {
            mpc_sessions: HashMap::new(),
            consensus_adapter,
            party_id: epoch_store.authority_name_to_party_id(&epoch_store.name.clone())?,
            epoch_store: Arc::downgrade(&epoch_store),
            epoch_id: epoch_store.epoch(),
            weighted_threshold_access_structure,
            validators_class_groups_public_keys_and_proofs: epoch_store
                .get_validators_class_groups_public_keys_and_proofs()
                .map_err(|e| DwalletMPCError::MPCManagerError(e.to_string()))?,
            cryptographic_computations_orchestrator: mpc_computations_orchestrator,
            malicious_handler,
            ordered_sessions_pending_for_computation: VecDeque::new(),
            sessions_pending_for_events: Default::default(),
            last_session_to_complete_in_current_epoch: 0,
            recognized_self_as_malicious: false,
            network_keys: Box::new(dwallet_network_keys),
            network_keys_receiver,
            next_epoch_committee_receiver,
            events_pending_for_next_active_committee: Vec::new(),
            events_pending_for_network_key: HashMap::new(),
            dwallet_mpc_metrics,
            threshold_not_reached_reports: Default::default(),
            next_active_committee: None,
            root_seed,
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

    /// Handle an incoming dWallet MPC message, coming from storage, either during bootstrapping or indirectly originating from the consensus
    /// (which writes the messages to the storage, from which we read them in the dWallet MPC Service and call this function.)
    pub(crate) fn handle_dwallet_message(&mut self, message: DWalletMPCDBMessage) {
        match message {
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
            DWalletMPCDBMessage::MPCSessionFailed(session_id) => {
                error!(session_id=?session_id, "dwallet MPC session failed");
                // TODO(@scaly) this is the wrong issue, also create a new one.
                // Also this doesn't get sent or handled, so what?
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

            if let Some(unreached_round_messages) = session
                .serialized_full_messages
                .remove(&(session.current_round - 1))
            {
                let malicious_parties = unreached_round_messages
                    .keys()
                    .cloned()
                    .collect::<Vec<PartyID>>();

                let malicious_authorities =
                    party_ids_to_authority_names(&malicious_parties, &epoch_store)?;

                self.malicious_handler
                    .report_malicious_actors(&malicious_authorities);
            }

            // Decrement the current round, as we are going to retry the previous round.
            session.current_round -= 1;
        }

        Ok(())
    }

    /// Handle the messages of a given consensus round.
    pub fn handle_consensus_round_messages(&mut self, messages: Vec<DWalletMPCDBMessage>) -> IkaResult {
        // We only update `next_active_committee` here, so once we update it there is no longer going to be any pending events for it in this epoch.
        if self.next_active_committee.is_none() {
            let got_next_active_committee = self.try_receiving_next_active_committee();
            if got_next_active_committee {
                // `..` stands for `RangeFull`, and calling `drain(..)` will drain (i.e. mutate) the vector thus removing all events from it.
                self.events_pending_for_next_active_committee.drain(..)
                    .for_each(|event| {
                    self.handle_dwallet_db_event(event);
                });
            }
        }

        // Check if we just got the public data for some network keys, and handle those events if so.
        self.events_pending_for_network_key.keys().filter(|key_id| self.network_keys.key_public_data_exists(key_id))
            .flat_map(|key_id| self.events_pending_for_network_key.remove(key_id)).flatten().for_each(|event| {
            self.handle_dwallet_db_event(event);
        });

        for message in messages {
            self.handle_dwallet_message(message);
        }

        // Check for ready to advance sessions, and clone and place their copy in an ordered queue waiting for computation.
        let ready_sessions_response = self.get_ready_to_advance_sessions()?;
        if !ready_sessions_response.malicious_actors.is_empty() {
            self.flag_parties_as_malicious(&ready_sessions_response.malicious_actors)?;
        }

        // Note that because the Ika consensus isn't in sync with the Sui consensus, it might be that a session has gotten quorum of messages whilst
        // the current validator haven't received the event from which its public input can be generated (and therefore cannot advance it yet).
        //
        // Because of this reason, we place these on two separate queues. Note that in either cases, we must use the copy at this point in time,
        // so that we will advance it with exactly the same messages as those who already have their event data ready.
        self.sessions_pending_for_events
            .extend(ready_sessions_response.pending_for_event_sessions);

        // Extend the pending for computation queue while keeping order.
        for ready_to_advance_session_copy in ready_sessions_response.ready_sessions {
            self.insert_session_into_ordered_pending_for_computation_queue(ready_to_advance_session_copy);
        }

        Ok(())
    }

    /// Handle an incoming malicious `report` from `reporting_authority`,
    /// and recognize ourselves as malicious in the case of a bug.
    fn handle_malicious_report(
        &mut self,
        reporting_authority: AuthorityName,
        report: MaliciousReport,
    ) -> DwalletMPCResult<()> {
        self.malicious_handler
            .report_malicious_actor(report.clone(), reporting_authority);

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

    fn new_mpc_event_data(
        &self,
        event: DBSuiEvent,
        session_info: &SessionInfo,
        next_active_committee: Option<Committee>
    ) -> Result<MPCEventData, DwalletMPCError> {
        let epoch_store = self.epoch_store()?;

        MPCEventData::try_new(
            event,
            session_info,
            epoch_store,
            &self.network_keys,
            next_active_committee,
            self.validators_class_groups_public_keys_and_proofs.clone(),
        )
    }

    /// Returns the sessions that can perform the next cryptographic round,
    /// and the list of malicious parties that has
    /// been detected while checking for such sessions.
    fn get_ready_to_advance_sessions(&mut self) -> DwalletMPCResult<ReadySessionsResponse> {
        let (ready_to_advance_sessions, malicious_parties) : (Vec<DWalletMPCSession>, Vec<Vec<PartyID>>) = self
            .mpc_sessions
            .iter_mut()
            .filter_map(|(_, ref mut session)| {
                let quorum_check_result = session.check_quorum_for_next_crypto_round().ok()?;
                if quorum_check_result.is_ready {
                    // We must first clone the session, as we approve to advance the current session
                    // in the current round and then start waiting for the next round's messages
                    // until it is ready to advance or finalized.
                    let session_clone = session.clone();

                    // Mutate the session stored in `mpc_sessions` to reflect the fact we have called `advance()` on this round,
                    // and prepare for the next round.
                    session.current_round += 1;
                    session.received_more_messages_since_last_advance = false;

                    Some((session_clone, quorum_check_result.malicious_parties))
                } else {
                    None
                }
            })
            .unzip();

        let malicious_parties: Vec<PartyID> = malicious_parties
            .into_iter()
            .flatten()
            .deduplicate_and_sort();

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
        let pending_for_computation = self.ordered_sessions_pending_for_computation.len();
        for _ in 0..pending_for_computation {
            if !self
                .cryptographic_computations_orchestrator
                .can_spawn_session()
            {
                warn!("No available CPUs for cryptographic computations, waiting for a free CPU");
                return;
            }
            // Safe to unwrap, as we just checked that the queue is not empty.
            let oldest_pending_session = self.ordered_sessions_pending_for_computation.pop_front().unwrap();
            // Safe to unwarp since the session was ready to compute.
            let live_session = self
                .mpc_sessions
                .get(&oldest_pending_session.session_identifier)
                .unwrap();

            // TODO(Scaly): What does it even mean for a session to be non-active, if its already ready for advance?
            // TODO - if its finished, it shouldn't be here, and also failed what??
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
                info!(
                    session_identifier=?oldest_pending_session.session_identifier,
                    last_session_to_complete_in_current_epoch=?self.last_session_to_complete_in_current_epoch,
                    "Session should not be computed yet, skipping"
                );
                self.ordered_sessions_pending_for_computation
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
                self.new_mpc_session(&message.session_identifier, None);
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

    /// Creates a new session with SID `session_identifier`,
    /// and insert it into the MPC session map `self.mpc_sessions`.
    pub(super) fn new_mpc_session(
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
            self.root_seed.clone(),
        );
        info!(
            // todo(zeev): add metadata.
            last_session_to_complete_in_current_epoch=?self.last_session_to_complete_in_current_epoch,
            "Adding MPC session to active sessions",
        );
        self.mpc_sessions.insert(*session_identifier, new_session);
    }

    fn try_receiving_next_active_committee(&mut self) -> bool {
        match self.next_epoch_committee_receiver.has_changed() {
            Ok(has_changed) => {
                if has_changed {
                    let committee = self.next_epoch_committee_receiver.borrow_and_update().clone();

                    if committee.epoch == self.epoch_id + 1 {
                        self.next_active_committee = Some(committee);

                        return true;
                    }
                }
                }
            Err(err) => {
                error!(?err, "failed to check next epoch committee receiver");
            }
        }

        false
    }

    fn update_network_keys(&mut self) {
        match self.network_keys_receiver.has_changed() {
            Ok(has_changed) => {
                if has_changed {
                    let new_keys = self.network_keys_receiver.borrow_and_update();
                    for (key_id, key_data) in new_keys.iter() {
                        info!(
                            "Updating (decrypting new shares) network key for key_id: {:?}",
                            key_id
                        );
                        self
                            .network_keys
                            .update_network_key(
                                *key_id,
                                key_data,
                                &self.weighted_threshold_access_structure,
                            )
                            .unwrap_or_else(|err| error!(?err, "failed to store network keys"));
                    }
                }
            }
            Err(err) => {
                error!(?err, "failed to check network keys receiver");
            }
        }
    }

    /// Insert `session` into `self.ordered_sessions_pending_for_computation`, keeping order:
    /// System sessions come first, and user sessions are sorted by their sequence number.
    ///
    /// Note: at this point, `mpc_event_data` must be set!
    fn insert_session_into_ordered_pending_for_computation_queue(&mut self, session: DWalletMPCSession) {
        let sequence_number = match session.mpc_event_data.as_ref().unwrap().session_type {
            SessionType::User { sequence_number } => {
                Some(sequence_number)
            }
            SessionType::System => None,
        };

        if let Some(index) = self.ordered_sessions_pending_for_computation.iter().position(|session_pending_for_computation| {
            match session_pending_for_computation.mpc_event_data.as_ref().unwrap().session_type {
                SessionType::User { sequence_number: pending_session_sequence_number } => {
                    if let Some(sequence_number) = sequence_number {
                        // Find the first pending session with a sequence number greater than the new session,
                        // so we can insert the new session right before it.
                        pending_session_sequence_number > sequence_number
                    } else {
                        // System session takes precedence over user sessions.
                        true
                    }
                }
                SessionType::System => {
                    // Existing system sessions take precedence over both new system sessions and user sessions.
                    false
                },
            }
        }) {
            self.ordered_sessions_pending_for_computation.insert(index, session);
        } else {
            // All existing pending sessions take precedence over the new one, so push it back.
            self.ordered_sessions_pending_for_computation.push_back(session);
        }
    }
}
