module pera_system::dwallet_network_key {
    public enum KeyType has store, drop, copy {
        Secp256k1,
        Ristretto,
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