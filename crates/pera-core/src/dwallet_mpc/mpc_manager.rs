use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::consensus_adapter::SubmitToConsensus;
use pera_types::base_types::{AuthorityName, ObjectID};
use pera_types::error::PeraResult;

use crate::dwallet_mpc::batches_manager::BatchedSignSession;
use crate::dwallet_mpc::mpc_events::{StartBatchedSignEvent, ValidatorDataForDWalletSecretShare};
use crate::dwallet_mpc::mpc_outputs_verifier::{DWalletMPCOutputsVerifier, OutputResult};
use crate::dwallet_mpc::mpc_party::{AsyncProtocol, MPCParty};
use crate::dwallet_mpc::network_dkg::{DwalletMPCNetworkKeyVersions, DwalletMPCNetworkKeysStatus};
use crate::dwallet_mpc::sign::SignFirstParty;
use crate::dwallet_mpc::{authority_name_to_party_id, DWalletMPCMessage};
use dwallet_mpc_types::dwallet_mpc::{
    DWalletMPCNetworkKey, MPCMessage, MPCPrivateOutput, MPCPublicOutput, MPCSessionStatus,
};
use crate::dwallet_mpc::{from_event, FIRST_EPOCH_ID};
use anyhow::anyhow;
use class_groups::dkg::Secp256k1Party;
use class_groups::DecryptionKeyShare;
use group::PartyID;
use homomorphic_encryption::AdditivelyHomomorphicDecryptionKeyShare;
use mpc::{Weight, WeightedThresholdAccessStructure};
use pera_config::NodeConfig;
use pera_types::committee::{EpochId, StakeUnit};
use pera_types::dwallet_mpc::DWalletMPCNetworkKeyScheme;
use pera_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use pera_types::event::Event;
use pera_types::messages_consensus::ConsensusTransaction;
use pera_types::messages_dwallet_mpc::{MPCRound, SessionInfo};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Weak};
use tokio::sync::mpsc::UnboundedSender;
use tracing::log::warn;
use tracing::{error, info};
use twopc_mpc::secp256k1::class_groups::DecryptionKey;
use twopc_mpc::sign::Protocol;
use crate::dwallet_mpc::mpc_session::DWalletMPCSession;

pub type DWalletMPCSender = UnboundedSender<DWalletMPCChannelMessage>;

/// The [`DWalletMPCManager`] manages MPC sessions:
/// — Keeping track of all MPC sessions,
/// — Executing all active sessions, and
/// — (De)activating sessions.
pub struct DWalletMPCManager {
    party_id: PartyID,
    /// Holds the active MPC sessions, cleaned every epoch switch.
    mpc_sessions: HashMap<ObjectID, DWalletMPCSession>,
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
    outputs_verifier: DWalletMPCOutputsVerifier,
    validators_data_for_network_dkg: Vec<ValidatorDataForDWalletSecretShare>,
}

/// The messages that the [`DWalletMPCManager`] can receive and process asynchronously.
pub enum DWalletMPCChannelMessage {
    /// An MPC message from another validator.
    Message(MPCMessage, AuthorityName, ObjectID),
    /// An output for a completed MPC message.
    Output(MPCPublicOutput, AuthorityName, SessionInfo),
    /// A new session event.
    Event(Event, SessionInfo),
    /// Signal delivery of messages has ended,
    /// now the sessions that received a quorum of messages can advance.
    EndOfDelivery,
    /// Start locking the next epoch committee by sending a [`ConsensusTransactionKind::LockNextCommittee`] message
    /// to the other validators.
    /// This starts when the current epoch time has ended, and it's time to start the
    /// reconfiguration process for the next epoch.
    StartLockNextEpochCommittee,
    /// A validator's public key and proof for the network DKG protocol
    /// Each validator's data is being emitted separately because the proof size is
    /// almost 250KB, which is the maximum event size in Sui.
    /// The manager accumulates the data until it received such an event for all validators, and then it starts the network DKG protocol.
    ValidatorDataForDKG(ValidatorDataForDWalletSecretShare),
}

impl DWalletMPCManager {
    pub async fn try_new(
        consensus_adapter: Arc<dyn SubmitToConsensus>,
        epoch_store: Arc<AuthorityPerEpochStore>,
        epoch_id: EpochId,
        node_config: NodeConfig,
    ) -> DwalletMPCResult<DWalletMPCSender> {
        let weighted_parties: HashMap<PartyID, Weight> = epoch_store
            .committee()
            .voting_rights
            .iter()
            .map(|(name, weight)| {
                Ok((
                    authority_name_to_party_id(&name, &epoch_store)?,
                    *weight as Weight,
                ))
            })
            .collect::<DwalletMPCResult<HashMap<PartyID, Weight>>>()?;
        let weighted_threshold_access_structure = WeightedThresholdAccessStructure::new(
            epoch_store.committee().quorum_threshold() as PartyID,
            weighted_parties,
        )
        .map_err(|e| DwalletMPCError::MPCManagerError(format!("{}", e)))?;

        epoch_store.dwallet_mpc_network_keys.get().ok_or(DwalletMPCError::MissingDwalletMPCDecryptionKeyShares)?.mock_network_dkg(
            epoch_store.clone(),
            authority_name_to_party_id(&epoch_store.name, &epoch_store)?,
            &weighted_threshold_access_structure,
        );

        let (sender, mut receiver) =
            tokio::sync::mpsc::unbounded_channel::<DWalletMPCChannelMessage>();
        let mut manager = Self {
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
            outputs_verifier: DWalletMPCOutputsVerifier::new(&epoch_store),
            validators_data_for_network_dkg: Vec::new(),
        };

        tokio::spawn(async move {
            while let Some(message) = receiver.recv().await {
                manager.handle_incoming_channel_message(message).await;
            }
        });

        Ok(sender)
    }

    async fn handle_incoming_channel_message(&mut self, message: DWalletMPCChannelMessage) {
        match message {
            DWalletMPCChannelMessage::Message(msg, authority, session_id) => {
                if let Err(err) = self.handle_message(&msg, authority, session_id) {
                    error!("failed to handle an MPC message with error: {:?}", err);
                }
            }
            DWalletMPCChannelMessage::Output(output, authority, session_info) => {
                let verification_result = self.outputs_verifier.try_verify_output(
                    &output,
                    &session_info,
                    authority.clone(),
                );
                match verification_result {
                    Ok(verification_result) => {
                        self.malicious_actors
                            .extend(verification_result.malicious_actors);
                    }
                    Err(err) => {
                        error!("Failed to verify output with error: {:?}", err);
                    }
                }
            }
            DWalletMPCChannelMessage::Event(event, session_info) => {
                if let Err(err) = self.handle_event(event, session_info) {
                    error!("Failed to handle event with error: {:?}", err);
                }
            }
            DWalletMPCChannelMessage::EndOfDelivery => {
                if let Err(err) = self.handle_end_of_delivery().await {
                    error!("failed to handle the end of delivery with error: {:?}", err);
                }
            }
            DWalletMPCChannelMessage::StartLockNextEpochCommittee => {
                if let Err(err) = self.start_lock_next_epoch().await {
                    error!(
                        "Failed to start lock next epoch committee with error: {:?}",
                        err
                    );
                }
            }
            DWalletMPCChannelMessage::ValidatorDataForDKG(data) => {
                self.validators_data_for_network_dkg.push(data);
            }
        }
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
        self.outputs_verifier.handle_new_event(&session_info);
        if let Ok((party, auxiliary_input, session_info)) = from_event(
            &event,
            &self,
            authority_name_to_party_id(&self.epoch_store()?.name, &*self.epoch_store()?)?,
        ) {
            self.push_new_mpc_session(auxiliary_input, party, session_info)?;
        };
        Ok(())
    }

    pub fn get_protocol_public_parameters(&self, key_version: u8) -> DwalletMPCResult<Vec<u8>> {
        // todo (yael): add mock with constant parameters
        if let Some(self_decryption_share) = self.epoch_store()?.dwallet_mpc_network_keys.get() {
            return self_decryption_share.get_protocol_public_parameters(
                DWalletMPCNetworkKeyScheme::Secp256k1,
                key_version,
            );
        }
        Err(DwalletMPCError::TwoPCMPCError(
            "Decryption share not found".to_string(),
        ))
    }

    /// Get the decryption share for the current party.
    // This will be changed in #382
    pub fn get_decryption_share(
        &self,
    ) -> DwalletMPCResult<HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>> {
        if let Some(self_decryption_share) = self.epoch_store()?.dwallet_mpc_network_keys.get() {
            match self_decryption_share
                .get_decryption_key_share(DWalletMPCNetworkKeyScheme::Secp256k1)
            {
                Ok(self_decryption_share) => {
                    return Ok(self_decryption_share
                        .get(self_decryption_share.len() - 1)
                        .ok_or(DwalletMPCError::TwoPCMPCError(
                            "Decryption share not found".to_string(),
                        ))?
                        .clone())
                }
                Err(e) => {}
            }
        }
        let epoch_store = self.epoch_store()?;
        let party_id = authority_name_to_party_id(&epoch_store.name, &epoch_store)?;
        let shares = self
            .node_config
            .dwallet_mpc_class_groups_decryption_shares
            .as_ref()
            .ok_or(DwalletMPCError::MissingDwalletMPCClassGroupsDecryptionShares)?;

        let share_value = shares
            .get(&party_id)
            .ok_or(DwalletMPCError::DwalletMPCClassGroupsDecryptionShareMissing(party_id))?
            .clone();

        let public_parameters = self
            .node_config
            .dwallet_mpc_decryption_shares_public_parameters
            .as_ref()
            .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionSharesPublicParameters)?;

        Ok(HashMap::from([(
            party_id,
            DecryptionKeyShare::new(party_id, share_value, public_parameters)
                .map_err(|e| DwalletMPCError::TwoPCMPCError(e.to_string()))?,
        )]))
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
        let mut malicious_parties = vec![];
        let mut messages = vec![];
        let epoch_store = self.epoch_store()?;

        let mut ready_to_advance = self
            .mpc_sessions
            .iter_mut()
            .filter_map(|(_, session)| {
                let received_weight: PartyID =
                    if let MPCSessionStatus::Active(round) = session.status {
                        session.pending_messages[round]
                            .keys()
                            .map(|authority_index| {
                                // Should never be "or"
                                // as we receive messages only from known authorities.
                                self.weighted_threshold_access_structure
                                    .party_to_weight
                                    .get(authority_index)
                                    .unwrap_or(&0)
                            })
                            .sum()
                    } else {
                        0
                    };

                let is_ready = (matches!(session.status, MPCSessionStatus::Active(_))
                    && received_weight as StakeUnit >= threshold)
                    || (session.status == MPCSessionStatus::FirstExecution);

                let is_manager_ready = if cfg!(feature = "with-network-dkg") {
                    (mpc_network_key_status == DwalletMPCNetworkKeysStatus::NotInitialized
                        && matches!(session.party(), MPCParty::NetworkDkg(_)))
                        && self.validators_data_for_network_dkg.len() == self.weighted_threshold_access_structure.party_to_weight.len()
                        || matches!(
                            mpc_network_key_status,
                            DwalletMPCNetworkKeysStatus::Ready(_)
                        )
                } else {
                    true
                };

                if is_ready && is_manager_ready {
                    Some(session)
                } else {
                    None
                }
            })
            .collect::<Vec<&mut DWalletMPCSession>>();

        if ready_to_advance.len() > 0 && cfg!(feature = "with-network-dkg") {
            // Itay: I verified that at this point you have all the validators data you need to start the network DKG
            todo!("Implement network DKG")
        }
        let mut malicious_parties = vec![];
        let mut messages = vec![];
        ready_to_advance
            .par_iter_mut()
            .map(|session| {
                (
                    session.advance(&self.weighted_threshold_access_structure, self.party_id),
                    session.session_info.session_id,
                )
            })
            .collect::<Vec<_>>()
            // Convert back to an iterator for processing.
            .into_iter()
            .try_for_each(|(result, session_id)| match result {
                Ok((message, malicious)) => {
                    messages.push((message, session_id));
                    malicious_parties.extend(malicious);
                    Ok(())
                }
                Err(DwalletMPCError::MaliciousParties(malicious)) => {
                    malicious_parties.extend(malicious);
                    Ok(())
                }
                // todo(zeev): if there is a fatal error, should we abort?
                Err(e) => Err(e),
            })?;

        self.flag_parties_as_malicious(&malicious_parties)?;

        // Need to send the messages' one by one, so the consensus adapter won't think they
        // are a [soft bundle](https://github.com/sui-foundation/sips/pull/19).
        for (message, session_id) in messages {
            // Update the manager with the new network decryption key share (if relevant).
            let session = self
                .mpc_sessions
                .get(&session_id)
                .ok_or(DwalletMPCError::MPCSessionNotFound { session_id })?;
            if let MPCSessionStatus::Finished(public_output, private_output) =
                session.status.clone()
            {
                if let MPCPrivateOutput::DecryptionKeyShare(private_output) = private_output {
                    if let MPCRound::NetworkDkg(key_type, _) = instance.session_info.mpc_round {
                        let a = base64::encode(&public_output);
                        println!("public_output: {:?}", a);
                        let network_keys = epoch_store
                            .dwallet_mpc_network_keys
                            .get()
                            .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionKeyShares)?;
                        //
                        // instance.session_info.mpc_round = MPCRound::NetworkDkg(
                        //     key_type,
                        //     Some(network_keys.add_key_version(
                        //         epoch_store.clone(),
                        //         key_type,
                        //         private_output,
                        //         public_output,
                        //         &self.weighted_threshold_access_structure,
                        //     )?),
                        // );
                    }
                    // self.update_dwallet_mpc_network_key(
                    //     &mut instance,
                    //     &session.session_info,
                    //     public_output,
                    //     private_output,
                    // )?;
                }
            }

            self.consensus_adapter
                .submit_to_consensus(&vec![message], &self.epoch_store()?)
                .await?;
        }
        Ok(())
    }

    /// Update the encryption of decryption key share with the new shares.
    /// This function is called when the network DKG protocol is done.
    // fn update_dwallet_mpc_network_key(
    //     &self,
    //     instance: &mut DWalletMPCInstance,
    //     session_info: &SessionInfo,
    //     public_output: MPCPublicOutput,
    //     private_output: HashMap<PartyID, class_groups::SecretKeyShareSizedNumber>,
    // ) -> DwalletMPCResult<()> {
    //     if let MPCRound::NetworkDkg(key_type, _) = session_info.mpc_round {
    //         let epoch_store = self.epoch_store()?;
    //         let network_keys = epoch_store
    //             .dwallet_mpc_network_keys
    //             .get()
    //             .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionKeyShares)?;
    //
    //         instance.session_info.mpc_round = MPCRound::NetworkDkg(key_type, network_keys.add_key_version(
    //             epoch_store.clone(),
    //             key_type,
    //             private_output,
    //             public_output,
    //             &self.weighted_threshold_access_structure,
    //         )?);
    //     }
    //     Ok(())
    // }

    fn epoch_store(&self) -> DwalletMPCResult<Arc<AuthorityPerEpochStore>> {
        self.epoch_store
            .upgrade()
            .ok_or(DwalletMPCError::EpochEnded(self.epoch_id))
    }

    /// Handles a message by forwarding it to the relevant MPC session.
    /// If the session does not exist, punish the sender.
    pub(crate) fn handle_message(
        &mut self,
        message: &[u8],
        authority_name: AuthorityName,
        session_id: ObjectID,
    ) -> DwalletMPCResult<()> {
        if self.malicious_actors.contains(&authority_name) {
            return Ok(());
        }
        let session = match self.mpc_sessions.get_mut(&session_id) {
            Some(session) => session,
            None => {
                warn!(
                    "received a message for an MPC session ID: `{:?}` which does not exist",
                    session_id
                );
                self.malicious_actors.insert(authority_name);
                return Ok(());
            }
        };
        match session.handle_message(&DWalletMPCMessage {
            message: message.to_vec(),
            authority: authority_name,
        }) {
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
    /// todo(zeev): clarify if it's restarted on epoch change.
    fn flag_parties_as_malicious(&mut self, malicious_parties: &[PartyID]) -> DwalletMPCResult<()> {
        let malicious_parties_names = malicious_parties
            .iter()
            .map(|party_id| {
                self.epoch_store()?
                    .committee()
                    .authority_by_index(*party_id as u32)
                    .cloned()
                    .ok_or(DwalletMPCError::AuthorityIndexNotFound(*party_id))
            })
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
        auxiliary_input: Vec<u8>,
        party: MPCParty,
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
            self.epoch_id,
            party,
            MPCSessionStatus::Pending,
            auxiliary_input,
            session_info.clone(),
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
        new_session.status = MPCSessionStatus::FirstExecution;
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
