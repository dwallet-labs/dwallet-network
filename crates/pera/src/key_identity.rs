// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::{fmt::Display, str::FromStr};

use anyhow::Error;
use pera_keys::keystore::{AccountKeystore, Keystore};
use pera_sdk::wallet_context::WalletContext;
use pera_types::base_types::PeraAddress;
use serde::Serialize;

/// An address or an alias associated with a key in the wallet
/// This is used to distinguish between an address or an alias,
/// enabling a user to use an alias for any command that requires an address.
#[derive(Serialize, Clone)]
pub enum KeyIdentity {
    Address(PeraAddress),
    Alias(String),
}

impl FromStr for KeyIdentity {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("0x") {
            Ok(KeyIdentity::Address(PeraAddress::from_str(s)?))
        } else {
            Ok(KeyIdentity::Alias(s.to_string()))
        }
    }
}

impl Display for KeyIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            KeyIdentity::Address(x) => x.to_string(),
            KeyIdentity::Alias(x) => x.to_string(),
        };
        write!(f, "{}", v)
    }
}

/// Get the PeraAddress corresponding to this key identity.
/// If no string is provided, then the current active address is returned.
pub fn get_identity_address(
    input: Option<KeyIdentity>,
    ctx: &mut WalletContext,
) -> Result<PeraAddress, Error> {
    if let Some(addr) = input {
        get_identity_address_from_keystore(addr, &ctx.config.keystore)
    } else {
        Ok(ctx.active_address()?)
    }
}

pub fn get_identity_address_from_keystore(
    input: KeyIdentity,
    keystore: &Keystore,
) -> Result<PeraAddress, Error> {
    match input {
        KeyIdentity::Address(x) => Ok(x),
        KeyIdentity::Alias(x) => Ok(*keystore.get_address_by_alias(x)?),
    }
}
