use anyhow::Result;
use std::{
    fmt::{Debug, Display, Formatter, Write},
    fs,
    path::PathBuf,
};
use sui_config::Config;
use sui_types::{base_types::SuiAddress, multiaddr::Multiaddr};

use clap::*;
use colored::Colorize;
use dwallet_classgroups_types::ClassGroupsKeyPairAndProof;
use dwallet_rng::RootSeed;
use fastcrypto::traits::{KeyPair, ToFromBytes};
use ika_config::node::read_authority_keypair_from_file;
use ika_config::validator_info::ValidatorInfo;
use ika_config::{IKA_SUI_CONFIG, ika_config_dir};
use ika_sui_client::ika_validator_transactions::{
    BecomeCandidateValidatorData, collect_commission,
    create_class_groups_public_key_and_proof_object, report_validator, request_add_validator,
    request_add_validator_candidate, request_remove_validator, request_remove_validator_candidate,
    request_withdraw_stake, rotate_commission_cap, rotate_operation_cap, set_next_commission,
    set_next_epoch_class_groups_pubkey_and_proof_bytes, set_next_epoch_consensus_address,
    set_next_epoch_consensus_pubkey_bytes, set_next_epoch_network_address,
    set_next_epoch_network_pubkey_bytes, set_next_epoch_p2p_address,
    set_next_epoch_protocol_pubkey_bytes, set_pricing_vote, set_validator_metadata,
    set_validator_name, stake_ika, undo_report_validator, validator_metadata,
    verify_commission_cap, verify_operation_cap, verify_validator_cap, withdraw_stake,
};
use ika_sui_client::metrics::SuiClientMetrics;
use ika_sui_client::{SuiClient, SuiClientInner};
use ika_types::crypto::generate_proof_of_possession;
use ika_types::messages_dwallet_mpc::IkaPackagesConfig;
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
    #[clap(name = "config-env")]
    ConfigEnv {
        #[clap(name = "ika-package-id", long)]
        ika_package_id: ObjectID,
        #[clap(name = "ika-common-package-id", long)]
        ika_common_package_id: ObjectID,
        #[clap(name = "ika-dwallet-2pc-mpc-package-id", long)]
        ika_dwallet_2pc_mpc_package_id: ObjectID,
        #[clap(name = "ika-system-package-id", long)]
        ika_system_package_id: ObjectID,
        #[clap(name = "ika-system-object-id", long)]
        ika_system_object_id: ObjectID,
    },
    #[clap(name = "become-candidate")]
    BecomeCandidate {
        #[clap(name = "validator-info-path")]
        validator_info_file: PathBuf,
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
    #[clap(name = "join-committee")]
    JoinCommittee {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
        #[clap(name = "validator-cap-id", long)]
        validator_cap_id: ObjectID,
    },
    #[clap(name = "stake-validator")]
    StakeValidator {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
        #[clap(name = "validator-id", long)]
        validator_id: ObjectID,
        #[clap(name = "ika-supply-id", long)]
        ika_supply_id: ObjectID,
        #[clap(name = "stake-amount", long)]
        stake_amount: u64,
    },
    #[clap(name = "leave-committee")]
    LeaveCommittee {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "validator-cap-id", long)]
        validator_cap_id: ObjectID,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
    #[clap(name = "remove-candidate")]
    RemoveCandidate {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "validator-cap-id", long)]
        validator_cap_id: ObjectID,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
    #[clap(name = "set-commission")]
    SetCommission {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "validator-operation-cap-id", long)]
        validator_operation_cap_id: ObjectID,
        #[clap(name = "new-commission-rate", long)]
        new_commission_rate: u16,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
    #[clap(name = "withdraw-stake")]
    WithdrawStake {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "staked-ika-id", long)]
        staked_ika_id: ObjectID,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
    #[clap(name = "request-withdraw-stake")]
    RequestWithdrawStake {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "staked-ika-id", long)]
        staked_ika_id: ObjectID,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
    #[clap(name = "report-validator")]
    ReportValidator {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "validator-operation-cap-id", long)]
        validator_operation_cap_id: ObjectID,
        #[clap(name = "reportee-id", long)]
        reportee_id: ObjectID,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
    #[clap(name = "undo-report-validator")]
    UndoReportValidator {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "validator-operation-cap-id", long)]
        validator_operation_cap_id: ObjectID,
        #[clap(name = "reportee-id", long)]
        reportee_id: ObjectID,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
    #[clap(name = "rotate-operation-cap")]
    RotateOperationCap {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "validator-cap-id", long)]
        validator_cap_id: ObjectID,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
    #[clap(name = "rotate-commission-cap")]
    RotateCommissionCap {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "validator-cap-id", long)]
        validator_cap_id: ObjectID,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
    #[clap(name = "collect-commission")]
    CollectCommission {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "validator-commission-cap-id", long)]
        validator_commission_cap_id: ObjectID,
        #[clap(name = "amount", long)]
        amount: Option<u64>,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
    #[clap(name = "set-validator-name")]
    SetValidatorName {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "validator-operation-cap-id", long)]
        validator_operation_cap_id: ObjectID,
        #[clap(name = "name", long)]
        name: String,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
    #[clap(name = "get-validator-metadata")]
    GetValidatorMetadata {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "validator-id", long)]
        validator_id: ObjectID,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
    #[clap(name = "set-validator-metadata")]
    SetValidatorMetadata {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "validator-operation-cap-id", long)]
        validator_operation_cap_id: ObjectID,
        #[clap(name = "metadata", long)]
        metadata: String,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
    #[clap(name = "set-next-epoch-network-address")]
    SetNextEpochNetworkAddress {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "validator-operation-cap-id", long)]
        validator_operation_cap_id: ObjectID,
        #[clap(name = "network-address", long)]
        network_address: String,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
    #[clap(name = "set-next-epoch-p2p-address")]
    SetNextEpochP2pAddress {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "validator-operation-cap-id", long)]
        validator_operation_cap_id: ObjectID,
        #[clap(name = "p2p-address", long)]
        p2p_address: String,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
    #[clap(name = "set-next-epoch-consensus-address")]
    SetNextEpochConsensusAddress {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "validator-operation-cap-id", long)]
        validator_operation_cap_id: ObjectID,
        #[clap(name = "consensus-address", long)]
        consensus_address: String,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
    #[clap(name = "set-next-epoch-protocol-pubkey")]
    SetNextEpochProtocolPubkey {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "validator-operation-cap-id", long)]
        validator_operation_cap_id: ObjectID,
        #[clap(name = "validator-info-path")]
        validator_info_file: PathBuf,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
    #[clap(name = "set-next-epoch-network-pubkey")]
    SetNextEpochNetworkPubkey {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "validator-operation-cap-id", long)]
        validator_operation_cap_id: ObjectID,
        #[clap(name = "validator-info-path")]
        validator_info_file: PathBuf,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
    #[clap(name = "set-next-epoch-consensus-pubkey")]
    SetNextEpochConsensusPubkey {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "validator-operation-cap-id", long)]
        validator_operation_cap_id: ObjectID,
        #[clap(name = "validator-info-path")]
        validator_info_file: PathBuf,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
    #[clap(name = "set-next-epoch-class-groups-pubkey")]
    SetNextEpochClassGroupsPubkey {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "validator-operation-cap-id", long)]
        validator_operation_cap_id: ObjectID,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
    #[clap(name = "verify-validator-cap")]
    VerifyValidatorCap {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "validator-cap-id", long)]
        validator_cap_id: ObjectID,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
    #[clap(name = "verify-operation-cap")]
    VerifyOperationCap {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "validator-operation-cap-id", long)]
        validator_operation_cap_id: ObjectID,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
    #[clap(name = "verify-commission-cap")]
    VerifyCommissionCap {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "validator-commission-cap-id", long)]
        validator_commission_cap_id: ObjectID,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
    #[clap(name = "set-pricing-vote")]
    SetPricingVote {
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
        #[clap(name = "validator-operation-cap-id", long)]
        validator_operation_cap_id: ObjectID,
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
    #[clap(name = "get-current-pricing-info")]
    GetCurrentPricingInfo {
        #[clap(name = "ika-sui-config", long)]
        ika_sui_config: Option<PathBuf>,
    },
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum IkaValidatorCommandResponse {
    MakeValidatorInfo,
    ConfigEnv(PathBuf),
    BecomeCandidate(SuiTransactionBlockResponse, BecomeCandidateValidatorData),
    JoinCommittee(SuiTransactionBlockResponse),
    StakeValidator(SuiTransactionBlockResponse),
    LeaveCommittee(SuiTransactionBlockResponse),
    RemoveCandidate(SuiTransactionBlockResponse),
    SetCommission(SuiTransactionBlockResponse),
    WithdrawStake(SuiTransactionBlockResponse),
    RequestWithdrawStake(SuiTransactionBlockResponse),
    ReportValidator(SuiTransactionBlockResponse),
    UndoReportValidator(SuiTransactionBlockResponse),
    RotateOperationCap(SuiTransactionBlockResponse),
    RotateCommissionCap(SuiTransactionBlockResponse),
    CollectCommission(SuiTransactionBlockResponse),
    SetValidatorName(SuiTransactionBlockResponse),
    GetValidatorMetadata(SuiTransactionBlockResponse),
    SetValidatorMetadata(SuiTransactionBlockResponse),
    SetNextEpochNetworkAddress(SuiTransactionBlockResponse),
    SetNextEpochP2pAddress(SuiTransactionBlockResponse),
    SetNextEpochConsensusAddress(SuiTransactionBlockResponse),
    SetNextEpochProtocolPubkey(SuiTransactionBlockResponse),
    SetNextEpochNetworkPubkey(SuiTransactionBlockResponse),
    SetNextEpochConsensusPubkey(SuiTransactionBlockResponse),
    SetNextEpochClassGroupsPubkey(SuiTransactionBlockResponse),
    VerifyValidatorCap(SuiTransactionBlockResponse),
    VerifyOperationCap(SuiTransactionBlockResponse),
    VerifyCommissionCap(SuiTransactionBlockResponse),
    SetPricingVote(SuiTransactionBlockResponse),
    FetchCurrentPricingInfo(PathBuf),
}

impl IkaValidatorCommand {
    pub async fn execute(
        self,
        context: &mut WalletContext,
    ) -> Result<IkaValidatorCommandResponse, anyhow::Error> {
        Ok(match self {
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
                let consensus_key_file_name = dir.join("consensus.key");

                make_key_files(protocol_key_file_name.clone(), true, None)?;
                make_key_files(network_key_file_name.clone(), false, None)?;
                make_key_files(consensus_key_file_name.clone(), false, None)?;

                let keypair = read_authority_keypair_from_file(&protocol_key_file_name);
                let consensus_keypair: NetworkKeyPair =
                    read_network_keypair_from_file(consensus_key_file_name)?;
                let network_keypair: NetworkKeyPair =
                    read_network_keypair_from_file(network_key_file_name)?;
                let pop = generate_proof_of_possession(&keypair, sender_sui_address);

                let class_groups_public_key_and_proof =
                    read_or_generate_seed_and_class_groups_key(dir.join("class-groups.seed"))?;

                let validator_info = ValidatorInfo {
                    name,
                    class_groups_public_key_and_proof: class_groups_public_key_and_proof
                        .encryption_key_and_proof(),
                    account_address: sender_sui_address,
                    protocol_public_key: keypair.public().into(),
                    consensus_public_key: consensus_keypair.public().clone(),
                    network_public_key: network_keypair.public().clone(),
                    computation_price: gas_price,
                    description,
                    image_url,
                    project_url,
                    commission_rate: DEFAULT_COMMISSION_RATE,
                    consensus_address: Multiaddr::try_from(format!("/dns/{host_name}/udp/8081"))?,
                    network_address: Multiaddr::try_from(format!(
                        "/dns/{host_name}/tcp/8080/http"
                    ))?,
                    p2p_address: Multiaddr::try_from(format!("/dns/{host_name}/udp/8084"))?,
                    proof_of_possession: pop,
                };

                let validator_info_file_name = dir.join("validator.info");
                let validator_info_bytes = serde_yaml::to_string(&validator_info)?;
                fs::write(validator_info_file_name.clone(), validator_info_bytes)?;
                println!("Generated validator info file: {validator_info_file_name:?}.");
                IkaValidatorCommandResponse::MakeValidatorInfo
            }
            IkaValidatorCommand::ConfigEnv {
                ika_package_id,
                ika_common_package_id,
                ika_dwallet_2pc_mpc_package_id,
                ika_system_package_id,
                ika_system_object_id,
            } => {
                let config = IkaPackagesConfig {
                    ika_package_id,
                    ika_common_package_id,
                    ika_dwallet_2pc_mpc_package_id,
                    ika_system_package_id,
                    ika_system_object_id,
                    // This is done on purpose,
                    // there is no ika_dwallet_coordinator_object_id at this stage.
                    ika_dwallet_coordinator_object_id: ObjectID::ZERO,
                };

                let config_path = ika_config_dir()?.join(IKA_SUI_CONFIG);
                config.save(&config_path)?;
                IkaValidatorCommandResponse::ConfigEnv(config_path)
            }
            IkaValidatorCommand::BecomeCandidate {
                validator_info_file,
                gas_budget,
                ika_sui_config,
            } => {
                let gas_budget = gas_budget.unwrap_or(DEFAULT_GAS_BUDGET);
                let ika_on_sui_config_path =
                    ika_sui_config.unwrap_or(ika_config_dir()?.join(IKA_SUI_CONFIG));
                let config: IkaPackagesConfig = PersistedConfig::read(&ika_on_sui_config_path)
                    .map_err(|err| {
                        err.context(format!(
                            "Cannot open Ika network config file at {ika_on_sui_config_path:?}"
                        ))
                    })?;

                let validator_info_bytes = fs::read_to_string(validator_info_file)?;
                let validator_info: ValidatorInfo = serde_yaml::from_str(&validator_info_bytes)?;

                let class_groups_keypair_and_proof_obj_ref = ika_sui_client::ika_validator_transactions::create_class_groups_public_key_and_proof_object(
                    context.active_address()?,
                    context,
                    config.ika_common_package_id,
                    validator_info.class_groups_public_key_and_proof.clone(),
                    gas_budget,
                ).await?;

                let (res, validator_data) = request_add_validator_candidate(
                    context,
                    &validator_info,
                    config.ika_system_package_id,
                    config.ika_system_object_id,
                    class_groups_keypair_and_proof_obj_ref,
                    gas_budget,
                )
                .await?;
                IkaValidatorCommandResponse::BecomeCandidate(res, validator_data)
            }
            IkaValidatorCommand::JoinCommittee {
                gas_budget,
                ika_sui_config,
                validator_cap_id,
            } => {
                let gas_budget = gas_budget.unwrap_or(DEFAULT_GAS_BUDGET);
                let config_path = ika_sui_config.unwrap_or(ika_config_dir()?.join(IKA_SUI_CONFIG));
                let config: IkaPackagesConfig =
                    PersistedConfig::read(&config_path).map_err(|err| {
                        err.context(format!(
                            "Cannot open Ika network config file at {config_path:?}"
                        ))
                    })?;

                let response = request_add_validator(
                    context,
                    config.ika_system_package_id,
                    config.ika_system_object_id,
                    validator_cap_id,
                    gas_budget,
                )
                .await?;
                IkaValidatorCommandResponse::JoinCommittee(response)
            }
            IkaValidatorCommand::StakeValidator {
                gas_budget,
                ika_sui_config,
                validator_id,
                ika_supply_id,
                stake_amount,
            } => {
                let gas_budget = gas_budget.unwrap_or(DEFAULT_GAS_BUDGET);
                let config_path = ika_sui_config.unwrap_or(ika_config_dir()?.join(IKA_SUI_CONFIG));
                let config: IkaPackagesConfig =
                    PersistedConfig::read(&config_path).map_err(|err| {
                        err.context(format!(
                            "Cannot open Ika network config file at {config_path:?}"
                        ))
                    })?;

                let res = stake_ika(
                    context,
                    config.ika_system_package_id,
                    config.ika_system_object_id,
                    ika_supply_id,
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
                let response = request_remove_validator(
                    context,
                    config.ika_system_package_id,
                    config.ika_system_object_id,
                    validator_cap_id,
                    gas_budget,
                )
                .await?;
                IkaValidatorCommandResponse::LeaveCommittee(response)
            }
            IkaValidatorCommand::RemoveCandidate {
                gas_budget,
                validator_cap_id,
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
                let response = request_remove_validator_candidate(
                    context,
                    config.ika_system_package_id,
                    config.ika_system_object_id,
                    validator_cap_id,
                    gas_budget,
                )
                .await?;
                IkaValidatorCommandResponse::RemoveCandidate(response)
            }
            IkaValidatorCommand::SetCommission {
                gas_budget,
                validator_operation_cap_id,
                new_commission_rate,
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
                let response = set_next_commission(
                    context,
                    config.ika_system_package_id,
                    config.ika_system_object_id,
                    validator_operation_cap_id,
                    new_commission_rate,
                    gas_budget,
                )
                .await?;
                IkaValidatorCommandResponse::SetCommission(response)
            }
            IkaValidatorCommand::WithdrawStake {
                gas_budget,
                staked_ika_id,
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
                let response = withdraw_stake(
                    context,
                    config.ika_system_package_id,
                    config.ika_system_object_id,
                    staked_ika_id,
                    gas_budget,
                )
                .await?;
                IkaValidatorCommandResponse::WithdrawStake(response)
            }
            IkaValidatorCommand::RequestWithdrawStake {
                gas_budget,
                staked_ika_id,
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
                let response = request_withdraw_stake(
                    context,
                    config.ika_system_package_id,
                    config.ika_system_object_id,
                    staked_ika_id,
                    gas_budget,
                )
                .await?;
                IkaValidatorCommandResponse::RequestWithdrawStake(response)
            }
            IkaValidatorCommand::ReportValidator {
                gas_budget,
                validator_operation_cap_id,
                reportee_id,
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
                let response = report_validator(
                    context,
                    config.ika_system_package_id,
                    config.ika_system_object_id,
                    validator_operation_cap_id,
                    reportee_id,
                    gas_budget,
                )
                .await?;
                IkaValidatorCommandResponse::ReportValidator(response)
            }
            IkaValidatorCommand::UndoReportValidator {
                gas_budget,
                validator_operation_cap_id,
                reportee_id,
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
                let response = undo_report_validator(
                    context,
                    config.ika_system_package_id,
                    config.ika_system_object_id,
                    validator_operation_cap_id,
                    reportee_id,
                    gas_budget,
                )
                .await?;
                IkaValidatorCommandResponse::UndoReportValidator(response)
            }
            IkaValidatorCommand::RotateOperationCap {
                gas_budget,
                validator_cap_id,
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
                let response = rotate_operation_cap(
                    context,
                    config.ika_system_package_id,
                    config.ika_system_object_id,
                    validator_cap_id,
                    gas_budget,
                )
                .await?;
                IkaValidatorCommandResponse::RotateOperationCap(response)
            }
            IkaValidatorCommand::RotateCommissionCap {
                gas_budget,
                validator_cap_id,
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
                let response = rotate_commission_cap(
                    context,
                    config.ika_system_package_id,
                    config.ika_system_object_id,
                    validator_cap_id,
                    gas_budget,
                )
                .await?;
                IkaValidatorCommandResponse::RotateCommissionCap(response)
            }
            IkaValidatorCommand::CollectCommission {
                gas_budget,
                validator_commission_cap_id,
                amount,
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
                let response = collect_commission(
                    context,
                    config.ika_system_package_id,
                    config.ika_system_object_id,
                    validator_commission_cap_id,
                    amount,
                    gas_budget,
                )
                .await?;
                IkaValidatorCommandResponse::CollectCommission(response)
            }
            IkaValidatorCommand::SetValidatorName {
                gas_budget,
                validator_operation_cap_id,
                name,
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
                let response = set_validator_name(
                    context,
                    config.ika_system_package_id,
                    config.ika_system_object_id,
                    validator_operation_cap_id,
                    name,
                    gas_budget,
                )
                .await?;
                IkaValidatorCommandResponse::SetValidatorName(response)
            }
            IkaValidatorCommand::GetValidatorMetadata {
                gas_budget,
                validator_id,
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
                let response = validator_metadata(
                    context,
                    config.ika_system_package_id,
                    config.ika_system_object_id,
                    validator_id,
                    gas_budget,
                )
                .await?;
                IkaValidatorCommandResponse::GetValidatorMetadata(response)
            }
            IkaValidatorCommand::SetValidatorMetadata {
                gas_budget,
                validator_operation_cap_id,
                metadata,
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
                let response = set_validator_metadata(
                    context,
                    config.ika_system_package_id,
                    config.ika_system_object_id,
                    validator_operation_cap_id,
                    metadata,
                    gas_budget,
                )
                .await?;
                IkaValidatorCommandResponse::SetValidatorMetadata(response)
            }
            IkaValidatorCommand::SetNextEpochNetworkAddress {
                gas_budget,
                validator_operation_cap_id,
                network_address,
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
                let response = set_next_epoch_network_address(
                    context,
                    config.ika_system_package_id,
                    config.ika_system_object_id,
                    validator_operation_cap_id,
                    network_address,
                    gas_budget,
                )
                .await?;
                IkaValidatorCommandResponse::SetNextEpochNetworkAddress(response)
            }
            IkaValidatorCommand::SetNextEpochP2pAddress {
                gas_budget,
                validator_operation_cap_id,
                p2p_address,
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
                let response = set_next_epoch_p2p_address(
                    context,
                    config.ika_system_package_id,
                    config.ika_system_object_id,
                    validator_operation_cap_id,
                    p2p_address,
                    gas_budget,
                )
                .await?;
                IkaValidatorCommandResponse::SetNextEpochP2pAddress(response)
            }
            IkaValidatorCommand::SetNextEpochConsensusAddress {
                gas_budget,
                validator_operation_cap_id,
                consensus_address,
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
                let response = set_next_epoch_consensus_address(
                    context,
                    config.ika_system_package_id,
                    config.ika_system_object_id,
                    validator_operation_cap_id,
                    consensus_address,
                    gas_budget,
                )
                .await?;
                IkaValidatorCommandResponse::SetNextEpochConsensusAddress(response)
            }
            IkaValidatorCommand::SetNextEpochProtocolPubkey {
                gas_budget,
                validator_operation_cap_id,
                validator_info_file,
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
                let validator_info_bytes = fs::read_to_string(validator_info_file)?;
                let validator_info: ValidatorInfo = serde_yaml::from_str(&validator_info_bytes)?;
                let response = set_next_epoch_protocol_pubkey_bytes(
                    context,
                    config.ika_system_package_id,
                    config.ika_system_object_id,
                    validator_operation_cap_id,
                    validator_info.protocol_public_key.as_bytes().to_vec(),
                    validator_info.proof_of_possession.as_ref().to_vec(),
                    gas_budget,
                )
                .await?;
                IkaValidatorCommandResponse::SetNextEpochProtocolPubkey(response)
            }
            IkaValidatorCommand::SetNextEpochNetworkPubkey {
                gas_budget,
                validator_operation_cap_id,
                validator_info_file,
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
                let validator_info_bytes = fs::read_to_string(validator_info_file)?;
                let validator_info: ValidatorInfo = serde_yaml::from_str(&validator_info_bytes)?;
                let response = set_next_epoch_network_pubkey_bytes(
                    context,
                    config.ika_system_package_id,
                    config.ika_system_object_id,
                    validator_operation_cap_id,
                    validator_info.network_public_key.as_bytes().to_vec(),
                    gas_budget,
                )
                .await?;
                IkaValidatorCommandResponse::SetNextEpochNetworkPubkey(response)
            }
            IkaValidatorCommand::SetNextEpochConsensusPubkey {
                gas_budget,
                validator_operation_cap_id,
                validator_info_file,
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
                let validator_info_bytes = fs::read_to_string(validator_info_file)?;
                let validator_info: ValidatorInfo = serde_yaml::from_str(&validator_info_bytes)?;
                let response = set_next_epoch_consensus_pubkey_bytes(
                    context,
                    config.ika_system_package_id,
                    config.ika_system_object_id,
                    validator_operation_cap_id,
                    validator_info.consensus_public_key.as_bytes().to_vec(),
                    gas_budget,
                )
                .await?;
                IkaValidatorCommandResponse::SetNextEpochConsensusPubkey(response)
            }
            IkaValidatorCommand::SetNextEpochClassGroupsPubkey {
                gas_budget,
                validator_operation_cap_id,
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

                // Create a new seed and class groups key
                let new_seed = RootSeed::random_seed();
                let new_class_groups_key = ClassGroupsKeyPairAndProof::from_seed(&new_seed);

                // Create the class groups object with the new key
                let class_groups_keypair_and_proof_obj_ref =
                    create_class_groups_public_key_and_proof_object(
                        context.active_address()?,
                        context,
                        config.ika_common_package_id,
                        new_class_groups_key.encryption_key_and_proof(),
                        gas_budget,
                    )
                    .await?;

                let response = set_next_epoch_class_groups_pubkey_and_proof_bytes(
                    context,
                    config.ika_system_package_id,
                    config.ika_system_object_id,
                    config.ika_common_package_id,
                    validator_operation_cap_id,
                    class_groups_keypair_and_proof_obj_ref,
                    gas_budget,
                )
                .await?;

                if response.status_ok().is_some() && response.status_ok().unwrap() {
                    // Save the new seed to class-groups.key file (override if exists)
                    let dir = std::env::current_dir()?;
                    let class_groups_key_file = dir.join("class-groups.key");
                    new_seed.save_to_file(class_groups_key_file.clone())?;
                    println!("Generated new class groups seed file: {class_groups_key_file:?}.");
                }

                IkaValidatorCommandResponse::SetNextEpochClassGroupsPubkey(response)
            }
            IkaValidatorCommand::VerifyValidatorCap {
                gas_budget,
                validator_cap_id,
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
                let response = verify_validator_cap(
                    context,
                    config.ika_system_package_id,
                    config.ika_system_object_id,
                    validator_cap_id,
                    gas_budget,
                )
                .await?;
                IkaValidatorCommandResponse::VerifyValidatorCap(response)
            }
            IkaValidatorCommand::VerifyOperationCap {
                gas_budget,
                validator_operation_cap_id,
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
                let response = verify_operation_cap(
                    context,
                    config.ika_system_package_id,
                    config.ika_system_object_id,
                    validator_operation_cap_id,
                    gas_budget,
                )
                .await?;
                IkaValidatorCommandResponse::VerifyOperationCap(response)
            }
            IkaValidatorCommand::VerifyCommissionCap {
                gas_budget,
                validator_commission_cap_id,
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
                let response = verify_commission_cap(
                    context,
                    config.ika_system_package_id,
                    config.ika_system_object_id,
                    validator_commission_cap_id,
                    gas_budget,
                )
                .await?;
                IkaValidatorCommandResponse::VerifyCommissionCap(response)
            }
            IkaValidatorCommand::SetPricingVote {
                gas_budget,
                validator_operation_cap_id,
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
                let response = set_pricing_vote(
                    context,
                    config.ika_system_package_id,
                    config.ika_system_object_id,
                    config.ika_dwallet_2pc_mpc_package_id,
                    config.ika_dwallet_coordinator_object_id,
                    validator_operation_cap_id,
                    gas_budget,
                )
                .await?;
                IkaValidatorCommandResponse::SetPricingVote(response)
            }
            IkaValidatorCommand::GetCurrentPricingInfo { ika_sui_config } => {
                let config_path = ika_sui_config.unwrap_or(ika_config_dir()?.join(IKA_SUI_CONFIG));
                let config: IkaPackagesConfig =
                    PersistedConfig::read(&config_path).map_err(|err| {
                        err.context(format!(
                            "Cannot open Ika network config file at {config_path:?}"
                        ))
                    })?;

                let client = SuiClient::new(
                    &context.get_active_env()?.rpc,
                    SuiClientMetrics::new_for_testing(),
                    config.ika_package_id,
                    config.ika_common_package_id,
                    config.ika_dwallet_2pc_mpc_package_id,
                    config.ika_system_package_id,
                    config.ika_system_object_id,
                    config.ika_dwallet_coordinator_object_id,
                )
                .await?;
                let current_pricing_info = client.get_pricing_info().await;
                current_pricing_info.iter().for_each(|entry| {
                    println!("Pricing Info: {:?} - {:?}", entry.key, entry.value);
                });
                IkaValidatorCommandResponse::FetchCurrentPricingInfo(config_path)
            }
        })
    }
}

impl Display for IkaValidatorCommandResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();
        match self {
            IkaValidatorCommandResponse::MakeValidatorInfo => {}
            IkaValidatorCommandResponse::BecomeCandidate(
                response,
                BecomeCandidateValidatorData {
                    validator_id,
                    validator_cap_id,
                    validator_operation_cap_id,
                    validator_commission_cap_id,
                },
            ) => {
                write!(writer, "{}", write_transaction_response(response)?)?;
                writeln!(writer, "Validator ID: {validator_id}")?;
                writeln!(writer, "Validator Cap ID: {validator_cap_id}")?;
                writeln!(
                    writer,
                    "Validator Operation Cap ID: {validator_operation_cap_id}"
                )?;
                writeln!(
                    writer,
                    "Validator Commission Cap ID: {validator_commission_cap_id}"
                )?;
            }
            IkaValidatorCommandResponse::JoinCommittee(response)
            | IkaValidatorCommandResponse::StakeValidator(response)
            | IkaValidatorCommandResponse::LeaveCommittee(response)
            | IkaValidatorCommandResponse::RemoveCandidate(response)
            | IkaValidatorCommandResponse::SetCommission(response)
            | IkaValidatorCommandResponse::WithdrawStake(response)
            | IkaValidatorCommandResponse::RequestWithdrawStake(response)
            | IkaValidatorCommandResponse::ReportValidator(response)
            | IkaValidatorCommandResponse::UndoReportValidator(response)
            | IkaValidatorCommandResponse::RotateOperationCap(response)
            | IkaValidatorCommandResponse::RotateCommissionCap(response)
            | IkaValidatorCommandResponse::CollectCommission(response)
            | IkaValidatorCommandResponse::SetValidatorName(response)
            | IkaValidatorCommandResponse::GetValidatorMetadata(response)
            | IkaValidatorCommandResponse::SetValidatorMetadata(response)
            | IkaValidatorCommandResponse::SetNextEpochNetworkAddress(response)
            | IkaValidatorCommandResponse::SetNextEpochP2pAddress(response)
            | IkaValidatorCommandResponse::SetNextEpochConsensusAddress(response)
            | IkaValidatorCommandResponse::SetNextEpochProtocolPubkey(response)
            | IkaValidatorCommandResponse::SetNextEpochNetworkPubkey(response)
            | IkaValidatorCommandResponse::SetNextEpochConsensusPubkey(response)
            | IkaValidatorCommandResponse::SetNextEpochClassGroupsPubkey(response)
            | IkaValidatorCommandResponse::VerifyValidatorCap(response)
            | IkaValidatorCommandResponse::VerifyOperationCap(response)
            | IkaValidatorCommandResponse::VerifyCommissionCap(response)
            | IkaValidatorCommandResponse::SetPricingVote(response) => {
                write!(writer, "{}", write_transaction_response(response)?)?;
            }
            IkaValidatorCommandResponse::ConfigEnv(path) => {
                writeln!(writer, "Ika Sui config file created at: {path:?}")?;
            }
            IkaValidatorCommandResponse::FetchCurrentPricingInfo(path) => {
                writeln!(
                    writer,
                    "Fetched current pricing info from Sui, you can view & edit it at: {path:?}"
                )?;
            }
        }
        write!(f, "{}", writer.trim_end_matches('\n'))
    }
}

impl Debug for IkaValidatorCommandResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = serde_json::to_string_pretty(self);
        let s = string.unwrap_or_else(|err| format!("{err}").red().to_string());
        write!(f, "{s}")
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
                    format!("{other:?}")
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
        println!("Use existing {file_name:?} key file.");
        return Ok(());
    } else if is_protocol_key {
        let (_, keypair) = get_authority_key_pair();
        write_authority_keypair_to_file(&keypair, file_name.clone())?;
        println!("Generated new key file: {file_name:?}.");
    } else {
        let kp = match key {
            Some(key) => {
                println!("Generated a new key file {file_name:?} based on `sui.keystore` file.");
                key
            }
            None => {
                let (_, kp, _, _) = generate_new_key(SignatureScheme::ED25519, None, None)?;
                println!("Generated new key file: {file_name:?}.");
                kp
            }
        };
        write_keypair_to_file(&kp, &file_name)?;
    }
    Ok(())
}

/// Generates the class groups a key pair and proof from a seed file if it exists,
/// otherwise generates and saves the seed.
fn read_or_generate_seed_and_class_groups_key(
    seed_path: PathBuf,
) -> Result<Box<ClassGroupsKeyPairAndProof>> {
    let seed = match RootSeed::from_file(seed_path.clone()) {
        Ok(seed) => {
            println!("Use existing seed: {seed_path:?}.",);
            seed
        }
        Err(_) => {
            let seed = RootSeed::random_seed();
            seed.save_to_file(seed_path.clone())?;
            println!("Generated class groups seed info file: {seed_path:?}.",);
            seed
        }
    };

    let class_groups_public_key_and_proof = Box::new(ClassGroupsKeyPairAndProof::from_seed(&seed));

    Ok(class_groups_public_key_and_proof)
}
