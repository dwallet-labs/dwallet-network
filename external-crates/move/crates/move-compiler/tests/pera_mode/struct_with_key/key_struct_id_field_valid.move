// valid
module a::m {
    use pera::object;
    struct S has key {
        id: object::UID
    }
}

module pera::object {
    struct UID has store {
        id: address,
    }
}
