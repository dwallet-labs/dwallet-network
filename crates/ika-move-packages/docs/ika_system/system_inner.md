---
title: Module `(ika_system=0x0)::system_inner`
---



-  [Struct `SystemInner`](#(ika_system=0x0)_system_inner_SystemInner)
-  [Struct `SystemEpochInfoEvent`](#(ika_system=0x0)_system_inner_SystemEpochInfoEvent)
-  [Struct `SystemCheckpointInfoEvent`](#(ika_system=0x0)_system_inner_SystemCheckpointInfoEvent)
-  [Struct `SetNextProtocolVersionEvent`](#(ika_system=0x0)_system_inner_SetNextProtocolVersionEvent)
-  [Struct `SetEpochDurationMsEvent`](#(ika_system=0x0)_system_inner_SetEpochDurationMsEvent)
-  [Struct `SetStakeSubsidyStartEpochEvent`](#(ika_system=0x0)_system_inner_SetStakeSubsidyStartEpochEvent)
-  [Struct `SetStakeSubsidyRateEvent`](#(ika_system=0x0)_system_inner_SetStakeSubsidyRateEvent)
-  [Struct `SetStakeSubsidyPeriodLengthEvent`](#(ika_system=0x0)_system_inner_SetStakeSubsidyPeriodLengthEvent)
-  [Struct `SetMinValidatorCountEvent`](#(ika_system=0x0)_system_inner_SetMinValidatorCountEvent)
-  [Struct `SetMaxValidatorCountEvent`](#(ika_system=0x0)_system_inner_SetMaxValidatorCountEvent)
-  [Struct `SetMinValidatorJoiningStakeEvent`](#(ika_system=0x0)_system_inner_SetMinValidatorJoiningStakeEvent)
-  [Struct `SetMaxValidatorChangeCountEvent`](#(ika_system=0x0)_system_inner_SetMaxValidatorChangeCountEvent)
-  [Struct `SetRewardSlashingRateEvent`](#(ika_system=0x0)_system_inner_SetRewardSlashingRateEvent)
-  [Struct `SetApprovedUpgradeEvent`](#(ika_system=0x0)_system_inner_SetApprovedUpgradeEvent)
-  [Struct `EndOfPublishEvent`](#(ika_system=0x0)_system_inner_EndOfPublishEvent)
-  [Struct `SetOrRemoveWitnessApprovingAdvanceEpochEvent`](#(ika_system=0x0)_system_inner_SetOrRemoveWitnessApprovingAdvanceEpochEvent)
-  [Constants](#@Constants_0)
-  [Function `create`](#(ika_system=0x0)_system_inner_create)
-  [Function `initialize`](#(ika_system=0x0)_system_inner_initialize)
-  [Function `request_add_validator_candidate`](#(ika_system=0x0)_system_inner_request_add_validator_candidate)
-  [Function `request_remove_validator_candidate`](#(ika_system=0x0)_system_inner_request_remove_validator_candidate)
-  [Function `request_add_validator`](#(ika_system=0x0)_system_inner_request_add_validator)
-  [Function `request_remove_validator`](#(ika_system=0x0)_system_inner_request_remove_validator)
-  [Function `validator_metadata`](#(ika_system=0x0)_system_inner_validator_metadata)
-  [Function `set_validator_metadata`](#(ika_system=0x0)_system_inner_set_validator_metadata)
-  [Function `set_next_commission`](#(ika_system=0x0)_system_inner_set_next_commission)
-  [Function `request_add_stake`](#(ika_system=0x0)_system_inner_request_add_stake)
-  [Function `request_withdraw_stake`](#(ika_system=0x0)_system_inner_request_withdraw_stake)
-  [Function `withdraw_stake`](#(ika_system=0x0)_system_inner_withdraw_stake)
-  [Function `report_validator`](#(ika_system=0x0)_system_inner_report_validator)
-  [Function `undo_report_validator`](#(ika_system=0x0)_system_inner_undo_report_validator)
-  [Function `rotate_operation_cap`](#(ika_system=0x0)_system_inner_rotate_operation_cap)
-  [Function `rotate_commission_cap`](#(ika_system=0x0)_system_inner_rotate_commission_cap)
-  [Function `collect_commission`](#(ika_system=0x0)_system_inner_collect_commission)
-  [Function `set_validator_name`](#(ika_system=0x0)_system_inner_set_validator_name)
-  [Function `set_next_epoch_network_address`](#(ika_system=0x0)_system_inner_set_next_epoch_network_address)
-  [Function `set_next_epoch_p2p_address`](#(ika_system=0x0)_system_inner_set_next_epoch_p2p_address)
-  [Function `set_next_epoch_consensus_address`](#(ika_system=0x0)_system_inner_set_next_epoch_consensus_address)
-  [Function `set_next_epoch_protocol_pubkey_bytes`](#(ika_system=0x0)_system_inner_set_next_epoch_protocol_pubkey_bytes)
-  [Function `set_next_epoch_network_pubkey_bytes`](#(ika_system=0x0)_system_inner_set_next_epoch_network_pubkey_bytes)
-  [Function `set_next_epoch_consensus_pubkey_bytes`](#(ika_system=0x0)_system_inner_set_next_epoch_consensus_pubkey_bytes)
-  [Function `set_next_epoch_class_groups_pubkey_and_proof_bytes`](#(ika_system=0x0)_system_inner_set_next_epoch_class_groups_pubkey_and_proof_bytes)
-  [Function `is_mid_epoch_time`](#(ika_system=0x0)_system_inner_is_mid_epoch_time)
-  [Function `is_end_epoch_time`](#(ika_system=0x0)_system_inner_is_end_epoch_time)
-  [Function `assert_mid_epoch_time`](#(ika_system=0x0)_system_inner_assert_mid_epoch_time)
-  [Function `assert_end_epoch_time`](#(ika_system=0x0)_system_inner_assert_end_epoch_time)
-  [Function `create_system_current_status_info`](#(ika_system=0x0)_system_inner_create_system_current_status_info)
-  [Function `initiate_advance_epoch`](#(ika_system=0x0)_system_inner_initiate_advance_epoch)
-  [Function `advance_epoch`](#(ika_system=0x0)_system_inner_advance_epoch)
-  [Function `initiate_mid_epoch_reconfiguration`](#(ika_system=0x0)_system_inner_initiate_mid_epoch_reconfiguration)
-  [Function `epoch`](#(ika_system=0x0)_system_inner_epoch)
-  [Function `protocol_version`](#(ika_system=0x0)_system_inner_protocol_version)
-  [Function `epoch_start_timestamp_ms`](#(ika_system=0x0)_system_inner_epoch_start_timestamp_ms)
-  [Function `validator_stake_amount`](#(ika_system=0x0)_system_inner_validator_stake_amount)
-  [Function `get_reporters_of`](#(ika_system=0x0)_system_inner_get_reporters_of)
-  [Function `token_exchange_rates`](#(ika_system=0x0)_system_inner_token_exchange_rates)
-  [Function `active_committee`](#(ika_system=0x0)_system_inner_active_committee)
-  [Function `next_epoch_active_committee`](#(ika_system=0x0)_system_inner_next_epoch_active_committee)
-  [Function `verify_validator_cap`](#(ika_system=0x0)_system_inner_verify_validator_cap)
-  [Function `verify_operation_cap`](#(ika_system=0x0)_system_inner_verify_operation_cap)
-  [Function `verify_commission_cap`](#(ika_system=0x0)_system_inner_verify_commission_cap)
-  [Function `verify_protocol_cap_impl`](#(ika_system=0x0)_system_inner_verify_protocol_cap_impl)
-  [Function `add_upgrade_cap_by_cap`](#(ika_system=0x0)_system_inner_add_upgrade_cap_by_cap)
-  [Function `authorize_upgrade`](#(ika_system=0x0)_system_inner_authorize_upgrade)
-  [Function `commit_upgrade`](#(ika_system=0x0)_system_inner_commit_upgrade)
-  [Function `process_checkpoint_message_by_quorum`](#(ika_system=0x0)_system_inner_process_checkpoint_message_by_quorum)
-  [Function `process_checkpoint_message`](#(ika_system=0x0)_system_inner_process_checkpoint_message)
-  [Function `verify_protocol_cap`](#(ika_system=0x0)_system_inner_verify_protocol_cap)
-  [Function `set_approved_upgrade_by_cap`](#(ika_system=0x0)_system_inner_set_approved_upgrade_by_cap)
-  [Function `set_or_remove_witness_approving_advance_epoch_by_cap`](#(ika_system=0x0)_system_inner_set_or_remove_witness_approving_advance_epoch_by_cap)
-  [Function `process_checkpoint_message_by_cap`](#(ika_system=0x0)_system_inner_process_checkpoint_message_by_cap)
-  [Function `set_approved_upgrade`](#(ika_system=0x0)_system_inner_set_approved_upgrade)
-  [Function `set_or_remove_witness_approving_advance_epoch`](#(ika_system=0x0)_system_inner_set_or_remove_witness_approving_advance_epoch)
-  [Function `calculate_rewards`](#(ika_system=0x0)_system_inner_calculate_rewards)
-  [Function `can_withdraw_staked_ika_early`](#(ika_system=0x0)_system_inner_can_withdraw_staked_ika_early)
-  [Function `epoch_duration_ms`](#(ika_system=0x0)_system_inner_epoch_duration_ms)


<pre><code><b>use</b> (ika=0x0)::ika;
<b>use</b> (ika_common=0x0)::bls_committee;
<b>use</b> (ika_common=0x0)::class_groups_public_key_and_proof;
<b>use</b> (ika_common=0x0)::extended_field;
<b>use</b> (ika_common=0x0)::multiaddr;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver">advance_epoch_approver</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set">pending_active_set</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values">pending_values</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap">protocol_cap</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury">protocol_treasury</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info">system_current_status_info</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate">token_exchange_rate</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap">validator_cap</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata">validator_metadata</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>;
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
<b>use</b> <a href="../sui/clock.md#sui_clock">sui::clock</a>;
<b>use</b> <a href="../sui/coin.md#sui_coin">sui::coin</a>;
<b>use</b> <a href="../sui/config.md#sui_config">sui::config</a>;
<b>use</b> <a href="../sui/deny_list.md#sui_deny_list">sui::deny_list</a>;
<b>use</b> <a href="../sui/dynamic_field.md#sui_dynamic_field">sui::dynamic_field</a>;
<b>use</b> <a href="../sui/dynamic_object_field.md#sui_dynamic_object_field">sui::dynamic_object_field</a>;
<b>use</b> <a href="../sui/event.md#sui_event">sui::event</a>;
<b>use</b> <a href="../sui/group_ops.md#sui_group_ops">sui::group_ops</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/object_table.md#sui_object_table">sui::object_table</a>;
<b>use</b> <a href="../sui/package.md#sui_package">sui::package</a>;
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



<a name="(ika_system=0x0)_system_inner_SystemInner"></a>

## Struct `SystemInner`

Uses SystemParametersV1 as the parameters.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: u64</code>
</dt>
<dd>
 The current epoch ID, starting from 0.
</dd>
<dt>
<code>epoch_start_tx_digest: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_protocol_version">protocol_version</a>: u64</code>
</dt>
<dd>
 The current protocol version, starting from 1.
</dd>
<dt>
<code>next_protocol_version: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>upgrade_caps: vector&lt;<a href="../sui/package.md#sui_package_UpgradeCap">sui::package::UpgradeCap</a>&gt;</code>
</dt>
<dd>
 Upgrade caps for this package and others like ika coin of the ika protocol.
</dd>
<dt>
<code>approved_upgrades: <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
 Approved upgrade for package id to its approved digest.
</dd>
<dt>
<code><a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>: (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a></code>
</dt>
<dd>
 Contains all information about the validators.
</dd>
<dt>
<code><a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_duration_ms">epoch_duration_ms</a>: u64</code>
</dt>
<dd>
 The duration of an epoch, in milliseconds.
</dd>
<dt>
<code>stake_subsidy_start_epoch: u64</code>
</dt>
<dd>
 The starting epoch in which stake subsidies start being paid out
</dd>
<dt>
<code><a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury">protocol_treasury</a>: (ika_system=0x0)::<a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_ProtocolTreasury">protocol_treasury::ProtocolTreasury</a></code>
</dt>
<dd>
 Schedule of stake subsidies given out each epoch.
</dd>
<dt>
<code><a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_start_timestamp_ms">epoch_start_timestamp_ms</a>: u64</code>
</dt>
<dd>
 Unix timestamp of the current epoch start.
</dd>
<dt>
<code>last_processed_checkpoint_sequence_number: u64</code>
</dt>
<dd>
 The last processed checkpoint sequence number.
</dd>
<dt>
<code>previous_epoch_last_checkpoint_sequence_number: u64</code>
</dt>
<dd>
 The last checkpoint sequence number of the previous epoch.
</dd>
<dt>
<code>total_messages_processed: u64</code>
</dt>
<dd>
 The total messages processed.
</dd>
<dt>
<code>remaining_rewards: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;</code>
</dt>
<dd>
 The fees paid for computation.
</dd>
<dt>
<code>authorized_protocol_cap_ids: vector&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;</code>
</dt>
<dd>
 List of authorized protocol cap ids.
</dd>
<dt>
<code>witness_approving_advance_epoch: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
 List of witnesses approving advance epoch.
 as part of the epoch advancement, we have to collect approval from all witnesses.
</dd>
<dt>
<code>received_end_of_publish: bool</code>
</dt>
<dd>
 Whether the system has received the end of publish message.
</dd>
<dt>
<code>extra_fields: <a href="../sui/bag.md#sui_bag_Bag">sui::bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_system_inner_SystemEpochInfoEvent"></a>

## Struct `SystemEpochInfoEvent`

Event containing system-level epoch information, emitted during
the epoch advancement message.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemEpochInfoEvent">SystemEpochInfoEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_protocol_version">protocol_version</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>total_stake: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>stake_subsidy_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>total_computation_fees: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>total_stake_rewards_distributed: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_system_inner_SystemCheckpointInfoEvent"></a>

## Struct `SystemCheckpointInfoEvent`

Event containing system-level checkpoint information, emitted during
the system checkpoint submission message.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemCheckpointInfoEvent">SystemCheckpointInfoEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>sequence_number: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_system_inner_SetNextProtocolVersionEvent"></a>

## Struct `SetNextProtocolVersionEvent`

Event emitted when protocol version is set via checkpoint message.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SetNextProtocolVersionEvent">SetNextProtocolVersionEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>next_protocol_version: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_system_inner_SetEpochDurationMsEvent"></a>

## Struct `SetEpochDurationMsEvent`

Event emitted when epoch duration is set via checkpoint message.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SetEpochDurationMsEvent">SetEpochDurationMsEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_duration_ms">epoch_duration_ms</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_system_inner_SetStakeSubsidyStartEpochEvent"></a>

## Struct `SetStakeSubsidyStartEpochEvent`

Event emitted when stake subsidy start epoch is set via checkpoint message.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SetStakeSubsidyStartEpochEvent">SetStakeSubsidyStartEpochEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>stake_subsidy_start_epoch: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_system_inner_SetStakeSubsidyRateEvent"></a>

## Struct `SetStakeSubsidyRateEvent`

Event emitted when stake subsidy rate is set via checkpoint message.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SetStakeSubsidyRateEvent">SetStakeSubsidyRateEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>stake_subsidy_rate: u16</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_system_inner_SetStakeSubsidyPeriodLengthEvent"></a>

## Struct `SetStakeSubsidyPeriodLengthEvent`

Event emitted when stake subsidy period length is set via checkpoint message.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SetStakeSubsidyPeriodLengthEvent">SetStakeSubsidyPeriodLengthEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>stake_subsidy_period_length: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_system_inner_SetMinValidatorCountEvent"></a>

## Struct `SetMinValidatorCountEvent`

Event emitted when minimum validator count is set via checkpoint message.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SetMinValidatorCountEvent">SetMinValidatorCountEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>min_validator_count: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_system_inner_SetMaxValidatorCountEvent"></a>

## Struct `SetMaxValidatorCountEvent`

Event emitted when maximum validator count is set via checkpoint message.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SetMaxValidatorCountEvent">SetMaxValidatorCountEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_validator_count: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_system_inner_SetMinValidatorJoiningStakeEvent"></a>

## Struct `SetMinValidatorJoiningStakeEvent`

Event emitted when minimum validator joining stake is set via checkpoint message.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SetMinValidatorJoiningStakeEvent">SetMinValidatorJoiningStakeEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>min_validator_joining_stake: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_system_inner_SetMaxValidatorChangeCountEvent"></a>

## Struct `SetMaxValidatorChangeCountEvent`

Event emitted when maximum validator change count is set via checkpoint message.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SetMaxValidatorChangeCountEvent">SetMaxValidatorChangeCountEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_validator_change_count: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_system_inner_SetRewardSlashingRateEvent"></a>

## Struct `SetRewardSlashingRateEvent`

Event emitted when reward slashing rate is set via checkpoint message.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SetRewardSlashingRateEvent">SetRewardSlashingRateEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reward_slashing_rate: u16</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_system_inner_SetApprovedUpgradeEvent"></a>

## Struct `SetApprovedUpgradeEvent`

Event emitted when approved upgrade is set via checkpoint message.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SetApprovedUpgradeEvent">SetApprovedUpgradeEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>package_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>digest: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_system_inner_EndOfPublishEvent"></a>

## Struct `EndOfPublishEvent`

Event emitted when the epoch ends.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EndOfPublishEvent">EndOfPublishEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_system_inner_SetOrRemoveWitnessApprovingAdvanceEpochEvent"></a>

## Struct `SetOrRemoveWitnessApprovingAdvanceEpochEvent`

Event emitted when witness approving advance epoch is set or removed via checkpoint message.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SetOrRemoveWitnessApprovingAdvanceEpochEvent">SetOrRemoveWitnessApprovingAdvanceEpochEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>witness_type: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>remove: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="(ika_system=0x0)_system_inner_PARAMS_MESSAGE_INTENT"></a>



<pre><code><b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_PARAMS_MESSAGE_INTENT">PARAMS_MESSAGE_INTENT</a>: vector&lt;u8&gt; = vector[2, 0, 0];
</code></pre>



<a name="(ika_system=0x0)_system_inner_SET_NEXT_PROTOCOL_VERSION_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SET_NEXT_PROTOCOL_VERSION_MESSAGE_TYPE">SET_NEXT_PROTOCOL_VERSION_MESSAGE_TYPE</a>: u32 = 0;
</code></pre>



<a name="(ika_system=0x0)_system_inner_SET_EPOCH_DURATION_MS_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SET_EPOCH_DURATION_MS_MESSAGE_TYPE">SET_EPOCH_DURATION_MS_MESSAGE_TYPE</a>: u32 = 1;
</code></pre>



<a name="(ika_system=0x0)_system_inner_SET_STAKE_SUBSIDY_START_EPOCH_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SET_STAKE_SUBSIDY_START_EPOCH_MESSAGE_TYPE">SET_STAKE_SUBSIDY_START_EPOCH_MESSAGE_TYPE</a>: u32 = 2;
</code></pre>



<a name="(ika_system=0x0)_system_inner_SET_STAKE_SUBSIDY_RATE_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SET_STAKE_SUBSIDY_RATE_MESSAGE_TYPE">SET_STAKE_SUBSIDY_RATE_MESSAGE_TYPE</a>: u32 = 3;
</code></pre>



<a name="(ika_system=0x0)_system_inner_SET_STAKE_SUBSIDY_PERIOD_LENGTH_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SET_STAKE_SUBSIDY_PERIOD_LENGTH_MESSAGE_TYPE">SET_STAKE_SUBSIDY_PERIOD_LENGTH_MESSAGE_TYPE</a>: u32 = 4;
</code></pre>



<a name="(ika_system=0x0)_system_inner_SET_MIN_VALIDATOR_COUNT_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SET_MIN_VALIDATOR_COUNT_MESSAGE_TYPE">SET_MIN_VALIDATOR_COUNT_MESSAGE_TYPE</a>: u32 = 5;
</code></pre>



<a name="(ika_system=0x0)_system_inner_SET_MAX_VALIDATOR_COUNT_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SET_MAX_VALIDATOR_COUNT_MESSAGE_TYPE">SET_MAX_VALIDATOR_COUNT_MESSAGE_TYPE</a>: u32 = 6;
</code></pre>



<a name="(ika_system=0x0)_system_inner_SET_MIN_VALIDATOR_JOINING_STAKE_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SET_MIN_VALIDATOR_JOINING_STAKE_MESSAGE_TYPE">SET_MIN_VALIDATOR_JOINING_STAKE_MESSAGE_TYPE</a>: u32 = 7;
</code></pre>



<a name="(ika_system=0x0)_system_inner_SET_MAX_VALIDATOR_CHANGE_COUNT_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SET_MAX_VALIDATOR_CHANGE_COUNT_MESSAGE_TYPE">SET_MAX_VALIDATOR_CHANGE_COUNT_MESSAGE_TYPE</a>: u32 = 8;
</code></pre>



<a name="(ika_system=0x0)_system_inner_SET_REWARD_SLASHING_RATE_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SET_REWARD_SLASHING_RATE_MESSAGE_TYPE">SET_REWARD_SLASHING_RATE_MESSAGE_TYPE</a>: u32 = 9;
</code></pre>



<a name="(ika_system=0x0)_system_inner_END_OF_PUBLISH_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_END_OF_PUBLISH_MESSAGE_TYPE">END_OF_PUBLISH_MESSAGE_TYPE</a>: u32 = 10;
</code></pre>



<a name="(ika_system=0x0)_system_inner_SET_APPROVED_UPGRADE_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SET_APPROVED_UPGRADE_MESSAGE_TYPE">SET_APPROVED_UPGRADE_MESSAGE_TYPE</a>: u32 = 11;
</code></pre>



<a name="(ika_system=0x0)_system_inner_SetOrRemoveWitnessApprovingAdvanceEpochMessageType"></a>



<pre><code><b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SetOrRemoveWitnessApprovingAdvanceEpochMessageType">SetOrRemoveWitnessApprovingAdvanceEpochMessageType</a>: u32 = 12;
</code></pre>



<a name="(ika_system=0x0)_system_inner_EHaveNotReachedEndEpochTime"></a>

The system has not reached the end epoch time.


<pre><code><b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EHaveNotReachedEndEpochTime">EHaveNotReachedEndEpochTime</a>: u64 = 0;
</code></pre>



<a name="(ika_system=0x0)_system_inner_EActiveBlsCommitteeMustInitialize"></a>

The active BLS committee must be initialized.


<pre><code><b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EActiveBlsCommitteeMustInitialize">EActiveBlsCommitteeMustInitialize</a>: u64 = 1;
</code></pre>



<a name="(ika_system=0x0)_system_inner_EIncorrectEpochInSystemCheckpoint"></a>

The epoch in the system checkpoint is incorrect.


<pre><code><b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EIncorrectEpochInSystemCheckpoint">EIncorrectEpochInSystemCheckpoint</a>: u64 = 2;
</code></pre>



<a name="(ika_system=0x0)_system_inner_EWrongSystemCheckpointSequenceNumber"></a>

The sequence number in the system checkpoint is incorrect.


<pre><code><b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EWrongSystemCheckpointSequenceNumber">EWrongSystemCheckpointSequenceNumber</a>: u64 = 3;
</code></pre>



<a name="(ika_system=0x0)_system_inner_EApprovedUpgradeNotFound"></a>

The approved upgrade is not found.


<pre><code><b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EApprovedUpgradeNotFound">EApprovedUpgradeNotFound</a>: u64 = 4;
</code></pre>



<a name="(ika_system=0x0)_system_inner_EUnauthorizedProtocolCap"></a>

The protocol cap is unauthorized.


<pre><code><b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EUnauthorizedProtocolCap">EUnauthorizedProtocolCap</a>: u64 = 5;
</code></pre>



<a name="(ika_system=0x0)_system_inner_ECannotInitialize"></a>

Too early for initialization time or already initialized.


<pre><code><b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_ECannotInitialize">ECannotInitialize</a>: u64 = 6;
</code></pre>



<a name="(ika_system=0x0)_system_inner_EHaveNotReachedMidEpochTime"></a>

The system has not reached the mid epoch time.


<pre><code><b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EHaveNotReachedMidEpochTime">EHaveNotReachedMidEpochTime</a>: u64 = 7;
</code></pre>



<a name="(ika_system=0x0)_system_inner_create"></a>

## Function `create`

Create a new IkaSystemState object and make it shared.
This function will be called only once in init.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_create">create</a>(upgrade_caps: vector&lt;<a href="../sui/package.md#sui_package_UpgradeCap">sui::package::UpgradeCap</a>&gt;, <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>: (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_protocol_version">protocol_version</a>: u64, <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_start_timestamp_ms">epoch_start_timestamp_ms</a>: u64, <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_duration_ms">epoch_duration_ms</a>: u64, stake_subsidy_start_epoch: u64, <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury">protocol_treasury</a>: (ika_system=0x0)::<a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_ProtocolTreasury">protocol_treasury::ProtocolTreasury</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): ((ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, (ika_system=0x0)::<a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_ProtocolCap">protocol_cap::ProtocolCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_create">create</a>(
    upgrade_caps: vector&lt;UpgradeCap&gt;,
    <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>: ValidatorSet,
    <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_protocol_version">protocol_version</a>: u64,
    <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_start_timestamp_ms">epoch_start_timestamp_ms</a>: u64,
    <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_duration_ms">epoch_duration_ms</a>: u64,
    stake_subsidy_start_epoch: u64,
    <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury">protocol_treasury</a>: ProtocolTreasury,
    ctx: &<b>mut</b> TxContext,
): (<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>, ProtocolCap) {
    <b>let</b> <a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap">protocol_cap</a> = <a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_create">protocol_cap::create</a>(ctx);
    <b>let</b> protocol_cap_id = object::id(&<a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap">protocol_cap</a>);
    <b>let</b> authorized_protocol_cap_ids = vector[protocol_cap_id];
    // This type is fixed <b>as</b> it's created at <a href="../ika_system/init.md#(ika_system=0x0)_init">init</a>. It should not be updated during type upgrade.
    <b>let</b> system_state = <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a> {
        <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: 0,
        epoch_start_tx_digest: *ctx.digest(),
        <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_protocol_version">protocol_version</a>,
        next_protocol_version: option::none(),
        upgrade_caps,
        approved_upgrades: vec_map::empty(),
        <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>,
        <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_duration_ms">epoch_duration_ms</a>,
        stake_subsidy_start_epoch,
        <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury">protocol_treasury</a>,
        <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_start_timestamp_ms">epoch_start_timestamp_ms</a>,
        last_processed_checkpoint_sequence_number: 0,
        previous_epoch_last_checkpoint_sequence_number: 0,
        total_messages_processed: 0,
        remaining_rewards: balance::zero(),
        authorized_protocol_cap_ids,
        witness_approving_advance_epoch: vector[],
        // We advance <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a> `0` immediately, and so the network doesn't participate in it and won't
        // send `END_OF_PUBLISH` - so we shouldn't expect one, and we set `received_end_of_publish`
        // to overcome the check in `<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_advance_epoch">advance_epoch</a>()`.
        received_end_of_publish: <b>true</b>,
        extra_fields: bag::new(ctx),
    };
    (system_state, <a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap">protocol_cap</a>)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_initialize"></a>

## Function `initialize`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_initialize">initialize</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, max_validator_change_count: u64, cap: &(ika_system=0x0)::<a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_ProtocolCap">protocol_cap::ProtocolCap</a>, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>): (ika_system=0x0)::<a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver_AdvanceEpochApprover">advance_epoch_approver::AdvanceEpochApprover</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_initialize">initialize</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    max_validator_change_count: u64,
    cap: &ProtocolCap,
    clock: &Clock,
): AdvanceEpochApprover {
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_verify_protocol_cap_impl">verify_protocol_cap_impl</a>(cap);
    <b>let</b> now = clock.timestamp_ms();
    <b>assert</b>!(self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a> == 0 && now &gt;= self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_start_timestamp_ms">epoch_start_timestamp_ms</a>, <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_ECannotInitialize">ECannotInitialize</a>);
    <b>assert</b>!(self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_active_committee">active_committee</a>().members().is_empty(), <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_ECannotInitialize">ECannotInitialize</a>);
    <b>let</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set">pending_active_set</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set">pending_active_set</a>();
    <b>assert</b>!(<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set">pending_active_set</a>.size() &gt;= <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set">pending_active_set</a>.min_validator_count(), <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_ECannotInitialize">ECannotInitialize</a>);
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.set_max_validator_change_count(max_validator_change_count);
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_initiate_mid_epoch_reconfiguration">initiate_mid_epoch_reconfiguration</a>();
    // Create <b>for</b> the first time the advance <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a> approver <b>for</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a> 0.
    // This is done to avoid the case where the <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a> 0 is advanced before the
    // committee is initialized.
    <a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver_create">advance_epoch_approver::create</a>(
        self.witness_approving_advance_epoch,
        balance::zero(),
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_request_add_validator_candidate"></a>

## Function `request_add_validator_candidate`

Can be called by anyone who wishes to become a validator candidate and starts accusing delegated
stakes in their staking pool. Once they have at least <code>MIN_VALIDATOR_JOINING_STAKE</code> amount of stake they
can call <code><a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_add_validator">request_add_validator</a></code> to officially become an active validator at the next epoch.
Aborts if the caller is already a pending or active validator, or a validator candidate.
Note: <code>proof_of_possession_bytes</code> MUST be a valid signature using proof_of_possession_sender and protocol_pubkey_bytes.
To produce a valid PoP, run [fn test_proof_of_possession_bytes].


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_add_validator_candidate">request_add_validator_candidate</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, name: <a href="../std/string.md#std_string_String">std::string::String</a>, protocol_pubkey_bytes: vector&lt;u8&gt;, network_pubkey_bytes: vector&lt;u8&gt;, consensus_pubkey_bytes: vector&lt;u8&gt;, class_groups_pubkey_and_proof_bytes: (ika_common=0x0)::class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof, proof_of_possession_bytes: vector&lt;u8&gt;, network_address: <a href="../std/string.md#std_string_String">std::string::String</a>, p2p_address: <a href="../std/string.md#std_string_String">std::string::String</a>, consensus_address: <a href="../std/string.md#std_string_String">std::string::String</a>, commission_rate: u16, metadata: (ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">validator_metadata::ValidatorMetadata</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): ((ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCommissionCap">validator_cap::ValidatorCommissionCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_add_validator_candidate">request_add_validator_candidate</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
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
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_add_validator_candidate">request_add_validator_candidate</a>(
        self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>,
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
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_request_remove_validator_candidate"></a>

## Function `request_remove_validator_candidate`

Called by a validator candidate to remove themselves from the candidacy. After this call
their staking pool becomes deactivate.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_remove_validator_candidate">request_remove_validator_candidate</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_remove_validator_candidate">request_remove_validator_candidate</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    cap: &ValidatorCap,
) {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_remove_validator_candidate">request_remove_validator_candidate</a>(self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>, cap);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_request_add_validator"></a>

## Function `request_add_validator`

Called by a validator candidate to add themselves to the active validator set beginning next epoch.
Aborts if the validator is a duplicate with one of the pending or active validators, or if the amount of
stake the validator has doesn't meet the min threshold, or if the number of new validators for the next
epoch has already reached the maximum.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_add_validator">request_add_validator</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_add_validator">request_add_validator</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    cap: &ValidatorCap,
) {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_add_validator">request_add_validator</a>(self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>, cap);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_request_remove_validator"></a>

## Function `request_remove_validator`

A validator can call this function to request a removal in the next epoch.
We use the sender of <code>ctx</code> to look up the validator
(i.e. sender must match the sui_address in the validator).
At the end of the epoch, the <code><a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a></code> object will be returned to the sui_address
of the validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_remove_validator">request_remove_validator</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_remove_validator">request_remove_validator</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    cap: &ValidatorCap,
) {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_remove_validator">request_remove_validator</a>(self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>, cap);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_validator_metadata"></a>

## Function `validator_metadata`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata">validator_metadata</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): (ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">validator_metadata::ValidatorMetadata</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata">validator_metadata</a>(
    self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    validator_id: ID,
): ValidatorMetadata {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata">validator_metadata</a>(validator_id)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_set_validator_metadata"></a>

## Function `set_validator_metadata`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_validator_metadata">set_validator_metadata</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, metadata: (ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">validator_metadata::ValidatorMetadata</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_validator_metadata">set_validator_metadata</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    cap: &ValidatorOperationCap,
    metadata: ValidatorMetadata,
) {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_validator_metadata">set_validator_metadata</a>(cap, metadata);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_set_next_commission"></a>

## Function `set_next_commission`

A validator can call this function to set a new commission rate, updated at the end of
the epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_commission">set_next_commission</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, new_commission_rate: u16, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_commission">set_next_commission</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    new_commission_rate: u16,
    cap: &ValidatorOperationCap,
) {
    self
        .<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>
        .<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_commission">set_next_commission</a>(
            new_commission_rate,
            cap,
            self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>,
        );
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_request_add_stake"></a>

## Function `request_add_stake`

Add stake to a validator's staking pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_add_stake">request_add_stake</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, stake: <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_add_stake">request_add_stake</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    stake: Coin&lt;IKA&gt;,
    validator_id: ID,
    ctx: &<b>mut</b> TxContext,
): StakedIka {
    self
        .<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>
        .<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_add_stake">request_add_stake</a>(
            self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>,
            validator_id,
            stake.into_balance(),
            ctx,
        )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_request_withdraw_stake"></a>

## Function `request_withdraw_stake`

Withdraw some portion of a stake from a validator's staking pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_withdraw_stake">request_withdraw_stake</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_withdraw_stake">request_withdraw_stake</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: &<b>mut</b> StakedIka,
) {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_withdraw_stake">request_withdraw_stake</a>(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>, self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_withdraw_stake"></a>

## Function `withdraw_stake`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_withdraw_stake">withdraw_stake</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_withdraw_stake">withdraw_stake</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: StakedIka,
    ctx: &<b>mut</b> TxContext,
): Coin&lt;IKA&gt; {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_withdraw_stake">withdraw_stake</a>(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>, self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>, ctx)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_report_validator"></a>

## Function `report_validator`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_report_validator">report_validator</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, reportee_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_report_validator">report_validator</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    cap: &ValidatorOperationCap,
    reportee_id: ID,
) {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_report_validator">report_validator</a>(cap, reportee_id);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_undo_report_validator"></a>

## Function `undo_report_validator`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_undo_report_validator">undo_report_validator</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, reportee_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_undo_report_validator">undo_report_validator</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    cap: &ValidatorOperationCap,
    reportee_id: ID,
) {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_undo_report_validator">undo_report_validator</a>(cap, reportee_id);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_rotate_operation_cap"></a>

## Function `rotate_operation_cap`

Create a new <code>ValidatorOperationCap</code> and registers it.
The original object is thus revoked.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_rotate_operation_cap">rotate_operation_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_rotate_operation_cap">rotate_operation_cap</a>(self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>, cap: &ValidatorCap, ctx: &<b>mut</b> TxContext): ValidatorOperationCap {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_rotate_operation_cap">rotate_operation_cap</a>(cap, ctx)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_rotate_commission_cap"></a>

## Function `rotate_commission_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_rotate_commission_cap">rotate_commission_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCommissionCap">validator_cap::ValidatorCommissionCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_rotate_commission_cap">rotate_commission_cap</a>(self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>, cap: &ValidatorCap, ctx: &<b>mut</b> TxContext): ValidatorCommissionCap {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_rotate_commission_cap">rotate_commission_cap</a>(cap, ctx)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_collect_commission"></a>

## Function `collect_commission`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_collect_commission">collect_commission</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCommissionCap">validator_cap::ValidatorCommissionCap</a>, amount: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_collect_commission">collect_commission</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    cap: &ValidatorCommissionCap,
    amount: Option&lt;u64&gt;,
    ctx: &<b>mut</b> TxContext,
): Coin&lt;IKA&gt; {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_collect_commission">collect_commission</a>(cap, amount).into_coin(ctx)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_set_validator_name"></a>

## Function `set_validator_name`

Sets a validator's name.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_validator_name">set_validator_name</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, name: <a href="../std/string.md#std_string_String">std::string::String</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_validator_name">set_validator_name</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    name: String,
    cap: &ValidatorOperationCap
) {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_validator_name">set_validator_name</a>(name, cap);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_set_next_epoch_network_address"></a>

## Function `set_next_epoch_network_address`

Sets a validator's network address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_network_address">set_next_epoch_network_address</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, network_address: <a href="../std/string.md#std_string_String">std::string::String</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_network_address">set_next_epoch_network_address</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    network_address: String,
    cap: &ValidatorOperationCap
) {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_network_address">set_next_epoch_network_address</a>(network_address, cap);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_set_next_epoch_p2p_address"></a>

## Function `set_next_epoch_p2p_address`

Sets a validator's p2p address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_p2p_address">set_next_epoch_p2p_address</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, p2p_address: <a href="../std/string.md#std_string_String">std::string::String</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_p2p_address">set_next_epoch_p2p_address</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    p2p_address: String,
    cap: &ValidatorOperationCap
) {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_p2p_address">set_next_epoch_p2p_address</a>(p2p_address, cap);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_set_next_epoch_consensus_address"></a>

## Function `set_next_epoch_consensus_address`

Sets a validator's consensus address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_consensus_address">set_next_epoch_consensus_address</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, consensus_address: <a href="../std/string.md#std_string_String">std::string::String</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_consensus_address">set_next_epoch_consensus_address</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    consensus_address: String,
    cap: &ValidatorOperationCap
) {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_consensus_address">set_next_epoch_consensus_address</a>(consensus_address, cap);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_set_next_epoch_protocol_pubkey_bytes"></a>

## Function `set_next_epoch_protocol_pubkey_bytes`

Sets a validator's public key of protocol key and proof of possession.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_protocol_pubkey_bytes">set_next_epoch_protocol_pubkey_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, protocol_pubkey_bytes: vector&lt;u8&gt;, proof_of_possession_bytes: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, ctx: &<a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_protocol_pubkey_bytes">set_next_epoch_protocol_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    protocol_pubkey_bytes: vector&lt;u8&gt;,
    proof_of_possession_bytes: vector&lt;u8&gt;,
    cap: &ValidatorOperationCap,
    ctx: &TxContext,
) {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_protocol_pubkey_bytes">set_next_epoch_protocol_pubkey_bytes</a>(protocol_pubkey_bytes, proof_of_possession_bytes, cap, ctx);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_set_next_epoch_network_pubkey_bytes"></a>

## Function `set_next_epoch_network_pubkey_bytes`

Sets a validator's public key of network key.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_network_pubkey_bytes">set_next_epoch_network_pubkey_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, network_pubkey_bytes: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_network_pubkey_bytes">set_next_epoch_network_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    network_pubkey_bytes: vector&lt;u8&gt;,
    cap: &ValidatorOperationCap
) {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_network_pubkey_bytes">set_next_epoch_network_pubkey_bytes</a>(network_pubkey_bytes, cap);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_set_next_epoch_consensus_pubkey_bytes"></a>

## Function `set_next_epoch_consensus_pubkey_bytes`

Sets a validator's public key of worker key.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_consensus_pubkey_bytes">set_next_epoch_consensus_pubkey_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, consensus_pubkey_bytes: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_consensus_pubkey_bytes">set_next_epoch_consensus_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    consensus_pubkey_bytes: vector&lt;u8&gt;,
    cap: &ValidatorOperationCap
) {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_consensus_pubkey_bytes">set_next_epoch_consensus_pubkey_bytes</a>(consensus_pubkey_bytes, cap);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_set_next_epoch_class_groups_pubkey_and_proof_bytes"></a>

## Function `set_next_epoch_class_groups_pubkey_and_proof_bytes`

Sets a validator's public key and its associated proof of class groups key.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_class_groups_pubkey_and_proof_bytes">set_next_epoch_class_groups_pubkey_and_proof_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, class_groups_pubkey_and_proof_bytes: (ika_common=0x0)::class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_class_groups_pubkey_and_proof_bytes">set_next_epoch_class_groups_pubkey_and_proof_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    class_groups_pubkey_and_proof_bytes: ClassGroupsPublicKeyAndProof,
    cap: &ValidatorOperationCap
) {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_class_groups_pubkey_and_proof_bytes">set_next_epoch_class_groups_pubkey_and_proof_bytes</a>(class_groups_pubkey_and_proof_bytes, cap);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_is_mid_epoch_time"></a>

## Function `is_mid_epoch_time`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_is_mid_epoch_time">is_mid_epoch_time</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_is_mid_epoch_time">is_mid_epoch_time</a>(self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>, clock: &Clock): bool {
    <b>let</b> now = clock.timestamp_ms();
    <b>let</b> last_epoch_change = self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_start_timestamp_ms">epoch_start_timestamp_ms</a>;
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a> &gt; 0 && self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_next_epoch_active_committee">next_epoch_active_committee</a>().is_none() && now &gt;= last_epoch_change + (self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_duration_ms">epoch_duration_ms</a> / 2)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_is_end_epoch_time"></a>

## Function `is_end_epoch_time`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_is_end_epoch_time">is_end_epoch_time</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_is_end_epoch_time">is_end_epoch_time</a>(self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>, clock: &Clock): bool {
    <b>let</b> now = clock.timestamp_ms();
    <b>let</b> last_epoch_change = self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_start_timestamp_ms">epoch_start_timestamp_ms</a>;
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a> &gt; 0 && self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_next_epoch_active_committee">next_epoch_active_committee</a>().is_some() && now &gt;= last_epoch_change + self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_duration_ms">epoch_duration_ms</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_assert_mid_epoch_time"></a>

## Function `assert_mid_epoch_time`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_assert_mid_epoch_time">assert_mid_epoch_time</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_assert_mid_epoch_time">assert_mid_epoch_time</a>(self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>, clock: &Clock) {
    <b>assert</b>!(self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_is_mid_epoch_time">is_mid_epoch_time</a>(clock), <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EHaveNotReachedMidEpochTime">EHaveNotReachedMidEpochTime</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_assert_end_epoch_time"></a>

## Function `assert_end_epoch_time`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_assert_end_epoch_time">assert_end_epoch_time</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_assert_end_epoch_time">assert_end_epoch_time</a>(self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>, clock: &Clock) {
    <b>assert</b>!(self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_is_end_epoch_time">is_end_epoch_time</a>(clock), <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EHaveNotReachedEndEpochTime">EHaveNotReachedEndEpochTime</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_create_system_current_status_info"></a>

## Function `create_system_current_status_info`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_create_system_current_status_info">create_system_current_status_info</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>): (ika_system=0x0)::<a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_SystemCurrentStatusInfo">system_current_status_info::SystemCurrentStatusInfo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_create_system_current_status_info">create_system_current_status_info</a>(self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>, clock: &Clock): SystemCurrentStatusInfo {
    <a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_create">system_current_status_info::create</a>(
        self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>,
        self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_is_mid_epoch_time">is_mid_epoch_time</a>(clock),
        self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_is_end_epoch_time">is_end_epoch_time</a>(clock),
        self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_active_committee">active_committee</a>(),
        self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_next_epoch_active_committee">next_epoch_active_committee</a>(),
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_initiate_advance_epoch"></a>

## Function `initiate_advance_epoch`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_initiate_advance_epoch">initiate_advance_epoch</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>): (ika_system=0x0)::<a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver_AdvanceEpochApprover">advance_epoch_approver::AdvanceEpochApprover</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_initiate_advance_epoch">initiate_advance_epoch</a>(
    self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    clock: &Clock,
): AdvanceEpochApprover {
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_assert_end_epoch_time">assert_end_epoch_time</a>(clock);
    <a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver_create">advance_epoch_approver::create</a>(
        self.witness_approving_advance_epoch,
        balance::zero(),
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_advance_epoch"></a>

## Function `advance_epoch`

This function should be called at the end of an epoch, and advances the system to the next epoch.
It does the following things:
1. Add storage charge to the storage fund.
2. Burn the storage rebates from the storage fund. These are already refunded to transaction sender's
gas coins.
3. Distribute computation charge to validator stake.
4. Update all validators.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_advance_epoch">advance_epoch</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, <a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver">advance_epoch_approver</a>: (ika_system=0x0)::<a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver_AdvanceEpochApprover">advance_epoch_approver::AdvanceEpochApprover</a>, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_advance_epoch">advance_epoch</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    <a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver">advance_epoch_approver</a>: AdvanceEpochApprover,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver">advance_epoch_approver</a>.assert_all_witnesses_approved();
    <b>assert</b>!(self.received_end_of_publish, <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EHaveNotReachedEndEpochTime">EHaveNotReachedEndEpochTime</a>);
    self.received_end_of_publish = <b>false</b>;
    self.epoch_start_tx_digest = *ctx.digest();
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_start_timestamp_ms">epoch_start_timestamp_ms</a> = clock.timestamp_ms();
    self.previous_epoch_last_checkpoint_sequence_number = self.last_processed_checkpoint_sequence_number;
    <b>let</b> dwallet_computation_and_consensus_validation_rewards = <a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver">advance_epoch_approver</a>.destroy();
    <b>let</b> <b>mut</b> stake_subsidy = balance::zero();
    // during the transition from <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a> N to <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a> N + 1, self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>() will <b>return</b> N
    <b>let</b> current_epoch = self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>();
    // Include stake subsidy in the rewards given out to validators and stakers.
    // Delay distributing any stake subsidies until after `stake_subsidy_start_epoch`.
    <b>if</b> (current_epoch &gt;= self.stake_subsidy_start_epoch) {
        stake_subsidy.join(self.<a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury">protocol_treasury</a>.stake_subsidy_for_distribution(ctx));
    };
    <b>let</b> stake_subsidy_amount = stake_subsidy.value();
    <b>let</b> total_computation_fees = dwallet_computation_and_consensus_validation_rewards.value();
    <b>let</b> <b>mut</b> total_reward = <a href="../sui/balance.md#sui_balance_zero">sui::balance::zero</a>&lt;IKA&gt;();
    total_reward.join(dwallet_computation_and_consensus_validation_rewards);
    total_reward.join(stake_subsidy);
    total_reward.join(self.remaining_rewards.withdraw_all());
    <b>let</b> total_reward_amount_before_distribution = total_reward.value();
    <b>let</b> new_epoch = current_epoch + 1;
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a> = new_epoch;
    <b>if</b> (self.next_protocol_version.is_some()) {
        self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_protocol_version">protocol_version</a> = self.next_protocol_version.extract();
    };
    self
        .<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>
        .<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_advance_epoch">advance_epoch</a>(
            new_epoch,
            &<b>mut</b> total_reward,
        );
    <b>let</b> new_total_stake = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.total_stake();
    <b>let</b> total_reward_amount_after_distribution = total_reward.value();
    <b>let</b> total_reward_distributed =
        total_reward_amount_before_distribution - total_reward_amount_after_distribution;
    // Because of precision issues with integer divisions, we expect that there will be some
    // remaining balance in `remaining_rewards`.
    self.remaining_rewards.join(total_reward);
    event::emit(<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemEpochInfoEvent">SystemEpochInfoEvent</a> {
        <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>,
        <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_protocol_version">protocol_version</a>: self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_protocol_version">protocol_version</a>,
        total_stake: new_total_stake,
        stake_subsidy_amount,
        total_computation_fees,
        total_stake_rewards_distributed: total_reward_distributed,
    });
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_initiate_mid_epoch_reconfiguration"></a>

## Function `initiate_mid_epoch_reconfiguration`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_initiate_mid_epoch_reconfiguration">initiate_mid_epoch_reconfiguration</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_initiate_mid_epoch_reconfiguration">initiate_mid_epoch_reconfiguration</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    clock: &Clock,
) {
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_assert_mid_epoch_time">assert_mid_epoch_time</a>(clock);
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_initiate_mid_epoch_reconfiguration">initiate_mid_epoch_reconfiguration</a>();
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_epoch"></a>

## Function `epoch`

Return the current epoch number. Useful for applications that need a coarse-grained concept of time,
since epochs are ever-increasing and epoch changes are intended to happen every 24 hours.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>(self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>): u64 {
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_protocol_version"></a>

## Function `protocol_version`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_protocol_version">protocol_version</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_protocol_version">protocol_version</a>(self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>): u64 {
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_protocol_version">protocol_version</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_epoch_start_timestamp_ms"></a>

## Function `epoch_start_timestamp_ms`

Returns unix timestamp of the start of current epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_start_timestamp_ms">epoch_start_timestamp_ms</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_start_timestamp_ms">epoch_start_timestamp_ms</a>(self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>): u64 {
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_start_timestamp_ms">epoch_start_timestamp_ms</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_validator_stake_amount"></a>

## Function `validator_stake_amount`

Returns the total amount staked with <code>validator_id</code>.
Aborts if <code>validator_id</code> is not an active validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_validator_stake_amount">validator_stake_amount</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_validator_stake_amount">validator_stake_amount</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    validator_id: ID,
): u64 {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.validator_total_stake_amount(validator_id)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_get_reporters_of"></a>

## Function `get_reporters_of`

Returns all the validators who are currently reporting <code>validator_id</code>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_get_reporters_of">get_reporters_of</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): <a href="../sui/vec_set.md#sui_vec_set_VecSet">sui::vec_set::VecSet</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_get_reporters_of">get_reporters_of</a>(self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>, validator_id: ID): VecSet&lt;ID&gt; {
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_get_reporters_of">get_reporters_of</a>(validator_id)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_token_exchange_rates"></a>

## Function `token_exchange_rates`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_token_exchange_rates">token_exchange_rates</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): &<a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, (ika_system=0x0)::<a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_TokenExchangeRate">token_exchange_rate::TokenExchangeRate</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_token_exchange_rates">token_exchange_rates</a>(
    self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    validator_id: ID,
): &Table&lt;u64, TokenExchangeRate&gt; {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_token_exchange_rates">token_exchange_rates</a>(validator_id)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_active_committee"></a>

## Function `active_committee`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_active_committee">active_committee</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>): (ika_common=0x0)::bls_committee::BlsCommittee
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_active_committee">active_committee</a>(self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>): BlsCommittee {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_active_committee">active_committee</a>()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_next_epoch_active_committee"></a>

## Function `next_epoch_active_committee`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_next_epoch_active_committee">next_epoch_active_committee</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;(ika_common=0x0)::bls_committee::BlsCommittee&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_next_epoch_active_committee">next_epoch_active_committee</a>(self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>): Option&lt;BlsCommittee&gt; {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_next_epoch_active_committee">next_epoch_active_committee</a>()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_verify_validator_cap"></a>

## Function `verify_validator_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_verify_validator_cap">verify_validator_cap</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>): (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_VerifiedValidatorCap">validator_cap::VerifiedValidatorCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_verify_validator_cap">verify_validator_cap</a>(
    self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    cap: &ValidatorCap,
): VerifiedValidatorCap {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_verify_validator_cap">verify_validator_cap</a>(cap);
    cap.create_verified_validator_cap()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_verify_operation_cap"></a>

## Function `verify_operation_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_verify_operation_cap">verify_operation_cap</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>): (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_VerifiedValidatorOperationCap">validator_cap::VerifiedValidatorOperationCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_verify_operation_cap">verify_operation_cap</a>(
    self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    cap: &ValidatorOperationCap,
): VerifiedValidatorOperationCap {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_verify_operation_cap">verify_operation_cap</a>(cap);
    cap.create_verified_validator_operation_cap()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_verify_commission_cap"></a>

## Function `verify_commission_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_verify_commission_cap">verify_commission_cap</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCommissionCap">validator_cap::ValidatorCommissionCap</a>): (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_VerifiedValidatorCommissionCap">validator_cap::VerifiedValidatorCommissionCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_verify_commission_cap">verify_commission_cap</a>(
    self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    cap: &ValidatorCommissionCap,
): VerifiedValidatorCommissionCap {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_verify_commission_cap">verify_commission_cap</a>(cap);
    cap.create_verified_validator_commission_cap()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_verify_protocol_cap_impl"></a>

## Function `verify_protocol_cap_impl`



<pre><code><b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_verify_protocol_cap_impl">verify_protocol_cap_impl</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, cap: &(ika_system=0x0)::<a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_ProtocolCap">protocol_cap::ProtocolCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_verify_protocol_cap_impl">verify_protocol_cap_impl</a>(
    self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    cap: &ProtocolCap,
) {
    <b>let</b> protocol_cap_id = object::id(cap);
    <b>assert</b>!(self.authorized_protocol_cap_ids.contains(&protocol_cap_id), <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EUnauthorizedProtocolCap">EUnauthorizedProtocolCap</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_add_upgrade_cap_by_cap"></a>

## Function `add_upgrade_cap_by_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_add_upgrade_cap_by_cap">add_upgrade_cap_by_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, cap: &(ika_system=0x0)::<a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_ProtocolCap">protocol_cap::ProtocolCap</a>, upgrade_cap: <a href="../sui/package.md#sui_package_UpgradeCap">sui::package::UpgradeCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_add_upgrade_cap_by_cap">add_upgrade_cap_by_cap</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    cap: &ProtocolCap,
    upgrade_cap: UpgradeCap,
) {
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_verify_protocol_cap_impl">verify_protocol_cap_impl</a>(cap);
    self.upgrade_caps.push_back(upgrade_cap);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_authorize_upgrade"></a>

## Function `authorize_upgrade`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_authorize_upgrade">authorize_upgrade</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, package_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): <a href="../sui/package.md#sui_package_UpgradeTicket">sui::package::UpgradeTicket</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_authorize_upgrade">authorize_upgrade</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    package_id: ID,
): UpgradeTicket {
    <b>assert</b>!(self.approved_upgrades.contains(&package_id), <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EApprovedUpgradeNotFound">EApprovedUpgradeNotFound</a>);
    <b>let</b> (_, digest) = self.approved_upgrades.remove(&package_id);
    <b>let</b> index = self.upgrade_caps.find_index!(|c| c.package() == package_id).extract();
    <b>let</b> policy = self.upgrade_caps[index].policy();
    self.upgrade_caps[index].authorize(policy, digest)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_commit_upgrade"></a>

## Function `commit_upgrade`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_commit_upgrade">commit_upgrade</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, receipt: <a href="../sui/package.md#sui_package_UpgradeReceipt">sui::package::UpgradeReceipt</a>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_commit_upgrade">commit_upgrade</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    receipt: UpgradeReceipt,
): ID {
    <b>let</b> receipt_cap_id = receipt.cap();
    <b>let</b> index = self.upgrade_caps.find_index!(|c| object::id(c) == receipt_cap_id).extract();
    <b>let</b> old_package_id = self.upgrade_caps[index].package();
    self.upgrade_caps[index].commit(receipt);
    old_package_id
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_process_checkpoint_message_by_quorum"></a>

## Function `process_checkpoint_message_by_quorum`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, signature: vector&lt;u8&gt;, signers_bitmap: vector&lt;u8&gt;, message: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    signature: vector&lt;u8&gt;,
    signers_bitmap: vector&lt;u8&gt;,
    message: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_active_committee">active_committee</a> = self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_active_committee">active_committee</a>();
    <b>assert</b>!(!<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_active_committee">active_committee</a>.members().is_empty(), <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EActiveBlsCommitteeMustInitialize">EActiveBlsCommitteeMustInitialize</a>);
    <b>let</b> <b>mut</b> intent_bytes = <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_PARAMS_MESSAGE_INTENT">PARAMS_MESSAGE_INTENT</a>;
    intent_bytes.append(message);
    intent_bytes.append(bcs::to_bytes(&self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>));
    <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_active_committee">active_committee</a>.verify_certificate(self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>, &signature, &signers_bitmap, &intent_bytes);
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_process_checkpoint_message">process_checkpoint_message</a>(message, ctx);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_process_checkpoint_message"></a>

## Function `process_checkpoint_message`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_process_checkpoint_message">process_checkpoint_message</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, message: vector&lt;u8&gt;, _ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_process_checkpoint_message">process_checkpoint_message</a>(self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>, message: vector&lt;u8&gt;, _ctx: &<b>mut</b> TxContext) {
    <b>let</b> <b>mut</b> bcs_body = bcs::new(<b>copy</b> message);
    <b>let</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a> = bcs_body.peel_u64();
    <b>assert</b>!(<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a> == self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>, <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EIncorrectEpochInSystemCheckpoint">EIncorrectEpochInSystemCheckpoint</a>);
    <b>let</b> sequence_number = bcs_body.peel_u64();
    <b>assert</b>!(self.last_processed_checkpoint_sequence_number + 1 == sequence_number, <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EWrongSystemCheckpointSequenceNumber">EWrongSystemCheckpointSequenceNumber</a>);
    self.last_processed_checkpoint_sequence_number = sequence_number;
    <b>let</b> timestamp_ms = bcs_body.peel_u64();
    event::emit(<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemCheckpointInfoEvent">SystemCheckpointInfoEvent</a> {
        <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>,
        sequence_number,
        timestamp_ms,
    });
    <b>let</b> len = bcs_body.peel_vec_length();
    <b>let</b> <b>mut</b> i = 0;
    // Note: the order of these fields, and the number must correspond to the Rust code in
    // `crates/ika-types/src/messages_system_checkpoints.rs`.
    <b>while</b> (i &lt; len) {
        <b>let</b> message_data_enum_tag = bcs_body.peel_enum_tag();
        // Parses params message BCS bytes directly.
        match (message_data_enum_tag) {
            <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SET_NEXT_PROTOCOL_VERSION_MESSAGE_TYPE">SET_NEXT_PROTOCOL_VERSION_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> next_protocol_version = bcs_body.peel_u64();
                self.next_protocol_version = option::some(next_protocol_version);
                event::emit(<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SetNextProtocolVersionEvent">SetNextProtocolVersionEvent</a> {
                    <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>,
                    next_protocol_version,
                });
            },
            <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SET_EPOCH_DURATION_MS_MESSAGE_TYPE">SET_EPOCH_DURATION_MS_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_duration_ms">epoch_duration_ms</a> = bcs_body.peel_u64();
                self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_duration_ms">epoch_duration_ms</a> = <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_duration_ms">epoch_duration_ms</a>;
                event::emit(<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SetEpochDurationMsEvent">SetEpochDurationMsEvent</a> {
                    <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>,
                    <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_duration_ms">epoch_duration_ms</a>,
                });
            },
            <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SET_STAKE_SUBSIDY_START_EPOCH_MESSAGE_TYPE">SET_STAKE_SUBSIDY_START_EPOCH_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> stake_subsidy_start_epoch = bcs_body.peel_u64();
                self.stake_subsidy_start_epoch = stake_subsidy_start_epoch;
                event::emit(<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SetStakeSubsidyStartEpochEvent">SetStakeSubsidyStartEpochEvent</a> {
                    <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>,
                    stake_subsidy_start_epoch,
                });
            },
            <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SET_STAKE_SUBSIDY_RATE_MESSAGE_TYPE">SET_STAKE_SUBSIDY_RATE_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> stake_subsidy_rate = bcs_body.peel_u16();
                self.<a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury">protocol_treasury</a>.set_stake_subsidy_rate(stake_subsidy_rate);
                event::emit(<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SetStakeSubsidyRateEvent">SetStakeSubsidyRateEvent</a> {
                    <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>,
                    stake_subsidy_rate,
                });
            },
            <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SET_STAKE_SUBSIDY_PERIOD_LENGTH_MESSAGE_TYPE">SET_STAKE_SUBSIDY_PERIOD_LENGTH_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> stake_subsidy_period_length = bcs_body.peel_u64();
                self.<a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury">protocol_treasury</a>.set_stake_subsidy_period_length(stake_subsidy_period_length);
                event::emit(<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SetStakeSubsidyPeriodLengthEvent">SetStakeSubsidyPeriodLengthEvent</a> {
                    <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>,
                    stake_subsidy_period_length,
                });
            },
            <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SET_MIN_VALIDATOR_COUNT_MESSAGE_TYPE">SET_MIN_VALIDATOR_COUNT_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> min_validator_count = bcs_body.peel_u64();
                self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.set_min_validator_count(min_validator_count);
                event::emit(<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SetMinValidatorCountEvent">SetMinValidatorCountEvent</a> {
                    <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>,
                    min_validator_count,
                });
            },
            <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SET_MAX_VALIDATOR_COUNT_MESSAGE_TYPE">SET_MAX_VALIDATOR_COUNT_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> max_validator_count = bcs_body.peel_u64();
                self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.set_max_validator_count(max_validator_count);
                event::emit(<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SetMaxValidatorCountEvent">SetMaxValidatorCountEvent</a> {
                    <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>,
                    max_validator_count,
                });
            },
            <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SET_MIN_VALIDATOR_JOINING_STAKE_MESSAGE_TYPE">SET_MIN_VALIDATOR_JOINING_STAKE_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> min_validator_joining_stake = bcs_body.peel_u64();
                self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.set_min_validator_joining_stake(min_validator_joining_stake);
                event::emit(<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SetMinValidatorJoiningStakeEvent">SetMinValidatorJoiningStakeEvent</a> {
                    <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>,
                    min_validator_joining_stake,
                });
            },
            <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SET_MAX_VALIDATOR_CHANGE_COUNT_MESSAGE_TYPE">SET_MAX_VALIDATOR_CHANGE_COUNT_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> max_validator_change_count = bcs_body.peel_u64();
                self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.set_max_validator_change_count(max_validator_change_count);
                event::emit(<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SetMaxValidatorChangeCountEvent">SetMaxValidatorChangeCountEvent</a> {
                    <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>,
                    max_validator_change_count,
                });
            },
            <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SET_REWARD_SLASHING_RATE_MESSAGE_TYPE">SET_REWARD_SLASHING_RATE_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> reward_slashing_rate = bcs_body.peel_u16();
                self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.set_reward_slashing_rate(reward_slashing_rate);
                event::emit(<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SetRewardSlashingRateEvent">SetRewardSlashingRateEvent</a> {
                    <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>,
                    reward_slashing_rate,
                });
            },
            <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SET_APPROVED_UPGRADE_MESSAGE_TYPE">SET_APPROVED_UPGRADE_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> package_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> digest = bcs_body.peel_option!(|bcs| bcs.peel_vec_u8());
                self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_approved_upgrade">set_approved_upgrade</a>(package_id, digest);
            },
            <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_END_OF_PUBLISH_MESSAGE_TYPE">END_OF_PUBLISH_MESSAGE_TYPE</a> =&gt; {
                self.received_end_of_publish = <b>true</b>;
                event::emit(<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EndOfPublishEvent">EndOfPublishEvent</a> {
                    <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>,
                });
            },
            <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SetOrRemoveWitnessApprovingAdvanceEpochMessageType">SetOrRemoveWitnessApprovingAdvanceEpochMessageType</a> =&gt; {
                <b>let</b> witness_type = bcs_body.peel_vec_u8().to_string();
                <b>let</b> remove = bcs_body.peel_bool();
                self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_or_remove_witness_approving_advance_epoch">set_or_remove_witness_approving_advance_epoch</a>(witness_type, remove);
            },
            _ =&gt; {
                // Unknown message type - skip
            }
        };
        i = i + 1;
    };
    self.total_messages_processed = self.total_messages_processed + i;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_verify_protocol_cap"></a>

## Function `verify_protocol_cap`

=== Protocol Cap Functions ===


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_verify_protocol_cap">verify_protocol_cap</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, cap: &(ika_system=0x0)::<a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_ProtocolCap">protocol_cap::ProtocolCap</a>): (ika_system=0x0)::<a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_VerifiedProtocolCap">protocol_cap::VerifiedProtocolCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_verify_protocol_cap">verify_protocol_cap</a>(
    self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    cap: &ProtocolCap,
): VerifiedProtocolCap {
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_verify_protocol_cap_impl">verify_protocol_cap_impl</a>(cap);
    <a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_create_verified">protocol_cap::create_verified</a>()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_set_approved_upgrade_by_cap"></a>

## Function `set_approved_upgrade_by_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_approved_upgrade_by_cap">set_approved_upgrade_by_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, cap: &(ika_system=0x0)::<a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_ProtocolCap">protocol_cap::ProtocolCap</a>, package_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, digest: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_approved_upgrade_by_cap">set_approved_upgrade_by_cap</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    cap: &ProtocolCap,
    package_id: ID,
    digest: Option&lt;vector&lt;u8&gt;&gt;,
) {
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_verify_protocol_cap_impl">verify_protocol_cap_impl</a>(cap);
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_approved_upgrade">set_approved_upgrade</a>(package_id, digest);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_set_or_remove_witness_approving_advance_epoch_by_cap"></a>

## Function `set_or_remove_witness_approving_advance_epoch_by_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_or_remove_witness_approving_advance_epoch_by_cap">set_or_remove_witness_approving_advance_epoch_by_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, cap: &(ika_system=0x0)::<a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_ProtocolCap">protocol_cap::ProtocolCap</a>, witness_type: <a href="../std/string.md#std_string_String">std::string::String</a>, remove: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_or_remove_witness_approving_advance_epoch_by_cap">set_or_remove_witness_approving_advance_epoch_by_cap</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    cap: &ProtocolCap,
    witness_type: String,
    remove: bool,
) {
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_verify_protocol_cap_impl">verify_protocol_cap_impl</a>(cap);
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_or_remove_witness_approving_advance_epoch">set_or_remove_witness_approving_advance_epoch</a>(witness_type, remove);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_process_checkpoint_message_by_cap"></a>

## Function `process_checkpoint_message_by_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_process_checkpoint_message_by_cap">process_checkpoint_message_by_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, cap: &(ika_system=0x0)::<a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_ProtocolCap">protocol_cap::ProtocolCap</a>, message: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_process_checkpoint_message_by_cap">process_checkpoint_message_by_cap</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    cap: &ProtocolCap,
    message: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_verify_protocol_cap_impl">verify_protocol_cap_impl</a>(cap);
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_process_checkpoint_message">process_checkpoint_message</a>(message, ctx);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_set_approved_upgrade"></a>

## Function `set_approved_upgrade`

=== Private Functions ===
Set approved upgrade for a package id.
If <code>digest</code> is <code>some</code>, it will be inserted into the <code>approved_upgrades</code> map.
If <code>digest</code> is <code>none</code>, it will be removed from the <code>approved_upgrades</code> map.


<pre><code><b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_approved_upgrade">set_approved_upgrade</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, package_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, digest: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_approved_upgrade">set_approved_upgrade</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    package_id: ID,
    <b>mut</b> digest: Option&lt;vector&lt;u8&gt;&gt;,
) {
    <b>if</b>(digest.is_some()) {
        <b>if</b>(self.approved_upgrades.contains(&package_id)) {
            *self.approved_upgrades.get_mut(&package_id) = digest.extract();
        } <b>else</b> {
            self.approved_upgrades.insert(package_id, digest.extract());
        }
    } <b>else</b> {
        <b>if</b>(self.approved_upgrades.contains(&package_id)) {
            self.approved_upgrades.remove(&package_id);
        }
    };
    event::emit(<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SetApprovedUpgradeEvent">SetApprovedUpgradeEvent</a> {
        <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>,
        package_id,
        digest,
    });
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_set_or_remove_witness_approving_advance_epoch"></a>

## Function `set_or_remove_witness_approving_advance_epoch`

Set or remove a witness approving advance epoch.
If <code>remove</code> is <code><b>true</b></code>, the witness will be removed from the list of witnesses approving advance epoch.
If <code>remove</code> is <code><b>false</b></code>, the witness will be added to the list of witnesses approving advance epoch.


<pre><code><b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_or_remove_witness_approving_advance_epoch">set_or_remove_witness_approving_advance_epoch</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, witness_type: <a href="../std/string.md#std_string_String">std::string::String</a>, remove: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_or_remove_witness_approving_advance_epoch">set_or_remove_witness_approving_advance_epoch</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    witness_type: String,
    remove: bool,
) {
    <b>let</b> (found, index) = self.witness_approving_advance_epoch.index_of(&witness_type);
    <b>if</b> (remove && found) {
        self.witness_approving_advance_epoch.remove(index);
    } <b>else</b> <b>if</b> (!found) {
        self.witness_approving_advance_epoch.push_back(witness_type);
    };
    event::emit(<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SetOrRemoveWitnessApprovingAdvanceEpochEvent">SetOrRemoveWitnessApprovingAdvanceEpochEvent</a> {
        <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>,
        witness_type,
        remove
    });
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_calculate_rewards"></a>

## Function `calculate_rewards`

Calculate the rewards for an amount with value <code>staked_principal</code>, staked in the validator with
the given <code>validator_id</code> between <code>activation_epoch</code> and <code>withdraw_epoch</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_calculate_rewards">calculate_rewards</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, node_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, staked_principal: u64, activation_epoch: u64, withdraw_epoch: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_calculate_rewards">calculate_rewards</a>(
    self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>,
    node_id: ID,
    staked_principal: u64,
    activation_epoch: u64,
    withdraw_epoch: u64,
): u64 {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_calculate_rewards">calculate_rewards</a>(node_id, staked_principal, activation_epoch, withdraw_epoch)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_can_withdraw_staked_ika_early"></a>

## Function `can_withdraw_staked_ika_early`

Check whether StakedIka can be withdrawn directly.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_can_withdraw_staked_ika_early">can_withdraw_staked_ika_early</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: &(ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_can_withdraw_staked_ika_early">can_withdraw_staked_ika_early</a>(self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: &StakedIka): bool {
    self.<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_can_withdraw_staked_ika_early">can_withdraw_staked_ika_early</a>(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>, self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_epoch_duration_ms"></a>

## Function `epoch_duration_ms`

Returns the duration of an epoch in milliseconds.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_duration_ms">epoch_duration_ms</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_duration_ms">epoch_duration_ms</a>(self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">SystemInner</a>): u64 {
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_duration_ms">epoch_duration_ms</a>
}
</code></pre>



</details>
