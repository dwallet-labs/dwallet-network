// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module base::friend_module {
    public fun call_friend(): u64 { base::base_module::friend_fun() }
}
