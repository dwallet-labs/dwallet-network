module 0x42::suppress_cases {
    use pera::tx_context::TxContext;

    #[allow(lint(prefer_mut_tx_context))]
    public fun suppressed_function(_ctx: &TxContext) {
    }

    #[allow(lint(prefer_mut_tx_context))]
    public fun multi_suppressed_function(_ctx: &TxContext) {
    }

    #[allow(lint(prefer_mut_tx_context))]
    public fun suppressed_multi_param(_a: u64, _ctx: &TxContext, _b: &mut TxContext) {
    }
}

// Mocking the pera::tx_context module
module pera::tx_context {
    struct TxContext has drop {}
}
