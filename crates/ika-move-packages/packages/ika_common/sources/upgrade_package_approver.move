// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module ika_common::upgrade_package_approver;

use ika_common::system_object_cap::SystemObjectCap;
use std::string::String;
use std::type_name;

// === Structs ===

/// This struct is an Hot-Potato that is passed around during upgrade package approval.
public struct UpgradePackageApprover {
    remaining_witnesses_to_approve: vector<String>,
    new_package_id: ID,
    old_package_id: ID,
}

// === Errors ===

/// Witness is not in the approver.
const EWitnessIsNotInApprover: u64 = 0;

// === Package Functions ===

public fun create(
    remaining_witnesses_to_approve: vector<String>,
    new_package_id: ID,
    old_package_id: ID,
    _: &SystemObjectCap,
): UpgradePackageApprover {
    UpgradePackageApprover {
        remaining_witnesses_to_approve,
        new_package_id,
        old_package_id,
    }
}

public fun assert_all_witnesses_approved(self: &UpgradePackageApprover) {
    assert!(self.remaining_witnesses_to_approve.is_empty(), EWitnessIsNotInApprover);
}

public fun new_package_id(self: &UpgradePackageApprover): ID {
    self.new_package_id
}

public fun old_package_id(self: &UpgradePackageApprover): ID {
    self.old_package_id
}

public fun destroy(self: UpgradePackageApprover, _: &SystemObjectCap) {
    let UpgradePackageApprover { .. } = self;
}

// === Public Functions ===

public fun approve_upgrade_package_by_witness<Witness: drop>(
    upgrade_package_approver: &mut UpgradePackageApprover,
    _: Witness,
) {
    let witness_type = type_name::get_with_original_ids<Witness>();
    let witness_type_name = witness_type.into_string().to_string();
    let (is_found, index) = upgrade_package_approver
        .remaining_witnesses_to_approve
        .index_of(&witness_type_name);
    assert!(is_found, EWitnessIsNotInApprover);
    upgrade_package_approver.remaining_witnesses_to_approve.remove(index);
}
