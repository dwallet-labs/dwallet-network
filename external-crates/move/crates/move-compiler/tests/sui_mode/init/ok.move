// valid init function
module a::m {
    use dwallet::tx_context;
    fun init(_: &mut tx_context::TxContext) {
    }
}

module dwallet::tx_context {
    struct TxContext has drop {}
}
