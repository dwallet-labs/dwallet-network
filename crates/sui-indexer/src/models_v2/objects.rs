// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use serde::de::DeserializeOwned;
use std::collections::HashMap;
use sui_json_rpc::coin_api::parse_to_struct_tag;

use diesel::prelude::*;
use move_bytecode_utils::module_cache::GetModule;
use sui_json_rpc_types::{Balance, Coin as SuiCoin};
use sui_types::base_types::{ObjectID, ObjectRef, SequenceNumber};
use sui_types::digests::ObjectDigest;
use sui_types::dynamic_field::{DynamicFieldInfo, DynamicFieldName, DynamicFieldType, Field};
use sui_types::object::Object;
use sui_types::object::ObjectRead;

use crate::errors::IndexerError;
use crate::schema_v2::{objects, objects_history};
use crate::types_v2::{IndexedDeletedObject, IndexedObject, ObjectStatus};

#[derive(Queryable)]
pub struct DynamicFieldColumn {
    pub object_id: Vec<u8>,
    pub object_version: i64,
    pub object_digest: Vec<u8>,
    pub df_kind: Option<i16>,
    pub df_name: Option<Vec<u8>>,
    pub df_object_type: Option<String>,
    pub df_object_id: Option<Vec<u8>>,
}

#[derive(Queryable)]
pub struct ObjectRefColumn {
    pub object_id: Vec<u8>,
    pub object_version: i64,
    pub object_digest: Vec<u8>,
}

// NOTE: please add updating statement like below in pg_indexer_store_v2.rs,
// if new columns are added here:
// objects::epoch.eq(excluded(objects::epoch))
#[derive(Queryable, Insertable, Debug, Identifiable, Clone, QueryableByName)]
#[diesel(table_name = objects, primary_key(object_id))]
pub struct StoredObject {
    pub object_id: Vec<u8>,
    pub object_version: i64,
    pub object_digest: Vec<u8>,
    pub checkpoint_sequence_number: i64,
    pub owner_type: i16,
    pub owner_id: Option<Vec<u8>>,
    /// The type of this object. This will be None if the object is a Package
    pub object_type: Option<String>,
    pub serialized_object: Vec<u8>,
    pub coin_type: Option<String>,
    // TODO deal with overflow
    pub coin_balance: Option<i64>,
    pub df_kind: Option<i16>,
    pub df_name: Option<Vec<u8>>,
    pub df_object_type: Option<String>,
    pub df_object_id: Option<Vec<u8>>,
}

#[derive(Queryable, Insertable, Debug, Identifiable, Clone, QueryableByName)]
#[diesel(table_name = objects_history, primary_key(object_id, object_version, checkpoint_sequence_number))]
pub struct StoredHistoryObject {
    pub object_id: Vec<u8>,
    pub object_version: i64,
    pub object_status: i16,
    pub object_digest: Option<Vec<u8>>,
    pub checkpoint_sequence_number: i64,
    pub owner_type: Option<i16>,
    pub owner_id: Option<Vec<u8>>,
    pub object_type: Option<String>,
    pub serialized_object: Option<Vec<u8>>,
    pub coin_type: Option<String>,
    pub coin_balance: Option<i64>,
    pub df_kind: Option<i16>,
    pub df_name: Option<Vec<u8>>,
    pub df_object_type: Option<String>,
    pub df_object_id: Option<Vec<u8>>,
}

impl From<StoredObject> for StoredHistoryObject {
    fn from(o: StoredObject) -> Self {
        Self {
            object_id: o.object_id,
            object_version: o.object_version,
            object_status: ObjectStatus::Active as i16,
            object_digest: Some(o.object_digest),
            checkpoint_sequence_number: o.checkpoint_sequence_number,
            owner_type: Some(o.owner_type),
            owner_id: o.owner_id,
            object_type: o.object_type,
            serialized_object: Some(o.serialized_object),
            coin_type: o.coin_type,
            coin_balance: o.coin_balance,
            df_kind: o.df_kind,
            df_name: o.df_name,
            df_object_type: o.df_object_type,
            df_object_id: o.df_object_id,
        }
    }
}

#[derive(Queryable, Insertable, Debug, Identifiable, Clone, QueryableByName)]
#[diesel(table_name = objects, primary_key(object_id))]
pub struct StoredDeletedObject {
    pub object_id: Vec<u8>,
    pub object_version: i64,
    pub checkpoint_sequence_number: i64,
}

impl From<IndexedDeletedObject> for StoredDeletedObject {
    fn from(o: IndexedDeletedObject) -> Self {
        Self {
            object_id: o.object_id.to_vec(),
            object_version: o.object_version as i64,
            checkpoint_sequence_number: o.checkpoint_sequence_number as i64,
        }
    }
}

#[derive(Queryable, Insertable, Debug, Identifiable, Clone, QueryableByName)]
#[diesel(table_name = objects_history, primary_key(object_id, object_version, checkpoint_sequence_number))]
pub struct StoredDeletedHistoryObject {
    pub object_id: Vec<u8>,
    pub object_version: i64,
    pub object_status: i16,
    pub checkpoint_sequence_number: i64,
}

impl From<StoredDeletedObject> for StoredDeletedHistoryObject {
    fn from(o: StoredDeletedObject) -> Self {
        Self {
            object_id: o.object_id,
            object_version: o.object_version,
            object_status: ObjectStatus::WrappedOrDeleted as i16,
            checkpoint_sequence_number: o.checkpoint_sequence_number,
        }
    }
}

impl From<IndexedObject> for StoredObject {
    fn from(o: IndexedObject) -> Self {
        Self {
            object_id: o.object_id.to_vec(),
            object_version: o.object_version as i64,
            object_digest: o.object_digest.into_inner().to_vec(),
            checkpoint_sequence_number: o.checkpoint_sequence_number as i64,
            owner_type: o.owner_type as i16,
            owner_id: o.owner_id.map(|id| id.to_vec()),
            object_type: o
                .object
                .type_()
                .map(|t| t.to_canonical_string(/* with_prefix */ true)),
            serialized_object: bcs::to_bytes(&o.object).unwrap(),
            coin_type: o.coin_type,
            coin_balance: o.coin_balance.map(|b| b as i64),
            df_kind: o.df_info.as_ref().map(|k| match k.type_ {
                DynamicFieldType::DynamicField => 0,
                DynamicFieldType::DynamicObject => 1,
            }),
            df_name: o.df_info.as_ref().map(|n| bcs::to_bytes(&n.name).unwrap()),
            df_object_type: o.df_info.as_ref().map(|v| v.object_type.clone()),
            df_object_id: o.df_info.as_ref().map(|v| v.object_id.to_vec()),
        }
    }
}

impl TryFrom<StoredObject> for Object {
    type Error = IndexerError;

    fn try_from(o: StoredObject) -> Result<Self, Self::Error> {
        bcs::from_bytes(&o.serialized_object).map_err(|e| {
            IndexerError::SerdeError(format!(
                "Failed to deserialize object: {:?}, error: {}",
                o.object_id, e
            ))
        })
    }
}

impl StoredObject {
    pub fn try_into_object_read(
        self,
        module_cache: &impl GetModule,
    ) -> Result<ObjectRead, IndexerError> {
        let oref = self.get_object_ref()?;
        let object: sui_types::object::Object = self.try_into()?;
        let layout = object.get_layout(module_cache)?;
        Ok(ObjectRead::Exists(oref, object, layout))
    }

    pub fn try_into_expectant_dynamic_field_info(
        self,
        module_cache: &impl GetModule,
    ) -> Result<DynamicFieldInfo, IndexerError> {
        match self.try_into_dynamic_field_info(module_cache).transpose() {
            Some(Ok(info)) => Ok(info),
            Some(Err(e)) => Err(e),
            None => Err(IndexerError::PersistentStorageDataCorruptionError(
                "Dynamic field object has incompatible dynamic field type: empty df_kind".into(),
            )),
        }
    }

    pub fn try_into_dynamic_field_info(
        self,
        module_cache: &impl GetModule,
    ) -> Result<Option<DynamicFieldInfo>, IndexerError> {
        if self.df_kind.is_none() {
            return Ok(None);
        }

        // Past this point, if there is any unexpected field, it's a data corruption error
        let object_id = ObjectID::from_bytes(&self.object_id).map_err(|_| {
            IndexerError::PersistentStorageDataCorruptionError(format!(
                "Can't convert {:?} to object_id",
                self.object_id
            ))
        })?;
        let object_digest = ObjectDigest::try_from(self.object_digest.as_slice()).map_err(|e| {
            IndexerError::PersistentStorageDataCorruptionError(format!(
                "object {} has incompatible object digest. Error: {e}",
                object_id
            ))
        })?;
        let df_object_id = if let Some(df_object_id) = self.df_object_id {
            ObjectID::from_bytes(df_object_id).map_err(|e| {
                IndexerError::PersistentStorageDataCorruptionError(format!(
                    "object {} has incompatible dynamic field type: df_object_id. Error: {e}",
                    object_id
                ))
            })
        } else {
            return Err(IndexerError::PersistentStorageDataCorruptionError(format!(
                "object {} has incompatible dynamic field type: empty df_object_id",
                object_id
            )));
        }?;
        let type_ = match self.df_kind {
            Some(0) => DynamicFieldType::DynamicField,
            Some(1) => DynamicFieldType::DynamicObject,
            _ => {
                return Err(IndexerError::PersistentStorageDataCorruptionError(format!(
                    "object {} has incompatible dynamic field type: empty df_kind",
                    object_id
                )))
            }
        };
        let name = if let Some(field_name) = self.df_name {
            let name: DynamicFieldName = bcs::from_bytes(&field_name).map_err(|e| {
                IndexerError::PersistentStorageDataCorruptionError(format!(
                    "object {} has incompatible dynamic field type: df_name. Error: {e}",
                    object_id
                ))
            })?;
            name
        } else {
            return Err(IndexerError::PersistentStorageDataCorruptionError(format!(
                "object {} has incompatible dynamic field type: empty df_name",
                object_id
            )));
        };
        let layout = move_bytecode_utils::layout::TypeLayoutBuilder::build_with_types(
            &name.type_,
            module_cache,
        )?;
        let sui_json_value = sui_json::SuiJsonValue::new(name.value.clone())?;
        let bcs_name = sui_json_value.to_bcs_bytes(&layout)?;
        let object_type =
            self.df_object_type
                .ok_or(IndexerError::PersistentStorageDataCorruptionError(format!(
                    "object {} has incompatible dynamic field type: empty df_object_type",
                    object_id
                )))?;
        Ok(Some(DynamicFieldInfo {
            version: SequenceNumber::from_u64(self.object_version as u64),
            digest: object_digest,
            type_,
            name,
            bcs_name,
            object_type,
            object_id: df_object_id,
        }))
    }

    pub fn get_object_ref(&self) -> Result<ObjectRef, IndexerError> {
        let object_id = ObjectID::from_bytes(self.object_id.clone()).map_err(|_| {
            IndexerError::SerdeError(format!("Can't convert {:?} to object_id", self.object_id))
        })?;
        let object_digest =
            ObjectDigest::try_from(self.object_digest.as_slice()).map_err(|_| {
                IndexerError::SerdeError(format!(
                    "Can't convert {:?} to object_digest",
                    self.object_digest
                ))
            })?;
        Ok((
            object_id,
            (self.object_version as u64).into(),
            object_digest,
        ))
    }

    pub fn to_dynamic_field<K, V>(&self) -> Option<Field<K, V>>
    where
        K: DeserializeOwned,
        V: DeserializeOwned,
    {
        let object: Object = bcs::from_bytes(&self.serialized_object).ok()?;

        let object = object.data.try_as_move()?;
        let ty = object.type_();

        if !ty.is_dynamic_field() {
            return None;
        }

        bcs::from_bytes(object.contents()).ok()
    }
}

impl TryFrom<StoredObject> for SuiCoin {
    type Error = IndexerError;

    fn try_from(o: StoredObject) -> Result<Self, Self::Error> {
        let object: Object = o.clone().try_into()?;
        let (coin_object_id, version, digest) = o.get_object_ref()?;
        let coin_type_canonical =
            o.coin_type
                .ok_or(IndexerError::PersistentStorageDataCorruptionError(format!(
                    "Object {} is supposed to be a coin but has an empty coin_type column",
                    coin_object_id,
                )))?;
        let coin_type = parse_to_struct_tag(coin_type_canonical.as_str())
            .map_err(|_| {
                IndexerError::PersistentStorageDataCorruptionError(format!(
                    "The type of object {} cannot be parsed as a struct tag",
                    coin_object_id,
                ))
            })?
            .to_string();
        let balance = o
            .coin_balance
            .ok_or(IndexerError::PersistentStorageDataCorruptionError(format!(
                "Object {} is supposed to be a coin but has an empy coin_balance column",
                coin_object_id,
            )))?;
        Ok(SuiCoin {
            coin_type,
            coin_object_id,
            version,
            digest,
            balance: balance as u64,
            previous_transaction: object.previous_transaction,
        })
    }
}

#[derive(QueryableByName)]
pub struct CoinBalance {
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub coin_type: String,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub coin_num: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub coin_balance: i64,
}

impl TryFrom<CoinBalance> for Balance {
    type Error = IndexerError;

    fn try_from(c: CoinBalance) -> Result<Self, Self::Error> {
        let coin_type = parse_to_struct_tag(c.coin_type.as_str())
            .map_err(|_| {
                IndexerError::PersistentStorageDataCorruptionError(
                    "The type of coin balance cannot be parsed as a struct tag".to_string(),
                )
            })?
            .to_string();
        Ok(Self {
            coin_type,
            coin_object_count: c.coin_num as usize,
            // TODO: deal with overflow
            total_balance: c.coin_balance as u128,
            locked_balance: HashMap::default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use move_core_types::{account_address::AccountAddress, language_storage::StructTag};
    use sui_types::{
        coin::Coin,
        digests::TransactionDigest,
        gas_coin::{GasCoin, GAS},
        object::{Data, MoveObject, ObjectInner, Owner},
        Identifier, TypeTag,
    };

    use super::*;

    #[test]
    fn test_canonical_string_of_object_type_for_coin() {
        let test_obj = Object::new_gas_for_testing();
        let indexed_obj = IndexedObject::from_object(1, test_obj, None);

        let stored_obj = StoredObject::from(indexed_obj);

        match stored_obj.object_type {
            Some(t) => {
                assert_eq!(t, "0x0000000000000000000000000000000000000000000000000000000000000002::coin::Coin<0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI>");
            }
            None => {
                panic!("object_type should not be none");
            }
        }
    }

    #[test]
    fn test_convert_stored_obj_to_sui_coin() {
        let test_obj = Object::new_gas_for_testing();
        let indexed_obj = IndexedObject::from_object(1, test_obj, None);

        let stored_obj = StoredObject::from(indexed_obj);

        let sui_coin = SuiCoin::try_from(stored_obj).unwrap();
        assert_eq!(sui_coin.coin_type, "0x2::dwlt::DWLT");
    }

    #[test]
    fn test_output_format_coin_balance() {
        let test_obj = Object::new_gas_for_testing();
        let indexed_obj = IndexedObject::from_object(1, test_obj, None);

        let stored_obj = StoredObject::from(indexed_obj);
        let test_balance = CoinBalance {
            coin_type: stored_obj.coin_type.unwrap(),
            coin_num: 1,
            coin_balance: 100,
        };
        let balance = Balance::try_from(test_balance).unwrap();
        assert_eq!(balance.coin_type, "0x2::dwlt::DWLT");
    }

    #[test]
    fn test_vec_of_coin_sui_conversion() {
        // 0xe7::vec_coin::VecCoin<vector<0x2::coin::Coin<0x2::dwlt::DWLT>>>
        let vec_coins_type = TypeTag::Vector(Box::new(
            Coin::type_(TypeTag::Struct(Box::new(GAS::type_()))).into(),
        ));
        let object_type = StructTag {
            address: AccountAddress::from_hex_literal("0xe7").unwrap(),
            module: Identifier::new("vec_coin").unwrap(),
            name: Identifier::new("VecCoin").unwrap(),
            type_params: vec![vec_coins_type],
        };

        let id = ObjectID::ZERO;
        let gas = 10;

        let contents = bcs::to_bytes(&vec![GasCoin::new(id, gas)]).unwrap();
        let data = Data::Move(
            unsafe {
                MoveObject::new_from_execution_with_limit(
                    object_type.into(),
                    true,
                    1.into(),
                    contents,
                    256,
                )
            }
            .unwrap(),
        );

        let owner = AccountAddress::from_hex_literal("0x1").unwrap();

        let object = ObjectInner {
            owner: Owner::AddressOwner(owner.into()),
            data,
            previous_transaction: TransactionDigest::genesis_marker(),
            storage_rebate: 0,
        }
        .into();

        let indexed_obj = IndexedObject::from_object(1, object, None);

        let stored_obj = StoredObject::from(indexed_obj);

        match stored_obj.object_type {
            Some(t) => {
                assert_eq!(t, "0x00000000000000000000000000000000000000000000000000000000000000e7::vec_coin::VecCoin<vector<0x0000000000000000000000000000000000000000000000000000000000000002::coin::Coin<0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI>>>");
            }
            None => {
                panic!("object_type should not be none");
            }
        }
    }
}
