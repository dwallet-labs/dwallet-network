// invalid Random by mutable reference

module a::m {
    public entry fun no_random_mut(_: &mut pera::random::Random) {
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
