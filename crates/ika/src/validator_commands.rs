use anyhow::Result;
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
use ika_types::crypto::{generate_proof_of_possession, AuthorityKeyPair};
use ika_types::dwallet_mpc_error::DwalletMPCResult;
use ika_types::sui::DEFAULT_COMMISSION_RATE;
use serde::Serialize;
use sui_keys::{
    key_derive::generate_new_key,
    keypair_file::{
        read_network_keypair_from_file, write_authority_keypair_to_file, write_keypair_to_file,
    },
};
use sui_sdk::wallet_context::WalletContext;
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
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum IkaValidatorCommandResponse {
    MakeValidatorInfo,
}

impl IkaValidatorCommand {
    pub async fn execute(
        self,
        context: &mut WalletContext,
    ) -> Result<IkaValidatorCommandResponse, anyhow::Error> {
        let sui_address = context.active_address()?;

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
        });
        ret
    }
}

impl Display for IkaValidatorCommandResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();
        match self {
            IkaValidatorCommandResponse::MakeValidatorInfo => {}
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
