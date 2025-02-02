---
title: Module `0x0::system_inner_v1`
---



-  [Struct `SystemParametersV1`](#0x0_system_inner_v1_SystemParametersV1)
-  [Struct `SystemInnerV1`](#0x0_system_inner_v1_SystemInnerV1)
-  [Struct `SystemEpochInfoEvent`](#0x0_system_inner_v1_SystemEpochInfoEvent)
-  [Struct `SystemQuorumVerifiedEvent`](#0x0_system_inner_v1_SystemQuorumVerifiedEvent)
-  [Struct `SystemProtocolCapVerifiedEvent`](#0x0_system_inner_v1_SystemProtocolCapVerifiedEvent)
-  [Struct `SystemCheckpointInfoEvent`](#0x0_system_inner_v1_SystemCheckpointInfoEvent)
-  [Struct `TestMessageEvent`](#0x0_system_inner_v1_TestMessageEvent)
-  [Constants](#@Constants_0)
-  [Function `create`](#0x0_system_inner_v1_create)
-  [Function `create_system_parameters`](#0x0_system_inner_v1_create_system_parameters)
-  [Function `initialize`](#0x0_system_inner_v1_initialize)
-  [Function `request_add_validator_candidate`](#0x0_system_inner_v1_request_add_validator_candidate)
-  [Function `request_remove_validator_candidate`](#0x0_system_inner_v1_request_remove_validator_candidate)
-  [Function `request_add_validator`](#0x0_system_inner_v1_request_add_validator)
-  [Function `request_remove_validator`](#0x0_system_inner_v1_request_remove_validator)
-  [Function `request_set_computation_price`](#0x0_system_inner_v1_request_set_computation_price)
-  [Function `set_candidate_validator_computation_price`](#0x0_system_inner_v1_set_candidate_validator_computation_price)
-  [Function `request_set_commission_rate`](#0x0_system_inner_v1_request_set_commission_rate)
-  [Function `set_candidate_validator_commission_rate`](#0x0_system_inner_v1_set_candidate_validator_commission_rate)
-  [Function `request_add_stake`](#0x0_system_inner_v1_request_add_stake)
-  [Function `request_add_stake_mul_coin`](#0x0_system_inner_v1_request_add_stake_mul_coin)
-  [Function `request_withdraw_stake`](#0x0_system_inner_v1_request_withdraw_stake)
-  [Function `convert_to_fungible_staked_ika`](#0x0_system_inner_v1_convert_to_fungible_staked_ika)
-  [Function `redeem_fungible_staked_ika`](#0x0_system_inner_v1_redeem_fungible_staked_ika)
-  [Function `report_validator`](#0x0_system_inner_v1_report_validator)
-  [Function `undo_report_validator`](#0x0_system_inner_v1_undo_report_validator)
-  [Function `rotate_operation_cap`](#0x0_system_inner_v1_rotate_operation_cap)
-  [Function `update_validator_payment_address`](#0x0_system_inner_v1_update_validator_payment_address)
-  [Function `update_validator_name`](#0x0_system_inner_v1_update_validator_name)
-  [Function `update_validator_description`](#0x0_system_inner_v1_update_validator_description)
-  [Function `update_validator_image_url`](#0x0_system_inner_v1_update_validator_image_url)
-  [Function `update_validator_project_url`](#0x0_system_inner_v1_update_validator_project_url)
-  [Function `update_validator_next_epoch_network_address`](#0x0_system_inner_v1_update_validator_next_epoch_network_address)
-  [Function `update_candidate_validator_network_address`](#0x0_system_inner_v1_update_candidate_validator_network_address)
-  [Function `update_validator_next_epoch_p2p_address`](#0x0_system_inner_v1_update_validator_next_epoch_p2p_address)
-  [Function `update_candidate_validator_p2p_address`](#0x0_system_inner_v1_update_candidate_validator_p2p_address)
-  [Function `update_validator_next_epoch_consensus_address`](#0x0_system_inner_v1_update_validator_next_epoch_consensus_address)
-  [Function `update_candidate_validator_consensus_address`](#0x0_system_inner_v1_update_candidate_validator_consensus_address)
-  [Function `update_validator_next_epoch_protocol_pubkey_bytes`](#0x0_system_inner_v1_update_validator_next_epoch_protocol_pubkey_bytes)
-  [Function `update_candidate_validator_protocol_pubkey_bytes`](#0x0_system_inner_v1_update_candidate_validator_protocol_pubkey_bytes)
-  [Function `update_validator_next_epoch_consensus_pubkey_bytes`](#0x0_system_inner_v1_update_validator_next_epoch_consensus_pubkey_bytes)
-  [Function `update_candidate_validator_consensus_pubkey_bytes`](#0x0_system_inner_v1_update_candidate_validator_consensus_pubkey_bytes)
-  [Function `update_validator_next_epoch_network_pubkey_bytes`](#0x0_system_inner_v1_update_validator_next_epoch_network_pubkey_bytes)
-  [Function `update_candidate_validator_network_pubkey_bytes`](#0x0_system_inner_v1_update_candidate_validator_network_pubkey_bytes)
-  [Function `advance_epoch`](#0x0_system_inner_v1_advance_epoch)
-  [Function `process_mid_epoch`](#0x0_system_inner_v1_process_mid_epoch)
-  [Function `epoch`](#0x0_system_inner_v1_epoch)
-  [Function `protocol_version`](#0x0_system_inner_v1_protocol_version)
-  [Function `upgrade_caps`](#0x0_system_inner_v1_upgrade_caps)
-  [Function `epoch_start_timestamp_ms`](#0x0_system_inner_v1_epoch_start_timestamp_ms)
-  [Function `validator_stake_amount`](#0x0_system_inner_v1_validator_stake_amount)
-  [Function `get_reporters_of`](#0x0_system_inner_v1_get_reporters_of)
-  [Function `pool_exchange_rates`](#0x0_system_inner_v1_pool_exchange_rates)
-  [Function `active_committee`](#0x0_system_inner_v1_active_committee)
-  [Function `process_checkpoint_message_by_cap`](#0x0_system_inner_v1_process_checkpoint_message_by_cap)
-  [Function `process_checkpoint_message_by_quorum`](#0x0_system_inner_v1_process_checkpoint_message_by_quorum)
-  [Function `process_checkpoint_message`](#0x0_system_inner_v1_process_checkpoint_message)
-  [Function `extract_coin_balance`](#0x0_system_inner_v1_extract_coin_balance)
-  [Function `authorize_update_message_by_cap`](#0x0_system_inner_v1_authorize_update_message_by_cap)
-  [Function `authorize_update_message`](#0x0_system_inner_v1_authorize_update_message)
-  [Function `commit_upgrade`](#0x0_system_inner_v1_commit_upgrade)


<pre><code><b>use</b> <a href="committee.md#0x0_committee">0x0::committee</a>;
<b>use</b> <a href="../ika/ika.md#0x0_ika">0x0::ika</a>;
<b>use</b> <a href="protocol_cap.md#0x0_protocol_cap">0x0::protocol_cap</a>;
<b>use</b> <a href="protocol_treasury.md#0x0_protocol_treasury">0x0::protocol_treasury</a>;
<b>use</b> <a href="staked_ika.md#0x0_staked_ika">0x0::staked_ika</a>;
<b>use</b> <a href="staking_pool.md#0x0_staking_pool">0x0::staking_pool</a>;
<b>use</b> <a href="validator_cap.md#0x0_validator_cap">0x0::validator_cap</a>;
<b>use</b> <a href="validator_inner.md#0x0_validator_inner_v1">0x0::validator_inner_v1</a>;
<b>use</b> <a href="validator_set.md#0x0_validator_set">0x0::validator_set</a>;
<b>use</b> <a href="../move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../move-stdlib/vector.md#0x1_vector">0x1::vector</a>;
<b>use</b> <a href="../sui-framework/bag.md#0x2_bag">0x2::bag</a>;
<b>use</b> <a href="../sui-framework/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../sui-framework/bcs.md#0x2_bcs">0x2::bcs</a>;
<b>use</b> <a href="../sui-framework/clock.md#0x2_clock">0x2::clock</a>;
<b>use</b> <a href="../sui-framework/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../sui-framework/event.md#0x2_event">0x2::event</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/package.md#0x2_package">0x2::package</a>;
<b>use</b> <a href="../sui-framework/pay.md#0x2_pay">0x2::pay</a>;
<b>use</b> <a href="../sui-framework/table.md#0x2_table">0x2::table</a>;
<b>use</b> <a href="../sui-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../sui-framework/vec_set.md#0x2_vec_set">0x2::vec_set</a>;
</code></pre>



<a name="0x0_system_inner_v1_SystemParametersV1"></a>

## Struct `SystemParametersV1`

The params of the system.


<pre><code><b>struct</b> <a href="system_inner.md#0x0_system_inner_v1_SystemParametersV1">SystemParametersV1</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>epoch_duration_ms: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 The duration of an epoch, in milliseconds.
</dd>
<dt>
<code>stake_subsidy_start_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 The starting epoch in which stake subsidies start being paid out
</dd>
<dt>
<code>min_validator_count: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 Minimum number of active validators at any moment.
</dd>
<dt>
<code>max_validator_count: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 Maximum number of active validators at any moment.
 We do not allow the number of validators in any epoch to go above this.
</dd>
<dt>
<code>min_validator_joining_stake: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 Lower-bound on the amount of stake required to become a validator.
</dd>
<dt>
<code>validator_low_stake_threshold: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 Validators with stake amount below <code>validator_low_stake_threshold</code> are considered to
 have low stake and will be escorted out of the validator set after being below this
 threshold for more than <code>validator_low_stake_grace_period</code> number of epochs.
</dd>
<dt>
<code>validator_very_low_stake_threshold: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 Validators with stake below <code>validator_very_low_stake_threshold</code> will be removed
 immediately at epoch change, no grace period.
</dd>
<dt>
<code>validator_low_stake_grace_period: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 A validator can have stake below <code>validator_low_stake_threshold</code>
 for this many epochs before being kicked out.
</dd>
<dt>
<code>reward_slashing_rate: u16</code>
</dt>
<dd>
 How many reward are slashed to punish a validator, in bps.
</dd>
<dt>
<code>lock_active_committee: bool</code>
</dt>
<dd>
 Lock active committee between epochs.
</dd>
<dt>
<code>extra_fields: <a href="../sui-framework/bag.md#0x2_bag_Bag">bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="0x0_system_inner_v1_SystemInnerV1"></a>

## Struct `SystemInnerV1`

Uses SystemParametersV1 as the parameters.


<pre><code><b>struct</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 The current epoch ID, starting from 0.
</dd>
<dt>
<code>protocol_version: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 The current protocol version, starting from 1.
</dd>
<dt>
<code>upgrade_caps: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../sui-framework/package.md#0x2_package_UpgradeCap">package::UpgradeCap</a>&gt;</code>
</dt>
<dd>
 Upgrade caps for this package and others like ika coin of the ika protocol.
</dd>
<dt>
<code>validators: <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a></code>
</dt>
<dd>
 Contains all information about the validators.
</dd>
<dt>
<code>parameters: <a href="system_inner.md#0x0_system_inner_v1_SystemParametersV1">system_inner_v1::SystemParametersV1</a></code>
</dt>
<dd>
 A list of system config parameters.
</dd>
<dt>
<code>computation_price_per_unit_size: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 The computation price per unit size for the current epoch.
</dd>
<dt>
<code><a href="protocol_treasury.md#0x0_protocol_treasury">protocol_treasury</a>: <a href="protocol_treasury.md#0x0_protocol_treasury_ProtocolTreasury">protocol_treasury::ProtocolTreasury</a></code>
</dt>
<dd>
 Schedule of stake subsidies given out each epoch.
</dd>
<dt>
<code>epoch_start_timestamp_ms: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 Unix timestamp of the current epoch start.
</dd>
<dt>
<code>total_messages_processed: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 The total messages processed.
</dd>
<dt>
<code>last_processed_checkpoint_sequence_number: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;</code>
</dt>
<dd>
 The last checkpoint sequence number processed.
</dd>
<dt>
<code>previous_epoch_last_checkpoint_sequence_number: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 The last checkpoint sequence number of previous epoch.
</dd>
<dt>
<code>computation_reward: <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;</code>
</dt>
<dd>
 The fees paid for computation.
</dd>
<dt>
<code>authorized_protocol_cap_ids: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>&gt;</code>
</dt>
<dd>
 List of authorized protocol cap ids.
</dd>
<dt>
<code>extra_fields: <a href="../sui-framework/bag.md#0x2_bag_Bag">bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="0x0_system_inner_v1_SystemEpochInfoEvent"></a>

## Struct `SystemEpochInfoEvent`

Event containing system-level epoch information, emitted during
the epoch advancement message.


<pre><code><b>struct</b> <a href="system_inner.md#0x0_system_inner_v1_SystemEpochInfoEvent">SystemEpochInfoEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>protocol_version: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>computation_price_per_unit_size: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>total_stake: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>stake_subsidy_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>total_computation_fees: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>total_stake_rewards_distributed: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>last_processed_checkpoint_sequence_number: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x0_system_inner_v1_SystemQuorumVerifiedEvent"></a>

## Struct `SystemQuorumVerifiedEvent`

Event emitted after verifing quorum of signature.


<pre><code><b>struct</b> <a href="system_inner.md#0x0_system_inner_v1_SystemQuorumVerifiedEvent">SystemQuorumVerifiedEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>total_signers_stake: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x0_system_inner_v1_SystemProtocolCapVerifiedEvent"></a>

## Struct `SystemProtocolCapVerifiedEvent`

Event emitted during verifing quorum checkpoint submmision signature.


<pre><code><b>struct</b> <a href="system_inner.md#0x0_system_inner_v1_SystemProtocolCapVerifiedEvent">SystemProtocolCapVerifiedEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>protocol_cap_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x0_system_inner_v1_SystemCheckpointInfoEvent"></a>

## Struct `SystemCheckpointInfoEvent`

Event containing system-level checkpoint information, emitted during
the checkpoint submmision message.


<pre><code><b>struct</b> <a href="system_inner.md#0x0_system_inner_v1_SystemCheckpointInfoEvent">SystemCheckpointInfoEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>sequence_number: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>timestamp_ms: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x0_system_inner_v1_TestMessageEvent"></a>

## Struct `TestMessageEvent`



<pre><code><b>struct</b> <a href="system_inner.md#0x0_system_inner_v1_TestMessageEvent">TestMessageEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>sequence_number: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>authority: u32</code>
</dt>
<dd>

</dd>
<dt>
<code>num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x0_system_inner_v1_BASIS_POINT_DENOMINATOR"></a>



<pre><code><b>const</b> <a href="system_inner.md#0x0_system_inner_v1_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a>: u16 = 10000;
</code></pre>



<a name="0x0_system_inner_v1_CHECKPOINT_MESSAGE_INTENT"></a>



<pre><code><b>const</b> <a href="system_inner.md#0x0_system_inner_v1_CHECKPOINT_MESSAGE_INTENT">CHECKPOINT_MESSAGE_INTENT</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [1, 0, 0];
</code></pre>



<a name="0x0_system_inner_v1_EActiveCommitteeMustInitialize"></a>



<pre><code>#[error]
<b>const</b> <a href="system_inner.md#0x0_system_inner_v1_EActiveCommitteeMustInitialize">EActiveCommitteeMustInitialize</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = b"Fitst active <a href="committee.md#0x0_committee">committee</a> must initialize.";
</code></pre>



<a name="0x0_system_inner_v1_EAdvancedToWrongEpoch"></a>



<pre><code><b>const</b> <a href="system_inner.md#0x0_system_inner_v1_EAdvancedToWrongEpoch">EAdvancedToWrongEpoch</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 8;
</code></pre>



<a name="0x0_system_inner_v1_EBpsTooLarge"></a>



<pre><code><b>const</b> <a href="system_inner.md#0x0_system_inner_v1_EBpsTooLarge">EBpsTooLarge</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 5;
</code></pre>



<a name="0x0_system_inner_v1_ECannotInitialize"></a>



<pre><code>#[error]
<b>const</b> <a href="system_inner.md#0x0_system_inner_v1_ECannotInitialize">ECannotInitialize</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = b"Too early for initialization time or alreay initialized.";
</code></pre>



<a name="0x0_system_inner_v1_EIncorrectEpochInCheckpoint"></a>



<pre><code>#[error]
<b>const</b> <a href="system_inner.md#0x0_system_inner_v1_EIncorrectEpochInCheckpoint">EIncorrectEpochInCheckpoint</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = b"The checkpoint epoch is incorrect.";
</code></pre>



<a name="0x0_system_inner_v1_ELimitExceeded"></a>



<pre><code><b>const</b> <a href="system_inner.md#0x0_system_inner_v1_ELimitExceeded">ELimitExceeded</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1;
</code></pre>



<a name="0x0_system_inner_v1_EUnauthorizedProtocolCap"></a>



<pre><code>#[error]
<b>const</b> <a href="system_inner.md#0x0_system_inner_v1_EUnauthorizedProtocolCap">EUnauthorizedProtocolCap</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = b"The protocol cap is unauthorized.";
</code></pre>



<a name="0x0_system_inner_v1_EWrongCheckpointSequenceNumber"></a>



<pre><code>#[error]
<b>const</b> <a href="system_inner.md#0x0_system_inner_v1_EWrongCheckpointSequenceNumber">EWrongCheckpointSequenceNumber</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = b"The checkpoint sequence number should be the expected next one.";
</code></pre>



<a name="0x0_system_inner_v1_create"></a>

## Function `create`

Create a new IkaSystemState object and make it shared.
This function will be called only once in init.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_create">create</a>(upgrade_caps: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../sui-framework/package.md#0x2_package_UpgradeCap">package::UpgradeCap</a>&gt;, validators: <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, protocol_version: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, epoch_start_timestamp_ms: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, parameters: <a href="system_inner.md#0x0_system_inner_v1_SystemParametersV1">system_inner_v1::SystemParametersV1</a>, <a href="protocol_treasury.md#0x0_protocol_treasury">protocol_treasury</a>: <a href="protocol_treasury.md#0x0_protocol_treasury_ProtocolTreasury">protocol_treasury::ProtocolTreasury</a>, authorized_protocol_cap_ids: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_create">create</a>(
    upgrade_caps: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;UpgradeCap&gt;,
    validators: ValidatorSet,
    protocol_version: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    epoch_start_timestamp_ms: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    parameters: <a href="system_inner.md#0x0_system_inner_v1_SystemParametersV1">SystemParametersV1</a>,
    <a href="protocol_treasury.md#0x0_protocol_treasury">protocol_treasury</a>: ProtocolTreasury,
    authorized_protocol_cap_ids: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;ID&gt;,
    ctx: &<b>mut</b> TxContext,
): <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a> {
    // This type is fixed <b>as</b> it's created at <a href="init.md#0x0_init">init</a>. It should not be updated during type upgrade.
    <b>let</b> system_state = <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a> {
        epoch: 0,
        protocol_version,
        upgrade_caps,
        validators,
        parameters,
        computation_price_per_unit_size: 0,
        <a href="protocol_treasury.md#0x0_protocol_treasury">protocol_treasury</a>,
        epoch_start_timestamp_ms,
        total_messages_processed: 0,
        last_processed_checkpoint_sequence_number: <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(),
        previous_epoch_last_checkpoint_sequence_number: 0,
        computation_reward: <a href="../sui-framework/balance.md#0x2_balance_zero">balance::zero</a>(),
        authorized_protocol_cap_ids,
        extra_fields: <a href="../sui-framework/bag.md#0x2_bag_new">bag::new</a>(ctx),
    };
    system_state
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_create_system_parameters"></a>

## Function `create_system_parameters`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_create_system_parameters">create_system_parameters</a>(epoch_duration_ms: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, stake_subsidy_start_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, min_validator_count: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, max_validator_count: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, min_validator_joining_stake: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, validator_low_stake_threshold: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, validator_very_low_stake_threshold: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, validator_low_stake_grace_period: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, reward_slashing_rate: u16, lock_active_committee: bool, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="system_inner.md#0x0_system_inner_v1_SystemParametersV1">system_inner_v1::SystemParametersV1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_create_system_parameters">create_system_parameters</a>(
    epoch_duration_ms: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    stake_subsidy_start_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    // Validator <a href="committee.md#0x0_committee">committee</a> parameters
    min_validator_count: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    max_validator_count: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    min_validator_joining_stake: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    validator_low_stake_threshold: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    validator_very_low_stake_threshold: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    validator_low_stake_grace_period: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    reward_slashing_rate: u16,
    lock_active_committee: bool,
    ctx: &<b>mut</b> TxContext,
): <a href="system_inner.md#0x0_system_inner_v1_SystemParametersV1">SystemParametersV1</a> {
    // Rates can't be higher than 100%.
    <b>assert</b>!(
        reward_slashing_rate &lt;= <a href="system_inner.md#0x0_system_inner_v1_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a>,
        <a href="system_inner.md#0x0_system_inner_v1_EBpsTooLarge">EBpsTooLarge</a>,
    );
    <a href="system_inner.md#0x0_system_inner_v1_SystemParametersV1">SystemParametersV1</a> {
        epoch_duration_ms,
        stake_subsidy_start_epoch,
        min_validator_count,
        max_validator_count,
        min_validator_joining_stake,
        validator_low_stake_threshold,
        validator_very_low_stake_threshold,
        validator_low_stake_grace_period,
        reward_slashing_rate,
        lock_active_committee,
        extra_fields: <a href="../sui-framework/bag.md#0x2_bag_new">bag::new</a>(ctx),
    }
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_initialize"></a>

## Function `initialize`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_initialize">initialize</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_initialize">initialize</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &Clock,
) {
    <b>let</b> now = <a href="../sui-framework/clock.md#0x2_clock">clock</a>.timestamp_ms();
    <b>assert</b>!(self.epoch == 0 && now &gt;= self.epoch_start_timestamp_ms, <a href="system_inner.md#0x0_system_inner_v1_ECannotInitialize">ECannotInitialize</a>);
    <b>assert</b>!(self.<a href="system_inner.md#0x0_system_inner_v1_active_committee">active_committee</a>().members().is_empty(), <a href="system_inner.md#0x0_system_inner_v1_ECannotInitialize">ECannotInitialize</a>);
    self.validators.<a href="system_inner.md#0x0_system_inner_v1_initialize">initialize</a>();
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_request_add_validator_candidate"></a>

## Function `request_add_validator_candidate`

Can be called by anyone who wishes to become a validator candidate and starts accuring delegated
stakes in their staking pool. Once they have at least <code>MIN_VALIDATOR_JOINING_STAKE</code> amount of stake they
can call <code>request_add_validator</code> to officially become an active validator at the next epoch.
Aborts if the caller is already a pending or active validator, or a validator candidate.
Note: <code>proof_of_possession_bytes</code> MUST be a valid signature using proof_of_possession_sender and protocol_pubkey_bytes.
To produce a valid PoP, run [fn test_proof_of_possession_bytes].


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_request_add_validator_candidate">request_add_validator_candidate</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, payment_address: <b>address</b>, protocol_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, network_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, consensus_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, proof_of_possession_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, name: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, description: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, image_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, project_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, network_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, p2p_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, consensus_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, computation_price: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, commission_rate: u16, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): (<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, <a href="validator_cap.md#0x0_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_request_add_validator_candidate">request_add_validator_candidate</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
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
    self.validators.<a href="system_inner.md#0x0_system_inner_v1_request_add_validator_candidate">request_add_validator_candidate</a>(
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
    )
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_request_remove_validator_candidate"></a>

## Function `request_remove_validator_candidate`

Called by a validator candidate to remove themselves from the candidacy. After this call
their staking pool becomes deactivate.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_request_remove_validator_candidate">request_remove_validator_candidate</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_request_remove_validator_candidate">request_remove_validator_candidate</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    cap: &ValidatorCap,
) {
    self.validators.<a href="system_inner.md#0x0_system_inner_v1_request_remove_validator_candidate">request_remove_validator_candidate</a>(self.epoch, cap);
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_request_add_validator"></a>

## Function `request_add_validator`

Called by a validator candidate to add themselves to the active validator set beginning next epoch.
Aborts if the validator is a duplicate with one of the pending or active validators, or if the amount of
stake the validator has doesn't meet the min threshold, or if the number of new validators for the next
epoch has already reached the maximum.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_request_add_validator">request_add_validator</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_request_add_validator">request_add_validator</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    cap: &ValidatorCap,
) {
    <b>assert</b>!(
        self.validators.pending_active_validators_count() &lt; self.parameters.max_validator_count,
        <a href="system_inner.md#0x0_system_inner_v1_ELimitExceeded">ELimitExceeded</a>,
    );

    self.validators.<a href="system_inner.md#0x0_system_inner_v1_request_add_validator">request_add_validator</a>(self.parameters.min_validator_joining_stake, cap);
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_request_remove_validator"></a>

## Function `request_remove_validator`

A validator can call this function to request a removal in the next epoch.
We use the sender of <code>ctx</code> to look up the validator
(i.e. sender must match the sui_address in the validator).
At the end of the epoch, the <code><a href="validator.md#0x0_validator">validator</a></code> object will be returned to the sui_address
of the validator.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_request_remove_validator">request_remove_validator</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_request_remove_validator">request_remove_validator</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>, cap: &ValidatorCap) {
    // Only check <b>min</b> <a href="validator.md#0x0_validator">validator</a> condition <b>if</b> the current number of validators satisfy the constraint.
    // This is so that <b>if</b> we somehow already are in a state <b>where</b> we have less than <b>min</b> validators, it no longer matters
    // and is ok <b>to</b> stay so. This is useful for a <a href="test.md#0x0_test">test</a> setup.
    <b>if</b> (self.<a href="system_inner.md#0x0_system_inner_v1_active_committee">active_committee</a>().members().length() &gt;= self.parameters.min_validator_count) {
        <b>assert</b>!(
            self.validators.pending_active_validators_count() &gt; self.parameters.min_validator_count,
            <a href="system_inner.md#0x0_system_inner_v1_ELimitExceeded">ELimitExceeded</a>,
        );
    };

    self.validators.<a href="system_inner.md#0x0_system_inner_v1_request_remove_validator">request_remove_validator</a>(cap)
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_request_set_computation_price"></a>

## Function `request_set_computation_price`

A validator can call this function to submit a new computation price quote, to be
used for the computation price per unit size calculation at the end of the epoch.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_request_set_computation_price">request_set_computation_price</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, operation_cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, new_computation_price: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_request_set_computation_price">request_set_computation_price</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    operation_cap: &ValidatorOperationCap,
    new_computation_price: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
) {
    // Verify that the capability is still valid.
    self.validators.verify_operation_cap(operation_cap);
    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self
        .validators
        .get_validator_mut_with_operation_cap(operation_cap);

    <a href="validator.md#0x0_validator">validator</a>.<a href="system_inner.md#0x0_system_inner_v1_request_set_computation_price">request_set_computation_price</a>(operation_cap, new_computation_price);
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_set_candidate_validator_computation_price"></a>

## Function `set_candidate_validator_computation_price`

This function is used to set new computation price for candidate validators


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_set_candidate_validator_computation_price">set_candidate_validator_computation_price</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, operation_cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, new_computation_price: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_set_candidate_validator_computation_price">set_candidate_validator_computation_price</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    operation_cap: &ValidatorOperationCap,
    new_computation_price: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
) {
    // Verify that the capability is still valid.
    self.validators.verify_operation_cap(operation_cap);
    <b>let</b> candidate = self
        .validators
        .get_validator_mut_with_operation_cap_including_candidates(operation_cap);
    candidate.set_candidate_computation_price(operation_cap, new_computation_price)
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_request_set_commission_rate"></a>

## Function `request_set_commission_rate`

A validator can call this function to set a new commission rate, updated at the end of
the epoch.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_request_set_commission_rate">request_set_commission_rate</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, new_commission_rate: u16, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_request_set_commission_rate">request_set_commission_rate</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    new_commission_rate: u16,
    cap: &ValidatorCap,
) {
    self
        .validators
        .<a href="system_inner.md#0x0_system_inner_v1_request_set_commission_rate">request_set_commission_rate</a>(
            new_commission_rate,
            cap,
        )
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_set_candidate_validator_commission_rate"></a>

## Function `set_candidate_validator_commission_rate`

This function is used to set new commission rate for candidate validators


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_set_candidate_validator_commission_rate">set_candidate_validator_commission_rate</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, new_commission_rate: u16, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_set_candidate_validator_commission_rate">set_candidate_validator_commission_rate</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    new_commission_rate: u16,
    cap: &ValidatorCap,
) {
    <b>let</b> candidate = self.validators.get_validator_mut_with_cap_including_candidates(cap);
    candidate.set_candidate_commission_rate(new_commission_rate)
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_request_add_stake"></a>

## Function `request_add_stake`

Add stake to a validator's staking pool.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_request_add_stake">request_add_stake</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, stake: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;, validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="staked_ika.md#0x0_staked_ika_StakedIka">staked_ika::StakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_request_add_stake">request_add_stake</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    stake: Coin&lt;IKA&gt;,
    validator_id: ID,
    ctx: &<b>mut</b> TxContext,
): StakedIka {
    self
        .validators
        .<a href="system_inner.md#0x0_system_inner_v1_request_add_stake">request_add_stake</a>(
            self.epoch,
            validator_id,
            stake.into_balance(),
            ctx,
        )
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_request_add_stake_mul_coin"></a>

## Function `request_add_stake_mul_coin`

Add stake to a validator's staking pool using multiple coins.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_request_add_stake_mul_coin">request_add_stake_mul_coin</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, stakes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;&gt;, stake_amount: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;, validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="staked_ika.md#0x0_staked_ika_StakedIka">staked_ika::StakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_request_add_stake_mul_coin">request_add_stake_mul_coin</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    stakes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;Coin&lt;IKA&gt;&gt;,
    stake_amount: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;,
    validator_id: ID,
    ctx: &<b>mut</b> TxContext,
): StakedIka {
    <b>let</b> <a href="../sui-framework/balance.md#0x2_balance">balance</a> = <a href="system_inner.md#0x0_system_inner_v1_extract_coin_balance">extract_coin_balance</a>(stakes, stake_amount, ctx);
    self.validators.<a href="system_inner.md#0x0_system_inner_v1_request_add_stake">request_add_stake</a>(self.epoch, validator_id, <a href="../sui-framework/balance.md#0x2_balance">balance</a>, ctx)
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_request_withdraw_stake"></a>

## Function `request_withdraw_stake`

Withdraw some portion of a stake from a validator's staking pool.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_request_withdraw_stake">request_withdraw_stake</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: <a href="staked_ika.md#0x0_staked_ika_StakedIka">staked_ika::StakedIka</a>): <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_request_withdraw_stake">request_withdraw_stake</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: StakedIka,
): Balance&lt;IKA&gt; {
    self.validators.<a href="system_inner.md#0x0_system_inner_v1_request_withdraw_stake">request_withdraw_stake</a>(self.epoch, <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>)
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_convert_to_fungible_staked_ika"></a>

## Function `convert_to_fungible_staked_ika`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_convert_to_fungible_staked_ika">convert_to_fungible_staked_ika</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: <a href="staked_ika.md#0x0_staked_ika_StakedIka">staked_ika::StakedIka</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_convert_to_fungible_staked_ika">convert_to_fungible_staked_ika</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: StakedIka,
    ctx: &<b>mut</b> TxContext,
): FungibleStakedIka {
    self.validators.<a href="system_inner.md#0x0_system_inner_v1_convert_to_fungible_staked_ika">convert_to_fungible_staked_ika</a>(self.epoch, <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>, ctx)
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_redeem_fungible_staked_ika"></a>

## Function `redeem_fungible_staked_ika`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_redeem_fungible_staked_ika">redeem_fungible_staked_ika</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, fungible_staked_ika: <a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>): <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_redeem_fungible_staked_ika">redeem_fungible_staked_ika</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    fungible_staked_ika: FungibleStakedIka,
): Balance&lt;IKA&gt; {
    self.validators.<a href="system_inner.md#0x0_system_inner_v1_redeem_fungible_staked_ika">redeem_fungible_staked_ika</a>(self.epoch, fungible_staked_ika)
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_report_validator"></a>

## Function `report_validator`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_report_validator">report_validator</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, reportee_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_report_validator">report_validator</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    cap: &ValidatorOperationCap,
    reportee_id: ID,
) {
    self.validators.<a href="system_inner.md#0x0_system_inner_v1_report_validator">report_validator</a>(cap, reportee_id);
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_undo_report_validator"></a>

## Function `undo_report_validator`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_undo_report_validator">undo_report_validator</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, reportee_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_undo_report_validator">undo_report_validator</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    cap: &ValidatorOperationCap,
    reportee_id: ID,
) {
    self.validators.<a href="system_inner.md#0x0_system_inner_v1_undo_report_validator">undo_report_validator</a>(cap, reportee_id);
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_rotate_operation_cap"></a>

## Function `rotate_operation_cap`

Create a new <code>ValidatorOperationCap</code> and registers it.
The original object is thus revoked.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_rotate_operation_cap">rotate_operation_cap</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="validator_cap.md#0x0_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_rotate_operation_cap">rotate_operation_cap</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>, cap: &ValidatorCap, ctx: &<b>mut</b> TxContext): ValidatorOperationCap {
    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.validators.get_validator_mut_with_cap_including_candidates(cap);
    <a href="validator.md#0x0_validator">validator</a>.new_validator_operation_cap(cap, ctx)
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_update_validator_payment_address"></a>

## Function `update_validator_payment_address`

Update a validator's payment address.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_validator_payment_address">update_validator_payment_address</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, payment_address: <b>address</b>, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_validator_payment_address">update_validator_payment_address</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    payment_address: <b>address</b>,
    cap: &ValidatorCap
) {
    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.validators.get_validator_mut_with_cap_including_candidates(cap);

    <a href="validator.md#0x0_validator">validator</a>.update_payment_address(payment_address);
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_update_validator_name"></a>

## Function `update_validator_name`

Update a validator's name.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_validator_name">update_validator_name</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, name: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_validator_name">update_validator_name</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    name: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap
) {
    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.validators.get_validator_mut_with_cap_including_candidates(cap);

    <a href="validator.md#0x0_validator">validator</a>.update_name(name);
    self.validators.assert_no_pending_or_active_duplicates(cap.validator_id());
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_update_validator_description"></a>

## Function `update_validator_description`

Update a validator's description


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_validator_description">update_validator_description</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, description: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_validator_description">update_validator_description</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    description: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap
) {
    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.validators.get_validator_mut_with_cap_including_candidates(cap);
    <a href="validator.md#0x0_validator">validator</a>.update_description(description);
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_update_validator_image_url"></a>

## Function `update_validator_image_url`

Update a validator's image url


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_validator_image_url">update_validator_image_url</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, image_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_validator_image_url">update_validator_image_url</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    image_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap
) {
    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.validators.get_validator_mut_with_cap_including_candidates(cap);
    <a href="validator.md#0x0_validator">validator</a>.update_image_url(image_url);
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_update_validator_project_url"></a>

## Function `update_validator_project_url`

Update a validator's project url


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_validator_project_url">update_validator_project_url</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, project_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_validator_project_url">update_validator_project_url</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    project_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap
) {
    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.validators.get_validator_mut_with_cap_including_candidates(cap);
    <a href="validator.md#0x0_validator">validator</a>.update_project_url(project_url);
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_update_validator_next_epoch_network_address"></a>

## Function `update_validator_next_epoch_network_address`

Update a validator's network address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_validator_next_epoch_network_address">update_validator_next_epoch_network_address</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, network_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_validator_next_epoch_network_address">update_validator_next_epoch_network_address</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    network_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap
) {
    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.validators.get_validator_mut_with_cap(cap);
    <a href="validator.md#0x0_validator">validator</a>.update_next_epoch_network_address(network_address);
    self.validators.assert_no_pending_or_active_duplicates(cap.validator_id());
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_update_candidate_validator_network_address"></a>

## Function `update_candidate_validator_network_address`

Update candidate validator's network address.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_candidate_validator_network_address">update_candidate_validator_network_address</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, network_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_candidate_validator_network_address">update_candidate_validator_network_address</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    network_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap
) {
    <b>let</b> candidate = self.validators.get_validator_mut_with_cap_including_candidates(cap);
    candidate.update_candidate_network_address(network_address);
    self.validators.assert_no_pending_or_active_duplicates(cap.validator_id());
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_update_validator_next_epoch_p2p_address"></a>

## Function `update_validator_next_epoch_p2p_address`

Update a validator's p2p address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_validator_next_epoch_p2p_address">update_validator_next_epoch_p2p_address</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, p2p_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_validator_next_epoch_p2p_address">update_validator_next_epoch_p2p_address</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    p2p_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap
) {
    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.validators.get_validator_mut_with_cap(cap);
    <a href="validator.md#0x0_validator">validator</a>.update_next_epoch_p2p_address(p2p_address);
    self.validators.assert_no_pending_or_active_duplicates(cap.validator_id());
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_update_candidate_validator_p2p_address"></a>

## Function `update_candidate_validator_p2p_address`

Update candidate validator's p2p address.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_candidate_validator_p2p_address">update_candidate_validator_p2p_address</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, p2p_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_candidate_validator_p2p_address">update_candidate_validator_p2p_address</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    p2p_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap
) {
    <b>let</b> candidate = self.validators.get_validator_mut_with_cap_including_candidates(cap);
    candidate.update_candidate_p2p_address(p2p_address);
    self.validators.assert_no_pending_or_active_duplicates(cap.validator_id());
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_update_validator_next_epoch_consensus_address"></a>

## Function `update_validator_next_epoch_consensus_address`

Update a validator's consensus address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_validator_next_epoch_consensus_address">update_validator_next_epoch_consensus_address</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, consensus_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_validator_next_epoch_consensus_address">update_validator_next_epoch_consensus_address</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    consensus_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap
) {
    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.validators.get_validator_mut_with_cap(cap);
    <a href="validator.md#0x0_validator">validator</a>.update_next_epoch_consensus_address(consensus_address);
    self.validators.assert_no_pending_or_active_duplicates(cap.validator_id());
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_update_candidate_validator_consensus_address"></a>

## Function `update_candidate_validator_consensus_address`

Update candidate validator's consensus address.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_candidate_validator_consensus_address">update_candidate_validator_consensus_address</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, consensus_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_candidate_validator_consensus_address">update_candidate_validator_consensus_address</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    consensus_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap
) {
    <b>let</b> candidate = self.validators.get_validator_mut_with_cap_including_candidates(cap);
    candidate.update_candidate_consensus_address(consensus_address);
    self.validators.assert_no_pending_or_active_duplicates(cap.validator_id());
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_update_validator_next_epoch_protocol_pubkey_bytes"></a>

## Function `update_validator_next_epoch_protocol_pubkey_bytes`

Update a validator's public key of protocol key and proof of possession.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_validator_next_epoch_protocol_pubkey_bytes">update_validator_next_epoch_protocol_pubkey_bytes</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, protocol_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, proof_of_possession_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, ctx: &<a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_validator_next_epoch_protocol_pubkey_bytes">update_validator_next_epoch_protocol_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    protocol_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    proof_of_possession_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap,
    ctx: &TxContext,
) {
    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.validators.get_validator_mut_with_cap(cap);
    <a href="validator.md#0x0_validator">validator</a>.update_next_epoch_protocol_pubkey_bytes(protocol_pubkey, proof_of_possession_bytes, ctx);
    self.validators.assert_no_pending_or_active_duplicates(cap.validator_id());
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_update_candidate_validator_protocol_pubkey_bytes"></a>

## Function `update_candidate_validator_protocol_pubkey_bytes`

Update candidate validator's public key of protocol key and proof of possession.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_candidate_validator_protocol_pubkey_bytes">update_candidate_validator_protocol_pubkey_bytes</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, protocol_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, proof_of_possession_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, ctx: &<a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_candidate_validator_protocol_pubkey_bytes">update_candidate_validator_protocol_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    protocol_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    proof_of_possession_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap,
    ctx: &TxContext,
) {
    <b>let</b> candidate = self.validators.get_validator_mut_with_cap_including_candidates(cap);
    candidate.update_candidate_protocol_pubkey_bytes(protocol_pubkey, proof_of_possession_bytes, ctx);
    self.validators.assert_no_pending_or_active_duplicates(cap.validator_id());
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_update_validator_next_epoch_consensus_pubkey_bytes"></a>

## Function `update_validator_next_epoch_consensus_pubkey_bytes`

Update a validator's public key of worker key.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_validator_next_epoch_consensus_pubkey_bytes">update_validator_next_epoch_consensus_pubkey_bytes</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, consensus_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_validator_next_epoch_consensus_pubkey_bytes">update_validator_next_epoch_consensus_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    consensus_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap
) {
    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.validators.get_validator_mut_with_cap(cap);
    <a href="validator.md#0x0_validator">validator</a>.update_next_epoch_consensus_pubkey_bytes(consensus_pubkey_bytes);
    self.validators.assert_no_pending_or_active_duplicates(cap.validator_id());
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_update_candidate_validator_consensus_pubkey_bytes"></a>

## Function `update_candidate_validator_consensus_pubkey_bytes`

Update candidate validator's public key of worker key.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_candidate_validator_consensus_pubkey_bytes">update_candidate_validator_consensus_pubkey_bytes</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, consensus_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_candidate_validator_consensus_pubkey_bytes">update_candidate_validator_consensus_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    consensus_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap
) {
    <b>let</b> candidate = self.validators.get_validator_mut_with_cap_including_candidates(cap);
    candidate.update_candidate_consensus_pubkey_bytes(consensus_pubkey_bytes);
    self.validators.assert_no_pending_or_active_duplicates(cap.validator_id());
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_update_validator_next_epoch_network_pubkey_bytes"></a>

## Function `update_validator_next_epoch_network_pubkey_bytes`

Update a validator's public key of network key.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_validator_next_epoch_network_pubkey_bytes">update_validator_next_epoch_network_pubkey_bytes</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, network_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_validator_next_epoch_network_pubkey_bytes">update_validator_next_epoch_network_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    network_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap
) {
    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = self.validators.get_validator_mut_with_cap(cap);
    <a href="validator.md#0x0_validator">validator</a>.update_next_epoch_network_pubkey_bytes(network_pubkey);
    self.validators.assert_no_pending_or_active_duplicates(cap.validator_id());
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_update_candidate_validator_network_pubkey_bytes"></a>

## Function `update_candidate_validator_network_pubkey_bytes`

Update candidate validator's public key of network key.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_candidate_validator_network_pubkey_bytes">update_candidate_validator_network_pubkey_bytes</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, network_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_update_candidate_validator_network_pubkey_bytes">update_candidate_validator_network_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    network_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap
) {
    <b>let</b> candidate = self.validators.get_validator_mut_with_cap_including_candidates(cap);
    candidate.update_candidate_network_pubkey_bytes(network_pubkey);
    self.validators.assert_no_pending_or_active_duplicates(cap.validator_id());
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_advance_epoch"></a>

## Function `advance_epoch`

This function should be called at the end of an epoch, and advances the system to the next epoch.
It does the following things:
1. Add storage charge to the storage fund.
2. Burn the storage rebates from the storage fund. These are already refunded to transaction sender's
gas coins.
3. Distribute computation charge to validator stake.
4. Update all validators.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_advance_epoch">advance_epoch</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, new_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, next_protocol_version: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, epoch_start_timestamp_ms: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_advance_epoch">advance_epoch</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    new_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    next_protocol_version: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    epoch_start_timestamp_ms: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, // Timestamp of the epoch start
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> prev_epoch_start_timestamp = self.epoch_start_timestamp_ms;
    self.epoch_start_timestamp_ms = epoch_start_timestamp_ms;

    // TODO: remove this in later upgrade.
    <b>if</b> (self.parameters.stake_subsidy_start_epoch &gt; 0) {
        self.parameters.stake_subsidy_start_epoch = 20;
    };

    <b>let</b> <b>mut</b> stake_subsidy = <a href="../sui-framework/balance.md#0x2_balance_zero">balance::zero</a>();

    // during the transition from epoch N <b>to</b> epoch N + 1, self.<a href="system_inner.md#0x0_system_inner_v1_epoch">epoch</a>() will <b>return</b> N
    <b>let</b> epoch = self.<a href="system_inner.md#0x0_system_inner_v1_epoch">epoch</a>();
    // Include stake subsidy in the rewards given out <b>to</b> validators and stakers.
    // Delay distributing any stake subsidies until after `stake_subsidy_start_epoch`.
    // And <b>if</b> this epoch is shorter than the regular epoch duration, don't distribute any stake subsidy.
    <b>if</b> (
        epoch &gt;= self.parameters.stake_subsidy_start_epoch  &&
            epoch_start_timestamp_ms &gt;= prev_epoch_start_timestamp + self.parameters.epoch_duration_ms
    ) {
        stake_subsidy.join(self.<a href="protocol_treasury.md#0x0_protocol_treasury">protocol_treasury</a>.stake_subsidy_for_distribution(ctx));
    };


    <b>let</b> computation_reward_amount_before_distribution = self.computation_reward.value();

    <b>let</b> stake_subsidy_amount = stake_subsidy.value();
    <b>let</b> <b>mut</b> total_reward = sui::balance::zero&lt;IKA&gt;();
    total_reward.join(self.computation_reward.withdraw_all());
    total_reward.join(stake_subsidy);
    <b>let</b> total_reward_amount_before_distribution = total_reward.value();

    self.epoch = self.epoch + 1;
    // Sanity check <b>to</b> make sure we are advancing <b>to</b> the right epoch.
    <b>assert</b>!(new_epoch == self.epoch, <a href="system_inner.md#0x0_system_inner_v1_EAdvancedToWrongEpoch">EAdvancedToWrongEpoch</a>);

    self
        .validators
        .<a href="system_inner.md#0x0_system_inner_v1_advance_epoch">advance_epoch</a>(
            epoch,
            new_epoch,
            &<b>mut</b> total_reward,
            self.parameters.reward_slashing_rate,

            ctx,
        );

    <b>let</b> new_total_stake = self.validators.total_stake();

    <b>let</b> total_reward_amount_after_distribution = total_reward.value();
    <b>let</b> total_reward_distributed =
         total_reward_amount_before_distribution - total_reward_amount_after_distribution;

    // Because of precision issues <b>with</b> integer divisions, we expect that there will be some
    // remaining <a href="../sui-framework/balance.md#0x2_balance">balance</a> in `computation_reward`.
    self.computation_reward.join(total_reward);

    self.protocol_version = next_protocol_version;

    <b>let</b> active_committee = self.<a href="system_inner.md#0x0_system_inner_v1_active_committee">active_committee</a>();
    // Derive the computation price per unit size for the new epoch
    self.computation_price_per_unit_size = self.validators.derive_computation_price_per_unit_size(&active_committee);

    <b>let</b> last_processed_checkpoint_sequence_number = *self.last_processed_checkpoint_sequence_number.borrow();
    self.previous_epoch_last_checkpoint_sequence_number = last_processed_checkpoint_sequence_number;

    <a href="../sui-framework/event.md#0x2_event_emit">event::emit</a>(<a href="system_inner.md#0x0_system_inner_v1_SystemEpochInfoEvent">SystemEpochInfoEvent</a> {
        epoch: self.epoch,
        protocol_version: self.protocol_version,
        computation_price_per_unit_size: self.computation_price_per_unit_size,
        total_stake: new_total_stake,
        stake_subsidy_amount,
        total_computation_fees: computation_reward_amount_before_distribution,
        total_stake_rewards_distributed: total_reward_distributed,
        last_processed_checkpoint_sequence_number,
    });
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_process_mid_epoch"></a>

## Function `process_mid_epoch`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_process_mid_epoch">process_mid_epoch</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_process_mid_epoch">process_mid_epoch</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
) {
    self.validators.<a href="system_inner.md#0x0_system_inner_v1_process_mid_epoch">process_mid_epoch</a>(
        self.epoch,
        self.parameters.lock_active_committee,
        self.parameters.validator_low_stake_threshold,
        self.parameters.validator_very_low_stake_threshold,
        self.parameters.validator_low_stake_grace_period,
    );
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_epoch"></a>

## Function `epoch`

Return the current epoch number. Useful for applications that need a coarse-grained concept of time,
since epochs are ever-increasing and epoch changes are intended to happen every 24 hours.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_epoch">epoch</a>(self: &<a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_epoch">epoch</a>(self: &<a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    self.epoch
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_protocol_version"></a>

## Function `protocol_version`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_protocol_version">protocol_version</a>(self: &<a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_protocol_version">protocol_version</a>(self: &<a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    self.protocol_version
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_upgrade_caps"></a>

## Function `upgrade_caps`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_upgrade_caps">upgrade_caps</a>(self: &<a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>): &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../sui-framework/package.md#0x2_package_UpgradeCap">package::UpgradeCap</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_upgrade_caps">upgrade_caps</a>(self: &<a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>): &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;UpgradeCap&gt; {
    &self.upgrade_caps
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_epoch_start_timestamp_ms"></a>

## Function `epoch_start_timestamp_ms`

Returns unix timestamp of the start of current epoch


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_epoch_start_timestamp_ms">epoch_start_timestamp_ms</a>(self: &<a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_epoch_start_timestamp_ms">epoch_start_timestamp_ms</a>(self: &<a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    self.epoch_start_timestamp_ms
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_validator_stake_amount"></a>

## Function `validator_stake_amount`

Returns the total amount staked with <code>validator_id</code>.
Aborts if <code>validator_id</code> is not an active validator.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_validator_stake_amount">validator_stake_amount</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_validator_stake_amount">validator_stake_amount</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    validator_id: ID,
): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    self.validators.validator_total_stake_amount(validator_id)
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_get_reporters_of"></a>

## Function `get_reporters_of`

Returns all the validators who are currently reporting <code>validator_id</code>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_get_reporters_of">get_reporters_of</a>(self: &<a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>): <a href="../sui-framework/vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_get_reporters_of">get_reporters_of</a>(self: &<a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>, validator_id: ID): VecSet&lt;ID&gt; {
    self.<a href="system_inner.md#0x0_system_inner_v1_get_reporters_of">get_reporters_of</a>(validator_id)
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_pool_exchange_rates"></a>

## Function `pool_exchange_rates`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_pool_exchange_rates">pool_exchange_rates</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>): &<a href="../sui-framework/table.md#0x2_table_Table">table::Table</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="staking_pool.md#0x0_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_pool_exchange_rates">pool_exchange_rates</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    validator_id: ID,
): &Table&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, PoolTokenExchangeRate&gt; {
    <b>let</b> validators = &<b>mut</b> self.validators;
    validators.<a href="system_inner.md#0x0_system_inner_v1_pool_exchange_rates">pool_exchange_rates</a>(validator_id)
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_active_committee"></a>

## Function `active_committee`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_active_committee">active_committee</a>(self: &<a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>): <a href="committee.md#0x0_committee_Committee">committee::Committee</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_active_committee">active_committee</a>(self: &<a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>): Committee {
    <b>let</b> <a href="validator_set.md#0x0_validator_set">validator_set</a> = &self.validators;
    <a href="validator_set.md#0x0_validator_set">validator_set</a>.<a href="system_inner.md#0x0_system_inner_v1_active_committee">active_committee</a>()
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_process_checkpoint_message_by_cap"></a>

## Function `process_checkpoint_message_by_cap`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_process_checkpoint_message_by_cap">process_checkpoint_message_by_cap</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, cap: &<a href="protocol_cap.md#0x0_protocol_cap_ProtocolCap">protocol_cap::ProtocolCap</a>, message: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_process_checkpoint_message_by_cap">process_checkpoint_message_by_cap</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    cap: &ProtocolCap,
    message: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> protocol_cap_id = <a href="../sui-framework/object.md#0x2_object_id">object::id</a>(cap);

    <b>assert</b>!(self.authorized_protocol_cap_ids.contains(&protocol_cap_id), <a href="system_inner.md#0x0_system_inner_v1_EUnauthorizedProtocolCap">EUnauthorizedProtocolCap</a>);

    <a href="../sui-framework/event.md#0x2_event_emit">event::emit</a>(<a href="system_inner.md#0x0_system_inner_v1_SystemProtocolCapVerifiedEvent">SystemProtocolCapVerifiedEvent</a> {
        epoch: self.epoch,
        protocol_cap_id: <a href="../sui-framework/object.md#0x2_object_id">object::id</a>(cap),
    });

    self.<a href="system_inner.md#0x0_system_inner_v1_process_checkpoint_message">process_checkpoint_message</a>(message, ctx);
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_process_checkpoint_message_by_quorum"></a>

## Function `process_checkpoint_message_by_quorum`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, signature: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, signers_bitmap: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, message: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    signature: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    signers_bitmap: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    message: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> <b>mut</b> intent_bytes = <a href="system_inner.md#0x0_system_inner_v1_CHECKPOINT_MESSAGE_INTENT">CHECKPOINT_MESSAGE_INTENT</a>;
    intent_bytes.append(message);
    intent_bytes.append(<a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&self.epoch));

    <b>let</b> total_signers_stake = self.<a href="system_inner.md#0x0_system_inner_v1_active_committee">active_committee</a>().verify_certificate(&signature, &signers_bitmap, &intent_bytes);

    <a href="../sui-framework/event.md#0x2_event_emit">event::emit</a>(<a href="system_inner.md#0x0_system_inner_v1_SystemQuorumVerifiedEvent">SystemQuorumVerifiedEvent</a> {
        epoch: self.epoch,
        total_signers_stake,
    });

    self.<a href="system_inner.md#0x0_system_inner_v1_process_checkpoint_message">process_checkpoint_message</a>(message, ctx);
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_process_checkpoint_message"></a>

## Function `process_checkpoint_message`



<pre><code><b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_process_checkpoint_message">process_checkpoint_message</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, message: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_process_checkpoint_message">process_checkpoint_message</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    message: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(!self.<a href="system_inner.md#0x0_system_inner_v1_active_committee">active_committee</a>().members().is_empty(), <a href="system_inner.md#0x0_system_inner_v1_EActiveCommitteeMustInitialize">EActiveCommitteeMustInitialize</a>);

    // first <b>let</b>'s make sure it's the correct checkpoint message
    <b>let</b> <b>mut</b> bcs_body = bcs::new(<b>copy</b> message);

    <b>let</b> epoch = bcs_body.peel_u64();
    <b>assert</b>!(epoch == self.epoch, <a href="system_inner.md#0x0_system_inner_v1_EIncorrectEpochInCheckpoint">EIncorrectEpochInCheckpoint</a>);

    <b>let</b> sequence_number = bcs_body.peel_u64();

    <b>if</b>(self.last_processed_checkpoint_sequence_number.is_none()) {
        <b>assert</b>!(sequence_number == 0, <a href="system_inner.md#0x0_system_inner_v1_EWrongCheckpointSequenceNumber">EWrongCheckpointSequenceNumber</a>);
        self.last_processed_checkpoint_sequence_number.fill(sequence_number);
    } <b>else</b> {
        <b>assert</b>!(sequence_number &gt; 0 && *self.last_processed_checkpoint_sequence_number.borrow() + 1 == sequence_number, <a href="system_inner.md#0x0_system_inner_v1_EWrongCheckpointSequenceNumber">EWrongCheckpointSequenceNumber</a>);
        self.last_processed_checkpoint_sequence_number.swap(sequence_number);
    };

    //<b>let</b> network_total_messages = bcs_body.peel_u64();
    //<b>let</b> previous_digest = bcs_body.peel_option!(|previous_digest| previous_digest.peel_vec_u8() );
    <b>let</b> timestamp_ms = bcs_body.peel_u64();

    <a href="../sui-framework/event.md#0x2_event_emit">event::emit</a>(<a href="system_inner.md#0x0_system_inner_v1_SystemCheckpointInfoEvent">SystemCheckpointInfoEvent</a> {
        epoch,
        sequence_number,
        timestamp_ms,
    });

    // now <b>let</b>'s process message

    //<b>assert</b>!(<b>false</b>, 456);

    <b>let</b> len = bcs_body.peel_vec_length();
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> message_data_type = bcs_body.peel_vec_length();
            <b>if</b> (message_data_type == 0) {
                // InitiateProcessMidEpoch
                self.<a href="system_inner.md#0x0_system_inner_v1_process_mid_epoch">process_mid_epoch</a>();
            } <b>else</b> <b>if</b> (message_data_type == 1) {
                // EndOfEpochMessage
                <b>let</b> len = bcs_body.peel_vec_length();
                <b>let</b> <b>mut</b> i = 0;
                <b>while</b> (i &lt; len) {
                    <b>let</b> end_of_epch_message_type = bcs_body.peel_vec_length();
                    // AdvanceEpoch
                    <b>if</b>(end_of_epch_message_type == 0) {
                        <b>let</b> new_epoch = bcs_body.peel_u64();
                        <b>let</b> next_protocol_version = bcs_body.peel_u64();
                        <b>let</b> epoch_start_timestamp_ms = bcs_body.peel_u64();
                        self.<a href="system_inner.md#0x0_system_inner_v1_advance_epoch">advance_epoch</a>(new_epoch, next_protocol_version, epoch_start_timestamp_ms, ctx);
                    };
                    i = i + 1;
                };
            } <b>else</b> <b>if</b> (message_data_type == 2) {
                //TestMessage
                <b>let</b> authority = bcs_body.peel_u32();
                <b>let</b> num = bcs_body.peel_u64();
                <a href="../sui-framework/event.md#0x2_event_emit">event::emit</a>(<a href="system_inner.md#0x0_system_inner_v1_TestMessageEvent">TestMessageEvent</a> {
                    epoch,
                    sequence_number,
                    authority,
                    num,
                });
            };
        i = i + 1;
    };
    self.total_messages_processed = self.total_messages_processed + i;
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_extract_coin_balance"></a>

## Function `extract_coin_balance`

Extract required Balance from vector of Coin<IKA>, transfer the remainder back to sender.


<pre><code><b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_extract_coin_balance">extract_coin_balance</a>(coins: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;&gt;, amount: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_extract_coin_balance">extract_coin_balance</a>(
    <b>mut</b> coins: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;Coin&lt;IKA&gt;&gt;,
    amount: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;,
    ctx: &<b>mut</b> TxContext,
): Balance&lt;IKA&gt; {
    <b>let</b> <b>mut</b> merged_coin = coins.pop_back();
    merged_coin.join_vec(coins);

    <b>let</b> <b>mut</b> total_balance = merged_coin.into_balance();
    // <b>return</b> the full amount <b>if</b> amount is not specified
    <b>if</b> (amount.is_some()) {
        <b>let</b> amount = amount.destroy_some();
        <b>let</b> <a href="../sui-framework/balance.md#0x2_balance">balance</a> = total_balance.split(amount);
        // <a href="../sui-framework/transfer.md#0x2_transfer">transfer</a> back the remainder <b>if</b> non zero.
        <b>if</b> (total_balance.value() &gt; 0) {
            <a href="../sui-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(total_balance.into_coin(ctx), ctx.sender());
        } <b>else</b> {
            total_balance.destroy_zero();
        };
        <a href="../sui-framework/balance.md#0x2_balance">balance</a>
    } <b>else</b> {
        total_balance
    }
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_authorize_update_message_by_cap"></a>

## Function `authorize_update_message_by_cap`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_authorize_update_message_by_cap">authorize_update_message_by_cap</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, cap: &<a href="protocol_cap.md#0x0_protocol_cap_ProtocolCap">protocol_cap::ProtocolCap</a>, package_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, digest: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../sui-framework/package.md#0x2_package_UpgradeTicket">package::UpgradeTicket</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_authorize_update_message_by_cap">authorize_update_message_by_cap</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    cap: &ProtocolCap,
    package_id: ID,
    digest: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
): UpgradeTicket {
    <b>let</b> protocol_cap_id = <a href="../sui-framework/object.md#0x2_object_id">object::id</a>(cap);

    <b>assert</b>!(self.authorized_protocol_cap_ids.contains(&protocol_cap_id), <a href="system_inner.md#0x0_system_inner_v1_EUnauthorizedProtocolCap">EUnauthorizedProtocolCap</a>);

    <a href="../sui-framework/event.md#0x2_event_emit">event::emit</a>(<a href="system_inner.md#0x0_system_inner_v1_SystemProtocolCapVerifiedEvent">SystemProtocolCapVerifiedEvent</a> {
        epoch: self.epoch,
        protocol_cap_id: <a href="../sui-framework/object.md#0x2_object_id">object::id</a>(cap),
    });

    self.<a href="system_inner.md#0x0_system_inner_v1_authorize_update_message">authorize_update_message</a>(package_id, digest)
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_authorize_update_message"></a>

## Function `authorize_update_message`



<pre><code><b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_authorize_update_message">authorize_update_message</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, package_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, digest: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../sui-framework/package.md#0x2_package_UpgradeTicket">package::UpgradeTicket</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_authorize_update_message">authorize_update_message</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    package_id: ID,
    digest: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
): UpgradeTicket  {
    <b>let</b> index = self.upgrade_caps.find_index!(|c| c.<a href="../sui-framework/package.md#0x2_package">package</a>() == package_id).extract();
    <b>let</b> policy = self.upgrade_caps[index].policy();
    self.upgrade_caps[index].authorize(policy, digest)
}
</code></pre>



</details>

<a name="0x0_system_inner_v1_commit_upgrade"></a>

## Function `commit_upgrade`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_commit_upgrade">commit_upgrade</a>(self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>, receipt: <a href="../sui-framework/package.md#0x2_package_UpgradeReceipt">package::UpgradeReceipt</a>): <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system_inner.md#0x0_system_inner_v1_commit_upgrade">commit_upgrade</a>(
    self: &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">SystemInnerV1</a>,
    receipt: UpgradeReceipt,
): ID {
    <b>let</b> receipt_cap_id = receipt.cap();
    <b>let</b> index = self.upgrade_caps.find_index!(|c| <a href="../sui-framework/object.md#0x2_object_id">object::id</a>(c) == receipt_cap_id).extract();
    <b>let</b> old_package_id = self.upgrade_caps[index].<a href="../sui-framework/package.md#0x2_package">package</a>();
    self.upgrade_caps[index].commit(receipt);
    old_package_id
}
</code></pre>



</details>
