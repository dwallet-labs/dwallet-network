// valid, Clock by immutable reference

module a::m {
    public entry fun yes_clock_ref(_: &dwallet::clock::Clock) {
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
