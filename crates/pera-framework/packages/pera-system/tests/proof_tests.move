#[test_only]
module pera_system::proof_tests {
    use pera::test_scenario;
    use pera::test_scenario::TransactionEffects;
    use pera_system::proof::launch_proof_mpc_flow;

    // <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<
    const EWrongEventNumber: u64 = 0;
    const EWrongFrozenObjectsNum: u64 = 1;
    const EWrongCreatedObjectsNum: u64 = 2;
    const EObjectMismatchCreateAndFrozen: u64 = 3;
    // <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<

    #[test]
    public fun test_launch_proof_mpc_flow_succesfull() {
        let sender = @0x1;
        let mut scenario = test_scenario::begin(sender);
        test_scenario::next_tx(&mut scenario, sender);
        {
            let ctx = test_scenario::ctx(&mut scenario);
            launch_proof_mpc_flow(ctx);
        };
        let effects: TransactionEffects = test_scenario::end(scenario);
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
}
