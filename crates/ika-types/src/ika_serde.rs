// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::fmt::Debug;
use std::fmt::Write;

use ika_protocol_config::ProtocolVersion;
use schemars::JsonSchema;
use serde;
use serde::de::Deserializer;
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::{DeserializeAs, SerializeAs};
use sui_types::sui_serde::BigInt;

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
