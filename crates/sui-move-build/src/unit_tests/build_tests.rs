// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::path::Path;

use crate::BuildConfig;

#[test]
fn generate_struct_layouts() {
    // build the Sui framework and generate struct layouts to make sure nothing crashes
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
        .join("sui-framework")
        .join("packages")
        .join("dwallet-framework");
    let pkg = BuildConfig::new_for_testing().build(path).unwrap();
    let registry = pkg.generate_struct_layouts();
    // check for a couple of types that aren't likely to go away
    assert!(registry.contains_key(
        "0000000000000000000000000000000000000000000000000000000000000001::string::String"
    ));
    assert!(registry.contains_key(
        "0000000000000000000000000000000000000000000000000000000000000002::object::UID"
    ));
    assert!(registry.contains_key(
        "0000000000000000000000000000000000000000000000000000000000000002::tx_context::TxContext"
    ));
}
