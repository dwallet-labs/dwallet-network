// invalid, one-time witness type candidate used in a different module

module a::n {
    use dwallet::sui;
    use dwallet::tx_context;

    fun init(_otw: dwallet::SUI, _ctx: &mut tx_context::TxContext) {
    }

}


module dwallet::tx_context {
    struct TxContext has drop {}
}

module dwallet::sui {
    struct SUI has drop {}
}
