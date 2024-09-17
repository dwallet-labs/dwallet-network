// allowed since no object is being created with the UID

module a::m {
    use pera::object::UID;

    struct Foo has key {
        id: UID,
    }

    public fun transfer(_: UID) {
        abort 0
    }

    public fun foo(f: Foo) {
        let Foo { id } = f;
        transfer(id);
    }

}

module pera::object {
    struct UID has store {
        id: address,
    }
}
