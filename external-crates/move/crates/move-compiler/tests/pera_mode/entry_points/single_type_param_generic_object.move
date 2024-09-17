module a::m {
    use pera::object;
    use pera::tx_context;
    struct Obj<T> has key {
        id: object::UID,
        value: T,
    }
    public entry fun foo<T: store>(_: Obj<T>, _: &mut tx_context::TxContext) {
        abort 0
    }

}

module pera::object {
    struct UID has store {
        id: address,
    }
}

module pera::tx_context {
    struct TxContext has drop {}
}
