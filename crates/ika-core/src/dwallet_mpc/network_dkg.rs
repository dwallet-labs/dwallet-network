//! This module contains the network DKG protocol for the dWallet MPC sessions.
//! The network DKG protocol handles generating the network Decryption-Key shares.
//! The module provides the management of the network Decryption-Key shares and
//! the network DKG protocol.
//! It provides inner mutability for the [`EpochStore`]
//! to update the network decryption key shares synchronously.
use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::dwallet_mpc::advance_and_serialize;
use crate::dwallet_mpc::mpc_session::AsyncProtocol;
use class_groups::dkg::{
    RistrettoParty, RistrettoPublicInput, Secp256k1Party, Secp256k1PublicInput,
};
use class_groups::{
    DecryptionKeyShare, SecretKeyShareSizedInteger, DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER,
};
use commitment::CommitmentSizedNumber;
use dwallet_classgroups_types::{ClassGroupsDecryptionKey, ClassGroupsEncryptionKeyAndProof};
use dwallet_mpc_types::dwallet_mpc::{DWalletMPCNetworkKeyScheme, NetworkDecryptionKeyShares};
use group::{ristretto, secp256k1, PartyID};
use homomorphic_encryption::AdditivelyHomomorphicDecryptionKeyShare;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_dwallet_mpc::{
    DWalletMPCSuiEvent, MPCProtocolInitData, SessionInfo, StartNetworkDKGEvent,
};
use mpc::{AsynchronousRoundResult, WeightedThresholdAccessStructure};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use sui_types::base_types::ObjectID;
use tracing::log::info;
use twopc_mpc::secp256k1::class_groups::{
    FUNDAMENTAL_DISCRIMINANT_LIMBS, NON_FUNDAMENTAL_DISCRIMINANT_LIMBS,
};
use twopc_mpc::sign::Protocol;
use twopc_mpc::ProtocolPublicParameters;

/// The status of the network supported key types for the dWallet MPC sessions.
#[derive(Clone, Debug, PartialEq)]
pub enum DwalletMPCNetworkKeysStatus {
    /// The network supported key types have been updated or initialized.
    Ready(HashSet<ObjectID>),
    /// None of the network supported key types have been initialized.
    NotInitialized,
}

/// Holds the network keys of the dWallet MPC protocols.
pub struct DwalletMPCNetworkKeyVersions {
    inner: Arc<RwLock<DwalletMPCNetworkKeyVersionsInner>>,
    node_context: NodeContext,
}

pub enum NodeContext {
    Validator(ValidatorContext),
    FullNode,
}

pub struct ValidatorContext {
    pub party_id: PartyID,
    pub class_groups_decryption_key: ClassGroupsDecryptionKey,
    pub validator_decryption_key_share: RwLock<
        HashMap<ObjectID, HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>>,
    >,
}

pub trait NetworkDecryptionKey {
    fn get_decryption_key_shares_from_public_output(
        &self,
        party_id: PartyID,
        decryption_key: ClassGroupsDecryptionKey,
    ) -> DwalletMPCResult<HashMap<PartyID, SecretKeyShareSizedInteger>>;
}

impl NetworkDecryptionKey for NetworkDecryptionKeyShares {
    fn get_decryption_key_shares_from_public_output(
        &self,
        party_id: PartyID,
        decryption_key: ClassGroupsDecryptionKey,
    ) -> DwalletMPCResult<HashMap<PartyID, SecretKeyShareSizedInteger>> {
        let dkg_public_output = <Secp256k1Party as CreatePublicOutput>::new(
            &self.encryption_key,
            &self.public_verification_keys,
            &self.current_epoch_encryptions_of_shares_per_crt_prime,
        )?;

        let secret_share = dkg_public_output
            .default_decryption_key_shares::<secp256k1::GroupElement>(party_id, decryption_key)
            .map_err(|err| {
                println!("Error: {:?}", err);
                DwalletMPCError::ClassGroupsError(err.to_string())
            })?;
        Ok(secret_share)
    }
}

impl ValidatorContext {
    pub fn add_decryption_secret_shares(
        &self,
        key_id: ObjectID,
        key: NetworkDecryptionKeyShares,
    ) -> DwalletMPCResult<()> {
        let secret_key_share = key.get_decryption_key_shares_from_public_output(
            self.party_id,
            self.class_groups_decryption_key,
        )?;

        let self_decryption_key_share = Self::convert_secret_key_shares_to_decryption_shares(
            secret_key_share,
            &key.decryption_key_share_public_parameters,
        )?;

        let mut inner = self
            .validator_decryption_key_share
            .write()
            .map_err(|_| DwalletMPCError::LockError)?;
        inner.insert(key_id, self_decryption_key_share);
        Ok(())
    }

    pub fn update_decryption_secret_shares(
        &self,
        key_id: ObjectID,
        key: NetworkDecryptionKeyShares,
    ) -> DwalletMPCResult<()> {
        let secret_key_share = key.get_decryption_key_shares_from_public_output(
            self.party_id,
            self.class_groups_decryption_key,
        )?;

        let self_decryption_key_share = Self::convert_secret_key_shares_to_decryption_shares(
            secret_key_share,
            &key.decryption_key_share_public_parameters,
        )?;

        let mut inner = self
            .validator_decryption_key_share
            .write()
            .map_err(|_| DwalletMPCError::LockError)?;
        inner.insert(key_id, self_decryption_key_share);
        Ok(())
    }

    fn convert_secret_key_shares_to_decryption_shares(
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
pub struct DwalletMPCNetworkKeyVersionsInner {
    /// The dWallet MPC network decryption key shares (encrypted).
    /// Map from key type to the encryption of the share of the key of that type.
    pub network_decryption_keys: HashMap<ObjectID, NetworkDecryptionKeyShares>,
    /// The status of the network supported key types for the dWallet MPC sessions.
    pub status: DwalletMPCNetworkKeysStatus, // Todo : remove
}

impl DwalletMPCNetworkKeyVersions {
    /// Creates a new instance of the network encryption key shares.
    pub fn new(node_context: NodeContext) -> Self {
        Self {
            inner: Arc::new(RwLock::new(DwalletMPCNetworkKeyVersionsInner {
                network_decryption_keys: HashMap::new(),
                status: DwalletMPCNetworkKeysStatus::NotInitialized,
            })),
            node_context,
        }
    }

    pub fn network_decryption_keys(&self) -> HashMap<ObjectID, NetworkDecryptionKeyShares> {
        self.inner
            .read()
            .map(|inner| inner.network_decryption_keys.clone())
            .unwrap_or_default()
    }

    pub fn validator_decryption_key_share(
        &self,
    ) -> DwalletMPCResult<
        HashMap<ObjectID, HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>>,
    > {
        Ok(match &self.node_context {
            NodeContext::Validator(validator_context) => validator_context
                .validator_decryption_key_share
                .read()
                .map_err(|_| DwalletMPCError::LockError)?
                .clone(),
            NodeContext::FullNode => {
                info!("Full node context does not have decryption secret shares");
                HashMap::new()
            }
        })
    }

    pub fn add_new_network_key(
        &self,
        key_id: ObjectID,
        key: NetworkDecryptionKeyShares,
    ) -> DwalletMPCResult<()> {
        let mut inner = self.inner.write().map_err(|_| DwalletMPCError::LockError)?;
        inner.network_decryption_keys.insert(key_id, key.clone());
        match &self.node_context {
            NodeContext::Validator(validator_context) => {
                validator_context.add_decryption_secret_shares(key_id, key)?;
            }
            NodeContext::FullNode => {}
        }
        if let DwalletMPCNetworkKeysStatus::Ready(keys) = &mut inner.status {
            keys.insert(key_id);
            inner.status = DwalletMPCNetworkKeysStatus::Ready(keys.clone());
        } else {
            inner.status = DwalletMPCNetworkKeysStatus::Ready(HashSet::from([key_id]));
        }
        Ok(())
    }

    pub fn update_network_key(
        &self,
        key_id: ObjectID,
        key: NetworkDecryptionKeyShares,
    ) -> DwalletMPCResult<()> {
        let mut inner = self.inner.write().map_err(|_| DwalletMPCError::LockError)?;
        inner.network_decryption_keys.insert(key_id, key.clone());
        match &self.node_context {
            NodeContext::Validator(validator_context) => {
                validator_context.update_decryption_secret_shares(key_id, key)?;
            }
            NodeContext::FullNode => {}
        }
        Ok(())
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
                            secp256k1::GroupElement
                        >().map_err(|e| DwalletMPCError::ClassGroupsError(e.to_string()))?)?,
                    decryption_key_share_public_parameters: bcs::to_bytes(
                        &dkg_output.default_decryption_key_share_public_parameters::<
                            secp256k1::GroupElement
                        >(access_structure).map_err(|e|DwalletMPCError::ClassGroupsError(e.to_string()))?)?,
                    encryption_key: bcs::to_bytes(&dkg_output.encryption_key)?,
                    public_verification_keys: bcs::to_bytes(&dkg_output.public_verification_keys)?,
                })
            }
            DWalletMPCNetworkKeyScheme::Ristretto => Ok(NetworkDecryptionKeyShares {
                epoch,
                current_epoch_encryptions_of_shares_per_crt_prime: vec![],
                previous_epoch_encryptions_of_shares_per_crt_prime: vec![],
                encryption_scheme_public_parameters: vec![],
                decryption_key_share_public_parameters: vec![],
                encryption_key: vec![],
                public_verification_keys: vec![],
            }),
        }
    }

    /// Returns all versions of the decryption key shares for the specified key type.
    // Todo (#382): Replace with the actual type once the DKG protocol is ready.
    pub fn get_decryption_key_share(
        &self,
        key_id: ObjectID,
    ) -> DwalletMPCResult<HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>> {
        let decryption_secret_shares = self.validator_decryption_key_share()?;
        Ok(decryption_secret_shares
            .get(&key_id)
            .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionKeyShares)?
            .clone())
    }

    pub fn get_decryption_public_parameters(&self, key_id: &ObjectID) -> DwalletMPCResult<Vec<u8>> {
        let inner = self.inner.read().map_err(|_| DwalletMPCError::LockError)?;
        Ok(inner
            .network_decryption_keys
            .get(key_id)
            .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionKeyShares)?
            .decryption_key_share_public_parameters
            .clone())
    }

    pub fn get_protocol_public_parameters(
        &self,
        key_id: &ObjectID,
        key_scheme: DWalletMPCNetworkKeyScheme,
    ) -> DwalletMPCResult<Vec<u8>> {
        let inner = self.inner.read().map_err(|_| DwalletMPCError::LockError)?;
        let encryption_scheme_public_parameters = bcs::from_bytes(
            &inner
                .network_decryption_keys
                .get(&key_id)
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
    key_id: ObjectID,
    session_id: CommitmentSizedNumber,
    weighted_threshold_access_structure: &WeightedThresholdAccessStructure,
    party_id: PartyID,
    public_input: &[u8],
    key_scheme: &DWalletMPCNetworkKeyScheme,
    messages: Vec<HashMap<PartyID, Vec<u8>>>,
    class_groups_decryption_key: ClassGroupsDecryptionKey,
    epoch_store: Arc<AuthorityPerEpochStore>,
) -> DwalletMPCResult<mpc::AsynchronousRoundResult<Vec<u8>, Vec<u8>, Vec<u8>>> {
    #[cfg(not(feature = "with-network-dkg"))]
    {
        let secret_shares = shared_wasm_class_groups::decryption_key_share(party_id);

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

        // if let Some(network_key) = epoch_store.dwallet_mpc_network_keys.get() {
        //     network_key.add_key_version(
        //         key_id,
        //         epoch_store.clone(),
        //         key_scheme.clone(),
        //         self_decryption_key_share,
        //         public_output,
        //         weighted_threshold_access_structure,
        //     )?;
        //     return Ok(res);
        // }
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
    return Ok(res);

    // match &res {
    //     AsynchronousRoundResult::Finalize {
    //         malicious_parties: _,
    //         private_output,
    //         public_output,
    //     } => {
    //         // Update the network dWallet MPC keys with the new one.
    //         // Todo (#507): Save the output only after it has been verified first by quorum of validators.
    //         if let Some(network_key) = epoch_store.dwallet_mpc_network_keys.get() {
    //             network_key.add_key_version(
    //                 key_id,
    //                 epoch_store.clone(),
    //                 key_scheme.clone(),
    //                 bcs::from_bytes(&private_output)?,
    //                 public_output.clone(),
    //                 weighted_threshold_access_structure,
    //             )?;
    //             return Ok(res);
    //         };
    //         Err(DwalletMPCError::DwalletMPCNetworkKeysNotFound)
    //     }
    //     _ => Ok(res),
    // }
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
        session_id: deserialized_event.session_id,
        mpc_round: MPCProtocolInitData::NetworkDkg(
            DWalletMPCNetworkKeyScheme::Secp256k1,
            deserialized_event.event_data,
        ),
    }
}

fn network_dkg_ristretto_session_info(
    deserialized_event: DWalletMPCSuiEvent<StartNetworkDKGEvent>,
) -> SessionInfo {
    SessionInfo {
        session_id: deserialized_event.session_id,
        mpc_round: MPCProtocolInitData::NetworkDkg(
            DWalletMPCNetworkKeyScheme::Ristretto,
            deserialized_event.event_data,
        ),
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
            let encryption_scheme_public_parameters = public_output
                .default_encryption_scheme_public_parameters::<secp256k1::GroupElement>()
                .map_err(|e| DwalletMPCError::ClassGroupsError(e.to_string()))?;
            let decryption_key_share_public_parameters = public_output
                .default_decryption_key_share_public_parameters::<secp256k1::GroupElement>(
                    weighted_threshold_access_structure,
                )
                .map_err(|e| DwalletMPCError::ClassGroupsError(e.to_string()))?;

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
                public_verification_keys: bcs::to_bytes(&public_output.public_verification_keys)?,
            })
        }
        DWalletMPCNetworkKeyScheme::Ristretto => todo!("Ristretto key scheme"),
    }
}

pub trait CreatePublicOutput: mpc::Party {
    fn new(
        encryption_key: &Vec<u8>,
        public_verification_keys: &Vec<u8>,
        current_epoch_shares: &Vec<u8>,
    ) -> DwalletMPCResult<Self::PublicOutput>;
}

impl CreatePublicOutput for Secp256k1Party {
    fn new(
        encryption_key: &Vec<u8>,
        public_verification_keys: &Vec<u8>,
        current_epoch_encryptions_of_shares_per_crt_prime: &Vec<u8>,
    ) -> DwalletMPCResult<Self::PublicOutput> {
        let dkg_public_output = Self::PublicOutput {
            encryption_key: bcs::from_bytes(encryption_key)?,
            public_verification_keys: bcs::from_bytes(public_verification_keys)?,
            encryptions_of_shares_per_crt_prime: bcs::from_bytes(
                current_epoch_encryptions_of_shares_per_crt_prime,
            )?,
        };

        Ok(dkg_public_output)
    }
}
