---
title: Module `(ika_system=0x0)::bls_committee`
---



-  [Struct `BlsCommitteeMember`](#(ika_system=0x0)_bls_committee_BlsCommitteeMember)
-  [Struct `BlsCommittee`](#(ika_system=0x0)_bls_committee_BlsCommittee)
-  [Struct `CommitteeQuorumVerifiedEvent`](#(ika_system=0x0)_bls_committee_CommitteeQuorumVerifiedEvent)
-  [Constants](#@Constants_0)
-  [Function `new_bls_committee_member`](#(ika_system=0x0)_bls_committee_new_bls_committee_member)
-  [Function `voting_power`](#(ika_system=0x0)_bls_committee_voting_power)
-  [Function `validator_id`](#(ika_system=0x0)_bls_committee_validator_id)
-  [Function `new_bls_committee`](#(ika_system=0x0)_bls_committee_new_bls_committee)
-  [Function `empty`](#(ika_system=0x0)_bls_committee_empty)
-  [Function `members`](#(ika_system=0x0)_bls_committee_members)
-  [Function `validator_ids`](#(ika_system=0x0)_bls_committee_validator_ids)
-  [Function `contains`](#(ika_system=0x0)_bls_committee_contains)
-  [Function `init_voting_power_info`](#(ika_system=0x0)_bls_committee_init_voting_power_info)
-  [Function `total_stake`](#(ika_system=0x0)_bls_committee_total_stake)
-  [Function `adjust_voting_power`](#(ika_system=0x0)_bls_committee_adjust_voting_power)
-  [Function `check_invariants`](#(ika_system=0x0)_bls_committee_check_invariants)
-  [Function `total_voting_power`](#(ika_system=0x0)_bls_committee_total_voting_power)
-  [Function `quorum_threshold`](#(ika_system=0x0)_bls_committee_quorum_threshold)
-  [Function `verify_certificate`](#(ika_system=0x0)_bls_committee_verify_certificate)
-  [Function `verify_quorum`](#(ika_system=0x0)_bls_committee_verify_quorum)


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



<a name="(ika_system=0x0)_bls_committee_BlsCommitteeMember"></a>

## Struct `BlsCommitteeMember`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommitteeMember">BlsCommitteeMember</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_validator_id">validator_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>protocol_pubkey: <a href="../sui/group_ops.md#sui_group_ops_Element">sui::group_ops::Element</a>&lt;<a href="../sui/bls12381.md#sui_bls12381_UncompressedG1">sui::bls12381::UncompressedG1</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_voting_power">voting_power</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>stake: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_bls_committee_BlsCommittee"></a>

## Struct `BlsCommittee`

Represents the current committee in the system.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">BlsCommittee</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>: vector&lt;(ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommitteeMember">bls_committee::BlsCommitteeMember</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>aggregated_protocol_pubkey: <a href="../sui/group_ops.md#sui_group_ops_Element">sui::group_ops::Element</a>&lt;<a href="../sui/bls12381.md#sui_bls12381_G1">sui::bls12381::G1</a>&gt;</code>
</dt>
<dd>
 The aggregation of public keys for all members of the committee
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_bls_committee_CommitteeQuorumVerifiedEvent"></a>

## Struct `CommitteeQuorumVerifiedEvent`

Event emitted after verifing quorum of signature.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_CommitteeQuorumVerifiedEvent">CommitteeQuorumVerifiedEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>total_signers_stake: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="(ika_system=0x0)_bls_committee_BLS_SIGNATURE_LEN"></a>



<pre><code><b>const</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BLS_SIGNATURE_LEN">BLS_SIGNATURE_LEN</a>: u64 = 96;
</code></pre>



<a name="(ika_system=0x0)_bls_committee_EInvalidBitmap"></a>



<pre><code><b>const</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_EInvalidBitmap">EInvalidBitmap</a>: u64 = 0;
</code></pre>



<a name="(ika_system=0x0)_bls_committee_EInvalidSignature"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_EInvalidSignature">EInvalidSignature</a>: vector&lt;u8&gt; = b"Invalid certificate signature.";
</code></pre>



<a name="(ika_system=0x0)_bls_committee_EInvalidSignatureLength"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_EInvalidSignatureLength">EInvalidSignatureLength</a>: vector&lt;u8&gt; = b"The length of the provided bls signature is incorrect.";
</code></pre>



<a name="(ika_system=0x0)_bls_committee_EInvalidVotingPower"></a>



<pre><code><b>const</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_EInvalidVotingPower">EInvalidVotingPower</a>: u64 = 4;
</code></pre>



<a name="(ika_system=0x0)_bls_committee_ENotEnoughStake"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_ENotEnoughStake">ENotEnoughStake</a>: vector&lt;u8&gt; = b"Not enough stake of signers <b>for</b> the bls signature.";
</code></pre>



<a name="(ika_system=0x0)_bls_committee_ERelativePowerMismatch"></a>



<pre><code><b>const</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_ERelativePowerMismatch">ERelativePowerMismatch</a>: u64 = 2;
</code></pre>



<a name="(ika_system=0x0)_bls_committee_ETotalPowerMismatch"></a>



<pre><code><b>const</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_ETotalPowerMismatch">ETotalPowerMismatch</a>: u64 = 1;
</code></pre>



<a name="(ika_system=0x0)_bls_committee_EVotingPowerOverThreshold"></a>



<pre><code><b>const</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_EVotingPowerOverThreshold">EVotingPowerOverThreshold</a>: u64 = 3;
</code></pre>



<a name="(ika_system=0x0)_bls_committee_MAX_VOTING_POWER"></a>



<pre><code><b>const</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_MAX_VOTING_POWER">MAX_VOTING_POWER</a>: u64 = 1;
</code></pre>



<a name="(ika_system=0x0)_bls_committee_QUORUM_THRESHOLD"></a>

Quorum threshold for our fixed voting power - any message signed by this much voting power can be trusted
up to BFT assumptions


<pre><code><b>const</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_QUORUM_THRESHOLD">QUORUM_THRESHOLD</a>: u64 = 3;
</code></pre>



<a name="(ika_system=0x0)_bls_committee_TOTAL_VOTING_POWER"></a>

Set total_voting_power as 10_000 by convention. Individual voting powers can be interpreted
as easily understandable basis points (e.g., voting_power: 100 = 1%, voting_power: 1 = 0.01%) rather than
opaque quantities whose meaning changes from epoch to epoch as the total amount staked shifts.
Fixing the total voting power allows clients to hardcode the quorum threshold and total_voting power rather
than recomputing these.


<pre><code><b>const</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_TOTAL_VOTING_POWER">TOTAL_VOTING_POWER</a>: u64 = 4;
</code></pre>



<a name="(ika_system=0x0)_bls_committee_new_bls_committee_member"></a>

## Function `new_bls_committee_member`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_new_bls_committee_member">new_bls_committee_member</a>(<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_validator_id">validator_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, protocol_pubkey: <a href="../sui/group_ops.md#sui_group_ops_Element">sui::group_ops::Element</a>&lt;<a href="../sui/bls12381.md#sui_bls12381_UncompressedG1">sui::bls12381::UncompressedG1</a>&gt;, stake: u64): (ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommitteeMember">bls_committee::BlsCommitteeMember</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_new_bls_committee_member">new_bls_committee_member</a>(
    <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_validator_id">validator_id</a>: ID,
    protocol_pubkey: Element&lt;UncompressedG1&gt;,
    stake: u64,
): <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommitteeMember">BlsCommitteeMember</a> {
    <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommitteeMember">BlsCommitteeMember</a> {
        <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_validator_id">validator_id</a>,
        protocol_pubkey,
        <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_voting_power">voting_power</a>: 0,
        stake,
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_bls_committee_voting_power"></a>

## Function `voting_power`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_voting_power">voting_power</a>(member: &(ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommitteeMember">bls_committee::BlsCommitteeMember</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_voting_power">voting_power</a>(member: &<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommitteeMember">BlsCommitteeMember</a>): u64 {
    member.<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_voting_power">voting_power</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_bls_committee_validator_id"></a>

## Function `validator_id`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_validator_id">validator_id</a>(member: &(ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommitteeMember">bls_committee::BlsCommitteeMember</a>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_validator_id">validator_id</a>(member: &<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommitteeMember">BlsCommitteeMember</a>): ID {
    member.<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_validator_id">validator_id</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_bls_committee_new_bls_committee"></a>

## Function `new_bls_committee`

Create a new committee from members.
Each member's voting power is initialized using their stake. We then attempt to cap their voting power
at <code><a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_MAX_VOTING_POWER">MAX_VOTING_POWER</a></code>. If <code><a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_MAX_VOTING_POWER">MAX_VOTING_POWER</a></code> is not a feasible cap, we pick the lowest possible cap.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_new_bls_committee">new_bls_committee</a>(<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>: vector&lt;(ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommitteeMember">bls_committee::BlsCommitteeMember</a>&gt;): (ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">bls_committee::BlsCommittee</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_new_bls_committee">new_bls_committee</a>(<b>mut</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>: vector&lt;<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommitteeMember">BlsCommitteeMember</a>&gt;): <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">BlsCommittee</a> {
    // If threshold_pct is too small, it's possible that even when all <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a> reach the threshold we still don't
    // have 100%. So we bound the threshold_pct to be always enough to find a solution.
    <b>let</b> threshold = <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_TOTAL_VOTING_POWER">TOTAL_VOTING_POWER</a>.min(<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_MAX_VOTING_POWER">MAX_VOTING_POWER</a>.max(<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_TOTAL_VOTING_POWER">TOTAL_VOTING_POWER</a>.divide_and_round_up(<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>.length())));
    <b>let</b> remaining_power = <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_init_voting_power_info">init_voting_power_info</a>(&<b>mut</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>, threshold);
    <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_adjust_voting_power">adjust_voting_power</a>(&<b>mut</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>, threshold, remaining_power);
    <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_check_invariants">check_invariants</a>(&<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>);
    // Compute the total aggregated key, e.g. the sum of all <b>public</b> keys in the committee.
    <b>let</b> aggregated_protocol_pubkey = bls12381::uncompressed_g1_to_g1(
        &bls12381::uncompressed_g1_sum(
            &<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>.map!(|member| member.protocol_pubkey),
        ),
    );
    <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">BlsCommittee</a> {
        <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>,
        aggregated_protocol_pubkey
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_bls_committee_empty"></a>

## Function `empty`

Creates an empty committee. Only relevant for init phase.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_empty">empty</a>(): (ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">bls_committee::BlsCommittee</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_empty">empty</a>(): <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">BlsCommittee</a> {
    <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">BlsCommittee</a> {
        <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>: vector[],
        aggregated_protocol_pubkey: bls12381::g1_identity()
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_bls_committee_members"></a>

## Function `members`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>(self: &(ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">bls_committee::BlsCommittee</a>): &vector&lt;(ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommitteeMember">bls_committee::BlsCommitteeMember</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>(self: &<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">BlsCommittee</a>): &vector&lt;<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommitteeMember">BlsCommitteeMember</a>&gt; {
    &self.<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_bls_committee_validator_ids"></a>

## Function `validator_ids`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_validator_ids">validator_ids</a>(self: &(ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">bls_committee::BlsCommittee</a>): vector&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_validator_ids">validator_ids</a>(self: &<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">BlsCommittee</a>): vector&lt;ID&gt; {
    self.<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>().map_ref!(|m| m.<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_validator_id">validator_id</a>())
}
</code></pre>



</details>

<a name="(ika_system=0x0)_bls_committee_contains"></a>

## Function `contains`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_contains">contains</a>(self: &(ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">bls_committee::BlsCommittee</a>, <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_validator_id">validator_id</a>: &<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_contains">contains</a>(self: &<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">BlsCommittee</a>, <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_validator_id">validator_id</a>: &ID): bool {
    self.<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>().any!(|m| m.<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_validator_id">validator_id</a>() == <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_validator_id">validator_id</a>)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_bls_committee_init_voting_power_info"></a>

## Function `init_voting_power_info`

Create the initial voting power of each member, set using their stake, but capped using threshold.
We also perform insertion sort while creating the voting power list, by maintaining the list in
descending order using voting power.
Anything beyond the threshold is added to the remaining_power, which is also returned.


<pre><code><b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_init_voting_power_info">init_voting_power_info</a>(<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>: &<b>mut</b> vector&lt;(ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommitteeMember">bls_committee::BlsCommitteeMember</a>&gt;, threshold: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_init_voting_power_info">init_voting_power_info</a>(
    <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>: &<b>mut</b> vector&lt;<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommitteeMember">BlsCommitteeMember</a>&gt;,
    threshold: u64,
): u64 {
    <b>let</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_total_stake">total_stake</a> = <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_total_stake">total_stake</a>(<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>);
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>.length();
    <b>let</b> <b>mut</b> total_power = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> m = &<b>mut</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>[i];
        <b>let</b> stake = m.stake;
        <b>let</b> adjusted_stake = stake <b>as</b> u128 * (<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_TOTAL_VOTING_POWER">TOTAL_VOTING_POWER</a> <b>as</b> u128) / (<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_total_stake">total_stake</a> <b>as</b> u128);
        <b>let</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_voting_power">voting_power</a> = (adjusted_stake <b>as</b> u64).min(threshold);
        m.<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_voting_power">voting_power</a> = <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_voting_power">voting_power</a>;
        total_power = total_power + <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_voting_power">voting_power</a>;
        i = i + 1;
    };
    <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_TOTAL_VOTING_POWER">TOTAL_VOTING_POWER</a> - total_power
}
</code></pre>



</details>

<a name="(ika_system=0x0)_bls_committee_total_stake"></a>

## Function `total_stake`

Sum up the total stake of all members.


<pre><code><b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_total_stake">total_stake</a>(<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>: &vector&lt;(ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommitteeMember">bls_committee::BlsCommitteeMember</a>&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_total_stake">total_stake</a>(<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>: &vector&lt;<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommitteeMember">BlsCommitteeMember</a>&gt;): u64 {
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>.length();
    <b>let</b> <b>mut</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_total_stake">total_stake</a> = 0;
    <b>while</b> (i &lt; len) {
        <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_total_stake">total_stake</a> = <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_total_stake">total_stake</a> + <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>[i].stake;
        i = i + 1;
    };
    <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_total_stake">total_stake</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_bls_committee_adjust_voting_power"></a>

## Function `adjust_voting_power`

Distribute remaining_power to members that are not capped at threshold.


<pre><code><b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_adjust_voting_power">adjust_voting_power</a>(<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>: &<b>mut</b> vector&lt;(ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommitteeMember">bls_committee::BlsCommitteeMember</a>&gt;, threshold: u64, remaining_power: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_adjust_voting_power">adjust_voting_power</a>(
    <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>: &<b>mut</b> vector&lt;<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommitteeMember">BlsCommitteeMember</a>&gt;,
    threshold: u64,
    <b>mut</b> remaining_power: u64,
) {
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>.length();
    <b>while</b> (i &lt; len && remaining_power &gt; 0) {
        <b>let</b> v = &<b>mut</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>[i];
        // planned is the amount of extra power we want to distribute to this member.
        <b>let</b> planned = remaining_power.divide_and_round_up(len - i);
        // target is the targeting power this member will reach, capped by threshold.
        <b>let</b> target = threshold.min(v.<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_voting_power">voting_power</a> + planned);
        // actual is the actual amount of power we will be distributing to this member.
        <b>let</b> actual = remaining_power.min(target - v.<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_voting_power">voting_power</a>);
        v.<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_voting_power">voting_power</a> = v.<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_voting_power">voting_power</a> + actual;
        <b>assert</b>!(v.<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_voting_power">voting_power</a> &lt;= threshold, <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_EVotingPowerOverThreshold">EVotingPowerOverThreshold</a>);
        remaining_power = remaining_power - actual;
        i = i + 1;
    };
    <b>assert</b>!(remaining_power == 0, <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_ETotalPowerMismatch">ETotalPowerMismatch</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_bls_committee_check_invariants"></a>

## Function `check_invariants`

Check a few invariants that must hold after setting the voting power.


<pre><code><b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_check_invariants">check_invariants</a>(<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>: &vector&lt;(ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommitteeMember">bls_committee::BlsCommitteeMember</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_check_invariants">check_invariants</a>(<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>: &vector&lt;<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommitteeMember">BlsCommitteeMember</a>&gt;,) {
    // First check that the total voting power must be <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_TOTAL_VOTING_POWER">TOTAL_VOTING_POWER</a>.
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>.length();
    <b>let</b> <b>mut</b> total = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_voting_power">voting_power</a> = <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>[i].<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_voting_power">voting_power</a>;
        <b>assert</b>!(<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_voting_power">voting_power</a> &gt; 0, <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_EInvalidVotingPower">EInvalidVotingPower</a>);
        total = total + <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_voting_power">voting_power</a>;
        i = i + 1;
    };
    <b>assert</b>!(total == <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_TOTAL_VOTING_POWER">TOTAL_VOTING_POWER</a>, <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_ETotalPowerMismatch">ETotalPowerMismatch</a>);
    // Second check that <b>if</b> member A's stake is larger than B's stake, A's voting power must be no less
    // than B's voting power; similarly, <b>if</b> A's stake is less than B's stake, A's voting power must be no larger
    // than B's voting power.
    <b>let</b> <b>mut</b> a = 0;
    <b>while</b> (a &lt; len) {
        <b>let</b> <b>mut</b> b = a + 1;
        <b>while</b> (b &lt; len) {
            <b>let</b> member_a = &<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>[a];
            <b>let</b> member_b = &<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>[b];
            <b>let</b> stake_a = member_a.stake;
            <b>let</b> stake_b = member_b.stake;
            <b>let</b> power_a = member_a.<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_voting_power">voting_power</a>;
            <b>let</b> power_b = member_b.<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_voting_power">voting_power</a>;
            <b>if</b> (stake_a &gt; stake_b) {
                <b>assert</b>!(power_a &gt;= power_b, <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_ERelativePowerMismatch">ERelativePowerMismatch</a>);
            };
            <b>if</b> (stake_a &lt; stake_b) {
                <b>assert</b>!(power_a &lt;= power_b, <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_ERelativePowerMismatch">ERelativePowerMismatch</a>);
            };
            b = b + 1;
        };
        a = a + 1;
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_bls_committee_total_voting_power"></a>

## Function `total_voting_power`

Return the (constant) total voting power


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_total_voting_power">total_voting_power</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_total_voting_power">total_voting_power</a>(): u64 {
    <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_TOTAL_VOTING_POWER">TOTAL_VOTING_POWER</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_bls_committee_quorum_threshold"></a>

## Function `quorum_threshold`

Return the (constant) quorum threshold


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_quorum_threshold">quorum_threshold</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_quorum_threshold">quorum_threshold</a>(): u64 {
    <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_QUORUM_THRESHOLD">QUORUM_THRESHOLD</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_bls_committee_verify_certificate"></a>

## Function `verify_certificate`

Verify an aggregate BLS signature is a certificate in the epoch, and return
the type of certificate and the bytes certified. The <code>signers</code> vector is
an increasing list of indexes into the <code>committee</code> vector.
If there is a certificate, the function returns the total stake. Otherwise, it aborts.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_verify_certificate">verify_certificate</a>(self: &(ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">bls_committee::BlsCommittee</a>, epoch: u64, signature: &vector&lt;u8&gt;, signers_bitmap: &vector&lt;u8&gt;, intent_bytes: &vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_verify_certificate">verify_certificate</a>(
    self: &<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">BlsCommittee</a>,
    epoch: u64,
    signature: &vector&lt;u8&gt;,
    signers_bitmap: &vector&lt;u8&gt;,
    intent_bytes: &vector&lt;u8&gt;,
) {
    <b>assert</b>!(signature.length() == <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BLS_SIGNATURE_LEN">BLS_SIGNATURE_LEN</a>, <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_EInvalidSignatureLength">EInvalidSignatureLength</a>);
    <b>let</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a> = &self.<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>;
    // Use the signers_bitmap to construct the key and the <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_voting_power">voting_power</a>.
    <b>let</b> <b>mut</b> non_signer_aggregate_voting_power = 0;
    <b>let</b> <b>mut</b> non_signer_public_keys: vector&lt;Element&lt;UncompressedG1&gt;&gt; = vector::empty();
    <b>let</b> <b>mut</b> offset: u64 = 0;
    <b>let</b> n_members = <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>.length();
    <b>let</b> max_bitmap_len_bytes = n_members.divide_and_round_up(8);
    // The signers bitmap must not be longer than necessary to hold all <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>.
    // It may be shorter, in which case the excluded <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a> are treated <b>as</b> non-signers.
    <b>assert</b>!(signers_bitmap.length() &lt;= max_bitmap_len_bytes, <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_EInvalidBitmap">EInvalidBitmap</a>);
    // Iterate over the signers bitmap and check <b>if</b> each member is a signer.
    max_bitmap_len_bytes.do!(|i| {
        // Get the current byte or 0 <b>if</b> we've reached the end of the bitmap.
        <b>let</b> byte = <b>if</b> (i &lt; signers_bitmap.length()) {
            signers_bitmap[i]
        } <b>else</b> {
            0
        };
        (8u8).do!(|i| {
            <b>let</b> index = offset + (i <b>as</b> u64);
            <b>let</b> is_signer = (byte &gt;&gt; i) & 1 == 1;
            // If the index is out of bounds, the bit must be 0 to ensure
            // uniqueness of the signers_bitmap.
            <b>if</b> (index &gt;= n_members) {
                <b>assert</b>!(!is_signer, <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_EInvalidBitmap">EInvalidBitmap</a>);
                <b>return</b>
            };
            // There will be fewer non-signers than signers, so we handle
            // non-signers here.
            <b>if</b> (!is_signer) {
                <b>let</b> member = &<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_members">members</a>[index];
                non_signer_aggregate_voting_power = non_signer_aggregate_voting_power + member.<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_voting_power">voting_power</a>;
                non_signer_public_keys.push_back(member.protocol_pubkey);
            };
        });
        offset = offset + 8;
    });
    // Compute the aggregate <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_voting_power">voting_power</a> <b>as</b> the difference between the total voting power
    // and the total voting power of the non-signers.
    <b>let</b> aggregate_voting_power = <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_TOTAL_VOTING_POWER">TOTAL_VOTING_POWER</a> - non_signer_aggregate_voting_power;
    <b>assert</b>!(<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_verify_quorum">verify_quorum</a>(aggregate_voting_power), <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_ENotEnoughStake">ENotEnoughStake</a>);
    // Compute the aggregate <b>public</b> key <b>as</b> the difference between the total
    // aggregated key and the sum of the non-signer <b>public</b> keys.
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
        <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_EInvalidSignature">EInvalidSignature</a>,
    );
    event::emit(<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_CommitteeQuorumVerifiedEvent">CommitteeQuorumVerifiedEvent</a> {
        epoch,
        total_signers_stake: aggregate_voting_power,
    });
}
</code></pre>



</details>

<a name="(ika_system=0x0)_bls_committee_verify_quorum"></a>

## Function `verify_quorum`

Returns true if the voting power is more than the aggregate voting power of quorum members of a committee.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_verify_quorum">verify_quorum</a>(aggregate_voting_power: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_verify_quorum">verify_quorum</a>(aggregate_voting_power: u64): bool {
    aggregate_voting_power &gt;= <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_QUORUM_THRESHOLD">QUORUM_THRESHOLD</a>
}
</code></pre>



</details>
