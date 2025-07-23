// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use anyhow::Result;
use std::{
    fmt,
    fmt::{Debug, Display, Formatter, Write},
    path::PathBuf,
};
use std::fs::File;
use std::io::BufReader;
use clap::Subcommand;
use colored::Colorize;
// use hex;
use ika_config::{IKA_SUI_CONFIG, ika_config_dir};
use ika_sui_client::ika_validator_transactions::{
    set_approved_upgrade_by_cap, set_paused_curves_and_signature_algorithms,
    set_supported_and_pricing,
};
use ika_types::messages_dwallet_mpc::IkaPackagesConfig;
use sui_config::PersistedConfig;
use sui_sdk::rpc_types::SuiTransactionBlockResponse;
use sui_sdk::wallet_context::WalletContext;
use sui_types::base_types::ObjectID;
use sui_types::collection_types::Entry;
use ika_types::sui::{PricingInfoKey, PricingInfoValue};

const DEFAULT_GAS_BUDGET: u64 = 200_000_000;

#[derive(Subcommand)]
pub enum IkaProtocolCommand {
    // #[clap(name = "set-approved-upgrade-by-cap")]
    // SetApprovedUpgradeByCap {
    //     #[clap(name = "gas-budget", long)]
    //     gas_budget: Option<u64>,
    //     #[clap(name = "protocol-cap-id", long)]
    //     protocol_cap_id: ObjectID,
    //     #[clap(name = "package-id", long)]
    //     package_id: ObjectID,
    //     #[clap(name = "digest", long)]
    //     digest: Option<String>,
    //     #[clap(name = "ika-sui-config", long)]
    //     ika_sui_config: Option<PathBuf>,
    // },
    #[clap(name = "set-paused-curves-and-signature-algorithms")]
    SetPausedCurvesAndSignatureAlgorithms {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "protocol-cap-id", long)]
        system_object_cap_id: ObjectID,
        #[clap(name = "paused-curves", long)]
        paused_curves: Vec<u32>,
        #[clap(name = "paused-signature-algorithms", long)]
        paused_signature_algorithms: Vec<u32>,
        #[clap(name = "paused-hash-schemes", long)]
        paused_hash_schemes: Vec<u32>,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
    #[clap(name = "set-supported-and-pricing")]
    SetSupportedAndPricing {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "protocol-cap-id", long)]
        system_object_cap_id: ObjectID,
        #[clap(name = "default-pricing", long)]
        default_pricing_yaml: PathBuf,
        #[clap(
            name = "supported-curves-to-signature-algorithms-to-hash-schemes",
            long
        )]
        supported_curves_to_signature_algorithms_to_hash_schemes: PathBuf,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
}

pub enum IkaProtocolCommandResponse {
    SetApprovedUpgradeByCap(SuiTransactionBlockResponse),
    SetPausedCurvesAndSignatureAlgorithms(SuiTransactionBlockResponse),
    SetSupportedAndPricing(SuiTransactionBlockResponse),
}

impl IkaProtocolCommand {
    pub async fn execute(
        self,
        context: &mut WalletContext,
    ) -> Result<IkaProtocolCommandResponse, anyhow::Error> {
        let response = match self {
            // IkaProtocolCommand::SetApprovedUpgradeByCap {
            //     gas_budget,
            //     protocol_cap_id,
            //     package_id,
            //     digest,
            //     ika_sui_config,
            // } => {
            //     let gas_budget = gas_budget.unwrap_or(DEFAULT_GAS_BUDGET);
            //    let config_path = ika_sui_config.unwrap_or(ika_config_dir()?.join(IKA_SUI_CONFIG));
            //                 let config: IkaPackagesConfig =
            //                     PersistedConfig::read(&config_path).map_err(|err| {
            //                         err.context(format!(
            //                             "Cannot open Ika network config file at {config_path:?}"
            //                         ))
            //                     })?;
            //
            //     let digest_bytes = digest.map(|d| hex::decode(d).unwrap_or_default());
            //
            //     let response = set_approved_upgrade_by_cap(
            //         context,
            //         config.ika_system_package_id,
            //         config.ika_system_object_id,
            //         protocol_cap_id,
            //         package_id,
            //         digest_bytes,
            //         gas_budget,
            //     )
            //     .await?;
            //     IkaProtocolCommandResponse::SetApprovedUpgradeByCap(response)
            // }
            IkaProtocolCommand::SetPausedCurvesAndSignatureAlgorithms {
                gas_budget,
                system_object_cap_id,
                paused_curves,
                paused_signature_algorithms,
                paused_hash_schemes,
                ika_sui_config,
            } => {
                let gas_budget = gas_budget.unwrap_or(DEFAULT_GAS_BUDGET);
                let config_path = ika_sui_config.unwrap_or(ika_config_dir()?.join(IKA_SUI_CONFIG));
                let config: IkaPackagesConfig =
                    PersistedConfig::read(&config_path).map_err(|err| {
                        err.context(format!(
                            "Cannot open Ika network config file at {config_path:?}"
                        ))
                    })?;

                let response = set_paused_curves_and_signature_algorithms(
                    context,
                    config.ika_dwallet_2pc_mpc_package_id,
                    config.ika_dwallet_coordinator_object_id,
                    config.ika_common_package_id,
                    system_object_cap_id,
                    paused_curves,
                    paused_signature_algorithms,
                    paused_hash_schemes,
                    gas_budget,
                )
                .await?;
                IkaProtocolCommandResponse::SetPausedCurvesAndSignatureAlgorithms(response)
            }
            IkaProtocolCommand::SetSupportedAndPricing {
                gas_budget,
                system_object_cap_id,
                default_pricing_yaml,
                supported_curves_to_signature_algorithms_to_hash_schemes,
                ika_sui_config,
            } => {
                let gas_budget = gas_budget.unwrap_or(DEFAULT_GAS_BUDGET);
                let config_path = ika_sui_config.unwrap_or(ika_config_dir()?.join(IKA_SUI_CONFIG));
                let config: IkaPackagesConfig =
                    PersistedConfig::read(&config_path).map_err(|err| {
                        err.context(format!(
                            "Cannot open Ika network config file at {config_path:?}"
                        ))
                    })?;

                let default_pricing_yaml: Vec<Entry<PricingInfoKey, PricingInfoValue>> =
                    serde_yaml::from_reader(BufReader::new(File::open(default_pricing_yaml)?))?;

                let response = set_supported_and_pricing(
                    context,
                    config.ika_dwallet_2pc_mpc_package_id,
                    config.ika_dwallet_coordinator_object_id,
                    config.ika_common_package_id,
                    system_object_cap_id,
                    default_pricing_yaml,
                    false,
                    gas_budget,
                )
                .await?;
                IkaProtocolCommandResponse::SetSupportedAndPricing(response)
            }
        };
        Ok(response)
    }
}

impl Display for IkaProtocolCommandResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();
        match self {
            IkaProtocolCommandResponse::SetApprovedUpgradeByCap(response)
            | IkaProtocolCommandResponse::SetPausedCurvesAndSignatureAlgorithms(response)
            | IkaProtocolCommandResponse::SetSupportedAndPricing(response) => {
                write!(
                    writer,
                    "{}",
                    write_transaction_response_without_transaction_data(response)?
                )?;
            }
        }
        write!(f, "{}", writer.trim_end_matches('\n'))
    }
}

impl Debug for IkaProtocolCommandResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl IkaProtocolCommandResponse {
    pub fn print(&self, pretty: bool) {
        if pretty {
            println!("{self}");
        } else {
            println!("{self:?}");
        }
    }
}

fn write_transaction_response_without_transaction_data(
    response: &SuiTransactionBlockResponse,
) -> Result<String, fmt::Error> {
    // we requested with for full_content, so the following content should be available.
    let success = response.status_ok().unwrap();
    let lines = vec![
        String::from("----- Transaction Digest ----"),
        response.digest.to_string(),
        String::from("----- Transaction Effects ----"),
        response.effects.as_ref().unwrap().to_string(),
    ];
    let mut writer = String::new();
    for line in lines {
        let colorized_line = if success { line.green() } else { line.red() };
        writeln!(writer, "{}", colorized_line)?;
    }
    Ok(writer)
}
