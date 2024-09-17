// valid Random by immutable reference

module a::m {
    public entry fun yes_random_ref(_: &pera::random::Random) {
        abort 0
    }
}

module pera::random {
    struct Random has key {
        id: pera::object::UID,
    }
}

module pera::object {
    struct UID has store {
        id: address,
    }
}
