// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

/// Implements Sui Object Display for user-owned objects.
///
/// The default fields for Display are:
/// - name
/// - description
/// - image_url
/// - link
/// - project_url
///
/// Optionally:
/// - thumbnail_url
/// - creator
module ika_system::display;

use std::type_name;
use sui::{display::{Self, Display}, object_bag::{Self, ObjectBag}, package::Publisher};
use ika_system::staked_ika::StakedIka;

/// The wrapper that stores the objects.
public struct ObjectDisplay has key {
    id: UID,
    inner: ObjectBag,
}

/// The dynamic field key to use
public struct PublisherKey() has copy, drop, store;

/// Creates the `ObjectDisplay` instance with default objects in it.
public(package) fun create(p: Publisher, ctx: &mut TxContext) {
    let mut inner = object_bag::new(ctx);

    inner.add(type_name::get<StakedIka>(), init_staked_ika_display(&p, ctx));
    inner.add(PublisherKey(), p);

    transfer::share_object(ObjectDisplay { id: object::new(ctx), inner })
}


/// Creates initial `Display` for the `StakedIka` type.
fun init_staked_ika_display(p: &Publisher, ctx: &mut TxContext): Display<StakedIka> {
    let mut d = display::new(p, ctx);

    d.add(b"name".to_string(), b"Staked IKA ({principal} INKU)".to_string());
    d.add(
        b"description".to_string(),
        b"Staked for validator: {validator_id}, activates at: {activation_epoch}".to_string(),
    );
    d.add(b"image_url".to_string(), ika::ika::get_staked_ika_icon_url().to_string());
    d.add(b"project_url".to_string(), b"https://ika.xyz/".to_string());
    d.add(b"link".to_string(), b"".to_string());
    d.update_version();
    d
}
