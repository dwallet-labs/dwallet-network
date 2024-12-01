#[test_only]
module pera_system::dwallet_ecdsa_k1_tests {
    use pera::test_scenario;
    use pera::test_scenario::TransactionEffects;
    use pera::test_utils;
    use pera::vec_map::VecMap;
    use pera_system::dwallet;
    use pera_system::dwallet::DWalletCap;
    use pera_system::dwallet_2pc_mpc_ecdsa_k1;
    use pera_system::dwallet_2pc_mpc_ecdsa_k1::{DKGFirstRoundOutput, Presign, SignOutput};
    use pera_system::dwallet_2pc_mpc_ecdsa_k1::{ENotSystemAddress, EDwalletCapMismatch, EDwalletMismatch};

    const SENDER_ADDRESS: address = @0xA;
    const SYSTEM_ADDRESS: address = @0x0;

    const EWrongEventNumber: u64 = 0;
    const EWrongCreatedObjectsNum: u64 = 1;
    const EWrongFrozenObjectsNum: u64 = 2;
    const EWrongTransferredObjectsNum: u64 = 3;
    const EObjectTransferredToWrongAddress: u64 = 4;
    const EWrongTransferredObject: u64 = 5;
    const EWrongSessionAddress: u64 = 7;

    #[test]
    public fun test_launch_dkg_first_round() {
        let mut scenario = test_scenario::begin(SENDER_ADDRESS);
        scenario.next_tx(SENDER_ADDRESS);
        {
            let ctx = scenario.ctx();
            dwallet_2pc_mpc_ecdsa_k1::launch_dkg_first_round(ctx);
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
    public fun test_create_dkg_first_round_output() {
        let mut scenario = test_scenario::begin(SYSTEM_ADDRESS);
        scenario.next_tx(SYSTEM_ADDRESS);
        {
            let ctx = scenario.ctx();
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

        let events_num = test_scenario::num_user_events(&effects);
        assert!(events_num == 0, EWrongEventNumber);

        let created_objects = test_scenario::created(&effects);
        assert!(std::vector::length(&created_objects) == 1, EWrongCreatedObjectsNum);

        let frozen_objects = test_scenario::frozen(&effects);
        assert!(std::vector::length(&frozen_objects) == 0, EWrongFrozenObjectsNum);

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
        {
            let ctx = test_scenario::ctx(&mut scenario);
            let dwallet_cap = dwallet::create_dwallet_cap(ctx);
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

        let events_num = test_scenario::num_user_events(&effects);
        assert!(events_num == 1, EWrongEventNumber);

        let created_objects = test_scenario::created(&effects);
        assert!(std::vector::length(&created_objects) == 0, EWrongCreatedObjectsNum);
    }

    #[test]
    public fun test_create_dkg_second_round_output() {
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

        let events_num = test_scenario::num_user_events(&effects);
        assert!(events_num == 1, EWrongEventNumber);

        let created_objects = test_scenario::created(&effects);
        assert!(std::vector::length(&created_objects) == 1, EWrongCreatedObjectsNum);

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

    #[test]
    public fun test_launch_presign_first_round() {
        let sender = SENDER_ADDRESS;
        let mut scenario = test_scenario::begin(sender);

        // Create necessary objects before the transaction
        let dwallet_cap;
        let dwallet;

        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);
            dwallet_cap = dwallet::create_dwallet_cap(ctx);
            let dkg_output: vector<u8> = std::vector::singleton(0xAA);
            dwallet = pera_system::dwallet_2pc_mpc_ecdsa_k1::create_mock_dwallet(dkg_output, ctx);
        };

        // Call `launch_presign_first_round` in a new transaction
        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);

            // Emit the event for the first round of presign
            dwallet_2pc_mpc_ecdsa_k1::launch_presign_first_round(&dwallet, ctx);

            // Clean up created objects
            test_utils::destroy(dwallet);
            test_utils::destroy(dwallet_cap);
        };

        let effects: TransactionEffects = test_scenario::end(scenario);

        // Verify the expected event was emitted
        let events_num = test_scenario::num_user_events(&effects);
        assert!(events_num == 1, EWrongEventNumber);

        // Ensure no new objects were created
        let created_objects = test_scenario::created(&effects);
        assert!(std::vector::length(&created_objects) == 0, EWrongCreatedObjectsNum);

        // Ensure no frozen objects
        let frozen_objects = test_scenario::frozen(&effects);
        assert!(std::vector::length(&frozen_objects) == 0, EWrongFrozenObjectsNum);
    }

    #[test]
    public fun test_launch_presign_second_round() {
        let sender = SYSTEM_ADDRESS;
        let initiator = SENDER_ADDRESS;
        let mut scenario = test_scenario::begin(sender);

        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);

            let dwallet_id = object::id_from_address(@0x01);
            let dwallet_cap_id = object::id_from_address(@0x02);
            let first_round_session_id = object::id_from_address(@0x03);
            let dkg_output: vector<u8> = std::vector::singleton(0xAA);
            let first_round_output: vector<u8> = std::vector::singleton(0xBB);

            dwallet_2pc_mpc_ecdsa_k1::launch_presign_second_round_for_testing(
                initiator,
                dwallet_id,
                dkg_output,
                dwallet_cap_id,
                first_round_output,
                first_round_session_id,
                ctx,
            );

            test_utils::destroy(dwallet_id);
            test_utils::destroy(dwallet_cap_id);
            test_utils::destroy(first_round_session_id);
        };

        let effects: TransactionEffects = test_scenario::end(scenario);
        let events_num = test_scenario::num_user_events(&effects);
        assert!(events_num == 1, EWrongEventNumber);
    }

    #[test]
    public fun test_create_second_presign_round_output() {
        let sender = SYSTEM_ADDRESS;
        let initiator = SENDER_ADDRESS;
        let mut scenario = test_scenario::begin(sender);

        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);

            let session_id = object::id_from_address(@0x01);
            let first_round_session_id = object::id_from_address(@0x02);
            let dwallet_cap_id = object::id_from_address(@0x03);
            let dwallet_id = object::id_from_address(@0x04);
            let first_round_output: vector<u8> = std::vector::singleton(0xAA);
            let second_round_output: vector<u8> = std::vector::singleton(0xAB);

            dwallet_2pc_mpc_ecdsa_k1::create_second_presign_round_output_for_testing(
                initiator,
                session_id,
                first_round_session_id,
                first_round_output,
                second_round_output,
                dwallet_cap_id,
                dwallet_id,
                ctx,
            );

            test_utils::destroy(session_id);
            test_utils::destroy(dwallet_cap_id);
            test_utils::destroy(dwallet_id);
        };

        let effects: TransactionEffects = test_scenario::end(scenario);

        let events_num = test_scenario::num_user_events(&effects);
        assert!(events_num == 1, EWrongEventNumber);

        let created_objects = test_scenario::created(&effects);
        assert!(std::vector::length(&created_objects) == 1, EWrongCreatedObjectsNum);

        let transferred_objects = test_scenario::ids_for_address<Presign>(initiator);
        assert!(std::vector::length(&transferred_objects) == 1, EWrongTransferredObjectsNum);

        let transferred_map: VecMap<ID, address> = test_scenario::transferred_to_account(&effects);
        let (id, address) = transferred_map.get_entry_by_idx(0);
        assert!(*address == initiator, EObjectTransferredToWrongAddress);
        assert!(id == transferred_objects[0], EWrongTransferredObject);
    }

    #[test]
    public fun test_sign() {
        let sender = SENDER_ADDRESS;
        let mut scenario = test_scenario::begin(sender);
        let dwallet;
        let dwallet_cap_id;
        let dwallet_cap;
        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);
            let dkg_output: vector<u8> = std::vector::singleton(0xAA);
            dwallet = pera_system::dwallet_2pc_mpc_ecdsa_k1::create_mock_dwallet(dkg_output, ctx);
            dwallet_cap_id = dwallet::get_dwallet_cap_id(&dwallet);
        };

        let presign;

        test_scenario::next_tx(&mut scenario, sender);
        {
            dwallet_cap = test_scenario::take_from_address<DWalletCap>(&scenario, sender);
            let ctx = test_scenario::ctx(&mut scenario);
            let presign_first_round_output: vector<u8> = std::vector::singleton(0xAA);
            let presign_second_round_output: vector<u8> = std::vector::singleton(0xAB);
            let first_round_session_id = object::id_from_address(tx_context::fresh_object_address(ctx));
            presign = pera_system::dwallet_2pc_mpc_ecdsa_k1::create_mock_presign(
                object::id(&dwallet),
                dwallet_cap_id,
                presign_first_round_output,
                presign_second_round_output,
                first_round_session_id,
                ctx,
            );
        };

        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);

            let hashed_message: vector<u8> = std::vector::singleton(0xAA);
            let centralized_signed_message: vector<u8> = std::vector::singleton(0xDD);
            let presign_session_id = object::id_from_address(@0x03);

            pera_system::dwallet_2pc_mpc_ecdsa_k1::sign(
                &dwallet_cap,
                hashed_message,
                &dwallet,
                &presign,
                centralized_signed_message,
                presign_session_id,
                ctx
            );

            test_utils::destroy(dwallet_cap);
            test_utils::destroy(dwallet);
            test_utils::destroy(presign);
        };

        let effects: TransactionEffects = test_scenario::end(scenario);

        let events_num = test_scenario::num_user_events(&effects);
        assert!(events_num == 1, EWrongEventNumber);
    }


    #[test]
    public fun test_create_sign_output() {
        let sender = SYSTEM_ADDRESS;
        let initiator = SENDER_ADDRESS;
        let mut scenario = test_scenario::begin(sender);

        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);

            let session_id = object::id_from_address(@0x01);
            let dwallet_id = object::id_from_address(@0x02);
            let output: vector<u8> = std::vector::singleton(0xAA);

            dwallet_2pc_mpc_ecdsa_k1::create_sign_output_for_testing(
                dwallet_id,
                initiator,
                session_id,
                output,
                ctx
            );

            test_utils::destroy(session_id);
        };

        let effects: TransactionEffects = test_scenario::end(scenario);

        let created_objects = test_scenario::created(&effects);
        assert!(std::vector::length(&created_objects) == 1, EWrongCreatedObjectsNum);

        let transferred_objects = test_scenario::ids_for_address<SignOutput>(initiator);
        assert!(std::vector::length(&transferred_objects) == 1, EWrongTransferredObjectsNum);

        // Verify that the object was transferred to the correct address.
        let transferred_map: VecMap<ID, address> = test_scenario::transferred_to_account(&effects);
        let (id, address) = transferred_map.get_entry_by_idx(0);
        assert!(*address == initiator, EObjectTransferredToWrongAddress);
        assert!(id == transferred_objects[0], EWrongTransferredObject);
    }

    #[test]
    #[expected_failure(abort_code = EDwalletCapMismatch)]
    public fun test_sign_fails_due_to_invalid_dwallet_cap() {
        let sender = SENDER_ADDRESS;
        let mut scenario = test_scenario::begin(sender);

        let dwallet;
        let presign;

        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);
            let dkg_output: vector<u8> = std::vector::singleton(0xAA);
            dwallet = pera_system::dwallet_2pc_mpc_ecdsa_k1::create_mock_dwallet(dkg_output, ctx);
        };

        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);
            let presign_first_round_output: vector<u8> = std::vector::singleton(0xAA);
            let presign_second_round_output: vector<u8> = std::vector::singleton(0xAB);
            let first_round_session_id = object::id_from_address(tx_context::fresh_object_address(ctx));
            presign = pera_system::dwallet_2pc_mpc_ecdsa_k1::create_mock_presign(
                object::id(&dwallet),
                dwallet::get_dwallet_cap_id(&dwallet),
                presign_first_round_output,
                presign_second_round_output,
                first_round_session_id,
                ctx,
            );
        };

        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);

            let invalid_dwallet_cap = dwallet::create_dwallet_cap(ctx);
            let hashed_message: vector<u8> = std::vector::singleton(0xAA);
            let centralized_signed_message: vector<u8> = std::vector::singleton(0xDD);
            let presign_session_id = object::id_from_address(@0x03);

            pera_system::dwallet_2pc_mpc_ecdsa_k1::sign(
                &invalid_dwallet_cap,
                hashed_message,
                &dwallet,
                &presign,
                centralized_signed_message,
                presign_session_id,
                ctx
            );
            test_utils::destroy(presign);
            test_utils::destroy(invalid_dwallet_cap);
            test_utils::destroy(dwallet);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = EDwalletMismatch)]
    public fun test_sign_fails_due_to_invalid_dwallet_id() {
        let sender = SENDER_ADDRESS;
        let mut scenario = test_scenario::begin(sender);

        let dwallet_cap;
        let dwallet;
        let invalid_dwallet;
        let presign;

        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);
            let dkg_output: vector<u8> = std::vector::singleton(0xAA);
            let dkg_output2: vector<u8> = std::vector::singleton(0xAB);
            dwallet = pera_system::dwallet_2pc_mpc_ecdsa_k1::create_mock_dwallet(dkg_output, ctx);
            invalid_dwallet = pera_system::dwallet_2pc_mpc_ecdsa_k1::create_mock_dwallet(dkg_output2, ctx);
        };

        test_scenario::next_tx(&mut scenario, sender);
        {
            dwallet_cap = test_scenario::take_from_address<DWalletCap>(&scenario, sender);
            let ctx = test_scenario::ctx(&mut scenario);
            let presign_first_round_output: vector<u8> = std::vector::singleton(0xAA);
            let presign_second_round_output: vector<u8> = std::vector::singleton(0xAB);
            let first_round_session_id = object::id_from_address(tx_context::fresh_object_address(ctx));
            presign = pera_system::dwallet_2pc_mpc_ecdsa_k1::create_mock_presign(
                object::id(&dwallet),
                object::id(&dwallet_cap),
                presign_first_round_output,
                presign_second_round_output,
                first_round_session_id,
                ctx,
            );
        };

        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);

            // Call the `sign` function with the incorrect DWallet
            let hashed_message: vector<u8> = std::vector::singleton(0xAA);
            let centralized_signed_message: vector<u8> = std::vector::singleton(0xDD);
            let presign_session_id = object::id_from_address(@0x03);

            pera_system::dwallet_2pc_mpc_ecdsa_k1::sign(
                &dwallet_cap,
                hashed_message,
                &invalid_dwallet,
                &presign,
                centralized_signed_message,
                presign_session_id,
                ctx
            );

            test_utils::destroy(presign);
            test_utils::destroy(invalid_dwallet);
            test_utils::destroy(dwallet_cap);
            test_utils::destroy(dwallet);
        };

        test_scenario::end(scenario);
    }
}