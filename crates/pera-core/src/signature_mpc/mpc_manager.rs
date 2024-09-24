use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::consensus_adapter::SubmitToConsensus;
use crate::signature_mpc::mpc_events::{CreatedProofMPCEvent, MPCEvent};
use anyhow::anyhow;
use group::{secp256k1, GroupElement};
use maurer::knowledge_of_discrete_log::PublicParameters;
use pera_types::base_types::{AuthorityName, ObjectID};
use pera_types::event::Event;
use pera_types::messages_consensus::ConsensusTransaction;
use proof::mpc::{AdvanceResult, Party};
use rand_core::OsRng;
use std::collections::{HashMap, VecDeque};
use std::io;
use std::io::Write;
use std::marker::PhantomData;
use std::sync::{Arc, Weak};
use std::time::Duration;
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio::time::sleep;
use tracing::debug;

#[derive(Debug)]
pub enum MPCInput {
    InitEvent(CreatedProofMPCEvent),
    Message,
}

struct MPCInstance {
    status: MPCSessionStatus,
    /// The channel to send message to this instance
    input_receiver: Option<mpsc::Sender<MPCInput>>,
    pending_messages: Vec<MPCInput>,
}

impl MPCInstance {
    fn set_active(&mut self) {
        self.status = MPCSessionStatus::Active;
        // TODO (#256): Replace hard coded 100 with the number of validators times 10
        let (messages_handler_sender, messages_handler_receiver) = mpsc::channel(100);
        self.input_receiver = Some(messages_handler_sender);
        self.spawn_mpc_messages_handler(messages_handler_receiver);
    }
    // /// Spawns an asynchronous task to handle incoming messages for a new MPC instance.
    // /// The [`SignatureMPCManager`] will forward any message related to that instance to this channel.
    // fn spawn_mpc_messages_handler(&self, mut receiver: mpsc::Receiver<MPCInput>) {
    //     let public_parameters = self.language_public_parameters.clone();
    //     let consensus_adapter = Arc::clone(&self.consensus_adapter);
    //     let epoch_store = self.epoch_store.clone();
    //     let threshold = self.threshold.clone();
    //
    //     tokio::spawn(async move {
    //         let mut party: ProofParty;
    //         while let Some(message) = receiver.recv().await {
    //             match message {
    //                 MPCInput::InitEvent(_) => {
    //                     party = match Self::handle_mpc_proof_init_event(
    //                         public_parameters.clone(),
    //                         consensus_adapter.clone(),
    //                         epoch_store.clone(),
    //                         threshold,
    //                     )
    //                         .await
    //                     {
    //                         Ok(party) => party,
    //                         Err(err) => {
    //                             // This should never happen, as there should be on-chain verification on the init transaction
    //                             return;
    //                         }
    //                     };
    //                 }
    //                 MPCInput::Message => {
    //                     // TODO (#235): Implement MPC messages handling
    //                 }
    //             }
    //         }
    //     });
    // }

    /// Spawns an asynchronous task to handle incoming messages.
    /// The [`MPCService`] will forward any message related to that instance to this channel.
    fn spawn_mpc_messages_handler(
        &self, mut receiver: mpsc::Receiver<MPCInput>,
        language_public_parameters: maurer::language::PublicParameters<{ maurer::SOUND_PROOFS_REPETITIONS }, Lang>,
        consensus_adapter: Arc<dyn SubmitToConsensus>,
        epoch_store: Weak<AuthorityPerEpochStore>,
        threshold: usize,
    ) {
        tokio::spawn(async move {
            let mut party: ProofParty;
            while let Some(message) = receiver.recv().await {
                match message {
                    MPCInput::InitEvent(_) => {
                        party = match Self::handle_mpc_proof_init_event(
                            language_public_parameters.clone(),
                            consensus_adapter.clone(),
                            epoch_store.clone(),
                            threshold,
                        )
                            .await
                        {
                            Ok(party) => party,
                            Err(err) => {
                                // This should never happen, as there should be on-chain verification on the init transaction
                                return;
                            }
                        };
                    }
                    MPCInput::Message => {
                        // TODO (#235): Implement MPC messages handling
                    }
                }

                // TODO (#235): Implement MPC messages handling
            }
        });
    }

    async fn handle_message(&mut self, message: MPCInput) {
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

    pub fn handle_mpc_message(
        &mut self,
        message: &[u8],
        authority_name: AuthorityName,
    ) -> anyhow::Result<()> {
        // TODO (#235): Implement MPC messages handling
        Ok(())
    }

    /// Spawns a new MPC instance if the number of active instances is below the limit
    /// Otherwise, adds the instance to the pending queue
    async fn handle_proof_init_event(&mut self, event: CreatedProofMPCEvent) {
        info!(
            "Received start flow event for session ID {:?}",
            event.session_id
        );

        let mut new_instance = MPCInstance {
            status: MPCSessionStatus::Pending,
            input_receiver: None,
            pending_messages: vec![],
        };

        // Activate the instance if possible
        if self.active_instances_counter < MAX_ACTIVE_MPC_INSTANCES {
            new_instance.set_active();
            self.active_instances_counter += 1;
        } else {
            self.pending_instances_queue
                .push_back(event.session_id.bytes);
        };
        new_instance.handle_message(MPCInput::InitEvent(event.clone())).await;

        self.mpc_instances
            .insert(event.session_id.clone().bytes, new_instance);

        info!(
            "Added MPCInstance to service for session_id {:?}",
            event.session_id
        );
    }
}
