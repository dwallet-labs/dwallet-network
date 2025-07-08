---
title: Module `(ika_dwallet_2pc_mpc=0x0)::ika_dwallet_2pc_mpc_init`
---



-  [Struct `IKA_DWALLET_2PC_MPC_INIT`](#(ika_dwallet_2pc_mpc=0x0)_ika_dwallet_2pc_mpc_init_IKA_DWALLET_2PC_MPC_INIT)
-  [Struct `InitCap`](#(ika_dwallet_2pc_mpc=0x0)_ika_dwallet_2pc_mpc_init_InitCap)
-  [Function `init`](#(ika_dwallet_2pc_mpc=0x0)_ika_dwallet_2pc_mpc_init_init)
-  [Function `initialize`](#(ika_dwallet_2pc_mpc=0x0)_ika_dwallet_2pc_mpc_init_initialize)


<pre><code><b>use</b> (ika=0x0)::ika;
<b>use</b> (ika_common=0x0)::<b>address</b>;
<b>use</b> (ika_common=0x0)::bls_committee;
<b>use</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator">coordinator</a>;
<b>use</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner">coordinator_inner</a>;
<b>use</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing">dwallet_pricing</a>;
<b>use</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/display.md#(ika_dwallet_2pc_mpc=0x0)_ika_dwallet_2pc_mpc_display">ika_dwallet_2pc_mpc_display</a>;
<b>use</b> (ika_system=0x0)::advance_epoch_approver;
<b>use</b> (ika_system=0x0)::protocol_cap;
<b>use</b> (ika_system=0x0)::system_current_status_info;
<b>use</b> (ika_system=0x0)::validator_cap;
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
<b>use</b> <a href="../sui/display.md#sui_display">sui::display</a>;
<b>use</b> <a href="../sui/dynamic_field.md#sui_dynamic_field">sui::dynamic_field</a>;
<b>use</b> <a href="../sui/dynamic_object_field.md#sui_dynamic_object_field">sui::dynamic_object_field</a>;
<b>use</b> <a href="../sui/ed25519.md#sui_ed25519">sui::ed25519</a>;
<b>use</b> <a href="../sui/event.md#sui_event">sui::event</a>;
<b>use</b> <a href="../sui/group_ops.md#sui_group_ops">sui::group_ops</a>;
<b>use</b> <a href="../sui/hash.md#sui_hash">sui::hash</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/object_bag.md#sui_object_bag">sui::object_bag</a>;
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



<a name="(ika_dwallet_2pc_mpc=0x0)_ika_dwallet_2pc_mpc_init_IKA_DWALLET_2PC_MPC_INIT"></a>

## Struct `IKA_DWALLET_2PC_MPC_INIT`

The OTW to create <code>Publisher</code> and <code>Display</code> objects.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/ika_dwallet_2pc_mpc_init.md#(ika_dwallet_2pc_mpc=0x0)_ika_dwallet_2pc_mpc_init_IKA_DWALLET_2PC_MPC_INIT">IKA_DWALLET_2PC_MPC_INIT</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_ika_dwallet_2pc_mpc_init_InitCap"></a>

## Struct `InitCap`

Must only be created by <code><a href="../ika_system/ika_dwallet_2pc_mpc_init.md#(ika_dwallet_2pc_mpc=0x0)_ika_dwallet_2pc_mpc_init_init">init</a></code>.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/ika_dwallet_2pc_mpc_init.md#(ika_dwallet_2pc_mpc=0x0)_ika_dwallet_2pc_mpc_init_InitCap">InitCap</a> <b>has</b> key, store
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
<code>publisher: <a href="../sui/package.md#sui_package_Publisher">sui::package::Publisher</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_ika_dwallet_2pc_mpc_init_init"></a>

## Function `init`

Init function, creates an init cap and transfers it to the sender.
This allows the sender to call the function to actually initialize the system
with the corresponding parameters. Once that function is called, the cap is destroyed.


<pre><code><b>fun</b> <a href="../ika_system/ika_dwallet_2pc_mpc_init.md#(ika_dwallet_2pc_mpc=0x0)_ika_dwallet_2pc_mpc_init_init">init</a>(otw: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/ika_dwallet_2pc_mpc_init.md#(ika_dwallet_2pc_mpc=0x0)_ika_dwallet_2pc_mpc_init_IKA_DWALLET_2PC_MPC_INIT">ika_dwallet_2pc_mpc_init::IKA_DWALLET_2PC_MPC_INIT</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/ika_dwallet_2pc_mpc_init.md#(ika_dwallet_2pc_mpc=0x0)_ika_dwallet_2pc_mpc_init_init">init</a>(otw: <a href="../ika_system/ika_dwallet_2pc_mpc_init.md#(ika_dwallet_2pc_mpc=0x0)_ika_dwallet_2pc_mpc_init_IKA_DWALLET_2PC_MPC_INIT">IKA_DWALLET_2PC_MPC_INIT</a>, ctx: &<b>mut</b> TxContext) {
    <b>let</b> id = object::new(ctx);
    <b>let</b> publisher = package::claim(otw, ctx);
    <b>let</b> init_cap = <a href="../ika_system/ika_dwallet_2pc_mpc_init.md#(ika_dwallet_2pc_mpc=0x0)_ika_dwallet_2pc_mpc_init_InitCap">InitCap</a> { id, publisher };
    transfer::transfer(init_cap, ctx.sender());
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_ika_dwallet_2pc_mpc_init_initialize"></a>

## Function `initialize`

Function to initialize ika and share the system object.
This can only be called once, after which the <code><a href="../ika_system/ika_dwallet_2pc_mpc_init.md#(ika_dwallet_2pc_mpc=0x0)_ika_dwallet_2pc_mpc_init_InitCap">InitCap</a></code> is destroyed.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/ika_dwallet_2pc_mpc_init.md#(ika_dwallet_2pc_mpc=0x0)_ika_dwallet_2pc_mpc_init_initialize">initialize</a>(init_cap: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/ika_dwallet_2pc_mpc_init.md#(ika_dwallet_2pc_mpc=0x0)_ika_dwallet_2pc_mpc_init_InitCap">ika_dwallet_2pc_mpc_init::InitCap</a>, advance_epoch_approver: &<b>mut</b> (ika_system=0x0)::advance_epoch_approver::AdvanceEpochApprover, pricing: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">dwallet_pricing::DWalletPricing</a>, supported_curves_to_signature_algorithms_to_hash_schemes: <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u32, <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u32, vector&lt;u32&gt;&gt;&gt;, dwallet_cap_image_url: <a href="../std/string.md#std_string_String">std::string::String</a>, imported_key_dwallet_cap_image_url: <a href="../std/string.md#std_string_String">std::string::String</a>, unverified_presign_cap_image_url: <a href="../std/string.md#std_string_String">std::string::String</a>, verified_presign_cap_image_url: <a href="../std/string.md#std_string_String">std::string::String</a>, unverified_partial_user_signature_cap_image_url: <a href="../std/string.md#std_string_String">std::string::String</a>, verified_partial_user_signature_cap_image_url: <a href="../std/string.md#std_string_String">std::string::String</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/ika_dwallet_2pc_mpc_init.md#(ika_dwallet_2pc_mpc=0x0)_ika_dwallet_2pc_mpc_init_initialize">initialize</a>(
    init_cap: <a href="../ika_system/ika_dwallet_2pc_mpc_init.md#(ika_dwallet_2pc_mpc=0x0)_ika_dwallet_2pc_mpc_init_InitCap">InitCap</a>,
    advance_epoch_approver: &<b>mut</b> AdvanceEpochApprover,
    pricing: DWalletPricing,
    supported_curves_to_signature_algorithms_to_hash_schemes: VecMap&lt;u32, VecMap&lt;u32, vector&lt;u32&gt;&gt;&gt;,
    dwallet_cap_image_url: String,
    imported_key_dwallet_cap_image_url: String,
    unverified_presign_cap_image_url: String,
    verified_presign_cap_image_url: String,
    unverified_partial_user_signature_cap_image_url: String,
    verified_partial_user_signature_cap_image_url: String,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> <a href="../ika_system/ika_dwallet_2pc_mpc_init.md#(ika_dwallet_2pc_mpc=0x0)_ika_dwallet_2pc_mpc_init_InitCap">InitCap</a> { id, publisher } = init_cap;
    id.delete();
    <b>let</b> package_id_string = type_name::get&lt;<a href="../ika_system/ika_dwallet_2pc_mpc_init.md#(ika_dwallet_2pc_mpc=0x0)_ika_dwallet_2pc_mpc_init_InitCap">InitCap</a>&gt;().get_address().into_bytes();
    <b>let</b> package_id = address::from_ascii_bytes(&package_id_string).to_id();
    <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_create">coordinator::create</a>(
        package_id,
        advance_epoch_approver,
        pricing,
        supported_curves_to_signature_algorithms_to_hash_schemes,
        ctx,
    );
    <a href="../ika_system/display.md#(ika_dwallet_2pc_mpc=0x0)_ika_dwallet_2pc_mpc_display_create">ika_dwallet_2pc_mpc_display::create</a>(
        publisher,
        dwallet_cap_image_url,
        imported_key_dwallet_cap_image_url,
        unverified_presign_cap_image_url,
        verified_presign_cap_image_url,
        unverified_partial_user_signature_cap_image_url,
        verified_partial_user_signature_cap_image_url,
        ctx,
    );
}
</code></pre>



</details>
