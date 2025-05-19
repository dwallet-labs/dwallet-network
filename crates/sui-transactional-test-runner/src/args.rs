// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use anyhow::{bail, ensure};
use clap;
use move_command_line_common::parser::{parse_u256, parse_u64};
use move_command_line_common::values::{ParsableValue, ParsedValue};
use move_command_line_common::{parser::Parser as MoveCLParser, values::ValueToken};
use move_core_types::runtime_value::{MoveStruct, MoveValue};
use move_core_types::u256::U256;
use move_symbol_pool::Symbol;
use move_transactional_test_runner::tasks::SyntaxChoice;
use sui_types::base_types::{SequenceNumber, SuiAddress};
use sui_types::move_package::UpgradePolicy;
use sui_types::object::{Object, Owner};
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::{Argument, CallArg, ObjectArg};

use crate::test_adapter::{FakeID, SuiTestAdapter};

pub const SUI_ARGS_LONG: &str = "sui-args";

#[derive(Debug, clap::Parser)]
pub struct SuiRunArgs {
    #[arg(long = "sender")]
    pub sender: Option<String>,
    #[arg(long = "gas-price")]
    pub gas_price: Option<u64>,
    #[arg(long = "summarize")]
    pub summarize: bool,
}

#[derive(Debug, clap::Parser, Default)]
pub struct SuiPublishArgs {
    #[arg(long = "sender")]
    pub sender: Option<String>,
    #[arg(long = "upgradeable", action = clap::ArgAction::SetTrue)]
    pub upgradeable: bool,
    #[arg(long = "dependencies", num_args(1..))]
    pub dependencies: Vec<String>,
    #[arg(long = "gas-price")]
    pub gas_price: Option<u64>,
}

#[derive(Debug, clap::Parser)]
pub struct SuiInitArgs {
    #[arg(long = "accounts", num_args(1..))]
    pub accounts: Option<Vec<String>>,
    #[arg(long = "protocol-version")]
    pub protocol_version: Option<u64>,
    #[arg(long = "max-gas")]
    pub max_gas: Option<u64>,
    #[arg(long = "shared-object-deletion")]
    pub shared_object_deletion: Option<bool>,
    #[arg(long = "simulator")]
    pub simulator: bool,
    #[arg(long = "custom-validator-account")]
    pub custom_validator_account: bool,
    #[arg(long = "reference-gas-price")]
    pub reference_gas_price: Option<u64>,
    #[arg(long = "default-gas-price")]
    pub default_gas_price: Option<u64>,
}

#[derive(Debug, clap::Parser)]
pub struct ViewObjectCommand {
    #[command(value_parser = parse_fake_id)]
    pub id: FakeID,
}

#[derive(Debug, clap::Parser)]
pub struct TransferObjectCommand {
    #[command(value_parser = parse_fake_id)]
    pub id: FakeID,
    #[arg(long = "recipient")]
    pub recipient: String,
    #[arg(long = "sender")]
    pub sender: Option<String>,
    #[arg(long = "gas-budget")]
    pub gas_budget: Option<u64>,
    #[arg(long = "gas-price")]
    pub gas_price: Option<u64>,
}

#[derive(Debug, clap::Parser)]
pub struct ConsensusCommitPrologueCommand {
    #[arg(long = "timestamp-ms")]
    pub timestamp_ms: u64,
}

#[derive(Debug, clap::Parser)]
pub struct ProgrammableTransactionCommand {
    #[arg(long = "sender")]
    pub sender: Option<String>,
    #[arg(long = "gas-budget")]
    pub gas_budget: Option<u64>,
    #[arg(long = "gas-price")]
    pub gas_price: Option<u64>,
    #[arg(long = "dev-inspect")]
    pub dev_inspect: bool,
    #[arg(
        long = "inputs",
        value_parser = ParsedValue::<SuiExtraValueArgs>::parse,
        num_args(1..),
        action = clap::ArgAction::Append,
    )]
    pub inputs: Vec<ParsedValue<SuiExtraValueArgs>>,
}

#[derive(Debug, clap::Parser)]
pub struct UpgradePackageCommand {
    #[arg(long = "package")]
    pub package: String,
    #[arg(long = "upgrade-capability", value_parser = parse_fake_id)]
    pub upgrade_capability: FakeID,
    #[arg(long = "dependencies", num_args(1..))]
    pub dependencies: Vec<String>,
    #[arg(long = "sender")]
    pub sender: String,
    #[arg(long = "gas-budget")]
    pub gas_budget: Option<u64>,
    #[arg(long = "syntax")]
    pub syntax: Option<SyntaxChoice>,
    #[arg(long = "policy", default_value="compatible", value_parser = parse_policy)]
    pub policy: u8,
    #[arg(long = "gas-price")]
    pub gas_price: Option<u64>,
}

#[derive(Debug, clap::Parser)]
pub struct StagePackageCommand {
    #[arg(long = "syntax")]
    pub syntax: Option<SyntaxChoice>,
    #[arg(long = "dependencies", num_args(1..))]
    pub dependencies: Vec<String>,
}

#[derive(Debug, clap::Parser)]
pub struct SetAddressCommand {
    pub address: String,
    #[command(value_parser = ParsedValue::<SuiExtraValueArgs>::parse)]
    pub input: ParsedValue<SuiExtraValueArgs>,
}

#[derive(Debug, clap::Parser)]
pub struct AdvanceClockCommand {
    #[arg(long = "duration-ns")]
    pub duration_ns: u64,
}

#[derive(Debug, clap::Parser)]
pub struct RunGraphqlCommand {
    #[arg(long = "show-usage")]
    pub show_usage: bool,
    #[arg(long = "show-headers")]
    pub show_headers: bool,
    #[arg(long = "show-service-version")]
    pub show_service_version: bool,
    #[arg(long, num_args(1..))]
    pub variables: Vec<String>,
    #[arg(long, num_args(1..))]
    pub cursors: Vec<String>,
}

#[derive(Debug, clap::Parser)]
pub struct CreateCheckpointCommand {
    pub count: Option<u64>,
}

#[derive(Debug, clap::Parser)]
pub struct AdvanceEpochCommand {
    pub count: Option<u64>,
    #[arg(long = "create-random-state")]
    pub create_random_state: bool,
}

#[derive(Debug, clap::Parser)]
pub struct SetRandomStateCommand {
    #[arg(long = "randomness-round")]
    pub randomness_round: u64,
    #[arg(long = "random-bytes")]
    pub random_bytes: String,
    #[arg(long = "randomness-initial-version")]
    pub randomness_initial_version: u64,
}

#[derive(Debug, clap::Parser)]
pub enum SuiSubcommand {
    #[arg(name = "view-object")]
    ViewObject(ViewObjectCommand),
    #[arg(name = "transfer-object")]
    TransferObject(TransferObjectCommand),
    #[arg(name = "consensus-commit-prologue")]
    ConsensusCommitPrologue(ConsensusCommitPrologueCommand),
    #[arg(name = "programmable")]
    ProgrammableTransaction(ProgrammableTransactionCommand),
    #[arg(name = "upgrade")]
    UpgradePackage(UpgradePackageCommand),
    #[arg(name = "stage-package")]
    StagePackage(StagePackageCommand),
    #[arg(name = "set-address")]
    SetAddress(SetAddressCommand),
    #[arg(name = "create-checkpoint")]
    CreateCheckpoint(CreateCheckpointCommand),
    #[arg(name = "advance-epoch")]
    AdvanceEpoch(AdvanceEpochCommand),
    #[arg(name = "advance-clock")]
    AdvanceClock(AdvanceClockCommand),
    #[arg(name = "set-random-state")]
    SetRandomState(SetRandomStateCommand),
    #[arg(name = "view-checkpoint")]
    ViewCheckpoint,
    #[arg(name = "run-graphql")]
    RunGraphql(RunGraphqlCommand),
    #[arg(name = "view-graphql-variables")]
    ViewGraphqlVariables,
}

#[derive(Clone, Debug)]
pub enum SuiExtraValueArgs {
    Object(FakeID, Option<SequenceNumber>),
    Digest(String),
    Receiving(FakeID, Option<SequenceNumber>),
    ImmShared(FakeID, Option<SequenceNumber>),
}

pub enum SuiValue {
    MoveValue(MoveValue),
    Object(FakeID, Option<SequenceNumber>),
    ObjVec(Vec<(FakeID, Option<SequenceNumber>)>),
    Digest(String),
    Receiving(FakeID, Option<SequenceNumber>),
    ImmShared(FakeID, Option<SequenceNumber>),
}

impl SuiExtraValueArgs {
    fn parse_object_value<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        parser: &mut MoveCLParser<'a, ValueToken, I>,
    ) -> anyhow::Result<Self> {
        let (fake_id, version) = Self::parse_receiving_or_object_value(parser, "object")?;
        Ok(SuiExtraValueArgs::Object(fake_id, version))
    }

    fn parse_receiving_value<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        parser: &mut MoveCLParser<'a, ValueToken, I>,
    ) -> anyhow::Result<Self> {
        let (fake_id, version) = Self::parse_receiving_or_object_value(parser, "receiving")?;
        Ok(SuiExtraValueArgs::Receiving(fake_id, version))
    }

    fn parse_read_shared_value<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        parser: &mut MoveCLParser<'a, ValueToken, I>,
    ) -> anyhow::Result<Self> {
        let (fake_id, version) = Self::parse_receiving_or_object_value(parser, "immshared")?;
        Ok(SuiExtraValueArgs::ImmShared(fake_id, version))
    }

    fn parse_digest_value<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        parser: &mut MoveCLParser<'a, ValueToken, I>,
    ) -> anyhow::Result<Self> {
        let contents = parser.advance(ValueToken::Ident)?;
        ensure!(contents == "digest");
        parser.advance(ValueToken::LParen)?;
        let package = parser.advance(ValueToken::Ident)?;
        parser.advance(ValueToken::RParen)?;
        Ok(SuiExtraValueArgs::Digest(package.to_owned()))
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
            let address: SuiAddress = SuiAddress::from_bytes(&u256_bytes).unwrap();
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

impl SuiValue {
    fn assert_move_value(self) -> MoveValue {
        match self {
            SuiValue::MoveValue(v) => v,
            SuiValue::Object(_, _) => panic!("unexpected nested Sui object in args"),
            SuiValue::ObjVec(_) => panic!("unexpected nested Sui object vector in args"),
            SuiValue::Digest(_) => panic!("unexpected nested Sui package digest in args"),
            SuiValue::Receiving(_, _) => panic!("unexpected nested Sui receiving object in args"),
            SuiValue::ImmShared(_, _) => panic!("unexpected nested Sui shared object in args"),
        }
    }

    fn assert_object(self) -> (FakeID, Option<SequenceNumber>) {
        match self {
            SuiValue::MoveValue(_) => panic!("unexpected nested non-object value in args"),
            SuiValue::Object(id, version) => (id, version),
            SuiValue::ObjVec(_) => panic!("unexpected nested Sui object vector in args"),
            SuiValue::Digest(_) => panic!("unexpected nested Sui package digest in args"),
            SuiValue::Receiving(_, _) => panic!("unexpected nested Sui receiving object in args"),
            SuiValue::ImmShared(_, _) => panic!("unexpected nested Sui shared object in args"),
        }
    }

    fn resolve_object(
        fake_id: FakeID,
        version: Option<SequenceNumber>,
        test_adapter: &SuiTestAdapter,
    ) -> anyhow::Result<Object> {
        let id = match test_adapter.fake_to_real_object_id(fake_id) {
            Some(id) => id,
            None => bail!("INVALID TEST. Unknown object, object({})", fake_id),
        };
        let obj_res = if let Some(v) = version {
            sui_types::storage::ObjectStore::get_object_by_key(&*test_adapter.executor, &id, v)
        } else {
            sui_types::storage::ObjectStore::get_object(&*test_adapter.executor, &id)
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
        test_adapter: &SuiTestAdapter,
    ) -> anyhow::Result<ObjectArg> {
        let obj = Self::resolve_object(fake_id, version, test_adapter)?;
        Ok(ObjectArg::Receiving(obj.compute_object_reference()))
    }

    fn read_shared_arg(
        fake_id: FakeID,
        version: Option<SequenceNumber>,
        test_adapter: &SuiTestAdapter,
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
        test_adapter: &SuiTestAdapter,
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

    pub(crate) fn into_call_arg(self, test_adapter: &SuiTestAdapter) -> anyhow::Result<CallArg> {
        Ok(match self {
            SuiValue::Object(fake_id, version) => {
                CallArg::Object(Self::object_arg(fake_id, version, test_adapter)?)
            }
            SuiValue::MoveValue(v) => CallArg::Pure(v.simple_serialize().unwrap()),
            SuiValue::Receiving(fake_id, version) => {
                CallArg::Object(Self::receiving_arg(fake_id, version, test_adapter)?)
            }
            SuiValue::ImmShared(fake_id, version) => {
                CallArg::Object(Self::read_shared_arg(fake_id, version, test_adapter)?)
            }
            SuiValue::ObjVec(_) => bail!("obj vec is not supported as an input"),
            SuiValue::Digest(pkg) => {
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
        test_adapter: &SuiTestAdapter,
    ) -> anyhow::Result<Argument> {
        match self {
            SuiValue::ObjVec(vec) => builder.make_obj_vec(
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

impl ParsableValue for SuiExtraValueArgs {
    type ConcreteValue = SuiValue;

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
        Ok(SuiValue::MoveValue(v))
    }

    fn concrete_vector(elems: Vec<Self::ConcreteValue>) -> anyhow::Result<Self::ConcreteValue> {
        if !elems.is_empty() && matches!(elems[0], SuiValue::Object(_, _)) {
            Ok(SuiValue::ObjVec(
                elems.into_iter().map(SuiValue::assert_object).collect(),
            ))
        } else {
            Ok(SuiValue::MoveValue(MoveValue::Vector(
                elems.into_iter().map(SuiValue::assert_move_value).collect(),
            )))
        }
    }

    fn concrete_struct(values: Vec<Self::ConcreteValue>) -> anyhow::Result<Self::ConcreteValue> {
        Ok(SuiValue::MoveValue(MoveValue::Struct(MoveStruct(
            values.into_iter().map(|v| v.assert_move_value()).collect(),
        ))))
    }

    fn into_concrete_value(
        self,
        _mapping: &impl Fn(&str) -> Option<move_core_types::account_address::AccountAddress>,
    ) -> anyhow::Result<Self::ConcreteValue> {
        match self {
            SuiExtraValueArgs::Object(id, version) => Ok(SuiValue::Object(id, version)),
            SuiExtraValueArgs::Digest(pkg) => Ok(SuiValue::Digest(pkg)),
            SuiExtraValueArgs::Receiving(id, version) => Ok(SuiValue::Receiving(id, version)),
            SuiExtraValueArgs::ImmShared(id, version) => Ok(SuiValue::ImmShared(id, version)),
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
        let address: SuiAddress = SuiAddress::from_bytes(&u256_bytes).unwrap();
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
