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
use schemars::_private::NoSerialize;
use std::cmp::PartialEq;
use std::collections::{HashMap, VecDeque};
use std::future::Future;
use std::io;
use std::io::Write;
use std::marker::PhantomData;
use std::sync::{Arc, Weak};
use std::time::Duration;
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
    pending_messages: HashMap<PartyID, ProofMessage>,
    consensus_adapter: Arc<dyn SubmitToConsensus>,
    epoch_store: Weak<AuthorityPerEpochStore>,
    /// The threshold number of parties required to participate in each round of the Proof MPC protocol
    mpc_threshold_number_of_parties: usize,
    session_id: ObjectID,
    party: ProofParty,
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
        match Self::start_proof_mpc_flow(
            public_parameters,
            Arc::clone(&consensus_adapter),
            epoch_store.clone(),
            mpc_threshold_number_of_parties,
            session_id,
        )
        .await
        {
            Ok(party) => {
                Self {
                    status: MPCSessionStatus::Active,
                    pending_messages: HashMap::new(),
                    consensus_adapter,
                    epoch_store,
                    mpc_threshold_number_of_parties,
                    session_id,
                    party,
                }
            }
            Err(err) => {
                // This should never happen, as there should be on-chain verification on the init transaction, and
                // we are ignoring failed transactions.
                panic!("Error initializing the MPC proof flow: {:?}", err);
            }
        }
    }

    // /// Spawns an asynchronous task to handle incoming messages.
    // /// The [`MPCService`] will forward any message related to that instance to this channel.
    // fn spawn_mpc_messages_handler(
    //     &self,
    //     mut receiver: mpsc::Receiver<ProofMPCMessage>,
    //     mut party: ProofParty,
    // ) {
    //     let consensus_adapter = Arc::clone(&self.consensus_adapter);
    //     let epoch_store = self.epoch_store.clone();
    //     let threshold = self.mpc_threshold_number_of_parties;
    //     let session_id = self.session_id;
    //     tokio::spawn(async move {
    //         let mut messages = HashMap::new();
    //
    //         let Some(epoch_store) = epoch_store.upgrade() else {
    //             // TODO: (#259) Handle the case when the epoch switched in the middle of the MPC instance
    //             return;
    //         };
    //         while let Some(message) = receiver.recv().await {
    //             match Self::store_message_and_advance(
    //                 party,
    //                 consensus_adapter.clone(),
    //                 threshold,
    //                 &session_id,
    //                 &mut messages,
    //                 epoch_store.clone(),
    //                 &message,
    //             )
    //             .await
    //             {
    //                 Some(new_party) => party = new_party,
    //                 None => {
    //                     return;
    //                 }
    //             }
    //         }
    //     });
    // }

    async fn advance_if_possible(&mut self) {
        if self.pending_messages.len() >= self.mpc_threshold_number_of_parties {
            self.party.clone().advance(
                self.pending_messages.clone(),
                &(),
                &mut OsRng,
            );
        }
    }

    // async fn store_message_and_advance(
    //     mut party: ProofParty,
    //     consensus_adapter: Arc<dyn SubmitToConsensus>,
    //     threshold: usize,
    //     session_id: &ObjectID,
    //     mut messages: &mut HashMap<PartyID, (Proof<1, Lang, PhantomData<()>>, Vec<Value>)>,
    //     epoch_store: Arc<AuthorityPerEpochStore>,
    //     message: &ProofMPCMessage,
    // ) -> Option<ProofParty> {
    //     let _ = Self::store_message(&message, &mut messages, Arc::clone(&epoch_store));
    //     if messages.len() == threshold {
    //         return Self::advance_party(
    //             party,
    //             consensus_adapter,
    //             session_id,
    //             &mut messages,
    //             epoch_store,
    //         )
    //         .await;
    //     }
    //     Some(party)
    // }

    async fn advance(self) -> Option<ProofParty> {
        let messages = self.pending_messages.clone();
        self.party.advance(messages, &(), &mut OsRng);
        None
        // {
            // Ok(advance_result) => match advance_result {
            //     AdvanceResult::Advance((message, new_party)) => {
            //         let message_tx = ConsensusTransaction::new_signature_mpc_message(
            //             epoch_store.name,
            //             bcs::to_bytes(&message).unwrap(),
            //             session_id.clone(),
            //         );
            //         // TODO (#270): Handle Proof flow in a synchronous way & propagate this result
            //         consensus_adapter
            //             .submit_to_consensus(&[message_tx], &epoch_store)
            //             .await
            //             .expect("failed to submit to consensus");
            //         Some(new_party)
            //     }
            //     AdvanceResult::Finalize(output) => {
            //         // TODO (#238): Verify the output and write it to the chain
            //         println!("Finalized output: {:?}", output);
            //         None
            //     }
            // },
            // Err(_) => {
            //     // TODO (#263): Mark the sender as malicious
            //     None
            // }
        // }
    }

    fn store_message(
        message: &ProofMPCMessage,
        mut messages: &mut HashMap<PartyID, (Proof<1, Lang, PhantomData<()>>, Vec<Value>)>,
        epoch_store: Arc<AuthorityPerEpochStore>,
    ) -> anyhow::Result<()> {
        let party_id = authority_name_to_party_id(message.authority, &epoch_store)?;
        if messages.contains_key(&party_id) {
            // TODO(#260): Punish an authority that sends multiple messages in the same round
            return Err(anyhow!(
                "Authority {} already sent a message in this round",
                epoch_store.name
            ));
        }

        match bcs::from_bytes(&message.message) {
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
    ) -> anyhow::Result<ProofParty> {
        let batch_size = 1;
        let party_state: ProofParty =
            proof::aggregation::asynchronous::Party::new_proof_round_party(
                public_parameters,
                PhantomData,
                threshold as PartyID,
                batch_size,
                &mut OsRng,
            )?;
        let mut new_party = Self {
            status: MPCSessionStatus::Active,
            pending_messages: HashMap::new(),
            consensus_adapter,
            epoch_store,
            mpc_threshold_number_of_parties: threshold,
            session_id,
            party: party_state,
        };

        match new_party.advance().await
        {
            None => Err(anyhow!("Finalization reached unexpectedly")),
            Some(new_party) => Ok(new_party),
        }
    }

    fn save_message(&mut self, message: ProofMPCMessage) {
        match self.status {
            MPCSessionStatus::Active | MPCSessionStatus::Pending => {
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
            threshold: ((num_of_parties * 2) + 2) / 3,
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
        // iterate over all active instances copilot do it don't add more docs write the code
        for (session_id, mut instance) in self.mpc_instances.iter_mut() {
            if instance.status == MPCSessionStatus::Active {
                if instance.pending_messages.len() >= self.threshold {
                    instance.advance_if_possible().await;
                }
            }
        }
        Ok(())
    }

    pub fn handle_mpc_message(
        &mut self,
        message: &[u8],
        authority_name: AuthorityName,
        session_id: ObjectID,
    ) -> PeraResult<()> {
        let Some(mut instance) = self.mpc_instances.get_mut(&session_id) else {
            // TODO (#261): Punish a validator that sends a message related to a non-existing mpc instance
            return Ok(());
        };
        instance.save_message(ProofMPCMessage {
            message: message.to_vec(),
            authority: authority_name,
        });
        Ok(())
    }

    /// Spawns a new MPC instance if the number of active instances is below the limit
    /// Otherwise, adds the instance to the pending queue
    async fn push_new_mpc_instance(&mut self, event: CreatedProofMPCEvent) -> PeraResult<()> {
        if self.mpc_instances.contains_key(&event.session_id.bytes) {
            // This should never happen, as the session ID is a move UniqueID
            error!(
                "Received start flow event for session ID {:?} that already exists",
                event.session_id
            );
        }

        info!(
            "Received start flow event for session ID {:?}",
            event.session_id
        );

        // Activate the instance if possible
        if self.active_instances_counter < self.max_active_mpc_instances {
            let new_instance = MPCInstance::new(
                Arc::clone(&self.consensus_adapter),
                self.epoch_store.clone(),
                self.threshold,
                event.session_id.clone().bytes,
                self.language_public_parameters.clone(),
            ).await;
            self.mpc_instances
                .insert(event.session_id.clone().bytes, new_instance);
            self.active_instances_counter += 1;
            info!(
                "Added MPCInstance to MPC manager for session_id {:?}",
                event.session_id
            );
        } else {
            self.pending_instances_queue
                .push_back(event.session_id.bytes);
            info!(
                "Added MPCInstance to pending queue for session_id {:?}",
                event.session_id
            );
        };
        Ok(())
    }
}
