use eyre::Report;
use helios::prelude::networks::Network;

pub struct EthClientConfig {
    // Eth Network (Mainnet, Goerli, etc).
    pub network: Network,
    // Eth RPC URL.
    pub execution_rpc: String,
    // Consensus RPC URL.
    pub consensus_rpc: String,
    // Smart contract address.
    pub contract_addr: String,
    // Data Slot.
    pub data_slot: u64,
    // Message Hash.
    pub message: String,
    // DWalletID
    pub dwallet_id: Vec<u8>,
    pub max_checkpoint_age: u64,
    // Beacon Checkpoint
    pub checkpoint: String,
}

impl EthClientConfig {
    pub fn new(
        network: Network,
        execution_rpc: String,
        contract_addr: String,
        consensus_rpc: String,
        data_slot: u64,
        dwallet_id: Vec<u8>,
        message: String,
        max_checkpoint_age: u64,
        checkpoint: String,
    ) -> Result<Self, anyhow::Error> {
        let result = Self {
            network,
            execution_rpc,
            contract_addr,
            consensus_rpc,
            data_slot,
            dwallet_id,
            message,
            max_checkpoint_age,
            checkpoint,
        };
        Ok(result)
    }
}
