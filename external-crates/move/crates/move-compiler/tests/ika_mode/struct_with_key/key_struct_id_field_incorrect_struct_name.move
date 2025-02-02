// invalid, objects need UID not ID
module a::m {
    use ika::object;
    struct S has key {
        id: object::ID
    }
}

module ika::object {
    struct ID has store {
        id: address,
    }
}
