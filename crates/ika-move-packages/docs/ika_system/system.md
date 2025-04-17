---
title: Module `(ika_system=0x0)::system`
---

Ika System State Type Upgrade Guide
<code><a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a></code> is a thin wrapper around <code>SystemInnerVX</code> that provides a versioned interface.
The <code><a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a></code> object has a fixed ID 0x5, and the <code>SystemInnerVX</code> object is stored as a dynamic field.
There are a few different ways to upgrade the <code>SystemInnerVX</code> type:

The simplest and one that doesn't involve a real upgrade is to just add dynamic fields to the <code>extra_fields</code> field
of <code>SystemInnerVX</code> or any of its sub type. This is useful when we are in a rush, or making a small change,
or still experimenting a new field.

To properly upgrade the <code>SystemInnerVX</code> type, we need to ship a new framework that does the following:
1. Define a new <code>SystemInnerVX</code>type (e.g. <code>SystemInnerV1</code>).
2. Define a data migration function that migrates the old <code>SystemInnerVX</code> to the new one (i.e. SystemInnerV1).
3. Replace all uses of <code>SystemInnerVX</code> with <code>SystemInnerV1</code> in both ika_system.move and system_inner_v1.move,
with the exception of the <code><a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_v1_create">system_inner_v1::create</a></code> function, which should always return the init type.
4. Inside <code>load_inner_maybe_upgrade</code> function, check the current version in the wrapper, and if it's not the latest version,
call the data migration function to upgrade the inner object. Make sure to also update the version in the wrapper.
A detailed example can be found in ika/tests/framework_upgrades/mock_ika_systems/shallow_upgrade.
Along with the Move change, we also need to update the Rust code to support the new type. This includes:
1. Define a new <code>SystemInnerVX</code> struct type that matches the new Move type, and implement the SystemTrait.
2. Update the <code><a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a></code> struct to include the new version as a new enum variant.
3. Update the <code>get_ika_system_state</code> function to handle the new version.
To test that the upgrade will be successful, we need to modify <code>ika_system_state_production_upgrade_test</code> test in
protocol_version_tests and trigger a real upgrade using the new framework. We will need to keep this directory as old version,
put the new framework in a new directory, and run the test to exercise the upgrade.

To upgrade Validator type, besides everything above, we also need to:
1. Define a new Validator type (e.g. ValidatorV2).
2. Define a data migration function that migrates the old Validator to the new one (i.e. ValidatorV2).
3. Replace all uses of Validator with ValidatorV2 except the init creation function.
4. In validator_wrapper::upgrade_to_latest, check the current version in the wrapper, and if it's not the latest version,
call the data migration function to upgrade it.
In Rust, we also need to add a new case in <code>get_validator_from_table</code>.
Note that it is possible to upgrade SystemInnerVX without upgrading Validator, but not the other way around.
And when we only upgrade SystemInnerVX, the version of Validator in the wrapper will not be updated, and hence may become
inconsistent with the version of SystemInnerVX. This is fine as long as we don't use the Validator version to determine
the SystemInnerVX version, or vice versa.


-  [Struct `System`](#(ika_system=0x0)_system_System)
-  [Constants](#@Constants_0)
-  [Function `create`](#(ika_system=0x0)_system_create)
-  [Function `initialize`](#(ika_system=0x0)_system_initialize)
-  [Function `request_add_validator_candidate`](#(ika_system=0x0)_system_request_add_validator_candidate)
-  [Function `request_add_validator_candidate_non_entry`](#(ika_system=0x0)_system_request_add_validator_candidate_non_entry)
-  [Function `request_remove_validator_candidate`](#(ika_system=0x0)_system_request_remove_validator_candidate)
-  [Function `request_add_validator`](#(ika_system=0x0)_system_request_add_validator)
-  [Function `request_remove_validator`](#(ika_system=0x0)_system_request_remove_validator)
-  [Function `request_set_computation_price`](#(ika_system=0x0)_system_request_set_computation_price)
-  [Function `set_candidate_validator_computation_price`](#(ika_system=0x0)_system_set_candidate_validator_computation_price)
-  [Function `request_set_commission_rate`](#(ika_system=0x0)_system_request_set_commission_rate)
-  [Function `set_candidate_validator_commission_rate`](#(ika_system=0x0)_system_set_candidate_validator_commission_rate)
-  [Function `request_add_stake`](#(ika_system=0x0)_system_request_add_stake)
-  [Function `request_add_stake_non_entry`](#(ika_system=0x0)_system_request_add_stake_non_entry)
-  [Function `request_add_stake_mul_coin`](#(ika_system=0x0)_system_request_add_stake_mul_coin)
-  [Function `request_withdraw_stake`](#(ika_system=0x0)_system_request_withdraw_stake)
-  [Function `convert_to_fungible_staked_ika`](#(ika_system=0x0)_system_convert_to_fungible_staked_ika)
-  [Function `redeem_fungible_staked_ika`](#(ika_system=0x0)_system_redeem_fungible_staked_ika)
-  [Function `request_withdraw_stake_non_entry`](#(ika_system=0x0)_system_request_withdraw_stake_non_entry)
-  [Function `report_validator`](#(ika_system=0x0)_system_report_validator)
-  [Function `undo_report_validator`](#(ika_system=0x0)_system_undo_report_validator)
-  [Function `rotate_operation_cap`](#(ika_system=0x0)_system_rotate_operation_cap)
-  [Function `rotate_operation_cap_non_entry`](#(ika_system=0x0)_system_rotate_operation_cap_non_entry)
-  [Function `update_validator_payment_address`](#(ika_system=0x0)_system_update_validator_payment_address)
-  [Function `update_validator_name`](#(ika_system=0x0)_system_update_validator_name)
-  [Function `update_validator_description`](#(ika_system=0x0)_system_update_validator_description)
-  [Function `update_validator_image_url`](#(ika_system=0x0)_system_update_validator_image_url)
-  [Function `update_validator_project_url`](#(ika_system=0x0)_system_update_validator_project_url)
-  [Function `update_validator_next_epoch_network_address`](#(ika_system=0x0)_system_update_validator_next_epoch_network_address)
-  [Function `update_candidate_validator_network_address`](#(ika_system=0x0)_system_update_candidate_validator_network_address)
-  [Function `update_validator_next_epoch_p2p_address`](#(ika_system=0x0)_system_update_validator_next_epoch_p2p_address)
-  [Function `update_candidate_validator_p2p_address`](#(ika_system=0x0)_system_update_candidate_validator_p2p_address)
-  [Function `update_validator_next_epoch_consensus_address`](#(ika_system=0x0)_system_update_validator_next_epoch_consensus_address)
-  [Function `update_candidate_validator_consensus_address`](#(ika_system=0x0)_system_update_candidate_validator_consensus_address)
-  [Function `update_validator_next_epoch_protocol_pubkey_bytes`](#(ika_system=0x0)_system_update_validator_next_epoch_protocol_pubkey_bytes)
-  [Function `update_candidate_validator_protocol_pubkey_bytes`](#(ika_system=0x0)_system_update_candidate_validator_protocol_pubkey_bytes)
-  [Function `update_validator_next_epoch_consensus_pubkey_bytes`](#(ika_system=0x0)_system_update_validator_next_epoch_consensus_pubkey_bytes)
-  [Function `update_candidate_validator_consensus_pubkey_bytes`](#(ika_system=0x0)_system_update_candidate_validator_consensus_pubkey_bytes)
-  [Function `update_validator_next_epoch_class_groups_pubkey_and_proof_bytes`](#(ika_system=0x0)_system_update_validator_next_epoch_class_groups_pubkey_and_proof_bytes)
-  [Function `update_candidate_validator_class_groups_pubkey_and_proof_bytes`](#(ika_system=0x0)_system_update_candidate_validator_class_groups_pubkey_and_proof_bytes)
-  [Function `update_validator_next_epoch_network_pubkey_bytes`](#(ika_system=0x0)_system_update_validator_next_epoch_network_pubkey_bytes)
-  [Function `update_candidate_validator_network_pubkey_bytes`](#(ika_system=0x0)_system_update_candidate_validator_network_pubkey_bytes)
-  [Function `pool_exchange_rates`](#(ika_system=0x0)_system_pool_exchange_rates)
-  [Function `active_committee`](#(ika_system=0x0)_system_active_committee)
-  [Function `process_checkpoint_message_by_cap`](#(ika_system=0x0)_system_process_checkpoint_message_by_cap)
-  [Function `process_checkpoint_message_by_quorum`](#(ika_system=0x0)_system_process_checkpoint_message_by_quorum)
-  [Function `request_reconfig_mid_epoch`](#(ika_system=0x0)_system_request_reconfig_mid_epoch)
-  [Function `request_lock_epoch_sessions`](#(ika_system=0x0)_system_request_lock_epoch_sessions)
-  [Function `request_advance_epoch`](#(ika_system=0x0)_system_request_advance_epoch)
-  [Function `request_advance_network_keys`](#(ika_system=0x0)_system_request_advance_network_keys)
-  [Function `request_dwallet_network_decryption_key_dkg_by_cap`](#(ika_system=0x0)_system_request_dwallet_network_decryption_key_dkg_by_cap)
-  [Function `authorize_update_message_by_cap`](#(ika_system=0x0)_system_authorize_update_message_by_cap)
-  [Function `commit_upgrade`](#(ika_system=0x0)_system_commit_upgrade)
-  [Function `migrate`](#(ika_system=0x0)_system_migrate)
-  [Function `inner_mut`](#(ika_system=0x0)_system_inner_mut)
-  [Function `inner`](#(ika_system=0x0)_system_inner)


<pre><code><b>use</b> (ika=0x0)::ika;
<b>use</b> (ika_system=0x0)::<b>address</b>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee">bls_committee</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof">class_groups_public_key_and_proof</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1">dwallet_2pc_mpc_secp256k1</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner">dwallet_2pc_mpc_secp256k1_inner</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing">dwallet_pricing</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap">protocol_cap</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury">protocol_treasury</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_v1">system_inner_v1</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap">validator_cap</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1">validator_inner_v1</a>;
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
<b>use</b> <a href="../sui/pay.md#sui_pay">sui::pay</a>;
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
<b>use</b> <a href="../sui/versioned.md#sui_versioned">sui::versioned</a>;
</code></pre>



<a name="(ika_system=0x0)_system_System"></a>

## Struct `System`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a> <b>has</b> key
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
<code>version: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>package_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>new_package_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="(ika_system=0x0)_system_EHaveNotReachedEndEpochTime"></a>



<pre><code><b>const</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_EHaveNotReachedEndEpochTime">EHaveNotReachedEndEpochTime</a>: u64 = 3;
</code></pre>



<a name="(ika_system=0x0)_system_EHaveNotReachedMidEpochTime"></a>



<pre><code><b>const</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_EHaveNotReachedMidEpochTime">EHaveNotReachedMidEpochTime</a>: u64 = 2;
</code></pre>



<a name="(ika_system=0x0)_system_EInvalidMigration"></a>



<pre><code><b>const</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_EInvalidMigration">EInvalidMigration</a>: u64 = 1;
</code></pre>



<a name="(ika_system=0x0)_system_EWrongInnerVersion"></a>



<pre><code><b>const</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_EWrongInnerVersion">EWrongInnerVersion</a>: u64 = 0;
</code></pre>



<a name="(ika_system=0x0)_system_VERSION"></a>

Flag to indicate the version of the ika system.


<pre><code><b>const</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_VERSION">VERSION</a>: u64 = 1;
</code></pre>



<a name="(ika_system=0x0)_system_create"></a>

## Function `create`

Create a new System object and make it shared.
This function will be called only once in init.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_create">create</a>(package_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, upgrade_caps: vector&lt;<a href="../sui/package.md#sui_package_UpgradeCap">sui::package::UpgradeCap</a>&gt;, validators: (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, protocol_version: u64, epoch_start_timestamp_ms: u64, parameters: (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_v1_SystemParametersV1">system_inner_v1::SystemParametersV1</a>, <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury">protocol_treasury</a>: (ika_system=0x0)::<a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_ProtocolTreasury">protocol_treasury::ProtocolTreasury</a>, authorized_protocol_cap_ids: vector&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_create">create</a>(
    package_id: ID,
    upgrade_caps: vector&lt;UpgradeCap&gt;,
    validators: ValidatorSet,
    protocol_version: u64,
    epoch_start_timestamp_ms: u64,
    parameters: SystemParametersV1,
    <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury">protocol_treasury</a>: ProtocolTreasury,
    authorized_protocol_cap_ids: vector&lt;ID&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> system_state = <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_v1_create">system_inner_v1::create</a>(
        upgrade_caps,
        validators,
        protocol_version,
        epoch_start_timestamp_ms,
        parameters,
        <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury">protocol_treasury</a>,
        authorized_protocol_cap_ids,
        ctx,
    );
    <b>let</b> <b>mut</b> self = <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a> {
        id: object::new(ctx),
        version: <a href="../ika_system/system.md#(ika_system=0x0)_system_VERSION">VERSION</a>,
        package_id,
        new_package_id: option::none(),
    };
    dynamic_field::add(&<b>mut</b> self.id, <a href="../ika_system/system.md#(ika_system=0x0)_system_VERSION">VERSION</a>, system_state);
    transfer::share_object(self);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_initialize"></a>

## Function `initialize`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_initialize">initialize</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_initialize">initialize</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> package_id = self.package_id;
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_initialize">initialize</a>(clock, package_id, ctx);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_request_add_validator_candidate"></a>

## Function `request_add_validator_candidate`

Can be called by anyone who wishes to become a validator candidate and starts accruing delegated
stakes in their staking pool. Once they have at least <code>MIN_VALIDATOR_JOINING_STAKE</code> amount of stake they
can call <code><a href="../ika_system/system.md#(ika_system=0x0)_system_request_add_validator">request_add_validator</a></code> to officially become an active validator at the next epoch.
Aborts if the caller is already a pending or active validator, or a validator candidate.
Note: <code>proof_of_possession_bytes</code> MUST be a valid signature using sui_address and protocol_pubkey_bytes.
To produce a valid PoP, run [fn test_proof_of_possession_bytes].


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_add_validator_candidate">request_add_validator_candidate</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, pubkey_bytes: vector&lt;u8&gt;, network_pubkey_bytes: vector&lt;u8&gt;, consensus_pubkey_bytes: vector&lt;u8&gt;, class_groups_pubkey_and_proof_bytes: (ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof</a>, proof_of_possession_bytes: vector&lt;u8&gt;, name: vector&lt;u8&gt;, description: vector&lt;u8&gt;, image_url: vector&lt;u8&gt;, project_url: vector&lt;u8&gt;, network_address: vector&lt;u8&gt;, p2p_address: vector&lt;u8&gt;, consensus_address: vector&lt;u8&gt;, computation_price: u64, commission_rate: u16, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_add_validator_candidate">request_add_validator_candidate</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    pubkey_bytes: vector&lt;u8&gt;,
    network_pubkey_bytes: vector&lt;u8&gt;,
    consensus_pubkey_bytes: vector&lt;u8&gt;,
    class_groups_pubkey_and_proof_bytes: ClassGroupsPublicKeyAndProof,
    proof_of_possession_bytes: vector&lt;u8&gt;,
    name: vector&lt;u8&gt;,
    description: vector&lt;u8&gt;,
    image_url: vector&lt;u8&gt;,
    project_url: vector&lt;u8&gt;,
    network_address: vector&lt;u8&gt;,
    p2p_address: vector&lt;u8&gt;,
    consensus_address: vector&lt;u8&gt;,
    computation_price: u64,
    commission_rate: u16,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> (cap, operation_cap) = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_request_add_validator_candidate_non_entry">request_add_validator_candidate_non_entry</a>(
        ctx.sender(),
        pubkey_bytes,
        network_pubkey_bytes,
        consensus_pubkey_bytes,
        class_groups_pubkey_and_proof_bytes,
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
    transfer::public_transfer(cap, ctx.sender());
    transfer::public_transfer(operation_cap, ctx.sender());
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_request_add_validator_candidate_non_entry"></a>

## Function `request_add_validator_candidate_non_entry`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_add_validator_candidate_non_entry">request_add_validator_candidate_non_entry</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, payment_address: <b>address</b>, protocol_pubkey_bytes: vector&lt;u8&gt;, network_pubkey_bytes: vector&lt;u8&gt;, consensus_pubkey_bytes: vector&lt;u8&gt;, class_groups_pubkey_and_proof_bytes: (ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof</a>, proof_of_possession_bytes: vector&lt;u8&gt;, name: vector&lt;u8&gt;, description: vector&lt;u8&gt;, image_url: vector&lt;u8&gt;, project_url: vector&lt;u8&gt;, network_address: vector&lt;u8&gt;, p2p_address: vector&lt;u8&gt;, consensus_address: vector&lt;u8&gt;, computation_price: u64, commission_rate: u16, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): ((ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_add_validator_candidate_non_entry">request_add_validator_candidate_non_entry</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    payment_address: <b>address</b>,
    protocol_pubkey_bytes: vector&lt;u8&gt;,
    network_pubkey_bytes: vector&lt;u8&gt;,
    consensus_pubkey_bytes: vector&lt;u8&gt;,
    class_groups_pubkey_and_proof_bytes: ClassGroupsPublicKeyAndProof,
    proof_of_possession_bytes: vector&lt;u8&gt;,
    name: vector&lt;u8&gt;,
    description: vector&lt;u8&gt;,
    image_url: vector&lt;u8&gt;,
    project_url: vector&lt;u8&gt;,
    network_address: vector&lt;u8&gt;,
    p2p_address: vector&lt;u8&gt;,
    consensus_address: vector&lt;u8&gt;,
    computation_price: u64,
    commission_rate: u16,
    ctx: &<b>mut</b> TxContext,
): (ValidatorCap, ValidatorOperationCap) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_request_add_validator_candidate">request_add_validator_candidate</a>(
        payment_address,
        protocol_pubkey_bytes,
        network_pubkey_bytes,
        consensus_pubkey_bytes,
        class_groups_pubkey_and_proof_bytes,
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

<a name="(ika_system=0x0)_system_request_remove_validator_candidate"></a>

## Function `request_remove_validator_candidate`

Called by a validator candidate to remove themselves from the candidacy. After this call
their staking pool becomes deactivate.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_remove_validator_candidate">request_remove_validator_candidate</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_remove_validator_candidate">request_remove_validator_candidate</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_request_remove_validator_candidate">request_remove_validator_candidate</a>(cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_request_add_validator"></a>

## Function `request_add_validator`

Called by a validator candidate to add themselves to the active validator set beginning next epoch.
Aborts if the validator is a duplicate with one of the pending or active validators, or if the amount of
stake the validator has doesn't meet the min threshold, or if the number of new validators for the next
epoch has already reached the maximum.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_add_validator">request_add_validator</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_add_validator">request_add_validator</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_request_add_validator">request_add_validator</a>(cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_request_remove_validator"></a>

## Function `request_remove_validator`

A validator can call this function to request a removal in the next epoch.
We use the sender of <code>ctx</code> to look up the validator
(i.e. sender must match the sui_address in the validator).
At the end of the epoch, the <code><a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a></code> object will be returned to the sui_address
of the validator.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_remove_validator">request_remove_validator</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_remove_validator">request_remove_validator</a>(self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>, cap: &ValidatorCap) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_request_remove_validator">request_remove_validator</a>(cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_request_set_computation_price"></a>

## Function `request_set_computation_price`

A validator can call this entry function to submit a new computation price quote, to be
used for the computation price per unit size calculation at the end of the epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_set_computation_price">request_set_computation_price</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, new_computation_price: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_set_computation_price">request_set_computation_price</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    cap: &ValidatorOperationCap,
    new_computation_price: u64,
) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_request_set_computation_price">request_set_computation_price</a>(cap, new_computation_price)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_set_candidate_validator_computation_price"></a>

## Function `set_candidate_validator_computation_price`

This entry function is used to set new computation price for candidate validators


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_set_candidate_validator_computation_price">set_candidate_validator_computation_price</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, new_computation_price: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_set_candidate_validator_computation_price">set_candidate_validator_computation_price</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    cap: &ValidatorOperationCap,
    new_computation_price: u64,
) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_set_candidate_validator_computation_price">set_candidate_validator_computation_price</a>(cap, new_computation_price)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_request_set_commission_rate"></a>

## Function `request_set_commission_rate`

A validator can call this entry function to set a new commission rate, updated at the end of
the epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_set_commission_rate">request_set_commission_rate</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, new_commission_rate: u16, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_set_commission_rate">request_set_commission_rate</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    new_commission_rate: u16,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_request_set_commission_rate">request_set_commission_rate</a>(new_commission_rate, cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_set_candidate_validator_commission_rate"></a>

## Function `set_candidate_validator_commission_rate`

This entry function is used to set new commission rate for candidate validators


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_set_candidate_validator_commission_rate">set_candidate_validator_commission_rate</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, new_commission_rate: u16, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_set_candidate_validator_commission_rate">set_candidate_validator_commission_rate</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    new_commission_rate: u16,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_set_candidate_validator_commission_rate">set_candidate_validator_commission_rate</a>(new_commission_rate, cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_request_add_stake"></a>

## Function `request_add_stake`

Add stake to a validator's staking pool.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_add_stake">request_add_stake</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, stake: <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_add_stake">request_add_stake</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    stake: Coin&lt;IKA&gt;,
    validator_id: ID,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a> = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_request_add_stake_non_entry">request_add_stake_non_entry</a>(stake, validator_id, ctx);
    transfer::public_transfer(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>, ctx.sender());
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_request_add_stake_non_entry"></a>

## Function `request_add_stake_non_entry`

The non-entry version of <code><a href="../ika_system/system.md#(ika_system=0x0)_system_request_add_stake">request_add_stake</a></code>, which returns the staked IKA instead of transferring it to the sender.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_add_stake_non_entry">request_add_stake_non_entry</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, stake: <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_add_stake_non_entry">request_add_stake_non_entry</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    stake: Coin&lt;IKA&gt;,
    validator_id: ID,
    ctx: &<b>mut</b> TxContext,
): StakedIka {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_request_add_stake">request_add_stake</a>(stake, validator_id, ctx)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_request_add_stake_mul_coin"></a>

## Function `request_add_stake_mul_coin`

Add stake to a validator's staking pool using multiple coins.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_add_stake_mul_coin">request_add_stake_mul_coin</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, stakes: vector&lt;<a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;&gt;, stake_amount: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_add_stake_mul_coin">request_add_stake_mul_coin</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    stakes: vector&lt;Coin&lt;IKA&gt;&gt;,
    stake_amount: option::Option&lt;u64&gt;,
    validator_id: ID,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    <b>let</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a> = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_request_add_stake_mul_coin">request_add_stake_mul_coin</a>(stakes, stake_amount, validator_id, ctx);
    transfer::public_transfer(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>, ctx.sender());
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_request_withdraw_stake"></a>

## Function `request_withdraw_stake`

Withdraw stake from a validator's staking pool.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_withdraw_stake">request_withdraw_stake</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_withdraw_stake">request_withdraw_stake</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: StakedIka,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> withdrawn_stake = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_request_withdraw_stake_non_entry">request_withdraw_stake_non_entry</a>(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>);
    transfer::public_transfer(withdrawn_stake.into_coin(ctx), ctx.sender());
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_convert_to_fungible_staked_ika"></a>

## Function `convert_to_fungible_staked_ika`

Convert StakedIka into a FungibleStakedIka object.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_convert_to_fungible_staked_ika">convert_to_fungible_staked_ika</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_convert_to_fungible_staked_ika">convert_to_fungible_staked_ika</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: StakedIka,
    ctx: &<b>mut</b> TxContext,
): FungibleStakedIka {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_convert_to_fungible_staked_ika">convert_to_fungible_staked_ika</a>(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>, ctx)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_redeem_fungible_staked_ika"></a>

## Function `redeem_fungible_staked_ika`

Convert FungibleStakedIka into a StakedIka object.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_redeem_fungible_staked_ika">redeem_fungible_staked_ika</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, fungible_staked_ika: (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_redeem_fungible_staked_ika">redeem_fungible_staked_ika</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    fungible_staked_ika: FungibleStakedIka,
): Balance&lt;IKA&gt; {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_redeem_fungible_staked_ika">redeem_fungible_staked_ika</a>(fungible_staked_ika)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_request_withdraw_stake_non_entry"></a>

## Function `request_withdraw_stake_non_entry`

Non-entry version of <code><a href="../ika_system/system.md#(ika_system=0x0)_system_request_withdraw_stake">request_withdraw_stake</a></code> that returns the withdrawn IKA instead of transferring it to the sender.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_withdraw_stake_non_entry">request_withdraw_stake_non_entry</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_withdraw_stake_non_entry">request_withdraw_stake_non_entry</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: StakedIka,
): Balance&lt;IKA&gt; {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_request_withdraw_stake">request_withdraw_stake</a>(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_report_validator"></a>

## Function `report_validator`

Report a validator as a bad or non-performant actor in the system.
Succeeds if all the following are satisfied:
1. both the reporter in <code>cap</code> and the input <code>reportee_id</code> are active validators.
2. reporter and reportee not the same address.
3. the cap object is still valid.
This function is idempotent.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_report_validator">report_validator</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, reportee_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_report_validator">report_validator</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    cap: &ValidatorOperationCap,
    reportee_id: ID,
) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_report_validator">report_validator</a>(cap, reportee_id)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_undo_report_validator"></a>

## Function `undo_report_validator`

Undo a <code><a href="../ika_system/system.md#(ika_system=0x0)_system_report_validator">report_validator</a></code> action. Aborts if
1. the reportee is not a currently active validator or
2. the sender has not previously reported the <code>reportee_id</code>, or
3. the cap is not valid


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_undo_report_validator">undo_report_validator</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, reportee_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_undo_report_validator">undo_report_validator</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    cap: &ValidatorOperationCap,
    reportee_id: ID,
) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_undo_report_validator">undo_report_validator</a>(cap, reportee_id)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_rotate_operation_cap"></a>

## Function `rotate_operation_cap`

Create a new <code>ValidatorOperationCap</code>, transfer it to the
validator and registers it. The original object is thus revoked.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_rotate_operation_cap">rotate_operation_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_rotate_operation_cap">rotate_operation_cap</a>(self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>, cap: &ValidatorCap, ctx: &<b>mut</b> TxContext) {
    <b>let</b> operation_cap = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_rotate_operation_cap_non_entry">rotate_operation_cap_non_entry</a>(cap, ctx);
    transfer::public_transfer(operation_cap, ctx.sender());
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_rotate_operation_cap_non_entry"></a>

## Function `rotate_operation_cap_non_entry`

Create a new <code>ValidatorOperationCap</code> and registers it. The original object is thus revoked.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_rotate_operation_cap_non_entry">rotate_operation_cap_non_entry</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_rotate_operation_cap_non_entry">rotate_operation_cap_non_entry</a>(self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>, cap: &ValidatorCap, ctx: &<b>mut</b> TxContext): ValidatorOperationCap {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_rotate_operation_cap">rotate_operation_cap</a>(cap, ctx)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_update_validator_payment_address"></a>

## Function `update_validator_payment_address`

Update a validator's payment address.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_payment_address">update_validator_payment_address</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, payment_address: <b>address</b>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_payment_address">update_validator_payment_address</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    payment_address: <b>address</b>,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_payment_address">update_validator_payment_address</a>(payment_address, cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_update_validator_name"></a>

## Function `update_validator_name`

Update a validator's name.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_name">update_validator_name</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, name: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_name">update_validator_name</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    name: vector&lt;u8&gt;,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_name">update_validator_name</a>(name, cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_update_validator_description"></a>

## Function `update_validator_description`

Update a validator's description


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_description">update_validator_description</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, description: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_description">update_validator_description</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    description: vector&lt;u8&gt;,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_description">update_validator_description</a>(description, cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_update_validator_image_url"></a>

## Function `update_validator_image_url`

Update a validator's image url


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_image_url">update_validator_image_url</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, image_url: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_image_url">update_validator_image_url</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    image_url: vector&lt;u8&gt;,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_image_url">update_validator_image_url</a>(image_url, cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_update_validator_project_url"></a>

## Function `update_validator_project_url`

Update a validator's project url


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_project_url">update_validator_project_url</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, project_url: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_project_url">update_validator_project_url</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    project_url: vector&lt;u8&gt;,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_project_url">update_validator_project_url</a>(project_url, cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_update_validator_next_epoch_network_address"></a>

## Function `update_validator_next_epoch_network_address`

Update a validator's network address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_next_epoch_network_address">update_validator_next_epoch_network_address</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, network_address: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_next_epoch_network_address">update_validator_next_epoch_network_address</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    network_address: vector&lt;u8&gt;,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_next_epoch_network_address">update_validator_next_epoch_network_address</a>(network_address, cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_update_candidate_validator_network_address"></a>

## Function `update_candidate_validator_network_address`

Update candidate validator's network address.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_candidate_validator_network_address">update_candidate_validator_network_address</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, network_address: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_candidate_validator_network_address">update_candidate_validator_network_address</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    network_address: vector&lt;u8&gt;,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_update_candidate_validator_network_address">update_candidate_validator_network_address</a>(network_address, cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_update_validator_next_epoch_p2p_address"></a>

## Function `update_validator_next_epoch_p2p_address`

Update a validator's p2p address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_next_epoch_p2p_address">update_validator_next_epoch_p2p_address</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, p2p_address: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_next_epoch_p2p_address">update_validator_next_epoch_p2p_address</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    p2p_address: vector&lt;u8&gt;,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_next_epoch_p2p_address">update_validator_next_epoch_p2p_address</a>(p2p_address, cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_update_candidate_validator_p2p_address"></a>

## Function `update_candidate_validator_p2p_address`

Update candidate validator's p2p address.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_candidate_validator_p2p_address">update_candidate_validator_p2p_address</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, p2p_address: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_candidate_validator_p2p_address">update_candidate_validator_p2p_address</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    p2p_address: vector&lt;u8&gt;,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_update_candidate_validator_p2p_address">update_candidate_validator_p2p_address</a>(p2p_address, cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_update_validator_next_epoch_consensus_address"></a>

## Function `update_validator_next_epoch_consensus_address`

Update a validator's consensus address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_next_epoch_consensus_address">update_validator_next_epoch_consensus_address</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, consensus_address: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_next_epoch_consensus_address">update_validator_next_epoch_consensus_address</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    consensus_address: vector&lt;u8&gt;,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_next_epoch_consensus_address">update_validator_next_epoch_consensus_address</a>(consensus_address, cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_update_candidate_validator_consensus_address"></a>

## Function `update_candidate_validator_consensus_address`

Update candidate validator's consensus address.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_candidate_validator_consensus_address">update_candidate_validator_consensus_address</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, consensus_address: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_candidate_validator_consensus_address">update_candidate_validator_consensus_address</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    consensus_address: vector&lt;u8&gt;,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_update_candidate_validator_consensus_address">update_candidate_validator_consensus_address</a>(consensus_address, cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_update_validator_next_epoch_protocol_pubkey_bytes"></a>

## Function `update_validator_next_epoch_protocol_pubkey_bytes`

Update a validator's public key of protocol key and proof of possession.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_next_epoch_protocol_pubkey_bytes">update_validator_next_epoch_protocol_pubkey_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, protocol_pubkey: vector&lt;u8&gt;, proof_of_possession_bytes: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, ctx: &<a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_next_epoch_protocol_pubkey_bytes">update_validator_next_epoch_protocol_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    protocol_pubkey: vector&lt;u8&gt;,
    proof_of_possession_bytes: vector&lt;u8&gt;,
    cap: &ValidatorCap,
    ctx: &TxContext,
) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_next_epoch_protocol_pubkey_bytes">update_validator_next_epoch_protocol_pubkey_bytes</a>(protocol_pubkey, proof_of_possession_bytes, cap, ctx)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_update_candidate_validator_protocol_pubkey_bytes"></a>

## Function `update_candidate_validator_protocol_pubkey_bytes`

Update candidate validator's public key of protocol key and proof of possession.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_candidate_validator_protocol_pubkey_bytes">update_candidate_validator_protocol_pubkey_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, protocol_pubkey: vector&lt;u8&gt;, proof_of_possession_bytes: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, ctx: &<a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_candidate_validator_protocol_pubkey_bytes">update_candidate_validator_protocol_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    protocol_pubkey: vector&lt;u8&gt;,
    proof_of_possession_bytes: vector&lt;u8&gt;,
    cap: &ValidatorCap,
    ctx: &TxContext,
) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_update_candidate_validator_protocol_pubkey_bytes">update_candidate_validator_protocol_pubkey_bytes</a>(protocol_pubkey, proof_of_possession_bytes, cap, ctx)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_update_validator_next_epoch_consensus_pubkey_bytes"></a>

## Function `update_validator_next_epoch_consensus_pubkey_bytes`

Update a validator's public key of worker key.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_next_epoch_consensus_pubkey_bytes">update_validator_next_epoch_consensus_pubkey_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, consensus_pubkey_bytes: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_next_epoch_consensus_pubkey_bytes">update_validator_next_epoch_consensus_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    consensus_pubkey_bytes: vector&lt;u8&gt;,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_next_epoch_consensus_pubkey_bytes">update_validator_next_epoch_consensus_pubkey_bytes</a>(consensus_pubkey_bytes, cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_update_candidate_validator_consensus_pubkey_bytes"></a>

## Function `update_candidate_validator_consensus_pubkey_bytes`

Update candidate validator's public key of worker key.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_candidate_validator_consensus_pubkey_bytes">update_candidate_validator_consensus_pubkey_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, consensus_pubkey_bytes: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_candidate_validator_consensus_pubkey_bytes">update_candidate_validator_consensus_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    consensus_pubkey_bytes: vector&lt;u8&gt;,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_update_candidate_validator_consensus_pubkey_bytes">update_candidate_validator_consensus_pubkey_bytes</a>(consensus_pubkey_bytes, cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_update_validator_next_epoch_class_groups_pubkey_and_proof_bytes"></a>

## Function `update_validator_next_epoch_class_groups_pubkey_and_proof_bytes`

Update a validator's public key of class groups key and its associated proof.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_next_epoch_class_groups_pubkey_and_proof_bytes">update_validator_next_epoch_class_groups_pubkey_and_proof_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, class_groups_pubkey_and_proof: (ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_next_epoch_class_groups_pubkey_and_proof_bytes">update_validator_next_epoch_class_groups_pubkey_and_proof_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    class_groups_pubkey_and_proof: ClassGroupsPublicKeyAndProof,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_next_epoch_class_groups_pubkey_and_proof_bytes">update_validator_next_epoch_class_groups_pubkey_and_proof_bytes</a>(class_groups_pubkey_and_proof, cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_update_candidate_validator_class_groups_pubkey_and_proof_bytes"></a>

## Function `update_candidate_validator_class_groups_pubkey_and_proof_bytes`

Update candidate validator's public key of class groups key and its associated proof.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_candidate_validator_class_groups_pubkey_and_proof_bytes">update_candidate_validator_class_groups_pubkey_and_proof_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, class_groups_pubkey_and_proof: (ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_candidate_validator_class_groups_pubkey_and_proof_bytes">update_candidate_validator_class_groups_pubkey_and_proof_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    class_groups_pubkey_and_proof: ClassGroupsPublicKeyAndProof,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_update_candidate_validator_class_groups_pubkey_and_proof_bytes">update_candidate_validator_class_groups_pubkey_and_proof_bytes</a>(class_groups_pubkey_and_proof, cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_update_validator_next_epoch_network_pubkey_bytes"></a>

## Function `update_validator_next_epoch_network_pubkey_bytes`

Update a validator's public key of network key.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_next_epoch_network_pubkey_bytes">update_validator_next_epoch_network_pubkey_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, network_pubkey: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_next_epoch_network_pubkey_bytes">update_validator_next_epoch_network_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    network_pubkey: vector&lt;u8&gt;,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_update_validator_next_epoch_network_pubkey_bytes">update_validator_next_epoch_network_pubkey_bytes</a>(network_pubkey, cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_update_candidate_validator_network_pubkey_bytes"></a>

## Function `update_candidate_validator_network_pubkey_bytes`

Update candidate validator's public key of network key.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_candidate_validator_network_pubkey_bytes">update_candidate_validator_network_pubkey_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, network_pubkey: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_update_candidate_validator_network_pubkey_bytes">update_candidate_validator_network_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    network_pubkey: vector&lt;u8&gt;,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_update_candidate_validator_network_pubkey_bytes">update_candidate_validator_network_pubkey_bytes</a>(network_pubkey, cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_pool_exchange_rates"></a>

## Function `pool_exchange_rates`

Getter of the pool token exchange rate of a validator. Works for both active and inactive pools.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_pool_exchange_rates">pool_exchange_rates</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): &<a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_pool_exchange_rates">pool_exchange_rates</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    validator_id: ID,
): &Table&lt;u64, PoolTokenExchangeRate&gt; {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_pool_exchange_rates">pool_exchange_rates</a>(validator_id)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_active_committee"></a>

## Function `active_committee`

Getter returning ids of the currently active validators.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_active_committee">active_committee</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>): (ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">bls_committee::BlsCommittee</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_active_committee">active_committee</a>(self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>): BlsCommittee {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner">inner</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_active_committee">active_committee</a>()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_process_checkpoint_message_by_cap"></a>

## Function `process_checkpoint_message_by_cap`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_process_checkpoint_message_by_cap">process_checkpoint_message_by_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, cap: &(ika_system=0x0)::<a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_ProtocolCap">protocol_cap::ProtocolCap</a>, message: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_process_checkpoint_message_by_cap">process_checkpoint_message_by_cap</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    cap: &ProtocolCap,
    message: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_process_checkpoint_message_by_cap">process_checkpoint_message_by_cap</a>(cap, message, ctx);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_process_checkpoint_message_by_quorum"></a>

## Function `process_checkpoint_message_by_quorum`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1">dwallet_2pc_mpc_secp256k1</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWalletCoordinator">dwallet_2pc_mpc_secp256k1::DWalletCoordinator</a>, signature: vector&lt;u8&gt;, signers_bitmap: vector&lt;u8&gt;, message: vector&lt;u8&gt;, message2: vector&lt;u8&gt;, message3: vector&lt;u8&gt;, message4: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1">dwallet_2pc_mpc_secp256k1</a>: &<b>mut</b> DWalletCoordinator,
    signature: vector&lt;u8&gt;,
    signers_bitmap: vector&lt;u8&gt;,
    <b>mut</b> message: vector&lt;u8&gt;,
    message2: vector&lt;u8&gt;,
    message3: vector&lt;u8&gt;,
    message4: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    message.append(message2);
    message.append(message3);
    message.append(message4);
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1">dwallet_2pc_mpc_secp256k1</a>, signature, signers_bitmap, message, ctx);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_request_reconfig_mid_epoch"></a>

## Function `request_reconfig_mid_epoch`

Locks the committee of the next epoch to allow starting the reconfiguration process.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_reconfig_mid_epoch">request_reconfig_mid_epoch</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, dwallet_coordinator: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWalletCoordinator">dwallet_2pc_mpc_secp256k1::DWalletCoordinator</a>, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_reconfig_mid_epoch">request_reconfig_mid_epoch</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>, dwallet_coordinator: &<b>mut</b> DWalletCoordinator, clock: &Clock, ctx: &<b>mut</b> TxContext
) {
    <b>let</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_inner">inner</a> = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    <b>assert</b>!(clock.timestamp_ms() &gt; <a href="../ika_system/system.md#(ika_system=0x0)_system_inner">inner</a>.epoch_start_timestamp_ms() + (<a href="../ika_system/system.md#(ika_system=0x0)_system_inner">inner</a>.epoch_duration_ms() / 2), <a href="../ika_system/system.md#(ika_system=0x0)_system_EHaveNotReachedMidEpochTime">EHaveNotReachedMidEpochTime</a>);
    <a href="../ika_system/system.md#(ika_system=0x0)_system_inner">inner</a>.emit_start_reshare_events(dwallet_coordinator.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>(), ctx);
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().process_mid_epoch();
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_request_lock_epoch_sessions"></a>

## Function `request_lock_epoch_sessions`

Locks the MPC sessions that should get completed as part of the current epoch.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_lock_epoch_sessions">request_lock_epoch_sessions</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, dwallet_coordinator: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWalletCoordinator">dwallet_2pc_mpc_secp256k1::DWalletCoordinator</a>, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>, _ctx: &<a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_lock_epoch_sessions">request_lock_epoch_sessions</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>, dwallet_coordinator: &<b>mut</b> DWalletCoordinator, clock: &Clock, _ctx: &TxContext
) {
    <b>let</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_inner">inner</a> = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    <b>assert</b>!(clock.timestamp_ms() &gt; <a href="../ika_system/system.md#(ika_system=0x0)_system_inner">inner</a>.epoch_start_timestamp_ms() + (<a href="../ika_system/system.md#(ika_system=0x0)_system_inner">inner</a>.epoch_duration_ms()), <a href="../ika_system/system.md#(ika_system=0x0)_system_EHaveNotReachedEndEpochTime">EHaveNotReachedEndEpochTime</a>);
    dwallet_coordinator.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().lock_last_active_session_sequence_number();
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_request_advance_epoch"></a>

## Function `request_advance_epoch`

Advances the epoch to the next epoch.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_advance_epoch">request_advance_epoch</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, dwallet_coordinator: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWalletCoordinator">dwallet_2pc_mpc_secp256k1::DWalletCoordinator</a>, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_advance_epoch">request_advance_epoch</a>(self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>, dwallet_coordinator: &<b>mut</b> DWalletCoordinator, clock: &Clock, ctx: &<b>mut</b> TxContext) {
    <b>let</b> inner_system = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    <b>let</b> inner_dwallet = dwallet_coordinator.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    <b>assert</b>!(inner_dwallet.all_current_epoch_sessions_completed(), 5);
    inner_system.advance_epoch(clock.timestamp_ms(), ctx);
    dwallet_coordinator.advance_epoch(inner_system.<a href="../ika_system/system.md#(ika_system=0x0)_system_active_committee">active_committee</a>());
    inner_system.advance_network_keys(dwallet_coordinator);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_request_advance_network_keys"></a>

## Function `request_advance_network_keys`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_advance_network_keys">request_advance_network_keys</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, dwallet_coordinator: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWalletCoordinator">dwallet_2pc_mpc_secp256k1::DWalletCoordinator</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_advance_network_keys">request_advance_network_keys</a>(self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>, dwallet_coordinator: &<b>mut</b> DWalletCoordinator) {
    <b>let</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_inner">inner</a> = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    <a href="../ika_system/system.md#(ika_system=0x0)_system_inner">inner</a>.advance_network_keys(dwallet_coordinator);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_request_dwallet_network_decryption_key_dkg_by_cap"></a>

## Function `request_dwallet_network_decryption_key_dkg_by_cap`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_dwallet_network_decryption_key_dkg_by_cap">request_dwallet_network_decryption_key_dkg_by_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1">dwallet_2pc_mpc_secp256k1</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWalletCoordinator">dwallet_2pc_mpc_secp256k1::DWalletCoordinator</a>, cap: &(ika_system=0x0)::<a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_ProtocolCap">protocol_cap::ProtocolCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_dwallet_network_decryption_key_dkg_by_cap">request_dwallet_network_decryption_key_dkg_by_cap</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1">dwallet_2pc_mpc_secp256k1</a>: &<b>mut</b> DWalletCoordinator,
    cap: &ProtocolCap,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_request_dwallet_network_decryption_key_dkg_by_cap">request_dwallet_network_decryption_key_dkg_by_cap</a>(<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1">dwallet_2pc_mpc_secp256k1</a>, cap, ctx);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_authorize_update_message_by_cap"></a>

## Function `authorize_update_message_by_cap`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_authorize_update_message_by_cap">authorize_update_message_by_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, cap: &(ika_system=0x0)::<a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_ProtocolCap">protocol_cap::ProtocolCap</a>, package_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, digest: vector&lt;u8&gt;): <a href="../sui/package.md#sui_package_UpgradeTicket">sui::package::UpgradeTicket</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_authorize_update_message_by_cap">authorize_update_message_by_cap</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    cap: &ProtocolCap,
    package_id: ID,
    digest: vector&lt;u8&gt;,
): UpgradeTicket {
    <b>let</b> self = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_authorize_update_message_by_cap">authorize_update_message_by_cap</a>(cap, package_id, digest)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_commit_upgrade"></a>

## Function `commit_upgrade`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_commit_upgrade">commit_upgrade</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, receipt: <a href="../sui/package.md#sui_package_UpgradeReceipt">sui::package::UpgradeReceipt</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_commit_upgrade">commit_upgrade</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    receipt: UpgradeReceipt,
) {
    <b>let</b> new_package_id = receipt.package();
    <b>let</b> old_package_id = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_commit_upgrade">commit_upgrade</a>(receipt);
    <b>if</b> (self.package_id == old_package_id) {
        self.new_package_id = option::some(new_package_id);
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_migrate"></a>

## Function `migrate`

Migrate the staking object to the new package id.

This function sets the new package id and version and can be modified in future versions
to migrate changes in the <code>system_inner</code> object if needed.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_migrate">migrate</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_migrate">migrate</a>(
        self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
) {
    <b>assert</b>!(self.version &lt; <a href="../ika_system/system.md#(ika_system=0x0)_system_VERSION">VERSION</a>, <a href="../ika_system/system.md#(ika_system=0x0)_system_EInvalidMigration">EInvalidMigration</a>);
    // Move the old <a href="../ika_system/system.md#(ika_system=0x0)_system">system</a> state <a href="../ika_system/system.md#(ika_system=0x0)_system_inner">inner</a> to the new version.
    <b>let</b> system_inner: SystemInnerV1 = dynamic_field::remove(&<b>mut</b> self.id, self.version);
    dynamic_field::add(&<b>mut</b> self.id, <a href="../ika_system/system.md#(ika_system=0x0)_system_VERSION">VERSION</a>, system_inner);
    self.version = <a href="../ika_system/system.md#(ika_system=0x0)_system_VERSION">VERSION</a>;
    // Set the new package id.
    <b>assert</b>!(self.new_package_id.is_some(), <a href="../ika_system/system.md#(ika_system=0x0)_system_EInvalidMigration">EInvalidMigration</a>);
    self.package_id = self.new_package_id.extract();
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_mut"></a>

## Function `inner_mut`

Get a mutable reference to <code>SystemInnerVX</code> from the <code><a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a></code>.


<pre><code><b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>): &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>(self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>): &<b>mut</b> SystemInnerV1 {
    <b>assert</b>!(self.version == <a href="../ika_system/system.md#(ika_system=0x0)_system_VERSION">VERSION</a>, <a href="../ika_system/system.md#(ika_system=0x0)_system_EWrongInnerVersion">EWrongInnerVersion</a>);
    dynamic_field::borrow_mut(&<b>mut</b> self.id, <a href="../ika_system/system.md#(ika_system=0x0)_system_VERSION">VERSION</a>)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner"></a>

## Function `inner`

Get an immutable reference to <code>SystemInnerVX</code> from the <code><a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a></code>.


<pre><code><b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_inner">inner</a>(self: &(ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>): &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_inner">inner</a>(self: &<a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>): &SystemInnerV1 {
    <b>assert</b>!(self.version == <a href="../ika_system/system.md#(ika_system=0x0)_system_VERSION">VERSION</a>, <a href="../ika_system/system.md#(ika_system=0x0)_system_EWrongInnerVersion">EWrongInnerVersion</a>);
    dynamic_field::borrow(&self.id, <a href="../ika_system/system.md#(ika_system=0x0)_system_VERSION">VERSION</a>)
}
</code></pre>



</details>
