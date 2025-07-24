// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use ika_types::messages_dwallet_mpc::{IkaPackagesConfig, IkaPackagesConfigFile};
use std::path::PathBuf;
use sui_config::PersistedConfig;
use sui_sdk::wallet_context::WalletContext;

#[macro_use]
pub mod ika_commands;
#[cfg(feature = "protocol-commands")]
pub(crate) mod protocol_commands;
pub(crate) mod validator_commands;

pub(crate) fn read_ika_sui_config_yaml(
    context: &WalletContext,
    config_path: &PathBuf,
) -> Result<IkaPackagesConfig, anyhow::Error> {
    let config: IkaPackagesConfigFile = PersistedConfig::read(&config_path).map_err(|err| {
        err.context(format!(
            "Cannot open Ika network config file at {config_path:?}"
        ))
    })?;
    let sui_env = context.get_active_env()?.alias.clone();
    let config = config
        .0
        .get(&sui_env)
        .ok_or_else(|| {
            anyhow::anyhow!("Ika network config not found for Sui environment: {sui_env}")
        })?
        .clone();
    Ok(config)
}
