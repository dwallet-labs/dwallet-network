module ika::object {
    public struct ID()
    public struct UID()
}
module ika::transfer {}
module ika::tx_context {
    public struct TxContext()
}

module a::m {
    use ika::object::{Self, ID, UID};
    use ika::transfer;
    use ika::tx_context::{Self, TxContext};
}
