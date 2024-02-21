// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use insta::assert_snapshot;
use std::fs::write;
use std::path::PathBuf;

#[test]
fn test_schema_sdl_export() {
    let sdl = sui_graphql_rpc::schema_sdl_export();

    // update the current schema file
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(["schema", "current_progress_schema.graphql"]);
    write(path, &sdl).unwrap();

    assert_snapshot!(sdl);
}
