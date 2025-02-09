// Copyright (c) dWallet Labs Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module ika_system::dwallet_pricing;


/// Represents a capability granting control over a specific dWallet.
public struct DWalletPricing2PcMpcSecp256K1 has key, store {
    id: UID,
    /// The computation IKA price per dkg for the current epoch.
    computation_ika_price_per_dkg: u64,
    /// The computation SUI price per dkg for the current epoch.
    computation_sui_price_per_dkg: u64,
    /// The computation IKA price per re-encrypt user share for the current epoch.
    computation_ika_price_per_re_encrypt_user_share: u64,
    /// The computation SUI price per re-encrypt user share for the current epoch.
    computation_sui_price_per_re_encrypt_user_share: u64,
    /// The computation IKA price per ecdsa presign for the current epoch.
    computation_ika_price_per_ecdsa_presign: u64,
    /// The computation SUI price per ecdsa presign for the current epoch.
    computation_sui_price_per_ecdsa_presign: u64,
    /// The computation IKA price per ecdsa sign for the current epoch.
    computation_ika_price_per_ecdsa_sign: u64,
    /// The computation SUI price per ecdsa sign for the current epoch.
    computation_sui_price_per_ecdsa_sign: u64,
    /// The computation IKA price per ecdsa future sign for the current epoch.
    computation_ika_price_per_ecdsa_future_sign: u64,
    /// The computation SUI price per ecdsa future sign for the current epoch.
    computation_sui_price_per_ecdsa_future_sign: u64,
    /// The computation IKA price per ecdsa sign with partial user signature for the current epoch.
    computation_ika_price_per_ecdsa_sign_with_partial_user_signature: u64,
    /// The computation SUI price per ecdsa sign with partial user signature for the current epoch.
    computation_sui_price_per_ecdsa_sign_with_partial_user_signature: u64,
}

/// Create a new [`DWalletPricing2PcMpcSecp256K1`] object.
///
/// Holds the pricing for the current epoch.
///
/// ### Returns
/// The newly created `DWalletPricing2PcMpcSecp256K1` object.
public(package) fun create_dwallet_pricing_2pc_mpc_secp256k1(
    computation_ika_price_per_dkg: u64,
    computation_sui_price_per_dkg: u64,
    computation_ika_price_per_re_encrypt_user_share: u64,
    computation_sui_price_per_re_encrypt_user_share: u64,
    computation_ika_price_per_ecdsa_presign: u64,
    computation_sui_price_per_ecdsa_presign: u64,
    computation_ika_price_per_ecdsa_sign: u64,
    computation_sui_price_per_ecdsa_sign: u64,
    computation_ika_price_per_ecdsa_future_sign: u64,
    computation_sui_price_per_ecdsa_future_sign: u64,
    computation_ika_price_per_ecdsa_sign_with_partial_user_signature: u64,
    computation_sui_price_per_ecdsa_sign_with_partial_user_signature: u64,
    ctx: &mut TxContext
): DWalletPricing2PcMpcSecp256K1 {
    DWalletPricing2PcMpcSecp256K1 {
        id: object::new(ctx),
        computation_ika_price_per_dkg,
        computation_sui_price_per_dkg,
        computation_ika_price_per_re_encrypt_user_share,
        computation_sui_price_per_re_encrypt_user_share,
        computation_ika_price_per_ecdsa_presign,
        computation_sui_price_per_ecdsa_presign,
        computation_ika_price_per_ecdsa_sign,
        computation_sui_price_per_ecdsa_sign,
        computation_ika_price_per_ecdsa_future_sign,
        computation_sui_price_per_ecdsa_future_sign,
        computation_ika_price_per_ecdsa_sign_with_partial_user_signature,
        computation_sui_price_per_ecdsa_sign_with_partial_user_signature,
    }
}

/// Returns the IKA price per dkg.
public fun computation_ika_price_per_dkg(self: &DWalletPricing2PcMpcSecp256K1): u64 {
    self.computation_ika_price_per_dkg
}

/// Returns the SUI price per dkg.
public fun computation_sui_price_per_dkg(self: &DWalletPricing2PcMpcSecp256K1): u64 {
    self.computation_sui_price_per_dkg
}

/// Returns the IKA price per re-encrypt user share.
public fun computation_ika_price_per_re_encrypt_user_share(self: &DWalletPricing2PcMpcSecp256K1): u64 {
    self.computation_ika_price_per_re_encrypt_user_share
}

/// Returns the SUI price per re-encrypt user share.
public fun computation_sui_price_per_re_encrypt_user_share(self: &DWalletPricing2PcMpcSecp256K1): u64 {
    self.computation_sui_price_per_re_encrypt_user_share
}

/// Returns the IKA price per ecdsa presign.
public fun computation_ika_price_per_ecdsa_presign(self: &DWalletPricing2PcMpcSecp256K1): u64 {
    self.computation_ika_price_per_ecdsa_presign
}

/// Returns the SUI price per ecdsa presign.
public fun computation_sui_price_per_ecdsa_presign(self: &DWalletPricing2PcMpcSecp256K1): u64 {
    self.computation_sui_price_per_ecdsa_presign
}

/// Returns the IKA price per ecdsa sign.
public fun computation_ika_price_per_ecdsa_sign(self: &DWalletPricing2PcMpcSecp256K1): u64 {
    self.computation_ika_price_per_ecdsa_sign
}

/// Returns the SUI price per ecdsa sign.
public fun computation_sui_price_per_ecdsa_sign(self: &DWalletPricing2PcMpcSecp256K1): u64 {
    self.computation_sui_price_per_ecdsa_sign
}

/// Returns the IKA price per ecdsa future sign.
public fun computation_ika_price_per_ecdsa_future_sign(self: &DWalletPricing2PcMpcSecp256K1): u64 {
    self.computation_ika_price_per_ecdsa_future_sign
}

/// Returns the SUI price per ecdsa future sign.
public fun computation_sui_price_per_ecdsa_future_sign(self: &DWalletPricing2PcMpcSecp256K1): u64 {
    self.computation_sui_price_per_ecdsa_future_sign
}

/// Returns the IKA price per ecdsa sign with partial user signature.
public fun computation_ika_price_per_ecdsa_sign_with_partial_user_signature(self: &DWalletPricing2PcMpcSecp256K1): u64 {
    self.computation_ika_price_per_ecdsa_sign_with_partial_user_signature
}

/// Returns the SUI price per ecdsa sign with partial user signature.
public fun computation_sui_price_per_ecdsa_sign_with_partial_user_signature(self: &DWalletPricing2PcMpcSecp256K1): u64 {
    self.computation_sui_price_per_ecdsa_sign_with_partial_user_signature
}