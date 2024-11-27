#[test_only]
module pera_system::dwallet_ecdsa_k1_tests {
    use pera::test_scenario;
    use pera::test_scenario::TransactionEffects;
    use pera::test_utils;
    use pera::vec_map::VecMap;
    use pera_system::dwallet::DWalletCap;
    use pera_system::dwallet;
    use pera_system::dwallet_2pc_mpc_ecdsa_k1;
    use pera_system::dwallet_2pc_mpc_ecdsa_k1::{DKGFirstRoundOutput, PresignSessionOutput, Presign, SignOutput};
    use pera_system::dwallet_2pc_mpc_ecdsa_k1::ENotSystemAddress;
    use pera_system::dwallet_2pc_mpc_ecdsa_k1::Secp256K1;

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
    public fun test_create_dkg_first_round_output() {
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
        let mut scenario = test_scenario::begin(SENDER_ADDRESS);

        let dwallet_cap_id = object::id_from_address(@0x01);
        let dkg_output: vector<u8> = std::vector::empty();
        let dwallet = dwallet::create_dwallet<Secp256K1>(
            object::id_from_address(@0x02),
            dwallet_cap_id,
            dkg_output,
            test_scenario::ctx(&mut scenario),
        );

        test_scenario::next_tx(&mut scenario, SENDER_ADDRESS);
        {
            let ctx = test_scenario::ctx(&mut scenario);
            dwallet_2pc_mpc_ecdsa_k1::launch_presign_first_round(&dwallet, ctx);
        };

        let effects: TransactionEffects = test_scenario::end(scenario);
        let events_num = test_scenario::num_user_events(&effects);
        assert!(events_num == 1, EWrongEventNumber);
        test_utils::destroy(dwallet);
    }

    #[test]
    public fun test_create_first_presign_round_output_and_launch_second_round() {
        let sender = SYSTEM_ADDRESS;
        let initiator = SENDER_ADDRESS;
        let mut scenario = test_scenario::begin(sender);

        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);

            let session_id = object::id_from_address(@0x01);
            let dwallet_cap_id = object::id_from_address(@0x02);
            let dwallet_id = object::id_from_address(@0x03);
            let first_round_output: vector<u8> = std::vector::singleton(0xAA);
            let dkg_output: vector<u8> = std::vector::singleton(0xBB);

            dwallet_2pc_mpc_ecdsa_k1::create_first_presign_round_output_and_launch_second_round_for_testing(
                initiator,
                session_id,
                first_round_output,
                dwallet_cap_id,
                dwallet_id,
                dkg_output,
                ctx,
            );

            test_utils::destroy(session_id);
            test_utils::destroy(dwallet_cap_id);
            test_utils::destroy(dwallet_id);
        };

        let effects: TransactionEffects = test_scenario::end(scenario);

        let created_objects = test_scenario::created(&effects);
        assert!(std::vector::length(&created_objects) == 1, EWrongCreatedObjectsNum);

        let frozen_objects = test_scenario::frozen(&effects);
        assert!(std::vector::length(&frozen_objects) == 0, EWrongFrozenObjectsNum);

        let transferred_objects = test_scenario::ids_for_address<PresignSessionOutput>(initiator);
        assert!(std::vector::length(&transferred_objects) == 1, EWrongTransferredObjectsNum);

        // Verify that the correct object was transferred.
        let transferred_map: VecMap<ID, address> = test_scenario::transferred_to_account(&effects);
        let (id, address) = transferred_map.get_entry_by_idx(0);
        assert!(*address == initiator, EObjectTransferredToWrongAddress);
        assert!(id == transferred_objects[0], EWrongTransferredObject);

        let events_num = test_scenario::num_user_events(&effects);
        assert!(events_num == 1, EWrongEventNumber);
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
            let dwallet_cap_id = object::id_from_address(@0x02);
            let dwallet_id = object::id_from_address(@0x03);
            let presign_output: vector<u8> = std::vector::singleton(0xAA);

            dwallet_2pc_mpc_ecdsa_k1::create_second_presign_round_output_for_testing(
                initiator,
                session_id,
                presign_output,
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

        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);

            let hashed_message: vector<u8> = std::vector::singleton(0xAA);
            let presign: vector<u8> = std::vector::singleton(0xBB);
            let dkg_output: vector<u8> = std::vector::singleton(0xCC);
            let centralized_signed_message: vector<u8> = std::vector::singleton(0xDD);
            let presign_session_id = object::id_from_address(@0x01);

            dwallet_2pc_mpc_ecdsa_k1::sign(
                hashed_message,
                presign,
                dkg_output,
                centralized_signed_message,
                presign_session_id,
                ctx,
            );

            test_utils::destroy(presign_session_id);
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
            let signing_output: vector<u8> = std::vector::singleton(0xAA);

            dwallet_2pc_mpc_ecdsa_k1::create_sign_output_for_testing(
                initiator,
                session_id,
                signing_output,
                ctx,
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
}