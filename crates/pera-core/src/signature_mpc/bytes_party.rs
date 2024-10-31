use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::mem;
use std::sync::{Arc, Mutex};
use group::PartyID;
use mpc::Advance;
use once_cell::sync::OnceCell;
use serde::{Serialize, Deserialize};
use twopc_mpc::dkg::decentralized_party::encryption_of_secret_key_share_round::AuxiliaryInput;

pub enum MPCParty {
    DefaultParty,
    FirstDKGBytesParty(FirstDKGBytesParty),
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
            MPCParty::DefaultParty => Err(twopc_mpc::Error::InvalidParameters),
        }
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
        let secp256k1_group_public_parameters =
            class_groups_constants::protocol_public_parameters().map_err(|_| twopc_mpc::Error::InvalidPublicParameters)?;

        let parties = (0..3).collect::<HashSet<PartyID>>();
        let session_id = commitment::CommitmentSizedNumber::from_u8(8);
        let a = AuxiliaryInput {
            protocol_public_parameters: secp256k1_group_public_parameters,
            party_id: 1,
            threshold: 3,
            number_of_parties: 4,
            parties: parties.clone(),
            session_id,
        };

        let messages = messages.into_iter().map(|(k, v)| (k, bcs::from_bytes(&v).unwrap())).collect();
        let result = self.party.advance(messages, &a, &mut rand_core::OsRng)?; // todo: remove unwrap

        match result {
            mpc::AdvanceResult::Advance((message, new_party)) =>
                Ok(AdvanceResult::Advance((bcs::to_bytes(&message).unwrap(), MPCParty::FirstDKGBytesParty(Self { party: new_party })))),
            mpc::AdvanceResult::Finalize(output) =>
                Ok(AdvanceResult::Finalize(bcs::to_bytes(&output).unwrap())),
        }
    }
}

