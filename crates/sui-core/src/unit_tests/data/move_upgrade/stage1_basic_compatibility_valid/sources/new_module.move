// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module base_addr::new_module {
    public fun this_is_a_new_module() { }

    public fun i_can_call_funs_in_other_modules_that_already_existed(): u64 {
        base_addr::friend_module::friend_call()
    }
}
