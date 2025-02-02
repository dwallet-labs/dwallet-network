// allowed, anything can be done with a UID after unpacking, as long as it isn't repacked
module a::m {
    use ika::object::UID;

    struct Foo has key {
        id: UID,
    }

    public fun foo(f: Foo, v: &mut vector<UID>) {
        let Foo { id } = f;
        std::vector::push_back(v, id)
    }

}

module ika::object {
    struct UID has store {
        id: address,
    }
}
