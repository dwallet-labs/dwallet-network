// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use anyhow::anyhow;
use move_core_types::annotated_value::{MoveStruct, MoveValue};
use move_core_types::ident_str;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{StructTag, TypeTag};
use serde_json::json;

use ika_types::base_types::{ObjectDigest, SequenceNumber};
use ika_types::base_types::{ObjectID, IkaAddress};
use ika_types::gas_coin::GasCoin;
use ika_types::object::{MoveObject, Owner};
use ika_types::{parse_ika_struct_tag, MOVE_STDLIB_ADDRESS, IKA_FRAMEWORK_ADDRESS};

use crate::{ObjectChange, IkaMoveStruct, IkaMoveValue};

#[test]
fn test_move_value_to_ika_coin() {
    let id = ObjectID::random();
    let value = 10000;
    let coin = GasCoin::new(id, value);

    let move_object = MoveObject::new_gas_coin(SequenceNumber::new(), id, value);
    let layout = GasCoin::layout();

    let move_struct = move_object.to_move_struct(&layout).unwrap();
    let ika_struct = IkaMoveStruct::from(move_struct);
    let gas_coin = GasCoin::try_from(&ika_struct).unwrap();
    assert_eq!(coin.value(), gas_coin.value());
    assert_eq!(coin.id(), gas_coin.id());
}

#[test]
fn test_move_value_to_string() {
    let test_string = "Some test string";
    let bytes = test_string.as_bytes();
    let values = bytes
        .iter()
        .map(|u8| MoveValue::U8(*u8))
        .collect::<Vec<_>>();

    let move_value = MoveValue::Struct(MoveStruct {
        type_: StructTag {
            address: MOVE_STDLIB_ADDRESS,
            module: ident_str!("string").to_owned(),
            name: ident_str!("String").to_owned(),
            type_params: vec![],
        },
        fields: vec![(ident_str!("bytes").to_owned(), MoveValue::Vector(values))],
    });

    let ika_value = IkaMoveValue::from(move_value);

    assert!(matches!(ika_value, IkaMoveValue::String(s) if s == test_string));
}

#[test]
fn test_option() {
    // bugfix for https://github.com/MystenLabs/sui/issues/4995
    let option = MoveValue::Struct(MoveStruct {
        type_: StructTag {
            address: MOVE_STDLIB_ADDRESS,
            module: Identifier::from_str("option").unwrap(),
            name: Identifier::from_str("Option").unwrap(),
            type_params: vec![TypeTag::U8],
        },
        fields: vec![(
            Identifier::from_str("vec").unwrap(),
            MoveValue::Vector(vec![MoveValue::U8(5)]),
        )],
    });
    let ika_value = IkaMoveValue::from(option);
    assert!(matches!(
        ika_value,
        IkaMoveValue::Option(value) if *value == Some(IkaMoveValue::Number(5))
    ));
}

#[test]
fn test_move_value_to_url() {
    let test_url = "http://testing.com";
    let bytes = test_url.as_bytes();
    let values = bytes
        .iter()
        .map(|u8| MoveValue::U8(*u8))
        .collect::<Vec<_>>();

    let string_move_value = MoveValue::Struct(MoveStruct {
        type_: StructTag {
            address: MOVE_STDLIB_ADDRESS,
            module: ident_str!("string").to_owned(),
            name: ident_str!("String").to_owned(),
            type_params: vec![],
        },
        fields: vec![(ident_str!("bytes").to_owned(), MoveValue::Vector(values))],
    });

    let url_move_value = MoveValue::Struct(MoveStruct {
        type_: StructTag {
            address: IKA_FRAMEWORK_ADDRESS,
            module: ident_str!("url").to_owned(),
            name: ident_str!("Url").to_owned(),
            type_params: vec![],
        },
        fields: vec![(ident_str!("url").to_owned(), string_move_value)],
    });

    let ika_value = IkaMoveValue::from(url_move_value);

    assert!(matches!(ika_value, IkaMoveValue::String(s) if s == test_url));
}

#[test]
fn test_serde() {
    let test_values = [
        IkaMoveValue::Number(u32::MAX),
        IkaMoveValue::UID {
            id: ObjectID::random(),
        },
        IkaMoveValue::String("some test string".to_string()),
        IkaMoveValue::Address(IkaAddress::random_for_testing_only()),
        IkaMoveValue::Bool(true),
        IkaMoveValue::Option(Box::new(None)),
        IkaMoveValue::Vector(vec![
            IkaMoveValue::Number(1000000),
            IkaMoveValue::Number(2000000),
            IkaMoveValue::Number(3000000),
        ]),
    ];

    for value in test_values {
        let json = serde_json::to_string(&value).unwrap();
        let serde_value: IkaMoveValue = serde_json::from_str(&json)
            .map_err(|e| anyhow!("Serde failed for [{:?}], Error msg : {}", value, e))
            .unwrap();
        assert_eq!(
            value, serde_value,
            "Error converting {:?} [{json}], got {:?}",
            value, serde_value
        )
    }
}

#[test]
fn test_serde_bytearray() {
    // ensure that we serialize byte arrays as number array
    let test_values = MoveValue::Vector(vec![MoveValue::U8(1), MoveValue::U8(2), MoveValue::U8(3)]);
    let ika_move_value = IkaMoveValue::from(test_values);
    let json = serde_json::to_value(&ika_move_value).unwrap();
    assert_eq!(json, json!([1, 2, 3]));
}

#[test]
fn test_serde_number() {
    // ensure that we serialize byte arrays as number array
    let test_values = MoveValue::U8(1);
    let ika_move_value = IkaMoveValue::from(test_values);
    let json = serde_json::to_value(&ika_move_value).unwrap();
    assert_eq!(json, json!(1));
    let test_values = MoveValue::U16(1);
    let ika_move_value = IkaMoveValue::from(test_values);
    let json = serde_json::to_value(&ika_move_value).unwrap();
    assert_eq!(json, json!(1));
    let test_values = MoveValue::U32(1);
    let ika_move_value = IkaMoveValue::from(test_values);
    let json = serde_json::to_value(&ika_move_value).unwrap();
    assert_eq!(json, json!(1));
}

#[test]
fn test_type_tag_struct_tag_devnet_inc_222() {
    let offending_tags = [
        "0x1::address::MyType",
        "0x1::vector::MyType",
        "0x1::address::MyType<0x1::address::OtherType>",
        "0x1::address::MyType<0x1::address::OtherType, 0x1::vector::VecTyper>",
        "0x1::address::address<0x1::vector::address, 0x1::vector::vector>",
    ];

    for tag in offending_tags {
        let oc = ObjectChange::Created {
            sender: Default::default(),
            owner: Owner::Immutable,
            object_type: parse_ika_struct_tag(tag).unwrap(),
            object_id: ObjectID::random(),
            version: Default::default(),
            digest: ObjectDigest::random(),
        };

        let serde_json = serde_json::to_string(&oc).unwrap();
        let deser: ObjectChange = serde_json::from_str(&serde_json).unwrap();
        assert_eq!(oc, deser);
    }
}
