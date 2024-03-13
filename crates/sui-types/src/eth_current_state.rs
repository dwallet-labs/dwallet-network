use helios::consensus::types::{Header, SyncCommittee};

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
pub struct EthStateSuiObject {
    pub data: Vec<u8>,
}
