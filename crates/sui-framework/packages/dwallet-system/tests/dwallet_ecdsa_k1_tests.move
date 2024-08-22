#[test_only]
#[allow(unused_use)]
module dwallet_system::dwallet_ecdsa_k1_tests {
    use std::vector;
    use dwallet::object;

    use dwallet::object::ID;
    use dwallet::test_scenario;
    use dwallet::test_scenario::TransactionEffects;
    use dwallet::test_utils;
    use dwallet::vec_map;
    use dwallet_system::dwallet::{create_mock_sign_session, SignSession, MaliciousAggregatorSignOutput, SignOutput};

    use dwallet_system::dwallet_2pc_mpc_ecdsa_k1;
    use dwallet_system::dwallet_2pc_mpc_ecdsa_k1::{DKGSession, DKGSessionOutput, EEmptyCommitment, ENotSystemAddress,
        verify_and_create_sign_output_for_testing, create_mock_sign_data, SignData
    };

    // <<<<<<<<<<<<<<<<<<<<<<<< Constants <<<<<<<<<<<<<<<<<<<<<<<<
    const SENDER_ADDRESS: address = @0xA;
    const SYSTEM_ADDRESS: address = @0x0;

    // <<<<<<<<<<<<<<<<<<<<<<<< Constants <<<<<<<<<<<<<<<<<<<<<<<<

    // <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<
    const EWrongEventNumber: u64 = 0;
    const EWrongCreatedObjectsNum: u64 = 1;
    const EWrongFrozenObjectsNum: u64 = 2;
    const EWrongTransferredObjectsNum: u64 = 3;
    const EObjectTransferredToWrongAddress: u64 = 4;
    const EWrongTransferredObject: u64 = 5;
    const EWrongCreatedObject: u64 = 5;

    // <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<

    fun set_up(): (address, test_scenario::Scenario) {
        let sender = SENDER_ADDRESS;
        let scenario = test_scenario::begin(sender);
        (sender, scenario)
    }

    #[test]
    public fun test_create_dkg_session() {
        let (sender, scenario) = set_up();
        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);
            let commitment_to_centralized_party_secret_key_share = b"testing";
            let dwallet_cap = dwallet_2pc_mpc_ecdsa_k1::create_dkg_session(
                commitment_to_centralized_party_secret_key_share,
                ctx
            );

            test_utils::destroy(dwallet_cap);
        };

        let effects: TransactionEffects = test_scenario::end(scenario);

        let events_num = test_scenario::num_user_events(&effects);
        assert!(events_num == 1, EWrongEventNumber);

        let created_objects = test_scenario::created(&effects);
        assert!(vector::length(&created_objects) == 1, EWrongCreatedObjectsNum);

        let frozen_objects = test_scenario::frozen(&effects);
        assert!(vector::length(&frozen_objects) == 1, EWrongFrozenObjectsNum);
    }

    #[test]
    #[expected_failure(abort_code = EEmptyCommitment)]
    public fun test_create_dkg_session_empty_commitment() {
        let (sender, scenario) = set_up();
        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);
            let commitment_to_centralized_party_secret_key_share = vector::empty<u8>();
            let dwallet_cap = dwallet_2pc_mpc_ecdsa_k1::create_dkg_session(
                commitment_to_centralized_party_secret_key_share,
                ctx
            );

            test_utils::destroy(dwallet_cap);
        };

        test_scenario::end(scenario);
    }

    #[test]
    public fun test_create_dkg_output() {
        let (sender, scenario) = set_up();
        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);
            let commitment_to_centralized_party_secret_key_share = b"testing";
            let dwallet_cap = dwallet_2pc_mpc_ecdsa_k1::create_dkg_session(
                commitment_to_centralized_party_secret_key_share,
                ctx
            );
            test_utils::destroy(dwallet_cap);
        };

        // Get effects and perform next transaction as SYSTEM_ADDRESS.
        let effects = test_scenario::next_tx(&mut scenario, SYSTEM_ADDRESS);
        let frozen_objects = test_scenario::frozen(&effects);
        assert!(vector::length(&frozen_objects) == 1, EWrongEventNumber);

        {
            let dkg_session = test_scenario::take_immutable<DKGSession>(&scenario);
            let commitment_to_centralized_party_secret_key_share = b"testing";
            let secret_key_share_encryption_and_proof = vector::empty<u8>();
            let ctx = test_scenario::ctx(&mut scenario);

            dwallet_2pc_mpc_ecdsa_k1::create_dkg_output_for_testing(
                &dkg_session,
                commitment_to_centralized_party_secret_key_share,
                secret_key_share_encryption_and_proof,
                ctx
            );
            test_scenario::return_immutable(dkg_session);
        };

        let effects: TransactionEffects = test_scenario::end(scenario);

        let created_objects = test_scenario::created(&effects);
        assert!(vector::length(&created_objects) == 1, EWrongCreatedObjectsNum);

        let sessions_transferred = test_scenario::ids_for_address<DKGSessionOutput>(SENDER_ADDRESS);
        assert!(vector::length(&sessions_transferred) == 1, EWrongTransferredObjectsNum);

        let session_id = vector::borrow(&sessions_transferred, 0);

        let transferred_objects = test_scenario::transferred_to_account(&effects);
        let (id, address) = vec_map::pop<ID, address>(&mut transferred_objects);
        assert!(address == SENDER_ADDRESS, EObjectTransferredToWrongAddress);
        assert!(id == *session_id, EWrongTransferredObject);
    }

    #[test]
    #[expected_failure(abort_code = ENotSystemAddress)]
    public fun test_create_dkg_output_not_system_address() {
        let (sender, scenario) = set_up();
        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);

            let commitment_to_centralized_party_secret_key_share = b"testing";
            let dwallet_cap = dwallet_2pc_mpc_ecdsa_k1::create_dkg_session(
                commitment_to_centralized_party_secret_key_share,
                ctx
            );
            test_utils::destroy(dwallet_cap);
        };

        // Get effects and perform next transaction as SENDER_ADDRESS (not SYSTEM_ADDRESS).
        let effects = test_scenario::next_tx(&mut scenario, SENDER_ADDRESS);
        let frozen_objects = test_scenario::frozen(&effects);
        assert!(vector::length(&frozen_objects) == 1, EWrongEventNumber);

        let dkg_session = test_scenario::take_immutable<DKGSession>(&scenario);

        let commitment_to_centralized_party_secret_key_share = vector::empty<u8>();
        let secret_key_share_encryption_and_proof = vector::empty<u8>();
        let ctx = test_scenario::ctx(&mut scenario);

        dwallet_2pc_mpc_ecdsa_k1::create_dkg_output_for_testing(
            &dkg_session,
            commitment_to_centralized_party_secret_key_share,
            secret_key_share_encryption_and_proof,
            ctx
        );

        test_utils::destroy(dkg_session);
        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = ENotSystemAddress)]
    public fun test_verify_and_create_sign_output_not_system_address() {
        let (sender, scenario) = set_up();
        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);
            let sign_data = create_mock_sign_data(object::id_from_address(@0xA));
            create_mock_sign_session(vector::empty(), vector::empty(), sign_data, ctx);
        };
        test_scenario::next_tx(&mut scenario, SENDER_ADDRESS);
        {
            let sign_session = test_scenario::take_immutable<SignSession<SignData>>(&scenario);
            let ctx = test_scenario::ctx(&mut scenario);
            verify_and_create_sign_output_for_testing(&sign_session, vector::empty(), vector::empty(), ctx);
            test_utils::destroy(sign_session);
        };
        test_scenario::end(scenario);
    }

    #[test]
    public fun test_verify_and_create_sign_output_successful() {
        let (sender, scenario) = set_up();
        test_scenario::next_tx(&mut scenario, sender);
        {
            // A dwallet public key, a message, and a valid signature that been signed by the dwallet private key.
            let messages = vector[vector[83, 105, 103, 110, 32, 105, 116, 33, 33, 33]];
            let dwallet_public_key = vector[3, 254, 2, 228, 100, 194, 223, 2, 105, 140, 230, 159, 139, 227, 58, 68, 131, 176, 138, 177, 8, 245, 230, 127, 210, 9, 143, 227, 255, 126, 131, 90, 236];

            let ctx = test_scenario::ctx(&mut scenario);
            let sign_data = create_mock_sign_data(object::id_from_address(@0xA));
            create_mock_sign_session(messages, dwallet_public_key, sign_data, ctx);
        };
        test_scenario::next_tx(&mut scenario, SYSTEM_ADDRESS);
        {
            let signatures: vector<vector<u8>> = vector[vector[178, 76, 60, 199, 138, 181, 166, 18, 64, 0, 90, 61, 129, 125, 56, 156, 110, 221, 178, 74, 4, 144, 219, 66, 255, 197, 231, 190, 173, 59, 189, 209, 32, 69, 113, 111, 25, 139, 175, 185, 40, 37, 80, 8, 196, 237, 91, 1, 114, 129, 81, 173, 65, 100, 180, 222, 48, 143, 227, 252, 194, 174, 68, 170]];
            let sign_session = test_scenario::take_immutable<SignSession<SignData>>(&scenario);
            let ctx = test_scenario::ctx(&mut scenario);
            verify_and_create_sign_output_for_testing(&sign_session, signatures, vector::empty(), ctx);
            let effects = test_scenario::next_tx(&mut scenario, SYSTEM_ADDRESS);
            assert!(test_scenario::has_most_recent_immutable<SignOutput>(), EWrongCreatedObject);
            let created_objects = test_scenario::created(&effects);
            assert!(vector::length(&created_objects) == 1, EWrongCreatedObjectsNum);
            let events_num = test_scenario::num_user_events(&effects);
            assert!(events_num == 1, EWrongEventNumber);
            test_utils::destroy(sign_session);
        };
        test_scenario::end(scenario);
    }

    #[test]
    public fun test_verify_and_create_sign_output_malicious_path() {
        let (sender, scenario) = set_up();
        test_scenario::next_tx(&mut scenario, sender);
        {
            // A dwallet public key, a message, and an invalid signature that has not been signed by the dwallet private key.
            let messages = vector[vector[83, 105, 103, 110, 32, 105, 116, 33, 33, 33]];
            let dwallet_public_key = vector[5, 254, 2, 228, 100, 194, 223, 2, 105, 140, 230, 159, 139, 227, 58, 68, 131, 176, 138, 177, 8, 245, 230, 127, 210, 9, 143, 227, 255, 126, 131, 90, 236];

            let ctx = test_scenario::ctx(&mut scenario);
            let sign_data = create_mock_sign_data(object::id_from_address(@0xA));
            create_mock_sign_session(messages, dwallet_public_key, sign_data, ctx);
        };
        test_scenario::next_tx(&mut scenario, SYSTEM_ADDRESS);
        {
            let malicious_signatures: vector<vector<u8>> = vector[vector[178, 76, 60, 199, 138, 181, 166, 18, 64, 0, 90, 61, 129, 125, 56, 156, 110, 221, 178, 74, 4, 144, 219, 66, 255, 197, 231, 190, 173, 59, 189, 209, 32, 69, 113, 111, 25, 139, 175, 185, 40, 37, 80, 8, 196, 237, 91, 1, 114, 129, 81, 173, 65, 100, 180, 222, 48, 143, 227, 252, 194, 174, 68, 170]];
            let sign_session = test_scenario::take_immutable<SignSession<SignData>>(&scenario);
            let ctx = test_scenario::ctx(&mut scenario);
            verify_and_create_sign_output_for_testing(&sign_session, malicious_signatures, vector::empty(), ctx);
            let effects = test_scenario::next_tx(&mut scenario, SYSTEM_ADDRESS);
            assert!(test_scenario::has_most_recent_immutable<MaliciousAggregatorSignOutput>(), EWrongCreatedObject);
            let created_objects = test_scenario::created(&effects);
            assert!(vector::length(&created_objects) == 1, EWrongCreatedObjectsNum);
            let events_num = test_scenario::num_user_events(&effects);
            assert!(events_num == 1, EWrongEventNumber);
            test_utils::destroy(sign_session);
        };
        test_scenario::end(scenario);
    }
}
