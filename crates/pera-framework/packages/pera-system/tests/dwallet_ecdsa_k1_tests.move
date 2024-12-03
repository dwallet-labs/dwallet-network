#[test_only]
module pera_system::dwallet_ecdsa_k1_tests {
    use pera::test_scenario;
    use pera::test_scenario::TransactionEffects;
    use pera::test_utils;
    use pera::vec_map::VecMap;
    use pera_system::dwallet::DWalletCap;
    use pera_system::dwallet;
    use pera_system::dwallet_2pc_mpc_ecdsa_k1;
    use pera_system::dwallet_2pc_mpc_ecdsa_k1::DKGFirstRoundOutput;
    use pera_system::dwallet_2pc_mpc_ecdsa_k1::ENotSystemAddress;

    const SENDER_ADDRESS: address = @0xA;
    const SYSTEM_ADDRESS: address = @0x0;

    const EWrongEventNumber: u64 = 0;
    const EWrongCreatedObjectsNum: u64 = 1;
    const EWrongFrozenObjectsNum: u64 = 2;
    const EWrongTransferredObjectsNum: u64 = 3;
    const EObjectTransferredToWrongAddress: u64 = 4;
    const EWrongTransferredObject: u64 = 5;
    const EWrongSessionAddress: u64 = 7;
    const EWrongDwalletCapAddress: u64 = 8;

    #[test]
    public fun test_launch_dkg_first_round() {
        let mut scenario = test_scenario::begin(SENDER_ADDRESS);
        let scenario_mut = &mut scenario;
        scenario_mut.next_tx(SENDER_ADDRESS);
        {
            let ctx = scenario_mut.ctx();
            let session_id = dwallet_2pc_mpc_ecdsa_k1::launch_dkg_first_round(ctx);
            assert!(session_id != @0x0, EWrongSessionAddress);
            test_utils::destroy(session_id);
        };
        let effects: TransactionEffects = scenario.end();

        let events_num = test_scenario::num_user_events(&effects);
        assert!(events_num == 1, EWrongEventNumber);

        let created_objects = test_scenario::created(&effects);
        assert!(std::vector::length(&created_objects) == 1, EWrongCreatedObjectsNum);

        let frozen_objects = test_scenario::frozen(&effects);
        assert!(std::vector::length(&frozen_objects) == 0, EWrongFrozenObjectsNum);
    }

    #[test]
    public fun test_create_dkg_first_round_output_for_testing() {
        let mut scenario = test_scenario::begin(SYSTEM_ADDRESS);
        let scenario_mut = &mut scenario;
        scenario_mut.next_tx(SYSTEM_ADDRESS);
        {
            let ctx = scenario_mut.ctx();
            let session_id = object::id_from_address(@0x10);
            let dwallet_cap_id = object::id_from_address(@0x11);
            let output: vector<u8> = std::vector::empty();

            dwallet_2pc_mpc_ecdsa_k1::create_dkg_first_round_output_for_testing(
                SENDER_ADDRESS,
                session_id,
                output,
                dwallet_cap_id,
                ctx,
            );

            test_utils::destroy(session_id);
            test_utils::destroy(dwallet_cap_id);
        };

        let effects: TransactionEffects = scenario.end();

        // Verify the emitted events.
        let events_num = test_scenario::num_user_events(&effects);
        assert!(events_num == 1, EWrongEventNumber);

        // Verify the number of created objects.
        let created_objects = test_scenario::created(&effects);
        assert!(std::vector::length(&created_objects) == 1, EWrongCreatedObjectsNum);

        // Verify the number of frozen objects.
        let frozen_objects = test_scenario::frozen(&effects);
        assert!(std::vector::length(&frozen_objects) == 0, EWrongFrozenObjectsNum);


        // Make sure that the initiator of the session, got the object.
        let sessions_transferred = test_scenario::ids_for_address<DKGFirstRoundOutput>(SENDER_ADDRESS);
        assert!(std::vector::length(&sessions_transferred) == 1, EWrongTransferredObjectsNum);

        let session_id = std::vector::borrow(&sessions_transferred, 0);
        let transferred_objects: VecMap<ID, address> = test_scenario::transferred_to_account(&effects);
        let (id, address) = transferred_objects.get_entry_by_idx(0);
        assert!(*address == SENDER_ADDRESS, EObjectTransferredToWrongAddress);
        assert!(id == *session_id, EWrongTransferredObject);
    }

    #[test]
    public fun test_launch_dkg_second_round() {
        let sender = SENDER_ADDRESS;
        let mut scenario = test_scenario::begin(sender);
        test_scenario::next_tx(&mut scenario, sender);
        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);
            let cap_id = dwallet::create_dwallet_cap(ctx);
            assert!(cap_id != object::id_from_address(@0x0), EWrongDwalletCapAddress);
        };
        test_scenario::next_tx(&mut scenario, sender);
        {
            let dwallet_cap = test_scenario::take_from_address<DWalletCap>(&scenario, sender);
            let first_round_output: vector<u8> = std::vector::empty();
            let public_key_share_and_proof: vector<u8> = std::vector::empty();
            let first_round_session_id = object::id_from_address(@0x10);

            let session_id = dwallet_2pc_mpc_ecdsa_k1::launch_dkg_second_round(
                &dwallet_cap,
                public_key_share_and_proof,
                first_round_output,
                first_round_session_id,
                test_scenario::ctx(&mut scenario),
            );

            assert!(session_id != @0x0, EWrongSessionAddress);
            test_utils::destroy(first_round_session_id);
            test_utils::destroy(dwallet_cap);
        };

        let effects: TransactionEffects = test_scenario::end(scenario);

        // Verify the emitted events
        let events_num = test_scenario::num_user_events(&effects);
        assert!(events_num == 1, EWrongEventNumber);

        // Verify the number of created objects.
        let created_objects = test_scenario::created(&effects);
        assert!(std::vector::length(&created_objects) == 0, EWrongCreatedObjectsNum);
    }

    #[test]
    public fun test_create_dkg_second_round_output_for_testing() {
        let mut scenario = test_scenario::begin(SYSTEM_ADDRESS);
        test_scenario::next_tx(&mut scenario, SYSTEM_ADDRESS);
        {
            let ctx = test_scenario::ctx(&mut scenario);

            let session_id = object::id_from_address(@0x20);
            let dwallet_cap_id = object::id_from_address(@0x30);
            let output: vector<u8> = std::vector::empty();

            dwallet_2pc_mpc_ecdsa_k1::create_dkg_second_round_output_for_testing(
                SENDER_ADDRESS,
                session_id,
                output,
                dwallet_cap_id,
                ctx,
            );

            test_utils::destroy(session_id);
            test_utils::destroy(dwallet_cap_id);
        };

        let effects: TransactionEffects = test_scenario::end(scenario);

        // Verify the emitted events.
        let events_num = test_scenario::num_user_events(&effects);
        assert!(events_num == 1, EWrongEventNumber);

        // Verify the number of created objects.
        let created_objects = test_scenario::created(&effects);
        assert!(std::vector::length(&created_objects) == 1, EWrongCreatedObjectsNum);

        // Verify the frozen objects.
        let frozen_objects = test_scenario::frozen(&effects);
        assert!(std::vector::length(&frozen_objects) == 1, EWrongFrozenObjectsNum);
    }

    #[test]
    #[expected_failure(abort_code = ENotSystemAddress)]
    public fun test_create_dkg_first_round_output_not_system_address() {
        let sender = SENDER_ADDRESS;
        let mut scenario = test_scenario::begin(sender);
        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);
            let session_id = object::id_from_address(@0x10);
            let dwallet_cap_id = object::id_from_address(@0x11);
            let output: vector<u8> = std::vector::empty();

            dwallet_2pc_mpc_ecdsa_k1::create_dkg_first_round_output_for_testing(
                sender,
                session_id,
                output,
                dwallet_cap_id,
                ctx,
            );
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = ENotSystemAddress)]
    public fun test_create_dkg_second_round_output_not_system_address() {
        let sender = SENDER_ADDRESS;
        let mut scenario = test_scenario::begin(sender);
        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);
            let session_id = object::id_from_address(@0x20);
            let dwallet_cap_id = object::id_from_address(@0x30);
            let output: vector<u8> = std::vector::empty();

            dwallet_2pc_mpc_ecdsa_k1::create_dkg_second_round_output_for_testing(
                sender,
                session_id,
                output,
                dwallet_cap_id,
                ctx,
            );
        };

        test_scenario::end(scenario);
    }
}