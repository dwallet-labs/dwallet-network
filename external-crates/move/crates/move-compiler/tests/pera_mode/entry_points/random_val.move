// invalid Random by value

module a::m {
    public entry fun no_random_val(_: pera::random::Random) {
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
