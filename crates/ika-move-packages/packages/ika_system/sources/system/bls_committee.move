// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module ika_system::bls_committee;

use sui::bls12381::{Self, G1, UncompressedG1};
use sui::group_ops::{Self, Element};
use sui::event;

public struct BlsCommitteeMember has store, copy, drop {
    validator_id: ID,
    protocol_pubkey: Element<UncompressedG1>
}

/// Represents the current committee in the system.
public struct BlsCommittee has store, copy, drop {
    members: vector<BlsCommitteeMember>,
    /// The aggregation of public keys for all members of the committee
    aggregated_protocol_pubkey: Element<G1>,
    quorum_threshold: u64,
    validity_threshold: u64,
}

/// Event emitted after verifying quorum of signature.
public struct CommitteeQuorumVerifiedEvent has copy, drop {
    epoch: u64,
    signer_count: u64,
}

const BLS_SIGNATURE_LEN: u64 = 96;

const EInvalidBitmap: u64 = 0;
// const ETotalPowerMismatch: u64 = 1;
// const EInvalidVotingPower: u64 = 4;

#[error]
const EInvalidSignatureLength: vector<u8> = b"The length of the provided bls signature is incorrect.";

#[error]
const EInvalidSignature: vector<u8> = b"Invalid certificate signature.";

#[error]
const ENotEnoughStake: vector<u8> = b"Not enough stake of signers for the bls signature.";

public(package) fun new_bls_committee_member(
    validator_id: ID,
    protocol_pubkey: Element<UncompressedG1>
): BlsCommitteeMember {
    BlsCommitteeMember {
        validator_id,
        protocol_pubkey,
    }
}

public(package) fun validator_id(member: &BlsCommitteeMember): ID {
    member.validator_id
}

/// Create a new committee from members.
/// Each member has equal voting power of 1.
/// Total voting power is equal to the number of members.
public(package) fun new_bls_committee(members: vector<BlsCommitteeMember>): BlsCommittee {
    // Compute the total aggregated key, e.g. the sum of all public keys in the committee.
    let aggregated_protocol_pubkey = bls12381::uncompressed_g1_to_g1(
        &bls12381::uncompressed_g1_sum(
            &members.map!(|member| member.protocol_pubkey),
        ),
    );


    let quorum_threshold = (2 * (members.length() / 3)) + 1;
    let validity_threshold = (members.length() / 3) + 1;

    BlsCommittee {
        members,
        aggregated_protocol_pubkey,
        quorum_threshold,
        validity_threshold,
    }
}

/// Creates an empty committee. Only relevant for init phase.
public(package) fun empty(): BlsCommittee {
    BlsCommittee {
        members: vector[],
        aggregated_protocol_pubkey: bls12381::g1_identity(),
        quorum_threshold: 0,
        validity_threshold: 0,
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

/// Return the total voting power (number of members in the committee)
public fun total_voting_power(self: &BlsCommittee): u64 {
    self.members.length()
}

/// Return the quorum threshold (2n/3 + 1) calculated on demand
public fun quorum_threshold(self: &BlsCommittee): u64 {
    self.quorum_threshold
}

/// Return the validity threshold (n/3 + 1) calculated on demand
public fun validity_threshold(self: &BlsCommittee): u64 {
    self.validity_threshold
}

/// Verify an aggregate BLS signature is a certificate in the epoch, and return
/// the type of certificate and the bytes certified. The `signers` vector is
/// an increasing list of indexes into the `committee` vector.
/// If there is no certificate, the function aborts.
public(package) fun verify_certificate(
    self: &BlsCommittee,
    epoch: u64,
    signature: &vector<u8>,
    signers_bitmap: &vector<u8>,
    intent_bytes: &vector<u8>,
) {
    assert!(signature.length() == BLS_SIGNATURE_LEN, EInvalidSignatureLength);
    let members = &self.members;

    // Count non-signers instead of summing their voting powers
    let mut non_signer_count = 0;
    let mut non_signer_public_keys: vector<Element<UncompressedG1>> = vector::empty();
    let mut offset: u64 = 0;
    let n_members = members.length();
    let max_bitmap_len_bytes = n_members.divide_and_round_up(8);

    // The signers bitmap must not be longer than necessary to hold all members.
    // It may be shorter, in which case the excluded members are treated as non-signers.
    assert!(signers_bitmap.length() == max_bitmap_len_bytes, EInvalidBitmap);

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
                non_signer_count = non_signer_count + 1;
                non_signer_public_keys.push_back(member.protocol_pubkey);
            };
        });
        offset = offset + 8;
    });

    // Compute the aggregate voting power as the number of signers.
    let signer_count = n_members - non_signer_count;

    assert!(is_quorum_threshold(self, signer_count), ENotEnoughStake);

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
        signer_count,
    });
}

/// Returns true if the voting power meets or exceeds the quorum threshold.
/// Calculates the threshold on demand based on total voting power.
public(package) fun is_quorum_threshold(self: &BlsCommittee, signer_count: u64): bool {
    signer_count >= self.quorum_threshold
}

/// Returns true if the voting power meets or exceeds the validity threshold.
/// Calculates the threshold on demand based on total voting power.
public(package) fun is_validity_threshold(self: &BlsCommittee, signer_count: u64): bool {
    signer_count >= self.validity_threshold
}
