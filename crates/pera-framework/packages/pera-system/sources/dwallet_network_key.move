module pera_system::dwallet_network_key {
    use pera::event;

    /// Represents the key schemes supported by the system.
    const Secp256k1: u8 = 0;
    const Ristretto: u8 = 1;

    /// Checks if the key scheme is supported by the system
    public(package) fun is_valid_key_scheme(val: u8): bool {
        return match (val) {
            Secp256k1 | Ristretto => true,
            _ => false,
        }
    }

    /// Event to start the network DKG.
    public struct StartNetworkDKGEvent has store, copy, drop {
        session_id: ID,
        key_scheme: u8,
    }

    /// Function to emit a new StartNetworkDKGEvent.
    public(package) fun start_network_dkg(key_scheme: u8, ctx: &mut TxContext) {
        let session_id = object::id_from_address(tx_context::fresh_object_address(ctx));
        event::emit(StartNetworkDKGEvent {
            session_id,
            key_scheme,
        });
    }

    /// Struct to store the network encryption of decryption key shares
    public struct DwalletMPCNetworkKey has store, copy {
        epoch: u64,
        current_epoch_shares: vector<vector<u8>>,
        previous_epoch_shares: vector<vector<u8>>,
        protocol_public_parameters: vector<u8>,
        decryption_public_parameters: vector<u8>,
    }

    public fun protocol_public_parameters(self: &DwalletMPCNetworkKey): vector<u8> {
        self.protocol_public_parameters
    }

    /// Function to create a new DwalletMPCNetworkKey.
    public(package) fun new_encrypted_network_decryption_key_shares(
        epoch: u64,
        current_epoch_shares: vector<vector<u8>>,
        previous_epoch_shares: vector<vector<u8>>,
        protocol_public_parameters: vector<u8>,
        decryption_public_parameters: vector<u8>,
    ): DwalletMPCNetworkKey {
        DwalletMPCNetworkKey {
            epoch,
            current_epoch_shares,
            previous_epoch_shares,
            protocol_public_parameters,
            decryption_public_parameters,
        }
    }

    /// Function to update the shares of the network encryption of decryption key.
    public fun update_new_shares(
        self: &mut DwalletMPCNetworkKey,
        new_shares: vector<vector<u8>>,
        epoch: u64
    ) {
        self.previous_epoch_shares = self.current_epoch_shares;
        self.current_epoch_shares = new_shares;
        self.epoch = epoch;
    }
}