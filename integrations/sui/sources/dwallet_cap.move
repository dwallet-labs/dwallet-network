module dwallet_network::dwallet_cap {
    use std::vector;
    use sui::object::{Self, ID, UID};
    use sui::event;
    use sui::tx_context::TxContext;


    struct DWalletCap has key, store {
        id: UID,
        dwallet_network_cap_id: ID,
    }


    struct DWalletNetworkInitCapRequest has copy, drop {
        cap_id: ID,
        dwallet_network_cap_id: ID,
    }


    struct DWalletNetworkApproveRequest has copy, drop {
        cap_id: ID,
        message: vector<u8>,
    }
    

    public fun create_cap(dwallet_network_cap_id: ID, ctx: &mut TxContext): DWalletCap {
        let cap = DWalletCap {
            id: object::new(ctx),
            dwallet_network_cap_id: dwallet_network_cap_id
        };

        event::emit(DWalletNetworkInitCapRequest {
            cap_id: object::id(&cap),
            dwallet_network_cap_id: dwallet_network_cap_id,
        });
    
        cap
    }


    public fun approve_message(cap: &DWalletCap, message: vector<u8>){
        event::emit(DWalletNetworkApproveRequest{
            cap_id: object::id(cap),
            message: message,
        });
    }
}
