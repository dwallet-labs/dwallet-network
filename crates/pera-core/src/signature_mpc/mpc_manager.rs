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
use mockall::Any;
use pera_types::base_types::{AuthorityName, ObjectID};
use pera_types::event::Event;
use pera_types::messages_consensus::ConsensusTransaction;
use proof::mpc::{AdvanceResult, Party};
use rand_core::OsRng;
use schemars::_private::NoSerialize;
use std::collections::{HashMap, VecDeque};
use std::io;
use std::io::Write;
use std::marker::PhantomData;
use std::sync::{Arc, Weak};
use std::time::Duration;
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio::time::sleep;
use tracing::{debug, error, info};

struct ProofMPCMessage {
    message: Vec<u8>,
    authority: AuthorityName,
}

fn authority_name_to_party_id(
    authority_name: AuthorityName,
    epoch_store: &AuthorityPerEpochStore,
) -> anyhow::Result<PartyID> {
    Ok(epoch_store
        .committee()
        .authority_index(&authority_name)
        .ok_or_else(|| anyhow!("Authority {} not found in the committee", authority_name))?
        as PartyID)
}

struct MPCInstance {
    status: MPCSessionStatus,
    /// The channel to send message to this instance
    input_receiver: Option<mpsc::Sender<ProofMPCMessage>>,
    pending_messages: Vec<ProofMPCMessage>,
    language_public_parameters:
        maurer::language::PublicParameters<{ maurer::SOUND_PROOFS_REPETITIONS }, Lang>,
    consensus_adapter: Arc<dyn SubmitToConsensus>,
    epoch_store: Weak<AuthorityPerEpochStore>,
    threshold: usize,
    session_id: ObjectID,
}

impl MPCInstance {
    async fn set_active(&mut self) {
        match Self::start_proof_mpc_flow(
            self.language_public_parameters.clone(),
            Arc::clone(&self.consensus_adapter),
            self.epoch_store.clone(),
            self.threshold.clone(),
            self.session_id,
        )
        .await
        {
            Ok((party, message)) => {
                // TODO (#256): Replace hard coded 100 with the number of validators times 10
                let (messages_handler_sender, messages_handler_receiver) = mpsc::channel(100);
                self.input_receiver = Some(messages_handler_sender);
                self.status = MPCSessionStatus::Active;
                self.spawn_mpc_messages_handler(messages_handler_receiver, party, message);
            }
            Err(err) => {
                // This should never happen, as there should be on-chain verification on the init transaction, and
                // we are ignoring failed transactions.
                error!("Error initializing the MPC proof flow: {:?}", err);
            }
        }
    }

    /// Spawns an asynchronous task to handle incoming messages.
    /// The [`MPCService`] will forward any message related to that instance to this channel.
    fn spawn_mpc_messages_handler(
        &self,
        mut receiver: mpsc::Receiver<ProofMPCMessage>,
        mut party: ProofParty,
        first_message: Vec<u8>,
    ) {
        let consensus_adapter = Arc::clone(&self.consensus_adapter);
        let epoch_store = self.epoch_store.clone();
        let threshold = self.threshold.clone();
        let session_id = self.session_id.clone();

        tokio::spawn(async move {
            let mut messages = HashMap::new();

            let Some(epoch_store) = epoch_store.upgrade() else {
                // TODO: (#259) Handle the case when the epoch switched in the middle of the MPC instance
                return;
            };

            let _ =
                Self::insert_mpc_message(&first_message, &mut messages, Arc::clone(&epoch_store));

            while let Some(message) = receiver.recv().await {
                let _ = Self::insert_mpc_message(
                    &message.message,
                    &mut messages,
                    Arc::clone(&epoch_store),
                );
                if messages.len() == threshold {
                    party = match party.advance(messages.clone(), &(), &mut OsRng) {
                        Ok(advance_result) => match advance_result {
                            AdvanceResult::Advance((message, new_party)) => {
                                let message_tx = ConsensusTransaction::new_signature_mpc_message(
                                    epoch_store.name,
                                    bcs::to_bytes(&message).unwrap(),
                                    session_id,
                                );
                                let _ = consensus_adapter
                                    .submit_to_consensus(&[message_tx], &epoch_store)
                                    .await;
                                new_party
                            }
                            AdvanceResult::Finalize(output) => {
                                // TODO (#238): Verify the output and write it to the chain
                                return;
                            }
                        },
                        _ => {
                            return;
                        }
                    }
                }
            }
        });
    }

    fn insert_mpc_message(
        message: &Vec<u8>,
        mut messages: &mut HashMap<PartyID, (Proof<1, Lang, PhantomData<()>>, Vec<Value>)>,
        epoch_store: Arc<AuthorityPerEpochStore>,
    ) -> anyhow::Result<()> {
        let party_id = authority_name_to_party_id(epoch_store.name, &epoch_store)?;

        if messages.contains_key(&party_id) {
            // TODO(#260): Punish an authority that sends multiple messages in the same round
            return Err(anyhow!(
                "Authority {} already sent a message in this round",
                epoch_store.name
            ));
        }

        match bcs::from_bytes(&message) {
            Ok(message) => {
                messages.insert(party_id, message);
                Ok(())
            }
            Err(err) => Err(anyhow!("Error deserializing the first message: {:?}", err)),
        }
    }

    async fn start_proof_mpc_flow(
        public_parameters: maurer::language::PublicParameters<
            { maurer::SOUND_PROOFS_REPETITIONS },
            Lang,
        >,
        consensus_adapter: Arc<dyn SubmitToConsensus>,
        epoch_store: Weak<AuthorityPerEpochStore>,
        threshold: usize,
        session_id: ObjectID,
    ) -> anyhow::Result<(ProofParty, Vec<u8>)> {
        let batch_size = 1;
        let party: ProofParty = proof::aggregation::asynchronous::Party::new_proof_round_party(
            public_parameters,
            PhantomData,
            threshold as PartyID,
            batch_size,
            &mut OsRng,
        )?;

        match party.advance(HashMap::new(), &(), &mut OsRng) {
            Ok(advance_result) => match advance_result {
                AdvanceResult::Advance((message, new_party)) => {
                    let Some(epoch_store) = epoch_store.upgrade() else {
                        // TODO: (#259) Handle the case when the epoch switched in the middle of the MPC instance
                        return Err(anyhow!("Epoch store not found"));
                    };
                    let message_tx = ConsensusTransaction::new_signature_mpc_message(
                        epoch_store.name,
                        bcs::to_bytes(&message)?,
                        session_id,
                    );
                    consensus_adapter
                        .submit_to_consensus(&vec![message_tx], &epoch_store)
                        .await?;
                    Ok((new_party, bcs::to_bytes(&message)?))
                }
                AdvanceResult::Finalize(output) => {
                    Err(anyhow!("Finalization reached unexpectedly"))
                }
            },
            Err(err) => Err(anyhow!("Error advancing the party: {:?}", err)),
        }
    }

    async fn handle_message(&mut self, message: ProofMPCMessage) {
        match self.status {
            MPCSessionStatus::Active => {
                if let Some(input_receiver) = &self.input_receiver {
                    let _ = input_receiver.send(message).await;
                }
            }
            MPCSessionStatus::Pending => {
                self.pending_messages.push(message);
            }
            MPCSessionStatus::Finished => {
                // Do nothing
            }
        }
    }
}

/// Possible statuses of an MPC session:
/// - Active: The session is currently running; new messages will be forwarded to the session.
/// - Pending: Too many active instances are running atm; incoming messages will be queued. The session
/// will be activated once there is room, i.e. when enough active instances finish.
/// - Finished: The session is finished and pending removal; incoming messages will not be forwarded.
#[derive(Clone, Copy)]
enum MPCSessionStatus {
    Active,
    Pending,
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
    threshold: usize,
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
        epoch_store_weak: Weak<AuthorityPerEpochStore>,
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
            epoch_store: epoch_store_weak,
            max_active_mpc_instances,
            threshold: ((num_of_parties / 3) * 2) + 1,
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

    pub async fn handle_mpc_message(
        &mut self,
        message: &[u8],
        authority_name: AuthorityName,
        session_id: ObjectID,
    ) -> anyhow::Result<()> {
        let mut instance = self
            .mpc_instances
            .get_mut(&session_id)
            // TODO (#261): Punish a validator that sends a message related to a non-existing mpc instance
            .ok_or_else(|| anyhow!("MPC instance not found"))?;
        instance
            .handle_message(ProofMPCMessage {
                message: message.to_vec(),
                authority: authority_name,
            })
            .await;
        Ok(())
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

        let mut new_instance = MPCInstance {
            status: MPCSessionStatus::Pending,
            input_receiver: None,
            pending_messages: vec![],
            language_public_parameters: self.language_public_parameters.clone(),
            consensus_adapter: Arc::clone(&self.consensus_adapter),
            epoch_store: self.epoch_store.clone(),
            threshold: self.threshold,
            session_id: event.session_id.bytes.clone(),
        };

        // Activate the instance if possible
        if self.active_instances_counter < self.max_active_mpc_instances {
            new_instance.set_active().await;
            self.active_instances_counter += 1;
        } else {
            self.pending_instances_queue
                .push_back(event.session_id.bytes);
        };

        self.mpc_instances
            .insert(event.session_id.clone().bytes, new_instance);

        info!(
            "Added MPCInstance to service for session_id {:?}",
            event.session_id
        );
    }
}
