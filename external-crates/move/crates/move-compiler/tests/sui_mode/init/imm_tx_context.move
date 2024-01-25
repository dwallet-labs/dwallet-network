// TxContext can be immutable, even for init
module a::m {
    use dwallet::tx_context;
    fun init(_ctx: &tx_context::TxContext) {
        abort 0
    }
}

module dwallet::tx_context {
    struct TxContext has drop {}
}
