use anyhow::Result;
use std::collections::HashSet;
use std::{
    fmt::{Debug, Display, Formatter, Write},
    fs,
    path::PathBuf,
};
use sui_types::{base_types::SuiAddress, crypto::Signable, multiaddr::Multiaddr};

use clap::*;
use colored::Colorize;
use dwallet_classgroups_types::{
    generate_class_groups_keypair_and_proof_from_seed, read_class_groups_from_file,
    write_class_groups_keypair_and_proof_to_file, ClassGroupsKeyPairAndProof,
};
use fastcrypto::traits::KeyPair;
use fastcrypto::traits::ToFromBytes;
use ika_config::node::read_authority_keypair_from_file;
use ika_config::validator_info::ValidatorInfo;
use ika_config::{ika_config_dir, IKA_NETWORK_CONFIG};
use ika_sui_client::ika_validator_transactions::{
    request_add_validator, request_add_validator_candidate, request_remove_validator, stake_ika,
};
use ika_swarm_config::network_config::NetworkConfig;
use ika_types::crypto::{generate_proof_of_possession, AuthorityKeyPair};
use ika_types::dwallet_mpc_error::DwalletMPCResult;
use ika_types::sui::DEFAULT_COMMISSION_RATE;
use serde::Serialize;
use sui::validator_commands::write_transaction_response;
use sui_config::PersistedConfig;
use sui_keys::{
    key_derive::generate_new_key,
    keypair_file::{
        read_network_keypair_from_file, write_authority_keypair_to_file, write_keypair_to_file,
    },
};
use sui_sdk::rpc_types::SuiTransactionBlockResponse;
use sui_sdk::wallet_context::WalletContext;
use sui_types::base_types::ObjectID;
use sui_types::crypto::get_authority_key_pair;
use sui_types::crypto::{NetworkKeyPair, SignatureScheme, SuiKeyPair};

const DEFAULT_GAS_BUDGET: u64 = 200_000_000; // 0.2 SUI

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum IkaValidatorCommand {
    #[clap(name = "make-validator-info")]
    MakeValidatorInfo {
        name: String,
        description: String,
        image_url: String,
        project_url: String,
        host_name: String,
        gas_price: u64,
        sender_sui_address: SuiAddress,
    },
    #[clap(name = "become-candidate")]
    BecomeCandidate {
        #[clap(name = "validator-info-path")]
        file: PathBuf,
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "ika-system-package-id", long)]
        ika_system_config_file: Option<PathBuf>,
    },
    #[clap(name = "join-committee")]
    JoinCommittee {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "ika-system-package-id", long)]
        ika_system_config_file: Option<PathBuf>,
        #[clap(name = "validator-cap-id", long)]
        validator_cap_id: ObjectID,
    },
    #[clap(name = "stake-validator")]
    StakeValidator {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "ika-system-package-id", long)]
        ika_system_config_file: Option<PathBuf>,
        #[clap(name = "validator-id", long)]
        validator_id: ObjectID,
        #[clap(name = "ika-coin-id", long)]
        ika_coin_id: ObjectID,
        #[clap(name = "stake-amount", long)]
        stake_amount: u64,
    },
    #[clap(name = "leave-committee")]
    LeaveCommittee {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "validator-cap-id", long)]
        validator_cap_id: ObjectID,
        #[clap(name = "ika-system-package-id", long)]
        ika_system_config_file: Option<PathBuf>,
    },
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum IkaValidatorCommandResponse {
    MakeValidatorInfo,
    BecomeCandidate(SuiTransactionBlockResponse, ObjectID, ObjectID),
    JoinCommittee(SuiTransactionBlockResponse),
    StakeValidator(SuiTransactionBlockResponse),
    LeaveCommittee(SuiTransactionBlockResponse),
}

impl IkaValidatorCommand {
    pub async fn execute(
        self,
        context: &mut WalletContext,
    ) -> Result<IkaValidatorCommandResponse, anyhow::Error> {
        let ret = Ok(match self {
            IkaValidatorCommand::MakeValidatorInfo {
                name,
                description,
                image_url,
                project_url,
                host_name,
                gas_price,
                sender_sui_address,
            } => {
                let dir = std::env::current_dir()?;
                let protocol_key_file_name = dir.join("protocol.key");
                let network_key_file_name = dir.join("network.key");
                let worker_key_file_name = dir.join("worker.key");

                make_key_files(protocol_key_file_name.clone(), true, None)?;
                make_key_files(network_key_file_name.clone(), false, None)?;
                make_key_files(worker_key_file_name.clone(), false, None)?;

                let keypair = read_authority_keypair_from_file(&protocol_key_file_name);
                let consensus_keypair: NetworkKeyPair =
                    read_network_keypair_from_file(worker_key_file_name)?;
                let network_keypair: NetworkKeyPair =
                    read_network_keypair_from_file(network_key_file_name)?;
                let pop = generate_proof_of_possession(&keypair, sender_sui_address);

                let class_groups_public_key_and_proof =
                    read_or_generate_from_seed_class_groups_key(
                        dir.join("class-groups.key"),
                        &keypair,
                    )?;

                let validator_info = ValidatorInfo {
                    name,
                    class_groups_public_key_and_proof: class_groups_public_key_and_proof
                        .public_bytes(),
                    account_address: sender_sui_address,
                    protocol_public_key: keypair.public().into(),
                    consensus_public_key: consensus_keypair.public().clone(),
                    network_public_key: network_keypair.public().clone(),
                    computation_price: gas_price,
                    description,
                    image_url,
                    project_url,
                    commission_rate: DEFAULT_COMMISSION_RATE,
                    consensus_address: Multiaddr::try_from(format!("/dns/{}/udp/8081", host_name))?,
                    network_address: Multiaddr::try_from(format!(
                        "/dns/{}/tcp/8080/http",
                        host_name
                    ))?,
                    p2p_address: Multiaddr::try_from(format!("/dns/{}/udp/8084", host_name))?,
                    proof_of_possession: pop,
                };

                let validator_info_file_name = dir.join("validator.info");
                let validator_info_bytes = serde_yaml::to_string(&validator_info)?;
                fs::write(validator_info_file_name.clone(), validator_info_bytes)?;
                println!(
                    "Generated validator info file: {:?}.",
                    validator_info_file_name
                );
                IkaValidatorCommandResponse::MakeValidatorInfo
            }
            IkaValidatorCommand::BecomeCandidate {
                file,
                gas_budget,
                ika_system_config_file,
            } => {
                let gas_budget = gas_budget.unwrap_or(DEFAULT_GAS_BUDGET);
                let config_path =
                    ika_system_config_file.unwrap_or(ika_config_dir()?.join(IKA_NETWORK_CONFIG));
                let config: NetworkConfig = PersistedConfig::read(&config_path).map_err(|err| {
                    err.context(format!(
                        "Cannot open Ika network config file at {:?}",
                        config_path
                    ))
                })?;

                let validator_info_bytes = fs::read_to_string(file)?;
                let validator_info: ValidatorInfo = serde_yaml::from_str(&validator_info_bytes)?;

                let class_groups_keypair_and_proof_obj_ref = ika_sui_client::ika_validator_transactions::create_class_groups_public_key_and_proof_object(
                    context.active_address()?,
                    context,
                    config.ika_system_package_id.clone(),
                    validator_info.class_groups_public_key_and_proof.clone(),
                    gas_budget,
                ).await?;

                let (res, validator_id, validator_cap_id) = request_add_validator_candidate(
                    context,
                    &validator_info,
                    config.ika_system_package_id.clone(),
                    config.system_id.clone(),
                    class_groups_keypair_and_proof_obj_ref,
                    gas_budget,
                )
                .await?;
                IkaValidatorCommandResponse::BecomeCandidate(res, validator_id, validator_cap_id)
            }
            IkaValidatorCommand::JoinCommittee {
                gas_budget,
                ika_system_config_file,
                validator_cap_id,
            } => {
                let gas_budget = gas_budget.unwrap_or(DEFAULT_GAS_BUDGET);
                let config_path =
                    ika_system_config_file.unwrap_or(ika_config_dir()?.join(IKA_NETWORK_CONFIG));
                let config: NetworkConfig = PersistedConfig::read(&config_path).map_err(|err| {
                    err.context(format!(
                        "Cannot open Ika network config file at {:?}",
                        config_path
                    ))
                })?;

                let response = request_add_validator(
                    context,
                    config.ika_system_package_id.clone(),
                    config.system_id.clone(),
                    validator_cap_id,
                    gas_budget,
                )
                .await?;
                IkaValidatorCommandResponse::JoinCommittee(response)
            }
            IkaValidatorCommand::StakeValidator {
                gas_budget,
                ika_system_config_file,
                validator_id,
                ika_coin_id,
                stake_amount,
            } => {
                let gas_budget = gas_budget.unwrap_or(DEFAULT_GAS_BUDGET);
                let config_path =
                    ika_system_config_file.unwrap_or(ika_config_dir()?.join(IKA_NETWORK_CONFIG));
                let config: NetworkConfig = PersistedConfig::read(&config_path).map_err(|err| {
                    err.context(format!(
                        "Cannot open Ika network config file at {:?}",
                        config_path
                    ))
                })?;

                let res = stake_ika(
                    context,
                    config.ika_system_package_id.clone(),
                    config.system_id.clone(),
                    ika_coin_id,
                    validator_id,
                    stake_amount,
                    gas_budget,
                )
                .await?;

                IkaValidatorCommandResponse::StakeValidator(res)
            }
            IkaValidatorCommand::LeaveCommittee {
                gas_budget,
                validator_cap_id,
                ika_system_config_file,
            } => {
                let gas_budget = gas_budget.unwrap_or(DEFAULT_GAS_BUDGET);
                let config_path =
                    ika_system_config_file.unwrap_or(ika_config_dir()?.join(IKA_NETWORK_CONFIG));
                let config: NetworkConfig = PersistedConfig::read(&config_path).map_err(|err| {
                    err.context(format!(
                        "Cannot open Ika network config file at {:?}",
                        config_path
                    ))
                })?;
                let response = request_remove_validator(
                    context,
                    config.ika_system_package_id.clone(),
                    config.system_id.clone(),
                    validator_cap_id,
                    gas_budget,
                )
                .await?;
                IkaValidatorCommandResponse::LeaveCommittee(response)
            }
        });
        ret
    }
}

impl Display for IkaValidatorCommandResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();
        match self {
            IkaValidatorCommandResponse::MakeValidatorInfo => {}
            IkaValidatorCommandResponse::BecomeCandidate(
                response,
                validator_id,
                validator_cap_id,
            ) => {
                write!(writer, "{}", write_transaction_response(response)?)?;
                writeln!(writer, "Validator ID: {}", validator_id)?;
                writeln!(writer, "Validator Cap ID: {}", validator_cap_id)?;
            }
            IkaValidatorCommandResponse::JoinCommittee(response)
            | IkaValidatorCommandResponse::StakeValidator(response)
            | IkaValidatorCommandResponse::LeaveCommittee(response) => {
                write!(writer, "{}", write_transaction_response(response)?)?;
            }
        }
        write!(f, "{}", writer.trim_end_matches('\n'))
    }
}

impl Debug for IkaValidatorCommandResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = serde_json::to_string_pretty(self);
        let s = match string {
            Ok(s) => s,
            Err(err) => format!("{err}").red().to_string(),
        };
        write!(f, "{}", s)
    }
}

impl IkaValidatorCommandResponse {
    pub fn print(&self, pretty: bool) {
        match self {
            IkaValidatorCommandResponse::MakeValidatorInfo => {}
            other => {
                let line = if pretty {
                    format!("{other}")
                } else {
                    format!("{:?}", other)
                };
                for line in line.lines() {
                    println!("{line}");
                }
            }
        }
    }
}

fn make_key_files(
    file_name: PathBuf,
    is_protocol_key: bool,
    key: Option<SuiKeyPair>,
) -> Result<()> {
    if file_name.exists() {
        println!("Use existing {:?} key file.", file_name);
        return Ok(());
    } else if is_protocol_key {
        let (_, keypair) = get_authority_key_pair();
        write_authority_keypair_to_file(&keypair, file_name.clone())?;
        println!("Generated new key file: {:?}.", file_name);
    } else {
        let kp = match key {
            Some(key) => {
                println!(
                    "Generated new key file {:?} based on sui.keystore file.",
                    file_name
                );
                key
            }
            None => {
                let (_, kp, _, _) = generate_new_key(SignatureScheme::ED25519, None, None)?;
                println!("Generated new key file: {:?}.", file_name);
                kp
            }
        };
        write_keypair_to_file(&kp, &file_name)?;
    }
    Ok(())
}

/// Reads the class groups key pair and proof from a file if it exists, otherwise generates it from the seed.
/// The seed is the private key of the authority key pair.
fn read_or_generate_from_seed_class_groups_key(
    file_path: PathBuf,
    seed: &AuthorityKeyPair,
) -> Result<Box<ClassGroupsKeyPairAndProof>> {
    match read_class_groups_from_file(file_path.clone()) {
        Ok(class_groups_public_key_and_proof) => {
            println!("Use existing: {:?}.", file_path,);
            Ok(class_groups_public_key_and_proof)
        }
        Err(_) => {
            let class_groups_public_key_and_proof =
                Box::new(generate_class_groups_keypair_and_proof_from_seed(
                    seed.copy().private().as_bytes().try_into()?,
                ));
            write_class_groups_keypair_and_proof_to_file(
                &class_groups_public_key_and_proof,
                file_path.clone(),
            )?;
            println!(
                "Generated class groups key pair info file: {:?}.",
                file_path,
            );
            Ok(class_groups_public_key_and_proof)
        }
    }
}
