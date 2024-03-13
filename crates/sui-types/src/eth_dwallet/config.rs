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
    pub message: Vec<u8>,
    // DWalletID
    pub dwallet_id: [u8; 20],
    pub max_checkpoint_age: u64,
}

impl EthClientConfig {
    pub fn new(
        network: Network,
        execution_rpc: String,
        contract_addr: String,
        consensus_rpc: String,
        data_slot: u64,
        dwallet_id: Vec<u8>,
        message: Vec<u8>,
        max_checkpoint_age: u64,
    ) -> Result<Self, anyhow::Error> {
        let result = Self {
            network,
            execution_rpc,
            contract_addr,
            consensus_rpc,
            data_slot,
            dwallet_id: <[u8; 20]>::try_from(dwallet_id.as_slice())?,
            message,
            max_checkpoint_age,
        };
        Ok(result)
    }
}
