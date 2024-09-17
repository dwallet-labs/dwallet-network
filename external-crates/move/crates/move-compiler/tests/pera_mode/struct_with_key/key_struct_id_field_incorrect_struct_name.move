// invalid, objects need UID not ID
module a::m {
    use pera::object;
    struct S has key {
        id: object::ID
    }
}

module pera::object {
    struct ID has store {
        id: address,
    }
}
