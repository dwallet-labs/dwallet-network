#[test_only]
module dwallet_system::dwallet_tests {
    use std::vector;

    use dwallet::object::Self;
    use dwallet::test_scenario;
    use dwallet::test_scenario::TransactionEffects;
    use dwallet::test_utils;
    use dwallet::tx_context;

    use dwallet_system::dwallet;
    use dwallet_system::dwallet::{create_dwallet_cap, EMesssageApprovalDWalletMismatch};
    use dwallet_system::dwallet_2pc_mpc_ecdsa_k1::{create_mock_sign_data, create_mock_sign_data_event};

    fun set_up(): (address, test_scenario::Scenario) {
        let sender = @0x1;
        let scenario = test_scenario::begin(sender);
        (sender, scenario)
    }

    // <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<
    const EWrongEventNumber: u64 = 0;
    const EWrongFrozenObjectsNum: u64 = 1;
    const EWrongCreatedObjectsNum: u64 = 2;
    const EObjectMismatchCreateAndFrozen: u64 = 3;
    // <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<

    #[test]
    public fun test_sign_succesfull() {
        let (sender, scenario) = set_up();
        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);
            let dwallet_id = object::id_from_address(@0xC);
            let dwallet_cap = create_dwallet_cap(ctx);

            let messages = vector::empty<vector<u8>>();
            vector::push_back(&mut messages, b"message_1");
            vector::push_back(&mut messages, b"message_1");
            let message_approvals = dwallet::approve_messages(&dwallet_cap, messages);

            let sign_data = create_mock_sign_data(object::id_from_address(@0xA));
            let sign_data_event = create_mock_sign_data_event(object::id_from_address(@0xA));

            let sign_messages = vector::empty<vector<u8>>();
            vector::push_back(&mut sign_messages, b"message_1");
            vector::push_back(&mut sign_messages, b"message_1");

            let partial_user_signed_messages = dwallet::create_partial_user_signed_messages(
                dwallet_id,
                object::id(&dwallet_cap),
                sign_messages,
                vector::empty(),
                sign_data,
                sign_data_event,
                ctx
            );

            dwallet::sign(partial_user_signed_messages, message_approvals, ctx);
            test_utils::destroy(dwallet_cap);
        };
        let effects: TransactionEffects = test_scenario::end(scenario);

        // Make sure that all transaction effects are correct.
        let events_num = test_scenario::num_user_events(&effects);
        assert!(events_num == 1, EWrongEventNumber);
        let frozen_objects = test_scenario::frozen(&effects);
        let created_objects = test_scenario::created(&effects);
        assert!(vector::length(&frozen_objects) == 1, EWrongFrozenObjectsNum);
        assert!(vector::length(&created_objects) == 1, EWrongCreatedObjectsNum);
        let c = vector::borrow(&created_objects, 0);
        let f = vector::borrow(&frozen_objects, 0);
        assert!(c == f, EObjectMismatchCreateAndFrozen);
    }

    #[test]
    #[expected_failure(abort_code = EMesssageApprovalDWalletMismatch)]
    public fun test_sign_with_vector_length_missmatch() {
        let ctx = &mut tx_context::dummy();
        let dwallet_id = object::id_from_address(@0xC);
        let dwallet_cap = create_dwallet_cap(ctx);

        let messages = vector::empty<vector<u8>>();
        vector::push_back(&mut messages, b"message_1");
        let message_approvals = dwallet::approve_messages(&dwallet_cap, messages);

        let sign_data = create_mock_sign_data(object::id_from_address(@0xA));
        let sign_data_event = create_mock_sign_data_event(object::id_from_address(@0xA));

        let sign_messages = vector::empty<vector<u8>>();
        vector::push_back(&mut sign_messages, b"message_1");
        vector::push_back(&mut sign_messages, b"message_2");

        let partial_user_signed_messages = dwallet::create_partial_user_signed_messages(
            dwallet_id,
            object::id(&dwallet_cap),
            sign_messages,
            vector::empty(),
            sign_data,
            sign_data_event,
            ctx
        );

        dwallet::sign(partial_user_signed_messages, message_approvals, ctx);
        test_utils::destroy(dwallet_cap);
    }

    #[test]
    #[expected_failure(abort_code = EMesssageApprovalDWalletMismatch)]
    public fun test_sign_with_extra_approved_messages() {
        let ctx = &mut tx_context::dummy();

        let messages = vector::empty<vector<u8>>();
        vector::push_back(&mut messages, b"message_1");
        vector::push_back(&mut messages, b"message_2");

        let dwallet_id = object::id_from_address(@0xC);
        let dwallet_cap = create_dwallet_cap(ctx);
        let message_approvals = dwallet::approve_messages(&dwallet_cap, messages);

        let sign_data = create_mock_sign_data(object::id_from_address(@0xA));

        let sign_data_event = create_mock_sign_data_event(object::id_from_address(@0xA));

        let sign_messages = vector::empty<vector<u8>>();
        vector::push_back(&mut sign_messages, b"message_1");

        let partial_user_signed_messages = dwallet::create_partial_user_signed_messages(
            dwallet_id,
            object::id(&dwallet_cap),
            sign_messages,
            vector::empty(),
            sign_data,
            sign_data_event,
            ctx
        );

        dwallet::sign(partial_user_signed_messages, message_approvals, ctx);
        test_utils::destroy(dwallet_cap);
    }

    #[test]
    #[expected_failure(abort_code = EMesssageApprovalDWalletMismatch)]
    public fun test_sign_with_different_messages_order() {
        let ctx = &mut tx_context::dummy();

        let messages = vector::empty<vector<u8>>();
        vector::push_back(&mut messages, b"message_2");
        vector::push_back(&mut messages, b"message_1");

        let dwallet_id = object::id_from_address(@0xC);
        let dwallet_cap = create_dwallet_cap(ctx);
        let message_approvals = dwallet::approve_messages(&dwallet_cap, messages);

        let sign_data = create_mock_sign_data(object::id_from_address(@0xA));

        let sign_data_event = create_mock_sign_data_event(object::id_from_address(@0xA));

        let sign_messages = vector::empty<vector<u8>>();
        vector::push_back(&mut sign_messages, b"message_1");
        vector::push_back(&mut sign_messages, b"message_2");

        let partial_user_signed_messages = dwallet::create_partial_user_signed_messages(
            dwallet_id,
            object::id(&dwallet_cap),
            sign_messages,
            vector::empty(),
            sign_data,
            sign_data_event,
            ctx
        );

        dwallet::sign(partial_user_signed_messages, message_approvals, ctx);
        test_utils::destroy(dwallet_cap);
    }

    #[test]
    #[expected_failure(abort_code = EMesssageApprovalDWalletMismatch)]
    public fun test_sign_with_different_messages() {
        let ctx = &mut tx_context::dummy();

        let messages = vector::empty<vector<u8>>();
        vector::push_back(&mut messages, b"message_3");
        vector::push_back(&mut messages, b"message_4");

        let dwallet_id = object::id_from_address(@0xC);
        let dwallet_cap = create_dwallet_cap(ctx);
        let message_approvals = dwallet::approve_messages(&dwallet_cap, messages);

        let sign_data = create_mock_sign_data(object::id_from_address(@0xA));

        let sign_data_event = create_mock_sign_data_event(object::id_from_address(@0xA));

        let sign_messages = vector::empty<vector<u8>>();
        vector::push_back(&mut sign_messages, b"message_1");
        vector::push_back(&mut sign_messages, b"message_2");

        let partial_user_signed_messages = dwallet::create_partial_user_signed_messages(
            dwallet_id,
            object::id(&dwallet_cap),
            sign_messages,
            vector::empty(),
            sign_data,
            sign_data_event,
            ctx
        );

        dwallet::sign(partial_user_signed_messages, message_approvals, ctx);
        test_utils::destroy(dwallet_cap);
    }

    #[test]
    #[expected_failure(abort_code = EMesssageApprovalDWalletMismatch)]
    public fun test_sign_with_cap_id_missmatch() {
        let ctx = &mut tx_context::dummy();

        let messages = vector::empty<vector<u8>>();
        vector::push_back(&mut messages, b"message_1");

        let dwallet_id = object::id_from_address(@0xC);
        let dwallet_cap = create_dwallet_cap(ctx);
        let message_approvals = dwallet::approve_messages(&dwallet_cap, messages);

        let sign_data = create_mock_sign_data(object::id_from_address(@0xA));

        let sign_data_event = create_mock_sign_data_event(object::id_from_address(@0xA));

        let sign_messages = vector::empty<vector<u8>>();
        vector::push_back(&mut sign_messages, b"message_1");

        let different_dwallet_cap_id = object::id_from_address(@0xD);
        let partial_user_signed_messages = dwallet::create_partial_user_signed_messages(
            dwallet_id,
            different_dwallet_cap_id,
            sign_messages,
            vector::empty(),
            sign_data,
            sign_data_event,
            ctx
        );

        dwallet::sign(partial_user_signed_messages, message_approvals, ctx);
        test_utils::destroy(dwallet_cap);
    }
}
