module sui_state_proof::sui_state_proof {
    use std::vector;
    use sui::object::{Self, ID, UID};
    use sui::event;
    use sui::tx_context::TxContext;


    struct DWalletCap has key, store {
        id: UID,
    }

    struct DWalletNetworkRequest has copy, drop {
        cap_id: ID,
        message: vector<u8>,
    }

    public fun create_cap(ctx: &mut TxContext): DWalletCap {
        let cap = DWalletCap {
            id: object::new(ctx),
        };

        event::emit(DWalletNetworkRequest {
            cap_id: object::id(&cap),
            message: vector::empty(),
        });
    
        cap
    }


    public fun approve_message(cap: &DWalletCap, message: vector<u8>){
        event::emit(DWalletNetworkRequest{
            cap_id: object::id(cap),
            message: message,
        });
    }
}
