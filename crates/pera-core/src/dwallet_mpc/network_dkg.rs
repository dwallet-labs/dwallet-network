//! This module contains the network DKG protocol for the dWallet MPC sessions.
//! The network DKG protocol handles generating the network Decryption-Key shares.
//! The module provides the management of the network Decryption-Key shares and
//! the network DKG protocol.
//! It provides inner mutability for the [`EpochStore`]
//! to update the network decryption key shares synchronously.
use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::dwallet_mpc::advance;
use crate::dwallet_mpc::mpc_events::StartNetworkDKGEvent;
use crate::dwallet_mpc::mpc_party::{AsyncProtocol, MPCParty};
use class_groups::dkg::{
    RistrettoParty, RistrettoPublicInput, Secp256k1Party, Secp256k1PublicInput,
};
use class_groups::{
    encryption_key, CompactIbqf, KnowledgeOfDiscreteLogUCProof,
    CRT_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS, DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER, MAX_PRIMES,
    SECP256K1_FUNDAMENTAL_DISCRIMINANT_LIMBS, SECP256K1_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS,
    SECP256K1_SCALAR_LIMBS,
};
use commitment::CommitmentSizedNumber;
use dwallet_mpc_types::class_groups_key::{read_class_groups_from_file_real, read_class_groups_private_key_from_file_real, ClassGroupsDecryptionKey, ClassGroupsEncryptionKeyAndProof};
use dwallet_mpc_types::dwallet_mpc::MPCPrivateOutput;
use group::{ristretto, secp256k1, PartyID};
use homomorphic_encryption::AdditivelyHomomorphicDecryptionKeyShare;
use mpc::{
    AsynchronousRoundResult, AsynchronouslyAdvanceable, Party, WeightedThresholdAccessStructure,
};
use pera_types::crypto::Signable;
use pera_types::dwallet_mpc::{DWalletMPCNetworkKeyScheme, DwalletMPCNetworkKey};
use pera_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use pera_types::messages_dwallet_mpc::{MPCRound, SessionInfo};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use twopc_mpc::sign::Protocol;

/// The status of the network supported key types for the dWallet MPC sessions.
#[derive(Clone, Debug, PartialEq)]
pub enum DwalletMPCNetworkKeysStatus {
    /// The network supported key types have been updated or initialized.
    Ready(HashSet<DWalletMPCNetworkKeyScheme>),
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
    pub validator_decryption_key_share: HashMap<
        DWalletMPCNetworkKeyScheme,
        Vec<HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>>,
    >,
    /// The dWallet MPC network decryption key shares (encrypted).
    /// Map from key type to the encryption of the key version.
    pub key_shares_versions: HashMap<DWalletMPCNetworkKeyScheme, Vec<DwalletMPCNetworkKey>>,
    /// The status of the network supported key types for the dWallet MPC sessions.
    pub status: DwalletMPCNetworkKeysStatus,
}

impl DwalletMPCNetworkKeyVersions {
    /// Creates a new instance of the network encryption key shares.
    pub fn new(epoch_store: &AuthorityPerEpochStore) -> Self {
        let decryption_key_share = HashMap::new();
        // epoch_store
        //     .load_validator_decryption_key_shares_from_system_state()
        //     .unwrap_or(HashMap::new());
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

    pub fn mock_network_dkg(&self, epoch_store: Arc<AuthorityPerEpochStore>, party_id: PartyID, weighted_threshold_access_structure: &WeightedThresholdAccessStructure){
        let public_output = class_groups_constants::network_dkg_final_output();
        let decryption_shares = public_output.default_decryption_key_shares::<SECP256K1_SCALAR_LIMBS, SECP256K1_FUNDAMENTAL_DISCRIMINANT_LIMBS, secp256k1::GroupElement>(party_id, weighted_threshold_access_structure, mock_cg_private_key()).unwrap();
        self.add_key_version(
            epoch_store,
            DWalletMPCNetworkKeyScheme::Secp256k1,
            decryption_shares,
            bcs::to_bytes(&public_output).unwrap(),
            &weighted_threshold_access_structure,
        ).unwrap();

        let mut inner = self.inner.write().map_err(|_| DwalletMPCError::LockError).unwrap();
        inner.status = DwalletMPCNetworkKeysStatus::Ready(HashSet::from([DWalletMPCNetworkKeyScheme::Secp256k1]));
    }

    /// Returns the latest version of the given key type.
    pub fn key_version(&self, key_type: DWalletMPCNetworkKeyScheme) -> DwalletMPCResult<u8> {
        let inner = self.inner.read().map_err(|_| DwalletMPCError::LockError)?;
        Ok(inner
            .validator_decryption_key_share
            .get(&key_type)
            .ok_or(DwalletMPCError::InvalidMPCPartyType)?
            .len() as u8
            - 1)
    }

    /// Update the key version with the new shares.
    /// Used after the re-sharing is done.
    pub fn update_key_version(
        &self,
        key_type: DWalletMPCNetworkKeyScheme,
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
        key_type: DWalletMPCNetworkKeyScheme,
        self_decryption_key_share: HashMap<PartyID, class_groups::SecretKeyShareSizedNumber>,
        dkg_public_output: Vec<u8>,
        access_structure: &WeightedThresholdAccessStructure,
    ) -> DwalletMPCResult<DwalletMPCNetworkKey> {
        let mut inner = self.inner.write().map_err(|_| DwalletMPCError::LockError)?;

        let new_key_version = Self::new_dwallet_mpc_network_key(
            dkg_public_output,
            key_type,
            epoch_store.epoch(),
            access_structure,
        )?;
        let pp = bcs::from_bytes(&new_key_version.decryption_public_parameters)?;
        let self_decryption_key_share = self_decryption_key_share
            .into_iter()
            .map(|(party_id, secret_key_share)| {
                Ok((
                    party_id,
                    <AsyncProtocol as Protocol>::DecryptionKeyShare::new(
                        party_id,
                        secret_key_share,
                        &pp,
                    )
                    .unwrap(),
                ))
            })
            .collect::<DwalletMPCResult<HashMap<_, _>>>()?;

        inner
            .key_shares_versions
            .insert(key_type.clone(), vec![new_key_version.clone()]);
        inner
            .validator_decryption_key_share
            .insert(key_type, vec![self_decryption_key_share]);

        if let DwalletMPCNetworkKeysStatus::Ready(keys) = &mut inner.status {
            keys.insert(key_type);
            inner.status = DwalletMPCNetworkKeysStatus::Ready(keys.clone());
        } else {
            inner.status = DwalletMPCNetworkKeysStatus::Ready(HashSet::from([key_type]));
        }
        Ok(new_key_version.clone())
    }

    fn new_dwallet_mpc_network_key(
        dkg_output: Vec<u8>,
        key_scheme: DWalletMPCNetworkKeyScheme,
        epoch: u64,
        access_structure: &WeightedThresholdAccessStructure,
    ) -> DwalletMPCResult<DwalletMPCNetworkKey> {
        match key_scheme {
            DWalletMPCNetworkKeyScheme::Secp256k1 => {
                let dkg_output: <Secp256k1Party as Party>::PublicOutput =
                    bcs::from_bytes(&dkg_output)?;
                Ok(DwalletMPCNetworkKey {
                    epoch,
                    current_epoch_shares: vec![bcs::to_bytes(&dkg_output.encryptions_of_shares_per_crt_prime)?],
                    previous_epoch_shares: vec![],
                    protocol_public_parameters: bcs::to_bytes(&dkg_output.default_encryption_scheme_public_parameters::<SECP256K1_SCALAR_LIMBS, SECP256K1_FUNDAMENTAL_DISCRIMINANT_LIMBS, secp256k1::GroupElement>().map_err(|_| DwalletMPCError::ClassGroupsError)?)?,
                    decryption_public_parameters: bcs::to_bytes(&dkg_output.default_decryption_key_share_public_parameters::<SECP256K1_SCALAR_LIMBS, SECP256K1_FUNDAMENTAL_DISCRIMINANT_LIMBS, secp256k1::GroupElement>(access_structure).map_err(|_|DwalletMPCError::ClassGroupsError)?)?,
                })
            }
            DWalletMPCNetworkKeyScheme::Ristretto => Ok(DwalletMPCNetworkKey {
                epoch,
                current_epoch_shares: vec![],
                previous_epoch_shares: vec![],
                protocol_public_parameters: vec![],
                decryption_public_parameters: vec![],
            }),
        }
    }

    /// Returns all versions of the decryption key shares for the specified key type.
    // Todo (#382): Replace with the actual type once the DKG protocol is ready.
    pub fn get_decryption_key_share(
        &self,
        key_type: DWalletMPCNetworkKeyScheme,
    ) -> DwalletMPCResult<Vec<HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>>>
    {
        let inner = self.inner.read().map_err(|_| DwalletMPCError::LockError)?;
        Ok(inner
            .validator_decryption_key_share
            .get(&key_type)
            .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionKeyShares)?
            .clone())
    }

    pub fn get_protocol_public_parameters(
        &self,
        key_scheme: DWalletMPCNetworkKeyScheme,
        key_version: u8,
    ) -> DwalletMPCResult<Vec<u8>> {
        let inner = self.inner.read().map_err(|_| DwalletMPCError::LockError)?;
        Ok(inner
            .key_shares_versions
            .get(&key_scheme)
            .ok_or(DwalletMPCError::InvalidMPCPartyType)?
            .get(key_version as usize)
            .ok_or(DwalletMPCError::InvalidMPCPartyType)?
            .protocol_public_parameters
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
    key_scheme: &DWalletMPCNetworkKeyScheme,
    messages: Vec<HashMap<PartyID, Vec<u8>>>,
) -> DwalletMPCResult<mpc::AsynchronousRoundResult<Vec<u8>, MPCPrivateOutput, Vec<u8>>> {
    let output = match key_scheme {
        // Todo (#382): Replace with the actual implementation once the DKG protocol is ready.
        DWalletMPCNetworkKeyScheme::Secp256k1 => advance::<Secp256k1Party>(
            session_id,
            party_id,
            &weighted_threshold_access_structure,
            messages,
            bcs::from_bytes(public_input)?,
            read_class_groups_private_key_from_file_real("class-groups-0x65152c88f31ae37ceda117b57ee755fc0a5b035a2ecfde61d6c982ffea818d09.key").unwrap(),
        ),
        // Todo (#382): Replace with the actual implementation once the DKG protocol is ready.
        DWalletMPCNetworkKeyScheme::Ristretto => advance::<RistrettoParty>(
            session_id,
            party_id,
            &weighted_threshold_access_structure,
            messages,
            bcs::from_bytes(public_input)?,
            read_class_groups_private_key_from_file_real("class-groups-0x65152c88f31ae37ceda117b57ee755fc0a5b035a2ecfde61d6c982ffea818d09.key").unwrap(),
        ),
    }?;

    match output {
        AsynchronousRoundResult::Finalize {
            malicious_parties,
            private_output,
            public_output,
        } => Ok(AsynchronousRoundResult::Finalize {
            malicious_parties,
            private_output: MPCPrivateOutput::DecryptionKeyShare(private_output),
            public_output,
        }),
        AsynchronousRoundResult::Advance {
            malicious_parties,
            message,
        } => Ok(AsynchronousRoundResult::Advance {
            malicious_parties,
            message,
        }),
    }
}

pub(super) fn network_dkg_party(
    deserialized_event: StartNetworkDKGEvent,
) -> DwalletMPCResult<(MPCParty, Vec<u8>, SessionInfo)> {
    match DWalletMPCNetworkKeyScheme::try_from(deserialized_event.key_scheme)? {
        DWalletMPCNetworkKeyScheme::Secp256k1 => Ok(dkg_secp256k1_party(deserialized_event)?),
        DWalletMPCNetworkKeyScheme::Ristretto => Ok(dkg_ristretto_party(deserialized_event)?),
    }
}

pub(super) fn network_dkg_session_info(
    deserialized_event: StartNetworkDKGEvent,
) -> DwalletMPCResult<SessionInfo> {
    match DWalletMPCNetworkKeyScheme::try_from(deserialized_event.key_scheme)? {
        DWalletMPCNetworkKeyScheme::Secp256k1 => Ok(dkg_secp256k1_session_info(deserialized_event)),
        DWalletMPCNetworkKeyScheme::Ristretto => Ok(dkg_ristretto_session_info(deserialized_event)),
    }
}

fn dkg_secp256k1_party(
    deserialized_event: StartNetworkDKGEvent,
) -> DwalletMPCResult<(MPCParty, Vec<u8>, SessionInfo)> {
    Ok((
        MPCParty::NetworkDkg(DWalletMPCNetworkKeyScheme::Secp256k1),
        generate_secp256k1_dkg_party_public_input(HashMap::new())?,
        dkg_secp256k1_session_info(deserialized_event),
    ))
}

fn dkg_secp256k1_session_info(deserialized_event: StartNetworkDKGEvent) -> SessionInfo {
    SessionInfo {
        flow_session_id: deserialized_event.session_id.bytes,
        session_id: deserialized_event.session_id.bytes,
        initiating_user_address: Default::default(),
        mpc_round: MPCRound::NetworkDkg(DWalletMPCNetworkKeyScheme::Secp256k1, None),
    }
}

fn dkg_ristretto_party(
    deserialized_event: StartNetworkDKGEvent,
) -> DwalletMPCResult<(MPCParty, Vec<u8>, SessionInfo)> {
    Ok((
        MPCParty::NetworkDkg(DWalletMPCNetworkKeyScheme::Ristretto),
        generate_ristretto_dkg_party_public_input(HashMap::new())?,
        dkg_ristretto_session_info(deserialized_event),
    ))
}

fn dkg_ristretto_session_info(deserialized_event: StartNetworkDKGEvent) -> SessionInfo {
    SessionInfo {
        flow_session_id: deserialized_event.session_id.bytes,
        session_id: deserialized_event.session_id.bytes,
        initiating_user_address: Default::default(),
        mpc_round: MPCRound::NetworkDkg(DWalletMPCNetworkKeyScheme::Ristretto, None),
    }
}

// Todo (#382): Replace with the actual implementation once the DKG protocol is ready.
fn generate_secp256k1_dkg_party_public_input(
    _secret_key_share_sized_encryption_keys_and_proofs: HashMap<
        PartyID,
        ClassGroupsEncryptionKeyAndProof,
    >,
) -> DwalletMPCResult<Vec<u8>> {
    let public_params = Secp256k1PublicInput::new::<secp256k1::GroupElement>(
        secp256k1::scalar::PublicParameters::default(),
        DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER,
        mock_class_groups_encryption_keys_and_proofs(),
    )
    .map_err(|e| DwalletMPCError::DKGNotOnFirstEpoch)?; // change to the actual error
    bcs::to_bytes(&public_params).map_err(|e| DwalletMPCError::BcsError(e))
}

// Todo (#382): Replace with the actual implementation once the DKG protocol is ready.
fn generate_ristretto_dkg_party_public_input(
    _secret_key_share_sized_encryption_keys_and_proofs: HashMap<
        PartyID,
        ClassGroupsEncryptionKeyAndProof,
    >,
) -> DwalletMPCResult<Vec<u8>> {
    let public_params = RistrettoPublicInput::new::<ristretto::GroupElement>(
        ristretto::scalar::PublicParameters::default(),
        DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER,
        mock_class_groups_encryption_keys_and_proofs(),
    )
    .unwrap();
    bcs::to_bytes(&public_params).map_err(|e| DwalletMPCError::BcsError(e))
}

fn mock_class_groups_encryption_keys_and_proofs() -> HashMap<
    PartyID,
    [(
        CompactIbqf<{ CRT_NON_FUNDAMENTAL_DISCRIMINANT_LIMBS }>,
        KnowledgeOfDiscreteLogUCProof,
    ); MAX_PRIMES],
> {
    let mut encryption_keys_and_proofs = HashMap::new();
    (1..=4).for_each(|i| {
        encryption_keys_and_proofs.insert(
            i as PartyID,
            read_class_groups_from_file_real("class-groups-0x65152c88f31ae37ceda117b57ee755fc0a5b035a2ecfde61d6c982ffea818d09.key").unwrap(),
        );
    });
    encryption_keys_and_proofs
}

pub fn mock_cg_private_key() -> ClassGroupsDecryptionKey {
    read_class_groups_private_key_from_file_real("class-groups-0x65152c88f31ae37ceda117b57ee755fc0a5b035a2ecfde61d6c982ffea818d09.key").unwrap()
}
