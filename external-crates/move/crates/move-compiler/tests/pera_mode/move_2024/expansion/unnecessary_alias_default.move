module pera::object {
    public struct ID()
    public struct UID()
}
module pera::transfer {}
module pera::tx_context {
    public struct TxContext()
}

module a::m {
    use pera::object::{Self, ID, UID};
    use pera::transfer;
    use pera::tx_context::{Self, TxContext};
}
