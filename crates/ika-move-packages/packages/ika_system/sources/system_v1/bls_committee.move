// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module ika_system::bls_committee;

use sui::bls12381::{Self, G1, UncompressedG1};
use sui::group_ops::{Self, Element};
use sui::event;

public struct BlsCommitteeMember has store, copy, drop {
    validator_id: ID,
    protocol_pubkey: Element<UncompressedG1>,
    voting_power: u64,
    stake: u64,
}

/// Represents the current committee in the system.
public struct BlsCommittee has store, copy, drop {
    members: vector<BlsCommitteeMember>,
    /// The aggregation of public keys for all members of the committee
    aggregated_protocol_pubkey: Element<G1>,
}

/// Event emitted after verifing quorum of signature.
public struct CommitteeQuorumVerifiedEvent has copy, drop {
    epoch: u64,
    total_signers_stake: u64,
}

/// Set total_voting_power as 10_000 by convention. Individual voting powers can be interpreted
/// as easily understandable basis points (e.g., voting_power: 100 = 1%, voting_power: 1 = 0.01%) rather than
/// opaque quantities whose meaning changes from epoch to epoch as the total amount staked shifts.
/// Fixing the total voting power allows clients to hardcode the quorum threshold and total_voting power rather
/// than recomputing these.
const TOTAL_VOTING_POWER: u64 = 4;

/// Quorum threshold for our fixed voting power - any message signed by this much voting power can be trusted
/// up to BFT assumptions
const QUORUM_THRESHOLD: u64 = 3;

// Cap voting power of an individual validator at 10%.
const MAX_VOTING_POWER: u64 = 1;

const BLS_SIGNATURE_LEN: u64 = 96;

const EInvalidBitmap: u64 = 0;
const ETotalPowerMismatch: u64 = 1;
const ERelativePowerMismatch: u64 = 2;
const EVotingPowerOverThreshold: u64 = 3;
const EInvalidVotingPower: u64 = 4;

#[error]
const EInvalidSignatureLength: vector<u8> = b"The length of the provided bls signature is incorrect.";

#[error]
const EInvalidSignature: vector<u8> = b"Invalid certificate signature.";

#[error]
const ENotEnoughStake: vector<u8> = b"Not enough stake of signers for the bls signature.";

public(package) fun new_bls_committee_member(
    validator_id: ID,
    protocol_pubkey: Element<UncompressedG1>,
    stake: u64,
): BlsCommitteeMember {
    BlsCommitteeMember {
        validator_id,
        protocol_pubkey,
        voting_power: 0,
        stake,
    }
}

public(package) fun voting_power(member: &BlsCommitteeMember): u64 {
    member.voting_power
}

public(package) fun validator_id(member: &BlsCommitteeMember): ID {
    member.validator_id
}

/// Create a new committee from members.
/// Each member's voting power is initialized using their stake. We then attempt to cap their voting power
/// at `MAX_VOTING_POWER`. If `MAX_VOTING_POWER` is not a feasible cap, we pick the lowest possible cap.
public(package) fun new_bls_committee(mut members: vector<BlsCommitteeMember>): BlsCommittee {
    // If threshold_pct is too small, it's possible that even when all members reach the threshold we still don't
    // have 100%. So we bound the threshold_pct to be always enough to find a solution.
    let threshold = TOTAL_VOTING_POWER.min(MAX_VOTING_POWER.max(TOTAL_VOTING_POWER.divide_and_round_up(members.length())));
    let remaining_power = init_voting_power_info(&mut members, threshold);
    adjust_voting_power(&mut members, threshold, remaining_power);
    check_invariants(&members);

    // Compute the total aggregated key, e.g. the sum of all public keys in the committee.
    let aggregated_protocol_pubkey = bls12381::uncompressed_g1_to_g1(
        &bls12381::uncompressed_g1_sum(
            &members.map!(|member| member.protocol_pubkey),
        ),
    );

    BlsCommittee {
        members,
        aggregated_protocol_pubkey
    }
}

/// Creates an empty committee. Only relevant for init phase.
public(package) fun empty(): BlsCommittee {
    BlsCommittee {
        members: vector[],
        aggregated_protocol_pubkey: bls12381::g1_identity()
    }
}

public(package) fun members(self: &BlsCommittee): &vector<BlsCommitteeMember> {
    &self.members
}

public(package) fun validator_ids(self: &BlsCommittee): vector<ID> {
    self.members().map_ref!(|m| m.validator_id())
}

public(package) fun contains(self: &BlsCommittee, validator_id: &ID): bool {
    self.members().any!(|m| m.validator_id() == validator_id)
}

/// Create the initial voting power of each member, set using their stake, but capped using threshold.
/// We also perform insertion sort while creating the voting power list, by maintaining the list in
/// descending order using voting power.
/// Anything beyond the threshold is added to the remaining_power, which is also returned.
fun init_voting_power_info(
    members: &mut vector<BlsCommitteeMember>,
    threshold: u64,
): u64 {
    let total_stake = total_stake(members);
    let mut i = 0;
    let len = members.length();
    let mut total_power = 0;
    while (i < len) {
        let m = &mut members[i];
        let stake = m.stake;
        let adjusted_stake = stake as u128 * (TOTAL_VOTING_POWER as u128) / (total_stake as u128);
        let voting_power = (adjusted_stake as u64).min(threshold);
        m.voting_power = voting_power;
        total_power = total_power + voting_power;
        i = i + 1;
    };
    TOTAL_VOTING_POWER - total_power
}

/// Sum up the total stake of all members.
fun total_stake(members: &vector<BlsCommitteeMember>): u64 {
    let mut i = 0;
    let len = members.length();
    let mut total_stake = 0;
    while (i < len) {
        total_stake = total_stake + members[i].stake;
        i = i + 1;
    };
    total_stake
}

/// Distribute remaining_power to members that are not capped at threshold.
fun adjust_voting_power(
    members: &mut vector<BlsCommitteeMember>,
    threshold: u64,
    mut remaining_power: u64,
) {
    let mut i = 0;
    let len = members.length();
    while (i < len && remaining_power > 0) {
        let v = &mut members[i];
        // planned is the amount of extra power we want to distribute to this member.
        let planned = remaining_power.divide_and_round_up(len - i);
        // target is the targeting power this member will reach, capped by threshold.
        let target = threshold.min(v.voting_power + planned);
        // actual is the actual amount of power we will be distributing to this member.
        let actual = remaining_power.min(target - v.voting_power);
        v.voting_power = v.voting_power + actual;
        assert!(v.voting_power <= threshold, EVotingPowerOverThreshold);
        remaining_power = remaining_power - actual;
        i = i + 1;
    };
    assert!(remaining_power == 0, ETotalPowerMismatch);
}

/// Check a few invariants that must hold after setting the voting power.
fun check_invariants(members: &vector<BlsCommitteeMember>,) {
    // First check that the total voting power must be TOTAL_VOTING_POWER.
    let mut i = 0;
    let len = members.length();
    let mut total = 0;
    while (i < len) {
        let voting_power = members[i].voting_power;
        assert!(voting_power > 0, EInvalidVotingPower);
        total = total + voting_power;
        i = i + 1;
    };
    assert!(total == TOTAL_VOTING_POWER, ETotalPowerMismatch);

    // Second check that if member A's stake is larger than B's stake, A's voting power must be no less
    // than B's voting power; similarly, if A's stake is less than B's stake, A's voting power must be no larger
    // than B's voting power.
    let mut a = 0;
    while (a < len) {
        let mut b = a + 1;
        while (b < len) {
            let member_a = &members[a];
            let member_b = &members[b];
            let stake_a = member_a.stake;
            let stake_b = member_b.stake;
            let power_a = member_a.voting_power;
            let power_b = member_b.voting_power;
            if (stake_a > stake_b) {
                assert!(power_a >= power_b, ERelativePowerMismatch);
            };
            if (stake_a < stake_b) {
                assert!(power_a <= power_b, ERelativePowerMismatch);
            };
            b = b + 1;
        };
        a = a + 1;
    }
}

/// Return the (constant) total voting power
public fun total_voting_power(): u64 {
    TOTAL_VOTING_POWER
}

/// Return the (constant) quorum threshold
public fun quorum_threshold(): u64 {
    QUORUM_THRESHOLD
}

/// Verify an aggregate BLS signature is a certificate in the epoch, and return
/// the type of certificate and the bytes certified. The `signers` vector is
/// an increasing list of indexes into the `committee` vector.
/// If there is a certificate, the function returns the total stake. Otherwise, it aborts.
public(package) fun verify_certificate(
    self: &BlsCommittee,
    epoch: u64,
    signature: &vector<u8>,
    signers_bitmap: &vector<u8>,
    intent_bytes: &vector<u8>,
) {
    assert!(signature.length() == BLS_SIGNATURE_LEN, EInvalidSignatureLength);
    let members = &self.members;

    // Use the signers_bitmap to construct the key and the voting_power.

    let mut non_signer_aggregate_voting_power = 0;
    let mut non_signer_public_keys: vector<Element<UncompressedG1>> = vector::empty();
    let mut offset: u64 = 0;
    let n_members = members.length();
    let max_bitmap_len_bytes = n_members.divide_and_round_up(8);

    // The signers bitmap must not be longer than necessary to hold all members.
    // It may be shorter, in which case the excluded members are treated as non-signers.
    assert!(signers_bitmap.length() <= max_bitmap_len_bytes, EInvalidBitmap);

    // Iterate over the signers bitmap and check if each member is a signer.
    max_bitmap_len_bytes.do!(|i| {
        // Get the current byte or 0 if we've reached the end of the bitmap.
        let byte = if (i < signers_bitmap.length()) {
            signers_bitmap[i]
        } else {
            0
        };

        (8u8).do!(|i| {
            let index = offset + (i as u64);
            let is_signer = (byte >> i) & 1 == 1;

            // If the index is out of bounds, the bit must be 0 to ensure
            // uniqueness of the signers_bitmap.
            if (index >= n_members) {
                assert!(!is_signer, EInvalidBitmap);
                return
            };

            // There will be fewer non-signers than signers, so we handle
            // non-signers here.
            if (!is_signer) {
                let member = &members[index];
                non_signer_aggregate_voting_power = non_signer_aggregate_voting_power + member.voting_power;
                non_signer_public_keys.push_back(member.protocol_pubkey);
            };
        });
        offset = offset + 8;
    });

    // Compute the aggregate voting_power as the difference between the total voting power
    // and the total voting power of the non-signers.
    let aggregate_voting_power = TOTAL_VOTING_POWER - non_signer_aggregate_voting_power;

    assert!(verify_quorum(aggregate_voting_power), ENotEnoughStake);


    // Compute the aggregate public key as the difference between the total
    // aggregated key and the sum of the non-signer public keys.
    let aggregate_key = bls12381::g1_sub(
        &self.aggregated_protocol_pubkey,
        &bls12381::uncompressed_g1_to_g1(
            &bls12381::uncompressed_g1_sum(&non_signer_public_keys),
        ),
    );

    // Verify the signature
    let pub_key_bytes = group_ops::bytes(&aggregate_key);
    assert!(
        bls12381::bls12381_min_pk_verify(
            signature,
            pub_key_bytes,
            intent_bytes,
        ),
        EInvalidSignature,
    );

    event::emit(CommitteeQuorumVerifiedEvent {
        epoch,
        total_signers_stake: aggregate_voting_power,
    });
}


/// Returns true if the voting power is more than the aggregate voting power of quorum members of a committee.
public(package) fun verify_quorum(aggregate_voting_power: u64): bool {
    aggregate_voting_power >= QUORUM_THRESHOLD
}
