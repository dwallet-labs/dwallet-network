/// This module manages the storage of the network dWallet MPC keys and associated data.
module pera_system::dwallet_network_key {
    use pera::event;
    use pera_system::validator_set::{ValidatorDataForDWalletSecretShare, emit_validator_data_for_secret_share};

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

    /// Function to start a new network DKG.
    /// It emits a [`StartNetworkDKGEvent`] and emits the [`ValidatorDataForDWalletSecretShare`] for each validator,
    /// with its public key and proof, that are needed for the DKG process.
    ///
    /// Each validator's data is being emitted separately because the proof size is
    /// almost 250KB, which is the maximum event size in Sui.
    public(package) fun start_network_dkg(
        key_scheme: u8,
        validators_data: vector<ValidatorDataForDWalletSecretShare>,
        ctx: &mut TxContext
    ) {
        let session_id = object::id_from_address(tx_context::fresh_object_address(ctx));

        event::emit(StartNetworkDKGEvent {
            session_id,
            key_scheme,
        });
        let validators_len = validators_data.length();
        let mut i = 0;
        while (i < validators_len) {
            let validator_data = validators_data[i];
            emit_validator_data_for_secret_share(validator_data);
            i = i + 1;
        }
    }

    /// Struct to store the network encryption of decryption key shares
    public struct NetworkDecryptionKeyShares has store, copy {
        epoch: u64,
        current_epoch_shares: vector<u8>,
        previous_epoch_shares: vector<u8>,
        protocol_public_parameters: vector<u8>,
        decryption_public_parameters: vector<u8>,
        encryption_key: vector<u8>,
        reconstructed_commitments_to_sharing: vector<u8>,
    }

    /// Function to create a new NetworkDecryptionKeyShares.
    public(package) fun new_encrypted_network_decryption_key_shares(
        epoch: u64,
        current_epoch_shares: vector<u8>,
        previous_epoch_shares: vector<u8>,
        protocol_public_parameters: vector<u8>,
        decryption_public_parameters: vector<u8>,
        encryption_key: vector<u8>,
        reconstructed_commitments_to_sharing: vector<u8>,
    ): NetworkDecryptionKeyShares {
        NetworkDecryptionKeyShares {
            epoch,
            current_epoch_shares,
            previous_epoch_shares,
            protocol_public_parameters,
            decryption_public_parameters,
            encryption_key,
            reconstructed_commitments_to_sharing,
        }
    }

    /// Function to update the shares of the network encryption of decryption key.
    public fun update_new_shares(
        self: &mut NetworkDecryptionKeyShares,
        new_shares: vector<u8>,
        epoch: u64
    ) {
        self.previous_epoch_shares = self.current_epoch_shares;
        self.current_epoch_shares = new_shares;
        self.epoch = epoch;
    }
}
