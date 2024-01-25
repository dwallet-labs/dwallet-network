// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module main_package::main_module {
    use dep_package::dep_module;

    fun foo(): u64 {
        dep_module::foo()
    }

}
