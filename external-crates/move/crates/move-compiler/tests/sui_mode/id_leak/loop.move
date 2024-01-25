// allowed, even though a bit pointless
module a::m {
    use dwallet::object::{Self, UID};
    use dwallet::tx_context::{Self, TxContext};
    use dwallet::transfer::transfer;

    struct Obj has key {
        id: UID
    }

    public entry fun transmute(ctx: &mut TxContext) {
        let i = 0;
        let id = object::new(ctx);
        while (i <= 10) {
            object::delete(id);
            id = object::new(ctx);
            i = i + 1;
        };
        let obj = Obj { id };
        transfer(obj, tx_context::sender(ctx))
    }

}

module dwallet::object {
    struct UID has store {
        id: address,
    }
    public fun new(_: &mut dwallet::tx_context::TxContext): UID {
        abort 0
    }
    public fun delete(_: UID) {
        abort 0
    }
}

module dwallet::tx_context {
    struct TxContext has drop {}
    public fun sender(_: &TxContext): address {
        @0
    }
}

module dwallet::transfer {
    public fun transfer<T: key>(_: T, _: address) {
        abort 0
    }
}
