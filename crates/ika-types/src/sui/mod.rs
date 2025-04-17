// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::committee::StakeUnit;
use crate::crypto::AuthorityName;
use crate::sui::system_inner_v1::DWalletCoordinatorInnerV1;
use crate::sui::system_inner_v1::DWalletNetworkDecryptionKeyCap;
use enum_dispatch::enum_dispatch;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::TypeTag;
use move_core_types::{ident_str, identifier::IdentStr, language_storage::StructTag};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sui_types::base_types::ObjectID;
use sui_types::collection_types::TableVec;
use sui_types::storage::ObjectStore;
use sui_types::versioned::Versioned;
use sui_types::MoveTypeTagTrait;
use system_inner_v1::SystemInnerV1;
use system_inner_v1::UpgradeCap;
use validator_inner_v1::ValidatorInnerV1;

pub mod epoch_start_system;
pub mod system_inner_v1;
pub mod validator_inner_v1;

#[cfg(msim)]
use self::simtest_ika_system_state_inner::{
    SimTestIkaSystemStateInnerDeepV2, SimTestIkaSystemStateInnerShallowV2,
    SimTestIkaSystemStateInnerV1, SimTestValidatorDeepV2, SimTestValidatorV1,
};

/// Default computation price of 1000 NIka
pub const DEFAULT_VALIDATOR_COMPUTATION_PRICE: u64 = 1000;
/// Default commission rate of 2%
pub const DEFAULT_COMMISSION_RATE: u16 = 200;

pub const INIT_CAP_STRUCT_NAME: &IdentStr = ident_str!("InitCap");
pub const SYSTEM_STRUCT_NAME: &IdentStr = ident_str!("System");
pub const VALIDATOR_CAP_STRUCT_NAME: &IdentStr = ident_str!("ValidatorCap");
pub const PROTOCOL_CAP_STRUCT_NAME: &IdentStr = ident_str!("ProtocolCap");
pub const DWALLET_COORDINATOR_STRUCT_NAME: &IdentStr = ident_str!("DWalletCoordinator");

pub const SYSTEM_MODULE_NAME: &IdentStr = ident_str!("system");
pub const INIT_MODULE_NAME: &IdentStr = ident_str!("init");
pub const VALIDATOR_CAP_MODULE_NAME: &IdentStr = ident_str!("validator_cap");
pub const PROTOCOL_CAP_MODULE_NAME: &IdentStr = ident_str!("protocol_cap");
pub const DWALLET_2PC_MPC_SECP256K1_MODULE_NAME: &IdentStr =
    ident_str!("dwallet_2pc_mpc_secp256k1");

pub const INITIALIZE_FUNCTION_NAME: &IdentStr = ident_str!("initialize");
pub const REQUEST_ADD_VALIDATOR_CANDIDATE_FUNCTION_NAME: &IdentStr =
    ident_str!("request_add_validator_candidate");
pub const REQUEST_ADD_VALIDATOR_FUNCTION_NAME: &IdentStr = ident_str!("request_add_validator");
pub const REQUEST_ADD_STAKE_FUNCTION_NAME: &IdentStr = ident_str!("request_add_stake");
pub const REQUEST_REMOVE_VALIDATOR_FUNCTION_NAME: &IdentStr =
    ident_str!("request_remove_validator");
pub const PROCESS_CHECKPOINT_MESSAGE_BY_QUORUM_FUNCTION_NAME: &IdentStr =
    ident_str!("process_checkpoint_message_by_quorum");
pub const REQUEST_MID_EPOCH_FUNCTION_NAME: &IdentStr = ident_str!("request_reconfig_mid_epoch");
pub const REQUEST_LOCK_EPOCH_SESSIONS_FUNCTION_NAME: &IdentStr =
    ident_str!("request_lock_epoch_sessions");
pub const REQUEST_ADVANCE_EPOCH_FUNCTION_NAME: &IdentStr = ident_str!("request_advance_epoch");
pub const REQUEST_DWALLET_NETWORK_DECRYPTION_KEY_DKG_BY_CAP_FUNCTION_NAME: &IdentStr =
    ident_str!("request_dwallet_network_decryption_key_dkg_by_cap");

pub const CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_MODULE_NAME: &IdentStr =
    ident_str!("class_groups_public_key_and_proof");
pub const CREATE_CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_BUILDER_FUNCTION_NAME: &IdentStr =
    ident_str!("empty");
pub const ADD_PAIR_TO_CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_FUNCTION_NAME: &IdentStr =
    ident_str!("add_public_key_and_proof");
pub const FINISH_CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_FUNCTION_NAME: &IdentStr = ident_str!("finish");

#[cfg(msim)]
pub const IKA_SYSTEM_STATE_SIM_TEST_V1: u64 = 18446744073709551605; // u64::MAX - 10
#[cfg(msim)]
pub const IKA_SYSTEM_STATE_SIM_TEST_SHALLOW_V2: u64 = 18446744073709551606; // u64::MAX - 9
#[cfg(msim)]
pub const IKA_SYSTEM_STATE_SIM_TEST_DEEP_V2: u64 = 18446744073709551607; // u64::MAX - 8

/// Rust version of the Move ika::ika_system::IkaSystemState type
/// In Rust, this type should be rarely used since it's just a thin
/// wrapper used to access the inner object.
/// Within this module, we use it to determine the current version of the system state inner object type,
/// so that we could deserialize the inner object correctly.
/// Outside of this module, we only use it in testing.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct System {
    pub id: ObjectID,
    pub version: u64,
    pub package_id: ObjectID,
    pub new_package_id: Option<ObjectID>,
}

/// Rust version of the Move DWalletCoordinator type
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DWalletCoordinator {
    pub id: ObjectID,
    pub version: u64,
    pub package_id: ObjectID,
    pub new_package_id: Option<ObjectID>,
}

impl System {
    pub fn type_(ika_system_package_address: AccountAddress) -> StructTag {
        StructTag {
            address: ika_system_package_address,
            name: SYSTEM_STRUCT_NAME.to_owned(),
            module: SYSTEM_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }

    pub fn type_tag(ika_system_package_address: AccountAddress) -> TypeTag {
        TypeTag::Struct(Box::new(Self::type_(ika_system_package_address)))
    }
}

/// This is the standard API that all inner system state object type should implement.
#[enum_dispatch]
pub trait SystemInnerTrait {
    fn epoch(&self) -> u64;
    fn computation_price_per_unit_size(&self) -> u64;
    fn protocol_version(&self) -> u64;
    fn upgrade_caps(&self) -> &Vec<UpgradeCap>;
    fn epoch_start_timestamp_ms(&self) -> u64;
    fn last_processed_checkpoint_sequence_number(&self) -> Option<u64>;
    fn previous_epoch_last_checkpoint_sequence_number(&self) -> u64;
    fn epoch_duration_ms(&self) -> u64;
    fn dwallet_2pc_mpc_secp256k1_id(&self) -> Option<ObjectID>;
    fn dwallet_2pc_mpc_secp256k1_network_decryption_keys(
        &self,
    ) -> &Vec<DWalletNetworkDecryptionKeyCap>;
    fn get_ika_next_epoch_committee(
        &self,
    ) -> Option<HashMap<ObjectID, (AuthorityName, StakeUnit)>>;
}

/// IkaSystemIkaSystemStateInnerState provides an abstraction over multiple versions of the inner IkaSystemStateInner object.
/// This should be the primary interface to the system state object in Rust.
/// We use enum dispatch to dispatch all methods defined in IkaSystemStateTrait to the actual
/// implementation in the inner types.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[enum_dispatch(SystemInnerTrait)]
pub enum SystemInner {
    V1(SystemInnerV1),
}

/// A wrapper around the different versions of the DWalletCoordinator.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum DWalletCoordinatorInner {
    V1(DWalletCoordinatorInnerV1),
}

/// This is the fixed type used by init.
pub type SystemInnerInit = SystemInnerV1;
pub type ValidatorInnerInit = ValidatorInnerV1;

impl SystemInner {
    /// Always return the version that we will be using for init.
    /// Init always uses this version regardless of the current version.
    /// Note that since it's possible for the actual init of the network to diverge from the
    /// init of the latest Rust code, it's important that we only use this for tooling purposes.
    pub fn into_init_version_for_tooling(self) -> SystemInnerInit {
        match self {
            SystemInner::V1(inner) => inner,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Element {
    bytes: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Default)]
pub struct PoolTokenExchangeRate {
    ika_amount: u64,
    pool_token_amount: u64,
}

impl PoolTokenExchangeRate {
    /// Rate of the staking pool, pool token amount : Ika amount
    pub fn rate(&self) -> f64 {
        if self.ika_amount == 0 {
            1_f64
        } else {
            self.pool_token_amount as f64 / self.ika_amount as f64
        }
    }

    pub fn new(ika_amount: u64, pool_token_amount: u64) -> Self {
        Self {
            ika_amount,
            pool_token_amount,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Validator {
    pub id: ObjectID,
    pub inner: Versioned,
}

/// Rust representation of the Move ika::class_groups::ClassGroupsPublicKeyAndProofBuilder type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ClassGroupsPublicKeyAndProofBuilder;

impl ClassGroupsPublicKeyAndProofBuilder {
    /// Return the Move struct tag for this type
    pub fn type_(ika_system_package_address: AccountAddress) -> StructTag {
        StructTag {
            address: ika_system_package_address,
            name: ident_str!("ClassGroupsPublicKeyAndProofBuilder").to_owned(),
            module: CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// Rust version of the Move ika::class_groups::ClassGroupsPublicKeyAndProof type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ClassGroupsPublicKeyAndProof {
    pub id: ObjectID,
    pub public_keys_and_proofs: TableVec,
}

impl ClassGroupsPublicKeyAndProof {
    /// Return the Move struct tag for this type
    pub fn type_(ika_system_package_address: AccountAddress) -> StructTag {
        StructTag {
            address: ika_system_package_address,
            name: ident_str!("ClassGroupsPublicKeyAndProof").to_owned(),
            module: CLASS_GROUPS_PUBLIC_KEY_AND_PROOF_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}
