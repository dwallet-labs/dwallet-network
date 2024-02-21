// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::collections::BTreeMap;

use move_core_types::ident_str;
use move_core_types::identifier::Identifier;
use serde_json::json;

use crate::base_types::{ObjectDigest, SuiAddress, TransactionDigest};
use crate::event::{Event, EventEnvelope};
use crate::filter::{EventFilter, Filter};
use crate::gas_coin::GasCoin;
use crate::object::OBJECT_START_VERSION;
use crate::{ObjectID, MOVE_STDLIB_ADDRESS, SUI_FRAMEWORK_ADDRESS};

#[test]
fn test_move_event_filter() {
    let event_coin_id = ObjectID::random();
    // Create a test move event, borrowing GasCoin as the MoveEvent object.
    // TODO this is a bit of a nonsensical test as GasCoin does not implement drop, but it likely
    // doesn't matter as we just are testing a BCS type + value
    let move_event = Event {
        package_id: ObjectID::from(SUI_FRAMEWORK_ADDRESS),
        transaction_module: Identifier::from(ident_str!("test_module")),
        sender: SuiAddress::random_for_testing_only(),
        type_: GasCoin::type_(),
        contents: GasCoin::new(event_coin_id, 10000).to_bcs_bytes(),
    };
    let envelope = EventEnvelope {
        timestamp: 0,
        tx_digest: TransactionDigest::random(),
        seq_num: 0,
        event_num: 0,
        event: move_event,
        move_struct_json_value: json!(BTreeMap::from([("balance", 10000)])),
    };

    let filters = vec![
        EventFilter::MoveEventType(GasCoin::type_()),
        EventFilter::Module(Identifier::from(ident_str!("test_module"))),
        EventFilter::Package(ObjectID::from(SUI_FRAMEWORK_ADDRESS)),
        EventFilter::MoveEventField {
            path: "/balance".to_string(),
            value: json!(10000),
        },
    ];

    // All of the filter should return true.
    for filter in &filters {
        assert!(filter.matches(&envelope))
    }

    assert!(EventFilter::MatchAll(filters.clone()).matches(&envelope));
    assert!(EventFilter::MatchAny(filters.clone()).matches(&envelope));

    // This filter should return false
    let false_filter = EventFilter::Package(ObjectID::from(MOVE_STDLIB_ADDRESS));
    assert!(!false_filter.matches(&envelope));

    // Add the false filter to the vec of filter
    let mut filters = filters;
    filters.push(false_filter);

    // Match all should == false and Match Any should still eq true.
    assert!(!EventFilter::MatchAll(filters.clone()).matches(&envelope));
    assert!(EventFilter::MatchAny(filters.clone()).matches(&envelope));
}

/*#[test]
fn test_transfer_filter() {
    let object_id = ObjectID::random();
    let sender = SuiAddress::random_for_testing_only();
    let recipient = Owner::AddressOwner(SuiAddress::random_for_testing_only());
    // Create a test transfer event.
    let move_event = Event::TransferObject {
        package_id: ObjectID::from(SUI_FRAMEWORK_ADDRESS),
        transaction_module: Identifier::from(ident_str!("test_module")),
        sender,
        recipient,
        object_type: "0x2::example::Object".into(),
        object_id,
        version: Default::default(),
    };
    let envelope = EventEnvelope {
        timestamp: 0,
        tx_digest: TransactionDigest::random(),
        seq_num: 1,
        event_num: 0,
        event: move_event,
        move_struct_json_value: None,
    };

    let filters = vec![
        EventFilter::Package(ObjectID::from(SUI_FRAMEWORK_ADDRESS)),
        EventFilter::Module(Identifier::from(ident_str!("test_module"))),
        EventFilter::SenderAddress(sender),
    ];

    // All filter should return true.
    for filter in &filters {
        assert!(
            filter.matches(&envelope),
            "event = {:?}, filter = {:?}",
            envelope,
            filter
        )
    }
}*/

/*#[test]
fn test_publish_filter() {
    let package_id = ObjectID::random();
    let sender = SuiAddress::random_for_testing_only();
    let version = OBJECT_START_VERSION;
    let digest = ObjectDigest::random();
    // Create a test publish event.
    let move_event = Event::Publish {
        sender,
        package_id,
        version,
        digest,
    };
    let envelope = EventEnvelope {
        timestamp: 0,
        tx_digest: TransactionDigest::random(),
        seq_num: 0,
        event_num: 0,
        event: move_event,
        move_struct_json_value: None,
    };

    let filters = vec![
        EventFilter::Package(package_id),
        EventFilter::SenderAddress(sender),
    ];

    // All filter should return true.
    for filter in &filters {
        assert!(
            filter.matches(&envelope),
            "event = {:?}, filter = {:?}",
            envelope,
            filter
        )
    }
}

#[test]
fn test_delete_object_filter() {
    let package_id = ObjectID::random();
    let object_id = ObjectID::random();
    let sender = SuiAddress::random_for_testing_only();
    // Create a test delete object event.
    let move_event = Event::DeleteObject {
        package_id,
        transaction_module: Identifier::from(ident_str!("test_module")),
        sender,
        object_id,
        version: Default::default(),
    };
    let envelope = EventEnvelope {
        timestamp: 0,
        tx_digest: TransactionDigest::random(),
        event_num: 0,
        seq_num: 0,
        event: move_event,
        move_struct_json_value: None,
    };

    let filters = vec![
        EventFilter::Package(package_id),
        EventFilter::Module(Identifier::from(ident_str!("test_module"))),
        EventFilter::SenderAddress(sender),
    ];

    // All filter should return true.
    for filter in &filters {
        assert!(
            filter.matches(&envelope),
            "event = {:?}, filter = {:?}",
            envelope,
            filter
        )
    }
}

#[test]
fn test_new_object_filter() {
    let package_id = ObjectID::random();
    let object_id = ObjectID::random();
    let sender = SuiAddress::random_for_testing_only();
    let recipient = Owner::AddressOwner(SuiAddress::random_for_testing_only());
    // Create a test new object event.
    let move_event = Event::NewObject {
        package_id,
        transaction_module: Identifier::from(ident_str!("test_module")),
        sender,
        recipient,
        object_type: "0x2::example::Object".into(),
        object_id,
        version: OBJECT_START_VERSION,
    };
    let envelope = EventEnvelope {
        timestamp: 0,
        tx_digest: TransactionDigest::random(),
        seq_num: 0,
        event_num: 0,
        event: move_event,
        move_struct_json_value: None,
    };

    let filters = vec![
        EventFilter::Package(package_id),
        EventFilter::Module(Identifier::from(ident_str!("test_module"))),
        EventFilter::SenderAddress(sender),
    ];

    // All filter should return true.
    for filter in &filters {
        assert!(
            filter.matches(&envelope),
            "event = {:?}, filter = {:?}",
            envelope,
            filter
        )
    }
}

#[test]
fn test_epoch_change_filter() {
    // Create a test epoch change event.
    let move_event = Event::EpochChange(0);
    let envelope = EventEnvelope {
        timestamp: 0,
        tx_digest: TransactionDigest::random(),
        seq_num: 1,
        event_num: 0,
        event: move_event,
        move_struct_json_value: None,
    };

    assert!(EventFilter::EventType(EventType::EpochChange).matches(&envelope))
}

#[test]
fn test_checkpoint_filter() {
    // Create a stub move event.
    let move_event = Event::Checkpoint(0);
    let envelope = EventEnvelope {
        timestamp: 0,
        tx_digest: TransactionDigest::random(),
        seq_num: 1,
        event_num: 0,
        event: move_event,
        move_struct_json_value: None,
    };
    assert!(EventFilter::EventType(EventType::Checkpoint).matches(&envelope))
}*/
