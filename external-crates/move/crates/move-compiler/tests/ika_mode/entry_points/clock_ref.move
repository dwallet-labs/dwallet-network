// valid, Clock by immutable reference

module a::m {
    public entry fun yes_clock_ref(_: &ika::clock::Clock) {
        abort 0
    }
}

module ika::clock {
    struct Clock has key {
        id: ika::object::UID,
    }
}

module ika::object {
    struct UID has store {
        id: address,
    }
}
