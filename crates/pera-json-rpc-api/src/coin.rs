// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use jsonrpsee::core::RpcResult;
use jsonrpsee::proc_macros::rpc;
use pera_json_rpc_types::{Balance, CoinPage, PeraCoinMetadata};
use pera_open_rpc_macros::open_rpc;
use pera_types::balance::Supply;
use pera_types::base_types::{ObjectID, PeraAddress};

#[open_rpc(namespace = "perax", tag = "Coin Query API")]
#[rpc(server, client, namespace = "perax")]
pub trait CoinReadApi {
    /// Return all Coin<`coin_type`> objects owned by an address.
    #[method(name = "getCoins")]
    async fn get_coins(
        &self,
        /// the owner's Pera address
        owner: PeraAddress,
        /// optional type name for the coin (e.g., 0x168da5bf1f48dafc111b0a488fa454aca95e0b5e::usdc::USDC), default to 0x2::pera::PERA if not specified.
        coin_type: Option<String>,
        /// optional paging cursor
        cursor: Option<ObjectID>,
        /// maximum number of items per page
        limit: Option<usize>,
    ) -> RpcResult<CoinPage>;

    /// Return all Coin objects owned by an address.
    #[method(name = "getAllCoins")]
    async fn get_all_coins(
        &self,
        /// the owner's Pera address
        owner: PeraAddress,
        /// optional paging cursor
        cursor: Option<ObjectID>,
        /// maximum number of items per page
        limit: Option<usize>,
    ) -> RpcResult<CoinPage>;

    /// Return the total coin balance for one coin type, owned by the address owner.
    #[method(name = "getBalance")]
    async fn get_balance(
        &self,
        /// the owner's Pera address
        owner: PeraAddress,
        /// optional type names for the coin (e.g., 0x168da5bf1f48dafc111b0a488fa454aca95e0b5e::usdc::USDC), default to 0x2::pera::PERA if not specified.
        coin_type: Option<String>,
    ) -> RpcResult<Balance>;

    /// Return the total coin balance for all coin type, owned by the address owner.
    #[method(name = "getAllBalances")]
    async fn get_all_balances(
        &self,
        /// the owner's Pera address
        owner: PeraAddress,
    ) -> RpcResult<Vec<Balance>>;

    /// Return metadata(e.g., symbol, decimals) for a coin
    #[method(name = "getCoinMetadata")]
    async fn get_coin_metadata(
        &self,
        /// type name for the coin (e.g., 0x168da5bf1f48dafc111b0a488fa454aca95e0b5e::usdc::USDC)
        coin_type: String,
    ) -> RpcResult<Option<PeraCoinMetadata>>;

    /// Return total supply for a coin
    #[method(name = "getTotalSupply")]
    async fn get_total_supply(
        &self,
        /// type name for the coin (e.g., 0x168da5bf1f48dafc111b0a488fa454aca95e0b5e::usdc::USDC)
        coin_type: String,
    ) -> RpcResult<Supply>;
}
