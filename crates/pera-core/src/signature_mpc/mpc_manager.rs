use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::consensus_adapter::SubmitToConsensus;
use crate::signature_mpc::mpc_events::{CreatedProofMPCEvent, MPCEvent};
use anyhow::anyhow;
use group::secp256k1::group_element::Value;
use group::{secp256k1, GroupElement, PartyID};
use im::hashmap;
use itertools::Itertools;
use maurer::knowledge_of_discrete_log::PublicParameters;
use maurer::Proof;
use pera_types::base_types::{AuthorityName, ObjectID};
use pera_types::error::{PeraError, PeraResult};
use pera_types::event::Event;
use pera_types::messages_consensus::ConsensusTransaction;
use proof::mpc::{AdvanceResult, Party};
use rand_core::OsRng;
use rayon::prelude::*;
use schemars::_private::NoSerialize;
use std::cmp::PartialEq;
use std::collections::{HashMap, VecDeque};
use std::future::Future;
use std::io::Write;
use std::marker::PhantomData;
use std::sync::{Arc, Weak};
use std::time::Duration;
use std::{io, mem};
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio::time::sleep;
use tracing::{debug, error, info};

#[derive(Clone)]
struct ProofMPCMessage {
    message: Vec<u8>,
    authority: AuthorityName,
}
pub type ProofMessage = (Proof<1, Lang, PhantomData<()>>, Vec<Value>);

fn authority_name_to_party_id(
    authority_name: AuthorityName,
    epoch_store: &AuthorityPerEpochStore,
) -> PeraResult<PartyID> {
    Ok(epoch_store
        .committee()
        .authority_index(&authority_name)
        // This should never happen, as the validator only accepts messages from committee members
        .ok_or_else(|| {
            PeraError::InvalidCommittee(
                "Received a proof MPC message from a validator that is not in the committee"
                    .to_string(),
            )
        })? as PartyID)
}

struct MPCInstance {
    status: MPCSessionStatus,
    pending_messages: HashMap<PartyID, ProofMessage>,
    consensus_adapter: Arc<dyn SubmitToConsensus>,
    epoch_store: Weak<AuthorityPerEpochStore>,
    /// The threshold number of parties required to participate in each round of the Proof MPC protocol
    mpc_threshold_number_of_parties: usize,
    session_id: ObjectID,
    party: Option<ProofParty>,
}

type ProofPublicParameters =
    maurer::language::PublicParameters<{ maurer::SOUND_PROOFS_REPETITIONS }, Lang>;

impl MPCInstance {
    async fn new(
        consensus_adapter: Arc<dyn SubmitToConsensus>,
        epoch_store: Weak<AuthorityPerEpochStore>,
        mpc_threshold_number_of_parties: usize,
        session_id: ObjectID,
        public_parameters: ProofPublicParameters,
    ) -> Self {
        let mut new_instance = Self {
            status: MPCSessionStatus::Active,
            pending_messages: HashMap::new(),
            consensus_adapter: consensus_adapter.clone(),
            epoch_store: epoch_store.clone(),
            mpc_threshold_number_of_parties,
            session_id,
            party: None,
        };
        match new_instance.start_proof_mpc_flow(public_parameters).await {
            Ok(party) => {
                new_instance.party = Some(party);
                new_instance
            }
            Err(err) => {
                // This should never happen, as there should be on-chain verification on the init transaction, and
                // we are ignoring failed transactions.
                error!("Error initializing the MPC proof flow: {:?}", err);
                new_instance.status = MPCSessionStatus::Finished;
                new_instance
            }
        }
    }

    fn advance(&mut self) -> Option<ConsensusTransaction> {
        let party = mem::take(&mut self.party);
        let Some(party) = party else {
            // This should never happen, the party is initialized in the constructor
            // and advance should not be called more than once simultaneously for the same instance
            error!("Party is not initialized");
            return None;
        };
        let Ok(advance_result) = party.advance(self.pending_messages.clone(), &(), &mut OsRng)
        else {
            // TODO (#263): Mark and punish the malicious validators that caused this advance to fail
            return None;
        };
        match advance_result {
            AdvanceResult::Advance((message, party)) => {
                self.party = Some(party);
                return self.new_signature_mpc_message(message);
            }
            AdvanceResult::Finalize(output) => {
                // TODO (#238): Verify the output and write it to the chain
                println!("Finalized output: {:?}", output);
                self.status = MPCSessionStatus::Finished;
            }
        }
        None
    }

    fn new_signature_mpc_message(&self, message: ProofMessage) -> Option<ConsensusTransaction> {
        let Some(epoch_store) = self.epoch_store.upgrade() else {
            // TODO: (#259) Handle the case when the epoch switched in the middle of the MPC instance
            return None;
        };
        Some(ConsensusTransaction::new_signature_mpc_message(
            epoch_store.name,
            bcs::to_bytes(&message).unwrap(),
            self.session_id.clone(),
        ))
    }

    fn store_message(
        &mut self,
        message: &ProofMPCMessage,
        epoch_store: Arc<AuthorityPerEpochStore>,
    ) -> PeraResult<()> {
        let party_id = authority_name_to_party_id(message.authority, &epoch_store)?;
        if self.pending_messages.contains_key(&party_id) {
            // TODO(#260): Punish an authority that sends multiple messages in the same round
            return Ok(());
        }

        match bcs::from_bytes(&message.message) {
            Ok(message) => {
                self.pending_messages.insert(party_id, message);
                Ok(())
            }
            Err(err) => Err(PeraError::ObjectDeserializationError {
                error: err.to_string(),
            }),
        }
    }

    async fn start_proof_mpc_flow(
        &self,
        public_parameters: ProofPublicParameters,
    ) -> anyhow::Result<ProofParty> {
        let batch_size = 1;
        let party: ProofParty = proof::aggregation::asynchronous::Party::new_proof_round_party(
            public_parameters,
            PhantomData,
            self.mpc_threshold_number_of_parties as PartyID,
            batch_size,
            &mut OsRng,
        )?;
        let Some(epoch_store) = self.epoch_store.upgrade() else {
            // TODO: (#259) Handle the case when the epoch switched in the middle of the MPC instance
            return Err(anyhow!("Epoch store not found"));
        };
        let Ok(advance_result) = party.advance(HashMap::new(), &(), &mut OsRng) else {
            // This should never happen, as there should be on chain verification on the initial event input
            return Err(anyhow!(
                "Error performing first step for session id: {:?}",
                self.session_id
            ));
        };
        match advance_result {
            AdvanceResult::Advance((message, party)) => {
                let message_tx = self.new_signature_mpc_message(message).ok_or_else(|| {
                    anyhow!(
                        "Epoch store switched in the middle of session: {:?}",
                        self.session_id
                    )
                })?;
                self.consensus_adapter
                    .submit_to_consensus(&[message_tx], &epoch_store)
                    .await?;
                Ok(party)
            }
            AdvanceResult::Finalize(_) => Err(anyhow!(
                "Finalization reached unexpectedly: {:?}",
                self.session_id
            )),
        }
    }

    fn handle_message(&mut self, message: ProofMPCMessage) -> PeraResult<()> {
        match self.status {
            MPCSessionStatus::Active => {
                let Some(epoch_store) = self.epoch_store.upgrade() else {
                    // TODO: (#259) Handle the case when the epoch switched in the middle of the MPC instance
                    return Ok(());
                };
                self.store_message(&message, epoch_store)
            }
            MPCSessionStatus::Finished => {
                // Do nothing
                Ok(())
            }
        }
    }
}

/// Possible statuses of an MPC session:
/// - Active: The session is currently running; new messages will be forwarded to the session.
/// - Finished: The session is finished and pending removal; incoming messages will not be forwarded,
/// but will not be marked as malicious.
#[derive(Clone, Copy, PartialEq)]
enum MPCSessionStatus {
    Active,
    Finished,
}

/// The `MPCService` is responsible for managing MPC instances:
/// - keeping track of all MPC instances,
/// - executing all active instances, and
/// - (de)activating instances.
pub struct SignatureMPCManager {
    mpc_instances: HashMap<ObjectID, MPCInstance>,
    /// Used to keep track of the order in which pending instances are received so they are activated in order of arrival.
    pending_instances_queue: VecDeque<ObjectID>,
    // TODO (#257): Make sure the counter is always in sync with the number of active instances.
    active_instances_counter: usize,
    language_public_parameters:
        maurer::language::PublicParameters<{ maurer::SOUND_PROOFS_REPETITIONS }, Lang>,
    consensus_adapter: Arc<dyn SubmitToConsensus>,
    pub epoch_store: Weak<AuthorityPerEpochStore>,
    pub max_active_mpc_instances: usize,
    mpc_threshold_number_of_parties: usize,
}

type Lang = maurer::knowledge_of_discrete_log::Language<secp256k1::Scalar, secp256k1::GroupElement>;
type ProofParty = proof::aggregation::asynchronous::Party<
    maurer::Proof<{ maurer::SOUND_PROOFS_REPETITIONS }, Lang, PhantomData<()>>,
>;

fn generate_language_public_parameters<const REPETITIONS: usize>(
) -> maurer::language::PublicParameters<REPETITIONS, Lang> {
    let secp256k1_scalar_public_parameters = secp256k1::scalar::PublicParameters::default();

    let secp256k1_group_public_parameters = secp256k1::group_element::PublicParameters::default();

    PublicParameters::new::<secp256k1::Scalar, secp256k1::GroupElement>(
        secp256k1_scalar_public_parameters,
        secp256k1_group_public_parameters.clone(),
        secp256k1_group_public_parameters.generator,
    )
}

impl SignatureMPCManager {
    pub fn new(
        consensus_adapter: Arc<dyn SubmitToConsensus>,
        epoch_store: Weak<AuthorityPerEpochStore>,
        max_active_mpc_instances: usize,
        num_of_parties: usize,
    ) -> Self {
        Self {
            mpc_instances: HashMap::new(),
            pending_instances_queue: VecDeque::new(),
            active_instances_counter: 0,
            language_public_parameters: generate_language_public_parameters::<
                { maurer::SOUND_PROOFS_REPETITIONS },
            >(),
            consensus_adapter,
            epoch_store,
            max_active_mpc_instances,
            // TODO (#268): Take into account the validator's voting power
            mpc_threshold_number_of_parties: ((num_of_parties * 2) + 2) / 3,
        }
    }

    /// Filter the relevant MPC events from the transaction events & handle them
    pub async fn handle_mpc_events(&mut self, events: &Vec<Event>) -> anyhow::Result<()> {
        for event in events {
            if CreatedProofMPCEvent::type_() == event.type_ {
                let deserialized_event: CreatedProofMPCEvent = bcs::from_bytes(&event.contents)?;
                self.push_new_mpc_instance(deserialized_event).await;
                debug!("event: CreatedProofMPCEvent {:?}", event);
            };
        }
        Ok(())
    }

    pub async fn handle_end_of_delivery(&mut self) -> PeraResult {
        let txs_to_send: Vec<ConsensusTransaction> = self
            .mpc_instances
            .iter_mut()
            .filter(|(_, instance)| {
                instance.status == MPCSessionStatus::Active
                    && instance.pending_messages.len() >= self.mpc_threshold_number_of_parties
            })
            .collect::<Vec<_>>()
            .par_iter_mut()
            .filter_map(|(_, ref mut instance)| instance.advance())
            .collect();
        let Some(epoch_store) = self.epoch_store.upgrade() else {
            // TODO: (#259) Handle the case when the epoch switched in the middle of the MPC instance
            return Ok(());
        };
        self.consensus_adapter
            .submit_to_consensus(&txs_to_send, &epoch_store)
            .await
    }

    pub fn handle_message(
        &mut self,
        message: &[u8],
        authority_name: AuthorityName,
        session_id: ObjectID,
    ) -> PeraResult<()> {
        let Some(mut instance) = self.mpc_instances.get_mut(&session_id) else {
            // TODO (#261): Punish a validator that sends a message related to a non-existing mpc instance
            return Ok(());
        };
        instance.handle_message(ProofMPCMessage {
            message: message.to_vec(),
            authority: authority_name,
        })
    }

    /// Spawns a new MPC instance if the number of active instances is below the limit
    /// Otherwise, adds the instance to the pending queue
    async fn push_new_mpc_instance(&mut self, event: CreatedProofMPCEvent) {
        if self.mpc_instances.contains_key(&event.session_id.bytes) {
            // This should never happen, as the session ID is a move UniqueID
            error!(
                "Received start flow event for session ID {:?} that already exists",
                event.session_id
            );
            return;
        }

        info!(
            "Received start flow event for session ID {:?}",
            event.session_id
        );

        if self.active_instances_counter > self.max_active_mpc_instances {
            self.pending_instances_queue
                .push_back(event.session_id.bytes);
            info!(
                "Added MPCInstance to pending queue for session_id {:?}",
                event.session_id
            );
            return;
        }
        let new_instance = MPCInstance::new(
            Arc::clone(&self.consensus_adapter),
            self.epoch_store.clone(),
            self.mpc_threshold_number_of_parties,
            event.session_id.clone().bytes,
            self.language_public_parameters.clone(),
        )
        .await;
        self.mpc_instances
            .insert(event.session_id.clone().bytes, new_instance);
        self.active_instances_counter += 1;
        info!(
            "Added MPCInstance to MPC manager for session_id {:?}",
            event.session_id
        );
    }
}
