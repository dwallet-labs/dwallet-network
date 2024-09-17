module a::m {
    struct Obj has key { id: pera::object::UID }
}

module pera::object {
    struct UID has store { value: address }
    public fun borrow_address(id: &UID): &address { &id.value }
}
