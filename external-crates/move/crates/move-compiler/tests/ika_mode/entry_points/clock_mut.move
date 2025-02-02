// invalid, Clock by mutable reference

module a::m {
    public entry fun no_clock_mut(_: &mut ika::clock::Clock) {
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
