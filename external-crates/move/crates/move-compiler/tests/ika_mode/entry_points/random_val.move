// invalid Random by value

module a::m {
    public entry fun no_random_val(_: ika::random::Random) {
        abort 0
    }
}

module ika::random {
    struct Random has key {
        id: ika::object::UID,
    }
}

module ika::object {
    struct UID has store {
        id: address,
    }
}
