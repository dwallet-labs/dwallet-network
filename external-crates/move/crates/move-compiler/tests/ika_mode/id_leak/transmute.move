// not allowed, it re-uses an ID in a new object
module a::m {
    use ika::object::UID;
    use ika::tx_context::{Self, TxContext};
    use ika::transfer::transfer;

    struct Cat has key {
        id: UID,
    }

    struct Dog has key {
        id: UID,
    }

    public fun transmute(cat: Cat, ctx: &mut TxContext) {
        let Cat { id } = cat;
        let dog = Dog { id };
        transfer(dog, tx_context::sender(ctx));
    }

}

module ika::object {
    struct UID has store {
        id: address,
    }
}

module ika::tx_context {
    struct TxContext has drop {}
    public fun sender(_: &TxContext): address {
        @0
    }
}

module ika::transfer {
    public fun transfer<T: key>(_: T, _: address) {
        abort 0
    }
}
