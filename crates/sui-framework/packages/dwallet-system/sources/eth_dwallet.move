//
// `eth_dwallet` module provides functionality for managing Ethereum dWallets within the dwallet_system.
// It includes structures to represent dWallet capabilities, functions to create and manage Ethereum dWallets,
// and native functions to verify message proofs.
//

module dwallet_system::eth_dwallet {
    use dwallet::object::{Self, ID, UID};
    use dwallet::transfer;
    use dwallet::tx_context::TxContext;

    use dwallet_system::dwallet;
    use dwallet_system::dwallet::{get_dwallet_cap_id};
    use dwallet_system::dwallet::{DWallet, DWalletCap, MessageApproval};
    use dwallet_system::dwallet_2pc_mpc_ecdsa_k1::{Secp256K1};
    use dwallet_system::ethereum_state;
    use dwallet_system::ethereum_state::{EthereumState, LatestEthereumState};

    const EInvalidStateProof: u64 = 1;
    const EStateObjectMismatch: u64 = 2;
    const EInvalidDWalletCap: u64 = 3;

    /// Holds the DWalletCap along with Ethereum-specific information.
    struct EthereumDWalletCap has key {
        id: UID,
        dwallet_cap: DWalletCap,
        latest_ethereum_state_id: ID,
    }

    /// Creates an Ethereum dWallet capability object by wrapping an existing DWalletCap.
    public entry fun create_eth_dwallet_cap(
        dwallet_cap: DWalletCap,
        latest_ethereum_state: &LatestEthereumState,
        ctx: &mut TxContext
    ) {
        let eth_dwallet_cap = EthereumDWalletCap {
            id: object::new(ctx),
            dwallet_cap,
            latest_ethereum_state_id: object::id(latest_ethereum_state),
        };
        transfer::share_object(eth_dwallet_cap);
    }

    /// Verifies the provided proof using the `verify_message_proof` native function.
    /// If the proof is valid, the message is approved.
    public fun approve_message(
        eth_dwallet_cap: &EthereumDWalletCap,
        message: vector<u8>,
        dwallet: &DWallet<Secp256K1>,
        latest_ethereum_state: &LatestEthereumState,
        eth_state: &EthereumState,
        proof: vector<u8>,
    ): vector<MessageApproval> {
        let latest_ethereum_state_id = object::id(latest_ethereum_state);

        // Validate that the Ethereum dWallet capability is for the correct network.
        assert!(eth_dwallet_cap.latest_ethereum_state_id == latest_ethereum_state_id, EStateObjectMismatch);

        // Validate that the EthereumState is for the correct network.
        assert!(
            latest_ethereum_state_id == ethereum_state::get_ethereum_state_latest_state_id(eth_state),
            EStateObjectMismatch
        );

        // Validate that the Ethereum dWallet capability is for the correct DWallet.
        let dwallet_cap_id = get_dwallet_cap_id(dwallet);
        assert!(object::id(&eth_dwallet_cap.dwallet_cap) == dwallet_cap_id, EInvalidDWalletCap);

        let eth_state_state_root = ethereum_state::get_ethereum_state_state_root(eth_state);
        let contract_address = ethereum_state::get_contract_address(latest_ethereum_state);
        let data_slot = ethereum_state::get_contract_approved_transactions_slot(latest_ethereum_state);

        let is_valid = verify_message_proof(
            proof,
            message,
            object::id_to_bytes(&object::id(dwallet)),
            data_slot,
            contract_address,
            eth_state_state_root,
        );
        assert!(is_valid, EInvalidStateProof);

        let message_approvals = dwallet::approve_messages(&eth_dwallet_cap.dwallet_cap, vector[message]);
        message_approvals
    }

    /// Verify the Message inside the Storage Merkle Root.
    native fun verify_message_proof(
        proof: vector<u8>,
        message: vector<u8>,
        dwallet_id: vector<u8>,
        data_slot: u64,
        contract_address: vector<u8>,
        eth_state_state_root: vector<u8>,
    ): bool;
}
