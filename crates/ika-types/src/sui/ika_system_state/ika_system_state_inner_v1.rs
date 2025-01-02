// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use sui_types::balance::Balance;
use sui_types::coin::TreasuryCap;
use sui_types::base_types::{ObjectID, SuiAddress};
use sui_types::collection_types::{Bag, Table, TableVec, VecMap, VecSet};
use crate::committee::{CommitteeWithNetworkMetadata, NetworkMetadata};
use crate::crypto::{verify_proof_of_possession, AuthorityPublicKey, AuthoritySignature};
use crate::crypto::{AuthorityPublicKeyBytes, NetworkPublicKey};
use sui_types::error::SuiError;
use sui_types::id::ID;
use sui_types::multiaddr::Multiaddr;
use sui_types::storage::ObjectStore;
use anyhow::Result;
use fastcrypto::traits::ToFromBytes;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

use super::epoch_start_ika_system_state::{EpochStartValidatorInfoV1, EpochStartSystemState};
use super::ika_system_state_summary::{IkaSystemStateSummary, IkaValidatorSummary};
use super::{AdvanceEpochParams, IkaSystemStateTrait};

const E_METADATA_INVALID_POP: u64 = 0;
const E_METADATA_INVALID_PUBKEY: u64 = 1;
const E_METADATA_INVALID_NET_PUBKEY: u64 = 2;
const E_METADATA_INVALID_WORKER_PUBKEY: u64 = 3;
const E_METADATA_INVALID_NET_ADDR: u64 = 4;
const E_METADATA_INVALID_P2P_ADDR: u64 = 5;
const E_METADATA_INVALID_PRIMARY_ADDR: u64 = 6;
const E_METADATA_INVALID_WORKER_ADDR: u64 = 7;

/// Rust version of the Move ika::ika_system::SystemParameters type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct SystemParametersV1 {
    /// The duration of an epoch, in milliseconds.
    pub epoch_duration_ms: u64,

    /// The starting epoch in which stake subsidies start being paid out
    pub stake_subsidy_start_epoch: u64,

    /// Minimum number of active validators at any moment.
    pub min_validator_count: u64,

    /// Maximum number of active validators at any moment.
    /// We do not allow the number of validators in any epoch to go above this.
    pub max_validator_count: u64,

    /// Lower-bound on the amount of stake required to become a validator.
    pub min_validator_joining_stake: u64,

    /// Validators with stake amount below `validator_low_stake_threshold` are considered to
    /// have low stake and will be escorted out of the validator set after being below this
    /// threshold for more than `validator_low_stake_grace_period` number of epochs.
    pub validator_low_stake_threshold: u64,

    /// Validators with stake below `validator_very_low_stake_threshold` will be removed
    /// immediately at epoch change, no grace period.
    pub validator_very_low_stake_threshold: u64,

    /// A validator can have stake below `validator_low_stake_threshold`
    /// for this many epochs before being kicked out.
    pub validator_low_stake_grace_period: u64,

    /// how many reward are slashed to punish a validator, in bps.
    pub reward_slashing_rate: u64,

    /// Any extra fields that's not defined statically.
    pub extra_fields: Bag,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Element {
    bytes: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ValidatorMetadataV1 {
    pub sui_address: SuiAddress,
    pub protocol_pubkey_bytes: Vec<u8>,
    pub protocol_pubkey: Element,
    pub network_pubkey_bytes: Vec<u8>,
    pub worker_pubkey_bytes: Vec<u8>,
    pub proof_of_possession_bytes: Vec<u8>,
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub project_url: String,
    pub net_address: String,
    pub p2p_address: String,
    pub primary_address: String,
    pub worker_address: String,
    pub next_epoch_protocol_pubkey_bytes: Option<Vec<u8>>,
    pub next_epoch_proof_of_possession_bytes: Option<Vec<u8>>,
    pub next_epoch_network_pubkey_bytes: Option<Vec<u8>>,
    pub next_epoch_worker_pubkey_bytes: Option<Vec<u8>>,
    pub next_epoch_net_address: Option<String>,
    pub next_epoch_p2p_address: Option<String>,
    pub next_epoch_primary_address: Option<String>,
    pub next_epoch_worker_address: Option<String>,
    pub extra_fields: Bag,
}

#[derive(derive_more::Debug, Clone, Eq, PartialEq)]
pub struct VerifiedValidatorMetadataV1 {
    pub sui_address: SuiAddress,
    pub protocol_pubkey: AuthorityPublicKey,
    pub network_pubkey: NetworkPublicKey,
    pub worker_pubkey: NetworkPublicKey,
    #[debug(ignore)]
    pub proof_of_possession_bytes: Vec<u8>,
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub project_url: String,
    pub net_address: Multiaddr,
    pub p2p_address: Multiaddr,
    pub primary_address: Multiaddr,
    pub worker_address: Multiaddr,
    pub next_epoch_protocol_pubkey: Option<AuthorityPublicKey>,
    pub next_epoch_proof_of_possession_bytes: Option<Vec<u8>>,
    pub next_epoch_network_pubkey: Option<NetworkPublicKey>,
    pub next_epoch_worker_pubkey: Option<NetworkPublicKey>,
    pub next_epoch_net_address: Option<Multiaddr>,
    pub next_epoch_p2p_address: Option<Multiaddr>,
    pub next_epoch_primary_address: Option<Multiaddr>,
    pub next_epoch_worker_address: Option<Multiaddr>,
}

impl VerifiedValidatorMetadataV1 {
    pub fn ika_pubkey_bytes(&self) -> AuthorityPublicKeyBytes {
        (&self.protocol_pubkey).into()
    }
}

impl ValidatorMetadataV1 {
    /// Verify validator metadata and return a verified version (on success) or error code (on failure)
    pub fn verify(&self) -> Result<VerifiedValidatorMetadataV1, u64> {
        let protocol_pubkey = AuthorityPublicKey::from_bytes(self.protocol_pubkey_bytes.as_ref())
            .map_err(|_| E_METADATA_INVALID_PUBKEY)?;

        // Verify proof of possession for the protocol key
        let pop = AuthoritySignature::from_bytes(self.proof_of_possession_bytes.as_ref())
            .map_err(|_| E_METADATA_INVALID_POP)?;
        verify_proof_of_possession(&pop, &protocol_pubkey, self.sui_address)
            .map_err(|_| E_METADATA_INVALID_POP)?;

        let network_pubkey = NetworkPublicKey::from_bytes(self.network_pubkey_bytes.as_ref())
            .map_err(|_| E_METADATA_INVALID_NET_PUBKEY)?;
        let worker_pubkey = NetworkPublicKey::from_bytes(self.worker_pubkey_bytes.as_ref())
            .map_err(|_| E_METADATA_INVALID_WORKER_PUBKEY)?;
        if worker_pubkey == network_pubkey {
            return Err(E_METADATA_INVALID_WORKER_PUBKEY);
        }

        let net_address = Multiaddr::try_from(self.net_address.clone())
            .map_err(|_| E_METADATA_INVALID_NET_ADDR)?;

        // Ensure p2p, primary, and worker addresses are both Multiaddr's and valid anemo addresses
        let p2p_address = Multiaddr::try_from(self.p2p_address.clone())
            .map_err(|_| E_METADATA_INVALID_P2P_ADDR)?;
        p2p_address
            .to_anemo_address()
            .map_err(|_| E_METADATA_INVALID_P2P_ADDR)?;

        let primary_address = Multiaddr::try_from(self.primary_address.clone())
            .map_err(|_| E_METADATA_INVALID_PRIMARY_ADDR)?;
        primary_address
            .to_anemo_address()
            .map_err(|_| E_METADATA_INVALID_PRIMARY_ADDR)?;

        let worker_address = Multiaddr::try_from(self.worker_address.clone())
            .map_err(|_| E_METADATA_INVALID_WORKER_ADDR)?;
        worker_address
            .to_anemo_address()
            .map_err(|_| E_METADATA_INVALID_WORKER_ADDR)?;

        let next_epoch_protocol_pubkey = match self.next_epoch_protocol_pubkey_bytes.clone() {
            None => Ok::<Option<AuthorityPublicKey>, u64>(None),
            Some(bytes) => Ok(Some(
                AuthorityPublicKey::from_bytes(bytes.as_ref())
                    .map_err(|_| E_METADATA_INVALID_PUBKEY)?,
            )),
        }?;

        let next_epoch_pop = match self.next_epoch_proof_of_possession_bytes.clone() {
            None => Ok::<Option<AuthoritySignature>, u64>(None),
            Some(bytes) => Ok(Some(
                AuthoritySignature::from_bytes(bytes.as_ref())
                    .map_err(|_| E_METADATA_INVALID_POP)?,
            )),
        }?;
        // Verify proof of possession for the next epoch protocol key
        if let Some(ref next_epoch_protocol_pubkey) = next_epoch_protocol_pubkey {
            match next_epoch_pop {
                Some(next_epoch_pop) => {
                    verify_proof_of_possession(
                        &next_epoch_pop,
                        next_epoch_protocol_pubkey,
                        self.sui_address,
                    )
                    .map_err(|_| E_METADATA_INVALID_POP)?;
                }
                None => {
                    return Err(E_METADATA_INVALID_POP);
                }
            }
        }

        let next_epoch_network_pubkey = match self.next_epoch_network_pubkey_bytes.clone() {
            None => Ok::<Option<NetworkPublicKey>, u64>(None),
            Some(bytes) => Ok(Some(
                NetworkPublicKey::from_bytes(bytes.as_ref())
                    .map_err(|_| E_METADATA_INVALID_NET_PUBKEY)?,
            )),
        }?;

        let next_epoch_worker_pubkey: Option<NetworkPublicKey> =
            match self.next_epoch_worker_pubkey_bytes.clone() {
                None => Ok::<Option<NetworkPublicKey>, u64>(None),
                Some(bytes) => Ok(Some(
                    NetworkPublicKey::from_bytes(bytes.as_ref())
                        .map_err(|_| E_METADATA_INVALID_WORKER_PUBKEY)?,
                )),
            }?;
        if next_epoch_network_pubkey.is_some()
            && next_epoch_network_pubkey == next_epoch_worker_pubkey
        {
            return Err(E_METADATA_INVALID_WORKER_PUBKEY);
        }

        let next_epoch_net_address = match self.next_epoch_net_address.clone() {
            None => Ok::<Option<Multiaddr>, u64>(None),
            Some(address) => Ok(Some(
                Multiaddr::try_from(address).map_err(|_| E_METADATA_INVALID_NET_ADDR)?,
            )),
        }?;

        let next_epoch_p2p_address = match self.next_epoch_p2p_address.clone() {
            None => Ok::<Option<Multiaddr>, u64>(None),
            Some(address) => {
                let address =
                    Multiaddr::try_from(address).map_err(|_| E_METADATA_INVALID_P2P_ADDR)?;
                address
                    .to_anemo_address()
                    .map_err(|_| E_METADATA_INVALID_P2P_ADDR)?;

                Ok(Some(address))
            }
        }?;

        let next_epoch_primary_address = match self.next_epoch_primary_address.clone() {
            None => Ok::<Option<Multiaddr>, u64>(None),
            Some(address) => {
                let address =
                    Multiaddr::try_from(address).map_err(|_| E_METADATA_INVALID_PRIMARY_ADDR)?;
                address
                    .to_anemo_address()
                    .map_err(|_| E_METADATA_INVALID_PRIMARY_ADDR)?;

                Ok(Some(address))
            }
        }?;

        let next_epoch_worker_address = match self.next_epoch_worker_address.clone() {
            None => Ok::<Option<Multiaddr>, u64>(None),
            Some(address) => {
                let address =
                    Multiaddr::try_from(address).map_err(|_| E_METADATA_INVALID_WORKER_ADDR)?;
                address
                    .to_anemo_address()
                    .map_err(|_| E_METADATA_INVALID_WORKER_ADDR)?;

                Ok(Some(address))
            }
        }?;

        Ok(VerifiedValidatorMetadataV1 {
            sui_address: self.sui_address,
            protocol_pubkey,
            network_pubkey,
            worker_pubkey,
            proof_of_possession_bytes: self.proof_of_possession_bytes.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            image_url: self.image_url.clone(),
            project_url: self.project_url.clone(),
            net_address,
            p2p_address,
            primary_address,
            worker_address,
            next_epoch_protocol_pubkey,
            next_epoch_proof_of_possession_bytes: self.next_epoch_proof_of_possession_bytes.clone(),
            next_epoch_network_pubkey,
            next_epoch_worker_pubkey,
            next_epoch_net_address,
            next_epoch_p2p_address,
            next_epoch_primary_address,
            next_epoch_worker_address,
        })
    }
}

/// Rust version of the Move ika::validator::Validator type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ValidatorV1 {
    metadata: ValidatorMetadataV1,
    #[serde(skip)]
    verified_metadata: OnceCell<VerifiedValidatorMetadataV1>,

    pub voting_power: u64,
    pub operation_cap_id: ID,
    pub computation_price: u64,
    pub staking_pool: StakingPoolV1,
    pub commission_rate: u64,
    pub next_epoch_stake: u64,
    pub next_epoch_computation_price: u64,
    pub next_epoch_commission_rate: u64,
    pub extra_fields: Bag,
}

impl ValidatorV1 {
    pub fn verified_metadata(&self) -> &VerifiedValidatorMetadataV1 {
        self.verified_metadata.get_or_init(|| {
            self.metadata
                .verify()
                .expect("Validity of metadata should be verified on-chain")
        })
    }

    pub fn into_ika_validator_summary(self) -> IkaValidatorSummary {
        let Self {
            metadata:
                ValidatorMetadataV1 {
                    sui_address,
                    protocol_pubkey_bytes,
                    protocol_pubkey: _,
                    network_pubkey_bytes,
                    worker_pubkey_bytes,
                    proof_of_possession_bytes,
                    name,
                    description,
                    image_url,
                    project_url,
                    net_address,
                    p2p_address,
                    primary_address,
                    worker_address,
                    next_epoch_protocol_pubkey_bytes,
                    next_epoch_proof_of_possession_bytes,
                    next_epoch_network_pubkey_bytes,
                    next_epoch_worker_pubkey_bytes,
                    next_epoch_net_address,
                    next_epoch_p2p_address,
                    next_epoch_primary_address,
                    next_epoch_worker_address,
                    extra_fields: _,
                },
            verified_metadata: _,
            voting_power,
            operation_cap_id,
            computation_price,
            staking_pool:
                StakingPoolV1 {
                    id: staking_pool_id,
                    activation_epoch: staking_pool_activation_epoch,
                    deactivation_epoch: staking_pool_deactivation_epoch,
                    ika_balance: staking_pool_ika_balance,
                    rewards_pool,
                    pool_token_balance,
                    exchange_rates:
                        Table {
                            id: exchange_rates_id,
                            size: exchange_rates_size,
                        },
                    pending_stake,
                    pending_total_ika_withdraw,
                    pending_pool_token_withdraw,
                    extra_fields: _,
                },
            commission_rate,
            next_epoch_stake,
            next_epoch_computation_price,
            next_epoch_commission_rate,
            extra_fields: _,
        } = self;
        IkaValidatorSummary {
            sui_address,
            protocol_pubkey_bytes,
            network_pubkey_bytes,
            worker_pubkey_bytes,
            proof_of_possession_bytes,
            name,
            description,
            image_url,
            project_url,
            net_address,
            p2p_address,
            primary_address,
            worker_address,
            next_epoch_protocol_pubkey_bytes,
            next_epoch_proof_of_possession_bytes,
            next_epoch_network_pubkey_bytes,
            next_epoch_worker_pubkey_bytes,
            next_epoch_net_address,
            next_epoch_p2p_address,
            next_epoch_primary_address,
            next_epoch_worker_address,
            voting_power,
            operation_cap_id: operation_cap_id.bytes,
            computation_price,
            staking_pool_id,
            staking_pool_activation_epoch,
            staking_pool_deactivation_epoch,
            staking_pool_ika_balance,
            rewards_pool: rewards_pool.value(),
            pool_token_balance,
            exchange_rates_id,
            exchange_rates_size,
            pending_stake,
            pending_total_ika_withdraw,
            pending_pool_token_withdraw,
            commission_rate,
            next_epoch_stake,
            next_epoch_computation_price,
            next_epoch_commission_rate,
        }
    }
}

/// Rust version of the Move ika_system::staking_pool::StakingPool type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct StakingPoolV1 {
    pub id: ObjectID,
    pub activation_epoch: Option<u64>,
    pub deactivation_epoch: Option<u64>,
    pub ika_balance: u64,
    pub rewards_pool: Balance,
    pub pool_token_balance: u64,
    pub exchange_rates: Table,
    pub pending_stake: u64,
    pub pending_total_ika_withdraw: u64,
    pub pending_pool_token_withdraw: u64,
    pub extra_fields: Bag,
}

/// Rust version of the Move ika_system::validator_set::ValidatorSet type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ValidatorSetV1 {
    pub total_stake: u64,
    pub active_validators: Vec<ValidatorV1>,
    pub pending_active_validators: TableVec,
    pub pending_removals: Vec<u64>,
    pub staking_pool_mappings: Table,
    pub inactive_validators: Table,
    pub validator_candidates: Table,
    pub at_risk_validators: VecMap<SuiAddress, u64>,
    pub extra_fields: Bag,
}

/// Rust version of the Move ika_system::storage_fund::StorageFund type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct StorageFundV1 {
    pub total_object_storage_rebates: Balance,
    pub non_refundable_balance: Balance,
}

/// Rust version of the Move ika_system::ika_system::IkaSystemStateInner type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct IkaSystemStateInnerV1 {
    pub epoch: u64,
    pub protocol_version: u64,
    pub system_state_version: u64,
    pub validators: ValidatorSetV1,
    pub storage_fund: StorageFundV1,
    pub parameters: SystemParametersV1,
    pub computation_price_per_unit_size: u64,
    pub validator_report_records: VecMap<SuiAddress, VecSet<SuiAddress>>,
    pub ika_treasury: IkaTreasuryV1,
    pub safe_mode: bool,
    pub safe_mode_storage_rewards: Balance,
    pub safe_mode_computation_rewards: Balance,
    pub safe_mode_storage_rebates: u64,
    pub safe_mode_non_refundable_storage_fee: u64,
    pub epoch_start_timestamp_ms: u64,
    pub total_actions_submitted: u64,
    pub last_processed_checkpoint_sequence_number: Option<u64>,
    pub previous_epoch_last_checkpoint_sequence_number: u64,
    pub computation_reward: Balance,
    pub extra_fields: Bag,
    // TODO: Use getters instead of all pub.
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct IkaTreasuryV1 {
    /// TreasuryCap of IKA tokens.
    pub treasury_cap: TreasuryCap,

    /// Count of the number of times stake subsidies have been distributed.
    pub stake_subsidy_distribution_counter: u64,

    /// The rate at which the amount per distribution is calculated based on
    /// period nad total supply. Expressed in basis points.
    pub stake_subsidy_rate: u16,

    /// The amount of stake subsidy to be distrabtured per distribution.
    /// This amount changes based on `stake_subsidy_rate`.
    pub stake_subsidy_amount_per_distribution: u64,

    /// Number of distributions to occur before the amount per distribution will be recalculated.
    pub stake_subsidy_period_length: u64,

    pub extra_fields: Bag,
}

impl IkaSystemStateTrait for IkaSystemStateInnerV1 {
    fn epoch(&self) -> u64 {
        self.epoch
    }

    fn computation_price_per_unit_size(&self) -> u64 {
        self.computation_price_per_unit_size
    }

    fn protocol_version(&self) -> u64 {
        self.protocol_version
    }

    fn system_state_version(&self) -> u64 {
        self.system_state_version
    }

    fn epoch_start_timestamp_ms(&self) -> u64 {
        self.epoch_start_timestamp_ms
    }

    fn last_processed_checkpoint_sequence_number(&self) -> Option<u64> {
        self.last_processed_checkpoint_sequence_number
    }

    fn previous_epoch_last_checkpoint_sequence_number(&self) -> u64 {
        self.previous_epoch_last_checkpoint_sequence_number
    }

    fn epoch_duration_ms(&self) -> u64 {
        self.parameters.epoch_duration_ms
    }

    fn safe_mode(&self) -> bool {
        self.safe_mode
    }

    fn advance_epoch_safe_mode(&mut self, params: &AdvanceEpochParams) {
        self.epoch = params.epoch;
        self.safe_mode = true;
        self.safe_mode_storage_rewards
            .deposit_for_safe_mode(params.storage_charge);
        self.safe_mode_storage_rebates += params.storage_rebate;
        self.safe_mode_computation_rewards
            .deposit_for_safe_mode(params.computation_charge);
        self.safe_mode_non_refundable_storage_fee += params.non_refundable_storage_fee;
        self.epoch_start_timestamp_ms = params.epoch_start_timestamp_ms;
        self.protocol_version = params.next_protocol_version.as_u64();
    }

    fn get_current_epoch_committee(&self) -> CommitteeWithNetworkMetadata {
        let validators = self
            .validators
            .active_validators
            .iter()
            .map(|validator| {
                let verified_metadata = validator.verified_metadata();
                let name = verified_metadata.ika_pubkey_bytes();
                (
                    name,
                    (
                        validator.voting_power,
                        NetworkMetadata {
                            network_address: verified_metadata.net_address.clone(),
                            narwhal_primary_address: verified_metadata.primary_address.clone(),
                            network_public_key: Some(verified_metadata.network_pubkey.clone()),
                        },
                    ),
                )
            })
            .collect();
        CommitteeWithNetworkMetadata::new(self.epoch, validators)
    }

    fn get_pending_active_validators<S: ObjectStore + ?Sized>(
        &self,
        object_store: &S,
    ) -> Result<Vec<IkaValidatorSummary>, SuiError> {
        // let table_id = self.validators.pending_active_validators.contents.id;
        // let table_size = self.validators.pending_active_validators.contents.size;
        // let validators: Vec<ValidatorV1> =
        //     get_validators_from_table_vec(object_store, table_id, table_size)?;
        // Ok(validators
        //     .into_iter()
        //     .map(|v| v.into_ika_validator_summary())
        //     .collect())
        Ok(vec![])
    }

    fn into_epoch_start_state(self) -> EpochStartSystemState {
        EpochStartSystemState::new_v1(
            self.epoch,
            self.protocol_version,
            self.computation_price_per_unit_size,
            self.safe_mode,
            self.epoch_start_timestamp_ms,
            self.parameters.epoch_duration_ms,
            self.validators
                .active_validators
                .iter()
                .map(|validator| {
                    let metadata = validator.verified_metadata();
                    EpochStartValidatorInfoV1 {
                        sui_address: metadata.sui_address,
                        protocol_pubkey: metadata.protocol_pubkey.clone(),
                        narwhal_network_pubkey: metadata.network_pubkey.clone(),
                        narwhal_worker_pubkey: metadata.worker_pubkey.clone(),
                        ika_net_address: metadata.net_address.clone(),
                        p2p_address: metadata.p2p_address.clone(),
                        narwhal_primary_address: metadata.primary_address.clone(),
                        narwhal_worker_address: metadata.worker_address.clone(),
                        voting_power: validator.voting_power,
                        hostname: metadata.name.clone(),
                    }
                })
                .collect(),
        )
    }

    fn into_ika_system_state_summary(self) -> IkaSystemStateSummary {
        // If you are making any changes to IkaSystemStateV1 or any of its dependent types before
        // mainnet, please also update IkaSystemStateSummary and its corresponding TS type.
        // Post-mainnet, we will need to introduce a new version.
        let Self {
            epoch,
            protocol_version,
            system_state_version,
            validators:
                ValidatorSetV1 {
                    total_stake,
                    active_validators,
                    pending_active_validators:
                        TableVec {
                            contents:
                                Table {
                                    id: pending_active_validators_id,
                                    size: pending_active_validators_size,
                                },
                        },
                    pending_removals,
                    staking_pool_mappings:
                        Table {
                            id: staking_pool_mappings_id,
                            size: staking_pool_mappings_size,
                        },
                    inactive_validators:
                        Table {
                            id: inactive_pools_id,
                            size: inactive_pools_size,
                        },
                    validator_candidates:
                        Table {
                            id: validator_candidates_id,
                            size: validator_candidates_size,
                        },
                    at_risk_validators:
                        VecMap {
                            contents: at_risk_validators,
                        },
                    extra_fields: _,
                },
            storage_fund,
            parameters:
                SystemParametersV1 {
                    stake_subsidy_start_epoch,
                    epoch_duration_ms,
                    min_validator_count,
                    max_validator_count,
                    min_validator_joining_stake,
                    validator_low_stake_threshold,
                    validator_very_low_stake_threshold,
                    validator_low_stake_grace_period,
                    reward_slashing_rate: _,
                    extra_fields: _,
                },
            computation_price_per_unit_size,
            validator_report_records:
                VecMap {
                    contents: validator_report_records,
                },
            ika_treasury:
                IkaTreasuryV1 {
                    treasury_cap,
                    stake_subsidy_rate,
                    stake_subsidy_distribution_counter,
                    stake_subsidy_amount_per_distribution,
                    stake_subsidy_period_length,
                    extra_fields: _,
                },
            safe_mode,
            safe_mode_storage_rewards,
            safe_mode_computation_rewards,
            safe_mode_storage_rebates,
            safe_mode_non_refundable_storage_fee,
            epoch_start_timestamp_ms,
            total_actions_submitted: _,
            last_processed_checkpoint_sequence_number: _,
            previous_epoch_last_checkpoint_sequence_number: _,
            computation_reward: _,
            extra_fields: _,
        } = self;
        IkaSystemStateSummary {
            epoch,
            protocol_version,
            system_state_version,
            storage_fund_total_object_storage_rebates: storage_fund
                .total_object_storage_rebates
                .value(),
            storage_fund_non_refundable_balance: storage_fund.non_refundable_balance.value(),
            computation_price_per_unit_size,
            safe_mode,
            safe_mode_storage_rewards: safe_mode_storage_rewards.value(),
            safe_mode_computation_rewards: safe_mode_computation_rewards.value(),
            safe_mode_storage_rebates,
            safe_mode_non_refundable_storage_fee,
            epoch_start_timestamp_ms,
            stake_subsidy_start_epoch,
            epoch_duration_ms,
            stake_subsidy_distribution_counter,
            ika_treasury_total_supply: treasury_cap.total_supply.value,
            stake_subsidy_amount_per_distribution,
            total_stake,
            active_validators: active_validators
                .into_iter()
                .map(|v| v.into_ika_validator_summary())
                .collect(),
            pending_active_validators_id,
            pending_active_validators_size,
            pending_removals,
            staking_pool_mappings_id,
            staking_pool_mappings_size,
            inactive_pools_id,
            inactive_pools_size,
            validator_candidates_id,
            validator_candidates_size,
            at_risk_validators: at_risk_validators
                .into_iter()
                .map(|e| (e.key, e.value))
                .collect(),
            validator_report_records: validator_report_records
                .into_iter()
                .map(|e| (e.key, e.value.contents))
                .collect(),
            min_validator_count,
            max_validator_count,
            min_validator_joining_stake,
            validator_low_stake_threshold,
            validator_very_low_stake_threshold,
            validator_low_stake_grace_period,
            stake_subsidy_period_length,
            stake_subsidy_rate,
        }
    }
}

/// Rust version of the Move ika_system::validator_cap::UnverifiedValidatorOperationCap type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct UnverifiedValidatorOperationCapV1 {
    pub id: ObjectID,
    pub authorizer_validator_address: SuiAddress,
}
