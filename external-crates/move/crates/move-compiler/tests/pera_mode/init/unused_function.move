// init is unused but does not error because we are in Pera mode
module a::m {
    fun init(_: &mut pera::tx_context::TxContext) {}
}

module pera::tx_context {
    struct TxContext has drop {}
}
