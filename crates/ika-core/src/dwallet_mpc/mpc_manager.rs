use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::consensus_adapter::SubmitToConsensus;
use crate::dwallet_mpc::crytographic_computation::{
    ComputationId, ComputationRequest, CryptographicComputationsOrchestrator,
};
use crate::dwallet_mpc::dwallet_mpc_metrics::DWalletMPCMetrics;
use crate::dwallet_mpc::mpc_outputs_verifier::{
    DWalletMPCOutputsVerifier, OutputVerificationResult, OutputVerificationStatus,
};
use crate::dwallet_mpc::mpc_session::{DWalletMPCSession, MPCEventData};
use crate::dwallet_mpc::network_dkg::instantiate_dwallet_mpc_network_decryption_key_shares_from_public_output;
use crate::dwallet_mpc::network_dkg::{DwalletMPCNetworkKeys, ValidatorPrivateDecryptionKeyData};
use crate::dwallet_mpc::{
    authority_name_to_party_id_from_committee, generate_access_structure_from_committee,
    get_validators_class_groups_public_keys_and_proofs, party_ids_to_authority_names,
};
use crate::stake_aggregator::StakeAggregator;
use dwallet_classgroups_types::ClassGroupsKeyPairAndProof;
use dwallet_mpc_types::dwallet_mpc::{
    DWalletMPCNetworkKeyScheme, MPCMessage, MPCPrivateOutput, MPCSessionStatus,
    SerializedWrappedMPCPublicOutput,
};
use dwallet_rng::RootSeed;
use group::PartyID;
use ika_config::NodeConfig;
use ika_types::committee::ClassGroupsEncryptionKeyAndProof;
use ika_types::committee::{Committee, EpochId};
use ika_types::crypto::AuthorityName;
use ika_types::crypto::AuthorityPublicKeyBytes;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::error::IkaResult;
use ika_types::messages_dwallet_mpc::{
    DWalletMPCEvent, DWalletMPCMessage, DWalletMPCOutputMessage, DWalletNetworkEncryptionKeyData,
    MaliciousReport, SessionIdentifier, SessionType, ThresholdNotReachedReport,
};
use ika_types::sui::EpochStartSystemTrait;
use itertools::Itertools;
use mpc::WeightedThresholdAccessStructure;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
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
    pub(crate) party_id: PartyID,
    /// A map of all MPC sessions that start execution in this epoch.
    /// These include completed sessions, and they are never to be removed from this
    /// mapping until the epoch advances.
    pub(crate) mpc_sessions: HashMap<SessionIdentifier, DWalletMPCSession>,
    pub(crate) epoch_id: EpochId,
    validator_name: AuthorityPublicKeyBytes,
    pub(crate) committee: Arc<Committee>,
    pub(crate) access_structure: WeightedThresholdAccessStructure,
    pub(crate) validators_class_groups_public_keys_and_proofs:
        HashMap<PartyID, ClassGroupsEncryptionKeyAndProof>,
    pub(crate) cryptographic_computations_orchestrator: CryptographicComputationsOrchestrator,

    /// The set of malicious actors that were agreed upon by a quorum of validators.
    /// This agreement is done synchronically, and thus is it safe to filter malicious actors.
    /// Any message/output from these authorities will be ignored.
    /// This list is maintained during the Epoch.
    /// This happens automatically because the [`DWalletMPCManager`]
    /// is part of the [`AuthorityPerEpochStore`].
    malicious_actors: HashSet<AuthorityName>,
    pub(crate) outputs_verifier: DWalletMPCOutputsVerifier,

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

struct ReadySessionsResponse {
    ready_sessions: Vec<DWalletMPCSession>,
    pending_for_event_sessions: Vec<DWalletMPCSession>,
}

impl DWalletMPCManager {
    pub(crate) fn new(
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
        let outputs_verifier = DWalletMPCOutputsVerifier::new(dwallet_mpc_metrics.clone());

        let root_seed = node_config
            .root_seed
            .clone()
            .ok_or(DwalletMPCError::MissingRootSeed)?
            .root_seed()
            .clone();

        let access_structure = generate_access_structure_from_committee(&committee)?;

        let mpc_computations_orchestrator =
            CryptographicComputationsOrchestrator::try_new(root_seed.clone())?;
        let party_id = authority_name_to_party_id_from_committee(&committee, &validator_name)?;

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
        Ok(Self {
            mpc_sessions: HashMap::new(),
            party_id: authority_name_to_party_id_from_committee(&committee, &validator_name)?,
            epoch_id,
            access_structure,
            validators_class_groups_public_keys_and_proofs:
                get_validators_class_groups_public_keys_and_proofs(&committee)?,
            cryptographic_computations_orchestrator: mpc_computations_orchestrator,
            malicious_actors: HashSet::new(),
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
            outputs_verifier,
            root_seed,
        })
    }

    pub(crate) fn sync_last_session_to_complete_in_current_epoch(
        &mut self,
        previous_value_for_last_session_to_complete_in_current_epoch: u64,
    ) {
        if previous_value_for_last_session_to_complete_in_current_epoch
            > self.last_session_to_complete_in_current_epoch
        {
            self.last_session_to_complete_in_current_epoch =
                previous_value_for_last_session_to_complete_in_current_epoch;
        }
    }

    /// Handle the messages of a given consensus round.
    pub fn handle_consensus_round_messages(
        &mut self,
        consensus_round: u64,
        messages: Vec<DWalletMPCMessage>,
    ) {
        for (_, session) in self.mpc_sessions.iter_mut() {
            // Set the `messages_by_consensus_round` for every open MPC session for the current consensus round to an empty map.
            // This is important, as we count on the `messages_by_consensus_round` to hold entries for all consensus rounds since the session's inception,
            // when we check for delay.
            session
                .messages_by_consensus_round
                .insert(consensus_round, HashMap::new());
        }

        for message in messages {
            self.handle_message(consensus_round, message);
        }
    }

    /// Handles a message by forwarding it to the relevant MPC session.
    /// If the session does not exist, punish the sender.
    pub(crate) fn handle_message(&mut self, consensus_round: u64, message: DWalletMPCMessage) {
        let session_identifier = message.session_identifier;
        let sender_authority = message.authority;
        let mpc_round_number = message.round_number;

        let Ok(sender_party_id) =
            authority_name_to_party_id_from_committee(&self.committee, &sender_authority)
        else {
            error!(
                session_identifier=?session_identifier,
                sender_authority=?sender_authority,
                receiver_authority=?self.validator_name,
                mpc_round_number=?mpc_round_number,
                "Got a message for an authority without party ID",
            );

            return;
        };

        info!(
            session_identifier=?session_identifier,
            sender_authority=?sender_authority,
            receiver_authority=?self.validator_name,
            mpc_round_number=?mpc_round_number,
            "Received an MPC message for session",
        );

        debug!(
            session_identifier=?session_identifier,
            sender_authority=?sender_authority,
            receiver_authority=?self.validator_name,
            mpc_round_number=?mpc_round_number,
            message=?message.message,
            "Received an MPC message for session with contents",
        );

        if self.is_malicious_actor(&sender_authority) {
            info!(
                session_identifier=?session_identifier,
                sender_authority=?sender_authority,
                receiver_authority=?self.validator_name,
                mpc_round_number=?mpc_round_number,
                "Ignoring message from malicious authority",
            );

            return;
        }

        let session = match self.mpc_sessions.entry(session_identifier) {
            Entry::Occupied(session) => session.into_mut(),
            Entry::Vacant(_) => {
                info!(
                    ?session_identifier,
                    sender_authority=?sender_authority,
                    receiver_authority=?self.validator_name,
                    mpc_round_number=?mpc_round_number,
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

        if session.status == MPCSessionStatus::Active {
            session.store_message(consensus_round, sender_party_id, message);
        }
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
        let with_mpc_event_data = if mpc_event_data.is_some() {
            true
        } else {
            false
        };

        let new_session = DWalletMPCSession::new(
            self.validator_name,
            self.committee.clone(),
            self.epoch_id,
            MPCSessionStatus::Active,
            *session_identifier,
            self.party_id,
            self.access_structure.clone(),
            mpc_event_data,
            self.network_dkg_third_round_delay,
            self.decryption_key_reconfiguration_third_round_delay,
            self.dwallet_mpc_metrics.clone(),
        );

        info!(
            party_id=self.party_id,
            authority=?self.validator_name,
            with_mpc_event_data,
            ?session_identifier,
            last_session_to_complete_in_current_epoch=?self.last_session_to_complete_in_current_epoch,
            "Adding a new MPC session to the active sessions map",
        );

        self.mpc_sessions.insert(*session_identifier, new_session);
    }

    /// Spawns all ready MPC cryptographic computations on separate threads using Rayon.
    /// If no local CPUs are available, computations will execute as CPUs are freed.
    ///
    /// A session must have its `mpc_event_data` set in order to be advanced.
    ///
    /// System sessions are always advanced if a CPU is free, user sessions are only advanced
    /// if they come before the last session to complete in the current epoch (at the current time).
    ///
    /// System sessions are always advanced before any user session,
    /// and both system and user sessions are ordered internally by their sequence numbers.
    ///
    /// The messages to advance with are built on the spot, assuming they satisfy required conditions.
    /// They are put on a `ComputationRequest` and forwarded to the `orchestrator` for execution.
    pub(crate) async fn perform_cryptographic_computation(
        &mut self,
    ) -> HashMap<
        ComputationId,
        DwalletMPCResult<
            mpc::AsynchronousRoundResult<
                MPCMessage,
                MPCPrivateOutput,
                SerializedWrappedMPCPublicOutput,
            >,
        >,
    > {
        let computation_requests: Vec<_> = self
            .mpc_sessions
            .iter()
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
                // Safe to `unwrap`, we filtered sessions without `mpc_event_data`.
                let session_mpc_event_data = session.mpc_event_data.as_ref().unwrap();

                let other_session_mpc_event_data = other_session.mpc_event_data.as_ref().unwrap();

                // Sort by descending order, placing system sessions before user ones and sorting session of the same type by sequence number.
                other_session_mpc_event_data.cmp(session_mpc_event_data)
            })
            .flat_map(|(&session_identifier, session)| {
                session.build_messages_to_advance().map(
                    |(consensus_round, messages_for_advance)| {
                        let attempt_number = session.get_attempt_number();

                        // Safe to `unwrap()`, as the session is ready to advance so `mpc_event_data` must be `Some()`.
                        let mpc_event_data = session.mpc_event_data.clone().unwrap();

                        let computation_id = ComputationId {
                            session_identifier,
                            consensus_round,
                            mpc_round: session.current_mpc_round,
                            attempt_number,
                        };

                        let computation_request = ComputationRequest {
                            party_id: self.party_id,
                            validator_name: self.validator_name.clone(),
                            committee: self.committee.clone(),
                            access_structure: self.access_structure.clone(),
                            request_input: mpc_event_data.request_input,
                            private_input: mpc_event_data.private_input,
                            public_input: mpc_event_data.public_input,
                            decryption_key_shares: mpc_event_data.decryption_key_shares,
                            messages: messages_for_advance,
                        };

                        (computation_id, computation_request)
                    },
                )
            })
            .collect();

        let completed_computation_results = self
            .cryptographic_computations_orchestrator
            .receive_completed_computations();
        for (computation_id, computation_request) in computation_requests {
            let computation_executing = self
                .cryptographic_computations_orchestrator
                .try_spawn_cryptographic_computation(
                    computation_id,
                    computation_request,
                    self.dwallet_mpc_metrics.clone(),
                )
                .await;

            if !computation_executing {
                return completed_computation_results;
            }
        }

        completed_computation_results
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

    pub(crate) fn handle_dwallet_db_output(
        &mut self,
        output: &DWalletMPCOutputMessage,
    ) -> IkaResult<OutputVerificationResult> {
        let DWalletMPCOutputMessage {
            authority,
            session_request,
            output,
        } = output;
        let authority_index =
            authority_name_to_party_id_from_committee(&self.committee, authority)?;

        // TODO(scaly): think if we want to keep this malicious reporting.
        let output_verification_result = self.outputs_verifier
            .try_verify_output(output, session_request, *authority, self.validator_name.clone(), self.committee.clone())
            .unwrap_or_else(|e| {
                error!(session_identifier=?session_request.session_identifier, authority_index=?authority_index, error=?e, "error verifying DWalletMPCOutput output");
                OutputVerificationResult {
                    result: OutputVerificationStatus::Malicious,
                    malicious_actors: vec![*authority],
                }
            });
        Ok(output_verification_result)
    }

    pub(crate) fn record_threshold_not_reached(
        &mut self,
        consensus_round: u64,
        session_identifier: SessionIdentifier,
    ) {
        if let Some(session) = self.mpc_sessions.get_mut(&session_identifier) {
            session.record_threshold_not_reached(consensus_round)
        }
    }

    pub(crate) fn is_malicious_actor(&self, authority: &AuthorityName) -> bool {
        self.malicious_actors.contains(authority)
    }

    /// Records malicious actors that were identified as part of the execution of an MPC session.
    pub(crate) fn record_malicious_actors(&mut self, authorities: &[AuthorityName]) {
        self.malicious_actors.extend(authorities);

        if self.is_malicious_actor(&self.validator_name) {
            self.recognized_self_as_malicious = true;

            error!(
                authority=?self.validator_name,
                "node recognized itself as malicious"
            );
        }
    }
}
