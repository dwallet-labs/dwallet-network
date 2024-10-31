use std::collections::{HashMap, HashSet};
use group::PartyID;
use mpc::Advance;
use twopc_mpc::dkg::decentralized_party::encryption_of_secret_key_share_round::AuxiliaryInput;
use pera_types::event::Event;
use crate::signature_mpc;
use crate::signature_mpc::auxiliary_inputs::{get_dkg_first_round_auxiliary_input, get_dkg_second_round_auxiliary_input};
use crate::signature_mpc::mpc_events::{CreatedDKGSessionEvent, MPCEvent, StartDKGSecondRoundEvent};

pub type PartyAuxiliaryInput = Vec<u8>;

pub enum MPCParty {
    DefaultParty,
    FirstDKGBytesParty(FirstDKGBytesParty),
    SecondDKGBytesParty(SecondDKGBytesParty),
    // ... there will be more parties here
}

/// The default party is the party that is used when the party is not specified.
/// We only implemented it to be able to use `mem::take` which requires that the type has `Default` implemented.
impl Default for MPCParty {
    fn default() -> Self {
        MPCParty::DefaultParty
    }
}

impl MPCParty {
    pub fn advance(self, messages: HashMap<PartyID, Vec<u8>>, auxiliary_input: Vec<u8>) -> twopc_mpc::Result<AdvanceResult> {
        match self {
            MPCParty::FirstDKGBytesParty(party) => party.advance(messages, auxiliary_input),
            MPCParty::SecondDKGBytesParty(party) => party.advance(messages, auxiliary_input),
            MPCParty::DefaultParty => Err(twopc_mpc::Error::InvalidParameters),
        }
    }

    // todo: implement from event to party
    pub fn from_event(event: Event) -> anyhow::Result<(Self, PartyAuxiliaryInput)> {
        if event.type_ == CreatedDKGSessionEvent::type_() {
            let deserialized_event: CreatedDKGSessionEvent =
                bcs::from_bytes(&event.contents)?;
            return Ok((
                MPCParty::FirstDKGBytesParty(FirstDKGBytesParty { party: <AsyncProtocol as twopc_mpc::dkg::Protocol>::EncryptionOfSecretKeyShareRoundParty::default() }),
                get_dkg_first_round_auxiliary_input(deserialized_event.session_id.bytes.to_vec())
            ));
        } else if event.type_ == StartDKGSecondRoundEvent::type_() {
            let deserialized_event: StartDKGSecondRoundEvent =
                bcs::from_bytes(&event.contents)?;
            let public_key_share_and_proof =
                bcs::from_bytes(&deserialized_event.public_key_share_and_proof)?;
            let first_round_output =
                bcs::from_bytes(&deserialized_event.first_round_output)?;
            return Ok((
                MPCParty::SecondDKGBytesParty(SecondDKGBytesParty { party: <AsyncProtocol as twopc_mpc::dkg::Protocol>::ProofVerificationRoundParty::default() }),
                get_dkg_second_round_auxiliary_input(
                    first_round_output,
                    public_key_share_and_proof,
                    deserialized_event.session_id.bytes.to_vec(),
                ),
            ));
        }
        Ok((MPCParty::DefaultParty, Vec::new()))
    }
}

pub trait BytesParty : Sync + Send {
    fn advance(self, messages: HashMap<PartyID, Vec<u8>>, auxiliary_input: Vec<u8>) -> Result<AdvanceResult, twopc_mpc::Error>;
}

pub enum AdvanceResult {
    Advance((Vec<u8>, MPCParty)),
    Finalize(Vec<u8>),
}

type AsyncProtocol = twopc_mpc::secp256k1::class_groups::AsyncProtocol;

#[derive(Default)]
pub struct FirstDKGBytesParty {
    pub(crate) party: <AsyncProtocol as twopc_mpc::dkg::Protocol>::EncryptionOfSecretKeyShareRoundParty,
}

impl BytesParty for FirstDKGBytesParty {
    fn advance(
        self,
        messages: HashMap<PartyID, Vec<u8>>,
        auxiliary_input: Vec<u8>,
    ) -> Result<AdvanceResult, twopc_mpc::Error> {
        let auxiliary_input = bcs::from_bytes(&auxiliary_input).map_err(|_| {twopc_mpc::Error::InvalidParameters})?;
        let messages = messages.into_iter().map(|(k, v)| (k, bcs::from_bytes(&v).unwrap())).collect();
        let result = self.party.advance(messages, &auxiliary_input, &mut rand_core::OsRng)?;

        match result {
            mpc::AdvanceResult::Advance((message, new_party)) =>
                Ok(AdvanceResult::Advance((bcs::to_bytes(&message).unwrap(), MPCParty::FirstDKGBytesParty(Self { party: new_party })))),
            mpc::AdvanceResult::Finalize(output) =>
                Ok(AdvanceResult::Finalize(bcs::to_bytes(&output).unwrap())),
        }
    }
}

pub struct SecondDKGBytesParty {
    pub(crate) party: <AsyncProtocol as twopc_mpc::dkg::Protocol>::ProofVerificationRoundParty,
}

impl BytesParty for SecondDKGBytesParty {
    fn advance(
        self,
        messages: HashMap<PartyID, Vec<u8>>,
        auxiliary_input: Vec<u8>,
    ) -> Result<AdvanceResult, twopc_mpc::Error> {
        let auxiliary_input = bcs::from_bytes(&auxiliary_input).unwrap();
        let messages = messages.into_iter().map(|(k, v)| (k, bcs::from_bytes(&v).unwrap())).collect();
        let result = self.party.advance(messages, &auxiliary_input, &mut rand_core::OsRng)?; // todo: remove unwrap

        match result {
            mpc::AdvanceResult::Advance((message, new_party)) =>
                Ok(AdvanceResult::Advance((bcs::to_bytes(&message).unwrap(), MPCParty::SecondDKGBytesParty(Self { party: new_party })))),
            mpc::AdvanceResult::Finalize(output) =>
                Ok(AdvanceResult::Finalize(bcs::to_bytes(&output).unwrap())),
        }
    }
}

