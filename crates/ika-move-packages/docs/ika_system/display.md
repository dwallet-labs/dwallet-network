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


<pre><code><b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../sui/address.md#sui_address">sui::address</a>;
<b>use</b> <a href="../sui/dynamic_field.md#sui_dynamic_field">sui::dynamic_field</a>;
<b>use</b> <a href="../sui/dynamic_object_field.md#sui_dynamic_object_field">sui::dynamic_object_field</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/object_bag.md#sui_object_bag">sui::object_bag</a>;
<b>use</b> <a href="../sui/package.md#sui_package">sui::package</a>;
<b>use</b> <a href="../sui/party.md#sui_party">sui::party</a>;
<b>use</b> <a href="../sui/transfer.md#sui_transfer">sui::transfer</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
<b>use</b> <a href="../sui/types.md#sui_types">sui::types</a>;
<b>use</b> <a href="../sui/vec_map.md#sui_vec_map">sui::vec_map</a>;
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
    // inner.add(type_name::get&lt;StakedIka&gt;(), init_staked_ika_display(&p, staked_ika_image_url, ctx));
    // inner.add(
    //     type_name::get&lt;DWalletCap&gt;(),
    //     init_dwallet_cap_display(&p, dwallet_cap_image_url, ctx),
    // );
    // inner.add(
    //     type_name::get&lt;ImportedKeyDWalletCap&gt;(),
    //     init_imported_key_dwallet_cap_display(&p, imported_key_dwallet_cap_image_url, ctx),
    // );
    // inner.add(
    //     type_name::get&lt;UnverifiedPresignCap&gt;(),
    //     init_unverified_presign_cap_display(&p, unverified_presign_cap_image_url, ctx),
    // );
    // inner.add(
    //     type_name::get&lt;VerifiedPresignCap&gt;(),
    //     init_verified_presign_cap_display(&p, verified_presign_cap_image_url, ctx),
    // );
    // inner.add(
    //     type_name::get&lt;UnverifiedPartialUserSignatureCap&gt;(),
    //     init_unverified_partial_user_signature_cap_display(
    //         &p,
    //         unverified_partial_user_signature_cap_image_url,
    //         ctx,
    //     ),
    // );
    // inner.add(
    //     type_name::get&lt;VerifiedPartialUserSignatureCap&gt;(),
    //     init_verified_partial_user_signature_cap_display(
    //         &p,
    //         verified_partial_user_signature_cap_image_url,
    //         ctx,
    //     ),
    // );
    inner.add(<a href="../ika_system/display.md#(ika_system=0x0)_display_PublisherKey">PublisherKey</a>(), p);
    transfer::share_object(<a href="../ika_system/display.md#(ika_system=0x0)_display_ObjectDisplay">ObjectDisplay</a> { id: object::new(ctx), inner })
}
</code></pre>



</details>
