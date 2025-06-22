---
title: Module `(ika_system=0x0)::validator_set`
---



-  [Struct `ValidatorSet`](#(ika_system=0x0)_validator_set_ValidatorSet)
-  [Struct `ValidatorEpochInfoEventV1`](#(ika_system=0x0)_validator_set_ValidatorEpochInfoEventV1)
-  [Struct `ValidatorJoinEvent`](#(ika_system=0x0)_validator_set_ValidatorJoinEvent)
-  [Struct `ValidatorLeaveEvent`](#(ika_system=0x0)_validator_set_ValidatorLeaveEvent)
-  [Constants](#@Constants_0)
-  [Function `new`](#(ika_system=0x0)_validator_set_new)
-  [Function `request_add_validator_candidate`](#(ika_system=0x0)_validator_set_request_add_validator_candidate)
-  [Function `request_remove_validator_candidate`](#(ika_system=0x0)_validator_set_request_remove_validator_candidate)
-  [Function `update_pending_active_set`](#(ika_system=0x0)_validator_set_update_pending_active_set)
-  [Function `request_add_validator`](#(ika_system=0x0)_validator_set_request_add_validator)
-  [Function `assert_no_pending_or_active_duplicates`](#(ika_system=0x0)_validator_set_assert_no_pending_or_active_duplicates)
-  [Function `request_remove_validator`](#(ika_system=0x0)_validator_set_request_remove_validator)
-  [Function `request_add_stake`](#(ika_system=0x0)_validator_set_request_add_stake)
-  [Function `request_withdraw_stake`](#(ika_system=0x0)_validator_set_request_withdraw_stake)
-  [Function `withdraw_stake`](#(ika_system=0x0)_validator_set_withdraw_stake)
-  [Function `rotate_operation_cap`](#(ika_system=0x0)_validator_set_rotate_operation_cap)
-  [Function `rotate_commission_cap`](#(ika_system=0x0)_validator_set_rotate_commission_cap)
-  [Function `collect_commission`](#(ika_system=0x0)_validator_set_collect_commission)
-  [Function `set_validator_name`](#(ika_system=0x0)_validator_set_set_validator_name)
-  [Function `validator_metadata`](#(ika_system=0x0)_validator_set_validator_metadata)
-  [Function `set_validator_metadata`](#(ika_system=0x0)_validator_set_set_validator_metadata)
-  [Function `set_next_commission`](#(ika_system=0x0)_validator_set_set_next_commission)
-  [Function `set_next_epoch_network_address`](#(ika_system=0x0)_validator_set_set_next_epoch_network_address)
-  [Function `set_next_epoch_p2p_address`](#(ika_system=0x0)_validator_set_set_next_epoch_p2p_address)
-  [Function `set_next_epoch_consensus_address`](#(ika_system=0x0)_validator_set_set_next_epoch_consensus_address)
-  [Function `set_next_epoch_protocol_pubkey_bytes`](#(ika_system=0x0)_validator_set_set_next_epoch_protocol_pubkey_bytes)
-  [Function `set_next_epoch_network_pubkey_bytes`](#(ika_system=0x0)_validator_set_set_next_epoch_network_pubkey_bytes)
-  [Function `set_next_epoch_consensus_pubkey_bytes`](#(ika_system=0x0)_validator_set_set_next_epoch_consensus_pubkey_bytes)
-  [Function `set_next_epoch_class_groups_pubkey_and_proof_bytes`](#(ika_system=0x0)_validator_set_set_next_epoch_class_groups_pubkey_and_proof_bytes)
-  [Function `set_pricing_vote`](#(ika_system=0x0)_validator_set_set_pricing_vote)
-  [Function `process_mid_epoch`](#(ika_system=0x0)_validator_set_process_mid_epoch)
-  [Function `advance_epoch`](#(ika_system=0x0)_validator_set_advance_epoch)
-  [Function `activate_added_validators`](#(ika_system=0x0)_validator_set_activate_added_validators)
-  [Function `set_min_validator_count`](#(ika_system=0x0)_validator_set_set_min_validator_count)
-  [Function `set_max_validator_count`](#(ika_system=0x0)_validator_set_set_max_validator_count)
-  [Function `set_min_validator_joining_stake`](#(ika_system=0x0)_validator_set_set_min_validator_joining_stake)
-  [Function `set_max_validator_change_count`](#(ika_system=0x0)_validator_set_set_max_validator_change_count)
-  [Function `set_reward_slashing_rate`](#(ika_system=0x0)_validator_set_set_reward_slashing_rate)
-  [Function `total_stake`](#(ika_system=0x0)_validator_set_total_stake)
-  [Function `validator_total_stake_amount`](#(ika_system=0x0)_validator_set_validator_total_stake_amount)
-  [Function `token_exchange_rates`](#(ika_system=0x0)_validator_set_token_exchange_rates)
-  [Function `pending_active_validators_count`](#(ika_system=0x0)_validator_set_pending_active_validators_count)
-  [Function `is_active_validator`](#(ika_system=0x0)_validator_set_is_active_validator)
-  [Function `get_reporters_of`](#(ika_system=0x0)_validator_set_get_reporters_of)
-  [Function `is_duplicate_with_pending_validator`](#(ika_system=0x0)_validator_set_is_duplicate_with_pending_validator)
-  [Function `get_validator_mut`](#(ika_system=0x0)_validator_set_get_validator_mut)
-  [Function `get_validator`](#(ika_system=0x0)_validator_set_get_validator)
-  [Function `get_validator_indices`](#(ika_system=0x0)_validator_set_get_validator_indices)
-  [Function `verify_operation_cap`](#(ika_system=0x0)_validator_set_verify_operation_cap)
-  [Function `process_pending_validators`](#(ika_system=0x0)_validator_set_process_pending_validators)
-  [Function `calculate_total_stakes`](#(ika_system=0x0)_validator_set_calculate_total_stakes)
-  [Function `compute_reward_adjustments`](#(ika_system=0x0)_validator_set_compute_reward_adjustments)
-  [Function `compute_slashed_validators`](#(ika_system=0x0)_validator_set_compute_slashed_validators)
-  [Function `compute_unadjusted_reward_distribution`](#(ika_system=0x0)_validator_set_compute_unadjusted_reward_distribution)
-  [Function `compute_adjusted_reward_distribution`](#(ika_system=0x0)_validator_set_compute_adjusted_reward_distribution)
-  [Function `distribute_reward`](#(ika_system=0x0)_validator_set_distribute_reward)
-  [Function `emit_validator_epoch_events`](#(ika_system=0x0)_validator_set_emit_validator_epoch_events)
-  [Function `report_validator`](#(ika_system=0x0)_validator_set_report_validator)
-  [Function `undo_report_validator`](#(ika_system=0x0)_validator_set_undo_report_validator)
-  [Function `report_validator_impl`](#(ika_system=0x0)_validator_set_report_validator_impl)
-  [Function `undo_report_validator_impl`](#(ika_system=0x0)_validator_set_undo_report_validator_impl)
-  [Function `active_committee`](#(ika_system=0x0)_validator_set_active_committee)
-  [Function `next_epoch_active_committee`](#(ika_system=0x0)_validator_set_next_epoch_active_committee)
-  [Function `pending_active_set`](#(ika_system=0x0)_validator_set_pending_active_set)
-  [Function `is_validator_candidate`](#(ika_system=0x0)_validator_set_is_validator_candidate)
-  [Function `is_inactive_validator`](#(ika_system=0x0)_validator_set_is_inactive_validator)
-  [Function `calculate_rewards`](#(ika_system=0x0)_validator_set_calculate_rewards)
-  [Function `can_withdraw_staked_ika_early`](#(ika_system=0x0)_validator_set_can_withdraw_staked_ika_early)


<pre><code><b>use</b> (ika=0x0)::ika;
<b>use</b> (ika_system=0x0)::<b>address</b>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee">bls_committee</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof">class_groups_public_key_and_proof</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner">dwallet_2pc_mpc_coordinator_inner</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing">dwallet_pricing</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/extended_field.md#(ika_system=0x0)_extended_field">extended_field</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/multiaddr.md#(ika_system=0x0)_multiaddr">multiaddr</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set">pending_active_set</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values">pending_values</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate">token_exchange_rate</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>;
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
<b>use</b> <a href="../sui/ed25519.md#sui_ed25519">sui::ed25519</a>;
<b>use</b> <a href="../sui/event.md#sui_event">sui::event</a>;
<b>use</b> <a href="../sui/group_ops.md#sui_group_ops">sui::group_ops</a>;
<b>use</b> <a href="../sui/hash.md#sui_hash">sui::hash</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/object_table.md#sui_object_table">sui::object_table</a>;
<b>use</b> <a href="../sui/party.md#sui_party">sui::party</a>;
<b>use</b> <a href="../sui/priority_queue.md#sui_priority_queue">sui::priority_queue</a>;
<b>use</b> <a href="../sui/sui.md#sui_sui">sui::sui</a>;
<b>use</b> <a href="../sui/table.md#sui_table">sui::table</a>;
<b>use</b> <a href="../sui/table_vec.md#sui_table_vec">sui::table_vec</a>;
<b>use</b> <a href="../sui/transfer.md#sui_transfer">sui::transfer</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
<b>use</b> <a href="../sui/types.md#sui_types">sui::types</a>;
<b>use</b> <a href="../sui/url.md#sui_url">sui::url</a>;
<b>use</b> <a href="../sui/vec_map.md#sui_vec_map">sui::vec_map</a>;
<b>use</b> <a href="../sui/vec_set.md#sui_vec_set">sui::vec_set</a>;
</code></pre>



<a name="(ika_system=0x0)_validator_set_ValidatorSet"></a>

## Struct `ValidatorSet`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_total_stake">total_stake</a>: u64</code>
</dt>
<dd>
 Total amount of stake from all active validators at the beginning of the epoch.
</dd>
<dt>
<code>reward_slashing_rate: u16</code>
</dt>
<dd>
 How many reward are slashed to punish a validator, in bps.
</dd>
<dt>
<code>validators: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, (ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>&gt;</code>
</dt>
<dd>
 A table that contains all validators
</dd>
<dt>
<code><a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_active_committee">active_committee</a>: (ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">bls_committee::BlsCommittee</a></code>
</dt>
<dd>
 The current list of active committee of validators.
</dd>
<dt>
<code><a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_next_epoch_active_committee">next_epoch_active_committee</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;(ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">bls_committee::BlsCommittee</a>&gt;</code>
</dt>
<dd>
 The next list of active committee of validators.
 It will become the active_committee at the end of the epoch.
</dd>
<dt>
<code>previous_committee: (ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">bls_committee::BlsCommittee</a></code>
</dt>
<dd>
 The current list of previous committee of validators.
</dd>
<dt>
<code><a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set">pending_active_set</a>: (ika_system=0x0)::<a href="../ika_system/extended_field.md#(ika_system=0x0)_extended_field_ExtendedField">extended_field::ExtendedField</a>&lt;(ika_system=0x0)::<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">pending_active_set::PendingActiveSet</a>&gt;</code>
</dt>
<dd>
 The next list of pending active set of validators to be next_epoch_active_committee.
 It will start from the last next_epoch_active_committee and will be
 process between middle of the epochs and will be finalize
 at the middle of the epoch.
</dd>
<dt>
<code>validator_report_records: <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, <a href="../sui/vec_set.md#sui_vec_set_VecSet">sui::vec_set::VecSet</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;&gt;</code>
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
<code>extra_fields: <a href="../sui/bag.md#sui_bag_Bag">sui::bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_validator_set_ValidatorEpochInfoEventV1"></a>

## Struct `ValidatorEpochInfoEventV1`

Event containing staking and rewards related information of
each validator, emitted during epoch advancement.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorEpochInfoEventV1">ValidatorEpochInfoEventV1</a> <b>has</b> <b>copy</b>, drop
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
<code>validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>stake: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>commission_rate: u16</code>
</dt>
<dd>
</dd>
<dt>
<code>staking_rewards: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate">token_exchange_rate</a>: (ika_system=0x0)::<a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_TokenExchangeRate">token_exchange_rate::TokenExchangeRate</a></code>
</dt>
<dd>
</dd>
<dt>
<code>tallying_rule_reporters: vector&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>tallying_rule_global_score: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_validator_set_ValidatorJoinEvent"></a>

## Struct `ValidatorJoinEvent`

Event emitted every time a new validator joins the committee.
The epoch value corresponds to the first epoch this change takes place.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorJoinEvent">ValidatorJoinEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_validator_set_ValidatorLeaveEvent"></a>

## Struct `ValidatorLeaveEvent`

Event emitted every time a validator leaves the committee.
The epoch value corresponds to the first epoch this change takes place.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorLeaveEvent">ValidatorLeaveEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>withdrawing_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
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


<a name="(ika_system=0x0)_validator_set_BASIS_POINT_DENOMINATOR"></a>



<pre><code><b>const</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a>: u16 = 10000;
</code></pre>



<a name="(ika_system=0x0)_validator_set_BASIS_POINT_DENOMINATOR_U128"></a>



<pre><code><b>const</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_BASIS_POINT_DENOMINATOR_U128">BASIS_POINT_DENOMINATOR_U128</a>: u128 = 10000;
</code></pre>



<a name="(ika_system=0x0)_validator_set_MIN_STAKING_THRESHOLD"></a>



<pre><code><b>const</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>: u64 = 1000000000;
</code></pre>



<a name="(ika_system=0x0)_validator_set_ENonValidatorInReportRecords"></a>

The validator is not in the report records.


<pre><code><b>const</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ENonValidatorInReportRecords">ENonValidatorInReportRecords</a>: u64 = 0;
</code></pre>



<a name="(ika_system=0x0)_validator_set_EDuplicateValidator"></a>

The validator is already in the validator set.


<pre><code><b>const</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_EDuplicateValidator">EDuplicateValidator</a>: u64 = 1;
</code></pre>



<a name="(ika_system=0x0)_validator_set_ENotAValidator"></a>

The validator is not a validator.


<pre><code><b>const</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ENotAValidator">ENotAValidator</a>: u64 = 2;
</code></pre>



<a name="(ika_system=0x0)_validator_set_EValidatorNotCandidate"></a>

The validator is not a candidate.


<pre><code><b>const</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_EValidatorNotCandidate">EValidatorNotCandidate</a>: u64 = 3;
</code></pre>



<a name="(ika_system=0x0)_validator_set_EStakingBelowThreshold"></a>

The validator is not staking below the threshold.


<pre><code><b>const</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_EStakingBelowThreshold">EStakingBelowThreshold</a>: u64 = 4;
</code></pre>



<a name="(ika_system=0x0)_validator_set_EValidatorAlreadyRemoved"></a>

The validator is already removed.


<pre><code><b>const</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_EValidatorAlreadyRemoved">EValidatorAlreadyRemoved</a>: u64 = 5;
</code></pre>



<a name="(ika_system=0x0)_validator_set_ECannotReportOneself"></a>

The validator cannot report on itself.


<pre><code><b>const</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ECannotReportOneself">ECannotReportOneself</a>: u64 = 6;
</code></pre>



<a name="(ika_system=0x0)_validator_set_EReportRecordNotFound"></a>

The report record is not found.


<pre><code><b>const</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_EReportRecordNotFound">EReportRecordNotFound</a>: u64 = 7;
</code></pre>



<a name="(ika_system=0x0)_validator_set_ECannotJoinActiveSet"></a>

The validator cannot join the active set.


<pre><code><b>const</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ECannotJoinActiveSet">ECannotJoinActiveSet</a>: u64 = 8;
</code></pre>



<a name="(ika_system=0x0)_validator_set_EBpsTooLarge"></a>

The bps is too large.


<pre><code><b>const</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_EBpsTooLarge">EBpsTooLarge</a>: u64 = 9;
</code></pre>



<a name="(ika_system=0x0)_validator_set_EInvalidCap"></a>

The cap is invalid.


<pre><code><b>const</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_EInvalidCap">EInvalidCap</a>: u64 = 10;
</code></pre>



<a name="(ika_system=0x0)_validator_set_EProcessMidEpochOnlyAfterAdvanceEpoch"></a>

Process mid epoch can be called only after advance epoch.


<pre><code><b>const</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_EProcessMidEpochOnlyAfterAdvanceEpoch">EProcessMidEpochOnlyAfterAdvanceEpoch</a>: u64 = 11;
</code></pre>



<a name="(ika_system=0x0)_validator_set_EAdvanceEpochOnlyAfterProcessMidEpoch"></a>

Advance epoch can be called only after process mid epoch.


<pre><code><b>const</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_EAdvanceEpochOnlyAfterProcessMidEpoch">EAdvanceEpochOnlyAfterProcessMidEpoch</a>: u64 = 12;
</code></pre>



<a name="(ika_system=0x0)_validator_set_new"></a>

## Function `new`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_new">new</a>(min_validator_count: u64, max_validator_count: u64, min_validator_joining_stake: u64, max_validator_change_count: u64, reward_slashing_rate: u16, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_new">new</a>(
    min_validator_count: u64,
    max_validator_count: u64,
    min_validator_joining_stake: u64,
    max_validator_change_count: u64,
    reward_slashing_rate: u16,
    ctx: &<b>mut</b> TxContext,
): <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a> {
    // Rates can't be higher than 100%.
    <b>assert</b>!(
        reward_slashing_rate &lt;= <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a>,
        <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_EBpsTooLarge">EBpsTooLarge</a>,
    );
    <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a> {
        <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_total_stake">total_stake</a>: 0,
        reward_slashing_rate,
        validators: object_table::new(ctx),
        <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_active_committee">active_committee</a>: <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_empty">bls_committee::empty</a>(),
        <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_next_epoch_active_committee">next_epoch_active_committee</a>: option::none(),
        previous_committee: <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_empty">bls_committee::empty</a>(),
        <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set">pending_active_set</a>: <a href="../ika_system/extended_field.md#(ika_system=0x0)_extended_field_new">extended_field::new</a>(<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_new">pending_active_set::new</a>(min_validator_count, max_validator_count, min_validator_joining_stake, max_validator_change_count), ctx),
        validator_report_records: vec_map::empty(),
        extra_fields: bag::new(ctx),
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_request_add_validator_candidate"></a>

## Function `request_add_validator_candidate`

Called by <code>ika_system</code> to add a new validator candidate.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_request_add_validator_candidate">request_add_validator_candidate</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, current_epoch: u64, name: <a href="../std/string.md#std_string_String">std::string::String</a>, protocol_pubkey_bytes: vector&lt;u8&gt;, network_pubkey_bytes: vector&lt;u8&gt;, consensus_pubkey_bytes: vector&lt;u8&gt;, class_groups_pubkey_and_proof_bytes: (ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof</a>, proof_of_possession_bytes: vector&lt;u8&gt;, network_address: <a href="../std/string.md#std_string_String">std::string::String</a>, p2p_address: <a href="../std/string.md#std_string_String">std::string::String</a>, consensus_address: <a href="../std/string.md#std_string_String">std::string::String</a>, commission_rate: u16, metadata: (ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">validator_metadata::ValidatorMetadata</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): ((ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCommissionCap">validator_cap::ValidatorCommissionCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_request_add_validator_candidate">request_add_validator_candidate</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
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
    commission_rate: u16,
    metadata: ValidatorMetadata,
    ctx: &<b>mut</b> TxContext,
): (ValidatorCap, ValidatorOperationCap, ValidatorCommissionCap) {
    <b>let</b> (<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>, cap, operation_cap, commission_cap) = <a href="../ika_system/validator.md#(ika_system=0x0)_validator_new">validator::new</a>(
        current_epoch,
        name,
        protocol_pubkey_bytes,
        network_pubkey_bytes,
        consensus_pubkey_bytes,
        class_groups_pubkey_and_proof_bytes,
        proof_of_possession_bytes,
        network_address,
        p2p_address,
        consensus_address,
        commission_rate,
        metadata,
        ctx,
    );
    <b>let</b> validator_id = <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.validator_id();
    // The next assertions are not critical <b>for</b> the protocol, but they are here to catch problematic configs earlier.
    <b>assert</b>!(!<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_is_duplicate_with_pending_validator">is_duplicate_with_pending_validator</a>(self, &<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>), <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_EDuplicateValidator">EDuplicateValidator</a>);
    <b>assert</b>!(!self.validators.contains(validator_id), <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_EDuplicateValidator">EDuplicateValidator</a>);
    <b>assert</b>!(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.is_preactive(), <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_EValidatorNotCandidate">EValidatorNotCandidate</a>);
    self.validators.add(validator_id, <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>);
    (cap, operation_cap, commission_cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_request_remove_validator_candidate"></a>

## Function `request_remove_validator_candidate`

Called by <code>ika_system</code> to remove a validator candidate, and move them to <code>inactive_committee</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_request_remove_validator_candidate">request_remove_validator_candidate</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, epoch: u64, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_request_remove_validator_candidate">request_remove_validator_candidate</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    epoch: u64,
    cap: &ValidatorCap,
) {
    <b>let</b> validator_id = cap.validator_id();
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
    <b>assert</b>!(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.is_preactive(), <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_EValidatorNotCandidate">EValidatorNotCandidate</a>);
    // Set the <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> to withdrawing state
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.set_withdrawing(cap, epoch);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_update_pending_active_set"></a>

## Function `update_pending_active_set`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_update_pending_active_set">update_pending_active_set</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, current_epoch: u64, committee_selected: bool, insert_if_not_in_set: bool): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_update_pending_active_set">update_pending_active_set</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    validator_id: ID,
    current_epoch: u64,
    committee_selected: bool,
    insert_if_not_in_set: bool,
): bool {
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
    <b>let</b> balance = <b>if</b> (committee_selected) {
        <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.ika_balance_at_epoch(current_epoch + 2)
    } <b>else</b> {
        <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.ika_balance_at_epoch(current_epoch + 1)
    };
    <b>let</b> (in_set, <b>mut</b> removed_validator_id) = <b>if</b> (insert_if_not_in_set) {
        self.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set">pending_active_set</a>.borrow_mut().insert_or_update_or_remove(validator_id, balance)
    } <b>else</b> {
        self.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set">pending_active_set</a>.borrow_mut().update_or_remove(validator_id, balance)
    };
    <b>if</b>(removed_validator_id.is_some()) {
        <b>let</b> removed_validator = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator_mut">get_validator_mut</a>(removed_validator_id.extract());
        <b>let</b> new_epoch = current_epoch + 1;
        removed_validator.deactivate(new_epoch);
        event::emit(<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorLeaveEvent">ValidatorLeaveEvent</a> {
            withdrawing_epoch: new_epoch,
            validator_id,
            is_voluntary: <b>false</b>,
        });
    };
    in_set
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_request_add_validator"></a>

## Function `request_add_validator`

Called by <code>ika_system</code> to add a new validator to <code>pending_active_validators</code>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_request_add_validator">request_add_validator</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, current_epoch: u64, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_request_add_validator">request_add_validator</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    current_epoch: u64,
    cap: &ValidatorCap,
) {
    <b>let</b> validator_id = cap.validator_id();
    <b>assert</b>!(self.validators.contains(validator_id), <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_EValidatorNotCandidate">EValidatorNotCandidate</a>);
    <b>let</b> committee_selected = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_next_epoch_active_committee">next_epoch_active_committee</a>.is_some();
    // We have to remove and to add again because we can have 2 refs to self
    <b>let</b> <b>mut</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.validators.remove(validator_id);
    <b>assert</b>!(!self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_is_duplicate_with_pending_validator">is_duplicate_with_pending_validator</a>(&<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>), <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_EDuplicateValidator">EDuplicateValidator</a>);
    <b>assert</b>!(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.is_preactive(), <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_EValidatorNotCandidate">EValidatorNotCandidate</a>);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.activate(cap, current_epoch, committee_selected);
    self.validators.add(validator_id, <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>);
    <b>let</b> in_set = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_update_pending_active_set">update_pending_active_set</a>(validator_id, current_epoch, committee_selected, <b>true</b>);
    <b>assert</b>!(in_set, <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ECannotJoinActiveSet">ECannotJoinActiveSet</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_assert_no_pending_or_active_duplicates"></a>

## Function `assert_no_pending_or_active_duplicates`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_assert_no_pending_or_active_duplicates">assert_no_pending_or_active_duplicates</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_assert_no_pending_or_active_duplicates">assert_no_pending_or_active_duplicates</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    validator_id: ID,
) {
    <b>assert</b>!(self.validators.contains(validator_id), <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ENotAValidator">ENotAValidator</a>);
    // We have to remove and to add again because we can have 2 refs to self
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.validators.remove(validator_id);
    <b>assert</b>!(!self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_is_duplicate_with_pending_validator">is_duplicate_with_pending_validator</a>(&<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>), <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_EDuplicateValidator">EDuplicateValidator</a>);
    self.validators.add(validator_id, <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_request_remove_validator"></a>

## Function `request_remove_validator`

Called by <code>ika_system</code>, to remove a validator.
The index of the validator is added to <code>pending_removals</code> and
will be processed at the end of epoch.
Only an active validator can request to be removed.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_request_remove_validator">request_remove_validator</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, current_epoch: u64, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_request_remove_validator">request_remove_validator</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    current_epoch: u64,
    cap: &ValidatorCap,
) {
    <b>let</b> validator_id = cap.validator_id();
    <b>let</b> committee_selected = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_next_epoch_active_committee">next_epoch_active_committee</a>.is_some();
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
    <b>assert</b>!(!<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.is_withdrawing(), <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_EValidatorAlreadyRemoved">EValidatorAlreadyRemoved</a>);
    <b>let</b> withdrawing_epoch = <b>if</b> (committee_selected) {
        current_epoch + 2
    } <b>else</b> {
        current_epoch + 1
    };
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.set_withdrawing(cap, withdrawing_epoch);
    self.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set">pending_active_set</a>.borrow_mut().remove(validator_id);
    event::emit(<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorLeaveEvent">ValidatorLeaveEvent</a> {
        withdrawing_epoch,
        validator_id,
        is_voluntary: <b>true</b>,
    });
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_request_add_stake"></a>

## Function `request_add_stake`

Called by <code>ika_system</code>, to add a new stake to the validator.
This request is added to the validator's validator's pending stake entries, processed at the end
of the epoch.
Aborts in case the staking amount is smaller than MIN_STAKING_THRESHOLD


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_request_add_stake">request_add_stake</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, epoch: u64, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, stake: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_request_add_stake">request_add_stake</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    epoch: u64,
    validator_id: ID,
    stake: Balance&lt;IKA&gt;,
    ctx: &<b>mut</b> TxContext,
): StakedIka {
    <b>let</b> committee_selected = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_next_epoch_active_committee">next_epoch_active_committee</a>.is_some();
    <b>let</b> ika_amount = stake.value();
    <b>assert</b>!(ika_amount &gt;= <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>, <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_EStakingBelowThreshold">EStakingBelowThreshold</a>);
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
    <b>let</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a> = <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_request_add_stake">request_add_stake</a>(
        stake,
        epoch,
        committee_selected,
        ctx
    );
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_update_pending_active_set">update_pending_active_set</a>(validator_id, epoch, committee_selected, <b>false</b>);
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_request_withdraw_stake"></a>

## Function `request_withdraw_stake`

Requests withdrawal of the given amount from the <code>StakedIKA</code>, marking it as
<code>Withdrawing</code>. Once the epoch is greater than the <code>withdraw_epoch</code>, the
withdrawal can be performed.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_request_withdraw_stake">request_withdraw_stake</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>, current_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_request_withdraw_stake">request_withdraw_stake</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: &<b>mut</b> StakedIka,
    current_epoch: u64,
) {
    <b>let</b> validator_id = <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.validator_id();
    <b>let</b> committee_selected = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_next_epoch_active_committee">next_epoch_active_committee</a>.is_some();
    <b>let</b> is_current_committee = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_active_committee">active_committee</a>.contains(&validator_id);
    <b>let</b> is_next_committee = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_next_epoch_active_committee">next_epoch_active_committee</a>.is_some_and!(|c| c.contains(&validator_id));
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_request_withdraw_stake">request_withdraw_stake</a>(
        <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>,
        is_current_committee,
        is_next_committee,
        current_epoch
    );
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_update_pending_active_set">update_pending_active_set</a>(validator_id, current_epoch, committee_selected, <b>false</b>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_withdraw_stake"></a>

## Function `withdraw_stake`

Perform the withdrawal of the staked WAL, returning the amount to the caller.
The <code>StakedWal</code> must be in the <code>Withdrawing</code> state, and the epoch must be
greater than the <code>withdraw_epoch</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_withdraw_stake">withdraw_stake</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>, current_epoch: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_withdraw_stake">withdraw_stake</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: StakedIka,
    current_epoch: u64,
    ctx: &<b>mut</b> TxContext,
): Coin&lt;IKA&gt; {
    <b>let</b> validator_id = <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.validator_id();
    <b>let</b> committee_selected = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_next_epoch_active_committee">next_epoch_active_committee</a>.is_some();
    <b>let</b> is_current_committee = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_active_committee">active_committee</a>.contains(&validator_id);
    <b>let</b> is_next_committee = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_next_epoch_active_committee">next_epoch_active_committee</a>.is_some_and!(|c| c.contains(&validator_id));
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
    <b>let</b> ika_balance = <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_withdraw_stake">withdraw_stake</a>(
        <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>,
        is_current_committee,
        is_next_committee,
        current_epoch
    );
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_update_pending_active_set">update_pending_active_set</a>(validator_id, current_epoch, committee_selected, <b>false</b>);
    ika_balance.into_coin(ctx)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_rotate_operation_cap"></a>

## Function `rotate_operation_cap`

Create a new <code>ValidatorOperationCap</code> and registers it.
The original object is thus revoked.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_rotate_operation_cap">rotate_operation_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_rotate_operation_cap">rotate_operation_cap</a>(self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>, cap: &ValidatorCap, ctx: &<b>mut</b> TxContext): ValidatorOperationCap {
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator_mut">get_validator_mut</a>(cap.validator_id());
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_rotate_operation_cap">rotate_operation_cap</a>(cap, ctx)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_rotate_commission_cap"></a>

## Function `rotate_commission_cap`

Create a new <code>ValidatorCommissionCap</code> and registers it.
The original object is thus revoked.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_rotate_commission_cap">rotate_commission_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCommissionCap">validator_cap::ValidatorCommissionCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_rotate_commission_cap">rotate_commission_cap</a>(self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>, cap: &ValidatorCap, ctx: &<b>mut</b> TxContext): ValidatorCommissionCap {
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator_mut">get_validator_mut</a>(cap.validator_id());
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_rotate_commission_cap">rotate_commission_cap</a>(cap, ctx)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_collect_commission"></a>

## Function `collect_commission`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_collect_commission">collect_commission</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCommissionCap">validator_cap::ValidatorCommissionCap</a>, amount: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_collect_commission">collect_commission</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    cap: &ValidatorCommissionCap,
    amount: Option&lt;u64&gt;,
): Balance&lt;IKA&gt; {
    <b>let</b> validator_id = cap.validator_id();
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_collect_commission">collect_commission</a>(cap, amount)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_set_validator_name"></a>

## Function `set_validator_name`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_validator_name">set_validator_name</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, name: <a href="../std/string.md#std_string_String">std::string::String</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_validator_name">set_validator_name</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    name: String,
    cap: &ValidatorOperationCap,
) {
    <b>let</b> validator_id = cap.validator_id();
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.set_name(name, cap);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_validator_metadata"></a>

## Function `validator_metadata`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata">validator_metadata</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): (ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">validator_metadata::ValidatorMetadata</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata">validator_metadata</a>(
    self: &<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    validator_id: ID,
): ValidatorMetadata {
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator">get_validator</a>(validator_id);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>().metadata()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_set_validator_metadata"></a>

## Function `set_validator_metadata`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_validator_metadata">set_validator_metadata</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, metadata: (ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">validator_metadata::ValidatorMetadata</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_validator_metadata">set_validator_metadata</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    cap: &ValidatorOperationCap,
    metadata: ValidatorMetadata,
) {
    <b>let</b> validator_id = cap.validator_id();
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_validator_metadata">set_validator_metadata</a>(cap, metadata);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_set_next_commission"></a>

## Function `set_next_commission`

Request to set commission rate for the validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_next_commission">set_next_commission</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, new_commission_rate: u16, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, current_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_next_commission">set_next_commission</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    new_commission_rate: u16,
    cap: &ValidatorOperationCap,
    current_epoch: u64,
) {
    <b>let</b> validator_id = cap.validator_id();
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_next_commission">set_next_commission</a>(new_commission_rate, current_epoch, cap);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_set_next_epoch_network_address"></a>

## Function `set_next_epoch_network_address`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_next_epoch_network_address">set_next_epoch_network_address</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, network_address: <a href="../std/string.md#std_string_String">std::string::String</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_next_epoch_network_address">set_next_epoch_network_address</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    network_address: String,
    cap: &ValidatorOperationCap,
) {
    <b>let</b> validator_id = cap.validator_id();
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_next_epoch_network_address">set_next_epoch_network_address</a>(network_address, cap);
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_assert_no_pending_or_active_duplicates">assert_no_pending_or_active_duplicates</a>(validator_id);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_set_next_epoch_p2p_address"></a>

## Function `set_next_epoch_p2p_address`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_next_epoch_p2p_address">set_next_epoch_p2p_address</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, p2p_address: <a href="../std/string.md#std_string_String">std::string::String</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_next_epoch_p2p_address">set_next_epoch_p2p_address</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    p2p_address: String,
    cap: &ValidatorOperationCap,
) {
    <b>let</b> validator_id = cap.validator_id();
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_next_epoch_p2p_address">set_next_epoch_p2p_address</a>(p2p_address, cap);
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_assert_no_pending_or_active_duplicates">assert_no_pending_or_active_duplicates</a>(validator_id);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_set_next_epoch_consensus_address"></a>

## Function `set_next_epoch_consensus_address`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_next_epoch_consensus_address">set_next_epoch_consensus_address</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, consensus_address: <a href="../std/string.md#std_string_String">std::string::String</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_next_epoch_consensus_address">set_next_epoch_consensus_address</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    consensus_address: String,
    cap: &ValidatorOperationCap,
) {
    <b>let</b> validator_id = cap.validator_id();
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_next_epoch_consensus_address">set_next_epoch_consensus_address</a>(consensus_address, cap);
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_assert_no_pending_or_active_duplicates">assert_no_pending_or_active_duplicates</a>(validator_id);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_set_next_epoch_protocol_pubkey_bytes"></a>

## Function `set_next_epoch_protocol_pubkey_bytes`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_next_epoch_protocol_pubkey_bytes">set_next_epoch_protocol_pubkey_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, protocol_pubkey_bytes: vector&lt;u8&gt;, proof_of_possession: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, ctx: &<a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_next_epoch_protocol_pubkey_bytes">set_next_epoch_protocol_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    protocol_pubkey_bytes: vector&lt;u8&gt;,
    proof_of_possession: vector&lt;u8&gt;,
    cap: &ValidatorOperationCap,
    ctx: &TxContext,
) {
    <b>let</b> validator_id = cap.validator_id();
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_next_epoch_protocol_pubkey_bytes">set_next_epoch_protocol_pubkey_bytes</a>(protocol_pubkey_bytes, proof_of_possession, cap, ctx);
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_assert_no_pending_or_active_duplicates">assert_no_pending_or_active_duplicates</a>(validator_id);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_set_next_epoch_network_pubkey_bytes"></a>

## Function `set_next_epoch_network_pubkey_bytes`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_next_epoch_network_pubkey_bytes">set_next_epoch_network_pubkey_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, network_pubkey_bytes: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_next_epoch_network_pubkey_bytes">set_next_epoch_network_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    network_pubkey_bytes: vector&lt;u8&gt;,
    cap: &ValidatorOperationCap,
) {
    <b>let</b> validator_id = cap.validator_id();
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_next_epoch_network_pubkey_bytes">set_next_epoch_network_pubkey_bytes</a>(network_pubkey_bytes, cap);
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_assert_no_pending_or_active_duplicates">assert_no_pending_or_active_duplicates</a>(validator_id);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_set_next_epoch_consensus_pubkey_bytes"></a>

## Function `set_next_epoch_consensus_pubkey_bytes`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_next_epoch_consensus_pubkey_bytes">set_next_epoch_consensus_pubkey_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, consensus_pubkey_bytes: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_next_epoch_consensus_pubkey_bytes">set_next_epoch_consensus_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    consensus_pubkey_bytes: vector&lt;u8&gt;,
    cap: &ValidatorOperationCap,
) {
    <b>let</b> validator_id = cap.validator_id();
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_next_epoch_consensus_pubkey_bytes">set_next_epoch_consensus_pubkey_bytes</a>(consensus_pubkey_bytes, cap);
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_assert_no_pending_or_active_duplicates">assert_no_pending_or_active_duplicates</a>(validator_id);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_set_next_epoch_class_groups_pubkey_and_proof_bytes"></a>

## Function `set_next_epoch_class_groups_pubkey_and_proof_bytes`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_next_epoch_class_groups_pubkey_and_proof_bytes">set_next_epoch_class_groups_pubkey_and_proof_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, class_groups_pubkey_and_proof_bytes: (ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_next_epoch_class_groups_pubkey_and_proof_bytes">set_next_epoch_class_groups_pubkey_and_proof_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    class_groups_pubkey_and_proof_bytes: ClassGroupsPublicKeyAndProof,
    cap: &ValidatorOperationCap,
) {
    <b>let</b> validator_id = cap.validator_id();
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_next_epoch_class_groups_pubkey_and_proof_bytes">set_next_epoch_class_groups_pubkey_and_proof_bytes</a>(class_groups_pubkey_and_proof_bytes, cap);
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_assert_no_pending_or_active_duplicates">assert_no_pending_or_active_duplicates</a>(validator_id);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_set_pricing_vote"></a>

## Function `set_pricing_vote`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_pricing_vote">set_pricing_vote</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, dwallet_coordinator_inner: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, pricing: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing">dwallet_pricing::DWalletPricing</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_pricing_vote">set_pricing_vote</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    dwallet_coordinator_inner: &<b>mut</b> DWalletCoordinatorInner,
    pricing: DWalletPricing,
    cap: &ValidatorOperationCap,
) {
    <b>let</b> validator_id = cap.validator_id();
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_verify_operation_cap">verify_operation_cap</a>(cap);
    dwallet_coordinator_inner.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_pricing_vote">set_pricing_vote</a>(validator_id, pricing);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_process_mid_epoch"></a>

## Function `process_mid_epoch`

Process the pending validator changes at mid epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_process_mid_epoch">process_mid_epoch</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_process_mid_epoch">process_mid_epoch</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
) {
    <b>assert</b>!(self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_next_epoch_active_committee">next_epoch_active_committee</a>.is_none(), <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_EProcessMidEpochOnlyAfterAdvanceEpoch">EProcessMidEpochOnlyAfterAdvanceEpoch</a>);
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_process_pending_validators">process_pending_validators</a>();
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_advance_epoch"></a>

## Function `advance_epoch`

Update the validator set at the end of epoch.
It does the following things:
1. Distribute stake award.
2. Process pending stake deposits and withdraws for each validator (<code>adjust_stake</code>).
3. Process pending stake deposits, and withdraws.
4. Process pending validator application and withdraws.
5. At the end, we calculate the total stake for the new epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_advance_epoch">advance_epoch</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, new_epoch: u64, total_reward: &<b>mut</b> <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_advance_epoch">advance_epoch</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    new_epoch: u64,
    total_reward: &<b>mut</b> Balance&lt;IKA&gt;,
) {
    <b>assert</b>!(self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_next_epoch_active_committee">next_epoch_active_committee</a>.is_some(), <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_EAdvanceEpochOnlyAfterProcessMidEpoch">EAdvanceEpochOnlyAfterProcessMidEpoch</a>);
    <b>let</b> total_voting_power = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_active_committee">active_committee</a>.total_voting_power();
    // Compute the reward distribution without taking into account the tallying rule slashing.
    <b>let</b> unadjusted_staking_reward_amounts = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_compute_unadjusted_reward_distribution">compute_unadjusted_reward_distribution</a>(
        total_voting_power,
        total_reward.value(),
    );
    // Use the tallying rule report records <b>for</b> the epoch to compute validators that will be
    // punished.
    <b>let</b> slashed_validators = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_compute_slashed_validators">compute_slashed_validators</a>();
    // <b>let</b> total_slashed_validator_voting_power = self.sum_voting_power_by_validator_indices(
    //     slashed_validators,
    // );
    <b>let</b> total_slashed_validator_voting_power = slashed_validators.length();
    <b>let</b> slashed_validator_indices = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator_indices">get_validator_indices</a>(&slashed_validators);
    // Compute the reward adjustments of slashed validators, to be taken into
    // account in adjusted reward computation.
    <b>let</b> (
        total_staking_reward_adjustment,
        individual_staking_reward_adjustments,
    ) = <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_compute_reward_adjustments">compute_reward_adjustments</a>(
        slashed_validator_indices,
        self.reward_slashing_rate,
        &unadjusted_staking_reward_amounts,
    );
    // Compute the adjusted amounts of stake each <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> should get given the tallying rule
    // reward adjustments we computed before.
    // `<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_compute_adjusted_reward_distribution">compute_adjusted_reward_distribution</a>` must be called before `<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_distribute_reward">distribute_reward</a>` and `adjust_stake_and_computation_price` to
    // make sure we are using the current epoch's stake information to compute reward distribution.
    <b>let</b> (
        adjusted_staking_reward_amounts,
    ) = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_compute_adjusted_reward_distribution">compute_adjusted_reward_distribution</a>(
        total_voting_power,
        total_slashed_validator_voting_power,
        unadjusted_staking_reward_amounts,
        total_staking_reward_adjustment,
        individual_staking_reward_adjustments
    );
    // Distribute the rewards before adjusting stake so that we immediately start compounding
    // the rewards <b>for</b> validators and stakers.
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_distribute_reward">distribute_reward</a>(
        new_epoch,
        &adjusted_staking_reward_amounts,
        total_reward
    );
    self.previous_committee = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_active_committee">active_committee</a>;
    // Change to the next <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> committee
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_active_committee">active_committee</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_next_epoch_active_committee">next_epoch_active_committee</a>.extract();
    // Activate validators that were added during `<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_process_mid_epoch">process_mid_epoch</a>`
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_activate_added_validators">activate_added_validators</a>(new_epoch);
    self.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set">pending_active_set</a>.borrow_mut().reset_validator_changes();
    // Emit events after we have processed all the rewards distribution and pending stakes.
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_emit_validator_epoch_events">emit_validator_epoch_events</a>(
        new_epoch,
        &adjusted_staking_reward_amounts,
        &slashed_validators,
    );
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_total_stake">total_stake</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_calculate_total_stakes">calculate_total_stakes</a>();
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_activate_added_validators"></a>

## Function `activate_added_validators`



<pre><code><b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_activate_added_validators">activate_added_validators</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, new_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_activate_added_validators">activate_added_validators</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    new_epoch: u64,
) {
    <b>let</b> members = *self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_active_committee">active_committee</a>.members();
    members.do!(|member| {
        <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator_mut">get_validator_mut</a>(member.validator_id());
        <b>if</b>(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.activation_epoch().is_some_and!(|epoch| epoch == new_epoch)) {
            <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_advance_epoch">advance_epoch</a>(balance::zero(), new_epoch);
            event::emit(<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorJoinEvent">ValidatorJoinEvent</a> {
                epoch: new_epoch,
                validator_id: <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.validator_id(),
            });
        };
    });
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_set_min_validator_count"></a>

## Function `set_min_validator_count`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_min_validator_count">set_min_validator_count</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, min_validator_count: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_min_validator_count">set_min_validator_count</a>(self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>, min_validator_count: u64) {
    self.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set">pending_active_set</a>.borrow_mut().<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_min_validator_count">set_min_validator_count</a>(min_validator_count);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_set_max_validator_count"></a>

## Function `set_max_validator_count`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_max_validator_count">set_max_validator_count</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, max_validator_count: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_max_validator_count">set_max_validator_count</a>(self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>, max_validator_count: u64) {
    self.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set">pending_active_set</a>.borrow_mut().<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_max_validator_count">set_max_validator_count</a>(max_validator_count);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_set_min_validator_joining_stake"></a>

## Function `set_min_validator_joining_stake`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_min_validator_joining_stake">set_min_validator_joining_stake</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, min_validator_joining_stake: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_min_validator_joining_stake">set_min_validator_joining_stake</a>(self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>, min_validator_joining_stake: u64) {
    self.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set">pending_active_set</a>.borrow_mut().<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_min_validator_joining_stake">set_min_validator_joining_stake</a>(min_validator_joining_stake);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_set_max_validator_change_count"></a>

## Function `set_max_validator_change_count`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_max_validator_change_count">set_max_validator_change_count</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, max_validator_change_count: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_max_validator_change_count">set_max_validator_change_count</a>(self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>, max_validator_change_count: u64) {
    self.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set">pending_active_set</a>.borrow_mut().<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_max_validator_change_count">set_max_validator_change_count</a>(max_validator_change_count);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_set_reward_slashing_rate"></a>

## Function `set_reward_slashing_rate`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_reward_slashing_rate">set_reward_slashing_rate</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, reward_slashing_rate: u16)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_set_reward_slashing_rate">set_reward_slashing_rate</a>(self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>, reward_slashing_rate: u16) {
    self.reward_slashing_rate = reward_slashing_rate;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_total_stake"></a>

## Function `total_stake`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_total_stake">total_stake</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_total_stake">total_stake</a>(self: &<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>): u64 {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_total_stake">total_stake</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_validator_total_stake_amount"></a>

## Function `validator_total_stake_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_validator_total_stake_amount">validator_total_stake_amount</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_validator_total_stake_amount">validator_total_stake_amount</a>(self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>, validator_id: ID): u64 {
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator">get_validator</a>(self, validator_id);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.ika_balance()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_token_exchange_rates"></a>

## Function `token_exchange_rates`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_token_exchange_rates">token_exchange_rates</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): &<a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, (ika_system=0x0)::<a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_TokenExchangeRate">token_exchange_rate::TokenExchangeRate</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_token_exchange_rates">token_exchange_rates</a>(
    self: &<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    validator_id: ID,
): &Table&lt;u64, TokenExchangeRate&gt; {
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator">get_validator</a>(validator_id);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.exchange_rates()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_pending_active_validators_count"></a>

## Function `pending_active_validators_count`

Get the total number of pending validators.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_pending_active_validators_count">pending_active_validators_count</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_pending_active_validators_count">pending_active_validators_count</a>(self: &<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>): u64 {
    self.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set">pending_active_set</a>.borrow().size()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_is_active_validator"></a>

## Function `is_active_validator`

Returns true if exists in active validators.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_is_active_validator">is_active_validator</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_is_active_validator">is_active_validator</a>(
    self: &<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    validator_id: ID,
): bool {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_active_committee">active_committee</a>.contains(&validator_id)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_get_reporters_of"></a>

## Function `get_reporters_of`

Returns all the validators who are currently reporting <code>validator_id</code>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_reporters_of">get_reporters_of</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): <a href="../sui/vec_set.md#sui_vec_set_VecSet">sui::vec_set::VecSet</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_reporters_of">get_reporters_of</a>(self: &<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>, validator_id: ID): VecSet&lt;ID&gt; {
    <b>if</b> (self.validator_report_records.contains(&validator_id)) {
        self.validator_report_records[&validator_id]
    } <b>else</b> {
        vec_set::empty()
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_is_duplicate_with_pending_validator"></a>

## Function `is_duplicate_with_pending_validator`

Checks whether <code>new_validator</code> is duplicate with any currently pending validators in the pending active set.


<pre><code><b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_is_duplicate_with_pending_validator">is_duplicate_with_pending_validator</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, new_validator: &(ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_is_duplicate_with_pending_validator">is_duplicate_with_pending_validator</a>(self: &<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>, new_validator: &Validator): bool {
    <b>let</b> pending_active_validator_ids = self.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set">pending_active_set</a>.borrow().active_ids();
    pending_active_validator_ids.any!(|id| {
        <b>if</b>(new_validator.validator_id() == *id) {
            <b>false</b>
        } <b>else</b> {
            <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator">get_validator</a>(*id);
            <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>().is_duplicate(new_validator.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>())
        }
    })
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_get_validator_mut"></a>

## Function `get_validator_mut`

Get mutable reference to a validator by id.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator_mut">get_validator_mut</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator_mut">get_validator_mut</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    validator_id: ID,
): &<b>mut</b> Validator {
    <b>assert</b>!(self.validators.contains(validator_id), <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ENotAValidator">ENotAValidator</a>);
    self.validators.borrow_mut(validator_id)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_get_validator"></a>

## Function `get_validator`

Get reference to a validator by id.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator">get_validator</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): &(ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator">get_validator</a>(self: &<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>, validator_id: ID): &Validator {
    <b>assert</b>!(self.validators.contains(validator_id), <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ENotAValidator">ENotAValidator</a>);
    self.validators.borrow(validator_id)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_get_validator_indices"></a>

## Function `get_validator_indices`

Given a vector of validator ids to look for, return their indices in the validator vector.
Aborts if any id isn't in the given validator vector.


<pre><code><b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator_indices">get_validator_indices</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, look_for_indices_ids: &vector&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;): vector&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator_indices">get_validator_indices</a>(
    self: &<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    look_for_indices_ids: &vector&lt;ID&gt;,
): vector&lt;u64&gt; {
    <b>let</b> validators = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_active_committee">active_committee</a>.validator_ids();
    <b>let</b> length = look_for_indices_ids.length();
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> <b>mut</b> res = vector[];
    <b>while</b> (i &lt; length) {
        <b>let</b> validator_id = look_for_indices_ids[i];
        <b>let</b> (found, index_opt) = validators.index_of(&validator_id);
        <b>assert</b>!(found, <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ENotAValidator">ENotAValidator</a>);
        res.push_back(index_opt);
        i = i + 1;
    };
    res
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_verify_operation_cap"></a>

## Function `verify_operation_cap`

Verify the operation capability is valid for a Validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_verify_operation_cap">verify_operation_cap</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_verify_operation_cap">verify_operation_cap</a>(
    self: &<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    cap: &ValidatorOperationCap,
) {
    <b>let</b> validator_id = cap.validator_id();
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator">get_validator</a>(validator_id);
    <b>assert</b>!(<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.operation_cap_id() == &object::id(cap), <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_EInvalidCap">EInvalidCap</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_process_pending_validators"></a>

## Function `process_pending_validators`

Process the pending new validators. They will be <code><a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_next_epoch_active_committee">next_epoch_active_committee</a></code> and activated during <code><a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_advance_epoch">advance_epoch</a></code>.


<pre><code><b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_process_pending_validators">process_pending_validators</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_process_pending_validators">process_pending_validators</a>(self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>) {
    <b>let</b> <b>mut</b> next_epoch_active_members = vector[];
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> pending_active_validator_ids = self.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set">pending_active_set</a>.borrow().active_ids();
    <b>let</b> length = pending_active_validator_ids.length();
    <b>while</b> (i &lt; length) {
        <b>let</b> validator_id = pending_active_validator_ids[i];
        <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
        next_epoch_active_members.push_back(new_bls_committee_member(validator_id, *<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>().protocol_pubkey()));
        i = i + 1;
    };
    <b>let</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_next_epoch_active_committee">next_epoch_active_committee</a> = new_bls_committee(next_epoch_active_members);
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_next_epoch_active_committee">next_epoch_active_committee</a>.fill(<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_next_epoch_active_committee">next_epoch_active_committee</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_calculate_total_stakes"></a>

## Function `calculate_total_stakes`

Calculate the total active validator stake.


<pre><code><b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_calculate_total_stakes">calculate_total_stakes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_calculate_total_stakes">calculate_total_stakes</a>(self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>): u64 {
    <b>let</b> <b>mut</b> stake = 0;
    <b>let</b> members = *self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_active_committee">active_committee</a>.members();
    members.do!(|member| {
        <b>let</b> validator_id = member.validator_id();
        <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator_mut">get_validator_mut</a>(validator_id);
        stake = stake + <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.ika_balance();
    });
    stake
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_compute_reward_adjustments"></a>

## Function `compute_reward_adjustments`

Compute both the individual reward adjustments and total reward adjustment for staking rewards
as well as storage fund rewards.


<pre><code><b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_compute_reward_adjustments">compute_reward_adjustments</a>(slashed_validator_indices: vector&lt;u64&gt;, reward_slashing_rate: u16, unadjusted_staking_reward_amounts: &vector&lt;u64&gt;): (u64, <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u64, u64&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_compute_reward_adjustments">compute_reward_adjustments</a>(
    <b>mut</b> slashed_validator_indices: vector&lt;u64&gt;,
    reward_slashing_rate: u16,
    unadjusted_staking_reward_amounts: &vector&lt;u64&gt;,
): (
    u64, // sum of staking reward adjustments
    VecMap&lt;u64, u64&gt;, // mapping of individual <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>'s staking reward adjustment from index -&gt; amount
) {
    <b>let</b> <b>mut</b> total_staking_reward_adjustment = 0;
    <b>let</b> <b>mut</b> individual_staking_reward_adjustments = vec_map::empty();
    <b>while</b> (!slashed_validator_indices.is_empty()) {
        <b>let</b> validator_index = slashed_validator_indices.pop_back();
        // Use the slashing rate to compute the amount of staking rewards slashed from this punished <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.
        <b>let</b> unadjusted_staking_reward = unadjusted_staking_reward_amounts[validator_index];
        <b>let</b> staking_reward_adjustment_u128 =
            unadjusted_staking_reward <b>as</b> u128 * (reward_slashing_rate <b>as</b> u128)
                / <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_BASIS_POINT_DENOMINATOR_U128">BASIS_POINT_DENOMINATOR_U128</a>;
        // Insert into individual mapping and record into the total adjustment sum.
        individual_staking_reward_adjustments.insert(
            validator_index,
            staking_reward_adjustment_u128 <b>as</b> u64,
        );
        total_staking_reward_adjustment =
            total_staking_reward_adjustment + (staking_reward_adjustment_u128 <b>as</b> u64);
    };
    (
        total_staking_reward_adjustment,
        individual_staking_reward_adjustments,
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_compute_slashed_validators"></a>

## Function `compute_slashed_validators`

Process the validator report records of the epoch and return the ids of the
non-performant validators according to the input threshold.


<pre><code><b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_compute_slashed_validators">compute_slashed_validators</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>): vector&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_compute_slashed_validators">compute_slashed_validators</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
): vector&lt;ID&gt; {
    <b>let</b> <b>mut</b> slashed_validators = vector[];
    <b>while</b> (!self.validator_report_records.is_empty()) {
        <b>let</b> (validator_id, reporters) = self.validator_report_records.pop();
        <b>assert</b>!(
            <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_is_active_validator">is_active_validator</a>(self, validator_id),
            <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ENonValidatorInReportRecords">ENonValidatorInReportRecords</a>,
        );
        // Sum up the voting power of validators that have reported this <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> and check <b>if</b> it <b>has</b>
        // passed the slashing threshold.
        // <b>let</b> reporter_votes = sum_voting_power_by_validator_indices(
        //     self,
        //     reporters.into_keys(),
        // );
        <b>let</b> reporter_votes = reporters.size();
        //<b>if</b> (reporter_votes &gt;= quorum_threshold()) {
        <b>if</b> (self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_active_committee">active_committee</a>.is_quorum_threshold(reporter_votes)) {
            slashed_validators.push_back(validator_id);
        }
    };
    slashed_validators
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_compute_unadjusted_reward_distribution"></a>

## Function `compute_unadjusted_reward_distribution`

Given the current list of active validators, the total stake and total reward,
calculate the amount of reward each validator should get, without taking into
account the tallying rule results.
Returns the unadjusted amounts of staking reward for each validator.


<pre><code><b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_compute_unadjusted_reward_distribution">compute_unadjusted_reward_distribution</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, total_voting_power: u64, total_reward: u64): vector&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_compute_unadjusted_reward_distribution">compute_unadjusted_reward_distribution</a>(
    self: &<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    total_voting_power: u64,
    total_reward: u64,
): vector&lt;u64&gt; {
    <b>let</b> members = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_active_committee">active_committee</a>.members();
    <b>let</b> reward_amounts = members.map_ref!(|_| {
        // Integer divisions will truncate the results. Because of this, we expect that at the end
        // there will be some reward remaining in `total_reward`.
        // Use u128 to avoid multiplication overflow.
        <b>let</b> reward_amount =
            (total_reward <b>as</b> u128) / (total_voting_power <b>as</b> u128);
        reward_amount <b>as</b> u64
    });
    reward_amounts
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_compute_adjusted_reward_distribution"></a>

## Function `compute_adjusted_reward_distribution`

Use the reward adjustment info to compute the adjusted rewards each validator should get.
Returns the staking rewards each validator gets.
The staking rewards are shared with the stakers.


<pre><code><b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_compute_adjusted_reward_distribution">compute_adjusted_reward_distribution</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, total_voting_power: u64, total_slashed_validator_voting_power: u64, unadjusted_staking_reward_amounts: vector&lt;u64&gt;, total_staking_reward_adjustment: u64, individual_staking_reward_adjustments: <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u64, u64&gt;): vector&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_compute_adjusted_reward_distribution">compute_adjusted_reward_distribution</a>(
    self: &<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    total_voting_power: u64,
    total_slashed_validator_voting_power: u64,
    unadjusted_staking_reward_amounts: vector&lt;u64&gt;,
    total_staking_reward_adjustment: u64,
    individual_staking_reward_adjustments: VecMap&lt;u64, u64&gt;,
): vector&lt;u64&gt; {
    <b>let</b> total_unslashed_validator_voting_power =
        total_voting_power - total_slashed_validator_voting_power;
    <b>let</b> <b>mut</b> adjusted_staking_reward_amounts = vector[];
    <b>let</b> members = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_active_committee">active_committee</a>.members();
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> length = members.length();
    <b>while</b> (i &lt; length) {
        // Integer divisions will truncate the results. Because of this, we expect that at the end
        // there will be some reward remaining in `total_reward`.
        // Use u128 to avoid multiplication overflow.
        // Compute adjusted staking reward.
        <b>let</b> unadjusted_staking_reward_amount = unadjusted_staking_reward_amounts[i];
        <b>let</b> adjusted_staking_reward_amount = // If the <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> is one of the slashed ones, then subtract the adjustment.
        <b>if</b> (individual_staking_reward_adjustments.contains(&i)) {
            <b>let</b> adjustment = individual_staking_reward_adjustments[&i];
            unadjusted_staking_reward_amount - adjustment
        } <b>else</b> {
            // Otherwise the slashed rewards should be distributed among the unslashed
            // validators so add the corresponding adjustment.
            <b>let</b> adjustment =
                total_staking_reward_adjustment <b>as</b> u128
                                   / (total_unslashed_validator_voting_power <b>as</b> u128);
            unadjusted_staking_reward_amount + (adjustment <b>as</b> u64)
        };
        adjusted_staking_reward_amounts.push_back(adjusted_staking_reward_amount);
        i = i + 1;
    };
    adjusted_staking_reward_amounts
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_distribute_reward"></a>

## Function `distribute_reward`

Distribute rewards to validators and stakers


<pre><code><b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_distribute_reward">distribute_reward</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, new_epoch: u64, adjusted_staking_reward_amounts: &vector&lt;u64&gt;, staking_rewards: &<b>mut</b> <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_distribute_reward">distribute_reward</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    new_epoch: u64,
    adjusted_staking_reward_amounts: &vector&lt;u64&gt;,
    staking_rewards: &<b>mut</b> Balance&lt;IKA&gt;,
) {
    <b>let</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set">pending_active_set</a> = self.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set">pending_active_set</a>.borrow_mut();
    <b>let</b> members = *self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_active_committee">active_committee</a>.members();
    <b>let</b> length = members.length();
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; length) {
        <b>let</b> validator_id = members[i].validator_id();
        <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = &<b>mut</b> self.validators[validator_id];
        <b>let</b> staking_reward_amount = adjusted_staking_reward_amounts[i];
        <b>let</b> validator_rewards = staking_rewards.split(staking_reward_amount);
        <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_advance_epoch">advance_epoch</a>(validator_rewards, new_epoch);
        <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set">pending_active_set</a>.update(validator_id, <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.ika_balance_at_epoch(new_epoch));
        i = i + 1;
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_emit_validator_epoch_events"></a>

## Function `emit_validator_epoch_events`

Emit events containing information of each validator for the epoch,
including stakes, rewards, performance, etc.


<pre><code><b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_emit_validator_epoch_events">emit_validator_epoch_events</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, new_epoch: u64, staking_rewards_amounts: &vector&lt;u64&gt;, slashed_validators: &vector&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_emit_validator_epoch_events">emit_validator_epoch_events</a>(
    self: &<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    new_epoch: u64,
    staking_rewards_amounts: &vector&lt;u64&gt;,
    slashed_validators: &vector&lt;ID&gt;,
) {
    <b>let</b> members = *self.previous_committee.members();
    <b>let</b> num_validators = members.length();
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; num_validators) {
        <b>let</b> member = members[i];
        <b>let</b> validator_id = member.validator_id();
        <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator">get_validator</a>(validator_id);
        <b>let</b> tallying_rule_reporters = <b>if</b> (self.validator_report_records.contains(&validator_id)) {
            self.validator_report_records[&validator_id].into_keys()
        } <b>else</b> {
            vector[]
        };
        <b>let</b> tallying_rule_global_score = <b>if</b> (slashed_validators.contains(&validator_id)) 0
        <b>else</b> 1;
        event::emit(<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorEpochInfoEventV1">ValidatorEpochInfoEventV1</a> {
            epoch: new_epoch,
            validator_id,
            //reference_gas_survey_quote: <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.computation_price(),
            stake: <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.ika_balance(),
            commission_rate: <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.commission_rate(),
            staking_rewards: staking_rewards_amounts[i],
            <a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate">token_exchange_rate</a>: <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.exchange_rate_at_epoch(new_epoch),
            tallying_rule_reporters,
            tallying_rule_global_score,
        });
        i = i + 1;
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_report_validator"></a>

## Function `report_validator`

Report a validator as a bad or non-performant actor in the system.
Succeeds if all the following are satisfied:
1. both the reporter in <code>cap</code> and the input <code>reportee_id</code> are active validators.
2. reporter and reportee not the same address.
3. the cap object is still valid.
This function is idempotent.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_report_validator">report_validator</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, operation_cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, reportee_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_report_validator">report_validator</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    operation_cap: &ValidatorOperationCap,
    reportee_id: ID,
) {
    // Reportee needs to be an active <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>
    <b>assert</b>!(self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_is_active_validator">is_active_validator</a>(reportee_id), <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ENotAValidator">ENotAValidator</a>);
    // Verify the represented reporter <b>address</b> is an active <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>, and the capability is still valid.
    <b>assert</b>!(self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_is_active_validator">is_active_validator</a>(operation_cap.validator_id()), <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ENotAValidator">ENotAValidator</a>);
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_verify_operation_cap">verify_operation_cap</a>(operation_cap);
    <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_report_validator_impl">report_validator_impl</a>(operation_cap, reportee_id, &<b>mut</b> self.validator_report_records);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_undo_report_validator"></a>

## Function `undo_report_validator`

Undo a <code><a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_report_validator">report_validator</a></code> action. Aborts if
1. the reportee is not a currently active validator or
2. the sender has not previously reported the <code>reportee_id</code>, or
3. the cap is not valid


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_undo_report_validator">undo_report_validator</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, operation_cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, reportee_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_undo_report_validator">undo_report_validator</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    operation_cap: &ValidatorOperationCap,
    reportee_id: ID,
) {
    // Verify the represented reporter <b>address</b> is an active <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>, and the capability is still valid.
    <b>assert</b>!(self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_is_active_validator">is_active_validator</a>(operation_cap.validator_id()), <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ENotAValidator">ENotAValidator</a>);
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_verify_operation_cap">verify_operation_cap</a>(operation_cap);
    <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_undo_report_validator_impl">undo_report_validator_impl</a>(operation_cap, reportee_id, &<b>mut</b> self.validator_report_records);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_report_validator_impl"></a>

## Function `report_validator_impl`



<pre><code><b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_report_validator_impl">report_validator_impl</a>(operation_cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, reportee_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, validator_report_records: &<b>mut</b> <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, <a href="../sui/vec_set.md#sui_vec_set_VecSet">sui::vec_set::VecSet</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_report_validator_impl">report_validator_impl</a>(
    operation_cap: &ValidatorOperationCap,
    reportee_id: ID,
    validator_report_records: &<b>mut</b> VecMap&lt;ID, VecSet&lt;ID&gt;&gt;,
) {
    <b>let</b> reporter_id = operation_cap.validator_id();
    <b>assert</b>!(reporter_id != reportee_id, <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ECannotReportOneself">ECannotReportOneself</a>);
    <b>if</b> (!validator_report_records.contains(&reportee_id)) {
        validator_report_records.insert(reportee_id, vec_set::singleton(reporter_id));
    } <b>else</b> {
        <b>let</b> reporters = validator_report_records.get_mut(&reportee_id);
        <b>if</b> (!reporters.contains(&reporter_id)) {
            reporters.insert(reporter_id);
        }
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_undo_report_validator_impl"></a>

## Function `undo_report_validator_impl`



<pre><code><b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_undo_report_validator_impl">undo_report_validator_impl</a>(operation_cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, reportee_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, validator_report_records: &<b>mut</b> <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, <a href="../sui/vec_set.md#sui_vec_set_VecSet">sui::vec_set::VecSet</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_undo_report_validator_impl">undo_report_validator_impl</a>(
    operation_cap: &ValidatorOperationCap,
    reportee_id: ID,
    validator_report_records: &<b>mut</b> VecMap&lt;ID, VecSet&lt;ID&gt;&gt;,
) {
    <b>assert</b>!(validator_report_records.contains(&reportee_id), <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_EReportRecordNotFound">EReportRecordNotFound</a>);
    <b>let</b> reporters = validator_report_records.get_mut(&reportee_id);
    <b>let</b> reporter_id = operation_cap.validator_id();
    <b>assert</b>!(reporters.contains(&reporter_id), <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_EReportRecordNotFound">EReportRecordNotFound</a>);
    reporters.remove(&reporter_id);
    <b>if</b> (reporters.is_empty()) {
        validator_report_records.remove(&reportee_id);
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_active_committee"></a>

## Function `active_committee`

Return the active validators in <code>self</code>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_active_committee">active_committee</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>): (ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">bls_committee::BlsCommittee</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_active_committee">active_committee</a>(self: &<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>): BlsCommittee {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_active_committee">active_committee</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_next_epoch_active_committee"></a>

## Function `next_epoch_active_committee`

Return the next epoch active committee in <code>self</code>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_next_epoch_active_committee">next_epoch_active_committee</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;(ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">bls_committee::BlsCommittee</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_next_epoch_active_committee">next_epoch_active_committee</a>(self: &<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>): Option&lt;BlsCommittee&gt; {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_next_epoch_active_committee">next_epoch_active_committee</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_pending_active_set"></a>

## Function `pending_active_set`

Return the pending active set in <code>self</code>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set">pending_active_set</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>): &(ika_system=0x0)::<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">pending_active_set::PendingActiveSet</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set">pending_active_set</a>(self: &<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>): &PendingActiveSet {
    self.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set">pending_active_set</a>.borrow()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_is_validator_candidate"></a>

## Function `is_validator_candidate`

Returns true if the <code>validator_id</code> is a validator candidate.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_is_validator_candidate">is_validator_candidate</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_is_validator_candidate">is_validator_candidate</a>(self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>, validator_id: ID): bool {
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator">get_validator</a>(validator_id);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.is_preactive()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_is_inactive_validator"></a>

## Function `is_inactive_validator`

Returns true if the validator identified by <code>validator_id</code> is of an inactive validator.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_is_inactive_validator">is_inactive_validator</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_is_inactive_validator">is_inactive_validator</a>(self: &<b>mut</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>, validator_id: ID): bool {
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator">get_validator</a>(validator_id);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.is_withdrawing()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_calculate_rewards"></a>

## Function `calculate_rewards`

Calculate the rewards for an amount with value <code>staked_principal</code>, staked in the validator with
the given <code>validator_id</code> between <code>activation_epoch</code> and <code>withdraw_epoch</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_calculate_rewards">calculate_rewards</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, staked_principal: u64, activation_epoch: u64, withdraw_epoch: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_calculate_rewards">calculate_rewards</a>(
    self: &<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    validator_id: ID,
    staked_principal: u64,
    activation_epoch: u64,
    withdraw_epoch: u64,
): u64 {
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_get_validator">get_validator</a>(validator_id);
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_calculate_rewards">calculate_rewards</a>(staked_principal, activation_epoch, withdraw_epoch)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_set_can_withdraw_staked_ika_early"></a>

## Function `can_withdraw_staked_ika_early`

Check whether StakedIka can be withdrawn directly.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_can_withdraw_staked_ika_early">can_withdraw_staked_ika_early</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: &(ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>, current_epoch: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_can_withdraw_staked_ika_early">can_withdraw_staked_ika_early</a>(
    self: &<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">ValidatorSet</a>,
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: &StakedIka,
    current_epoch: u64,
): bool {
    <b>let</b> validator_id = <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.validator_id();
    <b>let</b> is_next_committee = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_next_epoch_active_committee">next_epoch_active_committee</a>.is_some_and!(|c| c.contains(&validator_id));
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.can_withdraw_early(is_next_committee, current_epoch)
}
</code></pre>



</details>
