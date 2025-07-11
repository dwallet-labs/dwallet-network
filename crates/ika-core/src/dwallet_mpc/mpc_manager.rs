use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::consensus_adapter::SubmitToConsensus;
use crate::dwallet_mpc::cryptographic_computations_orchestrator::{ComputationId, ComputationRequest, CryptographicComputationsOrchestrator};
use crate::dwallet_mpc::dwallet_mpc_metrics::DWalletMPCMetrics;
use crate::dwallet_mpc::malicious_handler::MaliciousHandler;
use crate::dwallet_mpc::mpc_protocols::network_dkg::{
    DwalletMPCNetworkKeys, ValidatorPrivateDecryptionKeyData,
};
use crate::dwallet_mpc::mpc_session::{DWalletMPCSession, MPCEventData};
use crate::dwallet_mpc::network_dkg::instantiate_dwallet_mpc_network_decryption_key_shares_from_public_output;
use crate::dwallet_mpc::{
    authority_name_to_party_id_from_committee, generate_access_structure_from_committee,
    get_validators_class_groups_public_keys_and_proofs, party_ids_to_authority_names,
};
use crate::stake_aggregator::StakeAggregator;
use dwallet_classgroups_types::ClassGroupsKeyPairAndProof;
use dwallet_mpc_types::dwallet_mpc::{DWalletMPCNetworkKeyScheme, MPCMessage, MPCPrivateOutput, MPCSessionStatus, SerializedWrappedMPCPublicOutput};
use dwallet_rng::RootSeed;
use group::PartyID;
use ika_config::NodeConfig;
use ika_types::committee::ClassGroupsEncryptionKeyAndProof;
use ika_types::committee::{Committee, EpochId};
use ika_types::crypto::AuthorityName;
use ika_types::crypto::AuthorityPublicKeyBytes;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_dwallet_mpc::{
    DWalletMPCEvent, DWalletMPCMessage, DWalletNetworkEncryptionKeyData, MaliciousReport,
    SessionIdentifier, SessionType, ThresholdNotReachedReport,
};
use ika_types::sui::EpochStartSystemTrait;
use mpc::WeightedThresholdAccessStructure;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use itertools::Itertools;
use sui_types::base_types::ObjectID;
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
    pub(crate) epoch_id: EpochId,
    validator_name: AuthorityPublicKeyBytes,
    pub(crate) committee: Arc<Committee>,
    pub(crate) access_structure: WeightedThresholdAccessStructure,
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

    pub(crate) last_session_to_complete_in_current_epoch: u64,
    pub(crate) recognized_self_as_malicious: bool,
    pub(crate) network_keys: Box<DwalletMPCNetworkKeys>,
    network_keys_receiver: Receiver<Arc<HashMap<ObjectID, DWalletNetworkEncryptionKeyData>>>,
    /// Events that wait for the network key to update.
    /// Once we get the network key, these events will be executed.
    pub(crate) events_pending_for_network_key: HashMap<ObjectID, Vec<DWalletMPCEvent>>,
    pub(crate) events_pending_for_next_active_committee: Vec<DWalletMPCEvent>,
    pub(crate) next_epoch_committee_receiver: watch::Receiver<Committee>,
    pub(crate) next_active_committee: Option<Committee>,
    pub(crate) dwallet_mpc_metrics: Arc<DWalletMPCMetrics>,
    pub(crate) threshold_not_reached_reports:
        HashMap<ThresholdNotReachedReport, StakeAggregator<(), true>>,

    network_dkg_third_round_delay: usize,
    decryption_key_reconfiguration_third_round_delay: usize,

    /// The root seed of this validator, used for deriving the session and round-specific seed for advancing MPC sessions.
    /// SECURITY NOTICE: *MUST KEEP PRIVATE*.
    root_seed: RootSeed,
}

// TODO(Scaly): delete this struct
/// The messages that the [`DWalletMPCManager`] can receive and process asynchronously.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DWalletMPCDBMessage {
    /// An MPC message from another validator.
    Message(DWalletMPCMessage),
}

struct ReadySessionsResponse {
    ready_sessions: Vec<DWalletMPCSession>,
    pending_for_event_sessions: Vec<DWalletMPCSession>,
}

impl DWalletMPCManager {
    pub(crate) fn must_create_dwallet_mpc_manager(
        consensus_adapter: Arc<dyn SubmitToConsensus>,
        validator_name: AuthorityPublicKeyBytes,
        committee: Arc<Committee>,
        epoch_id: EpochId,
        network_keys_receiver: Receiver<Arc<HashMap<ObjectID, DWalletNetworkEncryptionKeyData>>>,
        next_epoch_committee_receiver: Receiver<Committee>,
        node_config: NodeConfig,
        network_dkg_third_round_delay: usize,
        decryption_key_reconfiguration_third_round_delay: usize,
        dwallet_mpc_metrics: Arc<DWalletMPCMetrics>,
    ) -> Self {
        Self::try_new(
            consensus_adapter.clone(),
            validator_name,
            committee,
            epoch_id,
            network_keys_receiver,
            next_epoch_committee_receiver,
            node_config.clone(),
            network_dkg_third_round_delay,
            decryption_key_reconfiguration_third_round_delay,
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
        validator_name: AuthorityPublicKeyBytes,
        committee: Arc<Committee>,
        epoch_id: EpochId,
        network_keys_receiver: Receiver<Arc<HashMap<ObjectID, DWalletNetworkEncryptionKeyData>>>,
        next_epoch_committee_receiver: watch::Receiver<Committee>,
        node_config: NodeConfig,
        network_dkg_third_round_delay: usize,
        decryption_key_reconfiguration_third_round_delay: usize,
        dwallet_mpc_metrics: Arc<DWalletMPCMetrics>,
    ) -> DwalletMPCResult<Self> {
        let access_structure =
            generate_access_structure_from_committee(&committee)?;

        let mpc_computations_orchestrator = CryptographicComputationsOrchestrator::try_new()?;
        let party_id = authority_name_to_party_id_from_committee(&committee, &validator_name)?;
        let root_seed = node_config
            .root_seed
            .clone()
            .ok_or(DwalletMPCError::MissingRootSeed)?
            .root_seed()
            .clone();
        let class_groups_key_pair = ClassGroupsKeyPairAndProof::from_seed(&root_seed);

        // Verify that the validators local class-groups key is the
        // same as stored in the system state object onchain.
        // This makes sure the seed we are using is the same seed we used at setup
        // to create the encryption key, and thus it assures we will generate the same decryption key too.
        let onchain_class_groups_encryption_key_and_proof: ClassGroupsEncryptionKeyAndProof =
            committee.class_groups_public_key_and_proof(&validator_name)?;
        if onchain_class_groups_encryption_key_and_proof
            != class_groups_key_pair.encryption_key_and_proof()
        {
            return Err(DwalletMPCError::MPCManagerError(
                "validator's class-groups key does not match the one stored in the system state object".to_string(),
            ));
        }

        let validator_private_data = ValidatorPrivateDecryptionKeyData {
            party_id,
            class_groups_decryption_key: class_groups_key_pair.decryption_key(),
            validator_decryption_key_shares: HashMap::new(),
        };
        let dwallet_network_keys = DwalletMPCNetworkKeys::new(validator_private_data);

        // Re-initialize the malicious handler every epoch. This is done intentionally:
        // We want to "forget" the malicious actors from the previous epoch and start from scratch.
        let malicious_handler = MaliciousHandler::new(committee.clone());
        Ok(Self {
            mpc_sessions: HashMap::new(),
            consensus_adapter,
            party_id: authority_name_to_party_id_from_committee(&committee, &validator_name)?,
            epoch_id,
            access_structure,
            validators_class_groups_public_keys_and_proofs:
                get_validators_class_groups_public_keys_and_proofs(&committee)?,
            cryptographic_computations_orchestrator: mpc_computations_orchestrator,
            malicious_handler,
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
            validator_name,
            committee,
            network_dkg_third_round_delay,
            decryption_key_reconfiguration_third_round_delay,
            root_seed,
        })
    }

    pub(crate) fn sync_last_session_to_complete_in_current_epoch(
        &mut self,
        previous_value_for_last_session_to_complete_in_current_epoch: u64,
    ) {
        if previous_value_for_last_session_to_complete_in_current_epoch
            <= self.last_session_to_complete_in_current_epoch
        {
            return;
        }
        self.last_session_to_complete_in_current_epoch =
            previous_value_for_last_session_to_complete_in_current_epoch;
    }

    /// Handle an incoming dWallet MPC message, coming from storage, either during bootstrapping or indirectly originating from the consensus
    /// (which writes the messages to the storage, from which we read them in the dWallet MPC Service and call this function.)
    pub(crate) fn handle_dwallet_message(&mut self, message: DWalletMPCDBMessage) {
        // TODO(Scaly): delete this function this, just call handle_message
        match message {
            DWalletMPCDBMessage::Message(message) => {
                self.handle_message(message.clone());
            }
        }
    }

    /// Handle the messages of a given consensus round.
    pub fn handle_consensus_round_messages(&mut self, consensus_round: u64, messages: Vec<DWalletMPCDBMessage>) {
        for (_, session) in self.mpc_sessions.iter_mut() {
            // Set the `messages_by_consensus_round` for every open MPC session for the current consensus round to an empty map.
            // This is important, as we count on the `messages_by_consensus_round` to hold entries for all consensus rounds since the session's inception,
            // when we check for delay.
            session.messages_by_consensus_round.insert(consensus_round, HashMap::new());
        }

        for message in messages {
            self.handle_dwallet_message(message);
        }

        // TODO(Scaly): set the message to advance here or no?
    }

    // TODO(Scaly): delete, move recognize selves elsewhere
    /// Handle an incoming malicious `report` from `reporting_authority`,
    /// and recognize ourselves as malicious in the case of a bug.
    fn handle_malicious_report(
        &mut self,
        reporting_authority: AuthorityName,
        report: MaliciousReport,
    ) {
        self.malicious_handler
            .report_malicious_actor(report.clone(), reporting_authority);

        if self
            .malicious_handler
            .is_malicious_actor(&self.validator_name)
        {
            self.recognized_self_as_malicious = true;

            error!(
                authority=?self.validator_name,
                reporting_authority=?reporting_authority,
                malicious_actors=?report.malicious_actors,
                session_identifier=?report.session_identifier,
                "node recognized itself as malicious"
            );
        }
    }

    /// Spawns all ready MPC cryptographic computations using Rayon.
    /// If no local CPUs are available, computations will execute as CPUs are freed.
    pub(crate) async fn perform_cryptographic_computation(&mut self) -> HashMap<ComputationId, DwalletMPCResult<
        mpc::AsynchronousRoundResult<MPCMessage, MPCPrivateOutput, SerializedWrappedMPCPublicOutput>,
    >> {
        // TODO: sync last_session_to_complete_in_current_epoch?
        let computation_requests: Vec<_> = self.mpc_sessions.iter()
            .filter(|(_, session)| {
                if let Some(mpc_event_data) = &session.mpc_event_data {
                    // Always advance system sessions, and only advance user session
                    // if they come before the last session to complete in the current epoch (at the current time).
                    match mpc_event_data.session_type {
                        SessionType::User => {
                            mpc_event_data.session_sequence_number
                                <= self.last_session_to_complete_in_current_epoch
                        }
                        SessionType::System => true,
                    }
                } else {
                    // Cannot advance sessions without MPC event data
                    false
                }
            })
            .sorted_by(|(_, session), (_, other_session)| {
                // Sort by descending order, placing system sessions before user ones and sorting session of the same type by sequence number.
                other_session.mpc_event_data.as_ref().unwrap().cmp(session.mpc_event_data.as_ref().unwrap())
            })
            .flat_map(|(&session_identifier, session)| {
            session.get_messages_to_advance().map(|messages_for_advance| {
                let attempt_number = session.get_attempt_number();

                // Safe to `unwrap()`, as the session is ready to advance so `mpc_event_data` must be `Some()`.
                let mpc_event_data = session.mpc_event_data.clone().unwrap();

                let computation_id = ComputationId {
                    session_identifier,
                    mpc_round,
                    attempt_number
                };

                let computation_request = ComputationRequest {
                    party_id: self.party_id,
                    validator_name: self.validator_name.clone(),
                    committee: self.committee.clone(),
                    access_structure: self.access_structure.clone(),
                    input: mpc_event_data.request_input,
                    messages: messages_for_advance,
                };

                (computation_id, computation_request)
            })
        }).collect();

        let completed_computation_results = self.cryptographic_computations_orchestrator.receive_completed_computations();
        for (computation_id, computation_request) in computation_requests {
            let computation_executing = self
                .cryptographic_computations_orchestrator
                .try_spawn_cryptographic_computation(computation_id, computation_request, self.dwallet_mpc_metrics.clone())
                .await;

            if !computation_executing
            {
                return completed_computation_results;
            }
        }

        completed_computation_results
    }



    /// Handles a message by forwarding it to the relevant MPC session.
    /// If the session does not exist, punish the sender.
    pub(crate) fn handle_message(&mut self, message: DWalletMPCMessage) {
        let session_identifier = message.session_identifier;
        let sender_authority = message.authority;
        let mpc_round_number = message.round_number;
        let mpc_protocol = message.mpc_protocol.clone();

        let Ok(sender_party_id) =
            authority_name_to_party_id_from_committee(&self.committee, &sender_authority)
        else {
            error!(
                session_identifier=?session_identifier,
                sender_authority=?sender_authority,
                receiver_authority=?self.validator_name,
                mpc_round_number=?mpc_round_number,
                mpc_protocol=?mpc_protocol,
                "Got a message for an authority without party ID",
            );

            return;
        };

        info!(
            session_identifier=?session_identifier,
            sender_authority=?sender_authority,
            receiver_authority=?self.validator_name,
            mpc_round_number=?mpc_round_number,
            mpc_protocol=mpc_protocol,
            "Received an MPC message for session",
        );

        debug!(
            session_identifier=?session_identifier,
            sender_authority=?sender_authority,
            receiver_authority=?self.validator_name,
            mpc_round_number=?mpc_round_number,
            mpc_protocol=mpc_protocol,
            message=?message.message,
            "Received an MPC message for session with contents",
        );

        if self.malicious_handler.is_malicious_actor(&sender_authority) {
            info!(
                session_identifier=?session_identifier,
                sender_authority=?sender_authority,
                receiver_authority=?self.validator_name,
                mpc_round_number=?mpc_round_number,
                mpc_protocol=?mpc_protocol,
                "Ignoring message from malicious authority",
            );

            return;
        }

        let session = match self.mpc_sessions.entry(session_identifier) {
            Entry::Occupied(session) => session.into_mut(),
            Entry::Vacant(_) => {
                info!(
                    session_identifier=?session_identifier,
                    sender_authority=?sender_authority,
                    receiver_authority=?self.validator_name,
                    mpc_round_number=?mpc_round_number,
                    mpc_protocol=?mpc_protocol,
                    "received a message for an MPC session before receiving an event requesting it"
                );

                // This can happen if the session is not in the active sessions,
                // but we still want to store the message.
                // We will create a new session for it.
                self.new_mpc_session(&session_identifier, None);
                // Safe to `unwrap()`: we just created the session.
                self.mpc_sessions.get_mut(&session_identifier).unwrap()
            }
        };

        let is_malicious = session.store_message(sender_party_id, message);
        if is_malicious {
            error!(
                session_identifier=?session_identifier,
                sender_authority=?sender_authority,
                receiver_authority=?self.validator_name,
                mpc_round_number=?mpc_round_number,
                mpc_protocol=?mpc_protocol,
                sender_party_id=sender_party_id,
                "Validator sent a malicious message"
            );

            self.flag_parties_as_malicious(&[sender_party_id]);
        }
    }

    /// Convert the indices of the malicious parties to their addresses and store them
    /// in the malicious actors set.
    /// New messages from these parties will be ignored.
    /// Restarted for each epoch.
    fn flag_parties_as_malicious(&mut self, malicious_parties: &[PartyID]) {
        // TODO(Scaly): why is this a different flow? why here

        let malicious_parties_names =
            party_ids_to_authority_names(malicious_parties, &self.committee);
        warn!(
            "dWallet MPC flagged the following parties as malicious: {:?}",
            malicious_parties_names
        );

        self.malicious_handler
            .report_malicious_actors(&malicious_parties_names);
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
            self.validator_name,
            self.committee.clone(),
            self.consensus_adapter.clone(),
            self.epoch_id,
            MPCSessionStatus::Active,
            *session_identifier,
            self.party_id,
            self.access_structure.clone(),
            mpc_event_data,
            self.network_dkg_third_round_delay,
            self.decryption_key_reconfiguration_third_round_delay,
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

    pub(crate) fn try_receiving_next_active_committee(&mut self) -> bool {
        match self.next_epoch_committee_receiver.has_changed() {
            Ok(has_changed) => {
                if has_changed {
                    let committee = self
                        .next_epoch_committee_receiver
                        .borrow_and_update()
                        .clone();

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

    pub(crate) fn maybe_update_network_keys(&mut self) -> Vec<ObjectID> {
        match self.network_keys_receiver.has_changed() {
            Ok(has_changed) => {
                if has_changed {
                    let access_structure = &self.access_structure;
                    let new_keys = self.network_keys_receiver.borrow_and_update();

                    let mut new_key_ids = vec![];
                    for (key_id, key_data) in new_keys.iter() {
                        match instantiate_dwallet_mpc_network_decryption_key_shares_from_public_output(
                            key_data.current_epoch,
                            DWalletMPCNetworkKeyScheme::Secp256k1,
                            access_structure,
                            key_data.clone(),
                        ) {
                            Ok(key) => {
                                info!(key_id=?key_id, "Updating (decrypting new shares) network key for key_id");
                                if let Err(e) = self
                                    .network_keys
                                    .update_network_key(
                                        *key_id,
                                        &key,
                                        &self.access_structure,
                                    ) {
                                    error!(error=?e, key_id=?key_id, "failed to update the network key");
                                } else {
                                    new_key_ids.push(*key_id);
                                }
                            }
                            Err(err) => {
                                error!(
                                    ?err,
                                    key_id=?key_id,
                                    "failed to instantiate network decryption key shares from public output for"
                                );
                            }
                        }
                    }

                    new_key_ids
                } else {
                    vec![]
                }
            }
            Err(err) => {
                error!(?err, "failed to check network keys receiver");

                vec![]
            }
        }
    }
}
