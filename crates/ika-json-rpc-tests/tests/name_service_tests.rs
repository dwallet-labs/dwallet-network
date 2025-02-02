// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;
use ika_json_rpc::name_service::{self, Domain};
use ika_types::{
    base_types::{ObjectID, IkaAddress},
    collection_types::VecMap,
};

#[test]
fn test_parent_extraction() {
    let mut name = Domain::from_str("leaf.node.test.ika").unwrap();

    assert_eq!(name.parent().to_string(), "node.test.ika");

    name = Domain::from_str("node.test.ika").unwrap();

    assert_eq!(name.parent().to_string(), "test.ika");
}

#[test]
fn test_expirations() {
    let system_time: u64 = 100;

    let mut name = name_service::NameRecord {
        nft_id: ika_types::id::ID::new(ObjectID::random()),
        data: VecMap { contents: vec![] },
        target_address: Some(IkaAddress::random_for_testing_only()),
        expiration_timestamp_ms: system_time + 10,
    };

    assert!(!name.is_node_expired(system_time));

    name.expiration_timestamp_ms = system_time - 10;

    assert!(name.is_node_expired(system_time));
}

#[test]
fn test_name_service_outputs() {
    assert_eq!("@test".parse::<Domain>().unwrap().to_string(), "test.ika");
    assert_eq!(
        "test.ika".parse::<Domain>().unwrap().to_string(),
        "test.ika"
    );
    assert_eq!(
        "test@sld".parse::<Domain>().unwrap().to_string(),
        "test.sld.ika"
    );
    assert_eq!(
        "test.test@example".parse::<Domain>().unwrap().to_string(),
        "test.test.example.ika"
    );
    assert_eq!(
        "ika@ika".parse::<Domain>().unwrap().to_string(),
        "ika.ika.ika"
    );

    assert_eq!("@ika".parse::<Domain>().unwrap().to_string(), "ika.ika");

    assert_eq!(
        "test*test@test".parse::<Domain>().unwrap().to_string(),
        "test.test.test.ika"
    );
    assert_eq!(
        "test.test.ika".parse::<Domain>().unwrap().to_string(),
        "test.test.ika"
    );
    assert_eq!(
        "test.test.test.ika".parse::<Domain>().unwrap().to_string(),
        "test.test.test.ika"
    );
}

#[test]
fn test_different_wildcard() {
    assert_eq!("test.ika".parse::<Domain>(), "test*ika".parse::<Domain>(),);

    assert_eq!("@test".parse::<Domain>(), "test*ika".parse::<Domain>(),);
}

#[test]
fn test_invalid_inputs() {
    assert!("*".parse::<Domain>().is_err());
    assert!(".".parse::<Domain>().is_err());
    assert!("@".parse::<Domain>().is_err());
    assert!("@inner.ika".parse::<Domain>().is_err());
    assert!("@inner*ika".parse::<Domain>().is_err());
    assert!("test@".parse::<Domain>().is_err());
    assert!("ika".parse::<Domain>().is_err());
    assert!("test.test@example.ika".parse::<Domain>().is_err());
    assert!("test@test@example".parse::<Domain>().is_err());
}

#[test]
fn output_tests() {
    let mut domain = "test.ika".parse::<Domain>().unwrap();
    assert!(domain.format(name_service::DomainFormat::Dot) == "test.ika");
    assert!(domain.format(name_service::DomainFormat::At) == "@test");

    domain = "test.test.ika".parse::<Domain>().unwrap();
    assert!(domain.format(name_service::DomainFormat::Dot) == "test.test.ika");
    assert!(domain.format(name_service::DomainFormat::At) == "test@test");

    domain = "test.test.test.ika".parse::<Domain>().unwrap();
    assert!(domain.format(name_service::DomainFormat::Dot) == "test.test.test.ika");
    assert!(domain.format(name_service::DomainFormat::At) == "test.test@test");

    domain = "test.test.test.test.ika".parse::<Domain>().unwrap();
    assert!(domain.format(name_service::DomainFormat::Dot) == "test.test.test.test.ika");
    assert!(domain.format(name_service::DomainFormat::At) == "test.test.test@test");
}
