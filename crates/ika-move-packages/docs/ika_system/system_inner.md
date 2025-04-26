---
title: Module `(ika_system=0x0)::system_inner`
---



-  [Struct `SystemParametersV1`](#(ika_system=0x0)_system_inner_SystemParametersV1)
-  [Struct `SystemInnerV1`](#(ika_system=0x0)_system_inner_SystemInnerV1)
-  [Struct `SystemEpochInfoEvent`](#(ika_system=0x0)_system_inner_SystemEpochInfoEvent)
-  [Struct `SystemProtocolCapVerifiedEvent`](#(ika_system=0x0)_system_inner_SystemProtocolCapVerifiedEvent)
-  [Struct `SystemCheckpointInfoEvent`](#(ika_system=0x0)_system_inner_SystemCheckpointInfoEvent)
-  [Constants](#@Constants_0)
-  [Function `create`](#(ika_system=0x0)_system_inner_create)
-  [Function `advance_network_keys`](#(ika_system=0x0)_system_inner_advance_network_keys)
-  [Function `create_system_parameters`](#(ika_system=0x0)_system_inner_create_system_parameters)
-  [Function `initialize`](#(ika_system=0x0)_system_inner_initialize)
-  [Function `request_add_validator_candidate`](#(ika_system=0x0)_system_inner_request_add_validator_candidate)
-  [Function `request_remove_validator_candidate`](#(ika_system=0x0)_system_inner_request_remove_validator_candidate)
-  [Function `request_add_validator`](#(ika_system=0x0)_system_inner_request_add_validator)
-  [Function `request_remove_validator`](#(ika_system=0x0)_system_inner_request_remove_validator)
-  [Function `set_validator_metadata`](#(ika_system=0x0)_system_inner_set_validator_metadata)
-  [Function `set_next_commission`](#(ika_system=0x0)_system_inner_set_next_commission)
-  [Function `request_add_stake`](#(ika_system=0x0)_system_inner_request_add_stake)
-  [Function `request_withdraw_stake`](#(ika_system=0x0)_system_inner_request_withdraw_stake)
-  [Function `withdraw_stake`](#(ika_system=0x0)_system_inner_withdraw_stake)
-  [Function `report_validator`](#(ika_system=0x0)_system_inner_report_validator)
-  [Function `undo_report_validator`](#(ika_system=0x0)_system_inner_undo_report_validator)
-  [Function `rotate_operation_cap`](#(ika_system=0x0)_system_inner_rotate_operation_cap)
-  [Function `rotate_commission_cap`](#(ika_system=0x0)_system_inner_rotate_commission_cap)
-  [Function `set_validator_name`](#(ika_system=0x0)_system_inner_set_validator_name)
-  [Function `set_next_epoch_network_address`](#(ika_system=0x0)_system_inner_set_next_epoch_network_address)
-  [Function `set_next_epoch_p2p_address`](#(ika_system=0x0)_system_inner_set_next_epoch_p2p_address)
-  [Function `set_next_epoch_consensus_address`](#(ika_system=0x0)_system_inner_set_next_epoch_consensus_address)
-  [Function `set_next_epoch_protocol_pubkey_bytes`](#(ika_system=0x0)_system_inner_set_next_epoch_protocol_pubkey_bytes)
-  [Function `set_next_epoch_consensus_pubkey_bytes`](#(ika_system=0x0)_system_inner_set_next_epoch_consensus_pubkey_bytes)
-  [Function `set_next_epoch_class_groups_pubkey_and_proof_bytes`](#(ika_system=0x0)_system_inner_set_next_epoch_class_groups_pubkey_and_proof_bytes)
-  [Function `set_next_epoch_network_pubkey_bytes`](#(ika_system=0x0)_system_inner_set_next_epoch_network_pubkey_bytes)
-  [Function `advance_epoch`](#(ika_system=0x0)_system_inner_advance_epoch)
-  [Function `process_mid_epoch`](#(ika_system=0x0)_system_inner_process_mid_epoch)
-  [Function `epoch`](#(ika_system=0x0)_system_inner_epoch)
-  [Function `protocol_version`](#(ika_system=0x0)_system_inner_protocol_version)
-  [Function `upgrade_caps`](#(ika_system=0x0)_system_inner_upgrade_caps)
-  [Function `epoch_start_timestamp_ms`](#(ika_system=0x0)_system_inner_epoch_start_timestamp_ms)
-  [Function `validator_stake_amount`](#(ika_system=0x0)_system_inner_validator_stake_amount)
-  [Function `get_reporters_of`](#(ika_system=0x0)_system_inner_get_reporters_of)
-  [Function `pool_exchange_rates`](#(ika_system=0x0)_system_inner_pool_exchange_rates)
-  [Function `active_committee`](#(ika_system=0x0)_system_inner_active_committee)
-  [Function `next_epoch_active_committee`](#(ika_system=0x0)_system_inner_next_epoch_active_committee)
-  [Function `verify_cap`](#(ika_system=0x0)_system_inner_verify_cap)
-  [Function `process_checkpoint_message_by_cap`](#(ika_system=0x0)_system_inner_process_checkpoint_message_by_cap)
-  [Function `process_checkpoint_message_by_quorum`](#(ika_system=0x0)_system_inner_process_checkpoint_message_by_quorum)
-  [Function `request_dwallet_network_decryption_key_dkg_by_cap`](#(ika_system=0x0)_system_inner_request_dwallet_network_decryption_key_dkg_by_cap)
-  [Function `process_checkpoint_message`](#(ika_system=0x0)_system_inner_process_checkpoint_message)
-  [Function `authorize_update_message_by_cap`](#(ika_system=0x0)_system_inner_authorize_update_message_by_cap)
-  [Function `authorize_update_message`](#(ika_system=0x0)_system_inner_authorize_update_message)
-  [Function `commit_upgrade`](#(ika_system=0x0)_system_inner_commit_upgrade)
-  [Function `epoch_duration_ms`](#(ika_system=0x0)_system_inner_epoch_duration_ms)


<pre><code><b>use</b> (ika=0x0)::ika;
<b>use</b> (ika_system=0x0)::<b>address</b>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee">bls_committee</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof">class_groups_public_key_and_proof</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1">dwallet_2pc_mpc_secp256k1</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner">dwallet_2pc_mpc_secp256k1_inner</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing">dwallet_pricing</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/extended_field.md#(ika_system=0x0)_extended_field">extended_field</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/multiaddr.md#(ika_system=0x0)_multiaddr">multiaddr</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values">pending_values</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/exchange_rate.md#(ika_system=0x0)_pool_exchange_rate">pool_exchange_rate</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap">protocol_cap</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury">protocol_treasury</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a>;
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
<b>use</b> <a href="../sui/ed25519.md#sui_ed25519">sui::ed25519</a>;
<b>use</b> <a href="../sui/event.md#sui_event">sui::event</a>;
<b>use</b> <a href="../sui/group_ops.md#sui_group_ops">sui::group_ops</a>;
<b>use</b> <a href="../sui/hash.md#sui_hash">sui::hash</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/object_table.md#sui_object_table">sui::object_table</a>;
<b>use</b> <a href="../sui/package.md#sui_package">sui::package</a>;
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



<a name="(ika_system=0x0)_system_inner_SystemParametersV1"></a>

## Struct `SystemParametersV1`

The params of the system.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemParametersV1">SystemParametersV1</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
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
<code>min_validator_count: u64</code>
</dt>
<dd>
 Minimum number of active validators at any moment.
</dd>
<dt>
<code>max_validator_count: u64</code>
</dt>
<dd>
 Maximum number of active validators at any moment.
 We do not allow the number of validators in any epoch to go above this.
</dd>
<dt>
<code>min_validator_joining_stake: u64</code>
</dt>
<dd>
 Lower-bound on the amount of stake required to become a validator.
</dd>
<dt>
<code>validator_low_stake_threshold: u64</code>
</dt>
<dd>
 Validators with stake amount below <code>validator_low_stake_threshold</code> are considered to
 have low stake and will be escorted out of the validator set after being below this
 threshold for more than <code>validator_low_stake_grace_period</code> number of epochs.
</dd>
<dt>
<code>validator_very_low_stake_threshold: u64</code>
</dt>
<dd>
 Validators with stake below <code>validator_very_low_stake_threshold</code> will be removed
 immediately at epoch change, no grace period.
</dd>
<dt>
<code>validator_low_stake_grace_period: u64</code>
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
<code>extra_fields: <a href="../sui/bag.md#sui_bag_Bag">sui::bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_system_inner_SystemInnerV1"></a>

## Struct `SystemInnerV1`

Uses SystemParametersV1 as the parameters.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a> <b>has</b> store
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
<code><a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_protocol_version">protocol_version</a>: u64</code>
</dt>
<dd>
 The current protocol version, starting from 1.
</dd>
<dt>
<code><a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_upgrade_caps">upgrade_caps</a>: vector&lt;<a href="../sui/package.md#sui_package_UpgradeCap">sui::package::UpgradeCap</a>&gt;</code>
</dt>
<dd>
 Upgrade caps for this package and others like ika coin of the ika protocol.
</dd>
<dt>
<code>validators: (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a></code>
</dt>
<dd>
 Contains all information about the validators.
</dd>
<dt>
<code>parameters: (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemParametersV1">system_inner::SystemParametersV1</a></code>
</dt>
<dd>
 A list of system config parameters.
</dd>
<dt>
<code>computation_price_per_unit_size: u64</code>
</dt>
<dd>
 The computation price per unit size for the current epoch.
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
<code>total_messages_processed: u64</code>
</dt>
<dd>
 The total messages processed.
</dd>
<dt>
<code>last_processed_checkpoint_sequence_number: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
 The last checkpoint sequence number processed.
</dd>
<dt>
<code>previous_epoch_last_checkpoint_sequence_number: u64</code>
</dt>
<dd>
 The last checkpoint sequence number of previous epoch.
</dd>
<dt>
<code>computation_reward: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;</code>
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
<code>dwallet_2pc_mpc_secp256k1_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>dwallet_2pc_mpc_secp256k1_network_decryption_keys: vector&lt;(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletNetworkDecryptionKeyCap">dwallet_2pc_mpc_secp256k1_inner::DWalletNetworkDecryptionKeyCap</a>&gt;</code>
</dt>
<dd>
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
<code>computation_price_per_unit_size: u64</code>
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
<dt>
<code>last_processed_checkpoint_sequence_number: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_system_inner_SystemProtocolCapVerifiedEvent"></a>

## Struct `SystemProtocolCapVerifiedEvent`

Event emitted during verifing quorum checkpoint submmision signature.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemProtocolCapVerifiedEvent">SystemProtocolCapVerifiedEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>protocol_cap_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_system_inner_SystemCheckpointInfoEvent"></a>

## Struct `SystemCheckpointInfoEvent`

Event containing system-level checkpoint information, emitted during
the checkpoint submmision message.


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

<a name="@Constants_0"></a>

## Constants


<a name="(ika_system=0x0)_system_inner_BASIS_POINT_DENOMINATOR"></a>



<pre><code><b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a>: u16 = 10000;
</code></pre>



<a name="(ika_system=0x0)_system_inner_CHECKPOINT_MESSAGE_INTENT"></a>



<pre><code><b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_CHECKPOINT_MESSAGE_INTENT">CHECKPOINT_MESSAGE_INTENT</a>: vector&lt;u8&gt; = vector[1, 0, 0];
</code></pre>



<a name="(ika_system=0x0)_system_inner_EActiveBlsCommitteeMustInitialize"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EActiveBlsCommitteeMustInitialize">EActiveBlsCommitteeMustInitialize</a>: vector&lt;u8&gt; = b"First active committee must <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_initialize">initialize</a>.";
</code></pre>



<a name="(ika_system=0x0)_system_inner_EBpsTooLarge"></a>



<pre><code><b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EBpsTooLarge">EBpsTooLarge</a>: u64 = 5;
</code></pre>



<a name="(ika_system=0x0)_system_inner_ECannotInitialize"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_ECannotInitialize">ECannotInitialize</a>: vector&lt;u8&gt; = b"Too early <b>for</b> initialization time or already initialized.";
</code></pre>



<a name="(ika_system=0x0)_system_inner_EHaveNotReachedMidEpochTime"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EHaveNotReachedMidEpochTime">EHaveNotReachedMidEpochTime</a>: vector&lt;u8&gt; = b"The <a href="../ika_system/system.md#(ika_system=0x0)_system">system</a> <b>has</b> not reached the mid <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a> time.";
</code></pre>



<a name="(ika_system=0x0)_system_inner_EIncorrectEpochInCheckpoint"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EIncorrectEpochInCheckpoint">EIncorrectEpochInCheckpoint</a>: vector&lt;u8&gt; = b"The checkpoint <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a> is incorrect.";
</code></pre>



<a name="(ika_system=0x0)_system_inner_ELimitExceeded"></a>



<pre><code><b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_ELimitExceeded">ELimitExceeded</a>: u64 = 1;
</code></pre>



<a name="(ika_system=0x0)_system_inner_ENextCommitteeNotSetOnAdvanceEpoch"></a>



<pre><code><b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_ENextCommitteeNotSetOnAdvanceEpoch">ENextCommitteeNotSetOnAdvanceEpoch</a>: u64 = 6;
</code></pre>



<a name="(ika_system=0x0)_system_inner_EUnauthorizedProtocolCap"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EUnauthorizedProtocolCap">EUnauthorizedProtocolCap</a>: vector&lt;u8&gt; = b"The protocol cap is unauthorized.";
</code></pre>



<a name="(ika_system=0x0)_system_inner_EWrongCheckpointSequenceNumber"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EWrongCheckpointSequenceNumber">EWrongCheckpointSequenceNumber</a>: vector&lt;u8&gt; = b"The checkpoint sequence number should be the expected next one.";
</code></pre>



<a name="(ika_system=0x0)_system_inner_EWrongEpochState"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EWrongEpochState">EWrongEpochState</a>: vector&lt;u8&gt; = b"The <a href="../ika_system/system.md#(ika_system=0x0)_system">system</a> is in the wrong <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a> state <b>for</b> the operation.";
</code></pre>



<a name="(ika_system=0x0)_system_inner_create"></a>

## Function `create`

Create a new IkaSystemState object and make it shared.
This function will be called only once in init.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_create">create</a>(<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_upgrade_caps">upgrade_caps</a>: vector&lt;<a href="../sui/package.md#sui_package_UpgradeCap">sui::package::UpgradeCap</a>&gt;, validators: (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_protocol_version">protocol_version</a>: u64, <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_start_timestamp_ms">epoch_start_timestamp_ms</a>: u64, parameters: (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemParametersV1">system_inner::SystemParametersV1</a>, <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury">protocol_treasury</a>: (ika_system=0x0)::<a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_ProtocolTreasury">protocol_treasury::ProtocolTreasury</a>, authorized_protocol_cap_ids: vector&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_create">create</a>(
    <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_upgrade_caps">upgrade_caps</a>: vector&lt;UpgradeCap&gt;,
    validators: ValidatorSet,
    <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_protocol_version">protocol_version</a>: u64,
    <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_start_timestamp_ms">epoch_start_timestamp_ms</a>: u64,
    parameters: <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemParametersV1">SystemParametersV1</a>,
    <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury">protocol_treasury</a>: ProtocolTreasury,
    authorized_protocol_cap_ids: vector&lt;ID&gt;,
    ctx: &<b>mut</b> TxContext,
): <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a> {
    // This type is fixed <b>as</b> it's created at <a href="../ika_system/init.md#(ika_system=0x0)_init">init</a>. It should not be updated during type upgrade.
    <b>let</b> system_state = <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a> {
        <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: 0,
        <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_protocol_version">protocol_version</a>,
        <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_upgrade_caps">upgrade_caps</a>,
        validators,
        parameters,
        computation_price_per_unit_size: 0,
        <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury">protocol_treasury</a>,
        <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_start_timestamp_ms">epoch_start_timestamp_ms</a>,
        total_messages_processed: 0,
        last_processed_checkpoint_sequence_number: option::none(),
        previous_epoch_last_checkpoint_sequence_number: 0,
        computation_reward: balance::zero(),
        authorized_protocol_cap_ids,
        dwallet_2pc_mpc_secp256k1_id: option::none(),
        dwallet_2pc_mpc_secp256k1_network_decryption_keys: vector[],
        extra_fields: bag::new(ctx),
    };
    system_state
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_advance_network_keys"></a>

## Function `advance_network_keys`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_advance_network_keys">advance_network_keys</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1">dwallet_2pc_mpc_secp256k1</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_advance_network_keys">advance_network_keys</a>(
    self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>, <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1">dwallet_2pc_mpc_secp256k1</a>: &<b>mut</b> DWalletCoordinatorInner
): Balance&lt;IKA&gt; {
    <b>let</b> <b>mut</b> total_reward = <a href="../sui/balance.md#sui_balance_zero">sui::balance::zero</a>&lt;IKA&gt;();
    self.dwallet_2pc_mpc_secp256k1_network_decryption_keys.do_ref!(|cap| {
        total_reward.join(<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1">dwallet_2pc_mpc_secp256k1</a>.advance_epoch_dwallet_network_decryption_key(cap));
    });
    total_reward
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_create_system_parameters"></a>

## Function `create_system_parameters`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_create_system_parameters">create_system_parameters</a>(<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_duration_ms">epoch_duration_ms</a>: u64, stake_subsidy_start_epoch: u64, min_validator_count: u64, max_validator_count: u64, min_validator_joining_stake: u64, validator_low_stake_threshold: u64, validator_very_low_stake_threshold: u64, validator_low_stake_grace_period: u64, reward_slashing_rate: u16, lock_active_committee: bool, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemParametersV1">system_inner::SystemParametersV1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_create_system_parameters">create_system_parameters</a>(
    <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_duration_ms">epoch_duration_ms</a>: u64,
    stake_subsidy_start_epoch: u64,
    // Validator committee parameters
    min_validator_count: u64,
    max_validator_count: u64,
    min_validator_joining_stake: u64,
    validator_low_stake_threshold: u64,
    validator_very_low_stake_threshold: u64,
    validator_low_stake_grace_period: u64,
    reward_slashing_rate: u16,
    lock_active_committee: bool,
    ctx: &<b>mut</b> TxContext,
): <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemParametersV1">SystemParametersV1</a> {
    // Rates can't be higher than 100%.
    <b>assert</b>!(
        reward_slashing_rate &lt;= <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a>,
        <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EBpsTooLarge">EBpsTooLarge</a>,
    );
    <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemParametersV1">SystemParametersV1</a> {
        <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_duration_ms">epoch_duration_ms</a>,
        stake_subsidy_start_epoch,
        min_validator_count,
        max_validator_count,
        min_validator_joining_stake,
        validator_low_stake_threshold,
        validator_very_low_stake_threshold,
        validator_low_stake_grace_period,
        reward_slashing_rate,
        lock_active_committee,
        extra_fields: bag::new(ctx),
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_initialize"></a>

## Function `initialize`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_initialize">initialize</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>, package_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_initialize">initialize</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    clock: &Clock,
    package_id: ID,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> now = clock.timestamp_ms();
    <b>assert</b>!(self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a> == 0 && now &gt;= self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_start_timestamp_ms">epoch_start_timestamp_ms</a>, <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_ECannotInitialize">ECannotInitialize</a>);
    <b>assert</b>!(self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_active_committee">active_committee</a>().members().is_empty(), <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_ECannotInitialize">ECannotInitialize</a>);
    // self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a> = self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a> + 1;
    // self.validators.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_initialize">initialize</a>();
    self.validators.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_process_mid_epoch">process_mid_epoch</a>(
        self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>,
        self.parameters.lock_active_committee,
        self.parameters.validator_low_stake_threshold,
        self.parameters.validator_very_low_stake_threshold,
        self.parameters.validator_low_stake_grace_period,
    );
    <b>let</b> pricing = <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_create_dwallet_pricing_2pc_mpc_secp256k1">ika_system::dwallet_pricing::create_dwallet_pricing_2pc_mpc_secp256k1</a>(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ctx);
    <b>let</b> <b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1">dwallet_2pc_mpc_secp256k1</a> = <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_create_dwallet_coordinator">dwallet_2pc_mpc_secp256k1::create_dwallet_coordinator</a>(package_id, self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>, self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_active_committee">active_committee</a>(), pricing, ctx);
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner">dwallet_2pc_mpc_secp256k1_inner</a> = <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1">dwallet_2pc_mpc_secp256k1</a>.inner_mut();
    <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner">dwallet_2pc_mpc_secp256k1_inner</a>.lock_last_active_session_sequence_number();
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_advance_epoch">advance_epoch</a>(<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner">dwallet_2pc_mpc_secp256k1_inner</a>, clock, ctx);
    self.dwallet_2pc_mpc_secp256k1_id.fill(object::id(&<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1">dwallet_2pc_mpc_secp256k1</a>));
    <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1">dwallet_2pc_mpc_secp256k1</a>.share_dwallet_coordinator();
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_request_add_validator_candidate"></a>

## Function `request_add_validator_candidate`

Can be called by anyone who wishes to become a validator candidate and starts accuring delegated
stakes in their staking pool. Once they have at least <code>MIN_VALIDATOR_JOINING_STAKE</code> amount of stake they
can call <code><a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_add_validator">request_add_validator</a></code> to officially become an active validator at the next epoch.
Aborts if the caller is already a pending or active validator, or a validator candidate.
Note: <code>proof_of_possession_bytes</code> MUST be a valid signature using proof_of_possession_sender and protocol_pubkey_bytes.
To produce a valid PoP, run [fn test_proof_of_possession_bytes].


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_add_validator_candidate">request_add_validator_candidate</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, name: <a href="../std/string.md#std_string_String">std::string::String</a>, protocol_pubkey_bytes: vector&lt;u8&gt;, network_pubkey_bytes: vector&lt;u8&gt;, consensus_pubkey_bytes: vector&lt;u8&gt;, class_groups_pubkey_and_proof_bytes: (ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof</a>, proof_of_possession_bytes: vector&lt;u8&gt;, network_address: <a href="../std/string.md#std_string_String">std::string::String</a>, p2p_address: <a href="../std/string.md#std_string_String">std::string::String</a>, consensus_address: <a href="../std/string.md#std_string_String">std::string::String</a>, commission_rate: u16, metadata: (ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">validator_metadata::ValidatorMetadata</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): ((ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCommissionCap">validator_cap::ValidatorCommissionCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_add_validator_candidate">request_add_validator_candidate</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
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
    self.validators.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_add_validator_candidate">request_add_validator_candidate</a>(
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


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_remove_validator_candidate">request_remove_validator_candidate</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_remove_validator_candidate">request_remove_validator_candidate</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    cap: &ValidatorCap,
) {
    self.validators.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_remove_validator_candidate">request_remove_validator_candidate</a>(self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>, cap);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_request_add_validator"></a>

## Function `request_add_validator`

Called by a validator candidate to add themselves to the active validator set beginning next epoch.
Aborts if the validator is a duplicate with one of the pending or active validators, or if the amount of
stake the validator has doesn't meet the min threshold, or if the number of new validators for the next
epoch has already reached the maximum.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_add_validator">request_add_validator</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_add_validator">request_add_validator</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    cap: &ValidatorCap,
) {
    <b>assert</b>!(
        self.validators.pending_active_validators_count() &lt; self.parameters.max_validator_count,
        <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_ELimitExceeded">ELimitExceeded</a>,
    );
    self.validators.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_add_validator">request_add_validator</a>(self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>, cap, self.parameters.min_validator_joining_stake);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_request_remove_validator"></a>

## Function `request_remove_validator`

A validator can call this function to request a removal in the next epoch.
We use the sender of <code>ctx</code> to look up the validator
(i.e. sender must match the sui_address in the validator).
At the end of the epoch, the <code>validator</code> object will be returned to the sui_address
of the validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_remove_validator">request_remove_validator</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_remove_validator">request_remove_validator</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    cap: &ValidatorCap,
) {
    // Only check min validator condition <b>if</b> the current number of validators satisfy the constraint.
    // This is so that <b>if</b> we somehow already are in a state where we have less than min validators, it no longer matters
    // and is ok to stay so. This is useful <b>for</b> a test setup.
    <b>if</b> (self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_active_committee">active_committee</a>().members().length() &gt;= self.parameters.min_validator_count) {
        <b>assert</b>!(
            self.validators.pending_active_validators_count() &gt; self.parameters.min_validator_count,
            <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_ELimitExceeded">ELimitExceeded</a>,
        );
    };
    self.validators.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_remove_validator">request_remove_validator</a>(self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>, cap);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_set_validator_metadata"></a>

## Function `set_validator_metadata`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_validator_metadata">set_validator_metadata</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, metadata: (ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">validator_metadata::ValidatorMetadata</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_validator_metadata">set_validator_metadata</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    cap: &ValidatorOperationCap,
    metadata: ValidatorMetadata,
) {
    self.validators.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_validator_metadata">set_validator_metadata</a>(cap, metadata);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_set_next_commission"></a>

## Function `set_next_commission`

A validator can call this function to set a new commission rate, updated at the end of
the epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_commission">set_next_commission</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, new_commission_rate: u16, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_commission">set_next_commission</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    new_commission_rate: u16,
    cap: &ValidatorOperationCap,
) {
    self
        .validators
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


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_add_stake">request_add_stake</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, stake: <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_add_stake">request_add_stake</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    stake: Coin&lt;IKA&gt;,
    validator_id: ID,
    ctx: &<b>mut</b> TxContext,
): StakedIka {
    self
        .validators
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


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_withdraw_stake">request_withdraw_stake</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_withdraw_stake">request_withdraw_stake</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: &<b>mut</b> StakedIka,
) {
    self.validators.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_withdraw_stake">request_withdraw_stake</a>(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>, self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_withdraw_stake"></a>

## Function `withdraw_stake`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_withdraw_stake">withdraw_stake</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_withdraw_stake">withdraw_stake</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: StakedIka,
    ctx: &<b>mut</b> TxContext,
): Coin&lt;IKA&gt; {
    self.validators.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_withdraw_stake">withdraw_stake</a>(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>, self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>, ctx)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_report_validator"></a>

## Function `report_validator`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_report_validator">report_validator</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, reportee_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_report_validator">report_validator</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    cap: &ValidatorOperationCap,
    reportee_id: ID,
) {
    self.validators.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_report_validator">report_validator</a>(cap, reportee_id);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_undo_report_validator"></a>

## Function `undo_report_validator`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_undo_report_validator">undo_report_validator</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, reportee_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_undo_report_validator">undo_report_validator</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    cap: &ValidatorOperationCap,
    reportee_id: ID,
) {
    self.validators.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_undo_report_validator">undo_report_validator</a>(cap, reportee_id);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_rotate_operation_cap"></a>

## Function `rotate_operation_cap`

Create a new <code>ValidatorOperationCap</code> and registers it.
The original object is thus revoked.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_rotate_operation_cap">rotate_operation_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_rotate_operation_cap">rotate_operation_cap</a>(self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>, cap: &ValidatorCap, ctx: &<b>mut</b> TxContext): ValidatorOperationCap {
    <b>let</b> validator = self.validators.get_validator_mut_with_cap_including_candidates(cap);
    validator.new_validator_operation_cap(cap, ctx)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_rotate_commission_cap"></a>

## Function `rotate_commission_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_rotate_commission_cap">rotate_commission_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCommissionCap">validator_cap::ValidatorCommissionCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_rotate_commission_cap">rotate_commission_cap</a>(self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>, cap: &ValidatorCap, ctx: &<b>mut</b> TxContext): ValidatorCommissionCap {
    <b>let</b> validator = self.validators.get_validator_mut_with_cap_including_candidates(cap);
    validator.new_validator_commission_cap(cap, ctx)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_set_validator_name"></a>

## Function `set_validator_name`

Sets a validator's name.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_validator_name">set_validator_name</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, name: <a href="../std/string.md#std_string_String">std::string::String</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_validator_name">set_validator_name</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    name: String,
    cap: &ValidatorOperationCap
) {
    self.validators.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_validator_name">set_validator_name</a>(name, cap);}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_set_next_epoch_network_address"></a>

## Function `set_next_epoch_network_address`

Sets a validator's network address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_network_address">set_next_epoch_network_address</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, network_address: <a href="../std/string.md#std_string_String">std::string::String</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_network_address">set_next_epoch_network_address</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    network_address: String,
    cap: &ValidatorOperationCap
) {
    self.validators.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_network_address">set_next_epoch_network_address</a>(network_address, cap);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_set_next_epoch_p2p_address"></a>

## Function `set_next_epoch_p2p_address`

Sets a validator's p2p address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_p2p_address">set_next_epoch_p2p_address</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, p2p_address: <a href="../std/string.md#std_string_String">std::string::String</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_p2p_address">set_next_epoch_p2p_address</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    p2p_address: String,
    cap: &ValidatorOperationCap
) {
    self.validators.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_p2p_address">set_next_epoch_p2p_address</a>(p2p_address, cap);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_set_next_epoch_consensus_address"></a>

## Function `set_next_epoch_consensus_address`

Sets a validator's consensus address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_consensus_address">set_next_epoch_consensus_address</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, consensus_address: <a href="../std/string.md#std_string_String">std::string::String</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_consensus_address">set_next_epoch_consensus_address</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    consensus_address: String,
    cap: &ValidatorOperationCap
) {
    self.validators.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_consensus_address">set_next_epoch_consensus_address</a>(consensus_address, cap);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_set_next_epoch_protocol_pubkey_bytes"></a>

## Function `set_next_epoch_protocol_pubkey_bytes`

Sets a validator's public key of protocol key and proof of possession.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_protocol_pubkey_bytes">set_next_epoch_protocol_pubkey_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, protocol_pubkey_bytes: vector&lt;u8&gt;, proof_of_possession_bytes: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, ctx: &<a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_protocol_pubkey_bytes">set_next_epoch_protocol_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    protocol_pubkey_bytes: vector&lt;u8&gt;,
    proof_of_possession_bytes: vector&lt;u8&gt;,
    cap: &ValidatorOperationCap,
    ctx: &TxContext,
) {
    self.validators.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_protocol_pubkey_bytes">set_next_epoch_protocol_pubkey_bytes</a>(protocol_pubkey_bytes, proof_of_possession_bytes, cap, ctx);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_set_next_epoch_consensus_pubkey_bytes"></a>

## Function `set_next_epoch_consensus_pubkey_bytes`

Sets a validator's public key of worker key.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_consensus_pubkey_bytes">set_next_epoch_consensus_pubkey_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, consensus_pubkey_bytes: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_consensus_pubkey_bytes">set_next_epoch_consensus_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    consensus_pubkey_bytes: vector&lt;u8&gt;,
    cap: &ValidatorOperationCap
) {
    self.validators.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_consensus_pubkey_bytes">set_next_epoch_consensus_pubkey_bytes</a>(consensus_pubkey_bytes, cap);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_set_next_epoch_class_groups_pubkey_and_proof_bytes"></a>

## Function `set_next_epoch_class_groups_pubkey_and_proof_bytes`

Sets a validator's public key and its associated proof of class groups key.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_class_groups_pubkey_and_proof_bytes">set_next_epoch_class_groups_pubkey_and_proof_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, class_groups_pubkey_and_proof_bytes: (ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_class_groups_pubkey_and_proof_bytes">set_next_epoch_class_groups_pubkey_and_proof_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    class_groups_pubkey_and_proof_bytes: ClassGroupsPublicKeyAndProof,
    cap: &ValidatorOperationCap
) {
    self.validators.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_class_groups_pubkey_and_proof_bytes">set_next_epoch_class_groups_pubkey_and_proof_bytes</a>(class_groups_pubkey_and_proof_bytes, cap);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_set_next_epoch_network_pubkey_bytes"></a>

## Function `set_next_epoch_network_pubkey_bytes`

Sets a validator's public key of network key.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_network_pubkey_bytes">set_next_epoch_network_pubkey_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, network_pubkey_bytes: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_network_pubkey_bytes">set_next_epoch_network_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    network_pubkey_bytes: vector&lt;u8&gt;,
    cap: &ValidatorOperationCap
) {
    self.validators.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_set_next_epoch_network_pubkey_bytes">set_next_epoch_network_pubkey_bytes</a>(network_pubkey_bytes, cap);
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


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_advance_epoch">advance_epoch</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, dwallet_coordinator: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_advance_epoch">advance_epoch</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    dwallet_coordinator: &<b>mut</b> DWalletCoordinatorInner,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> now = clock.timestamp_ms();
    <b>let</b> last_epoch_change = self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_start_timestamp_ms">epoch_start_timestamp_ms</a>;
    <b>if</b> (self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a> == 0) <b>assert</b>!(now &gt;= last_epoch_change, <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EWrongEpochState">EWrongEpochState</a>)
    <b>else</b> <b>assert</b>!(now &gt;= last_epoch_change + self.parameters.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_duration_ms">epoch_duration_ms</a>, <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EWrongEpochState">EWrongEpochState</a>);
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_start_timestamp_ms">epoch_start_timestamp_ms</a> = now;
    <b>let</b> <b>mut</b> stake_subsidy = balance::zero();
    // during the transition from <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a> N to <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a> N + 1, self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>() will <b>return</b> N
    <b>let</b> current_epoch = self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>();
    // Include stake subsidy in the rewards given out to validators and stakers.
    // Delay distributing any stake subsidies until after `stake_subsidy_start_epoch`.
    // And <b>if</b> this <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a> is shorter than the regular <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a> duration, don't distribute any stake subsidy.
    <b>if</b> (
        current_epoch &gt;= self.parameters.stake_subsidy_start_epoch
    ) {
        stake_subsidy.join(self.<a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury">protocol_treasury</a>.stake_subsidy_for_distribution(ctx));
    };
    <b>let</b> computation_reward_amount_before_distribution = self.computation_reward.value();
    <b>let</b> epoch_computation_reward = dwallet_coordinator.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_advance_epoch">advance_epoch</a>(self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_next_epoch_active_committee">next_epoch_active_committee</a>());
    <b>let</b> decryption_keys_rewards = self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_advance_network_keys">advance_network_keys</a>(dwallet_coordinator);
    <b>let</b> stake_subsidy_amount = stake_subsidy.value();
    <b>let</b> <b>mut</b> total_reward = <a href="../sui/balance.md#sui_balance_zero">sui::balance::zero</a>&lt;IKA&gt;();
    total_reward.join(epoch_computation_reward);
    total_reward.join(self.computation_reward.withdraw_all());
    total_reward.join(stake_subsidy);
    self.computation_reward.join(decryption_keys_rewards);
    <b>let</b> total_reward_amount_before_distribution = total_reward.value();
    <b>let</b> new_epoch = current_epoch + 1;
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a> = new_epoch;
    self
        .validators
        .<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_advance_epoch">advance_epoch</a>(
            current_epoch,
            new_epoch,
            &<b>mut</b> total_reward,
            self.parameters.reward_slashing_rate,
        );
    <b>let</b> new_total_stake = self.validators.total_stake();
    <b>let</b> total_reward_amount_after_distribution = total_reward.value();
    <b>let</b> total_reward_distributed =
         total_reward_amount_before_distribution - total_reward_amount_after_distribution;
    // Because of precision issues with integer divisions, we expect that there will be some
    // remaining balance in `computation_reward`.
    self.computation_reward.join(total_reward);
    //<b>let</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_active_committee">active_committee</a> = self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_active_committee">active_committee</a>();
    // // Derive the computation price per unit size <b>for</b> the new <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>
    //self.computation_price_per_unit_size = self.validators.derive_computation_price_per_unit_size(&<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_active_committee">active_committee</a>);
    <b>let</b> <b>mut</b> last_processed_checkpoint_sequence_number = 0;
    <b>if</b> (self.last_processed_checkpoint_sequence_number.is_some()) {
        last_processed_checkpoint_sequence_number = *self.last_processed_checkpoint_sequence_number.borrow();
        self.previous_epoch_last_checkpoint_sequence_number = last_processed_checkpoint_sequence_number;
    };
    event::emit(<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemEpochInfoEvent">SystemEpochInfoEvent</a> {
        <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>,
        <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_protocol_version">protocol_version</a>: self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_protocol_version">protocol_version</a>,
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

<a name="(ika_system=0x0)_system_inner_process_mid_epoch"></a>

## Function `process_mid_epoch`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_process_mid_epoch">process_mid_epoch</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>, dwallet_coordinator_inner: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_process_mid_epoch">process_mid_epoch</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    clock: &Clock,
    dwallet_coordinator_inner: &<b>mut</b> DWalletCoordinatorInner,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(self.validators.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_next_epoch_active_committee">next_epoch_active_committee</a>().is_none() && clock.timestamp_ms() &gt; self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_start_timestamp_ms">epoch_start_timestamp_ms</a> + (self.parameters.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_duration_ms">epoch_duration_ms</a> / 2), <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EHaveNotReachedMidEpochTime">EHaveNotReachedMidEpochTime</a>);
    self.validators.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_process_mid_epoch">process_mid_epoch</a>(
        self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>,
        self.parameters.lock_active_committee,
        self.parameters.validator_low_stake_threshold,
        self.parameters.validator_very_low_stake_threshold,
        self.parameters.validator_low_stake_grace_period,
    );
    self.dwallet_2pc_mpc_secp256k1_network_decryption_keys.do_ref!(|cap| dwallet_coordinator_inner.emit_start_reshare_event(cap, ctx));
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_epoch"></a>

## Function `epoch`

Return the current epoch number. Useful for applications that need a coarse-grained concept of time,
since epochs are ever-increasing and epoch changes are intended to happen every 24 hours.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>(self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>): u64 {
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_protocol_version"></a>

## Function `protocol_version`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_protocol_version">protocol_version</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_protocol_version">protocol_version</a>(self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>): u64 {
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_protocol_version">protocol_version</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_upgrade_caps"></a>

## Function `upgrade_caps`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_upgrade_caps">upgrade_caps</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>): &vector&lt;<a href="../sui/package.md#sui_package_UpgradeCap">sui::package::UpgradeCap</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_upgrade_caps">upgrade_caps</a>(self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>): &vector&lt;UpgradeCap&gt; {
    &self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_upgrade_caps">upgrade_caps</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_epoch_start_timestamp_ms"></a>

## Function `epoch_start_timestamp_ms`

Returns unix timestamp of the start of current epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_start_timestamp_ms">epoch_start_timestamp_ms</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_start_timestamp_ms">epoch_start_timestamp_ms</a>(self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>): u64 {
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_start_timestamp_ms">epoch_start_timestamp_ms</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_validator_stake_amount"></a>

## Function `validator_stake_amount`

Returns the total amount staked with <code>validator_id</code>.
Aborts if <code>validator_id</code> is not an active validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_validator_stake_amount">validator_stake_amount</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_validator_stake_amount">validator_stake_amount</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    validator_id: ID,
): u64 {
    self.validators.validator_total_stake_amount(validator_id)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_get_reporters_of"></a>

## Function `get_reporters_of`

Returns all the validators who are currently reporting <code>validator_id</code>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_get_reporters_of">get_reporters_of</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): <a href="../sui/vec_set.md#sui_vec_set_VecSet">sui::vec_set::VecSet</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_get_reporters_of">get_reporters_of</a>(self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>, validator_id: ID): VecSet&lt;ID&gt; {
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_get_reporters_of">get_reporters_of</a>(validator_id)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_pool_exchange_rates"></a>

## Function `pool_exchange_rates`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_pool_exchange_rates">pool_exchange_rates</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): &<a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, (ika_system=0x0)::<a href="../ika_system/exchange_rate.md#(ika_system=0x0)_pool_exchange_rate_PoolExchangeRate">pool_exchange_rate::PoolExchangeRate</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_pool_exchange_rates">pool_exchange_rates</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    validator_id: ID,
): &Table&lt;u64, PoolExchangeRate&gt; {
    <b>let</b> validators = &<b>mut</b> self.validators;
    validators.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_pool_exchange_rates">pool_exchange_rates</a>(validator_id)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_active_committee"></a>

## Function `active_committee`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_active_committee">active_committee</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>): (ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">bls_committee::BlsCommittee</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_active_committee">active_committee</a>(self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>): BlsCommittee {
    <b>let</b> <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a> = &self.validators;
    <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set">validator_set</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_active_committee">active_committee</a>()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_next_epoch_active_committee"></a>

## Function `next_epoch_active_committee`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_next_epoch_active_committee">next_epoch_active_committee</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>): (ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">bls_committee::BlsCommittee</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_next_epoch_active_committee">next_epoch_active_committee</a>(self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>): BlsCommittee {
    <b>let</b> next_epoch_committee = self.validators.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_next_epoch_active_committee">next_epoch_active_committee</a>();
    <b>assert</b>!(next_epoch_committee.is_some(), <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_ENextCommitteeNotSetOnAdvanceEpoch">ENextCommitteeNotSetOnAdvanceEpoch</a>);
    <b>return</b> *next_epoch_committee.borrow()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_verify_cap"></a>

## Function `verify_cap`



<pre><code><b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_verify_cap">verify_cap</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, cap: &(ika_system=0x0)::<a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_ProtocolCap">protocol_cap::ProtocolCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_verify_cap">verify_cap</a>(
    self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    cap: &ProtocolCap,
) {
    <b>let</b> protocol_cap_id = object::id(cap);
    <b>assert</b>!(self.authorized_protocol_cap_ids.contains(&protocol_cap_id), <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EUnauthorizedProtocolCap">EUnauthorizedProtocolCap</a>);
    event::emit(<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemProtocolCapVerifiedEvent">SystemProtocolCapVerifiedEvent</a> {
        <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>,
        protocol_cap_id: object::id(cap),
    });
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_process_checkpoint_message_by_cap"></a>

## Function `process_checkpoint_message_by_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_process_checkpoint_message_by_cap">process_checkpoint_message_by_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, cap: &(ika_system=0x0)::<a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_ProtocolCap">protocol_cap::ProtocolCap</a>, message: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_process_checkpoint_message_by_cap">process_checkpoint_message_by_cap</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    cap: &ProtocolCap,
    message: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_verify_cap">verify_cap</a>(cap);
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_process_checkpoint_message">process_checkpoint_message</a>(message, ctx);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_process_checkpoint_message_by_quorum"></a>

## Function `process_checkpoint_message_by_quorum`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1">dwallet_2pc_mpc_secp256k1</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWalletCoordinator">dwallet_2pc_mpc_secp256k1::DWalletCoordinator</a>, signature: vector&lt;u8&gt;, signers_bitmap: vector&lt;u8&gt;, message: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1">dwallet_2pc_mpc_secp256k1</a>: &<b>mut</b> DWalletCoordinator,
    signature: vector&lt;u8&gt;,
    signers_bitmap: vector&lt;u8&gt;,
    message: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a> = self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>;
    <b>let</b> <b>mut</b> intent_bytes = <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_CHECKPOINT_MESSAGE_INTENT">CHECKPOINT_MESSAGE_INTENT</a>;
    intent_bytes.append(message);
    intent_bytes.append(bcs::to_bytes(&<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>));
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_active_committee">active_committee</a>().verify_certificate(<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>, &signature, &signers_bitmap, &intent_bytes);
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_process_checkpoint_message">process_checkpoint_message</a>(message, ctx);
    // TODO: seperate this to its own process
    <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1">dwallet_2pc_mpc_secp256k1</a>.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(signature, signers_bitmap, message, ctx);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_request_dwallet_network_decryption_key_dkg_by_cap"></a>

## Function `request_dwallet_network_decryption_key_dkg_by_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_dwallet_network_decryption_key_dkg_by_cap">request_dwallet_network_decryption_key_dkg_by_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1">dwallet_2pc_mpc_secp256k1</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWalletCoordinator">dwallet_2pc_mpc_secp256k1::DWalletCoordinator</a>, cap: &(ika_system=0x0)::<a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_ProtocolCap">protocol_cap::ProtocolCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_request_dwallet_network_decryption_key_dkg_by_cap">request_dwallet_network_decryption_key_dkg_by_cap</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1">dwallet_2pc_mpc_secp256k1</a>: &<b>mut</b> DWalletCoordinator,
    cap: &ProtocolCap,
    ctx: &<b>mut</b> TxContext,
) {
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_verify_cap">verify_cap</a>(cap);
    <b>let</b> key_cap = <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1">dwallet_2pc_mpc_secp256k1</a>.request_dwallet_network_decryption_key_dkg(ctx);
    self.dwallet_2pc_mpc_secp256k1_network_decryption_keys.push_back(key_cap);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_process_checkpoint_message"></a>

## Function `process_checkpoint_message`



<pre><code><b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_process_checkpoint_message">process_checkpoint_message</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, message: vector&lt;u8&gt;, _ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_process_checkpoint_message">process_checkpoint_message</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    message: vector&lt;u8&gt;,
    _ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(!self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_active_committee">active_committee</a>().members().is_empty(), <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EActiveBlsCommitteeMustInitialize">EActiveBlsCommitteeMustInitialize</a>);
    // first <b>let</b>'s make sure it's the correct checkpoint message
    <b>let</b> <b>mut</b> bcs_body = bcs::new(<b>copy</b> message);
    <b>let</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a> = bcs_body.peel_u64();
    <b>assert</b>!(<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a> == self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>, <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EIncorrectEpochInCheckpoint">EIncorrectEpochInCheckpoint</a>);
    <b>let</b> sequence_number = bcs_body.peel_u64();
    <b>if</b>(self.last_processed_checkpoint_sequence_number.is_none()) {
        <b>assert</b>!(sequence_number == 0, <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EWrongCheckpointSequenceNumber">EWrongCheckpointSequenceNumber</a>);
        self.last_processed_checkpoint_sequence_number.fill(sequence_number);
    } <b>else</b> {
        <b>assert</b>!(sequence_number &gt; 0 && *self.last_processed_checkpoint_sequence_number.borrow() + 1 == sequence_number, <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EWrongCheckpointSequenceNumber">EWrongCheckpointSequenceNumber</a>);
        self.last_processed_checkpoint_sequence_number.swap(sequence_number);
    };
    //<b>let</b> network_total_messages = bcs_body.peel_u64();
    //<b>let</b> previous_digest = bcs_body.peel_option!(|previous_digest| previous_digest.peel_vec_u8() );
    <b>let</b> timestamp_ms = bcs_body.peel_u64();
    event::emit(<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemCheckpointInfoEvent">SystemCheckpointInfoEvent</a> {
        <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>,
        sequence_number,
        timestamp_ms,
    });
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_authorize_update_message_by_cap"></a>

## Function `authorize_update_message_by_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_authorize_update_message_by_cap">authorize_update_message_by_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, cap: &(ika_system=0x0)::<a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_ProtocolCap">protocol_cap::ProtocolCap</a>, package_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, digest: vector&lt;u8&gt;): <a href="../sui/package.md#sui_package_UpgradeTicket">sui::package::UpgradeTicket</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_authorize_update_message_by_cap">authorize_update_message_by_cap</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    cap: &ProtocolCap,
    package_id: ID,
    digest: vector&lt;u8&gt;,
): UpgradeTicket {
    <b>let</b> protocol_cap_id = object::id(cap);
    <b>assert</b>!(self.authorized_protocol_cap_ids.contains(&protocol_cap_id), <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_EUnauthorizedProtocolCap">EUnauthorizedProtocolCap</a>);
    event::emit(<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemProtocolCapVerifiedEvent">SystemProtocolCapVerifiedEvent</a> {
        <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>: self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch">epoch</a>,
        protocol_cap_id: object::id(cap),
    });
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_authorize_update_message">authorize_update_message</a>(package_id, digest)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_authorize_update_message"></a>

## Function `authorize_update_message`



<pre><code><b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_authorize_update_message">authorize_update_message</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, package_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, digest: vector&lt;u8&gt;): <a href="../sui/package.md#sui_package_UpgradeTicket">sui::package::UpgradeTicket</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_authorize_update_message">authorize_update_message</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    package_id: ID,
    digest: vector&lt;u8&gt;,
): UpgradeTicket  {
    <b>let</b> index = self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_upgrade_caps">upgrade_caps</a>.find_index!(|c| c.package() == package_id).extract();
    <b>let</b> policy = self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_upgrade_caps">upgrade_caps</a>[index].policy();
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_upgrade_caps">upgrade_caps</a>[index].authorize(policy, digest)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_commit_upgrade"></a>

## Function `commit_upgrade`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_commit_upgrade">commit_upgrade</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>, receipt: <a href="../sui/package.md#sui_package_UpgradeReceipt">sui::package::UpgradeReceipt</a>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_commit_upgrade">commit_upgrade</a>(
    self: &<b>mut</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>,
    receipt: UpgradeReceipt,
): ID {
    <b>let</b> receipt_cap_id = receipt.cap();
    <b>let</b> index = self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_upgrade_caps">upgrade_caps</a>.find_index!(|c| object::id(c) == receipt_cap_id).extract();
    <b>let</b> old_package_id = self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_upgrade_caps">upgrade_caps</a>[index].package();
    self.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_upgrade_caps">upgrade_caps</a>[index].commit(receipt);
    old_package_id
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_epoch_duration_ms"></a>

## Function `epoch_duration_ms`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_duration_ms">epoch_duration_ms</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">system_inner::SystemInnerV1</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_duration_ms">epoch_duration_ms</a>(self: &<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInnerV1">SystemInnerV1</a>): u64 {
    self.parameters.<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_epoch_duration_ms">epoch_duration_ms</a>
}
</code></pre>



</details>
