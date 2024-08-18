/*
 * eth_dwallet module provides functionality for managing Ethereum dWallets within the dwallet_system.
 * It includes structures to represent dWallet capabilities, functions to create and manage Ethereum dWallets,
 * and native functions to verify message proofs.
*/

module dwallet_system::eth_dwallet {
    use dwallet::object::{Self, UID};
    use dwallet::transfer;
    use dwallet::tx_context::TxContext;
    use dwallet_system::ethereum_state;
    use dwallet_system::ethereum_state::{LatestEthereumState, EthereumState};
    use dwallet_system::dwallet;
    use dwallet_system::dwallet::{DWalletCap, MessageApproval};

    const EInvalidStateProof: u64 = 1;
    const ENetworkMismatch: u64 = 2;

    /// Holds the DWalletCap along with Ethereum-specific information.
    struct EthereumDWalletCap has key {
        id: UID,
        dwallet_cap: DWalletCap,
        network: vector<u8>,
    }

    /// Creates an Ethereum dWallet capability object by wrapping an existing DWalletCap.
    public entry fun create_eth_dwallet_cap(
        dwallet_cap: DWalletCap,
        network: vector<u8>,
        ctx: &mut TxContext
    ) {
        let eth_dwallet_cap = EthereumDWalletCap {
            id: object::new(ctx),
            dwallet_cap,
            network,
        };
        transfer::share_object(eth_dwallet_cap);
    }

    /// Verifies the provided proof using the `verify_message_proof` native function.
    /// If the proof is valid, the message is approved.
    public fun approve_message(
        eth_dwallet_cap: &EthereumDWalletCap,
        message: vector<u8>,
        dwallet_id: vector<u8>,
        proof: vector<u8>,
        latest_ethereum_state: &LatestEthereumState,
        current_wrapped_eth_state: &EthereumState,
    ): vector<MessageApproval> {
        // Validate that the Ethereum dWallet capability is for the correct network.
        let state_network = ethereum_state::get_ethereum_state_network(latest_ethereum_state);
        assert!(eth_dwallet_cap.network == state_network, ENetworkMismatch);

        let state_root = ethereum_state::get_ethereum_state_root(current_wrapped_eth_state);
        let contract_address = ethereum_state::get_contract_address(latest_ethereum_state);
        let data_slot = ethereum_state::get_contract_approved_transactions_slot(latest_ethereum_state);

        let is_valid = verify_message_proof(proof, message, dwallet_id, data_slot, state_root, contract_address);
        assert!(is_valid, EInvalidStateProof);
        dwallet::approve_messages(&eth_dwallet_cap.dwallet_cap, vector[message])
    }

    /// Verify the Message inside the Storage Merkle Root.
    native fun verify_message_proof(
        proof: vector<u8>,
        message: vector<u8>,
        dwallet_id: vector<u8>,
        data_slot: u64,
        state_root: vector<u8>,
        contract_address: vector<u8>,
    ): bool;
}
