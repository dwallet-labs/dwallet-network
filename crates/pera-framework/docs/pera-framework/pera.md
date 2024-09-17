---
title: Module `0x2::pera`
---

Coin<PERA> is the token used to pay for gas in Pera.
It has 9 decimals, and the smallest unit (10^-9) is called "npera".


-  [Struct `PERA`](#0x2_pera_PERA)
-  [Constants](#@Constants_0)
-  [Function `new`](#0x2_pera_new)
-  [Function `transfer`](#0x2_pera_transfer)


<pre><code><b>use</b> <a href="../move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../pera-framework/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../pera-framework/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../pera-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../pera-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../pera-framework/url.md#0x2_url">0x2::url</a>;
</code></pre>



<a name="0x2_pera_PERA"></a>

## Struct `PERA`

Name of the coin


<pre><code><b>struct</b> <a href="../pera-framework/pera.md#0x2_pera_PERA">PERA</a> <b>has</b> drop
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


<a name="0x2_pera_ENotSystemAddress"></a>

Sender is not @0x0 the system address.


<pre><code><b>const</b> <a href="../pera-framework/pera.md#0x2_pera_ENotSystemAddress">ENotSystemAddress</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1;
</code></pre>



<a name="0x2_pera_EAlreadyMinted"></a>



<pre><code><b>const</b> <a href="../pera-framework/pera.md#0x2_pera_EAlreadyMinted">EAlreadyMinted</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
</code></pre>



<a name="0x2_pera_NPERA_PER_PERA"></a>

The amount of NPera per Pera token based on the fact that npera is
10^-9 of a Pera token


<pre><code><b>const</b> <a href="../pera-framework/pera.md#0x2_pera_NPERA_PER_PERA">NPERA_PER_PERA</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1000000000;
</code></pre>



<a name="0x2_pera_TOTAL_SUPPLY_NPERA"></a>

The total supply of Pera denominated in NPera (10 Billion * 10^9)


<pre><code><b>const</b> <a href="../pera-framework/pera.md#0x2_pera_TOTAL_SUPPLY_NPERA">TOTAL_SUPPLY_NPERA</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 10000000000000000000;
</code></pre>



<a name="0x2_pera_TOTAL_SUPPLY_PERA"></a>

The total supply of Pera denominated in whole Pera tokens (10 Billion)


<pre><code><b>const</b> <a href="../pera-framework/pera.md#0x2_pera_TOTAL_SUPPLY_PERA">TOTAL_SUPPLY_PERA</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 10000000000;
</code></pre>



<a name="0x2_pera_new"></a>

## Function `new`

Register the <code><a href="../pera-framework/pera.md#0x2_pera_PERA">PERA</a></code> Coin to acquire its <code>Supply</code>.
This should be called only once during genesis creation.


<pre><code><b>fun</b> <a href="../pera-framework/pera.md#0x2_pera_new">new</a>(ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../pera-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../pera-framework/pera.md#0x2_pera_PERA">pera::PERA</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../pera-framework/pera.md#0x2_pera_new">new</a>(ctx: &<b>mut</b> TxContext): Balance&lt;<a href="../pera-framework/pera.md#0x2_pera_PERA">PERA</a>&gt; {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../pera-framework/pera.md#0x2_pera_ENotSystemAddress">ENotSystemAddress</a>);
    <b>assert</b>!(ctx.epoch() == 0, <a href="../pera-framework/pera.md#0x2_pera_EAlreadyMinted">EAlreadyMinted</a>);

    <b>let</b> (treasury, metadata) = <a href="../pera-framework/coin.md#0x2_coin_create_currency">coin::create_currency</a>(
        <a href="../pera-framework/pera.md#0x2_pera_PERA">PERA</a> {},
        9,
        b"<a href="../pera-framework/pera.md#0x2_pera_PERA">PERA</a>",
        b"Pera",
        // TODO: add appropriate description and logo <a href="../pera-framework/url.md#0x2_url">url</a>
        b"",
        <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(),
        ctx
    );
    <a href="../pera-framework/transfer.md#0x2_transfer_public_freeze_object">transfer::public_freeze_object</a>(metadata);
    <b>let</b> <b>mut</b> supply = treasury.treasury_into_supply();
    <b>let</b> total_pera = supply.increase_supply(<a href="../pera-framework/pera.md#0x2_pera_TOTAL_SUPPLY_NPERA">TOTAL_SUPPLY_NPERA</a>);
    supply.destroy_supply();
    total_pera
}
</code></pre>



</details>

<a name="0x2_pera_transfer"></a>

## Function `transfer`



<pre><code><b>public</b> entry <b>fun</b> <a href="../pera-framework/transfer.md#0x2_transfer">transfer</a>(c: <a href="../pera-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../pera-framework/pera.md#0x2_pera_PERA">pera::PERA</a>&gt;, recipient: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../pera-framework/transfer.md#0x2_transfer">transfer</a>(c: <a href="../pera-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../pera-framework/pera.md#0x2_pera_PERA">PERA</a>&gt;, recipient: <b>address</b>) {
    <a href="../pera-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(c, recipient)
}
</code></pre>



</details>
