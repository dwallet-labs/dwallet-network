use std::str::FromStr;

use anyhow::anyhow;
use ethers::prelude::H160;
use ethers::prelude::*;
use ethers::utils::hex::ToHexExt;
use ethers::utils::keccak256;
use ethers::utils::rlp::RlpStream;
use eyre::{eyre, Report};
use helios::client::{Client, ClientBuilder, FileDB};
use helios::config::checkpoints;
use helios::config::networks::Network;
use helios::consensus::types::Bytes32;
// use helios::prelude::*;
use tracing::info;

use crate::eth_dwallet::config::{EthLightClientConfig, ProofParameters};
use crate::eth_dwallet::proof::{verify_proof, Proof, ProofResponse};
use crate::eth_dwallet::utils;
use crate::eth_dwallet::utils::is_empty_value;
use helios::consensus::types::primitives::U64;
use helios::errors::BlockNotFoundError;
use helios::types::BlockTag;

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
        proof_parameters: ProofParameters,
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

fn create_account_proof(contract_addr: &Address, state_root: &Bytes32, proof: &EIP1186ProofResponse) -> Proof {
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

/// Fetch the latest checkpoint
/// More info here:
/// https://www.ledger.com/academy/ethereum-proof-of-stake-pos-explained#:~:text=Under%20Proof%20of%20Stake%20(PoS,6.4%20minutes)%20is%20a%20checkpoint.
async fn fetch_latest_checkpoint(network: Network) -> Result<String, anyhow::Error> {
    let checkpoint_fb = checkpoints::CheckpointFallback::new()
        .build()
        .await
        .map_err(|e| anyhow!("failed to create checkpoint fallback: {}", e))?;
    let checkpoint = checkpoint_fb
        .fetch_latest_checkpoint(&network)
        .await
        .map_err(|e| {
            anyhow!(
                "failed to fetch latest checkpoint from fallback services: {}",
                e
            )
        })?;
    info!("fetched latest Ethereum `{network}` checkpoint: `{checkpoint}`");
    Ok(checkpoint.to_string())
}

fn create_storage_proof(message_map_index: H256, proof: EIP1186ProofResponse) -> Result<Proof, Report> {
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

pub fn encode_account(proof: &EIP1186ProofResponse) -> Vec<u8> {
    let mut stream = RlpStream::new_list(4);
    stream.append(&proof.nonce);
    stream.append(&proof.balance);
    stream.append(&proof.storage_hash);
    stream.append(&proof.code_hash);
    let encoded = stream.out();
    encoded.to_vec()
}
