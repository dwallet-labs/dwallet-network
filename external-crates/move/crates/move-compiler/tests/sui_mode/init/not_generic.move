// init functions cannot have generics
module a::m {
    use dwallet::tx_context;
    fun init<T>(_ctx: &mut tx_context::TxContext) {
        abort 0
    }
}

module dwallet::tx_context {
    struct TxContext has drop {}
}
