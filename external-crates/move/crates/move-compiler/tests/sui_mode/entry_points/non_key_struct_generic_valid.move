// valid, T has store, thus Obj has key

module a::m {
    use pera::object;

    struct Obj<T> has key { id: object::UID, value: T }

    public entry fun no<T: store>(_: Obj<T>) {
        abort 0
    }
}

module pera::object {
    struct UID has store {
        id: address,
    }
}
