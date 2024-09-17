module a::m {
    fun init(_ctx: who::TxContext) {}
}

module a::beep {
    struct BEEP has drop {}
    fun init(_: Who, _ctx: &mut pera::tx_context::TxContext) {}
}

module pera::tx_context {
    struct TxContext {}
}
