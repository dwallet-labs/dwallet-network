use std::collections::{HashMap, HashSet};
use group::PartyID;
use mpc::Advance;
use serde::{Serialize, Deserialize};
use twopc_mpc::dkg::decentralized_party::encryption_of_secret_key_share_round::AuxiliaryInput;

pub trait BytesParty : Sized + Sync + Send{
    fn advance(self, messages: HashMap<PartyID, Vec<u8>>, auxiliary_input: Vec<u8>) -> twopc_mpc::Result<AdvanceResult<Self>>;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum AdvanceResult<P: BytesParty> {
    Advance((Vec<u8>, P)),
    Finalize(Vec<u8>),
}

type AsyncProtocol = twopc_mpc::secp256k1::class_groups::AsyncProtocol;

struct FirstDKGBytesParty {
    party: <AsyncProtocol as twopc_mpc::dkg::Protocol>::EncryptionOfSecretKeyShareRoundParty,
}

impl BytesParty for FirstDKGBytesParty {
    fn advance(self, messages: HashMap<PartyID, Vec<u8>>, auxiliary_input: Vec<u8>) -> twopc_mpc::Result<AdvanceResult<Self>> {
        let secp256k1_group_public_parameters =
            class_groups_constants::protocol_public_parameters().map_err(|_| twopc_mpc::Error::InvalidPublicParameters)?;

        let parties = (0..3).collect::<HashSet<PartyID>>();
        // let session_id = commitment::CommitmentSizedNumber::from_be_slice(&session_id);
        let session_id = commitment::CommitmentSizedNumber::from_u8(8);
        let a = AuxiliaryInput {
            protocol_public_parameters: secp256k1_group_public_parameters,
            party_id: 1,
            threshold: 3,
            number_of_parties: 4,
            parties: parties.clone(),
            session_id,
        };

        // Todo: Remove this unwrap
        let messages = messages.into_iter().map(|(k, v)| (k, bcs::from_bytes(&v).unwrap())).collect();

        let result = self.party.advance(messages, &a, &mut rand_core::OsRng)?;
        match result {
            mpc::AdvanceResult::Advance((message, party)) => Ok(AdvanceResult::Advance((bcs::to_bytes(&message).unwrap(), FirstDKGBytesParty { party }))),
            mpc::AdvanceResult::Finalize(message) => Ok(AdvanceResult::Finalize(bcs::to_bytes(&message).unwrap())),
        }
    }

}

