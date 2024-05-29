use anyhow::anyhow;
use move_core_types::ident_str;
use move_core_types::identifier::IdentStr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::id::{ID, UID};

pub const ETH_DWALLET_MODULE_NAME: &IdentStr = ident_str!("eth_dwallet");
pub const ETHEREUM_STATE_MODULE_NAME: &IdentStr = ident_str!("ethereum_state");
pub const LATEST_ETH_STATE_STRUCT_NAME: &IdentStr = ident_str!("LatestEthereumState");
pub const INIT_STATE_FUNC_NAME: &IdentStr = ident_str!("init_state");
pub const CREATE_ETH_DWALLET_CAP_FUNC_NAME: &IdentStr = ident_str!("create_eth_dwallet_cap");
pub const VERIFY_ETH_STATE_FUNC_NAME: &IdentStr = ident_str!("verify_new_eth_state");
pub const APPROVE_MESSAGE_FUNC_NAME: &IdentStr = ident_str!("approve_message");

/// Rust version of the Move sui_system::eth_dwallet::EthDWalletCap type
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct EthDWalletCap {
    pub id: UID,
    pub dwallet_cap_id: ID,
    pub eth_smart_contract_addr: String,
    pub eth_smart_contract_slot: u64,
}
