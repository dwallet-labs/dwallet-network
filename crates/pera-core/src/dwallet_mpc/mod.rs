use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use group::PartyID;
use pera_mpc_types::dwallet_mpc::MPCMessage;
use pera_types::base_types::AuthorityName;
use pera_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use std::sync::Arc;

mod dkg;
pub(crate) mod mpc_events;
mod mpc_instance;
pub mod mpc_manager;
pub(crate) mod mpc_party;
mod presign;
pub(crate) mod sign;

/// The message a Validator can send to the other parties while
/// running a dWallet MPC session.
#[derive(Clone)]
pub(crate) struct DWalletMPCMessage {
    /// The serialized message.
    pub(crate) message: MPCMessage,
    /// The authority (Validator) that sent the message.
    pub(crate) authority: AuthorityName,
}

/// Convert a given authority name (address) to it's corresponding [`PartyID`].
/// The [`PartyID`] is the index of the authority in the committee.
pub(crate) fn authority_name_to_party_id(
    authority_name: &AuthorityName,
    epoch_store: &Arc<AuthorityPerEpochStore>,
) -> DwalletMPCResult<PartyID> {
    epoch_store
        .committee()
        .authority_index(authority_name)
        // Need to add 1 because the authority index is 0-based,
        // and the twopc_mpc library uses 1-based party IDs.
        .map(|index| (index + 1) as PartyID)
        .ok_or_else(|| DwalletMPCError::AuthorityNameNotFound(*authority_name))
}
