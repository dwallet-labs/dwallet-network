// Copyright (c) dWallet Labs Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

/// This module provides structures and functions for managing pricing information for a dWallet.
/// Each operation (e.g., DKG, re-encrypt user share, ECDSA presign, etc.) has its own pricing data,
/// represented by a `PricingPerOperation`. Each `PricingPerOperation` holds three values:
///   - **consensus_validation_ika**: The consensus validation IKA price.
///   - **computation_ika**: The computation_ika IKA price.
///   - **gas_fee_reimbursement_sui**: The SUI reimbursement.
///
/// The main struct, `DWalletPricing`, now holds one `PricingPerOperation` per operation.
/// The DKG operation is split into two separate rounds:
///   - `dkg_first_round`
///   - `dkg_second_round`
module ika_system::dwallet_pricing;

use sui::priority_queue::{Self, PriorityQueue};
use ika_system::bls_committee::BlsCommittee;
use sui::vec_map::{Self, VecMap};


/// Holds pricing information for a dWallet.
/// The vector is indexed by the curve and signature algorithm and protocol.
public struct DWalletPricing has copy, drop, store {
    /// The pricing for each curve and signature algorithm and protocol.
    /// The first key is the curve, the second is the signature algorithm, the third is the protocol.
    pricing_map: VecMap<DWalletPricingKey, DWalletPricingValue>,
}

public struct DWalletPricingKey has copy, drop, store {
    curve: u32,
    signature_algorithm: Option<u32>,
    protocol: u32,
}

/// Holds pricing information for a single operation.
/// The fields are ordered so that the consensus validation price is first.
public struct DWalletPricingValue has copy, drop, store {
    consensus_validation_ika: u64,
    computation_ika: u64,
    gas_fee_reimbursement_sui: u64,
    gas_fee_reimbursement_sui_for_system_calls: u64,
}

public struct DWalletPricingCalculationVotes has copy, drop, store {
    bls_committee: BlsCommittee,
    default_pricing: DWalletPricing,
    working_pricing: DWalletPricing,
}

/// Creates a new [`DWalletPricing`] object.
///
/// Initializes the table with the given pricing values for each operation.
///
/// # Parameters
///
/// - `ctx`: The transaction context.
///
/// # Returns
///
/// A newly created instance of `DWalletPricing`.
public fun empty(): DWalletPricing {
    DWalletPricing {
        pricing_map: vec_map::empty(),
    }
}

/// Inserts pricing information for a specific operation into the [`DWalletPricing`] table.
///
/// # Parameters
///
/// - `self`: The [`DWalletPricing`] object.
/// - `key`: The key for the operation.
/// - `value`: The pricing information for the operation.
///
/// # Returns
///
/// The [`DWalletPricing`] object.
public fun insert_or_update_dwallet_pricing(self: &mut DWalletPricing, curve: u32, signature_algorithm: Option<u32>, protocol: u32, consensus_validation_ika: u64, computation_ika: u64, gas_fee_reimbursement_sui: u64, gas_fee_reimbursement_sui_for_system_calls: u64) {
    self.insert_or_update_dwallet_pricing_value(curve, signature_algorithm, protocol, DWalletPricingValue {
        consensus_validation_ika,
        computation_ika,
        gas_fee_reimbursement_sui,
        gas_fee_reimbursement_sui_for_system_calls,
    })
}

fun insert_or_update_dwallet_pricing_value(self: &mut DWalletPricing, curve: u32, signature_algorithm: Option<u32>, protocol: u32, value: DWalletPricingValue) {
    let key = DWalletPricingKey {
        curve,
        signature_algorithm,
        protocol,
    };
    if(self.pricing_map.contains(&key)) {
        let existing_value = &mut self.pricing_map[&key];
        *existing_value = value;
    } else {
        self.pricing_map.insert(key, value);
    };
}

/// Returns the pricing information for a specific operation from the [`DWalletPricing`] table.
///
/// # Parameters
///
/// - `self`: The [`DWalletPricing`] object.
/// - `key`: The key for the operation.
///
/// # Returns
///
/// The pricing information for the operation.
public(package) fun try_get_dwallet_pricing_value(self: &DWalletPricing, curve: u32, signature_algorithm: Option<u32>, protocol: u32): Option<DWalletPricingValue> {
    let key = DWalletPricingKey {
        curve,
        signature_algorithm,
        protocol,
    };
    self.pricing_map.try_get(&key)
}

/// Getter for the consensus_validation_ika field of a DWalletPricingValue.
public fun consensus_validation_ika(self: &DWalletPricingValue): u64 {
    self.consensus_validation_ika
}

/// Getter for the computation_ika field of a DWalletPricingValue.
public fun computation_ika(self: &DWalletPricingValue): u64 {
    self.computation_ika
}

/// Getter for the gas_fee_reimbursement_sui field of a DWalletPricingValue.
public fun gas_fee_reimbursement_sui(self: &DWalletPricingValue): u64 {
    self.gas_fee_reimbursement_sui
}

/// Getter for the gas_fee_reimbursement_sui_for_system_calls field of a DWalletPricingValue.
public fun gas_fee_reimbursement_sui_for_system_calls(self: &DWalletPricingValue): u64 {
    self.gas_fee_reimbursement_sui_for_system_calls
}

public(package) fun new_pricing_calculation(bls_committee: BlsCommittee, default_pricing: DWalletPricing): DWalletPricingCalculationVotes {
    DWalletPricingCalculationVotes {
        bls_committee,
        default_pricing,
        working_pricing: empty(),
    }
}

public(package) fun committee_members_for_pricing_calculation_votes(calculation: &DWalletPricingCalculationVotes): vector<ID> {
    calculation.bls_committee.members().map_ref!(|member| {
        member.validator_id()
    })
}

public(package) fun calculate_pricing_quorum_below(calculation: &mut DWalletPricingCalculationVotes, pricing: vector<DWalletPricing>, curve: u32, signature_algorithm: Option<u32>, protocol: u32) {
    let mut values = vector[];
    pricing.do_ref!(|pricing| {
        let value = pricing.try_get_dwallet_pricing_value(curve, signature_algorithm, protocol);
        values.push_back(value.get_with_default(calculation.default_pricing.try_get_dwallet_pricing_value(curve, signature_algorithm, protocol).extract()));
    });
    let value = pricing_value_quorum_below(calculation.bls_committee, values);
    calculation.working_pricing.insert_or_update_dwallet_pricing_value(curve, signature_algorithm, protocol, value);
}

public(package) fun pricing_value_quorum_below(bls_committee: BlsCommittee, values: vector<DWalletPricingValue>): DWalletPricingValue {
    let mut consensus_validation_ika = priority_queue::new(vector[]);
    let mut computation_ika = priority_queue::new(vector[]);
    let mut gas_fee_reimbursement_sui = priority_queue::new(vector[]);
    let mut gas_fee_reimbursement_sui_for_system_calls = priority_queue::new(vector[]);
    values.do_ref!(|value| {
        consensus_validation_ika.insert(value.consensus_validation_ika(), 1);
        computation_ika.insert(value.computation_ika(), 1);
        gas_fee_reimbursement_sui.insert(value.gas_fee_reimbursement_sui(), 1);
        gas_fee_reimbursement_sui_for_system_calls.insert(value.gas_fee_reimbursement_sui_for_system_calls(), 1);
    });
    let consensus_validation_ika_quorum_below = quorum_below(bls_committee, &mut consensus_validation_ika);
    let computation_ika_quorum_below = quorum_below(bls_committee, &mut computation_ika);
    let gas_fee_reimbursement_sui_quorum_below = quorum_below(bls_committee, &mut gas_fee_reimbursement_sui);
    let gas_fee_reimbursement_sui_for_system_calls_quorum_below = quorum_below(bls_committee, &mut gas_fee_reimbursement_sui_for_system_calls);
    DWalletPricingValue {
        consensus_validation_ika: consensus_validation_ika_quorum_below,
        computation_ika: computation_ika_quorum_below,
        gas_fee_reimbursement_sui: gas_fee_reimbursement_sui_quorum_below,
        gas_fee_reimbursement_sui_for_system_calls: gas_fee_reimbursement_sui_for_system_calls_quorum_below,
    }
}

/// Take the lowest value, s.t. a quorum  (2f + 1) voted for a value lower or equal to this.
fun quorum_below(bls_committee: BlsCommittee, vote_queue: &mut PriorityQueue<u64>): u64 {
    let mut sum_votes = bls_committee.total_voting_power();
    // We have a quorum initially, so we remove nodes until doing so breaks the quorum.
    // The value at that point is the minimum value with support from a quorum.
    loop {
        let (value, votes) = vote_queue.pop_max();
        sum_votes = sum_votes - votes;
        if (!bls_committee.is_quorum_threshold(sum_votes)) {
            return value
        };
    }
}

public(package) fun is_calculation_completed(calculation: &DWalletPricingCalculationVotes): bool {
    let mut i = 0;
    let (keys, _) = calculation.default_pricing.pricing_map.into_keys_values();
    while (i < keys.length()) {
        let key = keys[i];
        if(calculation.working_pricing.pricing_map.try_get(&key).is_none()) {
            return false
        };
        i = i + 1;
    };
    true
}

public(package) fun calculated_pricing(calculation: &DWalletPricingCalculationVotes): DWalletPricing {
    calculation.working_pricing
}
