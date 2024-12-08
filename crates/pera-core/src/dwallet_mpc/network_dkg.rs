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
use crate::dwallet_mpc::mpc_party::{advance, MPCParty};
use class_groups::dkg::proof_helpers::{generate_secret_share_sized_keypair_and_proof, KnowledgeOfDiscreteLogUCProof};
use class_groups::{CompactIbqf};
use commitment::CommitmentSizedNumber;
use mpc::WeightedThresholdAccessStructure;
use rand_core::SeedableRng;
use dwallet_mpc_types::ClassGroupsPublicKeyAndProof;
use crate::dwallet_mpc::dkg::DKGFirstParty;
use crate::dwallet_mpc::mpc_manager::twopc_error_to_pera_error;

const NONE_OBJ_ID: ObjectID = ObjectID::from_single_byte(0);
const SECP256K1_DKG_SESSION_ID: ObjectID = ObjectID::from_single_byte(0);
const RISTRETTO_DKG_SESSION_ID: ObjectID = ObjectID::from_single_byte(1);
const FIRST_EPOCH_ID: EpochId = 0;

// pub type DecryptionKey = Uint<{ class_groups::SECRET_KEY_SHARE_DISCRIMINANT_LIMBS }>;
// pub type EncryptionKey = CompactIbqf<{ class_groups::SECRET_KEY_SHARE_DISCRIMINANT_LIMBS }>;
// pub type Proof = KnowledgeOfDiscreteLogUCProof;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DkgState {
    Init,
    Advance,
    Finalize(Vec<u8>, HashSet<PartyID>, HashSet<PartyID>),
    Completed(Vec<u8>, HashSet<PartyID>, HashSet<PartyID>),
}

pub fn new_dkg_secp256k1_instance(
    epoch_store: Arc<AuthorityPerEpochStore>,
) -> PeraResult<DWalletMPCInstance> {
    Ok(DWalletMPCInstance::new(
        Arc::downgrade(&epoch_store),
        FIRST_EPOCH_ID,
        MPCParty::NetworkDkgSecp256k1Party,
        MPCSessionStatus::FirstExecution,
        generate_secp256k1_dkg_party_public_input(epoch_store.committee_validators_class_groups_public_keys_and_proofs()?),
        SessionInfo {
            flow_session_id: SECP256K1_DKG_SESSION_ID,
            session_id: SECP256K1_DKG_SESSION_ID,
            initiating_user_address: Default::default(),
            dwallet_cap_id: NONE_OBJ_ID,
            mpc_round: MPCRound::NetworkDkg,
        },
        None,
    ))
}

pub fn new_dkg_ristretto_instance(
    epoch_store: Arc<AuthorityPerEpochStore>,
) -> PeraResult<DWalletMPCInstance> {
    Ok(DWalletMPCInstance::new(
        Arc::downgrade(&epoch_store),
        FIRST_EPOCH_ID,
        MPCParty::NetworkDkgRistrettoParty,
        MPCSessionStatus::FirstExecution,
        generate_ristretto_dkg_party_public_input(epoch_store.committee_validators_class_groups_public_keys_and_proofs()?),
        SessionInfo {
            flow_session_id: RISTRETTO_DKG_SESSION_ID,
            session_id: RISTRETTO_DKG_SESSION_ID,
            initiating_user_address: Default::default(),
            dwallet_cap_id: NONE_OBJ_ID,
            mpc_round: MPCRound::NetworkDkg,
        },
        None,
    ))
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
            mpc_instances: HashMap::new(),
            party_id,
        }
    }

    pub async fn init(epoch_store: Arc<AuthorityPerEpochStore>) -> PeraResult<HashMap<ObjectID, DWalletMPCInstance>> {
        if epoch_store.epoch() != FIRST_EPOCH_ID {
            return Err(PeraError::InternalDWalletMPCError);
        }
        let dkg_secp256k1_instance = new_dkg_secp256k1_instance(
            epoch_store.clone(),
        )?;
        let dkg_ristretto_instance = new_dkg_ristretto_instance(
            epoch_store.clone(),
        )?;

        Ok(HashMap::from(
            [
                (dkg_secp256k1_instance.session_info.session_id.clone(), dkg_secp256k1_instance),
                (dkg_ristretto_instance.session_info.session_id.clone(), dkg_ristretto_instance),
            ]
        ))
    }

    async fn advance(weighted_threshold_access_structure: &WeightedThresholdAccessStructure, epoch_store: Arc<AuthorityPerEpochStore>, instance: &mut DWalletMPCInstance, authority_name: AuthorityName, key_type: KeyTypes, messages: Vec<HashMap<PartyID, Vec<u8>>>) -> PeraResult<mpc::AsynchronousRoundResult<Vec<u8>, Vec<u8>, Vec<u8>>>  {
        Ok(match key_type {
            KeyTypes::Secp256k1 => {
                advance::<DKGFirstParty>(
                    CommitmentSizedNumber::from_le_slice(SECP256K1_DKG_SESSION_ID.to_vec().as_slice()),
                    authority_name_to_party_id(&authority_name, &epoch_store)?,
                    &weighted_threshold_access_structure,
                    messages,
                    bcs::from_bytes(&instance.public_input)?,
                    (),
                )
            }
            KeyTypes::Ristretto => {
                advance::<DKGFirstParty>(
                    CommitmentSizedNumber::from_le_slice(RISTRETTO_DKG_SESSION_ID.to_vec().as_slice()),
                    authority_name_to_party_id(&authority_name, &epoch_store)?,
                    &weighted_threshold_access_structure,
                    messages,
                    bcs::from_bytes(&instance.public_input)?,
                    (),
                )
            }
        }?)
    }

    pub fn status(&self) -> &DkgState {
        &self.status
    }
}