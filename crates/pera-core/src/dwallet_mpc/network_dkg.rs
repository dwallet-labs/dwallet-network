//! This module contains the network DKG protocol for the dWallet MPC sessions.
//! The network DKG protocol handles generating the network Decryption-Key shares.
//! The module provides the management of the network Decryption-Key shares and
//! the network DKG protocol.
//! It provides inner mutability for the [`EpochStore`]
//! to update the network decryption key shares synchronously.
use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::dwallet_mpc::mpc_events::StartNetworkDKGEvent;
use crate::dwallet_mpc::mpc_session::AsyncProtocol;
use crate::dwallet_mpc::{advance, authority_name_to_party_id};
use class_groups::dkg::{
    RistrettoParty, RistrettoPublicInput, Secp256k1Party, Secp256k1PublicInput,
};
use class_groups::{
    SecretKeyShareSizedNumber, DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER,
    SECP256K1_FUNDAMENTAL_DISCRIMINANT_LIMBS, SECP256K1_SCALAR_LIMBS,
};
use commitment::CommitmentSizedNumber;
use dwallet_classgroups_types::mock_class_groups::{
    mock_cg_encryption_keys_and_proofs, mock_cg_private_key,
};
use dwallet_classgroups_types::{ClassGroupsDecryptionKey, ClassGroupsEncryptionKeyAndProof};
use dwallet_mpc_types::dwallet_mpc::{DWalletMPCNetworkKeyScheme, NetworkDecryptionKeyShares};
use group::{ristretto, secp256k1, PartyID};
use homomorphic_encryption::AdditivelyHomomorphicDecryptionKeyShare;
use mpc::WeightedThresholdAccessStructure;
use pera_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use pera_types::messages_dwallet_mpc::{MPCRound, SessionInfo};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use twopc_mpc::secp256k1::class_groups::{
    FUNDAMENTAL_DISCRIMINANT_LIMBS, NON_FUNDAMENTAL_DISCRIMINANT_LIMBS,
};
use twopc_mpc::sign::Protocol;
use twopc_mpc::ProtocolPublicParameters;

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
    pub key_shares_versions: HashMap<DWalletMPCNetworkKeyScheme, Vec<NetworkDecryptionKeyShares>>,
    /// The status of the network supported key types for the dWallet MPC sessions.
    pub status: DwalletMPCNetworkKeysStatus,
}

impl DwalletMPCNetworkKeyVersions {
    /// Creates a new instance of the network encryption key shares.
    pub fn new(
        epoch_store: &AuthorityPerEpochStore,
        weighted_threshold_access_structure: &WeightedThresholdAccessStructure,
        class_groups_decryption_key: ClassGroupsDecryptionKey,
    ) -> Self {
        // Safe to unwrap because the authority name is always present in the epoch store.
        let party_id = authority_name_to_party_id(&epoch_store.name, &epoch_store).unwrap();

        #[cfg(not(feature = "with-network-dkg"))]
        {
            return Self::mock_network_dkg(
                epoch_store,
                &weighted_threshold_access_structure,
                party_id,
            );
        }

        let encryption = epoch_store
            .load_decryption_key_shares_from_system_state()
            .unwrap_or(HashMap::new());
        let decryption_key_share = Self::validator_decryption_key_shares(
            &encryption,
            weighted_threshold_access_structure,
            class_groups_decryption_key,
            party_id,
        )
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

    fn mock_network_dkg(
        epoch_store: &AuthorityPerEpochStore,
        weighted_threshold_access_structure: &WeightedThresholdAccessStructure,
        party_id: PartyID,
    ) -> Self {
        let public_output = class_groups_constants::network_dkg_final_output();
        let decryption_shares = class_groups_constants::decryption_key_share(party_id);

        let new_key_version = Self::new_dwallet_mpc_network_key(
            bcs::to_bytes(&public_output).unwrap(),
            DWalletMPCNetworkKeyScheme::Secp256k1,
            epoch_store.epoch(),
            &weighted_threshold_access_structure,
        )
        .unwrap();

        let self_decryption_key_share = decryption_shares
            .into_iter()
            .map(|(party_id, secret_key_share)| {
                Ok((
                    party_id,
                    <AsyncProtocol as Protocol>::DecryptionKeyShare::new(
                        party_id,
                        secret_key_share,
                        &bcs::from_bytes(
                            &class_groups_constants::decryption_key_share_public_parameters(),
                        )?,
                    )
                    .unwrap(),
                ))
            })
            .collect::<DwalletMPCResult<HashMap<_, _>>>()
            .unwrap();

        Self {
            inner: Arc::new(RwLock::new(DwalletMPCNetworkKeyVersionsInner {
                validator_decryption_key_share: HashMap::from([(
                    DWalletMPCNetworkKeyScheme::Secp256k1,
                    vec![self_decryption_key_share],
                )]),
                key_shares_versions: HashMap::from([(
                    DWalletMPCNetworkKeyScheme::Secp256k1,
                    vec![new_key_version],
                )]),
                status: DwalletMPCNetworkKeysStatus::Ready(HashSet::from([
                    DWalletMPCNetworkKeyScheme::Secp256k1,
                ])),
            })),
        }
    }

    /// Retrieves the *running validator's* decryption key shares for each key scheme
    /// if they exist in the `key_shares_versions`.
    ///
    /// The data is sourced from the epoch's initial system state.
    /// The returned value is a map where:
    /// - The key represents the key scheme.
    /// - The value is a vector of decryption key shares for each network DKG.
    fn validator_decryption_key_shares(
        key_shares_versions: &HashMap<DWalletMPCNetworkKeyScheme, Vec<NetworkDecryptionKeyShares>>,
        weighted_threshold_access_structure: &WeightedThresholdAccessStructure,
        secret_key: ClassGroupsDecryptionKey,
        party_id: PartyID,
    ) -> DwalletMPCResult<
        HashMap<
            DWalletMPCNetworkKeyScheme,
            Vec<HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>>,
        >,
    > {
        key_shares_versions
            .into_iter()
            .map(|(key_scheme, encryption_shares)| {
                let shares = Self::decrypt_shares(
                    &encryption_shares,
                    party_id,
                    weighted_threshold_access_structure,
                    secret_key,
                )?;

                Ok((*key_scheme, shares))
            })
            .collect::<DwalletMPCResult<HashMap<_, _>>>()
    }

    fn decrypt_shares(
        shares: &Vec<NetworkDecryptionKeyShares>,
        party_id: PartyID,
        weighted_threshold_access_structure: &WeightedThresholdAccessStructure,
        secret_key: ClassGroupsDecryptionKey,
    ) -> DwalletMPCResult<Vec<HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>>>
    {
        shares
            .iter()
            .map(|share| {
                let dkg_public_output = <Secp256k1Party as CreatePublicOutput>::new(
                    &share.encryption_key,
                    &share.reconstructed_commitments_to_sharing,
                    &share.current_epoch_shares,
                )?;

                let shares_primes = dkg_public_output.default_decryption_key_shares::<
                    SECP256K1_SCALAR_LIMBS,
                    FUNDAMENTAL_DISCRIMINANT_LIMBS,
                    secp256k1::GroupElement,
                >(
                    party_id,
                    weighted_threshold_access_structure,
                    secret_key,
                )
                    .map_err(|err| {
                        DwalletMPCError::ClassGroupsError(err.to_string())
                    })?;
                Self::convert_secret_key_shares_to_decryption_shares(
                    shares_primes,
                    &share.decryption_public_parameters,
                )
            })
            .collect::<DwalletMPCResult<
                Vec<HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>>,
            >>()
    }

    fn convert_secret_key_shares_to_decryption_shares(
        shares_primes: HashMap<PartyID, SecretKeyShareSizedNumber>,
        public_parameters: &[u8],
    ) -> DwalletMPCResult<HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>> {
        let public_params = bcs::from_bytes(public_parameters)
            .map_err(|err| DwalletMPCError::ClassGroupsError(err.to_string()))?;

        shares_primes
            .into_iter()
            .map(|(party_id, secret_key_share)| {
                let decryption_key_share = <AsyncProtocol as Protocol>::DecryptionKeyShare::new(
                    party_id,
                    secret_key_share,
                    &public_params,
                )
                .map_err(|err| DwalletMPCError::ClassGroupsError(err.to_string()))?;

                Ok((party_id, decryption_key_share))
            })
            .collect::<DwalletMPCResult<HashMap<_, _>>>()
    }

    /// Returns the latest version of the given key type.
    /// The latest version is the last element in the vector (length -1).
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
        new_shares: Vec<u8>,
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
        self_decryption_key_share: HashMap<PartyID, SecretKeyShareSizedNumber>,
        dkg_public_output: Vec<u8>,
        access_structure: &WeightedThresholdAccessStructure,
    ) -> DwalletMPCResult<NetworkDecryptionKeyShares> {
        let mut inner = self.inner.write().map_err(|_| DwalletMPCError::LockError)?;

        let new_key_version = Self::new_dwallet_mpc_network_key(
            dkg_public_output,
            key_type,
            epoch_store.epoch(),
            access_structure,
        )?;
        let self_decryption_key_share = self_decryption_key_share
            .into_iter()
            .map(|(party_id, secret_key_share)| {
                Ok((
                    party_id,
                    <AsyncProtocol as Protocol>::DecryptionKeyShare::new(
                        party_id,
                        secret_key_share,
                        &bcs::from_bytes(&new_key_version.decryption_public_parameters)?,
                    )?,
                ))
            })
            .collect::<DwalletMPCResult<HashMap<_, _>>>()?;

        inner
            .key_shares_versions
            .insert(key_type.clone(), vec![new_key_version.clone()]);

        inner
            .validator_decryption_key_share
            .insert(key_type.clone(), vec![self_decryption_key_share]);
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
    ) -> DwalletMPCResult<NetworkDecryptionKeyShares> {
        match key_scheme {
            DWalletMPCNetworkKeyScheme::Secp256k1 => {
                let dkg_output: <Secp256k1Party as mpc::Party>::PublicOutput =
                    bcs::from_bytes(&dkg_output)?;
                Ok(NetworkDecryptionKeyShares {
                    epoch,
                    current_epoch_shares: bcs::to_bytes(&dkg_output.encryptions_of_shares_per_crt_prime)?,
                    previous_epoch_shares: vec![],
                    protocol_public_parameters: bcs::to_bytes(
                        &dkg_output.default_encryption_scheme_public_parameters::<
                            SECP256K1_SCALAR_LIMBS,
                            SECP256K1_FUNDAMENTAL_DISCRIMINANT_LIMBS,
                            secp256k1::GroupElement
                        >().map_err(|e| DwalletMPCError::ClassGroupsError(e.to_string()))?)?,
                    decryption_public_parameters: bcs::to_bytes(
                        &dkg_output.default_decryption_key_share_public_parameters::<
                            SECP256K1_SCALAR_LIMBS,
                            SECP256K1_FUNDAMENTAL_DISCRIMINANT_LIMBS,
                            secp256k1::GroupElement
                        >(access_structure).map_err(|e|DwalletMPCError::ClassGroupsError(e.to_string()))?)?,
                    encryption_key: bcs::to_bytes(&dkg_output.encryption_key)?,
                    reconstructed_commitments_to_sharing: bcs::to_bytes(&dkg_output.reconstructed_commitments_to_sharing)?,
                })
            }
            DWalletMPCNetworkKeyScheme::Ristretto => Ok(NetworkDecryptionKeyShares {
                epoch,
                current_epoch_shares: vec![],
                previous_epoch_shares: vec![],
                protocol_public_parameters: vec![],
                decryption_public_parameters: vec![],
                encryption_key: vec![],
                reconstructed_commitments_to_sharing: vec![],
            }),
        }
    }

    /// Returns all versions of the decryption key shares for the specified key type.
    // Todo (#382): Replace with the actual type once the DKG protocol is ready.
    pub fn get_decryption_key_share(
        &self,
        key_scheme: DWalletMPCNetworkKeyScheme,
    ) -> DwalletMPCResult<Vec<HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>>>
    {
        let inner = self.inner.read().map_err(|_| DwalletMPCError::LockError)?;
        Ok(inner
            .validator_decryption_key_share
            .get(&key_scheme)
            .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionKeyShares)?
            .clone())
    }

    pub fn get_decryption_public_parameters(
        &self,
        key_scheme: DWalletMPCNetworkKeyScheme,
        key_version: u8,
    ) -> DwalletMPCResult<Vec<u8>> {
        let inner = self.inner.read().map_err(|_| DwalletMPCError::LockError)?;
        Ok(inner
            .key_shares_versions
            .get(&key_scheme)
            .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionKeyShares)?
            .get(key_version as usize)
            .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionKeyShares)?
            .decryption_public_parameters
            .clone())
    }

    pub fn get_protocol_public_parameters(
        &self,
        key_scheme: DWalletMPCNetworkKeyScheme,
        key_version: u8,
    ) -> DwalletMPCResult<Vec<u8>> {
        let inner = self.inner.read().map_err(|_| DwalletMPCError::LockError)?;
        let pp = inner
            .key_shares_versions
            .get(&key_scheme)
            .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionKeyShares)?
            .get(key_version as usize)
            .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionKeyShares)?
            .protocol_public_parameters
            .clone();

        match key_scheme {
            DWalletMPCNetworkKeyScheme::Secp256k1 => {
                bcs::to_bytes(&ProtocolPublicParameters::new::<
                    { secp256k1::SCALAR_LIMBS },
                    { FUNDAMENTAL_DISCRIMINANT_LIMBS },
                    { NON_FUNDAMENTAL_DISCRIMINANT_LIMBS },
                    secp256k1::GroupElement,
                >(bcs::from_bytes(&pp)?))
                .map_err(|e| DwalletMPCError::BcsError(e))
            }
            DWalletMPCNetworkKeyScheme::Ristretto => {
                todo!()
            }
        }
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
) -> DwalletMPCResult<mpc::AsynchronousRoundResult<Vec<u8>, Vec<u8>, Vec<u8>>> {
    match key_scheme {
        // Todo (#382): Replace with the actual implementation once the DKG protocol is ready.
        DWalletMPCNetworkKeyScheme::Secp256k1 => advance::<Secp256k1Party>(
            session_id,
            party_id,
            &weighted_threshold_access_structure,
            messages,
            bcs::from_bytes(public_input)?,
            mock_cg_private_key()?,
        ),
        // Todo (#382): Replace with the actual implementation once the DKG protocol is ready.
        DWalletMPCNetworkKeyScheme::Ristretto => advance::<RistrettoParty>(
            session_id,
            party_id,
            &weighted_threshold_access_structure,
            messages,
            bcs::from_bytes(public_input)?,
            mock_cg_private_key()?,
        ),
    }
}
pub(super) fn network_dkg_public_input(
    deserialized_event: StartNetworkDKGEvent,
) -> DwalletMPCResult<Vec<u8>> {
    match DWalletMPCNetworkKeyScheme::try_from(deserialized_event.key_scheme)? {
        DWalletMPCNetworkKeyScheme::Secp256k1 => {
            generate_secp256k1_dkg_party_public_input(HashMap::new())
        }
        DWalletMPCNetworkKeyScheme::Ristretto => {
            generate_ristretto_dkg_party_public_input(HashMap::new())
        }
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

fn dkg_secp256k1_session_info(deserialized_event: StartNetworkDKGEvent) -> SessionInfo {
    SessionInfo {
        flow_session_id: deserialized_event.session_id.bytes,
        session_id: deserialized_event.session_id.bytes,
        initiating_user_address: Default::default(),
        mpc_round: MPCRound::NetworkDkg(DWalletMPCNetworkKeyScheme::Secp256k1, None),
    }
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
        mock_cg_encryption_keys_and_proofs()?,
    )
    .map_err(|e| DwalletMPCError::InvalidMPCPartyType)?; // change to the actual error
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
        mock_cg_encryption_keys_and_proofs()?,
    )
    .map_err(|e| DwalletMPCError::InvalidMPCPartyType)?;
    bcs::to_bytes(&public_params).map_err(|e| DwalletMPCError::BcsError(e))
}

pub(crate) fn dwallet_mpc_network_key_from_session_output(
    epoch: u64,
    key_scheme: DWalletMPCNetworkKeyScheme,
    weighted_threshold_access_structure: &WeightedThresholdAccessStructure,
    public_output: &[u8],
) -> DwalletMPCResult<NetworkDecryptionKeyShares> {
    match key_scheme {
        DWalletMPCNetworkKeyScheme::Secp256k1 => {
            let public_output: <Secp256k1Party as mpc::Party>::PublicOutput =
                bcs::from_bytes(&public_output)?;
            let protocol_public_parameters =
                public_output.default_encryption_scheme_public_parameters::<
                    SECP256K1_SCALAR_LIMBS,
                    SECP256K1_FUNDAMENTAL_DISCRIMINANT_LIMBS,
                    secp256k1::GroupElement,
                >().map_err(|e| DwalletMPCError::ClassGroupsError(e.to_string()))?;
            let decryption_public_parameters = public_output.default_decryption_key_share_public_parameters::<
                SECP256K1_SCALAR_LIMBS,
                SECP256K1_FUNDAMENTAL_DISCRIMINANT_LIMBS,
                secp256k1::GroupElement,
            >(weighted_threshold_access_structure).map_err(|e| DwalletMPCError::ClassGroupsError(e.to_string()))?;

            Ok(NetworkDecryptionKeyShares {
                epoch,
                current_epoch_shares: bcs::to_bytes(
                    &public_output.encryptions_of_shares_per_crt_prime,
                )?,
                previous_epoch_shares: vec![],
                protocol_public_parameters: bcs::to_bytes(&protocol_public_parameters)?,
                decryption_public_parameters: bcs::to_bytes(&decryption_public_parameters)?,
                encryption_key: bcs::to_bytes(&public_output.encryption_key)?,
                reconstructed_commitments_to_sharing: bcs::to_bytes(
                    &public_output.reconstructed_commitments_to_sharing,
                )?,
            })
        }
        DWalletMPCNetworkKeyScheme::Ristretto => todo!("Ristretto key scheme"),
    }
}

pub trait CreatePublicOutput: mpc::Party {
    fn new(
        encryption_key: &Vec<u8>,
        reconstructed_commitments_to_sharing: &Vec<u8>,
        current_epoch_shares: &Vec<u8>,
    ) -> DwalletMPCResult<Self::PublicOutput>;
}

impl CreatePublicOutput for Secp256k1Party {
    fn new(
        encryption_key: &Vec<u8>,
        reconstructed_commitments_to_sharing: &Vec<u8>,
        current_epoch_shares: &Vec<u8>,
    ) -> DwalletMPCResult<Self::PublicOutput> {
        let dkg_public_output = Self::PublicOutput {
            encryption_key: bcs::from_bytes(encryption_key)?,
            reconstructed_commitments_to_sharing: bcs::from_bytes(
                reconstructed_commitments_to_sharing,
            )?,
            encryptions_of_shares_per_crt_prime: bcs::from_bytes(current_epoch_shares)?,
        };

        Ok(dkg_public_output)
    }
}
