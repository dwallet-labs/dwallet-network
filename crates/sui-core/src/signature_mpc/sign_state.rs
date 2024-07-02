use crate::signature_mpc::sign_round::SignRound;
use signature_mpc::decrypt::PartialDecryptionProof;
use signature_mpc::twopc_mpc_protocols;
use signature_mpc::twopc_mpc_protocols::{
    DecentralizedPartyPresign, DecryptionPublicParameters, PaillierModulusSizedNumber, PartyID,
    ProtocolContext, PublicNonceEncryptedPartialSignatureAndProof, SecretKeyShareSizedNumber,
};
use std::collections::{HashMap, HashSet};
use sui_types::base_types::EpochId;
use sui_types::messages_signature_mpc::{SignMessage, SignatureMPCSessionID};
use crate::signature_mpc::identifiable_abort::spawn_proof_generation_and_conditional_malicious_identification;

#[derive(Clone)]
pub(crate) struct SignState {
    epoch: EpochId,
    pub party_id: PartyID,
    parties: HashSet<PartyID>,
    aggregator_party_id: PartyID,
    pub tiresias_public_parameters: DecryptionPublicParameters,
    pub tiresias_key_share_decryption_key_share: SecretKeyShareSizedNumber,
    pub messages: Option<Vec<Vec<u8>>>,
    pub public_nonce_encrypted_partial_signature_and_proofs:
        Option<Vec<PublicNonceEncryptedPartialSignatureAndProof<ProtocolContext>>>,
    pub presigns: Option<Vec<DecentralizedPartyPresign>>,
    pub decryption_shares:
        HashMap<PartyID, Vec<(PaillierModulusSizedNumber, PaillierModulusSizedNumber)>>,
    pub proofs: Option<HashMap<PartyID, Vec<(PartialDecryptionProof)>>>,
    pub failed_messages_indices: Option<Vec<usize>>,
    pub involved_parties: Vec<PartyID>,
}

impl SignState {
    pub(crate) fn new(
        tiresias_key_share_decryption_key_share: SecretKeyShareSizedNumber,
        tiresias_public_parameters: DecryptionPublicParameters,
        epoch: EpochId,
        party_id: PartyID,
        parties: HashSet<PartyID>,
        session_id: SignatureMPCSessionID,
    ) -> Self {
        let aggregator_party_id = ((u64::from_be_bytes((&session_id.0[0..8]).try_into().unwrap())
            % parties.len() as u64)
            + 1) as PartyID;

        Self {
            epoch,
            party_id,
            parties,
            aggregator_party_id,
            tiresias_public_parameters,
            messages: None,
            public_nonce_encrypted_partial_signature_and_proofs: None,
            decryption_shares: HashMap::new(),
            tiresias_key_share_decryption_key_share,
            presigns: None,
            proofs: None,
            failed_messages_indices: None,
            involved_parties: Vec::new(),
        }
    }

    pub(crate) fn set(
        &mut self,
        messages: Vec<Vec<u8>>,
        public_nonce_encrypted_partial_signature_and_proofs: Vec<
            PublicNonceEncryptedPartialSignatureAndProof<ProtocolContext>,
        >,
        presigns: Vec<DecentralizedPartyPresign>,
    ) {
        self.messages = Some(messages);
        self.public_nonce_encrypted_partial_signature_and_proofs =
            Some(public_nonce_encrypted_partial_signature_and_proofs);
        self.presigns = Some(presigns);
    }

    pub(crate) fn insert_first_round(
        &mut self,
        party_id: PartyID,
        message: SignMessage,
    ) -> twopc_mpc_protocols::Result<()> {
        match message {
            SignMessage::DecryptionShares(shares) => {
                let _ = self.decryption_shares.insert(party_id, shares);
            }
            SignMessage::Proofs { proofs, failed_messages_indices, involved_parties } => {
                self.failed_messages_indices = Some(failed_messages_indices.clone());
                self.involved_parties = involved_parties.clone();
                self.insert_proofs(party_id, proofs.clone());
            }
        }
        Ok(())
    }

    pub(crate) fn insert_proofs(&mut self, party_id: PartyID, new_proofs: Vec<PartialDecryptionProof>) {
        if let Some(proofs_map) = &mut self.proofs {
            proofs_map.insert(party_id, new_proofs);
        } else {
            let mut proofs_map = HashMap::from([(party_id, new_proofs)]);
            self.proofs = Some(proofs_map);
        }
    }

    pub(crate) fn ready_for_complete_first_round(&self, round: &SignRound) -> bool {
        match round {
            SignRound::FirstRound { .. } => {
                self.received_all_decryption_shares() && self.party_id == self.aggregator_party_id
            }
            _ => false,
        }
    }

    pub(crate) fn received_all_decryption_shares(&self) -> bool {
        return self.decryption_shares.len() == self.parties.len();
    }

    pub(crate) fn should_identify_malicious_actors(&self) -> bool {
        if let Some(proofs) = self.clone().proofs {
            return proofs.len() == self.parties.clone().len() && self.received_all_decryption_shares();
        }
        return false;
    }
}
