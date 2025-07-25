---
title: Module `ika_common::bls_committee`
---



-  [Struct `BlsCommitteeMember`](#ika_common_bls_committee_BlsCommitteeMember)
-  [Struct `BlsCommittee`](#ika_common_bls_committee_BlsCommittee)
-  [Struct `CommitteeQuorumVerifiedEvent`](#ika_common_bls_committee_CommitteeQuorumVerifiedEvent)
-  [Constants](#@Constants_0)
-  [Function `total_voting_power`](#ika_common_bls_committee_total_voting_power)
-  [Function `quorum_threshold`](#ika_common_bls_committee_quorum_threshold)
-  [Function `validity_threshold`](#ika_common_bls_committee_validity_threshold)
-  [Function `new_bls_committee_member`](#ika_common_bls_committee_new_bls_committee_member)
-  [Function `validator_id`](#ika_common_bls_committee_validator_id)
-  [Function `new_bls_committee`](#ika_common_bls_committee_new_bls_committee)
-  [Function `empty`](#ika_common_bls_committee_empty)
-  [Function `members`](#ika_common_bls_committee_members)
-  [Function `validator_ids`](#ika_common_bls_committee_validator_ids)
-  [Function `contains`](#ika_common_bls_committee_contains)
-  [Function `verify_certificate`](#ika_common_bls_committee_verify_certificate)
-  [Function `is_quorum_threshold`](#ika_common_bls_committee_is_quorum_threshold)
-  [Function `is_validity_threshold`](#ika_common_bls_committee_is_validity_threshold)


<pre><code><b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/u64.md#std_u64">std::u64</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../sui/address.md#sui_address">sui::address</a>;
<b>use</b> <a href="../sui/bcs.md#sui_bcs">sui::bcs</a>;
<b>use</b> <a href="../sui/bls12381.md#sui_bls12381">sui::bls12381</a>;
<b>use</b> <a href="../sui/event.md#sui_event">sui::event</a>;
<b>use</b> <a href="../sui/group_ops.md#sui_group_ops">sui::group_ops</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
</code></pre>



<a name="ika_common_bls_committee_BlsCommitteeMember"></a>

## Struct `BlsCommitteeMember`

Represents a single member of the BLS committee with their validator ID and protocol public key


<pre><code><b>public</b> <b>struct</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommitteeMember">BlsCommitteeMember</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_common/bls_committee.md#ika_common_bls_committee_validator_id">validator_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>protocol_pubkey: <a href="../sui/group_ops.md#sui_group_ops_Element">sui::group_ops::Element</a>&lt;<a href="../sui/bls12381.md#sui_bls12381_UncompressedG1">sui::bls12381::UncompressedG1</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="ika_common_bls_committee_BlsCommittee"></a>

## Struct `BlsCommittee`

Represents the current committee in the system with aggregated public keys and voting thresholds


<pre><code><b>public</b> <b>struct</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommittee">BlsCommittee</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_common/bls_committee.md#ika_common_bls_committee_members">members</a>: vector&lt;<a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommitteeMember">ika_common::bls_committee::BlsCommitteeMember</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>aggregated_protocol_pubkey: <a href="../sui/group_ops.md#sui_group_ops_Element">sui::group_ops::Element</a>&lt;<a href="../sui/bls12381.md#sui_bls12381_G1">sui::bls12381::G1</a>&gt;</code>
</dt>
<dd>
 The aggregation of public keys for all members of the committee
</dd>
<dt>
<code><a href="../ika_common/bls_committee.md#ika_common_bls_committee_quorum_threshold">quorum_threshold</a>: u64</code>
</dt>
<dd>
 Minimum signatures required for quorum (2n/3 + 1)
</dd>
<dt>
<code><a href="../ika_common/bls_committee.md#ika_common_bls_committee_validity_threshold">validity_threshold</a>: u64</code>
</dt>
<dd>
 Minimum signatures required for validity (n/3 + 1)
</dd>
</dl>


</details>

<a name="ika_common_bls_committee_CommitteeQuorumVerifiedEvent"></a>

## Struct `CommitteeQuorumVerifiedEvent`

Event emitted after verifying quorum of signature


<pre><code><b>public</b> <b>struct</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_CommitteeQuorumVerifiedEvent">CommitteeQuorumVerifiedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>signer_count: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="ika_common_bls_committee_BLS_SIGNATURE_LEN"></a>



<pre><code><b>const</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_BLS_SIGNATURE_LEN">BLS_SIGNATURE_LEN</a>: u64 = 96;
</code></pre>



<a name="ika_common_bls_committee_EInvalidBitmap"></a>

The signers bitmap is invalid.


<pre><code><b>const</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_EInvalidBitmap">EInvalidBitmap</a>: u64 = 0;
</code></pre>



<a name="ika_common_bls_committee_EInvalidSignatureLength"></a>

The length of the provided bls signature is incorrect.


<pre><code><b>const</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_EInvalidSignatureLength">EInvalidSignatureLength</a>: u64 = 1;
</code></pre>



<a name="ika_common_bls_committee_EInvalidSignature"></a>

Invalid certificate signature.


<pre><code><b>const</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_EInvalidSignature">EInvalidSignature</a>: u64 = 2;
</code></pre>



<a name="ika_common_bls_committee_ENotEnoughStake"></a>

Not enough stake of signers for the bls signature.


<pre><code><b>const</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_ENotEnoughStake">ENotEnoughStake</a>: u64 = 3;
</code></pre>



<a name="ika_common_bls_committee_total_voting_power"></a>

## Function `total_voting_power`

Returns the total voting power (number of members in the committee)


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_total_voting_power">total_voting_power</a>(self: &<a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommittee">ika_common::bls_committee::BlsCommittee</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_total_voting_power">total_voting_power</a>(self: &<a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommittee">BlsCommittee</a>): u64 {
    self.<a href="../ika_common/bls_committee.md#ika_common_bls_committee_members">members</a>.length()
}
</code></pre>



</details>

<a name="ika_common_bls_committee_quorum_threshold"></a>

## Function `quorum_threshold`

Returns the quorum threshold (2n/3 + 1) for the committee


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_quorum_threshold">quorum_threshold</a>(self: &<a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommittee">ika_common::bls_committee::BlsCommittee</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_quorum_threshold">quorum_threshold</a>(self: &<a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommittee">BlsCommittee</a>): u64 {
    self.<a href="../ika_common/bls_committee.md#ika_common_bls_committee_quorum_threshold">quorum_threshold</a>
}
</code></pre>



</details>

<a name="ika_common_bls_committee_validity_threshold"></a>

## Function `validity_threshold`

Returns the validity threshold (n/3 + 1) for the committee


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_validity_threshold">validity_threshold</a>(self: &<a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommittee">ika_common::bls_committee::BlsCommittee</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_validity_threshold">validity_threshold</a>(self: &<a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommittee">BlsCommittee</a>): u64 {
    self.<a href="../ika_common/bls_committee.md#ika_common_bls_committee_validity_threshold">validity_threshold</a>
}
</code></pre>



</details>

<a name="ika_common_bls_committee_new_bls_committee_member"></a>

## Function `new_bls_committee_member`

Creates a new BLS committee member with the given validator ID and protocol public key


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_new_bls_committee_member">new_bls_committee_member</a>(<a href="../ika_common/bls_committee.md#ika_common_bls_committee_validator_id">validator_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, protocol_pubkey: <a href="../sui/group_ops.md#sui_group_ops_Element">sui::group_ops::Element</a>&lt;<a href="../sui/bls12381.md#sui_bls12381_UncompressedG1">sui::bls12381::UncompressedG1</a>&gt;): <a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommitteeMember">ika_common::bls_committee::BlsCommitteeMember</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_new_bls_committee_member">new_bls_committee_member</a>(
    <a href="../ika_common/bls_committee.md#ika_common_bls_committee_validator_id">validator_id</a>: ID,
    protocol_pubkey: Element&lt;UncompressedG1&gt;
): <a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommitteeMember">BlsCommitteeMember</a> {
    <a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommitteeMember">BlsCommitteeMember</a> {
        <a href="../ika_common/bls_committee.md#ika_common_bls_committee_validator_id">validator_id</a>,
        protocol_pubkey,
    }
}
</code></pre>



</details>

<a name="ika_common_bls_committee_validator_id"></a>

## Function `validator_id`

Returns the validator ID of the committee member


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_validator_id">validator_id</a>(member: &<a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommitteeMember">ika_common::bls_committee::BlsCommitteeMember</a>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_validator_id">validator_id</a>(member: &<a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommitteeMember">BlsCommitteeMember</a>): ID {
    member.<a href="../ika_common/bls_committee.md#ika_common_bls_committee_validator_id">validator_id</a>
}
</code></pre>



</details>

<a name="ika_common_bls_committee_new_bls_committee"></a>

## Function `new_bls_committee`

Creates a new BLS committee from a vector of members
Each member has equal voting power of 1, total voting power equals number of members
Calculates quorum threshold (2n/3 + 1) and validity threshold (n/3 + 1)


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_new_bls_committee">new_bls_committee</a>(<a href="../ika_common/bls_committee.md#ika_common_bls_committee_members">members</a>: vector&lt;<a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommitteeMember">ika_common::bls_committee::BlsCommitteeMember</a>&gt;): <a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommittee">ika_common::bls_committee::BlsCommittee</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_new_bls_committee">new_bls_committee</a>(<a href="../ika_common/bls_committee.md#ika_common_bls_committee_members">members</a>: vector&lt;<a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommitteeMember">BlsCommitteeMember</a>&gt;): <a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommittee">BlsCommittee</a> {
    // Compute the total aggregated key, e.g. the sum of all <b>public</b> keys in the committee
    <b>let</b> aggregated_protocol_pubkey = bls12381::uncompressed_g1_to_g1(
        &bls12381::uncompressed_g1_sum(
            &<a href="../ika_common/bls_committee.md#ika_common_bls_committee_members">members</a>.map!(|member| member.protocol_pubkey),
        ),
    );
    <b>let</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_quorum_threshold">quorum_threshold</a> = (2 * (<a href="../ika_common/bls_committee.md#ika_common_bls_committee_members">members</a>.length() / 3)) + 1;
    <b>let</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_validity_threshold">validity_threshold</a> = (<a href="../ika_common/bls_committee.md#ika_common_bls_committee_members">members</a>.length() / 3) + 1;
    <a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommittee">BlsCommittee</a> {
        <a href="../ika_common/bls_committee.md#ika_common_bls_committee_members">members</a>,
        aggregated_protocol_pubkey,
        <a href="../ika_common/bls_committee.md#ika_common_bls_committee_quorum_threshold">quorum_threshold</a>,
        <a href="../ika_common/bls_committee.md#ika_common_bls_committee_validity_threshold">validity_threshold</a>,
    }
}
</code></pre>



</details>

<a name="ika_common_bls_committee_empty"></a>

## Function `empty`

Creates an empty committee with zero thresholds
Only relevant for initialization phase


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_empty">empty</a>(): <a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommittee">ika_common::bls_committee::BlsCommittee</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_empty">empty</a>(): <a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommittee">BlsCommittee</a> {
    <a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommittee">BlsCommittee</a> {
        <a href="../ika_common/bls_committee.md#ika_common_bls_committee_members">members</a>: vector[],
        aggregated_protocol_pubkey: bls12381::g1_identity(),
        <a href="../ika_common/bls_committee.md#ika_common_bls_committee_quorum_threshold">quorum_threshold</a>: 0,
        <a href="../ika_common/bls_committee.md#ika_common_bls_committee_validity_threshold">validity_threshold</a>: 0,
    }
}
</code></pre>



</details>

<a name="ika_common_bls_committee_members"></a>

## Function `members`

Returns an immutable reference to committee members


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_members">members</a>(self: &<a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommittee">ika_common::bls_committee::BlsCommittee</a>): &vector&lt;<a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommitteeMember">ika_common::bls_committee::BlsCommitteeMember</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_members">members</a>(self: &<a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommittee">BlsCommittee</a>): &vector&lt;<a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommitteeMember">BlsCommitteeMember</a>&gt; {
    &self.<a href="../ika_common/bls_committee.md#ika_common_bls_committee_members">members</a>
}
</code></pre>



</details>

<a name="ika_common_bls_committee_validator_ids"></a>

## Function `validator_ids`

Returns a vector of all validator IDs in the committee


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_validator_ids">validator_ids</a>(self: &<a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommittee">ika_common::bls_committee::BlsCommittee</a>): vector&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_validator_ids">validator_ids</a>(self: &<a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommittee">BlsCommittee</a>): vector&lt;ID&gt; {
    self.<a href="../ika_common/bls_committee.md#ika_common_bls_committee_members">members</a>().map_ref!(|m| m.<a href="../ika_common/bls_committee.md#ika_common_bls_committee_validator_id">validator_id</a>())
}
</code></pre>



</details>

<a name="ika_common_bls_committee_contains"></a>

## Function `contains`

Checks if the committee contains a specific validator ID


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_contains">contains</a>(self: &<a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommittee">ika_common::bls_committee::BlsCommittee</a>, <a href="../ika_common/bls_committee.md#ika_common_bls_committee_validator_id">validator_id</a>: &<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_contains">contains</a>(self: &<a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommittee">BlsCommittee</a>, <a href="../ika_common/bls_committee.md#ika_common_bls_committee_validator_id">validator_id</a>: &ID): bool {
    self.<a href="../ika_common/bls_committee.md#ika_common_bls_committee_members">members</a>().any!(|m| m.<a href="../ika_common/bls_committee.md#ika_common_bls_committee_validator_id">validator_id</a>() == <a href="../ika_common/bls_committee.md#ika_common_bls_committee_validator_id">validator_id</a>)
}
</code></pre>



</details>

<a name="ika_common_bls_committee_verify_certificate"></a>

## Function `verify_certificate`

Verifies an aggregate BLS signature is a certificate in the epoch
The <code>signers_bitmap</code> represents which validators signed the certificate
Returns successfully if signature is valid and meets quorum threshold, otherwise aborts


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_verify_certificate">verify_certificate</a>(self: &<a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommittee">ika_common::bls_committee::BlsCommittee</a>, epoch: u64, signature: &vector&lt;u8&gt;, signers_bitmap: &vector&lt;u8&gt;, intent_bytes: &vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_verify_certificate">verify_certificate</a>(
    self: &<a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommittee">BlsCommittee</a>,
    epoch: u64,
    signature: &vector&lt;u8&gt;,
    signers_bitmap: &vector&lt;u8&gt;,
    intent_bytes: &vector&lt;u8&gt;,
) {
    <b>assert</b>!(signature.length() == <a href="../ika_common/bls_committee.md#ika_common_bls_committee_BLS_SIGNATURE_LEN">BLS_SIGNATURE_LEN</a>, <a href="../ika_common/bls_committee.md#ika_common_bls_committee_EInvalidSignatureLength">EInvalidSignatureLength</a>);
    <b>let</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_members">members</a> = &self.<a href="../ika_common/bls_committee.md#ika_common_bls_committee_members">members</a>;
    // Count non-signers instead of summing their voting powers
    <b>let</b> <b>mut</b> non_signer_count = 0;
    <b>let</b> <b>mut</b> non_signer_public_keys: vector&lt;Element&lt;UncompressedG1&gt;&gt; = vector::empty();
    <b>let</b> <b>mut</b> offset: u64 = 0;
    <b>let</b> n_members = <a href="../ika_common/bls_committee.md#ika_common_bls_committee_members">members</a>.length();
    <b>let</b> max_bitmap_len_bytes = n_members.divide_and_round_up(8);
    // The signers bitmap must not be longer than necessary to hold all <a href="../ika_common/bls_committee.md#ika_common_bls_committee_members">members</a>
    // It may be shorter, in which case the excluded <a href="../ika_common/bls_committee.md#ika_common_bls_committee_members">members</a> are treated <b>as</b> non-signers
    <b>assert</b>!(signers_bitmap.length() == max_bitmap_len_bytes, <a href="../ika_common/bls_committee.md#ika_common_bls_committee_EInvalidBitmap">EInvalidBitmap</a>);
    // Iterate over the signers bitmap and check <b>if</b> each member is a signer
    max_bitmap_len_bytes.do!(|i| {
        // Get the current byte or 0 <b>if</b> we've reached the end of the bitmap
        <b>let</b> byte = <b>if</b> (i &lt; signers_bitmap.length()) {
            signers_bitmap[i]
        } <b>else</b> {
            0
        };
        (8u8).do!(|i| {
            <b>let</b> index = offset + (i <b>as</b> u64);
            <b>let</b> is_signer = (byte &gt;&gt; i) & 1 == 1;
            // If the index is out of bounds, the bit must be 0 to ensure
            // uniqueness of the signers_bitmap
            <b>if</b> (index &gt;= n_members) {
                <b>assert</b>!(!is_signer, <a href="../ika_common/bls_committee.md#ika_common_bls_committee_EInvalidBitmap">EInvalidBitmap</a>);
                <b>return</b>
            };
            // There will be fewer non-signers than signers, so we handle
            // non-signers here
            <b>if</b> (!is_signer) {
                <b>let</b> member = &<a href="../ika_common/bls_committee.md#ika_common_bls_committee_members">members</a>[index];
                non_signer_count = non_signer_count + 1;
                non_signer_public_keys.push_back(member.protocol_pubkey);
            };
        });
        offset = offset + 8;
    });
    // Compute the aggregate voting power <b>as</b> the number of signers
    <b>let</b> signer_count = n_members - non_signer_count;
    <b>assert</b>!(<a href="../ika_common/bls_committee.md#ika_common_bls_committee_is_quorum_threshold">is_quorum_threshold</a>(self, signer_count), <a href="../ika_common/bls_committee.md#ika_common_bls_committee_ENotEnoughStake">ENotEnoughStake</a>);
    // Compute the aggregate <b>public</b> key <b>as</b> the difference between the total
    // aggregated key and the sum of the non-signer <b>public</b> keys
    <b>let</b> aggregate_key = bls12381::g1_sub(
        &self.aggregated_protocol_pubkey,
        &bls12381::uncompressed_g1_to_g1(
            &bls12381::uncompressed_g1_sum(&non_signer_public_keys),
        ),
    );
    // Verify the signature
    <b>let</b> pub_key_bytes = group_ops::bytes(&aggregate_key);
    <b>assert</b>!(
        bls12381::bls12381_min_pk_verify(
            signature,
            pub_key_bytes,
            intent_bytes,
        ),
        <a href="../ika_common/bls_committee.md#ika_common_bls_committee_EInvalidSignature">EInvalidSignature</a>,
    );
    event::emit(<a href="../ika_common/bls_committee.md#ika_common_bls_committee_CommitteeQuorumVerifiedEvent">CommitteeQuorumVerifiedEvent</a> {
        epoch,
        signer_count,
    });
}
</code></pre>



</details>

<a name="ika_common_bls_committee_is_quorum_threshold"></a>

## Function `is_quorum_threshold`

Returns true if the voting power meets or exceeds the quorum threshold


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_is_quorum_threshold">is_quorum_threshold</a>(self: &<a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommittee">ika_common::bls_committee::BlsCommittee</a>, signer_count: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_is_quorum_threshold">is_quorum_threshold</a>(self: &<a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommittee">BlsCommittee</a>, signer_count: u64): bool {
    signer_count &gt;= self.<a href="../ika_common/bls_committee.md#ika_common_bls_committee_quorum_threshold">quorum_threshold</a>
}
</code></pre>



</details>

<a name="ika_common_bls_committee_is_validity_threshold"></a>

## Function `is_validity_threshold`

Returns true if the voting power meets or exceeds the validity threshold


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_is_validity_threshold">is_validity_threshold</a>(self: &<a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommittee">ika_common::bls_committee::BlsCommittee</a>, signer_count: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/bls_committee.md#ika_common_bls_committee_is_validity_threshold">is_validity_threshold</a>(self: &<a href="../ika_common/bls_committee.md#ika_common_bls_committee_BlsCommittee">BlsCommittee</a>, signer_count: u64): bool {
    signer_count &gt;= self.<a href="../ika_common/bls_committee.md#ika_common_bls_committee_validity_threshold">validity_threshold</a>
}
</code></pre>



</details>
