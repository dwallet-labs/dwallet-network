//! A module with logic to manage the batched sessions.
//! The struct, [`DWalletMPCBatchesManager`] stores all the batched
//! sessions that are currently being processed, and decides whether a batch is
//! completed by checking if it received all the expected VERIFIED batch outputs.
//! When a batch is completed, it returns the output of the entire batch,
//! which can be written to the chain through a system transaction.
use crate::dwallet_mpc::mpc_session::AsyncProtocol;
use dwallet_mpc_types::dwallet_mpc::{MPCMessage, MPCPublicOutput};
use sui_types::base_types::ObjectID;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_dwallet_mpc::{MPCProtocolInitData, SessionInfo, SingleSignSessionData};
use std::collections::{HashMap, HashSet};

/// Structs to hold the batches sign session data.
///
/// Stores the batch data and every verified output when it's ready.
/// When the entire batch is completed, it returns the batch output,
/// which is being written to the chain at once through
/// a system transaction.
pub struct BatchedSignSession {
    /// A map that contains the ready signatures,
    /// keyed by their hashed message.
    /// When this map contains all the hashed messages,
    /// the batched sign session is ready to be written to the chain.
    /// HashedMsg -> Sign Output.
    hashed_msg_to_signature: HashMap<Vec<u8>, MPCPublicOutput>,
    /// A list of all the messages that need to be signed,
    /// in the order they were received.
    /// The output list of signatures will be written to the chain in the same order.
    ordered_messages: Vec<MPCMessage>,
}

/// A struct to hold the batches presign sessions data,
/// equivalent to the [`BatchedSignSession`] struct, but for presigns.
pub struct BatchedPresignSession {
    /// The amount of presigns that will get created in this batch.
    batch_size: u64,
    /// A map between the first presign session ID to the verified, serialized presign object.
    /// The first round's session ID is needed for the centralized sign flow.
    verified_presigns: Vec<(ObjectID, MPCPublicOutput)>,
}

/// A struct to manage the batched sign sessions.
/// It stores all the batched sign sessions that are currently being processed,
/// and decides whether a batch is completed by checking if all the messages were signed.
pub struct DWalletMPCBatchesManager {
    /// The batched sign sessions that are currently being processed.
    batched_sign_sessions: HashMap<ObjectID, BatchedSignSession>,
    batched_presign_sessions: HashMap<ObjectID, BatchedPresignSession>,
}

type NoncePublicShareAndEncryptionOfMaskedNonceSharePart =
<AsyncProtocol as twopc_mpc::presign::Protocol>::NoncePublicShareAndEncryptionOfMaskedNonceSharePart;

impl DWalletMPCBatchesManager {
    pub fn new() -> Self {
        DWalletMPCBatchesManager {
            batched_sign_sessions: HashMap::new(),
            batched_presign_sessions: HashMap::new(),
        }
    }

    /// Handle a new event by initializing a new batched session
    /// if the event is a start batch event.
    /// Clears duplicate messages if the user/fullnode sends the same message twice.
    pub(crate) fn store_new_session(&mut self, session_info: &SessionInfo) {
        match &session_info.mpc_round {
            MPCProtocolInitData::BatchedSign(hashed_messages) => {
                let mut seen = HashSet::new();
                let unique_messages = hashed_messages
                    .clone()
                    .into_iter()
                    .filter(|x| seen.insert(x.clone()))
                    .collect();
                self.batched_sign_sessions.insert(
                    session_info.session_id,
                    BatchedSignSession {
                        hashed_msg_to_signature: HashMap::new(),
                        ordered_messages: unique_messages,
                    },
                );
            }
            MPCProtocolInitData::BatchedPresign(batch_size) => {
                self.batched_presign_sessions.insert(
                    session_info.session_id,
                    BatchedPresignSession {
                        batch_size: *batch_size,
                        verified_presigns: vec![],
                    },
                );
            }
            _ => {}
        }
    }

    /// Store a verified output in its corresponding batch.
    pub(crate) fn store_verified_output(
        &mut self,
        session_info: SessionInfo,
        output: MPCPublicOutput,
    ) -> DwalletMPCResult<()> {
        match session_info.mpc_round {
            MPCProtocolInitData::Sign(SingleSignSessionData {
                batch_session_id,
                hashed_message: message,
                ..
            }) => {
                self.store_verified_sign_output(batch_session_id, message.clone(), output)?;
            }
            MPCProtocolInitData::PresignSecond(_, ref first_round_output, batch_session_id) => {
                let presign =
                    parse_presign_from_first_and_second_outputs(first_round_output, &output)?;
                self.store_verified_presign_output(
                    batch_session_id,
                    session_info.flow_session_id,
                    bcs::to_bytes(&presign)?,
                )?;
            }
            _ => {}
        }
        Ok(())
    }

    /// Checks if a batch is completed.
    /// Return Some with the batch output if it is completed, otherwise return None.
    pub(crate) fn is_batch_completed(
        &self,
        session_info: &SessionInfo,
    ) -> DwalletMPCResult<Option<Vec<u8>>> {
        match session_info.mpc_round {
            MPCProtocolInitData::Sign(SingleSignSessionData {
                batch_session_id, ..
            }) => self.is_sign_batch_completed(batch_session_id),
            MPCProtocolInitData::PresignSecond(_, _, batch_session_id) => {
                self.is_presign_batch_completed(batch_session_id)
            }
            _ => Ok(None),
        }
    }

    fn store_verified_sign_output(
        &mut self,
        batch_session_id: ObjectID,
        hashed_message: Vec<u8>,
        signed_message: Vec<u8>,
    ) -> DwalletMPCResult<()> {
        let batched_sign_session = self
            .batched_sign_sessions
            .get_mut(&batch_session_id)
            .ok_or(DwalletMPCError::MPCSessionNotFound {
                session_id: batch_session_id,
            })?;
        batched_sign_session
            .hashed_msg_to_signature
            .insert(hashed_message, signed_message);
        Ok(())
    }

    fn store_verified_presign_output(
        &mut self,
        batch_session_id: ObjectID,
        first_round_session_id: ObjectID,
        output: Vec<u8>,
    ) -> DwalletMPCResult<()> {
        let batched_presign_session = self
            .batched_presign_sessions
            .get_mut(&batch_session_id)
            .ok_or(DwalletMPCError::MPCSessionNotFound {
                session_id: batch_session_id,
            })?;
        batched_presign_session
            .verified_presigns
            .push((first_round_session_id, output));
        Ok(())
    }

    /// Check if a batched sign session is completed.
    /// If it is, return the output of the entire batch.
    /// Otherwise, return None.
    fn is_sign_batch_completed(&self, session_id: ObjectID) -> DwalletMPCResult<Option<Vec<u8>>> {
        let batched_sign_session = self
            .batched_sign_sessions
            .get(&session_id)
            .ok_or(DwalletMPCError::MPCSessionNotFound { session_id })?;
        if batched_sign_session.hashed_msg_to_signature.values().len()
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
        }
    }

    fn is_presign_batch_completed(
        &self,
        session_id: ObjectID,
    ) -> DwalletMPCResult<Option<Vec<u8>>> {
        let batched_presign_session = self
            .batched_presign_sessions
            .get(&session_id)
            .ok_or(DwalletMPCError::MPCSessionNotFound { session_id })?;
        if batched_presign_session.verified_presigns.len() as u64
            == batched_presign_session.batch_size
        {
            Ok(Some(bcs::to_bytes(
                &batched_presign_session.verified_presigns,
            )?))
        } else {
            Ok(None)
        }
    }
}

fn parse_presign_from_first_and_second_outputs(
    first_output: &[u8],
    second_output: &[u8],
) -> DwalletMPCResult<<AsyncProtocol as twopc_mpc::presign::Protocol>::Presign> {
    let first_output: <AsyncProtocol as twopc_mpc::presign::Protocol>::EncryptionOfMaskAndMaskedNonceShare =
        bcs::from_bytes(&first_output)?;
    let second_output: (
        NoncePublicShareAndEncryptionOfMaskedNonceSharePart,
        NoncePublicShareAndEncryptionOfMaskedNonceSharePart,
    ) = bcs::from_bytes(&second_output)?;
    Ok((first_output, second_output).into())
}
