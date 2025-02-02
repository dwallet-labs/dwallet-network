---
title: Module `0x0::init`
---



-  [Resource `Init`](#0x0_init_Init)
-  [Resource `InitCap`](#0x0_init_InitCap)
-  [Constants](#@Constants_0)
-  [Function `init`](#0x0_init_init)
-  [Function `initialize`](#0x0_init_initialize)
-  [Function `destroy`](#0x0_init_destroy)


<pre><code><b>use</b> <a href="../ika/ika.md#0x0_ika">0x0::ika</a>;
<b>use</b> <a href="protocol_cap.md#0x0_protocol_cap">0x0::protocol_cap</a>;
<b>use</b> <a href="protocol_treasury.md#0x0_protocol_treasury">0x0::protocol_treasury</a>;
<b>use</b> <a href="system.md#0x0_system">0x0::system</a>;
<b>use</b> <a href="system_inner.md#0x0_system_inner_v1">0x0::system_inner_v1</a>;
<b>use</b> <a href="validator_set.md#0x0_validator_set">0x0::validator_set</a>;
<b>use</b> <a href="../move-stdlib/ascii.md#0x1_ascii">0x1::ascii</a>;
<b>use</b> <a href="../move-stdlib/type_name.md#0x1_type_name">0x1::type_name</a>;
<b>use</b> <a href="../sui-framework/address.md#0x2_address">0x2::address</a>;
<b>use</b> <a href="../sui-framework/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/package.md#0x2_package">0x2::package</a>;
<b>use</b> <a href="../sui-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
</code></pre>



<a name="0x0_init_Init"></a>

## Resource `Init`



<pre><code><b>struct</b> <a href="init.md#0x0_init_Init">Init</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x0_init_InitCap"></a>

## Resource `InitCap`

Must only be created by <code><a href="init.md#0x0_init">init</a></code>.


<pre><code><b>struct</b> <a href="init.md#0x0_init_InitCap">InitCap</a> <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x0_init_EInvalidUpgradeCap"></a>



<pre><code><b>const</b> <a href="init.md#0x0_init_EInvalidUpgradeCap">EInvalidUpgradeCap</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1;
</code></pre>



<a name="0x0_init_init"></a>

## Function `init`

Init function, creates an init cap and transfers it to the sender.
This allows the sender to call the function to actually initialize the system
with the corresponding parameters. Once that function is called, the cap is destroyed.


<pre><code><b>fun</b> <a href="init.md#0x0_init">init</a>(ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="init.md#0x0_init">init</a>(ctx: &<b>mut</b> TxContext) {
    <b>let</b> id = <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx);
    <b>let</b> init_cap = <a href="init.md#0x0_init_InitCap">InitCap</a> {
        id,
    };
    <a href="../sui-framework/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(init_cap, ctx.sender());
}
</code></pre>



</details>

<a name="0x0_init_initialize"></a>

## Function `initialize`

Function to initialize ika and share the system object.
This can only be called once, after which the <code><a href="init.md#0x0_init_InitCap">InitCap</a></code> is destroyed.


<pre><code><b>public</b> <b>fun</b> <a href="init.md#0x0_init_initialize">initialize</a>(cap: <a href="init.md#0x0_init_InitCap">init::InitCap</a>, ika_upgrade_cap: <a href="../sui-framework/package.md#0x2_package_UpgradeCap">package::UpgradeCap</a>, ika_system_upgrade_cap: <a href="../sui-framework/package.md#0x2_package_UpgradeCap">package::UpgradeCap</a>, protocol_treasury_cap: <a href="../sui-framework/coin.md#0x2_coin_TreasuryCap">coin::TreasuryCap</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;, protocol_version: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, chain_start_timestamp_ms: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, epoch_duration_ms: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, stake_subsidy_start_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, stake_subsidy_rate: u16, stake_subsidy_period_length: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, min_validator_count: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, max_validator_count: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, min_validator_joining_stake: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, validator_low_stake_threshold: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, validator_very_low_stake_threshold: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, validator_low_stake_grace_period: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, reward_slashing_rate: u16, lock_active_committee: bool, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="protocol_cap.md#0x0_protocol_cap_ProtocolCap">protocol_cap::ProtocolCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="init.md#0x0_init_initialize">initialize</a>(
    cap: <a href="init.md#0x0_init_InitCap">InitCap</a>,
    ika_upgrade_cap: UpgradeCap,
    ika_system_upgrade_cap: UpgradeCap,
    protocol_treasury_cap: TreasuryCap&lt;IKA&gt;,
    protocol_version: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    chain_start_timestamp_ms: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    epoch_duration_ms: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    // Stake Subsidy parameters
    stake_subsidy_start_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    stake_subsidy_rate: u16,
    stake_subsidy_period_length: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
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
): ProtocolCap {
    <b>let</b> ika_package_id = ika_upgrade_cap.<a href="../sui-framework/package.md#0x2_package">package</a>();
    <b>let</b> ika_system_package_id = ika_system_upgrade_cap.<a href="../sui-framework/package.md#0x2_package">package</a>();

    <b>assert</b>!(
        <a href="../move-stdlib/type_name.md#0x1_type_name_get">type_name::get</a>&lt;IKA&gt;().get_address() == ika_package_id.to_address().to_ascii_string(),
        <a href="init.md#0x0_init_EInvalidUpgradeCap">EInvalidUpgradeCap</a>,
    );

    <b>assert</b>!(
        <a href="../move-stdlib/type_name.md#0x1_type_name_get">type_name::get</a>&lt;<a href="init.md#0x0_init_InitCap">InitCap</a>&gt;().get_address() == ika_system_package_id.to_address().to_ascii_string(),
        <a href="init.md#0x0_init_EInvalidUpgradeCap">EInvalidUpgradeCap</a>,
    );

    <b>let</b> upgrade_caps = <a href="../move-stdlib/vector.md#0x1_vector">vector</a>[ika_upgrade_cap, ika_system_upgrade_cap];

    <b>let</b> validators = <a href="validator_set.md#0x0_validator_set_new">validator_set::new</a>(ctx);

    <b>let</b> system_parameters = <a href="system_inner.md#0x0_system_inner_v1_create_system_parameters">system_inner_v1::create_system_parameters</a>(
        epoch_duration_ms,
        stake_subsidy_start_epoch,
        // Validator <a href="committee.md#0x0_committee">committee</a> parameters
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

    <b>let</b> stake_subsidy = <a href="protocol_treasury.md#0x0_protocol_treasury_create">protocol_treasury::create</a>(
        protocol_treasury_cap,
        stake_subsidy_rate,
        stake_subsidy_period_length,
        ctx,
    );

    <b>let</b> <a href="protocol_cap.md#0x0_protocol_cap">protocol_cap</a> = <a href="protocol_cap.md#0x0_protocol_cap_new_protocol_cap">protocol_cap::new_protocol_cap</a>(ctx);

    <b>let</b> authorized_protocol_cap_ids = <a href="../move-stdlib/vector.md#0x1_vector">vector</a>[<a href="../sui-framework/object.md#0x2_object_id">object::id</a>(&<a href="protocol_cap.md#0x0_protocol_cap">protocol_cap</a>)];

    <a href="system.md#0x0_system_create">system::create</a>(
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

    cap.<a href="init.md#0x0_init_destroy">destroy</a>();

    <a href="protocol_cap.md#0x0_protocol_cap">protocol_cap</a>
}
</code></pre>



</details>

<a name="0x0_init_destroy"></a>

## Function `destroy`



<pre><code><b>fun</b> <a href="init.md#0x0_init_destroy">destroy</a>(cap: <a href="init.md#0x0_init_InitCap">init::InitCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="init.md#0x0_init_destroy">destroy</a>(cap: <a href="init.md#0x0_init_InitCap">InitCap</a>) {
    <b>let</b> <a href="init.md#0x0_init_InitCap">InitCap</a> { id } = cap;
    id.delete();
}
</code></pre>



</details>
