// invalid, Clock by value

module a::m {
    public entry fun no_clock_val(_: dwallet::clock::Clock) {
        abort 0
    }
}

module dwallet::clock {
    struct Clock has key {
        id: dwallet::object::UID,
    }
}

module dwallet::object {
    struct UID has store {
        id: address,
    }
}
