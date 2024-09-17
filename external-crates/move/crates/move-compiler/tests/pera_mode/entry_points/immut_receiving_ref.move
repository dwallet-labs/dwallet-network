// valid, Receiving type by immut ref with object type param

module a::m {
    use pera::object;
    use pera::transfer::Receiving;

    struct S has key { id: object::UID }

    public entry fun yes(_: &Receiving<S>) { }
}

module pera::object {
    struct UID has store {
        id: address,
    }
}

module pera::transfer {
    struct Receiving<phantom T: key> has drop {
        id: address
    }
}
