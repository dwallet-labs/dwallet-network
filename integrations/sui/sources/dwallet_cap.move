module dwallet_network::dwallet_cap {
    use std::vector;
    use sui::object::{Self, ID, UID};
    use sui::event;
    use sui::tx_context::TxContext;
    use std::hash::sha2_256;
    use sui::ecdsa_r1;

    const EHashMismatch: u64 = 0;
    const EInvalidSignature: u64 = 1;

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

    public fun create_cap(
        dwallet_network_cap_id: ID,
        ctx: &mut TxContext
    ): DWalletCap {
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

    public fun approve_message(cap: &DWalletCap, message: vector<u8>) {
        event::emit(
            DWalletNetworkApproveRequest {
                cap_id: object::id(cap),
                message: message,
            }
        );
    }

    public fun bind_dwallet_to_authority(
        binder_id: ID,
        dwallet_cap_id: ID,
        bind_to_authority_id: ID,
        nonce: u64,
        virgin_bound: bool,
        hash: vector<u8>,
        signature: vector<u8>,
        pk: vector<u8>,
    ) {
        let info_as_vec = vector::empty();
        vector::append(
            &mut info_as_vec,
            object::id_to_bytes(&binder_id)
        );
        vector::append(
            &mut info_as_vec,
            object::id_to_bytes(&dwallet_cap_id)
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

        let constructed_hash = sha2_256(info_as_vec);

        let constructed_hash_len: u64 = vector::length(&constructed_hash);
        let hash_len: u64 = vector::length(&hash);
        assert!(constructed_hash_len == hash_len, EHashMismatch);

        // compare constructed hash with the hash
        let i: u64 = 0;
        while (i < constructed_hash_len) {
            let constructed_byte = vector::pop_back(&mut constructed_hash);
            let byte = vector::pop_back(&mut hash);
            assert!(byte == constructed_byte, EHashMismatch);
            i = i + 1;
        };

        // The last param 1 represents the hash function used is SHA256, the default hash function used when signing in CLI.
        let verify = ecdsa_r1::secp256r1_verify(&signature, &pk, &hash, 1);
        assert!(verify == true, EInvalidSignature);
    }
}
