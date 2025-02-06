---
title: Module `ika::ika`
---

Coin<IKA> is the token used to pay for gas in Ika.
It has 9 decimals, and the smallest unit (10^-9) is called "nika".
Module: ika


-  [Struct `IKA`](#ika_ika_IKA)
-  [Constants](#@Constants_0)
-  [Function `init`](#ika_ika_init)


<pre><code><b>use</b> <a href="../../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../../sui/address.md#sui_address">sui::address</a>;
<b>use</b> <a href="../../sui/bag.md#sui_bag">sui::bag</a>;
<b>use</b> <a href="../../sui/balance.md#sui_balance">sui::balance</a>;
<b>use</b> <a href="../../sui/coin.md#sui_coin">sui::coin</a>;
<b>use</b> <a href="../../sui/config.md#sui_config">sui::config</a>;
<b>use</b> <a href="../../sui/deny_list.md#sui_deny_list">sui::deny_list</a>;
<b>use</b> <a href="../../sui/dynamic_field.md#sui_dynamic_field">sui::dynamic_field</a>;
<b>use</b> <a href="../../sui/dynamic_object_field.md#sui_dynamic_object_field">sui::dynamic_object_field</a>;
<b>use</b> <a href="../../sui/event.md#sui_event">sui::event</a>;
<b>use</b> <a href="../../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../../sui/table.md#sui_table">sui::table</a>;
<b>use</b> <a href="../../sui/transfer.md#sui_transfer">sui::transfer</a>;
<b>use</b> <a href="../../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
<b>use</b> <a href="../../sui/types.md#sui_types">sui::types</a>;
<b>use</b> <a href="../../sui/url.md#sui_url">sui::url</a>;
<b>use</b> <a href="../../sui/vec_set.md#sui_vec_set">sui::vec_set</a>;
</code></pre>



<a name="ika_ika_IKA"></a>

## Struct `IKA`

The OTW for the <code><a href="../ika/ika.md#ika_ika_IKA">IKA</a></code> coin.


<pre><code><b>public</b> <b>struct</b> <a href="../ika/ika.md#ika_ika_IKA">IKA</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="ika_ika_NIKA_PER_IKA"></a>

The amount of NIka per Ika token based on the fact that nika is
10^-9 of a Ika token


<pre><code><b>const</b> <a href="../ika/ika.md#ika_ika_NIKA_PER_IKA">NIKA_PER_IKA</a>: u64 = 1000000000;
</code></pre>



<a name="ika_ika_init"></a>

## Function `init`



<pre><code><b>fun</b> <a href="../ika/ika.md#ika_ika_init">init</a>(otw: <a href="../ika/ika.md#ika_ika_IKA">ika::ika::IKA</a>, ctx: &<b>mut</b> <a href="../../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika/ika.md#ika_ika_init">init</a>(otw: <a href="../ika/ika.md#ika_ika_IKA">IKA</a>, ctx: &<b>mut</b> TxContext) {
    <b>let</b> (treasury_cap, coin_metadata) = coin::create_currency(
        otw,
        9, // decimals,
        b"<a href="../ika/ika.md#ika_ika_IKA">IKA</a>", // symbol,
        b"Ika", // name,
        b"<a href="../ika/ika.md#ika_ika_IKA">IKA</a> Token", // description,
        option::none(), // url (currently, empty)
        ctx,
    );
    transfer::public_transfer(treasury_cap, ctx.sender());
    transfer::public_share_object(coin_metadata);
}
</code></pre>



</details>
