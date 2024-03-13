use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::id::{ID, UID};

/// Rust version of the Move sui_system::eth_dwallet::EthDWalletCap type
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct EthDWalletCap {
    pub id: UID,
    pub dwallet_cap_id: ID,
    pub eth_smart_contract_addr: Vec<u8>,
    pub eth_smart_contract_slot: u64,
}
