// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

fn main() {
    cynic_codegen::register_schema("pera")
        .from_sdl_file("../pera-graphql-rpc/schema.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
