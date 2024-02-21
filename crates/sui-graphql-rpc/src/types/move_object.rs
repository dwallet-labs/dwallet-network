// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use super::coin::CoinDowncastError;
use super::coin_metadata::{CoinMetadata, CoinMetadataDowncastError};
use super::move_type::MoveType;
use super::move_value::MoveValue;
use super::stake::StakedSuiDowncastError;
use super::{coin::Coin, object::Object};
use crate::context_data::package_cache::PackageCache;
use crate::error::Error;
use crate::types::stake::StakedSui;
use async_graphql::*;
use sui_package_resolver::Resolver;
use sui_types::object::{Data, MoveObject as NativeMoveObject};
use sui_types::TypeTag;

#[derive(Clone)]
pub(crate) struct MoveObject {
    /// Representation of this Move Object as a generic Object.
    pub super_: Object,

    /// Move-object-specific data, extracted from the native representation at
    /// `graphql_object.native_object.data`.
    pub native: NativeMoveObject,
}

pub(crate) struct MoveObjectDowncastError;

#[Object]
impl MoveObject {
    /// Displays the contents of the MoveObject in a JSON string and through graphql types.  Also
    /// provides the flat representation of the type signature, and the bcs of the corresponding
    /// data
    async fn contents(&self) -> Option<MoveValue> {
        let type_ = TypeTag::from(self.native.type_().clone());
        Some(MoveValue::new(type_, self.native.contents().into()))
    }

    /// Determines whether a transaction can transfer this object, using the TransferObjects
    /// transaction command or `sui::transfer::public_transfer`, both of which require the object to
    /// have the `key` and `store` abilities.
    async fn has_public_transfer(&self, ctx: &Context<'_>) -> Result<bool> {
        let resolver: &Resolver<PackageCache> = ctx
            .data()
            .map_err(|_| Error::Internal("Unable to fetch Package Cache.".to_string()))
            .extend()?;

        let type_ = MoveType::new(TypeTag::from(self.native.type_().clone()));
        let set = type_.abilities_impl(resolver).await.extend()?;
        Ok(set.has_key() && set.has_store())
    }

    /// Attempts to convert the Move object into an Object
    /// This provides additional information such as version and digest on the top-level
    async fn as_object(&self) -> &Object {
        &self.super_
    }

    /// Attempts to convert the Move object into a `0x2::coin::Coin`.
    async fn as_coin(&self) -> Result<Option<Coin>, Error> {
        match Coin::try_from(self) {
            Ok(coin) => Ok(Some(coin)),
            Err(CoinDowncastError::NotACoin) => Ok(None),
            Err(CoinDowncastError::Bcs(e)) => {
                Err(Error::Internal(format!("Failed to deserialize coin: {e}")))
            }
        }
    }

    /// Attempts to convert the Move object into a `0x3::staking_pool::StakedSui`.
    async fn as_staked_sui(&self) -> Result<Option<StakedSui>, Error> {
        match StakedSui::try_from(self) {
            Ok(coin) => Ok(Some(coin)),
            Err(StakedSuiDowncastError::NotAStakedSui) => Ok(None),
            Err(StakedSuiDowncastError::Bcs(e)) => Err(Error::Internal(format!(
                "Failed to deserialize staked sui: {e}"
            ))),
        }
    }

    /// Attempts to convert the Move object into a `0x2::coin::CoinMetadata`.
    async fn as_coin_metadata(&self) -> Result<Option<CoinMetadata>, Error> {
        match CoinMetadata::try_from(self) {
            Ok(metadata) => Ok(Some(metadata)),
            Err(CoinMetadataDowncastError::NotCoinMetadata) => Ok(None),
            Err(CoinMetadataDowncastError::Bcs(e)) => Err(Error::Internal(format!(
                "Failed to deserialize coin metadata: {e}"
            ))),
        }
    }
}

impl TryFrom<&Object> for MoveObject {
    type Error = MoveObjectDowncastError;

    fn try_from(object: &Object) -> Result<Self, Self::Error> {
        if let Data::Move(move_object) = &object.native.data {
            Ok(Self {
                super_: object.clone(),
                native: move_object.clone(),
            })
        } else {
            Err(MoveObjectDowncastError)
        }
    }
}
