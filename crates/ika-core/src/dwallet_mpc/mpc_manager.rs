use crate::authority::authority_per_epoch_store::{
    AuthorityPerEpochStore, ConsensusCertificateResult, ConsensusCommitOutput,
};
use crate::consensus_adapter::SubmitToConsensus;
use ika_types::error::{IkaError, IkaResult};
use sui_types::base_types::ObjectID;

use crate::dwallet_mpc::cryptographic_computations_orchestrator::{
    ComputationUpdate, CryptographicComputationsOrchestrator,
};
use crate::dwallet_mpc::malicious_handler::{MaliciousHandler, ReportStatus};
use crate::dwallet_mpc::mpc_outputs_verifier::DWalletMPCOutputsVerifier;
use crate::dwallet_mpc::mpc_session::{AsyncProtocol, DWalletMPCSession, MPCEventData};
use crate::dwallet_mpc::network_dkg::DwalletMPCNetworkKeys;
use crate::dwallet_mpc::party_id_to_authority_name;
use crate::dwallet_mpc::sign::{
    LAST_SIGN_ROUND_INDEX, SIGN_LAST_ROUND_COMPUTATION_CONSTANT_SECONDS,
};
use crate::dwallet_mpc::{party_ids_to_authority_names, session_input_from_event};
use class_groups::DecryptionKeyShare;
use crypto_bigint::Zero;
use dwallet_classgroups_types::ClassGroupsEncryptionKeyAndProof;
use dwallet_mpc_types::dwallet_mpc::{
    DWalletMPCNetworkKeyScheme, MPCPrivateInput, MPCPrivateOutput, MPCPublicInput, MPCPublicOutput,
    MPCSessionStatus,
};
use fastcrypto::hash::HashFunction;
use fastcrypto::traits::ToFromBytes;
use futures::future::err;
use group::PartyID;
use homomorphic_encryption::AdditivelyHomomorphicDecryptionKeyShare;
use ika_config::NodeConfig;
use ika_types::committee::{EpochId, StakeUnit};
use ika_types::crypto::AuthorityName;
use ika_types::crypto::AuthorityPublicKeyBytes;
use ika_types::crypto::DefaultHash;
use ika_types::digests::Digest;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_consensus::ConsensusTransaction;
use ika_types::messages_dwallet_mpc::{
    AdvanceResult, DBSuiEvent, DWalletMPCEvent, DWalletMPCMessage, MPCProtocolInitData,
    MaliciousReport, SessionInfo, StartPresignFirstRoundEvent,
};
use itertools::Itertools;
use mpc::WeightedThresholdAccessStructure;
use serde::{Deserialize, Serialize};
use shared_crypto::intent::HashingIntentScope;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Weak};
use sui_json_rpc_types::SuiEvent;
use sui_storage::mutex_table::MutexGuard;
use sui_types::digests::TransactionDigest;
use sui_types::event::Event;
use sui_types::id::ID;
use tokio::runtime::Handle;
use tokio::sync::mpsc::UnboundedSender;
use tracing::{debug, error, info, warn};
use twopc_mpc::sign::Protocol;
use typed_store::Map;

/// The [`DWalletMPCManager`] manages MPC sessions:
/// — Keeping track of all MPC sessions,
/// — Executing all active sessions, and
/// — (De)activating sessions.
///
/// The correct way to use the manager is to create it along with all other Ika components
/// at the start of each epoch.
/// Ensuring it is destroyed when the epoch ends and providing a clean slate for each new epoch.
pub struct DWalletMPCManager {
    /// The party ID of the current authority. Based on the authority index in the committee.
    party_id: PartyID,
    /// MPC sessions that where created.
    pub(crate) mpc_sessions: HashMap<ObjectID, DWalletMPCSession>,
    pub(crate) pending_sessions: HashMap<u64, DWalletMPCSession>,
    consensus_adapter: Arc<dyn SubmitToConsensus>,
    pub(super) node_config: NodeConfig,
    epoch_store: Weak<AuthorityPerEpochStore>,
    epoch_id: EpochId,
    weighted_threshold_access_structure: WeightedThresholdAccessStructure,
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
    /// A message indicating that a session failed due to malicious parties.
    /// We can receive new messages for this session with other validators,
    /// and re-run the round again to make it succeed.
    /// AuthorityName is the name of the authority that reported the malicious parties.
    SessionFailedWithMaliciousParties(AuthorityName, MaliciousReport),
}

struct ReadySessionsResponse {
    ready_sessions: Vec<DWalletMPCSession>,
    pending_for_event_sessions: Vec<DWalletMPCSession>,
    malicious_actors: Vec<PartyID>,
}

impl DWalletMPCManager {
    pub fn try_new(
        consensus_adapter: Arc<dyn SubmitToConsensus>,
        epoch_store: Arc<AuthorityPerEpochStore>,
        epoch_id: EpochId,
        node_config: NodeConfig,
    ) -> DwalletMPCResult<Self> {
        let weighted_threshold_access_structure =
            epoch_store.get_weighted_threshold_access_structure()?;
        let mpc_computations_orchestrator =
            CryptographicComputationsOrchestrator::try_new(&epoch_store)?;
        Ok(Self {
            mpc_sessions: HashMap::new(),
            pending_sessions: Default::default(),
            consensus_adapter,
            party_id: epoch_store.authority_name_to_party_id(&epoch_store.name.clone())?,
            epoch_store: Arc::downgrade(&epoch_store),
            epoch_id,
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
        })
    }

    pub(crate) fn update_last_session_to_complete_in_current_epoch(
        &mut self,
        last_session_to_complete_in_current_epoch: u64,
    ) {
        if last_session_to_complete_in_current_epoch
            <= self.last_session_to_complete_in_current_epoch
        {
            return;
        }
        for session_sequence_number in self.last_session_to_complete_in_current_epoch
            ..=last_session_to_complete_in_current_epoch
        {
            if let Some(session) = self.pending_sessions.remove(&session_sequence_number) {
                info!(session_sequence_number=?session_sequence_number,
                      new_last_session_to_complete_in_current_epoch=?last_session_to_complete_in_current_epoch,
                     "Adding session sequence number to active sessions");

                self.mpc_sessions.insert(session.session_id, session);
            }
        }
        self.last_session_to_complete_in_current_epoch = last_session_to_complete_in_current_epoch;
    }

    pub(crate) async fn handle_dwallet_db_event(&mut self, event: DWalletMPCEvent) {
        if let Err(err) = self.handle_event(event.event, event.session_info).await {
            error!("failed to handle event with error: {:?}", err);
        }
    }

    pub(crate) async fn handle_dwallet_db_message(&mut self, message: DWalletMPCDBMessage) {
        match message {
            DWalletMPCDBMessage::PerformCryptographicComputations => {
                self.perform_cryptographic_computation();
            }
            DWalletMPCDBMessage::Message(message) => {
                if let Err(err) = self.handle_message(message.clone()) {
                    error!(
                        ?err,
                        session_id=?message.session_id,
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
            DWalletMPCDBMessage::SessionFailedWithMaliciousParties(authority_name, report) => {
                if let Err(err) = self
                    .handle_session_failed_with_malicious_parties_message(authority_name, report)
                {
                    error!(
                        "dWallet MPC session failed with malicious parties with error: {:?}",
                        err
                    );
                }
            }
        }
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

    fn handle_session_failed_with_malicious_parties_message(
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
        let epoch_store = self.epoch_store()?;
        let status = self
            .malicious_handler
            .report_malicious_actor(report.clone(), reporting_authority)?;

        match status {
            // Quorum reached, remove the malicious parties from the session messages.
            ReportStatus::QuorumReached => {
                if report.advance_result == AdvanceResult::Success {
                    // No need to re-perform the last step, as the advance was successful.
                    return Ok(());
                }
                if let Some(mut session) = self.mpc_sessions.get_mut(&report.session_id) {
                    // For every advance we increase the round number by 1,
                    // so to re-run the same round we decrease it by 1.
                    session.pending_quorum_for_highest_round_number -= 1;
                    // Remove malicious parties from the session messages.
                    let round_messages = session
                        .serialized_full_messages
                        .get_mut(session.pending_quorum_for_highest_round_number)
                        .ok_or(DwalletMPCError::MPCSessionNotFound {
                            session_id: report.session_id,
                        })?;

                    self.malicious_handler
                        .get_malicious_actors_ids(epoch_store)?
                        .iter()
                        .for_each(|malicious_actor| {
                            round_messages.remove(malicious_actor);
                        });
                }
            }
            ReportStatus::WaitingForQuorum => {}
            ReportStatus::OverQuorum => {}
        }

        Ok(())
    }

    async fn handle_event(
        &mut self,
        event: DBSuiEvent,
        session_info: SessionInfo,
    ) -> DwalletMPCResult<()> {
        let (public_input, private_input) = session_input_from_event(event, &self).await?;
        let mpc_event_data = Some(MPCEventData {
            init_protocol_data: session_info.mpc_round.clone(),
            public_input,
            private_input,
            decryption_share: match session_info.mpc_round {
                MPCProtocolInitData::Sign(init_event) => self
                    .get_decryption_key_shares(&init_event.event_data.dwallet_mpc_network_key_id)?,
                _ => HashMap::new(),
            },
        });
        if let Some(mut session) = self.mpc_sessions.get_mut(&session_info.session_id) {
            warn!(
                "received an event for an existing session with `session_id`: {:?}",
                session_info.session_id
            );
            if session.mpc_event_data.is_none() {
                session.mpc_event_data = mpc_event_data;
            }
        } else if let Some(mut session) =
            self.pending_sessions.get_mut(&session_info.sequence_number)
        {
            warn!(
                "received an event for an existing pending session with `session_id`: {:?}",
                session_info.session_id
            );
            if session.mpc_event_data.is_none() {
                session.mpc_event_data = mpc_event_data;
            }
            if session_info.is_immediate {
                let _ = session;
                let session = self
                    .pending_sessions
                    .remove(&session_info.sequence_number)
                    // Safe to unwrap,
                    // as we just checked
                    // that the session exists in the pending sessions.
                    .unwrap();
                self.mpc_sessions
                    .insert(session_info.session_id, session.clone());
            }
        } else {
            if session_info.is_immediate {
                self.push_mpc_immediate_session(
                    &session_info.session_id,
                    mpc_event_data,
                    session_info.sequence_number,
                );
            } else {
                self.push_new_mpc_session(
                    &session_info.session_id,
                    mpc_event_data,
                    session_info.sequence_number,
                );
            }
        }
        Ok(())
    }

    pub(crate) async fn get_protocol_public_parameters(
        &self,
        key_id: &ObjectID,
        key_scheme: DWalletMPCNetworkKeyScheme,
    ) -> Vec<u8> {
        loop {
            if let Ok(dwallet_mpc_network_keys) = self.dwallet_mpc_network_keys() {
                if let Ok(protocol_public_parameters) = dwallet_mpc_network_keys
                    .get_protocol_public_parameters(key_id, key_scheme)
                    .await
                {
                    return protocol_public_parameters;
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    }

    fn dwallet_mpc_network_keys(&self) -> DwalletMPCResult<Arc<DwalletMPCNetworkKeys>> {
        Ok(self
            .epoch_store()?
            .dwallet_mpc_network_keys
            .get()
            .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionKeyShares)?
            .clone())
    }

    pub(super) fn get_decryption_key_share_public_parameters(
        &self,
        key_id: &ObjectID,
    ) -> DwalletMPCResult<Vec<u8>> {
        self.dwallet_mpc_network_keys()?
            .get_decryption_public_parameters(key_id)
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
        let decryption_shares = self
            .dwallet_mpc_network_keys()?
            .get_decryption_key_share(key_id.clone())?;

        Ok(decryption_shares)
    }

    /// Returns the sessions that can perform the next cryptographic round, and the list of malicious parties that has
    /// been detected while checking for such sessions.
    fn get_ready_to_advance_sessions(&mut self) -> DwalletMPCResult<ReadySessionsResponse> {
        let quorum_check_results: Vec<(DWalletMPCSession, Vec<PartyID>)> = self
            .mpc_sessions
            .iter_mut()
            .filter_map(|(_, ref mut session)| {
                let quorum_check_result = session.check_quorum_for_next_crypto_round();
                if quorum_check_result.is_ready {
                    // We must first clone the session, as we approve to advance the current session
                    // in the current round and then start waiting for the next round's messages
                    // until it is ready to advance or finalized.
                    session.pending_quorum_for_highest_round_number += 1;
                    let mut session_clone = session.clone();
                    session_clone
                        .serialized_full_messages
                        .truncate(session.pending_quorum_for_highest_round_number);
                    Some((session_clone, quorum_check_result.malicious_parties))
                } else {
                    None
                }
            })
            .collect();

        let malicious_parties: Vec<PartyID> = quorum_check_results
            .clone()
            .into_iter()
            .map(|(_, malicious_parties)| malicious_parties)
            .flatten()
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
    pub(crate) fn perform_cryptographic_computation(&mut self) {
        for (index, pending_for_event_session) in
            self.pending_for_events_order.clone().iter().enumerate()
        {
            let Some(live_session) = self.mpc_sessions.get(&pending_for_event_session.session_id)
            else {
                // This should never happen
                continue;
            };
            if live_session.mpc_event_data.is_some() {
                info!(
                    session_id=?pending_for_event_session.session_id,
                    "Received event data for session"
                );
                let mut ready_to_advance_session = pending_for_event_session.clone();
                ready_to_advance_session.mpc_event_data = live_session.mpc_event_data.clone();
                self.pending_for_computation_order
                    .push_back(ready_to_advance_session);
                self.pending_for_events_order.remove(index);
            }
        }
        while !self.pending_for_computation_order.is_empty() {
            if !self
                .cryptographic_computations_orchestrator
                .can_spawn_session()
            {
                info!("No available CPUs for cryptographic computations, waiting for a free CPU");
                return;
            }
            let oldest_pending_session = self.pending_for_computation_order.pop_front().unwrap();
            let live_session = self
                .mpc_sessions
                .get(&oldest_pending_session.session_id)
                .unwrap();
            if live_session.status != MPCSessionStatus::Active {
                info!(
                    session_id=?oldest_pending_session.session_id,
                    "Session is not active, skipping"
                );
                continue;
            }
            if let Err(err) = self
                .cryptographic_computations_orchestrator
                .spawn_session(&oldest_pending_session)
            {
                error!("failed to spawn session with err: {:?}", err);
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
            session_id=?message.session_id,
            from_authority=?message.authority,
            receiving_authority=?self.epoch_store()?.name,
            crypto_round_number=?message.round_number,
            "Received a message for session",
        );
        if self
            .malicious_handler
            .get_malicious_actors_names()
            .contains(&message.authority)
        {
            info!(
                session_id=?message.session_id,
                from_authority=?message.authority,
                receiving_authority=?self.epoch_store()?.name,
                crypto_round_number=?message.round_number,
                "Received a message for from malicious authority",
            );
            // Ignore a malicious actor's messages.
            return Ok(());
        }
        let session = match self.mpc_sessions.get_mut(&message.session_id) {
            Some(session) => session,
            None => {
                warn!(
                    session_id=?message.session_id,
                    from_authority=?message.authority,
                    receiving_authority=?self.epoch_store()?.name,
                    crypto_round_number=?message.round_number,
                    "received a message for an MPC session ID, which an event has not yet received for"
                );
                &mut self.push_new_mpc_session(
                    &message.session_id,
                    None,
                    message.session_sequence_number,
                )
            }
        };
        match session.store_message(&message) {
            Err(DwalletMPCError::MaliciousParties(malicious_parties)) => {
                error!(
                    session_id=?message.session_id,
                    from_authority=?message.authority,
                    receiving_authority=?self.epoch_store()?.name,
                    crypto_round_number=?message.round_number,
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

    /// Flags the given authorities as malicious.
    /// Future messages from these authorities will be ignored.
    pub(crate) fn flag_authorities_as_malicious(&mut self, malicious_parties: &[AuthorityName]) {
        self.malicious_handler
            .report_malicious_actors(&malicious_parties);
    }

    /// Spawns a new MPC session if the number of active sessions is below the limit.
    /// Otherwise, add the session to the pending queue.
    pub(super) fn push_new_mpc_session(
        &mut self,
        session_id: &ObjectID,
        mpc_event_data: Option<MPCEventData>,
        session_sequence_number: u64,
    ) -> DWalletMPCSession {
        if self.mpc_sessions.contains_key(&session_id) {
            // This can happpen because the event will be loaded once from the `load_missed_events` function,
            // and once by querying the events from Sui.
            // These sessions are ignored since we already have them in the `mpc_sessions` map.
            warn!(
                "received start flow event for session ID {:?} that already exists",
                &session_id
            );
        }
        info!(
            "Received start MPC flow event for session ID {:?}",
            session_id
        );

        let new_session = DWalletMPCSession::new(
            self.epoch_store.clone(),
            self.consensus_adapter.clone(),
            self.epoch_id,
            MPCSessionStatus::Active,
            session_id.clone(),
            self.party_id,
            self.weighted_threshold_access_structure.clone(),
            mpc_event_data,
            session_sequence_number,
        );
        if session_sequence_number <= self.last_session_to_complete_in_current_epoch {
            info!(
                session_sequence_number=?session_sequence_number,
                last_session_to_complete_in_current_epoch=?self.last_session_to_complete_in_current_epoch,
                "Adding MPC session to active sessions",
            );
            self.mpc_sessions
                .insert(session_id.clone(), new_session.clone());
        } else {
            info!(
                session_sequence_number=?session_sequence_number,
                last_session_to_complete_in_current_epoch=?self.last_session_to_complete_in_current_epoch,
                "Adding MPC session to pending sessions, as its sequence number is too high",
            );
            self.pending_sessions
                .insert(session_sequence_number, new_session.clone());
        }
        new_session
    }

    /// Spawns a new MPC session immediately.
    pub(super) fn push_mpc_immediate_session(
        &mut self,
        session_id: &ObjectID,
        mpc_event_data: Option<MPCEventData>,
        session_sequence_number: u64,
    ) -> DWalletMPCSession {
        if self.mpc_sessions.contains_key(&session_id) {
            // This can happen because the event will be loaded once from the `load_missed_events`
            // function, and once by querying the events from Sui.
            // These sessions are ignored since we already have them in the `mpc_sessions` map.
            warn!(
                "received start flow event for an immediate session ID {:?} that already exists",
                &session_id
            );
            // Unwrap is safe since we just checked that the session exists in the map.
            return self.mpc_sessions.get(session_id).unwrap().clone();
        }
        info!(
            "Received start MPC flow event for immediate session ID: {:?}",
            session_id
        );

        let new_session = DWalletMPCSession::new(
            self.epoch_store.clone(),
            self.consensus_adapter.clone(),
            self.epoch_id,
            MPCSessionStatus::Active,
            session_id.clone(),
            self.party_id,
            self.weighted_threshold_access_structure.clone(),
            mpc_event_data,
            session_sequence_number,
        );
        info!(
            session_sequence_number=?session_sequence_number,
            last_session_to_complete_in_current_epoch=?self.last_session_to_complete_in_current_epoch,
            "Adding MPC an immediate session to active sessions",
        );
        self.mpc_sessions
            .insert(session_id.clone(), new_session.clone());
        new_session
    }
}
