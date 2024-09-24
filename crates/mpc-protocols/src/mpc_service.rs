use crate::mpc_events::{CreatedProofMPCEvent, MPCEvent};
use log::{debug, info};
use pera_types::base_types::ObjectID;
use pera_types::event::Event;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use tokio::sync::{mpsc, Mutex, RwLock};

// TODO (#253): Make this configurable from a config file
const MAX_ACTIVE_MPC_INSTANCES: usize = 100;

pub enum MPCInput {
    InitEvent(CreatedProofMPCEvent),
    Message,
}

struct MPCInstance {
    status: MPCSessionStatus,
    /// The channel to send message to this instance's message handler thread when this instance is active
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

    /// Spawns an asynchronous task to handle incoming messages.
    /// The [`MPCService`] will forward any message related to that instance to this channel.
    fn spawn_mpc_messages_handler(&self, mut receiver: mpsc::Receiver<MPCInput>) {
        tokio::spawn(async move {
            while let Some(message) = receiver.recv().await {
                // TODO (#235): Implement MPC messages handling
            }
        });
    }

    fn handle_message(&mut self, message: MPCInput) {
        match self.status {
            MPCSessionStatus::Active => {
                if let Some(input_receiver) = &self.input_receiver {
                    let _ = input_receiver.send(message);
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

/// Possible status of an MPC session
/// - Active: The session is currently running, new messages will be forwarded to the session
/// - Pending: The session is waiting for a slot to become active, when the init received there were already `MAX_ACTIVE_MPC_INSTANCES` active sessions
/// - Finished: The session has finished, no more messages will be forwarded. The session will be removed from the service after a timeout.
/// We want to keep it for some time so leftover messages related to it won't be treated as malicious.
#[derive(Clone, Copy)]
enum MPCSessionStatus {
    Active,
    Pending,
    Finished,
}

/// The `MPCService` is responsible for managing MPC instances:
/// - It keeps track of all MPC instances
/// - Runs the MPC session for each active instance
/// - Ensures that the number of active sessions does not go over `MAX_ACTIVE_MPC_INSTANCES` at the same time
pub struct MPCService {
    mpc_instances: HashMap<ObjectID, MPCInstance>,
    /// Being used to keep track of the order of the received pending instances, so we'll know which one to launch first.
    pending_instances_queue: VecDeque<ObjectID>,
    // TODO (#257): Make sure the counter is always in sync with the number of active instances.
    active_instances_counter: usize,
}

impl MPCService {
    pub fn new() -> Self {
        Self {
            mpc_instances: HashMap::new(),
            pending_instances_queue: VecDeque::new(),
            active_instances_counter: 0,
        }
    }


    /// Filter the relevant MPC events from the transaction events & handle them
    pub fn handle_mpc_events(&mut self, events: &Vec<Event>) -> anyhow::Result<()> {
        for event in events {
            if CreatedProofMPCEvent::type_() == event.type_ {
                let deserialized_event: CreatedProofMPCEvent = bcs::from_bytes(&event.contents)?;
                self.handle_proof_init_event(deserialized_event);
                debug!("event: CreatedProofMPCEvent {:?}", event);
            };
        }
        Ok(())
    }

    /// Handles a proof initialization event
    /// Spawns a new MPC instance if the number of active instances is below the limit
    /// Otherwise, adds the instance to the pending queue
    /// Handles a proof initialization event
    /// Spawns a new MPC instance if the number of active instances is below the limit
    /// Otherwise, adds the instance to the pending queue
    fn handle_proof_init_event(&mut self, event: CreatedProofMPCEvent) {
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
            new_instance.handle_message(MPCInput::InitEvent(event.clone()));
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
    // fn handle_proof_init_event(&mut self, event: CreatedProofMPCEvent) {
    //     info!(
    //         "Received start flow event for session ID {:?}",
    //         event.session_id
    //     );
    //     // If the number of active instances exceeds the limit, add to pending
    //     if self.active_instances_counter >= MAX_ACTIVE_MPC_INSTANCES {
    //         self.mpc_instances.insert(
    //             event.session_id.clone().bytes,
    //             MPCInstance {
    //                 status: MPCSessionStatus::Pending,
    //                 input_receiver: None,
    //                 pending_messages: vec![],
    //             },
    //         );
    //         self.pending_instances_queue
    //             .push_back(event.session_id.bytes);
    //         return;
    //     }
    //     // TODO (#256): Replace hard coded 100 with the number of validators times 10
    //     let (messages_handler_sender, messages_handler_receiver) = mpsc::channel(100);
    //     self.spawn_mpc_messages_handler(messages_handler_receiver);
    //     let _ = messages_handler_sender.send(MPCInput::InitEvent(event.clone()));
    //     self.mpc_instances.insert(
    //         event.session_id.clone().bytes,
    //         MPCInstance {
    //             status: MPCSessionStatus::Active,
    //             input_receiver: Some(messages_handler_sender),
    //             pending_messages: vec![],
    //         },
    //     );
    //     self.active_instances_counter += 1;
    //     info!(
    //         "Added MPCInstance to service for session_id {:?}",
    //         event.session_id
    //     );
    // }
}
