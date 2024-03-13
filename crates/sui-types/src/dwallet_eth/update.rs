use helios::consensus::types::{FinalityUpdate, OptimisticUpdate, Update};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UpdatesResponse {
    pub updates: Vec<Update>,
    pub finality_update: FinalityUpdate,
    pub optimistic_update: OptimisticUpdate,
    pub provided_checkpoint: String,
}