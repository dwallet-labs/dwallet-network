//! This module contains the network DKG protocol for the dWallet MPC sessions.
//! The network DKG protocol handles generating the network Decryption-Key shares.
//! The module provides the management of the network Decryption-Key shares and
//! the network DKG protocol.
//! It provides inner mutability for the [`EpochStore`]
//! to update the network decryption key shares synchronously.
use crate::dwallet_mpc::advance_and_serialize;
use crate::dwallet_mpc::mpc_session::AsyncProtocol;
use crate::dwallet_mpc::reshare::ReshareSecp256k1Party;
use class_groups::dkg::{
    RistrettoParty, RistrettoPublicInput, Secp256k1Party, Secp256k1PublicInput,
};
use class_groups::{
    DecryptionKeyShare, Secp256k1DecryptionKeySharePublicParameters, SecretKeyShareSizedInteger,
    DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER, SECRET_KEY_SHARE_LIMBS,
};
use commitment::CommitmentSizedNumber;
use dwallet_classgroups_types::{ClassGroupsDecryptionKey, ClassGroupsEncryptionKeyAndProof};
use dwallet_mpc_types::dwallet_mpc::{
    DWalletMPCNetworkKeyScheme, NetworkDecryptionKeyPublicData,
    NetworkDecryptionKeyPublicOutputType,
};
use group::{ristretto, secp256k1, GroupElement, PartyID};
use homomorphic_encryption::AdditivelyHomomorphicDecryptionKeyShare;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_dwallet_mpc::{
    DWalletMPCSuiEvent, DWalletNetworkDecryptionKeyData, DWalletNetworkDecryptionKeyState,
    MPCProtocolInitData, SessionInfo, StartNetworkDKGEvent,
};
use mpc::secret_sharing::shamir::over_the_integers::PrecomputedValues;
use mpc::{AsynchronousRoundResult, WeightedThresholdAccessStructure};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use sui_types::base_types::ObjectID;
use tokio::sync::RwLock;
use tracing::{info, warn};
use twopc_mpc::secp256k1::class_groups::{
    FUNDAMENTAL_DISCRIMINANT_LIMBS, NON_FUNDAMENTAL_DISCRIMINANT_LIMBS,
};
use twopc_mpc::sign::Protocol;
use twopc_mpc::ProtocolPublicParameters;

/// Holds the network (decryption) keys of the network MPC protocols.
pub struct DwalletMPCNetworkKeys {
    /// Holds all network (decryption) keys for the current network in encrypted form.
    /// This data is identical for all the Validator nodes.
    /// Limits the access to the network keys to a single thread at a time.
    inner: Arc<RwLock<DwalletMPCNetworkKeyInner>>,

    ///
    validator_private_dec_key_data: ValidatorPrivateDecryptionKeyData,
}

/// Holds the private decryption key data for a validator node.
pub struct ValidatorPrivateDecryptionKeyData {
    /// The unique party ID of the validator, representing its index within the committee.
    pub party_id: PartyID,

    /// The validator's class groups decryption key.
    pub class_groups_decryption_key: ClassGroupsDecryptionKey,

    /// A thread-safe map of the validator's decryption key shares.
    ///
    /// This structure maps each key ID (`ObjectID`) to a sub-map of `PartyID`
    /// to the corresponding decryption key share.
    /// These shares are used in multi-party cryptographic protocols.
    /// NOTE: EACH PARTY IN HERE IS A **VIRTUAL PARTY**.
    /// NOTE 2: `ObjectID` is the ID of the network decryption key, not the party.
    pub validator_decryption_key_shares: RwLock<
        HashMap<ObjectID, HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>>,
    >,
}

fn get_decryption_key_shares_from_public_output(
    shares: &NetworkDecryptionKeyPublicData,
    party_id: PartyID,
    decryption_key: ClassGroupsDecryptionKey,
    weighted_threshold_access_structure: &WeightedThresholdAccessStructure,
) -> DwalletMPCResult<HashMap<PartyID, SecretKeyShareSizedInteger>> {
    match shares.state {
        NetworkDecryptionKeyPublicOutputType::NetworkDkg => {
            #[cfg(not(feature = "with-network-dkg"))]
            {
                let secret_shares = shared_wasm_class_groups::decryption_key_shares(party_id);
                return Ok(secret_shares);
            }

            #[cfg(feature = "with-network-dkg")]
            {
                let dkg_public_output: <Secp256k1Party as mpc::Party>::PublicOutput =
                    bcs::from_bytes(&shares.public_output)?;

                let secret_shares = dkg_public_output
                    .default_decryption_key_shares::<secp256k1::GroupElement>(
                        party_id,
                        weighted_threshold_access_structure,
                        decryption_key,
                    )
                    .map_err(|err| DwalletMPCError::ClassGroupsError(err.to_string()))?;
                Ok(secret_shares)
            }
        }
        NetworkDecryptionKeyPublicOutputType::Reshare => {
            let public_output: <ReshareSecp256k1Party as mpc::Party>::PublicOutput =
                bcs::from_bytes(&shares.public_output)?;

            let secret_shares = public_output
                .decrypt_decryption_key_shares::<secp256k1::GroupElement>(
                    party_id,
                    weighted_threshold_access_structure,
                    decryption_key,
                )
                .map_err(|err| DwalletMPCError::ClassGroupsError(err.to_string()))?;
            Ok(secret_shares)
        }
    }
}

impl ValidatorPrivateDecryptionKeyData {
    /// Stores the new decryption key shares of the validator.
    /// Decrypts the decryption key shares (for all the virtual parties)
    /// from the public output of the network DKG protocol.
    pub async fn store_decryption_secret_shares(
        &self,
        key_id: ObjectID,
        key: NetworkDecryptionKeyPublicData,
        weighted_threshold_access_structure: &WeightedThresholdAccessStructure,
    ) -> DwalletMPCResult<()> {
        let secret_key_shares = get_decryption_key_shares_from_public_output(
            &key,
            self.party_id,
            self.class_groups_decryption_key,
            weighted_threshold_access_structure,
        )?;

        let self_decryption_key_shares = Self::convert_secret_key_shares_type_to_decryption_shares(
            secret_key_shares,
            &key.decryption_key_share_public_parameters,
        )?;

        let mut inner = self.validator_decryption_key_shares.write().await;
        inner.insert(key_id, self_decryption_key_shares);
        Ok(())
    }

    /// Only for type convertion.
    fn convert_secret_key_shares_type_to_decryption_shares(
        secret_shares: HashMap<PartyID, SecretKeyShareSizedInteger>,
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
}

/// Encapsulates all the fields in a single structure for atomic access.
pub struct DwalletMPCNetworkKeyInner {
    /// The encrypted network (decryption) key shares used by the dWallet MPC network.
    ///
    /// This map holds the encrypted shares of decryption keys,
    /// indexed by a unique key ID (`ObjectID`).
    /// Each entry contains the set of shares
    /// needed to collaboratively decrypt data without any single party
    /// holding the full key.
    ///
    /// These shares are part of a threshold decryption scheme, where multiple parties contribute their share
    /// to reconstruct the original key securely.
    pub network_decryption_keys: HashMap<ObjectID, NetworkDecryptionKeyPublicData>,
}

impl DwalletMPCNetworkKeys {
    /// Creates a new instance of the network decryption key shares.
    pub fn new(node_context: ValidatorPrivateDecryptionKeyData) -> Self {
        Self {
            inner: Arc::new(RwLock::new(DwalletMPCNetworkKeyInner {
                network_decryption_keys: HashMap::new(),
            })),
            validator_private_dec_key_data: node_context,
        }
    }

    pub async fn network_decryption_keys(
        &self,
    ) -> HashMap<ObjectID, NetworkDecryptionKeyPublicData> {
        self.inner.read().await.network_decryption_keys.clone()
    }

    pub async fn validator_decryption_keys_shares(
        &self,
    ) -> HashMap<ObjectID, HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>> {
        self.validator_private_dec_key_data
            .validator_decryption_key_shares
            .read()
            .await
            .clone()
    }

    /// Adds a new network key to the network decryption keys.
    pub async fn add_new_network_key(
        &self,
        key_id: ObjectID,
        key: NetworkDecryptionKeyPublicData,
        weighted_threshold_access_structure: &WeightedThresholdAccessStructure,
    ) -> DwalletMPCResult<()> {
        let mut inner = self.inner.write().await;
        inner.network_decryption_keys.insert(key_id, key.clone());
        self.validator_private_dec_key_data
            .store_decryption_secret_shares(key_id, key, weighted_threshold_access_structure)
            .await?;
        Ok(())
    }

    pub async fn update_network_key(
        &self,
        key_id: ObjectID,
        key: NetworkDecryptionKeyPublicData,
        weighted_threshold_access_structure: &WeightedThresholdAccessStructure,
    ) -> DwalletMPCResult<()> {
        let mut inner = self.inner.write().await;
        inner.network_decryption_keys.insert(key_id, key.clone());
        self.validator_private_dec_key_data
            .store_decryption_secret_shares(key_id, key, weighted_threshold_access_structure)
            .await?;
        Ok(())
    }

    /// Returns all the decryption key shares for any specified key ID.
    pub async fn get_decryption_key_share(
        &self,
        key_id: ObjectID,
    ) -> DwalletMPCResult<HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>> {
        Ok(self
            .validator_decryption_keys_shares()
            .await
            .get(&key_id)
            .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionKeyShares)?
            .clone())
    }

    pub async fn get_decryption_public_parameters(
        &self,
        key_id: &ObjectID,
    ) -> DwalletMPCResult<Vec<u8>> {
        Ok(self
            .inner
            .read()
            .await
            .network_decryption_keys
            .get(key_id)
            .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionKeyShares)?
            .decryption_key_share_public_parameters
            .clone())
    }

    async fn try_get_decryption_keys(
        &self,
        key_id: &ObjectID,
    ) -> DwalletMPCResult<Option<NetworkDecryptionKeyPublicData>> {
        let inner = self.inner.read().await;
        Ok(inner
            .network_decryption_keys
            .get(&key_id)
            .map(|v| v.clone()))
    }

    /// Retrieves the protocol public parameters for the specified key ID.
    /// This function assumes the given key_id is a valid key ID, and retries getting it until it has been synced from
    /// the Sui network.
    pub async fn get_protocol_public_parameters(
        &self,
        key_id: &ObjectID,
        key_scheme: DWalletMPCNetworkKeyScheme,
    ) -> DwalletMPCResult<Vec<u8>> {
        loop {
            let Ok(Some(result)) = self.try_get_decryption_keys(key_id).await else {
                warn!("failed to fetch the network decryption key shares for key ID: {:?}, trying again", key_id);
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                continue;
            };
            let decryption_key_share_public_parameters =
                bcs::from_bytes::<Secp256k1DecryptionKeySharePublicParameters>(
                    &result.decryption_key_share_public_parameters,
                )?;

            match key_scheme {
                DWalletMPCNetworkKeyScheme::Secp256k1 => {
                    return bcs::to_bytes(&ProtocolPublicParameters::new::<
                        { secp256k1::SCALAR_LIMBS },
                        { FUNDAMENTAL_DISCRIMINANT_LIMBS },
                        { NON_FUNDAMENTAL_DISCRIMINANT_LIMBS },
                        secp256k1::GroupElement,
                    >(
                        decryption_key_share_public_parameters.encryption_scheme_public_parameters,
                    ))
                    .map_err(|e| DwalletMPCError::BcsError(e))
                }
                DWalletMPCNetworkKeyScheme::Ristretto => {
                    todo!()
                }
            }
        }
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
) -> DwalletMPCResult<AsynchronousRoundResult<Vec<u8>, Vec<u8>, Vec<u8>>> {
    // todo(zeev): move the mock to a func.
    #[cfg(not(feature = "with-network-dkg"))]
    {
        let secret_shares = shared_wasm_class_groups::decryption_key_shares(party_id);

        let self_decryption_key_share = secret_shares
            .into_iter()
            .map(|(party_id, secret_key_share)| Ok((party_id, secret_key_share)))
            .collect::<DwalletMPCResult<HashMap<_, _>>>()?;

        let private_output = bcs::to_bytes(&self_decryption_key_share)?;
        let public_output = bcs::to_bytes(&shared_wasm_class_groups::network_dkg_final_output())?;

        let res = AsynchronousRoundResult::Finalize {
            malicious_parties: Vec::new(),
            private_output: private_output.clone(),
            public_output: public_output.clone(),
        };

        return Ok(res);
    }

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
    Ok(res)
}

pub(super) fn network_dkg_public_input(
    encryption_keys_and_proofs: HashMap<PartyID, ClassGroupsEncryptionKeyAndProof>,
    key_scheme: DWalletMPCNetworkKeyScheme,
) -> DwalletMPCResult<Vec<u8>> {
    match key_scheme {
        DWalletMPCNetworkKeyScheme::Secp256k1 => {
            generate_secp256k1_dkg_party_public_input(encryption_keys_and_proofs)
        }
        DWalletMPCNetworkKeyScheme::Ristretto => {
            generate_ristretto_dkg_party_public_input(encryption_keys_and_proofs)
        }
    }
}

pub(super) fn network_dkg_session_info(
    deserialized_event: DWalletMPCSuiEvent<StartNetworkDKGEvent>,
    key_scheme: DWalletMPCNetworkKeyScheme,
) -> DwalletMPCResult<SessionInfo> {
    match key_scheme {
        DWalletMPCNetworkKeyScheme::Secp256k1 => {
            Ok(network_dkg_secp256k1_session_info(deserialized_event))
        }
        DWalletMPCNetworkKeyScheme::Ristretto => {
            Ok(network_dkg_ristretto_session_info(deserialized_event))
        }
    }
}

fn network_dkg_secp256k1_session_info(
    deserialized_event: DWalletMPCSuiEvent<StartNetworkDKGEvent>,
) -> SessionInfo {
    SessionInfo {
        sequence_number: deserialized_event.session_sequence_number,
        session_id: deserialized_event.session_id,
        mpc_round: MPCProtocolInitData::NetworkDkg(
            DWalletMPCNetworkKeyScheme::Secp256k1,
            deserialized_event,
        ),
        is_immediate: true,
    }
}

fn network_dkg_ristretto_session_info(
    deserialized_event: DWalletMPCSuiEvent<StartNetworkDKGEvent>,
) -> SessionInfo {
    SessionInfo {
        sequence_number: deserialized_event.session_sequence_number,
        session_id: deserialized_event.session_id,
        mpc_round: MPCProtocolInitData::NetworkDkg(
            DWalletMPCNetworkKeyScheme::Ristretto,
            deserialized_event,
        ),
        is_immediate: true,
    }
}

fn generate_secp256k1_dkg_party_public_input(
    encryption_keys_and_proofs: HashMap<PartyID, ClassGroupsEncryptionKeyAndProof>,
) -> DwalletMPCResult<Vec<u8>> {
    let public_params = Secp256k1PublicInput::new::<secp256k1::GroupElement>(
        secp256k1::scalar::PublicParameters::default(),
        DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER,
        encryption_keys_and_proofs,
    )
    .map_err(|e| DwalletMPCError::InvalidMPCPartyType)?;
    bcs::to_bytes(&public_params).map_err(|e| DwalletMPCError::BcsError(e))
}

fn generate_ristretto_dkg_party_public_input(
    encryption_keys_and_proofs: HashMap<PartyID, ClassGroupsEncryptionKeyAndProof>,
) -> DwalletMPCResult<Vec<u8>> {
    let public_params = RistrettoPublicInput::new::<ristretto::GroupElement>(
        ristretto::scalar::PublicParameters::default(),
        DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER,
        encryption_keys_and_proofs,
    )
    .map_err(|e| DwalletMPCError::InvalidMPCPartyType)?;
    bcs::to_bytes(&public_params).map_err(|e| DwalletMPCError::BcsError(e))
}

pub(crate) fn instantiate_dwallet_mpc_network_decryption_key_shares_from_public_output(
    epoch: u64,
    key_scheme: DWalletMPCNetworkKeyScheme,
    weighted_threshold_access_structure: &WeightedThresholdAccessStructure,
    key_data: DWalletNetworkDecryptionKeyData,
) -> DwalletMPCResult<NetworkDecryptionKeyPublicData> {
    if (key_data.current_reconfiguration_public_output.is_empty()) {
        instantiate_dwallet_mpc_network_decryption_key_shares_from_dkg_public_output(
            epoch,
            key_scheme,
            weighted_threshold_access_structure,
            &key_data.network_dkg_public_output,
        )
    } else {
        instantiate_dwallet_mpc_network_decryption_key_shares_from_reshare_public_output(
            epoch,
            weighted_threshold_access_structure,
            &key_data.current_reconfiguration_public_output,
        )
    }
}

fn instantiate_dwallet_mpc_network_decryption_key_shares_from_reshare_public_output(
    epoch: u64,
    weighted_threshold_access_structure: &WeightedThresholdAccessStructure,
    public_output_bytes: &[u8],
) -> DwalletMPCResult<NetworkDecryptionKeyPublicData> {
    let public_output: <ReshareSecp256k1Party as mpc::Party>::PublicOutput =
        bcs::from_bytes(&public_output_bytes)?;
    let decryption_key_share_public_parameters = public_output
        .default_decryption_key_share_public_parameters::<secp256k1::GroupElement>(
            weighted_threshold_access_structure,
        )
        .map_err(|e| DwalletMPCError::ClassGroupsError(e.to_string()))?;

    Ok(NetworkDecryptionKeyPublicData {
        epoch,
        state: NetworkDecryptionKeyPublicOutputType::Reshare,
        public_output: public_output_bytes.to_vec(),
        decryption_key_share_public_parameters: bcs::to_bytes(
            &decryption_key_share_public_parameters,
        )?,
    })
}

fn instantiate_dwallet_mpc_network_decryption_key_shares_from_dkg_public_output(
    epoch: u64,
    key_scheme: DWalletMPCNetworkKeyScheme,
    weighted_threshold_access_structure: &WeightedThresholdAccessStructure,
    public_output_bytes: &[u8],
) -> DwalletMPCResult<NetworkDecryptionKeyPublicData> {
    match key_scheme {
        DWalletMPCNetworkKeyScheme::Secp256k1 => {
            let public_output: <Secp256k1Party as mpc::Party>::PublicOutput =
                bcs::from_bytes(&public_output_bytes)?;
            let decryption_key_share_public_parameters = public_output
                .default_decryption_key_share_public_parameters::<secp256k1::GroupElement>(
                    weighted_threshold_access_structure,
                )
                .map_err(|e| DwalletMPCError::ClassGroupsError(e.to_string()))?;
            Ok(NetworkDecryptionKeyPublicData {
                epoch,
                state: NetworkDecryptionKeyPublicOutputType::NetworkDkg,
                public_output: public_output_bytes.to_vec(),
                decryption_key_share_public_parameters: bcs::to_bytes(
                    &decryption_key_share_public_parameters,
                )?,
            })
        }
        DWalletMPCNetworkKeyScheme::Ristretto => todo!("Ristretto key scheme"),
    }
}
