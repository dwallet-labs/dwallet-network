use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::consensus_adapter::SubmitToConsensus;
use crate::dwallet_mpc::dkg::DKGFirstParty;
use crate::dwallet_mpc::mpc_instance::{
    authority_name_to_party_id, DWalletMPCInstance, DWalletMPCMessage, MPCSessionStatus,
};
use crate::dwallet_mpc::mpc_manager::twopc_error_to_pera_error;
use crate::dwallet_mpc::mpc_party::{advance, MPCParty};
use class_groups::dkg::proof_helpers::{
    generate_secret_share_sized_keypair_and_proof, KnowledgeOfDiscreteLogUCProof,
};
use class_groups::CompactIbqf;
use commitment::CommitmentSizedNumber;
use crypto_bigint::Uint;
use dwallet_mpc_types::ClassGroupsPublicKeyAndProof;
use group::PartyID;
use homomorphic_encryption::AdditivelyHomomorphicDecryptionKeyShare;
use jsonrpsee::core::Serialize;
use mpc::WeightedThresholdAccessStructure;
use pera_types::base_types::{AuthorityName, EpochId, ObjectID};
use pera_types::error::{PeraError, PeraResult};
use pera_types::messages_consensus::{ConsensusTransaction, ConsensusTransactionKind};
use pera_types::messages_dwallet_mpc::{MPCRound, SessionInfo};
use rand_core::SeedableRng;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

const NONE_OBJ_ID: ObjectID = ObjectID::from_single_byte(0);
const SECP256K1_DKG_SESSION_ID: ObjectID = ObjectID::from_single_byte(0);
const RISTRETTO_DKG_SESSION_ID: ObjectID = ObjectID::from_single_byte(1);
pub const FIRST_EPOCH_ID: EpochId = 0;

// pub type DecryptionKey = Uint<{ class_groups::SECRET_KEY_SHARE_DISCRIMINANT_LIMBS }>;
// pub type EncryptionKey = CompactIbqf<{ class_groups::SECRET_KEY_SHARE_DISCRIMINANT_LIMBS }>;
// pub type Proof = KnowledgeOfDiscreteLogUCProof;

fn new_dkg_secp256k1_instance(
    epoch_store: Arc<AuthorityPerEpochStore>,
) -> PeraResult<DWalletMPCInstance> {
    if epoch_store.epoch() != FIRST_EPOCH_ID {
        return Err(PeraError::InternalDWalletMPCError);
    }
    Ok(DWalletMPCInstance::new(
        Arc::downgrade(&epoch_store),
        FIRST_EPOCH_ID,
        MPCParty::NetworkDkg(KeyTypes::Secp256k1),
        MPCSessionStatus::FirstExecution,
        generate_secp256k1_dkg_party_public_input(
            epoch_store.committee_validators_class_groups_public_keys_and_proofs()?,
        ),
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

fn new_dkg_ristretto_instance(
    epoch_store: Arc<AuthorityPerEpochStore>,
) -> PeraResult<DWalletMPCInstance> {
    if epoch_store.epoch() != FIRST_EPOCH_ID {
        return Err(PeraError::InternalDWalletMPCError);
    }
    Ok(DWalletMPCInstance::new(
        Arc::downgrade(&epoch_store),
        FIRST_EPOCH_ID,
        MPCParty::NetworkDkg(KeyTypes::Ristretto),
        MPCSessionStatus::FirstExecution,
        generate_ristretto_dkg_party_public_input(
            epoch_store.committee_validators_class_groups_public_keys_and_proofs()?,
        ),
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
    secret_key_share_sized_encryption_keys_and_proofs: HashMap<
        PartyID,
        ClassGroupsPublicKeyAndProof,
    >,
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
    secret_key_share_sized_encryption_keys_and_proofs: HashMap<
        PartyID,
        ClassGroupsPublicKeyAndProof,
    >,
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

pub struct NetworkDkg;

impl NetworkDkg {
    pub fn init(
        epoch_store: Arc<AuthorityPerEpochStore>,
    ) -> PeraResult<HashMap<ObjectID, DWalletMPCInstance>> {
        if epoch_store.epoch() != FIRST_EPOCH_ID {
            return Err(PeraError::InternalDWalletMPCError);
        }
        let dkg_secp256k1_instance = new_dkg_secp256k1_instance(epoch_store.clone())?;
        let dkg_ristretto_instance = new_dkg_ristretto_instance(epoch_store.clone())?;

        Ok(HashMap::from([
            (
                dkg_secp256k1_instance.session_info.session_id.clone(),
                dkg_secp256k1_instance,
            ),
            (
                dkg_ristretto_instance.session_info.session_id.clone(),
                dkg_ristretto_instance,
            ),
        ]))
    }

    pub(crate) fn advance(
        weighted_threshold_access_structure: &WeightedThresholdAccessStructure,
        party_id: PartyID,
        public_input: &[u8],
        key_type: &KeyTypes,
        messages: Vec<HashMap<PartyID, Vec<u8>>>,
    ) -> PeraResult<mpc::AsynchronousRoundResult<Vec<u8>, Vec<u8>, Vec<u8>>> {
        Ok(match key_type {
            KeyTypes::Secp256k1 => advance::<DKGFirstParty>(
                CommitmentSizedNumber::from_le_slice(SECP256K1_DKG_SESSION_ID.to_vec().as_slice()),
                party_id,
                &weighted_threshold_access_structure,
                messages,
                bcs::from_bytes(public_input)?,
                (),
            ),
            KeyTypes::Ristretto => advance::<DKGFirstParty>(
                CommitmentSizedNumber::from_le_slice(RISTRETTO_DKG_SESSION_ID.to_vec().as_slice()),
                party_id,
                &weighted_threshold_access_structure,
                messages,
                bcs::from_bytes(public_input)?,
                (),
            ),
        }?)
    }
}
