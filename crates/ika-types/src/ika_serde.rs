// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::fmt;
use std::fmt::Write;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;
use std::str::FromStr;

use fastcrypto::encoding::Hex;
use schemars::JsonSchema;
use serde;
use serde::de::{Deserializer, Error};
use serde::ser::{Error as SerError, Serializer};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::DisplayFromStr;
use serde_with::{Bytes, DeserializeAs, SerializeAs};
use sui_types::sui_serde::BigInt;
use ika_protocol_config::ProtocolVersion;

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Copy, JsonSchema)]
#[serde(rename = "ProtocolVersion")]
pub struct AsProtocolVersion(#[schemars(with = "BigInt<u64>")] u64);

impl SerializeAs<ProtocolVersion> for AsProtocolVersion {
    fn serialize_as<S>(value: &ProtocolVersion, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = value.as_u64().to_string();
        s.serialize(serializer)
    }
}

impl<'de> DeserializeAs<'de, ProtocolVersion> for AsProtocolVersion {
    fn deserialize_as<D>(deserializer: D) -> Result<ProtocolVersion, D::Error>
    where
        D: Deserializer<'de>,
    {
        let b = BigInt::<u64>::deserialize(deserializer)?;
        Ok(ProtocolVersion::from(*b))
    }
}
