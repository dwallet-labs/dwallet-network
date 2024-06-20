use anyhow::{anyhow, Error};
use ethers::addressbook::Address;
use ethers::utils::__serde_json::{Number, Value};
use ethers::utils::hex::hex;
use helios::config::networks::Network;
use serde::Serialize;
use sui_sdk::json::SuiJsonValue;
use sui_sdk::rpc_types::{SuiObjectDataOptions, SuiRawData};
use sui_sdk::sui_client_config::{EthLightClientConfig, SuiEnv};
use sui_sdk::types::base_types::ObjectID;
use sui_sdk::types::eth_types::eth_dwallet_cap::EthDWalletCap;
use sui_sdk::types::eth_types::eth_state::EthState;
use sui_sdk::types::object::Owner;
use sui_sdk::types::transaction::SharedInputObject;
use sui_sdk::wallet_context::WalletContext;
use crate::light_client::EthLightClient;
use crate::proof::{ProofParameters, ProofResponse};
use crate::update::UpdatesResponse;


fn serialize_object<T>(object: &T) -> Result<SuiJsonValue, Error>
    where
        T: ?Sized + Serialize,
{
    let object_bytes = bcs::to_bytes(&object)?;
    let object_json = object_bytes
        .iter()
        .map(|v| Value::Number(Number::from(*v)))
        .collect();
    Ok(SuiJsonValue::new(Value::Array(object_json))?)
}

pub async fn get_object_bcs_by_id(
    context: &mut WalletContext,
    object_id: ObjectID,
) -> Result<SuiRawData, Error> {
    let object_resp = context
        .get_client()
        .await?
        .read_api()
        .get_object_with_options(
            object_id,
            SuiObjectDataOptions::default().with_bcs().with_owner(),
        )
        .await?;

    match object_resp.data {
        Some(data) => Ok(
            data.bcs.ok_or_else(|| anyhow!("missing object data"))?,
        ),
        None => Err(anyhow!("Could not find object with ID: {:?}", object_id)),
    }
}

pub async fn get_shared_object_input_by_id(
    context: &mut WalletContext,
    object_id: ObjectID,
) -> Result<SharedInputObject, Error> {
    let object_resp = context
        .get_client()
        .await?
        .read_api()
        .get_object_with_options(object_id, SuiObjectDataOptions::default().with_owner())
        .await?;

    match object_resp.data {
        Some(_) => {
            let owner = object_resp
                .owner()
                .ok_or_else(|| anyhow!("missing object owner"))?;
            let initial_shared_version = match owner {
                Owner::Shared {
                    initial_shared_version,
                } => initial_shared_version,
                _ => return Err(anyhow!("Object is not shared")),
            };
            Ok(SharedInputObject {
                id: object_id,
                initial_shared_version,
                mutable: true,
            })
        }
        None => Err(anyhow!("Could not find object with ID: {:?}", object_id)),
    }
}

pub fn get_data_from_eth_dwallet_cap(
    eth_dwallet_cap_obj: EthDWalletCap,
) -> Result<(u64, Address), Error> {
    let data_slot = eth_dwallet_cap_obj.eth_smart_contract_slot;
    let contract_addr: String = eth_dwallet_cap_obj.eth_smart_contract_addr;
    let contract_addr = contract_addr.clone().parse::<Address>()?;
    Ok((data_slot, contract_addr))
}

pub fn get_eth_config(context: &mut WalletContext) -> Result<EthLightClientConfig, Error> {
    let mut eth_lc_config = EthLightClientConfig::default();

    let sui_env_config = context.config.get_active_env()?;
    let eth_execution_rpc = sui_env_config
        .eth_execution_rpc
        .clone()
        .ok_or_else(|| anyhow!("ETH execution RPC configuration not found"))?;
    let eth_consensus_rpc = sui_env_config
        .eth_consensus_rpc
        .clone()
        .ok_or_else(|| anyhow!("ETH consensus RPC configuration not found"))?;

    eth_lc_config.network = get_network_by_sui_env(sui_env_config)?;

    eth_lc_config.execution_rpc = eth_execution_rpc;
    eth_lc_config.consensus_rpc = eth_consensus_rpc;

    Ok(eth_lc_config)
}

pub fn get_network_by_sui_env(sui_env_config: &SuiEnv) -> Result<Network, Error> {
    let network = match sui_env_config.alias.as_str() {
        "mainnet" => Network::MAINNET,
        "testnet" => Network::HOLESKY,
        "localnet" => get_eth_devnet_network(sui_env_config)?,
        _ => Network::MAINNET,
    };
    Ok(network)
}

pub async fn init_light_client(
    eth_client_config: EthLightClientConfig,
) -> Result<EthLightClient, Error> {
    let mut eth_lc = EthLightClient::new(eth_client_config.clone()).await?;
    eth_lc.start().await?;
    Ok(eth_lc)
}

pub fn get_eth_devnet_network(sui_env_config: &SuiEnv) -> Result<Network, Error> {
    let eth_chain_id = sui_env_config
        .eth_chain_id
        .clone()
        .ok_or_else(|| anyhow!("ETH Chain ID configuration not found"))?;
    let eth_genesis_time = sui_env_config
        .eth_genesis_time
        .clone()
        .ok_or_else(|| anyhow!("ETH Genesis Time configuration not found"))?;
    let eth_genesis_validators_root = sui_env_config
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

pub async fn fetch_proofs(
    eth_lc: &mut EthLightClient,
    eth_state: &EthState,
    contract_addr: &Address,
    proof_parameters: ProofParameters,
) -> Result<ProofResponse, Error> {
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

pub async fn fetch_consensus_updates(eth_state: &mut EthState) -> Result<UpdatesResponse, Error> {
    let updates = eth_state
        .get_updates(&eth_state.clone().last_checkpoint)
        .await
        .map_err(|e| anyhow!("error fetching updates from Consensus RPC: {e}"))?;
    Ok(updates)
}

pub fn hex_str_to_bytes(s: &str) -> eyre::Result<Vec<u8>> {
    let stripped = s.strip_prefix("0x").unwrap_or(s);
    Ok(hex::decode(stripped)?)
}