use crate::authority::authority_per_epoch_store::{AuthorityPerEpochStore, ConsensusCommitOutput};
use crate::consensus_adapter::SubmitToConsensus;
use pera_types::base_types::{AuthorityName, ObjectID};
use pera_types::error::PeraResult;

use crate::dwallet_mpc::cryptographic_computations_orchestrator::CryptographicComputationsOrchestrator;
use crate::dwallet_mpc::malicious_handler::{MaliciousHandler, ReportStatus};
use crate::dwallet_mpc::mpc_events::ValidatorDataForNetworkDKG;
use crate::dwallet_mpc::mpc_outputs_verifier::DWalletMPCOutputsVerifier;
use crate::dwallet_mpc::mpc_session::{AsyncProtocol, DWalletMPCSession};
use crate::dwallet_mpc::network_dkg::DwalletMPCNetworkKeysStatus;
use crate::dwallet_mpc::session_input_from_event;
use crate::dwallet_mpc::sign::{
    LAST_SIGN_ROUND_INDEX, SIGN_LAST_ROUND_COMPUTATION_CONSTANT_SECONDS,
};
use crate::dwallet_mpc::{authority_name_to_party_id, party_id_to_authority_name};
use crate::epoch::randomness::SINGLETON_KEY;
use class_groups::DecryptionKeyShare;
use dwallet_mpc_types::dwallet_mpc::{
    DWalletMPCNetworkKeyScheme, MPCMessage, MPCPrivateInput, MPCPrivateOutput, MPCPublicInput,
    MPCPublicOutput, MPCSessionStatus,
};
use fastcrypto::hash::HashFunction;
use fastcrypto::traits::ToFromBytes;
use futures::future::err;
use group::PartyID;
use homomorphic_encryption::AdditivelyHomomorphicDecryptionKeyShare;
use mpc::WeightedThresholdAccessStructure;
use pera_config::NodeConfig;
use pera_types::committee::{EpochId, StakeUnit};
use pera_types::crypto::AuthorityPublicKeyBytes;
use pera_types::crypto::DefaultHash;
use pera_types::digests::Digest;
use pera_types::digests::TransactionDigest;
use pera_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use pera_types::event::Event;
use pera_types::messages_consensus::{ConsensusTransaction, DWalletMPCMessage};
use pera_types::messages_dwallet_mpc::{
    DWalletMPCEvent, DWalletMPCLocalComputationMetadata, MPCProtocolInitData, MaliciousReport,
    SessionInfo, SignIASessionData,
};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use shared_crypto::intent::HashingIntentScope;
use std::collections::{HashMap, HashSet, VecDeque};
use std::mem;
use std::sync::{Arc, Weak};
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
pub struct DWalletMPCManager {
    party_id: PartyID,
    /// Holds the active MPC sessions, cleaned every epoch switch.
    pub(crate) mpc_sessions: HashMap<ObjectID, DWalletMPCSession>,
    /// Used to keep track of the order in which pending sessions are received,
    /// so they are activated in order of arrival.
    pending_sessions_queue: VecDeque<DWalletMPCSession>,
    // TODO (#257): Make sure the counter is always in sync with the number of active sessions.
    /// Keep track of the active sessions to avoid exceeding the limit.
    /// We can't use the length of `mpc_sessions` since it is never cleaned.
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
    malicious_handler: MaliciousHandler,
}

/// The messages that the [`DWalletMPCManager`] can receive & process asynchronously.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DWalletMPCDBMessage {
    /// An MPC message from another validator.
    Message(DWalletMPCMessage),
    /// Signal delivery of messages has ended,
    /// now the sessions that received a quorum of messages can advance.
    EndOfDelivery,
    /// Start locking the next epoch committee by sending a [`ConsensusTransactionKind::LockNextCommittee`] message
    /// to the other validators.
    /// This starts when the current epoch time has ended, and it's time to start the
    /// reconfiguration process for the next epoch.
    StartLockNextEpochCommittee,
    /// A vote received from another validator to lock the next committee.
    /// After receiving a quorum of those messages, a system TX
    /// to lock the next epoch's committee will get created.
    LockNextEpochCommitteeVote(AuthorityName),
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
            max_active_mpc_sessions: node_config.max_active_dwallet_mpc_sessions,
            node_config,
            weighted_threshold_access_structure,
            validators_data_for_network_dkg: HashMap::new(),
            cryptographic_computations_orchestrator: mpc_computations_orchestrator,
            malicious_handler: MaliciousHandler::new(quorum_threshold, weighted_parties),
        })
    }

    pub(crate) async fn handle_dwallet_db_event(&mut self, event: DWalletMPCEvent) {
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
            DWalletMPCDBMessage::StartLockNextEpochCommittee => {
                if let Err(err) = self.start_lock_next_epoch().await {
                    error!(
                        "Failed to start lock next epoch committee with error: {:?}",
                        err
                    );
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
            DWalletMPCDBMessage::MPCSessionFailed(session_id) => {
                // TODO (#524): Handle failed MPC sessions
            }
            DWalletMPCDBMessage::LockNextEpochCommitteeVote(_) => {}
            DWalletMPCDBMessage::SessionFailedWithMaliciousParties(authority_name, report) => {
                if let Err(err) =
                    self.handle_session_failed_with_malicious_parties(authority_name, report)
                {
                    error!(
                        "dWallet MPC session failed with malicious parties with error: {:?}",
                        err
                    );
                }
            }
        }
    }

    fn handle_session_failed_with_malicious_parties(
        &mut self,
        authority_name: AuthorityName,
        report: MaliciousReport,
    ) -> DwalletMPCResult<()> {
        let epoch_store = self.epoch_store()?;
        let status = self
            .malicious_handler
            .report_malicious_actor(report.clone(), authority_name)?;

        match status {
            // Quorum reached, remove the malicious parties from the session messages.
            ReportStatus::QuorumReached => {
                if let Some(mut session) = self.mpc_sessions.get_mut(&report.session_id) {
                    if let MPCProtocolInitData::SignIdentifiableAbort(ia_data) =
                        &session.session_info.mpc_round
                    {
                        // If we reached a quorum on the malicious actors reported in a sign flow,
                        // we should remove them and re-run the last round without them.
                        let sign_session_id = ia_data.sign_session_id.clone();
                        // Drops to remove the mutable references to `self`, so we'll be able to
                        // get & change the sign session this IA session is pointing to.
                        drop(&ia_data);
                        session.status = MPCSessionStatus::Finished;
                        drop(session);
                        let Some(mut sign_session) = self.mpc_sessions.get_mut(&sign_session_id)
                        else {
                            return Err(DwalletMPCError::MPCSessionNotFound {
                                session_id: report.session_id,
                            });
                        };
                        sign_session.status = MPCSessionStatus::Active;
                        let malicious_parties = self
                            .malicious_handler
                            .get_malicious_actors_ids(epoch_store)?;
                        sign_session
                            .rerun_last_round_without_malicious_parties(&malicious_parties)?;
                    } else {
                        let malicious_parties = self
                            .malicious_handler
                            .get_malicious_actors_ids(epoch_store)?;
                        session.rerun_last_round_without_malicious_parties(&malicious_parties)?;
                    }
                }
            }
            ReportStatus::OverQuorum => {}
            ReportStatus::WaitingForQuorum => {
                let Some(mut session) = self.mpc_sessions.get_mut(&report.session_id) else {
                    error!(
                        "failed to get session with session_id: {:?}",
                        report.session_id
                    );
                    return Err(DwalletMPCError::MPCSessionNotFound {
                        session_id: report.session_id,
                    });
                };
                if matches!(
                    &session.session_info.mpc_round,
                    MPCProtocolInitData::Sign(..)
                ) {
                    // If one of the validators reports a failure due to malicious actors in the
                    // sign flow, all validators should start a dedicated sign IA flow.
                    // This unique behavior is needed because ideally the sign computation step
                    // only get executed by one validator, and in order to agree on the malicious
                    // actors all validators should run this step.
                    if session.status == MPCSessionStatus::Active {
                        session.status = MPCSessionStatus::Failed;
                        let session_id_bytes: [u8; 32] = session
                            .session_info
                            .session_id
                            .to_vec()
                            .try_into()
                            .expect("Vec<u8> must have exactly 32 elements");
                        let sign_ia_session_id =
                            ObjectID::derive_id(TransactionDigest::from(session_id_bytes), 0);
                        // TODO (#564): Make sure the pending messages from the involved parties
                        // constitute a quorum, and if not mark the initiating authority as
                        // malicious.
                        let pending_messages: Vec<HashMap<PartyID, MPCMessage>> = session
                            .pending_messages
                            .iter()
                            .map(|round_messages| {
                                let filtered_round_messages: HashMap<PartyID, MPCMessage> =
                                    round_messages
                                        .iter()
                                        .filter_map(|(party_id, messages)| {
                                            if report.involved_parties.contains(party_id) {
                                                Some((*party_id, messages.clone()))
                                            } else {
                                                None
                                            }
                                        })
                                        .collect();
                                filtered_round_messages.clone()
                            })
                            .collect();
                        let session_clone = session.clone();
                        // Need to get rid of the immutable reference to `self` before using it
                        // as mutable
                        drop(session);
                        self.push_new_mpc_session(
                            session_clone.public_input.clone(),
                            None,
                            SessionInfo {
                                flow_session_id: sign_ia_session_id,
                                session_id: sign_ia_session_id,
                                initiating_user_address: session_clone
                                    .session_info
                                    .initiating_user_address,
                                mpc_round: MPCProtocolInitData::SignIdentifiableAbort(
                                    SignIASessionData {
                                        initiating_authority: authority_name,
                                        claimed_malicious_actors: report.malicious_actors,
                                        sign_session_id: session_clone.session_info.session_id,
                                        parties_used_for_last_step: report.involved_parties,
                                    },
                                ),
                            },
                            pending_messages,
                        )?;
                    }
                }
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

    async fn start_lock_next_epoch(&mut self) -> PeraResult {
        self.consensus_adapter
            .submit_to_consensus(
                &vec![self.new_lock_next_committee_message()?],
                &self.epoch_store()?,
            )
            .await?;
        Ok(())
    }

    fn new_lock_next_committee_message(&self) -> DwalletMPCResult<ConsensusTransaction> {
        Ok(ConsensusTransaction::new_lock_next_committee_message(
            self.epoch_store()?.name,
            self.epoch_store()?.epoch(),
        ))
    }

    fn handle_event(&mut self, event: Event, session_info: SessionInfo) -> DwalletMPCResult<()> {
        let (public_input, private_input) = session_input_from_event(&event, &self)?;
        self.push_new_mpc_session(
            public_input,
            private_input,
            session_info,
            vec![HashMap::new()],
        )?;
        Ok(())
    }

    pub(super) fn get_protocol_public_parameters(
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

    /// Advance all the MPC sessions that either received enough messages
    /// or perform the first step of the flow.
    /// We parallelize the advances with `Rayon` to speed up the process.
    pub async fn handle_end_of_delivery(&mut self) -> PeraResult {
        let threshold = self.epoch_store()?.committee().quorum_threshold();
        let mpc_network_key_status = self
            .epoch_store()?
            .dwallet_mpc_network_keys
            .get()
            .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionKeyShares)?
            .status()?;
        let mut ready_to_advance: Vec<DWalletMPCSession> = self
            .mpc_sessions
            .iter_mut()
            .filter_map(|(_, session)| {
                let received_weight: PartyID = match session.status {
                    MPCSessionStatus::Active => session
                        .pending_messages
                        .get(session.pending_quorum_for_highest_round_number)
                        .unwrap_or(&HashMap::new())
                        .keys()
                        .filter_map(|authority_index| {
                            self.weighted_threshold_access_structure
                                .party_to_weight
                                .get(authority_index)
                        })
                        .sum(),
                    _ => 0,
                };

                let is_ready = match session.status {
                    MPCSessionStatus::Active => {
                        received_weight as StakeUnit >= threshold
                            || session.pending_quorum_for_highest_round_number == 0
                    }
                    _ => false,
                };

                let is_valid_network_dkg_transaction =
                    matches!(
                        session.session_info.mpc_round,
                        MPCProtocolInitData::NetworkDkg(..)
                    ) && self.validators_data_for_network_dkg.len()
                        == self
                            .weighted_threshold_access_structure
                            .party_to_weight
                            .len();

                let is_manager_ready = !cfg!(feature = "with-network-dkg")
                    || (is_valid_network_dkg_transaction
                        || matches!(
                            mpc_network_key_status,
                            DwalletMPCNetworkKeysStatus::Ready(_)
                        ));
                if is_ready && is_manager_ready {
                    let session_clone = session.clone();
                    session.pending_quorum_for_highest_round_number =
                        session.pending_quorum_for_highest_round_number + 1;
                    Some(session_clone)
                } else {
                    None
                }
            })
            .collect();

        self.cryptographic_computations_orchestrator
            .insert_ready_sessions(ready_to_advance);
        Ok(())
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
            self.cryptographic_computations_orchestrator
                .currently_running_sessions_count += 1;
            if let Err(err) = self.spawn_session(&session) {
                error!("failed to spawn session with err: {:?}", err);
                return;
            }
        }
    }

    fn spawn_session(&self, session: &DWalletMPCSession) -> DwalletMPCResult<()> {
        let session_id = session.session_info.session_id;
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
        let handle = tokio::runtime::Handle::current();
        let session = session.clone();
        let finished_computation_sender = self
            .cryptographic_computations_orchestrator
            .completed_computation_channel_sender
            .clone();
        if matches!(
            session.session_info.mpc_round,
            MPCProtocolInitData::Sign(..)
        ) && session.pending_quorum_for_highest_round_number == LAST_SIGN_ROUND_INDEX
        {
            self.spawn_aggregated_sign(session_id, handle, session, finished_computation_sender)?;
        } else {
            rayon::spawn_fifo(move || {
                if let Err(err) = session.advance(&handle) {
                    error!("failed to advance session with error: {:?}", err);
                }
                if let Err(err) = finished_computation_sender.send(()) {
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
        &self,
        session_id: ObjectID,
        handle: Handle,
        session: DWalletMPCSession,
        finished_computation_sender: UnboundedSender<()>,
    ) -> DwalletMPCResult<()> {
        let sign_last_step_delay =
            self.calculate_last_sign_step_validator_delay(&session.session_info)?;
        let epoch_store = self.epoch_store()?;
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(
                sign_last_step_delay as u64,
            ))
            .await;
            let manager = epoch_store.get_dwallet_mpc_manager().await;
            let Some(session) = manager.mpc_sessions.get(&session_id) else {
                error!(
                    "failed to get session when checking if sign last round should get executed"
                );
                return;
            };
            if session.status == MPCSessionStatus::Active {
                info!(
                    "running last sign cryptographic step for session_id: {:?}",
                    session_id
                );
                let session = session.clone();
                rayon::spawn_fifo(move || {
                    if let Err(err) = session.advance(&handle) {
                        error!("failed to advance session with error: {:?}", err);
                    }
                    if let Err(err) = finished_computation_sender.send(()) {
                        error!(
                            "Failed to send a finished computation message with error: {:?}",
                            err
                        );
                    }
                });
            }
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

    fn epoch_store(&self) -> DwalletMPCResult<Arc<AuthorityPerEpochStore>> {
        self.epoch_store
            .upgrade()
            .ok_or(DwalletMPCError::EpochEnded(self.epoch_id))
    }

    /// Deterministically decides by the session ID how long this validator should wait before
    /// running the last step of the sign protocol.
    /// If while waiting, the validator receives a valid signature for this session,
    /// it will not run the last step in the sign protocol, and save computation resources.
    fn calculate_last_sign_step_validator_delay(
        &self,
        session_info: &SessionInfo,
    ) -> DwalletMPCResult<usize> {
        let session_id_as_32_bytes: [u8; 32] = session_info.session_id.into_bytes();
        let positions = &self
            .epoch_store()?
            .committee()
            .shuffle_by_stake_from_tx_digest(&TransactionDigest::new(session_id_as_32_bytes));
        let authority_name = &self.epoch_store()?.name;
        let position = positions
            .iter()
            .position(|&x| x == *authority_name)
            .ok_or(DwalletMPCError::InvalidMPCPartyType)?;
        Ok(SIGN_LAST_ROUND_COMPUTATION_CONSTANT_SECONDS * position)
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
                self.malicious_handler
                    .report_malicious_actors(&vec![message.authority]);
                return Ok(());
            }
        };
        match session.handle_message(&message) {
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
        let malicious_party_names = malicious_parties
            .iter()
            .map(|party_id| party_id_to_authority_name(*party_id, &*self.epoch_store()?))
            .collect::<DwalletMPCResult<Vec<AuthorityName>>>()?;
        warn!(
            "dWallet MPC flagged the following parties as malicious: {:?}",
            malicious_party_names
        );

        self.malicious_handler
            .report_malicious_actors(&malicious_party_names);
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
        public_input: MPCPublicInput,
        private_input: MPCPrivateInput,
        session_info: SessionInfo,
        pending_messages: Vec<HashMap<PartyID, MPCMessage>>,
    ) -> DwalletMPCResult<()> {
        if self.mpc_sessions.contains_key(&session_info.session_id) {
            // This should never happen, as the session ID is a Move UniqueID.
            error!(
                "received start flow event for session ID {:?} that already exists",
                &session_info.session_id
            );
            return Ok(());
        }
        info!(
            "Received start MPC flow event for session ID {:?}",
            session_info.session_id
        );

        let mut new_session = DWalletMPCSession::new(
            self.epoch_store.clone(),
            self.consensus_adapter.clone(),
            self.epoch_id,
            MPCSessionStatus::Pending,
            public_input,
            session_info.clone(),
            self.party_id,
            self.weighted_threshold_access_structure.clone(),
            match session_info.mpc_round {
                MPCProtocolInitData::NetworkDkg(..) => HashMap::new(),
                _ => self.get_decryption_key_shares(
                    DWalletMPCNetworkKeyScheme::Secp256k1,
                    Some(self.network_key_version(DWalletMPCNetworkKeyScheme::Secp256k1)? as usize),
                )?,
            },
            private_input,
            pending_messages,
        );
        // TODO (#311): Make sure validator don't mark other validators
        // TODO (#311): as malicious or take any active action while syncing
        if self.active_sessions_counter > self.max_active_mpc_sessions {
            self.pending_sessions_queue.push_back(new_session);
            info!(
                "Added MPCSession to pending queue for session_id {:?}",
                &session_info.session_id
            );
            return Ok(());
        }
        new_session.status = MPCSessionStatus::Active;
        self.mpc_sessions
            .insert(session_info.session_id, new_session);
        self.active_sessions_counter += 1;
        info!(
            "Added MPCSession to MPC manager for session_id {:?}",
            session_info.session_id
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
