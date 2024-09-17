// valid, Clock by immutable reference

module a::m {
    public entry fun yes_clock_ref(_: &pera::clock::Clock) {
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
