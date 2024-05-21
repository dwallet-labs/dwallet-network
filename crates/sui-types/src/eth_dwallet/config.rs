use eyre::Report;
use helios::prelude::networks::Network;
#[derive(Default, Clone)]
pub struct EthLightClientConfig {
    // Eth Network (Mainnet, Goerli, etc).
    pub network: Network,
    // Eth RPC URL.
    pub execution_rpc: String,
    // Consensus RPC URL.
    pub consensus_rpc: String,
    pub max_checkpoint_age: u64,
    // Beacon Checkpoint
    pub checkpoint: String,
}

#[derive(Default, Clone)]
pub struct ProofParameters {
    pub message: String,
    pub dwallet_id: Vec<u8>,
    pub data_slot: u64,
}
impl EthLightClientConfig {
    pub fn new(
        network: Network,
        execution_rpc: String,
        consensus_rpc: String,
        max_checkpoint_age: u64,
        checkpoint: String,
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            network,
            execution_rpc,
            consensus_rpc,
            max_checkpoint_age,
            checkpoint,
        })
    }
}
