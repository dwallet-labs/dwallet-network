// correct, bool field specified at source level

module a::m {
    use pera::tx_context;

    struct M has drop { some_field: bool }

    fun init(_otw: M, _ctx: &mut tx_context::TxContext) {
        return;
    }
}

module pera::tx_context {
    struct TxContext has drop {}
}
