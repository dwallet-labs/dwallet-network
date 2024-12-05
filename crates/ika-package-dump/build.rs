// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

fn main() {
    cynic_codegen::register_schema("ika")
        .from_sdl_file("../ika-graphql-rpc/schema.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
