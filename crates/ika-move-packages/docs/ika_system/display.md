---
title: Module `(ika=0x0)::display`
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


-  [Struct `ObjectDisplay`](#(ika=0x0)_display_ObjectDisplay)
-  [Struct `PublisherKey`](#(ika=0x0)_display_PublisherKey)
-  [Function `create`](#(ika=0x0)_display_create)
-  [Function `init_staked_ika_display`](#(ika=0x0)_display_init_staked_ika_display)


<pre><code><b>use</b> (ika=0x0)::ika;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../sui/address.md#sui_address">sui::address</a>;
<b>use</b> <a href="../sui/bag.md#sui_bag">sui::bag</a>;
<b>use</b> <a href="../sui/balance.md#sui_balance">sui::balance</a>;
<b>use</b> <a href="../sui/coin.md#sui_coin">sui::coin</a>;
<b>use</b> <a href="../sui/config.md#sui_config">sui::config</a>;
<b>use</b> <a href="../sui/deny_list.md#sui_deny_list">sui::deny_list</a>;
<b>use</b> <a href="../sui/display.md#sui_display">sui::display</a>;
<b>use</b> <a href="../sui/dynamic_field.md#sui_dynamic_field">sui::dynamic_field</a>;
<b>use</b> <a href="../sui/dynamic_object_field.md#sui_dynamic_object_field">sui::dynamic_object_field</a>;
<b>use</b> <a href="../sui/event.md#sui_event">sui::event</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/object_bag.md#sui_object_bag">sui::object_bag</a>;
<b>use</b> <a href="../sui/package.md#sui_package">sui::package</a>;
<b>use</b> <a href="../sui/table.md#sui_table">sui::table</a>;
<b>use</b> <a href="../sui/transfer.md#sui_transfer">sui::transfer</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
<b>use</b> <a href="../sui/types.md#sui_types">sui::types</a>;
<b>use</b> <a href="../sui/url.md#sui_url">sui::url</a>;
<b>use</b> <a href="../sui/vec_map.md#sui_vec_map">sui::vec_map</a>;
<b>use</b> <a href="../sui/vec_set.md#sui_vec_set">sui::vec_set</a>;
</code></pre>



<a name="(ika=0x0)_display_ObjectDisplay"></a>

## Struct `ObjectDisplay`

The wrapper that stores the objects.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/display.md#(ika=0x0)_display_ObjectDisplay">ObjectDisplay</a> <b>has</b> key
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

<a name="(ika=0x0)_display_PublisherKey"></a>

## Struct `PublisherKey`

The dynamic field key to use


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/display.md#(ika=0x0)_display_PublisherKey">PublisherKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="(ika=0x0)_display_create"></a>

## Function `create`

Creates the <code><a href="../ika_system/display.md#(ika=0x0)_display_ObjectDisplay">ObjectDisplay</a></code> instance with default objects in it.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/display.md#(ika=0x0)_display_create">create</a>(p: <a href="../sui/package.md#sui_package_Publisher">sui::package::Publisher</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/display.md#(ika=0x0)_display_create">create</a>(p: Publisher, ctx: &<b>mut</b> TxContext) {
    <b>let</b> <b>mut</b> inner = object_bag::new(ctx);
    inner.add(type_name::get&lt;StakedIka&gt;(), <a href="../ika_system/display.md#(ika=0x0)_display_init_staked_ika_display">init_staked_ika_display</a>(&p, ctx));
    inner.add(<a href="../ika_system/display.md#(ika=0x0)_display_PublisherKey">PublisherKey</a>(), p);
    transfer::share_object(<a href="../ika_system/display.md#(ika=0x0)_display_ObjectDisplay">ObjectDisplay</a> { id: object::new(ctx), inner })
}
</code></pre>



</details>

<a name="(ika=0x0)_display_init_staked_ika_display"></a>

## Function `init_staked_ika_display`

Creates initial <code>Display</code> for the <code>StakedIka</code> type.


<pre><code><b>fun</b> <a href="../ika_system/display.md#(ika=0x0)_display_init_staked_ika_display">init_staked_ika_display</a>(p: &<a href="../sui/package.md#sui_package_Publisher">sui::package::Publisher</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/display.md#sui_display_Display">sui::display::Display</a>&lt;(ika=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/display.md#(ika=0x0)_display_init_staked_ika_display">init_staked_ika_display</a>(p: &Publisher, ctx: &<b>mut</b> TxContext): Display&lt;StakedIka&gt; {
    <b>let</b> <b>mut</b> d = display::new(p, ctx);
    d.add(b"name".to_string(), b"Staked IKA ({principal} INKU)".to_string());
    d.add(
        b"description".to_string(),
        b"Staked <b>for</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>: {validator_id}, activates at: {activation_epoch}".to_string(),
    );
    d.add(b"image_url".to_string(), <a href="../ika_system/ika.md#(ika=0x0)_ika_get_staked_ika_icon_url">ika::ika::get_staked_ika_icon_url</a>().to_string());
    d.add(b"project_url".to_string(), b"https://ika.xyz/".to_string());
    d.add(b"link".to_string(), b"".to_string());
    d.update_version();
    d
}
</code></pre>



</details>
