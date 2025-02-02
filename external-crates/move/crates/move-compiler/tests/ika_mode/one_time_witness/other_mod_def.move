// invalid, one-time witness type candidate used in a different module

module a::n {
    use ika::ika;
    use ika::tx_context;

    fun init(_otw: ika::IKA, _ctx: &mut tx_context::TxContext) {
    }

}


module ika::tx_context {
    struct TxContext has drop {}
}

module ika::ika {
    struct IKA has drop {}
}
