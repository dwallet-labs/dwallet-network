---
title: Module `(ika_system=0x0)::staked_ika`
---



-  [Struct `StakedIka`](#(ika_system=0x0)_staked_ika_StakedIka)
-  [Struct `FungibleStakedIka`](#(ika_system=0x0)_staked_ika_FungibleStakedIka)
-  [Constants](#@Constants_0)
-  [Function `create`](#(ika_system=0x0)_staked_ika_create)
-  [Function `into_balance`](#(ika_system=0x0)_staked_ika_into_balance)
-  [Function `create_fungible`](#(ika_system=0x0)_staked_ika_create_fungible)
-  [Function `validator_id`](#(ika_system=0x0)_staked_ika_validator_id)
-  [Function `fungible_staked_ika_validator_id`](#(ika_system=0x0)_staked_ika_fungible_staked_ika_validator_id)
-  [Function `staked_ika_amount`](#(ika_system=0x0)_staked_ika_staked_ika_amount)
-  [Function `stake_activation_epoch`](#(ika_system=0x0)_staked_ika_stake_activation_epoch)
-  [Function `fungible_staked_ika_value`](#(ika_system=0x0)_staked_ika_fungible_staked_ika_value)
-  [Function `split_fungible_staked_ika`](#(ika_system=0x0)_staked_ika_split_fungible_staked_ika)
-  [Function `join_fungible_staked_ika`](#(ika_system=0x0)_staked_ika_join_fungible_staked_ika)
-  [Function `split`](#(ika_system=0x0)_staked_ika_split)
-  [Function `split_staked_ika`](#(ika_system=0x0)_staked_ika_split_staked_ika)
-  [Function `join_staked_ika`](#(ika_system=0x0)_staked_ika_join_staked_ika)
-  [Function `is_equal_staking_metadata`](#(ika_system=0x0)_staked_ika_is_equal_staking_metadata)
-  [Function `destroy`](#(ika_system=0x0)_staked_ika_destroy)


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

A self-custodial object holding the staked IKA tokens.


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
<code><a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the validator we are staking with.
</dd>
<dt>
<code><a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_stake_activation_epoch">stake_activation_epoch</a>: u64</code>
</dt>
<dd>
 The epoch at which the stake becomes active.
</dd>
<dt>
<code>principal: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;</code>
</dt>
<dd>
 The staked IKA tokens.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_staked_ika_FungibleStakedIka"></a>

## Struct `FungibleStakedIka`

An alternative to <code><a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a></code> that holds the pool token amount instead of the IKA balance.
StakedIka objects can be converted to FungibleStakedIkas after the initial warmup period.
The advantage of this is that you can now merge multiple StakedIka objects from different
activation epochs into a single FungibleStakedIka object.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_FungibleStakedIka">FungibleStakedIka</a> <b>has</b> key, store
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
<code><a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the validator we are staking with.
</dd>
<dt>
<code>value: u64</code>
</dt>
<dd>
 The pool token amount.
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="(ika_system=0x0)_staked_ika_EIncompatibleStakedIka"></a>



<pre><code><b>const</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_EIncompatibleStakedIka">EIncompatibleStakedIka</a>: u64 = 12;
</code></pre>



<a name="(ika_system=0x0)_staked_ika_EInsufficientIkaTokenBalance"></a>



<pre><code><b>const</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_EInsufficientIkaTokenBalance">EInsufficientIkaTokenBalance</a>: u64 = 3;
</code></pre>



<a name="(ika_system=0x0)_staked_ika_EInsufficientPoolTokenBalance"></a>



<pre><code><b>const</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_EInsufficientPoolTokenBalance">EInsufficientPoolTokenBalance</a>: u64 = 0;
</code></pre>



<a name="(ika_system=0x0)_staked_ika_EStakedIkaBelowThreshold"></a>



<pre><code><b>const</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_EStakedIkaBelowThreshold">EStakedIkaBelowThreshold</a>: u64 = 18;
</code></pre>



<a name="(ika_system=0x0)_staked_ika_EWrongPool"></a>



<pre><code><b>const</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_EWrongPool">EWrongPool</a>: u64 = 1;
</code></pre>



<a name="(ika_system=0x0)_staked_ika_MIN_STAKING_THRESHOLD"></a>

StakedIka objects cannot be split to below this amount.


<pre><code><b>const</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>: u64 = 1000000000;
</code></pre>



<a name="(ika_system=0x0)_staked_ika_create"></a>

## Function `create`

Create a new staked ika.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_create">create</a>(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_stake_activation_epoch">stake_activation_epoch</a>: u64, principal: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_create">create</a>(
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a>: ID,
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_stake_activation_epoch">stake_activation_epoch</a>: u64,
    principal: Balance&lt;IKA&gt;,
    ctx: &<b>mut</b> TxContext,
): <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a> {
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a> {
        id: object::new(ctx),
        <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a>,
        <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_stake_activation_epoch">stake_activation_epoch</a>,
        principal
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staked_ika_into_balance"></a>

## Function `into_balance`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_into_balance">into_balance</a>(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_into_balance">into_balance</a>(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a>): Balance&lt;IKA&gt; {
    <b>let</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a> {
        id,
        <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a>: _,
        <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_stake_activation_epoch">stake_activation_epoch</a>: _,
        principal,
    } = <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>;
    object::delete(id);
    principal
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staked_ika_create_fungible"></a>

## Function `create_fungible`

Create a new fungible staked ika.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_create_fungible">create_fungible</a>(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, value: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_create_fungible">create_fungible</a>(
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a>: ID,
    value: u64,
    ctx: &<b>mut</b> TxContext,
): <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_FungibleStakedIka">FungibleStakedIka</a> {
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_FungibleStakedIka">FungibleStakedIka</a> {
        id: object::new(ctx),
        <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a>,
        value
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staked_ika_validator_id"></a>

## Function `validator_id`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a>(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: &(ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a>(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: &<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a>): ID { <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a> }
</code></pre>



</details>

<a name="(ika_system=0x0)_staked_ika_fungible_staked_ika_validator_id"></a>

## Function `fungible_staked_ika_validator_id`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_fungible_staked_ika_validator_id">fungible_staked_ika_validator_id</a>(fungible_staked_ika: &(ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_fungible_staked_ika_validator_id">fungible_staked_ika_validator_id</a>(fungible_staked_ika: &<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_FungibleStakedIka">FungibleStakedIka</a>): ID {
    fungible_staked_ika.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staked_ika_staked_ika_amount"></a>

## Function `staked_ika_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_staked_ika_amount">staked_ika_amount</a>(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: &(ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_staked_ika_amount">staked_ika_amount</a>(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: &<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a>): u64 { <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.principal.value() }
</code></pre>



</details>

<a name="(ika_system=0x0)_staked_ika_stake_activation_epoch"></a>

## Function `stake_activation_epoch`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_stake_activation_epoch">stake_activation_epoch</a>(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: &(ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_stake_activation_epoch">stake_activation_epoch</a>(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: &<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a>): u64 {
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_stake_activation_epoch">stake_activation_epoch</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staked_ika_fungible_staked_ika_value"></a>

## Function `fungible_staked_ika_value`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_fungible_staked_ika_value">fungible_staked_ika_value</a>(fungible_staked_ika: &(ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_fungible_staked_ika_value">fungible_staked_ika_value</a>(fungible_staked_ika: &<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_FungibleStakedIka">FungibleStakedIka</a>): u64 {
    fungible_staked_ika.value
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staked_ika_split_fungible_staked_ika"></a>

## Function `split_fungible_staked_ika`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_split_fungible_staked_ika">split_fungible_staked_ika</a>(fungible_staked_ika: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>, split_amount: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_split_fungible_staked_ika">split_fungible_staked_ika</a>(
    fungible_staked_ika: &<b>mut</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_FungibleStakedIka">FungibleStakedIka</a>,
    split_amount: u64,
    ctx: &<b>mut</b> TxContext,
): <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_FungibleStakedIka">FungibleStakedIka</a> {
    <b>assert</b>!(split_amount &lt;= fungible_staked_ika.value, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_EInsufficientPoolTokenBalance">EInsufficientPoolTokenBalance</a>);
    fungible_staked_ika.value = fungible_staked_ika.value - split_amount;
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_FungibleStakedIka">FungibleStakedIka</a> {
        id: object::new(ctx),
        <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a>: fungible_staked_ika.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a>,
        value: split_amount,
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staked_ika_join_fungible_staked_ika"></a>

## Function `join_fungible_staked_ika`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_join_fungible_staked_ika">join_fungible_staked_ika</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>, other: (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_join_fungible_staked_ika">join_fungible_staked_ika</a>(self: &<b>mut</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_FungibleStakedIka">FungibleStakedIka</a>, other: <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_FungibleStakedIka">FungibleStakedIka</a>) {
    <b>let</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_FungibleStakedIka">FungibleStakedIka</a> { id, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a>, value } = other;
    <b>assert</b>!(self.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a> == <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_EWrongPool">EWrongPool</a>);
    object::delete(id);
    self.value = self.value + value;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staked_ika_split"></a>

## Function `split`

Split StakedIka <code>self</code> to two parts, one with principal <code>split_amount</code>,
and the remaining principal is left in <code>self</code>.
All the other parameters of the StakedIka like <code><a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_stake_activation_epoch">stake_activation_epoch</a></code> or <code>pool_id</code> remain the same.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_split">split</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>, split_amount: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_split">split</a>(self: &<b>mut</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a>, split_amount: u64, ctx: &<b>mut</b> TxContext): <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a> {
    <b>let</b> original_amount = self.principal.value();
    <b>assert</b>!(split_amount &lt;= original_amount, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_EInsufficientIkaTokenBalance">EInsufficientIkaTokenBalance</a>);
    <b>let</b> remaining_amount = original_amount - split_amount;
    // Both resulting parts should have at least <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>.
    <b>assert</b>!(remaining_amount &gt;= <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_EStakedIkaBelowThreshold">EStakedIkaBelowThreshold</a>);
    <b>assert</b>!(split_amount &gt;= <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_EStakedIkaBelowThreshold">EStakedIkaBelowThreshold</a>);
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a> {
        id: object::new(ctx),
        <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a>: self.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a>,
        <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_stake_activation_epoch">stake_activation_epoch</a>: self.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_stake_activation_epoch">stake_activation_epoch</a>,
        principal: self.principal.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_split">split</a>(split_amount),
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staked_ika_split_staked_ika"></a>

## Function `split_staked_ika`

Split the given StakedIka to the two parts, one with principal <code>split_amount</code>,
transfer the newly split part to the sender address.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_split_staked_ika">split_staked_ika</a>(stake: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>, split_amount: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_split_staked_ika">split_staked_ika</a>(stake: &<b>mut</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a>, split_amount: u64, ctx: &<b>mut</b> TxContext) {
    transfer::transfer(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_split">split</a>(stake, split_amount, ctx), ctx.sender());
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staked_ika_join_staked_ika"></a>

## Function `join_staked_ika`

Consume the staked ika <code>other</code> and add its value to <code>self</code>.
Aborts if some of the staking parameters are incompatible (pool id, stake activation epoch, etc.)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_join_staked_ika">join_staked_ika</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>, other: (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_join_staked_ika">join_staked_ika</a>(self: &<b>mut</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a>, other: <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a>) {
    <b>assert</b>!(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_is_equal_staking_metadata">is_equal_staking_metadata</a>(self, &other), <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_EIncompatibleStakedIka">EIncompatibleStakedIka</a>);
    <b>let</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a> {
        id,
        <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a>: _,
        <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_stake_activation_epoch">stake_activation_epoch</a>: _,
        principal,
    } = other;
    id.delete();
    self.principal.join(principal);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staked_ika_is_equal_staking_metadata"></a>

## Function `is_equal_staking_metadata`

Returns true if all the staking parameters of the staked ika except the principal are identical


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_is_equal_staking_metadata">is_equal_staking_metadata</a>(self: &(ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>, other: &(ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_is_equal_staking_metadata">is_equal_staking_metadata</a>(self: &<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a>, other: &<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">StakedIka</a>): bool {
    (self.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a> == other.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_validator_id">validator_id</a>) &&
        (self.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_stake_activation_epoch">stake_activation_epoch</a> == other.<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_stake_activation_epoch">stake_activation_epoch</a>)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staked_ika_destroy"></a>

## Function `destroy`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_destroy">destroy</a>(fungible_staked_ika: (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_destroy">destroy</a>(
    fungible_staked_ika: <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_FungibleStakedIka">FungibleStakedIka</a>
) {
    <b>let</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_FungibleStakedIka">FungibleStakedIka</a> { id, .. } = fungible_staked_ika;
    id.delete();
}
</code></pre>



</details>
