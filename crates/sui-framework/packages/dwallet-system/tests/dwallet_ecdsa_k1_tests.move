#[test_only]
module dwallet_system::dwallet_ecdsa_k1_tests {
    use std::vector;

    use dwallet::object::ID;
    use dwallet::test_scenario;
    use dwallet::test_scenario::TransactionEffects;
    use dwallet::test_utils;
    use dwallet::vec_map;

    use dwallet_system::dwallet_2pc_mpc_ecdsa_k1;
    use dwallet_system::dwallet_2pc_mpc_ecdsa_k1::{DKGSession, DKGSessionOutput, EEmptyCommitment, ENotSystemAddress};

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
}
