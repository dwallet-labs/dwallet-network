// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module object_no_id::test_only_object_no_id {
    #[test_only]
    struct NotObject has key {f: u64}

    #[test]
    fun bad_share() {
        dwallet::transfer::share_object(NotObject{f: 42});
    }
}
