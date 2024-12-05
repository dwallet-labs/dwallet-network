use pera_types::base_types::{AuthorityName, EpochId, ObjectID};
use std::sync::Arc;
use std::collections::{HashMap, HashSet};
use group::PartyID;
use crypto_bigint::Uint;
use homomorphic_encryption::AdditivelyHomomorphicDecryptionKeyShare;
use jsonrpsee::core::Serialize;
use serde::Deserialize;
use pera_types::error::{PeraError, PeraResult};
use pera_types::messages_consensus::{ConsensusTransaction, ConsensusTransactionKind};
use pera_types::messages_dwallet_mpc::{MPCRound, SessionInfo};
use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::consensus_adapter::SubmitToConsensus;
use crate::dwallet_mpc::mpc_instance::{authority_name_to_party_id, DWalletMPCInstance, DWalletMPCMessage, MPCSessionStatus};
use crate::dwallet_mpc::mpc_party::MPCParty;

const NONE_OBJ_ID: ObjectID = ObjectID::from_single_byte(0);
const SECP256K1_DKG_SESSION_ID: ObjectID = ObjectID::from_single_byte(0);
const RISTRETTO_DKG_SESSION_ID: ObjectID = ObjectID::from_single_byte(1);


// pub type DecryptionKey = Uint<{ class_groups::SECRET_KEY_SHARE_DISCRIMINANT_LIMBS }>;
// pub type EncryptionKey = CompactIbqf<{ class_groups::SECRET_KEY_SHARE_DISCRIMINANT_LIMBS }>;
// pub type Proof = KnowledgeOfDiscreteLogUCProof;

// pub type DecryptionKey = Vec<u8>;
// pub type EncryptionKey = Vec<u8>;
// pub type Proof = Vec<u8>;


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DkgState {
    Init,
    Advance,
    Finalize(Vec<u8>, HashSet<PartyID>, HashSet<PartyID>),
    Completed(Vec<u8>, HashSet<PartyID>, HashSet<PartyID>),
}



pub fn new_dkg_secp256k1_instance(
    epoch_id: EpochId,
    epoch_store: Arc<AuthorityPerEpochStore>,
) -> DWalletMPCInstance {
    DWalletMPCInstance::new(
        Arc::downgrade(&epoch_store),
        epoch_id,
        MPCParty::NetworkDkgSecp256k1Party,
        MPCSessionStatus::FirstExecution,
        Vec::new(),
        SessionInfo {
            flow_session_id: SECP256K1_DKG_SESSION_ID,
            session_id: SECP256K1_DKG_SESSION_ID,
            initiating_user_address: Default::default(),
            dwallet_cap_id: NONE_OBJ_ID,
            mpc_round: MPCRound::NetworkDkg,
        },
        None,
    )
}

pub fn new_dkg_ristretto_instance(
    epoch_id: EpochId,
    epoch_store: Arc<AuthorityPerEpochStore>,
) -> DWalletMPCInstance {
    DWalletMPCInstance::new(
        Arc::downgrade(&epoch_store),
        epoch_id,
        MPCParty::NetworkDkgRistrettoParty,
        MPCSessionStatus::FirstExecution,
        Vec::new(),
        SessionInfo {
            flow_session_id: RISTRETTO_DKG_SESSION_ID,
            session_id: RISTRETTO_DKG_SESSION_ID,
            initiating_user_address: Default::default(),
            dwallet_cap_id: NONE_OBJ_ID,
            mpc_round: MPCRound::NetworkDkg,
        },
        None,
    )
}

fn generate_secp256k1_dkg_party_public_input(
    secret_key_share_sized_encryption_keys_and_proofs: HashMap<PartyID, ClassGroupsPublicKeyAndProof>,
) -> Vec<u8> {
    // let public_input = Secp256k1Party::PublicInput::new(
    //     &(),
    //     (),
    //     (),
    //     (),
    //     secret_key_share_sized_encryption_keys_and_proofs,
    // ).unwrap();
    // bcs::to_bytes(&public_input).unwrap()
    <DKGFirstParty as crate::dwallet_mpc::dkg::DKGFirstPartyPublicInputGenerator>::generate_public_input()
}

fn generate_ristretto_dkg_party_public_input(
    secret_key_share_sized_encryption_keys_and_proofs: HashMap<PartyID, ClassGroupsPublicKeyAndProof>,
) -> Vec<u8> {
    // let public_input = RistrettoParty::PublicInput::new(
    //     &(),
    //     (),
    //     (),
    //     Parameters {},
    //     secret_key_share_sized_encryption_keys_and_proofs,
    // ).unwrap();
    // bcs::to_bytes(&public_input).unwrap()
    <DKGFirstParty as crate::dwallet_mpc::dkg::DKGFirstPartyPublicInputGenerator>::generate_public_input()
}

use class_groups::dkg::proof_helpers::{generate_secret_share_sized_keypair_and_proof, KnowledgeOfDiscreteLogUCProof};
use class_groups::{CompactIbqf};
use mpc::WeightedThresholdAccessStructure;
use rand_core::SeedableRng;
use dwallet_mpc_types::ClassGroupsPublicKeyAndProof;
use crate::dwallet_mpc::dkg::DKGFirstParty;
use crate::dwallet_mpc::mpc_manager::twopc_error_to_pera_error;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum KeyTypes {
    Secp256k1,
    Ristretto,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NetworkDkgMessage {
    Message(KeyTypes, Vec<u8>),
    Output(Vec<u8>),
}

pub struct NetworkDkg {
    status: DkgState,
    epoch_id: EpochId,
    epoch_store: Arc<AuthorityPerEpochStore>,
    authority_private_key: [u8; 32],
    consensus_adapter: Arc<dyn SubmitToConsensus>,
    secret_key_share_sized_encryption_keys_and_proofs: HashMap<PartyID, ClassGroupsPublicKeyAndProof>,
    pub decryption_key: Uint<{ class_groups::SECRET_KEY_SHARE_DISCRIMINANT_LIMBS }>,
    mpc_instances: HashMap<ObjectID, DWalletMPCInstance>,
    party_id: PartyID,
}

impl NetworkDkg {
    pub(crate) fn new(
        epoch_id: EpochId,
        epoch_store: Arc<AuthorityPerEpochStore>,
        authority_private_key: [u8; 32],
        consensus_adapter: Arc<dyn SubmitToConsensus>,
        party_id: PartyID,
    ) -> Self {
        Self {
            status: DkgState::Init,
            epoch_id,
            epoch_store,
            authority_private_key,
            consensus_adapter,
            secret_key_share_sized_encryption_keys_and_proofs: HashMap::new(),
            decryption_key: Uint::from_u8(0),
            mpc_instances: HashMap::new(),
            party_id,
        }
    }

    pub async fn start(&mut self) -> PeraResult<> {
        let mut rng = rand_chacha::ChaCha20Rng::from_seed(self.authority_private_key);
        let (decryption_key, proof, encryption_key) = generate_secret_share_sized_keypair_and_proof(&mut rng)
            .map_err(|err| twopc_error_to_pera_error(err.into()))?;

        // let (decryption_key, proof, encryption_key) = (Uint::from_u8(0), Vec::new(), Vec::new());

        // let ((proof, encryption_key), decryption_key) = mock_keypair_generation();
        // self.decryption_key = decryption_key;

        let dkg_secp256k1_instance = new_dkg_secp256k1_instance(
            self.epoch_id,
            self.epoch_store.clone(),
        );
        let dkg_ristretto_instance = new_dkg_ristretto_instance(
            self.epoch_id,
            self.epoch_store.clone(),
        );

        self.mpc_instances = HashMap::from(
            [
                (dkg_secp256k1_instance.session_info.session_id.clone(), dkg_secp256k1_instance),
                (dkg_ristretto_instance.session_info.session_id.clone(), dkg_ristretto_instance),
            ]
        );

        self.first_round().await?;

        Ok(())
    }

    async fn first_round(
        &mut self,
    ) -> PeraResult {
        if !matches!(self.status, DkgState::Init) {
            return Err(PeraError::InternalDWalletMPCError); // todo (yael): return error
        }
        self.status = DkgState::Advance;
        let mut secp256k1_instance = self.mpc_instances.remove(&SECP256K1_DKG_SESSION_ID).unwrap();
        secp256k1_instance.public_input = generate_secp256k1_dkg_party_public_input(self.epoch_store.committee_validators_class_groups_public_keys_and_proofs()?);
        self.advance(&mut secp256k1_instance, self.epoch_store.name, KeyTypes::Secp256k1).await?;
        self.mpc_instances.insert(secp256k1_instance.session_info.session_id.clone(), secp256k1_instance);

        let mut ristretto_instance = self.mpc_instances.remove(&RISTRETTO_DKG_SESSION_ID).unwrap();
        ristretto_instance.public_input = generate_ristretto_dkg_party_public_input(self.epoch_store.committee_validators_class_groups_public_keys_and_proofs()?);
        self.advance(&mut ristretto_instance, self.epoch_store.name, KeyTypes::Ristretto).await?;
        self.mpc_instances.insert(ristretto_instance.session_info.session_id.clone(), ristretto_instance);

        Ok(())
    }

    pub async fn handle_message(&mut self, message: &[u8], authority_name: AuthorityName) -> PeraResult {
        if matches!(self.status, DkgState::Completed(_, _, _)) {
            return Err(PeraError::InternalDWalletMPCError); // todo (yael): return error
        }

        let message: NetworkDkgMessage = bcs::from_bytes(message)?;

        match message {
            NetworkDkgMessage::Message(key_type, message) => {
                let message = DWalletMPCMessage { authority: authority_name, message };
                let mut instance = match key_type {
                    KeyTypes::Secp256k1 => self.mpc_instances.remove(&SECP256K1_DKG_SESSION_ID).unwrap(),
                    KeyTypes::Ristretto => self.mpc_instances.remove(&RISTRETTO_DKG_SESSION_ID).unwrap(),
                };
                instance.handle_message(message)?;
                let round = if let MPCSessionStatus::Active(round) = instance.status {
                    round
                } else {
                    return Err(PeraError::InternalDWalletMPCError);
                };
                if instance.pending_messages[round].len() == self.epoch_store.committee().voting_rights.len() {
                    self.advance(&mut instance, authority_name, key_type).await?;
                }
                self.mpc_instances.insert(instance.session_info.session_id.clone(), instance);
            }
            NetworkDkgMessage::Output(output) => {
                let (self_output, valid_parties, malicious_parties) = match self.status.clone() {
                    DkgState::Finalize(self_output, valid_parties, malicious_parties) => (self_output, valid_parties, malicious_parties),
                    _ => return Err(PeraError::InternalDWalletMPCError),
                };

                let party_id = &authority_name_to_party_id(&authority_name, &self.epoch_store)?;
                if malicious_parties.contains(party_id) || valid_parties.contains(party_id) {
                    // ignore the message
                    return Ok(());
                }

                if *self_output == output.clone() {
                    let mut valid_parties = valid_parties.clone();
                    valid_parties.insert(*party_id);
                    self.status = DkgState::Finalize(output.clone(), valid_parties.clone(), malicious_parties.clone());
                    if valid_parties.len() == self.epoch_store.committee().voting_rights.len() { // fix this to threshold
                        self.status = DkgState::Completed(output.clone(), valid_parties.clone(), malicious_parties.clone());
                    }
                    // call system transaction
                } else {
                    let mut malicious_parties = malicious_parties.clone();
                    malicious_parties.insert(*party_id);
                    self.status = DkgState::Finalize(output.clone(), valid_parties.clone(), malicious_parties.clone());
                    if malicious_parties.len() == self.epoch_store.committee().voting_rights.len() { // fix this to 1/3 + 1
                        panic!("Failed to complete DKG");
                    }
                }
            }
        }
        Ok(())
    }

    async fn advance(&mut self, instance: &mut DWalletMPCInstance, authority_name: AuthorityName, key_type: KeyTypes) -> PeraResult {
        let weighted_parties: HashMap<PartyID, PartyID> = self.epoch_store
            .committee()
            .voting_rights
            .iter()
            .map(|(name, weight)| {
                Ok((
                    authority_name_to_party_id(&name, &self.epoch_store)?,
                    *weight as PartyID,
                ))
            })
            .collect::<PeraResult<HashMap<PartyID, PartyID>>>()?;
        let weighted_threshold_access_structure = WeightedThresholdAccessStructure::new(
            self.epoch_store.committee().voting_rights.len() as PartyID,
            weighted_parties.clone(),
        )
            .map_err(|_| PeraError::InternalDWalletMPCError)?;
        let (transaction, malicious_parties) = instance.advance(&weighted_threshold_access_structure, self.party_id)?;
        // todo (yael): handle malicious parties
        let transaction = match transaction.kind {
            ConsensusTransactionKind::DWalletMPCMessage(_, message, _) => {
                let message = NetworkDkgMessage::Message(key_type, message);
                ConsensusTransaction::new_pera_network_dkg_message(
                    self.epoch_store.name,
                    bcs::to_bytes(&message)?,
                )
            }
            ConsensusTransactionKind::DWalletMPCOutput(authority, _, message) => {
                self.status = DkgState::Finalize(message.clone(), HashSet::new(), HashSet::new()); // ::from(malicious_parties));
                let message = NetworkDkgMessage::Output(message);
                // todo (yael): save both authority name and party id to encrypted decryption share
                // every party will have both network party id from committee and authority name
                ConsensusTransaction::new_dwallet_mpc_output(
                    authority,
                    bcs::to_bytes(&message)?,
                    instance.session_info.clone(),
                )
            }
            _ => {
                return Err(PeraError::InternalDWalletMPCError);
            }
        };
        self.consensus_adapter.submit_to_consensus(&vec![transaction], &self.epoch_store).await?;

        Ok(())
    }

    pub fn status(&self) -> &DkgState {
        &self.status
    }
}