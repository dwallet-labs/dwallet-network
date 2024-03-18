module dwallet::eth_state {

    use dwallet::object;
    use dwallet::transfer;
    use dwallet::tx_context;
    use dwallet::tx_context::TxContext;
    use dwallet::object::UID;

    const ENotSystemAddress: u64 = 0;

    struct EthState has key {
        id: UID,
        data: vector<u8>,
    }

    #[allow(unused_function)]
    // Create and share the singleton EthState -- this function is
    // called exactly once, during genesis.
    public fun create_eth_state(ctx: &mut TxContext) {
        assert!(tx_context::sender(ctx) == @0x0, ENotSystemAddress);
        let data = b"{\"last_checkpoint\":\"0xa8ab0b7ab08b63839b668afa6b03beb4b50925bc0f0c65b4ee7b6c35a511b7ca\"}";

        transfer::share_object(EthState {
            id: object::eth_state_object(),
            data,
        })
    }

    public fun update_eth_state(
        self: &mut EthState,
        data: vector<u8>,
    ) {
        self.data = data;
    }
}
