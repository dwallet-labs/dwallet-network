#[test_only]
module dwallet_system::dwallet_tests {
    use std::vector;

    use dwallet::object::Self;
    use dwallet::table;
    use dwallet::test_scenario;
    use dwallet::test_scenario::TransactionEffects;
    use dwallet::test_utils;
    use dwallet::tx_context;

    use dwallet_system::dwallet;
    use dwallet_system::dwallet::{
        create_dwallet_cap,
        EMesssageApprovalDWalletMismatch,
        EInvalidEncryptionKeyScheme,
        EInvalidEncryptionKeyOwner,
        create_mock_active_encryption_keys,
        create_mock_encryption_key,
        get_active_encryption_key,
        get_active_encryption_keys_table
    };
    use dwallet_system::dwallet_2pc_mpc_ecdsa_k1::{create_mock_sign_data, create_mock_sign_data_event};

    fun set_up(): (address, test_scenario::Scenario) {
        let sender = @0x1;
        let scenario = test_scenario::begin(sender);
        (sender, scenario)
    }

    fun set_up_with_sender_address(sender: address): test_scenario::Scenario {
        let scenario = test_scenario::begin(sender);
        scenario
    }


    const VALID_SCHEME: u8 = 0;
    const INVALID_SCHEME: u8 = 100;


    // <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<
    const EWrongEventNumber: u64 = 0;
    const EWrongFrozenObjectsNum: u64 = 1;
    const EWrongCreatedObjectsNum: u64 = 2;
    const EObjectMismatchCreateAndFrozen: u64 = 3;
    const EObjectNotInTable: u64 = 4;
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

    #[test]
    public fun test_register_encryption_key_with_valid_input() {
        let scenario = set_up_with_sender_address(@0x92c28a0905643d2b861c12b3dd2aba20619b9748f3e5cb6165f9a4388c515668);
        {
            let mock_encryption_key = vector[1];
            let signed_encryption_key = vector[97, 159, 229, 209, 91, 100, 9, 202, 132, 225, 75, 24, 235, 53, 144, 103, 246, 193, 91, 93, 95, 160, 25, 74, 199, 26, 159, 199, 4, 208, 6, 67, 3, 20, 32, 35, 112, 114, 62, 134, 112, 246, 126, 69, 54, 34, 249, 141, 194, 115, 4, 38, 189, 110, 141, 174, 224, 174, 87, 194, 125, 211, 67, 4];
            let ctx = test_scenario::ctx(&mut scenario);
            let mock_sui_pubkey = vector[204, 188, 31, 23, 159, 78, 46, 145, 247, 191, 82, 249, 88, 130, 89, 6, 254, 235, 251, 29, 151, 11, 249, 229, 128, 137, 15, 255, 24, 22, 102, 25];
            dwallet::register_encryption_key(mock_encryption_key, signed_encryption_key, mock_sui_pubkey, VALID_SCHEME, ctx);
        };
        let effects: TransactionEffects = test_scenario::end(scenario);
        let created_objects = test_scenario::created(&effects);
        assert!(vector::length(&created_objects) == 1, EWrongCreatedObjectsNum);

        let frozen_objects = test_scenario::frozen(&effects);
        assert!(vector::length(&frozen_objects) == 1, EWrongFrozenObjectsNum);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidEncryptionKeyScheme)]
    public fun test_register_active_encryption_key_with_invalid_scheme() {
        let (sender, scenario) = set_up();
        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);
            let key = vector::empty<u8>();
            dwallet::register_encryption_key(key, vector::empty(), vector::empty(), INVALID_SCHEME, ctx);
        };
        let effects: TransactionEffects = test_scenario::end(scenario);
        let created_objects = test_scenario::created(&effects);
        assert!(vector::length(&created_objects) == 0, EWrongCreatedObjectsNum);

        let frozen_objects = test_scenario::frozen(&effects);
        assert!(vector::length(&frozen_objects) == 0, EWrongFrozenObjectsNum);
    }

    #[test]
    public fun test_create_active_encryption_keys() {
        let (sender, scenario) = set_up();
        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);
            dwallet::create_active_encryption_keys(ctx);
        };
        let effects: TransactionEffects = test_scenario::end(scenario);
        let created_objects = test_scenario::created(&effects);
        assert!(vector::length(&created_objects) == 1, EWrongCreatedObjectsNum);

        let shared_objects = test_scenario::shared(&effects);
        assert!(vector::length(&shared_objects) == 1, EWrongFrozenObjectsNum);
    }

    #[test]
    public fun test_set_active_encryption_key() {
        let (sender, scenario) = set_up();
        let _ = test_scenario::next_tx(&mut scenario, sender);
        let ctx = test_scenario::ctx(&mut scenario);
        let active_encryption_keys = create_mock_active_encryption_keys(ctx);
        let key = vector::empty<u8>();

        let first_encryption_key = create_mock_encryption_key(key, VALID_SCHEME, tx_context::sender(ctx), ctx);
        dwallet::set_active_encryption_key(&mut active_encryption_keys, &first_encryption_key, ctx);
        assert!(
            table::contains(get_active_encryption_keys_table(&active_encryption_keys), tx_context::sender(ctx)),
            EObjectNotInTable
        );
        assert!(
            get_active_encryption_key(&active_encryption_keys, tx_context::sender(ctx)) == &object::id(
                &first_encryption_key
            ),
            EObjectNotInTable
        );

        let second_encryption_key = create_mock_encryption_key(key, VALID_SCHEME, tx_context::sender(ctx), ctx);
        dwallet::set_active_encryption_key(&mut active_encryption_keys, &second_encryption_key, ctx);
        assert!(
            table::contains(get_active_encryption_keys_table(&active_encryption_keys), tx_context::sender(ctx)),
            EObjectNotInTable
        );
        assert!(
            get_active_encryption_key(&active_encryption_keys, tx_context::sender(ctx)) == &object::id(
                &second_encryption_key
            ),
            EObjectNotInTable
        );

        test_utils::destroy(active_encryption_keys);
        test_utils::destroy(first_encryption_key);
        test_utils::destroy(second_encryption_key);

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidEncryptionKeyOwner)]
    public fun test_set_active_encryption_key_with_invalid_key_owner() {
        let (sender, scenario) = set_up();
        let _ = test_scenario::next_tx(&mut scenario, sender);
        let ctx = test_scenario::ctx(&mut scenario);
        let active_encryption_keys = create_mock_active_encryption_keys(ctx);
        let key = vector::empty<u8>();
        let invalid_sender = @0xD;

        let encryption_key = create_mock_encryption_key(key, VALID_SCHEME, invalid_sender, ctx);
        dwallet::set_active_encryption_key(&mut active_encryption_keys, &encryption_key, ctx);
        assert!(
            !table::contains(get_active_encryption_keys_table(&active_encryption_keys), tx_context::sender(ctx)),
            EWrongCreatedObjectsNum
        );

        test_utils::destroy(active_encryption_keys);
        test_utils::destroy(encryption_key);

        test_scenario::end(scenario);
    }

    #[test]
    public fun test_get_active_encryption_key() {
        let (sender, scenario) = set_up();
        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);
            let key = vector::empty<u8>();
            let active_encryption_keys = create_mock_active_encryption_keys(ctx);
            let encryption_key = create_mock_encryption_key(key, VALID_SCHEME, tx_context::sender(ctx), ctx);
            dwallet::set_active_encryption_key(&mut active_encryption_keys, &encryption_key, ctx);
            let key_id = dwallet::get_active_encryption_key(&active_encryption_keys, tx_context::sender(ctx));
            assert!(key_id == &object::id(&encryption_key), EObjectNotInTable);

            test_utils::destroy(active_encryption_keys);
            test_utils::destroy(encryption_key);
        };
        test_scenario::end(scenario);
    }
}
