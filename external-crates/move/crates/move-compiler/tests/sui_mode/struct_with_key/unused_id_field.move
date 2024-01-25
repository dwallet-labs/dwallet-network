module a::m {
    struct Obj has key { id: dwallet::object::UID }
}

module dwallet::object {
    struct UID has store { value: address }
    public fun borrow_address(id: &UID): &address { &id.value }
}
