// module support api for link and trigger future transaction
#[allow(unused_function, unused_field, unused_variable, unused_use)]
module dwallet_system::native_dwallet {
    use std::vector;
    use dwallet::object::{UID, Self, ID};
    use dwallet::tx_context::TxContext;
    use dwallet_system::dwallet::{Self, DWalletCap, MessageApproval};
    use dwallet_system::tendermint_lc::{Client, tendermint_state_proof, get_consensus_state, commitment_root, latest_height, state_proof, client_id};

    use dwallet::dynamic_field as field;
    use dwallet::dynamic_object_field as ofields;

    const EHeightInvalid: u64 = 0;
    const EStateInvalid: u64 = 1;
    const EClientInvalid: u64 = 2; 
    const EStateProofNoMessagesToApprove: u64 = 3;

    // Wrapper object wrap DWalletCap. DWalletCap owner (user) will transfer ownership to NativeDwallet Cap
    struct NativeDwalletCap has key, store {
        id: UID,
	// ID of client object. We use it to ensure NativeDwalletCap query
	// right client when verify future transaction.
        client_id: ID,
        dwallet_cap: DWalletCap
    }

    // Create Native Dwallet wrap dwallet and Native light client
    fun create_native_dwallet_cap(client: &Client, dwallet_cap: DWalletCap, ctx: &mut TxContext): NativeDwalletCap {
        let native_dwallet_cap = NativeDwalletCap {
            id: object::new(ctx),
            dwallet_cap,
            client_id: client_id(client)
        };
        native_dwallet_cap
    }

    public fun link_dwallet(client: &Client, dwallet_cap: DWalletCap,  height: u64,  proof: vector<u8>, prefix: vector<u8>, path: vector<u8>, value: vector<u8>, ctx: &mut TxContext): NativeDwalletCap {
        // prefix and path should be a const
        let lh = latest_height(client);
        assert!(height <= lh, EHeightInvalid);
        let valid = state_proof(client, height, proof, prefix, path, value);
        assert!(valid, EStateInvalid);
        return create_native_dwallet_cap(client, dwallet_cap, ctx)
    }


    // verify user sign `messages` data on Native network
    public fun verify_native_transaction(native_dwallet_cap: &NativeDwalletCap, client: &Client, height: u64,  proof: vector<u8>, prefix: vector<u8>, path: vector<u8>, messages: vector<vector<u8>>): vector<MessageApproval> {
    	assert!(object::id(client) == native_dwallet_cap.client_id, EClientInvalid);
    	assert!(vector::length(&messages) > 0, EStateProofNoMessagesToApprove);

	    //  check height
    	let lh = latest_height(client);
        assert!(height <= lh, EHeightInvalid);

	    // check state proof
	    let message: vector<u8> = *vector::borrow(&messages, 0);
	    let valid = state_proof(client, height, proof, prefix, path, message);
       assert!(valid, EStateInvalid);
	
	    // submit message
        dwallet::approve_messages(&native_dwallet_cap.dwallet_cap, messages)
    }
}
