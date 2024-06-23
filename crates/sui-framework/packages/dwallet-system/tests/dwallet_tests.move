#[test_only]
module dwallet_system::dwallet_tests {
    use dwallet::object::{Self};
    use dwallet_system::dwallet;
    use dwallet::tx_context;
    use std::vector;
    use dwallet_system::dwallet::EMesssageApprovalDWalletMismatch;
    use dwallet_system::dwallet_2pc_mpc_ecdsa_k1::{create_mock_sign_data_event, create_mock_sign_data};


    #[test]
    public fun test_approve_messages_with_matching_sign_message_happy_path() {
        let ctx = &mut tx_context::dummy();

        let messages = vector::empty<vector<u8>>();
        vector::push_back(&mut messages, b"message_1");
        vector::push_back(&mut messages, b"message_1");

        let dwallet_id = object::id_from_address(@0xC);
        let dwallet_cap_id = object::id_from_address(@0xB);
        let message_approvals = dwallet::mock_approve_messages(dwallet_cap_id, messages);

        let sign_data = create_mock_sign_data(object::id_from_address(@0xA));

        let sign_data_event = create_mock_sign_data_event(object::id_from_address(@0xA));

        let sign_messages = vector::empty<vector<u8>>();
        vector::push_back(&mut sign_messages,b"message_1");
        vector::push_back(&mut sign_messages,b"message_1");

        let partial_user_signed_messages = dwallet::create_partial_user_signed_messages(
            dwallet_id,
            dwallet_cap_id,
            sign_messages,
            sign_data,
            sign_data_event,
            ctx
        );

        dwallet::sign(partial_user_signed_messages, message_approvals, ctx)
    }

    #[test]
    #[expected_failure(abort_code = EMesssageApprovalDWalletMismatch)]
    public fun test_approve_messages_with_vector_length_missmatch_should_fail() {
        let ctx = &mut tx_context::dummy();

        let messages = vector::empty<vector<u8>>();
        vector::push_back(&mut messages, b"message_1");

        let dwallet_id = object::id_from_address(@0xC);
        let dwallet_cap_id = object::id_from_address(@0xB);
        let message_approvals = dwallet::mock_approve_messages(dwallet_cap_id, messages);

        let sign_data = create_mock_sign_data(object::id_from_address(@0xA));

        let sign_data_event = create_mock_sign_data_event(object::id_from_address(@0xA));

        let sign_messages = vector::empty<vector<u8>>();
        vector::push_back(&mut sign_messages, b"message_1");
        vector::push_back(&mut sign_messages, b"message_2");

        let partial_user_signed_messages = dwallet::create_partial_user_signed_messages(
            dwallet_id,
            dwallet_cap_id,
            sign_messages,
            sign_data,
            sign_data_event,
            ctx
        );

        dwallet::sign(partial_user_signed_messages, message_approvals, ctx)
    }

    #[test]
    #[expected_failure(abort_code = EMesssageApprovalDWalletMismatch)]
    public fun test_approve_messages_with_different_messages_order_should_fail() {
        let ctx = &mut tx_context::dummy();

        let messages = vector::empty<vector<u8>>();
        vector::push_back(&mut messages, b"message_2");
        vector::push_back(&mut messages, b"message_1");

        let dwallet_id = object::id_from_address(@0xC);
        let dwallet_cap_id = object::id_from_address(@0xB);
        let message_approvals = dwallet::mock_approve_messages(dwallet_cap_id, messages);

        let sign_data = create_mock_sign_data(object::id_from_address(@0xA));

        let sign_data_event = create_mock_sign_data_event(object::id_from_address(@0xA));

        let sign_messages = vector::empty<vector<u8>>();
        vector::push_back(&mut sign_messages, b"message_1");
        vector::push_back(&mut sign_messages, b"message_2");

        let partial_user_signed_messages = dwallet::create_partial_user_signed_messages(
            dwallet_id,
            dwallet_cap_id,
            sign_messages,
            sign_data,
            sign_data_event,
            ctx
        );

        dwallet::sign(partial_user_signed_messages, message_approvals, ctx)
    }

    #[test]
    #[expected_failure(abort_code = EMesssageApprovalDWalletMismatch)]
    public fun test_approve_messages_with_cap_id_missmatch_should_fail() {
        let ctx = &mut tx_context::dummy();

        let messages = vector::empty<vector<u8>>();
        vector::push_back(&mut messages, b"message_1");

        let dwallet_id = object::id_from_address(@0xC);
        let dwallet_cap_id = object::id_from_address(@0xB);
        let message_approvals = dwallet::mock_approve_messages(dwallet_cap_id, messages);

        let sign_data = create_mock_sign_data(object::id_from_address(@0xA));

        let sign_data_event = create_mock_sign_data_event(object::id_from_address(@0xA));

        let sign_messages = vector::empty<vector<u8>>();
        vector::push_back(&mut sign_messages, b"message_1");

        let different_dwallet_cap_id = object::id_from_address(@0xD);
        let partial_user_signed_messages = dwallet::create_partial_user_signed_messages(
            dwallet_id,
            different_dwallet_cap_id,
            sign_messages,
            sign_data,
            sign_data_event,
            ctx
        );

        dwallet::sign(partial_user_signed_messages, message_approvals, ctx)
    }
}
