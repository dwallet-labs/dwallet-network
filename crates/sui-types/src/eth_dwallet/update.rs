use helios::consensus::types::{FinalityUpdate, FinalityUpdateSerde, OptimisticUpdate, OptimisticUpdateSerde, Update, UpdateSerde};
use serde::{Deserializer, Serializer};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct UpdatesResponse {
    pub updates: Vec<Update>,
    pub finality_update: FinalityUpdate,
    pub optimistic_update: OptimisticUpdate,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct UpdatesResponseSerde {
    pub updates: Vec<UpdateSerde>,
    pub finality_update: FinalityUpdateSerde,
    pub optimistic_update: OptimisticUpdateSerde,
}

impl Default for UpdatesResponse {
    fn default() -> Self {
        UpdatesResponse {
            updates: vec![],
            finality_update: Default::default(),
            optimistic_update: Default::default(),
        }
    }
}

impl From<UpdatesResponseSerde> for UpdatesResponse {
    fn from(value: UpdatesResponseSerde) -> UpdatesResponse {
        UpdatesResponse {
            updates: value.updates.into_iter().map(|update| update.into()).collect(),
            finality_update: value.finality_update.into(),
            optimistic_update: value.optimistic_update.into(),
        }
    }
}

impl Into<UpdatesResponseSerde> for UpdatesResponse {
    fn into(self) -> UpdatesResponseSerde {
        UpdatesResponseSerde {
            updates: self.updates.into_iter().map(|update| update.into()).collect(),
            finality_update: self.finality_update.into(),
            optimistic_update: self.optimistic_update.into(),
        }
    }
}

impl UpdatesResponse {
    pub fn deserialize_from_bytes(bytes: Vec<u8>) -> Result<UpdatesResponse, anyhow::Error> {
        let updates_response_serde: UpdatesResponseSerde = bcs::from_bytes(&bytes)?;
        Ok(updates_response_serde.into())
    }
}