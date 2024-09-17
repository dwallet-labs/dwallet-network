// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use pera_json_rpc::name_service::{self, Domain};
use pera_types::{
    base_types::{ObjectID, PeraAddress},
    collection_types::VecMap,
};
use std::str::FromStr;

#[test]
fn test_parent_extraction() {
    let mut name = Domain::from_str("leaf.node.test.pera").unwrap();

    assert_eq!(name.parent().to_string(), "node.test.pera");

    name = Domain::from_str("node.test.pera").unwrap();

    assert_eq!(name.parent().to_string(), "test.pera");
}

#[test]
fn test_expirations() {
    let system_time: u64 = 100;

    let mut name = name_service::NameRecord {
        nft_id: pera_types::id::ID::new(ObjectID::random()),
        data: VecMap { contents: vec![] },
        target_address: Some(PeraAddress::random_for_testing_only()),
        expiration_timestamp_ms: system_time + 10,
    };

    assert!(!name.is_node_expired(system_time));

    name.expiration_timestamp_ms = system_time - 10;

    assert!(name.is_node_expired(system_time));
}

#[test]
fn test_name_service_outputs() {
    assert_eq!("@test".parse::<Domain>().unwrap().to_string(), "test.pera");
    assert_eq!(
        "test.pera".parse::<Domain>().unwrap().to_string(),
        "test.pera"
    );
    assert_eq!(
        "test@sld".parse::<Domain>().unwrap().to_string(),
        "test.sld.pera"
    );
    assert_eq!(
        "test.test@example".parse::<Domain>().unwrap().to_string(),
        "test.test.example.pera"
    );
    assert_eq!(
        "pera@pera".parse::<Domain>().unwrap().to_string(),
        "pera.pera.pera"
    );

    assert_eq!("@pera".parse::<Domain>().unwrap().to_string(), "pera.pera");

    assert_eq!(
        "test*test@test".parse::<Domain>().unwrap().to_string(),
        "test.test.test.pera"
    );
    assert_eq!(
        "test.test.pera".parse::<Domain>().unwrap().to_string(),
        "test.test.pera"
    );
    assert_eq!(
        "test.test.test.pera".parse::<Domain>().unwrap().to_string(),
        "test.test.test.pera"
    );
}

#[test]
fn test_different_wildcard() {
    assert_eq!("test.pera".parse::<Domain>(), "test*pera".parse::<Domain>(),);

    assert_eq!("@test".parse::<Domain>(), "test*pera".parse::<Domain>(),);
}

#[test]
fn test_invalid_inputs() {
    assert!("*".parse::<Domain>().is_err());
    assert!(".".parse::<Domain>().is_err());
    assert!("@".parse::<Domain>().is_err());
    assert!("@inner.pera".parse::<Domain>().is_err());
    assert!("@inner*pera".parse::<Domain>().is_err());
    assert!("test@".parse::<Domain>().is_err());
    assert!("pera".parse::<Domain>().is_err());
    assert!("test.test@example.pera".parse::<Domain>().is_err());
    assert!("test@test@example".parse::<Domain>().is_err());
}

#[test]
fn output_tests() {
    let mut domain = "test.pera".parse::<Domain>().unwrap();
    assert!(domain.format(name_service::DomainFormat::Dot) == "test.pera");
    assert!(domain.format(name_service::DomainFormat::At) == "@test");

    domain = "test.test.pera".parse::<Domain>().unwrap();
    assert!(domain.format(name_service::DomainFormat::Dot) == "test.test.pera");
    assert!(domain.format(name_service::DomainFormat::At) == "test@test");

    domain = "test.test.test.pera".parse::<Domain>().unwrap();
    assert!(domain.format(name_service::DomainFormat::Dot) == "test.test.test.pera");
    assert!(domain.format(name_service::DomainFormat::At) == "test.test@test");

    domain = "test.test.test.test.pera".parse::<Domain>().unwrap();
    assert!(domain.format(name_service::DomainFormat::Dot) == "test.test.test.test.pera");
    assert!(domain.format(name_service::DomainFormat::At) == "test.test.test@test");
}
