module dwallet_system::eth_dwallet {
    use dwallet::object::{Self, UID, ID};
    use dwallet::transfer;
    // use dwallet::tx_context;
    use dwallet::tx_context::TxContext;
    use dwallet_system::dwallet_2pc_mpc_ecdsa_k1::DWallet;

    use dwallet_system::dwallet;
    use dwallet_system::dwallet::{DWalletCap, MessageApproval};

    const EInvalidStateProof: u64 = 1;


    /// Holds the DWalletCap.
    struct EthDWalletCap has key {
        id: UID,
        dwallet_cap: DWalletCap,
        eth_smart_contract_addr: vector<u8>,
        eth_smart_contract_slot: u64,
    }

    struct EthState has key {
        id: UID,
        data: vector<u8>,
    }

    /// Create the Eth dWallet Object.
    /// Wrap the dWalletCap.
    public entry fun create_eth_dwallet_cap(
        dwallet_cap: DWalletCap,
        eth_smart_contract_addr: vector<u8>,
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

    /// Approve a message by a dWallet.
    public fun approve_message(
        eth_dwallet_cap: &EthDWalletCap,
        dwallet: &DWallet,
        message: vector<u8>,
        proof: vector<u8>,
        updates: vector<u8>,
        current_eth_state: vector<u8>,
        // ctx: &mut TxContext
    ): vector<MessageApproval> {
        let dwallet_id = object::id(dwallet);

        // todo(yuval): do we need to provide the shared state object? or we can fetch it here from SUI?

        let is_valid = verify_eth_state(
            proof,
            dwallet_id,
            eth_dwallet_cap.eth_smart_contract_slot,
            message,
            updates,
            current_eth_state
        );
        assert!(is_valid, EInvalidStateProof);
        dwallet::approve_messages(&eth_dwallet_cap.dwallet_cap, vector[message])
    }

    // #[allow(unused_function)]
    // // Create and share the singleton EthState -- this function is
    // // called exactly once, during genesis.
    // public fun create_eth_state() {
    //     // assert!(tx_context::sender(ctx) == @0x0, ENotSystemAddress);
    //     let data = b"{\"last_checkpoint\":\"0xa8ab0b7ab08b63839b668afa6b03beb4b50925bc0f0c65b4ee7b6c35a511b7ca\"}";
    //
    //     transfer::share_object(EthState {
    //         id: object::eth_state_object(),
    //         data,
    //     })
    // }

    public fun update_eth_state(
        self: &mut EthState,
        data: vector<u8>,
    ) {
        self.data = data;
    }

    /// Verify the Message inside the Storage Merkel Root.
    native fun verify_eth_state(
        proof: vector<u8>,
        dwallet_id: ID,
        eth_smart_contract_slot: u64,
        message: vector<u8>,
        updates: vector<u8>,
        current_eth_state: vector<u8>
    ): bool;
}
