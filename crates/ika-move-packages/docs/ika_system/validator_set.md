---
title: Module `0x0::validator_set`
---



-  [Struct `ValidatorSet`](#0x0_validator_set_ValidatorSet)
-  [Struct `ValidatorEpochInfoEventV1`](#0x0_validator_set_ValidatorEpochInfoEventV1)
-  [Struct `ValidatorJoinEvent`](#0x0_validator_set_ValidatorJoinEvent)
-  [Struct `ValidatorLeaveEvent`](#0x0_validator_set_ValidatorLeaveEvent)
-  [Constants](#@Constants_0)
-  [Function `new`](#0x0_validator_set_new)
-  [Function `initialize`](#0x0_validator_set_initialize)
-  [Function `request_add_validator_candidate`](#0x0_validator_set_request_add_validator_candidate)
-  [Function `request_remove_validator_candidate`](#0x0_validator_set_request_remove_validator_candidate)
-  [Function `request_add_validator`](#0x0_validator_set_request_add_validator)
-  [Function `assert_no_pending_or_active_duplicates`](#0x0_validator_set_assert_no_pending_or_active_duplicates)
-  [Function `request_remove_validator`](#0x0_validator_set_request_remove_validator)
-  [Function `request_add_stake`](#0x0_validator_set_request_add_stake)
-  [Function `request_withdraw_stake`](#0x0_validator_set_request_withdraw_stake)
-  [Function `convert_to_fungible_staked_ika`](#0x0_validator_set_convert_to_fungible_staked_ika)
-  [Function `redeem_fungible_staked_ika`](#0x0_validator_set_redeem_fungible_staked_ika)
-  [Function `request_set_commission_rate`](#0x0_validator_set_request_set_commission_rate)
-  [Function `process_mid_epoch`](#0x0_validator_set_process_mid_epoch)
-  [Function `advance_epoch`](#0x0_validator_set_advance_epoch)
-  [Function `activate_added_validators`](#0x0_validator_set_activate_added_validators)
-  [Function `update_and_process_low_stake_departures`](#0x0_validator_set_update_and_process_low_stake_departures)
-  [Function `effectuate_staged_metadata`](#0x0_validator_set_effectuate_staged_metadata)
-  [Function `derive_computation_price_per_unit_size`](#0x0_validator_set_derive_computation_price_per_unit_size)
-  [Function `total_stake`](#0x0_validator_set_total_stake)
-  [Function `validator_total_stake_amount`](#0x0_validator_set_validator_total_stake_amount)
-  [Function `pool_exchange_rates`](#0x0_validator_set_pool_exchange_rates)
-  [Function `pending_active_validators_count`](#0x0_validator_set_pending_active_validators_count)
-  [Function `is_active_validator`](#0x0_validator_set_is_active_validator)
-  [Function `get_reporters_of`](#0x0_validator_set_get_reporters_of)
-  [Function `count_duplicates_vec`](#0x0_validator_set_count_duplicates_vec)
-  [Function `is_duplicate_validator`](#0x0_validator_set_is_duplicate_validator)
-  [Function `is_duplicate_with_active_validator`](#0x0_validator_set_is_duplicate_with_active_validator)
-  [Function `is_duplicate_with_next_epoch_active_committee`](#0x0_validator_set_is_duplicate_with_next_epoch_active_committee)
-  [Function `is_duplicate_with_pending_validator`](#0x0_validator_set_is_duplicate_with_pending_validator)
-  [Function `get_validator_mut`](#0x0_validator_set_get_validator_mut)
-  [Function `get_validator_ref`](#0x0_validator_set_get_validator_ref)
-  [Function `get_candidate_or_active_validator_mut`](#0x0_validator_set_get_candidate_or_active_validator_mut)
-  [Function `get_candidate_or_active_or_inactive_validator_mut`](#0x0_validator_set_get_candidate_or_active_or_inactive_validator_mut)
-  [Function `get_active_or_pending_validator_mut`](#0x0_validator_set_get_active_or_pending_validator_mut)
-  [Function `get_active_or_pending_or_candidate_validator_mut`](#0x0_validator_set_get_active_or_pending_or_candidate_validator_mut)
-  [Function `get_validator_mut_with_operation_cap`](#0x0_validator_set_get_validator_mut_with_operation_cap)
-  [Function `get_validator_mut_with_operation_cap_including_candidates`](#0x0_validator_set_get_validator_mut_with_operation_cap_including_candidates)
-  [Function `get_validator_mut_with_cap`](#0x0_validator_set_get_validator_mut_with_cap)
-  [Function `get_validator_mut_with_cap_including_candidates`](#0x0_validator_set_get_validator_mut_with_cap_including_candidates)
-  [Function `get_validator_indices`](#0x0_validator_set_get_validator_indices)
-  [Function `verify_operation_cap`](#0x0_validator_set_verify_operation_cap)
-  [Function `process_validator_departure`](#0x0_validator_set_process_validator_departure)
-  [Function `clean_report_records_leaving_validator`](#0x0_validator_set_clean_report_records_leaving_validator)
-  [Function `process_pending_validators`](#0x0_validator_set_process_pending_validators)
-  [Function `process_pending_stakes_and_withdraws`](#0x0_validator_set_process_pending_stakes_and_withdraws)
-  [Function `calculate_total_stakes`](#0x0_validator_set_calculate_total_stakes)
-  [Function `adjust_stake_and_computation_price`](#0x0_validator_set_adjust_stake_and_computation_price)
-  [Function `compute_reward_adjustments`](#0x0_validator_set_compute_reward_adjustments)
-  [Function `compute_slashed_validators`](#0x0_validator_set_compute_slashed_validators)
-  [Function `compute_unadjusted_reward_distribution`](#0x0_validator_set_compute_unadjusted_reward_distribution)
-  [Function `compute_adjusted_reward_distribution`](#0x0_validator_set_compute_adjusted_reward_distribution)
-  [Function `distribute_reward`](#0x0_validator_set_distribute_reward)
-  [Function `emit_validator_epoch_events`](#0x0_validator_set_emit_validator_epoch_events)
-  [Function `sum_voting_power_by_validator_indices`](#0x0_validator_set_sum_voting_power_by_validator_indices)
-  [Function `report_validator`](#0x0_validator_set_report_validator)
-  [Function `undo_report_validator`](#0x0_validator_set_undo_report_validator)
-  [Function `report_validator_impl`](#0x0_validator_set_report_validator_impl)
-  [Function `undo_report_validator_impl`](#0x0_validator_set_undo_report_validator_impl)
-  [Function `active_committee`](#0x0_validator_set_active_committee)
-  [Function `next_epoch_active_committee`](#0x0_validator_set_next_epoch_active_committee)
-  [Function `next_pending_active_validators`](#0x0_validator_set_next_pending_active_validators)
-  [Function `is_validator_candidate`](#0x0_validator_set_is_validator_candidate)
-  [Function `is_inactive_validator`](#0x0_validator_set_is_inactive_validator)


<pre><code><b>use</b> <a href="committee.md#0x0_committee">0x0::committee</a>;
<b>use</b> <a href="../ika/ika.md#0x0_ika">0x0::ika</a>;
<b>use</b> <a href="staked_ika.md#0x0_staked_ika">0x0::staked_ika</a>;
<b>use</b> <a href="staking_pool.md#0x0_staking_pool">0x0::staking_pool</a>;
<b>use</b> <a href="validator.md#0x0_validator">0x0::validator</a>;
<b>use</b> <a href="validator_cap.md#0x0_validator_cap">0x0::validator_cap</a>;
<b>use</b> <a href="validator_inner.md#0x0_validator_inner_v1">0x0::validator_inner_v1</a>;
<b>use</b> <a href="../move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../move-stdlib/vector.md#0x1_vector">0x1::vector</a>;
<b>use</b> <a href="../sui-framework/bag.md#0x2_bag">0x2::bag</a>;
<b>use</b> <a href="../sui-framework/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../sui-framework/bls12381.md#0x2_bls12381">0x2::bls12381</a>;
<b>use</b> <a href="../sui-framework/event.md#0x2_event">0x2::event</a>;
<b>use</b> <a href="../sui-framework/group_ops.md#0x2_group_ops">0x2::group_ops</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/object_table.md#0x2_object_table">0x2::object_table</a>;
<b>use</b> <a href="../sui-framework/priority_queue.md#0x2_priority_queue">0x2::priority_queue</a>;
<b>use</b> <a href="../sui-framework/table.md#0x2_table">0x2::table</a>;
<b>use</b> <a href="../sui-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../sui-framework/vec_map.md#0x2_vec_map">0x2::vec_map</a>;
<b>use</b> <a href="../sui-framework/vec_set.md#0x2_vec_set">0x2::vec_set</a>;
</code></pre>



<a name="0x0_validator_set_ValidatorSet"></a>

## Struct `ValidatorSet`



<pre><code><b>struct</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>total_stake: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 Total amount of stake from all active validators at the beginning of the epoch.
</dd>
<dt>
<code>validators: <a href="../sui-framework/object_table.md#0x2_object_table_ObjectTable">object_table::ObjectTable</a>&lt;<a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, <a href="validator.md#0x0_validator_Validator">validator::Validator</a>&gt;</code>
</dt>
<dd>
 A talbe that contains all validators
</dd>
<dt>
<code>active_committee: <a href="committee.md#0x0_committee_Committee">committee::Committee</a></code>
</dt>
<dd>
 The current list of active committee of validators.
</dd>
<dt>
<code>next_epoch_active_committee: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="committee.md#0x0_committee_Committee">committee::Committee</a>&gt;</code>
</dt>
<dd>
 The next list of active committee of validators.
 It will become the active_committee at the end of the epoch.
</dd>
<dt>
<code>pending_active_validators: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>&gt;</code>
</dt>
<dd>
 The next list of peding active validators to be next_epoch_active_committee.
 It will start from the last next_epoch_active_committee and will be
 process between middle of the epochs and will be finlize
 at the middle of the epoch.
</dd>
<dt>
<code>at_risk_validators: <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;</code>
</dt>
<dd>
 Table storing the number of epochs during which a validator's stake has been below the low stake threshold.
</dd>
<dt>
<code>validator_report_records: <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, <a href="../sui-framework/vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>&gt;&gt;</code>
</dt>
<dd>
 A map storing the records of validator reporting each other.
 There is an entry in the map for each validator that has been reported
 at least once. The entry VecSet contains all the validators that reported
 them. If a validator has never been reported they don't have an entry in this map.
 This map persists across epoch: a peer continues being in a reported state until the
 reporter doesn't explicitly remove their report.
 Note that in case we want to support validator address change in future,
 the reports should be based on validator ids
</dd>
<dt>
<code>extra_fields: <a href="../sui-framework/bag.md#0x2_bag_Bag">bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="0x0_validator_set_ValidatorEpochInfoEventV1"></a>

## Struct `ValidatorEpochInfoEventV1`

Event containing staking and rewards related information of
each validator, emitted during epoch advancement.


<pre><code><b>struct</b> <a href="validator_set.md#0x0_validator_set_ValidatorEpochInfoEventV1">ValidatorEpochInfoEventV1</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>reference_gas_survey_quote: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>stake: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>voting_power: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>commission_rate: u16</code>
</dt>
<dd>

</dd>
<dt>
<code>pool_staking_reward: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>pool_token_exchange_rate: <a href="staking_pool.md#0x0_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a></code>
</dt>
<dd>

</dd>
<dt>
<code>tallying_rule_reporters: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>tallying_rule_global_score: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x0_validator_set_ValidatorJoinEvent"></a>

## Struct `ValidatorJoinEvent`

Event emitted every time a new validator joins the committee.
The epoch value corresponds to the first epoch this change takes place.


<pre><code><b>struct</b> <a href="validator_set.md#0x0_validator_set_ValidatorJoinEvent">ValidatorJoinEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x0_validator_set_ValidatorLeaveEvent"></a>

## Struct `ValidatorLeaveEvent`

Event emitted every time a validator leaves the committee.
The epoch value corresponds to the first epoch this change takes place.


<pre><code><b>struct</b> <a href="validator_set.md#0x0_validator_set_ValidatorLeaveEvent">ValidatorLeaveEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>is_voluntary: bool</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x0_validator_set_MIN_STAKING_THRESHOLD"></a>



<pre><code><b>const</b> <a href="validator_set.md#0x0_validator_set_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1000000000;
</code></pre>



<a name="0x0_validator_set_EInvalidCap"></a>



<pre><code><b>const</b> <a href="validator_set.md#0x0_validator_set_EInvalidCap">EInvalidCap</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 101;
</code></pre>



<a name="0x0_validator_set_BASIS_POINT_DENOMINATOR"></a>



<pre><code><b>const</b> <a href="validator_set.md#0x0_validator_set_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a>: u128 = 10000;
</code></pre>



<a name="0x0_validator_set_EAdvanceEpochOnlyAfterProcessMidEpoch"></a>



<pre><code>#[error]
<b>const</b> <a href="validator_set.md#0x0_validator_set_EAdvanceEpochOnlyAfterProcessMidEpoch">EAdvanceEpochOnlyAfterProcessMidEpoch</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = b"Advance epoch can be called only after process mid epoch.";
</code></pre>



<a name="0x0_validator_set_EAlreadyInitialized"></a>



<pre><code>#[error]
<b>const</b> <a href="validator_set.md#0x0_validator_set_EAlreadyInitialized">EAlreadyInitialized</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = b"Protocol cannot be initialized more than one time.";
</code></pre>



<a name="0x0_validator_set_ECannotReportOneself"></a>



<pre><code><b>const</b> <a href="validator_set.md#0x0_validator_set_ECannotReportOneself">ECannotReportOneself</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 17;
</code></pre>



<a name="0x0_validator_set_EDuplicateValidator"></a>



<pre><code><b>const</b> <a href="validator_set.md#0x0_validator_set_EDuplicateValidator">EDuplicateValidator</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 2;
</code></pre>



<a name="0x0_validator_set_EMinJoiningStakeNotReached"></a>



<pre><code><b>const</b> <a href="validator_set.md#0x0_validator_set_EMinJoiningStakeNotReached">EMinJoiningStakeNotReached</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 5;
</code></pre>



<a name="0x0_validator_set_ENonValidatorInReportRecords"></a>



<pre><code><b>const</b> <a href="validator_set.md#0x0_validator_set_ENonValidatorInReportRecords">ENonValidatorInReportRecords</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
</code></pre>



<a name="0x0_validator_set_ENotAValidator"></a>



<pre><code><b>const</b> <a href="validator_set.md#0x0_validator_set_ENotAValidator">ENotAValidator</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 4;
</code></pre>



<a name="0x0_validator_set_ENotActiveOrPendingValidator"></a>



<pre><code><b>const</b> <a href="validator_set.md#0x0_validator_set_ENotActiveOrPendingValidator">ENotActiveOrPendingValidator</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 9;
</code></pre>



<a name="0x0_validator_set_ENotCandidateOrActiveOrInactiveValidator"></a>



<pre><code><b>const</b> <a href="validator_set.md#0x0_validator_set_ENotCandidateOrActiveOrInactiveValidator">ENotCandidateOrActiveOrInactiveValidator</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 15;
</code></pre>



<a name="0x0_validator_set_ENotCandidateOrActiveOrPendingValidator"></a>



<pre><code><b>const</b> <a href="validator_set.md#0x0_validator_set_ENotCandidateOrActiveOrPendingValidator">ENotCandidateOrActiveOrPendingValidator</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 16;
</code></pre>



<a name="0x0_validator_set_ENotCandidateOrActiveValidator"></a>



<pre><code><b>const</b> <a href="validator_set.md#0x0_validator_set_ENotCandidateOrActiveValidator">ENotCandidateOrActiveValidator</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 14;
</code></pre>



<a name="0x0_validator_set_EProcessMidEpochOnlyAfterAdvanceEpoch"></a>



<pre><code>#[error]
<b>const</b> <a href="validator_set.md#0x0_validator_set_EProcessMidEpochOnlyAfterAdvanceEpoch">EProcessMidEpochOnlyAfterAdvanceEpoch</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = b"Process mid epoch can be called only after advance epoch.";
</code></pre>



<a name="0x0_validator_set_EReportRecordNotFound"></a>



<pre><code><b>const</b> <a href="validator_set.md#0x0_validator_set_EReportRecordNotFound">EReportRecordNotFound</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 18;
</code></pre>



<a name="0x0_validator_set_EStakingBelowThreshold"></a>



<pre><code><b>const</b> <a href="validator_set.md#0x0_validator_set_EStakingBelowThreshold">EStakingBelowThreshold</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 10;
</code></pre>



<a name="0x0_validator_set_EValidatorAlreadyRemoved"></a>



<pre><code><b>const</b> <a href="validator_set.md#0x0_validator_set_EValidatorAlreadyRemoved">EValidatorAlreadyRemoved</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 11;
</code></pre>



<a name="0x0_validator_set_EValidatorNotCandidate"></a>



<pre><code><b>const</b> <a href="validator_set.md#0x0_validator_set_EValidatorNotCandidate">EValidatorNotCandidate</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 7;
</code></pre>



<a name="0x0_validator_set_EValidatorSetEmpty"></a>



<pre><code><b>const</b> <a href="validator_set.md#0x0_validator_set_EValidatorSetEmpty">EValidatorSetEmpty</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 13;
</code></pre>



<a name="0x0_validator_set_new"></a>

## Function `new`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_new">new</a>(ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_new">new</a>(ctx: &<b>mut</b> TxContext): <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a> {
    // <b>let</b> total_stake = <a href="validator_set.md#0x0_validator_set_calculate_total_stakes">calculate_total_stakes</a>(&init_active_committee);
    // <b>let</b> <b>mut</b> staking_pool_mappings = <a href="../sui-framework/table.md#0x2_table_new">table::new</a>(ctx);
    // <b>let</b> num_validators = init_active_committee.length();
    // <b>let</b> <b>mut</b> i = 0;
    // <b>while</b> (i &lt; num_validators) {
    //     <b>let</b> <a href="validator.md#0x0_validator">validator</a> = &init_active_committee[i];
    //     staking_pool_mappings.add(staking_pool_id(<a href="validator.md#0x0_validator">validator</a>), sui_address(<a href="validator.md#0x0_validator">validator</a>));
    //     i = i + 1;
    // };
    <b>let</b> validators = <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a> {
        total_stake: 0,
        validators: <a href="../sui-framework/object_table.md#0x2_object_table_new">object_table::new</a>(ctx),
        active_committee: <a href="committee.md#0x0_committee_empty">committee::empty</a>(),
        next_epoch_active_committee: <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(),
        pending_active_validators: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>[],
        at_risk_validators: <a href="../sui-framework/vec_map.md#0x2_vec_map_empty">vec_map::empty</a>(),
        validator_report_records: <a href="../sui-framework/vec_map.md#0x2_vec_map_empty">vec_map::empty</a>(),
        extra_fields: <a href="../sui-framework/bag.md#0x2_bag_new">bag::new</a>(ctx),
    };
    //voting_power::set_voting_power(&<b>mut</b> validators.active_committee);
    validators
}
</code></pre>



</details>

<a name="0x0_validator_set_initialize"></a>

## Function `initialize`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_initialize">initialize</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_initialize">initialize</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>) {
    <b>assert</b>!(self.active_committee.members().is_empty(), <a href="validator_set.md#0x0_validator_set_EAlreadyInitialized">EAlreadyInitialized</a>);
    self.<a href="validator_set.md#0x0_validator_set_process_pending_validators">process_pending_validators</a>();
    self.active_committee = self.next_epoch_active_committee.extract();
    self.<a href="validator_set.md#0x0_validator_set_activate_added_validators">activate_added_validators</a>(0);
    self.total_stake = <a href="validator_set.md#0x0_validator_set_calculate_total_stakes">calculate_total_stakes</a>(self);
}
</code></pre>



</details>

<a name="0x0_validator_set_request_add_validator_candidate"></a>

## Function `request_add_validator_candidate`

Called by <code>ika_system</code> to add a new validator candidate.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_request_add_validator_candidate">request_add_validator_candidate</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, payment_address: <b>address</b>, protocol_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, network_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, consensus_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, proof_of_possession_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, name: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, description: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, image_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, project_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, network_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, p2p_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, consensus_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, computation_price: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, commission_rate: u16, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): (<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, <a href="validator_cap.md#0x0_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_request_add_validator_candidate">request_add_validator_candidate</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    payment_address: <b>address</b>,
    protocol_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    network_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    consensus_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    proof_of_possession_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    name: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    description: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    image_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    project_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    network_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    p2p_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    consensus_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    computation_price: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    commission_rate: u16,
    ctx: &<b>mut</b> TxContext,
): (ValidatorCap, ValidatorOperationCap) {

    <b>let</b> (<b>mut</b> <a href="validator.md#0x0_validator">validator</a>, cap, operation_cap) = <a href="validator.md#0x0_validator_create">validator::create</a>(
        payment_address,
        protocol_pubkey_bytes,
        network_pubkey_bytes,
        consensus_pubkey_bytes,
        proof_of_possession_bytes,
        name,
        description,
        image_url,
        project_url,
        network_address,
        p2p_address,
        consensus_address,
        computation_price,
        commission_rate,
        ctx,
    );

    <b>let</b> <a href="validator_inner.md#0x0_validator_inner_v1">validator_inner_v1</a> = <a href="validator.md#0x0_validator">validator</a>.load_validator_maybe_upgrade();

    <b>let</b> validator_id = <a href="validator_inner.md#0x0_validator_inner_v1">validator_inner_v1</a>.validator_id();

    // The next assertions are not critical for the protocol, but they are here <b>to</b> catch problematic configs earlier.
    <b>assert</b>!(
        !<a href="validator_set.md#0x0_validator_set_is_duplicate_with_active_validator">is_duplicate_with_active_validator</a>(self, <a href="validator_inner.md#0x0_validator_inner_v1">validator_inner_v1</a>)
                && !<a href="validator_set.md#0x0_validator_set_is_duplicate_with_pending_validator">is_duplicate_with_pending_validator</a>(self, <a href="validator_inner.md#0x0_validator_inner_v1">validator_inner_v1</a>)
                && !<a href="validator_set.md#0x0_validator_set_is_duplicate_with_next_epoch_active_committee">is_duplicate_with_next_epoch_active_committee</a>(self, <a href="validator_inner.md#0x0_validator_inner_v1">validator_inner_v1</a>),
        <a href="validator_set.md#0x0_validator_set_EDuplicateValidator">EDuplicateValidator</a>,
    );
    <b>assert</b>!(!self.validators.contains(validator_id), <a href="validator_set.md#0x0_validator_set_EDuplicateValidator">EDuplicateValidator</a>);

    <b>assert</b>!(<a href="validator_inner.md#0x0_validator_inner_v1">validator_inner_v1</a>.is_candidate(), <a href="validator_set.md#0x0_validator_set_EValidatorNotCandidate">EValidatorNotCandidate</a>);
    self
        .validators
        .add(
            validator_id,
            <a href="validator.md#0x0_validator">validator</a>,
        );
    (cap, operation_cap)
}
</code></pre>



</details>

<a name="0x0_validator_set_request_remove_validator_candidate"></a>

## Function `request_remove_validator_candidate`

Called by <code>ika_system</code> to remove a validator candidate, and move them to <code>inactive_committee</code>.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_request_remove_validator_candidate">request_remove_validator_candidate</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_request_remove_validator_candidate">request_remove_validator_candidate</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    cap: &ValidatorCap,
) {
    <b>let</b> validator_id = cap.validator_id();
    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.<a href="validator_set.md#0x0_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
    <b>assert</b>!(<a href="validator.md#0x0_validator">validator</a>.is_candidate(), <a href="validator_set.md#0x0_validator_set_EValidatorNotCandidate">EValidatorNotCandidate</a>);

    // Deactivate the staking pool.
    <a href="validator.md#0x0_validator">validator</a>.deactivate(epoch);
}
</code></pre>



</details>

<a name="0x0_validator_set_request_add_validator"></a>

## Function `request_add_validator`

Called by <code>ika_system</code> to add a new validator to <code>pending_active_validators</code>, which will be
processed at the end of epoch.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_request_add_validator">request_add_validator</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, min_joining_stake_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_request_add_validator">request_add_validator</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    min_joining_stake_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    cap: &ValidatorCap,
) {
    <b>let</b> validator_id = cap.validator_id();
    <b>assert</b>!(self.validators.contains(validator_id), <a href="validator_set.md#0x0_validator_set_EValidatorNotCandidate">EValidatorNotCandidate</a>);
    // We have <b>to</b> remove and <b>to</b> add again because we can have 2 refs <b>to</b> self
    <b>let</b> <b>mut</b> wrapper = self.validators.remove(validator_id);
    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = wrapper.load_validator_maybe_upgrade();
    <b>assert</b>!(
        !<a href="validator_set.md#0x0_validator_set_is_duplicate_with_active_validator">is_duplicate_with_active_validator</a>(self, <a href="validator.md#0x0_validator">validator</a>)
                && !<a href="validator_set.md#0x0_validator_set_is_duplicate_with_pending_validator">is_duplicate_with_pending_validator</a>(self, <a href="validator.md#0x0_validator">validator</a>)
                && !<a href="validator_set.md#0x0_validator_set_is_duplicate_with_next_epoch_active_committee">is_duplicate_with_next_epoch_active_committee</a>(self, <a href="validator.md#0x0_validator">validator</a>),
        <a href="validator_set.md#0x0_validator_set_EDuplicateValidator">EDuplicateValidator</a>,
    );
    <b>assert</b>!(<a href="validator.md#0x0_validator">validator</a>.is_candidate(), <a href="validator_set.md#0x0_validator_set_EValidatorNotCandidate">EValidatorNotCandidate</a>);
    <b>assert</b>!(<a href="validator.md#0x0_validator">validator</a>.total_stake_amount() &gt;= min_joining_stake_amount, <a href="validator_set.md#0x0_validator_set_EMinJoiningStakeNotReached">EMinJoiningStakeNotReached</a>);

    self.validators.add(validator_id, wrapper);

    self.pending_active_validators.push_back(validator_id);
}
</code></pre>



</details>

<a name="0x0_validator_set_assert_no_pending_or_active_duplicates"></a>

## Function `assert_no_pending_or_active_duplicates`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_assert_no_pending_or_active_duplicates">assert_no_pending_or_active_duplicates</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_assert_no_pending_or_active_duplicates">assert_no_pending_or_active_duplicates</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    validator_id: ID,
) {

    <b>let</b> active_validator_ids = self.active_committee.validator_ids();
    <b>let</b> pending_active_validators = self.pending_active_validators;

    <b>assert</b>!(self.validators.contains(validator_id), <a href="validator_set.md#0x0_validator_set_EValidatorNotCandidate">EValidatorNotCandidate</a>);
    // We have <b>to</b> remove and <b>to</b> add again because we can have 2 refs <b>to</b> self
    <b>let</b> <b>mut</b> wrapper = self.validators.remove(validator_id);
    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = wrapper.load_validator_maybe_upgrade();

    // Validator here must be active or pending, and thus must be identified <b>as</b> duplicate exactly once.
    <b>assert</b>!(
        <a href="validator_set.md#0x0_validator_set_count_duplicates_vec">count_duplicates_vec</a>(self, &active_validator_ids, <a href="validator.md#0x0_validator">validator</a>) +
                <a href="validator_set.md#0x0_validator_set_count_duplicates_vec">count_duplicates_vec</a>(self, &pending_active_validators, <a href="validator.md#0x0_validator">validator</a>) == 1,
        <a href="validator_set.md#0x0_validator_set_EDuplicateValidator">EDuplicateValidator</a>,
    );

    self.validators.add(validator_id, wrapper);
}
</code></pre>



</details>

<a name="0x0_validator_set_request_remove_validator"></a>

## Function `request_remove_validator`

Called by <code>ika_system</code>, to remove a validator.
The index of the validator is added to <code>pending_removals</code> and
will be processed at the end of epoch.
Only an active validator can request to be removed.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_request_remove_validator">request_remove_validator</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_request_remove_validator">request_remove_validator</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    cap: &ValidatorCap,
) {
    <b>let</b> validator_id = cap.validator_id();
    <b>assert</b>!(!self.pending_active_validators.contains(&validator_id), <a href="validator_set.md#0x0_validator_set_EValidatorAlreadyRemoved">EValidatorAlreadyRemoved</a>);
    self.pending_active_validators.push_back(validator_id);
}
</code></pre>



</details>

<a name="0x0_validator_set_request_add_stake"></a>

## Function `request_add_stake`

Called by <code>ika_system</code>, to add a new stake to the validator.
This request is added to the validator's staking pool's pending stake entries, processed at the end
of the epoch.
Aborts in case the staking amount is smaller than MIN_STAKING_THRESHOLD


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_request_add_stake">request_add_stake</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, stake: <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="staked_ika.md#0x0_staked_ika_StakedIka">staked_ika::StakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_request_add_stake">request_add_stake</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    validator_id: ID,
    stake: Balance&lt;IKA&gt;,
    ctx: &<b>mut</b> TxContext,
): StakedIka {
    <b>let</b> ika_amount = stake.value();
    <b>assert</b>!(ika_amount &gt;= <a href="validator_set.md#0x0_validator_set_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>, <a href="validator_set.md#0x0_validator_set_EStakingBelowThreshold">EStakingBelowThreshold</a>);
    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = <a href="validator_set.md#0x0_validator_set_get_candidate_or_active_validator_mut">get_candidate_or_active_validator_mut</a>(self, validator_id);
    <a href="validator.md#0x0_validator">validator</a>.<a href="validator_set.md#0x0_validator_set_request_add_stake">request_add_stake</a>(epoch, stake, ctx)
}
</code></pre>



</details>

<a name="0x0_validator_set_request_withdraw_stake"></a>

## Function `request_withdraw_stake`

Called by <code>ika_system</code>, to withdraw some share of a stake from the validator. The share to withdraw
is denoted by <code>principal_withdraw_amount</code>. One of two things occurs in this function:
1. If the <code><a href="staked_ika.md#0x0_staked_ika">staked_ika</a></code> is staked with an active validator, the request is added to the validator's
staking pool's pending stake withdraw entries, processed at the end of the epoch.
2. If the <code><a href="staked_ika.md#0x0_staked_ika">staked_ika</a></code> was staked with a validator that is no longer active,
the stake and any rewards corresponding to it will be immediately processed.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_request_withdraw_stake">request_withdraw_stake</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: <a href="staked_ika.md#0x0_staked_ika_StakedIka">staked_ika::StakedIka</a>): <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_request_withdraw_stake">request_withdraw_stake</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: StakedIka,
): Balance&lt;IKA&gt; {
    <b>let</b> validator_id = <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>.validator_id();
    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.<a href="validator_set.md#0x0_validator_set_get_candidate_or_active_or_inactive_validator_mut">get_candidate_or_active_or_inactive_validator_mut</a>(validator_id);
    <a href="validator.md#0x0_validator">validator</a>.<a href="validator_set.md#0x0_validator_set_request_withdraw_stake">request_withdraw_stake</a>(epoch, <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>)
}
</code></pre>



</details>

<a name="0x0_validator_set_convert_to_fungible_staked_ika"></a>

## Function `convert_to_fungible_staked_ika`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_convert_to_fungible_staked_ika">convert_to_fungible_staked_ika</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: <a href="staked_ika.md#0x0_staked_ika_StakedIka">staked_ika::StakedIka</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_convert_to_fungible_staked_ika">convert_to_fungible_staked_ika</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: StakedIka,
    ctx: &<b>mut</b> TxContext,
): FungibleStakedIka {
    <b>let</b> validator_id = <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>.validator_id();
    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.<a href="validator_set.md#0x0_validator_set_get_candidate_or_active_or_inactive_validator_mut">get_candidate_or_active_or_inactive_validator_mut</a>(validator_id);

    <a href="validator.md#0x0_validator">validator</a>.<a href="validator_set.md#0x0_validator_set_convert_to_fungible_staked_ika">convert_to_fungible_staked_ika</a>(epoch, <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>, ctx)
}
</code></pre>



</details>

<a name="0x0_validator_set_redeem_fungible_staked_ika"></a>

## Function `redeem_fungible_staked_ika`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_redeem_fungible_staked_ika">redeem_fungible_staked_ika</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, fungible_staked_ika: <a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>): <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_redeem_fungible_staked_ika">redeem_fungible_staked_ika</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    fungible_staked_ika: FungibleStakedIka,
): Balance&lt;IKA&gt; {
    <b>let</b> validator_id = fungible_staked_ika.validator_id();
    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.<a href="validator_set.md#0x0_validator_set_get_candidate_or_active_or_inactive_validator_mut">get_candidate_or_active_or_inactive_validator_mut</a>(validator_id);

    <a href="validator.md#0x0_validator">validator</a>.<a href="validator_set.md#0x0_validator_set_redeem_fungible_staked_ika">redeem_fungible_staked_ika</a>(epoch, fungible_staked_ika)
}
</code></pre>



</details>

<a name="0x0_validator_set_request_set_commission_rate"></a>

## Function `request_set_commission_rate`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_request_set_commission_rate">request_set_commission_rate</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, new_commission_rate: u16, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_request_set_commission_rate">request_set_commission_rate</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    new_commission_rate: u16,
    cap: &ValidatorCap,
) {
    <b>let</b> validator_id = cap.validator_id();
    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.<a href="validator_set.md#0x0_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
    <a href="validator.md#0x0_validator">validator</a>.<a href="validator_set.md#0x0_validator_set_request_set_commission_rate">request_set_commission_rate</a>(new_commission_rate);
}
</code></pre>



</details>

<a name="0x0_validator_set_process_mid_epoch"></a>

## Function `process_mid_epoch`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_process_mid_epoch">process_mid_epoch</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, lock_active_committee: bool, low_stake_threshold: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, very_low_stake_threshold: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, low_stake_grace_period: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_process_mid_epoch">process_mid_epoch</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    lock_active_committee: bool,
    low_stake_threshold: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    very_low_stake_threshold: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    low_stake_grace_period: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
) {
    <b>assert</b>!(self.next_epoch_active_committee.is_none(), <a href="validator_set.md#0x0_validator_set_EProcessMidEpochOnlyAfterAdvanceEpoch">EProcessMidEpochOnlyAfterAdvanceEpoch</a>);
    <b>let</b> new_epoch = epoch + 1;

    <b>if</b> (lock_active_committee) {
        // <b>if</b> we lock the <a href="committee.md#0x0_committee">committee</a> just keep it the same <b>as</b> last time
        self.next_epoch_active_committee.fill(self.active_committee)
    } <b>else</b> {
        // kick low stake validators out.
        self.<a href="validator_set.md#0x0_validator_set_update_and_process_low_stake_departures">update_and_process_low_stake_departures</a>(
            new_epoch,
            low_stake_threshold,
            very_low_stake_threshold,
            low_stake_grace_period,
        );

        // Note that all their staged next epoch metadata will be effectuated during next `advance_epoch`.
        self.<a href="validator_set.md#0x0_validator_set_process_pending_validators">process_pending_validators</a>();
    };
}
</code></pre>



</details>

<a name="0x0_validator_set_advance_epoch"></a>

## Function `advance_epoch`

Update the validator set at the end of epoch.
It does the following things:
1. Distribute stake award.
2. Process pending stake deposits and withdraws for each validator (<code>adjust_stake</code>).
3. Process pending stake deposits, and withdraws.
4. Process pending validator application and withdraws.
5. At the end, we calculate the total stake for the new epoch.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_advance_epoch">advance_epoch</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, new_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, total_reward: &<b>mut</b> <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;, reward_slashing_rate: u16, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_advance_epoch">advance_epoch</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    new_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    total_reward: &<b>mut</b> Balance&lt;IKA&gt;,
    reward_slashing_rate: u16,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(self.next_epoch_active_committee.is_some(), <a href="validator_set.md#0x0_validator_set_EAdvanceEpochOnlyAfterProcessMidEpoch">EAdvanceEpochOnlyAfterProcessMidEpoch</a>);

    <b>let</b> total_voting_power = total_voting_power();

    // Compute the reward distribution without taking into account the tallying rule slashing.
    <b>let</b> unadjusted_staking_reward_amounts = self.<a href="validator_set.md#0x0_validator_set_compute_unadjusted_reward_distribution">compute_unadjusted_reward_distribution</a>(
        total_voting_power,
        total_reward.value(),
    );

    // Use the tallying rule report records for the epoch <b>to</b> compute validators that will be
    // punished.
    <b>let</b> slashed_validators = self.<a href="validator_set.md#0x0_validator_set_compute_slashed_validators">compute_slashed_validators</a>();


    <b>let</b> total_slashed_validator_voting_power = self.<a href="validator_set.md#0x0_validator_set_sum_voting_power_by_validator_indices">sum_voting_power_by_validator_indices</a>(
        slashed_validators,
    );

    <b>let</b> slashed_validator_indices = self.<a href="validator_set.md#0x0_validator_set_get_validator_indices">get_validator_indices</a>(&slashed_validators);

    // Compute the reward adjustments of slashed validators, <b>to</b> be taken into
    // account in adjusted reward computation.
    <b>let</b> (
        total_staking_reward_adjustment,
        individual_staking_reward_adjustments,
    ) = <a href="validator_set.md#0x0_validator_set_compute_reward_adjustments">compute_reward_adjustments</a>(
        slashed_validator_indices,
        reward_slashing_rate,
        &unadjusted_staking_reward_amounts,
    );

    // Compute the adjusted amounts of stake each <a href="validator.md#0x0_validator">validator</a> should get given the tallying rule
    // reward adjustments we computed before.
    // `compute_adjusted_reward_distribution` must be called before `distribute_reward` and `adjust_stake_and_computation_price` <b>to</b>
    // make sure we are using the current epoch's stake information <b>to</b> compute reward distribution.
    <b>let</b> (
        adjusted_staking_reward_amounts,
    ) = self.<a href="validator_set.md#0x0_validator_set_compute_adjusted_reward_distribution">compute_adjusted_reward_distribution</a>(
        total_voting_power,
        total_slashed_validator_voting_power,
        unadjusted_staking_reward_amounts,
        total_staking_reward_adjustment,
        individual_staking_reward_adjustments
    );

    // Distribute the rewards before adjusting stake so that we immediately start compounding
    // the rewards for validators and stakers.
    self.<a href="validator_set.md#0x0_validator_set_distribute_reward">distribute_reward</a>(
        epoch,
        &adjusted_staking_reward_amounts,
        total_reward,
        ctx,
    );

    self.<a href="validator_set.md#0x0_validator_set_adjust_stake_and_computation_price">adjust_stake_and_computation_price</a>();

    self.<a href="validator_set.md#0x0_validator_set_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>(new_epoch);

    // Change <b>to</b> the next <a href="validator.md#0x0_validator">validator</a> <a href="committee.md#0x0_committee">committee</a>
    self.active_committee = self.next_epoch_active_committee.extract();

    // Activate validators that were added during `process_mid_epoch`
    self.<a href="validator_set.md#0x0_validator_set_activate_added_validators">activate_added_validators</a>(new_epoch);

    // Emit events after we have processed all the rewards distribution and pending stakes.
    self.<a href="validator_set.md#0x0_validator_set_emit_validator_epoch_events">emit_validator_epoch_events</a>(
        new_epoch,
        &adjusted_staking_reward_amounts,
        &slashed_validators,
    );

    self.total_stake = self.<a href="validator_set.md#0x0_validator_set_calculate_total_stakes">calculate_total_stakes</a>();

    // At this point, self.active_committee are updated for next epoch.
    // Now we process the staged <a href="validator.md#0x0_validator">validator</a> metadata.
    self.<a href="validator_set.md#0x0_validator_set_effectuate_staged_metadata">effectuate_staged_metadata</a>();
}
</code></pre>



</details>

<a name="0x0_validator_set_activate_added_validators"></a>

## Function `activate_added_validators`



<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_activate_added_validators">activate_added_validators</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, new_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_activate_added_validators">activate_added_validators</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    new_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
) {
    <b>let</b> members = *self.active_committee.members();
    members.do!(|member| {
        <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.<a href="validator_set.md#0x0_validator_set_get_validator_mut">get_validator_mut</a>(member.validator_id());
        <b>if</b>(<a href="validator.md#0x0_validator">validator</a>.is_candidate()) {
            <a href="validator.md#0x0_validator">validator</a>.activate(new_epoch);
            <a href="../sui-framework/event.md#0x2_event_emit">event::emit</a>(<a href="validator_set.md#0x0_validator_set_ValidatorJoinEvent">ValidatorJoinEvent</a> {
                epoch: new_epoch,
                validator_id: <a href="validator.md#0x0_validator">validator</a>.validator_id(),
            });
        };
    });
}
</code></pre>



</details>

<a name="0x0_validator_set_update_and_process_low_stake_departures"></a>

## Function `update_and_process_low_stake_departures`



<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_update_and_process_low_stake_departures">update_and_process_low_stake_departures</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, new_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, low_stake_threshold: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, very_low_stake_threshold: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, low_stake_grace_period: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_update_and_process_low_stake_departures">update_and_process_low_stake_departures</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    new_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    low_stake_threshold: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    very_low_stake_threshold: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    low_stake_grace_period: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
) {
    <b>let</b> pending_active_validators = self.pending_active_validators;
    // Iterate through all the active validators, record their low stake status, and kick them out <b>if</b> the condition is met.
    <b>let</b> <b>mut</b> i = pending_active_validators.length();
    <b>while</b> (i &gt; 0) {
        i = i - 1;
        <b>let</b> validator_id = pending_active_validators[i];

        <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.<a href="validator_set.md#0x0_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
        <b>let</b> stake = <a href="validator.md#0x0_validator">validator</a>.total_stake_amount();
        <b>if</b> (stake &gt;= low_stake_threshold) {
            // The <a href="validator.md#0x0_validator">validator</a> is safe. We remove their entry from the at_risk map <b>if</b> there exists one.
            <b>if</b> (self.at_risk_validators.contains(&validator_id)) {
                self.at_risk_validators.remove(&validator_id);
            }
        } <b>else</b> <b>if</b> (stake &gt;= very_low_stake_threshold) {
            // The stake is a bit below the threshold so we increment the entry of the <a href="validator.md#0x0_validator">validator</a> in the map.
            <b>let</b> new_low_stake_period = <b>if</b> (self.at_risk_validators.contains(&validator_id)) {
                <b>let</b> num_epochs = &<b>mut</b> self.at_risk_validators[&validator_id];
                *num_epochs = *num_epochs + 1;
                *num_epochs
            } <b>else</b> {
                self.at_risk_validators.insert(validator_id, 1);
                1
            };

            // If the grace period <b>has</b> passed, the <a href="validator.md#0x0_validator">validator</a> <b>has</b> <b>to</b> leave us.
            <b>if</b> (new_low_stake_period &gt; low_stake_grace_period) {
                <b>let</b> _ = self.pending_active_validators.remove(i);
                <a href="validator_set.md#0x0_validator_set_process_validator_departure">process_validator_departure</a>(
                    self,
                    new_epoch,
                    validator_id,
                    <b>false</b>, /* the <a href="validator.md#0x0_validator">validator</a> is kicked out involuntarily */
                );
            }
        } <b>else</b> {
            // The <a href="validator.md#0x0_validator">validator</a>'s stake is lower than the very low threshold so we kick them out immediately.
            <b>let</b> _ = self.pending_active_validators.remove(i);
            <a href="validator_set.md#0x0_validator_set_process_validator_departure">process_validator_departure</a>(
                self,
                new_epoch,
                validator_id,
                <b>false</b>, /* the <a href="validator.md#0x0_validator">validator</a> is kicked out involuntarily */
            );
        }
    }
}
</code></pre>



</details>

<a name="0x0_validator_set_effectuate_staged_metadata"></a>

## Function `effectuate_staged_metadata`

Effectutate pending next epoch metadata if they are staged.


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_effectuate_staged_metadata">effectuate_staged_metadata</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_effectuate_staged_metadata">effectuate_staged_metadata</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>) {
    <b>let</b> members = *self.active_committee.members();
    members.do!(|member| {
        <b>let</b> validator_id = member.validator_id();
        <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.<a href="validator_set.md#0x0_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
        <a href="validator.md#0x0_validator">validator</a>.<a href="validator_set.md#0x0_validator_set_effectuate_staged_metadata">effectuate_staged_metadata</a>();
    });
}
</code></pre>



</details>

<a name="0x0_validator_set_derive_computation_price_per_unit_size"></a>

## Function `derive_computation_price_per_unit_size`

Called by <code>ika_system</code> to derive computation price per unit size for the new epoch.
Derive the computation price per unit size based on the computation price quote submitted by each validator.
The returned computation price should be greater than or equal to 2/3 of the validators submitted
computation price, weighted by stake.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_derive_computation_price_per_unit_size">derive_computation_price_per_unit_size</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, <a href="committee.md#0x0_committee">committee</a>: &<a href="committee.md#0x0_committee_Committee">committee::Committee</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_derive_computation_price_per_unit_size">derive_computation_price_per_unit_size</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>, <a href="committee.md#0x0_committee">committee</a>: &Committee): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <b>let</b> vs = <a href="committee.md#0x0_committee">committee</a>.members();
    <b>let</b> num_validators = vs.length();
    <b>let</b> <b>mut</b> entries = <a href="../move-stdlib/vector.md#0x1_vector">vector</a>[];
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; num_validators) {
        <b>let</b> vid = vs[i].validator_id();

        <b>let</b> v = self.<a href="validator_set.md#0x0_validator_set_get_validator_ref">get_validator_ref</a>(vid);
        entries.push_back(
            pq::new_entry(v.computation_price(), vs[i].voting_power()),
        );
        i = i + 1;
    };
    // Build a priority queue that will pop entries <b>with</b> computation price from the highest <b>to</b> the lowest.
    <b>let</b> <b>mut</b> pq = pq::new(entries);
    <b>let</b> <b>mut</b> sum = 0;
    <b>let</b> threshold = total_voting_power() - quorum_threshold();
    <b>let</b> <b>mut</b> result = 0;
    <b>while</b> (sum &lt; threshold) {
        <b>let</b> (computation_price, voting_power) = pq.pop_max();
        result = computation_price;
        sum = sum + voting_power;
    };
    result
}
</code></pre>



</details>

<a name="0x0_validator_set_total_stake"></a>

## Function `total_stake`



<pre><code><b>public</b> <b>fun</b> <a href="validator_set.md#0x0_validator_set_total_stake">total_stake</a>(self: &<a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_set.md#0x0_validator_set_total_stake">total_stake</a>(self: &<a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    self.total_stake
}
</code></pre>



</details>

<a name="0x0_validator_set_validator_total_stake_amount"></a>

## Function `validator_total_stake_amount`



<pre><code><b>public</b> <b>fun</b> <a href="validator_set.md#0x0_validator_set_validator_total_stake_amount">validator_total_stake_amount</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_set.md#0x0_validator_set_validator_total_stake_amount">validator_total_stake_amount</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>, validator_id: ID): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = <a href="validator_set.md#0x0_validator_set_get_validator_ref">get_validator_ref</a>(self, validator_id);
    <a href="validator.md#0x0_validator">validator</a>.total_stake_amount()
}
</code></pre>



</details>

<a name="0x0_validator_set_pool_exchange_rates"></a>

## Function `pool_exchange_rates`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_pool_exchange_rates">pool_exchange_rates</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>): &<a href="../sui-framework/table.md#0x2_table_Table">table::Table</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="staking_pool.md#0x0_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_pool_exchange_rates">pool_exchange_rates</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    validator_id: ID,
): &Table&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, PoolTokenExchangeRate&gt; {
    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.<a href="validator_set.md#0x0_validator_set_get_validator_ref">get_validator_ref</a>(validator_id);
    <a href="validator.md#0x0_validator">validator</a>.get_staking_pool_ref().exchange_rates()
}
</code></pre>



</details>

<a name="0x0_validator_set_pending_active_validators_count"></a>

## Function `pending_active_validators_count`

Get the total number of pending validators.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_pending_active_validators_count">pending_active_validators_count</a>(self: &<a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_pending_active_validators_count">pending_active_validators_count</a>(self: &<a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    self.pending_active_validators.length()
}
</code></pre>



</details>

<a name="0x0_validator_set_is_active_validator"></a>

## Function `is_active_validator`

Returns true if exists in active validators.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_is_active_validator">is_active_validator</a>(self: &<a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_is_active_validator">is_active_validator</a>(
    self: &<a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    validator_id: ID,
): bool {
    self.active_committee.contains(&validator_id)
}
</code></pre>



</details>

<a name="0x0_validator_set_get_reporters_of"></a>

## Function `get_reporters_of`

Returns all the validators who are currently reporting <code>validator_id</code>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_get_reporters_of">get_reporters_of</a>(self: &<a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>): <a href="../sui-framework/vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_get_reporters_of">get_reporters_of</a>(self: &<a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>, validator_id: ID): VecSet&lt;ID&gt; {
    <b>if</b> (self.validator_report_records.contains(&validator_id)) {
        self.validator_report_records[&validator_id]
    } <b>else</b> {
        <a href="../sui-framework/vec_set.md#0x2_vec_set_empty">vec_set::empty</a>()
    }
}
</code></pre>



</details>

<a name="0x0_validator_set_count_duplicates_vec"></a>

## Function `count_duplicates_vec`



<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_count_duplicates_vec">count_duplicates_vec</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, validators: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>&gt;, <a href="validator.md#0x0_validator">validator</a>: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_count_duplicates_vec">count_duplicates_vec</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    validators: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;ID&gt;,
    <a href="validator.md#0x0_validator">validator</a>: &ValidatorInnerV1
): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <b>let</b> len = validators.length();
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> <b>mut</b> result = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> vid = validators[i];
        <b>let</b> v = self.<a href="validator_set.md#0x0_validator_set_get_validator_mut">get_validator_mut</a>(vid);
        <b>if</b> (v.is_duplicate(<a href="validator.md#0x0_validator">validator</a>)) {
            result = result + 1;
        };
        i = i + 1;
    };
    result
}
</code></pre>



</details>

<a name="0x0_validator_set_is_duplicate_validator"></a>

## Function `is_duplicate_validator`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_is_duplicate_validator">is_duplicate_validator</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, validators: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>&gt;, new_validator: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_is_duplicate_validator">is_duplicate_validator</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    validators: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;ID&gt;,
    new_validator: &ValidatorInnerV1,
): bool {
    <a href="validator_set.md#0x0_validator_set_count_duplicates_vec">count_duplicates_vec</a>(self, validators, new_validator) &gt; 0
}
</code></pre>



</details>

<a name="0x0_validator_set_is_duplicate_with_active_validator"></a>

## Function `is_duplicate_with_active_validator`

Checks whether <code>new_validator</code> is duplicate with any currently active validators.
It differs from <code>is_active_validator</code> in that the former checks
only the id but this function looks at more metadata.


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_is_duplicate_with_active_validator">is_duplicate_with_active_validator</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, new_validator: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_is_duplicate_with_active_validator">is_duplicate_with_active_validator</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>, new_validator: &ValidatorInnerV1): bool {
    <b>let</b> active_validator_ids = self.active_committee.validator_ids();
    <a href="validator_set.md#0x0_validator_set_is_duplicate_validator">is_duplicate_validator</a>(self, &active_validator_ids, new_validator)
}
</code></pre>



</details>

<a name="0x0_validator_set_is_duplicate_with_next_epoch_active_committee"></a>

## Function `is_duplicate_with_next_epoch_active_committee`

Checks whether <code>new_validator</code> is duplicate with any next epoch active validators.


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_is_duplicate_with_next_epoch_active_committee">is_duplicate_with_next_epoch_active_committee</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, new_validator: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_is_duplicate_with_next_epoch_active_committee">is_duplicate_with_next_epoch_active_committee</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>, new_validator: &ValidatorInnerV1): bool {
    <b>if</b>(self.next_epoch_active_committee.is_none()) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> next_epoch_active_validator_ids = self.next_epoch_active_committee.borrow().validator_ids();
    <a href="validator_set.md#0x0_validator_set_count_duplicates_vec">count_duplicates_vec</a>(self, &next_epoch_active_validator_ids, new_validator) &gt; 0
}
</code></pre>



</details>

<a name="0x0_validator_set_is_duplicate_with_pending_validator"></a>

## Function `is_duplicate_with_pending_validator`

Checks whether <code>new_validator</code> is duplicate with any currently pending validators.


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_is_duplicate_with_pending_validator">is_duplicate_with_pending_validator</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, new_validator: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_is_duplicate_with_pending_validator">is_duplicate_with_pending_validator</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>, new_validator: &ValidatorInnerV1): bool {
    <b>let</b> pending_active_validators = self.pending_active_validators;
    <a href="validator_set.md#0x0_validator_set_count_duplicates_vec">count_duplicates_vec</a>(self, &pending_active_validators, new_validator) &gt; 0
}
</code></pre>



</details>

<a name="0x0_validator_set_get_validator_mut"></a>

## Function `get_validator_mut`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_get_validator_mut">get_validator_mut</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>): &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_get_validator_mut">get_validator_mut</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    validator_id: ID,
): &<b>mut</b> ValidatorInnerV1 {
    <b>assert</b>!(self.validators.contains(validator_id), <a href="validator_set.md#0x0_validator_set_ENotAValidator">ENotAValidator</a>);
    self.validators.borrow_mut(validator_id).load_validator_maybe_upgrade()
}
</code></pre>



</details>

<a name="0x0_validator_set_get_validator_ref"></a>

## Function `get_validator_ref`



<pre><code><b>public</b> <b>fun</b> <a href="validator_set.md#0x0_validator_set_get_validator_ref">get_validator_ref</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>): &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_set.md#0x0_validator_set_get_validator_ref">get_validator_ref</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>, validator_id: ID): &ValidatorInnerV1 {
    self.<a href="validator_set.md#0x0_validator_set_get_validator_mut">get_validator_mut</a>(validator_id)
}
</code></pre>



</details>

<a name="0x0_validator_set_get_candidate_or_active_validator_mut"></a>

## Function `get_candidate_or_active_validator_mut`

Get mutable reference to either a candidate or an active validator by id.


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_get_candidate_or_active_validator_mut">get_candidate_or_active_validator_mut</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>): &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_get_candidate_or_active_validator_mut">get_candidate_or_active_validator_mut</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    validator_id: ID,
): &<b>mut</b> ValidatorInnerV1 {
    <b>let</b> is_active_validator = self.<a href="validator_set.md#0x0_validator_set_is_active_validator">is_active_validator</a>(validator_id);
    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.<a href="validator_set.md#0x0_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
    <b>assert</b>!(<a href="validator.md#0x0_validator">validator</a>.is_candidate() || is_active_validator, <a href="validator_set.md#0x0_validator_set_ENotCandidateOrActiveValidator">ENotCandidateOrActiveValidator</a>);
    <a href="validator.md#0x0_validator">validator</a>
}
</code></pre>



</details>

<a name="0x0_validator_set_get_candidate_or_active_or_inactive_validator_mut"></a>

## Function `get_candidate_or_active_or_inactive_validator_mut`

Get mutable reference to either a candidate or an active or an inactive validator by id.


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_get_candidate_or_active_or_inactive_validator_mut">get_candidate_or_active_or_inactive_validator_mut</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>): &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_get_candidate_or_active_or_inactive_validator_mut">get_candidate_or_active_or_inactive_validator_mut</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    validator_id: ID,
): &<b>mut</b> ValidatorInnerV1 {
    <b>let</b> is_active_validator = self.<a href="validator_set.md#0x0_validator_set_is_active_validator">is_active_validator</a>(validator_id);
    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.<a href="validator_set.md#0x0_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
    <b>assert</b>!(<a href="validator.md#0x0_validator">validator</a>.is_candidate() || <a href="validator.md#0x0_validator">validator</a>.is_inactive() || is_active_validator, <a href="validator_set.md#0x0_validator_set_ENotCandidateOrActiveOrInactiveValidator">ENotCandidateOrActiveOrInactiveValidator</a>);
    <a href="validator.md#0x0_validator">validator</a>
}
</code></pre>



</details>

<a name="0x0_validator_set_get_active_or_pending_validator_mut"></a>

## Function `get_active_or_pending_validator_mut`

Get mutable reference to an active or (if active does not exist) pending or (if pending and
active do not exist) by id.
Note: this function should be called carefully, only after verifying the transaction
sender has the ability to modify the <code>Validator</code>.


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_get_active_or_pending_validator_mut">get_active_or_pending_validator_mut</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>): &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_get_active_or_pending_validator_mut">get_active_or_pending_validator_mut</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    validator_id: ID,
): &<b>mut</b> ValidatorInnerV1 {
    <b>assert</b>!(self.active_committee.contains(&validator_id) || self.pending_active_validators.contains(&validator_id), <a href="validator_set.md#0x0_validator_set_ENotActiveOrPendingValidator">ENotActiveOrPendingValidator</a>);
    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.<a href="validator_set.md#0x0_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
    <a href="validator.md#0x0_validator">validator</a>
}
</code></pre>



</details>

<a name="0x0_validator_set_get_active_or_pending_or_candidate_validator_mut"></a>

## Function `get_active_or_pending_or_candidate_validator_mut`

Get mutable reference to an active or (if active does not exist) pending or (if pending and
active do not exist) or candidate validator by id.
Note: this function should be called carefully, only after verifying the transaction
sender has the ability to modify the <code>Validator</code>.


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_get_active_or_pending_or_candidate_validator_mut">get_active_or_pending_or_candidate_validator_mut</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>): &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_get_active_or_pending_or_candidate_validator_mut">get_active_or_pending_or_candidate_validator_mut</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    validator_id: ID,
): &<b>mut</b> ValidatorInnerV1 {
    <b>let</b> is_active_validator = self.<a href="validator_set.md#0x0_validator_set_is_active_validator">is_active_validator</a>(validator_id);
    <b>let</b> is_pending_active_validator = self.pending_active_validators.contains(&validator_id);

    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.<a href="validator_set.md#0x0_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
    <b>assert</b>!(is_active_validator || is_pending_active_validator || <a href="validator.md#0x0_validator">validator</a>.is_candidate(), <a href="validator_set.md#0x0_validator_set_ENotCandidateOrActiveOrPendingValidator">ENotCandidateOrActiveOrPendingValidator</a>);
    <a href="validator.md#0x0_validator">validator</a>
}
</code></pre>



</details>

<a name="0x0_validator_set_get_validator_mut_with_operation_cap"></a>

## Function `get_validator_mut_with_operation_cap`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_get_validator_mut_with_operation_cap">get_validator_mut_with_operation_cap</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, operation_cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>): &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_get_validator_mut_with_operation_cap">get_validator_mut_with_operation_cap</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    operation_cap: &ValidatorOperationCap,
): &<b>mut</b> ValidatorInnerV1 {
    <b>let</b> validator_id = operation_cap.validator_id();
    self.<a href="validator_set.md#0x0_validator_set_get_active_or_pending_validator_mut">get_active_or_pending_validator_mut</a>(validator_id)

}
</code></pre>



</details>

<a name="0x0_validator_set_get_validator_mut_with_operation_cap_including_candidates"></a>

## Function `get_validator_mut_with_operation_cap_including_candidates`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_get_validator_mut_with_operation_cap_including_candidates">get_validator_mut_with_operation_cap_including_candidates</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, operation_cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>): &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_get_validator_mut_with_operation_cap_including_candidates">get_validator_mut_with_operation_cap_including_candidates</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    operation_cap: &ValidatorOperationCap,
): &<b>mut</b> ValidatorInnerV1 {
    <b>let</b> validator_id = operation_cap.validator_id();
    self.<a href="validator_set.md#0x0_validator_set_get_active_or_pending_or_candidate_validator_mut">get_active_or_pending_or_candidate_validator_mut</a>(validator_id)
}
</code></pre>



</details>

<a name="0x0_validator_set_get_validator_mut_with_cap"></a>

## Function `get_validator_mut_with_cap`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_get_validator_mut_with_cap">get_validator_mut_with_cap</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>): &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_get_validator_mut_with_cap">get_validator_mut_with_cap</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    cap: &ValidatorCap,
): &<b>mut</b> ValidatorInnerV1 {
    <b>let</b> validator_id = cap.validator_id();
    self.<a href="validator_set.md#0x0_validator_set_get_active_or_pending_validator_mut">get_active_or_pending_validator_mut</a>(validator_id)
}
</code></pre>



</details>

<a name="0x0_validator_set_get_validator_mut_with_cap_including_candidates"></a>

## Function `get_validator_mut_with_cap_including_candidates`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_get_validator_mut_with_cap_including_candidates">get_validator_mut_with_cap_including_candidates</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>): &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_get_validator_mut_with_cap_including_candidates">get_validator_mut_with_cap_including_candidates</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    cap: &ValidatorCap,
): &<b>mut</b> ValidatorInnerV1 {
    <b>let</b> validator_id = cap.validator_id();
    self.<a href="validator_set.md#0x0_validator_set_get_active_or_pending_or_candidate_validator_mut">get_active_or_pending_or_candidate_validator_mut</a>(validator_id)
}
</code></pre>



</details>

<a name="0x0_validator_set_get_validator_indices"></a>

## Function `get_validator_indices`

Given a vector of validator ids to look for, return their indices in the validator vector.
Aborts if any id isn't in the given validator vector.


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_get_validator_indices">get_validator_indices</a>(self: &<a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, look_for_indices_ids: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>&gt;): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_get_validator_indices">get_validator_indices</a>(
    self: &<a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    look_for_indices_ids: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;ID&gt;,
): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt; {
    <b>let</b> validators = self.active_committee.validator_ids();
    <b>let</b> length = look_for_indices_ids.length();
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> <b>mut</b> res = <a href="../move-stdlib/vector.md#0x1_vector">vector</a>[];
    <b>while</b> (i &lt; length) {
        <b>let</b> validator_id = look_for_indices_ids[i];
        <b>let</b> (found, index_opt) = validators.index_of(&validator_id);
        <b>assert</b>!(found, <a href="validator_set.md#0x0_validator_set_ENotAValidator">ENotAValidator</a>);
        res.push_back(index_opt);
        i = i + 1;
    };
    res
}
</code></pre>



</details>

<a name="0x0_validator_set_verify_operation_cap"></a>

## Function `verify_operation_cap`

Verify the operation capability is valid for a Validator.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_verify_operation_cap">verify_operation_cap</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_verify_operation_cap">verify_operation_cap</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    cap: &ValidatorOperationCap,
) {
    <b>let</b> validator_id = cap.validator_id();
    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.<a href="validator_set.md#0x0_validator_set_get_validator_ref">get_validator_ref</a>(validator_id);
    <b>assert</b>!(<a href="validator.md#0x0_validator">validator</a>.operation_cap_id() == &<a href="../sui-framework/object.md#0x2_object_id">object::id</a>(cap), <a href="validator_set.md#0x0_validator_set_EInvalidCap">EInvalidCap</a>);
}
</code></pre>



</details>

<a name="0x0_validator_set_process_validator_departure"></a>

## Function `process_validator_departure`



<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_process_validator_departure">process_validator_departure</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, new_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, is_voluntary: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_process_validator_departure">process_validator_departure</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    new_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    validator_id: ID,
    is_voluntary: bool,
) {
    <b>if</b> (self.at_risk_validators.contains(&validator_id)) {
        self.at_risk_validators.remove(&validator_id);
    };

    <a href="validator_set.md#0x0_validator_set_clean_report_records_leaving_validator">clean_report_records_leaving_validator</a>(&<b>mut</b> self.validator_report_records, validator_id);

    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.<a href="validator_set.md#0x0_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);

    <b>let</b> validator_stake = <a href="validator.md#0x0_validator">validator</a>.total_stake_amount();

    // Deactivate the <a href="validator.md#0x0_validator">validator</a> and its staking pool
    <a href="validator.md#0x0_validator">validator</a>.deactivate(new_epoch);

    self.total_stake = self.total_stake - validator_stake;

    <a href="../sui-framework/event.md#0x2_event_emit">event::emit</a>(<a href="validator_set.md#0x0_validator_set_ValidatorLeaveEvent">ValidatorLeaveEvent</a> {
        epoch: new_epoch,
        validator_id,
        is_voluntary,
    });
}
</code></pre>



</details>

<a name="0x0_validator_set_clean_report_records_leaving_validator"></a>

## Function `clean_report_records_leaving_validator`



<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_clean_report_records_leaving_validator">clean_report_records_leaving_validator</a>(validator_report_records: &<b>mut</b> <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, <a href="../sui-framework/vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>&gt;&gt;, leaving_validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_clean_report_records_leaving_validator">clean_report_records_leaving_validator</a>(
    validator_report_records: &<b>mut</b> VecMap&lt;ID, VecSet&lt;ID&gt;&gt;,
    leaving_validator_id: ID,
) {
    // Remove the records about this <a href="validator.md#0x0_validator">validator</a>
    <b>if</b> (validator_report_records.contains(&leaving_validator_id)) {
        validator_report_records.remove(&leaving_validator_id);
    };

    // Remove the reports submitted by this <a href="validator.md#0x0_validator">validator</a>
    <b>let</b> reported_validators = validator_report_records.keys();
    <b>let</b> length = reported_validators.length();
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; length) {
        <b>let</b> reported_validator_id = &reported_validators[i];
        <b>let</b> reporters = &<b>mut</b> validator_report_records[reported_validator_id];
        <b>if</b> (reporters.contains(&leaving_validator_id)) {
            reporters.remove(&leaving_validator_id);
            <b>if</b> (reporters.is_empty()) {
                validator_report_records.remove(reported_validator_id);
            };
        };
        i = i + 1;
    }
}
</code></pre>



</details>

<a name="0x0_validator_set_process_pending_validators"></a>

## Function `process_pending_validators`

Process the pending new validators. They will be <code>next_epoch_active_committee</code> and activated during <code>advance_epoch</code>.


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_process_pending_validators">process_pending_validators</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_process_pending_validators">process_pending_validators</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>) {
    <b>let</b> <b>mut</b> next_epoch_active_members = <a href="../move-stdlib/vector.md#0x1_vector">vector</a>[];
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> length = self.pending_active_validators.length();
    <b>while</b> (i &lt; length) {
        <b>let</b> validator_id = self.pending_active_validators[i];
        <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.<a href="validator_set.md#0x0_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
        next_epoch_active_members.push_back(new_committee_member(validator_id, *<a href="validator.md#0x0_validator">validator</a>.protocol_pubkey(), <a href="validator.md#0x0_validator">validator</a>.total_stake_amount()));
        i = i + 1;
    };
    <b>let</b> next_epoch_active_committee = new_committee(next_epoch_active_members);
    self.next_epoch_active_committee.fill(next_epoch_active_committee);
}
</code></pre>



</details>

<a name="0x0_validator_set_process_pending_stakes_and_withdraws"></a>

## Function `process_pending_stakes_and_withdraws`

Process all active validators' pending stake deposits and withdraws.


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, new_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>, new_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>) {
    <b>let</b> members = *self.active_committee.members();
    members.do!(|member| {
        <b>let</b> validator_id = member.validator_id();
        <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.<a href="validator_set.md#0x0_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
        <a href="validator.md#0x0_validator">validator</a>.<a href="validator_set.md#0x0_validator_set_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>(new_epoch);
    });
}
</code></pre>



</details>

<a name="0x0_validator_set_calculate_total_stakes"></a>

## Function `calculate_total_stakes`

Calculate the total active validator stake.


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_calculate_total_stakes">calculate_total_stakes</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_calculate_total_stakes">calculate_total_stakes</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <b>let</b> <b>mut</b> stake = 0;
    <b>let</b> members = *self.active_committee.members();
    members.do!(|member| {
        <b>let</b> validator_id = member.validator_id();
        <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.<a href="validator_set.md#0x0_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
        stake = stake + <a href="validator.md#0x0_validator">validator</a>.total_stake_amount();
    });

    stake
}
</code></pre>



</details>

<a name="0x0_validator_set_adjust_stake_and_computation_price"></a>

## Function `adjust_stake_and_computation_price`

Process the pending stake changes for each validator.


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_adjust_stake_and_computation_price">adjust_stake_and_computation_price</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_adjust_stake_and_computation_price">adjust_stake_and_computation_price</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
) {
    <b>let</b> members = *self.active_committee.members();

    members.do!(|member| {
        <b>let</b> validator_id = member.validator_id();
        <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.<a href="validator_set.md#0x0_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
        <a href="validator.md#0x0_validator">validator</a>.<a href="validator_set.md#0x0_validator_set_adjust_stake_and_computation_price">adjust_stake_and_computation_price</a>();
    });
}
</code></pre>



</details>

<a name="0x0_validator_set_compute_reward_adjustments"></a>

## Function `compute_reward_adjustments`

Compute both the individual reward adjustments and total reward adjustment for staking rewards
as well as storage fund rewards.


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_compute_reward_adjustments">compute_reward_adjustments</a>(slashed_validator_indices: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;, reward_slashing_rate: u16, unadjusted_staking_reward_amounts: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;): (<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_compute_reward_adjustments">compute_reward_adjustments</a>(
    <b>mut</b> slashed_validator_indices: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;,
    reward_slashing_rate: u16,
    unadjusted_staking_reward_amounts: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;,
): (
    <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, // sum of staking reward adjustments
    VecMap&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;, // mapping of individual <a href="validator.md#0x0_validator">validator</a>'s staking reward adjustment from index -&gt; amount
) {
    <b>let</b> <b>mut</b> total_staking_reward_adjustment = 0;
    <b>let</b> <b>mut</b> individual_staking_reward_adjustments = <a href="../sui-framework/vec_map.md#0x2_vec_map_empty">vec_map::empty</a>();

    <b>while</b> (!slashed_validator_indices.is_empty()) {
        <b>let</b> validator_index = slashed_validator_indices.pop_back();

        // Use the slashing rate <b>to</b> compute the amount of staking rewards slashed from this punished <a href="validator.md#0x0_validator">validator</a>.
        <b>let</b> unadjusted_staking_reward = unadjusted_staking_reward_amounts[validator_index];
        <b>let</b> staking_reward_adjustment_u128 =
            unadjusted_staking_reward <b>as</b> u128 * (reward_slashing_rate <b>as</b> u128)
                / <a href="validator_set.md#0x0_validator_set_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a>;

        // Insert into individual mapping and record into the total adjustment sum.
        individual_staking_reward_adjustments.insert(
            validator_index,
            staking_reward_adjustment_u128 <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
        );
        total_staking_reward_adjustment =
            total_staking_reward_adjustment + (staking_reward_adjustment_u128 <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>);

    };

    (
        total_staking_reward_adjustment,
        individual_staking_reward_adjustments,
    )
}
</code></pre>



</details>

<a name="0x0_validator_set_compute_slashed_validators"></a>

## Function `compute_slashed_validators`

Process the validator report records of the epoch and return the ids of the
non-performant validators according to the input threshold.


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_compute_slashed_validators">compute_slashed_validators</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_compute_slashed_validators">compute_slashed_validators</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;ID&gt; {
    <b>let</b> <b>mut</b> slashed_validators = <a href="../move-stdlib/vector.md#0x1_vector">vector</a>[];
    <b>while</b> (!self.validator_report_records.is_empty()) {
        <b>let</b> (validator_id, reporters) = self.validator_report_records.pop();
        <b>assert</b>!(
            <a href="validator_set.md#0x0_validator_set_is_active_validator">is_active_validator</a>(self, validator_id),
            <a href="validator_set.md#0x0_validator_set_ENonValidatorInReportRecords">ENonValidatorInReportRecords</a>,
        );
        // Sum up the voting power of validators that have reported this <a href="validator.md#0x0_validator">validator</a> and check <b>if</b> it <b>has</b>
        // passed the slashing threshold.
        <b>let</b> reporter_votes = <a href="validator_set.md#0x0_validator_set_sum_voting_power_by_validator_indices">sum_voting_power_by_validator_indices</a>(
            self,
            reporters.into_keys(),
        );
        <b>if</b> (reporter_votes &gt;= quorum_threshold()) {
            slashed_validators.push_back(validator_id);
        }
    };
    slashed_validators
}
</code></pre>



</details>

<a name="0x0_validator_set_compute_unadjusted_reward_distribution"></a>

## Function `compute_unadjusted_reward_distribution`

Given the current list of active validators, the total stake and total reward,
calculate the amount of reward each validator should get, without taking into
account the tallying rule results.
Returns the unadjusted amounts of staking reward for each validator.


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_compute_unadjusted_reward_distribution">compute_unadjusted_reward_distribution</a>(self: &<a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, total_voting_power: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, total_reward: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_compute_unadjusted_reward_distribution">compute_unadjusted_reward_distribution</a>(
    self: &<a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    total_voting_power: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    total_reward: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt; {
    <b>let</b> members = self.active_committee.members();
    <b>let</b> reward_amounts = members.map_ref!(|member| {
        // Integer divisions will truncate the results. Because of this, we expect that at the end
        // there will be some reward remaining in `total_reward`.
        // Use u128 <b>to</b> avoid multiplication overflow.
        <b>let</b> voting_power: u128 = member.voting_power() <b>as</b> u128;
        <b>let</b> reward_amount =
            voting_power * (total_reward <b>as</b> u128) / (total_voting_power <b>as</b> u128);
        reward_amount <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
    });
    reward_amounts
}
</code></pre>



</details>

<a name="0x0_validator_set_compute_adjusted_reward_distribution"></a>

## Function `compute_adjusted_reward_distribution`

Use the reward adjustment info to compute the adjusted rewards each validator should get.
Returns the staking rewards each validator gets.
The staking rewards are shared with the stakers.


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_compute_adjusted_reward_distribution">compute_adjusted_reward_distribution</a>(self: &<a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, total_voting_power: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, total_slashed_validator_voting_power: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, unadjusted_staking_reward_amounts: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;, total_staking_reward_adjustment: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, individual_staking_reward_adjustments: <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_compute_adjusted_reward_distribution">compute_adjusted_reward_distribution</a>(
    self: &<a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    total_voting_power: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    total_slashed_validator_voting_power: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    unadjusted_staking_reward_amounts: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;,
    total_staking_reward_adjustment: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    individual_staking_reward_adjustments: VecMap&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;,
): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt; {
    <b>let</b> total_unslashed_validator_voting_power =
        total_voting_power - total_slashed_validator_voting_power;
    <b>let</b> <b>mut</b> adjusted_staking_reward_amounts = <a href="../move-stdlib/vector.md#0x1_vector">vector</a>[];
    <b>let</b> members = self.active_committee.members();
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> length = members.length();

    <b>while</b> (i &lt; length) {
        // Integer divisions will truncate the results. Because of this, we expect that at the end
        // there will be some reward remaining in `total_reward`.
        // Use u128 <b>to</b> avoid multiplication overflow.
        <b>let</b> voting_power = members[i].voting_power() <b>as</b> u128;

        // Compute adjusted staking reward.
        <b>let</b> unadjusted_staking_reward_amount = unadjusted_staking_reward_amounts[i];
        <b>let</b> adjusted_staking_reward_amount = // If the <a href="validator.md#0x0_validator">validator</a> is one of the slashed ones, then subtract the adjustment.
        <b>if</b> (individual_staking_reward_adjustments.contains(&i)) {
            <b>let</b> adjustment = individual_staking_reward_adjustments[&i];
            unadjusted_staking_reward_amount - adjustment
        } <b>else</b> {
            // Otherwise the slashed rewards should be distributed among the unslashed
            // validators so add the corresponding adjustment.
            <b>let</b> adjustment =
                total_staking_reward_adjustment <b>as</b> u128 * voting_power
                                   / (total_unslashed_validator_voting_power <b>as</b> u128);
            unadjusted_staking_reward_amount + (adjustment <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
        };
        adjusted_staking_reward_amounts.push_back(adjusted_staking_reward_amount);
        i = i + 1;
    };
    adjusted_staking_reward_amounts
}
</code></pre>



</details>

<a name="0x0_validator_set_distribute_reward"></a>

## Function `distribute_reward`



<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_distribute_reward">distribute_reward</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, adjusted_staking_reward_amounts: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;, staking_rewards: &<b>mut</b> <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_distribute_reward">distribute_reward</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    adjusted_staking_reward_amounts: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;,
    staking_rewards: &<b>mut</b> Balance&lt;IKA&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> members = *self.active_committee.members();
    <b>let</b> length = members.length();
    <b>assert</b>!(length &gt; 0, <a href="validator_set.md#0x0_validator_set_EValidatorSetEmpty">EValidatorSetEmpty</a>);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; length) {
        <b>let</b> validator_id = members[i].validator_id();
        <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.<a href="validator_set.md#0x0_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
        <b>let</b> staking_reward_amount = adjusted_staking_reward_amounts[i];
        <b>let</b> <b>mut</b> staker_reward = staking_rewards.split(staking_reward_amount);

        // Validator takes a cut of the rewards <b>as</b> commission.
        <b>let</b> validator_commission_amount =
            (staking_reward_amount <b>as</b> u128) * (<a href="validator.md#0x0_validator">validator</a>.commission_rate() <b>as</b> u128) / <a href="validator_set.md#0x0_validator_set_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a>;

        // The <a href="validator.md#0x0_validator">validator</a> reward commission.
        <b>let</b> validator_reward = staker_reward.split(validator_commission_amount <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>);

        // Add rewards <b>to</b> the <a href="validator.md#0x0_validator">validator</a>. Don't try and distribute rewards though <b>if</b> the payout is zero.
        <b>if</b> (validator_reward.value() &gt; 0) {
            <b>let</b> validator_address = <a href="validator.md#0x0_validator">validator</a>.payment_address();
            <b>let</b> rewards_stake = <a href="validator.md#0x0_validator">validator</a>.<a href="validator_set.md#0x0_validator_set_request_add_stake">request_add_stake</a>(
                epoch,
                validator_reward,
                ctx,
            );
            <a href="../sui-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(rewards_stake, validator_address);
        } <b>else</b> {
            validator_reward.destroy_zero();
        };

        // Add rewards <b>to</b> stake staking pool <b>to</b> auto compound for stakers.
        <a href="validator.md#0x0_validator">validator</a>.deposit_stake_rewards(staker_reward);
        i = i + 1;
    }
}
</code></pre>



</details>

<a name="0x0_validator_set_emit_validator_epoch_events"></a>

## Function `emit_validator_epoch_events`

Emit events containing information of each validator for the epoch,
including stakes, rewards, performance, etc.


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_emit_validator_epoch_events">emit_validator_epoch_events</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, new_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, pool_staking_reward_amounts: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;, slashed_validators: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_emit_validator_epoch_events">emit_validator_epoch_events</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    new_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    pool_staking_reward_amounts: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;,
    slashed_validators: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;ID&gt;,
) {
    <b>let</b> members = *self.active_committee.members();
    <b>let</b> num_validators = members.length();
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; num_validators) {
        <b>let</b> member = members[i];
        <b>let</b> validator_id = member.validator_id();
        <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.<a href="validator_set.md#0x0_validator_set_get_validator_ref">get_validator_ref</a>(validator_id);
        <b>let</b> tallying_rule_reporters = <b>if</b> (self.validator_report_records.contains(&validator_id)) {
            self.validator_report_records[&validator_id].into_keys()
        } <b>else</b> {
            <a href="../move-stdlib/vector.md#0x1_vector">vector</a>[]
        };
        <b>let</b> tallying_rule_global_score = <b>if</b> (slashed_validators.contains(&validator_id)) 0
        <b>else</b> 1;
        <a href="../sui-framework/event.md#0x2_event_emit">event::emit</a>(<a href="validator_set.md#0x0_validator_set_ValidatorEpochInfoEventV1">ValidatorEpochInfoEventV1</a> {
            epoch: new_epoch,
            validator_id,
            reference_gas_survey_quote: <a href="validator.md#0x0_validator">validator</a>.computation_price(),
            stake: <a href="validator.md#0x0_validator">validator</a>.total_stake_amount(),
            voting_power: member.voting_power(),
            commission_rate: <a href="validator.md#0x0_validator">validator</a>.commission_rate(),
            pool_staking_reward: pool_staking_reward_amounts[i],
            pool_token_exchange_rate: <a href="validator.md#0x0_validator">validator</a>.pool_token_exchange_rate_at_epoch(new_epoch),
            tallying_rule_reporters,
            tallying_rule_global_score,
        });
        i = i + 1;
    }
}
</code></pre>



</details>

<a name="0x0_validator_set_sum_voting_power_by_validator_indices"></a>

## Function `sum_voting_power_by_validator_indices`

Sum up the total stake of a given list of validator indices.


<pre><code><b>public</b> <b>fun</b> <a href="validator_set.md#0x0_validator_set_sum_voting_power_by_validator_indices">sum_voting_power_by_validator_indices</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, validator_ids: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_set.md#0x0_validator_set_sum_voting_power_by_validator_indices">sum_voting_power_by_validator_indices</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>, validator_ids: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;ID&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <b>let</b> validator_indices = <a href="validator_set.md#0x0_validator_set_get_validator_indices">get_validator_indices</a>(self, &validator_ids);
    <b>let</b> members = self.active_committee.members();
    <b>let</b> sum = validator_indices.fold!(0, |s, i|  {
        s + members[i].voting_power()
    });
    sum
}
</code></pre>



</details>

<a name="0x0_validator_set_report_validator"></a>

## Function `report_validator`

Report a validator as a bad or non-performant actor in the system.
Succeeds if all the following are satisfied:
1. both the reporter in <code>cap</code> and the input <code>reportee_id</code> are active validators.
2. reporter and reportee not the same address.
3. the cap object is still valid.
This function is idempotent.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_report_validator">report_validator</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, operation_cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, reportee_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_report_validator">report_validator</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    operation_cap: &ValidatorOperationCap,
    reportee_id: ID,
) {
    // Reportee needs <b>to</b> be an active <a href="validator.md#0x0_validator">validator</a>
    <b>assert</b>!(self.<a href="validator_set.md#0x0_validator_set_is_active_validator">is_active_validator</a>(reportee_id), <a href="validator_set.md#0x0_validator_set_ENotAValidator">ENotAValidator</a>);
    // Verify the represented reporter <b>address</b> is an active <a href="validator.md#0x0_validator">validator</a>, and the capability is still valid.
    <b>assert</b>!(self.<a href="validator_set.md#0x0_validator_set_is_active_validator">is_active_validator</a>(operation_cap.validator_id()), <a href="validator_set.md#0x0_validator_set_ENotAValidator">ENotAValidator</a>);
    self.<a href="validator_set.md#0x0_validator_set_verify_operation_cap">verify_operation_cap</a>(operation_cap);
    <a href="validator_set.md#0x0_validator_set_report_validator_impl">report_validator_impl</a>(operation_cap, reportee_id, &<b>mut</b> self.validator_report_records);
}
</code></pre>



</details>

<a name="0x0_validator_set_undo_report_validator"></a>

## Function `undo_report_validator`

Undo a <code>report_validator</code> action. Aborts if
1. the reportee is not a currently active validator or
2. the sender has not previously reported the <code>reportee_id</code>, or
3. the cap is not valid


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_undo_report_validator">undo_report_validator</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, operation_cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, reportee_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_set.md#0x0_validator_set_undo_report_validator">undo_report_validator</a>(
    self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>,
    operation_cap: &ValidatorOperationCap,
    reportee_id: ID,
) {
    // Verify the represented reporter <b>address</b> is an active <a href="validator.md#0x0_validator">validator</a>, and the capability is still valid.
    <b>assert</b>!(self.<a href="validator_set.md#0x0_validator_set_is_active_validator">is_active_validator</a>(operation_cap.validator_id()), <a href="validator_set.md#0x0_validator_set_ENotAValidator">ENotAValidator</a>);
    self.<a href="validator_set.md#0x0_validator_set_verify_operation_cap">verify_operation_cap</a>(operation_cap);
    <a href="validator_set.md#0x0_validator_set_undo_report_validator_impl">undo_report_validator_impl</a>(operation_cap, reportee_id, &<b>mut</b> self.validator_report_records);
}
</code></pre>



</details>

<a name="0x0_validator_set_report_validator_impl"></a>

## Function `report_validator_impl`



<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_report_validator_impl">report_validator_impl</a>(operation_cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, reportee_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, validator_report_records: &<b>mut</b> <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, <a href="../sui-framework/vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_report_validator_impl">report_validator_impl</a>(
    operation_cap: &ValidatorOperationCap,
    reportee_id: ID,
    validator_report_records: &<b>mut</b> VecMap&lt;ID, VecSet&lt;ID&gt;&gt;,
) {
    <b>let</b> reporter_id = operation_cap.validator_id();
    <b>assert</b>!(reporter_id != reportee_id, <a href="validator_set.md#0x0_validator_set_ECannotReportOneself">ECannotReportOneself</a>);
    <b>if</b> (!validator_report_records.contains(&reportee_id)) {
        validator_report_records.insert(reportee_id, <a href="../sui-framework/vec_set.md#0x2_vec_set_singleton">vec_set::singleton</a>(reporter_id));
    } <b>else</b> {
        <b>let</b> reporters = validator_report_records.get_mut(&reportee_id);
        <b>if</b> (!reporters.contains(&reporter_id)) {
            reporters.insert(reporter_id);
        }
    }
}
</code></pre>



</details>

<a name="0x0_validator_set_undo_report_validator_impl"></a>

## Function `undo_report_validator_impl`



<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_undo_report_validator_impl">undo_report_validator_impl</a>(operation_cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, reportee_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, validator_report_records: &<b>mut</b> <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, <a href="../sui-framework/vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator_set.md#0x0_validator_set_undo_report_validator_impl">undo_report_validator_impl</a>(
    operation_cap: &ValidatorOperationCap,
    reportee_id: ID,
    validator_report_records: &<b>mut</b> VecMap&lt;ID, VecSet&lt;ID&gt;&gt;,
) {
    <b>assert</b>!(validator_report_records.contains(&reportee_id), <a href="validator_set.md#0x0_validator_set_EReportRecordNotFound">EReportRecordNotFound</a>);
    <b>let</b> reporters = validator_report_records.get_mut(&reportee_id);

    <b>let</b> reporter_id = operation_cap.validator_id();
    <b>assert</b>!(reporters.contains(&reporter_id), <a href="validator_set.md#0x0_validator_set_EReportRecordNotFound">EReportRecordNotFound</a>);

    reporters.remove(&reporter_id);
    <b>if</b> (reporters.is_empty()) {
        validator_report_records.remove(&reportee_id);
    }
}
</code></pre>



</details>

<a name="0x0_validator_set_active_committee"></a>

## Function `active_committee`

Return the active validators in <code>self</code>


<pre><code><b>public</b> <b>fun</b> <a href="validator_set.md#0x0_validator_set_active_committee">active_committee</a>(self: &<a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>): <a href="committee.md#0x0_committee_Committee">committee::Committee</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_set.md#0x0_validator_set_active_committee">active_committee</a>(self: &<a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>): Committee {
    self.active_committee
}
</code></pre>



</details>

<a name="0x0_validator_set_next_epoch_active_committee"></a>

## Function `next_epoch_active_committee`

Return the next epoch active committee in <code>self</code>


<pre><code><b>public</b> <b>fun</b> <a href="validator_set.md#0x0_validator_set_next_epoch_active_committee">next_epoch_active_committee</a>(self: &<a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>): <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="committee.md#0x0_committee_Committee">committee::Committee</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_set.md#0x0_validator_set_next_epoch_active_committee">next_epoch_active_committee</a>(self: &<a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>): Option&lt;Committee&gt; {
    self.next_epoch_active_committee
}
</code></pre>



</details>

<a name="0x0_validator_set_next_pending_active_validators"></a>

## Function `next_pending_active_validators`

Return the pending active validators in <code>self</code>


<pre><code><b>public</b> <b>fun</b> <a href="validator_set.md#0x0_validator_set_next_pending_active_validators">next_pending_active_validators</a>(self: &<a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_set.md#0x0_validator_set_next_pending_active_validators">next_pending_active_validators</a>(self: &<a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;ID&gt; {
    self.pending_active_validators
}
</code></pre>



</details>

<a name="0x0_validator_set_is_validator_candidate"></a>

## Function `is_validator_candidate`

Returns true if the <code>validator_id</code> is a validator candidate.


<pre><code><b>public</b> <b>fun</b> <a href="validator_set.md#0x0_validator_set_is_validator_candidate">is_validator_candidate</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_set.md#0x0_validator_set_is_validator_candidate">is_validator_candidate</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>, validator_id: ID): bool {
    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.<a href="validator_set.md#0x0_validator_set_get_validator_ref">get_validator_ref</a>(validator_id);
    <a href="validator.md#0x0_validator">validator</a>.is_candidate()
}
</code></pre>



</details>

<a name="0x0_validator_set_is_inactive_validator"></a>

## Function `is_inactive_validator`

Returns true if the staking pool identified by <code>validator_id</code> is of an inactive validator.


<pre><code><b>public</b> <b>fun</b> <a href="validator_set.md#0x0_validator_set_is_inactive_validator">is_inactive_validator</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_set.md#0x0_validator_set_is_inactive_validator">is_inactive_validator</a>(self: &<b>mut</b> <a href="validator_set.md#0x0_validator_set_ValidatorSet">ValidatorSet</a>, validator_id: ID): bool {
    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.<a href="validator_set.md#0x0_validator_set_get_validator_ref">get_validator_ref</a>(validator_id);
    <a href="validator.md#0x0_validator">validator</a>.is_inactive()
}
</code></pre>



</details>
