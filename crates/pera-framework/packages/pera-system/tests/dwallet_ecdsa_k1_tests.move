#[test_only]
module pera_system::dwallet_ecdsa_k1_tests {
    use pera::test_scenario;
    use pera::test_scenario::TransactionEffects;
    use pera::test_utils;
    use pera::vec_map::VecMap;
    use pera_system::dwallet;
    use pera_system::dwallet::DWalletCap;
    use pera_system::dwallet_2pc_mpc_ecdsa_k1;
    use pera_system::dwallet_2pc_mpc_ecdsa_k1::{DKGFirstRoundOutput, Presign};
    use pera_system::dwallet_2pc_mpc_ecdsa_k1::{
        ENotSystemAddress,
        EMesssageApprovalDWalletMismatch,
        EApprovalsAndMessagesLenMismatch,
        EDwalletMismatch,
        EMissingApprovalOrWorngApprovalOrder,
        ECentrailizedsignedMessagesAndMessagesLenMismatch
    };

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
                session_id,
                output,
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
        assert!(id == session_id, EWrongTransferredObject);
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
                session_id,
                output,
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
        assert!(id == &transferred_objects[0], EWrongTransferredObject);
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

        // Third transaction: Approve messages and call the `sign` function.
        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);

            let mut messages_to_approve: vector<vector<u8>> = vector::empty();
            vector::push_back(&mut messages_to_approve, std::vector::singleton(0xAA));
            vector::push_back(&mut messages_to_approve, std::vector::singleton(0xBB));

            let mut message_approvals = pera_system::dwallet_2pc_mpc_ecdsa_k1::approve_messages(
                &dwallet_cap,
                &mut messages_to_approve
            );

            vector::push_back(&mut messages_to_approve, std::vector::singleton(0xAA));
            vector::push_back(&mut messages_to_approve, std::vector::singleton(0xBB));

            let mut centralized_signed_messages: vector<vector<u8>> = vector::empty();
            vector::push_back(&mut centralized_signed_messages, std::vector::singleton(0xDD));
            vector::push_back(&mut centralized_signed_messages, std::vector::singleton(0xEE));

            let presign_session_id = object::id_from_address(@0x03);

            pera_system::dwallet_2pc_mpc_ecdsa_k1::sign(
                dwallet_cap_id,
                &mut message_approvals,
                messages_to_approve,
                &presign,
                &dwallet,
                centralized_signed_messages,
                presign_session_id,
                ctx
            );

            test_utils::destroy(dwallet_cap);
            test_utils::destroy(dwallet);
            test_utils::destroy(presign);
        };

        let effects: TransactionEffects = test_scenario::end(scenario);

        let events_num = test_scenario::num_user_events(&effects);
        assert!(events_num == 3, EWrongEventNumber);
    }

    #[test]
    public fun test_create_sign_output() {
        let sender = SYSTEM_ADDRESS;
        let mut scenario = test_scenario::begin(sender);

        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);


            let mut output: vector<vector<u8>> = vector::empty();
            vector::push_back(&mut output, std::vector::singleton(0xAA));
            let session_id = object::id_from_address(@0x01);

            dwallet_2pc_mpc_ecdsa_k1::create_sign_output_for_testing(
                output,
                session_id,
                ctx
            );
            test_utils::destroy(session_id);
        };

        let effects: TransactionEffects = test_scenario::end(scenario);

        let events_num = test_scenario::num_user_events(&effects);
        assert!(events_num == 1, EWrongEventNumber);
    }


    #[test]
    #[expected_failure(abort_code = EMesssageApprovalDWalletMismatch)]
    public fun test_sign_fails_due_to_invalid_dwallet_cap() {
        let sender = SENDER_ADDRESS;
        let mut scenario = test_scenario::begin(sender);

        let dwallet;
        let presign;
        let invalid_dwallet_cap;

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

        // Third transaction: Create an invalid DWalletCap and attempt to sign.
        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);

            // Create an invalid DWalletCap (this cap does not match the dWallet).
            invalid_dwallet_cap = dwallet::create_dwallet_cap(ctx);

            let mut messages: vector<vector<u8>> = vector::empty();
            vector::push_back(&mut messages, std::vector::singleton(0xAA));

            // Create the approvals using the invalid dwallet cap.
            let mut message_approvals = pera_system::dwallet_2pc_mpc_ecdsa_k1::approve_messages(
                &invalid_dwallet_cap,
                &mut messages
            );

            // Need to push again since it was consumed by approve_messages();.
            vector::push_back(&mut messages, std::vector::singleton(0xAA));


            let mut centralized_signed_messages: vector<vector<u8>> = vector::empty();
            vector::push_back(&mut centralized_signed_messages, std::vector::singleton(0xDD));

            let presign_session_id = object::id_from_address(@0x03);

            // Call the sign function — this should fail with EDwalletCapMismatch.
            pera_system::dwallet_2pc_mpc_ecdsa_k1::sign(
                object::id(&invalid_dwallet_cap),
                &mut message_approvals,
                messages,
                &presign,
                &dwallet,
                centralized_signed_messages,
                presign_session_id,
                ctx
            );

            test_utils::destroy(invalid_dwallet_cap);
            test_utils::destroy(dwallet);
            test_utils::destroy(presign);
        };

        test_scenario::end(scenario);
    }


    #[test]
    #[expected_failure(abort_code = EDwalletMismatch)]
    public fun test_sign_fails_due_to_invalid_dwallet_id() {
        let sender = SENDER_ADDRESS;
        let mut scenario = test_scenario::begin(sender);

        let dwallet;
        let invalid_dwallet;
        let dwallet_cap;
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

        // Third transaction: Attempt to call the `sign` function with an **invalid DWallet**
        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);

            let mut messages: vector<vector<u8>> = vector::empty();
            vector::push_back(&mut messages, std::vector::singleton(0xAA));

            // Create the message approvals (correct dwallet_cap is used).
            let mut message_approvals = pera_system::dwallet_2pc_mpc_ecdsa_k1::approve_messages(
                &dwallet_cap,
                &mut messages
            );

            let mut centralized_signed_messages: vector<vector<u8>> = vector::empty();
            vector::push_back(&mut centralized_signed_messages, std::vector::singleton(0xDD));

            // Presign session ID
            let presign_session_id = object::id_from_address(@0x03);

            // Call the `sign` function with the **invalid dwallet** (this should fail).
            pera_system::dwallet_2pc_mpc_ecdsa_k1::sign(
                object::id(&dwallet_cap),
                &mut message_approvals,
                messages,
                &presign,
                &invalid_dwallet,
                centralized_signed_messages,
                presign_session_id,
                ctx
            );

            test_utils::destroy(invalid_dwallet);
            test_utils::destroy(dwallet);
            test_utils::destroy(dwallet_cap);
            test_utils::destroy(presign);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = EApprovalsAndMessagesLenMismatch)]
    public fun test_sign_fails_due_to_approvals_and_messages_len_mismatch() {
        let sender = SENDER_ADDRESS;
        let mut scenario = test_scenario::begin(sender);

        let dwallet;
        let dwallet_cap;
        let presign;

        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);
            let dkg_output: vector<u8> = std::vector::singleton(0xAA);
            dwallet = pera_system::dwallet_2pc_mpc_ecdsa_k1::create_mock_dwallet(dkg_output, ctx);
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

        // Third transaction: Attempt to call the `sign` function with a mismatch in approvals and messages.
        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);

            let mut messages: vector<vector<u8>> = vector::empty();
            vector::push_back(&mut messages, std::vector::singleton(0xAA));

            // Create the message approvals (correct dwallet_cap is used).
            let mut approvals_messages: vector<vector<u8>> = vector::empty();
            vector::push_back(&mut approvals_messages, std::vector::singleton(0xAA));
            vector::push_back(&mut approvals_messages, std::vector::singleton(0xBB));

            // Here we create a mismatch since there is 1 message, but 2 approvals.
            let mut message_approvals = pera_system::dwallet_2pc_mpc_ecdsa_k1::approve_messages(
                &dwallet_cap,
                &mut approvals_messages
            );

            let mut centralized_signed_messages: vector<vector<u8>> = vector::empty();
            vector::push_back(&mut centralized_signed_messages, std::vector::singleton(0xDD));

            let presign_session_id = object::id_from_address(@0x03);

            pera_system::dwallet_2pc_mpc_ecdsa_k1::sign(
                object::id(&dwallet_cap),
                &mut message_approvals,
                messages,
                &presign,
                &dwallet,
                centralized_signed_messages,
                presign_session_id,
                ctx
            );

            test_utils::destroy(dwallet);
            test_utils::destroy(dwallet_cap);
            test_utils::destroy(presign);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = EMissingApprovalOrWorngApprovalOrder)]
    public fun test_sign_fails_due_to_wrong_approval_order() {
        let sender = SENDER_ADDRESS;
        let mut scenario = test_scenario::begin(sender);

        let dwallet;
        let dwallet_cap;
        let presign;

        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);
            let dkg_output: vector<u8> = std::vector::singleton(0xAA);
            dwallet = pera_system::dwallet_2pc_mpc_ecdsa_k1::create_mock_dwallet(dkg_output, ctx);
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

        // Third transaction: Attempt to call the `sign` function with a wrong approval order.
        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);

            // Hashed messages to be signed (these are in the correct order).
            let mut messages: vector<vector<u8>> = vector::empty();
            vector::push_back(&mut messages, std::vector::singleton(0xAA)); // Message 1
            vector::push_back(&mut messages, std::vector::singleton(0xBB)); // Message 2

            // Create the message approvals (**but in the wrong order**).
            let mut approvals_messages: vector<vector<u8>> = vector::empty();
            vector::push_back(&mut approvals_messages, std::vector::singleton(0xBB));
            vector::push_back(&mut approvals_messages, std::vector::singleton(0xAA));

            // Here we create approvals for the messages, but since the approvals are **out of order**,
            // it will trigger the assertion.
            let mut message_approvals = pera_system::dwallet_2pc_mpc_ecdsa_k1::approve_messages(
                &dwallet_cap,
                &mut approvals_messages
            );

            let mut centralized_signed_messages: vector<vector<u8>> = vector::empty();
            vector::push_back(
                &mut centralized_signed_messages,
                std::vector::singleton(0xDD)
            );
            vector::push_back(
                &mut centralized_signed_messages,
                std::vector::singleton(0xEE)
            );

            let presign_session_id = object::id_from_address(@0x03);

            pera_system::dwallet_2pc_mpc_ecdsa_k1::sign(
                object::id(&dwallet_cap),
                &mut message_approvals,
                messages,
                &presign,
                &dwallet,
                centralized_signed_messages,
                presign_session_id,
                ctx
            );

            test_utils::destroy(dwallet);
            test_utils::destroy(dwallet_cap);
            test_utils::destroy(presign);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = ECentrailizedsignedMessagesAndMessagesLenMismatch)]
    public fun test_sign_fails_due_to_centralized_signed_messages_len_mismatch() {
        let sender = SENDER_ADDRESS;
        let mut scenario = test_scenario::begin(sender);

        let dwallet;
        let dwallet_cap;
        let presign;

        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);
            let dkg_output: vector<u8> = std::vector::singleton(0xAA);
            dwallet = pera_system::dwallet_2pc_mpc_ecdsa_k1::create_mock_dwallet(dkg_output, ctx);
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

            let mut messages: vector<vector<u8>> = vector::empty();
            vector::push_back(&mut messages, std::vector::singleton(0xAA));
            vector::push_back(&mut messages, std::vector::singleton(0xBB));

            // Create message approvals (2 approvals, same as the number of messages)
            let mut message_approvals = pera_system::dwallet_2pc_mpc_ecdsa_k1::approve_messages(
                &dwallet_cap,
                &mut messages
            );

            // Pushback since they were consumed by approve_messages.
            vector::push_back(&mut messages, std::vector::singleton(0xAA));
            vector::push_back(&mut messages, std::vector::singleton(0xBB));


            // Centralized signed messages (only 1 centralized signed message, should be 2).
            let mut centralized_signed_messages: vector<vector<u8>> = vector::empty();
            vector::push_back(
                &mut centralized_signed_messages,
                std::vector::singleton(0xDD)
            );

            let presign_session_id = object::id_from_address(@0x03);

            // Call the `sign` function (should fail due to mismatch).
            pera_system::dwallet_2pc_mpc_ecdsa_k1::sign(
                object::id(&dwallet_cap),
                &mut message_approvals,
                messages,
                &presign,
                &dwallet,
                centralized_signed_messages,
                presign_session_id,
                ctx
            );

            test_utils::destroy(dwallet);
            test_utils::destroy(dwallet_cap);
            test_utils::destroy(presign);
        };

        test_scenario::end(scenario);
    }
}