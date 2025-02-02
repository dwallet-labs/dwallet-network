// valid Random by immutable reference

module a::m {
    public entry fun yes_random_ref(_: &ika::random::Random) {
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
