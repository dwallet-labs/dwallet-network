module dwallet_system::eth_dwallet {
    use std::string::String;
    use dwallet::object::{Self, UID, ID};
    use dwallet::transfer;
    use dwallet::tx_context::TxContext;
    use dwallet_system::dwallet;
    use dwallet_system::dwallet::{DWalletCap, MessageApproval};

    const EInvalidStateProof: u64 = 1;

    /// Holds the DWalletCap.
    struct EthDWalletCap has key {
        id: UID,
        dwallet_cap: DWalletCap,
        eth_smart_contract_addr: String,
        eth_smart_contract_slot: u64,
    }

    struct EthState has key {
        id: UID,
        data: vector<u8>,
        time_slot: u64,
    }

    public fun init_first_eth_state(
        checkpoint: vector<u8>,
        network: vector<u8>,
        ctx: &mut TxContext
    ) {
        let data = create_initial_eth_state_data(checkpoint, network);
        let eth_state = EthState {
            id: object::new(ctx),
            data,
            time_slot: 0u64,
        };
        transfer::freeze_object(eth_state);
    }

    //todo: this should be initialized with the genesis state?
    // This object is mutable, and should hold the id of the latest EthState object that is verified.
    struct CurrentEthState has key {
        id: UID,
        eth_state_id: ID,
        last_slot: u64,
    }

    public fun update_current_eth_state(
        self: &mut CurrentEthState,
        eth_state: &EthState,
    ) {
        if (eth_state.time_slot > self.last_slot) {
            self.eth_state_id = object::id(eth_state);
            self.last_slot = eth_state.time_slot;
        }
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

    /// Verify the Eth state according to the updates.
    public fun verify_new_eth_state(
        updates_bytes: vector<u8>,
        state_bytes: vector<u8>,
        ctx: &mut TxContext,
    ) {
        let (data, time_slot) = verify_eth_state(
            updates_bytes,
            state_bytes
        );

        //todo: update time_slot check
        if (time_slot != 0) {
            transfer::freeze_object(EthState {
                id: object::new(ctx),
                data,
                time_slot,
            });
        }
    }

    /// Approve a message by a dWallet.
    public fun approve_message(
        eth_dwallet_cap: &EthDWalletCap,
        message: vector<u8>,
        proof: vector<u8>,
    ): vector<MessageApproval> {
        let is_valid = verify_message_proof(proof);
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
        proof: vector<u8>
    ): bool;

    native fun create_initial_eth_state_data(
        checkpoint: vector<u8>,
        network: vector<u8>
    ): vector<u8>;
}
