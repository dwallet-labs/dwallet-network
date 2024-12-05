// invalid, object must have UID as first field not some other field

module a::m {
    use ika::object;
    struct S has key {
        flag: bool,
        id: object::UID,
    }

    struct R has key {
        flag: bool,
        id: address,
    }
}

module ika::object {
    struct UID has store {
        id: address,
    }
}
