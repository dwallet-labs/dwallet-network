use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::dwallet_mpc::advance;
use crate::dwallet_mpc::dkg::DKGFirstParty;
use crate::dwallet_mpc::mpc_events::StartNetworkDKGEvent;
use crate::dwallet_mpc::mpc_party::MPCParty;
use commitment::CommitmentSizedNumber;
use dwallet_mpc_types::class_groups_key::{read_class_groups_from_file, read_class_groups_from_file_real, ClassGroupsEncryptionKeyAndProof};
use group::{secp256k1, PartyID};
use homomorphic_encryption::AdditivelyHomomorphicDecryptionKeyShare;
use mpc::WeightedThresholdAccessStructure;
use pera_types::dwallet_mpc::{DWalletMPCNetworkKey, EncryptionOfNetworkDecryptionKeyShares};
use pera_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use pera_types::messages_dwallet_mpc::{MPCRound, SessionInfo};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use class_groups::{CompactIbqf, KnowledgeOfDiscreteLogUCProof, CRT_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS, DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER, MAX_PRIMES};
use class_groups::dkg::{Secp256k1Party, Secp256k1PublicInput};

/// The status of the network supported key types for the dWallet MPC sessions.
#[derive(Clone, Debug, PartialEq)]
pub enum DwalletMPCNetworkKeysStatus {
    /// The network supported key types have been updated or initialized.
    Ready(HashSet<DWalletMPCNetworkKey>),
    /// None of the network supported key types have not been initialized.
    NotInitialized,
}

/// Hold the network keys of the dwallet mpc protocols.
pub struct DwalletMPCNetworkKeyVersions {
    /// The validators' decryption key shares.
    pub validator_decryption_key_share: Arc<RwLock<HashMap<DWalletMPCNetworkKey, Vec<Vec<u8>>>>>,
    /// The dWallet MPC network decryption key shares (encrypted).
    /// Map from key type to the encryption of the key version.
    pub key_shares_versions:
        Arc<RwLock<HashMap<DWalletMPCNetworkKey, Vec<EncryptionOfNetworkDecryptionKeyShares>>>>,
    /// The status of the network supported key types for the dWallet MPC sessions.
    pub status: Arc<RwLock<DwalletMPCNetworkKeysStatus>>,
}

impl DwalletMPCNetworkKeyVersions {
    /// Creates a new instance of the network encryption of decryption key shares.
    pub fn new(epoch_store: &AuthorityPerEpochStore) -> Self {
        let decryption_key_share = epoch_store
            .load_validator_decryption_key_shares_from_system_state()
            .unwrap_or(HashMap::new());
        let encryption = epoch_store
            .load_decryption_key_shares_from_system_state()
            .unwrap_or(HashMap::new());
        let status = if encryption.is_empty() || decryption_key_share.is_empty() {
            DwalletMPCNetworkKeysStatus::NotInitialized
        } else {
            DwalletMPCNetworkKeysStatus::Ready(decryption_key_share.keys().copied().collect())
        };

        Self {
            validator_decryption_key_share: Arc::new(RwLock::new(decryption_key_share)),
            key_shares_versions: Arc::new(RwLock::new(encryption)),
            status: Arc::new(RwLock::new(status)),
        }
    }

    /// Returns the latest version of the given key type.
    pub fn key_version(&self, key_type: DWalletMPCNetworkKey) -> DwalletMPCResult<u8> {
        let decryption_key_share = self
            .validator_decryption_key_share
            .read()
            .map_err(|_| DwalletMPCError::LockError)?;
        Ok(decryption_key_share
            .get(&key_type)
            .ok_or(DwalletMPCError::InvalidMPCPartyType)?
            .len() as u8)
    }

    /// Update the key version with the new shares. Used after the re-sharing is done.
    pub fn update_key_version(
        &self,
        key_type: DWalletMPCNetworkKey,
        version: u8,
        new_shares: Vec<Vec<u8>>,
    ) -> DwalletMPCResult<()> {
        let mut encryption = self
            .key_shares_versions
            .write()
            .map_err(|_| DwalletMPCError::LockError)?;
        let key_shares = encryption
            .get_mut(&key_type)
            .ok_or(DwalletMPCError::InvalidMPCPartyType)?;
        let current_version = key_shares
            .get_mut(version as usize)
            .ok_or(DwalletMPCError::InvalidMPCPartyType)?;

        current_version.previous_epoch_shares = current_version.current_epoch_shares.clone();
        current_version.current_epoch_shares = new_shares;
        Ok(())
    }

    /// Add a new key version with the given shares. Used after the network DKG is done.
    pub fn add_key_version(
        &self,
        epoch_store: Arc<AuthorityPerEpochStore>,
        key_type: DWalletMPCNetworkKey,
        self_decryption_key_share: Vec<u8>,
        encryption_of_decryption_shares: Vec<u8>,
    ) -> DwalletMPCResult<()> {
        let mut decryption_key_share = self
            .validator_decryption_key_share
            .write()
            .map_err(|_| DwalletMPCError::LockError)?;
        // Todo (#382): Replace with the actual type once the DKG protocol is ready.
        decryption_key_share
            .entry(key_type.clone())
            .or_insert_with(Vec::new)
            .push(self_decryption_key_share.clone());

        let mut encryption = self
            .key_shares_versions
            .write()
            .map_err(|_| DwalletMPCError::LockError)?;
        encryption.insert(
            key_type.clone(),
            vec![EncryptionOfNetworkDecryptionKeyShares {
                epoch: epoch_store.epoch(),
                // Todo (#382): Replace with the actual type once the DKG protocol is ready.
                current_epoch_shares: vec![encryption_of_decryption_shares],
                previous_epoch_shares: vec![],
            }],
        );

        let mut status = self
            .status
            .write()
            .map_err(|_| DwalletMPCError::LockError)?;
        if let DwalletMPCNetworkKeysStatus::Ready(keys) = &mut *status {
            keys.insert(key_type);
            *status = DwalletMPCNetworkKeysStatus::Ready(keys.clone());
        } else {
            *status = DwalletMPCNetworkKeysStatus::Ready(HashSet::from([key_type]));
        }
        Ok(())
    }

    /// Returns all versions of the decryption key shares for the specified key type.
    // Todo (#382): Replace with the actual type once the DKG protocol is ready.
    pub fn get_decryption_key_share(
        &self,
        key_type: DWalletMPCNetworkKey,
    ) -> DwalletMPCResult<Vec<Vec<u8>>> {
        let decryption_key_share = self
            .validator_decryption_key_share
            .read()
            .map_err(|_| DwalletMPCError::LockError)?;

        Ok(decryption_key_share
            .get(&key_type)
            .ok_or(DwalletMPCError::InvalidMPCPartyType)?
            .clone())
    }

    /// Returns the status of the dWallet MPC network keys.
    pub fn status(&self) -> DwalletMPCResult<DwalletMPCNetworkKeysStatus> {
        let status = self.status.read().map_err(|_| DwalletMPCError::LockError)?;
        Ok(status.clone())
    }
}

/// Advances the network DKG protocol for the supported key types.
pub(crate) fn advance_network_dkg(
    session_id: CommitmentSizedNumber,
    weighted_threshold_access_structure: &WeightedThresholdAccessStructure,
    party_id: PartyID,
    public_input: &[u8],
    key_type: &DWalletMPCNetworkKey,
    messages: Vec<HashMap<PartyID, Vec<u8>>>,
) -> DwalletMPCResult<mpc::AsynchronousRoundResult<Vec<u8>, Vec<u8>, Vec<u8>>> {
    Ok(match key_type {
        // Todo (#382): Replace with the actual implementation once the DKG protocol is ready.
        DWalletMPCNetworkKey::Secp256k1 => advance::<DKGFirstParty>(
            session_id,
            party_id,
            &weighted_threshold_access_structure,
            messages,
            bcs::from_bytes(public_input)?,
            (),
        ),
        // Todo (#382): Replace with the actual implementation once the DKG protocol is ready.
        DWalletMPCNetworkKey::Ristretto => advance::<DKGFirstParty>(
            session_id,
            party_id,
            &weighted_threshold_access_structure,
            messages,
            bcs::from_bytes(public_input)?,
            (),
        ),
    }?)
}

pub fn network_dkg_party(
    deserialized_event: StartNetworkDKGEvent,
) -> DwalletMPCResult<(MPCParty, Vec<u8>, SessionInfo)> {
    match DWalletMPCNetworkKey::try_from(deserialized_event.key_type)? {
        DWalletMPCNetworkKey::Secp256k1 => Ok(dkg_secp256k1_party(deserialized_event)?),
        DWalletMPCNetworkKey::Ristretto => Ok(dkg_ristretto_party(deserialized_event)?),
    }
}

pub fn network_dkg_session_info(
    deserialized_event: StartNetworkDKGEvent,
) -> DwalletMPCResult<SessionInfo> {
    match DWalletMPCNetworkKey::try_from(deserialized_event.key_type)? {
        DWalletMPCNetworkKey::Secp256k1 => Ok(dkg_secp256k1_session_info(deserialized_event)),
        DWalletMPCNetworkKey::Ristretto => Ok(dkg_ristretto_session_info(deserialized_event)),
        _ => Err(DwalletMPCError::InvalidMPCPartyType),
    }
}

fn dkg_secp256k1_party(
    deserialized_event: StartNetworkDKGEvent,
) -> DwalletMPCResult<(MPCParty, Vec<u8>, SessionInfo)> {
    Ok((
        MPCParty::NetworkDkg(DWalletMPCNetworkKey::Secp256k1),
        generate_secp256k1_dkg_party_public_input(HashMap::new())?,
        dkg_secp256k1_session_info(deserialized_event),
    ))
}

fn dkg_secp256k1_session_info(deserialized_event: StartNetworkDKGEvent) -> SessionInfo {
    SessionInfo {
        flow_session_id: deserialized_event.session_id.bytes,
        session_id: deserialized_event.session_id.bytes,
        initiating_user_address: Default::default(),
        mpc_round: MPCRound::NetworkDkg(DWalletMPCNetworkKey::Secp256k1),
    }
}

fn dkg_ristretto_party(
    deserialized_event: StartNetworkDKGEvent,
) -> DwalletMPCResult<(MPCParty, Vec<u8>, SessionInfo)> {
    Ok((
        MPCParty::NetworkDkg(DWalletMPCNetworkKey::Ristretto),
        generate_ristretto_dkg_party_public_input(HashMap::new())?,
        dkg_ristretto_session_info(deserialized_event),
    ))
}

fn dkg_ristretto_session_info(deserialized_event: StartNetworkDKGEvent) -> SessionInfo {
    SessionInfo {
        flow_session_id: deserialized_event.session_id.bytes,
        session_id: deserialized_event.session_id.bytes,
        initiating_user_address: Default::default(),
        mpc_round: MPCRound::NetworkDkg(DWalletMPCNetworkKey::Ristretto),
    }
}

// Todo (#382): Replace with the actual implementation once the DKG protocol is ready.
fn generate_secp256k1_dkg_party_public_input(
    secret_key_share_sized_encryption_keys_and_proofs: HashMap<
        PartyID,
        ClassGroupsEncryptionKeyAndProof,
    >,
) -> DwalletMPCResult<Vec<u8>> {
    let public_params = Secp256k1PublicInput::new(
    secp256k1::scalar::PublicParameters::default(),
    DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER,
    mock_class_groups_encryption_keys_and_proofs(),
    )?;
    bcs::to_bytes(&public_params).map_err(|e| DwalletMPCError::BcsError(e))
}

// Todo (#382): Replace with the actual implementation once the DKG protocol is ready.
fn generate_ristretto_dkg_party_public_input(
    secret_key_share_sized_encryption_keys_and_proofs: HashMap<
        PartyID,
        ClassGroupsEncryptionKeyAndProof,
    >,
) -> DwalletMPCResult<Vec<u8>> {
    <DKGFirstParty as crate::dwallet_mpc::dkg::DKGFirstPartyPublicInputGenerator>::generate_public_input()
}

fn mock_class_groups_encryption_keys_and_proofs() -> HashMap<PartyID, [(
    CompactIbqf<{ CRT_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS }>,
    KnowledgeOfDiscreteLogUCProof,
); MAX_PRIMES]> {
    let mut encryption_keys_and_proofs = HashMap::new();
    (1..=3).for_each(|i| {
        encryption_keys_and_proofs.insert(
            i as PartyID,
            read_class_groups_from_file_real("class-groups-0x65152c88f31ae37ceda117b57ee755fc0a5b035a2ecfde61d6c982ffea818d09.key")?,
        );
    });
    encryption_keys_and_proofs
}
