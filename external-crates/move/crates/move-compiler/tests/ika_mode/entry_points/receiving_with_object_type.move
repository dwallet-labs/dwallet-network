// valid, Receiving type with object type param

module a::m {
    use ika::object;
    use ika::transfer::Receiving;

    struct S has key { id: object::UID }

    public entry fun yes(_: Receiving<S>) { }
}

module ika::object {
    struct UID has store {
        id: address,
    }
}

module ika::transfer {
    struct Receiving<phantom T: key> has drop {
        id: address
    }
}
