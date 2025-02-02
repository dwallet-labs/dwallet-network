---
title: Module `0x0::committee`
---



-  [Struct `CommitteeMember`](#0x0_committee_CommitteeMember)
-  [Struct `Committee`](#0x0_committee_Committee)
-  [Constants](#@Constants_0)
-  [Function `new_committee_member`](#0x0_committee_new_committee_member)
-  [Function `voting_power`](#0x0_committee_voting_power)
-  [Function `validator_id`](#0x0_committee_validator_id)
-  [Function `new_committee`](#0x0_committee_new_committee)
-  [Function `empty`](#0x0_committee_empty)
-  [Function `members`](#0x0_committee_members)
-  [Function `validator_ids`](#0x0_committee_validator_ids)
-  [Function `contains`](#0x0_committee_contains)
-  [Function `init_voting_power_info`](#0x0_committee_init_voting_power_info)
-  [Function `total_stake`](#0x0_committee_total_stake)
-  [Function `adjust_voting_power`](#0x0_committee_adjust_voting_power)
-  [Function `check_invariants`](#0x0_committee_check_invariants)
-  [Function `total_voting_power`](#0x0_committee_total_voting_power)
-  [Function `quorum_threshold`](#0x0_committee_quorum_threshold)
-  [Function `verify_certificate`](#0x0_committee_verify_certificate)
-  [Function `verify_quorum`](#0x0_committee_verify_quorum)


<pre><code><b>use</b> <a href="../move-stdlib/u64.md#0x1_u64">0x1::u64</a>;
<b>use</b> <a href="../move-stdlib/vector.md#0x1_vector">0x1::vector</a>;
<b>use</b> <a href="../sui-framework/bls12381.md#0x2_bls12381">0x2::bls12381</a>;
<b>use</b> <a href="../sui-framework/group_ops.md#0x2_group_ops">0x2::group_ops</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
</code></pre>



<a name="0x0_committee_CommitteeMember"></a>

## Struct `CommitteeMember`



<pre><code><b>struct</b> <a href="committee.md#0x0_committee_CommitteeMember">CommitteeMember</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>protocol_pubkey: <a href="../sui-framework/group_ops.md#0x2_group_ops_Element">group_ops::Element</a>&lt;<a href="../sui-framework/bls12381.md#0x2_bls12381_UncompressedG1">bls12381::UncompressedG1</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>voting_power: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>stake: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x0_committee_Committee"></a>

## Struct `Committee`

Represents the current committee in the system.


<pre><code><b>struct</b> <a href="committee.md#0x0_committee_Committee">Committee</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>members: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="committee.md#0x0_committee_CommitteeMember">committee::CommitteeMember</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>total_aggregated_key: <a href="../sui-framework/group_ops.md#0x2_group_ops_Element">group_ops::Element</a>&lt;<a href="../sui-framework/bls12381.md#0x2_bls12381_G1">bls12381::G1</a>&gt;</code>
</dt>
<dd>
 The aggregation of public keys for all members of the committee
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x0_committee_BLS_SIGNATURE_LEN"></a>



<pre><code><b>const</b> <a href="committee.md#0x0_committee_BLS_SIGNATURE_LEN">BLS_SIGNATURE_LEN</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 96;
</code></pre>



<a name="0x0_committee_EInvalidBitmap"></a>



<pre><code><b>const</b> <a href="committee.md#0x0_committee_EInvalidBitmap">EInvalidBitmap</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
</code></pre>



<a name="0x0_committee_EInvalidSignature"></a>



<pre><code>#[error]
<b>const</b> <a href="committee.md#0x0_committee_EInvalidSignature">EInvalidSignature</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = b"Invalid certificate signature.";
</code></pre>



<a name="0x0_committee_EInvalidSignatureLength"></a>



<pre><code>#[error]
<b>const</b> <a href="committee.md#0x0_committee_EInvalidSignatureLength">EInvalidSignatureLength</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = b"The length of the provided bls signature is incorrect.";
</code></pre>



<a name="0x0_committee_EInvalidVotingPower"></a>



<pre><code><b>const</b> <a href="committee.md#0x0_committee_EInvalidVotingPower">EInvalidVotingPower</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 4;
</code></pre>



<a name="0x0_committee_ENotEnoughStake"></a>



<pre><code>#[error]
<b>const</b> <a href="committee.md#0x0_committee_ENotEnoughStake">ENotEnoughStake</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = b"Not enough stake of signers for the bls signature.";
</code></pre>



<a name="0x0_committee_ERelativePowerMismatch"></a>



<pre><code><b>const</b> <a href="committee.md#0x0_committee_ERelativePowerMismatch">ERelativePowerMismatch</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 2;
</code></pre>



<a name="0x0_committee_ETotalPowerMismatch"></a>



<pre><code><b>const</b> <a href="committee.md#0x0_committee_ETotalPowerMismatch">ETotalPowerMismatch</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1;
</code></pre>



<a name="0x0_committee_EVotingPowerOverThreshold"></a>



<pre><code><b>const</b> <a href="committee.md#0x0_committee_EVotingPowerOverThreshold">EVotingPowerOverThreshold</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 3;
</code></pre>



<a name="0x0_committee_MAX_VOTING_POWER"></a>



<pre><code><b>const</b> <a href="committee.md#0x0_committee_MAX_VOTING_POWER">MAX_VOTING_POWER</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1000;
</code></pre>



<a name="0x0_committee_QUORUM_THRESHOLD"></a>

Quorum threshold for our fixed voting power--any message signed by this much voting power can be trusted
up to BFT assumptions


<pre><code><b>const</b> <a href="committee.md#0x0_committee_QUORUM_THRESHOLD">QUORUM_THRESHOLD</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 6667;
</code></pre>



<a name="0x0_committee_TOTAL_VOTING_POWER"></a>

Set total_voting_power as 10_000 by convention. Individual voting powers can be interpreted
as easily understandable basis points (e.g., voting_power: 100 = 1%, voting_power: 1 = 0.01%) rather than
opaque quantities whose meaning changes from epoch to epoch as the total amount staked shifts.
Fixing the total voting power allows clients to hardcode the quorum threshold and total_voting power rather
than recomputing these.


<pre><code><b>const</b> <a href="committee.md#0x0_committee_TOTAL_VOTING_POWER">TOTAL_VOTING_POWER</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 10000;
</code></pre>



<a name="0x0_committee_new_committee_member"></a>

## Function `new_committee_member`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="committee.md#0x0_committee_new_committee_member">new_committee_member</a>(validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, protocol_pubkey: <a href="../sui-framework/group_ops.md#0x2_group_ops_Element">group_ops::Element</a>&lt;<a href="../sui-framework/bls12381.md#0x2_bls12381_UncompressedG1">bls12381::UncompressedG1</a>&gt;, stake: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="committee.md#0x0_committee_CommitteeMember">committee::CommitteeMember</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="committee.md#0x0_committee_new_committee_member">new_committee_member</a>(
    validator_id: ID,
    protocol_pubkey: Element&lt;UncompressedG1&gt;,
    stake: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
): <a href="committee.md#0x0_committee_CommitteeMember">CommitteeMember</a> {
    <a href="committee.md#0x0_committee_CommitteeMember">CommitteeMember</a> {
        validator_id,
        protocol_pubkey,
        voting_power: 0,
        stake,
    }
}
</code></pre>



</details>

<a name="0x0_committee_voting_power"></a>

## Function `voting_power`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="committee.md#0x0_committee_voting_power">voting_power</a>(member: &<a href="committee.md#0x0_committee_CommitteeMember">committee::CommitteeMember</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="committee.md#0x0_committee_voting_power">voting_power</a>(member: &<a href="committee.md#0x0_committee_CommitteeMember">CommitteeMember</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    member.voting_power
}
</code></pre>



</details>

<a name="0x0_committee_validator_id"></a>

## Function `validator_id`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="committee.md#0x0_committee_validator_id">validator_id</a>(member: &<a href="committee.md#0x0_committee_CommitteeMember">committee::CommitteeMember</a>): <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="committee.md#0x0_committee_validator_id">validator_id</a>(member: &<a href="committee.md#0x0_committee_CommitteeMember">CommitteeMember</a>): ID {
    member.validator_id
}
</code></pre>



</details>

<a name="0x0_committee_new_committee"></a>

## Function `new_committee`

Create a new committee from members.
Each member's voting power is initialized using their stake. We then attempt to cap their voting power
at <code><a href="committee.md#0x0_committee_MAX_VOTING_POWER">MAX_VOTING_POWER</a></code>. If <code><a href="committee.md#0x0_committee_MAX_VOTING_POWER">MAX_VOTING_POWER</a></code> is not a feasible cap, we pick the lowest possible cap.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="committee.md#0x0_committee_new_committee">new_committee</a>(members: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="committee.md#0x0_committee_CommitteeMember">committee::CommitteeMember</a>&gt;): <a href="committee.md#0x0_committee_Committee">committee::Committee</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="committee.md#0x0_committee_new_committee">new_committee</a>(<b>mut</b> members: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="committee.md#0x0_committee_CommitteeMember">CommitteeMember</a>&gt;): <a href="committee.md#0x0_committee_Committee">Committee</a> {
    // If threshold_pct is too small, it's possible that even when all members reach the threshold we still don't
    // have 100%. So we bound the threshold_pct <b>to</b> be always enough <b>to</b> find a solution.
    <b>let</b> threshold = <a href="committee.md#0x0_committee_TOTAL_VOTING_POWER">TOTAL_VOTING_POWER</a>.<b>min</b>(<a href="committee.md#0x0_committee_MAX_VOTING_POWER">MAX_VOTING_POWER</a>.max(<a href="committee.md#0x0_committee_TOTAL_VOTING_POWER">TOTAL_VOTING_POWER</a>.divide_and_round_up(members.length())));
    <b>let</b> remaining_power = <a href="committee.md#0x0_committee_init_voting_power_info">init_voting_power_info</a>(&<b>mut</b> members, threshold);
    <a href="committee.md#0x0_committee_adjust_voting_power">adjust_voting_power</a>(&<b>mut</b> members, threshold, remaining_power);
    <a href="committee.md#0x0_committee_check_invariants">check_invariants</a>(&members);

    // Compute the total aggregated key, e.g. the sum of all <b>public</b> keys in the <a href="committee.md#0x0_committee">committee</a>.
    <b>let</b> total_aggregated_key = <a href="../sui-framework/bls12381.md#0x2_bls12381_uncompressed_g1_to_g1">bls12381::uncompressed_g1_to_g1</a>(
        &<a href="../sui-framework/bls12381.md#0x2_bls12381_uncompressed_g1_sum">bls12381::uncompressed_g1_sum</a>(
            &members.map!(|member| member.protocol_pubkey),
        ),
    );

    <a href="committee.md#0x0_committee_Committee">Committee</a> {
        members,
        total_aggregated_key
    }
}
</code></pre>



</details>

<a name="0x0_committee_empty"></a>

## Function `empty`

Creates an empty committee. Only relevant for init phase.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="committee.md#0x0_committee_empty">empty</a>(): <a href="committee.md#0x0_committee_Committee">committee::Committee</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="committee.md#0x0_committee_empty">empty</a>(): <a href="committee.md#0x0_committee_Committee">Committee</a> {
    <a href="committee.md#0x0_committee_Committee">Committee</a> {
        members: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>[],
        total_aggregated_key: <a href="../sui-framework/bls12381.md#0x2_bls12381_g1_identity">bls12381::g1_identity</a>()
    }
}
</code></pre>



</details>

<a name="0x0_committee_members"></a>

## Function `members`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="committee.md#0x0_committee_members">members</a>(self: &<a href="committee.md#0x0_committee_Committee">committee::Committee</a>): &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="committee.md#0x0_committee_CommitteeMember">committee::CommitteeMember</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="committee.md#0x0_committee_members">members</a>(self: &<a href="committee.md#0x0_committee_Committee">Committee</a>): &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="committee.md#0x0_committee_CommitteeMember">CommitteeMember</a>&gt; {
    &self.members
}
</code></pre>



</details>

<a name="0x0_committee_validator_ids"></a>

## Function `validator_ids`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="committee.md#0x0_committee_validator_ids">validator_ids</a>(self: &<a href="committee.md#0x0_committee_Committee">committee::Committee</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="committee.md#0x0_committee_validator_ids">validator_ids</a>(self: &<a href="committee.md#0x0_committee_Committee">Committee</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;ID&gt; {
    self.<a href="committee.md#0x0_committee_members">members</a>().map_ref!(|m| m.<a href="committee.md#0x0_committee_validator_id">validator_id</a>())
}
</code></pre>



</details>

<a name="0x0_committee_contains"></a>

## Function `contains`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="committee.md#0x0_committee_contains">contains</a>(self: &<a href="committee.md#0x0_committee_Committee">committee::Committee</a>, validator_id: &<a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="committee.md#0x0_committee_contains">contains</a>(self: &<a href="committee.md#0x0_committee_Committee">Committee</a>, validator_id: &ID): bool {
    self.<a href="committee.md#0x0_committee_members">members</a>().any!(|m| m.<a href="committee.md#0x0_committee_validator_id">validator_id</a>() == validator_id)
}
</code></pre>



</details>

<a name="0x0_committee_init_voting_power_info"></a>

## Function `init_voting_power_info`

Create the initial voting power of each member, set using their stake, but capped using threshold.
We also perform insertion sort while creating the voting power list, by maintaining the list in
descending order using voting power.
Anything beyond the threshold is added to the remaining_power, which is also returned.


<pre><code><b>fun</b> <a href="committee.md#0x0_committee_init_voting_power_info">init_voting_power_info</a>(members: &<b>mut</b> <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="committee.md#0x0_committee_CommitteeMember">committee::CommitteeMember</a>&gt;, threshold: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="committee.md#0x0_committee_init_voting_power_info">init_voting_power_info</a>(
    members: &<b>mut</b> <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="committee.md#0x0_committee_CommitteeMember">CommitteeMember</a>&gt;,
    threshold: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <b>let</b> total_stake = <a href="committee.md#0x0_committee_total_stake">total_stake</a>(members);
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = members.length();
    <b>let</b> <b>mut</b> total_power = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> m = &<b>mut</b> members[i];
        <b>let</b> stake = m.stake;
        <b>let</b> adjusted_stake = stake <b>as</b> u128 * (<a href="committee.md#0x0_committee_TOTAL_VOTING_POWER">TOTAL_VOTING_POWER</a> <b>as</b> u128) / (total_stake <b>as</b> u128);
        <b>let</b> voting_power = (adjusted_stake <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>).<b>min</b>(threshold);
        m.voting_power = voting_power;
        total_power = total_power + voting_power;
        i = i + 1;
    };
    <a href="committee.md#0x0_committee_TOTAL_VOTING_POWER">TOTAL_VOTING_POWER</a> - total_power
}
</code></pre>



</details>

<a name="0x0_committee_total_stake"></a>

## Function `total_stake`

Sum up the total stake of all members.


<pre><code><b>fun</b> <a href="committee.md#0x0_committee_total_stake">total_stake</a>(members: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="committee.md#0x0_committee_CommitteeMember">committee::CommitteeMember</a>&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="committee.md#0x0_committee_total_stake">total_stake</a>(members: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="committee.md#0x0_committee_CommitteeMember">CommitteeMember</a>&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = members.length();
    <b>let</b> <b>mut</b> total_stake = 0;
    <b>while</b> (i &lt; len) {
        total_stake = total_stake + members[i].stake;
        i = i + 1;
    };
    total_stake
}
</code></pre>



</details>

<a name="0x0_committee_adjust_voting_power"></a>

## Function `adjust_voting_power`

Distribute remaining_power to members that are not capped at threshold.


<pre><code><b>fun</b> <a href="committee.md#0x0_committee_adjust_voting_power">adjust_voting_power</a>(members: &<b>mut</b> <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="committee.md#0x0_committee_CommitteeMember">committee::CommitteeMember</a>&gt;, threshold: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, remaining_power: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="committee.md#0x0_committee_adjust_voting_power">adjust_voting_power</a>(
    members: &<b>mut</b> <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="committee.md#0x0_committee_CommitteeMember">CommitteeMember</a>&gt;,
    threshold: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    <b>mut</b> remaining_power: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
) {
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = members.length();
    <b>while</b> (i &lt; len && remaining_power &gt; 0) {
        <b>let</b> v = &<b>mut</b> members[i];
        // planned is the amount of extra power we want <b>to</b> distribute <b>to</b> this member.
        <b>let</b> planned = remaining_power.divide_and_round_up(len - i);
        // target is the targeting power this member will reach, capped by threshold.
        <b>let</b> target = threshold.<b>min</b>(v.voting_power + planned);
        // actual is the actual amount of power we will be distributing <b>to</b> this member.
        <b>let</b> actual = remaining_power.<b>min</b>(target - v.voting_power);
        v.voting_power = v.voting_power + actual;
        <b>assert</b>!(v.<a href="committee.md#0x0_committee_voting_power">voting_power</a> &lt;= threshold, <a href="committee.md#0x0_committee_EVotingPowerOverThreshold">EVotingPowerOverThreshold</a>);
        remaining_power = remaining_power - actual;
        i = i + 1;
    };
    <b>assert</b>!(remaining_power == 0, <a href="committee.md#0x0_committee_ETotalPowerMismatch">ETotalPowerMismatch</a>);
}
</code></pre>



</details>

<a name="0x0_committee_check_invariants"></a>

## Function `check_invariants`

Check a few invariants that must hold after setting the voting power.


<pre><code><b>fun</b> <a href="committee.md#0x0_committee_check_invariants">check_invariants</a>(members: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="committee.md#0x0_committee_CommitteeMember">committee::CommitteeMember</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="committee.md#0x0_committee_check_invariants">check_invariants</a>(members: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="committee.md#0x0_committee_CommitteeMember">CommitteeMember</a>&gt;,) {
    // First check that the total voting power must be <a href="committee.md#0x0_committee_TOTAL_VOTING_POWER">TOTAL_VOTING_POWER</a>.
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = members.length();
    <b>let</b> <b>mut</b> total = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> voting_power = members[i].voting_power;
        <b>assert</b>!(voting_power &gt; 0, <a href="committee.md#0x0_committee_EInvalidVotingPower">EInvalidVotingPower</a>);
        total = total + voting_power;
        i = i + 1;
    };
    <b>assert</b>!(total == <a href="committee.md#0x0_committee_TOTAL_VOTING_POWER">TOTAL_VOTING_POWER</a>, <a href="committee.md#0x0_committee_ETotalPowerMismatch">ETotalPowerMismatch</a>);

    // Second check that <b>if</b> member A's stake is larger than B's stake, A's voting power must be no less
    // than B's voting power; similarly, <b>if</b> A's stake is less than B's stake, A's voting power must be no larger
    // than B's voting power.
    <b>let</b> <b>mut</b> a = 0;
    <b>while</b> (a &lt; len) {
        <b>let</b> <b>mut</b> b = a + 1;
        <b>while</b> (b &lt; len) {
            <b>let</b> member_a = &members[a];
            <b>let</b> member_b = &members[b];
            <b>let</b> stake_a = member_a.stake;
            <b>let</b> stake_b = member_b.stake;
            <b>let</b> power_a = member_a.voting_power;
            <b>let</b> power_b = member_b.voting_power;
            <b>if</b> (stake_a &gt; stake_b) {
                <b>assert</b>!(power_a &gt;= power_b, <a href="committee.md#0x0_committee_ERelativePowerMismatch">ERelativePowerMismatch</a>);
            };
            <b>if</b> (stake_a &lt; stake_b) {
                <b>assert</b>!(power_a &lt;= power_b, <a href="committee.md#0x0_committee_ERelativePowerMismatch">ERelativePowerMismatch</a>);
            };
            b = b + 1;
        };
        a = a + 1;
    }
}
</code></pre>



</details>

<a name="0x0_committee_total_voting_power"></a>

## Function `total_voting_power`

Return the (constant) total voting power


<pre><code><b>public</b> <b>fun</b> <a href="committee.md#0x0_committee_total_voting_power">total_voting_power</a>(): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="committee.md#0x0_committee_total_voting_power">total_voting_power</a>(): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <a href="committee.md#0x0_committee_TOTAL_VOTING_POWER">TOTAL_VOTING_POWER</a>
}
</code></pre>



</details>

<a name="0x0_committee_quorum_threshold"></a>

## Function `quorum_threshold`

Return the (constant) quorum threshold


<pre><code><b>public</b> <b>fun</b> <a href="committee.md#0x0_committee_quorum_threshold">quorum_threshold</a>(): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="committee.md#0x0_committee_quorum_threshold">quorum_threshold</a>(): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <a href="committee.md#0x0_committee_QUORUM_THRESHOLD">QUORUM_THRESHOLD</a>
}
</code></pre>



</details>

<a name="0x0_committee_verify_certificate"></a>

## Function `verify_certificate`

Verify an aggregate BLS signature is a certificate in the epoch, and return
the type of certificate and the bytes certified. The <code>signers</code> vector is
an increasing list of indexes into the <code><a href="committee.md#0x0_committee">committee</a></code> vector.
If there is a certificate, the function returns the total stake. Otherwise, it aborts.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="committee.md#0x0_committee_verify_certificate">verify_certificate</a>(self: &<a href="committee.md#0x0_committee_Committee">committee::Committee</a>, signature: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, signers_bitmap: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, intent_bytes: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="committee.md#0x0_committee_verify_certificate">verify_certificate</a>(
    self: &<a href="committee.md#0x0_committee_Committee">Committee</a>,
    signature: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    signers_bitmap: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    intent_bytes: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <b>assert</b>!(signature.length() == <a href="committee.md#0x0_committee_BLS_SIGNATURE_LEN">BLS_SIGNATURE_LEN</a>, <a href="committee.md#0x0_committee_EInvalidSignatureLength">EInvalidSignatureLength</a>);
    <b>let</b> members = &self.members;

    // Use the signers_bitmap <b>to</b> construct the key and the voting_power.

    <b>let</b> <b>mut</b> non_signer_aggregate_voting_power = 0;
    <b>let</b> <b>mut</b> non_signer_public_keys: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;Element&lt;UncompressedG1&gt;&gt; = <a href="../move-stdlib/vector.md#0x1_vector_empty">vector::empty</a>();
    <b>let</b> <b>mut</b> offset: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
    <b>let</b> n_members = members.length();
    <b>let</b> max_bitmap_len_bytes = n_members.divide_and_round_up(8);

    // The signers bitmap must not be longer than necessary <b>to</b> hold all members.
    // It may be shorter, in which case the excluded members are treated <b>as</b> non-signers.
    <b>assert</b>!(signers_bitmap.length() == max_bitmap_len_bytes, <a href="committee.md#0x0_committee_EInvalidBitmap">EInvalidBitmap</a>);

    // Iterate over the signers bitmap and check <b>if</b> each member is a signer.
    max_bitmap_len_bytes.do!(|i| {
        // Get the current byte or 0 <b>if</b> we've reached the end of the bitmap.
        <b>let</b> byte = <b>if</b> (i &lt; signers_bitmap.length()) {
            signers_bitmap[i]
        } <b>else</b> {
            0
        };

        (8u8).do!(|i| {
            <b>let</b> index = offset + (i <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>);
            <b>let</b> is_signer = (byte &gt;&gt; i) & 1 == 1;

            // If the index is out of bounds, the bit must be 0 <b>to</b> ensure
            // uniqueness of the signers_bitmap.
            <b>if</b> (index &gt;= n_members) {
                <b>assert</b>!(!is_signer, <a href="committee.md#0x0_committee_EInvalidBitmap">EInvalidBitmap</a>);
                <b>return</b>
            };

            // There will be fewer non-signers than signers, so we handle
            // non-signers here.
            <b>if</b> (!is_signer) {
                <b>let</b> member = &members[index];
                non_signer_aggregate_voting_power = non_signer_aggregate_voting_power + member.voting_power;
                non_signer_public_keys.push_back(member.protocol_pubkey);
            };
        });
        offset = offset + 8;
    });

    // Compute the aggregate voting_power <b>as</b> the difference between the total voting power
    // and the total voting power of the non-signers.
    <b>let</b> aggregate_voting_power = <a href="committee.md#0x0_committee_TOTAL_VOTING_POWER">TOTAL_VOTING_POWER</a> - non_signer_aggregate_voting_power;

    <b>assert</b>!(<a href="committee.md#0x0_committee_verify_quorum">verify_quorum</a>(aggregate_voting_power), <a href="committee.md#0x0_committee_ENotEnoughStake">ENotEnoughStake</a>);


    // Compute the aggregate <b>public</b> key <b>as</b> the difference between the total
    // aggregated key and the sum of the non-signer <b>public</b> keys.
    <b>let</b> aggregate_key = <a href="../sui-framework/bls12381.md#0x2_bls12381_g1_sub">bls12381::g1_sub</a>(
        &self.total_aggregated_key,
        &<a href="../sui-framework/bls12381.md#0x2_bls12381_uncompressed_g1_to_g1">bls12381::uncompressed_g1_to_g1</a>(
            &<a href="../sui-framework/bls12381.md#0x2_bls12381_uncompressed_g1_sum">bls12381::uncompressed_g1_sum</a>(&non_signer_public_keys),
        ),
    );

    // Verify the signature
    <b>let</b> pub_key_bytes = <a href="../sui-framework/group_ops.md#0x2_group_ops_bytes">group_ops::bytes</a>(&aggregate_key);
    <b>assert</b>!(
        <a href="../sui-framework/bls12381.md#0x2_bls12381_bls12381_min_pk_verify">bls12381::bls12381_min_pk_verify</a>(
            signature,
            pub_key_bytes,
            intent_bytes,
        ),
        <a href="committee.md#0x0_committee_EInvalidSignature">EInvalidSignature</a>,
    );

    aggregate_voting_power
}
</code></pre>



</details>

<a name="0x0_committee_verify_quorum"></a>

## Function `verify_quorum`

Returns true if the voting power is more than the aggregate voting power of quorum members of a committee.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="committee.md#0x0_committee_verify_quorum">verify_quorum</a>(aggregate_voting_power: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="committee.md#0x0_committee_verify_quorum">verify_quorum</a>(aggregate_voting_power: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): bool {
    aggregate_voting_power &gt;= <a href="committee.md#0x0_committee_QUORUM_THRESHOLD">QUORUM_THRESHOLD</a>
}
</code></pre>



</details>
