// module support api for link and trigger future transaction


module dwallet_system::native_api {
    use std::vector;
    use dwallet::object::{UID, Self, ID};
    use dwallet::tx_context::TxContext;
    use dwallet_system::dwallet::{Self, DWalletCap};
    use dwallet_system::tendermint_lc::StateProof;
    use dwallet_system::tendermint_lc::{Client, tendermint_state_proof, get_consensus_state, commitment_root, latest_height, state_proof, client_id};

    use dwallet::dynamic_field as field;
    use dwallet::dynamic_object_field as ofields;

    const EHeightInvalid: u64 = 0;
    const EStateInvalid: u64 = 1;

    // TODO: How own this object?

    struct NativeDwalletCap has key, store{
        id: UID,
        client_id: ID
    }

    fun create_native_dwallet_cap(client: &Client, dwallet: DWalletCap, ctx: &mut TxContext): NativeDwalletCap {
        let native_dwallet_cap = NativeDwalletCap {
            id: object::new(ctx),
            client_id: client_id(client)
        };

        // Question: Should we transfer dwallet_cap to create native dwallet
        ofields::add(&mut native_dwallet_cap.id, b"dcap", dwallet);
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

    public fun verify_transaction(dwallet_cap: &DWalletCap, client: &Client, height: u64, state_proof: StateProof){}
}
