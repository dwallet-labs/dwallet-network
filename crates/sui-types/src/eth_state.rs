use helios::consensus::types::{Header, SyncCommittee};
use move_core_types::ident_str;
use move_core_types::identifier::IdentStr;

pub const ETH_STATE_OBJECT_MODULE_NAME: &IdentStr = ident_str!("eth_state");
// Move struct name
pub const ETH_STATE_OBJECT_STRUCT_NAME: &IdentStr = ident_str!("EthState");

#[derive(Debug, Clone)]
pub struct EthCurrentState {
    pub last_checkpoint: Vec<u8>,
    pub current_sync_committee: SyncCommittee,
    pub next_sync_committee: Option<SyncCommittee>,
    pub finalized_header: Header,
    pub optimistic_header: Header,
    pub previous_max_active_participants: u64,
    pub current_max_active_participants: u64,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct EthStateObject {
    pub data: Vec<u8>,
}
