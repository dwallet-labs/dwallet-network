// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use move_core_types::ident_str;
use move_core_types::identifier::IdentStr;
use move_core_types::language_storage::StructTag;

use crate::balance::Balance;
use crate::base_types::ObjectID;
use crate::committee::EpochId;
use crate::error::IkaError;
use crate::gas_coin::NIKA_PER_IKA;
use crate::id::{ID, UID};
use crate::object::{Data, Object};
use crate::IKA_SYSTEM_ADDRESS;
use serde::Deserialize;
use serde::Serialize;

/// Maximum number of active validators at any moment.
/// We do not allow the number of validators in any epoch to go above this.
pub const MAX_VALIDATOR_COUNT: u64 = 150;

/// Lower-bound on the amount of stake required to become a validator.
///
/// 30 million IKA
pub const MIN_VALIDATOR_JOINING_STAKE_NIKA: u64 = 30_000_000 * NIKA_PER_IKA;

/// Validators with stake amount below `validator_low_stake_threshold` are considered to
/// have low stake and will be escorted out of the validator set after being below this
/// threshold for more than `validator_low_stake_grace_period` number of epochs.
///
/// 20 million IKA
pub const VALIDATOR_LOW_STAKE_THRESHOLD_NIKA: u64 = 20_000_000 * NIKA_PER_IKA;

/// Validators with stake below `validator_very_low_stake_threshold` will be removed
/// immediately at epoch change, no grace period.
///
/// 15 million IKA
pub const VALIDATOR_VERY_LOW_STAKE_THRESHOLD_NIKA: u64 = 15_000_000 * NIKA_PER_IKA;

/// A validator can have stake below `validator_low_stake_threshold`
/// for this many epochs before being kicked out.
pub const VALIDATOR_LOW_STAKE_GRACE_PERIOD: u64 = 7;

pub const STAKING_POOL_MODULE_NAME: &IdentStr = ident_str!("staking_pool");
pub const STAKED_IKA_STRUCT_NAME: &IdentStr = ident_str!("StakedIka");

pub const ADD_STAKE_MUL_COIN_FUN_NAME: &IdentStr = ident_str!("request_add_stake_mul_coin");
pub const ADD_STAKE_FUN_NAME: &IdentStr = ident_str!("request_add_stake");
pub const WITHDRAW_STAKE_FUN_NAME: &IdentStr = ident_str!("request_withdraw_stake");

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct StakedIka {
    id: UID,
    pool_id: ID,
    stake_activation_epoch: u64,
    principal: Balance,
}

impl StakedIka {
    pub fn type_() -> StructTag {
        StructTag {
            address: IKA_SYSTEM_ADDRESS,
            module: STAKING_POOL_MODULE_NAME.to_owned(),
            name: STAKED_IKA_STRUCT_NAME.to_owned(),
            type_params: vec![],
        }
    }

    pub fn is_staked_ika(s: &StructTag) -> bool {
        s.address == IKA_SYSTEM_ADDRESS
            && s.module.as_ident_str() == STAKING_POOL_MODULE_NAME
            && s.name.as_ident_str() == STAKED_IKA_STRUCT_NAME
            && s.type_params.is_empty()
    }

    pub fn id(&self) -> ObjectID {
        self.id.id.bytes
    }

    pub fn pool_id(&self) -> ObjectID {
        self.pool_id.bytes
    }

    pub fn activation_epoch(&self) -> EpochId {
        self.stake_activation_epoch
    }

    pub fn request_epoch(&self) -> EpochId {
        // TODO: this might change when we implement warm up period.
        self.stake_activation_epoch.saturating_sub(1)
    }

    pub fn principal(&self) -> u64 {
        self.principal.value()
    }
}

impl TryFrom<&Object> for StakedIka {
    type Error = IkaError;
    fn try_from(object: &Object) -> Result<Self, Self::Error> {
        match &object.data {
            Data::Move(o) => {
                if o.type_().is_staked_ika() {
                    return bcs::from_bytes(o.contents()).map_err(|err| IkaError::TypeError {
                        error: format!("Unable to deserialize StakedIka object: {:?}", err),
                    });
                }
            }
            Data::Package(_) => {}
        }

        Err(IkaError::TypeError {
            error: format!("Object type is not a StakedIka: {:?}", object),
        })
    }
}
