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
use crate::dwallet_mpc::mpc_events::ValidatorDataForNetworkDKG;
use crate::dwallet_mpc::mpc_outputs_verifier::DWalletMPCOutputsVerifier;
use crate::dwallet_mpc::mpc_session::{AsyncProtocol, DWalletMPCSession, MPCEventData};
use crate::dwallet_mpc::network_dkg::DwalletMPCNetworkKeysStatus;
use crate::dwallet_mpc::sign::{
    LAST_SIGN_ROUND_INDEX, SIGN_LAST_ROUND_COMPUTATION_CONSTANT_SECONDS,
};
use crate::dwallet_mpc::{authority_name_to_party_id, party_id_to_authority_name};
use crate::dwallet_mpc::{party_ids_to_authority_names, session_input_from_event};
use class_groups::DecryptionKeyShare;
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
    AdvanceResult, DBSuiEvent, DWalletMPCEvent, DWalletMPCLocalComputationMetadata,
    DWalletMPCMessage, MPCProtocolInitData, MPCSessionSpecificState, MaliciousReport, SessionInfo,
    SignIASessionState, StartPresignFirstRoundEvent,
};
use mpc::WeightedThresholdAccessStructure;
use rayon::ThreadPoolBuilder;
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
use tracing::log::debug;
use tracing::{error, info, warn};
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
    /// Used to keep track of the order in which pending sessions are received,
    /// so they are activated in order of arrival.
    pending_sessions_queue: VecDeque<DWalletMPCSession>,
    // TODO (#257): Make sure the counter is always in sync with the number of active sessions.
    /// Keep track of the active sessions to avoid exceeding the limit.
    /// We can't use the length of `mpc_sessions` since it contains both active and inactive sessions.
    active_sessions_counter: usize,
    consensus_adapter: Arc<dyn SubmitToConsensus>,
    pub(super) node_config: NodeConfig,
    epoch_store: Weak<AuthorityPerEpochStore>,
    max_active_mpc_sessions: usize,
    epoch_id: EpochId,
    weighted_threshold_access_structure: WeightedThresholdAccessStructure,
    pub(crate) validators_data_for_network_dkg: HashMap<PartyID, ValidatorDataForNetworkDKG>,
    pub(crate) cryptographic_computations_orchestrator: CryptographicComputationsOrchestrator,
    /// A struct for managing malicious actors in MPC protocols.
    /// This struct maintains a record of malicious actors reported by validators.
    /// An actor is deemed malicious if it is reported by a quorum of validators.
    /// Any message/output from these authorities will be ignored.
    /// This list is maintained during the Epoch.
    /// This happens automatically because the [`DWalletMPCManager`]
    /// is part of the [`AuthorityPerEpochStore`].
    pub(crate) malicious_handler: MaliciousHandler,
}

/// The messages that the [`DWalletMPCManager`] can receive and process asynchronously.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DWalletMPCDBMessage {
    /// An MPC message from another validator.
    Message(DWalletMPCMessage),
    /// Signal delivery of messages has ended,
    /// now the sessions that received a quorum of messages can advance.
    EndOfDelivery,
    /// A validator's public key and proof for the network DKG protocol.
    /// Each validator's data is being emitted separately because the proof size is
    /// almost 250 KB, which is the maximum event size in Sui.
    /// The manager accumulates the data until it receives such an event for all validators,
    /// and then it starts the network DKG protocol.
    ValidatorDataForDKG(ValidatorDataForNetworkDKG),
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

impl DWalletMPCManager {
    pub fn try_new(
        consensus_adapter: Arc<dyn SubmitToConsensus>,
        epoch_store: Arc<AuthorityPerEpochStore>,
        epoch_id: EpochId,
        node_config: NodeConfig,
    ) -> DwalletMPCResult<Self> {
        let weighted_threshold_access_structure =
            epoch_store.get_weighted_threshold_access_structure()?;
        let quorum_threshold = epoch_store.committee().quorum_threshold();
        let weighted_parties = epoch_store
            .committee()
            .voting_rights
            .iter()
            .cloned()
            .collect();
        let mpc_computations_orchestrator =
            CryptographicComputationsOrchestrator::try_new(&epoch_store)?;
        Ok(Self {
            mpc_sessions: HashMap::new(),
            pending_sessions_queue: VecDeque::new(),
            active_sessions_counter: 0,
            consensus_adapter,
            party_id: authority_name_to_party_id(&epoch_store.name.clone(), &epoch_store.clone())?,
            epoch_store: Arc::downgrade(&epoch_store),
            epoch_id,
            max_active_mpc_sessions: 200, //todo (yael): Ask sadika what about this . node_config.max_active_dwallet_mpc_sessions,
            node_config,
            weighted_threshold_access_structure,
            validators_data_for_network_dkg: HashMap::new(),
            cryptographic_computations_orchestrator: mpc_computations_orchestrator,
            malicious_handler: MaliciousHandler::new(quorum_threshold, weighted_parties),
        })
    }

    pub(crate) fn handle_dwallet_db_event(&mut self, event: DWalletMPCEvent) {
        if let Err(err) = self.handle_event(event.event, event.session_info) {
            error!("Failed to handle event with error: {:?}", err);
        }
    }

    pub(crate) async fn handle_dwallet_db_message(&mut self, message: DWalletMPCDBMessage) {
        match message {
            DWalletMPCDBMessage::PerformCryptographicComputations => {
                self.perform_cryptographic_computation();
            }
            DWalletMPCDBMessage::Message(message) => {
                if let Err(err) = self.handle_message(message) {
                    error!("failed to handle an MPC message with error: {:?}", err);
                }
            }
            DWalletMPCDBMessage::EndOfDelivery => {
                if let Err(err) = self.handle_end_of_delivery().await {
                    error!("failed to handle the end of delivery with error: {:?}", err);
                }
            }
            DWalletMPCDBMessage::ValidatorDataForDKG(data) => {
                if let Err(err) = self.handle_validator_data_for_network_dkg(data) {
                    error!(
                        "failed to handle validator data for DKG session with error: {:?}",
                        err
                    );
                }
            }
            DWalletMPCDBMessage::MPCSessionFailed(_session_id) => {
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
        while self.active_sessions_counter < self.max_active_mpc_sessions {
            if let Some(mut session) = self.pending_sessions_queue.pop_front() {
                session.status = MPCSessionStatus::Active;
                self.mpc_sessions.insert(session.session_id, session);
                self.active_sessions_counter += 1;
            } else {
                break;
            }
        }
        self.epoch_store()?
            .dwallet_mpc_network_keys
            .get()
            .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionKeyShares)?
            .status()?;

        let (ready_to_advance, malicious_parties) = self.get_ready_to_advance_sessions()?;
        if !malicious_parties.is_empty() {
            self.flag_parties_as_malicious(&malicious_parties)?;
        }
        self.cryptographic_computations_orchestrator
            .insert_ready_sessions(ready_to_advance);
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
                let _ = self.check_for_malicious_ia_report(&report);
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
                        .serialized_messages
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
            ReportStatus::WaitingForQuorum => {
                let Some(mut session) = self.mpc_sessions.get_mut(&report.session_id) else {
                    return Err(DwalletMPCError::MPCSessionNotFound {
                        session_id: report.session_id,
                    });
                };
                session.check_for_sign_ia_start(reporting_authority, report);
            }
            ReportStatus::OverQuorum => {}
        }

        Ok(())
    }

    /// Makes sure the first agreed-upon malicious report in a sign flow is equals to the request
    /// that triggered the Sign Identifiable Abort flow. If it isn't, we mark the validator that
    /// sent the request to start the Sign Identifiable Abort flow as malicious, as he sent a faulty
    /// report.
    fn check_for_malicious_ia_report(&mut self, report: &MaliciousReport) -> DwalletMPCResult<()> {
        let Some(mut session) = self.mpc_sessions.get_mut(&report.session_id) else {
            return Err(DwalletMPCError::MPCSessionNotFound {
                session_id: report.session_id,
            });
        };
        let Some(MPCSessionSpecificState::Sign(ref mut sign_state)) =
            &mut session.session_specific_state
        else {
            return Err(DwalletMPCError::AggregatedSignStateNotFound {
                session_id: report.session_id,
            });
        };
        if sign_state.verified_malicious_report.is_none() {
            sign_state.verified_malicious_report = Some(report.clone());
            if &sign_state.start_ia_flow_malicious_report != report {
                self.malicious_handler
                    .report_malicious_actors(&vec![sign_state.initiating_ia_authority]);
            }
        }
        Ok(())
    }

    fn handle_validator_data_for_network_dkg(
        &mut self,
        data: ValidatorDataForNetworkDKG,
    ) -> DwalletMPCResult<()> {
        let epoch_store = self.epoch_store()?;
        let party_id = authority_name_to_party_id(
            &AuthorityPublicKeyBytes::from_bytes(&data.protocol_pubkey_bytes)
                .map_err(|e| DwalletMPCError::InvalidPartyPublicKey(e))?,
            &epoch_store,
        )?;
        if self.validators_data_for_network_dkg.contains_key(&party_id) {
            debug!("Received duplicate data for party_id: {:?}", party_id);
        } else {
            self.validators_data_for_network_dkg.insert(party_id, data);
        }
        Ok(())
    }

    fn handle_event(
        &mut self,
        event: DBSuiEvent,
        session_info: SessionInfo,
    ) -> DwalletMPCResult<()> {
        let (public_input, private_input) = session_input_from_event(event, &self)?;
        let mpc_event_data = Some(MPCEventData {
            init_protocol_data: session_info.mpc_round.clone(),
            public_input,
            private_input,
            decryption_share: match session_info.mpc_round {
                MPCProtocolInitData::NetworkDkg(..) => HashMap::new(),
                _ => self.get_decryption_key_shares(
                    DWalletMPCNetworkKeyScheme::Secp256k1,
                    Some(self.network_key_version(DWalletMPCNetworkKeyScheme::Secp256k1)? as usize),
                )?,
            },
        });
        self.push_new_mpc_session(&session_info.session_id, mpc_event_data)?;
        Ok(())
    }

    pub(crate) fn get_protocol_public_parameters(
        &self,
        key_scheme: DWalletMPCNetworkKeyScheme,
        key_version: u8,
    ) -> DwalletMPCResult<Vec<u8>> {
        if let Some(self_decryption_share) = self.epoch_store()?.dwallet_mpc_network_keys.get() {
            return self_decryption_share.get_protocol_public_parameters(key_scheme, key_version);
        }
        Err(DwalletMPCError::TwoPCMPCError(
            "Decryption share not found".to_string(),
        ))
    }

    pub(super) fn get_decryption_key_share_public_parameters(
        &self,
        key_scheme: DWalletMPCNetworkKeyScheme,
        key_version: u8,
    ) -> DwalletMPCResult<Vec<u8>> {
        if let Some(self_decryption_share) = self.epoch_store()?.dwallet_mpc_network_keys.get() {
            return self_decryption_share.get_decryption_public_parameters(key_scheme, key_version);
        }
        Err(DwalletMPCError::TwoPCMPCError(
            "Decryption share not found".to_string(),
        ))
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
        key_scheme: DWalletMPCNetworkKeyScheme,
        key_version: Option<usize>,
    ) -> DwalletMPCResult<HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>> {
        let epoch_store = self.epoch_store()?;

        let decryption_shares = epoch_store
            .dwallet_mpc_network_keys
            .get()
            .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionKeyShares)?
            .get_decryption_key_share(key_scheme)?;
        let key_version = match key_version {
            Some(key_version) => key_version,
            None => self.network_key_version(key_scheme)? as usize,
        };
        Ok(decryption_shares
            .get(key_version)
            .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionKeyShares)?
            .clone())
    }

    /// Returns the sessions that can perform the next cryptographic round, and the list of malicious parties that has
    /// been detected while checking for such sessions.
    fn get_ready_to_advance_sessions(
        &mut self,
    ) -> DwalletMPCResult<(Vec<DWalletMPCSession>, Vec<PartyID>)> {
        let quorum_check_results: Vec<(DWalletMPCSession, Vec<PartyID>)> = self
            .mpc_sessions
            .iter_mut()
            .filter_map(|(_, ref mut session)| {
                let quorum_check_result = session.check_quorum_for_next_crypto_round();
                if quorum_check_result.is_ready {
                    // We must first clone the session, as we approve to advance the current session
                    // in the current round and then start waiting for the next round's messages
                    // until it is ready to advance or finalized.
                    session.pending_quorum_for_highest_round_number =
                        session.pending_quorum_for_highest_round_number + 1;
                    Some((session.clone(), quorum_check_result.malicious_parties))
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
        let ready_to_advance_sessions = quorum_check_results
            .into_iter()
            .map(|(session, _)| session)
            .collect();
        Ok((ready_to_advance_sessions, malicious_parties))
    }

    /// Spawns all ready MPC cryptographic computations using Rayon.
    /// If no local CPUs are available, computations will execute as CPUs are freed.
    pub(crate) fn perform_cryptographic_computation(&mut self) {
        while self
            .cryptographic_computations_orchestrator
            .currently_running_sessions_count
            < self
            .cryptographic_computations_orchestrator
            .available_cores_for_cryptographic_computations
        {
            let Some(oldest_computation_metadata) = self
                .cryptographic_computations_orchestrator
                .pending_for_computation_order
                .pop_front()
            else {
                return;
            };
            let Some(session) = self
                .cryptographic_computations_orchestrator
                .pending_computation_map
                .remove(&oldest_computation_metadata)
            else {
                return;
            };

            if let Err(err) = self.spawn_session(&session) {
                error!("failed to spawn session with err: {:?}", err);
                return;
            }
        }
    }

    fn spawn_session(&mut self, session: &DWalletMPCSession) -> DwalletMPCResult<()> {
        let Some(mpc_event_data) = &session.mpc_event_data else {
            return Err(DwalletMPCError::MissingEventDrivenData);
        };
        let session_id = session.session_id;
        if self
            .mpc_sessions
            .get(&session_id)
            .ok_or(DwalletMPCError::MPCSessionNotFound { session_id })?
            .status
            != MPCSessionStatus::Active
        {
            return Ok(());
        }
        // Hook the tokio thread pool to the rayon thread pool.
        let handle = Handle::current();
        let session = session.clone();
        let finished_computation_sender = self
            .cryptographic_computations_orchestrator
            .computation_channel_sender
            .clone();
        if matches!(
            mpc_event_data.init_protocol_data,
            MPCProtocolInitData::Sign(..)
        ) && session.pending_quorum_for_highest_round_number == LAST_SIGN_ROUND_INDEX
        {
            self.spawn_aggregated_sign(session_id, handle, session, finished_computation_sender)?;
        } else {
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
        }
        Ok(())
    }

    fn spawn_aggregated_sign(
        &mut self,
        session_id: ObjectID,
        handle: Handle,
        ready_to_advance_session: DWalletMPCSession,
        finished_computation_sender: UnboundedSender<ComputationUpdate>,
    ) -> DwalletMPCResult<()> {
        let validator_position =
            self.get_validator_position(&ready_to_advance_session.session_id)?;
        let epoch_store = self.epoch_store()?;
        tokio::spawn(async move {
            for _ in 0..validator_position {
                let manager = epoch_store.get_dwallet_mpc_manager().await;
                let Some(session) = manager.mpc_sessions.get(&session_id) else {
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
            let manager = epoch_store.get_dwallet_mpc_manager().await;
            let Some(live_session) = manager.mpc_sessions.get(&session_id) else {
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
                session_id
            );
            let session = ready_to_advance_session.clone();
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
        });
        Ok(())
    }

    /// Update the encryption of decryption key share with the new shares.
    /// This function is called when the network DKG protocol is done.
    fn update_dwallet_mpc_network_key(
        &self,
        session_info: &SessionInfo,
        public_output: MPCPublicOutput,
        private_output: MPCPrivateOutput,
    ) -> DwalletMPCResult<()> {
        if let MPCProtocolInitData::NetworkDkg(key_type, _) = session_info.mpc_round {
            let epoch_store = self.epoch_store()?;
            let network_keys = epoch_store
                .dwallet_mpc_network_keys
                .get()
                .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionKeyShares)?;

            network_keys.add_key_version(
                epoch_store.clone(),
                key_type,
                bcs::from_bytes(&private_output)?,
                public_output,
                &self.weighted_threshold_access_structure,
            )?;
        }
        Ok(())
    }

    /// Returns the epoch store.
    /// Errors if the epoch was switched in the middle.
    pub(crate) fn epoch_store(&self) -> DwalletMPCResult<Arc<AuthorityPerEpochStore>> {
        self.epoch_store
            .upgrade()
            .ok_or(DwalletMPCError::EpochEnded(self.epoch_id))
    }

    /// Deterministically decides by the session ID how long this validator should wait before
    /// running the last step of the sign protocol.
    /// If while waiting, the validator receives a valid signature for this session,
    /// it will not run the last step in the sign protocol, and save computation resources.
    fn get_validator_position(&self, session_id: &ObjectID) -> DwalletMPCResult<usize> {
        let session_id_as_32_bytes: [u8; 32] = session_id.into_bytes();
        let positions = &self
            .epoch_store()?
            .committee()
            .shuffle_by_stake_from_tx_digest(&TransactionDigest::new(session_id_as_32_bytes));
        let authority_name = &self.epoch_store()?.name;
        let position = positions
            .iter()
            .position(|&x| x == *authority_name)
            .ok_or(DwalletMPCError::InvalidMPCPartyType)?;
        Ok(position)
    }

    /// Handles a message by forwarding it to the relevant MPC session.
    /// If the session does not exist, punish the sender.
    pub(crate) fn handle_message(&mut self, message: DWalletMPCMessage) -> DwalletMPCResult<()> {
        if self
            .malicious_handler
            .get_malicious_actors_names()
            .contains(&message.authority)
        {
            // Ignore a malicious actor's messages.
            return Ok(());
        }
        let session = match self.mpc_sessions.get_mut(&message.session_id) {
            Some(session) => session,
            None => {
                warn!(
                    "received a message for an MPC session ID: `{:?}` which does not exist",
                    message.session_id
                );
                // TODO (#693): Keep messages for non-existing sessions.
                return Ok(());
            }
        };
        match session.store_message(&message) {
            Err(DwalletMPCError::MaliciousParties(malicious_parties)) => {
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
    pub(crate) fn push_new_mpc_session(
        &mut self,
        session_id: &ObjectID,
        mpc_event_data: Option<MPCEventData>,
    ) -> DwalletMPCResult<()> {
        if self.mpc_sessions.contains_key(&session_id) {
            // This should never happen, as the session ID is a Move UniqueID.
            error!(
                "received start flow event for session ID {:?} that already exists",
                &session_id
            );
            return Ok(());
        }
        info!(
            "Received start MPC flow event for session ID {:?}",
            session_id
        );

        let mut new_session = DWalletMPCSession::new(
            self.epoch_store.clone(),
            self.consensus_adapter.clone(),
            self.epoch_id,
            MPCSessionStatus::Pending,
            session_id.clone(),
            self.party_id,
            self.weighted_threshold_access_structure.clone(),
            mpc_event_data,
        );
        // TODO (#311): Make sure validator don't mark other validators
        // TODO (#311): as malicious or take any active action while syncing
        if self.active_sessions_counter >= self.max_active_mpc_sessions {
            self.pending_sessions_queue.push_back(new_session);
            info!(
                "Added MPCSession to pending queue for session_id {:?}",
                &session_id
            );
            return Ok(());
        }
        new_session.status = MPCSessionStatus::Active;
        self.mpc_sessions.insert(session_id.clone(), new_session);
        self.active_sessions_counter += 1;
        info!(
            "Added MPCSession to MPC manager for session_id {:?}",
            session_id
        );
        Ok(())
    }

    pub(super) fn network_key_version(
        &self,
        key_type: DWalletMPCNetworkKeyScheme,
    ) -> DwalletMPCResult<u8> {
        self.epoch_store()?
            .dwallet_mpc_network_keys
            .get()
            .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionKeyShares)?
            .key_version(key_type)
    }
}
