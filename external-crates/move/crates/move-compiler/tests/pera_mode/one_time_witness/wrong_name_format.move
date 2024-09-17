// invalid, wrong one-time witness type name format

module a::mod {
    use pera::tx_context;

    struct Mod has drop { dummy: bool }

    fun init(_mod: Mod, _ctx: &mut tx_context::TxContext) {
    }
}

module pera::tx_context {
    struct TxContext has drop {}
}
