module dwallet_system::ethereum_state {
    use dwallet::object::{Self, UID, ID};
    use dwallet::transfer;
    use dwallet::tx_context::TxContext;

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

    public fun init_state(checkpoint: vector<u8>, ctx: &mut TxContext){
        let state_data = create_initial_eth_state_data(checkpoint);
        let state = EthState {
            id: object::new(ctx),
            data: state_data,
            time_slot: 0u64,
        };

        transfer::share_object(LatestEthereumState {
            id: object::new(ctx),
            eth_state_id: object::id(&state),
            last_slot: state.time_slot,
        });
        transfer::freeze_object(state);
    }

    public fun update_latest_eth_state(
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
        latest_ethereum_state: &mut LatestEthereumState,
        ctx: &mut TxContext,
    ) {
        let (data, time_slot) = verify_eth_state(
            updates_bytes,
            state_bytes
        );

        let new_state = EthState {
            id: object::new(ctx),
            data,
            time_slot,
        };

        if (new_state.time_slot > latest_ethereum_state.last_slot) {
            latest_ethereum_state.eth_state_id = object::id(&new_state);
            latest_ethereum_state.last_slot = new_state.time_slot;
        };
        transfer::freeze_object(new_state);
    }

    /// Verify the Eth state according to the updates.
    native fun verify_eth_state(
        updates: vector<u8>,
        eth_state: vector<u8>
    ): (vector<u8>, u64);

    native fun create_initial_eth_state_data(
        checkpoint: vector<u8>,
    ): vector<u8>;
}
