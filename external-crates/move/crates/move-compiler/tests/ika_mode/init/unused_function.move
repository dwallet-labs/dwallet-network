// init is unused but does not error because we are in Ika mode
module a::m {
    fun init(_: &mut ika::tx_context::TxContext) {}
}

module ika::tx_context {
    struct TxContext has drop {}
}
