// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module dep_on_upgrading_package_transitive::my_module {
    use base_addr::base;
    use dep_on_upgrading_package::my_module;

    public fun call_return_0(): u64 { my_module::call_return_0() + base::return_0() }
}
