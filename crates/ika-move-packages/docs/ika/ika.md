---
title: Module `0x0::ika`
---

Coin<IKA> is the token used to pay for gas in Ika.
It has 9 decimals, and the smallest unit (10^-9) is called "nika".
Module: ika


-  [Struct `IKA`](#0x0_ika_IKA)
-  [Constants](#@Constants_0)
-  [Function `init`](#0x0_ika_init)


<pre><code><b>use</b> <a href="../move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../sui-framework/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../sui-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../sui-framework/url.md#0x2_url">0x2::url</a>;
</code></pre>



<a name="0x0_ika_IKA"></a>

## Struct `IKA`

The OTW for the <code><a href="ika.md#0x0_ika_IKA">IKA</a></code> coin.


<pre><code><b>struct</b> <a href="ika.md#0x0_ika_IKA">IKA</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dummy_field: bool</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x0_ika_NIKA_PER_IKA"></a>

The amount of NIka per Ika token based on the fact that nika is
10^-9 of a Ika token


<pre><code><b>const</b> <a href="ika.md#0x0_ika_NIKA_PER_IKA">NIKA_PER_IKA</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1000000000;
</code></pre>



<a name="0x0_ika_init"></a>

## Function `init`



<pre><code><b>fun</b> <a href="ika.md#0x0_ika_init">init</a>(otw: <a href="ika.md#0x0_ika_IKA">ika::IKA</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="ika.md#0x0_ika_init">init</a>(otw: <a href="ika.md#0x0_ika_IKA">IKA</a>, ctx: &<b>mut</b> TxContext) {
    <b>let</b> (treasury_cap, coin_metadata) = <a href="../sui-framework/coin.md#0x2_coin_create_currency">coin::create_currency</a>(
        otw,
        9, // decimals,
        b"<a href="ika.md#0x0_ika_IKA">IKA</a>", // symbol,
        b"Ika", // name,
        b"<a href="ika.md#0x0_ika_IKA">IKA</a> Token", // description,
        <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(), // <a href="../sui-framework/url.md#0x2_url">url</a> (currently, empty)
        ctx,
    );

    <a href="../sui-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(treasury_cap, ctx.sender());
    <a href="../sui-framework/transfer.md#0x2_transfer_public_share_object">transfer::public_share_object</a>(coin_metadata);
}
</code></pre>



</details>
