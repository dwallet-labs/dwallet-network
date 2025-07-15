---
title: Module `(ika_system=0x0)::token_exchange_rate`
---

A utility module which implements an <code>ExchangeRate</code> struct and its methods.
It stores a fixed point exchange rate between the IKA token and validator shares.


-  [Enum `TokenExchangeRate`](#(ika_system=0x0)_token_exchange_rate_TokenExchangeRate)
-  [Constants](#@Constants_0)
-  [Function `flat`](#(ika_system=0x0)_token_exchange_rate_flat)
-  [Function `new`](#(ika_system=0x0)_token_exchange_rate_new)
-  [Function `convert_to_ika_amount`](#(ika_system=0x0)_token_exchange_rate_convert_to_ika_amount)
-  [Function `convert_to_share_amount`](#(ika_system=0x0)_token_exchange_rate_convert_to_share_amount)


<pre><code></code></pre>



<a name="(ika_system=0x0)_token_exchange_rate_TokenExchangeRate"></a>

## Enum `TokenExchangeRate`

Represents the exchange rate for the staking validator.


<pre><code><b>public</b> <b>enum</b> <a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_TokenExchangeRate">TokenExchangeRate</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Variants</summary>


<dl>
<dt>
Variant <code>Flat</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>Variable</code>
</dt>
<dd>
</dd>

<dl>
<dt>
<code>ika_amount: u128</code>
</dt>
<dd>
 Amount of staked IKA tokens + rewards.
</dd>
</dl>


<dl>
<dt>
<code>share_amount: u128</code>
</dt>
<dd>
 Amount of total shares in the validator (<= ika_amount, as long as slashing is not
 implemented).
</dd>
</dl>

</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="(ika_system=0x0)_token_exchange_rate_EInvalidRate"></a>

The exchange rate between the shares and the IKA token is invalid.


<pre><code><b>const</b> <a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_EInvalidRate">EInvalidRate</a>: u64 = 0;
</code></pre>



<a name="(ika_system=0x0)_token_exchange_rate_flat"></a>

## Function `flat`

Create an empty exchange rate.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_flat">flat</a>(): (ika_system=0x0)::<a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_TokenExchangeRate">token_exchange_rate::TokenExchangeRate</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_flat">flat</a>(): <a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_TokenExchangeRate">TokenExchangeRate</a> {
    TokenExchangeRate::Flat
}
</code></pre>



</details>

<a name="(ika_system=0x0)_token_exchange_rate_new"></a>

## Function `new`

Create a new exchange rate with the given amounts.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_new">new</a>(ika_amount: u64, share_amount: u64): (ika_system=0x0)::<a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_TokenExchangeRate">token_exchange_rate::TokenExchangeRate</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_new">new</a>(ika_amount: u64, share_amount: u64): <a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_TokenExchangeRate">TokenExchangeRate</a> {
    // validator_token_amount &lt;= ika_amount <b>as</b> long <b>as</b> slashing is not implemented.
    <b>assert</b>!(share_amount &lt;= ika_amount, <a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_EInvalidRate">EInvalidRate</a>);
    <b>if</b> (ika_amount == 0 || share_amount == 0) {
        TokenExchangeRate::Flat
    } <b>else</b> {
        TokenExchangeRate::Variable {
            ika_amount: (ika_amount <b>as</b> u128),
            share_amount: (share_amount <b>as</b> u128),
        }
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_token_exchange_rate_convert_to_ika_amount"></a>

## Function `convert_to_ika_amount`

Assumptions:
- amount is at most the amount of shares in the validator


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_convert_to_ika_amount">convert_to_ika_amount</a>(exchange_rate: &(ika_system=0x0)::<a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_TokenExchangeRate">token_exchange_rate::TokenExchangeRate</a>, amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_convert_to_ika_amount">convert_to_ika_amount</a>(exchange_rate: &<a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_TokenExchangeRate">TokenExchangeRate</a>, amount: u64): u64 {
    match (exchange_rate) {
        TokenExchangeRate::Flat =&gt; amount,
        TokenExchangeRate::Variable { ika_amount, share_amount } =&gt; {
            <b>let</b> amount = (amount <b>as</b> u128);
            <b>let</b> res = (amount * *ika_amount) / *share_amount;
            res <b>as</b> u64
        },
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_token_exchange_rate_convert_to_share_amount"></a>

## Function `convert_to_share_amount`

Assumptions:
- amount is at most the amount of IKA in the validator


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_convert_to_share_amount">convert_to_share_amount</a>(exchange_rate: &(ika_system=0x0)::<a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_TokenExchangeRate">token_exchange_rate::TokenExchangeRate</a>, amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_convert_to_share_amount">convert_to_share_amount</a>(exchange_rate: &<a href="../ika_system/token_exchange_rate.md#(ika_system=0x0)_token_exchange_rate_TokenExchangeRate">TokenExchangeRate</a>, amount: u64): u64 {
    match (exchange_rate) {
        TokenExchangeRate::Flat =&gt; amount,
        TokenExchangeRate::Variable { ika_amount, share_amount } =&gt; {
            <b>let</b> amount = (amount <b>as</b> u128);
            <b>let</b> res = (amount * *share_amount) / *ika_amount;
            res <b>as</b> u64
        },
    }
}
</code></pre>



</details>
