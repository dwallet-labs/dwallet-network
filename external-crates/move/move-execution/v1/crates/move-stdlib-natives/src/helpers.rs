// Copyright (c) The Move Contributors
// SPDX-License-Identifier: BSD-3-Clause-Clear

use move_vm_runtime::native_functions::NativeFunction;

pub fn make_module_natives(
    natives: impl IntoIterator<Item = (impl Into<String>, NativeFunction)>,
) -> impl Iterator<Item = (String, NativeFunction)> {
    natives
        .into_iter()
        .map(|(func_name, func)| (func_name.into(), func))
}
