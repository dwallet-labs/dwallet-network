// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use ika_enum_compat_util::*;

use crate::{IkaMoveStruct, IkaMoveValue};

#[test]
fn enforce_order_test() {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(["tests", "staged", "ika_move_struct.yaml"]);
    check_enum_compat_order::<IkaMoveStruct>(path);

    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(["tests", "staged", "ika_move_value.yaml"]);
    check_enum_compat_order::<IkaMoveValue>(path);
}
