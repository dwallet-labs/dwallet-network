module a::m {
    struct Obj has key { id: ika::object::UID }
}

module ika::object {
    struct UID has store { value: address }
    public fun borrow_address(id: &UID): &address { &id.value }
}
