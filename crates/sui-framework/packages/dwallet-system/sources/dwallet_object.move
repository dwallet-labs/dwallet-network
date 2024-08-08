#[allow(unused_const)]
module dwallet_system::dwalllet_object {
    use dwallet::object;
    use dwallet::object::{UID, ID};
    use dwallet::tx_context::TxContext;

    #[allow(unused_field)]
    /// `DWallet` represents a wallet that is created after the DKG process.
    struct DWallet has key, store {
        id: UID,
        session_id: ID,
        dwallet_cap_id: ID,
        // `output` is output for `verify_decommitment_and_proof_of_centralized_party_public_key_share()`
        output: vector<u8>,
        public_key: vector<u8>,
    }

    public fun new_dwallet(session_id: ID, dwallet_cap_id: ID, output: vector<u8>, public_key: vector<u8>, ctx: &mut TxContext): DWallet {
        DWallet {
            id: object::new(ctx),
            session_id,
            dwallet_cap_id,
            output,
            public_key,
        }
    }

    public fun output(dwallet: &DWallet): vector<u8> { dwallet.output }

    public fun dwallet_cap_id(dwallet: &DWallet): ID { dwallet.dwallet_cap_id }
}