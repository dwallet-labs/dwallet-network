module dwallet_system::ethereum_state {
    use dwallet::object;
    use dwallet::transfer;

    struct EthState has key {
        id: UID,
        data: vector<u8>,
        time_slot: u64,
    }

    struct LatestEthereumState has key, store {
        id: UID,
        eth_state_id: ID,
        last_slot: u64,
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

    public fun update_current_eth_state(
        self: &mut LatestEthereumState,
        eth_state: &EthState,
    ) {
        if (eth_state.time_slot > self.last_slot) {
            self.eth_state_id = object::id(eth_state);
            self.last_slot = eth_state.time_slot;
        }
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

        //todo(yuval): update time_slot check
        if (time_slot != 0) {
            transfer::freeze_object(EthState {
                id: object::new(ctx),
                data,
                time_slot,
            });
        }
    }

    /// Verify the Eth state according to the updates.
    native fun verify_eth_state(
        updates: vector<u8>,
        eth_state: vector<u8>
    ): (vector<u8>, u64);

    native fun create_initial_eth_state_data(
        checkpoint: vector<u8>,
        network: vector<u8>
    ): vector<u8>;
}
