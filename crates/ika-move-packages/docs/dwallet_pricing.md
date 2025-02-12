---
title: Module `(ika_system=0x0)::dwallet_pricing`
---



-  [Struct `DWalletPricing2PcMpcSecp256K1`](#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1)
-  [Function `create_dwallet_pricing_2pc_mpc_secp256k1`](#(ika_system=0x0)_dwallet_pricing_create_dwallet_pricing_2pc_mpc_secp256k1)
-  [Function `computation_ika_price_per_dkg`](#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_dkg)
-  [Function `computation_sui_price_per_dkg`](#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_dkg)
-  [Function `computation_ika_price_per_re_encrypt_user_share`](#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_re_encrypt_user_share)
-  [Function `computation_sui_price_per_re_encrypt_user_share`](#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_re_encrypt_user_share)
-  [Function `computation_ika_price_per_ecdsa_presign`](#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_presign)
-  [Function `computation_sui_price_per_ecdsa_presign`](#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_presign)
-  [Function `computation_ika_price_per_ecdsa_sign`](#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_sign)
-  [Function `computation_sui_price_per_ecdsa_sign`](#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_sign)
-  [Function `computation_ika_price_per_ecdsa_future_sign`](#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_future_sign)
-  [Function `computation_sui_price_per_ecdsa_future_sign`](#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_future_sign)
-  [Function `computation_ika_price_per_ecdsa_sign_with_partial_user_signature`](#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_sign_with_partial_user_signature)
-  [Function `computation_sui_price_per_ecdsa_sign_with_partial_user_signature`](#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_sign_with_partial_user_signature)


<pre><code><b>use</b> <a href="../../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../../sui/address.md#sui_address">sui::address</a>;
<b>use</b> <a href="../../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
</code></pre>



<a name="(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1"></a>

## Struct `DWalletPricing2PcMpcSecp256K1`

Represents a capability granting control over a specific dWallet.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_dkg">computation_ika_price_per_dkg</a>: u64</code>
</dt>
<dd>
 The computation IKA price per dkg for the current epoch.
</dd>
<dt>
<code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_dkg">computation_sui_price_per_dkg</a>: u64</code>
</dt>
<dd>
 The computation SUI price per dkg for the current epoch.
</dd>
<dt>
<code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_re_encrypt_user_share">computation_ika_price_per_re_encrypt_user_share</a>: u64</code>
</dt>
<dd>
 The computation IKA price per re-encrypt user share for the current epoch.
</dd>
<dt>
<code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_re_encrypt_user_share">computation_sui_price_per_re_encrypt_user_share</a>: u64</code>
</dt>
<dd>
 The computation SUI price per re-encrypt user share for the current epoch.
</dd>
<dt>
<code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_presign">computation_ika_price_per_ecdsa_presign</a>: u64</code>
</dt>
<dd>
 The computation IKA price per ecdsa presign for the current epoch.
</dd>
<dt>
<code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_presign">computation_sui_price_per_ecdsa_presign</a>: u64</code>
</dt>
<dd>
 The computation SUI price per ecdsa presign for the current epoch.
</dd>
<dt>
<code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_sign">computation_ika_price_per_ecdsa_sign</a>: u64</code>
</dt>
<dd>
 The computation IKA price per ecdsa sign for the current epoch.
</dd>
<dt>
<code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_sign">computation_sui_price_per_ecdsa_sign</a>: u64</code>
</dt>
<dd>
 The computation SUI price per ecdsa sign for the current epoch.
</dd>
<dt>
<code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_future_sign">computation_ika_price_per_ecdsa_future_sign</a>: u64</code>
</dt>
<dd>
 The computation IKA price per ecdsa future sign for the current epoch.
</dd>
<dt>
<code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_future_sign">computation_sui_price_per_ecdsa_future_sign</a>: u64</code>
</dt>
<dd>
 The computation SUI price per ecdsa future sign for the current epoch.
</dd>
<dt>
<code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_sign_with_partial_user_signature">computation_ika_price_per_ecdsa_sign_with_partial_user_signature</a>: u64</code>
</dt>
<dd>
 The computation IKA price per ecdsa sign with partial user signature for the current epoch.
</dd>
<dt>
<code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_sign_with_partial_user_signature">computation_sui_price_per_ecdsa_sign_with_partial_user_signature</a>: u64</code>
</dt>
<dd>
 The computation SUI price per ecdsa sign with partial user signature for the current epoch.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_pricing_create_dwallet_pricing_2pc_mpc_secp256k1"></a>

## Function `create_dwallet_pricing_2pc_mpc_secp256k1`

Create a new [<code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a></code>] object.

Holds the pricing for the current epoch.


<a name="@Returns_0"></a>

##### Returns

The newly created <code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a></code> object.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_create_dwallet_pricing_2pc_mpc_secp256k1">create_dwallet_pricing_2pc_mpc_secp256k1</a>(<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_dkg">computation_ika_price_per_dkg</a>: u64, <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_dkg">computation_sui_price_per_dkg</a>: u64, <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_re_encrypt_user_share">computation_ika_price_per_re_encrypt_user_share</a>: u64, <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_re_encrypt_user_share">computation_sui_price_per_re_encrypt_user_share</a>: u64, <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_presign">computation_ika_price_per_ecdsa_presign</a>: u64, <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_presign">computation_sui_price_per_ecdsa_presign</a>: u64, <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_sign">computation_ika_price_per_ecdsa_sign</a>: u64, <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_sign">computation_sui_price_per_ecdsa_sign</a>: u64, <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_future_sign">computation_ika_price_per_ecdsa_future_sign</a>: u64, <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_future_sign">computation_sui_price_per_ecdsa_future_sign</a>: u64, <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_sign_with_partial_user_signature">computation_ika_price_per_ecdsa_sign_with_partial_user_signature</a>: u64, <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_sign_with_partial_user_signature">computation_sui_price_per_ecdsa_sign_with_partial_user_signature</a>: u64, ctx: &<b>mut</b> <a href="../../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">dwallet_pricing::DWalletPricing2PcMpcSecp256K1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_create_dwallet_pricing_2pc_mpc_secp256k1">create_dwallet_pricing_2pc_mpc_secp256k1</a>(
    <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_dkg">computation_ika_price_per_dkg</a>: u64,
    <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_dkg">computation_sui_price_per_dkg</a>: u64,
    <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_re_encrypt_user_share">computation_ika_price_per_re_encrypt_user_share</a>: u64,
    <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_re_encrypt_user_share">computation_sui_price_per_re_encrypt_user_share</a>: u64,
    <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_presign">computation_ika_price_per_ecdsa_presign</a>: u64,
    <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_presign">computation_sui_price_per_ecdsa_presign</a>: u64,
    <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_sign">computation_ika_price_per_ecdsa_sign</a>: u64,
    <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_sign">computation_sui_price_per_ecdsa_sign</a>: u64,
    <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_future_sign">computation_ika_price_per_ecdsa_future_sign</a>: u64,
    <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_future_sign">computation_sui_price_per_ecdsa_future_sign</a>: u64,
    <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_sign_with_partial_user_signature">computation_ika_price_per_ecdsa_sign_with_partial_user_signature</a>: u64,
    <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_sign_with_partial_user_signature">computation_sui_price_per_ecdsa_sign_with_partial_user_signature</a>: u64,
    ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a> {
    <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a> {
        id: object::new(ctx),
        <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_dkg">computation_ika_price_per_dkg</a>,
        <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_dkg">computation_sui_price_per_dkg</a>,
        <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_re_encrypt_user_share">computation_ika_price_per_re_encrypt_user_share</a>,
        <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_re_encrypt_user_share">computation_sui_price_per_re_encrypt_user_share</a>,
        <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_presign">computation_ika_price_per_ecdsa_presign</a>,
        <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_presign">computation_sui_price_per_ecdsa_presign</a>,
        <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_sign">computation_ika_price_per_ecdsa_sign</a>,
        <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_sign">computation_sui_price_per_ecdsa_sign</a>,
        <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_future_sign">computation_ika_price_per_ecdsa_future_sign</a>,
        <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_future_sign">computation_sui_price_per_ecdsa_future_sign</a>,
        <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_sign_with_partial_user_signature">computation_ika_price_per_ecdsa_sign_with_partial_user_signature</a>,
        <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_sign_with_partial_user_signature">computation_sui_price_per_ecdsa_sign_with_partial_user_signature</a>,
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_dkg"></a>

## Function `computation_ika_price_per_dkg`

Returns the IKA price per dkg.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_dkg">computation_ika_price_per_dkg</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">dwallet_pricing::DWalletPricing2PcMpcSecp256K1</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_dkg">computation_ika_price_per_dkg</a>(self: &<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a>): u64 {
    self.<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_dkg">computation_ika_price_per_dkg</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_dkg"></a>

## Function `computation_sui_price_per_dkg`

Returns the SUI price per dkg.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_dkg">computation_sui_price_per_dkg</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">dwallet_pricing::DWalletPricing2PcMpcSecp256K1</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_dkg">computation_sui_price_per_dkg</a>(self: &<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a>): u64 {
    self.<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_dkg">computation_sui_price_per_dkg</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_re_encrypt_user_share"></a>

## Function `computation_ika_price_per_re_encrypt_user_share`

Returns the IKA price per re-encrypt user share.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_re_encrypt_user_share">computation_ika_price_per_re_encrypt_user_share</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">dwallet_pricing::DWalletPricing2PcMpcSecp256K1</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_re_encrypt_user_share">computation_ika_price_per_re_encrypt_user_share</a>(self: &<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a>): u64 {
    self.<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_re_encrypt_user_share">computation_ika_price_per_re_encrypt_user_share</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_re_encrypt_user_share"></a>

## Function `computation_sui_price_per_re_encrypt_user_share`

Returns the SUI price per re-encrypt user share.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_re_encrypt_user_share">computation_sui_price_per_re_encrypt_user_share</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">dwallet_pricing::DWalletPricing2PcMpcSecp256K1</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_re_encrypt_user_share">computation_sui_price_per_re_encrypt_user_share</a>(self: &<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a>): u64 {
    self.<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_re_encrypt_user_share">computation_sui_price_per_re_encrypt_user_share</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_presign"></a>

## Function `computation_ika_price_per_ecdsa_presign`

Returns the IKA price per ecdsa presign.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_presign">computation_ika_price_per_ecdsa_presign</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">dwallet_pricing::DWalletPricing2PcMpcSecp256K1</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_presign">computation_ika_price_per_ecdsa_presign</a>(self: &<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a>): u64 {
    self.<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_presign">computation_ika_price_per_ecdsa_presign</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_presign"></a>

## Function `computation_sui_price_per_ecdsa_presign`

Returns the SUI price per ecdsa presign.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_presign">computation_sui_price_per_ecdsa_presign</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">dwallet_pricing::DWalletPricing2PcMpcSecp256K1</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_presign">computation_sui_price_per_ecdsa_presign</a>(self: &<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a>): u64 {
    self.<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_presign">computation_sui_price_per_ecdsa_presign</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_sign"></a>

## Function `computation_ika_price_per_ecdsa_sign`

Returns the IKA price per ecdsa sign.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_sign">computation_ika_price_per_ecdsa_sign</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">dwallet_pricing::DWalletPricing2PcMpcSecp256K1</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_sign">computation_ika_price_per_ecdsa_sign</a>(self: &<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a>): u64 {
    self.<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_sign">computation_ika_price_per_ecdsa_sign</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_sign"></a>

## Function `computation_sui_price_per_ecdsa_sign`

Returns the SUI price per ecdsa sign.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_sign">computation_sui_price_per_ecdsa_sign</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">dwallet_pricing::DWalletPricing2PcMpcSecp256K1</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_sign">computation_sui_price_per_ecdsa_sign</a>(self: &<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a>): u64 {
    self.<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_sign">computation_sui_price_per_ecdsa_sign</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_future_sign"></a>

## Function `computation_ika_price_per_ecdsa_future_sign`

Returns the IKA price per ecdsa future sign.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_future_sign">computation_ika_price_per_ecdsa_future_sign</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">dwallet_pricing::DWalletPricing2PcMpcSecp256K1</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_future_sign">computation_ika_price_per_ecdsa_future_sign</a>(self: &<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a>): u64 {
    self.<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_future_sign">computation_ika_price_per_ecdsa_future_sign</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_future_sign"></a>

## Function `computation_sui_price_per_ecdsa_future_sign`

Returns the SUI price per ecdsa future sign.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_future_sign">computation_sui_price_per_ecdsa_future_sign</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">dwallet_pricing::DWalletPricing2PcMpcSecp256K1</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_future_sign">computation_sui_price_per_ecdsa_future_sign</a>(self: &<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a>): u64 {
    self.<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_future_sign">computation_sui_price_per_ecdsa_future_sign</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_sign_with_partial_user_signature"></a>

## Function `computation_ika_price_per_ecdsa_sign_with_partial_user_signature`

Returns the IKA price per ecdsa sign with partial user signature.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_sign_with_partial_user_signature">computation_ika_price_per_ecdsa_sign_with_partial_user_signature</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">dwallet_pricing::DWalletPricing2PcMpcSecp256K1</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_sign_with_partial_user_signature">computation_ika_price_per_ecdsa_sign_with_partial_user_signature</a>(self: &<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a>): u64 {
    self.<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika_price_per_ecdsa_sign_with_partial_user_signature">computation_ika_price_per_ecdsa_sign_with_partial_user_signature</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_sign_with_partial_user_signature"></a>

## Function `computation_sui_price_per_ecdsa_sign_with_partial_user_signature`

Returns the SUI price per ecdsa sign with partial user signature.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_sign_with_partial_user_signature">computation_sui_price_per_ecdsa_sign_with_partial_user_signature</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">dwallet_pricing::DWalletPricing2PcMpcSecp256K1</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_sign_with_partial_user_signature">computation_sui_price_per_ecdsa_sign_with_partial_user_signature</a>(self: &<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a>): u64 {
    self.<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_sui_price_per_ecdsa_sign_with_partial_user_signature">computation_sui_price_per_ecdsa_sign_with_partial_user_signature</a>
}
</code></pre>



</details>
