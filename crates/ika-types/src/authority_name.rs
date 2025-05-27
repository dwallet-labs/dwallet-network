// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use hex::FromHex;
use rand::{rngs::OsRng, Rng};
use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
use std::{convert::TryFrom, fmt, str::FromStr};

/// A struct that represents an account address.
#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy)]
#[cfg_attr(any(test, feature = "fuzzing"), derive(proptest_derive::Arbitrary))]
#[cfg_attr(any(test, feature = "fuzzing"), derive(arbitrary::Arbitrary))]
pub struct AuthorityName([u8; AuthorityName::LENGTH]);

impl AuthorityName {
    pub const fn new(address: [u8; Self::LENGTH]) -> Self {
        Self(address)
    }

    /// The number of bytes in an address.
    pub const LENGTH: usize = 32;

    /// Hex address: 0x0
    pub const ZERO: Self = Self([0u8; Self::LENGTH]);

    /// Hex address: 0x1
    pub const ONE: Self = Self::get_hex_address_one();

    /// Hex address: 0x2
    pub const TWO: Self = Self::get_hex_address_two();

    pub const fn from_suffix(suffix: u16) -> AuthorityName {
        let mut addr = [0u8; AuthorityName::LENGTH];
        let [hi, lo] = suffix.to_be_bytes();
        addr[AuthorityName::LENGTH - 2] = hi;
        addr[AuthorityName::LENGTH - 1] = lo;
        AuthorityName::new(addr)
    }

    const fn get_hex_address_one() -> Self {
        let mut addr = [0u8; AuthorityName::LENGTH];
        addr[AuthorityName::LENGTH - 1] = 1u8;
        Self(addr)
    }

    const fn get_hex_address_two() -> Self {
        let mut addr = [0u8; AuthorityName::LENGTH];
        addr[AuthorityName::LENGTH - 1] = 2u8;
        Self(addr)
    }

    pub fn random() -> Self {
        let mut rng = OsRng;
        let buf: [u8; Self::LENGTH] = rng.gen();
        Self(buf)
    }

    /// Return a canonical string representation of the address
    /// Addresses are hex-encoded lowercase values of length ADDRESS_LENGTH (16, 20, or 32 depending on the Move platform)
    /// e.g., 0000000000000000000000000000000a, *not* 0x0000000000000000000000000000000a, 0xa, or 0xA
    /// Note: this function is guaranteed to be stable, and this is suitable for use inside
    /// Move native functions or the VM.
    /// However, one can pass with_prefix=true to get its representation with the 0x prefix.
    pub fn to_canonical_string(&self, with_prefix: bool) -> String {
        self.to_canonical_display(with_prefix).to_string()
    }

    /// Implements Display for the address, with the prefix 0x if with_prefix is true.
    pub fn to_canonical_display(&self, with_prefix: bool) -> impl fmt::Display + '_ {
        struct HexDisplay<'a> {
            data: &'a [u8],
            with_prefix: bool,
        }

        impl<'a> fmt::Display for HexDisplay<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                if self.with_prefix {
                    write!(f, "0x{}", hex::encode(self.data))
                } else {
                    write!(f, "{}", hex::encode(self.data))
                }
            }
        }
        HexDisplay {
            data: &self.0,
            with_prefix,
        }
    }

    pub fn short_str_lossless(&self) -> String {
        let hex_str = hex::encode(self.0).trim_start_matches('0').to_string();
        if hex_str.is_empty() {
            "0".to_string()
        } else {
            hex_str
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn into_bytes(self) -> [u8; Self::LENGTH] {
        self.0
    }

    pub fn from_hex_literal(literal: &str) -> Result<Self, AccountAddressParseError> {
        if !literal.starts_with("0x") {
            return Err(AccountAddressParseError);
        }

        let hex_len = literal.len() - 2;

        // If the string is too short, pad it
        if hex_len < Self::LENGTH * 2 {
            let mut hex_str = String::with_capacity(Self::LENGTH * 2);
            for _ in 0..Self::LENGTH * 2 - hex_len {
                hex_str.push('0');
            }
            hex_str.push_str(&literal[2..]);
            AuthorityName::from_hex(hex_str)
        } else {
            AuthorityName::from_hex(&literal[2..])
        }
    }

    pub fn to_hex_literal(&self) -> String {
        format!("0x{}", self.short_str_lossless())
    }

    pub fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, AccountAddressParseError> {
        <[u8; Self::LENGTH]>::from_hex(hex)
            .map_err(|_| AccountAddressParseError)
            .map(Self)
    }

    pub fn to_hex(&self) -> String {
        format!("{:x}", self)
    }

    pub fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self, AccountAddressParseError> {
        <[u8; Self::LENGTH]>::try_from(bytes.as_ref())
            .map_err(|_| AccountAddressParseError)
            .map(Self)
    }

    /// TODO (ade): use macro to enfornce determinism
    pub fn abstract_size_for_gas_metering(&self) -> AbstractMemorySize {
        AbstractMemorySize::new(Self::LENGTH as u64)
    }
}

impl AsRef<[u8]> for AuthorityName {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl std::ops::Deref for AuthorityName {
    type Target = [u8; Self::LENGTH];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for AuthorityName {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:x}", self)
    }
}

impl fmt::Debug for AuthorityName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self)
    }
}

impl fmt::LowerHex for AuthorityName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }

        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }

        Ok(())
    }
}

impl fmt::UpperHex for AuthorityName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }

        for byte in &self.0 {
            write!(f, "{:02X}", byte)?;
        }

        Ok(())
    }
}

impl From<[u8; AuthorityName::LENGTH]> for AuthorityName {
    fn from(bytes: [u8; AuthorityName::LENGTH]) -> Self {
        Self::new(bytes)
    }
}

impl TryFrom<&[u8]> for AuthorityName {
    type Error = AccountAddressParseError;

    /// Tries to convert the provided byte array into Address.
    fn try_from(bytes: &[u8]) -> Result<AuthorityName, AccountAddressParseError> {
        Self::from_bytes(bytes)
    }
}

impl TryFrom<Vec<u8>> for AuthorityName {
    type Error = AccountAddressParseError;

    /// Tries to convert the provided byte buffer into Address.
    fn try_from(bytes: Vec<u8>) -> Result<AuthorityName, AccountAddressParseError> {
        Self::from_bytes(bytes)
    }
}

impl From<AuthorityName> for Vec<u8> {
    fn from(addr: AuthorityName) -> Vec<u8> {
        addr.0.to_vec()
    }
}

impl From<&AuthorityName> for Vec<u8> {
    fn from(addr: &AuthorityName) -> Vec<u8> {
        addr.0.to_vec()
    }
}

impl From<AuthorityName> for [u8; AuthorityName::LENGTH] {
    fn from(addr: AuthorityName) -> Self {
        addr.0
    }
}

impl From<&AuthorityName> for [u8; AuthorityName::LENGTH] {
    fn from(addr: &AuthorityName) -> Self {
        addr.0
    }
}

impl From<&AuthorityName> for String {
    fn from(addr: &AuthorityName) -> String {
        ::hex::encode(addr.as_ref())
    }
}

impl TryFrom<String> for AuthorityName {
    type Error = AccountAddressParseError;

    fn try_from(s: String) -> Result<AuthorityName, AccountAddressParseError> {
        Self::from_hex(s)
    }
}

impl FromStr for AuthorityName {
    type Err = AccountAddressParseError;

    fn from_str(s: &str) -> Result<Self, AccountAddressParseError> {
        // Accept 0xADDRESS or ADDRESS
        if let Ok(address) = AuthorityName::from_hex_literal(s) {
            Ok(address)
        } else {
            Self::from_hex(s)
        }
    }
}

impl<'de> Deserialize<'de> for AuthorityName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let s = <String>::deserialize(deserializer)?;
            AuthorityName::from_str(&s).map_err(D::Error::custom)
        } else {
            // In order to preserve the Serde data model and help analysis tools,
            // make sure to wrap our value in a container with the same name
            // as the original type.
            #[derive(::serde::Deserialize)]
            #[serde(rename = "AccountAddress")]
            struct Value([u8; AuthorityName::LENGTH]);

            let value = Value::deserialize(deserializer)?;
            Ok(AuthorityName::new(value.0))
        }
    }
}

impl Serialize for AuthorityName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            self.to_hex().serialize(serializer)
        } else {
            // See comment in deserialize.
            serializer.serialize_newtype_struct("AccountAddress", &self.0)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AccountAddressParseError;

impl fmt::Display for AccountAddressParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Unable to parse AccountAddress (must be hex string of length {})",
            AuthorityName::LENGTH
        )
    }
}

impl std::error::Error for AccountAddressParseError {}