---
title: Module `0x3::dwallet_network_key`
---



-  [Struct `EncryptedNetwotkDecryptionKeyShares`](#0x3_dwallet_network_key_EncryptedNetwotkDecryptionKeyShares)
-  [Enum `KeyType`](#0x3_dwallet_network_key_KeyType)
-  [Function `new_encrypted_network_decryption_key_shares`](#0x3_dwallet_network_key_new_encrypted_network_decryption_key_shares)
-  [Function `update_new_shares`](#0x3_dwallet_network_key_update_new_shares)


<pre><code></code></pre>



<a name="0x3_dwallet_network_key_EncryptedNetwotkDecryptionKeyShares"></a>

## Struct `EncryptedNetwotkDecryptionKeyShares`



<pre><code><b>struct</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_EncryptedNetwotkDecryptionKeyShares">EncryptedNetwotkDecryptionKeyShares</a> <b>has</b> <b>copy</b>, store
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

<a name="0x3_dwallet_network_key_KeyType"></a>

## Enum `KeyType`



<pre><code><b>public</b> enum KeyType <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Variants</summary>


<dl>
<dt>
Variant <code>Secp256k1</code>
</dt>
<dd>

</dd>
<dt>
Variant <code>Ristretto</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_network_key_new_encrypted_network_decryption_key_shares"></a>

## Function `new_encrypted_network_decryption_key_shares`



<pre><code><b>public</b> <b>fun</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_new_encrypted_network_decryption_key_shares">new_encrypted_network_decryption_key_shares</a>(epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, current_epoch_shares: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, previous_epoch_shares: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;): <a href="dwallet_network_key.md#0x3_dwallet_network_key_EncryptedNetwotkDecryptionKeyShares">dwallet_network_key::EncryptedNetwotkDecryptionKeyShares</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_new_encrypted_network_decryption_key_shares">new_encrypted_network_decryption_key_shares</a>(epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, current_epoch_shares: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, previous_epoch_shares: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;): <a href="dwallet_network_key.md#0x3_dwallet_network_key_EncryptedNetwotkDecryptionKeyShares">EncryptedNetwotkDecryptionKeyShares</a> {
    <a href="dwallet_network_key.md#0x3_dwallet_network_key_EncryptedNetwotkDecryptionKeyShares">EncryptedNetwotkDecryptionKeyShares</a> {
        epoch,
        current_epoch_shares,
        previous_epoch_shares,
    }
}
</code></pre>



</details>

<a name="0x3_dwallet_network_key_update_new_shares"></a>

## Function `update_new_shares`



<pre><code><b>public</b> <b>fun</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_update_new_shares">update_new_shares</a>(self: &<b>mut</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_EncryptedNetwotkDecryptionKeyShares">dwallet_network_key::EncryptedNetwotkDecryptionKeyShares</a>, new_shares: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_update_new_shares">update_new_shares</a>(self: &<b>mut</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_EncryptedNetwotkDecryptionKeyShares">EncryptedNetwotkDecryptionKeyShares</a>, new_shares: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>) {
    self.previous_epoch_shares = self.current_epoch_shares;
    self.current_epoch_shares = new_shares;
    self.epoch = epoch;
}
</code></pre>



</details>
