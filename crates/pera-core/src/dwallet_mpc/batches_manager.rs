use pera_types::base_types::ObjectID;
use pera_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use pera_types::messages_dwallet_mpc::{MPCRound, SessionInfo};
use std::collections::{HashMap, HashSet};

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

/// A struct to manage the batched sign sessions.
/// It stores all the batched sign sessions that are currently being processed,
/// and decides whether a batch is completed by checking if all the messages were signed.
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

    /// Handle a new event by initializing a new batched sign session if the event is a batched sign event.
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

    /// Store a verified output in its corresponding batch.
    pub fn store_verified_output(
        &mut self,
        session_id: ObjectID,
        key: Vec<u8>,
        message: Vec<u8>,
    ) -> DwalletMPCResult<()> {
        let batched_sign_session = self
            .batched_sign_sessions
            .get_mut(&session_id)
            .ok_or(DwalletMPCError::MPCSessionNotFound { session_id })?;
        batched_sign_session
            .hashed_msg_to_signature
            .insert(key, message);
        Ok(())
    }

    /// Check if a batched sign session is completed.
    /// If it is, return the output of the entire batch.
    /// Otherwise, return None.
    pub fn is_batch_completed(&self, session_id: ObjectID) -> DwalletMPCResult<Option<Vec<u8>>> {
        let batched_sign_session = self
            .batched_sign_sessions
            .get(&session_id)
            .ok_or(DwalletMPCError::MPCSessionNotFound { session_id })?;
        return if batched_sign_session.hashed_msg_to_signature.values().len()
            == batched_sign_session.ordered_messages.len()
        {
            let new_output: Vec<Vec<u8>> = batched_sign_session
                .ordered_messages
                .iter()
                .map(|msg| {
                    Ok(batched_sign_session
                        .hashed_msg_to_signature
                        .get(msg)
                        .ok_or(DwalletMPCError::MissingMessageInBatch(msg.clone()))?
                        .clone())
                })
                .collect::<DwalletMPCResult<Vec<Vec<u8>>>>()?;
            Ok(Some(bcs::to_bytes(&new_output)?))
        } else {
            Ok(None)
        };
    }
}
