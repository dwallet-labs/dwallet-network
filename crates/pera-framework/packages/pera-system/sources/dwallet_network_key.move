module pera_system::dwallet_network_key {
    use pera::event;

    /// Represents the key types supported by the system
    const Secp256k1: u8 = 0;
    const Ristretto: u8 = 1;

    /// Checks if the key type is supported by the system
    public fun is_key_type(val: u8): bool {
        return match (val) {
            Secp256k1 | Ristretto => true,
            _ => false,
        }
    }

    /// Event to start the network DKG
    public struct StartNetworkDkgEvent has store, copy, drop {
        session_id: ID,
        key_type: u8,
    }

    /// Function to create a new StartNetworkDkgEvent
    // Todo (#400): Add user restrictions, so that only someone we choose can run this function
    public fun emit_start_network_decryption_key_share_generation(key_type: u8, ctx: &mut TxContext) {
        let session_id = object::id_from_address(tx_context::fresh_object_address(ctx));
        event::emit(StartNetworkDkgEvent {
            session_id,
            key_type,
        });
    }

    /// Struct to store the network encryption of decryption key shares
    public struct EncryptionOfNetworkDecryptionKeyShares has store, copy {
        epoch: u64,
        current_epoch_shares: vector<vector<u8>>,
        previous_epoch_shares: vector<vector<u8>>,
    }

    /// Function to create a new EncryptionOfNetworkDecryptionKeyShares
    public fun new_encrypted_network_decryption_key_shares(epoch: u64, current_epoch_shares: vector<vector<u8>>, previous_epoch_shares: vector<vector<u8>>): EncryptionOfNetworkDecryptionKeyShares {
        EncryptionOfNetworkDecryptionKeyShares {
            epoch,
            current_epoch_shares,
            previous_epoch_shares,
        }
    }

    /// Function to update the shares of the network encryption of decryption key
    public fun update_new_shares(self: &mut EncryptionOfNetworkDecryptionKeyShares, new_shares: vector<vector<u8>>, epoch: u64) {
        self.previous_epoch_shares = self.current_epoch_shares;
        self.current_epoch_shares = new_shares;
        self.epoch = epoch;
    }
}