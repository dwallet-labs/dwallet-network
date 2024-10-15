// pera mode has the implicit asliases:
// use pera::object::{Self, ID, UID};
// use pera::transfer;
// use pera::tx_context::{Self, TxContext};
module a::m {
    public struct S has key { id: UID, other: ID }
    public fun create(ctx: &mut TxContext) {
        transfer::transfer(
            S { id: object::new(ctx), other: object::id_from_address(@0) },
            tx_context::sender(ctx),
        )
    }
}


// we don't link out to the pera framework
module pera::object {
    public struct ID has copy, drop, store {
        bytes: address
    }

    public struct UID has store {
        id: ID,
    }

    public fun new(_: &mut TxContext): UID { abort 0 }
    public fun id_from_address(_: address): ID { abort 0 }
}
module pera::transfer {
    public fun transfer<T: key>(_: T, _: address) { abort 0 }
}
module pera::tx_context {
    public struct TxContext has drop {}
    public fun sender(_: &TxContext): address { @0 }
}