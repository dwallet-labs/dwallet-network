---
title: Module `(ika_system=0x0)::validator`
---



-  [Struct `Validator`](#(ika_system=0x0)_validator_Validator)
-  [Enum `ValidatorState`](#(ika_system=0x0)_validator_ValidatorState)
-  [Constants](#@Constants_0)
-  [Function `new`](#(ika_system=0x0)_validator_new)
-  [Function `activate`](#(ika_system=0x0)_validator_activate)
-  [Function `set_withdrawing`](#(ika_system=0x0)_validator_set_withdrawing)
-  [Function `deactivate`](#(ika_system=0x0)_validator_deactivate)
-  [Function `request_add_stake`](#(ika_system=0x0)_validator_request_add_stake)
-  [Function `request_withdraw_stake`](#(ika_system=0x0)_validator_request_withdraw_stake)
-  [Function `withdraw_stake`](#(ika_system=0x0)_validator_withdraw_stake)
-  [Function `advance_epoch`](#(ika_system=0x0)_validator_advance_epoch)
-  [Function `process_pending_stake`](#(ika_system=0x0)_validator_process_pending_stake)
-  [Function `verify_validator_cap`](#(ika_system=0x0)_validator_verify_validator_cap)
-  [Function `verify_operation_cap`](#(ika_system=0x0)_validator_verify_operation_cap)
-  [Function `verify_commission_cap`](#(ika_system=0x0)_validator_verify_commission_cap)
-  [Function `set_name`](#(ika_system=0x0)_validator_set_name)
-  [Function `set_validator_metadata`](#(ika_system=0x0)_validator_set_validator_metadata)
-  [Function `set_next_commission`](#(ika_system=0x0)_validator_set_next_commission)
-  [Function `set_next_epoch_network_address`](#(ika_system=0x0)_validator_set_next_epoch_network_address)
-  [Function `set_next_epoch_p2p_address`](#(ika_system=0x0)_validator_set_next_epoch_p2p_address)
-  [Function `set_next_epoch_consensus_address`](#(ika_system=0x0)_validator_set_next_epoch_consensus_address)
-  [Function `set_next_epoch_protocol_pubkey_bytes`](#(ika_system=0x0)_validator_set_next_epoch_protocol_pubkey_bytes)
-  [Function `set_next_epoch_network_pubkey_bytes`](#(ika_system=0x0)_validator_set_next_epoch_network_pubkey_bytes)
-  [Function `set_next_epoch_consensus_pubkey_bytes`](#(ika_system=0x0)_validator_set_next_epoch_consensus_pubkey_bytes)
-  [Function `set_next_epoch_class_groups_pubkey_and_proof_bytes`](#(ika_system=0x0)_validator_set_next_epoch_class_groups_pubkey_and_proof_bytes)
-  [Function `destroy_empty`](#(ika_system=0x0)_validator_destroy_empty)
-  [Function `exchange_rate_at_epoch`](#(ika_system=0x0)_validator_exchange_rate_at_epoch)
-  [Function `ika_balance_at_epoch`](#(ika_system=0x0)_validator_ika_balance_at_epoch)
-  [Function `rotate_operation_cap`](#(ika_system=0x0)_validator_rotate_operation_cap)
-  [Function `rotate_commission_cap`](#(ika_system=0x0)_validator_rotate_commission_cap)
-  [Function `collect_commission`](#(ika_system=0x0)_validator_collect_commission)
-  [Function `validator_id`](#(ika_system=0x0)_validator_validator_id)
-  [Function `validator_cap_id`](#(ika_system=0x0)_validator_validator_cap_id)
-  [Function `operation_cap_id`](#(ika_system=0x0)_validator_operation_cap_id)
-  [Function `commission_cap_id`](#(ika_system=0x0)_validator_commission_cap_id)
-  [Function `commission_rate`](#(ika_system=0x0)_validator_commission_rate)
-  [Function `commission_amount`](#(ika_system=0x0)_validator_commission_amount)
-  [Function `rewards_amount`](#(ika_system=0x0)_validator_rewards_amount)
-  [Function `ika_balance`](#(ika_system=0x0)_validator_ika_balance)
-  [Function `activation_epoch`](#(ika_system=0x0)_validator_activation_epoch)
-  [Function `validator_info`](#(ika_system=0x0)_validator_validator_info)
-  [Function `is_preactive`](#(ika_system=0x0)_validator_is_preactive)
-  [Function `is_active`](#(ika_system=0x0)_validator_is_active)
-  [Function `is_withdrawing`](#(ika_system=0x0)_validator_is_withdrawing)
-  [Function `withdrawing_epoch`](#(ika_system=0x0)_validator_withdrawing_epoch)
-  [Function `exchange_rates`](#(ika_system=0x0)_validator_exchange_rates)
-  [Function `is_empty`](#(ika_system=0x0)_validator_is_empty)
-  [Function `calculate_rewards`](#(ika_system=0x0)_validator_calculate_rewards)
-  [Function `is_preactive_at_epoch`](#(ika_system=0x0)_validator_is_preactive_at_epoch)


<pre><code><b>use</b> (ika=0x0)::ika;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof">class_groups_public_key_and_proof</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/extended_field.md#(ika_system=0x0)_extended_field">extended_field</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/multiaddr.md#(ika_system=0x0)_multiaddr">multiaddr</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values">pending_values</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate">token_exchange_rate</a>;
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
<b>use</b> <a href="../sui/party.md#sui_party">sui::party</a>;
<b>use</b> <a href="../sui/table.md#sui_table">sui::table</a>;
<b>use</b> <a href="../sui/table_vec.md#sui_table_vec">sui::table_vec</a>;
<b>use</b> <a href="../sui/transfer.md#sui_transfer">sui::transfer</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
<b>use</b> <a href="../sui/types.md#sui_types">sui::types</a>;
<b>use</b> <a href="../sui/url.md#sui_url">sui::url</a>;
<b>use</b> <a href="../sui/vec_map.md#sui_vec_map">sui::vec_map</a>;
<b>use</b> <a href="../sui/vec_set.md#sui_vec_set">sui::vec_set</a>;
</code></pre>



<a name="(ika_system=0x0)_validator_Validator"></a>

## Struct `Validator`

Represents a single validator. Even though it is never
transferred or shared, the <code>key</code> ability is added for discoverability
in the <code>ObjectTable</code>.

High level overview of the validator:
The validator maintains a balance of IKA 'ika_balance' that is increased
when stakes/rewards are added to the validator, and is decreased when
stakes are withdrawn.
To track the users' portion of the validator, we associate shares to the
staked IKA. Initially, the share price is 1 IKA per share.
When a new stake is added to the validator, the total number of shares
increases by an amount that corresponds to the share price at that
time. E.g., if the share price is 2 IKA per share, and 10 IKA are
added to the validator, the total number of shares is increased by 5
shares. The total number of shares is stored in 'num_shares'.

As stakes are added/withdrawn only in the granularity of epochs, we
maintain a share price per epoch in 'exchange_rates'.
StakedIka objects only need to store the epoch when they are created,
and the amount of IKA they locked. Whenever a settlement is performed
for a StakedIka, we calculate the number of shares that correspond to
the amount of IKA that was locked using the exchange rate at the time
of the lock, and then convert it to the amount of IKA that corresponds
to the current share price.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a> <b>has</b> key, store
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
 The validator info for the validator.
</dd>
<dt>
<code>state: (ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_ValidatorState">validator::ValidatorState</a></code>
</dt>
<dd>
 The current state of the validator.
</dd>
<dt>
<code><a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
 The epoch when the validator is / will be activated.
 Serves information purposes only, the checks are performed in the <code>state</code>
 property.
</dd>
<dt>
<code>latest_epoch: u64</code>
</dt>
<dd>
 Epoch when the validator was last updated.
</dd>
<dt>
<code><a href="../ika_system/validator.md#(ika_system=0x0)_validator_ika_balance">ika_balance</a>: u64</code>
</dt>
<dd>
 Currently staked IKA in the validator + rewards validator.
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
 <code><a href="../ika_system/validator.md#(ika_system=0x0)_validator_process_pending_stake">process_pending_stake</a></code>.
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
 The pending commission rate for the validator. Commission rate is applied in
 E+2, so we store the value for the matching epoch and apply it in the
 <code><a href="../ika_system/validator.md#(ika_system=0x0)_validator_advance_epoch">advance_epoch</a></code> function.
</dd>
<dt>
<code><a href="../ika_system/validator.md#(ika_system=0x0)_validator_commission_rate">commission_rate</a>: u16</code>
</dt>
<dd>
 The commission rate for the validator, in basis points.
</dd>
<dt>
<code><a href="../ika_system/validator.md#(ika_system=0x0)_validator_exchange_rates">exchange_rates</a>: <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, (ika_system=0x0)::<a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_TokenExchangeRate">token_exchange_rate::TokenExchangeRate</a>&gt;</code>
</dt>
<dd>
 Historical exchange rates for the validator. The key is the epoch when the
 exchange rate was set, and the value is the exchange rate (the ratio of
 the amount of IKA tokens for the validator shares).
</dd>
<dt>
<code>pending_stake: (ika_system=0x0)::<a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">pending_values::PendingValues</a></code>
</dt>
<dd>
 The amount of stake that will be added to the <code><a href="../ika_system/validator.md#(ika_system=0x0)_validator_ika_balance">ika_balance</a></code>. Can hold
 up to two keys: E+1 and E+2, due to the differences in the activation
 epoch.
 ```
 E+1 -> Balance
 E+2 -> Balance
 ```
 Single key is cleared in the <code><a href="../ika_system/validator.md#(ika_system=0x0)_validator_advance_epoch">advance_epoch</a></code> function, leaving only the
 next epoch's stake.
</dd>
<dt>
<code>rewards_pool: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;</code>
</dt>
<dd>
 The rewards that the validator has received from being in the committee.
</dd>
<dt>
<code>commission: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;</code>
</dt>
<dd>
 The commission that the validator has received from the rewards.
</dd>
<dt>
<code><a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_cap_id">validator_cap_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The ID of this validator's <code>ValidatorCap</code>
</dd>
<dt>
<code><a href="../ika_system/validator.md#(ika_system=0x0)_validator_operation_cap_id">operation_cap_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The ID of this validator's current valid <code>ValidatorOperationCap</code>
</dd>
<dt>
<code><a href="../ika_system/validator.md#(ika_system=0x0)_validator_commission_cap_id">commission_cap_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
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

<a name="(ika_system=0x0)_validator_ValidatorState"></a>

## Enum `ValidatorState`

Represents the state of the validator.


<pre><code><b>public</b> <b>enum</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_ValidatorState">ValidatorState</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Variants</summary>


<dl>
<dt>
Variant <code>PreActive</code>
</dt>
<dd>
 The validator is not active yet but can accept stakes.
</dd>
<dt>
Variant <code>Active</code>
</dt>
<dd>
 The validator is active and can accept stakes.
</dd>
<dt>
Variant <code>Withdrawing</code>
</dt>
<dd>
 The validator awaits the stake to be withdrawn. The value inside the
 variant is the epoch in which the validator will be withdrawn.
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


<a name="(ika_system=0x0)_validator_BASIS_POINT_DENOMINATOR"></a>

The number of basis points in 100%.


<pre><code><b>const</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a>: u16 = 10000;
</code></pre>



<a name="(ika_system=0x0)_validator_EValidatorAlreadyUpdated"></a>

The epoch of the validator has already been advanced.


<pre><code><b>const</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EValidatorAlreadyUpdated">EValidatorAlreadyUpdated</a>: u64 = 0;
</code></pre>



<a name="(ika_system=0x0)_validator_ECalculationError"></a>

Error in a calculation. Indicates that a sanity check failed.


<pre><code><b>const</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_ECalculationError">ECalculationError</a>: u64 = 1;
</code></pre>



<a name="(ika_system=0x0)_validator_EIncorrectEpochAdvance"></a>

The state of the validator and the parameters to advance the epoch are not consistent.


<pre><code><b>const</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EIncorrectEpochAdvance">EIncorrectEpochAdvance</a>: u64 = 2;
</code></pre>



<a name="(ika_system=0x0)_validator_EValidatorNotEmpty"></a>

Trying to destroy a non-empty validator.


<pre><code><b>const</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EValidatorNotEmpty">EValidatorNotEmpty</a>: u64 = 3;
</code></pre>



<a name="(ika_system=0x0)_validator_EValidatorIsNotPreActive"></a>

Validator is not in <code>PreActive</code> state.


<pre><code><b>const</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EValidatorIsNotPreActive">EValidatorIsNotPreActive</a>: u64 = 4;
</code></pre>



<a name="(ika_system=0x0)_validator_EValidatorAlreadyWithdrawing"></a>

Trying to set the validator to withdrawing state when it is already withdrawing.


<pre><code><b>const</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EValidatorAlreadyWithdrawing">EValidatorAlreadyWithdrawing</a>: u64 = 5;
</code></pre>



<a name="(ika_system=0x0)_validator_EValidatorIsNotActive"></a>

Validator is not in <code>Active</code> state.


<pre><code><b>const</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EValidatorIsNotActive">EValidatorIsNotActive</a>: u64 = 6;
</code></pre>



<a name="(ika_system=0x0)_validator_EZeroStake"></a>

Trying to stake zero amount.


<pre><code><b>const</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EZeroStake">EZeroStake</a>: u64 = 7;
</code></pre>



<a name="(ika_system=0x0)_validator_ENotStaked"></a>

StakedIka is already in <code>Withdrawing</code> state.


<pre><code><b>const</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_ENotStaked">ENotStaked</a>: u64 = 8;
</code></pre>



<a name="(ika_system=0x0)_validator_EIncorrectValidatorId"></a>

Trying to withdraw stake from the incorrect validator.


<pre><code><b>const</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EIncorrectValidatorId">EIncorrectValidatorId</a>: u64 = 9;
</code></pre>



<a name="(ika_system=0x0)_validator_ENotWithdrawing"></a>

Trying to withdraw active stake.


<pre><code><b>const</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_ENotWithdrawing">ENotWithdrawing</a>: u64 = 10;
</code></pre>



<a name="(ika_system=0x0)_validator_EWithdrawEpochNotReached"></a>

Attempt to withdraw before <code>withdraw_epoch</code>.


<pre><code><b>const</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EWithdrawEpochNotReached">EWithdrawEpochNotReached</a>: u64 = 11;
</code></pre>



<a name="(ika_system=0x0)_validator_EActivationEpochNotReached"></a>

Attempt to withdraw before <code><a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a></code>.


<pre><code><b>const</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EActivationEpochNotReached">EActivationEpochNotReached</a>: u64 = 12;
</code></pre>



<a name="(ika_system=0x0)_validator_EWithdrawDirectly"></a>

Requesting withdrawal for the stake that can be withdrawn directly.


<pre><code><b>const</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EWithdrawDirectly">EWithdrawDirectly</a>: u64 = 13;
</code></pre>



<a name="(ika_system=0x0)_validator_EIncorrectCommissionRate"></a>

Incorrect commission rate.


<pre><code><b>const</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EIncorrectCommissionRate">EIncorrectCommissionRate</a>: u64 = 14;
</code></pre>



<a name="(ika_system=0x0)_validator_EAuthorizationFailure"></a>

Trying to collect commission or change receiver without authorization.


<pre><code><b>const</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EAuthorizationFailure">EAuthorizationFailure</a>: u64 = 15;
</code></pre>



<a name="(ika_system=0x0)_validator_EZeroShares"></a>

The number of shares for the staked IKA are zero.


<pre><code><b>const</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EZeroShares">EZeroShares</a>: u64 = 16;
</code></pre>



<a name="(ika_system=0x0)_validator_new"></a>

## Function `new`

Create a new <code><a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a></code> object.
If committee is selected, the validator will be activated in the next epoch.
Otherwise, it will be activated in the current epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_new">new</a>(current_epoch: u64, name: <a href="../std/string.md#std_string_String">std::string::String</a>, protocol_pubkey_bytes: vector&lt;u8&gt;, network_pubkey_bytes: vector&lt;u8&gt;, consensus_pubkey_bytes: vector&lt;u8&gt;, class_groups_pubkey_and_proof_bytes: (ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof</a>, proof_of_possession_bytes: vector&lt;u8&gt;, network_address: <a href="../std/string.md#std_string_String">std::string::String</a>, p2p_address: <a href="../std/string.md#std_string_String">std::string::String</a>, consensus_address: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../ika_system/validator.md#(ika_system=0x0)_validator_commission_rate">commission_rate</a>: u16, metadata: (ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">validator_metadata::ValidatorMetadata</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): ((ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>, (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCommissionCap">validator_cap::ValidatorCommissionCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_new">new</a>(
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
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator_commission_rate">commission_rate</a>: u16,
    metadata: ValidatorMetadata,
    ctx: &<b>mut</b> TxContext,
): (<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>, ValidatorCap, ValidatorOperationCap, ValidatorCommissionCap) {
    <b>let</b> id = object::new(ctx);
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_id">validator_id</a> = id.to_inner();
    <b>let</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap">validator_cap</a> = <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_new_validator_cap">validator_cap::new_validator_cap</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_id">validator_id</a>, ctx);
    <b>let</b> validator_operation_cap = <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_new_validator_operation_cap">validator_cap::new_validator_operation_cap</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_id">validator_id</a>, ctx);
    <b>let</b> validator_commission_cap = <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_new_validator_commission_cap">validator_cap::new_validator_commission_cap</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_id">validator_id</a>, ctx);
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = <a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a> {
        id,
        <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>: <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_new">validator_info::new</a>(
            name,
            <a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_id">validator_id</a>,
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
        state: ValidatorState::PreActive,
        <a href="../ika_system/validator.md#(ika_system=0x0)_validator_exchange_rates">exchange_rates</a>: table::new(ctx),
        <a href="../ika_system/validator.md#(ika_system=0x0)_validator_commission_rate">commission_rate</a>,
        <a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a>: option::none(),
        latest_epoch: current_epoch,
        pending_stake: <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_empty">pending_values::empty</a>(),
        pending_shares_withdraw: <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_empty">pending_values::empty</a>(),
        pre_active_withdrawals: <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_empty">pending_values::empty</a>(),
        pending_commission_rate: <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_empty">pending_values::empty</a>(),
        <a href="../ika_system/validator.md#(ika_system=0x0)_validator_ika_balance">ika_balance</a>: 0,
        num_shares: 0,
        rewards_pool: balance::zero(),
        commission: balance::zero(),
        <a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_cap_id">validator_cap_id</a>: object::id(&<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap">validator_cap</a>),
        <a href="../ika_system/validator.md#(ika_system=0x0)_validator_operation_cap_id">operation_cap_id</a>: object::id(&validator_operation_cap),
        <a href="../ika_system/validator.md#(ika_system=0x0)_validator_commission_cap_id">commission_cap_id</a>: object::id(&validator_commission_cap),
        extra_fields: bag::new(ctx),
    };
    (
        <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>,
        <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap">validator_cap</a>,
        validator_operation_cap,
        validator_commission_cap
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_activate"></a>

## Function `activate`

Activate the validator for participation in the network.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_activate">activate</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>, <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap">validator_cap</a>: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, current_epoch: u64, committee_selected: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_activate">activate</a>(
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>,
    <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap">validator_cap</a>: &ValidatorCap,
    current_epoch: u64,
    committee_selected: bool,
) {
    <b>assert</b>!(<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap">validator_cap</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_id">validator_id</a>() == <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_id">validator_id</a>(), <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EAuthorizationFailure">EAuthorizationFailure</a>);
    <b>assert</b>!(object::id(<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap">validator_cap</a>) == <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_cap_id">validator_cap_id</a>, <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EAuthorizationFailure">EAuthorizationFailure</a>);
    <b>assert</b>!(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.state == ValidatorState::PreActive, <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EValidatorIsNotPreActive">EValidatorIsNotPreActive</a>);
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a> = <b>if</b> (committee_selected) {
        current_epoch + 2
    } <b>else</b> {
        current_epoch + 1
    };
    // // Add the initial exchange rate to the table.
    // <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_exchange_rates">exchange_rates</a>.add(<a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a>, <a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_flat">token_exchange_rate::flat</a>());
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.state = ValidatorState::Active;
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a>.fill(<a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_withdrawing"></a>

## Function `set_withdrawing`

Set the state of the validator to <code>Withdrawing</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_withdrawing">set_withdrawing</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>, <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap">validator_cap</a>: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, current_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_withdrawing">set_withdrawing</a>(
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>,
    <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap">validator_cap</a>: &ValidatorCap,
    current_epoch: u64,
) {
    <b>assert</b>!(<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap">validator_cap</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_id">validator_id</a>() == <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_id">validator_id</a>(), <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EAuthorizationFailure">EAuthorizationFailure</a>);
    <b>assert</b>!(object::id(<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap">validator_cap</a>) == <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_cap_id">validator_cap_id</a>, <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EAuthorizationFailure">EAuthorizationFailure</a>);
    <b>assert</b>!(!<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_is_withdrawing">is_withdrawing</a>(), <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EValidatorAlreadyWithdrawing">EValidatorAlreadyWithdrawing</a>);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.state = ValidatorState::Withdrawing(current_epoch);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_deactivate"></a>

## Function `deactivate`

Deactivate the validator from network participation by setting the state to <code>Withdrawing</code>.
This is a function to deactivate the validator from the network participation without validator cap.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_deactivate">deactivate</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>, deactivation_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_deactivate">deactivate</a>(
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>,
    deactivation_epoch: u64,
) {
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.state = ValidatorState::Withdrawing(deactivation_epoch);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_request_add_stake"></a>

## Function `request_add_stake`

Stake the given amount of IKA in the validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_request_add_stake">request_add_stake</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>, to_stake: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;, current_epoch: u64, committee_selected: bool, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_request_add_stake">request_add_stake</a>(
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>,
    to_stake: Balance&lt;IKA&gt;,
    current_epoch: u64,
    committee_selected: bool,
    ctx: &<b>mut</b> TxContext,
): StakedIka {
    <b>assert</b>!(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_is_preactive">is_preactive</a>() || <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_is_active">is_active</a>(), <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EValidatorIsNotActive">EValidatorIsNotActive</a>);
    <b>assert</b>!(to_stake.value() &gt; 0, <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EZeroStake">EZeroStake</a>);
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a> = <b>if</b> (committee_selected) {
        current_epoch + 2
    } <b>else</b> {
        current_epoch + 1
    };
    <b>let</b> staked_amount = to_stake.value();
    <b>let</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a> = <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_mint">staked_ika::mint</a>(
        <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_id">validator_id</a>(),
        to_stake,
        <a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a>,
        ctx,
    );
    // Add the stake to the pending stake either <b>for</b> E+1 or E+2.
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.pending_stake.insert_or_add(<a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a>, staked_amount);
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_request_withdraw_stake"></a>

## Function `request_withdraw_stake`

Request withdrawal of the given amount from the staked IKA.
Marks the <code>StakedIka</code> as withdrawing and updates the activation epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_request_withdraw_stake">request_withdraw_stake</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>, in_current_committee: bool, in_next_committee: bool, current_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_request_withdraw_stake">request_withdraw_stake</a>(
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>,
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: &<b>mut</b> StakedIka,
    in_current_committee: bool,
    in_next_committee: bool,
    current_epoch: u64,
) {
    <b>assert</b>!(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.value() &gt; 0, <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EZeroStake">EZeroStake</a>);
    <b>assert</b>!(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_id">validator_id</a>() == <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_id">validator_id</a>(), <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EIncorrectValidatorId">EIncorrectValidatorId</a>);
    <b>assert</b>!(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.is_staked(), <a href="../ika_system/validator.md#(ika_system=0x0)_validator_ENotStaked">ENotStaked</a>);
    // Only allow requesting <b>if</b> the stake cannot be withdrawn directly.
    <b>assert</b>!(!<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.can_withdraw_early(in_next_committee, current_epoch), <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EWithdrawDirectly">EWithdrawDirectly</a>);
    // Early withdrawal request: only possible <b>if</b> activation epoch <b>has</b> not been
    // reached, and the stake is already counted <b>for</b> the next committee selection.
    <b>if</b> (<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a>() == current_epoch + 1) {
        <b>let</b> withdraw_epoch = <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a>() + 1;
        // register principal in the early withdrawals, the value will get converted to
        // the token amount in the `<a href="../ika_system/validator.md#(ika_system=0x0)_validator_process_pending_stake">process_pending_stake</a>` function
        <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.pre_active_withdrawals.insert_or_add(withdraw_epoch, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.value());
        <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_withdrawing">set_withdrawing</a>(withdraw_epoch);
        <b>return</b>
    };
    <b>assert</b>!(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a>() &lt;= current_epoch, <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EActivationEpochNotReached">EActivationEpochNotReached</a>);
    // If the node is in the committee, the stake will be withdrawn in E+2,
    // otherwise in E+1.
    <b>let</b> withdraw_epoch = <b>if</b> (in_next_committee) {
        current_epoch + 2
    } <b>else</b> <b>if</b> (in_current_committee) {
        current_epoch + 1
    } <b>else</b> {
        <b>abort</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EWithdrawDirectly">EWithdrawDirectly</a>
    };
    <b>let</b> principal_amount = <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.value();
    <b>let</b> share_amount = <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>
        .<a href="../ika_system/validator.md#(ika_system=0x0)_validator_exchange_rate_at_epoch">exchange_rate_at_epoch</a>(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a>())
        .convert_to_share_amount(principal_amount);
    <b>assert</b>!(share_amount != 0, <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EZeroShares">EZeroShares</a>);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.pending_shares_withdraw.insert_or_add(withdraw_epoch, share_amount);
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_withdrawing">set_withdrawing</a>(withdraw_epoch);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_withdraw_stake"></a>

## Function `withdraw_stake`

Perform the withdrawal of the staked IKA, returning the amount to the caller.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_withdraw_stake">withdraw_stake</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>, in_current_committee: bool, in_next_committee: bool, current_epoch: u64): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_withdraw_stake">withdraw_stake</a>(
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>,
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: StakedIka,
    in_current_committee: bool,
    in_next_committee: bool,
    current_epoch: u64,
): Balance&lt;IKA&gt; {
    <b>assert</b>!(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.value() &gt; 0, <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EZeroStake">EZeroStake</a>);
    <b>assert</b>!(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_id">validator_id</a>() == <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_id">validator_id</a>(), <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EIncorrectValidatorId">EIncorrectValidatorId</a>);
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a> = <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a>();
    // One step, early withdrawal in the case when committee before
    // activation epoch hasn't been selected. covers both E+1 and E+2 cases.
    <b>if</b> (<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.can_withdraw_early(in_next_committee, current_epoch)) {
        <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.pending_stake.reduce(<a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.value());
        <b>return</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.into_balance()
    };
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_rewards_amount">rewards_amount</a> = <b>if</b> (
        !in_current_committee && !in_next_committee && <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.is_staked()
    ) {
        // One step withdrawal <b>for</b> an inactive node.
        <b>if</b> (<a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a> &gt; current_epoch) {
            // Not even active stake yet, remove from pending stake.
            <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.pending_stake.reduce(<a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.value());
            0
        } <b>else</b> {
            // Active stake, remove it with the current epoch <b>as</b> the withdraw epoch.
            <b>let</b> share_amount = <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>
                .<a href="../ika_system/validator.md#(ika_system=0x0)_validator_exchange_rate_at_epoch">exchange_rate_at_epoch</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a>)
                .convert_to_share_amount(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.value());
            <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.pending_shares_withdraw.insert_or_add(current_epoch, share_amount);
            <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_calculate_rewards">calculate_rewards</a>(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.value(), <a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a>, current_epoch)
        }
        // Note that <b>if</b> the stake is in state Withdrawing, it can either be
        // from a pre-active withdrawal, but then
        // (in_current_committee || in_next_committee) is <b>true</b> since it was
        // an early withdrawal, or from a standard two step withdrawal,
        // which is handled below.
    } <b>else</b> {
        // Normal two-step withdrawals.
        <b>assert</b>!(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_is_withdrawing">is_withdrawing</a>(), <a href="../ika_system/validator.md#(ika_system=0x0)_validator_ENotWithdrawing">ENotWithdrawing</a>);
        <b>assert</b>!(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.withdraw_epoch() &lt;= current_epoch, <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EWithdrawEpochNotReached">EWithdrawEpochNotReached</a>);
        <b>assert</b>!(<a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a> &lt;= current_epoch, <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EActivationEpochNotReached">EActivationEpochNotReached</a>);
        <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_calculate_rewards">calculate_rewards</a>(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.value(), <a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.withdraw_epoch())
    };
    <b>let</b> principal = <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.into_balance();
    // Withdraw rewards. Due to rounding errors, there's a chance that the
    // rewards amount is higher than the rewards <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>, in this case, we
    // withdraw the maximum amount possible.
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_rewards_amount">rewards_amount</a> = <a href="../ika_system/validator.md#(ika_system=0x0)_validator_rewards_amount">rewards_amount</a>.min(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.rewards_pool.value());
    <b>let</b> <b>mut</b> to_withdraw = <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.rewards_pool.split(<a href="../ika_system/validator.md#(ika_system=0x0)_validator_rewards_amount">rewards_amount</a>);
    to_withdraw.join(principal);
    to_withdraw
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_advance_epoch"></a>

## Function `advance_epoch`

Advance epoch for the <code><a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a></code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_advance_epoch">advance_epoch</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>, rewards: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;, current_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_advance_epoch">advance_epoch</a>(
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>,
    <b>mut</b> rewards: Balance&lt;IKA&gt;,
    current_epoch: u64,
) {
    <b>assert</b>!(current_epoch &gt; <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.latest_epoch, <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EValidatorAlreadyUpdated">EValidatorAlreadyUpdated</a>);
    // Sanity check.
    <b>assert</b>!(rewards.value() == 0 || <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_ika_balance">ika_balance</a> &gt; 0, <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EIncorrectEpochAdvance">EIncorrectEpochAdvance</a>);
    // Split the commission from the rewards.
    <b>let</b> total_rewards = rewards.value();
    <b>let</b> commission = rewards.split(
        total_rewards * (<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_commission_rate">commission_rate</a> <b>as</b> u64) / (<a href="../ika_system/validator.md#(ika_system=0x0)_validator_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a> <b>as</b> u64),
    );
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.commission.join(commission);
    // Update the <a href="../ika_system/validator.md#(ika_system=0x0)_validator_commission_rate">commission_rate</a> <b>for</b> the <a href="../ika_system/validator.md#(ika_system=0x0)_validator_new">new</a> epoch <b>if</b> there's a pending value.
    // Note that pending commission rates are set 2 epochs ahead, so users are
    // aware of the rate change in advance.
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.pending_commission_rate.inner().try_get(&current_epoch).do!(|<a href="../ika_system/validator.md#(ika_system=0x0)_validator_commission_rate">commission_rate</a>| {
        <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_commission_rate">commission_rate</a> = <a href="../ika_system/validator.md#(ika_system=0x0)_validator_commission_rate">commission_rate</a> <b>as</b> u16;
        <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.pending_commission_rate.flush(current_epoch);
    });
    // Add rewards to the <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> and update the `<a href="../ika_system/validator.md#(ika_system=0x0)_validator_ika_balance">ika_balance</a>`.
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_rewards_amount">rewards_amount</a> = rewards.value();
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.rewards_pool.join(rewards);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_ika_balance">ika_balance</a> = <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_ika_balance">ika_balance</a> + <a href="../ika_system/validator.md#(ika_system=0x0)_validator_rewards_amount">rewards_amount</a>;
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.latest_epoch = current_epoch;
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>.rotate_next_epoch_info();
    // Perform stake deduction / addition <b>for</b> the current epoch.
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_process_pending_stake">process_pending_stake</a>(current_epoch);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_process_pending_stake"></a>

## Function `process_pending_stake`

Process the pending stake and withdrawal requests for the validator. Called in the
<code><a href="../ika_system/validator.md#(ika_system=0x0)_validator_advance_epoch">advance_epoch</a></code> function in case the validator is in the committee and receives the
rewards. And may be called in user-facing functions to update the validator state,
if the validator is not in the committee.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_process_pending_stake">process_pending_stake</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>, current_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_process_pending_stake">process_pending_stake</a>(
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>,
    current_epoch: u64,
) {
    // Set the exchange rate <b>for</b> the current epoch.
    <b>let</b> exchange_rate = <a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_new">token_exchange_rate::new</a>(
        <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_ika_balance">ika_balance</a>,
        <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.num_shares,
    );
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_exchange_rates">exchange_rates</a>.add(current_epoch, exchange_rate);
    // Process additions.
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_ika_balance">ika_balance</a> = <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_ika_balance">ika_balance</a> + <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.pending_stake.flush(current_epoch);
    // Process withdrawals.
    // each value in pending withdrawals contains the principal which became
    // active in the previous epoch. so unlike other pending values, we need to
    // flush it one by one, recalculating the exchange rate and <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> share amount
    // <b>for</b> each early withdrawal epoch.
    <b>let</b> <b>mut</b> pre_active_shares_withdraw = 0;
    <b>let</b> <b>mut</b> pre_active_withdrawals = <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.pre_active_withdrawals.unwrap();
    pre_active_withdrawals.keys().do!(|epoch| <b>if</b> (epoch &lt;= current_epoch) {
        <b>let</b> (_, epoch_value) = pre_active_withdrawals.remove(&epoch);
        // recall that pre_active_withdrawals contains stakes that were
        // active <b>for</b> exactly 1 epoch.
        <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a> = epoch - 1;
        <b>let</b> shares_for_epoch = <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>
            .<a href="../ika_system/validator.md#(ika_system=0x0)_validator_exchange_rate_at_epoch">exchange_rate_at_epoch</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a>)
            .convert_to_share_amount(epoch_value);
        pre_active_shares_withdraw = pre_active_shares_withdraw + shares_for_epoch;
    });
    // don't forget to flush the early withdrawals since we worked on a <b>copy</b>
    <b>let</b> _ = <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.pre_active_withdrawals.flush(current_epoch);
    <b>let</b> shares_withdraw = <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.pending_shares_withdraw.flush(current_epoch);
    <b>let</b> pending_withdrawal = exchange_rate.convert_to_ika_amount(
        shares_withdraw + pre_active_shares_withdraw,
    );
    // Sanity check that the amount is not higher than the <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> balance.
    <b>assert</b>!(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_ika_balance">ika_balance</a> &gt;= pending_withdrawal, <a href="../ika_system/validator.md#(ika_system=0x0)_validator_ECalculationError">ECalculationError</a>);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_ika_balance">ika_balance</a> = <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_ika_balance">ika_balance</a> - pending_withdrawal;
    // Recalculate the total number of shares according to the exchange rate.
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.num_shares = exchange_rate.convert_to_share_amount(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_ika_balance">ika_balance</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_verify_validator_cap"></a>

## Function `verify_validator_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_verify_validator_cap">verify_validator_cap</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_verify_validator_cap">verify_validator_cap</a>(self: &<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>, cap: &ValidatorCap) {
    <b>assert</b>!(cap.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_id">validator_id</a>() == self.id.to_inner(), <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EAuthorizationFailure">EAuthorizationFailure</a>);
    <b>assert</b>!(object::id(cap) == self.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_cap_id">validator_cap_id</a>, <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EAuthorizationFailure">EAuthorizationFailure</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_verify_operation_cap"></a>

## Function `verify_operation_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_verify_operation_cap">verify_operation_cap</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_verify_operation_cap">verify_operation_cap</a>(self: &<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>, cap: &ValidatorOperationCap) {
    <b>assert</b>!(cap.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_id">validator_id</a>() == self.id.to_inner(), <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EAuthorizationFailure">EAuthorizationFailure</a>);
    <b>assert</b>!(object::id(cap) == self.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_operation_cap_id">operation_cap_id</a>, <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EAuthorizationFailure">EAuthorizationFailure</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_verify_commission_cap"></a>

## Function `verify_commission_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_verify_commission_cap">verify_commission_cap</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCommissionCap">validator_cap::ValidatorCommissionCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_verify_commission_cap">verify_commission_cap</a>(self: &<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>, cap: &ValidatorCommissionCap) {
    <b>assert</b>!(cap.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_id">validator_id</a>() == self.id.to_inner(), <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EAuthorizationFailure">EAuthorizationFailure</a>);
    <b>assert</b>!(object::id(cap) == self.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_commission_cap_id">commission_cap_id</a>, <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EAuthorizationFailure">EAuthorizationFailure</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_name"></a>

## Function `set_name`

Sets the name of the validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_name">set_name</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>, name: <a href="../std/string.md#std_string_String">std::string::String</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_name">set_name</a>(self: &<b>mut</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>, name: String, cap: &ValidatorOperationCap) {
    self.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_verify_operation_cap">verify_operation_cap</a>(cap);
    self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_name">set_name</a>(name);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_validator_metadata"></a>

## Function `set_validator_metadata`

Sets the node metadata.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_validator_metadata">set_validator_metadata</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, metadata: (ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">validator_metadata::ValidatorMetadata</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_validator_metadata">set_validator_metadata</a>(self: &<b>mut</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>, cap: &ValidatorOperationCap, metadata: ValidatorMetadata) {
    self.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_verify_operation_cap">verify_operation_cap</a>(cap);
    self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_validator_metadata">set_validator_metadata</a>(metadata);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_next_commission"></a>

## Function `set_next_commission`

Sets the next commission rate for the validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_next_commission">set_next_commission</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>, <a href="../ika_system/validator.md#(ika_system=0x0)_validator_commission_rate">commission_rate</a>: u16, current_epoch: u64, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_next_commission">set_next_commission</a>(
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>,
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator_commission_rate">commission_rate</a>: u16,
    current_epoch: u64,
    cap: &ValidatorOperationCap,
) {
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_verify_operation_cap">verify_operation_cap</a>(cap);
    <b>assert</b>!(<a href="../ika_system/validator.md#(ika_system=0x0)_validator_commission_rate">commission_rate</a> &lt;= <a href="../ika_system/validator.md#(ika_system=0x0)_validator_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a>, <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EIncorrectCommissionRate">EIncorrectCommissionRate</a>);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.pending_commission_rate.insert_or_replace(current_epoch + 2, <a href="../ika_system/validator.md#(ika_system=0x0)_validator_commission_rate">commission_rate</a> <b>as</b> u64);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_next_epoch_network_address"></a>

## Function `set_next_epoch_network_address`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_next_epoch_network_address">set_next_epoch_network_address</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>, network_address: <a href="../std/string.md#std_string_String">std::string::String</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_next_epoch_network_address">set_next_epoch_network_address</a>(
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>,
    network_address: String,
    cap: &ValidatorOperationCap,
) {
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_verify_operation_cap">verify_operation_cap</a>(cap);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_next_epoch_network_address">set_next_epoch_network_address</a>(network_address);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_next_epoch_p2p_address"></a>

## Function `set_next_epoch_p2p_address`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_next_epoch_p2p_address">set_next_epoch_p2p_address</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>, p2p_address: <a href="../std/string.md#std_string_String">std::string::String</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_next_epoch_p2p_address">set_next_epoch_p2p_address</a>(
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>,
    p2p_address: String,
    cap: &ValidatorOperationCap,
) {
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_verify_operation_cap">verify_operation_cap</a>(cap);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_next_epoch_p2p_address">set_next_epoch_p2p_address</a>(p2p_address);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_next_epoch_consensus_address"></a>

## Function `set_next_epoch_consensus_address`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_next_epoch_consensus_address">set_next_epoch_consensus_address</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>, consensus_address: <a href="../std/string.md#std_string_String">std::string::String</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_next_epoch_consensus_address">set_next_epoch_consensus_address</a>(
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>,
    consensus_address: String,
    cap: &ValidatorOperationCap,
) {
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_verify_operation_cap">verify_operation_cap</a>(cap);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_next_epoch_consensus_address">set_next_epoch_consensus_address</a>(consensus_address);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_next_epoch_protocol_pubkey_bytes"></a>

## Function `set_next_epoch_protocol_pubkey_bytes`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_next_epoch_protocol_pubkey_bytes">set_next_epoch_protocol_pubkey_bytes</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>, protocol_pubkey_bytes: vector&lt;u8&gt;, proof_of_possession: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, ctx: &<a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_next_epoch_protocol_pubkey_bytes">set_next_epoch_protocol_pubkey_bytes</a>(
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>,
    protocol_pubkey_bytes: vector&lt;u8&gt;,
    proof_of_possession: vector&lt;u8&gt;,
    cap: &ValidatorOperationCap,
    ctx: &TxContext,
) {
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_verify_operation_cap">verify_operation_cap</a>(cap);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_next_epoch_protocol_pubkey_bytes">set_next_epoch_protocol_pubkey_bytes</a>(protocol_pubkey_bytes, proof_of_possession, ctx);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_next_epoch_network_pubkey_bytes"></a>

## Function `set_next_epoch_network_pubkey_bytes`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_next_epoch_network_pubkey_bytes">set_next_epoch_network_pubkey_bytes</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>, network_pubkey_bytes: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_next_epoch_network_pubkey_bytes">set_next_epoch_network_pubkey_bytes</a>(
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>,
    network_pubkey_bytes: vector&lt;u8&gt;,
    cap: &ValidatorOperationCap,
) {
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_verify_operation_cap">verify_operation_cap</a>(cap);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_next_epoch_network_pubkey_bytes">set_next_epoch_network_pubkey_bytes</a>(network_pubkey_bytes);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_next_epoch_consensus_pubkey_bytes"></a>

## Function `set_next_epoch_consensus_pubkey_bytes`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_next_epoch_consensus_pubkey_bytes">set_next_epoch_consensus_pubkey_bytes</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>, consensus_pubkey_bytes: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_next_epoch_consensus_pubkey_bytes">set_next_epoch_consensus_pubkey_bytes</a>(
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>,
    consensus_pubkey_bytes: vector&lt;u8&gt;,
    cap: &ValidatorOperationCap,
) {
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_verify_operation_cap">verify_operation_cap</a>(cap);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_next_epoch_consensus_pubkey_bytes">set_next_epoch_consensus_pubkey_bytes</a>(consensus_pubkey_bytes);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_next_epoch_class_groups_pubkey_and_proof_bytes"></a>

## Function `set_next_epoch_class_groups_pubkey_and_proof_bytes`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_next_epoch_class_groups_pubkey_and_proof_bytes">set_next_epoch_class_groups_pubkey_and_proof_bytes</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>, class_groups_pubkey_and_proof_bytes: (ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_next_epoch_class_groups_pubkey_and_proof_bytes">set_next_epoch_class_groups_pubkey_and_proof_bytes</a>(
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>,
    class_groups_pubkey_and_proof_bytes: ClassGroupsPublicKeyAndProof,
    cap: &ValidatorOperationCap,
) {
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_verify_operation_cap">verify_operation_cap</a>(cap);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_set_next_epoch_class_groups_pubkey_and_proof_bytes">set_next_epoch_class_groups_pubkey_and_proof_bytes</a>(class_groups_pubkey_and_proof_bytes);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_destroy_empty"></a>

## Function `destroy_empty`

Destroy the validator if it is empty.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_destroy_empty">destroy_empty</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: (ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_destroy_empty">destroy_empty</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: <a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>) {
    <b>assert</b>!(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_is_empty">is_empty</a>(), <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EValidatorNotEmpty">EValidatorNotEmpty</a>);
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a> {
        id,
        <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>,
        <a href="../ika_system/validator.md#(ika_system=0x0)_validator_exchange_rates">exchange_rates</a>,
        rewards_pool,
        commission,
        extra_fields,
        ..,
    } = <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>;
    id.delete();
    <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>.destroy();
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator_exchange_rates">exchange_rates</a>.drop();
    commission.destroy_zero();
    rewards_pool.destroy_zero();
    extra_fields.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_destroy_empty">destroy_empty</a>();
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_exchange_rate_at_epoch"></a>

## Function `exchange_rate_at_epoch`

Returns the exchange rate for the given current or future epoch. If there
isn't a value for the specified epoch, it will look for the most recent
value down to the validator activation epoch.
Note that exchange rates are only set for epochs in which the node is in
the committee, and otherwise the rate remains static.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_exchange_rate_at_epoch">exchange_rate_at_epoch</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &(ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>, epoch: u64): (ika_system=0x0)::<a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_TokenExchangeRate">token_exchange_rate::TokenExchangeRate</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_exchange_rate_at_epoch">exchange_rate_at_epoch</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>, epoch: u64): TokenExchangeRate {
    // If the <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> is preactive then the exchange rate is always 1:1.
    <b>if</b> (<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_is_preactive_at_epoch">is_preactive_at_epoch</a>(epoch)) {
        <b>return</b> <a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_flat">token_exchange_rate::flat</a>()
    };
    <b>let</b> clamped_epoch = <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_withdrawing_epoch">withdrawing_epoch</a>().get_with_default(epoch);
    <b>let</b> <b>mut</b> epoch = clamped_epoch.min(epoch);
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a> = *<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a>.borrow();
    <b>while</b> (epoch &gt;= <a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a>) {
        <b>if</b> (<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_exchange_rates">exchange_rates</a>.contains(epoch)) {
            <b>return</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_exchange_rates">exchange_rates</a>[epoch]
        };
        epoch = epoch - 1;
    };
    <a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_flat">token_exchange_rate::flat</a>()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_ika_balance_at_epoch"></a>

## Function `ika_balance_at_epoch`

Returns the expected active stake for current or future epoch <code>E</code> for the validator.
It processes the pending stake and withdrawal requests from the current epoch
to <code>E</code>.

Should be the main function to calculate the active stake for the validator at
the given epoch, due to the complexity of the pending stake and withdrawal
requests, and lack of immediate updates.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_ika_balance_at_epoch">ika_balance_at_epoch</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &(ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>, epoch: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_ika_balance_at_epoch">ika_balance_at_epoch</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>, epoch: u64): u64 {
    <b>let</b> exchange_rate = <a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_new">token_exchange_rate::new</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_ika_balance">ika_balance</a>, <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.num_shares);
    <b>let</b> <b>mut</b> pre_active_shares_withdraw = 0;
    <b>let</b> pre_active_withdrawals = <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.pre_active_withdrawals.unwrap();
    pre_active_withdrawals.keys().do_ref!(|old_epoch| <b>if</b> (*old_epoch &lt;= epoch) {
        <b>let</b> ika_value = pre_active_withdrawals.get(old_epoch);
        // recall that pre_active_withdrawals contains stakes that were
        // active <b>for</b> exactly 1 epoch. since the node might have been
        // inactive, this list may contain more than one value
        // (although <a href="../ika_system/validator.md#(ika_system=0x0)_validator_exchange_rate_at_epoch">exchange_rate_at_epoch</a> will <b>return</b> the same value).
        <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a> = *old_epoch - 1;
        <b>let</b> shares_for_epoch = <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>
            .<a href="../ika_system/validator.md#(ika_system=0x0)_validator_exchange_rate_at_epoch">exchange_rate_at_epoch</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a>)
            .convert_to_share_amount(*ika_value);
        pre_active_shares_withdraw = pre_active_shares_withdraw + shares_for_epoch;
    });
    <b>let</b> shares_withdraw = <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.pending_shares_withdraw.value_at(epoch);
    <b>let</b> pending_withdrawal = exchange_rate.convert_to_ika_amount(
        shares_withdraw + pre_active_shares_withdraw,
    );
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_ika_balance">ika_balance</a> + <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.pending_stake.value_at(epoch) - pending_withdrawal
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_rotate_operation_cap"></a>

## Function `rotate_operation_cap`

Create a new <code>ValidatorOperationCap</code>, and registers it,
thus revoking the previous cap's permission.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_rotate_operation_cap">rotate_operation_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_rotate_operation_cap">rotate_operation_cap</a>(
    self: &<b>mut</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>,
    cap: &ValidatorCap,
    ctx: &<b>mut</b> TxContext,
): ValidatorOperationCap {
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_id">validator_id</a> = cap.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_id">validator_id</a>();
    self.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_verify_validator_cap">verify_validator_cap</a>(cap);
    <b>let</b> operation_cap = <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_new_validator_operation_cap">validator_cap::new_validator_operation_cap</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_id">validator_id</a>, ctx);
    self.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_operation_cap_id">operation_cap_id</a> = object::id(&operation_cap);
    operation_cap
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_rotate_commission_cap"></a>

## Function `rotate_commission_cap`

Create a new <code>ValidatorCommissionCap</code>, and registers it,
thus revoking the previous cap's permission.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_rotate_commission_cap">rotate_commission_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCommissionCap">validator_cap::ValidatorCommissionCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_rotate_commission_cap">rotate_commission_cap</a>(
    self: &<b>mut</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>,
    cap: &ValidatorCap,
    ctx: &<b>mut</b> TxContext,
): ValidatorCommissionCap {
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_id">validator_id</a> = cap.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_id">validator_id</a>();
    self.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_verify_validator_cap">verify_validator_cap</a>(cap);
    <b>let</b> commission_cap = <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_new_validator_commission_cap">validator_cap::new_validator_commission_cap</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_id">validator_id</a>, ctx);
    self.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_commission_cap_id">commission_cap_id</a> = object::id(&commission_cap);
    commission_cap
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_collect_commission"></a>

## Function `collect_commission`

Withdraws the commission from the validator. Amount is optional, if not provided,
the full commission is withdrawn.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_collect_commission">collect_commission</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCommissionCap">validator_cap::ValidatorCommissionCap</a>, amount: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_collect_commission">collect_commission</a>(
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<b>mut</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>,
    cap: &ValidatorCommissionCap,
    amount: Option&lt;u64&gt;,
): Balance&lt;IKA&gt; {
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_verify_commission_cap">verify_commission_cap</a>(cap);
    <b>if</b> (amount.is_some()) {
        <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.commission.split(*amount.borrow())
    } <b>else</b> {
        <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.commission.withdraw_all()
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_validator_id"></a>

## Function `validator_id`

Returns the validator id for the validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_id">validator_id</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &(ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_id">validator_id</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>): ID { <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.id.to_inner() }
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_validator_cap_id"></a>

## Function `validator_cap_id`

Returns the validator cap for the validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_cap_id">validator_cap_id</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &(ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_cap_id">validator_cap_id</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>): ID { <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_validator_cap_id">validator_cap_id</a> }
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_operation_cap_id"></a>

## Function `operation_cap_id`

Returns the operation cap id for the validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_operation_cap_id">operation_cap_id</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &(ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_operation_cap_id">operation_cap_id</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>): ID { <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_operation_cap_id">operation_cap_id</a> }
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_commission_cap_id"></a>

## Function `commission_cap_id`

Returns the commission cap id for the validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_commission_cap_id">commission_cap_id</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &(ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_commission_cap_id">commission_cap_id</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>): ID { <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_commission_cap_id">commission_cap_id</a> }
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_commission_rate"></a>

## Function `commission_rate`

Returns the commission rate for the validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_commission_rate">commission_rate</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &(ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>): u16
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_commission_rate">commission_rate</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>): u16 { <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_commission_rate">commission_rate</a> }
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_commission_amount"></a>

## Function `commission_amount`

Returns the commission amount for the validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_commission_amount">commission_amount</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &(ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_commission_amount">commission_amount</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>): u64 { <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.commission.value() }
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_rewards_amount"></a>

## Function `rewards_amount`

Returns the rewards amount for the validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_rewards_amount">rewards_amount</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &(ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_rewards_amount">rewards_amount</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>): u64 { <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.rewards_pool.value() }
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_ika_balance"></a>

## Function `ika_balance`

Returns the rewards for the validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_ika_balance">ika_balance</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &(ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_ika_balance">ika_balance</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>): u64 { <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_ika_balance">ika_balance</a> }
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_activation_epoch"></a>

## Function `activation_epoch`

Returns the activation epoch for the validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &(ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>): Option&lt;u64&gt; { <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a> }
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_validator_info"></a>

## Function `validator_info`

Returns the validator info for the validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &(ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>): &(ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>): &ValidatorInfo { &<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a> }
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_is_preactive"></a>

## Function `is_preactive`

Returns <code><b>true</b></code> if the validator is preactive.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_is_preactive">is_preactive</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &(ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_is_preactive">is_preactive</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>): bool { <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.state == ValidatorState::PreActive }
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_is_active"></a>

## Function `is_active`

Returns <code><b>true</b></code> if the validator is active.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_is_active">is_active</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &(ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_is_active">is_active</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>): bool { <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.state == ValidatorState::Active }
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_is_withdrawing"></a>

## Function `is_withdrawing`

Returns <code><b>true</b></code> if the validator is withdrawing.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_is_withdrawing">is_withdrawing</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &(ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_is_withdrawing">is_withdrawing</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>): bool {
    match (<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.state) {
        ValidatorState::Withdrawing(_) =&gt; <b>true</b>,
        _ =&gt; <b>false</b>,
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_withdrawing_epoch"></a>

## Function `withdrawing_epoch`

Returns the epoch in which the validator is withdrawing.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_withdrawing_epoch">withdrawing_epoch</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &(ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_withdrawing_epoch">withdrawing_epoch</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>): Option&lt;u64&gt; {
    match (<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.state) {
        ValidatorState::Withdrawing(epoch) =&gt; option::some(epoch),
        _ =&gt; option::none(),
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_exchange_rates"></a>

## Function `exchange_rates`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_exchange_rates">exchange_rates</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &(ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>): &<a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, (ika_system=0x0)::<a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_TokenExchangeRate">token_exchange_rate::TokenExchangeRate</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_exchange_rates">exchange_rates</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>): &Table&lt;u64, TokenExchangeRate&gt; {
    &<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_exchange_rates">exchange_rates</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_is_empty"></a>

## Function `is_empty`

Returns <code><b>true</b></code> if the validator is empty.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_is_empty">is_empty</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &(ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_is_empty">is_empty</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>): bool {
    <b>let</b> pending_stake = <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.pending_stake.unwrap();
    <b>let</b> non_empty = pending_stake.keys().count!(|epoch| pending_stake[epoch] != 0);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.rewards_pool.value() == 0 &&
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.num_shares == 0 &&
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.commission.value() == 0 &&
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_ika_balance">ika_balance</a> == 0 &&
    non_empty == 0
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_calculate_rewards"></a>

## Function `calculate_rewards`

Calculate the rewards for an amount with value <code>staked_principal</code>, staked in the validator between
<code><a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a></code> and <code>withdraw_epoch</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_calculate_rewards">calculate_rewards</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &(ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>, staked_principal: u64, <a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a>: u64, withdraw_epoch: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_calculate_rewards">calculate_rewards</a>(
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>,
    staked_principal: u64,
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a>: u64,
    withdraw_epoch: u64,
): u64 {
    <b>let</b> shares = <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>
        .<a href="../ika_system/validator.md#(ika_system=0x0)_validator_exchange_rate_at_epoch">exchange_rate_at_epoch</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a>)
        .convert_to_share_amount(staked_principal);
    <b>let</b> ika_amount = <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_exchange_rate_at_epoch">exchange_rate_at_epoch</a>(withdraw_epoch).convert_to_ika_amount(shares);
    <b>if</b> (ika_amount &gt;= staked_principal) {
        ika_amount - staked_principal
    } <b>else</b> 0
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_is_preactive_at_epoch"></a>

## Function `is_preactive_at_epoch`

Returns true if the provided validator is preactive at the provided epoch.


<pre><code><b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_is_preactive_at_epoch">is_preactive_at_epoch</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &(ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>, epoch: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_is_preactive_at_epoch">is_preactive_at_epoch</a>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: &<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>, epoch: u64): bool {
    // Either the <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> is currently preactive or the <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>'s starting epoch is later than the provided epoch.
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_is_preactive">is_preactive</a>() || (*<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator.md#(ika_system=0x0)_validator_activation_epoch">activation_epoch</a>.borrow() &gt; epoch)
}
</code></pre>



</details>
