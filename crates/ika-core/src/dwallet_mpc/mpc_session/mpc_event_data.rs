use std::collections::HashMap;
use group::PartyID;
use mpc::WeightedThresholdAccessStructure;
use tracing::error;
use twopc_mpc::sign::Protocol;
use dwallet_mpc_types::dwallet_mpc::MPCPrivateInput;
use ika_types::committee::{ClassGroupsEncryptionKeyAndProof, Committee};
use ika_types::dwallet_mpc_error::DwalletMPCError;
use ika_types::messages_dwallet_mpc::{AsyncProtocol, DWalletMPCEvent, MPCRequestInput, SessionType};
use crate::dwallet_mpc::mpc_session::input::PublicInput;
use crate::dwallet_mpc::mpc_session::session_input_from_event;
use crate::dwallet_mpc::network_dkg::DwalletMPCNetworkKeys;

/// The DWallet MPC session data that is based on the event that initiated the session.
#[derive(Clone, PartialEq, Eq)]
pub struct MPCEventData {
    pub private_input: MPCPrivateInput,
    pub request_input: MPCRequestInput,
    pub(crate) decryption_key_shares:
        Option<HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>>,
    pub(crate) session_type: SessionType,
    pub(crate) session_sequence_number: u64,
    pub(crate) public_input: PublicInput,
    pub(crate) requires_network_key_data: bool,
    pub(crate) requires_next_active_committee: bool,
}

impl MPCEventData {
    pub(crate) fn try_new(
        event: DWalletMPCEvent,
        access_structure: &WeightedThresholdAccessStructure,
        committee: &Committee,
        network_keys: &DwalletMPCNetworkKeys,
        next_active_committee: Option<Committee>,
        validators_class_groups_public_keys_and_proofs: HashMap<
            PartyID,
            ClassGroupsEncryptionKeyAndProof,
        >,
    ) -> Result<Self, DwalletMPCError> {
        let (public_input, private_input) = session_input_from_event(
            event.clone(),
            access_structure,
            committee,
            network_keys,
            next_active_committee,
            validators_class_groups_public_keys_and_proofs,
        )?;

        let needs_decryption_key_shares = matches!(
            event.session_request.request_input.clone(),
            MPCRequestInput::Sign(_) | MPCRequestInput::NetworkEncryptionKeyReconfiguration(_)
        );

        let decryption_key_shares = if needs_decryption_key_shares {
            if let Some(network_encryption_key_id) = event
                .session_request
                .request_input
                .get_network_encryption_key_id()
            {
                Some(network_keys.get_decryption_key_shares(&network_encryption_key_id)?)
            } else {
                error!(
                    should_never_happen =? true,
                    session_id=?event.session_request.session_identifier,
                    "failed to get network encryption key ID for a session that requires decryption key shares",
                );

                None
            }
        } else {
            None
        };

        let mpc_event_data = Self {
            session_type: event.session_request.session_type,
            session_sequence_number: event.session_request.session_sequence_number,
            request_input: event.session_request.request_input,
            private_input,
            decryption_key_shares,
            public_input,
            requires_network_key_data: event.session_request.requires_network_key_data,
            requires_next_active_committee: event.session_request.requires_next_active_committee,
        };

        Ok(mpc_event_data)
    }
}

impl PartialOrd<Self> for MPCEventData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MPCEventData {
    fn cmp(&self, other: &Self) -> Ordering {
        // System sessions have a higher priority than user session.
        // Both system and user sessions are sorted by their sequence number between themselves.
        match (self.session_type, other.session_type) {
            (SessionType::User, SessionType::User) => self.session_sequence_number.cmp(&other.session_sequence_number),
            (SessionType::System, SessionType::User) => Ordering::Greater,
            (SessionType::System, SessionType::System) => self.session_sequence_number.cmp(&other.session_sequence_number),
            (SessionType::User, SessionType::System,) => Ordering::Less,
        }
    }
}
