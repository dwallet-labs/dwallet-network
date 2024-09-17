// valid init function
module a::m {
    use pera::tx_context;
    fun init(_: &mut tx_context::TxContext) {
    }
}

module pera::tx_context {
    struct TxContext has drop {}
}
