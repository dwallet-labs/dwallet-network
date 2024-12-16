module dwallet_network::dwallet {
    use std::vector;
    use sui::object::{Self, ID, UID};
    use sui::event;
    use sui::tx_context::TxContext;
    use std::hash::sha2_256;
    use sui::ecdsa_k1;

    const EMessageMismatch: u64 = 0;
    const EMessageLenMismatch: u64 = 1;
    const EInvalidSignature: u64 = 2;

    const SHA256_HASH: u8 = 1;

    struct DWalletCap has key, store {
        id: UID,
        dwallet_network_cap_id: ID,
    }

    /// Event for binding a dWallet Network cap to Sui.
    struct DWalletNetworkInitCapRequest has copy, drop {
        cap_id: ID,
        dwallet_network_cap_id: ID,
    }

    /// Event for approving a message.
    struct DWalletNetworkApproveRequest has copy, drop {
        cap_id: ID,
        messages: vector<vector<u8>>,
    }
    

    public fun bind_dwallet_cap_to_sui(
        binder_id: ID,
        dwallet_network_cap_id: ID,
        bind_to_authority_id: ID,
        nonce: u64,
        virgin_bound: bool,
        message: vector<u8>,
        signature: vector<u8>,
        pk: vector<u8>,
        ctx: &mut TxContext
    ): DWalletCap {
        let info_as_vec = vector::empty();
        vector::append(
            &mut info_as_vec,
            object::id_to_bytes(&binder_id)
        );
        vector::append(
            &mut info_as_vec,
            object::id_to_bytes(&dwallet_network_cap_id)
        );
        vector::append(
            &mut info_as_vec,
            object::id_to_bytes(&bind_to_authority_id)
        );
        vector::append(
            &mut info_as_vec,
            std::bcs::to_bytes(&nonce)
        );
        vector::append(
            &mut info_as_vec,
            std::bcs::to_bytes(&virgin_bound)
        );

        let constructed_message = sha2_256(info_as_vec);
        let constructed_message_bcs = std::bcs::to_bytes(&constructed_message);
        let constructed_message_len: u64 = vector::length(&constructed_message_bcs);

        let message_len: u64 = vector::length(&message);
        assert!(constructed_message_len == message_len, EMessageLenMismatch);
        assert!(constructed_message_bcs == message, EMessageMismatch);

        let recovered = ecdsa_k1::secp256k1_ecrecover(&signature, &message, SHA256_HASH);
        assert!(recovered == pk, EInvalidSignature);

        let cap = DWalletCap {
            id: object::new(ctx),
            dwallet_network_cap_id: dwallet_network_cap_id
        };

        event::emit(
            DWalletNetworkInitCapRequest {
                cap_id: object::id(&cap),
                dwallet_network_cap_id: dwallet_network_cap_id,
            }
        );
    
        cap
    }


    public fun approve_message(cap: &DWalletCap, messages: vector<vector<u8>>) {
        event::emit(
            DWalletNetworkApproveRequest {
                cap_id: object::id(cap),
                messages: messages,
            }
        );
    }
}
