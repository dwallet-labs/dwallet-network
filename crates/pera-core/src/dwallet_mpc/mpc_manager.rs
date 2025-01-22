use crate::authority::authority_per_epoch_store::{AuthorityPerEpochStore, ConsensusCommitOutput};
use crate::consensus_adapter::SubmitToConsensus;
use pera_types::base_types::{AuthorityName, ObjectID};
use pera_types::error::PeraResult;

use crate::dwallet_mpc::mpc_events::ValidatorDataForNetworkDKG;
use crate::dwallet_mpc::mpc_outputs_verifier::DWalletMPCOutputsVerifier;
use crate::dwallet_mpc::mpc_session::{AsyncProtocol, DWalletMPCSession};
use crate::dwallet_mpc::network_dkg::DwalletMPCNetworkKeysStatus;
use crate::dwallet_mpc::session_input_from_event;
use crate::dwallet_mpc::{authority_name_to_party_id, party_id_to_authority_name};
use crate::epoch::randomness::SINGLETON_KEY;
use class_groups::DecryptionKeyShare;
use dwallet_mpc_types::dwallet_mpc::{
    DWalletMPCNetworkKeyScheme, MPCPrivateInput, MPCPrivateOutput, MPCPublicInput, MPCPublicOutput,
    MPCSessionStatus,
};
use fastcrypto::hash::HashFunction;
use fastcrypto::traits::ToFromBytes;
use group::PartyID;
use homomorphic_encryption::AdditivelyHomomorphicDecryptionKeyShare;
use mpc::WeightedThresholdAccessStructure;
use pera_config::NodeConfig;
use pera_types::committee::{EpochId, StakeUnit};
use pera_types::crypto::AuthorityPublicKeyBytes;
use pera_types::crypto::DefaultHash;
use pera_types::digests::Digest;
use pera_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use pera_types::event::Event;
use pera_types::messages_consensus::{ConsensusTransaction, DWalletMPCMessage};
use pera_types::messages_dwallet_mpc::{
    DWalletMPCEvent, DWalletMPCLocalComputationMetadata, MPCRound, SessionInfo,
};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use shared_crypto::intent::HashingIntentScope;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Weak};
use tokio::sync::mpsc::UnboundedSender;
use tracing::log::{debug, warn};
use tracing::{error, info};
use twopc_mpc::sign::Protocol;
use typed_store::Map;

/// The number of logical cores validator's need to allocate for ongoing, non-cryptographic computations.
/// Needed to understand how many sessions computations this validator can run in parallel.
const MACHINE_CORS_FOR_NON_COMPUTATION: usize = 3;

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
    /// A set of all the authorities that behaved maliciously at least once during the epoch.
    /// Any message/output from these authorities will be ignored.
    malicious_actors: HashSet<AuthorityName>,
    weighted_threshold_access_structure: WeightedThresholdAccessStructure,
    pub(crate) validators_data_for_network_dkg: HashMap<PartyID, ValidatorDataForNetworkDKG>,
    /// A map of the pending cryptographic computation sessions.
    /// This map is needed in order to remove a session that we received a quorum of messages for
    /// its next round, so running the current, completed round is redundant.
    pub(crate) pending_computation_map:
        HashMap<DWalletMPCLocalComputationMetadata, DWalletMPCSession>,
    /// The order of the [`pending_computation_map`]. Needed to process the computations in the order they were received.
    pending_for_computation_order: VecDeque<DWalletMPCLocalComputationMetadata>,
    /// The number of currently running cryptographic computations - i.e. computations we called [`rayon::spawn_fifo`] for,
    /// but we didn't receive a completion message for.
    pub(crate) currently_running_sessions_count: usize,
    /// The number of logical CPUs available for cryptographic computations on the validator's machine.
    available_cores_for_cryptographic_computations: usize,
    /// A channel sender to notify the manager that a computation has been completed.
    /// This is needed to decrease the [`currently_running_sessions_count`] when a computation is done.
    completed_computation_channel_sender: UnboundedSender<()>,
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
    /// After receiving a quorum of those messages, a system TX to lock the next epoch's committee will get created.
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
    /// A message to start process the cryptographic computations.
    /// This message is being sent every five seconds by the DWallet MPC Service,
    /// in order to skip redundant advancements that have already been completed by other validators.
    PerformCryptographicComputations,
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
        let completed_computation_channel_sender =
            Self::listen_for_completed_computations(&epoch_store);
        let available_cores_for_computations: usize = std::thread::available_parallelism()
            .map_err(|e| DwalletMPCError::FailedToGetAvailableParallelism)?
            .into();
        if !(available_cores_for_computations > MACHINE_CORS_FOR_NON_COMPUTATION) {
            return Err(DwalletMPCError::InsufficientCPUCores);
        }
        let available_cores_for_cryptographic_computations =
            available_cores_for_computations - MACHINE_CORS_FOR_NON_COMPUTATION;
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
            malicious_actors: HashSet::new(),
            weighted_threshold_access_structure,
            validators_data_for_network_dkg: HashMap::new(),
            pending_computation_map: HashMap::new(),
            pending_for_computation_order: VecDeque::new(),
            currently_running_sessions_count: 0,
            available_cores_for_cryptographic_computations,
            completed_computation_channel_sender,
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
        }
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
        self.push_new_mpc_session(public_input, private_input, session_info)?;
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
                        .get(session.round_number)
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
                        received_weight as StakeUnit >= threshold || session.round_number == 0
                    }
                    _ => false,
                };

                let is_valid_network_dkg_transaction =
                    matches!(session.session_info.mpc_round, MPCRound::NetworkDkg(..))
                        && self.validators_data_for_network_dkg.len()
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
                    session.round_number = session.round_number + 1;
                    Some(session_clone)
                } else {
                    None
                }
            })
            .collect();

        for session in ready_to_advance.into_iter() {
            self.pending_computation_map
                .remove(&DWalletMPCLocalComputationMetadata {
                    session_id: session.session_info.session_id,
                    crypto_round_number: session.round_number - 1,
                });
            let session_next_round_metadata = DWalletMPCLocalComputationMetadata {
                session_id: session.session_info.session_id,
                crypto_round_number: session.round_number,
            };
            self.pending_computation_map
                .insert(session_next_round_metadata, session.clone());
            self.pending_for_computation_order
                .push_back(session_next_round_metadata.clone());
        }
        Ok(())
    }

    fn perform_cryptographic_computation(&mut self) {
        while self.currently_running_sessions_count
            < self.available_cores_for_cryptographic_computations
        {
            let Some(oldest_computation_metadata) = self.pending_for_computation_order.pop_front()
            else {
                break;
            };
            let Some(session) = self
                .pending_computation_map
                .remove(&oldest_computation_metadata)
            else {
                return;
            };
            self.currently_running_sessions_count += 1;
            if let Err(err) = self.spawn_session(&session) {
                error!("Failed to spawn session with err: {:?}", err);
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
        let handle = tokio::runtime::Handle::current();
        let session = session.clone();
        let finished_computation_sender = self.completed_computation_channel_sender.clone();
        rayon::spawn_fifo(move || {
            if let Err(err) = session.advance(&handle) {
                error!("Failed to advance session with error: {:?}", err);
            }
            if let Err(err) = finished_computation_sender.send(()) {
                error!(
                    "Failed to send finished computation message with error: {:?}",
                    err
                );
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
        if let MPCRound::NetworkDkg(key_type, _) = session_info.mpc_round {
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

    /// Handles a message by forwarding it to the relevant MPC session.
    /// If the session does not exist, punish the sender.
    pub(crate) fn handle_message(&mut self, message: DWalletMPCMessage) -> DwalletMPCResult<()> {
        if self.malicious_actors.contains(&message.authority) {
            return Ok(());
        }
        let session = match self.mpc_sessions.get_mut(&message.session_id) {
            Some(session) => session,
            None => {
                warn!(
                    "received a message for an MPC session ID: `{:?}` which does not exist",
                    message.session_id
                );
                self.malicious_actors.insert(message.authority);
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
        let malicious_parties_names = malicious_parties
            .iter()
            .map(|party_id| party_id_to_authority_name(*party_id, &*self.epoch_store()?))
            .collect::<DwalletMPCResult<Vec<AuthorityName>>>()?;
        warn!(
            "[dWallet MPC] Flagged the following parties as malicious: {:?}",
            malicious_parties_names
        );
        self.malicious_actors.extend(malicious_parties_names);
        Ok(())
    }

    /// Spawns a new MPC session if the number of active sessions is below the limit.
    /// Otherwise, add the session to the pending queue.
    pub(crate) fn push_new_mpc_session(
        &mut self,
        public_input: MPCPublicInput,
        private_input: MPCPrivateInput,
        session_info: SessionInfo,
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
                MPCRound::NetworkDkg(..) => HashMap::new(),
                _ => self.get_decryption_key_shares(
                    DWalletMPCNetworkKeyScheme::Secp256k1,
                    Some(self.network_key_version(DWalletMPCNetworkKeyScheme::Secp256k1)? as usize),
                )?,
            },
            private_input,
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

    fn listen_for_completed_computations(
        epoch_store: &Arc<AuthorityPerEpochStore>,
    ) -> UnboundedSender<()> {
        let (completed_computation_channel_sender, mut completed_computation_channel_receiver) =
            tokio::sync::mpsc::unbounded_channel();
        let epoch_store_for_channel = epoch_store.clone();
        tokio::spawn(async move {
            loop {
                match completed_computation_channel_receiver.recv().await {
                    None => {
                        break;
                    }
                    Some(_) => {
                        epoch_store_for_channel
                            .get_dwallet_mpc_manager()
                            .await
                            .currently_running_sessions_count -= 1;
                    }
                }
            }
        });
        completed_computation_channel_sender
    }
}
