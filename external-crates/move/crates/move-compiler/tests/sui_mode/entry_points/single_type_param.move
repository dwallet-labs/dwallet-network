module a::m {
    use dwallet::tx_context;

    public entry fun foo<T>(_: T, _: &mut tx_context::TxContext) {
        abort 0
    }

}

module dwallet::tx_context {
    struct TxContext has drop {}
}
