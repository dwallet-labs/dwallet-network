// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[test_only]
module pera::versioned_tests {
    use pera::versioned;

    #[test]
    fun test_upgrade() {
        let mut ctx = tx_context::dummy();
        let mut wrapper = versioned::create(1, 1000, &mut ctx);
        assert!(versioned::version(&wrapper) == 1);
        assert!(versioned::load_value(&wrapper) == &1000);
        let (old, cap) = versioned::remove_value_for_upgrade(&mut wrapper);
        assert!(old == 1000);
        versioned::upgrade(&mut wrapper, 2, 2000, cap);
        assert!(versioned::version(&wrapper) == 2);
        assert!(versioned::load_value(&wrapper) == &2000);
        let value = versioned::destroy(wrapper);
        assert!(value == 2000);
    }
}
