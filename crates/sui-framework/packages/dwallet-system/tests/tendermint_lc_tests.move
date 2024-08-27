#[test_only]
module dwallet_system::lc_tests {

    const SENDER: address = @0x012;

    use dwallet::test_scenario;
    // use dwallet::dynamic_field as fields;
    use dwallet::test_utils::{Self};
    use dwallet_system::tendermint_lc::{init_lc, Self};

    fun sample_header(): vector<u8> {
        let header = vector[10, 38, 47, 105, 98, 99, 46, 108, 105, 103, 104, 116, 99, 108, 105, 101, 110, 116, 115, 46, 116, 101, 110, 100, 101, 114, 109, 105, 110, 116, 46, 118, 49, 46, 72, 101, 97, 100, 101, 114, 18, 144, 6, 10, 199, 4, 10, 139, 3, 10, 2, 8, 11, 18, 5, 105, 98, 99, 45, 48, 24, 10, 34, 12, 8, 249, 155, 196, 180, 6, 16, 214, 141, 149, 201, 1, 42, 72, 10, 32, 63, 110, 249, 22, 37, 107, 4, 75, 18, 162, 42, 21, 137, 99, 96, 240, 140, 200, 164, 161, 197, 55, 82, 220, 12, 174, 214, 112, 190, 53, 187, 231, 18, 36, 8, 1, 18, 32, 63, 17, 171, 213, 32, 225, 117, 142, 144, 231, 129, 73, 131, 68, 42, 59, 142, 254, 9, 251, 254, 195, 178, 215, 71, 68, 130, 48, 81, 182, 173, 27, 50, 32, 79, 252, 108, 15, 8, 169, 4, 11, 254, 17, 5, 82, 82, 3, 95, 94, 64, 100, 45, 199, 96, 184, 218, 1, 223, 108, 12, 217, 162, 68, 213, 49, 58, 32, 227, 176, 196, 66, 152, 252, 28, 20, 154, 251, 244, 200, 153, 111, 185, 36, 39, 174, 65, 228, 100, 155, 147, 76, 164, 149, 153, 27, 120, 82, 184, 85, 66, 32, 21, 60, 28, 187, 0, 190, 203, 52, 31, 17, 78, 57, 164, 96, 36, 158, 242, 75, 82, 181, 40, 107, 205, 43, 108, 57, 226, 105, 169, 115, 88, 23, 74, 32, 21, 60, 28, 187, 0, 190, 203, 52, 31, 17, 78, 57, 164, 96, 36, 158, 242, 75, 82, 181, 40, 107, 205, 43, 108, 57, 226, 105, 169, 115, 88, 23, 82, 32, 4, 128, 145, 188, 125, 220, 40, 63, 119, 191, 191, 145, 215, 60, 68, 218, 88, 195, 223, 138, 156, 188, 134, 116, 5, 216, 183, 243, 218, 173, 162, 47, 90, 32, 250, 205, 108, 46, 129, 229, 203, 91, 142, 79, 196, 93, 168, 227, 138, 45, 70, 86, 220, 18, 86, 11, 244, 226, 125, 168, 129, 42, 96, 98, 29, 246, 98, 32, 227, 176, 196, 66, 152, 252, 28, 20, 154, 251, 244, 200, 153, 111, 185, 36, 39, 174, 65, 228, 100, 155, 147, 76, 164, 149, 153, 27, 120, 82, 184, 85, 106, 32, 227, 176, 196, 66, 152, 252, 28, 20, 154, 251, 244, 200, 153, 111, 185, 36, 39, 174, 65, 228, 100, 155, 147, 76, 164, 149, 153, 27, 120, 82, 184, 85, 114, 20, 253, 79, 239, 42, 177, 84, 60, 49, 154, 2, 214, 137, 47, 128, 252, 111, 118, 173, 102, 61, 18, 182, 1, 8, 10, 26, 72, 10, 32, 182, 194, 136, 163, 170, 30, 176, 72, 77, 144, 162, 93, 116, 0, 128, 219, 23, 65, 152, 46, 8, 233, 167, 190, 246, 111, 27, 253, 156, 21, 122, 191, 18, 36, 8, 1, 18, 32, 131, 179, 33, 203, 78, 37, 147, 21, 113, 137, 157, 31, 230, 54, 116, 49, 161, 8, 119, 182, 20, 245, 145, 205, 203, 204, 2, 17, 204, 161, 227, 25, 34, 104, 8, 2, 18, 20, 253, 79, 239, 42, 177, 84, 60, 49, 154, 2, 214, 137, 47, 128, 252, 111, 118, 173, 102, 61, 26, 12, 8, 254, 155, 196, 180, 6, 16, 188, 190, 166, 206, 1, 34, 64, 254, 26, 184, 18, 153, 193, 115, 190, 24, 74, 69, 20, 214, 195, 117, 2, 7, 146, 60, 145, 211, 81, 87, 230, 171, 194, 245, 36, 55, 104, 3, 43, 12, 158, 169, 216, 144, 75, 143, 171, 98, 168, 177, 93, 1, 121, 168, 26, 5, 190, 88, 238, 31, 39, 252, 145, 30, 218, 179, 46, 40, 167, 198, 12, 18, 126, 10, 60, 10, 20, 253, 79, 239, 42, 177, 84, 60, 49, 154, 2, 214, 137, 47, 128, 252, 111, 118, 173, 102, 61, 18, 34, 10, 32, 100, 86, 35, 36, 181, 186, 222, 9, 123, 142, 243, 70, 224, 128, 72, 121, 143, 202, 62, 242, 101, 37, 230, 199, 28, 130, 172, 74, 226, 98, 211, 65, 24, 1, 18, 60, 10, 20, 253, 79, 239, 42, 177, 84, 60, 49, 154, 2, 214, 137, 47, 128, 252, 111, 118, 173, 102, 61, 18, 34, 10, 32, 100, 86, 35, 36, 181, 186, 222, 9, 123, 142, 243, 70, 224, 128, 72, 121, 143, 202, 62, 242, 101, 37, 230, 199, 28, 130, 172, 74, 226, 98, 211, 65, 24, 1, 24, 1, 26, 2, 16, 6, 34, 64, 10, 60, 10, 20, 253, 79, 239, 42, 177, 84, 60, 49, 154, 2, 214, 137, 47, 128, 252, 111, 118, 173, 102, 61, 18, 34, 10, 32, 100, 86, 35, 36, 181, 186, 222, 9, 123, 142, 243, 70, 224, 128, 72, 121, 143, 202, 62, 242, 101, 37, 230, 199, 28, 130, 172, 74, 226, 98, 211, 65, 24, 1, 24, 1];
        return header
    }

    fun set_up(): (u64, vector<u8>, vector<u8>, vector<u8>) { 
        // data from local chain. please check https://github.com/gonative-cc/tendermint-lightclient for more details
        let height = 6;
        let timestamp: vector<u8> = vector[50, 48, 50, 52, 45, 48, 55, 45, 49, 50, 84, 49, 49, 58, 48, 53, 58, 48, 57, 46, 51, 54, 54, 56, 52, 57, 49, 48, 52, 90];
        let next_validators_hash : vector<u8> = vector[21, 60, 28, 187, 0, 190, 203, 52, 31, 17, 78, 57, 164, 96, 36, 158, 242, 75, 82, 181, 40, 107, 205, 43, 108, 57, 226, 105, 169, 115, 88, 23];
        let root: vector<u8> = vector[250, 71, 122, 95, 80, 76, 172, 76, 196, 66, 160, 101, 147, 54, 30, 152, 195, 50, 162, 105, 97, 187, 215, 244, 26, 19, 62, 215, 255, 219, 119, 109];

        (
            height, 
            timestamp, 
            next_validators_hash,
            root
        )
    }

    #[test]
    fun tendermint_state_proof_test() {}
    #[test]
    fun tendermint_init_lc_test() {
        let scenario = test_scenario::begin(SENDER);
        let height = 4;

        let timestamp: vector<u8> = vector[1];
        let next_validators_hash : vector<u8> = vector[2];
        let root: vector<u8> = vector[3];
        let ctx = test_scenario::ctx(&mut scenario);
        let client = init_lc(height, timestamp, next_validators_hash, root, ctx);
        
        assert!(tendermint_lc::latest_height(&client) == height, 0);
        test_scenario::end(scenario);
        test_utils::destroy(client);
    }
    #[test]
    fun tendermint_verify_lc_test() {
        let scenario = test_scenario::begin(SENDER);

        let (height, timestamp, next_validators_hash, root) = set_up();
        let ctx = test_scenario::ctx(&mut scenario);
        let client = init_lc(height, timestamp, next_validators_hash, root, ctx);
        assert!(tendermint_lc::latest_height(&client) == height, 0);
        let header = sample_header();
        let ans = tendermint_lc::verify_lc(&client, header);

        assert!(ans == true, 0);
        test_scenario::end(scenario);
        test_utils::destroy(client);
    }

    #[test] 
    fun extract_consensus_state_test() {
        let header = sample_header();
        let cs = tendermint_lc::extract_consensus_state(header);

        // data from header, we already convert it to bytes.
        let next_validators_hash = vector[21, 60, 28, 187, 0, 190, 203, 52, 31, 17, 78, 57, 164, 96, 36, 158, 242, 75, 82, 181, 40, 107, 205, 43, 108, 57, 226, 105, 169, 115, 88, 23];
        let root = vector[250, 205, 108, 46, 129, 229, 203, 91, 142, 79, 196, 93, 168, 227, 138, 45, 70, 86, 220, 18, 86, 11, 244, 226, 125, 168, 129, 42, 96, 98, 29, 246];
        let timestamp = vector[10, 41, 84, 105, 109, 101, 115, 116, 97, 109, 112, 40, 50, 48, 50, 52, 45, 48, 55, 45, 49, 50, 84, 49, 49, 58, 48, 53, 58, 50, 57, 46, 52, 50, 49, 56, 55, 51, 51, 54, 54, 90, 41];
        
        assert!(tendermint_lc::height(&cs) == 10, 0);
        assert!(tendermint_lc::commitment_root(&cs) == root, 0);
        assert!(tendermint_lc::timestamp(&cs) == timestamp, 0);
        assert!(tendermint_lc::next_validators_hash(&cs) == next_validators_hash, 0);
    }
    #[test]
    fun tendermint_update_lc_test() {
        let scenario = test_scenario::begin(SENDER);
        let (height, timestamp, next_validators_hash, root) = set_up();

        let ctx = test_scenario::ctx(&mut scenario);
        let client = init_lc(height, timestamp, next_validators_hash, root, ctx);
        
        assert!(tendermint_lc::latest_height(&client) == height, 0);
        let header = sample_header();

        // shouldn't return error
        tendermint_lc::update_lc(&mut client, header);
        test_scenario::end(scenario);
        test_utils::destroy(client);
    }
}