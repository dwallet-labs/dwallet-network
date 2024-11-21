//! This module provides a wrapper around the DKG protocol from the 2PC-MPC library.
//!
//! It integrates both DKG parties (each representing a round in the DKG protocol) and
//! implements the [`BytesParty`] trait for seamless interaction with other MPC components.
use group::PartyID;
use mpc::{Advance, Party};
use std::collections::{HashMap, HashSet};
use twopc_mpc::dkg::Protocol;

pub type AsyncProtocol = twopc_mpc::secp256k1::class_groups::AsyncProtocol;
/// This struct represents the initial round of the DKG protocol.
pub type DKGFirstParty = <AsyncProtocol as Protocol>::EncryptionOfSecretKeyShareRoundParty;
/// This struct represents the final round of the DKG protocol.
pub type DKGSecondParty = <AsyncProtocol as Protocol>::ProofVerificationRoundParty;

/// A trait for generating auxiliary input for the initial round of the DKG protocol.
///
/// This trait is implemented to resolve compiler type ambiguities that arise in
/// the 2PC-MPC library when accessing `mpc::Party::AuxiliaryInput`.
/// It defines the parameters and logic
/// necessary to initiate the first round of the DKG protocol,
/// preparing the party with the essential session information and other contextual data.
pub(super) trait DKGFirstPartyAuxiliaryInputGenerator: Party {
    /// Generates the auxiliary input required for the first round of the DKG protocol.
    fn generate_auxiliary_input(
        session_id: Vec<u8>,
        number_of_parties: u16,
        party_id: PartyID,
    ) -> Vec<u8>;
}

/// A trait for generating auxiliary input for the last round of the DKG protocol.
///
/// This trait is implemented to resolve compiler type ambiguities that arise in the 2PC-MPC library
/// when accessing `mpc::Party::AuxiliaryInput`.
/// It defines the parameters and logic
/// necessary to initiate the second round of the DKG protocol,
/// preparing the party with the essential session information and other contextual data.
pub(super) trait DKGSecondPartyAuxiliaryInputGenerator: Party {
    /// Generates the auxiliary input required for the second round of the DKG protocol.
    /// The `session_id` is the unique identifier for the MPC session from the first round.
    fn generate_auxiliary_input(
        number_of_parties: u16,
        party_id: PartyID,
        first_round_output: Vec<u8>,
        centralized_party_public_key_share: Vec<u8>,
        session_id: Vec<u8>,
    ) -> Vec<u8>;
}

//noinspection RsSuperTraitIsNotImplemented
impl DKGFirstPartyAuxiliaryInputGenerator for DKGFirstParty {
    fn generate_auxiliary_input(
        session_id: Vec<u8>,
        number_of_parties: u16,
        party_id: PartyID,
    ) -> Vec<u8> {
        let secp256k1_group_public_parameters =
            class_groups_constants::protocol_public_parameters();
        let parties = (0..number_of_parties).collect::<HashSet<PartyID>>();
        let session_id = commitment::CommitmentSizedNumber::from_le_slice(&session_id);
        let aux = Self::AuxiliaryInput {
            protocol_public_parameters: secp256k1_group_public_parameters,
            party_id,
            // TODO (#268): Take the voting power into account when dealing with the threshold
            threshold: ((number_of_parties * 2) + 2) / 3,
            number_of_parties,
            parties: parties.clone(),
            session_id,
        };
        bcs::to_bytes(&aux).unwrap()
    }
}

impl DKGSecondPartyAuxiliaryInputGenerator for DKGSecondParty {
    fn generate_auxiliary_input(
        number_of_parties: u16,
        party_id: PartyID,
        first_round_output_buf: Vec<u8>,
        centralized_party_public_key_share_buf: Vec<u8>,
        session_id: Vec<u8>,
    ) -> Vec<u8> {
        let first_round_aux_buf = DKGFirstParty::generate_auxiliary_input(
            session_id.clone(),
            number_of_parties,
            party_id,
        );
        let first_aux = bcs::from_bytes(&first_round_aux_buf).unwrap();
        let first_round_output: <DKGFirstParty as Party>::Output =
            bcs::from_bytes(&first_round_output_buf).unwrap();
        let centralized_party_public_key_share: <AsyncProtocol as Protocol>::PublicKeyShareAndProof =
            bcs::from_bytes(&centralized_party_public_key_share_buf).unwrap();

        let aux: Self::AuxiliaryInput = (
            first_aux,
            first_round_output,
            centralized_party_public_key_share,
        )
            .into();
        bcs::to_bytes(&aux).unwrap()
    }
}

pub(super) fn advance<P: Advance>(
    party: P,
    messages: HashMap<PartyID, Vec<u8>>,
    auxiliary_input: P::AuxiliaryInput,
) -> Result<mpc::AdvanceResult<Vec<u8>, P, Vec<u8>>, twopc_mpc::Error> {
    let messages = messages
        .into_iter()
        .map(|(k, v)| {
            let message = bcs::from_bytes(&v).map_err(|_| twopc_mpc::Error::InvalidParameters);
            return match message {
                Ok(message) => Ok((k, message)),
                Err(err) => Err(err),
            };
        })
        .collect::<Result<HashMap<_, _>, _>>()?;

    let res = party
        .advance(messages, &auxiliary_input, &mut rand_core::OsRng)
        .unwrap();
    // .map_err(|error| {
    //     let Ok(error): Result<mpc::Error, _> = error.try_into() else {
    //         return PeraError::NonMPCEvent;
    //     };
    //     return PeraError::NonMPCEvent;
    // })?;

    Ok(match res {
        mpc::AdvanceResult::Advance((msg, party)) => {
            mpc::AdvanceResult::Advance((bcs::to_bytes(&msg).unwrap(), party))
        }
        mpc::AdvanceResult::Finalize(output) => {
            let output: P::OutputValue = output.into();
            mpc::AdvanceResult::Finalize(bcs::to_bytes(&output).unwrap())
        }
    })
}
