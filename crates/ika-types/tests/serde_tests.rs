// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use ika_types::base_types::ObjectType;
use ika_types::ika_serde::IkaStructTag;
use ika_types::parse_ika_struct_tag;
use move_core_types::language_storage::StructTag;
use serde::Serialize;
use serde_json::Value;
use serde_with::serde_as;
use std::str::FromStr;

#[test]
fn test_struct_tag_serde() {
    let tag = parse_ika_struct_tag("0x7f89cdffd8968affa0b47bef91adc5314e19509080470c45bfd434cd83a766b::ikafrens::IkaFren<0x7f89cdffd8968affa0b47bef91adc5314e19509080470c45bfd434cd83a766b::capy::Capy>").unwrap();
    #[serde_as]
    #[derive(Serialize)]
    struct TestStructTag(#[serde_as(as = "IkaStructTag")] StructTag);

    // serialize to json should not trim the leading 0
    let Value::String(json) = serde_json::to_value(TestStructTag(tag.clone())).unwrap() else {
        panic!()
    };
    assert_eq!(json, "0x07f89cdffd8968affa0b47bef91adc5314e19509080470c45bfd434cd83a766b::ikafrens::IkaFren<0x07f89cdffd8968affa0b47bef91adc5314e19509080470c45bfd434cd83a766b::capy::Capy>");

    let tag2 = parse_ika_struct_tag(&json).unwrap();
    assert_eq!(tag, tag2);
}

#[test]
fn test_object_type_to_string() {
    let object_type = ObjectType::from_str(
        "0x1a1aa18691be519899bf5187f5ce80af629407dd4f68d4175b99f4dc09497c1::custodian::AccountCap",
    )
    .unwrap();
    assert_eq!(
        object_type.to_string(),
        "0x01a1aa18691be519899bf5187f5ce80af629407dd4f68d4175b99f4dc09497c1::custodian::AccountCap"
    );
}
