// invalid, one-time witness type candidate used in a different module

module a::n {
    use pera::pera;
    use pera::tx_context;

    fun init(_otw: pera::PERA, _ctx: &mut tx_context::TxContext) {
    }

}


module pera::tx_context {
    struct TxContext has drop {}
}

module pera::pera {
    struct PERA has drop {}
}
