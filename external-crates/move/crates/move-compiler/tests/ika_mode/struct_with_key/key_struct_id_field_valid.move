// valid
module a::m {
    use ika::object;
    struct S has key {
        id: object::UID
    }
}

module ika::object {
    struct UID has store {
        id: address,
    }
}
