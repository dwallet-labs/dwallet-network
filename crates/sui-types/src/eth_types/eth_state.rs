use helios::config::networks::Network;
use helios::consensus::types::{Bytes32, Header, SyncCommittee};
use serde::{Deserialize, Serialize};
use crate::base_types::ObjectID;
use crate::id::UID;

#[derive(Deserialize, Serialize)]
pub struct EthStateObject {
    pub id: UID,
    pub data: Vec<u8>,
    pub time_slot: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EthState {
    #[serde(default)]
    pub last_checkpoint: String,
    #[serde(default)]
    pub latest_header: Header,
    #[serde(default)]
    pub current_sync_committee: SyncCommittee,
    #[serde(default)]
    pub next_sync_committee: Option<SyncCommittee>,
    #[serde(default)]
    pub finalized_header: Header,
    #[serde(default)]
    rpc: String,
    #[serde(default)]
    optimistic_header: Header,
    #[serde(default)]
    previous_max_active_participants: u64,
    #[serde(default)]
    current_max_active_participants: u64,
    #[serde(default)]
    network: Network,
    #[serde(default)]
    pub last_update_execution_block_number: u64,
    #[serde(default)]
    pub last_update_execution_state_root: Bytes32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LatestEthStateObject {
    pub id: UID,
    pub eth_state_id: ObjectID,
    pub time_slot: u64,
}