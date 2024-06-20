use helios::consensus::types::{Update,FinalityUpdate,OptimisticUpdate};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct UpdatesResponse {
    pub updates: Vec<Update>,
    pub finality_update: FinalityUpdate,
    pub optimistic_update: OptimisticUpdate,
}