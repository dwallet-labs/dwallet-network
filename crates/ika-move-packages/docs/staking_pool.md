---
title: Module `(ika_system=0x0)::staking_pool`
---



-  [Struct `StakingPool`](#(ika_system=0x0)_staking_pool_StakingPool)
-  [Struct `PoolTokenExchangeRate`](#(ika_system=0x0)_staking_pool_PoolTokenExchangeRate)
-  [Constants](#@Constants_0)
-  [Function `new`](#(ika_system=0x0)_staking_pool_new)
-  [Function `request_add_stake`](#(ika_system=0x0)_staking_pool_request_add_stake)
-  [Function `request_withdraw_stake`](#(ika_system=0x0)_staking_pool_request_withdraw_stake)
-  [Function `redeem_fungible_staked_ika`](#(ika_system=0x0)_staking_pool_redeem_fungible_staked_ika)
-  [Function `calculate_fungible_staked_ika_withdraw_amount`](#(ika_system=0x0)_staking_pool_calculate_fungible_staked_ika_withdraw_amount)
-  [Function `convert_to_fungible_staked_ika`](#(ika_system=0x0)_staking_pool_convert_to_fungible_staked_ika)
-  [Function `withdraw_from_principal`](#(ika_system=0x0)_staking_pool_withdraw_from_principal)
-  [Function `deposit_rewards`](#(ika_system=0x0)_staking_pool_deposit_rewards)
-  [Function `process_pending_stakes_and_withdraws`](#(ika_system=0x0)_staking_pool_process_pending_stakes_and_withdraws)
-  [Function `process_pending_stake_withdraw`](#(ika_system=0x0)_staking_pool_process_pending_stake_withdraw)
-  [Function `process_pending_stake`](#(ika_system=0x0)_staking_pool_process_pending_stake)
-  [Function `withdraw_rewards`](#(ika_system=0x0)_staking_pool_withdraw_rewards)
-  [Function `activate_staking_pool`](#(ika_system=0x0)_staking_pool_activate_staking_pool)
-  [Function `deactivate_staking_pool`](#(ika_system=0x0)_staking_pool_deactivate_staking_pool)
-  [Function `ika_balance`](#(ika_system=0x0)_staking_pool_ika_balance)
-  [Function `is_candidate`](#(ika_system=0x0)_staking_pool_is_candidate)
-  [Function `is_inactive`](#(ika_system=0x0)_staking_pool_is_inactive)
-  [Function `pool_token_exchange_rate_at_epoch`](#(ika_system=0x0)_staking_pool_pool_token_exchange_rate_at_epoch)
-  [Function `pending_stake_amount`](#(ika_system=0x0)_staking_pool_pending_stake_amount)
-  [Function `pending_stake_withdraw_amount`](#(ika_system=0x0)_staking_pool_pending_stake_withdraw_amount)
-  [Function `exchange_rates`](#(ika_system=0x0)_staking_pool_exchange_rates)
-  [Function `ika_amount`](#(ika_system=0x0)_staking_pool_ika_amount)
-  [Function `pool_token_amount`](#(ika_system=0x0)_staking_pool_pool_token_amount)
-  [Function `is_candidate_at_epoch`](#(ika_system=0x0)_staking_pool_is_candidate_at_epoch)
-  [Function `get_ika_amount`](#(ika_system=0x0)_staking_pool_get_ika_amount)
-  [Function `get_token_amount`](#(ika_system=0x0)_staking_pool_get_token_amount)
-  [Function `initial_exchange_rate`](#(ika_system=0x0)_staking_pool_initial_exchange_rate)
-  [Function `check_balance_invariants`](#(ika_system=0x0)_staking_pool_check_balance_invariants)


<pre><code><b>use</b> (ika=0x0)::ika;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>;
<b>use</b> <a href="../../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../../std/u64.md#std_u64">std::u64</a>;
<b>use</b> <a href="../../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../../sui/address.md#sui_address">sui::address</a>;
<b>use</b> <a href="../../sui/bag.md#sui_bag">sui::bag</a>;
<b>use</b> <a href="../../sui/balance.md#sui_balance">sui::balance</a>;
<b>use</b> <a href="../../sui/coin.md#sui_coin">sui::coin</a>;
<b>use</b> <a href="../../sui/config.md#sui_config">sui::config</a>;
<b>use</b> <a href="../../sui/deny_list.md#sui_deny_list">sui::deny_list</a>;
<b>use</b> <a href="../../sui/dynamic_field.md#sui_dynamic_field">sui::dynamic_field</a>;
<b>use</b> <a href="../../sui/dynamic_object_field.md#sui_dynamic_object_field">sui::dynamic_object_field</a>;
<b>use</b> <a href="../../sui/event.md#sui_event">sui::event</a>;
<b>use</b> <a href="../../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../../sui/table.md#sui_table">sui::table</a>;
<b>use</b> <a href="../../sui/transfer.md#sui_transfer">sui::transfer</a>;
<b>use</b> <a href="../../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
<b>use</b> <a href="../../sui/types.md#sui_types">sui::types</a>;
<b>use</b> <a href="../../sui/url.md#sui_url">sui::url</a>;
<b>use</b> <a href="../../sui/vec_set.md#sui_vec_set">sui::vec_set</a>;
</code></pre>



<a name="(ika_system=0x0)_staking_pool_StakingPool"></a>

## Struct `StakingPool`

A staking pool embedded in each validator struct in the system state object.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>validator_id: <a href="../../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>activation_epoch: <a href="../../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
 The epoch at which this pool became active.
 The value is <code>None</code> if the pool is pre-active and <code>Some(&lt;epoch_number&gt;)</code> if active or inactive.
</dd>
<dt>
<code>deactivation_epoch: <a href="../../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
 The epoch at which this staking pool ceased to be active. <code>None</code> = {pre-active, active},
 <code>Some(&lt;epoch_number&gt;)</code> if in-active, and it was de-activated at epoch <code>&lt;epoch_number&gt;</code>.
</dd>
<dt>
<code><a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a>: u64</code>
</dt>
<dd>
 The total number of IKA tokens in this pool, including the IKA in the rewards_pool, as well as in all the principal
 in the <code>StakedIka</code> object, updated at epoch boundaries.
</dd>
<dt>
<code>rewards_pool: <a href="../../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;</code>
</dt>
<dd>
 The epoch stake rewards will be added here at the end of each epoch.
</dd>
<dt>
<code>pool_token_balance: u64</code>
</dt>
<dd>
 Total number of pool tokens issued by the pool.
</dd>
<dt>
<code><a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_exchange_rates">exchange_rates</a>: <a href="../../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>&gt;</code>
</dt>
<dd>
 Exchange rate history of previous epochs. Key is the epoch number.
 The entries start from the <code>activation_epoch</code> of this pool and contains exchange rates at the beginning of each epoch,
 i.e., right after the rewards for the previous epoch have been deposited into the pool.
</dd>
<dt>
<code>pending_stake: u64</code>
</dt>
<dd>
 Pending stake amount for this epoch, emptied at epoch boundaries.
</dd>
<dt>
<code>pending_total_ika_withdraw: u64</code>
</dt>
<dd>
 Pending stake withdrawn during the current epoch, emptied at epoch boundaries.
 This includes both the principal and rewards IKA withdrawn.
</dd>
<dt>
<code>pending_pool_token_withdraw: u64</code>
</dt>
<dd>
 Pending pool token withdrawn during the current epoch, emptied at epoch boundaries.
</dd>
<dt>
<code>fungible_total_supply: u64</code>
</dt>
<dd>
 The total supply of fungible staked ika
</dd>
<dt>
<code>fungible_principal: <a href="../../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;</code>
</dt>
<dd>
 The total principal balance of fungible staked ika, without rewards which
 are withdrawn from the reward pool.
</dd>
<dt>
<code>extra_fields: <a href="../../sui/bag.md#sui_bag_Bag">sui::bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_staking_pool_PoolTokenExchangeRate"></a>

## Struct `PoolTokenExchangeRate`

Struct representing the exchange rate of the stake pool token to IKA.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_amount">ika_amount</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_pool_token_amount">pool_token_amount</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="(ika_system=0x0)_staking_pool_EActivationOfInactivePool"></a>



<pre><code><b>const</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EActivationOfInactivePool">EActivationOfInactivePool</a>: u64 = 16;
</code></pre>



<a name="(ika_system=0x0)_staking_pool_ECannotMintFungibleStakedIkaYet"></a>



<pre><code><b>const</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ECannotMintFungibleStakedIkaYet">ECannotMintFungibleStakedIkaYet</a>: u64 = 19;
</code></pre>



<a name="(ika_system=0x0)_staking_pool_EDeactivationOfInactivePool"></a>



<pre><code><b>const</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EDeactivationOfInactivePool">EDeactivationOfInactivePool</a>: u64 = 11;
</code></pre>



<a name="(ika_system=0x0)_staking_pool_EDelegationOfZeroIka"></a>



<pre><code><b>const</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EDelegationOfZeroIka">EDelegationOfZeroIka</a>: u64 = 17;
</code></pre>



<a name="(ika_system=0x0)_staking_pool_EDelegationToInactivePool"></a>



<pre><code><b>const</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EDelegationToInactivePool">EDelegationToInactivePool</a>: u64 = 10;
</code></pre>



<a name="(ika_system=0x0)_staking_pool_EInvariantFailure"></a>



<pre><code><b>const</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EInvariantFailure">EInvariantFailure</a>: u64 = 20;
</code></pre>



<a name="(ika_system=0x0)_staking_pool_EPoolAlreadyActive"></a>



<pre><code><b>const</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EPoolAlreadyActive">EPoolAlreadyActive</a>: u64 = 14;
</code></pre>



<a name="(ika_system=0x0)_staking_pool_ETokenBalancesDoNotMatchExchangeRate"></a>



<pre><code><b>const</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ETokenBalancesDoNotMatchExchangeRate">ETokenBalancesDoNotMatchExchangeRate</a>: u64 = 9;
</code></pre>



<a name="(ika_system=0x0)_staking_pool_EWrongPool"></a>



<pre><code><b>const</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EWrongPool">EWrongPool</a>: u64 = 1;
</code></pre>



<a name="(ika_system=0x0)_staking_pool_new"></a>

## Function `new`

Create a new, empty staking pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_new">new</a>(validator_id: <a href="../../sui/object.md#sui_object_ID">sui::object::ID</a>, ctx: &<b>mut</b> <a href="../../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_new">new</a>(validator_id: ID, ctx: &<b>mut</b> TxContext): <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a> {
    <b>let</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_exchange_rates">exchange_rates</a> = table::new(ctx);
    <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a> {
        validator_id,
        activation_epoch: option::none(),
        deactivation_epoch: option::none(),
        <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a>: 0,
        rewards_pool: balance::zero(),
        pool_token_balance: 0,
        <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_exchange_rates">exchange_rates</a>,
        pending_stake: 0,
        pending_total_ika_withdraw: 0,
        pending_pool_token_withdraw: 0,
        fungible_total_supply: 0,
        fungible_principal: balance::zero(),
        extra_fields: bag::new(ctx),
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_request_add_stake"></a>

## Function `request_add_stake`

Request to stake to a staking pool. The stake starts counting at the beginning of the next epoch,


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_request_add_stake">request_add_stake</a>(pool: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, stake: <a href="../../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;, stake_activation_epoch: u64, validator_id: <a href="../../sui/object.md#sui_object_ID">sui::object::ID</a>, ctx: &<b>mut</b> <a href="../../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_request_add_stake">request_add_stake</a>(
    pool: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>,
    stake: Balance&lt;IKA&gt;,
    stake_activation_epoch: u64,
    validator_id: ID,
    ctx: &<b>mut</b> TxContext,
): StakedIka {
    <b>let</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_amount">ika_amount</a> = stake.value();
    <b>assert</b>!(!<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_is_inactive">is_inactive</a>(pool), <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EDelegationToInactivePool">EDelegationToInactivePool</a>);
    <b>assert</b>!(<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_amount">ika_amount</a> &gt; 0, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EDelegationOfZeroIka">EDelegationOfZeroIka</a>);
    <b>let</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a> = <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_create">staked_ika::create</a>(
        validator_id,
        stake_activation_epoch,
        stake,
        ctx
    );
    pool.pending_stake = pool.pending_stake + <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_amount">ika_amount</a>;
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_request_withdraw_stake"></a>

## Function `request_withdraw_stake`

Request to withdraw the given stake plus rewards from a staking pool.
Both the principal and corresponding rewards in IKA are withdrawn.
A proportional amount of pool token withdraw is recorded and processed at epoch change time.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_request_withdraw_stake">request_withdraw_stake</a>(pool: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, epoch: u64, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>): <a href="../../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_request_withdraw_stake">request_withdraw_stake</a>(
    pool: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>,
    epoch: u64,
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: StakedIka,
): Balance&lt;IKA&gt; {
    // stake is inactive
    <b>if</b> (<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.stake_activation_epoch() &gt; epoch && epoch != 0) {
        <b>let</b> principal = <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.into_balance();
        pool.pending_stake = pool.pending_stake - principal.value();
        <b>return</b> principal
    };
    <b>let</b> (pool_token_withdraw_amount, <b>mut</b> principal_withdraw) = <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_withdraw_from_principal">withdraw_from_principal</a>(
        pool,
        <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>,
    );
    <b>let</b> principal_withdraw_amount = principal_withdraw.value();
    <b>let</b> rewards_withdraw = <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_withdraw_rewards">withdraw_rewards</a>(
        pool,
        principal_withdraw_amount,
        pool_token_withdraw_amount,
        epoch,
    );
    <b>let</b> total_ika_withdraw_amount = principal_withdraw_amount + rewards_withdraw.value();
    pool.pending_total_ika_withdraw = pool.pending_total_ika_withdraw + total_ika_withdraw_amount;
    pool.pending_pool_token_withdraw =
        pool.pending_pool_token_withdraw + pool_token_withdraw_amount;
    // If the pool is inactive, we immediately process the withdrawal.
    <b>if</b> (<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_is_inactive">is_inactive</a>(pool)) <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_process_pending_stake_withdraw">process_pending_stake_withdraw</a>(pool);
    // TODO: implement withdraw bonding period here.
    principal_withdraw.join(rewards_withdraw);
    principal_withdraw
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_redeem_fungible_staked_ika"></a>

## Function `redeem_fungible_staked_ika`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_redeem_fungible_staked_ika">redeem_fungible_staked_ika</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, epoch: u64, fungible_staked_ika: (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>): <a href="../../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_redeem_fungible_staked_ika">redeem_fungible_staked_ika</a>(
    self: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>,
    epoch: u64,
    fungible_staked_ika: FungibleStakedIka,
): Balance&lt;IKA&gt; {
    <b>let</b> validator_id = fungible_staked_ika.validator_id();
    <b>let</b> value = fungible_staked_ika.value();
    <b>assert</b>!(validator_id == self.validator_id, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EWrongPool">EWrongPool</a>);
    fungible_staked_ika.destroy();
    <b>let</b> latest_exchange_rate = self.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(epoch);
    <b>let</b> (principal_amount, rewards_amount) = <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_calculate_fungible_staked_ika_withdraw_amount">calculate_fungible_staked_ika_withdraw_amount</a>(
        latest_exchange_rate,
        value,
        balance::value(&self.fungible_principal),
        self.fungible_total_supply,
    );
    self.fungible_total_supply = self.fungible_total_supply - value;
    <b>let</b> <b>mut</b> ika_out = balance::split(&<b>mut</b> self.fungible_principal, principal_amount);
    balance::join(
        &<b>mut</b> ika_out,
        balance::split(&<b>mut</b> self.rewards_pool, rewards_amount),
    );
    self.pending_total_ika_withdraw = self.pending_total_ika_withdraw + balance::value(&ika_out);
    self.pending_pool_token_withdraw = self.pending_pool_token_withdraw + value;
    ika_out
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_calculate_fungible_staked_ika_withdraw_amount"></a>

## Function `calculate_fungible_staked_ika_withdraw_amount`

written in separate function so i can test with random values
returns (principal_withdraw_amount, rewards_withdraw_amount)


<pre><code><b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_calculate_fungible_staked_ika_withdraw_amount">calculate_fungible_staked_ika_withdraw_amount</a>(latest_exchange_rate: (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>, fungible_staked_ika_value: u64, fungible_staked_ika_principal_amount: u64, fungible_staked_ika_total_supply: u64): (u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_calculate_fungible_staked_ika_withdraw_amount">calculate_fungible_staked_ika_withdraw_amount</a>(
    latest_exchange_rate: <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>,
    fungible_staked_ika_value: u64,
    fungible_staked_ika_principal_amount: u64, // fungible_staked_ika_data.principal.value()
    fungible_staked_ika_total_supply: u64, // fungible_staked_ika_data.total_supply
): (u64, u64) {
    // 1. <b>if</b> the entire fungible staked total supply supply is redeemed, how much ika should we receive?
    <b>let</b> total_ika_amount = <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_get_ika_amount">get_ika_amount</a>(
        &latest_exchange_rate,
        fungible_staked_ika_total_supply,
    );
    // min with total_ika_amount to prevent underflow
    <b>let</b> fungible_staked_ika_principal_amount = <a href="../../std/u64.md#std_u64_min">std::u64::min</a>(
        fungible_staked_ika_principal_amount,
        total_ika_amount,
    );
    // 2. how much do we need to withdraw from the rewards pool?
    <b>let</b> total_rewards = total_ika_amount - fungible_staked_ika_principal_amount;
    // 3. proportionally withdraw from both wrt the fungible_staked_ika_value.
    <b>let</b> principal_withdraw_amount =
        (
            (fungible_staked_ika_value <b>as</b> u128)
            * (fungible_staked_ika_principal_amount <b>as</b> u128)
            / (fungible_staked_ika_total_supply <b>as</b> u128),
        ) <b>as</b> u64;
    <b>let</b> rewards_withdraw_amount =
        (
            (fungible_staked_ika_value <b>as</b> u128)
            * (total_rewards <b>as</b> u128)
            / (fungible_staked_ika_total_supply <b>as</b> u128),
        ) <b>as</b> u64;
    // <b>invariant</b> check, just in case
    <b>let</b> expected_ika_amount = <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_get_ika_amount">get_ika_amount</a>(&latest_exchange_rate, fungible_staked_ika_value);
    <b>assert</b>!(
        principal_withdraw_amount + rewards_withdraw_amount &lt;= expected_ika_amount,
        <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EInvariantFailure">EInvariantFailure</a>,
    );
    (principal_withdraw_amount, rewards_withdraw_amount)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_convert_to_fungible_staked_ika"></a>

## Function `convert_to_fungible_staked_ika`

Convert the given staked IKA to an FungibleStakedIka object


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_convert_to_fungible_staked_ika">convert_to_fungible_staked_ika</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, epoch: u64, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>, ctx: &<b>mut</b> <a href="../../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_convert_to_fungible_staked_ika">convert_to_fungible_staked_ika</a>(
    self: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>,
    epoch: u64,
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: StakedIka,
    ctx: &<b>mut</b> TxContext,
): FungibleStakedIka {
    <b>let</b> validator_id = <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.validator_id();
    <b>let</b> stake_activation_epoch = <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.stake_activation_epoch();
    <b>let</b> principal = <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.into_balance();
    <b>assert</b>!(validator_id == self.validator_id, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EWrongPool">EWrongPool</a>);
    <b>assert</b>!(epoch &gt;= stake_activation_epoch, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ECannotMintFungibleStakedIkaYet">ECannotMintFungibleStakedIkaYet</a>);
    <b>let</b> exchange_rate_at_staking_epoch = self.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(
        stake_activation_epoch,
    );
    <b>let</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_pool_token_amount">pool_token_amount</a> = <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_get_token_amount">get_token_amount</a>(
        &exchange_rate_at_staking_epoch,
        balance::value(&principal),
    );
    self.fungible_total_supply = self.fungible_total_supply + <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_pool_token_amount">pool_token_amount</a>;
    balance::join(&<b>mut</b> self.fungible_principal, principal);
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_create_fungible">staked_ika::create_fungible</a>(validator_id, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_pool_token_amount">pool_token_amount</a>, ctx)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_withdraw_from_principal"></a>

## Function `withdraw_from_principal`

Withdraw the principal IKA stored in the StakedIka object, and calculate the corresponding amount of pool
tokens using exchange rate at staking epoch.
Returns values are amount of pool tokens withdrawn and withdrawn principal portion of IKA.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_withdraw_from_principal">withdraw_from_principal</a>(pool: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>): (u64, <a href="../../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_withdraw_from_principal">withdraw_from_principal</a>(
    pool: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>,
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: StakedIka,
): (u64, Balance&lt;IKA&gt;) {
    // Check that the stake information matches the pool.
    <b>assert</b>!(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.validator_id() == pool.validator_id, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EWrongPool">EWrongPool</a>);
    <b>let</b> exchange_rate_at_staking_epoch = <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(
        pool,
        <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.stake_activation_epoch(),
    );
    <b>let</b> principal_withdraw = <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.into_balance();
    <b>let</b> pool_token_withdraw_amount = <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_get_token_amount">get_token_amount</a>(
        &exchange_rate_at_staking_epoch,
        principal_withdraw.value(),
    );
    (pool_token_withdraw_amount, principal_withdraw)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_deposit_rewards"></a>

## Function `deposit_rewards`

Called at epoch advancement times to add rewards (in IKA) to the staking pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_deposit_rewards">deposit_rewards</a>(pool: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, rewards: <a href="../../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_deposit_rewards">deposit_rewards</a>(pool: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>, rewards: Balance&lt;IKA&gt;) {
    pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a> = pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a> + rewards.value();
    pool.rewards_pool.join(rewards);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_process_pending_stakes_and_withdraws"></a>

## Function `process_pending_stakes_and_withdraws`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>(pool: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, new_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>(pool: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>, new_epoch: u64) {
    <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_process_pending_stake_withdraw">process_pending_stake_withdraw</a>(pool);
    <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_process_pending_stake">process_pending_stake</a>(pool);
    pool
        .<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_exchange_rates">exchange_rates</a>
        .add(
            new_epoch,
            <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> {
                <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_amount">ika_amount</a>: pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a>,
                <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_pool_token_amount">pool_token_amount</a>: pool.pool_token_balance,
            },
        );
    <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_check_balance_invariants">check_balance_invariants</a>(pool, new_epoch);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_process_pending_stake_withdraw"></a>

## Function `process_pending_stake_withdraw`

Called at epoch boundaries to process pending stake withdraws requested during the epoch.
Also called immediately upon withdrawal if the pool is inactive.


<pre><code><b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_process_pending_stake_withdraw">process_pending_stake_withdraw</a>(pool: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_process_pending_stake_withdraw">process_pending_stake_withdraw</a>(pool: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>) {
    pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a> = pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a> - pool.pending_total_ika_withdraw;
    pool.pool_token_balance = pool.pool_token_balance - pool.pending_pool_token_withdraw;
    pool.pending_total_ika_withdraw = 0;
    pool.pending_pool_token_withdraw = 0;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_process_pending_stake"></a>

## Function `process_pending_stake`

Called at epoch boundaries to process the pending stake.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_process_pending_stake">process_pending_stake</a>(pool: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_process_pending_stake">process_pending_stake</a>(pool: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>) {
    // Use the most up to date exchange rate with the rewards deposited and withdraws effectuated.
    <b>let</b> latest_exchange_rate = <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> {
        <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_amount">ika_amount</a>: pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a>,
        <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_pool_token_amount">pool_token_amount</a>: pool.pool_token_balance,
    };
    pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a> = pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a> + pool.pending_stake;
    pool.pool_token_balance = <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_get_token_amount">get_token_amount</a>(&latest_exchange_rate, pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a>);
    pool.pending_stake = 0;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_withdraw_rewards"></a>

## Function `withdraw_rewards`

This function does the following:
1. Calculates the total amount of IKA (including principal and rewards) that the provided pool tokens represent
at the current exchange rate.
2. Using the above number and the given <code>principal_withdraw_amount</code>, calculates the rewards portion of the
stake we should withdraw.
3. Withdraws the rewards portion from the rewards pool at the current exchange rate. We only withdraw the rewards
portion because the principal portion was already taken out of the staker's self custodied StakedIka.


<pre><code><b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_withdraw_rewards">withdraw_rewards</a>(pool: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, principal_withdraw_amount: u64, pool_token_withdraw_amount: u64, epoch: u64): <a href="../../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_withdraw_rewards">withdraw_rewards</a>(
    pool: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>,
    principal_withdraw_amount: u64,
    pool_token_withdraw_amount: u64,
    epoch: u64,
): Balance&lt;IKA&gt; {
    <b>let</b> exchange_rate = <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(pool, epoch);
    <b>let</b> total_ika_withdraw_amount = <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_get_ika_amount">get_ika_amount</a>(&exchange_rate, pool_token_withdraw_amount);
    <b>let</b> <b>mut</b> reward_withdraw_amount = <b>if</b> (total_ika_withdraw_amount &gt;= principal_withdraw_amount) {
        total_ika_withdraw_amount - principal_withdraw_amount
    } <b>else</b> 0;
    // This may happen when we are withdrawing everything from the pool and
    // the rewards pool balance may be less than reward_withdraw_amount.
    // TODO: FIGURE OUT EXACTLY WHY THIS CAN HAPPEN.
    reward_withdraw_amount = reward_withdraw_amount.min(pool.rewards_pool.value());
    pool.rewards_pool.split(reward_withdraw_amount)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_activate_staking_pool"></a>

## Function `activate_staking_pool`

Called by <code><a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a></code> module to activate a staking pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activate_staking_pool">activate_staking_pool</a>(pool: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, activation_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activate_staking_pool">activate_staking_pool</a>(pool: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>, activation_epoch: u64) {
    // Add the initial exchange rate to the table.
    pool
        .<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_exchange_rates">exchange_rates</a>
        .add(
            activation_epoch,
            <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_initial_exchange_rate">initial_exchange_rate</a>(),
        );
    // Check that the pool is preactive and not inactive.
    <b>assert</b>!(<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_is_candidate">is_candidate</a>(pool), <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EPoolAlreadyActive">EPoolAlreadyActive</a>);
    <b>assert</b>!(!<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_is_inactive">is_inactive</a>(pool), <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EActivationOfInactivePool">EActivationOfInactivePool</a>);
    // Fill in the active epoch.
    pool.activation_epoch.fill(activation_epoch);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_deactivate_staking_pool"></a>

## Function `deactivate_staking_pool`

Deactivate a staking pool by setting the <code>deactivation_epoch</code>. After
this pool deactivation, the pool stops earning rewards. Only stake
withdraws can be made to the pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_deactivate_staking_pool">deactivate_staking_pool</a>(pool: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, deactivation_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_deactivate_staking_pool">deactivate_staking_pool</a>(pool: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>, deactivation_epoch: u64) {
    // We can't deactivate an already deactivated pool.
    <b>assert</b>!(!<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_is_inactive">is_inactive</a>(pool), <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EDeactivationOfInactivePool">EDeactivationOfInactivePool</a>);
    pool.deactivation_epoch = option::some(deactivation_epoch);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_ika_balance"></a>

## Function `ika_balance`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a>(pool: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a>(pool: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>): u64 { pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a> }
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_is_candidate"></a>

## Function `is_candidate`

Returns true if the input staking pool is candidate.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_is_candidate">is_candidate</a>(pool: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_is_candidate">is_candidate</a>(pool: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>): bool {
    pool.activation_epoch.is_none()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_is_inactive"></a>

## Function `is_inactive`

Returns true if the input staking pool is inactive.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_is_inactive">is_inactive</a>(pool: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_is_inactive">is_inactive</a>(pool: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>): bool {
    pool.deactivation_epoch.is_some()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_pool_token_exchange_rate_at_epoch"></a>

## Function `pool_token_exchange_rate_at_epoch`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(pool: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, epoch: u64): (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(
    pool: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>,
    epoch: u64,
): <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> {
    // If the pool is preactive then the exchange rate is always 1:1.
    <b>if</b> (<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_is_candidate_at_epoch">is_candidate_at_epoch</a>(pool, epoch)) {
        <b>return</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_initial_exchange_rate">initial_exchange_rate</a>()
    };
    <b>let</b> clamped_epoch = pool.deactivation_epoch.get_with_default(epoch);
    <b>let</b> <b>mut</b> epoch = clamped_epoch.min(epoch);
    <b>let</b> activation_epoch = *pool.activation_epoch.borrow();
    // Find the latest epoch that's earlier than the given epoch with an <b>entry</b> in the table
    <b>while</b> (epoch &gt;= activation_epoch) {
        <b>if</b> (pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_exchange_rates">exchange_rates</a>.contains(epoch)) {
            <b>return</b> pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_exchange_rates">exchange_rates</a>[epoch]
        };
        epoch = epoch - 1;
    };
    // This line really should be unreachable. Do we want an <b>assert</b> <b>false</b> here?
    <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_initial_exchange_rate">initial_exchange_rate</a>()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_pending_stake_amount"></a>

## Function `pending_stake_amount`

Returns the total value of the pending staking requests for this staking pool.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_pending_stake_amount">pending_stake_amount</a>(<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a>: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_pending_stake_amount">pending_stake_amount</a>(<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a>: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>): u64 {
    <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a>.pending_stake
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_pending_stake_withdraw_amount"></a>

## Function `pending_stake_withdraw_amount`

Returns the total withdrawal from the staking pool this epoch.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_pending_stake_withdraw_amount">pending_stake_withdraw_amount</a>(<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a>: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_pending_stake_withdraw_amount">pending_stake_withdraw_amount</a>(<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a>: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>): u64 {
    <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a>.pending_total_ika_withdraw
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_exchange_rates"></a>

## Function `exchange_rates`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_exchange_rates">exchange_rates</a>(pool: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>): &<a href="../../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_exchange_rates">exchange_rates</a>(pool: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>): &Table&lt;u64, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>&gt; {
    &pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_exchange_rates">exchange_rates</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_ika_amount"></a>

## Function `ika_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_amount">ika_amount</a>(exchange_rate: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_amount">ika_amount</a>(exchange_rate: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>): u64 {
    exchange_rate.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_amount">ika_amount</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_pool_token_amount"></a>

## Function `pool_token_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_pool_token_amount">pool_token_amount</a>(exchange_rate: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_pool_token_amount">pool_token_amount</a>(exchange_rate: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>): u64 {
    exchange_rate.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_pool_token_amount">pool_token_amount</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_is_candidate_at_epoch"></a>

## Function `is_candidate_at_epoch`

Returns true if the provided staking pool is preactive at the provided epoch.


<pre><code><b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_is_candidate_at_epoch">is_candidate_at_epoch</a>(pool: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, epoch: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_is_candidate_at_epoch">is_candidate_at_epoch</a>(pool: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>, epoch: u64): bool {
    // Either the pool is currently preactive or the pool's starting epoch is later than the provided epoch.
    <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_is_candidate">is_candidate</a>(pool) || (*pool.activation_epoch.borrow() &gt; epoch)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_get_ika_amount"></a>

## Function `get_ika_amount`



<pre><code><b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_get_ika_amount">get_ika_amount</a>(exchange_rate: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>, token_amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_get_ika_amount">get_ika_amount</a>(exchange_rate: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>, token_amount: u64): u64 {
    // When either amount is 0, that means we have no stakes with this pool.
    // The other amount might be non-zero when there's dust left in the pool.
    <b>if</b> (exchange_rate.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_amount">ika_amount</a> == 0 || exchange_rate.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_pool_token_amount">pool_token_amount</a> == 0) {
        <b>return</b> token_amount
    };
    <b>let</b> res =
        exchange_rate.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_amount">ika_amount</a> <b>as</b> u128
                * (token_amount <b>as</b> u128)
                / (exchange_rate.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_pool_token_amount">pool_token_amount</a> <b>as</b> u128);
    res <b>as</b> u64
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_get_token_amount"></a>

## Function `get_token_amount`



<pre><code><b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_get_token_amount">get_token_amount</a>(exchange_rate: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_amount">ika_amount</a>: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_get_token_amount">get_token_amount</a>(exchange_rate: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_amount">ika_amount</a>: u64): u64 {
    // When either amount is 0, that means we have no stakes with this pool.
    // The other amount might be non-zero when there's dust left in the pool.
    <b>if</b> (exchange_rate.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_amount">ika_amount</a> == 0 || exchange_rate.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_pool_token_amount">pool_token_amount</a> == 0) {
        <b>return</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_amount">ika_amount</a>
    };
    <b>let</b> res =
        exchange_rate.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_pool_token_amount">pool_token_amount</a> <b>as</b> u128
                * (<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_amount">ika_amount</a> <b>as</b> u128)
                / (exchange_rate.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_amount">ika_amount</a> <b>as</b> u128);
    res <b>as</b> u64
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_initial_exchange_rate"></a>

## Function `initial_exchange_rate`



<pre><code><b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_initial_exchange_rate">initial_exchange_rate</a>(): (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_initial_exchange_rate">initial_exchange_rate</a>(): <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> {
    <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> { <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_amount">ika_amount</a>: 0, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_pool_token_amount">pool_token_amount</a>: 0 }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_check_balance_invariants"></a>

## Function `check_balance_invariants`



<pre><code><b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_check_balance_invariants">check_balance_invariants</a>(pool: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_check_balance_invariants">check_balance_invariants</a>(pool: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>, epoch: u64) {
    <b>let</b> exchange_rate = <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(pool, epoch);
    // check that the pool token balance and ika balance ratio matches the exchange rate stored.
    <b>let</b> expected = <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_get_token_amount">get_token_amount</a>(&exchange_rate, pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a>);
    <b>let</b> actual = pool.pool_token_balance;
    <b>assert</b>!(expected == actual, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ETokenBalancesDoNotMatchExchangeRate">ETokenBalancesDoNotMatchExchangeRate</a>)
}
</code></pre>



</details>
