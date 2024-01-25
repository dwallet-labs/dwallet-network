// invalid, objects need UID not ID
module a::m {
    use dwallet::object;
    struct S has key {
        id: object::ID
    }
}

module dwallet::object {
    struct ID has store {
        id: address,
    }
}
