//! This module contains the network DKG protocol for the dWallet MPC sessions.
//! The network DKG protocol handles generating the network Decryption-Key shares.
//! The module provides the management of the network Decryption-Key shares and
//! the network DKG protocol.
use crate::dwallet_mpc::advance_and_serialize;
use crate::dwallet_mpc::mpc_session::AsyncProtocol;
use crate::dwallet_mpc::reshare::ReshareSecp256k1Party;
use class_groups::dkg::{
    RistrettoParty, RistrettoPublicInput, Secp256k1Party, Secp256k1PublicInput,
};
use class_groups::{
    Secp256k1DecryptionKeySharePublicParameters, SecretKeyShareSizedInteger,
    DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER,
};
use commitment::CommitmentSizedNumber;
use dwallet_classgroups_types::{ClassGroupsDecryptionKey, ClassGroupsEncryptionKeyAndProof};
use dwallet_mpc_types::dwallet_mpc::{
    DWalletMPCNetworkKeyScheme, MPCMessage, MPCPrivateOutput, NetworkDecryptionKeyPublicData,
    NetworkDecryptionKeyPublicOutputType, SerializedWrappedMPCPublicOutput,
    VersionedNetworkDkgOutput,
};
use group::{ristretto, secp256k1, PartyID};
use homomorphic_encryption::AdditivelyHomomorphicDecryptionKeyShare;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_dwallet_mpc::{
    DWalletMPCSuiEvent, DWalletNetworkDecryptionKeyData, DWalletNetworkEncryptionKeyState,
    MPCProtocolInitData, SessionInfo, StartNetworkDKGEvent,
};
use mpc::{AsynchronousRoundResult, WeightedThresholdAccessStructure};
use std::collections::HashMap;
use sui_types::base_types::ObjectID;
use tap::Pipe;
use tracing::warn;
use twopc_mpc::secp256k1::class_groups::{
    FUNDAMENTAL_DISCRIMINANT_LIMBS, NON_FUNDAMENTAL_DISCRIMINANT_LIMBS,
};
use twopc_mpc::sign::Protocol;
use twopc_mpc::ProtocolPublicParameters;

/// Holds the network (decryption) keys of the network MPC protocols.
pub struct DwalletMPCNetworkKeys {
    /// Holds all network (decryption) keys for the current network in encrypted form.
    /// This data is identical for all the Validator nodes.
    pub(crate) network_encryption_keys: HashMap<ObjectID, NetworkDecryptionKeyPublicData>,
    pub(crate) validator_private_dec_key_data: ValidatorPrivateDecryptionKeyData,
}

/// Holds the private decryption key data for a validator node.
pub struct ValidatorPrivateDecryptionKeyData {
    /// The unique party ID of the validator, representing its index within the committee.
    pub party_id: PartyID,

    /// The validator's class groups decryption key.
    pub class_groups_decryption_key: ClassGroupsDecryptionKey,

    /// A map of the validator's decryption key shares.
    ///
    /// This structure maps each key ID (`ObjectID`) to a sub-map of `PartyID`
    /// to the corresponding decryption key share.
    /// These shares are used in multi-party cryptographic protocols.
    /// NOTE: EACH PARTY IN HERE IS A **VIRTUAL PARTY**.
    /// NOTE 2: `ObjectID` is the ID of the network decryption key, not the party.
    pub validator_decryption_key_shares:
        HashMap<ObjectID, HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>>,
}

fn get_decryption_key_shares_from_public_output(
    shares: &NetworkDecryptionKeyPublicData,
    party_id: PartyID,
    decryption_key: ClassGroupsDecryptionKey,
    weighted_threshold_access_structure: &WeightedThresholdAccessStructure,
) -> DwalletMPCResult<HashMap<PartyID, SecretKeyShareSizedInteger>> {
    match shares.state {
        NetworkDecryptionKeyPublicOutputType::NetworkDkg => match &shares.latest_public_output {
            VersionedNetworkDkgOutput::V1(public_output) => {
                let dkg_public_output: <Secp256k1Party as mpc::Party>::PublicOutput =
                    bcs::from_bytes(public_output)?;

                let secret_shares = dkg_public_output
                    .default_decryption_key_shares::<secp256k1::GroupElement>(
                        party_id,
                        weighted_threshold_access_structure,
                        decryption_key,
                    )
                    .map_err(|err| DwalletMPCError::ClassGroupsError(err.to_string()))?;
                Ok(secret_shares)
            }
        },
        NetworkDecryptionKeyPublicOutputType::Reshare => match &shares.latest_public_output {
            VersionedNetworkDkgOutput::V1(public_output) => {
                let public_output: <ReshareSecp256k1Party as mpc::Party>::PublicOutput =
                    bcs::from_bytes(public_output)?;

                let secret_shares = public_output
                    .decrypt_decryption_key_shares::<secp256k1::GroupElement>(
                        party_id,
                        weighted_threshold_access_structure,
                        decryption_key,
                    )
                    .map_err(|err| DwalletMPCError::ClassGroupsError(err.to_string()))?;
                Ok(secret_shares)
            }
        },
    }
}

impl ValidatorPrivateDecryptionKeyData {
    /// Stores the new decryption key shares of the validator.
    /// Decrypts the decryption key shares (for all the virtual parties)
    /// from the public output of the network DKG protocol.
    pub fn store_decryption_secret_shares(
        &mut self,
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

        self.validator_decryption_key_shares
            .insert(key_id, self_decryption_key_shares);
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

impl DwalletMPCNetworkKeys {
    pub fn new(node_context: ValidatorPrivateDecryptionKeyData) -> Self {
        Self {
            network_encryption_keys: Default::default(),
            validator_private_dec_key_data: node_context,
        }
    }

    pub fn validator_decryption_keys_shares(
        &self,
    ) -> HashMap<ObjectID, HashMap<PartyID, <AsyncProtocol as Protocol>::DecryptionKeyShare>> {
        self.validator_private_dec_key_data
            .validator_decryption_key_shares
            .clone()
    }

    pub fn update_network_key(
        &mut self,
        key_id: ObjectID,
        key: &NetworkDecryptionKeyPublicData,
        weighted_threshold_access_structure: &WeightedThresholdAccessStructure,
    ) -> DwalletMPCResult<()> {
        self.network_encryption_keys.insert(key_id, key.clone());
        self.validator_private_dec_key_data
            .store_decryption_secret_shares(
                key_id,
                key.clone(),
                weighted_threshold_access_structure,
            )
    }

    pub fn get_decryption_public_parameters(&self, key_id: &ObjectID) -> DwalletMPCResult<Vec<u8>> {
        Ok(self
            .network_encryption_keys
            .get(key_id)
            .ok_or(DwalletMPCError::MissingDwalletMPCDecryptionKeyShares(
                "".to_string(),
            ))?
            .decryption_key_share_public_parameters
            .clone())
    }

    /// Retrieves the protocol public parameters for the specified key ID.
    pub fn get_protocol_public_parameters(&self, key_id: &ObjectID) -> DwalletMPCResult<Vec<u8>> {
        let Some(result) = self.network_encryption_keys.get(key_id) else {
            warn!(
                "failed to fetch the network decryption key shares for key ID: {:?}",
                key_id
            );
            return Err(DwalletMPCError::WaitingForNetworkKey(*key_id));
        };
        Ok(result.protocol_public_parameters.clone())
    }

    pub async fn get_network_dkg_public_output(
        &self,
        key_id: &ObjectID,
    ) -> DwalletMPCResult<VersionedNetworkDkgOutput> {
        Ok(self
            .network_encryption_keys
            .get(key_id)
            .ok_or(DwalletMPCError::WaitingForNetworkKey(*key_id))?
            .network_dkg_output
            .clone())
    }
}

/// Advances the network DKG protocol for the supported key types.
pub(crate) fn advance_network_dkg(
    session_id: CommitmentSizedNumber,
    weighted_threshold_access_structure: &WeightedThresholdAccessStructure,
    party_id: PartyID,
    public_input: &[u8],
    key_scheme: &DWalletMPCNetworkKeyScheme,
    messages: HashMap<usize, HashMap<PartyID, Vec<u8>>>,
    class_groups_decryption_key: ClassGroupsDecryptionKey,
) -> DwalletMPCResult<
    AsynchronousRoundResult<MPCMessage, MPCPrivateOutput, SerializedWrappedMPCPublicOutput>,
> {
    let res = match key_scheme {
        DWalletMPCNetworkKeyScheme::Secp256k1 => {
            let result = advance_and_serialize::<Secp256k1Party>(
                session_id,
                party_id,
                weighted_threshold_access_structure,
                messages,
                bcs::from_bytes(public_input)?,
                class_groups_decryption_key,
            );
            match result.clone() {
                Ok(AsynchronousRoundResult::Finalize {
                    public_output,
                    malicious_parties,
                    private_output,
                }) => {
                    let public_output =
                        bcs::to_bytes(&VersionedNetworkDkgOutput::V1(public_output))?;
                    Ok(AsynchronousRoundResult::Finalize {
                        public_output,
                        malicious_parties,
                        private_output,
                    })
                }
                _ => result,
            }
        }
        DWalletMPCNetworkKeyScheme::Ristretto => advance_and_serialize::<RistrettoParty>(
            session_id,
            party_id,
            weighted_threshold_access_structure,
            messages,
            bcs::from_bytes(public_input)?,
            class_groups_decryption_key,
        ),
    }?;
    Ok(res)
}

pub(super) fn network_dkg_public_input(
    weighted_threshold_access_structure: &WeightedThresholdAccessStructure,
    encryption_keys_and_proofs: HashMap<PartyID, ClassGroupsEncryptionKeyAndProof>,
    key_scheme: DWalletMPCNetworkKeyScheme,
) -> DwalletMPCResult<Vec<u8>> {
    match key_scheme {
        DWalletMPCNetworkKeyScheme::Secp256k1 => generate_secp256k1_dkg_party_public_input(
            weighted_threshold_access_structure,
            encryption_keys_and_proofs,
        ),
        DWalletMPCNetworkKeyScheme::Ristretto => generate_ristretto_dkg_party_public_input(
            weighted_threshold_access_structure,
            encryption_keys_and_proofs,
        ),
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
        session_type: deserialized_event.session_type.clone(),
        session_id: deserialized_event.session_id,
        epoch: deserialized_event.epoch,
        mpc_round: MPCProtocolInitData::NetworkDkg(
            DWalletMPCNetworkKeyScheme::Secp256k1,
            deserialized_event,
        ),
    }
}

fn network_dkg_ristretto_session_info(
    deserialized_event: DWalletMPCSuiEvent<StartNetworkDKGEvent>,
) -> SessionInfo {
    SessionInfo {
        session_type: deserialized_event.session_type.clone(),
        session_id: deserialized_event.session_id,
        epoch: deserialized_event.epoch,
        mpc_round: MPCProtocolInitData::NetworkDkg(
            DWalletMPCNetworkKeyScheme::Ristretto,
            deserialized_event,
        ),
    }
}

fn generate_secp256k1_dkg_party_public_input(
    weighted_threshold_access_structure: &WeightedThresholdAccessStructure,
    encryption_keys_and_proofs: HashMap<PartyID, ClassGroupsEncryptionKeyAndProof>,
) -> DwalletMPCResult<Vec<u8>> {
    let public_params = Secp256k1PublicInput::new::<secp256k1::GroupElement>(
        weighted_threshold_access_structure,
        secp256k1::scalar::PublicParameters::default(),
        DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER,
        encryption_keys_and_proofs,
    )
    .map_err(|e| DwalletMPCError::InvalidMPCPartyType(e.to_string()))?;
    bcs::to_bytes(&public_params).map_err(DwalletMPCError::BcsError)
}

fn generate_ristretto_dkg_party_public_input(
    weighted_threshold_access_structure: &WeightedThresholdAccessStructure,
    encryption_keys_and_proofs: HashMap<PartyID, ClassGroupsEncryptionKeyAndProof>,
) -> DwalletMPCResult<Vec<u8>> {
    let public_params = RistrettoPublicInput::new::<ristretto::GroupElement>(
        weighted_threshold_access_structure,
        ristretto::scalar::PublicParameters::default(),
        DEFAULT_COMPUTATIONAL_SECURITY_PARAMETER,
        encryption_keys_and_proofs,
    )
    .map_err(|e| DwalletMPCError::InvalidMPCPartyType(e.to_string()))?;
    bcs::to_bytes(&public_params).map_err(DwalletMPCError::BcsError)
}

pub(crate) fn instantiate_dwallet_mpc_network_decryption_key_shares_from_public_output(
    epoch: u64,
    key_scheme: DWalletMPCNetworkKeyScheme,
    weighted_threshold_access_structure: &WeightedThresholdAccessStructure,
    key_data: DWalletNetworkDecryptionKeyData,
) -> DwalletMPCResult<NetworkDecryptionKeyPublicData> {
    if key_data.current_reconfiguration_public_output.is_empty() {
        if key_data.state == DWalletNetworkEncryptionKeyState::AwaitingNetworkDKG {
            return Err(DwalletMPCError::WaitingForNetworkKey(key_data.id));
        }
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
            &key_data.network_dkg_public_output,
        )
    }
}

fn instantiate_dwallet_mpc_network_decryption_key_shares_from_reshare_public_output(
    epoch: u64,
    weighted_threshold_access_structure: &WeightedThresholdAccessStructure,
    public_output_bytes: &SerializedWrappedMPCPublicOutput,
    network_dkg_public_output: &SerializedWrappedMPCPublicOutput,
) -> DwalletMPCResult<NetworkDecryptionKeyPublicData> {
    let mpc_public_output: VersionedNetworkDkgOutput =
        bcs::from_bytes(public_output_bytes).map_err(DwalletMPCError::BcsError)?;
    match &mpc_public_output {
        VersionedNetworkDkgOutput::V1(public_output_bytes) => {
            let public_output: <ReshareSecp256k1Party as mpc::Party>::PublicOutput =
                bcs::from_bytes(public_output_bytes)?;
            let decryption_key_share_public_parameters = public_output
                .default_decryption_key_share_public_parameters::<secp256k1::GroupElement>(
                    weighted_threshold_access_structure,
                )
                .map_err(|e| DwalletMPCError::ClassGroupsError(e.to_string()))?;
            let protocol_public_parameters = bcs::to_bytes(&ProtocolPublicParameters::new::<
                { secp256k1::SCALAR_LIMBS },
                { FUNDAMENTAL_DISCRIMINANT_LIMBS },
                { NON_FUNDAMENTAL_DISCRIMINANT_LIMBS },
                secp256k1::GroupElement,
            >(
                decryption_key_share_public_parameters
                    .encryption_scheme_public_parameters
                    .clone(),
            ))?;
            Ok(NetworkDecryptionKeyPublicData {
                epoch,
                state: NetworkDecryptionKeyPublicOutputType::Reshare,
                latest_public_output: mpc_public_output,
                decryption_key_share_public_parameters: bcs::to_bytes(
                    &decryption_key_share_public_parameters,
                )?,
                protocol_public_parameters,
                network_dkg_output: bcs::from_bytes(network_dkg_public_output)?,
            })
        }
    }
}

fn instantiate_dwallet_mpc_network_decryption_key_shares_from_dkg_public_output(
    epoch: u64,
    key_scheme: DWalletMPCNetworkKeyScheme,
    weighted_threshold_access_structure: &WeightedThresholdAccessStructure,
    public_output_bytes: &SerializedWrappedMPCPublicOutput,
) -> DwalletMPCResult<NetworkDecryptionKeyPublicData> {
    let mpc_public_output: VersionedNetworkDkgOutput =
        bcs::from_bytes(public_output_bytes).map_err(DwalletMPCError::BcsError)?;
    match key_scheme {
        DWalletMPCNetworkKeyScheme::Secp256k1 => match &mpc_public_output {
            VersionedNetworkDkgOutput::V1(public_output_bytes) => {
                let public_output: <Secp256k1Party as mpc::Party>::PublicOutput =
                    bcs::from_bytes(public_output_bytes)?;
                let decryption_key_share_public_parameters = public_output
                    .default_decryption_key_share_public_parameters::<secp256k1::GroupElement>(
                        weighted_threshold_access_structure,
                    )
                    .map_err(|e| DwalletMPCError::ClassGroupsError(e.to_string()))?;
                let protocol_public_parameters = bcs::to_bytes(&ProtocolPublicParameters::new::<
                    { secp256k1::SCALAR_LIMBS },
                    { FUNDAMENTAL_DISCRIMINANT_LIMBS },
                    { NON_FUNDAMENTAL_DISCRIMINANT_LIMBS },
                    secp256k1::GroupElement,
                >(
                    decryption_key_share_public_parameters
                        .encryption_scheme_public_parameters
                        .clone(),
                ))?;
                Ok(NetworkDecryptionKeyPublicData {
                    epoch,
                    state: NetworkDecryptionKeyPublicOutputType::NetworkDkg,
                    latest_public_output: mpc_public_output.clone(),
                    decryption_key_share_public_parameters: bcs::to_bytes(
                        &decryption_key_share_public_parameters,
                    )?,
                    network_dkg_output: mpc_public_output,
                    protocol_public_parameters,
                })
            }
        },
        DWalletMPCNetworkKeyScheme::Ristretto => todo!("Ristretto key scheme"),
    }
}
