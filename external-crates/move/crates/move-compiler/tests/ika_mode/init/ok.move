// valid init function
module a::m {
    use ika::tx_context;
    fun init(_: &mut tx_context::TxContext) {
    }
}

module ika::tx_context {
    struct TxContext has drop {}
}
