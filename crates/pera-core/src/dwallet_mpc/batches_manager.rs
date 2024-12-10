use pera_types::base_types::ObjectID;
use std::collections::{HashMap, HashSet};
use pera_types::dwallet_mpc_error::DwalletMPCResult;
use pera_types::error::PeraError::DwalletMPCError;
use pera_types::messages_dwallet_mpc::{MPCRound, SessionInfo};

/// A struct to hold the batched sign session data.
pub struct BatchedSignSession {
    /// A map that contains the ready signatures,
    /// keyed by their hashed message.
    /// When this map contains all the hashed messages,
    /// the batched sign session is ready to be written to the chain.
    /// HashedMsg -> Sign Output.
    pub hashed_msg_to_signature: HashMap<Vec<u8>, Vec<u8>>,
    /// A list of all the messages that need to be signed, in the order they were received.
    /// The output list of signatures will be written to the chain in the same order.
    pub ordered_messages: Vec<Vec<u8>>,
}

pub struct DWalletMPCBatchesManager {
    /// The batched sign sessions that are currently being processed.
    pub batched_sign_sessions: HashMap<ObjectID, BatchedSignSession>,
}

impl DWalletMPCBatchesManager {
    pub fn new() -> Self {
        DWalletMPCBatchesManager {
            batched_sign_sessions: HashMap::new(),
        }
    }

    pub fn handle_new_event(&mut self, session_info: &SessionInfo) {
        if let MPCRound::BatchedSign(hashed_messages) = &session_info.mpc_round {
            let mut seen = HashSet::new();
            let messages_without_duplicates = hashed_messages
                .clone()
                .into_iter()
                .filter(|x| seen.insert(x.clone()))
                .collect();
            self.batched_sign_sessions.insert(
                session_info.session_id,
                BatchedSignSession {
                    hashed_msg_to_signature: HashMap::new(),
                    ordered_messages: messages_without_duplicates,
                },
            );
        }
    }

    pub fn store_message(&mut self, session_id: &ObjectID, key: Vec<u8>, message: Vec<u8>) -> DwalletMPCResult<()> {
        let batched_sign_session = self
            .batched_sign_sessions
            .get_mut(session_id)
            .ok_or(DwalletMPCError)?;
        batched_sign_session
            .hashed_msg_to_signature
            .insert(key, message);
    }

    pub fn is_batch_completed
}
