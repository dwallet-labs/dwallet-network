//! This module contains the network DKG protocol for the dWallet MPC sessions.
//! The network DKG protocol handles generating the network Decryption-Key shares.
//! The module provides the management of the network Decryption-Key shares and
//! the network DKG protocol.
//! It provides inner mutability for the [`EpochStore`]
//! to update the network decryption key shares synchronously.
use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::dwallet_mpc::mpc_events::{StartNetworkDKGEvent, ValidatorDataForNetworkDKG};
use crate::dwallet_mpc::mpc_session::AsyncProtocol;
use crate::dwallet_mpc::{advance_and_serialize, authority_name_to_party_id};
use class_groups::dkg::{
    RistrettoParty, RistrettoPublicInput, Secp256k1Party, Secp256k1PublicInput,
};
use class_groups::{
    SecretKeyShareSizedNumber, DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER,
    SECP256K1_FUNDAMENTAL_DISCRIMINANT_LIMBS, SECP256K1_SCALAR_LIMBS,
};
use commitment::CommitmentSizedNumber;
use dwallet_classgroups_types::ClassGroupsDecryptionKey;
use dwallet_mpc_types::dwallet_mpc::{DWalletMPCNetworkKeyScheme, NetworkDecryptionKeyShares};
use group::{ristretto, secp256k1, PartyID};
use homomorphic_encryption::AdditivelyHomomorphicDecryptionKeyShare;
use mpc::{AsynchronousRoundResult, WeightedThresholdAccessStructure};
use pera_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use pera_types::messages_dwallet_mpc::{MPCProtocolInitData, SessionInfo};
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
    /// Map from key type to the encryption of the share of the key of that type.
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

        #[cfg(feature = "mock-class-groups")]
        // This is used only for development purposes, the key is valid, so it is safe to unwrap.
        let class_groups_decryption_key =
            dwallet_classgroups_types::mock_class_groups::mock_cg_private_key().unwrap();

        let network_mpc_keys = epoch_store
            .load_decryption_key_shares_from_system_state()
            .unwrap_or(HashMap::new());
        let decryption_key_shares = Self::validator_decryption_key_shares(
            &network_mpc_keys,
            weighted_threshold_access_structure,
            class_groups_decryption_key,
            party_id,
        )
        .unwrap_or(HashMap::new());
        let status = if network_mpc_keys.is_empty() || decryption_key_shares.is_empty() {
            DwalletMPCNetworkKeysStatus::NotInitialized
        } else {
            DwalletMPCNetworkKeysStatus::Ready(decryption_key_shares.keys().copied().collect())
        };

        Self {
            inner: Arc::new(RwLock::new(DwalletMPCNetworkKeyVersionsInner {
                validator_decryption_key_share: decryption_key_shares,
                key_shares_versions: network_mpc_keys,
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
        let secret_shares = class_groups_constants::decryption_key_share(party_id);

        let new_key_version = Self::new_dwallet_mpc_network_key(
            bcs::to_bytes(&public_output).unwrap(),
            DWalletMPCNetworkKeyScheme::Secp256k1,
            epoch_store.epoch(),
            &weighted_threshold_access_structure,
        )
        .unwrap();

        let self_decryption_key_share = secret_shares
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
        network_mpc_keys: &HashMap<DWalletMPCNetworkKeyScheme, Vec<NetworkDecryptionKeyShares>>,
        weighted_threshold_access_structure: &WeightedThresholdAccessStructure,
        decryption_key: ClassGroupsDecryptionKey,
        party_id: PartyID,
    ) -> DwalletMPCResult<
        HashMap<
            DWalletMPCNetworkKeyScheme,
            Vec<HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>>,
        >,
    > {
        network_mpc_keys
            .into_iter()
            .map(|(key_scheme, network_mpc_key_versions)| {
                let shares = Self::get_decryption_key_shares_from_public_output(
                    &network_mpc_key_versions,
                    party_id,
                    weighted_threshold_access_structure,
                    decryption_key,
                )?;

                Ok((*key_scheme, shares))
            })
            .collect::<DwalletMPCResult<HashMap<_, _>>>()
    }

    /// The network DKG public output holds the all decryption key shares, the shares are encrypted.
    /// This function decrypts the shares of the current validator with the decryption (Class-Groups) key
    /// and returns the decryption key shares.
    fn get_decryption_key_shares_from_public_output(
        shares: &Vec<NetworkDecryptionKeyShares>,
        party_id: PartyID,
        weighted_threshold_access_structure: &WeightedThresholdAccessStructure,
        decryption_key: ClassGroupsDecryptionKey,
    ) -> DwalletMPCResult<Vec<HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>>>
    {
        shares
            .iter()
            .map(|share| {
                let dkg_public_output = <Secp256k1Party as CreatePublicOutput>::new(
                    &share.encryption_key,
                    &share.reconstructed_commitments_to_sharing,
                    &share.current_epoch_encryptions_of_shares_per_crt_prime,
                )?;

                let secret_share = dkg_public_output.default_decryption_key_shares::<
                    SECP256K1_SCALAR_LIMBS,
                    FUNDAMENTAL_DISCRIMINANT_LIMBS,
                    secp256k1::GroupElement,
                >(
                    party_id,
                    weighted_threshold_access_structure,
                    decryption_key,
                )
                    .map_err(|err| {
                        DwalletMPCError::ClassGroupsError(err.to_string())
                    })?;
                Self::convert_secret_key_shares_to_decryption_shares(
                    secret_share,
                    &share.decryption_key_share_public_parameters,
                )
            })
            .collect::<DwalletMPCResult<
                Vec<HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>>,
            >>()
    }

    fn convert_secret_key_shares_to_decryption_shares(
        secret_shares: HashMap<PartyID, SecretKeyShareSizedNumber>,
        public_parameters: &[u8],
    ) -> DwalletMPCResult<HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>> {
        let public_params = bcs::from_bytes(public_parameters)
            .map_err(|err| DwalletMPCError::ClassGroupsError(err.to_string()))?;

        secret_shares
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
        key_scheme: DWalletMPCNetworkKeyScheme,
        version: u8,
        new_encryptions_of_shares_per_crt_prime: Vec<u8>,
    ) -> DwalletMPCResult<()> {
        let mut inner = self.inner.write().map_err(|_| DwalletMPCError::LockError)?;
        let key_shares = inner
            .key_shares_versions
            .get_mut(&key_scheme)
            .ok_or(DwalletMPCError::InvalidMPCPartyType)?;

        let current_version = key_shares
            .get_mut(version as usize)
            .ok_or(DwalletMPCError::InvalidMPCPartyType)?;

        current_version.previous_epoch_encryptions_of_shares_per_crt_prime = current_version
            .current_epoch_encryptions_of_shares_per_crt_prime
            .clone();
        current_version.current_epoch_encryptions_of_shares_per_crt_prime =
            new_encryptions_of_shares_per_crt_prime;
        Ok(())
    }

    /// Add a new key version with the given shares.
    /// Used after the network DKG is done.
    pub fn add_key_version(
        &self,
        epoch_store: Arc<AuthorityPerEpochStore>,
        key_scheme: DWalletMPCNetworkKeyScheme,
        secret_key_share: HashMap<PartyID, SecretKeyShareSizedNumber>,
        dkg_public_output: Vec<u8>,
        access_structure: &WeightedThresholdAccessStructure,
    ) -> DwalletMPCResult<NetworkDecryptionKeyShares> {
        let mut inner = self.inner.write().map_err(|_| DwalletMPCError::LockError)?;

        let new_key_version = Self::new_dwallet_mpc_network_key(
            dkg_public_output,
            key_scheme,
            epoch_store.epoch(),
            access_structure,
        )?;

        let self_decryption_key_share = secret_key_share
            .into_iter()
            .map(|(party_id, secret_key_share)| {
                Ok((
                    party_id,
                    <AsyncProtocol as Protocol>::DecryptionKeyShare::new(
                        party_id,
                        secret_key_share,
                        &bcs::from_bytes(&new_key_version.decryption_key_share_public_parameters)?,
                    )
                    .map_err(|e| DwalletMPCError::ClassGroupsError(e.to_string()))?,
                ))
            })
            .collect::<DwalletMPCResult<HashMap<_, _>>>()?;

        inner
            .key_shares_versions
            .insert(key_scheme.clone(), vec![new_key_version.clone()]);

        inner
            .validator_decryption_key_share
            .insert(key_scheme.clone(), vec![self_decryption_key_share]);
        if let DwalletMPCNetworkKeysStatus::Ready(keys) = &mut inner.status {
            keys.insert(key_scheme);
            inner.status = DwalletMPCNetworkKeysStatus::Ready(keys.clone());
        } else {
            inner.status = DwalletMPCNetworkKeysStatus::Ready(HashSet::from([key_scheme]));
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
                    current_epoch_encryptions_of_shares_per_crt_prime: bcs::to_bytes(&dkg_output.encryptions_of_shares_per_crt_prime)?,
                    previous_epoch_encryptions_of_shares_per_crt_prime: vec![],
                    encryption_scheme_public_parameters: bcs::to_bytes(
                        &dkg_output.default_encryption_scheme_public_parameters::<
                            SECP256K1_SCALAR_LIMBS,
                            SECP256K1_FUNDAMENTAL_DISCRIMINANT_LIMBS,
                            secp256k1::GroupElement
                        >().map_err(|e| DwalletMPCError::ClassGroupsError(e.to_string()))?)?,
                    decryption_key_share_public_parameters: bcs::to_bytes(
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
                current_epoch_encryptions_of_shares_per_crt_prime: vec![],
                previous_epoch_encryptions_of_shares_per_crt_prime: vec![],
                encryption_scheme_public_parameters: vec![],
                decryption_key_share_public_parameters: vec![],
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
            .decryption_key_share_public_parameters
            .clone())
    }

    pub fn get_protocol_public_parameters(
        &self,
        key_scheme: DWalletMPCNetworkKeyScheme,
        key_version: u8,
    ) -> DwalletMPCResult<Vec<u8>> {
        let inner = self.inner.read().map_err(|_| DwalletMPCError::LockError)?;
        let encryption_scheme_public_parameters = bcs::from_bytes(
            &inner
                .key_shares_versions
                .get(&key_scheme)
                .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionKeyShares)?
                .get(key_version as usize)
                .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionKeyShares)?
                .encryption_scheme_public_parameters,
        )?;

        match key_scheme {
            DWalletMPCNetworkKeyScheme::Secp256k1 => {
                bcs::to_bytes(&ProtocolPublicParameters::new::<
                    { secp256k1::SCALAR_LIMBS },
                    { FUNDAMENTAL_DISCRIMINANT_LIMBS },
                    { NON_FUNDAMENTAL_DISCRIMINANT_LIMBS },
                    secp256k1::GroupElement,
                >(encryption_scheme_public_parameters))
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
    class_groups_decryption_key: ClassGroupsDecryptionKey,
    epoch_store: Arc<AuthorityPerEpochStore>,
) -> DwalletMPCResult<mpc::AsynchronousRoundResult<Vec<u8>, Vec<u8>, Vec<u8>>> {
    let res = match key_scheme {
        DWalletMPCNetworkKeyScheme::Secp256k1 => advance_and_serialize::<Secp256k1Party>(
            session_id,
            party_id,
            &weighted_threshold_access_structure,
            messages,
            bcs::from_bytes(public_input)?,
            class_groups_decryption_key,
        ),
        DWalletMPCNetworkKeyScheme::Ristretto => advance_and_serialize::<RistrettoParty>(
            session_id,
            party_id,
            &weighted_threshold_access_structure,
            messages,
            bcs::from_bytes(public_input)?,
            class_groups_decryption_key,
        ),
    }?;

    match &res {
        AsynchronousRoundResult::Finalize {
            malicious_parties: _,
            private_output,
            public_output,
        } => {
            // Update the network dWallet MPC keys with the new one.
            // Todo (#507): Save the output only after it has been verified first by quorum of validators.
            if let Some(network_key) = epoch_store.dwallet_mpc_network_keys.get() {
                network_key.add_key_version(
                    epoch_store.clone(),
                    key_scheme.clone(),
                    bcs::from_bytes(&private_output)?,
                    public_output.clone(),
                    weighted_threshold_access_structure,
                )?;
                return Ok(res);
            };
            Err(DwalletMPCError::DwalletMPCNetworkKeysNotFound)
        }
        _ => Ok(res),
    }
}
pub(super) fn network_dkg_public_input(
    deserialized_event: StartNetworkDKGEvent,
    encryption_keys_and_proofs: &HashMap<PartyID, ValidatorDataForNetworkDKG>,
) -> DwalletMPCResult<Vec<u8>> {
    match DWalletMPCNetworkKeyScheme::try_from(deserialized_event.key_scheme)? {
        DWalletMPCNetworkKeyScheme::Secp256k1 => {
            generate_secp256k1_dkg_party_public_input(encryption_keys_and_proofs)
        }
        DWalletMPCNetworkKeyScheme::Ristretto => {
            generate_ristretto_dkg_party_public_input(encryption_keys_and_proofs)
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
        mpc_round: MPCProtocolInitData::NetworkDkg(DWalletMPCNetworkKeyScheme::Secp256k1, None),
    }
}

fn dkg_ristretto_session_info(deserialized_event: StartNetworkDKGEvent) -> SessionInfo {
    SessionInfo {
        flow_session_id: deserialized_event.session_id.bytes,
        session_id: deserialized_event.session_id.bytes,
        initiating_user_address: Default::default(),
        mpc_round: MPCProtocolInitData::NetworkDkg(DWalletMPCNetworkKeyScheme::Ristretto, None),
    }
}

#[cfg(feature = "mock-class-groups")]
fn encryption_keys_and_proofs_from_validator_data(
    _: &HashMap<PartyID, ValidatorDataForNetworkDKG>,
) -> DwalletMPCResult<
    HashMap<
        PartyID,
        dwallet_classgroups_types::mock_class_groups::CGEncryptionKeyAndProofForMockFromFile,
    >,
> {
    dwallet_classgroups_types::mock_class_groups::mock_cg_encryption_keys_and_proofs()
}

#[cfg(not(feature = "mock-class-groups"))]
fn encryption_keys_and_proofs_from_validator_data(
    encryption_keys_and_proofs: &HashMap<PartyID, ValidatorDataForNetworkDKG>,
) -> DwalletMPCResult<HashMap<PartyID, ClassGroupsEncryptionKeyAndProof>> {
    encryption_keys_and_proofs
        .iter()
        .map(|(party_id, data)| {
            Ok((
                party_id.clone(),
                bcs::from_bytes(&data.cg_pubkey_and_proof)?,
            ))
        })
        .collect::<DwalletMPCResult<HashMap<_, _>>>()
}

fn generate_secp256k1_dkg_party_public_input(
    encryption_keys_and_proofs: &HashMap<PartyID, ValidatorDataForNetworkDKG>,
) -> DwalletMPCResult<Vec<u8>> {
    let encryption_keys_and_proofs =
        encryption_keys_and_proofs_from_validator_data(encryption_keys_and_proofs)?;

    let public_params = Secp256k1PublicInput::new::<secp256k1::GroupElement>(
        secp256k1::scalar::PublicParameters::default(),
        DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER,
        encryption_keys_and_proofs,
    )
    .map_err(|e| DwalletMPCError::InvalidMPCPartyType)?;
    bcs::to_bytes(&public_params).map_err(|e| DwalletMPCError::BcsError(e))
}

fn generate_ristretto_dkg_party_public_input(
    encryption_keys_and_proofs: &HashMap<PartyID, ValidatorDataForNetworkDKG>,
) -> DwalletMPCResult<Vec<u8>> {
    let encryption_keys_and_proofs =
        encryption_keys_and_proofs_from_validator_data(encryption_keys_and_proofs)?;

    let public_params = RistrettoPublicInput::new::<ristretto::GroupElement>(
        ristretto::scalar::PublicParameters::default(),
        DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER,
        encryption_keys_and_proofs,
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
            let encryption_scheme_public_parameters =
                public_output.default_encryption_scheme_public_parameters::<
                    SECP256K1_SCALAR_LIMBS,
                    SECP256K1_FUNDAMENTAL_DISCRIMINANT_LIMBS,
                    secp256k1::GroupElement,
                >().map_err(|e| DwalletMPCError::ClassGroupsError(e.to_string()))?;
            let decryption_key_share_public_parameters = public_output.default_decryption_key_share_public_parameters::<
                SECP256K1_SCALAR_LIMBS,
                SECP256K1_FUNDAMENTAL_DISCRIMINANT_LIMBS,
                secp256k1::GroupElement,
            >(weighted_threshold_access_structure).map_err(|e| DwalletMPCError::ClassGroupsError(e.to_string()))?;

            Ok(NetworkDecryptionKeyShares {
                epoch,
                current_epoch_encryptions_of_shares_per_crt_prime: bcs::to_bytes(
                    &public_output.encryptions_of_shares_per_crt_prime,
                )?,
                previous_epoch_encryptions_of_shares_per_crt_prime: vec![],
                encryption_scheme_public_parameters: bcs::to_bytes(
                    &encryption_scheme_public_parameters,
                )?,
                decryption_key_share_public_parameters: bcs::to_bytes(
                    &decryption_key_share_public_parameters,
                )?,
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
        current_epoch_encryptions_of_shares_per_crt_prime: &Vec<u8>,
    ) -> DwalletMPCResult<Self::PublicOutput> {
        let dkg_public_output = Self::PublicOutput {
            encryption_key: bcs::from_bytes(encryption_key)?,
            reconstructed_commitments_to_sharing: bcs::from_bytes(
                reconstructed_commitments_to_sharing,
            )?,
            encryptions_of_shares_per_crt_prime: bcs::from_bytes(
                current_epoch_encryptions_of_shares_per_crt_prime,
            )?,
        };

        Ok(dkg_public_output)
    }
}
