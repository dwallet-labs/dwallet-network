use anyhow::anyhow;
use ethers::prelude::*;
use ethers::prelude::H160;
use ethers::utils::hex::ToHexExt;
use ethers::utils::keccak256;
use ethers::utils::rlp::RlpStream;
use eyre::{eyre, Report};
use helios::client::{Client, ClientBuilder, FileDB};
use helios::config::checkpoints;
use helios::config::networks::Network;
use helios::consensus::types::Bytes32;
use helios::consensus::types::primitives::U64;
use helios::errors::BlockNotFoundError;
use helios::execution::proof::verify_proof;
use helios::types::BlockTag;
use serde::Serialize;
use serde_json::{Number, Value};
use std::str::FromStr;
use sui_json_rpc_types::{SuiData, SuiObjectDataOptions, SuiRawData};
use sui_sdk::json::SuiJsonValue;
use sui_sdk::sui_client_config::SuiEnv;
use sui_sdk::wallet_context::WalletContext;
use sui_types::base_types::ObjectID;
use sui_types::eth_dwallet_cap::EthDWalletCap;
use sui_types::object::Owner;
use sui_types::transaction::SharedInputObject;
use tracing::info;

use crate::config::{EthLightClientConfig, ProofRequestParameters};
use crate::eth_state::{EthState, EthStateObject, LatestEthStateObject};
use crate::proof::{Proof, ProofResponse};
use crate::update::UpdatesResponse;
use crate::utils;

pub struct EthLightClient {
    pub client: Client<FileDB>,
    config: EthLightClientConfig,
}

impl EthLightClient {
    pub async fn new(conf: EthLightClientConfig) -> Result<Self, anyhow::Error> {
        let network = &conf.network;

        let client: Client<FileDB> = ClientBuilder::new()
            .network(network.clone())
            .execution_rpc(&conf.execution_rpc)
            .consensus_rpc(&conf.consensus_rpc)
            .checkpoint(&conf.checkpoint)
            .data_dir("/tmp/helios".parse()?)
            .build()
            .map_err(|e| anyhow!("failed to create client: {}", e))?;

        info!("EthLightClient created");

        Ok(Self {
            client,
            config: conf,
        })
    }

    pub async fn start(&mut self) -> Result<(), anyhow::Error> {
        self.client
            .start()
            .await
            .map_err(|e| anyhow!("failed to start client: {}", e))?;
        self.client.wait_synced().await;
        info!("EthLightClient connected");

        Ok(())
    }

    /// Get the Merkle Tree Proof (EIP1186Proof) for the client parameters.
    pub async fn get_proofs(
        &self,
        proof_parameters: ProofRequestParameters,
        contract_addr: &Address,
        block_number: u64,
        state_root: &Bytes32,
    ) -> eyre::Result<ProofResponse, Report> {
        let message_map_index = utils::get_message_storage_slot(
            proof_parameters.message.clone(),
            proof_parameters.dwallet_id.clone(),
            proof_parameters.data_slot,
        )?;

        let proof = self
            .client
            .get_proof(&contract_addr, &[message_map_index], block_number)
            .await?;

        let account_proof = create_account_proof(contract_addr, state_root, &proof);

        let storage_proof = create_storage_proof(message_map_index, proof)?;

        Ok(ProofResponse {
            account_proof,
            storage_proof,
        })
    }

    pub async fn get_block_number(&self) -> Result<u64, Report> {
        Ok(self.client.get_block_number().await?.as_u64())
    }
}

fn get_data_from_eth_dwallet_cap(
    eth_dwallet_cap_obj: EthDWalletCap,
) -> Result<(u64, Address), anyhow::Error> {
    let data_slot = eth_dwallet_cap_obj.eth_smart_contract_slot;
    let contract_addr: String = eth_dwallet_cap_obj.eth_smart_contract_addr;
    let contract_addr = contract_addr.clone().parse::<Address>()?;
    Ok((data_slot, contract_addr))
}

async fn init_light_client(
    eth_client_config: EthLightClientConfig,
) -> Result<EthLightClient, anyhow::Error> {
    let mut eth_lc = EthLightClient::new(eth_client_config.clone()).await?;
    eth_lc.start().await?;
    Ok(eth_lc)
}

async fn fetch_proofs(
    eth_lc: &mut EthLightClient,
    eth_state: &EthState,
    contract_addr: &Address,
    proof_parameters: ProofRequestParameters,
) -> Result<ProofResponse, anyhow::Error> {
    let proof = eth_lc
        .get_proofs(
            proof_parameters,
            contract_addr,
            eth_state.last_update_execution_block_number,
            &eth_state.last_update_execution_state_root,
        )
        .await
        .map_err(|e| anyhow!("error fetching proof from Consensus RPC: {e}"))?;
    Ok(proof)
}

async fn fetch_consensus_updates(eth_state: &mut EthState) -> Result<UpdatesResponse, anyhow::Error> {
    let updates = eth_state
        .get_updates(&eth_state.clone().last_checkpoint)
        .await
        .map_err(|e| anyhow!("error fetching updates from Consensus RPC: {e}"))?;
    Ok(updates)
}

fn get_eth_config(context: &mut WalletContext) -> Result<EthLightClientConfig, anyhow::Error> {
    let sui_env_config = context.config.get_active_env()?;
    if sui_env_config.eth_light_client.is_none() {
        return Err(anyhow!("ETH Light Client configuration not found"));
    }

    let eth_light_client_config = sui_env_config.eth_light_client.as_ref().unwrap();
    let eth_execution_rpc = eth_light_client_config
        .eth_execution_rpc
        .clone()
        .ok_or_else(|| anyhow!("ETH execution RPC configuration not found"))?;
    let eth_consensus_rpc = eth_light_client_config
        .eth_consensus_rpc
        .clone()
        .ok_or_else(|| anyhow!("ETH consensus RPC configuration not found"))?;

    let mut eth_lc_config = EthLightClientConfig::default();
    eth_lc_config.network = get_network_by_sui_env(sui_env_config)?;
    eth_lc_config.execution_rpc = eth_execution_rpc;
    eth_lc_config.consensus_rpc = eth_consensus_rpc;

    Ok(eth_lc_config)
}

fn get_network_by_sui_env(sui_env_config: &SuiEnv) -> Result<Network, anyhow::Error> {
    let network = match sui_env_config.alias.as_str() {
        "mainnet" => Network::MAINNET,
        "testnet" => Network::HOLESKY,
        "localnet" => get_eth_devnet_network(sui_env_config)?,
        _ => Network::MAINNET,
    };
    Ok(network)
}

fn get_eth_devnet_network(sui_env_config: &SuiEnv) -> Result<Network, anyhow::Error> {
    if sui_env_config.eth_light_client.is_none() {
        return Err(anyhow!("ETH Light Client configuration not found"));
    }

    let eth_light_client_config = sui_env_config.eth_light_client.as_ref().unwrap();
    let eth_chain_id = eth_light_client_config
        .eth_chain_id
        .clone()
        .ok_or_else(|| anyhow!("ETH Chain ID configuration not found"))?;
    let eth_genesis_time = eth_light_client_config
        .eth_genesis_time
        .clone()
        .ok_or_else(|| anyhow!("ETH Genesis Time configuration not found"))?;
    let eth_genesis_validators_root = eth_light_client_config
        .eth_genesis_validators_root
        .clone()
        .ok_or_else(|| anyhow!("ETH Genesis Validators Root configuration not found"))?;

    let chain_config = helios::config::types::ChainConfig {
        chain_id: eth_chain_id,
        genesis_time: eth_genesis_time,
        genesis_root: hex_str_to_bytes(&eth_genesis_validators_root).unwrap(),
    };
    Ok(Network::DEVNET(chain_config))
}

fn hex_str_to_bytes(s: &str) -> eyre::Result<Vec<u8>> {
    let stripped = s.strip_prefix("0x").unwrap_or(s);
    Ok(hex::decode(stripped)?)
}


fn create_account_proof(
    contract_addr: &Address,
    state_root: &Bytes32,
    proof: &EIP1186ProofResponse,
) -> Proof {
    let account_path = keccak256(contract_addr.as_bytes()).to_vec();
    let account_encoded = encode_account(&proof);

    let account_proof = Proof {
        proof: proof.clone().account_proof,
        root: state_root.as_slice().to_vec(),
        path: account_path,
        value: account_encoded,
    };
    account_proof
}

fn create_storage_proof(
    message_map_index: H256,
    proof: EIP1186ProofResponse,
) -> Result<Proof, Report> {
    // The storage proof for the specific message and dWalletID in the mapping.
    let msg_storage_proof = proof
        .storage_proof
        .iter()
        .find(|p| p.key == U256::from(message_map_index.as_bytes()))
        .ok_or_else(|| eyre!("Storage proof not found"))?;

    // 1 for True (if the message is approved, the value in the contract's storage map would be True).
    let storage_value = [1].to_vec();
    let mut msg_storage_proof_key_bytes = [0u8; 32];
    msg_storage_proof
        .key
        .to_big_endian(&mut msg_storage_proof_key_bytes);
    let storage_key_hash = keccak256(msg_storage_proof_key_bytes);

    let storage_proof = Proof {
        proof: msg_storage_proof.clone().proof,
        root: proof.storage_hash.as_bytes().to_vec(),
        path: storage_key_hash.to_vec(),
        value: storage_value,
    };
    Ok(storage_proof)
}

fn encode_account(proof: &EIP1186ProofResponse) -> Vec<u8> {
    let mut stream = RlpStream::new_list(4);
    stream.append(&proof.nonce);
    stream.append(&proof.balance);
    stream.append(&proof.storage_hash);
    stream.append(&proof.code_hash);
    let encoded = stream.out();
    encoded.to_vec()
}