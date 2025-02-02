---
title: Module `0x0::staking_pool`
---



-  [Struct `StakingPool`](#0x0_staking_pool_StakingPool)
-  [Struct `PoolTokenExchangeRate`](#0x0_staking_pool_PoolTokenExchangeRate)
-  [Constants](#@Constants_0)
-  [Function `new`](#0x0_staking_pool_new)
-  [Function `request_add_stake`](#0x0_staking_pool_request_add_stake)
-  [Function `request_withdraw_stake`](#0x0_staking_pool_request_withdraw_stake)
-  [Function `redeem_fungible_staked_ika`](#0x0_staking_pool_redeem_fungible_staked_ika)
-  [Function `calculate_fungible_staked_ika_withdraw_amount`](#0x0_staking_pool_calculate_fungible_staked_ika_withdraw_amount)
-  [Function `convert_to_fungible_staked_ika`](#0x0_staking_pool_convert_to_fungible_staked_ika)
-  [Function `withdraw_from_principal`](#0x0_staking_pool_withdraw_from_principal)
-  [Function `deposit_rewards`](#0x0_staking_pool_deposit_rewards)
-  [Function `process_pending_stakes_and_withdraws`](#0x0_staking_pool_process_pending_stakes_and_withdraws)
-  [Function `process_pending_stake_withdraw`](#0x0_staking_pool_process_pending_stake_withdraw)
-  [Function `process_pending_stake`](#0x0_staking_pool_process_pending_stake)
-  [Function `withdraw_rewards`](#0x0_staking_pool_withdraw_rewards)
-  [Function `activate_staking_pool`](#0x0_staking_pool_activate_staking_pool)
-  [Function `deactivate_staking_pool`](#0x0_staking_pool_deactivate_staking_pool)
-  [Function `ika_balance`](#0x0_staking_pool_ika_balance)
-  [Function `is_candidate`](#0x0_staking_pool_is_candidate)
-  [Function `is_inactive`](#0x0_staking_pool_is_inactive)
-  [Function `pool_token_exchange_rate_at_epoch`](#0x0_staking_pool_pool_token_exchange_rate_at_epoch)
-  [Function `pending_stake_amount`](#0x0_staking_pool_pending_stake_amount)
-  [Function `pending_stake_withdraw_amount`](#0x0_staking_pool_pending_stake_withdraw_amount)
-  [Function `exchange_rates`](#0x0_staking_pool_exchange_rates)
-  [Function `ika_amount`](#0x0_staking_pool_ika_amount)
-  [Function `pool_token_amount`](#0x0_staking_pool_pool_token_amount)
-  [Function `is_candidate_at_epoch`](#0x0_staking_pool_is_candidate_at_epoch)
-  [Function `get_ika_amount`](#0x0_staking_pool_get_ika_amount)
-  [Function `get_token_amount`](#0x0_staking_pool_get_token_amount)
-  [Function `initial_exchange_rate`](#0x0_staking_pool_initial_exchange_rate)
-  [Function `check_balance_invariants`](#0x0_staking_pool_check_balance_invariants)


<pre><code><b>use</b> <a href="../ika/ika.md#0x0_ika">0x0::ika</a>;
<b>use</b> <a href="staked_ika.md#0x0_staked_ika">0x0::staked_ika</a>;
<b>use</b> <a href="../move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../move-stdlib/u64.md#0x1_u64">0x1::u64</a>;
<b>use</b> <a href="../sui-framework/bag.md#0x2_bag">0x2::bag</a>;
<b>use</b> <a href="../sui-framework/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/table.md#0x2_table">0x2::table</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
</code></pre>



<a name="0x0_staking_pool_StakingPool"></a>

## Struct `StakingPool`

A staking pool embedded in each validator struct in the system state object.


<pre><code><b>struct</b> <a href="staking_pool.md#0x0_staking_pool_StakingPool">StakingPool</a> <b>has</b> store
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
<code>activation_epoch: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;</code>
</dt>
<dd>
 The epoch at which this pool became active.
 The value is <code>None</code> if the pool is pre-active and <code>Some(&lt;epoch_number&gt;)</code> if active or inactive.
</dd>
<dt>
<code>deactivation_epoch: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;</code>
</dt>
<dd>
 The epoch at which this staking pool ceased to be active. <code>None</code> = {pre-active, active},
 <code>Some(&lt;epoch_number&gt;)</code> if in-active, and it was de-activated at epoch <code>&lt;epoch_number&gt;</code>.
</dd>
<dt>
<code>ika_balance: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 The total number of IKA tokens in this pool, including the IKA in the rewards_pool, as well as in all the principal
 in the <code>StakedIka</code> object, updated at epoch boundaries.
</dd>
<dt>
<code>rewards_pool: <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;</code>
</dt>
<dd>
 The epoch stake rewards will be added here at the end of each epoch.
</dd>
<dt>
<code>pool_token_balance: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 Total number of pool tokens issued by the pool.
</dd>
<dt>
<code>exchange_rates: <a href="../sui-framework/table.md#0x2_table_Table">table::Table</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="staking_pool.md#0x0_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>&gt;</code>
</dt>
<dd>
 Exchange rate history of previous epochs. Key is the epoch number.
 The entries start from the <code>activation_epoch</code> of this pool and contains exchange rates at the beginning of each epoch,
 i.e., right after the rewards for the previous epoch have been deposited into the pool.
</dd>
<dt>
<code>pending_stake: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 Pending stake amount for this epoch, emptied at epoch boundaries.
</dd>
<dt>
<code>pending_total_ika_withdraw: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 Pending stake withdrawn during the current epoch, emptied at epoch boundaries.
 This includes both the principal and rewards IKA withdrawn.
</dd>
<dt>
<code>pending_pool_token_withdraw: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 Pending pool token withdrawn during the current epoch, emptied at epoch boundaries.
</dd>
<dt>
<code>fungible_total_supply: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 The total supply of fungible staked ika
</dd>
<dt>
<code>fungible_principal: <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;</code>
</dt>
<dd>
 The total principal balance of fungible staked ika, without rewards which
 are withdrawn from the reward pool.
</dd>
<dt>
<code>extra_fields: <a href="../sui-framework/bag.md#0x2_bag_Bag">bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="0x0_staking_pool_PoolTokenExchangeRate"></a>

## Struct `PoolTokenExchangeRate`

Struct representing the exchange rate of the stake pool token to IKA.


<pre><code><b>struct</b> <a href="staking_pool.md#0x0_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>ika_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>pool_token_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x0_staking_pool_EWrongPool"></a>



<pre><code><b>const</b> <a href="staking_pool.md#0x0_staking_pool_EWrongPool">EWrongPool</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1;
</code></pre>



<a name="0x0_staking_pool_EActivationOfInactivePool"></a>



<pre><code><b>const</b> <a href="staking_pool.md#0x0_staking_pool_EActivationOfInactivePool">EActivationOfInactivePool</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 16;
</code></pre>



<a name="0x0_staking_pool_ECannotMintFungibleStakedIkaYet"></a>



<pre><code><b>const</b> <a href="staking_pool.md#0x0_staking_pool_ECannotMintFungibleStakedIkaYet">ECannotMintFungibleStakedIkaYet</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 19;
</code></pre>



<a name="0x0_staking_pool_EDeactivationOfInactivePool"></a>



<pre><code><b>const</b> <a href="staking_pool.md#0x0_staking_pool_EDeactivationOfInactivePool">EDeactivationOfInactivePool</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 11;
</code></pre>



<a name="0x0_staking_pool_EDelegationOfZeroIka"></a>



<pre><code><b>const</b> <a href="staking_pool.md#0x0_staking_pool_EDelegationOfZeroIka">EDelegationOfZeroIka</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 17;
</code></pre>



<a name="0x0_staking_pool_EDelegationToInactivePool"></a>



<pre><code><b>const</b> <a href="staking_pool.md#0x0_staking_pool_EDelegationToInactivePool">EDelegationToInactivePool</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 10;
</code></pre>



<a name="0x0_staking_pool_EInvariantFailure"></a>



<pre><code><b>const</b> <a href="staking_pool.md#0x0_staking_pool_EInvariantFailure">EInvariantFailure</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 20;
</code></pre>



<a name="0x0_staking_pool_EPoolAlreadyActive"></a>



<pre><code><b>const</b> <a href="staking_pool.md#0x0_staking_pool_EPoolAlreadyActive">EPoolAlreadyActive</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 14;
</code></pre>



<a name="0x0_staking_pool_ETokenBalancesDoNotMatchExchangeRate"></a>



<pre><code><b>const</b> <a href="staking_pool.md#0x0_staking_pool_ETokenBalancesDoNotMatchExchangeRate">ETokenBalancesDoNotMatchExchangeRate</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 9;
</code></pre>



<a name="0x0_staking_pool_new"></a>

## Function `new`

Create a new, empty staking pool.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_new">new</a>(validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="staking_pool.md#0x0_staking_pool_StakingPool">staking_pool::StakingPool</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_new">new</a>(validator_id: ID, ctx: &<b>mut</b> TxContext): <a href="staking_pool.md#0x0_staking_pool_StakingPool">StakingPool</a> {
    <b>let</b> exchange_rates = <a href="../sui-framework/table.md#0x2_table_new">table::new</a>(ctx);
    <a href="staking_pool.md#0x0_staking_pool_StakingPool">StakingPool</a> {
        validator_id,
        activation_epoch: <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(),
        deactivation_epoch: <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(),
        ika_balance: 0,
        rewards_pool: <a href="../sui-framework/balance.md#0x2_balance_zero">balance::zero</a>(),
        pool_token_balance: 0,
        exchange_rates,
        pending_stake: 0,
        pending_total_ika_withdraw: 0,
        pending_pool_token_withdraw: 0,
        fungible_total_supply: 0,
        fungible_principal: <a href="../sui-framework/balance.md#0x2_balance_zero">balance::zero</a>(),
        extra_fields: <a href="../sui-framework/bag.md#0x2_bag_new">bag::new</a>(ctx),
    }
}
</code></pre>



</details>

<a name="0x0_staking_pool_request_add_stake"></a>

## Function `request_add_stake`

Request to stake to a staking pool. The stake starts counting at the beginning of the next epoch,


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_request_add_stake">request_add_stake</a>(pool: &<b>mut</b> <a href="staking_pool.md#0x0_staking_pool_StakingPool">staking_pool::StakingPool</a>, stake: <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;, stake_activation_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="staked_ika.md#0x0_staked_ika_StakedIka">staked_ika::StakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_request_add_stake">request_add_stake</a>(
    pool: &<b>mut</b> <a href="staking_pool.md#0x0_staking_pool_StakingPool">StakingPool</a>,
    stake: Balance&lt;IKA&gt;,
    stake_activation_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    validator_id: ID,
    ctx: &<b>mut</b> TxContext,
): StakedIka {
    <b>let</b> ika_amount = stake.value();
    <b>assert</b>!(!<a href="staking_pool.md#0x0_staking_pool_is_inactive">is_inactive</a>(pool), <a href="staking_pool.md#0x0_staking_pool_EDelegationToInactivePool">EDelegationToInactivePool</a>);
    <b>assert</b>!(ika_amount &gt; 0, <a href="staking_pool.md#0x0_staking_pool_EDelegationOfZeroIka">EDelegationOfZeroIka</a>);
    <b>let</b> <a href="staked_ika.md#0x0_staked_ika">staked_ika</a> = <a href="staked_ika.md#0x0_staked_ika_create">staked_ika::create</a>(
        validator_id,
        stake_activation_epoch,
        stake,
        ctx
    );
    pool.pending_stake = pool.pending_stake + ika_amount;
    <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>
}
</code></pre>



</details>

<a name="0x0_staking_pool_request_withdraw_stake"></a>

## Function `request_withdraw_stake`

Request to withdraw the given stake plus rewards from a staking pool.
Both the principal and corresponding rewards in IKA are withdrawn.
A proportional amount of pool token withdraw is recorded and processed at epoch change time.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_request_withdraw_stake">request_withdraw_stake</a>(pool: &<b>mut</b> <a href="staking_pool.md#0x0_staking_pool_StakingPool">staking_pool::StakingPool</a>, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: <a href="staked_ika.md#0x0_staked_ika_StakedIka">staked_ika::StakedIka</a>): <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_request_withdraw_stake">request_withdraw_stake</a>(
    pool: &<b>mut</b> <a href="staking_pool.md#0x0_staking_pool_StakingPool">StakingPool</a>,
    epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: StakedIka,
): Balance&lt;IKA&gt; {
    // stake is inactive
    <b>if</b> (<a href="staked_ika.md#0x0_staked_ika">staked_ika</a>.stake_activation_epoch() &gt; epoch && epoch != 0) {
        <b>let</b> principal = <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>.into_balance();
        pool.pending_stake = pool.pending_stake - principal.value();

        <b>return</b> principal
    };

    <b>let</b> (pool_token_withdraw_amount, <b>mut</b> principal_withdraw) = <a href="staking_pool.md#0x0_staking_pool_withdraw_from_principal">withdraw_from_principal</a>(
        pool,
        <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>,
    );
    <b>let</b> principal_withdraw_amount = principal_withdraw.value();

    <b>let</b> rewards_withdraw = <a href="staking_pool.md#0x0_staking_pool_withdraw_rewards">withdraw_rewards</a>(
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
    <b>if</b> (<a href="staking_pool.md#0x0_staking_pool_is_inactive">is_inactive</a>(pool)) <a href="staking_pool.md#0x0_staking_pool_process_pending_stake_withdraw">process_pending_stake_withdraw</a>(pool);

    // TODO: implement withdraw bonding period here.
    principal_withdraw.join(rewards_withdraw);
    principal_withdraw
}
</code></pre>



</details>

<a name="0x0_staking_pool_redeem_fungible_staked_ika"></a>

## Function `redeem_fungible_staked_ika`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_redeem_fungible_staked_ika">redeem_fungible_staked_ika</a>(self: &<b>mut</b> <a href="staking_pool.md#0x0_staking_pool_StakingPool">staking_pool::StakingPool</a>, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, fungible_staked_ika: <a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>): <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_redeem_fungible_staked_ika">redeem_fungible_staked_ika</a>(
    self: &<b>mut</b> <a href="staking_pool.md#0x0_staking_pool_StakingPool">StakingPool</a>,
    epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    fungible_staked_ika: FungibleStakedIka,
): Balance&lt;IKA&gt; {
    <b>let</b> validator_id = fungible_staked_ika.validator_id();
    <b>let</b> value = fungible_staked_ika.value();

    <b>assert</b>!(validator_id == self.validator_id, <a href="staking_pool.md#0x0_staking_pool_EWrongPool">EWrongPool</a>);

    fungible_staked_ika.destroy();

    <b>let</b> latest_exchange_rate = self.<a href="staking_pool.md#0x0_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(epoch);

    <b>let</b> (principal_amount, rewards_amount) = <a href="staking_pool.md#0x0_staking_pool_calculate_fungible_staked_ika_withdraw_amount">calculate_fungible_staked_ika_withdraw_amount</a>(
        latest_exchange_rate,
        value,
        <a href="../sui-framework/balance.md#0x2_balance_value">balance::value</a>(&self.fungible_principal),
        self.fungible_total_supply,
    );

    self.fungible_total_supply = self.fungible_total_supply - value;

    <b>let</b> <b>mut</b> ika_out = <a href="../sui-framework/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> self.fungible_principal, principal_amount);
    <a href="../sui-framework/balance.md#0x2_balance_join">balance::join</a>(
        &<b>mut</b> ika_out,
        <a href="../sui-framework/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> self.rewards_pool, rewards_amount),
    );

    self.pending_total_ika_withdraw = self.pending_total_ika_withdraw + <a href="../sui-framework/balance.md#0x2_balance_value">balance::value</a>(&ika_out);
    self.pending_pool_token_withdraw = self.pending_pool_token_withdraw + value;

    ika_out
}
</code></pre>



</details>

<a name="0x0_staking_pool_calculate_fungible_staked_ika_withdraw_amount"></a>

## Function `calculate_fungible_staked_ika_withdraw_amount`

written in separate function so i can test with random values
returns (principal_withdraw_amount, rewards_withdraw_amount)


<pre><code><b>fun</b> <a href="staking_pool.md#0x0_staking_pool_calculate_fungible_staked_ika_withdraw_amount">calculate_fungible_staked_ika_withdraw_amount</a>(latest_exchange_rate: <a href="staking_pool.md#0x0_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>, fungible_staked_ika_value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, fungible_staked_ika_principal_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, fungible_staked_ika_total_supply: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): (<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="staking_pool.md#0x0_staking_pool_calculate_fungible_staked_ika_withdraw_amount">calculate_fungible_staked_ika_withdraw_amount</a>(
    latest_exchange_rate: <a href="staking_pool.md#0x0_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>,
    fungible_staked_ika_value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    fungible_staked_ika_principal_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, // fungible_staked_ika_data.principal.value()
    fungible_staked_ika_total_supply: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, // fungible_staked_ika_data.total_supply
): (<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>) {
    // 1. <b>if</b> the entire fungible staked total supply supply is redeemed, how much <a href="../ika/ika.md#0x0_ika">ika</a> should we receive?
    <b>let</b> total_ika_amount = <a href="staking_pool.md#0x0_staking_pool_get_ika_amount">get_ika_amount</a>(
        &latest_exchange_rate,
        fungible_staked_ika_total_supply,
    );

    // <b>min</b> <b>with</b> total_ika_amount <b>to</b> prevent underflow
    <b>let</b> fungible_staked_ika_principal_amount = std::u64::min(
        fungible_staked_ika_principal_amount,
        total_ika_amount,
    );

    // 2. how much do we need <b>to</b> withdraw from the rewards pool?
    <b>let</b> total_rewards = total_ika_amount - fungible_staked_ika_principal_amount;

    // 3. proportionally withdraw from both wrt the fungible_staked_ika_value.
    <b>let</b> principal_withdraw_amount =
        (
            (fungible_staked_ika_value <b>as</b> u128)
            * (fungible_staked_ika_principal_amount <b>as</b> u128)
            / (fungible_staked_ika_total_supply <b>as</b> u128),
        ) <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>;

    <b>let</b> rewards_withdraw_amount =
        (
            (fungible_staked_ika_value <b>as</b> u128)
            * (total_rewards <b>as</b> u128)
            / (fungible_staked_ika_total_supply <b>as</b> u128),
        ) <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>;

    // <b>invariant</b> check, just in case
    <b>let</b> expected_ika_amount = <a href="staking_pool.md#0x0_staking_pool_get_ika_amount">get_ika_amount</a>(&latest_exchange_rate, fungible_staked_ika_value);
    <b>assert</b>!(
        principal_withdraw_amount + rewards_withdraw_amount &lt;= expected_ika_amount,
        <a href="staking_pool.md#0x0_staking_pool_EInvariantFailure">EInvariantFailure</a>,
    );

    (principal_withdraw_amount, rewards_withdraw_amount)
}
</code></pre>



</details>

<a name="0x0_staking_pool_convert_to_fungible_staked_ika"></a>

## Function `convert_to_fungible_staked_ika`

Convert the given staked IKA to an FungibleStakedIka object


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_convert_to_fungible_staked_ika">convert_to_fungible_staked_ika</a>(self: &<b>mut</b> <a href="staking_pool.md#0x0_staking_pool_StakingPool">staking_pool::StakingPool</a>, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: <a href="staked_ika.md#0x0_staked_ika_StakedIka">staked_ika::StakedIka</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_convert_to_fungible_staked_ika">convert_to_fungible_staked_ika</a>(
    self: &<b>mut</b> <a href="staking_pool.md#0x0_staking_pool_StakingPool">StakingPool</a>,
    epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: StakedIka,
    ctx: &<b>mut</b> TxContext,
): FungibleStakedIka {
    <b>let</b> validator_id = <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>.validator_id();
    <b>let</b> stake_activation_epoch = <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>.stake_activation_epoch();
    <b>let</b> principal = <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>.into_balance();

    <b>assert</b>!(validator_id == self.validator_id, <a href="staking_pool.md#0x0_staking_pool_EWrongPool">EWrongPool</a>);
    <b>assert</b>!(epoch &gt;= stake_activation_epoch, <a href="staking_pool.md#0x0_staking_pool_ECannotMintFungibleStakedIkaYet">ECannotMintFungibleStakedIkaYet</a>);


    <b>let</b> exchange_rate_at_staking_epoch = self.<a href="staking_pool.md#0x0_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(
        stake_activation_epoch,
    );

    <b>let</b> pool_token_amount = <a href="staking_pool.md#0x0_staking_pool_get_token_amount">get_token_amount</a>(
        &exchange_rate_at_staking_epoch,
        <a href="../sui-framework/balance.md#0x2_balance_value">balance::value</a>(&principal),
    );

    self.fungible_total_supply = self.fungible_total_supply + pool_token_amount;
    <a href="../sui-framework/balance.md#0x2_balance_join">balance::join</a>(&<b>mut</b> self.fungible_principal, principal);

    <a href="staked_ika.md#0x0_staked_ika_create_fungible">staked_ika::create_fungible</a>(validator_id, pool_token_amount, ctx)
}
</code></pre>



</details>

<a name="0x0_staking_pool_withdraw_from_principal"></a>

## Function `withdraw_from_principal`

Withdraw the principal IKA stored in the StakedIka object, and calculate the corresponding amount of pool
tokens using exchange rate at staking epoch.
Returns values are amount of pool tokens withdrawn and withdrawn principal portion of IKA.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_withdraw_from_principal">withdraw_from_principal</a>(pool: &<a href="staking_pool.md#0x0_staking_pool_StakingPool">staking_pool::StakingPool</a>, <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: <a href="staked_ika.md#0x0_staked_ika_StakedIka">staked_ika::StakedIka</a>): (<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_withdraw_from_principal">withdraw_from_principal</a>(
    pool: &<a href="staking_pool.md#0x0_staking_pool_StakingPool">StakingPool</a>,
    <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: StakedIka,
): (<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, Balance&lt;IKA&gt;) {
    // Check that the stake information matches the pool.
    <b>assert</b>!(<a href="staked_ika.md#0x0_staked_ika">staked_ika</a>.validator_id() == pool.validator_id, <a href="staking_pool.md#0x0_staking_pool_EWrongPool">EWrongPool</a>);

    <b>let</b> exchange_rate_at_staking_epoch = <a href="staking_pool.md#0x0_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(
        pool,
        <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>.stake_activation_epoch(),
    );
    <b>let</b> principal_withdraw = <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>.into_balance();
    <b>let</b> pool_token_withdraw_amount = <a href="staking_pool.md#0x0_staking_pool_get_token_amount">get_token_amount</a>(
        &exchange_rate_at_staking_epoch,
        principal_withdraw.value(),
    );

    (pool_token_withdraw_amount, principal_withdraw)
}
</code></pre>



</details>

<a name="0x0_staking_pool_deposit_rewards"></a>

## Function `deposit_rewards`

Called at epoch advancement times to add rewards (in IKA) to the staking pool.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_deposit_rewards">deposit_rewards</a>(pool: &<b>mut</b> <a href="staking_pool.md#0x0_staking_pool_StakingPool">staking_pool::StakingPool</a>, rewards: <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_deposit_rewards">deposit_rewards</a>(pool: &<b>mut</b> <a href="staking_pool.md#0x0_staking_pool_StakingPool">StakingPool</a>, rewards: Balance&lt;IKA&gt;) {
    pool.ika_balance = pool.ika_balance + rewards.value();
    pool.rewards_pool.join(rewards);
}
</code></pre>



</details>

<a name="0x0_staking_pool_process_pending_stakes_and_withdraws"></a>

## Function `process_pending_stakes_and_withdraws`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>(pool: &<b>mut</b> <a href="staking_pool.md#0x0_staking_pool_StakingPool">staking_pool::StakingPool</a>, new_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>(pool: &<b>mut</b> <a href="staking_pool.md#0x0_staking_pool_StakingPool">StakingPool</a>, new_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>) {
    <a href="staking_pool.md#0x0_staking_pool_process_pending_stake_withdraw">process_pending_stake_withdraw</a>(pool);
    <a href="staking_pool.md#0x0_staking_pool_process_pending_stake">process_pending_stake</a>(pool);
    pool
        .exchange_rates
        .add(
            new_epoch,
            <a href="staking_pool.md#0x0_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> {
                ika_amount: pool.ika_balance,
                pool_token_amount: pool.pool_token_balance,
            },
        );
    <a href="staking_pool.md#0x0_staking_pool_check_balance_invariants">check_balance_invariants</a>(pool, new_epoch);
}
</code></pre>



</details>

<a name="0x0_staking_pool_process_pending_stake_withdraw"></a>

## Function `process_pending_stake_withdraw`

Called at epoch boundaries to process pending stake withdraws requested during the epoch.
Also called immediately upon withdrawal if the pool is inactive.


<pre><code><b>fun</b> <a href="staking_pool.md#0x0_staking_pool_process_pending_stake_withdraw">process_pending_stake_withdraw</a>(pool: &<b>mut</b> <a href="staking_pool.md#0x0_staking_pool_StakingPool">staking_pool::StakingPool</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="staking_pool.md#0x0_staking_pool_process_pending_stake_withdraw">process_pending_stake_withdraw</a>(pool: &<b>mut</b> <a href="staking_pool.md#0x0_staking_pool_StakingPool">StakingPool</a>) {
    pool.ika_balance = pool.ika_balance - pool.pending_total_ika_withdraw;
    pool.pool_token_balance = pool.pool_token_balance - pool.pending_pool_token_withdraw;
    pool.pending_total_ika_withdraw = 0;
    pool.pending_pool_token_withdraw = 0;
}
</code></pre>



</details>

<a name="0x0_staking_pool_process_pending_stake"></a>

## Function `process_pending_stake`

Called at epoch boundaries to process the pending stake.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_process_pending_stake">process_pending_stake</a>(pool: &<b>mut</b> <a href="staking_pool.md#0x0_staking_pool_StakingPool">staking_pool::StakingPool</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_process_pending_stake">process_pending_stake</a>(pool: &<b>mut</b> <a href="staking_pool.md#0x0_staking_pool_StakingPool">StakingPool</a>) {
    // Use the most up <b>to</b> date exchange rate <b>with</b> the rewards deposited and withdraws effectuated.
    <b>let</b> latest_exchange_rate = <a href="staking_pool.md#0x0_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> {
        ika_amount: pool.ika_balance,
        pool_token_amount: pool.pool_token_balance,
    };
    pool.ika_balance = pool.ika_balance + pool.pending_stake;
    pool.pool_token_balance = <a href="staking_pool.md#0x0_staking_pool_get_token_amount">get_token_amount</a>(&latest_exchange_rate, pool.ika_balance);
    pool.pending_stake = 0;
}
</code></pre>



</details>

<a name="0x0_staking_pool_withdraw_rewards"></a>

## Function `withdraw_rewards`

This function does the following:
1. Calculates the total amount of IKA (including principal and rewards) that the provided pool tokens represent
at the current exchange rate.
2. Using the above number and the given <code>principal_withdraw_amount</code>, calculates the rewards portion of the
stake we should withdraw.
3. Withdraws the rewards portion from the rewards pool at the current exchange rate. We only withdraw the rewards
portion because the principal portion was already taken out of the staker's self custodied StakedIka.


<pre><code><b>fun</b> <a href="staking_pool.md#0x0_staking_pool_withdraw_rewards">withdraw_rewards</a>(pool: &<b>mut</b> <a href="staking_pool.md#0x0_staking_pool_StakingPool">staking_pool::StakingPool</a>, principal_withdraw_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, pool_token_withdraw_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="staking_pool.md#0x0_staking_pool_withdraw_rewards">withdraw_rewards</a>(
    pool: &<b>mut</b> <a href="staking_pool.md#0x0_staking_pool_StakingPool">StakingPool</a>,
    principal_withdraw_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    pool_token_withdraw_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
): Balance&lt;IKA&gt; {
    <b>let</b> exchange_rate = <a href="staking_pool.md#0x0_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(pool, epoch);
    <b>let</b> total_ika_withdraw_amount = <a href="staking_pool.md#0x0_staking_pool_get_ika_amount">get_ika_amount</a>(&exchange_rate, pool_token_withdraw_amount);
    <b>let</b> <b>mut</b> reward_withdraw_amount = <b>if</b> (total_ika_withdraw_amount &gt;= principal_withdraw_amount) {
        total_ika_withdraw_amount - principal_withdraw_amount
    } <b>else</b> 0;
    // This may happen when we are withdrawing everything from the pool and
    // the rewards pool <a href="../sui-framework/balance.md#0x2_balance">balance</a> may be less than reward_withdraw_amount.
    // TODO: FIGURE OUT EXACTLY WHY THIS CAN HAPPEN.
    reward_withdraw_amount = reward_withdraw_amount.<b>min</b>(pool.rewards_pool.value());
    pool.rewards_pool.split(reward_withdraw_amount)
}
</code></pre>



</details>

<a name="0x0_staking_pool_activate_staking_pool"></a>

## Function `activate_staking_pool`

Called by <code><a href="validator.md#0x0_validator">validator</a></code> module to activate a staking pool.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_activate_staking_pool">activate_staking_pool</a>(pool: &<b>mut</b> <a href="staking_pool.md#0x0_staking_pool_StakingPool">staking_pool::StakingPool</a>, activation_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_activate_staking_pool">activate_staking_pool</a>(pool: &<b>mut</b> <a href="staking_pool.md#0x0_staking_pool_StakingPool">StakingPool</a>, activation_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>) {
    // Add the initial exchange rate <b>to</b> the <a href="../sui-framework/table.md#0x2_table">table</a>.
    pool
        .exchange_rates
        .add(
            activation_epoch,
            <a href="staking_pool.md#0x0_staking_pool_initial_exchange_rate">initial_exchange_rate</a>(),
        );
    // Check that the pool is preactive and not inactive.
    <b>assert</b>!(<a href="staking_pool.md#0x0_staking_pool_is_candidate">is_candidate</a>(pool), <a href="staking_pool.md#0x0_staking_pool_EPoolAlreadyActive">EPoolAlreadyActive</a>);
    <b>assert</b>!(!<a href="staking_pool.md#0x0_staking_pool_is_inactive">is_inactive</a>(pool), <a href="staking_pool.md#0x0_staking_pool_EActivationOfInactivePool">EActivationOfInactivePool</a>);
    // Fill in the active epoch.
    pool.activation_epoch.fill(activation_epoch);
}
</code></pre>



</details>

<a name="0x0_staking_pool_deactivate_staking_pool"></a>

## Function `deactivate_staking_pool`

Deactivate a staking pool by setting the <code>deactivation_epoch</code>. After
this pool deactivation, the pool stops earning rewards. Only stake
withdraws can be made to the pool.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_deactivate_staking_pool">deactivate_staking_pool</a>(pool: &<b>mut</b> <a href="staking_pool.md#0x0_staking_pool_StakingPool">staking_pool::StakingPool</a>, deactivation_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_deactivate_staking_pool">deactivate_staking_pool</a>(pool: &<b>mut</b> <a href="staking_pool.md#0x0_staking_pool_StakingPool">StakingPool</a>, deactivation_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>) {
    // We can't deactivate an already deactivated pool.
    <b>assert</b>!(!<a href="staking_pool.md#0x0_staking_pool_is_inactive">is_inactive</a>(pool), <a href="staking_pool.md#0x0_staking_pool_EDeactivationOfInactivePool">EDeactivationOfInactivePool</a>);
    pool.deactivation_epoch = <a href="../move-stdlib/option.md#0x1_option_some">option::some</a>(deactivation_epoch);
}
</code></pre>



</details>

<a name="0x0_staking_pool_ika_balance"></a>

## Function `ika_balance`



<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_ika_balance">ika_balance</a>(pool: &<a href="staking_pool.md#0x0_staking_pool_StakingPool">staking_pool::StakingPool</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_ika_balance">ika_balance</a>(pool: &<a href="staking_pool.md#0x0_staking_pool_StakingPool">StakingPool</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> { pool.ika_balance }
</code></pre>



</details>

<a name="0x0_staking_pool_is_candidate"></a>

## Function `is_candidate`

Returns true if the input staking pool is candidate.


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_is_candidate">is_candidate</a>(pool: &<a href="staking_pool.md#0x0_staking_pool_StakingPool">staking_pool::StakingPool</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_is_candidate">is_candidate</a>(pool: &<a href="staking_pool.md#0x0_staking_pool_StakingPool">StakingPool</a>): bool {
    pool.activation_epoch.is_none()
}
</code></pre>



</details>

<a name="0x0_staking_pool_is_inactive"></a>

## Function `is_inactive`

Returns true if the input staking pool is inactive.


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_is_inactive">is_inactive</a>(pool: &<a href="staking_pool.md#0x0_staking_pool_StakingPool">staking_pool::StakingPool</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_is_inactive">is_inactive</a>(pool: &<a href="staking_pool.md#0x0_staking_pool_StakingPool">StakingPool</a>): bool {
    pool.deactivation_epoch.is_some()
}
</code></pre>



</details>

<a name="0x0_staking_pool_pool_token_exchange_rate_at_epoch"></a>

## Function `pool_token_exchange_rate_at_epoch`



<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(pool: &<a href="staking_pool.md#0x0_staking_pool_StakingPool">staking_pool::StakingPool</a>, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="staking_pool.md#0x0_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(
    pool: &<a href="staking_pool.md#0x0_staking_pool_StakingPool">StakingPool</a>,
    epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
): <a href="staking_pool.md#0x0_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> {
    // If the pool is preactive then the exchange rate is always 1:1.
    <b>if</b> (<a href="staking_pool.md#0x0_staking_pool_is_candidate_at_epoch">is_candidate_at_epoch</a>(pool, epoch)) {
        <b>return</b> <a href="staking_pool.md#0x0_staking_pool_initial_exchange_rate">initial_exchange_rate</a>()
    };
    <b>let</b> clamped_epoch = pool.deactivation_epoch.get_with_default(epoch);
    <b>let</b> <b>mut</b> epoch = clamped_epoch.<b>min</b>(epoch);
    <b>let</b> activation_epoch = *pool.activation_epoch.borrow();

    // Find the latest epoch that's earlier than the given epoch <b>with</b> an entry in the <a href="../sui-framework/table.md#0x2_table">table</a>
    <b>while</b> (epoch &gt;= activation_epoch) {
        <b>if</b> (pool.exchange_rates.contains(epoch)) {
            <b>return</b> pool.exchange_rates[epoch]
        };
        epoch = epoch - 1;
    };
    // This line really should be unreachable. Do we want an <b>assert</b> <b>false</b> here?
    <a href="staking_pool.md#0x0_staking_pool_initial_exchange_rate">initial_exchange_rate</a>()
}
</code></pre>



</details>

<a name="0x0_staking_pool_pending_stake_amount"></a>

## Function `pending_stake_amount`

Returns the total value of the pending staking requests for this staking pool.


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_pending_stake_amount">pending_stake_amount</a>(<a href="staking_pool.md#0x0_staking_pool">staking_pool</a>: &<a href="staking_pool.md#0x0_staking_pool_StakingPool">staking_pool::StakingPool</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_pending_stake_amount">pending_stake_amount</a>(<a href="staking_pool.md#0x0_staking_pool">staking_pool</a>: &<a href="staking_pool.md#0x0_staking_pool_StakingPool">StakingPool</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <a href="staking_pool.md#0x0_staking_pool">staking_pool</a>.pending_stake
}
</code></pre>



</details>

<a name="0x0_staking_pool_pending_stake_withdraw_amount"></a>

## Function `pending_stake_withdraw_amount`

Returns the total withdrawal from the staking pool this epoch.


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_pending_stake_withdraw_amount">pending_stake_withdraw_amount</a>(<a href="staking_pool.md#0x0_staking_pool">staking_pool</a>: &<a href="staking_pool.md#0x0_staking_pool_StakingPool">staking_pool::StakingPool</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_pending_stake_withdraw_amount">pending_stake_withdraw_amount</a>(<a href="staking_pool.md#0x0_staking_pool">staking_pool</a>: &<a href="staking_pool.md#0x0_staking_pool_StakingPool">StakingPool</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <a href="staking_pool.md#0x0_staking_pool">staking_pool</a>.pending_total_ika_withdraw
}
</code></pre>



</details>

<a name="0x0_staking_pool_exchange_rates"></a>

## Function `exchange_rates`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_exchange_rates">exchange_rates</a>(pool: &<a href="staking_pool.md#0x0_staking_pool_StakingPool">staking_pool::StakingPool</a>): &<a href="../sui-framework/table.md#0x2_table_Table">table::Table</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="staking_pool.md#0x0_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_exchange_rates">exchange_rates</a>(pool: &<a href="staking_pool.md#0x0_staking_pool_StakingPool">StakingPool</a>): &Table&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="staking_pool.md#0x0_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>&gt; {
    &pool.exchange_rates
}
</code></pre>



</details>

<a name="0x0_staking_pool_ika_amount"></a>

## Function `ika_amount`



<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_ika_amount">ika_amount</a>(exchange_rate: &<a href="staking_pool.md#0x0_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_ika_amount">ika_amount</a>(exchange_rate: &<a href="staking_pool.md#0x0_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    exchange_rate.ika_amount
}
</code></pre>



</details>

<a name="0x0_staking_pool_pool_token_amount"></a>

## Function `pool_token_amount`



<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_pool_token_amount">pool_token_amount</a>(exchange_rate: &<a href="staking_pool.md#0x0_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x0_staking_pool_pool_token_amount">pool_token_amount</a>(exchange_rate: &<a href="staking_pool.md#0x0_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    exchange_rate.pool_token_amount
}
</code></pre>



</details>

<a name="0x0_staking_pool_is_candidate_at_epoch"></a>

## Function `is_candidate_at_epoch`

Returns true if the provided staking pool is preactive at the provided epoch.


<pre><code><b>fun</b> <a href="staking_pool.md#0x0_staking_pool_is_candidate_at_epoch">is_candidate_at_epoch</a>(pool: &<a href="staking_pool.md#0x0_staking_pool_StakingPool">staking_pool::StakingPool</a>, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="staking_pool.md#0x0_staking_pool_is_candidate_at_epoch">is_candidate_at_epoch</a>(pool: &<a href="staking_pool.md#0x0_staking_pool_StakingPool">StakingPool</a>, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): bool {
    // Either the pool is currently preactive or the pool's starting epoch is later than the provided epoch.
    <a href="staking_pool.md#0x0_staking_pool_is_candidate">is_candidate</a>(pool) || (*pool.activation_epoch.borrow() &gt; epoch)
}
</code></pre>



</details>

<a name="0x0_staking_pool_get_ika_amount"></a>

## Function `get_ika_amount`



<pre><code><b>fun</b> <a href="staking_pool.md#0x0_staking_pool_get_ika_amount">get_ika_amount</a>(exchange_rate: &<a href="staking_pool.md#0x0_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>, token_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="staking_pool.md#0x0_staking_pool_get_ika_amount">get_ika_amount</a>(exchange_rate: &<a href="staking_pool.md#0x0_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>, token_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    // When either amount is 0, that means we have no stakes <b>with</b> this pool.
    // The other amount might be non-zero when there's dust left in the pool.
    <b>if</b> (exchange_rate.ika_amount == 0 || exchange_rate.pool_token_amount == 0) {
        <b>return</b> token_amount
    };
    <b>let</b> res =
        exchange_rate.ika_amount <b>as</b> u128
                * (token_amount <b>as</b> u128)
                / (exchange_rate.pool_token_amount <b>as</b> u128);
    res <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
}
</code></pre>



</details>

<a name="0x0_staking_pool_get_token_amount"></a>

## Function `get_token_amount`



<pre><code><b>fun</b> <a href="staking_pool.md#0x0_staking_pool_get_token_amount">get_token_amount</a>(exchange_rate: &<a href="staking_pool.md#0x0_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>, ika_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="staking_pool.md#0x0_staking_pool_get_token_amount">get_token_amount</a>(exchange_rate: &<a href="staking_pool.md#0x0_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>, ika_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    // When either amount is 0, that means we have no stakes <b>with</b> this pool.
    // The other amount might be non-zero when there's dust left in the pool.
    <b>if</b> (exchange_rate.ika_amount == 0 || exchange_rate.pool_token_amount == 0) {
        <b>return</b> ika_amount
    };
    <b>let</b> res =
        exchange_rate.pool_token_amount <b>as</b> u128
                * (ika_amount <b>as</b> u128)
                / (exchange_rate.ika_amount <b>as</b> u128);
    res <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
}
</code></pre>



</details>

<a name="0x0_staking_pool_initial_exchange_rate"></a>

## Function `initial_exchange_rate`



<pre><code><b>fun</b> <a href="staking_pool.md#0x0_staking_pool_initial_exchange_rate">initial_exchange_rate</a>(): <a href="staking_pool.md#0x0_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="staking_pool.md#0x0_staking_pool_initial_exchange_rate">initial_exchange_rate</a>(): <a href="staking_pool.md#0x0_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> {
    <a href="staking_pool.md#0x0_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> { ika_amount: 0, pool_token_amount: 0 }
}
</code></pre>



</details>

<a name="0x0_staking_pool_check_balance_invariants"></a>

## Function `check_balance_invariants`



<pre><code><b>fun</b> <a href="staking_pool.md#0x0_staking_pool_check_balance_invariants">check_balance_invariants</a>(pool: &<a href="staking_pool.md#0x0_staking_pool_StakingPool">staking_pool::StakingPool</a>, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="staking_pool.md#0x0_staking_pool_check_balance_invariants">check_balance_invariants</a>(pool: &<a href="staking_pool.md#0x0_staking_pool_StakingPool">StakingPool</a>, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>) {
    <b>let</b> exchange_rate = <a href="staking_pool.md#0x0_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(pool, epoch);
    // check that the pool token <a href="../sui-framework/balance.md#0x2_balance">balance</a> and <a href="../ika/ika.md#0x0_ika">ika</a> <a href="../sui-framework/balance.md#0x2_balance">balance</a> ratio matches the exchange rate stored.
    <b>let</b> expected = <a href="staking_pool.md#0x0_staking_pool_get_token_amount">get_token_amount</a>(&exchange_rate, pool.ika_balance);
    <b>let</b> actual = pool.pool_token_balance;
    <b>assert</b>!(expected == actual, <a href="staking_pool.md#0x0_staking_pool_ETokenBalancesDoNotMatchExchangeRate">ETokenBalancesDoNotMatchExchangeRate</a>)
}
</code></pre>



</details>
