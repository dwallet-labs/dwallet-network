use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::consensus_adapter::SubmitToConsensus;
use pera_types::base_types::{AuthorityName, ObjectID, PeraAddress};
use pera_types::error::{PeraError, PeraResult};

use crate::dwallet_mpc::mpc_events::StartBatchedSignEvent;
use crate::dwallet_mpc::mpc_instance::{
    authority_name_to_party_id, DWalletMPCInstance, DWalletMPCMessage, MPCSessionStatus,
};
use crate::dwallet_mpc::mpc_outputs_manager::{DWalletMPCOutputsManager, OutputVerificationResult};
use crate::dwallet_mpc::mpc_party::MPCParty;
use crate::dwallet_mpc::network_dkg::NetworkDkg;
use crate::dwallet_mpc::sign::BatchedSignSession;
use crate::dwallet_mpc::FIRST_EPOCH_ID;
use anyhow::anyhow;
use group::PartyID;
use homomorphic_encryption::AdditivelyHomomorphicDecryptionKeyShare;
use mpc::{Error, WeightedThresholdAccessStructure};
use pera_config::NodeConfig;
use pera_types::committee::{EpochId, StakeUnit};
use pera_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use pera_types::event::Event;
use pera_types::messages_consensus::ConsensusTransaction;
use pera_types::messages_dwallet_mpc::{MPCRound, SessionInfo};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Weak};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::MutexGuard;
use tracing::log::warn;
use tracing::{error, info};
use twopc_mpc::secp256k1::class_groups::DecryptionKeyShare;

#[derive(Debug, PartialEq)]
pub enum ManagerStatus {
    Active,
    WaitingForNetworkDKGCompletion,
}

/// The [`DWalletMPCManager`] manages MPC instances:
/// — Keeping track of all MPC instances,
/// — Executing all active instances, and
/// — (De)activating instances.
pub struct DWalletMPCManager {
    party_id: PartyID,
    batched_sign_sessions: HashMap<ObjectID, BatchedSignSession>,
    /// Holds the active MPC instances, cleaned every epoch switch.
    mpc_instances: HashMap<ObjectID, DWalletMPCInstance>,
    /// Used to keep track of the order in which pending instances are received,
    /// so they are activated in order of arrival.
    pending_instances_queue: VecDeque<DWalletMPCInstance>,
    // TODO (#257): Make sure the counter is always in sync with the number of active instances.
    /// Keep track of the active instances to avoid exceeding the limit.
    /// We can't use the length of `mpc_instances` since it is never cleaned.
    active_instances_counter: usize,
    consensus_adapter: Arc<dyn SubmitToConsensus>,
    pub(crate) node_config: NodeConfig,
    epoch_store: Weak<AuthorityPerEpochStore>,
    max_active_mpc_sessions: usize,
    epoch_id: EpochId,
    /// A set of all the authorities that behaved maliciously at least once during the epoch.
    /// Any message/output from these authorities will be ignored.
    pub(crate) malicious_actors: HashSet<AuthorityName>,
    weighted_threshold_access_structure: WeightedThresholdAccessStructure,
    weighted_parties: HashMap<PartyID, PartyID>,
    outputs_manager: DWalletMPCOutputsManager,
    status: ManagerStatus,
}

/// A channel that may be sent to the asynchronous [`DWalletMPCManager`].
pub enum DWalletMPCChannelMessage {
    /// An MPC message from another validator
    Message(Vec<u8>, AuthorityName, ObjectID),
    /// An output for a completed MPC message
    Output(Vec<u8>, AuthorityName, SessionInfo),
    /// A new session event
    Event(Event, SessionInfo),
    /// A signal that the delivery of messages has ended, now the instances that received a quorum of messages can advance
    EndOfDelivery,
    StartLockNextEpochCommittee,
}

impl DWalletMPCManager {
    pub async fn try_new(
        consensus_adapter: Arc<dyn SubmitToConsensus>,
        epoch_store: Arc<AuthorityPerEpochStore>,
        epoch_id: EpochId,
        node_config: NodeConfig,
    ) -> DwalletMPCResult<DWalletMPCSender> {
        let weighted_parties: HashMap<PartyID, PartyID> = epoch_store
            .committee()
            .voting_rights
            .iter()
            .map(|(name, weight)| {
                Ok((
                    authority_name_to_party_id(&name, &epoch_store)?,
                    *weight as PartyID,
                ))
            })
            .collect::<DwalletMPCResult<HashMap<PartyID, PartyID>>>()?;
        let weighted_threshold_access_structure = WeightedThresholdAccessStructure::new(
            epoch_store.committee().quorum_threshold() as PartyID,
            weighted_parties.clone(),
        )
        .map_err(|e| DwalletMPCError::MPCManagerError(format!("{}", e)))?;

        // Start the network DKG if this is the first epoch
        let (status, mpc_instances) = if epoch_id == FIRST_EPOCH_ID {
            (
                ManagerStatus::WaitingForNetworkDKGCompletion,
                NetworkDkg::init(epoch_store.clone())?,
            )
        } else {
            // Todo (#380): Load the network DKG outputs
            (ManagerStatus::Active, HashMap::new())
        };

        // Todo (#383): Remove the `outputs_manager` from the `DWalletMPCManager`
        let mut outputs_manager = DWalletMPCOutputsManager::new(&epoch_store);
        let mut epoch_store_outputs_manager = epoch_store.get_dwallet_mpc_outputs_manager().await?;
        for (network_dkg_session_id, _) in mpc_instances.iter() {
            outputs_manager.insert_new_output_instance(network_dkg_session_id);
            epoch_store_outputs_manager.insert_new_output_instance(network_dkg_session_id);
        }

        let (sender, mut receiver) =
            tokio::sync::mpsc::unbounded_channel::<DWalletMPCChannelMessage>();
        let mut manager = Self {
            mpc_instances,
            pending_instances_queue: VecDeque::new(),
            active_instances_counter: 0,
            consensus_adapter,
            party_id: authority_name_to_party_id(&epoch_store.name.clone(), &epoch_store.clone())?,
            epoch_store: Arc::downgrade(&epoch_store),
            epoch_id,
            max_active_mpc_sessions: node_config.max_active_dwallet_mpc_sessions,
            node_config,
            malicious_actors: HashSet::new(),
            weighted_threshold_access_structure,
            weighted_parties,
            batched_sign_sessions: HashMap::new(),
            outputs_manager,
            status,
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
                    error!("Failed to handle message with error: {:?}", err);
                }
            }
            DWalletMPCChannelMessage::Output(output, authority, session_info) => {
                let verification_result = self.outputs_manager.try_verify_output(
                    &output,
                    &session_info,
                    authority.clone(),
                );
                match verification_result {
                    Ok(verification_result) => match verification_result {
                        OutputVerificationResult::ValidWithNewOutput(_, malicious_parties) => {
                            self.malicious_actors.extend(malicious_parties);
                        }
                        OutputVerificationResult::ValidWithoutOutput(malicious_parties) => {
                            self.malicious_actors.extend(malicious_parties);
                        }
                        OutputVerificationResult::Valid(malicious_parties) => {
                            self.malicious_actors.extend(malicious_parties);
                        }
                        OutputVerificationResult::Malicious => {
                            self.malicious_actors.insert(authority);
                        }
                        OutputVerificationResult::Duplicate => {}
                    },
                    Err(err) => {
                        error!("Failed to verify output with error: {:?}", err);
                    }
                }
                {
                    self.malicious_actors.insert(authority);
                }
            }
            DWalletMPCChannelMessage::Event(event, session_info) => {
                if let Err(err) = self.handle_event(event, session_info) {
                    error!("Failed to handle event with error: {:?}", err);
                }
            }
            DWalletMPCChannelMessage::EndOfDelivery => {
                if let Err(err) = self.handle_end_of_delivery().await {
                    error!("Failed to handle end of delivery with error: {:?}", err);
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

    fn new_lock_next_committee_message(&self) -> PeraResult<ConsensusTransaction> {
        Ok(ConsensusTransaction::new_lock_next_committee_message(
            self.epoch_store()?.name,
            self.epoch_store()?.epoch(),
        ))
    }

    fn handle_event(&mut self, event: Event, session_info: SessionInfo) -> PeraResult {
        self.outputs_manager.handle_new_event(&session_info);
        if let Ok((party, auxiliary_input, session_info)) = MPCParty::from_event(
            &event,
            &self,
            authority_name_to_party_id(&self.epoch_store()?.name, &self.epoch_store()?)?,
        ) {
            self.push_new_mpc_instance(auxiliary_input, party, session_info)?;
        };
        Ok(())
    }

    pub fn get_decryption_share(&self) -> DwalletMPCResult<DecryptionKeyShare> {
        let party_id =
            authority_name_to_party_id(&self.epoch_store()?.name, &self.epoch_store()?.clone())?;
        let _ = self
            .node_config
            .dwallet_mpc_class_groups_decryption_shares
            .clone()
            .ok_or(DwalletMPCError::MissingDwalletMPCClassGroupsDecryptionShares)?
            .get(&party_id);
        let share = DecryptionKeyShare::new(
            party_id,
            self.node_config
                .dwallet_mpc_class_groups_decryption_shares
                .clone()
                .ok_or(DwalletMPCError::DwalletMPCClassGroupsDecryptionShareMissing(party_id))?
                .get(&party_id)
                .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionSharesPublicParameters)?
                .clone(),
            &self
                .node_config
                .dwallet_mpc_decryption_shares_public_parameters
                .clone()
                .unwrap(),
        )
        .map_err(|e| twopc_error_to_pera_error(e.into()))?;
        Ok(share)
    }

    /// Advance all the MPC instances that either received enough messages to, or perform the first step of the flow.
    /// We parallelize the advances with Rayon to speed up the process.
    pub async fn handle_end_of_delivery(&mut self) -> PeraResult {
        let threshold = self.epoch_store()?.committee().quorum_threshold();
        let mut ready_to_advance = self
            .mpc_instances
            .iter_mut()
            .filter_map(|(_, instance)| {
                let received_weight: PartyID =
                    if let MPCSessionStatus::Active(round) = instance.status {
                        instance.pending_messages[round]
                            .keys()
                            .map(|authority_index| {
                                // should never be "or" as we receive messages only from known authorities
                                self.weighted_parties.get(authority_index).unwrap_or(&0)
                            })
                            .sum()
                    } else {
                        0
                    };

                let is_ready = (matches!(instance.status, MPCSessionStatus::Active(_))
                    && received_weight as StakeUnit >= threshold)
                    || (instance.status == MPCSessionStatus::FirstExecution);

                let is_valid_status = (self.status
                    == ManagerStatus::WaitingForNetworkDKGCompletion
                    && matches!(instance.party(), MPCParty::NetworkDkg(_)))
                    || self.status == ManagerStatus::Active;

                if is_ready && is_valid_status {
                    Some(instance)
                } else {
                    None
                }
            })
            .collect::<Vec<&mut DWalletMPCInstance>>();

        let results: Vec<PeraResult<(ConsensusTransaction, Vec<PartyID>)>> = ready_to_advance
            .par_iter_mut()
            .map(|ref mut instance| {
                instance.advance(&self.weighted_threshold_access_structure, self.party_id)
            })
            .collect();
        let messages = results
            .into_iter()
            .filter_map(|result| {
                if let Err(PeraError::DWalletMPCMaliciousParties(malicious_parties)) = result {
                    self.flag_parties_as_malicious(malicious_parties).ok()?;
                    return None;
                } else if let Ok((message, malicious_parties)) = result {
                    self.flag_parties_as_malicious(malicious_parties).ok()?;
                    return Some(message);
                }
                None
            })
            .collect::<Vec<ConsensusTransaction>>();
        // Need to send the messages one by one so the consensus adapter won't think they are a [soft bundle](https://github.com/sui-foundation/sips/pull/19)
        for message in messages {
            self.consensus_adapter
                .submit_to_consensus(&vec![message], &self.epoch_store()?)
                .await?;
        }

        if self.status == ManagerStatus::WaitingForNetworkDKGCompletion {
            if self
                .mpc_instances
                .iter()
                .filter(|(_, instance)| matches!(instance.party(), MPCParty::NetworkDkg(_)))
                .all(|(_, instance)| matches!(instance.status, MPCSessionStatus::Finished(_)))
            {
                self.status = ManagerStatus::Active;
            }
        }
        Ok(())
    }

    fn epoch_store(&self) -> PeraResult<Arc<AuthorityPerEpochStore>> {
        self.epoch_store
            .upgrade()
            .ok_or(PeraError::EpochEnded(self.epoch_id))
    }

    /// Handles a message by forwarding it to the relevant MPC instance
    /// If the instance does not exist, punish the sender
    pub fn handle_message(
        &mut self,
        message: &[u8],
        authority_name: AuthorityName,
        session_id: ObjectID,
    ) -> PeraResult {
        if self.malicious_actors.contains(&authority_name) {
            return Ok(());
        }
        let Some(instance) = self.mpc_instances.get_mut(&session_id) else {
            warn!(
                "received a message for instance {:?} which does not exist",
                session_id
            );
            self.malicious_actors.insert(authority_name);
            return Ok(());
        };
        let handle_message_response = instance.handle_message(DWalletMPCMessage {
            message: message.to_vec(),
            authority: authority_name,
        });
        if let Err(PeraError::DWalletMPCMaliciousParties(malicious_parties)) =
            handle_message_response
        {
            self.flag_parties_as_malicious(malicious_parties)?;
            return Ok(());
        };
        handle_message_response
    }

    /// Convert the indices of the malicious parties to their addresses and store them
    /// in the malicious actors set
    /// New messages from these parties will be ignored
    pub fn flag_parties_as_malicious(&mut self, malicious_parties: Vec<PartyID>) -> PeraResult {
        let malicious_parties_names = malicious_parties
            .into_iter()
            .map(|party_id| {
                Ok(*self
                    .epoch_store()?
                    .committee()
                    .authority_by_index(party_id as u32)
                    .ok_or(PeraError::InvalidCommittee("".to_string()))?)
            })
            .collect::<PeraResult<Vec<AuthorityName>>>()?;
        warn!(
            "flagged the following parties as malicious: {:?}",
            malicious_parties_names
        );
        self.malicious_actors.extend(malicious_parties_names);
        Ok(())
    }

    /// Spawns a new MPC instance if the number of active instances is below the limit
    /// and the pending instances queue is empty. Otherwise, adds the instance to the pending queue
    pub fn push_new_mpc_instance(
        &mut self,
        auxiliary_input: Vec<u8>,
        party: MPCParty,
        session_info: SessionInfo,
    ) -> PeraResult {
        let session_id = session_info.session_id.clone();
        if self.mpc_instances.contains_key(&session_id) {
            // This should never happen, as the session ID is a move UniqueID
            error!(
                "Received start flow event for session ID {:?} that already exists",
                session_id
            );
            return Ok(());
        }

        info!("Received start flow event for session ID {:?}", session_id);
        let mut new_instance = DWalletMPCInstance::new(
            self.epoch_store.clone(),
            self.epoch_id,
            party,
            MPCSessionStatus::Pending,
            auxiliary_input,
            session_info,
            Some(self.get_decryption_share()?),
        );
        // TODO (#311): Make validator don't mark other validators as malicious or take any active action while syncing
        if self.active_instances_counter > self.max_active_mpc_sessions
            || !self.pending_instances_queue.is_empty()
        {
            self.pending_instances_queue.push_back(new_instance);
            info!(
                "Added MPCInstance to pending queue for session_id {:?}",
                session_id
            );
            return Ok(());
        }
        new_instance.status = MPCSessionStatus::FirstExecution;
        self.mpc_instances.insert(session_id.clone(), new_instance);
        self.active_instances_counter += 1;
        info!(
            "Added MPCInstance to MPC manager for session_id {:?}",
            session_id
        );
        Ok(())
    }
}

/// Convert a `twopc_mpc::Error` to a `PeraError`.
/// Needed this function and not a `From` implementation because when including the `twopc_mpc` crate
/// as a dependency in the `pera-types` crate there are many conflicting implementations.
pub fn twopc_error_to_pera_error(error: mpc::Error) -> PeraError {
    match error {
        Error::UnresponsiveParties(parties)
        | Error::InvalidMessage(parties)
        | Error::MaliciousMessage(parties) => PeraError::DWalletMPCMaliciousParties(parties),
        _ => PeraError::InternalDWalletMPCError,
    }
}

pub type DWalletMPCSender = UnboundedSender<DWalletMPCChannelMessage>;
