module pera_system::dwallet_network_key {
    const Secp256k1: u8 = 0;
    const Ristretto: u8 = 1;

    public fun is_key_type(val: u8): bool {
        return match (val) {
            Secp256k1 | Ristretto => true,
            // 1 => true,
            _ => false,
        }
    }

    public struct EncryptedNetworkDecryptionKeyShares has store, copy {
        epoch: u64,
        current_epoch_shares: vector<vector<u8>>,
        previous_epoch_shares: vector<vector<u8>>,
    }

    public fun new_encrypted_network_decryption_key_shares(epoch: u64, current_epoch_shares: vector<vector<u8>>, previous_epoch_shares: vector<vector<u8>>): EncryptedNetworkDecryptionKeyShares {
        EncryptedNetworkDecryptionKeyShares {
            epoch,
            current_epoch_shares,
            previous_epoch_shares,
        }
    }

    public fun update_new_shares(self: &mut EncryptedNetworkDecryptionKeyShares, new_shares: vector<vector<u8>>, epoch: u64) {
        self.previous_epoch_shares = self.current_epoch_shares;
        self.current_epoch_shares = new_shares;
        self.epoch = epoch;
    }
}