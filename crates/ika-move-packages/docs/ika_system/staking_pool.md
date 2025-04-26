---
title: Module `(ika_system=0x0)::staking_pool`
---



-  [Struct `StakingPool`](#(ika_system=0x0)_staking_pool_StakingPool)
-  [Enum `PoolState`](#(ika_system=0x0)_staking_pool_PoolState)
-  [Constants](#@Constants_0)
-  [Function `new`](#(ika_system=0x0)_staking_pool_new)
-  [Function `activate`](#(ika_system=0x0)_staking_pool_activate)
-  [Function `set_withdrawing`](#(ika_system=0x0)_staking_pool_set_withdrawing)
-  [Function `deactivate`](#(ika_system=0x0)_staking_pool_deactivate)
-  [Function `stake`](#(ika_system=0x0)_staking_pool_stake)
-  [Function `request_withdraw_stake`](#(ika_system=0x0)_staking_pool_request_withdraw_stake)
-  [Function `withdraw_stake`](#(ika_system=0x0)_staking_pool_withdraw_stake)
-  [Function `advance_epoch`](#(ika_system=0x0)_staking_pool_advance_epoch)
-  [Function `process_pending_stake`](#(ika_system=0x0)_staking_pool_process_pending_stake)
-  [Function `set_name`](#(ika_system=0x0)_staking_pool_set_name)
-  [Function `set_validator_metadata`](#(ika_system=0x0)_staking_pool_set_validator_metadata)
-  [Function `set_next_commission`](#(ika_system=0x0)_staking_pool_set_next_commission)
-  [Function `set_next_epoch_network_address`](#(ika_system=0x0)_staking_pool_set_next_epoch_network_address)
-  [Function `set_next_epoch_p2p_address`](#(ika_system=0x0)_staking_pool_set_next_epoch_p2p_address)
-  [Function `set_next_epoch_consensus_address`](#(ika_system=0x0)_staking_pool_set_next_epoch_consensus_address)
-  [Function `set_next_epoch_protocol_pubkey_bytes`](#(ika_system=0x0)_staking_pool_set_next_epoch_protocol_pubkey_bytes)
-  [Function `set_next_epoch_network_pubkey_bytes`](#(ika_system=0x0)_staking_pool_set_next_epoch_network_pubkey_bytes)
-  [Function `set_next_epoch_consensus_pubkey_bytes`](#(ika_system=0x0)_staking_pool_set_next_epoch_consensus_pubkey_bytes)
-  [Function `set_next_epoch_class_groups_pubkey_and_proof_bytes`](#(ika_system=0x0)_staking_pool_set_next_epoch_class_groups_pubkey_and_proof_bytes)
-  [Function `destroy_empty`](#(ika_system=0x0)_staking_pool_destroy_empty)
-  [Function `exchange_rate_at_epoch`](#(ika_system=0x0)_staking_pool_exchange_rate_at_epoch)
-  [Function `ika_balance_at_epoch`](#(ika_system=0x0)_staking_pool_ika_balance_at_epoch)
-  [Function `new_validator_operation_cap`](#(ika_system=0x0)_staking_pool_new_validator_operation_cap)
-  [Function `new_validator_commission_cap`](#(ika_system=0x0)_staking_pool_new_validator_commission_cap)
-  [Function `collect_commission`](#(ika_system=0x0)_staking_pool_collect_commission)
-  [Function `validator_id`](#(ika_system=0x0)_staking_pool_validator_id)
-  [Function `validator_cap_id`](#(ika_system=0x0)_staking_pool_validator_cap_id)
-  [Function `operation_cap_id`](#(ika_system=0x0)_staking_pool_operation_cap_id)
-  [Function `commission_cap_id`](#(ika_system=0x0)_staking_pool_commission_cap_id)
-  [Function `commission_rate`](#(ika_system=0x0)_staking_pool_commission_rate)
-  [Function `commission_amount`](#(ika_system=0x0)_staking_pool_commission_amount)
-  [Function `rewards_amount`](#(ika_system=0x0)_staking_pool_rewards_amount)
-  [Function `ika_balance`](#(ika_system=0x0)_staking_pool_ika_balance)
-  [Function `activation_epoch`](#(ika_system=0x0)_staking_pool_activation_epoch)
-  [Function `validator_info`](#(ika_system=0x0)_staking_pool_validator_info)
-  [Function `is_preactive`](#(ika_system=0x0)_staking_pool_is_preactive)
-  [Function `is_active`](#(ika_system=0x0)_staking_pool_is_active)
-  [Function `is_withdrawing`](#(ika_system=0x0)_staking_pool_is_withdrawing)
-  [Function `withdrawing_epoch`](#(ika_system=0x0)_staking_pool_withdrawing_epoch)
-  [Function `is_preactive_at_epoch`](#(ika_system=0x0)_staking_pool_is_preactive_at_epoch)
-  [Function `exchange_rates`](#(ika_system=0x0)_staking_pool_exchange_rates)
-  [Function `is_empty`](#(ika_system=0x0)_staking_pool_is_empty)
-  [Function `calculate_rewards`](#(ika_system=0x0)_staking_pool_calculate_rewards)


<pre><code><b>use</b> (ika=0x0)::ika;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof">class_groups_public_key_and_proof</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/extended_field.md#(ika_system=0x0)_extended_field">extended_field</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/multiaddr.md#(ika_system=0x0)_multiaddr">multiaddr</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values">pending_values</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/exchange_rate.md#(ika_system=0x0)_pool_exchange_rate">pool_exchange_rate</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap">validator_cap</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata">validator_metadata</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u64.md#std_u64">std::u64</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../sui/address.md#sui_address">sui::address</a>;
<b>use</b> <a href="../sui/bag.md#sui_bag">sui::bag</a>;
<b>use</b> <a href="../sui/balance.md#sui_balance">sui::balance</a>;
<b>use</b> <a href="../sui/bcs.md#sui_bcs">sui::bcs</a>;
<b>use</b> <a href="../sui/bls12381.md#sui_bls12381">sui::bls12381</a>;
<b>use</b> <a href="../sui/coin.md#sui_coin">sui::coin</a>;
<b>use</b> <a href="../sui/config.md#sui_config">sui::config</a>;
<b>use</b> <a href="../sui/deny_list.md#sui_deny_list">sui::deny_list</a>;
<b>use</b> <a href="../sui/dynamic_field.md#sui_dynamic_field">sui::dynamic_field</a>;
<b>use</b> <a href="../sui/dynamic_object_field.md#sui_dynamic_object_field">sui::dynamic_object_field</a>;
<b>use</b> <a href="../sui/event.md#sui_event">sui::event</a>;
<b>use</b> <a href="../sui/group_ops.md#sui_group_ops">sui::group_ops</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/table.md#sui_table">sui::table</a>;
<b>use</b> <a href="../sui/table_vec.md#sui_table_vec">sui::table_vec</a>;
<b>use</b> <a href="../sui/transfer.md#sui_transfer">sui::transfer</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
<b>use</b> <a href="../sui/types.md#sui_types">sui::types</a>;
<b>use</b> <a href="../sui/url.md#sui_url">sui::url</a>;
<b>use</b> <a href="../sui/vec_map.md#sui_vec_map">sui::vec_map</a>;
<b>use</b> <a href="../sui/vec_set.md#sui_vec_set">sui::vec_set</a>;
</code></pre>



<a name="(ika_system=0x0)_staking_pool_StakingPool"></a>

## Struct `StakingPool`

Represents a single staking pool for a token. Even though it is never
transferred or shared, the <code>key</code> ability is added for discoverability
in the <code>ObjectTable</code>.

High level overview of the staking pool:
The pool maintains a balance of IKA 'ika_balance' that is increased
when stakes/rewards are added to the pool, and is decreased when
stakes are withdrawn.
To track the users' portion of the pool, we associate shares to the
staked IKA. Initially, the share price is 1 IKA per share.
When a new stake is added to the pool, the total number of shares
increases by an amount that corresponds to the share price at that
time. E.g., if the share price is 2 IKA per share, and 10 IKA are
added to the pool, the total number of shares is increased by 5
shares. The total number of shares is stored in 'num_shares'.

As stakes are added/withdrawn only in the granularity of epochs, we
maintain a share price per epoch in 'exchange_rates'.
StakedIka objects only need to store the epoch when they are created,
and the amount of IKA they locked. Whenever a settlement is performed
for a StakedIka, we calculate the number of shares that correspond to
the amount of IKA that was locked using the exchange rate at the time
of the lock, and then convert it to the amount of IKA that corresponds
to the current share price.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a> <b>has</b> key, store
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
<code><a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>: (ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a></code>
</dt>
<dd>
 The validator info for the pool.
</dd>
<dt>
<code>state: (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_PoolState">staking_pool::PoolState</a></code>
</dt>
<dd>
 The current state of the pool.
</dd>
<dt>
<code><a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
 The epoch when the pool is / will be activated.
 Serves information purposes only, the checks are performed in the <code>state</code>
 property.
</dd>
<dt>
<code>latest_epoch: u64</code>
</dt>
<dd>
 Epoch when the pool was last updated.
</dd>
<dt>
<code><a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a>: u64</code>
</dt>
<dd>
 Currently staked IKA in the pool + rewards pool.
</dd>
<dt>
<code>num_shares: u64</code>
</dt>
<dd>
 The total number of shares in the current epoch.
</dd>
<dt>
<code>pending_shares_withdraw: (ika_system=0x0)::<a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">pending_values::PendingValues</a></code>
</dt>
<dd>
 The amount of the shares that will be withdrawn in E+1 or E+2.
 We use this amount to calculate the IKA withdrawal in the
 <code><a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_process_pending_stake">process_pending_stake</a></code>.
</dd>
<dt>
<code>pre_active_withdrawals: (ika_system=0x0)::<a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">pending_values::PendingValues</a></code>
</dt>
<dd>
 The amount of the stake requested for withdrawal for a node that may
 part of the next committee. Stores principals of not yet active stakes.
 In practice, those tokens are staked for exactly one epoch.
</dd>
<dt>
<code>pending_commission_rate: (ika_system=0x0)::<a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">pending_values::PendingValues</a></code>
</dt>
<dd>
 The pending commission rate for the pool. Commission rate is applied in
 E+2, so we store the value for the matching epoch and apply it in the
 <code><a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_advance_epoch">advance_epoch</a></code> function.
</dd>
<dt>
<code><a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_commission_rate">commission_rate</a>: u16</code>
</dt>
<dd>
 The commission rate for the pool, in basis points.
</dd>
<dt>
<code><a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_exchange_rates">exchange_rates</a>: <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, (ika_system=0x0)::<a href="../ika_system/exchange_rate.md#(ika_system=0x0)_pool_exchange_rate_PoolExchangeRate">pool_exchange_rate::PoolExchangeRate</a>&gt;</code>
</dt>
<dd>
 Historical exchange rates for the pool. The key is the epoch when the
 exchange rate was set, and the value is the exchange rate (the ratio of
 the amount of IKA tokens for the pool shares).
</dd>
<dt>
<code>pending_stake: (ika_system=0x0)::<a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">pending_values::PendingValues</a></code>
</dt>
<dd>
 The amount of stake that will be added to the <code><a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a></code>. Can hold
 up to two keys: E+1 and E+2, due to the differences in the activation
 epoch.
 ```
 E+1 -> Balance
 E+2 -> Balance
 ```
 Single key is cleared in the <code><a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_advance_epoch">advance_epoch</a></code> function, leaving only the
 next epoch's stake.
</dd>
<dt>
<code>rewards_pool: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;</code>
</dt>
<dd>
 The rewards that the pool has received from being in the committee.
</dd>
<dt>
<code>commission: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;</code>
</dt>
<dd>
 The commission that the pool has received from the rewards.
</dd>
<dt>
<code><a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_cap_id">validator_cap_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The ID of this validator's <code>ValidatorCap</code>
</dd>
<dt>
<code><a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_operation_cap_id">operation_cap_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The ID of this validator's current valid <code>ValidatorOperationCap</code>
</dd>
<dt>
<code><a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_commission_cap_id">commission_cap_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The ID of this validator's current valid <code>ValidatorCommissionCap</code>
</dd>
<dt>
<code>extra_fields: <a href="../sui/bag.md#sui_bag_Bag">sui::bag::Bag</a></code>
</dt>
<dd>
 Reserved for future use and migrations.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_staking_pool_PoolState"></a>

## Enum `PoolState`

Represents the state of the staking pool.


<pre><code><b>public</b> <b>enum</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_PoolState">PoolState</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Variants</summary>


<dl>
<dt>
Variant <code>Active</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>PreActive</code>
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
<code>0: u64</code>
</dt>
<dd>
</dd>
</dl>

</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="(ika_system=0x0)_staking_pool_EActivationEpochNotReached"></a>

Attempt to withdraw before <code><a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a></code>.


<pre><code><b>const</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EActivationEpochNotReached">EActivationEpochNotReached</a>: u64 = 12;
</code></pre>



<a name="(ika_system=0x0)_staking_pool_EAuthorizationFailure"></a>

Trying to collect commission or change receiver without authorization.


<pre><code><b>const</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EAuthorizationFailure">EAuthorizationFailure</a>: u64 = 15;
</code></pre>



<a name="(ika_system=0x0)_staking_pool_ECalculationError"></a>

Error in a calculation. Indicates that a sanity check failed.


<pre><code><b>const</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ECalculationError">ECalculationError</a>: u64 = 1;
</code></pre>



<a name="(ika_system=0x0)_staking_pool_EIncorrectCommissionRate"></a>

Incorrect commission rate.


<pre><code><b>const</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EIncorrectCommissionRate">EIncorrectCommissionRate</a>: u64 = 14;
</code></pre>



<a name="(ika_system=0x0)_staking_pool_EIncorrectEpochAdvance"></a>

The state of the pool and the parameters to advance the epoch are not consistent.


<pre><code><b>const</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EIncorrectEpochAdvance">EIncorrectEpochAdvance</a>: u64 = 2;
</code></pre>



<a name="(ika_system=0x0)_staking_pool_EIncorrectPoolId"></a>

Trying to withdraw stake from the incorrect pool.


<pre><code><b>const</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EIncorrectPoolId">EIncorrectPoolId</a>: u64 = 9;
</code></pre>



<a name="(ika_system=0x0)_staking_pool_ENotStaked"></a>

StakedIka is already in <code>Withdrawing</code> state.


<pre><code><b>const</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ENotStaked">ENotStaked</a>: u64 = 8;
</code></pre>



<a name="(ika_system=0x0)_staking_pool_ENotWithdrawing"></a>

Trying to withdraw active stake.


<pre><code><b>const</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ENotWithdrawing">ENotWithdrawing</a>: u64 = 10;
</code></pre>



<a name="(ika_system=0x0)_staking_pool_EPoolAlreadyUpdated"></a>

The epoch of the pool has already been advanced.


<pre><code><b>const</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EPoolAlreadyUpdated">EPoolAlreadyUpdated</a>: u64 = 0;
</code></pre>



<a name="(ika_system=0x0)_staking_pool_EPoolAlreadyWithdrawing"></a>

Trying to set the pool to withdrawing state when it is already withdrawing.


<pre><code><b>const</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EPoolAlreadyWithdrawing">EPoolAlreadyWithdrawing</a>: u64 = 5;
</code></pre>



<a name="(ika_system=0x0)_staking_pool_EPoolIsNotActive"></a>

Pool is not in <code>Active</code> state.


<pre><code><b>const</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EPoolIsNotActive">EPoolIsNotActive</a>: u64 = 6;
</code></pre>



<a name="(ika_system=0x0)_staking_pool_EPoolIsNotPreActive"></a>

Pool is not in <code>PreActive</code> state.


<pre><code><b>const</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EPoolIsNotPreActive">EPoolIsNotPreActive</a>: u64 = 4;
</code></pre>



<a name="(ika_system=0x0)_staking_pool_EPoolNotEmpty"></a>

Trying to destroy a non-empty pool.


<pre><code><b>const</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EPoolNotEmpty">EPoolNotEmpty</a>: u64 = 3;
</code></pre>



<a name="(ika_system=0x0)_staking_pool_EWithdrawDirectly"></a>

Requesting withdrawal for the stake that can be withdrawn directly.


<pre><code><b>const</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EWithdrawDirectly">EWithdrawDirectly</a>: u64 = 13;
</code></pre>



<a name="(ika_system=0x0)_staking_pool_EWithdrawEpochNotReached"></a>

Attempt to withdraw before <code>withdraw_epoch</code>.


<pre><code><b>const</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EWithdrawEpochNotReached">EWithdrawEpochNotReached</a>: u64 = 11;
</code></pre>



<a name="(ika_system=0x0)_staking_pool_EZeroShares"></a>

The number of shares for the staked IKA are zero.


<pre><code><b>const</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EZeroShares">EZeroShares</a>: u64 = 16;
</code></pre>



<a name="(ika_system=0x0)_staking_pool_EZeroStake"></a>

Trying to stake zero amount.


<pre><code><b>const</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EZeroStake">EZeroStake</a>: u64 = 7;
</code></pre>



<a name="(ika_system=0x0)_staking_pool_N_BASIS_POINTS"></a>



<pre><code><b>const</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_N_BASIS_POINTS">N_BASIS_POINTS</a>: u16 = 10000;
</code></pre>



<a name="(ika_system=0x0)_staking_pool_new"></a>

## Function `new`

Create a new <code><a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a></code> object.
If committee is selected, the pool will be activated in the next epoch.
Otherwise, it will be activated in the current epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_new">new</a>(current_epoch: u64, name: <a href="../std/string.md#std_string_String">std::string::String</a>, protocol_pubkey_bytes: vector&lt;u8&gt;, network_pubkey_bytes: vector&lt;u8&gt;, consensus_pubkey_bytes: vector&lt;u8&gt;, class_groups_pubkey_and_proof_bytes: (ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof</a>, proof_of_possession_bytes: vector&lt;u8&gt;, network_address: <a href="../std/string.md#std_string_String">std::string::String</a>, p2p_address: <a href="../std/string.md#std_string_String">std::string::String</a>, consensus_address: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_commission_rate">commission_rate</a>: u16, metadata: (ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">validator_metadata::ValidatorMetadata</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): ((ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCommissionCap">validator_cap::ValidatorCommissionCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_new">new</a>(
    current_epoch: u64,
    name: String,
    protocol_pubkey_bytes: vector&lt;u8&gt;,
    network_pubkey_bytes: vector&lt;u8&gt;,
    consensus_pubkey_bytes: vector&lt;u8&gt;,
    class_groups_pubkey_and_proof_bytes: ClassGroupsPublicKeyAndProof,
    proof_of_possession_bytes: vector&lt;u8&gt;,
    network_address: String,
    p2p_address: String,
    consensus_address: String,
    <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_commission_rate">commission_rate</a>: u16,
    metadata: ValidatorMetadata,
    ctx: &<b>mut</b> TxContext,
): (<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>, ValidatorCap, ValidatorOperationCap, ValidatorCommissionCap) {
    <b>let</b> id = object::new(ctx);
    <b>let</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a> = id.to_inner();
    <b>let</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap">validator_cap</a> = <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_new_validator_cap">validator_cap::new_validator_cap</a>(<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>, ctx);
    <b>let</b> validator_operation_cap = <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_new_validator_operation_cap">validator_cap::new_validator_operation_cap</a>(<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>, ctx);
    <b>let</b> validator_commission_cap = <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_new_validator_commission_cap">validator_cap::new_validator_commission_cap</a>(<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>, ctx);
    <b>let</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a> = <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a> {
        id,
        <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>: <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_new">validator_info::new</a>(
            name,
            <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>,
            protocol_pubkey_bytes,
            network_pubkey_bytes,
            consensus_pubkey_bytes,
            class_groups_pubkey_and_proof_bytes,
            proof_of_possession_bytes,
            network_address,
            p2p_address,
            consensus_address,
            metadata,
            ctx,
        ),
        state: PoolState::PreActive,
        <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_exchange_rates">exchange_rates</a>: table::new(ctx),
        <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_commission_rate">commission_rate</a>,
        <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a>: option::none(),
        latest_epoch: current_epoch,
        pending_stake: <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_empty">pending_values::empty</a>(),
        pending_shares_withdraw: <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_empty">pending_values::empty</a>(),
        pre_active_withdrawals: <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_empty">pending_values::empty</a>(),
        pending_commission_rate: <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_empty">pending_values::empty</a>(),
        <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a>: 0,
        num_shares: 0,
        rewards_pool: balance::zero(),
        commission: balance::zero(),
        <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_cap_id">validator_cap_id</a>: object::id(&<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap">validator_cap</a>),
        <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_operation_cap_id">operation_cap_id</a>: object::id(&validator_operation_cap),
        <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_commission_cap_id">commission_cap_id</a>: object::id(&validator_commission_cap),
        extra_fields: bag::new(ctx),
    };
    (
        <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a>,
        <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap">validator_cap</a>,
        validator_operation_cap,
        validator_commission_cap
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_activate"></a>

## Function `activate`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activate">activate</a>(pool: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap">validator_cap</a>: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, current_epoch: u64, committee_selected: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activate">activate</a>(
    pool: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>,
    <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap">validator_cap</a>: &ValidatorCap,
    current_epoch: u64,
    committee_selected: bool,
) {
    <b>assert</b>!(<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap">validator_cap</a>.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>() == pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>(), <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EAuthorizationFailure">EAuthorizationFailure</a>);
    <b>assert</b>!(object::id(<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap">validator_cap</a>) == pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_cap_id">validator_cap_id</a>, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EAuthorizationFailure">EAuthorizationFailure</a>);
    <b>assert</b>!(pool.state == PoolState::PreActive, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EPoolIsNotPreActive">EPoolIsNotPreActive</a>);
    <b>let</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a> = <b>if</b> (committee_selected) {
        current_epoch + 1
    } <b>else</b> {
        current_epoch
    };
    // // Add the initial exchange rate to the table.
    // pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_exchange_rates">exchange_rates</a>.add(<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a>, <a href="../ika_system/exchange_rate.md#(ika_system=0x0)_pool_exchange_rate_flat">pool_exchange_rate::flat</a>());
    pool.state = PoolState::Active;
    pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a>.fill(<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_set_withdrawing"></a>

## Function `set_withdrawing`

Set the state of the pool to <code>Withdrawing</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_withdrawing">set_withdrawing</a>(pool: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap">validator_cap</a>: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, current_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_withdrawing">set_withdrawing</a>(
    pool: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>,
    <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap">validator_cap</a>: &ValidatorCap,
    current_epoch: u64,
) {
    <b>assert</b>!(<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap">validator_cap</a>.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>() == pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>(), <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EAuthorizationFailure">EAuthorizationFailure</a>);
    <b>assert</b>!(object::id(<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap">validator_cap</a>) == pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_cap_id">validator_cap_id</a>, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EAuthorizationFailure">EAuthorizationFailure</a>);
    <b>assert</b>!(!pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_is_withdrawing">is_withdrawing</a>(), <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EPoolAlreadyWithdrawing">EPoolAlreadyWithdrawing</a>);
    pool.state = PoolState::Withdrawing(current_epoch + 1);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_deactivate"></a>

## Function `deactivate`

Set the state of the pool to <code>Withdrawing</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_deactivate">deactivate</a>(pool: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, deactivation_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_deactivate">deactivate</a>(
    pool: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>,
    deactivation_epoch: u64,
) {
    pool.state = PoolState::Withdrawing(deactivation_epoch);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_stake"></a>

## Function `stake`

Stake the given amount of IKA in the pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_stake">stake</a>(pool: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, to_stake: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;, current_epoch: u64, committee_selected: bool, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_stake">stake</a>(
    pool: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>,
    to_stake: Balance&lt;IKA&gt;,
    current_epoch: u64,
    committee_selected: bool,
    ctx: &<b>mut</b> TxContext,
): StakedIka {
    <b>assert</b>!(pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_is_preactive">is_preactive</a>() || pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_is_active">is_active</a>(), <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EPoolIsNotActive">EPoolIsNotActive</a>);
    <b>assert</b>!(to_stake.value() &gt; 0, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EZeroStake">EZeroStake</a>);
    <b>let</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a> = <b>if</b> (committee_selected) {
        current_epoch + 2
    } <b>else</b> {
        current_epoch + 1
    };
    <b>let</b> staked_amount = to_stake.value();
    <b>let</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a> = <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_mint">staked_ika::mint</a>(
        pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>(),
        to_stake,
        <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a>,
        ctx,
    );
    // Add the <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_stake">stake</a> to the pending <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_stake">stake</a> either <b>for</b> E+1 or E+2.
    pool.pending_stake.insert_or_add(<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a>, staked_amount);
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_request_withdraw_stake"></a>

## Function `request_withdraw_stake`

Request withdrawal of the given amount from the staked IKA.
Marks the <code>StakedIka</code> as withdrawing and updates the activation epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_request_withdraw_stake">request_withdraw_stake</a>(pool: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>, in_current_committee: bool, in_next_committee: bool, current_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_request_withdraw_stake">request_withdraw_stake</a>(
    pool: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>,
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: &<b>mut</b> StakedIka,
    in_current_committee: bool,
    in_next_committee: bool,
    current_epoch: u64,
) {
    <b>assert</b>!(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.value() &gt; 0, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EZeroStake">EZeroStake</a>);
    <b>assert</b>!(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>() == pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>(), <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EIncorrectPoolId">EIncorrectPoolId</a>);
    <b>assert</b>!(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.is_staked(), <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ENotStaked">ENotStaked</a>);
    // Only allow requesting <b>if</b> the <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_stake">stake</a> cannot be withdrawn directly.
    <b>assert</b>!(!<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.can_withdraw_early(in_next_committee, current_epoch), <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EWithdrawDirectly">EWithdrawDirectly</a>);
    // Early withdrawal request: only possible <b>if</b> activation epoch <b>has</b> not been
    // reached, and the <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_stake">stake</a> is already counted <b>for</b> the next committee selection.
    <b>if</b> (<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a>() == current_epoch + 1) {
        <b>let</b> withdraw_epoch = <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a>() + 1;
        // register principal in the early withdrawals, the value will get converted to
        // the token amount in the `<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_process_pending_stake">process_pending_stake</a>` function
        pool.pre_active_withdrawals.insert_or_add(withdraw_epoch, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.value());
        <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_withdrawing">set_withdrawing</a>(withdraw_epoch);
        <b>return</b>
    };
    <b>assert</b>!(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a>() &lt;= current_epoch, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EActivationEpochNotReached">EActivationEpochNotReached</a>);
    // If the node is in the committee, the <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_stake">stake</a> will be withdrawn in E+2,
    // otherwise in E+1.
    <b>let</b> withdraw_epoch = <b>if</b> (in_next_committee) {
        current_epoch + 2
    } <b>else</b> <b>if</b> (in_current_committee) {
        current_epoch + 1
    } <b>else</b> {
        <b>abort</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EWithdrawDirectly">EWithdrawDirectly</a>
    };
    <b>let</b> principal_amount = <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.value();
    <b>let</b> share_amount = pool
        .<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_exchange_rate_at_epoch">exchange_rate_at_epoch</a>(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a>())
        .convert_to_share_amount(principal_amount);
    <b>assert</b>!(share_amount != 0, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EZeroShares">EZeroShares</a>);
    pool.pending_shares_withdraw.insert_or_add(withdraw_epoch, share_amount);
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_withdrawing">set_withdrawing</a>(withdraw_epoch);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_withdraw_stake"></a>

## Function `withdraw_stake`

Perform the withdrawal of the staked IKA, returning the amount to the caller.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_withdraw_stake">withdraw_stake</a>(pool: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>, in_current_committee: bool, in_next_committee: bool, current_epoch: u64): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_withdraw_stake">withdraw_stake</a>(
    pool: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>,
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: StakedIka,
    in_current_committee: bool,
    in_next_committee: bool,
    current_epoch: u64,
): Balance&lt;IKA&gt; {
    <b>assert</b>!(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.value() &gt; 0, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EZeroStake">EZeroStake</a>);
    <b>assert</b>!(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>() == pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>(), <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EIncorrectPoolId">EIncorrectPoolId</a>);
    <b>let</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a> = <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a>();
    // One step, early withdrawal in the case when committee before
    // activation epoch hasn't been selected. covers both E+1 and E+2 cases.
    <b>if</b> (<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.can_withdraw_early(in_next_committee, current_epoch)) {
        pool.pending_stake.reduce(<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.value());
        <b>return</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.into_balance()
    };
    <b>let</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_rewards_amount">rewards_amount</a> = <b>if</b> (
        !in_current_committee && !in_next_committee && <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.is_staked()
    ) {
        // One step withdrawal <b>for</b> an inactive node.
        <b>if</b> (<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a> &gt; current_epoch) {
            // Not even active <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_stake">stake</a> yet, remove from pending <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_stake">stake</a>.
            pool.pending_stake.reduce(<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.value());
            0
        } <b>else</b> {
            // Active <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_stake">stake</a>, remove it with the current epoch <b>as</b> the withdraw epoch.
            <b>let</b> share_amount = pool
                .<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_exchange_rate_at_epoch">exchange_rate_at_epoch</a>(<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a>)
                .convert_to_share_amount(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.value());
            pool.pending_shares_withdraw.insert_or_add(current_epoch, share_amount);
            pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_calculate_rewards">calculate_rewards</a>(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.value(), <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a>, current_epoch)
        }
        // Note that <b>if</b> the <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_stake">stake</a> is in state Withdrawing, it can either be
        // from a pre-active withdrawal, but then
        // (in_current_committee || in_next_committee) is <b>true</b> since it was
        // an early withdrawal, or from a standard two step withdrawal,
        // which is handled below.
    } <b>else</b> {
        // Normal two-step withdrawals.
        <b>assert</b>!(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_is_withdrawing">is_withdrawing</a>(), <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ENotWithdrawing">ENotWithdrawing</a>);
        <b>assert</b>!(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.withdraw_epoch() &lt;= current_epoch, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EWithdrawEpochNotReached">EWithdrawEpochNotReached</a>);
        <b>assert</b>!(<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a> &lt;= current_epoch, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EActivationEpochNotReached">EActivationEpochNotReached</a>);
        pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_calculate_rewards">calculate_rewards</a>(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.value(), <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.withdraw_epoch())
    };
    <b>let</b> principal = <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.into_balance();
    // Withdraw rewards. Due to rounding errors, there's a chance that the
    // rewards amount is higher than the rewards pool, in this case, we
    // withdraw the maximum amount possible.
    <b>let</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_rewards_amount">rewards_amount</a> = <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_rewards_amount">rewards_amount</a>.min(pool.rewards_pool.value());
    <b>let</b> <b>mut</b> to_withdraw = pool.rewards_pool.split(<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_rewards_amount">rewards_amount</a>);
    to_withdraw.join(principal);
    to_withdraw
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_advance_epoch"></a>

## Function `advance_epoch`

Advance epoch for the <code><a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a></code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_advance_epoch">advance_epoch</a>(pool: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, rewards: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;, current_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_advance_epoch">advance_epoch</a>(
    pool: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>,
    <b>mut</b> rewards: Balance&lt;IKA&gt;,
    current_epoch: u64,
) {
    <b>assert</b>!(current_epoch &gt; pool.latest_epoch, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EPoolAlreadyUpdated">EPoolAlreadyUpdated</a>);
    // Sanity check.
    <b>assert</b>!(rewards.value() == 0 || pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a> &gt; 0, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EIncorrectEpochAdvance">EIncorrectEpochAdvance</a>);
    // Split the commission from the rewards.
    <b>let</b> total_rewards = rewards.value();
    <b>let</b> commission = rewards.split(
        total_rewards * (pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_commission_rate">commission_rate</a> <b>as</b> u64) / (<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_N_BASIS_POINTS">N_BASIS_POINTS</a> <b>as</b> u64),
    );
    pool.commission.join(commission);
    // Update the <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_commission_rate">commission_rate</a> <b>for</b> the <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_new">new</a> epoch <b>if</b> there's a pending value.
    // Note that pending commission rates are set 2 epochs ahead, so users are
    // aware of the rate change in advance.
    pool.pending_commission_rate.inner().try_get(&current_epoch).do!(|<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_commission_rate">commission_rate</a>| {
        pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_commission_rate">commission_rate</a> = <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_commission_rate">commission_rate</a> <b>as</b> u16;
        pool.pending_commission_rate.flush(current_epoch);
    });
    // Add rewards to the pool and update the `<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a>`.
    <b>let</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_rewards_amount">rewards_amount</a> = rewards.value();
    pool.rewards_pool.join(rewards);
    pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a> = pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a> + <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_rewards_amount">rewards_amount</a>;
    pool.latest_epoch = current_epoch;
    pool.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>.roatate_next_epoch_info();
    // Perform <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_stake">stake</a> deduction / addition <b>for</b> the current epoch.
    pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_process_pending_stake">process_pending_stake</a>(current_epoch);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_process_pending_stake"></a>

## Function `process_pending_stake`

Process the pending stake and withdrawal requests for the pool. Called in the
<code><a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_advance_epoch">advance_epoch</a></code> function in case the pool is in the committee and receives the
rewards. And may be called in user-facing functions to update the pool state,
if the pool is not in the committee.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_process_pending_stake">process_pending_stake</a>(pool: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, current_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_process_pending_stake">process_pending_stake</a>(
    pool: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>,
    current_epoch: u64,
) {
    // Set the exchange rate <b>for</b> the current epoch.
    <b>let</b> exchange_rate = <a href="../ika_system/exchange_rate.md#(ika_system=0x0)_pool_exchange_rate_new">pool_exchange_rate::new</a>(
        pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a>,
        pool.num_shares,
    );
    pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_exchange_rates">exchange_rates</a>.add(current_epoch, exchange_rate);
    // Process additions.
    pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a> = pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a> + pool.pending_stake.flush(current_epoch);
    // Process withdrawals.
    // each value in pending withdrawals contains the principal which became
    // active in the previous epoch. so unlike other pending values, we need to
    // flush it one by one, recalculating the exchange rate and pool share amount
    // <b>for</b> each early withdrawal epoch.
    <b>let</b> <b>mut</b> pre_active_shares_withdraw = 0;
    <b>let</b> <b>mut</b> pre_active_withdrawals = pool.pre_active_withdrawals.unwrap();
    pre_active_withdrawals.keys().do!(|epoch| <b>if</b> (epoch &lt;= current_epoch) {
        <b>let</b> (_, epoch_value) = pre_active_withdrawals.remove(&epoch);
        // recall that pre_active_withdrawals contains stakes that were
        // active <b>for</b> exactly 1 epoch.
        <b>let</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a> = epoch - 1;
        <b>let</b> shares_for_epoch = pool
            .<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_exchange_rate_at_epoch">exchange_rate_at_epoch</a>(<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a>)
            .convert_to_share_amount(epoch_value);
        pre_active_shares_withdraw = pre_active_shares_withdraw + shares_for_epoch;
    });
    // don't forget to flush the early withdrawals since we worked on a <b>copy</b>
    <b>let</b> _ = pool.pre_active_withdrawals.flush(current_epoch);
    <b>let</b> shares_withdraw = pool.pending_shares_withdraw.flush(current_epoch);
    <b>let</b> pending_withdrawal = exchange_rate.convert_to_ika_amount(
        shares_withdraw + pre_active_shares_withdraw,
    );
    // Sanity check that the amount is not higher than the pool balance.
    <b>assert</b>!(pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a> &gt;= pending_withdrawal, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ECalculationError">ECalculationError</a>);
    pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a> = pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a> - pending_withdrawal;
    // Recalculate the total number of shares according to the exchange rate.
    pool.num_shares = exchange_rate.convert_to_share_amount(pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_set_name"></a>

## Function `set_name`

Sets the name of the storage node.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_name">set_name</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, name: <a href="../std/string.md#std_string_String">std::string::String</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_name">set_name</a>(self: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>, name: String, cap: &ValidatorOperationCap) {
    <b>assert</b>!(cap.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>() == self.id.to_inner(), <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EAuthorizationFailure">EAuthorizationFailure</a>);
    <b>assert</b>!(object::id(cap) == self.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_operation_cap_id">operation_cap_id</a>, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EAuthorizationFailure">EAuthorizationFailure</a>);
    self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_name">set_name</a>(name);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_set_validator_metadata"></a>

## Function `set_validator_metadata`

Sets the node metadata.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_validator_metadata">set_validator_metadata</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, metadata: (ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">validator_metadata::ValidatorMetadata</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_validator_metadata">set_validator_metadata</a>(self: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>, cap: &ValidatorOperationCap, metadata: ValidatorMetadata) {
    <b>assert</b>!(cap.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>() == self.id.to_inner(), <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EAuthorizationFailure">EAuthorizationFailure</a>);
    <b>assert</b>!(object::id(cap) == self.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_operation_cap_id">operation_cap_id</a>, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EAuthorizationFailure">EAuthorizationFailure</a>);
    self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_validator_metadata">set_validator_metadata</a>(metadata);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_set_next_commission"></a>

## Function `set_next_commission`

Sets the next commission rate for the pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_next_commission">set_next_commission</a>(pool: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_commission_rate">commission_rate</a>: u16, current_epoch: u64, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_next_commission">set_next_commission</a>(
    pool: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>,
    <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_commission_rate">commission_rate</a>: u16,
    current_epoch: u64,
    cap: &ValidatorOperationCap,
) {
    <b>assert</b>!(cap.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>() == pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>(), <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EAuthorizationFailure">EAuthorizationFailure</a>);
    <b>assert</b>!(object::id(cap) == pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_operation_cap_id">operation_cap_id</a>, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EAuthorizationFailure">EAuthorizationFailure</a>);
    <b>assert</b>!(<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_commission_rate">commission_rate</a> &lt;= <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_N_BASIS_POINTS">N_BASIS_POINTS</a>, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EIncorrectCommissionRate">EIncorrectCommissionRate</a>);
    pool.pending_commission_rate.insert_or_replace(current_epoch + 2, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_commission_rate">commission_rate</a> <b>as</b> u64);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_set_next_epoch_network_address"></a>

## Function `set_next_epoch_network_address`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_next_epoch_network_address">set_next_epoch_network_address</a>(pool: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, network_address: <a href="../std/string.md#std_string_String">std::string::String</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_next_epoch_network_address">set_next_epoch_network_address</a>(
    pool: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>,
    network_address: String,
    cap: &ValidatorOperationCap,
) {
    <b>assert</b>!(cap.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>() == pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>(), <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EAuthorizationFailure">EAuthorizationFailure</a>);
    <b>assert</b>!(object::id(cap) == pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_operation_cap_id">operation_cap_id</a>, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EAuthorizationFailure">EAuthorizationFailure</a>);
    pool.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_next_epoch_network_address">set_next_epoch_network_address</a>(network_address);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_set_next_epoch_p2p_address"></a>

## Function `set_next_epoch_p2p_address`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_next_epoch_p2p_address">set_next_epoch_p2p_address</a>(pool: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, p2p_address: <a href="../std/string.md#std_string_String">std::string::String</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_next_epoch_p2p_address">set_next_epoch_p2p_address</a>(
    pool: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>,
    p2p_address: String,
    cap: &ValidatorOperationCap,
) {
    <b>assert</b>!(cap.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>() == pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>(), <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EAuthorizationFailure">EAuthorizationFailure</a>);
    <b>assert</b>!(object::id(cap) == pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_operation_cap_id">operation_cap_id</a>, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EAuthorizationFailure">EAuthorizationFailure</a>);
    pool.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_next_epoch_p2p_address">set_next_epoch_p2p_address</a>(p2p_address);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_set_next_epoch_consensus_address"></a>

## Function `set_next_epoch_consensus_address`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_next_epoch_consensus_address">set_next_epoch_consensus_address</a>(pool: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, consensus_address: <a href="../std/string.md#std_string_String">std::string::String</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_next_epoch_consensus_address">set_next_epoch_consensus_address</a>(
    pool: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>,
    consensus_address: String,
    cap: &ValidatorOperationCap,
) {
    <b>assert</b>!(cap.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>() == pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>(), <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EAuthorizationFailure">EAuthorizationFailure</a>);
    <b>assert</b>!(object::id(cap) == pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_operation_cap_id">operation_cap_id</a>, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EAuthorizationFailure">EAuthorizationFailure</a>);
    pool.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_next_epoch_consensus_address">set_next_epoch_consensus_address</a>(consensus_address);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_set_next_epoch_protocol_pubkey_bytes"></a>

## Function `set_next_epoch_protocol_pubkey_bytes`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_next_epoch_protocol_pubkey_bytes">set_next_epoch_protocol_pubkey_bytes</a>(pool: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, protocol_pubkey_bytes: vector&lt;u8&gt;, proof_of_possession: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, ctx: &<a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_next_epoch_protocol_pubkey_bytes">set_next_epoch_protocol_pubkey_bytes</a>(
    pool: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>,
    protocol_pubkey_bytes: vector&lt;u8&gt;,
    proof_of_possession: vector&lt;u8&gt;,
    cap: &ValidatorOperationCap,
    ctx: &TxContext,
) {
    <b>assert</b>!(cap.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>() == pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>(), <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EAuthorizationFailure">EAuthorizationFailure</a>);
    <b>assert</b>!(object::id(cap) == pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_operation_cap_id">operation_cap_id</a>, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EAuthorizationFailure">EAuthorizationFailure</a>);
    pool.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_next_epoch_protocol_pubkey_bytes">set_next_epoch_protocol_pubkey_bytes</a>(protocol_pubkey_bytes, proof_of_possession, ctx);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_set_next_epoch_network_pubkey_bytes"></a>

## Function `set_next_epoch_network_pubkey_bytes`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_next_epoch_network_pubkey_bytes">set_next_epoch_network_pubkey_bytes</a>(pool: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, network_pubkey_bytes: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_next_epoch_network_pubkey_bytes">set_next_epoch_network_pubkey_bytes</a>(
    pool: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>,
    network_pubkey_bytes: vector&lt;u8&gt;,
    cap: &ValidatorOperationCap,
) {
    <b>assert</b>!(cap.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>() == pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>(), <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EAuthorizationFailure">EAuthorizationFailure</a>);
    <b>assert</b>!(object::id(cap) == pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_operation_cap_id">operation_cap_id</a>, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EAuthorizationFailure">EAuthorizationFailure</a>);
    pool.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_next_epoch_network_pubkey_bytes">set_next_epoch_network_pubkey_bytes</a>(network_pubkey_bytes);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_set_next_epoch_consensus_pubkey_bytes"></a>

## Function `set_next_epoch_consensus_pubkey_bytes`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_next_epoch_consensus_pubkey_bytes">set_next_epoch_consensus_pubkey_bytes</a>(pool: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, consensus_pubkey_bytes: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_next_epoch_consensus_pubkey_bytes">set_next_epoch_consensus_pubkey_bytes</a>(
    pool: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>,
    consensus_pubkey_bytes: vector&lt;u8&gt;,
    cap: &ValidatorOperationCap,
) {
    <b>assert</b>!(cap.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>() == pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>(), <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EAuthorizationFailure">EAuthorizationFailure</a>);
    <b>assert</b>!(object::id(cap) == pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_operation_cap_id">operation_cap_id</a>, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EAuthorizationFailure">EAuthorizationFailure</a>);
    pool.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_next_epoch_consensus_pubkey_bytes">set_next_epoch_consensus_pubkey_bytes</a>(consensus_pubkey_bytes);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_set_next_epoch_class_groups_pubkey_and_proof_bytes"></a>

## Function `set_next_epoch_class_groups_pubkey_and_proof_bytes`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_next_epoch_class_groups_pubkey_and_proof_bytes">set_next_epoch_class_groups_pubkey_and_proof_bytes</a>(pool: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, class_groups_pubkey_and_proof_bytes: (ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_next_epoch_class_groups_pubkey_and_proof_bytes">set_next_epoch_class_groups_pubkey_and_proof_bytes</a>(
    pool: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>,
    class_groups_pubkey_and_proof_bytes: ClassGroupsPublicKeyAndProof,
    cap: &ValidatorOperationCap,
) {
    <b>assert</b>!(cap.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>() == pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>(), <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EAuthorizationFailure">EAuthorizationFailure</a>);
    <b>assert</b>!(object::id(cap) == pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_operation_cap_id">operation_cap_id</a>, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EAuthorizationFailure">EAuthorizationFailure</a>);
    pool.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_set_next_epoch_class_groups_pubkey_and_proof_bytes">set_next_epoch_class_groups_pubkey_and_proof_bytes</a>(class_groups_pubkey_and_proof_bytes);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_destroy_empty"></a>

## Function `destroy_empty`

Destroy the pool if it is empty.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_destroy_empty">destroy_empty</a>(pool: (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_destroy_empty">destroy_empty</a>(pool: <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>) {
    <b>assert</b>!(pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_is_empty">is_empty</a>(), <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EPoolNotEmpty">EPoolNotEmpty</a>);
    <b>let</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a> {
        id,
        <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>,
        <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_exchange_rates">exchange_rates</a>,
        rewards_pool,
        commission,
        extra_fields,
        ..,
    } = pool;
    id.delete();
    <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>.destroy();
    <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_exchange_rates">exchange_rates</a>.drop();
    commission.destroy_zero();
    rewards_pool.destroy_zero();
    extra_fields.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_destroy_empty">destroy_empty</a>();
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_exchange_rate_at_epoch"></a>

## Function `exchange_rate_at_epoch`

Returns the exchange rate for the given current or future epoch. If there
isn't a value for the specified epoch, it will look for the most recent
value down to the pool activation epoch.
Note that exchange rates are only set for epochs in which the node is in
the committee, and otherwise the rate remains static.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_exchange_rate_at_epoch">exchange_rate_at_epoch</a>(pool: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, epoch: u64): (ika_system=0x0)::<a href="../ika_system/exchange_rate.md#(ika_system=0x0)_pool_exchange_rate_PoolExchangeRate">pool_exchange_rate::PoolExchangeRate</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_exchange_rate_at_epoch">exchange_rate_at_epoch</a>(pool: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>, epoch: u64): PoolExchangeRate {
    // If the pool is preactive then the exchange rate is always 1:1.
    <b>if</b> (pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_is_preactive_at_epoch">is_preactive_at_epoch</a>(epoch)) {
        <b>return</b> <a href="../ika_system/exchange_rate.md#(ika_system=0x0)_pool_exchange_rate_flat">pool_exchange_rate::flat</a>()
    };
    <b>let</b> clamped_epoch = pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_withdrawing_epoch">withdrawing_epoch</a>().get_with_default(epoch);
    <b>let</b> <b>mut</b> epoch = clamped_epoch.min(epoch);
    <b>let</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a> = *pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a>.borrow();
    <b>while</b> (epoch &gt;= <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a>) {
        <b>if</b> (pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_exchange_rates">exchange_rates</a>.contains(epoch)) {
            <b>return</b> pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_exchange_rates">exchange_rates</a>[epoch]
        };
        epoch = epoch - 1;
    };
    <a href="../ika_system/exchange_rate.md#(ika_system=0x0)_pool_exchange_rate_flat">pool_exchange_rate::flat</a>()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_ika_balance_at_epoch"></a>

## Function `ika_balance_at_epoch`

Returns the expected active stake for current or future epoch <code>E</code> for the pool.
It processes the pending stake and withdrawal requests from the current epoch
to <code>E</code>.

Should be the main function to calculate the active stake for the pool at
the given epoch, due to the complexity of the pending stake and withdrawal
requests, and lack of immediate updates.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance_at_epoch">ika_balance_at_epoch</a>(pool: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, epoch: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance_at_epoch">ika_balance_at_epoch</a>(pool: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>, epoch: u64): u64 {
    <b>let</b> exchange_rate = <a href="../ika_system/exchange_rate.md#(ika_system=0x0)_pool_exchange_rate_new">pool_exchange_rate::new</a>(pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a>, pool.num_shares);
    <b>let</b> <b>mut</b> pre_active_shares_withdraw = 0;
    <b>let</b> pre_active_withdrawals = pool.pre_active_withdrawals.unwrap();
    pre_active_withdrawals.keys().do_ref!(|old_epoch| <b>if</b> (*old_epoch &lt;= epoch) {
        <b>let</b> ika_value = pre_active_withdrawals.get(old_epoch);
        // recall that pre_active_withdrawals contains stakes that were
        // active <b>for</b> exactly 1 epoch. since the node might have been
        // inactive, this list may contain more than one value
        // (although <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_exchange_rate_at_epoch">exchange_rate_at_epoch</a> will <b>return</b> the same value).
        <b>let</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a> = *old_epoch - 1;
        <b>let</b> shares_for_epoch = pool
            .<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_exchange_rate_at_epoch">exchange_rate_at_epoch</a>(<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a>)
            .convert_to_share_amount(*ika_value);
        pre_active_shares_withdraw = pre_active_shares_withdraw + shares_for_epoch;
    });
    <b>let</b> shares_withdraw = pool.pending_shares_withdraw.value_at(epoch);
    <b>let</b> pending_withdrawal = exchange_rate.convert_to_ika_amount(
        shares_withdraw + pre_active_shares_withdraw,
    );
    pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a> + pool.pending_stake.value_at(epoch) - pending_withdrawal
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_new_validator_operation_cap"></a>

## Function `new_validator_operation_cap`

Create a new <code>ValidatorOperationCap</code>, and registers it,
thus revoking the previous cap's permission.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_new_validator_operation_cap">new_validator_operation_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_new_validator_operation_cap">new_validator_operation_cap</a>(
    self: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>,
    cap: &ValidatorCap,
    ctx: &<b>mut</b> TxContext,
): ValidatorOperationCap {
    <b>let</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a> = cap.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>();
    <b>assert</b>!(<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a> == self.id.to_inner(), <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EAuthorizationFailure">EAuthorizationFailure</a>);
    <b>let</b> operation_cap = <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_new_validator_operation_cap">validator_cap::new_validator_operation_cap</a>(<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>, ctx);
    self.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_operation_cap_id">operation_cap_id</a> = object::id(&operation_cap);
    operation_cap
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_new_validator_commission_cap"></a>

## Function `new_validator_commission_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_new_validator_commission_cap">new_validator_commission_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCommissionCap">validator_cap::ValidatorCommissionCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_new_validator_commission_cap">new_validator_commission_cap</a>(
    self: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>,
    cap: &ValidatorCap,
    ctx: &<b>mut</b> TxContext,
): ValidatorCommissionCap {
    <b>let</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a> = cap.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>();
    <b>assert</b>!(<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a> == self.id.to_inner(), <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EAuthorizationFailure">EAuthorizationFailure</a>);
    <b>let</b> commission_cap = <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_new_validator_commission_cap">validator_cap::new_validator_commission_cap</a>(<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>, ctx);
    self.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_commission_cap_id">commission_cap_id</a> = object::id(&commission_cap);
    commission_cap
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_collect_commission"></a>

## Function `collect_commission`

Withdraws the commission from the pool. Amount is optional, if not provided,
the full commission is withdrawn.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_collect_commission">collect_commission</a>(pool: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCommissionCap">validator_cap::ValidatorCommissionCap</a>, amount: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_collect_commission">collect_commission</a>(
    pool: &<b>mut</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>,
    cap: &ValidatorCommissionCap,
    amount: Option&lt;u64&gt;,
): Balance&lt;IKA&gt; {
    <b>assert</b>!(cap.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>() == pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>(), <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EAuthorizationFailure">EAuthorizationFailure</a>);
    <b>assert</b>!(object::id(cap) == pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_commission_cap_id">commission_cap_id</a>, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_EAuthorizationFailure">EAuthorizationFailure</a>);
    <b>if</b> (amount.is_some()) {
        pool.commission.split(*amount.borrow())
    } <b>else</b> {
        pool.commission.withdraw_all()
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_validator_id"></a>

## Function `validator_id`

Returns the validator id for the pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>(pool: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_id">validator_id</a>(pool: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>): ID { pool.id.to_inner() }
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_validator_cap_id"></a>

## Function `validator_cap_id`

Returns the validator cap for the pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_cap_id">validator_cap_id</a>(pool: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_cap_id">validator_cap_id</a>(pool: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>): ID { pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_validator_cap_id">validator_cap_id</a> }
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_operation_cap_id"></a>

## Function `operation_cap_id`

Returns the operation cap id for the pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_operation_cap_id">operation_cap_id</a>(pool: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_operation_cap_id">operation_cap_id</a>(pool: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>): ID { pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_operation_cap_id">operation_cap_id</a> }
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_commission_cap_id"></a>

## Function `commission_cap_id`

Returns the commission cap id for the pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_commission_cap_id">commission_cap_id</a>(pool: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_commission_cap_id">commission_cap_id</a>(pool: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>): ID { pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_commission_cap_id">commission_cap_id</a> }
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_commission_rate"></a>

## Function `commission_rate`

Returns the commission rate for the pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_commission_rate">commission_rate</a>(pool: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>): u16
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_commission_rate">commission_rate</a>(pool: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>): u16 { pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_commission_rate">commission_rate</a> }
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_commission_amount"></a>

## Function `commission_amount`

Returns the commission amount for the pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_commission_amount">commission_amount</a>(pool: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_commission_amount">commission_amount</a>(pool: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>): u64 { pool.commission.value() }
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_rewards_amount"></a>

## Function `rewards_amount`

Returns the rewards amount for the pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_rewards_amount">rewards_amount</a>(pool: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_rewards_amount">rewards_amount</a>(pool: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>): u64 { pool.rewards_pool.value() }
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_ika_balance"></a>

## Function `ika_balance`

Returns the rewards for the pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a>(pool: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a>(pool: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>): u64 { pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a> }
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_activation_epoch"></a>

## Function `activation_epoch`

Returns the activation epoch for the pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a>(pool: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a>(pool: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>): Option&lt;u64&gt; { pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a> }
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_validator_info"></a>

## Function `validator_info`

Returns the validator info for the pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>(pool: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>): &(ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>(pool: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>): &ValidatorInfo { &pool.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a> }
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_is_preactive"></a>

## Function `is_preactive`

Returns <code><b>true</b></code> if the pool is preactive.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_is_preactive">is_preactive</a>(pool: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_is_preactive">is_preactive</a>(pool: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>): bool { pool.state == PoolState::PreActive }
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_is_active"></a>

## Function `is_active`

Returns <code><b>true</b></code> if the pool is active.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_is_active">is_active</a>(pool: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_is_active">is_active</a>(pool: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>): bool { pool.state == PoolState::Active }
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_is_withdrawing"></a>

## Function `is_withdrawing`

Returns <code><b>true</b></code> if the pool is withdrawing.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_is_withdrawing">is_withdrawing</a>(pool: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_is_withdrawing">is_withdrawing</a>(pool: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>): bool {
    match (pool.state) {
        PoolState::Withdrawing(_) =&gt; <b>true</b>,
        _ =&gt; <b>false</b>,
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_withdrawing_epoch"></a>

## Function `withdrawing_epoch`

Returns the epoch in which the pool is withdrawing.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_withdrawing_epoch">withdrawing_epoch</a>(pool: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_withdrawing_epoch">withdrawing_epoch</a>(pool: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>): Option&lt;u64&gt; {
    match (pool.state) {
        PoolState::Withdrawing(epoch) =&gt; option::some(epoch),
        _ =&gt; option::none(),
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_is_preactive_at_epoch"></a>

## Function `is_preactive_at_epoch`

Returns true if the provided staking pool is preactive at the provided epoch.


<pre><code><b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_is_preactive_at_epoch">is_preactive_at_epoch</a>(pool: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, epoch: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_is_preactive_at_epoch">is_preactive_at_epoch</a>(pool: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>, epoch: u64): bool {
    // Either the pool is currently preactive or the pool's starting epoch is later than the provided epoch.
    pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_is_preactive">is_preactive</a>() || (*pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a>.borrow() &gt; epoch)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_exchange_rates"></a>

## Function `exchange_rates`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_exchange_rates">exchange_rates</a>(pool: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>): &<a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, (ika_system=0x0)::<a href="../ika_system/exchange_rate.md#(ika_system=0x0)_pool_exchange_rate_PoolExchangeRate">pool_exchange_rate::PoolExchangeRate</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_exchange_rates">exchange_rates</a>(pool: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>): &Table&lt;u64, PoolExchangeRate&gt; {
    &pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_exchange_rates">exchange_rates</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_is_empty"></a>

## Function `is_empty`

Returns <code><b>true</b></code> if the pool is empty.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_is_empty">is_empty</a>(pool: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_is_empty">is_empty</a>(pool: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>): bool {
    <b>let</b> pending_stake = pool.pending_stake.unwrap();
    <b>let</b> non_empty = pending_stake.keys().count!(|epoch| pending_stake[epoch] != 0);
    pool.rewards_pool.value() == 0 &&
    pool.num_shares == 0 &&
    pool.commission.value() == 0 &&
    pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_ika_balance">ika_balance</a> == 0 &&
    non_empty == 0
}
</code></pre>



</details>

<a name="(ika_system=0x0)_staking_pool_calculate_rewards"></a>

## Function `calculate_rewards`

Calculate the rewards for an amount with value <code>staked_principal</code>, staked in the pool between
<code><a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a></code> and <code>withdraw_epoch</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_calculate_rewards">calculate_rewards</a>(pool: &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>, staked_principal: u64, <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a>: u64, withdraw_epoch: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_calculate_rewards">calculate_rewards</a>(
    pool: &<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">StakingPool</a>,
    staked_principal: u64,
    <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a>: u64,
    withdraw_epoch: u64,
): u64 {
    <b>let</b> shares = pool
        .<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_exchange_rate_at_epoch">exchange_rate_at_epoch</a>(<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_activation_epoch">activation_epoch</a>)
        .convert_to_share_amount(staked_principal);
    <b>let</b> ika_amount = pool.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_exchange_rate_at_epoch">exchange_rate_at_epoch</a>(withdraw_epoch).convert_to_ika_amount(shares);
    <b>if</b> (ika_amount &gt;= staked_principal) {
        ika_amount - staked_principal
    } <b>else</b> 0
}
</code></pre>



</details>
