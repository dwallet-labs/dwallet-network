//! This module contains the network DKG protocol for the dWallet MPC sessions.
//! The network DKG protocol handles generating the network Decryption-Key shares.
//! The module provides the management of the network Decryption-Key shares and
//! the network DKG protocol.
//! It provides inner mutability for the [`EpochStore`]
//! to update the network decryption key shares synchronously.
use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::dwallet_mpc::advance;
use crate::dwallet_mpc::dkg::DKGFirstParty;
use crate::dwallet_mpc::mpc_events::StartNetworkDKGEvent;
use crate::dwallet_mpc::mpc_party::MPCParty;
use commitment::CommitmentSizedNumber;
use dwallet_mpc_types::dwallet_mpc::{DWalletMPCNetworkKey, NetworkDecryptionKeyShares};
use group::PartyID;
use mpc::WeightedThresholdAccessStructure;
use pera_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use pera_types::messages_dwallet_mpc::{MPCRound, SessionInfo};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use dwallet_classgroups_types::ClassGroupsEncryptionKeyAndProof;

/// The status of the network supported key types for the dWallet MPC sessions.
#[derive(Clone, Debug, PartialEq)]
pub enum DwalletMPCNetworkKeysStatus {
    /// The network supported key types have been updated or initialized.
    Ready(HashSet<DWalletMPCNetworkKey>),
    /// None of the network supported key types have been initialized.
    NotInitialized,
}

/// Holds the network keys of the dWallet MPC protocols.
pub struct DwalletMPCNetworkKeyVersions {
    inner: Arc<RwLock<DwalletMPCNetworkKeyVersionsInner>>,
}

/// Encapsulates all the fields in a single structure for atomic access.
pub struct DwalletMPCNetworkKeyVersionsInner {
    /// The validators' decryption key shares.
    pub validator_decryption_key_share: HashMap<DWalletMPCNetworkKey, Vec<Vec<u8>>>,
    /// The dWallet MPC network decryption key shares (encrypted).
    /// Map from key type to the encryption of the key version.
    pub key_shares_versions: HashMap<DWalletMPCNetworkKey, Vec<NetworkDecryptionKeyShares>>,
    /// The status of the network supported key types for the dWallet MPC sessions.
    pub status: DwalletMPCNetworkKeysStatus,
}

impl DwalletMPCNetworkKeyVersions {
    /// Creates a new instance of the network encryption key shares.
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
            inner: Arc::new(RwLock::new(DwalletMPCNetworkKeyVersionsInner {
                validator_decryption_key_share: decryption_key_share,
                key_shares_versions: encryption,
                status,
            })),
        }
    }

    /// Returns the latest version of the given key type.
    pub fn key_version(&self, key_type: DWalletMPCNetworkKey) -> DwalletMPCResult<u8> {
        let inner = self.inner.read().map_err(|_| DwalletMPCError::LockError)?;
        Ok(inner
            .validator_decryption_key_share
            .get(&key_type)
            .ok_or(DwalletMPCError::InvalidMPCPartyType)?
            .len() as u8)
    }

    /// Update the key version with the new shares.
    /// Used after the re-sharing is done.
    pub fn update_key_version(
        &self,
        key_type: DWalletMPCNetworkKey,
        version: u8,
        new_shares: Vec<Vec<u8>>,
    ) -> DwalletMPCResult<()> {
        let mut inner = self.inner.write().map_err(|_| DwalletMPCError::LockError)?;
        let key_shares = inner
            .key_shares_versions
            .get_mut(&key_type)
            .ok_or(DwalletMPCError::InvalidMPCPartyType)?;
        let current_version = key_shares
            .get_mut(version as usize)
            .ok_or(DwalletMPCError::InvalidMPCPartyType)?;

        current_version.previous_epoch_shares = current_version.current_epoch_shares.clone();
        current_version.current_epoch_shares = new_shares;
        Ok(())
    }

    /// Add a new key version with the given shares.
    /// Used after the network DKG is done.
    pub fn add_key_version(
        &self,
        epoch_store: Arc<AuthorityPerEpochStore>,
        key_type: DWalletMPCNetworkKey,
        self_decryption_key_share: Vec<u8>,
        encryption_of_decryption_shares: Vec<u8>,
    ) -> DwalletMPCResult<()> {
        let mut inner = self.inner.write().map_err(|_| DwalletMPCError::LockError)?;

        inner
            .validator_decryption_key_share
            .entry(key_type.clone())
            .or_insert_with(Vec::new)
            .push(self_decryption_key_share.clone());

        inner.key_shares_versions.insert(
            key_type.clone(),
            vec![NetworkDecryptionKeyShares {
                epoch: epoch_store.epoch(),
                current_epoch_shares: vec![encryption_of_decryption_shares],
                previous_epoch_shares: vec![],
            }],
        );

        if let DwalletMPCNetworkKeysStatus::Ready(keys) = &mut inner.status {
            keys.insert(key_type);
            inner.status = DwalletMPCNetworkKeysStatus::Ready(keys.clone());
        } else {
            inner.status = DwalletMPCNetworkKeysStatus::Ready(HashSet::from([key_type]));
        }
        Ok(())
    }

    /// Returns all versions of the decryption key shares for the specified key type.
    // Todo (#382): Replace with the actual type once the DKG protocol is ready.
    pub fn get_decryption_key_share(
        &self,
        key_type: DWalletMPCNetworkKey,
    ) -> DwalletMPCResult<Vec<Vec<u8>>> {
        let inner = self.inner.read().map_err(|_| DwalletMPCError::LockError)?;
        Ok(inner
            .validator_decryption_key_share
            .get(&key_type)
            .ok_or(DwalletMPCError::InvalidMPCPartyType)?
            .clone())
    }

    /// Returns the status of the dWallet MPC network keys.
    pub fn status(&self) -> DwalletMPCResult<DwalletMPCNetworkKeysStatus> {
        let inner = self.inner.read().map_err(|_| DwalletMPCError::LockError)?;
        Ok(inner.status.clone())
    }
}

/// Advances the network DKG protocol for the supported key types.
pub(crate) fn advance_network_dkg(
    session_id: CommitmentSizedNumber,
    weighted_threshold_access_structure: &WeightedThresholdAccessStructure,
    party_id: PartyID,
    public_input: &[u8],
    key_scheme: &DWalletMPCNetworkKey,
    messages: Vec<HashMap<PartyID, Vec<u8>>>,
) -> DwalletMPCResult<mpc::AsynchronousRoundResult<Vec<u8>, Vec<u8>, Vec<u8>>> {
    Ok(match key_scheme {
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

pub(super) fn network_dkg_party(
    deserialized_event: StartNetworkDKGEvent,
) -> DwalletMPCResult<(MPCParty, Vec<u8>, SessionInfo)> {
    match DWalletMPCNetworkKey::try_from(deserialized_event.key_scheme)? {
        DWalletMPCNetworkKey::Secp256k1 => Ok(dkg_secp256k1_party(deserialized_event)?),
        DWalletMPCNetworkKey::Ristretto => Ok(dkg_ristretto_party(deserialized_event)?),
    }
}

pub(super) fn network_dkg_session_info(
    deserialized_event: StartNetworkDKGEvent,
) -> DwalletMPCResult<SessionInfo> {
    match DWalletMPCNetworkKey::try_from(deserialized_event.key_scheme)? {
        DWalletMPCNetworkKey::Secp256k1 => Ok(dkg_secp256k1_session_info(deserialized_event)),
        DWalletMPCNetworkKey::Ristretto => Ok(dkg_ristretto_session_info(deserialized_event)),
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
    _secret_key_share_sized_encryption_keys_and_proofs: HashMap<
        PartyID,
        ClassGroupsEncryptionKeyAndProof,
    >,
) -> DwalletMPCResult<Vec<u8>> {
    <DKGFirstParty as crate::dwallet_mpc::dkg::DKGFirstPartyPublicInputGenerator>::generate_public_input()
}

// Todo (#382): Replace with the actual implementation once the DKG protocol is ready.
fn generate_ristretto_dkg_party_public_input(
    _secret_key_share_sized_encryption_keys_and_proofs: HashMap<
        PartyID,
        ClassGroupsEncryptionKeyAndProof,
    >,
) -> DwalletMPCResult<Vec<u8>> {
    <DKGFirstParty as crate::dwallet_mpc::dkg::DKGFirstPartyPublicInputGenerator>::generate_public_input()
}
