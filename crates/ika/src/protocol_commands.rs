// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use anyhow::Result;
use std::{
    fmt,
    fmt::{Debug, Display, Formatter, Write},
    path::PathBuf,
};

use clap::Subcommand;
use colored::Colorize;
// use hex;
use ika_config::{IKA_SUI_CONFIG, ika_config_dir};
use ika_sui_client::ika_validator_transactions::{
    calculate_pricing_votes, set_approved_upgrade_by_cap,
    set_or_remove_witness_approving_advance_epoch_by_cap,
    set_paused_curves_and_signature_algorithms, set_supported_and_pricing,
};
use ika_types::messages_dwallet_mpc::IkaPackagesConfig;
use sui_config::PersistedConfig;
use sui_sdk::rpc_types::SuiTransactionBlockResponse;
use sui_sdk::wallet_context::WalletContext;
use sui_types::base_types::ObjectID;

const DEFAULT_GAS_BUDGET: u64 = 200_000_000;

#[derive(Subcommand)]
pub enum IkaProtocolCommand {
    #[clap(name = "set-or-remove-witness-approving-advance-epoch-by-cap")]
    SetOrRemoveWitnessApprovingAdvanceEpochByCap {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "protocol-cap-id", long)]
        protocol_cap_id: ObjectID,
        #[clap(name = "witness-type", long)]
        witness_type: String,
        #[clap(name = "remove", long)]
        remove: bool,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
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
    #[clap(name = "calculate-pricing-votes")]
    CalculatePricingVotes {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "curve", long)]
        curve: u32,
        #[clap(name = "signature-algorithm", long)]
        signature_algorithm: Option<u32>,
        #[clap(name = "protocol", long)]
        protocol: u32,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
    #[clap(name = "set-paused-curves-and-signature-algorithms")]
    SetPausedCurvesAndSignatureAlgorithms {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "protocol-cap-id", long)]
        protocol_cap_id: ObjectID,
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
        protocol_cap_id: ObjectID,
        #[clap(name = "default-pricing", long)]
        default_pricing: String,
        #[clap(
            name = "supported-curves-to-signature-algorithms-to-hash-schemes",
            long
        )]
        supported_curves_to_signature_algorithms_to_hash_schemes: String,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
}

pub enum IkaProtocolCommandResponse {
    SetOrRemoveWitnessApprovingAdvanceEpochByCap(SuiTransactionBlockResponse),
    SetApprovedUpgradeByCap(SuiTransactionBlockResponse),
    CalculatePricingVotes(SuiTransactionBlockResponse),
    SetPausedCurvesAndSignatureAlgorithms(SuiTransactionBlockResponse),
    SetSupportedAndPricing(SuiTransactionBlockResponse),
}

impl IkaProtocolCommand {
    pub async fn execute(
        self,
        context: &mut WalletContext,
    ) -> Result<IkaProtocolCommandResponse, anyhow::Error> {
        let response = match self {
            IkaProtocolCommand::SetOrRemoveWitnessApprovingAdvanceEpochByCap {
                gas_budget,
                protocol_cap_id,
                witness_type,
                remove,
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

                let response = set_or_remove_witness_approving_advance_epoch_by_cap(
                    context,
                    config.ika_system_package_id,
                    config.ika_system_object_id,
                    protocol_cap_id,
                    witness_type,
                    remove,
                    gas_budget,
                )
                .await?;
                IkaProtocolCommandResponse::SetOrRemoveWitnessApprovingAdvanceEpochByCap(response)
            }
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
            IkaProtocolCommand::CalculatePricingVotes {
                gas_budget,
                curve,
                signature_algorithm,
                protocol,
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

                let response = calculate_pricing_votes(
                    context,
                    config.ika_dwallet_2pc_mpc_package_id,
                    config.ika_dwallet_coordinator_object_id,
                    curve,
                    signature_algorithm,
                    protocol,
                    gas_budget,
                )
                .await?;
                IkaProtocolCommandResponse::CalculatePricingVotes(response)
            }
            IkaProtocolCommand::SetPausedCurvesAndSignatureAlgorithms {
                gas_budget,
                protocol_cap_id,
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
                    protocol_cap_id,
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
                protocol_cap_id,
                default_pricing,
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

                let response = set_supported_and_pricing(
                    context,
                    config.ika_dwallet_2pc_mpc_package_id,
                    config.ika_dwallet_coordinator_object_id,
                    config.ika_common_package_id,
                    protocol_cap_id,
                    default_pricing,
                    supported_curves_to_signature_algorithms_to_hash_schemes,
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
            IkaProtocolCommandResponse::SetOrRemoveWitnessApprovingAdvanceEpochByCap(response)
            | IkaProtocolCommandResponse::SetApprovedUpgradeByCap(response)
            | IkaProtocolCommandResponse::CalculatePricingVotes(response)
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
