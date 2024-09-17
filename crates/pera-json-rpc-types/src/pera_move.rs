// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use colored::Colorize;
use itertools::Itertools;
use move_binary_format::file_format::{Ability, AbilitySet, DatatypeTyParameter, Visibility};
use move_binary_format::normalized::{
    Field as NormalizedField, Function as PeraNormalizedFunction, Module as NormalizedModule,
    Struct as NormalizedStruct, Type as NormalizedType,
};
use move_core_types::annotated_value::{MoveStruct, MoveValue, MoveVariant};
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::StructTag;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_with::serde_as;
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Display, Formatter, Write};
use pera_macros::EnumVariantOrder;
use tracing::warn;

use pera_types::base_types::{ObjectID, PeraAddress};
use pera_types::pera_serde::PeraStructTag;

pub type PeraMoveTypeParameterIndex = u16;

#[cfg(test)]
#[path = "unit_tests/pera_move_tests.rs"]
mod pera_move_tests;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum PeraMoveAbility {
    Copy,
    Drop,
    Store,
    Key,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct PeraMoveAbilitySet {
    pub abilities: Vec<PeraMoveAbility>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum PeraMoveVisibility {
    Private,
    Public,
    Friend,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PeraMoveStructTypeParameter {
    pub constraints: PeraMoveAbilitySet,
    pub is_phantom: bool,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct PeraMoveNormalizedField {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: PeraMoveNormalizedType,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PeraMoveNormalizedStruct {
    pub abilities: PeraMoveAbilitySet,
    pub type_parameters: Vec<PeraMoveStructTypeParameter>,
    pub fields: Vec<PeraMoveNormalizedField>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum PeraMoveNormalizedType {
    Bool,
    U8,
    U16,
    U32,
    U64,
    U128,
    U256,
    Address,
    Signer,
    #[serde(rename_all = "camelCase")]
    Struct {
        address: String,
        module: String,
        name: String,
        type_arguments: Vec<PeraMoveNormalizedType>,
    },
    Vector(Box<PeraMoveNormalizedType>),
    TypeParameter(PeraMoveTypeParameterIndex),
    Reference(Box<PeraMoveNormalizedType>),
    MutableReference(Box<PeraMoveNormalizedType>),
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PeraMoveNormalizedFunction {
    pub visibility: PeraMoveVisibility,
    pub is_entry: bool,
    pub type_parameters: Vec<PeraMoveAbilitySet>,
    pub parameters: Vec<PeraMoveNormalizedType>,
    pub return_: Vec<PeraMoveNormalizedType>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct PeraMoveModuleId {
    address: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PeraMoveNormalizedModule {
    pub file_format_version: u32,
    pub address: String,
    pub name: String,
    pub friends: Vec<PeraMoveModuleId>,
    pub structs: BTreeMap<String, PeraMoveNormalizedStruct>,
    pub exposed_functions: BTreeMap<String, PeraMoveNormalizedFunction>,
}

impl PartialEq for PeraMoveNormalizedModule {
    fn eq(&self, other: &Self) -> bool {
        self.file_format_version == other.file_format_version
            && self.address == other.address
            && self.name == other.name
    }
}

impl From<NormalizedModule> for PeraMoveNormalizedModule {
    fn from(module: NormalizedModule) -> Self {
        Self {
            file_format_version: module.file_format_version,
            address: module.address.to_hex_literal(),
            name: module.name.to_string(),
            friends: module
                .friends
                .into_iter()
                .map(|module_id| PeraMoveModuleId {
                    address: module_id.address().to_hex_literal(),
                    name: module_id.name().to_string(),
                })
                .collect::<Vec<PeraMoveModuleId>>(),
            structs: module
                .structs
                .into_iter()
                .map(|(name, struct_)| (name.to_string(), PeraMoveNormalizedStruct::from(struct_)))
                .collect::<BTreeMap<String, PeraMoveNormalizedStruct>>(),
            exposed_functions: module
                .functions
                .into_iter()
                .filter_map(|(name, function)| {
                    // TODO: Do we want to expose the private functions as well?
                    (function.is_entry || function.visibility != Visibility::Private)
                        .then(|| (name.to_string(), PeraMoveNormalizedFunction::from(function)))
                })
                .collect::<BTreeMap<String, PeraMoveNormalizedFunction>>(),
        }
    }
}

impl From<PeraNormalizedFunction> for PeraMoveNormalizedFunction {
    fn from(function: PeraNormalizedFunction) -> Self {
        Self {
            visibility: match function.visibility {
                Visibility::Private => PeraMoveVisibility::Private,
                Visibility::Public => PeraMoveVisibility::Public,
                Visibility::Friend => PeraMoveVisibility::Friend,
            },
            is_entry: function.is_entry,
            type_parameters: function
                .type_parameters
                .into_iter()
                .map(|a| a.into())
                .collect::<Vec<PeraMoveAbilitySet>>(),
            parameters: function
                .parameters
                .into_iter()
                .map(PeraMoveNormalizedType::from)
                .collect::<Vec<PeraMoveNormalizedType>>(),
            return_: function
                .return_
                .into_iter()
                .map(PeraMoveNormalizedType::from)
                .collect::<Vec<PeraMoveNormalizedType>>(),
        }
    }
}

impl From<NormalizedStruct> for PeraMoveNormalizedStruct {
    fn from(struct_: NormalizedStruct) -> Self {
        Self {
            abilities: struct_.abilities.into(),
            type_parameters: struct_
                .type_parameters
                .into_iter()
                .map(PeraMoveStructTypeParameter::from)
                .collect::<Vec<PeraMoveStructTypeParameter>>(),
            fields: struct_
                .fields
                .into_iter()
                .map(PeraMoveNormalizedField::from)
                .collect::<Vec<PeraMoveNormalizedField>>(),
        }
    }
}

impl From<DatatypeTyParameter> for PeraMoveStructTypeParameter {
    fn from(type_parameter: DatatypeTyParameter) -> Self {
        Self {
            constraints: type_parameter.constraints.into(),
            is_phantom: type_parameter.is_phantom,
        }
    }
}

impl From<NormalizedField> for PeraMoveNormalizedField {
    fn from(normalized_field: NormalizedField) -> Self {
        Self {
            name: normalized_field.name.to_string(),
            type_: PeraMoveNormalizedType::from(normalized_field.type_),
        }
    }
}

impl From<NormalizedType> for PeraMoveNormalizedType {
    fn from(type_: NormalizedType) -> Self {
        match type_ {
            NormalizedType::Bool => PeraMoveNormalizedType::Bool,
            NormalizedType::U8 => PeraMoveNormalizedType::U8,
            NormalizedType::U16 => PeraMoveNormalizedType::U16,
            NormalizedType::U32 => PeraMoveNormalizedType::U32,
            NormalizedType::U64 => PeraMoveNormalizedType::U64,
            NormalizedType::U128 => PeraMoveNormalizedType::U128,
            NormalizedType::U256 => PeraMoveNormalizedType::U256,
            NormalizedType::Address => PeraMoveNormalizedType::Address,
            NormalizedType::Signer => PeraMoveNormalizedType::Signer,
            NormalizedType::Struct {
                address,
                module,
                name,
                type_arguments,
            } => PeraMoveNormalizedType::Struct {
                address: address.to_hex_literal(),
                module: module.to_string(),
                name: name.to_string(),
                type_arguments: type_arguments
                    .into_iter()
                    .map(PeraMoveNormalizedType::from)
                    .collect::<Vec<PeraMoveNormalizedType>>(),
            },
            NormalizedType::Vector(v) => {
                PeraMoveNormalizedType::Vector(Box::new(PeraMoveNormalizedType::from(*v)))
            }
            NormalizedType::TypeParameter(t) => PeraMoveNormalizedType::TypeParameter(t),
            NormalizedType::Reference(r) => {
                PeraMoveNormalizedType::Reference(Box::new(PeraMoveNormalizedType::from(*r)))
            }
            NormalizedType::MutableReference(mr) => {
                PeraMoveNormalizedType::MutableReference(Box::new(PeraMoveNormalizedType::from(*mr)))
            }
        }
    }
}

impl From<AbilitySet> for PeraMoveAbilitySet {
    fn from(set: AbilitySet) -> PeraMoveAbilitySet {
        Self {
            abilities: set
                .into_iter()
                .map(|a| match a {
                    Ability::Copy => PeraMoveAbility::Copy,
                    Ability::Drop => PeraMoveAbility::Drop,
                    Ability::Key => PeraMoveAbility::Key,
                    Ability::Store => PeraMoveAbility::Store,
                })
                .collect::<Vec<PeraMoveAbility>>(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum ObjectValueKind {
    ByImmutableReference,
    ByMutableReference,
    ByValue,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum MoveFunctionArgType {
    Pure,
    Object(ObjectValueKind),
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, Eq, PartialEq, EnumVariantOrder)]
#[serde(untagged, rename = "MoveValue")]
pub enum PeraMoveValue {
    // u64 and u128 are converted to String to avoid overflow
    Number(u32),
    Bool(bool),
    Address(PeraAddress),
    Vector(Vec<PeraMoveValue>),
    String(String),
    UID { id: ObjectID },
    Struct(PeraMoveStruct),
    Option(Box<Option<PeraMoveValue>>),
    Variant(PeraMoveVariant),
}

impl PeraMoveValue {
    /// Extract values from MoveValue without type information in json format
    pub fn to_json_value(self) -> Value {
        match self {
            PeraMoveValue::Struct(move_struct) => move_struct.to_json_value(),
            PeraMoveValue::Vector(values) => PeraMoveStruct::Runtime(values).to_json_value(),
            PeraMoveValue::Number(v) => json!(v),
            PeraMoveValue::Bool(v) => json!(v),
            PeraMoveValue::Address(v) => json!(v),
            PeraMoveValue::String(v) => json!(v),
            PeraMoveValue::UID { id } => json!({ "id": id }),
            PeraMoveValue::Option(v) => json!(v),
            PeraMoveValue::Variant(v) => v.to_json_value(),
        }
    }
}

impl Display for PeraMoveValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        match self {
            PeraMoveValue::Number(value) => write!(writer, "{}", value)?,
            PeraMoveValue::Bool(value) => write!(writer, "{}", value)?,
            PeraMoveValue::Address(value) => write!(writer, "{}", value)?,
            PeraMoveValue::String(value) => write!(writer, "{}", value)?,
            PeraMoveValue::UID { id } => write!(writer, "{id}")?,
            PeraMoveValue::Struct(value) => write!(writer, "{}", value)?,
            PeraMoveValue::Option(value) => write!(writer, "{:?}", value)?,
            PeraMoveValue::Vector(vec) => {
                write!(
                    writer,
                    "{}",
                    vec.iter().map(|value| format!("{value}")).join(",\n")
                )?;
            }
            PeraMoveValue::Variant(value) => write!(writer, "{}", value)?,
        }
        write!(f, "{}", writer.trim_end_matches('\n'))
    }
}

impl From<MoveValue> for PeraMoveValue {
    fn from(value: MoveValue) -> Self {
        match value {
            MoveValue::U8(value) => PeraMoveValue::Number(value.into()),
            MoveValue::U16(value) => PeraMoveValue::Number(value.into()),
            MoveValue::U32(value) => PeraMoveValue::Number(value),
            MoveValue::U64(value) => PeraMoveValue::String(format!("{value}")),
            MoveValue::U128(value) => PeraMoveValue::String(format!("{value}")),
            MoveValue::U256(value) => PeraMoveValue::String(format!("{value}")),
            MoveValue::Bool(value) => PeraMoveValue::Bool(value),
            MoveValue::Vector(values) => {
                PeraMoveValue::Vector(values.into_iter().map(|value| value.into()).collect())
            }
            MoveValue::Struct(value) => {
                // Best effort Pera core type conversion
                let MoveStruct { type_, fields } = &value;
                if let Some(value) = try_convert_type(type_, fields) {
                    return value;
                }
                PeraMoveValue::Struct(value.into())
            }
            MoveValue::Signer(value) | MoveValue::Address(value) => {
                PeraMoveValue::Address(PeraAddress::from(ObjectID::from(value)))
            }
            MoveValue::Variant(MoveVariant {
                type_,
                variant_name,
                tag: _,
                fields,
            }) => PeraMoveValue::Variant(PeraMoveVariant {
                type_: type_.clone(),
                variant: variant_name.to_string(),
                fields: fields
                    .into_iter()
                    .map(|(id, value)| (id.into_string(), value.into()))
                    .collect::<BTreeMap<_, _>>(),
            }),
        }
    }
}

fn to_bytearray(value: &[MoveValue]) -> Option<Vec<u8>> {
    if value.iter().all(|value| matches!(value, MoveValue::U8(_))) {
        let bytearray = value
            .iter()
            .flat_map(|value| {
                if let MoveValue::U8(u8) = value {
                    Some(*u8)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        Some(bytearray)
    } else {
        None
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, Eq, PartialEq)]
#[serde(rename = "MoveVariant")]
pub struct PeraMoveVariant {
    #[schemars(with = "String")]
    #[serde(rename = "type")]
    #[serde_as(as = "PeraStructTag")]
    pub type_: StructTag,
    pub variant: String,
    pub fields: BTreeMap<String, PeraMoveValue>,
}

impl PeraMoveVariant {
    pub fn to_json_value(self) -> Value {
        // We only care about values here, assuming type information is known at the client side.
        let fields = self
            .fields
            .into_iter()
            .map(|(key, value)| (key, value.to_json_value()))
            .collect::<BTreeMap<_, _>>();
        json!({
            "variant": self.variant,
            "fields": fields,
        })
    }
}

impl Display for PeraMoveVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        let PeraMoveVariant {
            type_,
            variant,
            fields,
        } = self;
        writeln!(writer)?;
        writeln!(writer, "  {}: {type_}", "type".bold().bright_black())?;
        writeln!(writer, "  {}: {variant}", "variant".bold().bright_black())?;
        for (name, value) in fields {
            let value = format!("{}", value);
            let value = if value.starts_with('\n') {
                indent(&value, 2)
            } else {
                value
            };
            writeln!(writer, "  {}: {value}", name.bold().bright_black())?;
        }

        write!(f, "{}", writer.trim_end_matches('\n'))
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, Eq, PartialEq, EnumVariantOrder)]
#[serde(untagged, rename = "MoveStruct")]
pub enum PeraMoveStruct {
    Runtime(Vec<PeraMoveValue>),
    WithTypes {
        #[schemars(with = "String")]
        #[serde(rename = "type")]
        #[serde_as(as = "PeraStructTag")]
        type_: StructTag,
        fields: BTreeMap<String, PeraMoveValue>,
    },
    WithFields(BTreeMap<String, PeraMoveValue>),
}

impl PeraMoveStruct {
    /// Extract values from MoveStruct without type information in json format
    pub fn to_json_value(self) -> Value {
        // Unwrap MoveStructs
        match self {
            PeraMoveStruct::Runtime(values) => {
                let values = values
                    .into_iter()
                    .map(|value| value.to_json_value())
                    .collect::<Vec<_>>();
                json!(values)
            }
            // We only care about values here, assuming struct type information is known at the client side.
            PeraMoveStruct::WithTypes { type_: _, fields } | PeraMoveStruct::WithFields(fields) => {
                let fields = fields
                    .into_iter()
                    .map(|(key, value)| (key, value.to_json_value()))
                    .collect::<BTreeMap<_, _>>();
                json!(fields)
            }
        }
    }

    pub fn field_value(&self, field_name: &str) -> Option<PeraMoveValue> {
        match self {
            PeraMoveStruct::WithFields(fields) => fields.get(field_name).cloned(),
            PeraMoveStruct::WithTypes { type_: _, fields } => fields.get(field_name).cloned(),
            _ => None,
        }
    }
}

impl Display for PeraMoveStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        match self {
            PeraMoveStruct::Runtime(_) => {}
            PeraMoveStruct::WithFields(fields) => {
                for (name, value) in fields {
                    writeln!(writer, "{}: {value}", name.bold().bright_black())?;
                }
            }
            PeraMoveStruct::WithTypes { type_, fields } => {
                writeln!(writer)?;
                writeln!(writer, "  {}: {type_}", "type".bold().bright_black())?;
                for (name, value) in fields {
                    let value = format!("{}", value);
                    let value = if value.starts_with('\n') {
                        indent(&value, 2)
                    } else {
                        value
                    };
                    writeln!(writer, "  {}: {value}", name.bold().bright_black())?;
                }
            }
        }
        write!(f, "{}", writer.trim_end_matches('\n'))
    }
}

fn indent<T: Display>(d: &T, indent: usize) -> String {
    d.to_string()
        .lines()
        .map(|line| format!("{:indent$}{}", "", line))
        .join("\n")
}

fn try_convert_type(type_: &StructTag, fields: &[(Identifier, MoveValue)]) -> Option<PeraMoveValue> {
    let struct_name = format!(
        "0x{}::{}::{}",
        type_.address.short_str_lossless(),
        type_.module,
        type_.name
    );
    let mut values = fields
        .iter()
        .map(|(id, value)| (id.to_string(), value))
        .collect::<BTreeMap<_, _>>();
    match struct_name.as_str() {
        "0x1::string::String" | "0x1::ascii::String" => {
            if let Some(MoveValue::Vector(bytes)) = values.remove("bytes") {
                return to_bytearray(bytes)
                    .and_then(|bytes| String::from_utf8(bytes).ok())
                    .map(PeraMoveValue::String);
            }
        }
        "0x2::url::Url" => {
            return values.remove("url").cloned().map(PeraMoveValue::from);
        }
        "0x2::object::ID" => {
            return values.remove("bytes").cloned().map(PeraMoveValue::from);
        }
        "0x2::object::UID" => {
            let id = values.remove("id").cloned().map(PeraMoveValue::from);
            if let Some(PeraMoveValue::Address(address)) = id {
                return Some(PeraMoveValue::UID {
                    id: ObjectID::from(address),
                });
            }
        }
        "0x2::balance::Balance" => {
            return values.remove("value").cloned().map(PeraMoveValue::from);
        }
        "0x1::option::Option" => {
            if let Some(MoveValue::Vector(values)) = values.remove("vec") {
                return Some(PeraMoveValue::Option(Box::new(
                    // in Move option is modeled as vec of 1 element
                    values.first().cloned().map(PeraMoveValue::from),
                )));
            }
        }
        _ => return None,
    }
    warn!(
        fields =? fields,
        "Failed to convert {struct_name} to PeraMoveValue"
    );
    None
}

impl From<MoveStruct> for PeraMoveStruct {
    fn from(move_struct: MoveStruct) -> Self {
        PeraMoveStruct::WithTypes {
            type_: move_struct.type_,
            fields: move_struct
                .fields
                .into_iter()
                .map(|(id, value)| (id.into_string(), value.into()))
                .collect(),
        }
    }
}
