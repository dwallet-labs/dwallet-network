module dwallet_system::eth_dwallet {
    use dwallet::object::{Self, UID, ID};
    use dwallet::transfer;
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
        slot: u64,
    }

    struct CurrentEthState has key {
        id: UID,
        eth_state_id: ID,
        last_slot: u64,
    }

    public fun update_current_eth_state(
        self: &mut CurrentEthState,
        eth_state: &EthState,
    ) {
        if (eth_state.slot > self.last_slot) {
            self.eth_state_id = object::id(eth_state);
            self.last_slot = eth_state.slot;
        }
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

    /// Verify the Eth state according to the updates.
    public fun update_eth_state(
        ctx: &mut TxContext,
        self: &EthState,
        updates: vector<u8>
    ) {
        let (data, slot) = verify_eth_state(
            updates,
            self.data
        );

        let new_state = EthState {
            id: object::new(ctx),
            data,
            slot,
        };

        transfer::freeze_object(new_state);

    }

    /// Approve a message by a dWallet.
    public fun approve_message(
        eth_dwallet_cap: &EthDWalletCap,
        dwallet: &DWallet,
        message: vector<u8>,
        proof: vector<u8>,
    ): vector<MessageApproval> {
        let dwallet_id = object::id(dwallet);

        let is_valid = verify_message_proof(
            proof,
            dwallet_id,
            eth_dwallet_cap.eth_smart_contract_slot,
            message,
        );
        assert!(is_valid, EInvalidStateProof);
        dwallet::approve_messages(&eth_dwallet_cap.dwallet_cap, vector[message])
    }

    /// Verify the Eth state according to the updates.
    native fun verify_eth_state(
        updates: vector<u8>,
        eth_state: vector<u8>
    ): (vector<u8>, u64);

    /// Verify the Message inside the Storage Merkel Root.
    native fun verify_message_proof(
        proof: vector<u8>,
        dwallet_id: ID,
        eth_smart_contract_slot: u64,
        message: vector<u8>,
    ): bool;
}
