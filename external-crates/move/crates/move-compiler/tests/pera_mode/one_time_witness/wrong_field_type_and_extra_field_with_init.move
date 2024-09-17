module a::beep {
    struct BEEP has drop {
        f0: u64,
        f1: bool,
    }
    fun init(_: BEEP, _ctx: &mut pera::tx_context::TxContext) {
    }
}

module pera::tx_context {
    struct TxContext has drop {}
}
