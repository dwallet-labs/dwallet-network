// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::test_adapter::{FakeID, IkaTestAdapter};
use anyhow::{bail, ensure};
use clap;
use clap::{Args, Parser};
use move_compiler::editions::Flavor;
use move_core_types::parsing::{
    parser::Parser as MoveCLParser,
    parser::{parse_u256, parse_u64},
    values::ValueToken,
    values::{ParsableValue, ParsedValue},
};
use move_core_types::runtime_value::{MoveStruct, MoveValue};
use move_core_types::u256::U256;
use move_symbol_pool::Symbol;
use move_transactional_test_runner::tasks::{RunCommand, SyntaxChoice};
use ika_graphql_rpc::test_infra::cluster::SnapshotLagConfig;
use ika_types::base_types::{SequenceNumber, IkaAddress};
use ika_types::move_package::UpgradePolicy;
use ika_types::object::{Object, Owner};
use ika_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use ika_types::transaction::{Argument, CallArg, ObjectArg};

pub const IKA_ARGS_LONG: &str = "ika-args";

#[derive(Clone, Debug, clap::Parser)]
pub struct IkaRunArgs {
    #[clap(long = "sender")]
    pub sender: Option<String>,
    #[clap(long = "gas-price")]
    pub gas_price: Option<u64>,
    #[clap(long = "summarize")]
    pub summarize: bool,
}

#[derive(Debug, clap::Parser, Default)]
pub struct IkaPublishArgs {
    #[clap(long = "sender")]
    pub sender: Option<String>,
    #[clap(long = "upgradeable", action = clap::ArgAction::SetTrue)]
    pub upgradeable: bool,
    #[clap(long = "dependencies", num_args(1..))]
    pub dependencies: Vec<String>,
    #[clap(long = "gas-price")]
    pub gas_price: Option<u64>,
}

#[derive(Debug, clap::Parser)]
pub struct IkaInitArgs {
    #[clap(long = "accounts", num_args(1..))]
    pub accounts: Option<Vec<String>>,
    #[clap(long = "protocol-version")]
    pub protocol_version: Option<u64>,
    #[clap(long = "max-gas")]
    pub max_gas: Option<u64>,
    #[clap(long = "shared-object-deletion")]
    pub shared_object_deletion: Option<bool>,
    #[clap(long = "simulator")]
    pub simulator: bool,
    #[clap(long = "custom-validator-account")]
    pub custom_validator_account: bool,
    #[clap(long = "reference-gas-price")]
    pub reference_gas_price: Option<u64>,
    #[clap(long = "default-gas-price")]
    pub default_gas_price: Option<u64>,
    #[clap(flatten)]
    pub snapshot_config: SnapshotLagConfig,
    #[clap(long = "flavor")]
    pub flavor: Option<Flavor>,
    /// The number of epochs to keep in the database. Epochs outside of this range will be pruned by
    /// the indexer.
    #[clap(long = "epochs-to-keep")]
    pub epochs_to_keep: Option<u64>,
}

#[derive(Debug, clap::Parser)]
pub struct ViewObjectCommand {
    #[clap(value_parser = parse_fake_id)]
    pub id: FakeID,
}

#[derive(Debug, clap::Parser)]
pub struct TransferObjectCommand {
    #[clap(value_parser = parse_fake_id)]
    pub id: FakeID,
    #[clap(long = "recipient")]
    pub recipient: String,
    #[clap(long = "sender")]
    pub sender: Option<String>,
    #[clap(long = "gas-budget")]
    pub gas_budget: Option<u64>,
    #[clap(long = "gas-price")]
    pub gas_price: Option<u64>,
}

#[derive(Debug, clap::Parser)]
pub struct ConsensusCommitPrologueCommand {
    #[clap(long = "timestamp-ms")]
    pub timestamp_ms: u64,
}

#[derive(Debug, clap::Parser)]
pub struct ProgrammableTransactionCommand {
    #[clap(long = "sender")]
    pub sender: Option<String>,
    #[clap(long = "sponsor")]
    pub sponsor: Option<String>,
    #[clap(long = "gas-budget")]
    pub gas_budget: Option<u64>,
    #[clap(long = "gas-price")]
    pub gas_price: Option<u64>,
    #[clap(long = "gas-payment", value_parser = parse_fake_id)]
    pub gas_payment: Option<FakeID>,
    #[clap(long = "dev-inspect")]
    pub dev_inspect: bool,
    #[clap(
        long = "inputs",
        value_parser = ParsedValue::<IkaExtraValueArgs>::parse,
        num_args(1..),
        action = clap::ArgAction::Append,
    )]
    pub inputs: Vec<ParsedValue<IkaExtraValueArgs>>,
}

#[derive(Debug, clap::Parser)]
pub struct UpgradePackageCommand {
    #[clap(long = "package")]
    pub package: String,
    #[clap(long = "upgrade-capability", value_parser = parse_fake_id)]
    pub upgrade_capability: FakeID,
    #[clap(long = "dependencies", num_args(1..))]
    pub dependencies: Vec<String>,
    #[clap(long = "sender")]
    pub sender: String,
    #[clap(long = "gas-budget")]
    pub gas_budget: Option<u64>,
    #[clap(long = "syntax")]
    pub syntax: Option<SyntaxChoice>,
    #[clap(long = "policy", default_value="compatible", value_parser = parse_policy)]
    pub policy: u8,
    #[clap(long = "gas-price")]
    pub gas_price: Option<u64>,
}

#[derive(Debug, clap::Parser)]
pub struct StagePackageCommand {
    #[clap(long = "syntax")]
    pub syntax: Option<SyntaxChoice>,
    #[clap(long = "dependencies", num_args(1..))]
    pub dependencies: Vec<String>,
}

#[derive(Debug, clap::Parser)]
pub struct SetAddressCommand {
    pub address: String,
    #[clap(value_parser = ParsedValue::<IkaExtraValueArgs>::parse)]
    pub input: ParsedValue<IkaExtraValueArgs>,
}

#[derive(Debug, clap::Parser)]
pub struct AdvanceClockCommand {
    #[clap(long = "duration-ns")]
    pub duration_ns: u64,
}

#[derive(Debug, clap::Parser)]
pub struct RunGraphqlCommand {
    #[clap(long = "show-usage")]
    pub show_usage: bool,
    #[clap(long = "show-headers")]
    pub show_headers: bool,
    #[clap(long = "show-service-version")]
    pub show_service_version: bool,
    #[clap(long, num_args(1..))]
    pub cursors: Vec<String>,
    #[clap(long)]
    pub wait_for_checkpoint_pruned: Option<u64>,
}

#[derive(Debug, clap::Parser)]
pub struct CreateCheckpointCommand {
    pub count: Option<u64>,
}

#[derive(Debug, clap::Parser)]
pub struct AdvanceEpochCommand {
    pub count: Option<u64>,
    #[clap(long = "create-random-state")]
    pub create_random_state: bool,
}

#[derive(Debug, clap::Parser)]
pub struct SetRandomStateCommand {
    #[clap(long = "randomness-round")]
    pub randomness_round: u64,
    #[clap(long = "random-bytes")]
    pub random_bytes: String,
    #[clap(long = "randomness-initial-version")]
    pub randomness_initial_version: u64,
}

#[derive(Debug)]
pub enum IkaSubcommand<ExtraValueArgs: ParsableValue, ExtraRunArgs: Parser> {
    ViewObject(ViewObjectCommand),
    TransferObject(TransferObjectCommand),
    ConsensusCommitPrologue(ConsensusCommitPrologueCommand),
    ProgrammableTransaction(ProgrammableTransactionCommand),
    UpgradePackage(UpgradePackageCommand),
    StagePackage(StagePackageCommand),
    SetAddress(SetAddressCommand),
    CreateCheckpoint(CreateCheckpointCommand),
    AdvanceEpoch(AdvanceEpochCommand),
    AdvanceClock(AdvanceClockCommand),
    SetRandomState(SetRandomStateCommand),
    ViewCheckpoint,
    RunGraphql(RunGraphqlCommand),
    Bench(RunCommand<ExtraValueArgs>, ExtraRunArgs),
}

impl<ExtraValueArgs: ParsableValue, ExtraRunArgs: Parser> clap::FromArgMatches
    for IkaSubcommand<ExtraValueArgs, ExtraRunArgs>
{
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        Ok(match matches.subcommand() {
            Some(("view-object", matches)) => {
                IkaSubcommand::ViewObject(ViewObjectCommand::from_arg_matches(matches)?)
            }
            Some(("transfer-object", matches)) => {
                IkaSubcommand::TransferObject(TransferObjectCommand::from_arg_matches(matches)?)
            }
            Some(("consensus-commit-prologue", matches)) => IkaSubcommand::ConsensusCommitPrologue(
                ConsensusCommitPrologueCommand::from_arg_matches(matches)?,
            ),
            Some(("programmable", matches)) => IkaSubcommand::ProgrammableTransaction(
                ProgrammableTransactionCommand::from_arg_matches(matches)?,
            ),
            Some(("upgrade", matches)) => {
                IkaSubcommand::UpgradePackage(UpgradePackageCommand::from_arg_matches(matches)?)
            }
            Some(("stage-package", matches)) => {
                IkaSubcommand::StagePackage(StagePackageCommand::from_arg_matches(matches)?)
            }
            Some(("set-address", matches)) => {
                IkaSubcommand::SetAddress(SetAddressCommand::from_arg_matches(matches)?)
            }
            Some(("create-checkpoint", matches)) => {
                IkaSubcommand::CreateCheckpoint(CreateCheckpointCommand::from_arg_matches(matches)?)
            }
            Some(("advance-epoch", matches)) => {
                IkaSubcommand::AdvanceEpoch(AdvanceEpochCommand::from_arg_matches(matches)?)
            }
            Some(("advance-clock", matches)) => {
                IkaSubcommand::AdvanceClock(AdvanceClockCommand::from_arg_matches(matches)?)
            }
            Some(("set-random-state", matches)) => {
                IkaSubcommand::SetRandomState(SetRandomStateCommand::from_arg_matches(matches)?)
            }
            Some(("view-checkpoint", _)) => IkaSubcommand::ViewCheckpoint,
            Some(("run-graphql", matches)) => {
                IkaSubcommand::RunGraphql(RunGraphqlCommand::from_arg_matches(matches)?)
            }
            Some(("bench", matches)) => IkaSubcommand::Bench(
                RunCommand::from_arg_matches(matches)?,
                ExtraRunArgs::from_arg_matches(matches)?,
            ),
            _ => {
                return Err(clap::Error::raw(
                    clap::error::ErrorKind::InvalidSubcommand,
                    "Invalid submcommand",
                ));
            }
        })
    }

    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        *self = Self::from_arg_matches(matches)?;
        Ok(())
    }
}

impl<ExtraValueArgs: ParsableValue, ExtraRunArgs: Parser> clap::CommandFactory
    for IkaSubcommand<ExtraValueArgs, ExtraRunArgs>
{
    fn command() -> clap::Command {
        clap::Command::new("ika_sub_command")
            .subcommand(ViewObjectCommand::command().name("view-object"))
            .subcommand(TransferObjectCommand::command().name("transfer-object"))
            .subcommand(ConsensusCommitPrologueCommand::command().name("consensus-commit-prologue"))
            .subcommand(ProgrammableTransactionCommand::command().name("programmable"))
            .subcommand(UpgradePackageCommand::command().name("upgrade"))
            .subcommand(StagePackageCommand::command().name("stage-package"))
            .subcommand(SetAddressCommand::command().name("set-address"))
            .subcommand(CreateCheckpointCommand::command().name("create-checkpoint"))
            .subcommand(AdvanceEpochCommand::command().name("advance-epoch"))
            .subcommand(AdvanceClockCommand::command().name("advance-clock"))
            .subcommand(SetRandomStateCommand::command().name("set-random-state"))
            .subcommand(clap::Command::new("view-checkpoint"))
            .subcommand(RunGraphqlCommand::command().name("run-graphql"))
            .subcommand(
                RunCommand::<ExtraValueArgs>::augment_args(ExtraRunArgs::command()).name("bench"),
            )
    }

    fn command_for_update() -> clap::Command {
        todo!()
    }
}

impl<ExtraValueArgs: ParsableValue, ExtraRunArgs: Parser> clap::Parser
    for IkaSubcommand<ExtraValueArgs, ExtraRunArgs>
{
}

#[derive(Clone, Debug)]
pub enum IkaExtraValueArgs {
    Object(FakeID, Option<SequenceNumber>),
    Digest(String),
    Receiving(FakeID, Option<SequenceNumber>),
    ImmShared(FakeID, Option<SequenceNumber>),
}

#[derive(Clone)]
pub enum IkaValue {
    MoveValue(MoveValue),
    Object(FakeID, Option<SequenceNumber>),
    ObjVec(Vec<(FakeID, Option<SequenceNumber>)>),
    Digest(String),
    Receiving(FakeID, Option<SequenceNumber>),
    ImmShared(FakeID, Option<SequenceNumber>),
}

impl IkaExtraValueArgs {
    fn parse_object_value<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        parser: &mut MoveCLParser<'a, ValueToken, I>,
    ) -> anyhow::Result<Self> {
        let (fake_id, version) = Self::parse_receiving_or_object_value(parser, "object")?;
        Ok(IkaExtraValueArgs::Object(fake_id, version))
    }

    fn parse_receiving_value<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        parser: &mut MoveCLParser<'a, ValueToken, I>,
    ) -> anyhow::Result<Self> {
        let (fake_id, version) = Self::parse_receiving_or_object_value(parser, "receiving")?;
        Ok(IkaExtraValueArgs::Receiving(fake_id, version))
    }

    fn parse_read_shared_value<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        parser: &mut MoveCLParser<'a, ValueToken, I>,
    ) -> anyhow::Result<Self> {
        let (fake_id, version) = Self::parse_receiving_or_object_value(parser, "immshared")?;
        Ok(IkaExtraValueArgs::ImmShared(fake_id, version))
    }

    fn parse_digest_value<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        parser: &mut MoveCLParser<'a, ValueToken, I>,
    ) -> anyhow::Result<Self> {
        let contents = parser.advance(ValueToken::Ident)?;
        ensure!(contents == "digest");
        parser.advance(ValueToken::LParen)?;
        let package = parser.advance(ValueToken::Ident)?;
        parser.advance(ValueToken::RParen)?;
        Ok(IkaExtraValueArgs::Digest(package.to_owned()))
    }

    fn parse_receiving_or_object_value<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        parser: &mut MoveCLParser<'a, ValueToken, I>,
        ident_name: &str,
    ) -> anyhow::Result<(FakeID, Option<SequenceNumber>)> {
        let contents = parser.advance(ValueToken::Ident)?;
        ensure!(contents == ident_name);
        parser.advance(ValueToken::LParen)?;
        let i_str = parser.advance(ValueToken::Number)?;
        let (i, _) = parse_u256(i_str)?;
        let fake_id = if let Some(ValueToken::Comma) = parser.peek_tok() {
            parser.advance(ValueToken::Comma)?;
            let j_str = parser.advance(ValueToken::Number)?;
            let (j, _) = parse_u64(j_str)?;
            if i > U256::from(u64::MAX) {
                bail!("Object ID too large")
            }
            FakeID::Enumerated(i.unchecked_as_u64(), j)
        } else {
            let mut u256_bytes = i.to_le_bytes().to_vec();
            u256_bytes.reverse();
            let address: IkaAddress = IkaAddress::from_bytes(&u256_bytes).unwrap();
            FakeID::Known(address.into())
        };
        parser.advance(ValueToken::RParen)?;
        let version = if let Some(ValueToken::AtSign) = parser.peek_tok() {
            parser.advance(ValueToken::AtSign)?;
            let v_str = parser.advance(ValueToken::Number)?;
            let (v, _) = parse_u64(v_str)?;
            Some(SequenceNumber::from_u64(v))
        } else {
            None
        };
        Ok((fake_id, version))
    }
}

impl IkaValue {
    fn assert_move_value(self) -> MoveValue {
        match self {
            IkaValue::MoveValue(v) => v,
            IkaValue::Object(_, _) => panic!("unexpected nested Ika object in args"),
            IkaValue::ObjVec(_) => panic!("unexpected nested Ika object vector in args"),
            IkaValue::Digest(_) => panic!("unexpected nested Ika package digest in args"),
            IkaValue::Receiving(_, _) => panic!("unexpected nested Ika receiving object in args"),
            IkaValue::ImmShared(_, _) => panic!("unexpected nested Ika shared object in args"),
        }
    }

    fn assert_object(self) -> (FakeID, Option<SequenceNumber>) {
        match self {
            IkaValue::MoveValue(_) => panic!("unexpected nested non-object value in args"),
            IkaValue::Object(id, version) => (id, version),
            IkaValue::ObjVec(_) => panic!("unexpected nested Ika object vector in args"),
            IkaValue::Digest(_) => panic!("unexpected nested Ika package digest in args"),
            IkaValue::Receiving(_, _) => panic!("unexpected nested Ika receiving object in args"),
            IkaValue::ImmShared(_, _) => panic!("unexpected nested Ika shared object in args"),
        }
    }

    fn resolve_object(
        fake_id: FakeID,
        version: Option<SequenceNumber>,
        test_adapter: &IkaTestAdapter,
    ) -> anyhow::Result<Object> {
        let id = match test_adapter.fake_to_real_object_id(fake_id) {
            Some(id) => id,
            None => bail!("INVALID TEST. Unknown object, object({})", fake_id),
        };
        let obj_res = if let Some(v) = version {
            ika_types::storage::ObjectStore::get_object_by_key(&*test_adapter.executor, &id, v)
        } else {
            ika_types::storage::ObjectStore::get_object(&*test_adapter.executor, &id)
        };
        let obj = match obj_res {
            Ok(Some(obj)) => obj,
            Err(_) | Ok(None) => bail!("INVALID TEST. Could not load object argument {}", id),
        };
        Ok(obj)
    }

    fn receiving_arg(
        fake_id: FakeID,
        version: Option<SequenceNumber>,
        test_adapter: &IkaTestAdapter,
    ) -> anyhow::Result<ObjectArg> {
        let obj = Self::resolve_object(fake_id, version, test_adapter)?;
        Ok(ObjectArg::Receiving(obj.compute_object_reference()))
    }

    fn read_shared_arg(
        fake_id: FakeID,
        version: Option<SequenceNumber>,
        test_adapter: &IkaTestAdapter,
    ) -> anyhow::Result<ObjectArg> {
        let obj = Self::resolve_object(fake_id, version, test_adapter)?;
        let id = obj.id();
        if let Owner::Shared {
            initial_shared_version,
        } = obj.owner
        {
            Ok(ObjectArg::SharedObject {
                id,
                initial_shared_version,
                mutable: false,
            })
        } else {
            bail!("{fake_id} is not a shared object.")
        }
    }

    fn object_arg(
        fake_id: FakeID,
        version: Option<SequenceNumber>,
        test_adapter: &IkaTestAdapter,
    ) -> anyhow::Result<ObjectArg> {
        let obj = Self::resolve_object(fake_id, version, test_adapter)?;
        let id = obj.id();
        match obj.owner {
            Owner::Shared {
                initial_shared_version,
            } => Ok(ObjectArg::SharedObject {
                id,
                initial_shared_version,
                mutable: true,
            }),
            Owner::AddressOwner(_) | Owner::ObjectOwner(_) | Owner::Immutable => {
                let obj_ref = obj.compute_object_reference();
                Ok(ObjectArg::ImmOrOwnedObject(obj_ref))
            }
        }
    }

    pub(crate) fn into_call_arg(self, test_adapter: &IkaTestAdapter) -> anyhow::Result<CallArg> {
        Ok(match self {
            IkaValue::Object(fake_id, version) => {
                CallArg::Object(Self::object_arg(fake_id, version, test_adapter)?)
            }
            IkaValue::MoveValue(v) => CallArg::Pure(v.simple_serialize().unwrap()),
            IkaValue::Receiving(fake_id, version) => {
                CallArg::Object(Self::receiving_arg(fake_id, version, test_adapter)?)
            }
            IkaValue::ImmShared(fake_id, version) => {
                CallArg::Object(Self::read_shared_arg(fake_id, version, test_adapter)?)
            }
            IkaValue::ObjVec(_) => bail!("obj vec is not supported as an input"),
            IkaValue::Digest(pkg) => {
                let pkg = Symbol::from(pkg);
                let Some(staged) = test_adapter.staged_modules.get(&pkg) else {
                    bail!("Unbound staged package '{pkg}'")
                };
                CallArg::Pure(bcs::to_bytes(&staged.digest).unwrap())
            }
        })
    }

    pub(crate) fn into_argument(
        self,
        builder: &mut ProgrammableTransactionBuilder,
        test_adapter: &IkaTestAdapter,
    ) -> anyhow::Result<Argument> {
        match self {
            IkaValue::ObjVec(vec) => builder.make_obj_vec(
                vec.iter()
                    .map(|(fake_id, version)| Self::object_arg(*fake_id, *version, test_adapter))
                    .collect::<Result<Vec<ObjectArg>, _>>()?,
            ),
            value => {
                let call_arg = value.into_call_arg(test_adapter)?;
                builder.input(call_arg)
            }
        }
    }
}

impl ParsableValue for IkaExtraValueArgs {
    type ConcreteValue = IkaValue;

    fn parse_value<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        parser: &mut MoveCLParser<'a, ValueToken, I>,
    ) -> Option<anyhow::Result<Self>> {
        match parser.peek()? {
            (ValueToken::Ident, "object") => Some(Self::parse_object_value(parser)),
            (ValueToken::Ident, "digest") => Some(Self::parse_digest_value(parser)),
            (ValueToken::Ident, "receiving") => Some(Self::parse_receiving_value(parser)),
            (ValueToken::Ident, "immshared") => Some(Self::parse_read_shared_value(parser)),
            _ => None,
        }
    }

    fn move_value_into_concrete(v: MoveValue) -> anyhow::Result<Self::ConcreteValue> {
        Ok(IkaValue::MoveValue(v))
    }

    fn concrete_vector(elems: Vec<Self::ConcreteValue>) -> anyhow::Result<Self::ConcreteValue> {
        if !elems.is_empty() && matches!(elems[0], IkaValue::Object(_, _)) {
            Ok(IkaValue::ObjVec(
                elems.into_iter().map(IkaValue::assert_object).collect(),
            ))
        } else {
            Ok(IkaValue::MoveValue(MoveValue::Vector(
                elems.into_iter().map(IkaValue::assert_move_value).collect(),
            )))
        }
    }

    fn concrete_struct(values: Vec<Self::ConcreteValue>) -> anyhow::Result<Self::ConcreteValue> {
        Ok(IkaValue::MoveValue(MoveValue::Struct(MoveStruct(
            values.into_iter().map(|v| v.assert_move_value()).collect(),
        ))))
    }

    fn into_concrete_value(
        self,
        _mapping: &impl Fn(&str) -> Option<move_core_types::account_address::AccountAddress>,
    ) -> anyhow::Result<Self::ConcreteValue> {
        match self {
            IkaExtraValueArgs::Object(id, version) => Ok(IkaValue::Object(id, version)),
            IkaExtraValueArgs::Digest(pkg) => Ok(IkaValue::Digest(pkg)),
            IkaExtraValueArgs::Receiving(id, version) => Ok(IkaValue::Receiving(id, version)),
            IkaExtraValueArgs::ImmShared(id, version) => Ok(IkaValue::ImmShared(id, version)),
        }
    }
}

fn parse_fake_id(s: &str) -> anyhow::Result<FakeID> {
    Ok(if let Some((s1, s2)) = s.split_once(',') {
        let (i, _) = parse_u64(s1)?;
        let (j, _) = parse_u64(s2)?;
        FakeID::Enumerated(i, j)
    } else {
        let (i, _) = parse_u256(s)?;
        let mut u256_bytes = i.to_le_bytes().to_vec();
        u256_bytes.reverse();
        let address: IkaAddress = IkaAddress::from_bytes(&u256_bytes).unwrap();
        FakeID::Known(address.into())
    })
}

fn parse_policy(x: &str) -> anyhow::Result<u8> {
    Ok(match x {
            "compatible" => UpgradePolicy::COMPATIBLE,
            "additive" => UpgradePolicy::ADDITIVE,
            "dep_only" => UpgradePolicy::DEP_ONLY,
        _ => bail!("Invalid upgrade policy {x}. Policy must be one of 'compatible', 'additive', or 'dep_only'")
    })
}
