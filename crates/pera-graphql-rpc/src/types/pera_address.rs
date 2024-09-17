// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::str::FromStr;

use crate::error::Error;
use async_graphql::*;
use move_core_types::account_address::AccountAddress;
use pera_types::base_types::{ObjectID, PeraAddress as NativePeraAddress};
use serde::{Deserialize, Serialize};

const PERA_ADDRESS_LENGTH: usize = 32;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy)]
pub(crate) struct PeraAddress([u8; PERA_ADDRESS_LENGTH]);

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub(crate) enum FromStrError {
    #[error("Invalid PeraAddress. Missing 0x prefix.")]
    NoPrefix,

    #[error(
        "Expected PeraAddress string with between 1 and {} digits ({} bytes), received {0}",
        PERA_ADDRESS_LENGTH * 2,
        PERA_ADDRESS_LENGTH,
    )]
    WrongLength(usize),

    #[error("Invalid character {0:?} at position {1}")]
    BadHex(char, usize),
}

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub(crate) enum FromVecError {
    #[error(
        "Expected PeraAddress with {} bytes, received {0}",
        PERA_ADDRESS_LENGTH
    )]
    WrongLength(usize),
}

impl PeraAddress {
    pub fn from_array(arr: [u8; PERA_ADDRESS_LENGTH]) -> Self {
        PeraAddress(arr)
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self, FromVecError> {
        <[u8; PERA_ADDRESS_LENGTH]>::try_from(bytes.as_ref())
            .map_err(|_| FromVecError::WrongLength(bytes.as_ref().len()))
            .map(PeraAddress)
    }
}

#[Scalar(use_type_description = true)]
impl ScalarType for PeraAddress {
    fn parse(value: Value) -> InputValueResult<Self> {
        let Value::String(s) = value else {
            return Err(InputValueError::expected_type(value));
        };

        Ok(PeraAddress::from_str(&s)?)
    }

    fn to_value(&self) -> Value {
        Value::String(format!("0x{}", hex::encode(self.0)))
    }
}

impl Description for PeraAddress {
    fn description() -> &'static str {
        "String containing 32B hex-encoded address, with a leading \"0x\". Leading zeroes can be \
         omitted on input but will always appear in outputs (PeraAddress in output is guaranteed \
         to be 66 characters long)."
    }
}

impl TryFrom<Vec<u8>> for PeraAddress {
    type Error = FromVecError;

    fn try_from(bytes: Vec<u8>) -> Result<Self, FromVecError> {
        Self::from_bytes(bytes)
    }
}

impl From<AccountAddress> for PeraAddress {
    fn from(value: AccountAddress) -> Self {
        PeraAddress(value.into_bytes())
    }
}

impl From<PeraAddress> for AccountAddress {
    fn from(value: PeraAddress) -> Self {
        AccountAddress::new(value.0)
    }
}

impl From<ObjectID> for PeraAddress {
    fn from(value: ObjectID) -> Self {
        PeraAddress(value.into_bytes())
    }
}

impl From<PeraAddress> for ObjectID {
    fn from(value: PeraAddress) -> Self {
        ObjectID::new(value.0)
    }
}

impl From<NativePeraAddress> for PeraAddress {
    fn from(value: NativePeraAddress) -> Self {
        PeraAddress(value.to_inner())
    }
}

impl From<PeraAddress> for NativePeraAddress {
    fn from(value: PeraAddress) -> Self {
        AccountAddress::from(value).into()
    }
}

impl FromStr for PeraAddress {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<Self, FromStrError> {
        let Some(s) = s.strip_prefix("0x") else {
            return Err(FromStrError::NoPrefix);
        };

        if s.is_empty() || s.len() > PERA_ADDRESS_LENGTH * 2 {
            return Err(FromStrError::WrongLength(s.len()));
        }

        let mut arr = [0u8; PERA_ADDRESS_LENGTH];
        hex::decode_to_slice(
            // Left pad with `0`-s up to PERA_ADDRESS_LENGTH * 2 characters long.
            format!("{:0>width$}", s, width = PERA_ADDRESS_LENGTH * 2),
            &mut arr[..],
        )
        .map_err(|e| match e {
            hex::FromHexError::InvalidHexCharacter { c, index } => {
                FromStrError::BadHex(c, index + 2)
            }
            hex::FromHexError::OddLength => unreachable!("SAFETY: Prevented by padding"),
            hex::FromHexError::InvalidStringLength => {
                unreachable!("SAFETY: Prevented by bounds check")
            }
        })?;

        Ok(PeraAddress(arr))
    }
}

impl std::fmt::Display for PeraAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("0x{}", hex::encode(self.0)))
    }
}

/// Parse a `PeraAddress` from its stored representation.  Failure is an internal error: the
/// database should never contain a malformed address (containing the wrong number of bytes).
pub(crate) fn addr(bytes: impl AsRef<[u8]>) -> Result<PeraAddress, Error> {
    PeraAddress::from_bytes(bytes.as_ref()).map_err(|e| {
        let bytes = bytes.as_ref().to_vec();
        Error::Internal(format!("Error deserializing address: {bytes:?}: {e}"))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_graphql::Value;

    const STR_ADDRESS: &str = "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    const ARR_ADDRESS: [u8; PERA_ADDRESS_LENGTH] = [
        1, 35, 69, 103, 137, 171, 205, 239, 1, 35, 69, 103, 137, 171, 205, 239, 1, 35, 69, 103,
        137, 171, 205, 239, 1, 35, 69, 103, 137, 171, 205, 239,
    ];
    const PERA_ADDRESS: PeraAddress = PeraAddress(ARR_ADDRESS);

    #[test]
    fn test_parse_valid_peraaddress() {
        let parsed = PeraAddress::from_str(STR_ADDRESS).unwrap();
        assert_eq!(parsed.0, ARR_ADDRESS);
    }

    #[test]
    fn test_to_value() {
        let value = ScalarType::to_value(&PERA_ADDRESS);
        assert_eq!(value, Value::String(STR_ADDRESS.to_string()));
    }

    #[test]
    fn test_from_array() {
        let addr = PeraAddress::from_array(ARR_ADDRESS);
        assert_eq!(addr, PERA_ADDRESS);
    }

    #[test]
    fn test_as_slice() {
        assert_eq!(PERA_ADDRESS.as_slice(), &ARR_ADDRESS);
    }

    #[test]
    fn test_round_trip() {
        let value = ScalarType::to_value(&PERA_ADDRESS);
        let parsed_back = ScalarType::parse(value).unwrap();
        assert_eq!(PERA_ADDRESS, parsed_back);
    }

    #[test]
    fn test_parse_no_prefix() {
        let err = PeraAddress::from_str(&STR_ADDRESS[2..]).unwrap_err();
        assert_eq!(FromStrError::NoPrefix, err);
    }

    #[test]
    fn test_parse_invalid_prefix() {
        let input = "1x".to_string() + &STR_ADDRESS[2..];
        let err = PeraAddress::from_str(&input).unwrap_err();
        assert_eq!(FromStrError::NoPrefix, err)
    }

    #[test]
    fn test_parse_invalid_length() {
        let input = STR_ADDRESS.to_string() + "0123";
        let err = PeraAddress::from_str(&input).unwrap_err();
        assert_eq!(FromStrError::WrongLength(68), err)
    }

    #[test]
    fn test_parse_invalid_characters() {
        let input = "0xg".to_string() + &STR_ADDRESS[3..];
        let err = PeraAddress::from_str(&input).unwrap_err();
        assert_eq!(FromStrError::BadHex('g', 2), err);
    }

    #[test]
    fn test_unicode_gibberish() {
        let parsed = PeraAddress::from_str("aAௗ0㌀0");
        assert!(parsed.is_err());
    }

    #[test]
    fn bad_scalar_type() {
        let input = Value::Number(0x42.into());
        let parsed = <PeraAddress as ScalarType>::parse(input);
        assert!(parsed.is_err());
    }
}
