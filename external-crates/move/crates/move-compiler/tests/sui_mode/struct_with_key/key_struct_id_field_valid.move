// valid
module a::m {
    use dwallet::object;
    struct S has key {
        id: object::UID
    }
}

module dwallet::object {
    struct UID has store {
        id: address,
    }
}
