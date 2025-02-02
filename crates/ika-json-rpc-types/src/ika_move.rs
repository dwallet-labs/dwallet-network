// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use colored::Colorize;
use itertools::Itertools;
use move_binary_format::file_format::{Ability, AbilitySet, DatatypeTyParameter, Visibility};
use move_binary_format::normalized::{
    Enum as NormalizedEnum, Field as NormalizedField, Function as IkaNormalizedFunction,
    Module as NormalizedModule, Struct as NormalizedStruct, Type as NormalizedType,
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
use ika_macros::EnumVariantOrder;
use tracing::warn;

use ika_types::base_types::{ObjectID, IkaAddress};
use ika_types::ika_serde::IkaStructTag;

pub type IkaMoveTypeParameterIndex = u16;

#[cfg(test)]
#[path = "unit_tests/ika_move_tests.rs"]
mod ika_move_tests;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum IkaMoveAbility {
    Copy,
    Drop,
    Store,
    Key,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct IkaMoveAbilitySet {
    pub abilities: Vec<IkaMoveAbility>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum IkaMoveVisibility {
    Private,
    Public,
    Friend,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IkaMoveStructTypeParameter {
    pub constraints: IkaMoveAbilitySet,
    pub is_phantom: bool,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct IkaMoveNormalizedField {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: IkaMoveNormalizedType,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IkaMoveNormalizedStruct {
    pub abilities: IkaMoveAbilitySet,
    pub type_parameters: Vec<IkaMoveStructTypeParameter>,
    pub fields: Vec<IkaMoveNormalizedField>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IkaMoveNormalizedEnum {
    pub abilities: IkaMoveAbilitySet,
    pub type_parameters: Vec<IkaMoveStructTypeParameter>,
    pub variants: BTreeMap<String, Vec<IkaMoveNormalizedField>>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum IkaMoveNormalizedType {
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
        type_arguments: Vec<IkaMoveNormalizedType>,
    },
    Vector(Box<IkaMoveNormalizedType>),
    TypeParameter(IkaMoveTypeParameterIndex),
    Reference(Box<IkaMoveNormalizedType>),
    MutableReference(Box<IkaMoveNormalizedType>),
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IkaMoveNormalizedFunction {
    pub visibility: IkaMoveVisibility,
    pub is_entry: bool,
    pub type_parameters: Vec<IkaMoveAbilitySet>,
    pub parameters: Vec<IkaMoveNormalizedType>,
    pub return_: Vec<IkaMoveNormalizedType>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct IkaMoveModuleId {
    address: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IkaMoveNormalizedModule {
    pub file_format_version: u32,
    pub address: String,
    pub name: String,
    pub friends: Vec<IkaMoveModuleId>,
    pub structs: BTreeMap<String, IkaMoveNormalizedStruct>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub enums: BTreeMap<String, IkaMoveNormalizedEnum>,
    pub exposed_functions: BTreeMap<String, IkaMoveNormalizedFunction>,
}

impl PartialEq for IkaMoveNormalizedModule {
    fn eq(&self, other: &Self) -> bool {
        self.file_format_version == other.file_format_version
            && self.address == other.address
            && self.name == other.name
    }
}

impl From<NormalizedModule> for IkaMoveNormalizedModule {
    fn from(module: NormalizedModule) -> Self {
        Self {
            file_format_version: module.file_format_version,
            address: module.address.to_hex_literal(),
            name: module.name.to_string(),
            friends: module
                .friends
                .into_iter()
                .map(|module_id| IkaMoveModuleId {
                    address: module_id.address().to_hex_literal(),
                    name: module_id.name().to_string(),
                })
                .collect::<Vec<IkaMoveModuleId>>(),
            structs: module
                .structs
                .into_iter()
                .map(|(name, struct_)| (name.to_string(), IkaMoveNormalizedStruct::from(struct_)))
                .collect::<BTreeMap<String, IkaMoveNormalizedStruct>>(),
            enums: module
                .enums
                .into_iter()
                .map(|(name, enum_)| (name.to_string(), IkaMoveNormalizedEnum::from(enum_)))
                .collect(),
            exposed_functions: module
                .functions
                .into_iter()
                .filter_map(|(name, function)| {
                    // TODO: Do we want to expose the private functions as well?
                    (function.is_entry || function.visibility != Visibility::Private)
                        .then(|| (name.to_string(), IkaMoveNormalizedFunction::from(function)))
                })
                .collect::<BTreeMap<String, IkaMoveNormalizedFunction>>(),
        }
    }
}

impl From<IkaNormalizedFunction> for IkaMoveNormalizedFunction {
    fn from(function: IkaNormalizedFunction) -> Self {
        Self {
            visibility: match function.visibility {
                Visibility::Private => IkaMoveVisibility::Private,
                Visibility::Public => IkaMoveVisibility::Public,
                Visibility::Friend => IkaMoveVisibility::Friend,
            },
            is_entry: function.is_entry,
            type_parameters: function
                .type_parameters
                .into_iter()
                .map(|a| a.into())
                .collect::<Vec<IkaMoveAbilitySet>>(),
            parameters: function
                .parameters
                .into_iter()
                .map(IkaMoveNormalizedType::from)
                .collect::<Vec<IkaMoveNormalizedType>>(),
            return_: function
                .return_
                .into_iter()
                .map(IkaMoveNormalizedType::from)
                .collect::<Vec<IkaMoveNormalizedType>>(),
        }
    }
}

impl From<NormalizedStruct> for IkaMoveNormalizedStruct {
    fn from(struct_: NormalizedStruct) -> Self {
        Self {
            abilities: struct_.abilities.into(),
            type_parameters: struct_
                .type_parameters
                .into_iter()
                .map(IkaMoveStructTypeParameter::from)
                .collect::<Vec<IkaMoveStructTypeParameter>>(),
            fields: struct_
                .fields
                .into_iter()
                .map(IkaMoveNormalizedField::from)
                .collect::<Vec<IkaMoveNormalizedField>>(),
        }
    }
}

impl From<NormalizedEnum> for IkaMoveNormalizedEnum {
    fn from(value: NormalizedEnum) -> Self {
        Self {
            abilities: value.abilities.into(),
            type_parameters: value
                .type_parameters
                .into_iter()
                .map(IkaMoveStructTypeParameter::from)
                .collect::<Vec<IkaMoveStructTypeParameter>>(),
            variants: value
                .variants
                .into_iter()
                .map(|variant| {
                    (
                        variant.name.to_string(),
                        variant
                            .fields
                            .into_iter()
                            .map(IkaMoveNormalizedField::from)
                            .collect::<Vec<IkaMoveNormalizedField>>(),
                    )
                })
                .collect::<BTreeMap<String, Vec<IkaMoveNormalizedField>>>(),
        }
    }
}

impl From<DatatypeTyParameter> for IkaMoveStructTypeParameter {
    fn from(type_parameter: DatatypeTyParameter) -> Self {
        Self {
            constraints: type_parameter.constraints.into(),
            is_phantom: type_parameter.is_phantom,
        }
    }
}

impl From<NormalizedField> for IkaMoveNormalizedField {
    fn from(normalized_field: NormalizedField) -> Self {
        Self {
            name: normalized_field.name.to_string(),
            type_: IkaMoveNormalizedType::from(normalized_field.type_),
        }
    }
}

impl From<NormalizedType> for IkaMoveNormalizedType {
    fn from(type_: NormalizedType) -> Self {
        match type_ {
            NormalizedType::Bool => IkaMoveNormalizedType::Bool,
            NormalizedType::U8 => IkaMoveNormalizedType::U8,
            NormalizedType::U16 => IkaMoveNormalizedType::U16,
            NormalizedType::U32 => IkaMoveNormalizedType::U32,
            NormalizedType::U64 => IkaMoveNormalizedType::U64,
            NormalizedType::U128 => IkaMoveNormalizedType::U128,
            NormalizedType::U256 => IkaMoveNormalizedType::U256,
            NormalizedType::Address => IkaMoveNormalizedType::Address,
            NormalizedType::Signer => IkaMoveNormalizedType::Signer,
            NormalizedType::Struct {
                address,
                module,
                name,
                type_arguments,
            } => IkaMoveNormalizedType::Struct {
                address: address.to_hex_literal(),
                module: module.to_string(),
                name: name.to_string(),
                type_arguments: type_arguments
                    .into_iter()
                    .map(IkaMoveNormalizedType::from)
                    .collect::<Vec<IkaMoveNormalizedType>>(),
            },
            NormalizedType::Vector(v) => {
                IkaMoveNormalizedType::Vector(Box::new(IkaMoveNormalizedType::from(*v)))
            }
            NormalizedType::TypeParameter(t) => IkaMoveNormalizedType::TypeParameter(t),
            NormalizedType::Reference(r) => {
                IkaMoveNormalizedType::Reference(Box::new(IkaMoveNormalizedType::from(*r)))
            }
            NormalizedType::MutableReference(mr) => {
                IkaMoveNormalizedType::MutableReference(Box::new(IkaMoveNormalizedType::from(*mr)))
            }
        }
    }
}

impl From<AbilitySet> for IkaMoveAbilitySet {
    fn from(set: AbilitySet) -> IkaMoveAbilitySet {
        Self {
            abilities: set
                .into_iter()
                .map(|a| match a {
                    Ability::Copy => IkaMoveAbility::Copy,
                    Ability::Drop => IkaMoveAbility::Drop,
                    Ability::Key => IkaMoveAbility::Key,
                    Ability::Store => IkaMoveAbility::Store,
                })
                .collect::<Vec<IkaMoveAbility>>(),
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
pub enum IkaMoveValue {
    // u64 and u128 are converted to String to avoid overflow
    Number(u32),
    Bool(bool),
    Address(IkaAddress),
    Vector(Vec<IkaMoveValue>),
    String(String),
    UID { id: ObjectID },
    Struct(IkaMoveStruct),
    Option(Box<Option<IkaMoveValue>>),
    Variant(IkaMoveVariant),
}

impl IkaMoveValue {
    /// Extract values from MoveValue without type information in json format
    pub fn to_json_value(self) -> Value {
        match self {
            IkaMoveValue::Struct(move_struct) => move_struct.to_json_value(),
            IkaMoveValue::Vector(values) => IkaMoveStruct::Runtime(values).to_json_value(),
            IkaMoveValue::Number(v) => json!(v),
            IkaMoveValue::Bool(v) => json!(v),
            IkaMoveValue::Address(v) => json!(v),
            IkaMoveValue::String(v) => json!(v),
            IkaMoveValue::UID { id } => json!({ "id": id }),
            IkaMoveValue::Option(v) => json!(v),
            IkaMoveValue::Variant(v) => v.to_json_value(),
        }
    }
}

impl Display for IkaMoveValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        match self {
            IkaMoveValue::Number(value) => write!(writer, "{}", value)?,
            IkaMoveValue::Bool(value) => write!(writer, "{}", value)?,
            IkaMoveValue::Address(value) => write!(writer, "{}", value)?,
            IkaMoveValue::String(value) => write!(writer, "{}", value)?,
            IkaMoveValue::UID { id } => write!(writer, "{id}")?,
            IkaMoveValue::Struct(value) => write!(writer, "{}", value)?,
            IkaMoveValue::Option(value) => write!(writer, "{:?}", value)?,
            IkaMoveValue::Vector(vec) => {
                write!(
                    writer,
                    "{}",
                    vec.iter().map(|value| format!("{value}")).join(",\n")
                )?;
            }
            IkaMoveValue::Variant(value) => write!(writer, "{}", value)?,
        }
        write!(f, "{}", writer.trim_end_matches('\n'))
    }
}

impl From<MoveValue> for IkaMoveValue {
    fn from(value: MoveValue) -> Self {
        match value {
            MoveValue::U8(value) => IkaMoveValue::Number(value.into()),
            MoveValue::U16(value) => IkaMoveValue::Number(value.into()),
            MoveValue::U32(value) => IkaMoveValue::Number(value),
            MoveValue::U64(value) => IkaMoveValue::String(format!("{value}")),
            MoveValue::U128(value) => IkaMoveValue::String(format!("{value}")),
            MoveValue::U256(value) => IkaMoveValue::String(format!("{value}")),
            MoveValue::Bool(value) => IkaMoveValue::Bool(value),
            MoveValue::Vector(values) => {
                IkaMoveValue::Vector(values.into_iter().map(|value| value.into()).collect())
            }
            MoveValue::Struct(value) => {
                // Best effort Ika core type conversion
                let MoveStruct { type_, fields } = &value;
                if let Some(value) = try_convert_type(type_, fields) {
                    return value;
                }
                IkaMoveValue::Struct(value.into())
            }
            MoveValue::Signer(value) | MoveValue::Address(value) => {
                IkaMoveValue::Address(IkaAddress::from(ObjectID::from(value)))
            }
            MoveValue::Variant(MoveVariant {
                type_,
                variant_name,
                tag: _,
                fields,
            }) => IkaMoveValue::Variant(IkaMoveVariant {
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
pub struct IkaMoveVariant {
    #[schemars(with = "String")]
    #[serde(rename = "type")]
    #[serde_as(as = "IkaStructTag")]
    pub type_: StructTag,
    pub variant: String,
    pub fields: BTreeMap<String, IkaMoveValue>,
}

impl IkaMoveVariant {
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

impl Display for IkaMoveVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        let IkaMoveVariant {
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
pub enum IkaMoveStruct {
    Runtime(Vec<IkaMoveValue>),
    WithTypes {
        #[schemars(with = "String")]
        #[serde(rename = "type")]
        #[serde_as(as = "IkaStructTag")]
        type_: StructTag,
        fields: BTreeMap<String, IkaMoveValue>,
    },
    WithFields(BTreeMap<String, IkaMoveValue>),
}

impl IkaMoveStruct {
    /// Extract values from MoveStruct without type information in json format
    pub fn to_json_value(self) -> Value {
        // Unwrap MoveStructs
        match self {
            IkaMoveStruct::Runtime(values) => {
                let values = values
                    .into_iter()
                    .map(|value| value.to_json_value())
                    .collect::<Vec<_>>();
                json!(values)
            }
            // We only care about values here, assuming struct type information is known at the client side.
            IkaMoveStruct::WithTypes { type_: _, fields } | IkaMoveStruct::WithFields(fields) => {
                let fields = fields
                    .into_iter()
                    .map(|(key, value)| (key, value.to_json_value()))
                    .collect::<BTreeMap<_, _>>();
                json!(fields)
            }
        }
    }

    pub fn field_value(&self, field_name: &str) -> Option<IkaMoveValue> {
        match self {
            IkaMoveStruct::WithFields(fields) => fields.get(field_name).cloned(),
            IkaMoveStruct::WithTypes { type_: _, fields } => fields.get(field_name).cloned(),
            _ => None,
        }
    }
}

impl Display for IkaMoveStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        match self {
            IkaMoveStruct::Runtime(_) => {}
            IkaMoveStruct::WithFields(fields) => {
                for (name, value) in fields {
                    writeln!(writer, "{}: {value}", name.bold().bright_black())?;
                }
            }
            IkaMoveStruct::WithTypes { type_, fields } => {
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

fn try_convert_type(type_: &StructTag, fields: &[(Identifier, MoveValue)]) -> Option<IkaMoveValue> {
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
                    .map(IkaMoveValue::String);
            }
        }
        "0x2::url::Url" => {
            return values.remove("url").cloned().map(IkaMoveValue::from);
        }
        "0x2::object::ID" => {
            return values.remove("bytes").cloned().map(IkaMoveValue::from);
        }
        "0x2::object::UID" => {
            let id = values.remove("id").cloned().map(IkaMoveValue::from);
            if let Some(IkaMoveValue::Address(address)) = id {
                return Some(IkaMoveValue::UID {
                    id: ObjectID::from(address),
                });
            }
        }
        "0x2::balance::Balance" => {
            return values.remove("value").cloned().map(IkaMoveValue::from);
        }
        "0x1::option::Option" => {
            if let Some(MoveValue::Vector(values)) = values.remove("vec") {
                return Some(IkaMoveValue::Option(Box::new(
                    // in Move option is modeled as vec of 1 element
                    values.first().cloned().map(IkaMoveValue::from),
                )));
            }
        }
        _ => return None,
    }
    warn!(
        fields =? fields,
        "Failed to convert {struct_name} to IkaMoveValue"
    );
    None
}

impl From<MoveStruct> for IkaMoveStruct {
    fn from(move_struct: MoveStruct) -> Self {
        IkaMoveStruct::WithTypes {
            type_: move_struct.type_,
            fields: move_struct
                .fields
                .into_iter()
                .map(|(id, value)| (id.into_string(), value.into()))
                .collect(),
        }
    }
}
