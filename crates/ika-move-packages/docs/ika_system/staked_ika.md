---
title: Module `(ika_system=0x0)::staked_ika`
---

Implements the <code><a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a></code> functionality - a staked IKA is an object that
represents a staked amount of IKAs in a staking pool. It is created in the
<code>staking_pool</code> on staking and can be split, joined, and burned. The burning
is performed via the <code>withdraw_stake</code> method in the <code>staking_pool</code>.


-  [Struct `StakedIka`](#(ika_system=0x0)_staked_ika_StakedIka)
-  [Enum `StakedIkaState`](#(ika_system=0x0)_staked_ika_StakedIkaState)
-  [Constants](#@Constants_0)
-  [Function `mint`](#(ika_system=0x0)_staked_ika_mint)
-  [Function `into_balance`](#(ika_system=0x0)_staked_ika_into_balance)
-  [Function `set_withdrawing`](#(ika_system=0x0)_staked_ika_set_withdrawing)
-  [Function `can_withdraw_early`](#(ika_system=0x0)_staked_ika_can_withdraw_early)
-  [Function `validator_id`](#(ika_system=0x0)_staked_ika_validator_id)
-  [Function `value`](#(ika_system=0x0)_staked_ika_value)
-  [Function `activation_epoch`](#(ika_system=0x0)_staked_ika_activation_epoch)
-  [Function `is_staked`](#(ika_system=0x0)_staked_ika_is_staked)
-  [Function `is_withdrawing`](#(ika_system=0x0)_staked_ika_is_withdrawing)
-  [Function `withdraw_epoch`](#(ika_system=0x0)_staked_ika_withdraw_epoch)
-  [Function `join`](#(ika_system=0x0)_staked_ika_join)
-  [Function `split`](#(ika_system=0x0)_staked_ika_split)


<pre><code><b>use</b> (ika=0x0)::ika;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../sui/address.md#sui_address">sui::address</a>;
<b>use</b> <a href="../sui/bag.md#sui_bag">sui::bag</a>;
<b>use</b> <a href="../sui/balance.md#sui_balance">sui::balance</a>;
<b>use</b> <a href="../sui/coin.md#sui_coin">sui::coin</a>;
<b>use</b> <a href="../sui/config.md#sui_config">sui::config</a>;
<b>use</b> <a href="../sui/deny_list.md#sui_deny_list">sui::deny_list</a>;
<b>use</b> <a href="../sui/dynamic_field.md#sui_dynamic_field">sui::dynamic_field</a>;
<b>use</b> <a href="../sui/dynamic_object_field.md#sui_dynamic_object_field">sui::dynamic_object_field</a>;
<b>use</b> <a href="../sui/event.md#sui_event">sui::event</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/table.md#sui_table">sui::table</a>;
<b>use</b> <a href="../sui/transfer.md#sui_transfer">sui::transfer</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
<b>use</b> <a href="../sui/types.md#sui_types">sui::types</a>;
<b>use</b> <a href="../sui/url.md#sui_url">sui::url</a>;
<b>use</b> <a href="../sui/vec_set.md#sui_vec_set">sui::vec_set</a>;
</code></pre>



<a name="(ika_system=0x0)_staked_ika_StakedIka"></a>

## Struct `StakedIka`

Represents a staked IKA, does not store the <code>Balance</code> inside, but uses
<code>u64</code> to represent the staked amount. Behaves similarly to <code>Balance</code> and
<code>Coin</code> providing methods to <code><a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_split">split</a></code> and <code><a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_join">join</a></code>.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>state: (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIkaState">staked_ika::StakedIkaState</a></code>
</dt>
<dd>
 Whether the staked IKA is active or withdrawing.
</dd>
<dt>
<code><a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the staking pool.
</dd>
<dt>
<code>principal: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;</code>
</dt>
<dd>
 The staked amount.
</dd>
<dt>
<code><a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_activation_epoch">activation_epoch</a>: u64</code>
</dt>
<dd>
 The Ikarus epoch when the staked IKA was activated.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_staked_ika_StakedIkaState"></a>

## Enum `StakedIkaState`

The state of the staked IKA. It can be either <code>Staked</code> or <code>Withdrawing</code>.
The <code>Withdrawing</code> state contains the epoch when the staked IKA can be
withdrawn.


<pre><code><b>public</b> <b>enum</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIkaState">StakedIkaState</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Variants</summary>


<dl>
<dt>
Variant <code>Staked</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>Withdrawing</code>
</dt>
<dd>
</dd>

<dl>
<dt>
<code><a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_withdraw_epoch">withdraw_epoch</a>: u64</code>
</dt>
<dd>
</dd>
</dl>

</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="(ika_system=0x0)_staked_ika_EAlreadyWithdrawing"></a>

Trying to mark stake as withdrawing when it is already marked as withdrawing.


<pre><code><b>const</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_EAlreadyWithdrawing">EAlreadyWithdrawing</a>: u64 = 6;
</code></pre>



<a name="(ika_system=0x0)_staked_ika_EInvalidAmount"></a>

The amount for the split is invalid.


<pre><code><b>const</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_EInvalidAmount">EInvalidAmount</a>: u64 = 2;
</code></pre>



<a name="(ika_system=0x0)_staked_ika_EMetadataMismatch"></a>

The metadata of two <code><a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a></code> objects does not match.


<pre><code><b>const</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_EMetadataMismatch">EMetadataMismatch</a>: u64 = 1;
</code></pre>



<a name="(ika_system=0x0)_staked_ika_ENotWithdrawing"></a>

The <code><a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a></code> is not in <code>Withdrawing</code> state.


<pre><code><b>const</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_ENotWithdrawing">ENotWithdrawing</a>: u64 = 0;
</code></pre>



<a name="(ika_system=0x0)_staked_ika_EStakeBelowThreshold"></a>

Stake is below the minimum staking threshold.


<pre><code><b>const</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_EStakeBelowThreshold">EStakeBelowThreshold</a>: u64 = 7;
</code></pre>



<a name="(ika_system=0x0)_staked_ika_MIN_STAKING_THRESHOLD"></a>

StakedIka objects must have a principal with at least this amount.


<pre><code><b>const</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>: u64 = 1000000000;
</code></pre>



<a name="(ika_system=0x0)_staked_ika_mint"></a>

## Function `mint`

Protected method to create a new staked IKA.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_mint">mint</a>(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, principal: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_activation_epoch">activation_epoch</a>: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_mint">mint</a>(
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a>: ID,
    principal: Balance&lt;IKA&gt;,
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_activation_epoch">activation_epoch</a>: u64,
    ctx: &<b>mut</b> TxContext,
): <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a> {
    <b>assert</b>!(principal.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_value">value</a>() &gt;= <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_EStakeBelowThreshold">EStakeBelowThreshold</a>);
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a> {
        id: object::new(ctx),
        state: StakedIkaState::Staked,
        <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a>,
        principal,
        <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_activation_epoch">activation_epoch</a>,
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staked_ika_into_balance"></a>

## Function `into_balance`

Burns the staked IKA and returns the <code>principal</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_into_balance">into_balance</a>(sw: (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_into_balance">into_balance</a>(sw: <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a>): Balance&lt;IKA&gt; {
    <b>let</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a> { id, principal, .. } = sw;
    id.delete();
    principal
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staked_ika_set_withdrawing"></a>

## Function `set_withdrawing`

Sets the staked IKA state to <code>Withdrawing</code>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_set_withdrawing">set_withdrawing</a>(sw: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_withdraw_epoch">withdraw_epoch</a>: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_set_withdrawing">set_withdrawing</a>(sw: &<b>mut</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_withdraw_epoch">withdraw_epoch</a>: u64) {
    <b>assert</b>!(sw.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_is_staked">is_staked</a>(), <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_EAlreadyWithdrawing">EAlreadyWithdrawing</a>);
    sw.state = StakedIkaState::Withdrawing { <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_withdraw_epoch">withdraw_epoch</a> };
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staked_ika_can_withdraw_early"></a>

## Function `can_withdraw_early`

Checks if the staked IKA can be withdrawn directly.

The staked IKA can be withdrawn early if:
- activation epoch is current epoch + 2
- activation epoch is current epoch + 1 and !node_in_next_committee
(or committee not selected yet)


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_can_withdraw_early">can_withdraw_early</a>(sw: &(ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>, node_in_next_committee: bool, current_epoch: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_can_withdraw_early">can_withdraw_early</a>(
    sw: &<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a>,
    node_in_next_committee: bool,
    current_epoch: u64,
): bool {
    <b>if</b> (sw.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_is_withdrawing">is_withdrawing</a>()) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_activation_epoch">activation_epoch</a> = sw.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_activation_epoch">activation_epoch</a>;
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_activation_epoch">activation_epoch</a> == current_epoch + 2 ||
    (sw.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_activation_epoch">activation_epoch</a> == current_epoch + 1 && !node_in_next_committee)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staked_ika_validator_id"></a>

## Function `validator_id`

Returns the <code><a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a></code> of the staked IKA.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a>(sw: &(ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a>(sw: &<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a>): ID { sw.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a> }
</code></pre>



</details>

<a name="(ika_system=0x0)_staked_ika_value"></a>

## Function `value`

Returns the <code>principal</code> of the staked IKA. Called <code><a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_value">value</a></code> to be consistent
with <code>Coin</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_value">value</a>(sw: &(ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_value">value</a>(sw: &<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a>): u64 { sw.principal.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_value">value</a>() }
</code></pre>



</details>

<a name="(ika_system=0x0)_staked_ika_activation_epoch"></a>

## Function `activation_epoch`

Returns the <code><a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_activation_epoch">activation_epoch</a></code> of the staked IKA.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_activation_epoch">activation_epoch</a>(sw: &(ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_activation_epoch">activation_epoch</a>(sw: &<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a>): u64 { sw.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_activation_epoch">activation_epoch</a> }
</code></pre>



</details>

<a name="(ika_system=0x0)_staked_ika_is_staked"></a>

## Function `is_staked`

Returns true if the staked IKA is in the <code>Staked</code> state.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_is_staked">is_staked</a>(sw: &(ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_is_staked">is_staked</a>(sw: &<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a>): bool { sw.state == StakedIkaState::Staked }
</code></pre>



</details>

<a name="(ika_system=0x0)_staked_ika_is_withdrawing"></a>

## Function `is_withdrawing`

Checks whether the staked IKA is in the <code>Withdrawing</code> state.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_is_withdrawing">is_withdrawing</a>(sw: &(ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_is_withdrawing">is_withdrawing</a>(sw: &<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a>): bool {
    match (sw.state) {
        StakedIkaState::Withdrawing { .. } =&gt; <b>true</b>,
        _ =&gt; <b>false</b>,
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staked_ika_withdraw_epoch"></a>

## Function `withdraw_epoch`

Returns the <code><a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_withdraw_epoch">withdraw_epoch</a></code> of the staked IKA if it is in the <code>Withdrawing</code>.
Aborts otherwise.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_withdraw_epoch">withdraw_epoch</a>(sw: &(ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_withdraw_epoch">withdraw_epoch</a>(sw: &<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a>): u64 {
    match (sw.state) {
        StakedIkaState::Withdrawing { <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_withdraw_epoch">withdraw_epoch</a>, .. } =&gt; <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_withdraw_epoch">withdraw_epoch</a>,
        _ =&gt; <b>abort</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_ENotWithdrawing">ENotWithdrawing</a>,
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staked_ika_join"></a>

## Function `join`

Joins the staked IKA with another staked IKA, adding the <code>principal</code> of the
<code>other</code> staked IKA to the current staked IKA.

Aborts if the <code><a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a></code> or <code><a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_activation_epoch">activation_epoch</a></code> of the staked IKAs do not match.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_join">join</a>(sw: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>, other: (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_join">join</a>(sw: &<b>mut</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a>, other: <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a>) {
    <b>assert</b>!(sw.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a> == other.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_EMetadataMismatch">EMetadataMismatch</a>);
    <b>assert</b>!(sw.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_activation_epoch">activation_epoch</a> == other.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_activation_epoch">activation_epoch</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_EMetadataMismatch">EMetadataMismatch</a>);
    // Simple scenario - staked ika is in `Staked` state. We guarantee that the
    // metadata is identical: same activation epoch and both are in the same state.
    <b>if</b> (sw.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_is_staked">is_staked</a>()) {
        <b>assert</b>!(other.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_is_staked">is_staked</a>(), <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_EMetadataMismatch">EMetadataMismatch</a>);
        <b>let</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a> { id, principal, .. } = other;
        sw.principal.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_join">join</a>(principal);
        id.delete();
        <b>return</b>
    };
    // Withdrawing scenario - we no longer check that the activation epoch is
    // the same, <b>as</b> the staked IKA is in the process of withdrawing. Instead,
    // we make sure that the withdraw epoch is the same.
    <b>assert</b>!(sw.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_is_withdrawing">is_withdrawing</a>() && other.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_is_withdrawing">is_withdrawing</a>(), <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_EMetadataMismatch">EMetadataMismatch</a>);
    <b>assert</b>!(sw.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_withdraw_epoch">withdraw_epoch</a>() == other.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_withdraw_epoch">withdraw_epoch</a>(), <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_EMetadataMismatch">EMetadataMismatch</a>);
    <b>let</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a> { id, principal, .. } = other;
    sw.principal.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_join">join</a>(principal);
    id.delete();
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staked_ika_split"></a>

## Function `split`

Splits the staked IKA into two parts, one with the <code>amount</code> and the other
with the remaining <code>principal</code>. The <code><a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a></code>, <code><a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_activation_epoch">activation_epoch</a></code> are the
same for both the staked IKAs.

Aborts if the <code>amount</code> is greater than the <code>principal</code> of the staked IKA.
Aborts if the <code>amount</code> is zero.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_split">split</a>(sw: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>, amount: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_split">split</a>(sw: &<b>mut</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a>, amount: u64, ctx: &<b>mut</b> TxContext): <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a> {
    <b>assert</b>!(sw.principal.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_value">value</a>() &gt; amount, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_EInvalidAmount">EInvalidAmount</a>);
    // Both parts after the <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_split">split</a> must have a principal of at least <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>.
    <b>assert</b>!(amount &gt;= <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_EStakeBelowThreshold">EStakeBelowThreshold</a>);
    <b>assert</b>!(sw.principal.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_value">value</a>() - amount &gt;= <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_EStakeBelowThreshold">EStakeBelowThreshold</a>);
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a> {
        id: object::new(ctx),
        state: sw.state, // state is preserved
        <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a>: sw.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a>,
        principal: sw.principal.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_split">split</a>(amount),
        <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_activation_epoch">activation_epoch</a>: sw.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_activation_epoch">activation_epoch</a>,
    }
}
</code></pre>



</details>
