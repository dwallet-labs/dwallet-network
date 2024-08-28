#[allow(unused_function, unused_field)]
module dwallet_system::tendermint_lc {
  
    use dwallet::object::{UID, Self};
    use dwallet::tx_context::TxContext;
    use dwallet::dynamic_field as field;

    const EUpdateFailed: u64 = 0;

    struct Client has key, store {
        id: UID,
        latest_height: u64
    }

    // struct Height has store, copy, drop{
    //     height: u64, 
    //     revision_height: u64
    // }

    struct ConsensusState has store, copy, drop {
        height: u64,
        timestamp: vector<u8>, 
        next_validators_hash: vector<u8>,
        commitment_root: vector<u8>
    }

    #[test_only]
    public fun height(cs: &ConsensusState) : u64 {
        cs.height
    }
    
    #[test_only]
    public fun timestamp(cs: &ConsensusState) : vector<u8> {
        cs.timestamp
    }

    #[test_only]
    public fun next_validators_hash(cs: &ConsensusState) : vector<u8> {
        cs.next_validators_hash
    }

    #[test_only]
    public fun commitment_root(cs: &ConsensusState) : vector<u8>{
        cs.commitment_root
    }
    
    public fun consensus_state(height: u64, timestamp: vector<u8>, next_validators_hash: vector<u8>, commitment_root: vector<u8>): ConsensusState {
       let consensus_state =  ConsensusState {
            timestamp, 
            next_validators_hash, 
            commitment_root,
            height
        };  
        consensus_state
    }

    public fun latest_height(client: &Client) : u64 {
        client.latest_height
    }

    public fun init_lc(height: u64, timestamp: vector<u8>, next_validators_hash: vector<u8>, commitment_root: vector<u8>, ctx: &mut TxContext): Client {
        let client = Client {
            id: object::new(ctx),
            latest_height: height
        };

        let cs = consensus_state(height, timestamp, next_validators_hash, commitment_root);
        field::add(&mut client.id, height, cs);
        // public object anyone call call client
        // transfer::share_object(client);
        client
    }

    
    public fun verify_lc(client: &Client, header: vector<u8>): bool{
        let latest_height = client.latest_height;
        // TODO: use trusted height from header.  
        let consensus_state: &ConsensusState = field::borrow(&client.id, latest_height);
        let timestamp = consensus_state.timestamp;
        let next_validators_hash = consensus_state.next_validators_hash;
        let commitment_root = consensus_state.commitment_root;

        tendermint_verify_lc(timestamp, next_validators_hash, commitment_root , header)
    }

    public fun update_lc(client: &mut Client, header: vector<u8>) {
        if (verify_lc(client, header)) {
            let consensus_state = extract_consensus_state(header);
            let height = consensus_state.height;
            if (height > client.latest_height) {
                client.latest_height = height;
            };
            field::add(&mut client.id, height, consensus_state);
        } else {
            abort EUpdateFailed
        }
    }
    
    public native fun extract_consensus_state(header:vector<u8>): ConsensusState;
    native fun tendermint_verify_lc(timestamp: vector<u8>, next_validators_hash: vector<u8>, commitment_root: vector<u8>, header: vector<u8>): bool; 
    public native fun tendermint_state_proof(proof: vector<u8>, root: vector<u8>, prefix: vector<u8>, path: vector<u8>, value: vector<u8>): bool; 
}