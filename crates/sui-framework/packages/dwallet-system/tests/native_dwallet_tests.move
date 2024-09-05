#[test_only]
#[allow(unused_function, unused_field, unused_use)]
module dwallet_system::native_dwallet_tests {

    use dwallet_system::tendermint_lc::{Self, Client, init_lc};
    use dwallet::test_scenario;

    use dwallet_system::native_dwallet::{link_dwallet};
    use dwallet_system::dwallet::{create_dwallet_cap, DWalletCap};
   
    use dwallet::test_utils;
    use dwallet_system::lc_tests::{state_proof_test_data};
    
    const SENDER: address = @0x010;


    public fun setup(): (vector<u8>, vector<u8>, vector<u8>, vector<u8>, vector<u8>, Client, test_scenario::Scenario) {

        let scenario = test_scenario::begin(SENDER);
        let next_validators_hash = vector[21, 60, 28, 187, 0, 190, 203, 52, 31, 17, 78, 57, 164, 96, 36, 158, 242, 75, 82, 181, 40, 107, 205, 43, 108, 57, 226, 105, 169, 115, 88, 23];

        let timestamp = vector[10, 41, 84, 105, 109, 101, 115, 116, 97, 109, 112, 40, 50, 48, 50, 52, 45, 48, 55, 45, 49, 50, 84, 49, 49, 58, 48, 53, 58, 50, 57, 46, 52, 50, 49, 56, 55, 51, 51, 54, 54, 90, 41];

        let height: u64 = 10;
        let ctx = test_scenario::ctx(&mut scenario);
	let 	(proof, prefix, path, value, root) = state_proof_test_data();
        let client = init_lc(height, timestamp, next_validators_hash, root, ctx);
        (proof, prefix, path, value, root, client, scenario)
    }
    
    #[test]
    fun link_dwallet_test() {
        let (proof, prefix, path, value, _root, client, scenario) = setup();
        let ctx = test_scenario::ctx(&mut scenario);
        let dwallet_cap = create_dwallet_cap(ctx);
        let height = 10;
	let native_dwallet_cap =  link_dwallet(&client, dwallet_cap, height, proof, prefix, path, value, ctx);
        test_utils::destroy(native_dwallet_cap);
        test_utils::destroy(client);
        test_scenario::end(scenario);
    }
}
