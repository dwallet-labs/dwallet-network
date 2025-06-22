---
title: Module `(ika_system=0x0)::system`
---


<a name="@Ika_System_Module_0"></a>

## Ika System Module


The <code><a href="../ika_system/system.md#(ika_system=0x0)_system">ika_system::system</a></code> module provides the core system state management for the Ika network.
It acts as the central coordinator for validators, staking, epochs, and network governance.


<a name="@Overview_1"></a>

### Overview


The module implements a versioned wrapper pattern around the core system state:
- <code><a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a></code>: A shared object that serves as the public interface and version manager
- <code>SystemInner</code>: The actual system state implementation containing all business logic
- <code>ProtocolCap</code>: Capability object for privileged system operations


<a name="@Architecture_2"></a>

### Architecture


The system uses a two-layer architecture:


<a name="@System_Wrapper_Layer_3"></a>

#### System Wrapper Layer

The <code><a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a></code> struct is a thin wrapper that:
- Maintains version information for upgrades
- Stores the package ID for authorization
- Holds the inner system state as a dynamic field
- Provides a stable public interface across versions


<a name="@SystemInner_Layer_4"></a>

#### SystemInner Layer

The <code>SystemInner</code> struct contains all the core functionality:
- Validator set management and operations
- Epoch progression and timing
- Staking and delegation logic
- Protocol treasury and rewards distribution
- dWallet network coordination
- System parameter management


<a name="@Key_Responsibilities_5"></a>

### Key Responsibilities



<a name="@Validator_Management_6"></a>

#### Validator Management

- Adding/removing validator candidates
- Managing validator metadata and configuration
- Handling validator state transitions (PreActive → Active → Withdrawing)
- Processing validator reports and governance actions


<a name="@Staking_Operations_7"></a>

#### Staking Operations

- Processing stake additions and withdrawals
- Managing staked IKA tokens and rewards
- Calculating token exchange rates across epochs
- Handling delegation to validators


<a name="@Epoch_Management_8"></a>

#### Epoch Management

- Coordinating epoch transitions
- Processing mid-epoch reconfigurations
- Managing epoch timing and duration
- Distributing stake subsidies and rewards


<a name="@dWallet_Integration_9"></a>

#### dWallet Integration

- Coordinating with dWallet 2PC MPC system
- Managing encryption keys and DKG processes
- Handling pricing and curve configurations
- Processing dWallet network operations


<a name="@System_Governance_10"></a>

#### System Governance

- Managing protocol upgrades via UpgradeCap
- Processing system parameter changes
- Handling protocol version transitions
- Coordinating checkpoint message processing


<a name="@State_Management_11"></a>

### State Management


The system maintains state across multiple components:
- **ValidatorSet**: Current and pending validator configurations
- **ProtocolTreasury**: Rewards, subsidies, and fee management
- **BLS Committee**: Cryptographic committee for consensus
- **Token Exchange Rates**: Historical staking reward calculations
- **Pending Values**: Future epoch configuration changes


<a name="@Ika_System_Upgrade_Guide_12"></a>

### Ika System Upgrade Guide

<code><a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a></code> is a versioned wrapper around <code>SystemInner</code> that provides upgrade capabilities.
The <code>SystemInner</code> object is stored as a dynamic field with the version as the key.
There are multiple approaches to upgrade the system state:

The simplest approach is to add dynamic fields to the <code>extra_fields</code> field of <code>SystemInner</code>
or any of its subtypes. This is useful for rapid changes, small modifications, or experimental features.

To perform a proper type upgrade of <code>SystemInner</code>, follow these steps:
1. Define a new <code>SystemInnerV2</code> type in system_inner.move.
2. Create a data migration function that transforms <code>SystemInner</code> to <code>SystemInnerV2</code>.
3. Update the <code><a href="../ika_system/system.md#(ika_system=0x0)_system_VERSION">VERSION</a></code> constant to 2 and replace all references to <code>SystemInner</code> with <code>SystemInnerV2</code>
in both system.move and system_inner.move.
4. Modify the <code><a href="../ika_system/system.md#(ika_system=0x0)_system_migrate">migrate</a></code> function to handle the version upgrade by:
- Removing the old inner object from the dynamic field
- Applying the data migration transformation
- Adding the new inner object with the updated version
5. Update the <code><a href="../ika_system/system.md#(ika_system=0x0)_system_inner">inner</a>()</code> and <code><a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>()</code> functions to work with the new version.

Along with the Move changes, update the Rust code:
1. Define a new <code>SystemInnerV2</code> struct that matches the Move type.
2. Update the <code><a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a></code> enum to include the new version variant.
3. Update relevant system state getter functions to handle the new version.

To upgrade Validator types:
1. Define a new Validator version (e.g. ValidatorV2) in validator.move.
2. Create migration functions to convert between validator versions.
3. Update validator creation and access functions to use the new version.
4. Update the validator set and related components to handle the new validator type.

In Rust, add new cases to handle the upgraded validator types in the appropriate getter functions.
Validator upgrades can be done independently of SystemInner upgrades, but ensure version consistency
across related components.


-  [Ika System Module](#@Ika_System_Module_0)
    -  [Overview](#@Overview_1)
    -  [Architecture](#@Architecture_2)
        -  [System Wrapper Layer](#@System_Wrapper_Layer_3)
        -  [SystemInner Layer](#@SystemInner_Layer_4)
    -  [Key Responsibilities](#@Key_Responsibilities_5)
        -  [Validator Management](#@Validator_Management_6)
        -  [Staking Operations](#@Staking_Operations_7)
        -  [Epoch Management](#@Epoch_Management_8)
        -  [dWallet Integration](#@dWallet_Integration_9)
        -  [System Governance](#@System_Governance_10)
    -  [State Management](#@State_Management_11)
    -  [Ika System Upgrade Guide](#@Ika_System_Upgrade_Guide_12)
-  [Struct `System`](#(ika_system=0x0)_system_System)
    -  [Fields](#@Fields_13)
    -  [Design Notes](#@Design_Notes_14)
    -  [Access Pattern](#@Access_Pattern_15)
-  [Constants](#@Constants_16)
-  [Function `create`](#(ika_system=0x0)_system_create)
-  [Function `initialize`](#(ika_system=0x0)_system_initialize)
-  [Function `request_add_validator_candidate`](#(ika_system=0x0)_system_request_add_validator_candidate)
-  [Function `request_remove_validator_candidate`](#(ika_system=0x0)_system_request_remove_validator_candidate)
-  [Function `request_add_validator`](#(ika_system=0x0)_system_request_add_validator)
-  [Function `request_remove_validator`](#(ika_system=0x0)_system_request_remove_validator)
-  [Function `set_next_commission`](#(ika_system=0x0)_system_set_next_commission)
-  [Function `request_add_stake`](#(ika_system=0x0)_system_request_add_stake)
-  [Function `request_withdraw_stake`](#(ika_system=0x0)_system_request_withdraw_stake)
-  [Function `withdraw_stake`](#(ika_system=0x0)_system_withdraw_stake)
-  [Function `report_validator`](#(ika_system=0x0)_system_report_validator)
-  [Function `undo_report_validator`](#(ika_system=0x0)_system_undo_report_validator)
-  [Function `rotate_operation_cap`](#(ika_system=0x0)_system_rotate_operation_cap)
-  [Function `rotate_commission_cap`](#(ika_system=0x0)_system_rotate_commission_cap)
-  [Function `collect_commission`](#(ika_system=0x0)_system_collect_commission)
-  [Function `set_validator_name`](#(ika_system=0x0)_system_set_validator_name)
-  [Function `validator_metadata`](#(ika_system=0x0)_system_validator_metadata)
-  [Function `set_validator_metadata`](#(ika_system=0x0)_system_set_validator_metadata)
-  [Function `set_next_epoch_network_address`](#(ika_system=0x0)_system_set_next_epoch_network_address)
-  [Function `set_next_epoch_p2p_address`](#(ika_system=0x0)_system_set_next_epoch_p2p_address)
-  [Function `set_next_epoch_consensus_address`](#(ika_system=0x0)_system_set_next_epoch_consensus_address)
-  [Function `set_next_epoch_protocol_pubkey_bytes`](#(ika_system=0x0)_system_set_next_epoch_protocol_pubkey_bytes)
-  [Function `set_next_epoch_network_pubkey_bytes`](#(ika_system=0x0)_system_set_next_epoch_network_pubkey_bytes)
-  [Function `set_next_epoch_consensus_pubkey_bytes`](#(ika_system=0x0)_system_set_next_epoch_consensus_pubkey_bytes)
-  [Function `set_next_epoch_class_groups_pubkey_and_proof_bytes`](#(ika_system=0x0)_system_set_next_epoch_class_groups_pubkey_and_proof_bytes)
-  [Function `set_pricing_vote`](#(ika_system=0x0)_system_set_pricing_vote)
-  [Function `token_exchange_rates`](#(ika_system=0x0)_system_token_exchange_rates)
-  [Function `active_committee`](#(ika_system=0x0)_system_active_committee)
-  [Function `next_epoch_active_committee`](#(ika_system=0x0)_system_next_epoch_active_committee)
-  [Function `request_reconfig_mid_epoch`](#(ika_system=0x0)_system_request_reconfig_mid_epoch)
-  [Function `request_lock_epoch_sessions`](#(ika_system=0x0)_system_request_lock_epoch_sessions)
-  [Function `request_advance_epoch`](#(ika_system=0x0)_system_request_advance_epoch)
-  [Function `request_dwallet_network_encryption_key_dkg_by_cap`](#(ika_system=0x0)_system_request_dwallet_network_encryption_key_dkg_by_cap)
-  [Function `set_supported_and_pricing`](#(ika_system=0x0)_system_set_supported_and_pricing)
-  [Function `set_paused_curves_and_signature_algorithms`](#(ika_system=0x0)_system_set_paused_curves_and_signature_algorithms)
-  [Function `authorize_upgrade_by_cap`](#(ika_system=0x0)_system_authorize_upgrade_by_cap)
-  [Function `authorize_upgrade_by_approval`](#(ika_system=0x0)_system_authorize_upgrade_by_approval)
-  [Function `commit_upgrade`](#(ika_system=0x0)_system_commit_upgrade)
-  [Function `process_checkpoint_message_by_cap`](#(ika_system=0x0)_system_process_checkpoint_message_by_cap)
-  [Function `process_checkpoint_message_by_quorum`](#(ika_system=0x0)_system_process_checkpoint_message_by_quorum)
-  [Function `migrate`](#(ika_system=0x0)_system_migrate)
-  [Function `calculate_rewards`](#(ika_system=0x0)_system_calculate_rewards)
-  [Function `can_withdraw_staked_ika_early`](#(ika_system=0x0)_system_can_withdraw_staked_ika_early)
-  [Function `dwallet_2pc_mpc_coordinator_network_encryption_key_ids`](#(ika_system=0x0)_system_dwallet_2pc_mpc_coordinator_network_encryption_key_ids)
-  [Function `inner_mut`](#(ika_system=0x0)_system_inner_mut)
-  [Function `inner`](#(ika_system=0x0)_system_inner)


<pre><code><b>use</b> (ika=0x0)::ika;
<b>use</b> (ika_system=0x0)::<b>address</b>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee">bls_committee</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof">class_groups_public_key_and_proof</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator">dwallet_2pc_mpc_coordinator</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner">dwallet_2pc_mpc_coordinator_inner</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing">dwallet_pricing</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/extended_field.md#(ika_system=0x0)_extended_field">extended_field</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/multiaddr.md#(ika_system=0x0)_multiaddr">multiaddr</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set">pending_active_set</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values">pending_values</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury">protocol_treasury</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner">system_inner</a>;
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
<b>use</b> <a href="../sui/ed25519.md#sui_ed25519">sui::ed25519</a>;
<b>use</b> <a href="../sui/event.md#sui_event">sui::event</a>;
<b>use</b> <a href="../sui/group_ops.md#sui_group_ops">sui::group_ops</a>;
<b>use</b> <a href="../sui/hash.md#sui_hash">sui::hash</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/object_table.md#sui_object_table">sui::object_table</a>;
<b>use</b> <a href="../sui/package.md#sui_package">sui::package</a>;
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



<a name="(ika_system=0x0)_system_System"></a>

## Struct `System`

The main system state object that coordinates the entire Ika network.

This is a shared object that acts as the central point for all system operations.
It maintains versioning information and delegates actual functionality to the
inner system state stored as a dynamic field.


<a name="@Fields_13"></a>

### Fields

- <code>id</code>: Unique identifier for this system object
- <code>version</code>: Current version of the inner system state structure
- <code>package_id</code>: ID of the current system package for upgrade authorization
- <code>new_package_id</code>: ID of the new package during upgrades (if any)


<a name="@Design_Notes_14"></a>

### Design Notes

The system uses dynamic fields to store the actual state, allowing for
type-safe upgrades while maintaining a stable object ID. The version field
ensures that operations are performed against the correct inner state type.


<a name="@Access_Pattern_15"></a>

### Access Pattern

All public functions delegate to <code><a href="../ika_system/system.md#(ika_system=0x0)_system_inner">inner</a>()</code> or <code><a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>()</code> which retrieve
the correctly versioned SystemInner from the dynamic field storage.


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

<a name="@Constants_16"></a>

## Constants


<a name="(ika_system=0x0)_system_EWrongInnerVersion"></a>

Attempted to access system inner with wrong version.


<pre><code><b>const</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_EWrongInnerVersion">EWrongInnerVersion</a>: u64 = 0;
</code></pre>



<a name="(ika_system=0x0)_system_EInvalidMigration"></a>

Invalid migration - either version not incremented or new_package_id not set.


<pre><code><b>const</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_EInvalidMigration">EInvalidMigration</a>: u64 = 1;
</code></pre>



<a name="(ika_system=0x0)_system_VERSION"></a>

Current version of the system state structure.
This version corresponds to SystemInner and should be incremented
when the inner system state structure changes requiring migration.

Version History:
- V1: Initial SystemInner implementation with core functionality


<pre><code><b>const</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_VERSION">VERSION</a>: u64 = 1;
</code></pre>



<a name="(ika_system=0x0)_system_create"></a>

## Function `create`

Create a new System object and make it shared.
This function will be called only once in init.

Creates the initial system state with the provided validators and parameters,
then wraps it in a versioned System object and makes it shared for network access.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_create">create</a>(package_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, upgrade_caps: vector&lt;<a href="../sui/package.md#sui_package_UpgradeCap">sui::package::UpgradeCap</a>&gt;, validators: (ika_system=0x0)::<a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_ValidatorSet">validator_set::ValidatorSet</a>, protocol_version: u64, epoch_start_timestamp_ms: u64, epoch_duration_ms: u64, stake_subsidy_start_epoch: u64, <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury">protocol_treasury</a>: (ika_system=0x0)::<a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_ProtocolTreasury">protocol_treasury::ProtocolTreasury</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_ProtocolCap">system_inner::ProtocolCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_create">create</a>(
    package_id: ID,
    upgrade_caps: vector&lt;UpgradeCap&gt;,
    validators: ValidatorSet,
    protocol_version: u64,
    epoch_start_timestamp_ms: u64,
    epoch_duration_ms: u64,
    stake_subsidy_start_epoch: u64,
    <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury">protocol_treasury</a>: ProtocolTreasury,
    ctx: &<b>mut</b> TxContext,
): ProtocolCap {
    <b>let</b> (system_state, protocol_cap) = <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_create">system_inner::create</a>(
        upgrade_caps,
        validators,
        protocol_version,
        epoch_start_timestamp_ms,
        epoch_duration_ms,
        stake_subsidy_start_epoch,
        <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury">protocol_treasury</a>,
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
    protocol_cap
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_initialize"></a>

## Function `initialize`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_initialize">initialize</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, pricing: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing">dwallet_pricing::DWalletPricing</a>, supported_curves_to_signature_algorithms_to_hash_schemes: <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u32, <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u32, vector&lt;u32&gt;&gt;&gt;, max_validator_change_count: u64, cap: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_ProtocolCap">system_inner::ProtocolCap</a>, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_initialize">initialize</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    pricing: DWalletPricing,
    supported_curves_to_signature_algorithms_to_hash_schemes: VecMap&lt;u32, VecMap&lt;u32, vector&lt;u32&gt;&gt;&gt;,
    max_validator_change_count: u64,
    cap: &ProtocolCap,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> package_id = self.package_id;
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_initialize">initialize</a>(
        pricing,
        supported_curves_to_signature_algorithms_to_hash_schemes,
        max_validator_change_count,
        package_id,
        cap,
        clock,
        ctx,
    );
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


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_add_validator_candidate">request_add_validator_candidate</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, name: <a href="../std/string.md#std_string_String">std::string::String</a>, protocol_pubkey_bytes: vector&lt;u8&gt;, network_pubkey_bytes: vector&lt;u8&gt;, consensus_pubkey_bytes: vector&lt;u8&gt;, class_groups_pubkey_and_proof_bytes: (ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof</a>, proof_of_possession_bytes: vector&lt;u8&gt;, network_address: <a href="../std/string.md#std_string_String">std::string::String</a>, p2p_address: <a href="../std/string.md#std_string_String">std::string::String</a>, consensus_address: <a href="../std/string.md#std_string_String">std::string::String</a>, commission_rate: u16, metadata: (ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">validator_metadata::ValidatorMetadata</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): ((ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCommissionCap">validator_cap::ValidatorCommissionCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_add_validator_candidate">request_add_validator_candidate</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
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
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_request_add_validator_candidate">request_add_validator_candidate</a>(
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

<a name="(ika_system=0x0)_system_request_remove_validator_candidate"></a>

## Function `request_remove_validator_candidate`

Called by a validator candidate to remove themselves from the candidacy. After this call
their staking pool becomes deactivate.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_remove_validator_candidate">request_remove_validator_candidate</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_remove_validator_candidate">request_remove_validator_candidate</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    cap: &ValidatorCap,
) {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_request_remove_validator_candidate">request_remove_validator_candidate</a>(cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_request_add_validator"></a>

## Function `request_add_validator`

Called by a validator candidate to add themselves to the active validator set beginning next epoch.
Aborts if the validator is a duplicate with one of the pending or active validators, or if the amount of
stake the validator has doesn't meet the min threshold, or if the number of new validators for the next
epoch has already reached the maximum.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_add_validator">request_add_validator</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_add_validator">request_add_validator</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    cap: &ValidatorCap,
) {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_request_add_validator">request_add_validator</a>(cap)
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


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_remove_validator">request_remove_validator</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_remove_validator">request_remove_validator</a>(self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>, cap: &ValidatorCap) {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_request_remove_validator">request_remove_validator</a>(cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_set_next_commission"></a>

## Function `set_next_commission`

A validator can call this function to set a new commission rate, updated at the end of
the epoch.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_set_next_commission">set_next_commission</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, new_commission_rate: u16, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_set_next_commission">set_next_commission</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    new_commission_rate: u16,
    cap: &ValidatorOperationCap,
) {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_set_next_commission">set_next_commission</a>(new_commission_rate, cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_request_add_stake"></a>

## Function `request_add_stake`

Add stake to a validator's staking pool.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_add_stake">request_add_stake</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, stake: <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_add_stake">request_add_stake</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    stake: Coin&lt;IKA&gt;,
    validator_id: ID,
    ctx: &<b>mut</b> TxContext,
): StakedIka {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_request_add_stake">request_add_stake</a>(stake, validator_id, ctx)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_request_withdraw_stake"></a>

## Function `request_withdraw_stake`

Marks the amount as a withdrawal to be processed and removes it from the stake weight of the
node. Allows the user to call withdraw_stake after the epoch change to the next epoch and
shard transfer is done.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_withdraw_stake">request_withdraw_stake</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_withdraw_stake">request_withdraw_stake</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: &<b>mut</b> StakedIka,
) {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_request_withdraw_stake">request_withdraw_stake</a>(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_withdraw_stake"></a>

## Function `withdraw_stake`

Withdraws the staked amount from the staking pool.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_withdraw_stake">withdraw_stake</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_withdraw_stake">withdraw_stake</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: StakedIka,
    ctx: &<b>mut</b> TxContext,
): Coin&lt;IKA&gt; {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_withdraw_stake">withdraw_stake</a>(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>, ctx)
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


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_report_validator">report_validator</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, reportee_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_report_validator">report_validator</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    cap: &ValidatorOperationCap,
    reportee_id: ID,
) {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_report_validator">report_validator</a>(cap, reportee_id)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_undo_report_validator"></a>

## Function `undo_report_validator`

Undo a <code><a href="../ika_system/system.md#(ika_system=0x0)_system_report_validator">report_validator</a></code> action. Aborts if
1. the reportee is not a currently active validator or
2. the sender has not previously reported the <code>reportee_id</code>, or
3. the cap is not valid


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_undo_report_validator">undo_report_validator</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, reportee_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_undo_report_validator">undo_report_validator</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    cap: &ValidatorOperationCap,
    reportee_id: ID,
) {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_undo_report_validator">undo_report_validator</a>(cap, reportee_id)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_rotate_operation_cap"></a>

## Function `rotate_operation_cap`

Create a new <code>ValidatorOperationCap</code> and registers it. The original object is thus revoked.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_rotate_operation_cap">rotate_operation_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_rotate_operation_cap">rotate_operation_cap</a>(self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>, cap: &ValidatorCap, ctx: &<b>mut</b> TxContext): ValidatorOperationCap {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_rotate_operation_cap">rotate_operation_cap</a>(cap, ctx)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_rotate_commission_cap"></a>

## Function `rotate_commission_cap`

Create a new <code>ValidatorCommissionCap</code> and registers it. The original object is thus revoked.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_rotate_commission_cap">rotate_commission_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCommissionCap">validator_cap::ValidatorCommissionCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_rotate_commission_cap">rotate_commission_cap</a>(self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>, cap: &ValidatorCap, ctx: &<b>mut</b> TxContext): ValidatorCommissionCap {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_rotate_commission_cap">rotate_commission_cap</a>(cap, ctx)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_collect_commission"></a>

## Function `collect_commission`

Withdraws the commission from the validator. Amount is optional, if not provided,
the full commission is withdrawn.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_collect_commission">collect_commission</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCommissionCap">validator_cap::ValidatorCommissionCap</a>, amount: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_collect_commission">collect_commission</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    cap: &ValidatorCommissionCap,
    amount: Option&lt;u64&gt;,
    ctx: &<b>mut</b> TxContext,
): Coin&lt;IKA&gt; {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_collect_commission">collect_commission</a>(cap, amount, ctx)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_set_validator_name"></a>

## Function `set_validator_name`

Set a validator's name.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_set_validator_name">set_validator_name</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, name: <a href="../std/string.md#std_string_String">std::string::String</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_set_validator_name">set_validator_name</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    name: String,
    cap: &ValidatorOperationCap,
) {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_set_validator_name">set_validator_name</a>(name, cap);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_validator_metadata"></a>

## Function `validator_metadata`

Get a validator's metadata.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata">validator_metadata</a>(self: &(ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): (ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">validator_metadata::ValidatorMetadata</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata">validator_metadata</a>(
    self: &<a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    validator_id: ID,
): ValidatorMetadata {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner">inner</a>().<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata">validator_metadata</a>(validator_id)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_set_validator_metadata"></a>

## Function `set_validator_metadata`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_set_validator_metadata">set_validator_metadata</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, metadata: (ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">validator_metadata::ValidatorMetadata</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_set_validator_metadata">set_validator_metadata</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    metadata: ValidatorMetadata,
    cap: &ValidatorOperationCap,
) {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_set_validator_metadata">set_validator_metadata</a>(cap, metadata);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_set_next_epoch_network_address"></a>

## Function `set_next_epoch_network_address`

Sets a validator's network address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_set_next_epoch_network_address">set_next_epoch_network_address</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, network_address: <a href="../std/string.md#std_string_String">std::string::String</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_set_next_epoch_network_address">set_next_epoch_network_address</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    network_address: String,
    cap: &ValidatorOperationCap,
) {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_set_next_epoch_network_address">set_next_epoch_network_address</a>(network_address, cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_set_next_epoch_p2p_address"></a>

## Function `set_next_epoch_p2p_address`

Sets a validator's p2p address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_set_next_epoch_p2p_address">set_next_epoch_p2p_address</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, p2p_address: <a href="../std/string.md#std_string_String">std::string::String</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_set_next_epoch_p2p_address">set_next_epoch_p2p_address</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    p2p_address: String,
    cap: &ValidatorOperationCap,
) {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_set_next_epoch_p2p_address">set_next_epoch_p2p_address</a>(p2p_address, cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_set_next_epoch_consensus_address"></a>

## Function `set_next_epoch_consensus_address`

Sets a validator's consensus address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_set_next_epoch_consensus_address">set_next_epoch_consensus_address</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, consensus_address: <a href="../std/string.md#std_string_String">std::string::String</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_set_next_epoch_consensus_address">set_next_epoch_consensus_address</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    consensus_address: String,
    cap: &ValidatorOperationCap,
) {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_set_next_epoch_consensus_address">set_next_epoch_consensus_address</a>(consensus_address, cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_set_next_epoch_protocol_pubkey_bytes"></a>

## Function `set_next_epoch_protocol_pubkey_bytes`

Sets a validator's public key of protocol key and proof of possession.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_set_next_epoch_protocol_pubkey_bytes">set_next_epoch_protocol_pubkey_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, protocol_pubkey: vector&lt;u8&gt;, proof_of_possession_bytes: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_set_next_epoch_protocol_pubkey_bytes">set_next_epoch_protocol_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    protocol_pubkey: vector&lt;u8&gt;,
    proof_of_possession_bytes: vector&lt;u8&gt;,
    cap: &ValidatorOperationCap,
    ctx: &<b>mut</b> TxContext,
) {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_set_next_epoch_protocol_pubkey_bytes">set_next_epoch_protocol_pubkey_bytes</a>(protocol_pubkey, proof_of_possession_bytes, cap, ctx)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_set_next_epoch_network_pubkey_bytes"></a>

## Function `set_next_epoch_network_pubkey_bytes`

Sets a validator's public key of network key.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_set_next_epoch_network_pubkey_bytes">set_next_epoch_network_pubkey_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, network_pubkey: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_set_next_epoch_network_pubkey_bytes">set_next_epoch_network_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    network_pubkey: vector&lt;u8&gt;,
    cap: &ValidatorOperationCap,
) {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_set_next_epoch_network_pubkey_bytes">set_next_epoch_network_pubkey_bytes</a>(network_pubkey, cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_set_next_epoch_consensus_pubkey_bytes"></a>

## Function `set_next_epoch_consensus_pubkey_bytes`

Sets a validator's public key of worker key.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_set_next_epoch_consensus_pubkey_bytes">set_next_epoch_consensus_pubkey_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, consensus_pubkey_bytes: vector&lt;u8&gt;, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_set_next_epoch_consensus_pubkey_bytes">set_next_epoch_consensus_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    consensus_pubkey_bytes: vector&lt;u8&gt;,
    cap: &ValidatorOperationCap,
) {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_set_next_epoch_consensus_pubkey_bytes">set_next_epoch_consensus_pubkey_bytes</a>(consensus_pubkey_bytes, cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_set_next_epoch_class_groups_pubkey_and_proof_bytes"></a>

## Function `set_next_epoch_class_groups_pubkey_and_proof_bytes`

Sets a validator's public key of class groups key and its associated proof.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_set_next_epoch_class_groups_pubkey_and_proof_bytes">set_next_epoch_class_groups_pubkey_and_proof_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, class_groups_pubkey_and_proof: (ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_set_next_epoch_class_groups_pubkey_and_proof_bytes">set_next_epoch_class_groups_pubkey_and_proof_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    class_groups_pubkey_and_proof: ClassGroupsPublicKeyAndProof,
    cap: &ValidatorOperationCap,
) {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_set_next_epoch_class_groups_pubkey_and_proof_bytes">set_next_epoch_class_groups_pubkey_and_proof_bytes</a>(class_groups_pubkey_and_proof, cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_set_pricing_vote"></a>

## Function `set_pricing_vote`

Sets a validator's pricing vote.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_set_pricing_vote">set_pricing_vote</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, dwallet_coordinator: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, pricing: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing">dwallet_pricing::DWalletPricing</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_set_pricing_vote">set_pricing_vote</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    dwallet_coordinator: &<b>mut</b> DWalletCoordinator,
    pricing: DWalletPricing,
    cap: &ValidatorOperationCap,
) {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_set_pricing_vote">set_pricing_vote</a>(dwallet_coordinator.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>(), pricing, cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_token_exchange_rates"></a>

## Function `token_exchange_rates`

Get the pool token exchange rate of a validator. Works for both active and inactive pools.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_token_exchange_rates">token_exchange_rates</a>(self: &(ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): &<a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, (ika_system=0x0)::<a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_TokenExchangeRate">token_exchange_rate::TokenExchangeRate</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_token_exchange_rates">token_exchange_rates</a>(
    self: &<a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    validator_id: ID,
): &Table&lt;u64, TokenExchangeRate&gt; {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner">inner</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_token_exchange_rates">token_exchange_rates</a>(validator_id)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_active_committee"></a>

## Function `active_committee`

Get the active committee of the current epoch.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_active_committee">active_committee</a>(self: &(ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>): (ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">bls_committee::BlsCommittee</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_active_committee">active_committee</a>(self: &<a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>): BlsCommittee {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner">inner</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_active_committee">active_committee</a>()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_next_epoch_active_committee"></a>

## Function `next_epoch_active_committee`

Get the active committee of the next epoch.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_next_epoch_active_committee">next_epoch_active_committee</a>(self: &(ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;(ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">bls_committee::BlsCommittee</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_next_epoch_active_committee">next_epoch_active_committee</a>(self: &<a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>): Option&lt;BlsCommittee&gt; {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner">inner</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_next_epoch_active_committee">next_epoch_active_committee</a>()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_request_reconfig_mid_epoch"></a>

## Function `request_reconfig_mid_epoch`

Locks the committee of the next epoch to allow starting the reconfiguration process.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_reconfig_mid_epoch">request_reconfig_mid_epoch</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, dwallet_coordinator: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_reconfig_mid_epoch">request_reconfig_mid_epoch</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>, dwallet_coordinator: &<b>mut</b> DWalletCoordinator, clock: &Clock, ctx: &<b>mut</b> TxContext
) {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().process_mid_epoch(clock, dwallet_coordinator.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>(), ctx);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_request_lock_epoch_sessions"></a>

## Function `request_lock_epoch_sessions`

Locks the MPC sessions that should get completed as part of the current epoch.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_lock_epoch_sessions">request_lock_epoch_sessions</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, dwallet_coordinator: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_lock_epoch_sessions">request_lock_epoch_sessions</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>, dwallet_coordinator: &<b>mut</b> DWalletCoordinator, clock: &Clock
) {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_request_lock_epoch_sessions">request_lock_epoch_sessions</a>(dwallet_coordinator.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>(), clock);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_request_advance_epoch"></a>

## Function `request_advance_epoch`

Advances the epoch to the next epoch.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_advance_epoch">request_advance_epoch</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, dwallet_coordinator: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_advance_epoch">request_advance_epoch</a>(self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>, dwallet_coordinator: &<b>mut</b> DWalletCoordinator, clock: &Clock, ctx: &<b>mut</b> TxContext) {
    <b>let</b> inner_system = self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    <b>let</b> inner_dwallet = dwallet_coordinator.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    inner_system.advance_epoch(inner_dwallet, clock, ctx);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_request_dwallet_network_encryption_key_dkg_by_cap"></a>

## Function `request_dwallet_network_encryption_key_dkg_by_cap`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_dwallet_network_encryption_key_dkg_by_cap">request_dwallet_network_encryption_key_dkg_by_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator">dwallet_2pc_mpc_coordinator</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, cap: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_ProtocolCap">system_inner::ProtocolCap</a>, params_for_network: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_request_dwallet_network_encryption_key_dkg_by_cap">request_dwallet_network_encryption_key_dkg_by_cap</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator">dwallet_2pc_mpc_coordinator</a>: &<b>mut</b> DWalletCoordinator,
    cap: &ProtocolCap,
    params_for_network: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_request_dwallet_network_encryption_key_dkg_by_cap">request_dwallet_network_encryption_key_dkg_by_cap</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator">dwallet_2pc_mpc_coordinator</a>.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>(), cap, params_for_network, ctx);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_set_supported_and_pricing"></a>

## Function `set_supported_and_pricing`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_set_supported_and_pricing">set_supported_and_pricing</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator">dwallet_2pc_mpc_coordinator</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, default_pricing: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing">dwallet_pricing::DWalletPricing</a>, supported_curves_to_signature_algorithms_to_hash_schemes: <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u32, <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u32, vector&lt;u32&gt;&gt;&gt;, protocol_cap: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_ProtocolCap">system_inner::ProtocolCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_set_supported_and_pricing">set_supported_and_pricing</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator">dwallet_2pc_mpc_coordinator</a>: &<b>mut</b> DWalletCoordinator,
    default_pricing: DWalletPricing,
    supported_curves_to_signature_algorithms_to_hash_schemes: VecMap&lt;u32, VecMap&lt;u32, vector&lt;u32&gt;&gt;&gt;,
    protocol_cap: &ProtocolCap,
) {
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner">dwallet_2pc_mpc_coordinator_inner</a> = <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator">dwallet_2pc_mpc_coordinator</a>.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_set_supported_and_pricing">set_supported_and_pricing</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner">dwallet_2pc_mpc_coordinator_inner</a>, default_pricing, supported_curves_to_signature_algorithms_to_hash_schemes, protocol_cap);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_set_paused_curves_and_signature_algorithms"></a>

## Function `set_paused_curves_and_signature_algorithms`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_set_paused_curves_and_signature_algorithms">set_paused_curves_and_signature_algorithms</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator">dwallet_2pc_mpc_coordinator</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, paused_curves: vector&lt;u32&gt;, paused_signature_algorithms: vector&lt;u32&gt;, paused_hash_schemes: vector&lt;u32&gt;, protocol_cap: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_ProtocolCap">system_inner::ProtocolCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_set_paused_curves_and_signature_algorithms">set_paused_curves_and_signature_algorithms</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator">dwallet_2pc_mpc_coordinator</a>: &<b>mut</b> DWalletCoordinator,
    paused_curves: vector&lt;u32&gt;,
    paused_signature_algorithms: vector&lt;u32&gt;,
    paused_hash_schemes: vector&lt;u32&gt;,
    protocol_cap: &ProtocolCap,
) {
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner">dwallet_2pc_mpc_coordinator_inner</a> = <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator">dwallet_2pc_mpc_coordinator</a>.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_set_paused_curves_and_signature_algorithms">set_paused_curves_and_signature_algorithms</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner">dwallet_2pc_mpc_coordinator_inner</a>, paused_curves, paused_signature_algorithms, paused_hash_schemes, protocol_cap);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_authorize_upgrade_by_cap"></a>

## Function `authorize_upgrade_by_cap`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_authorize_upgrade_by_cap">authorize_upgrade_by_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, cap: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_ProtocolCap">system_inner::ProtocolCap</a>, package_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, digest: vector&lt;u8&gt;): <a href="../sui/package.md#sui_package_UpgradeTicket">sui::package::UpgradeTicket</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_authorize_upgrade_by_cap">authorize_upgrade_by_cap</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    cap: &ProtocolCap,
    package_id: ID,
    digest: vector&lt;u8&gt;,
): UpgradeTicket {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_authorize_upgrade_by_cap">authorize_upgrade_by_cap</a>(cap, package_id, digest)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_authorize_upgrade_by_approval"></a>

## Function `authorize_upgrade_by_approval`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_authorize_upgrade_by_approval">authorize_upgrade_by_approval</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, package_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): <a href="../sui/package.md#sui_package_UpgradeTicket">sui::package::UpgradeTicket</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_authorize_upgrade_by_approval">authorize_upgrade_by_approval</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    package_id: ID,
): UpgradeTicket {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_authorize_upgrade_by_approval">authorize_upgrade_by_approval</a>(package_id)
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

<a name="(ika_system=0x0)_system_process_checkpoint_message_by_cap"></a>

## Function `process_checkpoint_message_by_cap`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_process_checkpoint_message_by_cap">process_checkpoint_message_by_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, cap: &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_ProtocolCap">system_inner::ProtocolCap</a>, message: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_process_checkpoint_message_by_cap">process_checkpoint_message_by_cap</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    cap: &ProtocolCap,
    message: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_process_checkpoint_message_by_cap">process_checkpoint_message_by_cap</a>(cap, message, ctx);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_process_checkpoint_message_by_quorum"></a>

## Function `process_checkpoint_message_by_quorum`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, signature: vector&lt;u8&gt;, signers_bitmap: vector&lt;u8&gt;, message: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(
    self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    signature: vector&lt;u8&gt;,
    signers_bitmap: vector&lt;u8&gt;,
    message: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>if</b> (1 == 1) {
        <b>abort</b> 1
    };
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(signature, signers_bitmap, message, ctx);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_migrate"></a>

## Function `migrate`

Migrate the staking object to the new package id.

This function sets the new package id and version and can be modified in future versions
to migrate changes in the <code><a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner">system_inner</a></code> object if needed.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_migrate">migrate</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_migrate">migrate</a>(
        self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
) {
    <b>assert</b>!(self.version &lt; <a href="../ika_system/system.md#(ika_system=0x0)_system_VERSION">VERSION</a>, <a href="../ika_system/system.md#(ika_system=0x0)_system_EInvalidMigration">EInvalidMigration</a>);
    // Move the old <a href="../ika_system/system.md#(ika_system=0x0)_system">system</a> state <a href="../ika_system/system.md#(ika_system=0x0)_system_inner">inner</a> to the new version.
    <b>let</b> <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner">system_inner</a>: SystemInner = dynamic_field::remove(&<b>mut</b> self.id, self.version);
    dynamic_field::add(&<b>mut</b> self.id, <a href="../ika_system/system.md#(ika_system=0x0)_system_VERSION">VERSION</a>, <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner">system_inner</a>);
    self.version = <a href="../ika_system/system.md#(ika_system=0x0)_system_VERSION">VERSION</a>;
    // Set the new package id.
    <b>assert</b>!(self.new_package_id.is_some(), <a href="../ika_system/system.md#(ika_system=0x0)_system_EInvalidMigration">EInvalidMigration</a>);
    self.package_id = self.new_package_id.extract();
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_calculate_rewards"></a>

## Function `calculate_rewards`

Calculate the rewards for an amount with value <code>staked_principal</code>, staked in the validator with
the given <code>validator_id</code> between <code>activation_epoch</code> and <code>withdraw_epoch</code>.

This function can be used with <code>dev_inspect</code> to calculate the expected rewards for a <code>StakedIka</code>
object or, more generally, the returns provided by a given validator over a given period.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_calculate_rewards">calculate_rewards</a>(self: &(ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, staked_principal: u64, activation_epoch: u64, withdraw_epoch: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_calculate_rewards">calculate_rewards</a>(
    self: &<a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>,
    validator_id: ID,
    staked_principal: u64,
    activation_epoch: u64,
    withdraw_epoch: u64,
): u64 {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner">inner</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_calculate_rewards">calculate_rewards</a>(validator_id, staked_principal, activation_epoch, withdraw_epoch)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_can_withdraw_staked_ika_early"></a>

## Function `can_withdraw_staked_ika_early`

Call <code><a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_can_withdraw_early">staked_ika::can_withdraw_early</a></code> to allow calling this method in applications.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_can_withdraw_staked_ika_early">can_withdraw_staked_ika_early</a>(self: &(ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: &(ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_can_withdraw_staked_ika_early">can_withdraw_staked_ika_early</a>(self: &<a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: &StakedIka): bool {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner">inner</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_can_withdraw_staked_ika_early">can_withdraw_staked_ika_early</a>(<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_dwallet_2pc_mpc_coordinator_network_encryption_key_ids"></a>

## Function `dwallet_2pc_mpc_coordinator_network_encryption_key_ids`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_dwallet_2pc_mpc_coordinator_network_encryption_key_ids">dwallet_2pc_mpc_coordinator_network_encryption_key_ids</a>(self: &(ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>): vector&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_dwallet_2pc_mpc_coordinator_network_encryption_key_ids">dwallet_2pc_mpc_coordinator_network_encryption_key_ids</a>(self: &<a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>): vector&lt;ID&gt; {
    self.<a href="../ika_system/system.md#(ika_system=0x0)_system_inner">inner</a>().<a href="../ika_system/system.md#(ika_system=0x0)_system_dwallet_2pc_mpc_coordinator_network_encryption_key_ids">dwallet_2pc_mpc_coordinator_network_encryption_key_ids</a>()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner_mut"></a>

## Function `inner_mut`

Get a mutable reference to <code>SystemInnerVX</code> from the <code><a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a></code>.


<pre><code><b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>): &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_inner_mut">inner_mut</a>(self: &<b>mut</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>): &<b>mut</b> SystemInner {
    <b>assert</b>!(self.version == <a href="../ika_system/system.md#(ika_system=0x0)_system_VERSION">VERSION</a>, <a href="../ika_system/system.md#(ika_system=0x0)_system_EWrongInnerVersion">EWrongInnerVersion</a>);
    dynamic_field::borrow_mut(&<b>mut</b> self.id, <a href="../ika_system/system.md#(ika_system=0x0)_system_VERSION">VERSION</a>)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_inner"></a>

## Function `inner`

Get an immutable reference to <code>SystemInnerVX</code> from the <code><a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a></code>.


<pre><code><b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_inner">inner</a>(self: &(ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system_System">system::System</a>): &(ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_SystemInner">system_inner::SystemInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/system.md#(ika_system=0x0)_system_inner">inner</a>(self: &<a href="../ika_system/system.md#(ika_system=0x0)_system_System">System</a>): &SystemInner {
    <b>assert</b>!(self.version == <a href="../ika_system/system.md#(ika_system=0x0)_system_VERSION">VERSION</a>, <a href="../ika_system/system.md#(ika_system=0x0)_system_EWrongInnerVersion">EWrongInnerVersion</a>);
    dynamic_field::borrow(&self.id, <a href="../ika_system/system.md#(ika_system=0x0)_system_VERSION">VERSION</a>)
}
</code></pre>



</details>
