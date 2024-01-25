// valid, Receiving type with object type param

module a::m {
    use dwallet::object;
    use dwallet::transfer::Receiving;

    struct S has key { id: object::UID }

    public entry fun yes(_: Receiving<S>) { }
}

module dwallet::object {
    struct UID has store {
        id: address,
    }
}

module dwallet::transfer {
    struct Receiving<phantom T: key> has drop {
        id: address
    }
}
