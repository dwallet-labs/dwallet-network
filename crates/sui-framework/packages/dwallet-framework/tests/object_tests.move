// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[test_only]
module dwallet::object_tests {
    use dwallet::address;
    use dwallet::object;
    use dwallet::tx_context;

    const EDifferentAddress: u64 = 0xF000;
    const EDifferentBytes: u64 = 0xF001;
    const EAddressRoundTrip: u64 = 0xF002;

    #[test]
    fun test_bytes_address_roundtrip() {
        let ctx = tx_context::dummy();

        let uid0 = object::new(&mut ctx);
        let uid1 = object::new(&mut ctx);

        let addr0 = object::uid_to_address(&uid0);
        let byte0 = object::uid_to_bytes(&uid0);
        let addr1 = object::uid_to_address(&uid1);
        let byte1 = object::uid_to_bytes(&uid1);

        assert!(addr0 != addr1, EDifferentAddress);
        assert!(byte0 != byte1, EDifferentBytes);

        assert!(addr0 == address::from_bytes(byte0), EAddressRoundTrip);
        assert!(addr1 == address::from_bytes(byte1), EAddressRoundTrip);

        object::delete(uid0);
        object::delete(uid1);
    }
}
