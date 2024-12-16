---
title: Module `0x3::dwallet_network_key`
---



-  [Struct `EncryptionOfNetworkDecryptionKeyShares`](#0x3_dwallet_network_key_EncryptionOfNetworkDecryptionKeyShares)
-  [Constants](#@Constants_0)
-  [Function `is_key_type`](#0x3_dwallet_network_key_is_key_type)
-  [Function `new_encrypted_network_decryption_key_shares`](#0x3_dwallet_network_key_new_encrypted_network_decryption_key_shares)
-  [Function `update_new_shares`](#0x3_dwallet_network_key_update_new_shares)


<pre><code></code></pre>



<a name="0x3_dwallet_network_key_EncryptionOfNetworkDecryptionKeyShares"></a>

## Struct `EncryptionOfNetworkDecryptionKeyShares`

Struct to store the network encryption of decryption key shares


<pre><code><b>struct</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_EncryptionOfNetworkDecryptionKeyShares">EncryptionOfNetworkDecryptionKeyShares</a> <b>has</b> <b>copy</b>, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>current_epoch_shares: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>previous_epoch_shares: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x3_dwallet_network_key_Ristretto"></a>



<pre><code><b>const</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_Ristretto">Ristretto</a>: u8 = 1;
</code></pre>



<a name="0x3_dwallet_network_key_Secp256k1"></a>



<pre><code><b>const</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_Secp256k1">Secp256k1</a>: u8 = 0;
</code></pre>



<a name="0x3_dwallet_network_key_is_key_type"></a>

## Function `is_key_type`



<pre><code><b>public</b> <b>fun</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_is_key_type">is_key_type</a>(val: u8): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_is_key_type">is_key_type</a>(val: u8): bool {
    <b>return</b> match (val) {
        <a href="dwallet_network_key.md#0x3_dwallet_network_key_Secp256k1">Secp256k1</a> | <a href="dwallet_network_key.md#0x3_dwallet_network_key_Ristretto">Ristretto</a> =&gt; <b>true</b>,
        _ =&gt; <b>false</b>,
    }
}
</code></pre>



</details>

<a name="0x3_dwallet_network_key_new_encrypted_network_decryption_key_shares"></a>

## Function `new_encrypted_network_decryption_key_shares`



<pre><code><b>public</b> <b>fun</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_new_encrypted_network_decryption_key_shares">new_encrypted_network_decryption_key_shares</a>(epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, current_epoch_shares: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, previous_epoch_shares: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;): <a href="dwallet_network_key.md#0x3_dwallet_network_key_EncryptionOfNetworkDecryptionKeyShares">dwallet_network_key::EncryptionOfNetworkDecryptionKeyShares</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_new_encrypted_network_decryption_key_shares">new_encrypted_network_decryption_key_shares</a>(epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, current_epoch_shares: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, previous_epoch_shares: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;): <a href="dwallet_network_key.md#0x3_dwallet_network_key_EncryptionOfNetworkDecryptionKeyShares">EncryptionOfNetworkDecryptionKeyShares</a> {
    <a href="dwallet_network_key.md#0x3_dwallet_network_key_EncryptionOfNetworkDecryptionKeyShares">EncryptionOfNetworkDecryptionKeyShares</a> {
        epoch,
        current_epoch_shares,
        previous_epoch_shares,
    }
}
</code></pre>



</details>

<a name="0x3_dwallet_network_key_update_new_shares"></a>

## Function `update_new_shares`



<pre><code><b>public</b> <b>fun</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_update_new_shares">update_new_shares</a>(self: &<b>mut</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_EncryptionOfNetworkDecryptionKeyShares">dwallet_network_key::EncryptionOfNetworkDecryptionKeyShares</a>, new_shares: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_update_new_shares">update_new_shares</a>(self: &<b>mut</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_EncryptionOfNetworkDecryptionKeyShares">EncryptionOfNetworkDecryptionKeyShares</a>, new_shares: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>) {
    self.previous_epoch_shares = self.current_epoch_shares;
    self.current_epoch_shares = new_shares;
    self.epoch = epoch;
}
</code></pre>



</details>
