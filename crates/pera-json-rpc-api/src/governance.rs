// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use jsonrpsee::core::RpcResult;
use jsonrpsee::proc_macros::rpc;

use pera_json_rpc_types::{DelegatedStake, PeraCommittee, ValidatorApys};
use pera_open_rpc_macros::open_rpc;
use pera_types::base_types::{ObjectID, PeraAddress};
use pera_types::pera_serde::BigInt;
use pera_types::pera_system_state::pera_system_state_summary::PeraSystemStateSummary;

#[open_rpc(namespace = "perax", tag = "Governance Read API")]
#[rpc(server, client, namespace = "perax")]
pub trait GovernanceReadApi {
    /// Return one or more [DelegatedStake]. If a Stake was withdrawn its status will be Unstaked.
    #[method(name = "getStakesByIds")]
    async fn get_stakes_by_ids(
        &self,
        staked_pera_ids: Vec<ObjectID>,
    ) -> RpcResult<Vec<DelegatedStake>>;

    /// Return all [DelegatedStake].
    #[method(name = "getStakes")]
    async fn get_stakes(&self, owner: PeraAddress) -> RpcResult<Vec<DelegatedStake>>;

    /// Return the committee information for the asked `epoch`.
    #[method(name = "getCommitteeInfo")]
    async fn get_committee_info(
        &self,
        /// The epoch of interest. If None, default to the latest epoch
        epoch: Option<BigInt<u64>>,
    ) -> RpcResult<PeraCommittee>;

    /// Return the latest PERA system state object on-chain.
    #[method(name = "getLatestPeraSystemState")]
    async fn get_latest_pera_system_state(&self) -> RpcResult<PeraSystemStateSummary>;

    /// Return the reference gas price for the network
    #[method(name = "getReferenceGasPrice")]
    async fn get_reference_gas_price(&self) -> RpcResult<BigInt<u64>>;

    /// Return the validator APY
    #[method(name = "getValidatorsApy")]
    async fn get_validators_apy(&self) -> RpcResult<ValidatorApys>;
}
