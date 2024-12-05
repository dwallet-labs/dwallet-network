module a::m {
    use ika::tx_context;

    public entry fun foo<T>(_: T, _: &mut tx_context::TxContext) {
        abort 0
    }

}

module ika::tx_context {
    struct TxContext has drop {}
}
