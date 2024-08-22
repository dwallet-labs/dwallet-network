use crate::base_types::ObjectID;
use anyhow::anyhow;
use move_core_types::ident_str;
use move_core_types::identifier::IdentStr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::id::{ID, UID};

pub const ETH_DWALLET_MODULE_NAME: &IdentStr = ident_str!("eth_dwallet");
pub const ETHEREUM_STATE_MODULE_NAME: &IdentStr = ident_str!("ethereum_state");
pub const LATEST_ETH_STATE_STRUCT_NAME: &IdentStr = ident_str!("LatestEthereumState");
pub const ETH_STATE_STRUCT_NAME: &IdentStr = ident_str!("EthereumState");
pub const INIT_STATE_FUNC_NAME: &IdentStr = ident_str!("init_state");
pub const CREATE_ETH_DWALLET_CAP_FUNC_NAME: &IdentStr = ident_str!("create_eth_dwallet_cap");
pub const VERIFY_ETH_STATE_FUNC_NAME: &IdentStr = ident_str!("verify_new_state");
pub const APPROVE_MESSAGE_FUNC_NAME: &IdentStr = ident_str!("approve_message");

/// Rust version of the Move [`sui_system::eth_dwallet::EthereumDWalletCap`] type.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct EthereumDWalletCap {
    pub id: UID,
    pub dwallet_cap_id: ID,
}

/// Rust version of the Move [`sui_system::ethereum_state::LatestEthereumState`] type.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LatestEthereumStateObject {
    pub id: UID,
    pub eth_state_id: ObjectID,
    pub time_slot: u64,
    pub eth_smart_contract_address: String,
    pub eth_smart_contract_slot: u64,
    pub network: Vec<u8>,
}

/// Rust version of the Move [`sui_system::ethereum_state::EthereumState`] type.
#[derive(Deserialize, Serialize)]
pub struct EthereumStateObject {
    pub id: UID,
    pub data: Vec<u8>,
    pub time_slot: u64,
    pub state_root: Vec<u8>,
    pub latest_ethereum_state_id: ObjectID,
}
