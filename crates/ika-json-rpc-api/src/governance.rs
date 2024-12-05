// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::core::RpcResult;
use jsonrpsee::proc_macros::rpc;

use ika_json_rpc_types::{DelegatedStake, IkaCommittee, ValidatorApys};
use ika_open_rpc_macros::open_rpc;
use ika_types::base_types::{ObjectID, IkaAddress};
use ika_types::ika_serde::BigInt;
use ika_types::ika_system_state::ika_system_state_summary::IkaSystemStateSummary;

#[open_rpc(namespace = "ikax", tag = "Governance Read API")]
#[rpc(server, client, namespace = "ikax")]
pub trait GovernanceReadApi {
    /// Return one or more [DelegatedStake]. If a Stake was withdrawn its status will be Unstaked.
    #[method(name = "getStakesByIds")]
    async fn get_stakes_by_ids(
        &self,
        staked_ika_ids: Vec<ObjectID>,
    ) -> RpcResult<Vec<DelegatedStake>>;

    /// Return all [DelegatedStake].
    #[method(name = "getStakes")]
    async fn get_stakes(&self, owner: IkaAddress) -> RpcResult<Vec<DelegatedStake>>;

    /// Return the committee information for the asked `epoch`.
    #[method(name = "getCommitteeInfo")]
    async fn get_committee_info(
        &self,
        /// The epoch of interest. If None, default to the latest epoch
        epoch: Option<BigInt<u64>>,
    ) -> RpcResult<IkaCommittee>;

    /// Return the latest IKA system state object on-chain.
    #[method(name = "getLatestIkaSystemState")]
    async fn get_latest_ika_system_state(&self) -> RpcResult<IkaSystemStateSummary>;

    /// Return the reference gas price for the network
    #[method(name = "getReferenceGasPrice")]
    async fn get_reference_gas_price(&self) -> RpcResult<BigInt<u64>>;

    /// Return the validator APY
    #[method(name = "getValidatorsApy")]
    async fn get_validators_apy(&self) -> RpcResult<ValidatorApys>;
}
