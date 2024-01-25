// init is unused but does not error because we are in Sui mode
module a::m {
    fun init(_: &mut dwallet::tx_context::TxContext) {}
}

module dwallet::tx_context {
    struct TxContext has drop {}
}
