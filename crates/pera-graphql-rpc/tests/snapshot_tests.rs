// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use insta::assert_snapshot;
use pera_graphql_rpc::server::builder::export_schema;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_schema_sdl_export() {
    let sdl = export_schema();

    // update the current schema file
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("schema.graphql");
    fs::write(path, &sdl).unwrap();

    assert_snapshot!(sdl);
}
