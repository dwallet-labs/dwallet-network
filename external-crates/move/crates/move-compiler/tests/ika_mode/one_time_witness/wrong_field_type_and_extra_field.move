module a::beep {
    struct BEEP has drop {
        f0: u64,
        f1: bool,
    }
    fun init(_ctx: &mut ika::tx_context::TxContext) {
    }
}

module ika::tx_context {
    struct TxContext has drop {}
}
