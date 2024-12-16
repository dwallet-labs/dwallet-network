use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::dwallet_mpc::dkg::DKGFirstParty;
use crate::dwallet_mpc::mpc_instance::DWalletMPCInstance;
use crate::dwallet_mpc::mpc_party::MPCParty;
use crate::dwallet_mpc::{
    advance, FIRST_EPOCH_ID, RISTRETTO_DKG_SESSION_ID, SECP256K1_DKG_SESSION_ID,
};
use commitment::CommitmentSizedNumber;
use dwallet_mpc_types::{dwallet_mpc::MPCSessionStatus, ClassGroupsPublicKeyAndProof};
use group::PartyID;
use homomorphic_encryption::AdditivelyHomomorphicDecryptionKeyShare;
use mpc::WeightedThresholdAccessStructure;
use pera_types::base_types::ObjectID;
use pera_types::dwallet_mpc::DWalletMPCNetworkKey;
use pera_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use pera_types::messages_dwallet_mpc::{MPCRound, SessionInfo};
use std::collections::HashMap;
use std::sync::Arc;

const NONE_OBJ_ID: ObjectID = ObjectID::from_single_byte(0);

fn new_dkg_secp256k1_instance(
    epoch_store: Arc<AuthorityPerEpochStore>,
) -> DwalletMPCResult<DWalletMPCInstance> {
    if epoch_store.epoch() != FIRST_EPOCH_ID {
        return Err(DwalletMPCError::DKGNotOnFirstEpoch);
    }
    Ok(DWalletMPCInstance::new(
        Arc::downgrade(&epoch_store),
        FIRST_EPOCH_ID,
        MPCParty::NetworkDkg(DWalletMPCNetworkKey::Secp256k1),
        MPCSessionStatus::FirstExecution,
        generate_secp256k1_dkg_party_public_input(
            epoch_store.committee_validators_class_groups_public_keys_and_proofs()?,
        )?,
        SessionInfo {
            flow_session_id: SECP256K1_DKG_SESSION_ID,
            session_id: SECP256K1_DKG_SESSION_ID,
            initiating_user_address: Default::default(),
            mpc_round: MPCRound::NetworkDkg(DWalletMPCNetworkKey::Secp256k1),
        },
        None,
    ))
}

fn new_dkg_ristretto_instance(
    epoch_store: Arc<AuthorityPerEpochStore>,
) -> DwalletMPCResult<DWalletMPCInstance> {
    if epoch_store.epoch() != FIRST_EPOCH_ID {
        return Err(DwalletMPCError::DKGNotOnFirstEpoch);
    }
    Ok(DWalletMPCInstance::new(
        Arc::downgrade(&epoch_store),
        FIRST_EPOCH_ID,
        MPCParty::NetworkDkg(DWalletMPCNetworkKey::Ristretto),
        MPCSessionStatus::FirstExecution,
        generate_ristretto_dkg_party_public_input(
            epoch_store.committee_validators_class_groups_public_keys_and_proofs()?,
        )?,
        SessionInfo {
            flow_session_id: RISTRETTO_DKG_SESSION_ID,
            session_id: RISTRETTO_DKG_SESSION_ID,
            initiating_user_address: Default::default(),
            mpc_round: MPCRound::NetworkDkg(DWalletMPCNetworkKey::Ristretto),
        },
        None,
    ))
}

// Todo (#382): Replace with the actual implementation once the DKG protocol is ready.
fn generate_secp256k1_dkg_party_public_input(
    secret_key_share_sized_encryption_keys_and_proofs: HashMap<
        PartyID,
        ClassGroupsPublicKeyAndProof,
    >,
) -> DwalletMPCResult<Vec<u8>> {
    <DKGFirstParty as crate::dwallet_mpc::dkg::DKGFirstPartyPublicInputGenerator>::generate_public_input()
}

// Todo (#382): Replace with the actual implementation once the DKG protocol is ready.
fn generate_ristretto_dkg_party_public_input(
    secret_key_share_sized_encryption_keys_and_proofs: HashMap<
        PartyID,
        ClassGroupsPublicKeyAndProof,
    >,
) -> DwalletMPCResult<Vec<u8>> {
    <DKGFirstParty as crate::dwallet_mpc::dkg::DKGFirstPartyPublicInputGenerator>::generate_public_input()
}
/// This struct is responsible for the network DKG protocol.
/// It manages the initialization and advancement of the network DKG supported key types.
pub struct NetworkDkg;

impl NetworkDkg {
    /// Initializes the network DKG protocol for the supported key types.
    pub fn init(
        epoch_store: Arc<AuthorityPerEpochStore>,
    ) -> DwalletMPCResult<HashMap<ObjectID, DWalletMPCInstance>> {
        if epoch_store.epoch() != FIRST_EPOCH_ID {
            return Err(DwalletMPCError::DKGNotOnFirstEpoch);
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

    /// Advances the network DKG protocol for the supported key types.
    pub(crate) fn advance(
        weighted_threshold_access_structure: &WeightedThresholdAccessStructure,
        party_id: PartyID,
        public_input: &[u8],
        key_type: &DWalletMPCNetworkKey,
        messages: Vec<HashMap<PartyID, Vec<u8>>>,
    ) -> DwalletMPCResult<mpc::AsynchronousRoundResult<Vec<u8>, Vec<u8>, Vec<u8>>> {
        Ok(match key_type {
            // Todo (#382): Replace with the actual implementation once the DKG protocol is ready.
            DWalletMPCNetworkKey::Secp256k1 => advance::<DKGFirstParty>(
                CommitmentSizedNumber::from_le_slice(SECP256K1_DKG_SESSION_ID.to_vec().as_slice()),
                party_id,
                &weighted_threshold_access_structure,
                messages,
                bcs::from_bytes(public_input)?,
                (),
            ),
            // Todo (#382): Replace with the actual implementation once the DKG protocol is ready.
            DWalletMPCNetworkKey::Ristretto => advance::<DKGFirstParty>(
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
