use pera_types::base_types::{AuthorityName, EpochId, ObjectID};
use std::sync::Arc;
use std::collections::HashMap;
use std::marker::PhantomData;
use class_groups::dkg::{RistrettoParty, Secp256k1Party};
use group::PartyID;
use crypto_bigint::Uint;
use homomorphic_encryption::AdditivelyHomomorphicDecryptionKeyShare;
use jsonrpsee::core::Serialize;
use serde::Deserialize;
use twopc_mpc::secp256k1::class_groups::DecryptionKeyShare;
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


pub fn new_dkg_secp256k1_instance(
    epoch_id: EpochId,
    epoch_store: Arc<AuthorityPerEpochStore>,
    authority_private_key: Uint<{ class_groups::SECRET_KEY_SHARE_DISCRIMINANT_LIMBS }>,
) -> DWalletMPCInstance {
    DWalletMPCInstance::new(
        Arc::downgrade(&epoch_store),
        epoch_id,
        MPCParty::NetworkDkgSecp256k1Party(authority_private_key),
        MPCSessionStatus::FirstExecution,
        Vec::new(),
        SessionInfo {
            mpc_session_id: SECP256K1_DKG_SESSION_ID,
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
    authority_private_key: Uint<{ class_groups::SECRET_KEY_SHARE_DISCRIMINANT_LIMBS }>,
) -> DWalletMPCInstance {
    DWalletMPCInstance::new(
        Arc::downgrade(&epoch_store),
        epoch_id,
        MPCParty::NetworkDkgRistrettoParty(authority_private_key),
        MPCSessionStatus::FirstExecution,
        Vec::new(),
        SessionInfo {
            mpc_session_id: RISTRETTO_DKG_SESSION_ID,
            session_id: RISTRETTO_DKG_SESSION_ID,
            initiating_user_address: Default::default(),
            dwallet_cap_id: NONE_OBJ_ID,
            mpc_round: MPCRound::NetworkDkg,
        },
        None,
    )
}

fn generate_secp256k1_dkg_party_public_input(
    secret_key_share_sized_encryption_keys_and_proofs: HashMap<PartyID, ClassGroupsEncryptionKeyAndProof>,
) -> Vec<u8> {
    let public_input = Secp256k1Party::PublicInput::new(
        &(),
        (),
        (),
        Parameters {},
        secret_key_share_sized_encryption_keys_and_proofs,
    ).unwrap();
    bcs::to_bytes(&public_input).unwrap()
}

fn generate_ristretto_dkg_party_public_input(
    secret_key_share_sized_encryption_keys_and_proofs: HashMap<PartyID, ClassGroupsEncryptionKeyAndProof>,
) -> Vec<u8> {
    let public_input = RistrettoParty::PublicInput::new(
        &(),
        (),
        (),
        Parameters {},
        secret_key_share_sized_encryption_keys_and_proofs,
    ).unwrap();
    bcs::to_bytes(&public_input).unwrap()
}

use class_groups::dkg::proof_helpers::{generate_secret_share_sized_keypair_and_proof, KnowledgeOfDiscreteLogUCProof};
use class_groups::{CompactIbqf, EquivalenceClass};

type ClassGroupsEncryptionKeyAndProof = (CompactIbqf<{ class_groups::SECRET_KEY_SHARE_DISCRIMINANT_LIMBS }>, KnowledgeOfDiscreteLogUCProof);

// pub type ClassGroupsEncryptionKeyAndProof = (String, String);

fn mock_keypair_generation() -> (ClassGroupsEncryptionKeyAndProof, Uint<{ class_groups::SECRET_KEY_SHARE_DISCRIMINANT_LIMBS }>) {
    ((String::from("yael"), String::from("abergel")), Uint::from_u8(0))
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum KeyTypes {
    Secp256k1,
    Ristretto,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NetworkDkgMessage {
    EncryptionKeyAndProof(ClassGroupsEncryptionKeyAndProof),
    Message(KeyTypes, Vec<u8>),
    Output(Vec<u8>),
}

pub struct NetworkDkg {
    status: MPCSessionStatus,
    epoch_id: EpochId,
    epoch_store: Arc<AuthorityPerEpochStore>,
    authority_private_key: [u8; 32],
    consensus_adapter: Arc<dyn SubmitToConsensus>,
    secret_key_share_sized_encryption_keys_and_proofs: HashMap<PartyID, ClassGroupsEncryptionKeyAndProof>,
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
            status: MPCSessionStatus::FirstExecution,
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

    pub async fn start(&mut self) -> PeraResult<ClassGroupsEncryptionKeyAndProof> {
        let mut rng = rand_chacha::ChaCha20Rng::from_seed(self.authority_private_key);
        // let ((proof, encryption_key), decryption_key) = generate_secret_share_sized_keypair_and_proof(&mut rng)
        //     .map_err(|err| twopc_error_to_pera_error(err.into()))?;

        let ((proof, encryption_key), decryption_key) = mock_keypair_generation();
        let message = NetworkDkgMessage::EncryptionKeyAndProof((proof.clone(), encryption_key.clone()));
        self.decryption_key = decryption_key;
        let transaction = ConsensusTransaction::new_pera_network_dkg_message(
            self.epoch_store.name,
            bcs::to_bytes(&message)?,
        );
        self.consensus_adapter.submit_to_consensus(&vec![transaction], &self.epoch_store).await?;

        let dkg_secp256k1_instance = new_dkg_secp256k1_instance(
            self.epoch_id,
            self.epoch_store.clone(),
            self.decryption_key.clone(),
        )?;
        let dkg_ristretto_instance = new_dkg_ristretto_instance(
            self.epoch_id,
            self.epoch_store.clone(),
            self.decryption_key.clone(),
        )?;

        self.mpc_instances = HashMap::from(
            [
                (dkg_secp256k1_instance.session_info.session_id.clone(), dkg_secp256k1_instance),
                (dkg_ristretto_instance.session_info.session_id.clone(), dkg_ristretto_instance),
            ]
        );

        Ok((proof, encryption_key))
    }

    async fn handle_encryption_key_and_proof(
        &mut self,
        authority_name: AuthorityName,
        encryption_key_and_proof: ClassGroupsEncryptionKeyAndProof,
    ) -> PeraResult {
        let (encryption_key, proof) = encryption_key_and_proof;
        let authority_id = authority_name_to_party_id(&authority_name, &self.epoch_store)?;
        if self.secret_key_share_sized_encryption_keys_and_proofs.contains_key(&authority_id) {
            return Err(PeraError::InternalDWalletMPCError);
        }
        self.secret_key_share_sized_encryption_keys_and_proofs.insert(authority_id, (encryption_key, proof));
        if self.secret_key_share_sized_encryption_keys_and_proofs.len() == self.epoch_store.committee().voting_rights.len() {
            let secp256k1_instance = self.mpc_instances.get_mut(&SECP256K1_DKG_SESSION_ID).unwrap();
            secp256k1_instance.public_input = generate_secp256k1_dkg_party_public_input(self.secret_key_share_sized_encryption_keys_and_proofs.clone());
            self.advance(secp256k1_instance, authority_name, KeyTypes::Secp256k1).await?;

            let ristretto_instance = self.mpc_instances.get_mut(&RISTRETTO_DKG_SESSION_ID).unwrap();
            ristretto_instance.public_input = generate_ristretto_dkg_party_public_input(self.secret_key_share_sized_encryption_keys_and_proofs.clone());
            self.advance(ristretto_instance, authority_name, KeyTypes::Ristretto).await?;
        }
        Ok(())
    }

    pub async fn handle_message(&mut self, message: &[u8], authority_name: AuthorityName) -> PeraResult {
        if !matches!(self.status, MPCSessionStatus::FirstExecution |  MPCSessionStatus::Active(_) | MPCSessionStatus::Finalizing(_)) {
            return Err(PeraError::InternalDWalletMPCError); // todo (yael): return error
        }

        let message: NetworkDkgMessage = bcs::from_bytes(message)?;
        match message {
            NetworkDkgMessage::EncryptionKeyAndProof(message) => {
                self.handle_encryption_key_and_proof(authority_name, message).await?;
            }
            NetworkDkgMessage::Message(key_type, message) => {
                let message = DWalletMPCMessage { authority: authority_name, message };
                let instance = match key_type {
                    KeyTypes::Secp256k1 => self.mpc_instances.get_mut(&SECP256K1_DKG_SESSION_ID).unwrap(),
                    KeyTypes::Ristretto => self.mpc_instances.get_mut(&RISTRETTO_DKG_SESSION_ID).unwrap(),
                };
                instance.handle_message(message)?;
                let round = if let MPCSessionStatus::Active(round) = instance.status {
                    round
                } else {
                    return Err(PeraError::InternalDWalletMPCError);
                };
                if instance.pending_messages[round].len() == self.epoch_store.committee().voting_rights.len() {
                    self.advance(instance, authority_name, key_type).await?;
                }
            }
            NetworkDkgMessage::Output(output) => {
                // finalize the instance
                //return status completed
            }
        }
        Ok(())
    }

    async fn advance(&mut self, instance: &mut DWalletMPCInstance, authority_name: AuthorityName, key_type: KeyTypes) -> PeraResult {
        let (transaction, malicious_parties) = instance.advance(&(), self.party_id)?;
        // todo (yael): handle malicious parties
        // convert transaction
        let transaction = match transaction.kind {
            ConsensusTransactionKind::DWalletMPCMessage(_, message, _) => {
                let message = NetworkDkgMessage::Message(key_type, message);
                ConsensusTransaction::new_pera_network_dkg_message(
                    self.epoch_store.name,
                    bcs::to_bytes(&message)?,
                )
            }
            ConsensusTransactionKind::DWalletMPCOutput(_, message) => {
                let message = NetworkDkgMessage::Output(message);
                ConsensusTransaction::new_pera_network_dkg_message(
                    self.epoch_store.name,
                    bcs::to_bytes(&message)?,
                )
            }
            _ => {
                return Err(PeraError::InternalDWalletMPCError);
            }
        };
        self.consensus_adapter.submit_to_consensus(&vec![transaction], &self.epoch_store).await?;

        Ok(())
    }

    pub fn status(&self) -> &MPCSessionStatus {
        &self.status
    }
}