// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use prometheus::{register_int_gauge_with_registry, IntGauge, Registry};
use std::sync::Arc;

// #[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
// pub enum MPCProtocolInitData {
//     /// The first round of the DKG protocol.
//     DKGFirst(DWalletMPCSuiEvent<StartDKGFirstRoundEvent>),
//     /// The second round of the DKG protocol.
//     /// Contains the data of the event that triggered the round,
//     /// and the network key version of the first round.
//     DKGSecond(DWalletMPCSuiEvent<StartDKGSecondRoundEvent>),
//     /// The first round of the Presign protocol for each message in the Batch.
//     /// Contains the `ObjectId` of the dWallet object,
//     /// the DKG decentralized output, the batch session ID (same for each message in the batch),
//     /// and the dWallets' network key version.
//     Presign(DWalletMPCSuiEvent<StartPresignFirstRoundEvent>),
//     /// The first and only round of the Sign protocol.
//     /// Contains all the data needed to sign the message.
//     Sign(DWalletMPCSuiEvent<StartSignEvent>),
//     /// The only round of the network DKG protocol.
//     /// Contains the network key scheme, the dWallet network decryption key object ID
//     /// and at the end of the session holds the new key version.
//     NetworkDkg(
//         DWalletMPCNetworkKeyScheme,
//         DWalletMPCSuiEvent<StartNetworkDKGEvent>,
//     ),
//     /// The round of verifying the encrypted share proof is valid and
//     /// that the signature on it is valid.
//     /// This is not a real MPC round,
//     /// but we use it to start the verification process using the same events mechanism
//     /// because the system does not support native functions.
//     EncryptedShareVerification(DWalletMPCSuiEvent<StartEncryptedShareVerificationEvent>),
//     PartialSignatureVerification(DWalletMPCSuiEvent<StartPartialSignaturesVerificationEvent>),
//     DecryptionKeyReshare(DWalletMPCSuiEvent<DWalletDecryptionKeyReshareRequestEvent>),
// }


pub struct DWalletMPCMetrics {
    advance_calls_for_dwallet_dkg_first_round: IntGauge,
    advance_completions_for_dwallet_dkg_first_round: IntGauge,
    received_start_dwallet_dkg_events_count: IntGauge,
    dwallet_dkg_first_round_completions_count: IntGauge,
    // DKG Second Round
    advance_calls_for_dwallet_dkg_second_round: IntGauge,
    advance_completions_for_dwallet_dkg_second_round: IntGauge,
    received_start_dwallet_dkg_second_round_events_count: IntGauge,
    dwallet_dkg_second_round_completions_count: IntGauge,

    // Presign
    advance_calls_for_presign: IntGauge,
    advance_completions_for_presign: IntGauge,
    received_start_presign_events_count: IntGauge,
    presign_round_completions_count: IntGauge,

    // Sign
    advance_calls_for_sign: IntGauge,
    advance_completions_for_sign: IntGauge,
    received_start_sign_events_count: IntGauge,
    sign_round_completions_count: IntGauge,

    // Network DKG
    advance_calls_for_network_dkg: IntGauge,
    advance_completions_for_network_dkg: IntGauge,
    received_start_network_dkg_events_count: IntGauge,
    network_dkg_round_completions_count: IntGauge,

    // Encrypted Share Verification
    advance_calls_for_encrypted_share_verification: IntGauge,
    advance_completions_for_encrypted_share_verification: IntGauge,
    received_start_encrypted_share_verification_events_count: IntGauge,
    encrypted_share_verification_round_completions_count: IntGauge,

    // Partial Signature Verification
    advance_calls_for_partial_signature_verification: IntGauge,
    advance_completions_for_partial_signature_verification: IntGauge,
    received_start_partial_signature_verification_events_count: IntGauge,
    partial_signature_verification_round_completions_count: IntGauge,

    // Decryption Key Reshare
    advance_calls_for_decryption_key_reshare: IntGauge,
    advance_completions_for_decryption_key_reshare: IntGauge,
    received_start_decryption_key_reshare_events_count: IntGauge,
    decryption_key_reshare_round_completions_count: IntGauge
}

impl DWalletMPCMetrics {
    pub fn new(registry: &Registry) -> Arc<Self> {
        let this = Self {
            advance_completions_for_dwallet_dkg_first_round: register_int_gauge_with_registry!(
                "advance_completions_for_dwallet_dkg_first_round",
                "Advance completions for dwallet dkg first round",
                registry
            )
                .unwrap(),

        };
        Arc::new(this)
    }
}
