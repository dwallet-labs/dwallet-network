// invalid, Clock by value

module a::m {
    public entry fun no_clock_val(_: pera::clock::Clock) {
        abort 0
    }
}

module pera::clock {
    struct Clock has key {
        id: pera::object::UID,
    }
}

module pera::object {
    struct UID has store {
        id: address,
    }
}
