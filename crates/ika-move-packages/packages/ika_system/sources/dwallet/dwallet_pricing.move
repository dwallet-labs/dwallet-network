// Copyright (c) dWallet Labs Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

/// # Module: dwallet_pricing
///
/// This module provides structures and functions for managing pricing information for a dWallet.
/// Each operation (e.g., DKG, re-encrypt user share, ECDSA presign, etc.) has its own pricing data,
/// represented by a `PricingPerOperation`. Each `PricingPerOperation` holds three values:
///   - **consensus_validation_ika**: The consensus validation IKA price.
///   - **computation_ika**: The computation_ika IKA price.
///   - **gas_fee_reimbursement_sui**: The SUI reimbursement.
/// 
/// The main struct, `DWalletPricing2PcMpcSecp256K1`, now holds one `PricingPerOperation` per operation.
/// The DKG operation is split into two separate rounds:
///   - `dkg_first_round`
///   - `dkg_second_round`
module ika_system::dwallet_pricing;

/// Holds pricing information for a single operation.
/// The fields are ordered so that the consensus validation price is first.
public struct PricingPerOperation has copy, drop, store {
    consensus_validation_ika: u64,
    computation_ika: u64,
    gas_fee_reimbursement_sui: u64,
}

/// Represents pricing information for various operations in a dWallet.
/// Each operation is represented by its own `PricingPerOperation`:
/// - `dkg_first_round`: Pricing for the first round of distributed key generation.
/// - `dkg_second_round`: Pricing for the second round of distributed key generation.
/// - `re_encrypt_user_share`: Pricing for re-encrypting user shares.
/// - `ecdsa_presign`: Pricing for ECDSA presigning.
/// - `ecdsa_sign`: Pricing for ECDSA signing.
/// - `ecdsa_future_sign`: Pricing for ECDSA future signing.
/// - `ecdsa_sign_with_partial_user_signature`: Pricing for ECDSA signing with partial user signature.
public struct DWalletPricing2PcMpcSecp256K1 has key, store {
    id: UID,
    dkg_first_round: PricingPerOperation,
    dkg_second_round: PricingPerOperation,
    re_encrypt_user_share: PricingPerOperation,
    ecdsa_presign: PricingPerOperation,
    ecdsa_sign: PricingPerOperation,
    ecdsa_future_sign: PricingPerOperation,
    ecdsa_sign_with_partial_user_signature: PricingPerOperation,
}

/// Creates a new [`DWalletPricing2PcMpcSecp256K1`] object.
///
/// Initializes pricing data for each operation by providing values for the three pricing fields for each operation.
///
/// # Parameters
///
/// - **DKG First Round Pricing:**
///   - `dkg_first_round_consensus_validation_ika`: Consensus validation IKA price.
///   - `dkg_first_round_computation_ika`: Computation IKA price.
///   - `dkg_first_round_gas_fee_reimbursement_sui`: SUI reimbursement.
///
/// - **DKG Second Round Pricing:**
///   - `dkg_second_round_consensus_validation_ika`: Consensus validation IKA price.
///   - `dkg_second_round_computation_ika`: Computation IKA price.
///   - `dkg_second_round_gas_fee_reimbursement_sui`: SUI reimbursement.
///
/// - **Re-encrypt User Share Pricing:**
///   - `re_encrypt_consensus_validation_ika`: Consensus validation IKA price.
///   - `re_encrypt_computation_ika`: Computation IKA price.
///   - `re_encrypt_gas_fee_reimbursement_sui`: SUI reimbursement.
///
/// - **ECDSA Presign Pricing:**
///   - `ecdsa_presign_consensus_validation_ika`: Consensus validation IKA price.
///   - `ecdsa_presign_computation_ika`: Computation IKA price.
///   - `ecdsa_presign_gas_fee_reimbursement_sui`: SUI reimbursement.
///
/// - **ECDSA Sign Pricing:**
///   - `ecdsa_sign_consensus_validation_ika`: Consensus validation IKA price.
///   - `ecdsa_sign_computation_ika`: Computation IKA price.
///   - `ecdsa_sign_gas_fee_reimbursement_sui`: SUI reimbursement.
///
/// - **ECDSA Future Sign Pricing:**
///   - `ecdsa_future_sign_consensus_validation_ika`: Consensus validation IKA price.
///   - `ecdsa_future_sign_computation_ika`: Computation IKA price.
///   - `ecdsa_future_sign_gas_fee_reimbursement_sui`: SUI reimbursement.
///
/// - **ECDSA Sign with Partial User Signature Pricing:**
///   - `ecdsa_partial_sign_consensus_validation_ika`: Consensus validation IKA price.
///   - `ecdsa_partial_sign_computation_ika`: Computation IKA price.
///   - `ecdsa_partial_sign_gas_fee_reimbursement_sui`: SUI reimbursement.
///
/// - `ctx`: The transaction context.
///
/// # Returns
///
/// A newly created instance of `DWalletPricing2PcMpcSecp256K1`.
public fun create_dwallet_pricing_2pc_mpc_secp256k1(
    // DKG First Round Pricing
    dkg_first_round_consensus_validation_ika: u64,
    dkg_first_round_computation_ika: u64,
    dkg_first_round_gas_fee_reimbursement_sui: u64,

    // DKG Second Round Pricing
    dkg_second_round_consensus_validation_ika: u64,
    dkg_second_round_computation_ika: u64,
    dkg_second_round_gas_fee_reimbursement_sui: u64,

    // Re-encrypt User Share Pricing
    re_encrypt_consensus_validation_ika: u64,
    re_encrypt_computation_ika: u64,
    re_encrypt_gas_fee_reimbursement_sui: u64,

    // ECDSA Presign Pricing
    ecdsa_presign_consensus_validation_ika: u64,
    ecdsa_presign_computation_ika: u64,
    ecdsa_presign_gas_fee_reimbursement_sui: u64,

    // ECDSA Sign Pricing
    ecdsa_sign_consensus_validation_ika: u64,
    ecdsa_sign_computation_ika: u64,
    ecdsa_sign_gas_fee_reimbursement_sui: u64,

    // ECDSA Future Sign Pricing
    ecdsa_future_sign_consensus_validation_ika: u64,
    ecdsa_future_sign_computation_ika: u64,
    ecdsa_future_sign_gas_fee_reimbursement_sui: u64,

    // ECDSA Sign with Partial User Signature Pricing
    ecdsa_partial_sign_consensus_validation_ika: u64,
    ecdsa_partial_sign_computation_ika: u64,
    ecdsa_partial_sign_gas_fee_reimbursement_sui: u64,

    ctx: &mut TxContext
): DWalletPricing2PcMpcSecp256K1 {
    DWalletPricing2PcMpcSecp256K1 {
        id: object::new(ctx),
        dkg_first_round: PricingPerOperation {
            consensus_validation_ika: dkg_first_round_consensus_validation_ika,
            computation_ika: dkg_first_round_computation_ika,
            gas_fee_reimbursement_sui: dkg_first_round_gas_fee_reimbursement_sui,
        },
        dkg_second_round: PricingPerOperation {
            consensus_validation_ika: dkg_second_round_consensus_validation_ika,
            computation_ika: dkg_second_round_computation_ika,
            gas_fee_reimbursement_sui: dkg_second_round_gas_fee_reimbursement_sui,
        },
        re_encrypt_user_share: PricingPerOperation {
            consensus_validation_ika: re_encrypt_consensus_validation_ika,
            computation_ika: re_encrypt_computation_ika,
            gas_fee_reimbursement_sui: re_encrypt_gas_fee_reimbursement_sui,
        },
        ecdsa_presign: PricingPerOperation {
            consensus_validation_ika: ecdsa_presign_consensus_validation_ika,
            computation_ika: ecdsa_presign_computation_ika,
            gas_fee_reimbursement_sui: ecdsa_presign_gas_fee_reimbursement_sui,
        },
        ecdsa_sign: PricingPerOperation {
            consensus_validation_ika: ecdsa_sign_consensus_validation_ika,
            computation_ika: ecdsa_sign_computation_ika,
            gas_fee_reimbursement_sui: ecdsa_sign_gas_fee_reimbursement_sui,
        },
        ecdsa_future_sign: PricingPerOperation {
            consensus_validation_ika: ecdsa_future_sign_consensus_validation_ika,
            computation_ika: ecdsa_future_sign_computation_ika,
            gas_fee_reimbursement_sui: ecdsa_future_sign_gas_fee_reimbursement_sui,
        },
        ecdsa_sign_with_partial_user_signature: PricingPerOperation {
            consensus_validation_ika: ecdsa_partial_sign_consensus_validation_ika,
            computation_ika: ecdsa_partial_sign_computation_ika,
            gas_fee_reimbursement_sui: ecdsa_partial_sign_gas_fee_reimbursement_sui,
        },
    }
}


/// Returns zero `PricingPerOperation`.
public fun zero(): PricingPerOperation {
    PricingPerOperation {
        consensus_validation_ika: 0,
        computation_ika: 0,
        gas_fee_reimbursement_sui: 0,
    }
}

/// Returns `PricingPerOperation` for the DKG first round.
public fun dkg_first_round(self: &DWalletPricing2PcMpcSecp256K1): PricingPerOperation {
    self.dkg_first_round
}

/// Returns `PricingPerOperation` for the DKG second round.
public fun dkg_second_round(self: &DWalletPricing2PcMpcSecp256K1): PricingPerOperation {
    self.dkg_second_round
}

/// Returns `PricingPerOperation` for the re-encrypt user share.
public fun re_encrypt_user_share(self: &DWalletPricing2PcMpcSecp256K1): PricingPerOperation {
    self.re_encrypt_user_share
}

/// Returns `PricingPerOperation` for the ECDSA presign.
public fun ecdsa_presign(self: &DWalletPricing2PcMpcSecp256K1): PricingPerOperation {
    self.ecdsa_presign
}

/// Returns `PricingPerOperation` for the ECDSA sign.
public fun ecdsa_sign(self: &DWalletPricing2PcMpcSecp256K1): PricingPerOperation {
    self.ecdsa_sign
}

/// Returns `PricingPerOperation` for the ECDSA future sign.
public fun ecdsa_future_sign(self: &DWalletPricing2PcMpcSecp256K1): PricingPerOperation {
    self.ecdsa_future_sign
}

/// Returns `PricingPerOperation` for the ECDSA sign with partial user signature.
public fun ecdsa_sign_with_partial_user_signature(self: &DWalletPricing2PcMpcSecp256K1): PricingPerOperation {
    self.ecdsa_sign_with_partial_user_signature
}

/// Getter for the consensus_validation_ika field of a PricingPerOperation.
public fun consensus_validation_ika(self: &PricingPerOperation): u64 {
    self.consensus_validation_ika
}

/// Getter for the computation_ika field of a PricingPerOperation.
public fun computation_ika(self: &PricingPerOperation): u64 {
    self.computation_ika
}

/// Getter for the gas_fee_reimbursement_sui field of a PricingPerOperation.
public fun gas_fee_reimbursement_sui(self: &PricingPerOperation): u64 {
    self.gas_fee_reimbursement_sui
}