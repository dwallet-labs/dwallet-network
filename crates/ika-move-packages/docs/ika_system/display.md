---
title: Module `(ika_system=0x0)::display`
---

Implements Sui Object Display for user-owned objects.

The default fields for Display are:
- name
- description
- image_url
- link
- project_url

Optionally:
- thumbnail_url
- creator


-  [Struct `ObjectDisplay`](#(ika_system=0x0)_display_ObjectDisplay)
-  [Struct `PublisherKey`](#(ika_system=0x0)_display_PublisherKey)
-  [Function `create`](#(ika_system=0x0)_display_create)
-  [Function `init_staked_ika_display`](#(ika_system=0x0)_display_init_staked_ika_display)
-  [Function `init_dwallet_cap_display`](#(ika_system=0x0)_display_init_dwallet_cap_display)
-  [Function `init_imported_key_dwallet_cap_display`](#(ika_system=0x0)_display_init_imported_key_dwallet_cap_display)
-  [Function `init_unverified_presign_cap_display`](#(ika_system=0x0)_display_init_unverified_presign_cap_display)
-  [Function `init_verified_presign_cap_display`](#(ika_system=0x0)_display_init_verified_presign_cap_display)
-  [Function `init_unverified_partial_user_signature_cap_display`](#(ika_system=0x0)_display_init_unverified_partial_user_signature_cap_display)
-  [Function `init_verified_partial_user_signature_cap_display`](#(ika_system=0x0)_display_init_verified_partial_user_signature_cap_display)


<pre><code><b>use</b> (ika=0x0)::ika;
<b>use</b> (ika_system=0x0)::<b>address</b>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee">bls_committee</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner">dwallet_2pc_mpc_coordinator_inner</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing">dwallet_pricing</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>;
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



<a name="(ika_system=0x0)_display_ObjectDisplay"></a>

## Struct `ObjectDisplay`

The wrapper that stores the objects.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/display.md#(ika_system=0x0)_display_ObjectDisplay">ObjectDisplay</a> <b>has</b> key
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
<code>inner: <a href="../sui/object_bag.md#sui_object_bag_ObjectBag">sui::object_bag::ObjectBag</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_display_PublisherKey"></a>

## Struct `PublisherKey`

The dynamic field key to use


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/display.md#(ika_system=0x0)_display_PublisherKey">PublisherKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="(ika_system=0x0)_display_create"></a>

## Function `create`

Creates the <code><a href="../ika_system/display.md#(ika_system=0x0)_display_ObjectDisplay">ObjectDisplay</a></code> instance with default objects in it.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/display.md#(ika_system=0x0)_display_create">create</a>(p: <a href="../sui/package.md#sui_package_Publisher">sui::package::Publisher</a>, staked_ika_image_url: <a href="../std/string.md#std_string_String">std::string::String</a>, dwallet_cap_image_url: <a href="../std/string.md#std_string_String">std::string::String</a>, imported_key_dwallet_cap_image_url: <a href="../std/string.md#std_string_String">std::string::String</a>, unverified_presign_cap_image_url: <a href="../std/string.md#std_string_String">std::string::String</a>, verified_presign_cap_image_url: <a href="../std/string.md#std_string_String">std::string::String</a>, unverified_partial_user_signature_cap_image_url: <a href="../std/string.md#std_string_String">std::string::String</a>, verified_partial_user_signature_cap_image_url: <a href="../std/string.md#std_string_String">std::string::String</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/display.md#(ika_system=0x0)_display_create">create</a>(
    p: Publisher,
    staked_ika_image_url: String,
    dwallet_cap_image_url: String,
    imported_key_dwallet_cap_image_url: String,
    unverified_presign_cap_image_url: String,
    verified_presign_cap_image_url: String,
    unverified_partial_user_signature_cap_image_url: String,
    verified_partial_user_signature_cap_image_url: String,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> <b>mut</b> inner = object_bag::new(ctx);
    inner.add(type_name::get&lt;StakedIka&gt;(), <a href="../ika_system/display.md#(ika_system=0x0)_display_init_staked_ika_display">init_staked_ika_display</a>(&p, staked_ika_image_url, ctx));
    inner.add(
        type_name::get&lt;DWalletCap&gt;(),
        <a href="../ika_system/display.md#(ika_system=0x0)_display_init_dwallet_cap_display">init_dwallet_cap_display</a>(&p, dwallet_cap_image_url, ctx),
    );
    inner.add(
        type_name::get&lt;ImportedKeyDWalletCap&gt;(),
        <a href="../ika_system/display.md#(ika_system=0x0)_display_init_imported_key_dwallet_cap_display">init_imported_key_dwallet_cap_display</a>(&p, imported_key_dwallet_cap_image_url, ctx),
    );
    inner.add(
        type_name::get&lt;UnverifiedPresignCap&gt;(),
        <a href="../ika_system/display.md#(ika_system=0x0)_display_init_unverified_presign_cap_display">init_unverified_presign_cap_display</a>(&p, unverified_presign_cap_image_url, ctx),
    );
    inner.add(
        type_name::get&lt;VerifiedPresignCap&gt;(),
        <a href="../ika_system/display.md#(ika_system=0x0)_display_init_verified_presign_cap_display">init_verified_presign_cap_display</a>(&p, verified_presign_cap_image_url, ctx),
    );
    inner.add(
        type_name::get&lt;UnverifiedPartialUserSignatureCap&gt;(),
        <a href="../ika_system/display.md#(ika_system=0x0)_display_init_unverified_partial_user_signature_cap_display">init_unverified_partial_user_signature_cap_display</a>(
            &p,
            unverified_partial_user_signature_cap_image_url,
            ctx,
        ),
    );
    inner.add(
        type_name::get&lt;VerifiedPartialUserSignatureCap&gt;(),
        <a href="../ika_system/display.md#(ika_system=0x0)_display_init_verified_partial_user_signature_cap_display">init_verified_partial_user_signature_cap_display</a>(
            &p,
            verified_partial_user_signature_cap_image_url,
            ctx,
        ),
    );
    inner.add(<a href="../ika_system/display.md#(ika_system=0x0)_display_PublisherKey">PublisherKey</a>(), p);
    transfer::share_object(<a href="../ika_system/display.md#(ika_system=0x0)_display_ObjectDisplay">ObjectDisplay</a> { id: object::new(ctx), inner })
}
</code></pre>



</details>

<a name="(ika_system=0x0)_display_init_staked_ika_display"></a>

## Function `init_staked_ika_display`

Creates initial <code>Display</code> for the <code>StakedIka</code> type.


<pre><code><b>fun</b> <a href="../ika_system/display.md#(ika_system=0x0)_display_init_staked_ika_display">init_staked_ika_display</a>(p: &<a href="../sui/package.md#sui_package_Publisher">sui::package::Publisher</a>, image_url: <a href="../std/string.md#std_string_String">std::string::String</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/display.md#sui_display_Display">sui::display::Display</a>&lt;(ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/display.md#(ika_system=0x0)_display_init_staked_ika_display">init_staked_ika_display</a>(
    p: &Publisher,
    image_url: String,
    ctx: &<b>mut</b> TxContext,
): Display&lt;StakedIka&gt; {
    <b>let</b> <b>mut</b> d = display::new(p, ctx);
    d.add(b"name".to_string(), b"Staked IKA ({principal} INKU)".to_string());
    d.add(
        b"description".to_string(),
        b"Staked <b>for</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: {validator_id}, activates at: {activation_epoch}".to_string(),
    );
    d.add(b"image_url".to_string(), image_url);
    d.add(b"project_url".to_string(), b"https://ika.xyz/".to_string());
    d.add(b"link".to_string(), b"".to_string());
    d.update_version();
    d
}
</code></pre>



</details>

<a name="(ika_system=0x0)_display_init_dwallet_cap_display"></a>

## Function `init_dwallet_cap_display`

Creates initial <code>Display</code> for the <code>DWalletCap</code> type.


<pre><code><b>fun</b> <a href="../ika_system/display.md#(ika_system=0x0)_display_init_dwallet_cap_display">init_dwallet_cap_display</a>(p: &<a href="../sui/package.md#sui_package_Publisher">sui::package::Publisher</a>, image_url: <a href="../std/string.md#std_string_String">std::string::String</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/display.md#sui_display_Display">sui::display::Display</a>&lt;(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">dwallet_2pc_mpc_coordinator_inner::DWalletCap</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/display.md#(ika_system=0x0)_display_init_dwallet_cap_display">init_dwallet_cap_display</a>(
    p: &Publisher,
    image_url: String,
    ctx: &<b>mut</b> TxContext,
): Display&lt;DWalletCap&gt; {
    <b>let</b> <b>mut</b> d = display::new(p, ctx);
    d.add(b"name".to_string(), b"DWallet Cap".to_string());
    d.add(
        b"description".to_string(),
        b"DWallet cap <b>for</b>: {dwallet_id}".to_string(),
    );
    d.add(b"image_url".to_string(), image_url);
    d.add(b"project_url".to_string(), b"https://ika.xyz/".to_string());
    d.add(b"link".to_string(), b"".to_string());
    d.update_version();
    d
}
</code></pre>



</details>

<a name="(ika_system=0x0)_display_init_imported_key_dwallet_cap_display"></a>

## Function `init_imported_key_dwallet_cap_display`

Creates initial <code>Display</code> for the <code>ImportedKeyDWalletCap</code> type.


<pre><code><b>fun</b> <a href="../ika_system/display.md#(ika_system=0x0)_display_init_imported_key_dwallet_cap_display">init_imported_key_dwallet_cap_display</a>(p: &<a href="../sui/package.md#sui_package_Publisher">sui::package::Publisher</a>, image_url: <a href="../std/string.md#std_string_String">std::string::String</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/display.md#sui_display_Display">sui::display::Display</a>&lt;(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyDWalletCap">dwallet_2pc_mpc_coordinator_inner::ImportedKeyDWalletCap</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/display.md#(ika_system=0x0)_display_init_imported_key_dwallet_cap_display">init_imported_key_dwallet_cap_display</a>(
    p: &Publisher,
    image_url: String,
    ctx: &<b>mut</b> TxContext,
): Display&lt;ImportedKeyDWalletCap&gt; {
    <b>let</b> <b>mut</b> d = display::new(p, ctx);
    d.add(b"name".to_string(), b"Imported Key DWallet Cap".to_string());
    d.add(
        b"description".to_string(),
        b"Imported key dWallet cap <b>for</b>: {dwallet_id}".to_string(),
    );
    d.add(b"image_url".to_string(), image_url);
    d.add(b"project_url".to_string(), b"https://ika.xyz/".to_string());
    d.add(b"link".to_string(), b"".to_string());
    d.update_version();
    d
}
</code></pre>



</details>

<a name="(ika_system=0x0)_display_init_unverified_presign_cap_display"></a>

## Function `init_unverified_presign_cap_display`

Creates initial <code>Display</code> for the <code>UnverifiedPresignCap</code> type.


<pre><code><b>fun</b> <a href="../ika_system/display.md#(ika_system=0x0)_display_init_unverified_presign_cap_display">init_unverified_presign_cap_display</a>(p: &<a href="../sui/package.md#sui_package_Publisher">sui::package::Publisher</a>, image_url: <a href="../std/string.md#std_string_String">std::string::String</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/display.md#sui_display_Display">sui::display::Display</a>&lt;(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">dwallet_2pc_mpc_coordinator_inner::UnverifiedPresignCap</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/display.md#(ika_system=0x0)_display_init_unverified_presign_cap_display">init_unverified_presign_cap_display</a>(
    p: &Publisher,
    image_url: String,
    ctx: &<b>mut</b> TxContext,
): Display&lt;UnverifiedPresignCap&gt; {
    <b>let</b> <b>mut</b> d = display::new(p, ctx);
    d.add(b"name".to_string(), b"Unverified Presign Cap".to_string());
    d.add(
        b"description".to_string(),
        b"Unverified presign cap <b>for</b>: {presign_id}, dWallet: {dwallet_id}".to_string(),
    );
    d.add(b"image_url".to_string(), image_url);
    d.add(b"project_url".to_string(), b"https://ika.xyz/".to_string());
    d.add(b"link".to_string(), b"".to_string());
    d.update_version();
    d
}
</code></pre>



</details>

<a name="(ika_system=0x0)_display_init_verified_presign_cap_display"></a>

## Function `init_verified_presign_cap_display`

Creates initial <code>Display</code> for the <code>VerifiedPresignCap</code> type.


<pre><code><b>fun</b> <a href="../ika_system/display.md#(ika_system=0x0)_display_init_verified_presign_cap_display">init_verified_presign_cap_display</a>(p: &<a href="../sui/package.md#sui_package_Publisher">sui::package::Publisher</a>, image_url: <a href="../std/string.md#std_string_String">std::string::String</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/display.md#sui_display_Display">sui::display::Display</a>&lt;(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">dwallet_2pc_mpc_coordinator_inner::VerifiedPresignCap</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/display.md#(ika_system=0x0)_display_init_verified_presign_cap_display">init_verified_presign_cap_display</a>(
    p: &Publisher,
    image_url: String,
    ctx: &<b>mut</b> TxContext,
): Display&lt;VerifiedPresignCap&gt; {
    <b>let</b> <b>mut</b> d = display::new(p, ctx);
    d.add(b"name".to_string(), b"Verified Presign Cap".to_string());
    d.add(
        b"description".to_string(),
        b"Verified presign cap <b>for</b>: {presign_id}, dWallet: {dwallet_id}".to_string(),
    );
    d.add(b"image_url".to_string(), image_url);
    d.add(b"project_url".to_string(), b"https://ika.xyz/".to_string());
    d.add(b"link".to_string(), b"".to_string());
    d.update_version();
    d
}
</code></pre>



</details>

<a name="(ika_system=0x0)_display_init_unverified_partial_user_signature_cap_display"></a>

## Function `init_unverified_partial_user_signature_cap_display`

Creates initial <code>Display</code> for the <code>UnverifiedPartialUserSignatureCap</code> type.


<pre><code><b>fun</b> <a href="../ika_system/display.md#(ika_system=0x0)_display_init_unverified_partial_user_signature_cap_display">init_unverified_partial_user_signature_cap_display</a>(p: &<a href="../sui/package.md#sui_package_Publisher">sui::package::Publisher</a>, image_url: <a href="../std/string.md#std_string_String">std::string::String</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/display.md#sui_display_Display">sui::display::Display</a>&lt;(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPartialUserSignatureCap">dwallet_2pc_mpc_coordinator_inner::UnverifiedPartialUserSignatureCap</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/display.md#(ika_system=0x0)_display_init_unverified_partial_user_signature_cap_display">init_unverified_partial_user_signature_cap_display</a>(
    p: &Publisher,
    image_url: String,
    ctx: &<b>mut</b> TxContext,
): Display&lt;UnverifiedPartialUserSignatureCap&gt; {
    <b>let</b> <b>mut</b> d = display::new(p, ctx);
    d.add(b"name".to_string(), b"Unverified Partial User Signature Cap".to_string());
    d.add(
        b"description".to_string(),
        b"Unverified partial user signature cap <b>for</b>: {partial_centralized_signed_message_id}".to_string(),
    );
    d.add(b"image_url".to_string(), image_url);
    d.add(b"project_url".to_string(), b"https://ika.xyz/".to_string());
    d.add(b"link".to_string(), b"".to_string());
    d.update_version();
    d
}
</code></pre>



</details>

<a name="(ika_system=0x0)_display_init_verified_partial_user_signature_cap_display"></a>

## Function `init_verified_partial_user_signature_cap_display`

Creates initial <code>Display</code> for the <code>VerifiedPartialUserSignatureCap</code> type.


<pre><code><b>fun</b> <a href="../ika_system/display.md#(ika_system=0x0)_display_init_verified_partial_user_signature_cap_display">init_verified_partial_user_signature_cap_display</a>(p: &<a href="../sui/package.md#sui_package_Publisher">sui::package::Publisher</a>, image_url: <a href="../std/string.md#std_string_String">std::string::String</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/display.md#sui_display_Display">sui::display::Display</a>&lt;(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap">dwallet_2pc_mpc_coordinator_inner::VerifiedPartialUserSignatureCap</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/display.md#(ika_system=0x0)_display_init_verified_partial_user_signature_cap_display">init_verified_partial_user_signature_cap_display</a>(
    p: &Publisher,
    image_url: String,
    ctx: &<b>mut</b> TxContext,
): Display&lt;VerifiedPartialUserSignatureCap&gt; {
    <b>let</b> <b>mut</b> d = display::new(p, ctx);
    d.add(b"name".to_string(), b"Verified Partial User Signature Cap".to_string());
    d.add(
        b"description".to_string(),
        b"Verified partial user signature cap <b>for</b>: {partial_centralized_signed_message_id}".to_string(),
    );
    d.add(b"image_url".to_string(), image_url);
    d.add(b"project_url".to_string(), b"https://ika.xyz/".to_string());
    d.add(b"link".to_string(), b"".to_string());
    d.update_version();
    d
}
</code></pre>



</details>
