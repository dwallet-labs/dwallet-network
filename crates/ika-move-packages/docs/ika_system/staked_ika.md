---
title: Module `0x0::staked_ika`
---



-  [Resource `StakedIka`](#0x0_staked_ika_StakedIka)
-  [Resource `FungibleStakedIka`](#0x0_staked_ika_FungibleStakedIka)
-  [Constants](#@Constants_0)
-  [Function `create`](#0x0_staked_ika_create)
-  [Function `into_balance`](#0x0_staked_ika_into_balance)
-  [Function `create_fungible`](#0x0_staked_ika_create_fungible)
-  [Function `validator_id`](#0x0_staked_ika_validator_id)
-  [Function `fungible_staked_ika_validator_id`](#0x0_staked_ika_fungible_staked_ika_validator_id)
-  [Function `staked_ika_amount`](#0x0_staked_ika_staked_ika_amount)
-  [Function `stake_activation_epoch`](#0x0_staked_ika_stake_activation_epoch)
-  [Function `fungible_staked_ika_value`](#0x0_staked_ika_fungible_staked_ika_value)
-  [Function `split_fungible_staked_ika`](#0x0_staked_ika_split_fungible_staked_ika)
-  [Function `join_fungible_staked_ika`](#0x0_staked_ika_join_fungible_staked_ika)
-  [Function `split`](#0x0_staked_ika_split)
-  [Function `split_staked_ika`](#0x0_staked_ika_split_staked_ika)
-  [Function `join_staked_ika`](#0x0_staked_ika_join_staked_ika)
-  [Function `is_equal_staking_metadata`](#0x0_staked_ika_is_equal_staking_metadata)
-  [Function `destroy`](#0x0_staked_ika_destroy)


<pre><code><b>use</b> <a href="../ika/ika.md#0x0_ika">0x0::ika</a>;
<b>use</b> <a href="../sui-framework/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
</code></pre>



<a name="0x0_staked_ika_StakedIka"></a>

## Resource `StakedIka`

A self-custodial object holding the staked IKA tokens.


<pre><code><b>struct</b> <a href="staked_ika.md#0x0_staked_ika_StakedIka">StakedIka</a> <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 ID of the validator we are staking with.
</dd>
<dt>
<code>stake_activation_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 The epoch at which the stake becomes active.
</dd>
<dt>
<code>principal: <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;</code>
</dt>
<dd>
 The staked IKA tokens.
</dd>
</dl>


</details>

<a name="0x0_staked_ika_FungibleStakedIka"></a>

## Resource `FungibleStakedIka`

An alternative to <code><a href="staked_ika.md#0x0_staked_ika_StakedIka">StakedIka</a></code> that holds the pool token amount instead of the IKA balance.
StakedIka objects can be converted to FungibleStakedIkas after the initial warmup period.
The advantage of this is that you can now merge multiple StakedIka objects from different
activation epochs into a single FungibleStakedIka object.


<pre><code><b>struct</b> <a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">FungibleStakedIka</a> <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 ID of the validator we are staking with.
</dd>
<dt>
<code>value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 The pool token amount.
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x0_staked_ika_EIncompatibleStakedIka"></a>



<pre><code><b>const</b> <a href="staked_ika.md#0x0_staked_ika_EIncompatibleStakedIka">EIncompatibleStakedIka</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 12;
</code></pre>



<a name="0x0_staked_ika_EInsufficientIkaTokenBalance"></a>



<pre><code><b>const</b> <a href="staked_ika.md#0x0_staked_ika_EInsufficientIkaTokenBalance">EInsufficientIkaTokenBalance</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 3;
</code></pre>



<a name="0x0_staked_ika_EInsufficientPoolTokenBalance"></a>



<pre><code><b>const</b> <a href="staked_ika.md#0x0_staked_ika_EInsufficientPoolTokenBalance">EInsufficientPoolTokenBalance</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
</code></pre>



<a name="0x0_staked_ika_EStakedIkaBelowThreshold"></a>



<pre><code><b>const</b> <a href="staked_ika.md#0x0_staked_ika_EStakedIkaBelowThreshold">EStakedIkaBelowThreshold</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 18;
</code></pre>



<a name="0x0_staked_ika_EWrongPool"></a>



<pre><code><b>const</b> <a href="staked_ika.md#0x0_staked_ika_EWrongPool">EWrongPool</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1;
</code></pre>



<a name="0x0_staked_ika_MIN_STAKING_THRESHOLD"></a>

StakedIka objects cannot be split to below this amount.


<pre><code><b>const</b> <a href="staked_ika.md#0x0_staked_ika_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1000000000;
</code></pre>



<a name="0x0_staked_ika_create"></a>

## Function `create`

Create a new staked ika.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="staked_ika.md#0x0_staked_ika_create">create</a>(validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, stake_activation_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, principal: <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="staked_ika.md#0x0_staked_ika_StakedIka">staked_ika::StakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="staked_ika.md#0x0_staked_ika_create">create</a>(
    validator_id: ID,
    stake_activation_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    principal: Balance&lt;IKA&gt;,
    ctx: &<b>mut</b> TxContext,
): <a href="staked_ika.md#0x0_staked_ika_StakedIka">StakedIka</a> {
    <a href="staked_ika.md#0x0_staked_ika_StakedIka">StakedIka</a> {
        id: <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx),
        validator_id,
        stake_activation_epoch,
        principal
    }
}
</code></pre>



</details>

<a name="0x0_staked_ika_into_balance"></a>

## Function `into_balance`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="staked_ika.md#0x0_staked_ika_into_balance">into_balance</a>(<a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: <a href="staked_ika.md#0x0_staked_ika_StakedIka">staked_ika::StakedIka</a>): <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="staked_ika.md#0x0_staked_ika_into_balance">into_balance</a>(<a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: <a href="staked_ika.md#0x0_staked_ika_StakedIka">StakedIka</a>): Balance&lt;IKA&gt; {
    <b>let</b> <a href="staked_ika.md#0x0_staked_ika_StakedIka">StakedIka</a> {
        id,
        validator_id: _,
        stake_activation_epoch: _,
        principal,
    } = <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>;
    <a href="../sui-framework/object.md#0x2_object_delete">object::delete</a>(id);
    principal
}
</code></pre>



</details>

<a name="0x0_staked_ika_create_fungible"></a>

## Function `create_fungible`

Create a new fungible staked ika.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="staked_ika.md#0x0_staked_ika_create_fungible">create_fungible</a>(validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="staked_ika.md#0x0_staked_ika_create_fungible">create_fungible</a>(
    validator_id: ID,
    value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    ctx: &<b>mut</b> TxContext,
): <a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">FungibleStakedIka</a> {
    <a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">FungibleStakedIka</a> {
        id: <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx),
        validator_id,
        value
    }
}
</code></pre>



</details>

<a name="0x0_staked_ika_validator_id"></a>

## Function `validator_id`



<pre><code><b>public</b> <b>fun</b> <a href="staked_ika.md#0x0_staked_ika_validator_id">validator_id</a>(<a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: &<a href="staked_ika.md#0x0_staked_ika_StakedIka">staked_ika::StakedIka</a>): <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="staked_ika.md#0x0_staked_ika_validator_id">validator_id</a>(<a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: &<a href="staked_ika.md#0x0_staked_ika_StakedIka">StakedIka</a>): ID { <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>.validator_id }
</code></pre>



</details>

<a name="0x0_staked_ika_fungible_staked_ika_validator_id"></a>

## Function `fungible_staked_ika_validator_id`



<pre><code><b>public</b> <b>fun</b> <a href="staked_ika.md#0x0_staked_ika_fungible_staked_ika_validator_id">fungible_staked_ika_validator_id</a>(fungible_staked_ika: &<a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>): <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="staked_ika.md#0x0_staked_ika_fungible_staked_ika_validator_id">fungible_staked_ika_validator_id</a>(fungible_staked_ika: &<a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">FungibleStakedIka</a>): ID {
    fungible_staked_ika.validator_id
}
</code></pre>



</details>

<a name="0x0_staked_ika_staked_ika_amount"></a>

## Function `staked_ika_amount`



<pre><code><b>public</b> <b>fun</b> <a href="staked_ika.md#0x0_staked_ika_staked_ika_amount">staked_ika_amount</a>(<a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: &<a href="staked_ika.md#0x0_staked_ika_StakedIka">staked_ika::StakedIka</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="staked_ika.md#0x0_staked_ika_staked_ika_amount">staked_ika_amount</a>(<a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: &<a href="staked_ika.md#0x0_staked_ika_StakedIka">StakedIka</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> { <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>.principal.value() }
</code></pre>



</details>

<a name="0x0_staked_ika_stake_activation_epoch"></a>

## Function `stake_activation_epoch`



<pre><code><b>public</b> <b>fun</b> <a href="staked_ika.md#0x0_staked_ika_stake_activation_epoch">stake_activation_epoch</a>(<a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: &<a href="staked_ika.md#0x0_staked_ika_StakedIka">staked_ika::StakedIka</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="staked_ika.md#0x0_staked_ika_stake_activation_epoch">stake_activation_epoch</a>(<a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: &<a href="staked_ika.md#0x0_staked_ika_StakedIka">StakedIka</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>.stake_activation_epoch
}
</code></pre>



</details>

<a name="0x0_staked_ika_fungible_staked_ika_value"></a>

## Function `fungible_staked_ika_value`



<pre><code><b>public</b> <b>fun</b> <a href="staked_ika.md#0x0_staked_ika_fungible_staked_ika_value">fungible_staked_ika_value</a>(fungible_staked_ika: &<a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="staked_ika.md#0x0_staked_ika_fungible_staked_ika_value">fungible_staked_ika_value</a>(fungible_staked_ika: &<a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">FungibleStakedIka</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    fungible_staked_ika.value
}
</code></pre>



</details>

<a name="0x0_staked_ika_split_fungible_staked_ika"></a>

## Function `split_fungible_staked_ika`



<pre><code><b>public</b> <b>fun</b> <a href="staked_ika.md#0x0_staked_ika_split_fungible_staked_ika">split_fungible_staked_ika</a>(fungible_staked_ika: &<b>mut</b> <a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>, split_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="staked_ika.md#0x0_staked_ika_split_fungible_staked_ika">split_fungible_staked_ika</a>(
    fungible_staked_ika: &<b>mut</b> <a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">FungibleStakedIka</a>,
    split_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    ctx: &<b>mut</b> TxContext,
): <a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">FungibleStakedIka</a> {
    <b>assert</b>!(split_amount &lt;= fungible_staked_ika.value, <a href="staked_ika.md#0x0_staked_ika_EInsufficientPoolTokenBalance">EInsufficientPoolTokenBalance</a>);

    fungible_staked_ika.value = fungible_staked_ika.value - split_amount;

    <a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">FungibleStakedIka</a> {
        id: <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx),
        validator_id: fungible_staked_ika.validator_id,
        value: split_amount,
    }
}
</code></pre>



</details>

<a name="0x0_staked_ika_join_fungible_staked_ika"></a>

## Function `join_fungible_staked_ika`



<pre><code><b>public</b> <b>fun</b> <a href="staked_ika.md#0x0_staked_ika_join_fungible_staked_ika">join_fungible_staked_ika</a>(self: &<b>mut</b> <a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>, other: <a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="staked_ika.md#0x0_staked_ika_join_fungible_staked_ika">join_fungible_staked_ika</a>(self: &<b>mut</b> <a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">FungibleStakedIka</a>, other: <a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">FungibleStakedIka</a>) {
    <b>let</b> <a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">FungibleStakedIka</a> { id, validator_id, value } = other;
    <b>assert</b>!(self.validator_id == validator_id, <a href="staked_ika.md#0x0_staked_ika_EWrongPool">EWrongPool</a>);

    <a href="../sui-framework/object.md#0x2_object_delete">object::delete</a>(id);

    self.value = self.value + value;
}
</code></pre>



</details>

<a name="0x0_staked_ika_split"></a>

## Function `split`

Split StakedIka <code>self</code> to two parts, one with principal <code>split_amount</code>,
and the remaining principal is left in <code>self</code>.
All the other parameters of the StakedIka like <code>stake_activation_epoch</code> or <code>pool_id</code> remain the same.


<pre><code><b>public</b> <b>fun</b> <a href="staked_ika.md#0x0_staked_ika_split">split</a>(self: &<b>mut</b> <a href="staked_ika.md#0x0_staked_ika_StakedIka">staked_ika::StakedIka</a>, split_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="staked_ika.md#0x0_staked_ika_StakedIka">staked_ika::StakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="staked_ika.md#0x0_staked_ika_split">split</a>(self: &<b>mut</b> <a href="staked_ika.md#0x0_staked_ika_StakedIka">StakedIka</a>, split_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> TxContext): <a href="staked_ika.md#0x0_staked_ika_StakedIka">StakedIka</a> {
    <b>let</b> original_amount = self.principal.value();
    <b>assert</b>!(split_amount &lt;= original_amount, <a href="staked_ika.md#0x0_staked_ika_EInsufficientIkaTokenBalance">EInsufficientIkaTokenBalance</a>);
    <b>let</b> remaining_amount = original_amount - split_amount;
    // Both resulting parts should have at least <a href="staked_ika.md#0x0_staked_ika_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>.
    <b>assert</b>!(remaining_amount &gt;= <a href="staked_ika.md#0x0_staked_ika_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>, <a href="staked_ika.md#0x0_staked_ika_EStakedIkaBelowThreshold">EStakedIkaBelowThreshold</a>);
    <b>assert</b>!(split_amount &gt;= <a href="staked_ika.md#0x0_staked_ika_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>, <a href="staked_ika.md#0x0_staked_ika_EStakedIkaBelowThreshold">EStakedIkaBelowThreshold</a>);
    <a href="staked_ika.md#0x0_staked_ika_StakedIka">StakedIka</a> {
        id: <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx),
        validator_id: self.validator_id,
        stake_activation_epoch: self.stake_activation_epoch,
        principal: self.principal.<a href="staked_ika.md#0x0_staked_ika_split">split</a>(split_amount),
    }
}
</code></pre>



</details>

<a name="0x0_staked_ika_split_staked_ika"></a>

## Function `split_staked_ika`

Split the given StakedIka to the two parts, one with principal <code>split_amount</code>,
transfer the newly split part to the sender address.


<pre><code><b>public</b> entry <b>fun</b> <a href="staked_ika.md#0x0_staked_ika_split_staked_ika">split_staked_ika</a>(stake: &<b>mut</b> <a href="staked_ika.md#0x0_staked_ika_StakedIka">staked_ika::StakedIka</a>, split_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="staked_ika.md#0x0_staked_ika_split_staked_ika">split_staked_ika</a>(stake: &<b>mut</b> <a href="staked_ika.md#0x0_staked_ika_StakedIka">StakedIka</a>, split_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> TxContext) {
    <a href="../sui-framework/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(<a href="staked_ika.md#0x0_staked_ika_split">split</a>(stake, split_amount, ctx), ctx.sender());
}
</code></pre>



</details>

<a name="0x0_staked_ika_join_staked_ika"></a>

## Function `join_staked_ika`

Consume the staked ika <code>other</code> and add its value to <code>self</code>.
Aborts if some of the staking parameters are incompatible (pool id, stake activation epoch, etc.)


<pre><code><b>public</b> entry <b>fun</b> <a href="staked_ika.md#0x0_staked_ika_join_staked_ika">join_staked_ika</a>(self: &<b>mut</b> <a href="staked_ika.md#0x0_staked_ika_StakedIka">staked_ika::StakedIka</a>, other: <a href="staked_ika.md#0x0_staked_ika_StakedIka">staked_ika::StakedIka</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="staked_ika.md#0x0_staked_ika_join_staked_ika">join_staked_ika</a>(self: &<b>mut</b> <a href="staked_ika.md#0x0_staked_ika_StakedIka">StakedIka</a>, other: <a href="staked_ika.md#0x0_staked_ika_StakedIka">StakedIka</a>) {
    <b>assert</b>!(<a href="staked_ika.md#0x0_staked_ika_is_equal_staking_metadata">is_equal_staking_metadata</a>(self, &other), <a href="staked_ika.md#0x0_staked_ika_EIncompatibleStakedIka">EIncompatibleStakedIka</a>);
    <b>let</b> <a href="staked_ika.md#0x0_staked_ika_StakedIka">StakedIka</a> {
        id,
        validator_id: _,
        stake_activation_epoch: _,
        principal,
    } = other;

    id.delete();
    self.principal.join(principal);
}
</code></pre>



</details>

<a name="0x0_staked_ika_is_equal_staking_metadata"></a>

## Function `is_equal_staking_metadata`

Returns true if all the staking parameters of the staked ika except the principal are identical


<pre><code><b>public</b> <b>fun</b> <a href="staked_ika.md#0x0_staked_ika_is_equal_staking_metadata">is_equal_staking_metadata</a>(self: &<a href="staked_ika.md#0x0_staked_ika_StakedIka">staked_ika::StakedIka</a>, other: &<a href="staked_ika.md#0x0_staked_ika_StakedIka">staked_ika::StakedIka</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="staked_ika.md#0x0_staked_ika_is_equal_staking_metadata">is_equal_staking_metadata</a>(self: &<a href="staked_ika.md#0x0_staked_ika_StakedIka">StakedIka</a>, other: &<a href="staked_ika.md#0x0_staked_ika_StakedIka">StakedIka</a>): bool {
    (self.validator_id == other.validator_id) &&
        (self.stake_activation_epoch == other.stake_activation_epoch)
}
</code></pre>



</details>

<a name="0x0_staked_ika_destroy"></a>

## Function `destroy`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="staked_ika.md#0x0_staked_ika_destroy">destroy</a>(fungible_staked_ika: <a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="staked_ika.md#0x0_staked_ika_destroy">destroy</a>(
    fungible_staked_ika: <a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">FungibleStakedIka</a>
) {
    <b>let</b> <a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">FungibleStakedIka</a> { id, .. } = fungible_staked_ika;
    id.delete();
}
</code></pre>



</details>
