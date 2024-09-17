// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use pera_enum_compat_util::*;

use crate::{PeraMoveStruct, PeraMoveValue};

#[test]
fn enforce_order_test() {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(["tests", "staged", "pera_move_struct.yaml"]);
    check_enum_compat_order::<PeraMoveStruct>(path);

    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(["tests", "staged", "pera_move_value.yaml"]);
    check_enum_compat_order::<PeraMoveValue>(path);
}
