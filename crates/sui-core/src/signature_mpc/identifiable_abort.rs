use std::collections::HashSet;
use std::mem;
use signature_mpc::twopc_mpc_protocols::{SignaturePartialDecryptionProofVerificationParty, PartyID, Result};
use sui_types::messages_signature_mpc::SignatureMPCBulletProofAggregatesMessage;


// Q: what is the output message of the identifiable abort protocol?
// Q: where and when the abort request is received among the other parties?


#[derive(Default)]
pub(crate) enum IdentifiableAbortRound {
    FirstRound {
        //place holder
        signature_partial_decryption_proof_verification_round_parties: Vec<None>
    },
    SecondRound,
    #[default]
    None,
}

impl IdentifiableAbortRound {
    pub(crate) fn new() {}

    pub(crate) fn complete_round(
        &mut self,
        state: IdentifiableAbortState,
    ) -> Result<IdentifiableAbortRoundCompletion> {
        let round = mem::take(self);
        match round {
            IdentifiableAbortRound::FirstRound {
                signature_partial_decryption_proof_verification_round_parties,
            } => {
                // call prove_correct_signature_partial_decryption
                Ok(IdentifiableAbortRoundCompletion::FirstRoundOutput())
            }
            IdentifiableAbortRound::SecondRound => {
                //call identify_malicious_decrypters
                Ok(IdentifiableAbortRoundCompletion::None)
            }
            IdentifiableAbortRound::None => {
                Ok(IdentifiableAbortRoundCompletion::None)
            }
        }

    }
}

pub(crate) enum IdentifiableAbortRoundCompletion {
    Message(),
    FirstRoundOutput(),
    SecondRoundOutput(),
    None,
}

#[derive(Clone)]
pub(crate) struct IdentifiableAbortState {
    party_id: PartyID,
    parties: HashSet<PartyID>,
}

impl IdentifiableAbortState {
    pub(crate) fn new(
        party_id: PartyID,
        parties: HashSet<PartyID>,
    ) -> Self {
        Self {
            party_id,
            parties: parties.clone(),
        }
    }
}
