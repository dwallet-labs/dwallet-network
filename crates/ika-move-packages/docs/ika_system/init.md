---
title: Module `(ika_system=0x0)::init`
---



-  [Struct `Init`](#(ika_system=0x0)_init_Init)
-  [Struct `InitCap`](#(ika_system=0x0)_init_InitCap)
-  [Constants](#@Constants_0)
-  [Function `init`](#(ika_system=0x0)_init_init)
-  [Function `initialize`](#(ika_system=0x0)_init_initialize)
-  [Function `destroy`](#(ika_system=0x0)_init_destroy)


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
<b>use</b> (ika_system=0x0)::<a href="../ika_system/system.md#(ika_system=0x0)_system">system</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner">system_inner</a>;
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



<a name="(ika_system=0x0)_init_Init"></a>

## Struct `Init`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/init.md#(ika_system=0x0)_init_Init">Init</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_init_InitCap"></a>

## Struct `InitCap`

Must only be created by <code><a href="../ika_system/init.md#(ika_system=0x0)_init">init</a></code>.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/init.md#(ika_system=0x0)_init_InitCap">InitCap</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="(ika_system=0x0)_init_EInvalidUpgradeCap"></a>



<pre><code><b>const</b> <a href="../ika_system/init.md#(ika_system=0x0)_init_EInvalidUpgradeCap">EInvalidUpgradeCap</a>: u64 = 1;
</code></pre>



<a name="(ika_system=0x0)_init_init"></a>

## Function `init`

Init function, creates an init cap and transfers it to the sender.
This allows the sender to call the function to actually initialize the system
with the corresponding parameters. Once that function is called, the cap is destroyed.


<pre><code><b>fun</b> <a href="../ika_system/init.md#(ika_system=0x0)_init">init</a>(ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/init.md#(ika_system=0x0)_init">init</a>(ctx: &<b>mut</b> TxContext) {
    <b>let</b> id = object::new(ctx);
    <b>let</b> init_cap = <a href="../ika_system/init.md#(ika_system=0x0)_init_InitCap">InitCap</a> {
        id,
    };
    transfer::transfer(init_cap, ctx.sender());
}
</code></pre>



</details>

<a name="(ika_system=0x0)_init_initialize"></a>

## Function `initialize`

Function to initialize ika and share the system object.
This can only be called once, after which the <code><a href="../ika_system/init.md#(ika_system=0x0)_init_InitCap">InitCap</a></code> is destroyed.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/init.md#(ika_system=0x0)_init_initialize">initialize</a>(cap: (ika_system=0x0)::<a href="../ika_system/init.md#(ika_system=0x0)_init_InitCap">init::InitCap</a>, ika_upgrade_cap: <a href="../sui/package.md#sui_package_UpgradeCap">sui::package::UpgradeCap</a>, ika_system_upgrade_cap: <a href="../sui/package.md#sui_package_UpgradeCap">sui::package::UpgradeCap</a>, protocol_treasury_cap: <a href="../sui/coin.md#sui_coin_TreasuryCap">sui::coin::TreasuryCap</a>&lt;(ika=0x0)::ika::IKA&gt;, protocol_version: u64, chain_start_timestamp_ms: u64, epoch_duration_ms: u64, stake_subsidy_start_epoch: u64, stake_subsidy_rate: u16, stake_subsidy_period_length: u64, min_validator_count: u64, max_validator_count: u64, min_validator_joining_stake: u64, validator_low_stake_threshold: u64, validator_very_low_stake_threshold: u64, validator_low_stake_grace_period: u64, reward_slashing_rate: u16, lock_active_committee: bool, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_ProtocolCap">protocol_cap::ProtocolCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/init.md#(ika_system=0x0)_init_initialize">initialize</a>(
    cap: <a href="../ika_system/init.md#(ika_system=0x0)_init_InitCap">InitCap</a>,
    ika_upgrade_cap: UpgradeCap,
    ika_system_upgrade_cap: UpgradeCap,
    protocol_treasury_cap: TreasuryCap&lt;IKA&gt;,
    protocol_version: u64,
    chain_start_timestamp_ms: u64,
    epoch_duration_ms: u64,
    // Stake Subsidy parameters
    stake_subsidy_start_epoch: u64,
    stake_subsidy_rate: u16,
    stake_subsidy_period_length: u64,
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
): ProtocolCap {
    <b>let</b> ika_package_id = ika_upgrade_cap.package();
    <b>let</b> ika_system_package_id = ika_system_upgrade_cap.package();
    <b>assert</b>!(
        type_name::get&lt;IKA&gt;().get_address() == ika_package_id.to_address().to_ascii_string(),
        <a href="../ika_system/init.md#(ika_system=0x0)_init_EInvalidUpgradeCap">EInvalidUpgradeCap</a>,
    );
    <b>assert</b>!(
        type_name::get&lt;<a href="../ika_system/init.md#(ika_system=0x0)_init_InitCap">InitCap</a>&gt;().get_address() == ika_system_package_id.to_address().to_ascii_string(),
        <a href="../ika_system/init.md#(ika_system=0x0)_init_EInvalidUpgradeCap">EInvalidUpgradeCap</a>,
    );
    <b>let</b> upgrade_caps = vector[ika_upgrade_cap, ika_system_upgrade_cap];
    <b>let</b> validators = <a href="../ika_system/validator_set.md#(ika_system=0x0)_validator_set_new">validator_set::new</a>(ctx);
    <b>let</b> system_parameters = <a href="../ika_system/system_inner.md#(ika_system=0x0)_system_inner_create_system_parameters">system_inner::create_system_parameters</a>(
        epoch_duration_ms,
        stake_subsidy_start_epoch,
        // Validator committee parameters
        min_validator_count,
        max_validator_count,
        min_validator_joining_stake,
        validator_low_stake_threshold,
        validator_very_low_stake_threshold,
        validator_low_stake_grace_period,
        reward_slashing_rate,
        lock_active_committee,
        ctx,
    );
    <b>let</b> stake_subsidy = <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_create">protocol_treasury::create</a>(
        protocol_treasury_cap,
        stake_subsidy_rate,
        stake_subsidy_period_length,
        ctx,
    );
    <b>let</b> <a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap">protocol_cap</a> = <a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_new_protocol_cap">protocol_cap::new_protocol_cap</a>(ctx);
    <b>let</b> authorized_protocol_cap_ids = vector[object::id(&<a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap">protocol_cap</a>)];
    <a href="../ika_system/system.md#(ika_system=0x0)_system_create">system::create</a>(
        ika_system_package_id,
        upgrade_caps,
        validators,
        protocol_version,
        chain_start_timestamp_ms,
        system_parameters,
        stake_subsidy,
        authorized_protocol_cap_ids,
        ctx,
    );
    cap.<a href="../ika_system/init.md#(ika_system=0x0)_init_destroy">destroy</a>();
    <a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap">protocol_cap</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_init_destroy"></a>

## Function `destroy`



<pre><code><b>fun</b> <a href="../ika_system/init.md#(ika_system=0x0)_init_destroy">destroy</a>(cap: (ika_system=0x0)::<a href="../ika_system/init.md#(ika_system=0x0)_init_InitCap">init::InitCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/init.md#(ika_system=0x0)_init_destroy">destroy</a>(cap: <a href="../ika_system/init.md#(ika_system=0x0)_init_InitCap">InitCap</a>) {
    <b>let</b> <a href="../ika_system/init.md#(ika_system=0x0)_init_InitCap">InitCap</a> { id } = cap;
    id.delete();
}
</code></pre>



</details>
