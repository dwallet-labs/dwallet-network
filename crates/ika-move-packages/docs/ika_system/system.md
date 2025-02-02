---
title: Module `0x0::system`
---

Ika System State Type Upgrade Guide
<code><a href="system.md#0x0_system_System">System</a></code> is a thin wrapper around <code>SystemInnerVX</code> that provides a versioned interface.
The <code><a href="system.md#0x0_system_System">System</a></code> object has a fixed ID 0x5, and the <code>SystemInnerVX</code> object is stored as a dynamic field.
There are a few different ways to upgrade the <code>SystemInnerVX</code> type:

The simplest and one that doesn't involve a real upgrade is to just add dynamic fields to the <code>extra_fields</code> field
of <code>SystemInnerVX</code> or any of its sub type. This is useful when we are in a rush, or making a small change,
or still experimenting a new field.

To properly upgrade the <code>SystemInnerVX</code> type, we need to ship a new framework that does the following:
1. Define a new <code>SystemInnerVX</code>type (e.g. <code>SystemInnerV1</code>).
2. Define a data migration function that migrates the old <code>SystemInnerVX</code> to the new one (i.e. SystemInnerV1).
3. Replace all uses of <code>SystemInnerVX</code> with <code>SystemInnerV1</code> in both ika_system.move and system_inner_v1.move,
with the exception of the <code><a href="system_inner.md#0x0_system_inner_v1_create">system_inner_v1::create</a></code> function, which should always return the init type.
4. Inside <code>load_inner_maybe_upgrade</code> function, check the current version in the wrapper, and if it's not the latest version,
call the data migration function to upgrade the inner object. Make sure to also update the version in the wrapper.
A detailed example can be found in ika/tests/framework_upgrades/mock_ika_systems/shallow_upgrade.
Along with the Move change, we also need to update the Rust code to support the new type. This includes:
1. Define a new <code>SystemInnerVX</code> struct type that matches the new Move type, and implement the SystemTrait.
2. Update the <code><a href="system.md#0x0_system_System">System</a></code> struct to include the new version as a new enum variant.
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


-  [Resource `System`](#0x0_system_System)
-  [Constants](#@Constants_0)
-  [Function `create`](#0x0_system_create)
-  [Function `initialize`](#0x0_system_initialize)
-  [Function `request_add_validator_candidate`](#0x0_system_request_add_validator_candidate)
-  [Function `request_add_validator_candidate_non_entry`](#0x0_system_request_add_validator_candidate_non_entry)
-  [Function `request_remove_validator_candidate`](#0x0_system_request_remove_validator_candidate)
-  [Function `request_add_validator`](#0x0_system_request_add_validator)
-  [Function `request_remove_validator`](#0x0_system_request_remove_validator)
-  [Function `request_set_computation_price`](#0x0_system_request_set_computation_price)
-  [Function `set_candidate_validator_computation_price`](#0x0_system_set_candidate_validator_computation_price)
-  [Function `request_set_commission_rate`](#0x0_system_request_set_commission_rate)
-  [Function `set_candidate_validator_commission_rate`](#0x0_system_set_candidate_validator_commission_rate)
-  [Function `request_add_stake`](#0x0_system_request_add_stake)
-  [Function `request_add_stake_non_entry`](#0x0_system_request_add_stake_non_entry)
-  [Function `request_add_stake_mul_coin`](#0x0_system_request_add_stake_mul_coin)
-  [Function `request_withdraw_stake`](#0x0_system_request_withdraw_stake)
-  [Function `convert_to_fungible_staked_ika`](#0x0_system_convert_to_fungible_staked_ika)
-  [Function `redeem_fungible_staked_ika`](#0x0_system_redeem_fungible_staked_ika)
-  [Function `request_withdraw_stake_non_entry`](#0x0_system_request_withdraw_stake_non_entry)
-  [Function `report_validator`](#0x0_system_report_validator)
-  [Function `undo_report_validator`](#0x0_system_undo_report_validator)
-  [Function `rotate_operation_cap`](#0x0_system_rotate_operation_cap)
-  [Function `rotate_operation_cap_non_entry`](#0x0_system_rotate_operation_cap_non_entry)
-  [Function `update_validator_payment_address`](#0x0_system_update_validator_payment_address)
-  [Function `update_validator_name`](#0x0_system_update_validator_name)
-  [Function `update_validator_description`](#0x0_system_update_validator_description)
-  [Function `update_validator_image_url`](#0x0_system_update_validator_image_url)
-  [Function `update_validator_project_url`](#0x0_system_update_validator_project_url)
-  [Function `update_validator_next_epoch_network_address`](#0x0_system_update_validator_next_epoch_network_address)
-  [Function `update_candidate_validator_network_address`](#0x0_system_update_candidate_validator_network_address)
-  [Function `update_validator_next_epoch_p2p_address`](#0x0_system_update_validator_next_epoch_p2p_address)
-  [Function `update_candidate_validator_p2p_address`](#0x0_system_update_candidate_validator_p2p_address)
-  [Function `update_validator_next_epoch_consensus_address`](#0x0_system_update_validator_next_epoch_consensus_address)
-  [Function `update_candidate_validator_consensus_address`](#0x0_system_update_candidate_validator_consensus_address)
-  [Function `update_validator_next_epoch_protocol_pubkey_bytes`](#0x0_system_update_validator_next_epoch_protocol_pubkey_bytes)
-  [Function `update_candidate_validator_protocol_pubkey_bytes`](#0x0_system_update_candidate_validator_protocol_pubkey_bytes)
-  [Function `update_validator_next_epoch_consensus_pubkey_bytes`](#0x0_system_update_validator_next_epoch_consensus_pubkey_bytes)
-  [Function `update_candidate_validator_consensus_pubkey_bytes`](#0x0_system_update_candidate_validator_consensus_pubkey_bytes)
-  [Function `update_validator_next_epoch_network_pubkey_bytes`](#0x0_system_update_validator_next_epoch_network_pubkey_bytes)
-  [Function `update_candidate_validator_network_pubkey_bytes`](#0x0_system_update_candidate_validator_network_pubkey_bytes)
-  [Function `pool_exchange_rates`](#0x0_system_pool_exchange_rates)
-  [Function `active_committee`](#0x0_system_active_committee)
-  [Function `process_checkpoint_message_by_cap`](#0x0_system_process_checkpoint_message_by_cap)
-  [Function `process_checkpoint_message_by_quorum`](#0x0_system_process_checkpoint_message_by_quorum)
-  [Function `authorize_update_message_by_cap`](#0x0_system_authorize_update_message_by_cap)
-  [Function `commit_upgrade`](#0x0_system_commit_upgrade)
-  [Function `migrate`](#0x0_system_migrate)
-  [Function `inner_mut`](#0x0_system_inner_mut)
-  [Function `inner`](#0x0_system_inner)


<pre><code><b>use</b> <a href="committee.md#0x0_committee">0x0::committee</a>;
<b>use</b> <a href="../ika/ika.md#0x0_ika">0x0::ika</a>;
<b>use</b> <a href="protocol_cap.md#0x0_protocol_cap">0x0::protocol_cap</a>;
<b>use</b> <a href="protocol_treasury.md#0x0_protocol_treasury">0x0::protocol_treasury</a>;
<b>use</b> <a href="staked_ika.md#0x0_staked_ika">0x0::staked_ika</a>;
<b>use</b> <a href="staking_pool.md#0x0_staking_pool">0x0::staking_pool</a>;
<b>use</b> <a href="system_inner.md#0x0_system_inner_v1">0x0::system_inner_v1</a>;
<b>use</b> <a href="validator_cap.md#0x0_validator_cap">0x0::validator_cap</a>;
<b>use</b> <a href="validator_set.md#0x0_validator_set">0x0::validator_set</a>;
<b>use</b> <a href="../move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../sui-framework/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../sui-framework/clock.md#0x2_clock">0x2::clock</a>;
<b>use</b> <a href="../sui-framework/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field">0x2::dynamic_field</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/package.md#0x2_package">0x2::package</a>;
<b>use</b> <a href="../sui-framework/table.md#0x2_table">0x2::table</a>;
<b>use</b> <a href="../sui-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
</code></pre>



<a name="0x0_system_System"></a>

## Resource `System`



<pre><code><b>struct</b> <a href="system.md#0x0_system_System">System</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>version: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>package_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>new_package_id: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x0_system_VERSION"></a>

Flag to indicate the version of the ika system.


<pre><code><b>const</b> <a href="system.md#0x0_system_VERSION">VERSION</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1;
</code></pre>



<a name="0x0_system_EInvalidMigration"></a>



<pre><code><b>const</b> <a href="system.md#0x0_system_EInvalidMigration">EInvalidMigration</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1;
</code></pre>



<a name="0x0_system_EWrongInnerVersion"></a>



<pre><code><b>const</b> <a href="system.md#0x0_system_EWrongInnerVersion">EWrongInnerVersion</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
</code></pre>



<a name="0x0_system_create"></a>

## Function `create`

Create a new System object and make it shared.
This function will be called only once in init.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="system.md#0x0_system_create">create</a>(package_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, upgrade_caps: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../sui-framework/package.md#0x2_package_UpgradeCap">package::UpgradeCap</a>&gt;, validators: <a href="validator_set.md#0x0_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, protocol_version: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, epoch_start_timestamp_ms: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, parameters: <a href="system_inner.md#0x0_system_inner_v1_SystemParametersV1">system_inner_v1::SystemParametersV1</a>, <a href="protocol_treasury.md#0x0_protocol_treasury">protocol_treasury</a>: <a href="protocol_treasury.md#0x0_protocol_treasury_ProtocolTreasury">protocol_treasury::ProtocolTreasury</a>, authorized_protocol_cap_ids: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="system.md#0x0_system_create">create</a>(
    package_id: ID,
    upgrade_caps: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;UpgradeCap&gt;,
    validators: ValidatorSet,
    protocol_version: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    epoch_start_timestamp_ms: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    parameters: SystemParametersV1,
    <a href="protocol_treasury.md#0x0_protocol_treasury">protocol_treasury</a>: ProtocolTreasury,
    authorized_protocol_cap_ids: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;ID&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> system_state = <a href="system_inner.md#0x0_system_inner_v1_create">system_inner_v1::create</a>(
        upgrade_caps,
        validators,
        protocol_version,
        epoch_start_timestamp_ms,
        parameters,
        <a href="protocol_treasury.md#0x0_protocol_treasury">protocol_treasury</a>,
        authorized_protocol_cap_ids,
        ctx,
    );
    <b>let</b> <b>mut</b> self = <a href="system.md#0x0_system_System">System</a> {
        id: <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx),
        version: <a href="system.md#0x0_system_VERSION">VERSION</a>,
        package_id,
        new_package_id: <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(),
    };
    <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_add">dynamic_field::add</a>(&<b>mut</b> self.id, <a href="system.md#0x0_system_VERSION">VERSION</a>, system_state);
    <a href="../sui-framework/transfer.md#0x2_transfer_share_object">transfer::share_object</a>(self);
}
</code></pre>



</details>

<a name="0x0_system_initialize"></a>

## Function `initialize`



<pre><code><b>public</b> <b>fun</b> <a href="system.md#0x0_system_initialize">initialize</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="system.md#0x0_system_initialize">initialize</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &Clock,
) {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_initialize">initialize</a>(<a href="../sui-framework/clock.md#0x2_clock">clock</a>);
}
</code></pre>



</details>

<a name="0x0_system_request_add_validator_candidate"></a>

## Function `request_add_validator_candidate`

Can be called by anyone who wishes to become a validator candidate and starts accruing delegated
stakes in their staking pool. Once they have at least <code>MIN_VALIDATOR_JOINING_STAKE</code> amount of stake they
can call <code>request_add_validator</code> to officially become an active validator at the next epoch.
Aborts if the caller is already a pending or active validator, or a validator candidate.
Note: <code>proof_of_possession_bytes</code> MUST be a valid signature using sui_address and protocol_pubkey_bytes.
To produce a valid PoP, run [fn test_proof_of_possession_bytes].


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_request_add_validator_candidate">request_add_validator_candidate</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, network_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, consensus_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, proof_of_possession_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, name: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, description: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, image_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, project_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, network_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, p2p_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, consensus_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, computation_price: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, commission_rate: u16, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_request_add_validator_candidate">request_add_validator_candidate</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
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
) {
    <b>let</b> (cap, operation_cap) = self.<a href="system.md#0x0_system_request_add_validator_candidate_non_entry">request_add_validator_candidate_non_entry</a>(
        ctx.sender(),
        pubkey_bytes,
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
    <a href="../sui-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(cap, ctx.sender());
    <a href="../sui-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(operation_cap, ctx.sender());
}
</code></pre>



</details>

<a name="0x0_system_request_add_validator_candidate_non_entry"></a>

## Function `request_add_validator_candidate_non_entry`



<pre><code><b>public</b> <b>fun</b> <a href="system.md#0x0_system_request_add_validator_candidate_non_entry">request_add_validator_candidate_non_entry</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, payment_address: <b>address</b>, protocol_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, network_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, consensus_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, proof_of_possession_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, name: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, description: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, image_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, project_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, network_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, p2p_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, consensus_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, computation_price: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, commission_rate: u16, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): (<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, <a href="validator_cap.md#0x0_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="system.md#0x0_system_request_add_validator_candidate_non_entry">request_add_validator_candidate_non_entry</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
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
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_request_add_validator_candidate">request_add_validator_candidate</a>(
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

<a name="0x0_system_request_remove_validator_candidate"></a>

## Function `request_remove_validator_candidate`

Called by a validator candidate to remove themselves from the candidacy. After this call
their staking pool becomes deactivate.


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_request_remove_validator_candidate">request_remove_validator_candidate</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_request_remove_validator_candidate">request_remove_validator_candidate</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_request_remove_validator_candidate">request_remove_validator_candidate</a>(cap)
}
</code></pre>



</details>

<a name="0x0_system_request_add_validator"></a>

## Function `request_add_validator`

Called by a validator candidate to add themselves to the active validator set beginning next epoch.
Aborts if the validator is a duplicate with one of the pending or active validators, or if the amount of
stake the validator has doesn't meet the min threshold, or if the number of new validators for the next
epoch has already reached the maximum.


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_request_add_validator">request_add_validator</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_request_add_validator">request_add_validator</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_request_add_validator">request_add_validator</a>(cap)
}
</code></pre>



</details>

<a name="0x0_system_request_remove_validator"></a>

## Function `request_remove_validator`

A validator can call this function to request a removal in the next epoch.
We use the sender of <code>ctx</code> to look up the validator
(i.e. sender must match the sui_address in the validator).
At the end of the epoch, the <code><a href="validator.md#0x0_validator">validator</a></code> object will be returned to the sui_address
of the validator.


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_request_remove_validator">request_remove_validator</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_request_remove_validator">request_remove_validator</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>, cap: &ValidatorCap) {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_request_remove_validator">request_remove_validator</a>(cap)
}
</code></pre>



</details>

<a name="0x0_system_request_set_computation_price"></a>

## Function `request_set_computation_price`

A validator can call this entry function to submit a new computation price quote, to be
used for the computation price per unit size calculation at the end of the epoch.


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_request_set_computation_price">request_set_computation_price</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, new_computation_price: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_request_set_computation_price">request_set_computation_price</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    cap: &ValidatorOperationCap,
    new_computation_price: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
) {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_request_set_computation_price">request_set_computation_price</a>(cap, new_computation_price)
}
</code></pre>



</details>

<a name="0x0_system_set_candidate_validator_computation_price"></a>

## Function `set_candidate_validator_computation_price`

This entry function is used to set new computation price for candidate validators


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_set_candidate_validator_computation_price">set_candidate_validator_computation_price</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, new_computation_price: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_set_candidate_validator_computation_price">set_candidate_validator_computation_price</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    cap: &ValidatorOperationCap,
    new_computation_price: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
) {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_set_candidate_validator_computation_price">set_candidate_validator_computation_price</a>(cap, new_computation_price)
}
</code></pre>



</details>

<a name="0x0_system_request_set_commission_rate"></a>

## Function `request_set_commission_rate`

A validator can call this entry function to set a new commission rate, updated at the end of
the epoch.


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_request_set_commission_rate">request_set_commission_rate</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, new_commission_rate: u16, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_request_set_commission_rate">request_set_commission_rate</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    new_commission_rate: u16,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_request_set_commission_rate">request_set_commission_rate</a>(new_commission_rate, cap)
}
</code></pre>



</details>

<a name="0x0_system_set_candidate_validator_commission_rate"></a>

## Function `set_candidate_validator_commission_rate`

This entry function is used to set new commission rate for candidate validators


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_set_candidate_validator_commission_rate">set_candidate_validator_commission_rate</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, new_commission_rate: u16, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_set_candidate_validator_commission_rate">set_candidate_validator_commission_rate</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    new_commission_rate: u16,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_set_candidate_validator_commission_rate">set_candidate_validator_commission_rate</a>(new_commission_rate, cap)
}
</code></pre>



</details>

<a name="0x0_system_request_add_stake"></a>

## Function `request_add_stake`

Add stake to a validator's staking pool.


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_request_add_stake">request_add_stake</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, stake: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;, validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_request_add_stake">request_add_stake</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    stake: Coin&lt;IKA&gt;,
    validator_id: ID,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> <a href="staked_ika.md#0x0_staked_ika">staked_ika</a> = self.<a href="system.md#0x0_system_request_add_stake_non_entry">request_add_stake_non_entry</a>(stake, validator_id, ctx);
    <a href="../sui-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(<a href="staked_ika.md#0x0_staked_ika">staked_ika</a>, ctx.sender());
}
</code></pre>



</details>

<a name="0x0_system_request_add_stake_non_entry"></a>

## Function `request_add_stake_non_entry`

The non-entry version of <code>request_add_stake</code>, which returns the staked IKA instead of transferring it to the sender.


<pre><code><b>public</b> <b>fun</b> <a href="system.md#0x0_system_request_add_stake_non_entry">request_add_stake_non_entry</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, stake: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;, validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="staked_ika.md#0x0_staked_ika_StakedIka">staked_ika::StakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="system.md#0x0_system_request_add_stake_non_entry">request_add_stake_non_entry</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    stake: Coin&lt;IKA&gt;,
    validator_id: ID,
    ctx: &<b>mut</b> TxContext,
): StakedIka {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_request_add_stake">request_add_stake</a>(stake, validator_id, ctx)
}
</code></pre>



</details>

<a name="0x0_system_request_add_stake_mul_coin"></a>

## Function `request_add_stake_mul_coin`

Add stake to a validator's staking pool using multiple coins.


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_request_add_stake_mul_coin">request_add_stake_mul_coin</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, stakes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;&gt;, stake_amount: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;, validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_request_add_stake_mul_coin">request_add_stake_mul_coin</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    stakes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;Coin&lt;IKA&gt;&gt;,
    stake_amount: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;,
    validator_id: ID,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    <b>let</b> <a href="staked_ika.md#0x0_staked_ika">staked_ika</a> = self.<a href="system.md#0x0_system_request_add_stake_mul_coin">request_add_stake_mul_coin</a>(stakes, stake_amount, validator_id, ctx);
    <a href="../sui-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(<a href="staked_ika.md#0x0_staked_ika">staked_ika</a>, ctx.sender());
}
</code></pre>



</details>

<a name="0x0_system_request_withdraw_stake"></a>

## Function `request_withdraw_stake`

Withdraw stake from a validator's staking pool.


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_request_withdraw_stake">request_withdraw_stake</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: <a href="staked_ika.md#0x0_staked_ika_StakedIka">staked_ika::StakedIka</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_request_withdraw_stake">request_withdraw_stake</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: StakedIka,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> withdrawn_stake = self.<a href="system.md#0x0_system_request_withdraw_stake_non_entry">request_withdraw_stake_non_entry</a>(<a href="staked_ika.md#0x0_staked_ika">staked_ika</a>);
    <a href="../sui-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(withdrawn_stake.into_coin(ctx), ctx.sender());
}
</code></pre>



</details>

<a name="0x0_system_convert_to_fungible_staked_ika"></a>

## Function `convert_to_fungible_staked_ika`

Convert StakedIka into a FungibleStakedIka object.


<pre><code><b>public</b> <b>fun</b> <a href="system.md#0x0_system_convert_to_fungible_staked_ika">convert_to_fungible_staked_ika</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: <a href="staked_ika.md#0x0_staked_ika_StakedIka">staked_ika::StakedIka</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="system.md#0x0_system_convert_to_fungible_staked_ika">convert_to_fungible_staked_ika</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: StakedIka,
    ctx: &<b>mut</b> TxContext,
): FungibleStakedIka {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_convert_to_fungible_staked_ika">convert_to_fungible_staked_ika</a>(<a href="staked_ika.md#0x0_staked_ika">staked_ika</a>, ctx)
}
</code></pre>



</details>

<a name="0x0_system_redeem_fungible_staked_ika"></a>

## Function `redeem_fungible_staked_ika`

Convert FungibleStakedIka into a StakedIka object.


<pre><code><b>public</b> <b>fun</b> <a href="system.md#0x0_system_redeem_fungible_staked_ika">redeem_fungible_staked_ika</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, fungible_staked_ika: <a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>): <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="system.md#0x0_system_redeem_fungible_staked_ika">redeem_fungible_staked_ika</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    fungible_staked_ika: FungibleStakedIka,
): Balance&lt;IKA&gt; {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_redeem_fungible_staked_ika">redeem_fungible_staked_ika</a>(fungible_staked_ika)
}
</code></pre>



</details>

<a name="0x0_system_request_withdraw_stake_non_entry"></a>

## Function `request_withdraw_stake_non_entry`

Non-entry version of <code>request_withdraw_stake</code> that returns the withdrawn IKA instead of transferring it to the sender.


<pre><code><b>public</b> <b>fun</b> <a href="system.md#0x0_system_request_withdraw_stake_non_entry">request_withdraw_stake_non_entry</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: <a href="staked_ika.md#0x0_staked_ika_StakedIka">staked_ika::StakedIka</a>): <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="system.md#0x0_system_request_withdraw_stake_non_entry">request_withdraw_stake_non_entry</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: StakedIka,
): Balance&lt;IKA&gt; {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_request_withdraw_stake">request_withdraw_stake</a>(<a href="staked_ika.md#0x0_staked_ika">staked_ika</a>)
}
</code></pre>



</details>

<a name="0x0_system_report_validator"></a>

## Function `report_validator`

Report a validator as a bad or non-performant actor in the system.
Succeeds if all the following are satisfied:
1. both the reporter in <code>cap</code> and the input <code>reportee_id</code> are active validators.
2. reporter and reportee not the same address.
3. the cap object is still valid.
This function is idempotent.


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_report_validator">report_validator</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, reportee_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_report_validator">report_validator</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    cap: &ValidatorOperationCap,
    reportee_id: ID,
) {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_report_validator">report_validator</a>(cap, reportee_id)
}
</code></pre>



</details>

<a name="0x0_system_undo_report_validator"></a>

## Function `undo_report_validator`

Undo a <code>report_validator</code> action. Aborts if
1. the reportee is not a currently active validator or
2. the sender has not previously reported the <code>reportee_id</code>, or
3. the cap is not valid


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_undo_report_validator">undo_report_validator</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, reportee_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_undo_report_validator">undo_report_validator</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    cap: &ValidatorOperationCap,
    reportee_id: ID,
) {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_undo_report_validator">undo_report_validator</a>(cap, reportee_id)
}
</code></pre>



</details>

<a name="0x0_system_rotate_operation_cap"></a>

## Function `rotate_operation_cap`

Create a new <code>ValidatorOperationCap</code>, transfer it to the
validator and registers it. The original object is thus revoked.


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_rotate_operation_cap">rotate_operation_cap</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_rotate_operation_cap">rotate_operation_cap</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>, cap: &ValidatorCap, ctx: &<b>mut</b> TxContext) {
    <b>let</b> operation_cap = self.<a href="system.md#0x0_system_rotate_operation_cap_non_entry">rotate_operation_cap_non_entry</a>(cap, ctx);
    <a href="../sui-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(operation_cap, ctx.sender());
}
</code></pre>



</details>

<a name="0x0_system_rotate_operation_cap_non_entry"></a>

## Function `rotate_operation_cap_non_entry`

Create a new <code>ValidatorOperationCap</code> and registers it. The original object is thus revoked.


<pre><code><b>public</b> <b>fun</b> <a href="system.md#0x0_system_rotate_operation_cap_non_entry">rotate_operation_cap_non_entry</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="validator_cap.md#0x0_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="system.md#0x0_system_rotate_operation_cap_non_entry">rotate_operation_cap_non_entry</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>, cap: &ValidatorCap, ctx: &<b>mut</b> TxContext): ValidatorOperationCap {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_rotate_operation_cap">rotate_operation_cap</a>(cap, ctx)
}
</code></pre>



</details>

<a name="0x0_system_update_validator_payment_address"></a>

## Function `update_validator_payment_address`

Update a validator's payment address.


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_validator_payment_address">update_validator_payment_address</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, payment_address: <b>address</b>, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_validator_payment_address">update_validator_payment_address</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    payment_address: <b>address</b>,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_update_validator_payment_address">update_validator_payment_address</a>(payment_address, cap)
}
</code></pre>



</details>

<a name="0x0_system_update_validator_name"></a>

## Function `update_validator_name`

Update a validator's name.


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_validator_name">update_validator_name</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, name: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_validator_name">update_validator_name</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    name: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_update_validator_name">update_validator_name</a>(name, cap)
}
</code></pre>



</details>

<a name="0x0_system_update_validator_description"></a>

## Function `update_validator_description`

Update a validator's description


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_validator_description">update_validator_description</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, description: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_validator_description">update_validator_description</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    description: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_update_validator_description">update_validator_description</a>(description, cap)
}
</code></pre>



</details>

<a name="0x0_system_update_validator_image_url"></a>

## Function `update_validator_image_url`

Update a validator's image url


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_validator_image_url">update_validator_image_url</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, image_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_validator_image_url">update_validator_image_url</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    image_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_update_validator_image_url">update_validator_image_url</a>(image_url, cap)
}
</code></pre>



</details>

<a name="0x0_system_update_validator_project_url"></a>

## Function `update_validator_project_url`

Update a validator's project url


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_validator_project_url">update_validator_project_url</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, project_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_validator_project_url">update_validator_project_url</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    project_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_update_validator_project_url">update_validator_project_url</a>(project_url, cap)
}
</code></pre>



</details>

<a name="0x0_system_update_validator_next_epoch_network_address"></a>

## Function `update_validator_next_epoch_network_address`

Update a validator's network address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_validator_next_epoch_network_address">update_validator_next_epoch_network_address</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, network_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_validator_next_epoch_network_address">update_validator_next_epoch_network_address</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    network_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_update_validator_next_epoch_network_address">update_validator_next_epoch_network_address</a>(network_address, cap)
}
</code></pre>



</details>

<a name="0x0_system_update_candidate_validator_network_address"></a>

## Function `update_candidate_validator_network_address`

Update candidate validator's network address.


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_candidate_validator_network_address">update_candidate_validator_network_address</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, network_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_candidate_validator_network_address">update_candidate_validator_network_address</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    network_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_update_candidate_validator_network_address">update_candidate_validator_network_address</a>(network_address, cap)
}
</code></pre>



</details>

<a name="0x0_system_update_validator_next_epoch_p2p_address"></a>

## Function `update_validator_next_epoch_p2p_address`

Update a validator's p2p address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_validator_next_epoch_p2p_address">update_validator_next_epoch_p2p_address</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, p2p_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_validator_next_epoch_p2p_address">update_validator_next_epoch_p2p_address</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    p2p_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_update_validator_next_epoch_p2p_address">update_validator_next_epoch_p2p_address</a>(p2p_address, cap)
}
</code></pre>



</details>

<a name="0x0_system_update_candidate_validator_p2p_address"></a>

## Function `update_candidate_validator_p2p_address`

Update candidate validator's p2p address.


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_candidate_validator_p2p_address">update_candidate_validator_p2p_address</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, p2p_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_candidate_validator_p2p_address">update_candidate_validator_p2p_address</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    p2p_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_update_candidate_validator_p2p_address">update_candidate_validator_p2p_address</a>(p2p_address, cap)
}
</code></pre>



</details>

<a name="0x0_system_update_validator_next_epoch_consensus_address"></a>

## Function `update_validator_next_epoch_consensus_address`

Update a validator's consensus address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_validator_next_epoch_consensus_address">update_validator_next_epoch_consensus_address</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, consensus_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_validator_next_epoch_consensus_address">update_validator_next_epoch_consensus_address</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    consensus_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_update_validator_next_epoch_consensus_address">update_validator_next_epoch_consensus_address</a>(consensus_address, cap)
}
</code></pre>



</details>

<a name="0x0_system_update_candidate_validator_consensus_address"></a>

## Function `update_candidate_validator_consensus_address`

Update candidate validator's consensus address.


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_candidate_validator_consensus_address">update_candidate_validator_consensus_address</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, consensus_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_candidate_validator_consensus_address">update_candidate_validator_consensus_address</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    consensus_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_update_candidate_validator_consensus_address">update_candidate_validator_consensus_address</a>(consensus_address, cap)
}
</code></pre>



</details>

<a name="0x0_system_update_validator_next_epoch_protocol_pubkey_bytes"></a>

## Function `update_validator_next_epoch_protocol_pubkey_bytes`

Update a validator's public key of protocol key and proof of possession.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_validator_next_epoch_protocol_pubkey_bytes">update_validator_next_epoch_protocol_pubkey_bytes</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, protocol_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, proof_of_possession_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, ctx: &<a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_validator_next_epoch_protocol_pubkey_bytes">update_validator_next_epoch_protocol_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    protocol_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    proof_of_possession_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap,
    ctx: &TxContext,
) {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_update_validator_next_epoch_protocol_pubkey_bytes">update_validator_next_epoch_protocol_pubkey_bytes</a>(protocol_pubkey, proof_of_possession_bytes, cap, ctx)
}
</code></pre>



</details>

<a name="0x0_system_update_candidate_validator_protocol_pubkey_bytes"></a>

## Function `update_candidate_validator_protocol_pubkey_bytes`

Update candidate validator's public key of protocol key and proof of possession.


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_candidate_validator_protocol_pubkey_bytes">update_candidate_validator_protocol_pubkey_bytes</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, protocol_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, proof_of_possession_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, ctx: &<a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_candidate_validator_protocol_pubkey_bytes">update_candidate_validator_protocol_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    protocol_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    proof_of_possession_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap,
    ctx: &TxContext,
) {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_update_candidate_validator_protocol_pubkey_bytes">update_candidate_validator_protocol_pubkey_bytes</a>(protocol_pubkey, proof_of_possession_bytes, cap, ctx)
}
</code></pre>



</details>

<a name="0x0_system_update_validator_next_epoch_consensus_pubkey_bytes"></a>

## Function `update_validator_next_epoch_consensus_pubkey_bytes`

Update a validator's public key of worker key.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_validator_next_epoch_consensus_pubkey_bytes">update_validator_next_epoch_consensus_pubkey_bytes</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, consensus_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_validator_next_epoch_consensus_pubkey_bytes">update_validator_next_epoch_consensus_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    consensus_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_update_validator_next_epoch_consensus_pubkey_bytes">update_validator_next_epoch_consensus_pubkey_bytes</a>(consensus_pubkey_bytes, cap)
}
</code></pre>



</details>

<a name="0x0_system_update_candidate_validator_consensus_pubkey_bytes"></a>

## Function `update_candidate_validator_consensus_pubkey_bytes`

Update candidate validator's public key of worker key.


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_candidate_validator_consensus_pubkey_bytes">update_candidate_validator_consensus_pubkey_bytes</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, consensus_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_candidate_validator_consensus_pubkey_bytes">update_candidate_validator_consensus_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    consensus_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_update_candidate_validator_consensus_pubkey_bytes">update_candidate_validator_consensus_pubkey_bytes</a>(consensus_pubkey_bytes, cap)
}
</code></pre>



</details>

<a name="0x0_system_update_validator_next_epoch_network_pubkey_bytes"></a>

## Function `update_validator_next_epoch_network_pubkey_bytes`

Update a validator's public key of network key.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_validator_next_epoch_network_pubkey_bytes">update_validator_next_epoch_network_pubkey_bytes</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, network_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_validator_next_epoch_network_pubkey_bytes">update_validator_next_epoch_network_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    network_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_update_validator_next_epoch_network_pubkey_bytes">update_validator_next_epoch_network_pubkey_bytes</a>(network_pubkey, cap)
}
</code></pre>



</details>

<a name="0x0_system_update_candidate_validator_network_pubkey_bytes"></a>

## Function `update_candidate_validator_network_pubkey_bytes`

Update candidate validator's public key of network key.


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_candidate_validator_network_pubkey_bytes">update_candidate_validator_network_pubkey_bytes</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, network_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="system.md#0x0_system_update_candidate_validator_network_pubkey_bytes">update_candidate_validator_network_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    network_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    cap: &ValidatorCap,
) {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_update_candidate_validator_network_pubkey_bytes">update_candidate_validator_network_pubkey_bytes</a>(network_pubkey, cap)
}
</code></pre>



</details>

<a name="0x0_system_pool_exchange_rates"></a>

## Function `pool_exchange_rates`

Getter of the pool token exchange rate of a validator. Works for both active and inactive pools.


<pre><code><b>public</b> <b>fun</b> <a href="system.md#0x0_system_pool_exchange_rates">pool_exchange_rates</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>): &<a href="../sui-framework/table.md#0x2_table_Table">table::Table</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="staking_pool.md#0x0_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="system.md#0x0_system_pool_exchange_rates">pool_exchange_rates</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    validator_id: ID,
): &Table&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, PoolTokenExchangeRate&gt; {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_pool_exchange_rates">pool_exchange_rates</a>(validator_id)
}
</code></pre>



</details>

<a name="0x0_system_active_committee"></a>

## Function `active_committee`

Getter returning ids of the currently active validators.


<pre><code><b>public</b> <b>fun</b> <a href="system.md#0x0_system_active_committee">active_committee</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>): <a href="committee.md#0x0_committee_Committee">committee::Committee</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="system.md#0x0_system_active_committee">active_committee</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>): Committee {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner">inner</a>();
    self.<a href="system.md#0x0_system_active_committee">active_committee</a>()
}
</code></pre>



</details>

<a name="0x0_system_process_checkpoint_message_by_cap"></a>

## Function `process_checkpoint_message_by_cap`



<pre><code><b>public</b> <b>fun</b> <a href="system.md#0x0_system_process_checkpoint_message_by_cap">process_checkpoint_message_by_cap</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, cap: &<a href="protocol_cap.md#0x0_protocol_cap_ProtocolCap">protocol_cap::ProtocolCap</a>, message: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="system.md#0x0_system_process_checkpoint_message_by_cap">process_checkpoint_message_by_cap</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    cap: &ProtocolCap,
    message: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_process_checkpoint_message_by_cap">process_checkpoint_message_by_cap</a>(cap, message, ctx);
}
</code></pre>



</details>

<a name="0x0_system_process_checkpoint_message_by_quorum"></a>

## Function `process_checkpoint_message_by_quorum`



<pre><code><b>public</b> <b>fun</b> <a href="system.md#0x0_system_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, signature: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, signers_bitmap: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, message: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="system.md#0x0_system_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    signature: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    signers_bitmap: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    message: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(signature, signers_bitmap, message, ctx);
}
</code></pre>



</details>

<a name="0x0_system_authorize_update_message_by_cap"></a>

## Function `authorize_update_message_by_cap`



<pre><code><b>public</b> <b>fun</b> <a href="system.md#0x0_system_authorize_update_message_by_cap">authorize_update_message_by_cap</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, cap: &<a href="protocol_cap.md#0x0_protocol_cap_ProtocolCap">protocol_cap::ProtocolCap</a>, package_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, digest: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../sui-framework/package.md#0x2_package_UpgradeTicket">package::UpgradeTicket</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="system.md#0x0_system_authorize_update_message_by_cap">authorize_update_message_by_cap</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    cap: &ProtocolCap,
    package_id: ID,
    digest: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
): UpgradeTicket {
    <b>let</b> self = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>();
    self.<a href="system.md#0x0_system_authorize_update_message_by_cap">authorize_update_message_by_cap</a>(cap, package_id, digest)
}
</code></pre>



</details>

<a name="0x0_system_commit_upgrade"></a>

## Function `commit_upgrade`



<pre><code><b>public</b> <b>fun</b> <a href="system.md#0x0_system_commit_upgrade">commit_upgrade</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>, receipt: <a href="../sui-framework/package.md#0x2_package_UpgradeReceipt">package::UpgradeReceipt</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="system.md#0x0_system_commit_upgrade">commit_upgrade</a>(
    self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
    receipt: UpgradeReceipt,
) {
    <b>let</b> new_package_id = receipt.<a href="../sui-framework/package.md#0x2_package">package</a>();
    <b>let</b> old_package_id = self.<a href="system.md#0x0_system_inner_mut">inner_mut</a>().<a href="system.md#0x0_system_commit_upgrade">commit_upgrade</a>(receipt);
    <b>if</b> (self.package_id == old_package_id) {
        self.new_package_id = <a href="../move-stdlib/option.md#0x1_option_some">option::some</a>(new_package_id);
    }
}
</code></pre>



</details>

<a name="0x0_system_migrate"></a>

## Function `migrate`

Migrate the staking object to the new package id.

This function sets the new package id and version and can be modified in future versions
to migrate changes in the <code>system_inner</code> object if needed.


<pre><code><b>public</b> <b>fun</b> <a href="system.md#0x0_system_migrate">migrate</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="system.md#0x0_system_migrate">migrate</a>(
        self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>,
) {
    <b>assert</b>!(self.version &lt; <a href="system.md#0x0_system_VERSION">VERSION</a>, <a href="system.md#0x0_system_EInvalidMigration">EInvalidMigration</a>);

    // Move the <b>old</b> <a href="system.md#0x0_system">system</a> state inner <b>to</b> the new version.
    <b>let</b> system_inner: SystemInnerV1 = <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_remove">dynamic_field::remove</a>(&<b>mut</b> self.id, self.version);
    <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_add">dynamic_field::add</a>(&<b>mut</b> self.id, <a href="system.md#0x0_system_VERSION">VERSION</a>, system_inner);
    self.version = <a href="system.md#0x0_system_VERSION">VERSION</a>;

    // Set the new <a href="../sui-framework/package.md#0x2_package">package</a> id.
    <b>assert</b>!(self.new_package_id.is_some(), <a href="system.md#0x0_system_EInvalidMigration">EInvalidMigration</a>);
    self.package_id = self.new_package_id.extract();
}
</code></pre>



</details>

<a name="0x0_system_inner_mut"></a>

## Function `inner_mut`

Get a mutable reference to <code>SystemInnerVX</code> from the <code><a href="system.md#0x0_system_System">System</a></code>.


<pre><code><b>fun</b> <a href="system.md#0x0_system_inner_mut">inner_mut</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">system::System</a>): &<b>mut</b> <a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="system.md#0x0_system_inner_mut">inner_mut</a>(self: &<b>mut</b> <a href="system.md#0x0_system_System">System</a>): &<b>mut</b> SystemInnerV1 {
    <b>assert</b>!(self.version == <a href="system.md#0x0_system_VERSION">VERSION</a>, <a href="system.md#0x0_system_EWrongInnerVersion">EWrongInnerVersion</a>);
    <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_borrow_mut">dynamic_field::borrow_mut</a>(&<b>mut</b> self.id, <a href="system.md#0x0_system_VERSION">VERSION</a>)
}
</code></pre>



</details>

<a name="0x0_system_inner"></a>

## Function `inner`

Get an immutable reference to <code>SystemInnerVX</code> from the <code><a href="system.md#0x0_system_System">System</a></code>.


<pre><code><b>fun</b> <a href="system.md#0x0_system_inner">inner</a>(self: &<a href="system.md#0x0_system_System">system::System</a>): &<a href="system_inner.md#0x0_system_inner_v1_SystemInnerV1">system_inner_v1::SystemInnerV1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="system.md#0x0_system_inner">inner</a>(self: &<a href="system.md#0x0_system_System">System</a>): &SystemInnerV1 {
    <b>assert</b>!(self.version == <a href="system.md#0x0_system_VERSION">VERSION</a>, <a href="system.md#0x0_system_EWrongInnerVersion">EWrongInnerVersion</a>);
    <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_borrow">dynamic_field::borrow</a>(&self.id, <a href="system.md#0x0_system_VERSION">VERSION</a>)
}
</code></pre>



</details>
