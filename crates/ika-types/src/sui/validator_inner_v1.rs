use super::{ClassGroupsPublicKeyAndProof, Element};
use crate::crypto::{
    verify_proof_of_possession, AuthorityPublicKey, AuthorityPublicKeyBytes, AuthoritySignature,
    NetworkPublicKey,
};
use jsonrpsee::core::Serialize;
use mysten_network::Multiaddr;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use sui_types::balance::Balance;
use sui_types::base_types::{ObjectID, SuiAddress};
use sui_types::collection_types::{Bag, Table, TableVec};
use sui_types::crypto::ToFromBytes;

const E_METADATA_INVALID_POP: u64 = 0;
const E_METADATA_INVALID_PUBKEY: u64 = 1;
const E_METADATA_INVALID_NET_PUBKEY: u64 = 2;
const E_METADATA_INVALID_WORKER_PUBKEY: u64 = 3;
const E_METADATA_INVALID_NET_ADDR: u64 = 4;
const E_METADATA_INVALID_P2P_ADDR: u64 = 5;
const E_METADATA_INVALID_PRIMARY_ADDR: u64 = 6;
const E_METADATA_INVALID_WORKER_ADDR: u64 = 7;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ValidatorMetadataV1 {
    pub payment_address: SuiAddress,
    pub proof_of_possession_sender: SuiAddress,
    pub protocol_pubkey_bytes: Vec<u8>,
    pub protocol_pubkey: Element,
    pub proof_of_possession_bytes: Vec<u8>,
    pub network_pubkey_bytes: Vec<u8>,
    pub consensus_pubkey_bytes: Vec<u8>,
    pub class_groups_public_key_and_proof: TableVec,
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub project_url: String,
    pub network_address: String,
    pub p2p_address: String,
    pub consensus_address: String,
    pub next_epoch_protocol_pubkey_bytes: Option<Vec<u8>>,
    pub next_epoch_proof_of_possession_bytes: Option<Vec<u8>>,
    pub next_epoch_network_pubkey_bytes: Option<Vec<u8>>,
    pub next_epoch_consensus_pubkey_bytes: Option<Vec<u8>>,
    pub next_epoch_network_address: Option<String>,
    pub next_epoch_class_groups_public_key_and_proof: Option<ClassGroupsPublicKeyAndProof>,
    pub next_epoch_p2p_address: Option<String>,
    pub next_epoch_consensus_address: Option<String>,
    pub extra_fields: Bag,
}

#[derive(derive_more::Debug, Clone, Eq, PartialEq)]
pub struct VerifiedValidatorMetadataV1 {
    pub proof_of_possession_sender: SuiAddress,
    pub protocol_pubkey: AuthorityPublicKey,
    pub network_pubkey: NetworkPublicKey,
    pub consensus_pubkey: NetworkPublicKey,
    pub class_groups_public_key_and_proof: TableVec,
    #[debug(ignore)]
    pub proof_of_possession_bytes: Vec<u8>,
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub project_url: String,
    pub network_address: Multiaddr,
    pub p2p_address: Multiaddr,
    pub consensus_address: Multiaddr,
    pub next_epoch_protocol_pubkey: Option<AuthorityPublicKey>,
    pub next_epoch_proof_of_possession_bytes: Option<Vec<u8>>,
    pub next_epoch_network_pubkey: Option<NetworkPublicKey>,
    pub next_epoch_consensus_pubkey: Option<NetworkPublicKey>,
    pub next_epoch_network_address: Option<Multiaddr>,
    pub next_epoch_class_groups_public_key_and_proof: Option<ClassGroupsPublicKeyAndProof>,
    pub next_epoch_p2p_address: Option<Multiaddr>,
    pub next_epoch_consensus_address: Option<Multiaddr>,
}

impl VerifiedValidatorMetadataV1 {
    pub fn ika_pubkey_bytes(&self) -> AuthorityPublicKeyBytes {
        (&self.protocol_pubkey).into()
    }
}

impl ValidatorMetadataV1 {
    /// Verify validator metadata and return a verified version (on success) or error code (on failure)
    pub fn verify(&self) -> anyhow::Result<VerifiedValidatorMetadataV1, u64> {
        let protocol_pubkey = AuthorityPublicKey::from_bytes(self.protocol_pubkey_bytes.as_ref())
            .map_err(|_| E_METADATA_INVALID_PUBKEY)?;

        // Verify proof of possession for the protocol key
        let pop = AuthoritySignature::from_bytes(self.proof_of_possession_bytes.as_ref())
            .map_err(|_| E_METADATA_INVALID_POP)?;
        verify_proof_of_possession(&pop, &protocol_pubkey, self.proof_of_possession_sender)
            .map_err(|_| E_METADATA_INVALID_POP)?;

        let network_pubkey = NetworkPublicKey::from_bytes(self.network_pubkey_bytes.as_ref())
            .map_err(|_| E_METADATA_INVALID_NET_PUBKEY)?;
        let consensus_pubkey = NetworkPublicKey::from_bytes(self.consensus_pubkey_bytes.as_ref())
            .map_err(|_| E_METADATA_INVALID_WORKER_PUBKEY)?;
        if consensus_pubkey == network_pubkey {
            return Err(E_METADATA_INVALID_WORKER_PUBKEY);
        }

        let network_address = Multiaddr::try_from(self.network_address.clone())
            .map_err(|_| E_METADATA_INVALID_NET_ADDR)?;

        // Ensure p2p, primary, and worker addresses are both Multiaddr's and valid anemo addresses
        let p2p_address = Multiaddr::try_from(self.p2p_address.clone())
            .map_err(|_| E_METADATA_INVALID_P2P_ADDR)?;
        p2p_address
            .to_anemo_address()
            .map_err(|_| E_METADATA_INVALID_P2P_ADDR)?;

        let consensus_address = Multiaddr::try_from(self.consensus_address.clone())
            .map_err(|_| E_METADATA_INVALID_PRIMARY_ADDR)?;
        consensus_address
            .to_anemo_address()
            .map_err(|_| E_METADATA_INVALID_PRIMARY_ADDR)?;

        let next_epoch_protocol_pubkey = match self.next_epoch_protocol_pubkey_bytes.clone() {
            None => Ok::<Option<AuthorityPublicKey>, u64>(None),
            Some(bytes) => Ok(Some(
                AuthorityPublicKey::from_bytes(bytes.as_ref())
                    .map_err(|_| E_METADATA_INVALID_PUBKEY)?,
            )),
        }?;

        let next_epoch_pop = match self.next_epoch_proof_of_possession_bytes.clone() {
            None => Ok::<Option<AuthoritySignature>, u64>(None),
            Some(bytes) => Ok(Some(
                AuthoritySignature::from_bytes(bytes.as_ref())
                    .map_err(|_| E_METADATA_INVALID_POP)?,
            )),
        }?;
        // Verify proof of possession for the next epoch protocol key
        if let Some(ref next_epoch_protocol_pubkey) = next_epoch_protocol_pubkey {
            match next_epoch_pop {
                Some(next_epoch_pop) => {
                    verify_proof_of_possession(
                        &next_epoch_pop,
                        next_epoch_protocol_pubkey,
                        self.proof_of_possession_sender,
                    )
                    .map_err(|_| E_METADATA_INVALID_POP)?;
                }
                None => {
                    return Err(E_METADATA_INVALID_POP);
                }
            }
        }

        let next_epoch_network_pubkey = match self.next_epoch_network_pubkey_bytes.clone() {
            None => Ok::<Option<NetworkPublicKey>, u64>(None),
            Some(bytes) => Ok(Some(
                NetworkPublicKey::from_bytes(bytes.as_ref())
                    .map_err(|_| E_METADATA_INVALID_NET_PUBKEY)?,
            )),
        }?;

        let next_epoch_consensus_pubkey: Option<NetworkPublicKey> =
            match self.next_epoch_consensus_pubkey_bytes.clone() {
                None => Ok::<Option<NetworkPublicKey>, u64>(None),
                Some(bytes) => Ok(Some(
                    NetworkPublicKey::from_bytes(bytes.as_ref())
                        .map_err(|_| E_METADATA_INVALID_WORKER_PUBKEY)?,
                )),
            }?;
        if next_epoch_network_pubkey.is_some()
            && next_epoch_network_pubkey == next_epoch_consensus_pubkey
        {
            return Err(E_METADATA_INVALID_WORKER_PUBKEY);
        }

        let next_epoch_network_address = match self.next_epoch_network_address.clone() {
            None => Ok::<Option<Multiaddr>, u64>(None),
            Some(address) => Ok(Some(
                Multiaddr::try_from(address).map_err(|_| E_METADATA_INVALID_NET_ADDR)?,
            )),
        }?;

        let next_epoch_p2p_address = match self.next_epoch_p2p_address.clone() {
            None => Ok::<Option<Multiaddr>, u64>(None),
            Some(address) => {
                let address =
                    Multiaddr::try_from(address).map_err(|_| E_METADATA_INVALID_P2P_ADDR)?;
                address
                    .to_anemo_address()
                    .map_err(|_| E_METADATA_INVALID_P2P_ADDR)?;

                Ok(Some(address))
            }
        }?;

        let next_epoch_consensus_address = match self.next_epoch_consensus_address.clone() {
            None => Ok::<Option<Multiaddr>, u64>(None),
            Some(address) => {
                let address =
                    Multiaddr::try_from(address).map_err(|_| E_METADATA_INVALID_PRIMARY_ADDR)?;
                address
                    .to_anemo_address()
                    .map_err(|_| E_METADATA_INVALID_PRIMARY_ADDR)?;

                Ok(Some(address))
            }
        }?;

        Ok(VerifiedValidatorMetadataV1 {
            proof_of_possession_sender: self.proof_of_possession_sender,
            protocol_pubkey,
            network_pubkey,
            consensus_pubkey,
            class_groups_public_key_and_proof: self.class_groups_public_key_and_proof.clone(),
            proof_of_possession_bytes: self.proof_of_possession_bytes.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            image_url: self.image_url.clone(),
            project_url: self.project_url.clone(),
            network_address,
            p2p_address,
            consensus_address,
            next_epoch_protocol_pubkey,
            next_epoch_proof_of_possession_bytes: self.next_epoch_proof_of_possession_bytes.clone(),
            next_epoch_network_pubkey,
            next_epoch_consensus_pubkey,
            next_epoch_network_address,
            next_epoch_class_groups_public_key_and_proof: self
                .next_epoch_class_groups_public_key_and_proof
                .clone(),
            next_epoch_p2p_address,
            next_epoch_consensus_address,
        })
    }
}

/// Rust version of the Move ika::validator_inner::ValidatorInner type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ValidatorInnerV1 {
    pub validator_id: ObjectID,
    metadata: ValidatorMetadataV1,
    #[serde(skip)]
    verified_metadata: OnceCell<VerifiedValidatorMetadataV1>,

    pub cap_id: ObjectID,
    pub operation_cap_id: ObjectID,
    pub computation_price: u64,
    pub staking_pool: StakingPoolV1,
    pub commission_rate: u16,
    pub next_epoch_stake: u64,
    pub next_epoch_computation_price: u64,
    pub next_epoch_commission_rate: u16,
    pub extra_fields: Bag,
}

impl ValidatorInnerV1 {
    pub fn verified_metadata(&self) -> &VerifiedValidatorMetadataV1 {
        self.verified_metadata.get_or_init(|| {
            self.metadata
                .verify()
                .expect("Validity of metadata should be verified on-chain")
        })
    }
}

/// Rust version of the Move ika_system::staking_pool::StakingPool type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct StakingPoolV1 {
    pub id: ObjectID,
    pub activation_epoch: Option<u64>,
    pub deactivation_epoch: Option<u64>,
    pub ika_balance: u64,
    pub rewards_pool: Balance,
    pub pool_token_balance: u64,
    pub exchange_rates: Table,
    pub pending_stake: u64,
    pub pending_total_ika_withdraw: u64,
    pub pending_pool_token_withdraw: u64,
    pub total_supply: u64,
    pub principal: Balance,
    pub extra_fields: Bag,
}
