/*
 * eth_dwallet module provides functionality for managing Ethereum dWallets within the dwallet_system.
 * It includes structures to represent dWallet capabilities, functions to create and manage Ethereum dWallets,
 * and native functions to verify message proofs.
*/

module dwallet_system::eth_dwallet {
    use dwallet::object::{Self, UID};
    use dwallet::transfer;
    use dwallet::tx_context::TxContext;
    use dwallet_system::dwallet;
    use dwallet_system::dwallet::{DWalletCap, MessageApproval};

    const EInvalidStateProof: u64 = 1;

    /// Holds the DWalletCap along with Ethereum-specific information.
    struct EthereumDWalletCap has key {
        id: UID,
        dwallet_cap: DWalletCap,
    }

    /// Creates an Ethereum dWallet capability object by wrapping an existing DWalletCap.
    public entry fun create_eth_dwallet_cap(
        dwallet_cap: DWalletCap,
        ctx: &mut TxContext
    ) {
        let eth_dwallet_cap = EthereumDWalletCap {
            id: object::new(ctx),
            dwallet_cap,
        };
        transfer::share_object(eth_dwallet_cap);
    }

    /// Verifies the provided proof using the `verify_message_proof` native function.
    /// If the proof is valid, the message is approved.
    public fun approve_message(
        eth_dwallet_cap: &EthereumDWalletCap,
        message: vector<u8>,
        dwallet_id: vector<u8>,
        data_slot: u64,
        proof: vector<u8>,
    ): vector<MessageApproval> {
        let is_valid = verify_message_proof(proof, message, dwallet_id, data_slot);
        assert!(is_valid, EInvalidStateProof);
        dwallet::approve_messages(&eth_dwallet_cap.dwallet_cap, vector[message])
    }

    /// Verify the Message inside the Storage Merkle Root.
    native fun verify_message_proof(
        proof: vector<u8>,
        message: vector<u8>,
        dwallet_id: vector<u8>,
        data_slot: u64,
    ): bool;
}
