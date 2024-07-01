module dwallet_system::eth_dwallet {
    use std::string::String;
    use dwallet::object::{Self, UID};
    use dwallet::transfer;
    use dwallet::tx_context::TxContext;
    // use dwallet_system::dwallet;
    // use dwallet_system::dwallet::{DWalletCap, MessageApproval};

    const EInvalidStateProof: u64 = 1;

    // todo(yuval): after merging dwallet.move module, remove DWalletCap implementation from here.
    struct DWalletCap has key, store {
        id: UID,
    }

    /// Holds the DWalletCap.
    struct EthDWalletCap has key {
        id: UID,
        dwallet_cap: DWalletCap,
        eth_smart_contract_addr: String,
        eth_smart_contract_slot: u64,
    }

    /// Create the Eth dWallet Object.
    /// Wrap the dWalletCap.
    public entry fun create_eth_dwallet_cap(
        dwallet_cap: DWalletCap,
        eth_smart_contract_addr: String,
        eth_smart_contract_slot: u64,
        ctx: &mut TxContext
    ) {
        let eth_dwallet_cap = EthDWalletCap {
            id: object::new(ctx),
            dwallet_cap,
            eth_smart_contract_addr,
            eth_smart_contract_slot
        };
        transfer::share_object(eth_dwallet_cap);
    }

    // todo(yuval): after merging dwallet.move module, fix parameters name and uncomment function call.
    /// Approve a message by a dWallet.
    public fun approve_message(
        _eth_dwallet_cap: &EthDWalletCap,
        _message: vector<u8>,
        proof: vector<u8>,
    ) {
        let is_valid = verify_message_proof(proof);
        assert!(is_valid, EInvalidStateProof);
        // dwallet::approve_messages(&eth_dwallet_cap.dwallet_cap, vector[message])
    }

    /// Verify the Message inside the Storage Merkel Root.
    native fun verify_message_proof(
        proof: vector<u8>
    ): bool;
}