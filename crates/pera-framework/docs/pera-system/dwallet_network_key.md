---
title: Module `0x3::dwallet_network_key`
---

This module manages the storage of the network dWallet MPC keys and associated data.


-  [Struct `StartNetworkDKGEvent`](#0x3_dwallet_network_key_StartNetworkDKGEvent)
-  [Struct `NetworkDecryptionKeyShares`](#0x3_dwallet_network_key_NetworkDecryptionKeyShares)
-  [Constants](#@Constants_0)
-  [Function `is_valid_key_scheme`](#0x3_dwallet_network_key_is_valid_key_scheme)
-  [Function `start_network_dkg`](#0x3_dwallet_network_key_start_network_dkg)
-  [Function `new_encrypted_network_decryption_key_shares`](#0x3_dwallet_network_key_new_encrypted_network_decryption_key_shares)
-  [Function `update_new_shares`](#0x3_dwallet_network_key_update_new_shares)


<pre><code><b>use</b> <a href="../pera-framework/event.md#0x2_event">0x2::event</a>;
<b>use</b> <a href="../pera-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../pera-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="validator_set.md#0x3_validator_set">0x3::validator_set</a>;
</code></pre>



<a name="0x3_dwallet_network_key_StartNetworkDKGEvent"></a>

## Struct `StartNetworkDKGEvent`

Event to start the network DKG.


<pre><code><b>struct</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_StartNetworkDKGEvent">StartNetworkDKGEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>key_scheme: u8</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_network_key_NetworkDecryptionKeyShares"></a>

## Struct `NetworkDecryptionKeyShares`

Struct to store the network encryption of decryption key shares


<pre><code><b>struct</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_NetworkDecryptionKeyShares">NetworkDecryptionKeyShares</a> <b>has</b> <b>copy</b>, store
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
<code>current_epoch_shares: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>previous_epoch_shares: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>protocol_public_parameters: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>decryption_public_parameters: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>encryption_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>reconstructed_commitments_to_sharing: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
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

Represents the key schemes supported by the system.


<pre><code><b>const</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_Secp256k1">Secp256k1</a>: u8 = 0;
</code></pre>



<a name="0x3_dwallet_network_key_is_valid_key_scheme"></a>

## Function `is_valid_key_scheme`

Checks if the key scheme is supported by the system


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_is_valid_key_scheme">is_valid_key_scheme</a>(val: u8): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_is_valid_key_scheme">is_valid_key_scheme</a>(val: u8): bool {
    <b>return</b> match (val) {
        <a href="dwallet_network_key.md#0x3_dwallet_network_key_Secp256k1">Secp256k1</a> | <a href="dwallet_network_key.md#0x3_dwallet_network_key_Ristretto">Ristretto</a> =&gt; <b>true</b>,
        _ =&gt; <b>false</b>,
    }
}
</code></pre>



</details>

<a name="0x3_dwallet_network_key_start_network_dkg"></a>

## Function `start_network_dkg`

Function to start a new network DKG.
It emits a [<code><a href="dwallet_network_key.md#0x3_dwallet_network_key_StartNetworkDKGEvent">StartNetworkDKGEvent</a></code>] and emits the [<code>ValidatorDataForDWalletSecretShare</code>] for each validator,
with its public key and proof, that are needed for the DKG process.

Each validator's data is being emitted separately because the proof size is
almost 250KB, which is the maximum event size in Sui.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_start_network_dkg">start_network_dkg</a>(key_scheme: u8, validators_data: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="validator_set.md#0x3_validator_set_ValidatorDataForDWalletSecretShare">validator_set::ValidatorDataForDWalletSecretShare</a>&gt;, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_start_network_dkg">start_network_dkg</a>(
    key_scheme: u8,
    validators_data: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;ValidatorDataForDWalletSecretShare&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> session_id = <a href="../pera-framework/object.md#0x2_object_id_from_address">object::id_from_address</a>(<a href="../pera-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx));

    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet_network_key.md#0x3_dwallet_network_key_StartNetworkDKGEvent">StartNetworkDKGEvent</a> {
        session_id,
        key_scheme,
    });
    <b>let</b> validators_len = validators_data.length();
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; validators_len) {
        <b>let</b> validator_data = validators_data[i];
        emit_validator_data_for_secret_share(validator_data);
        i = i + 1;
    }
}
</code></pre>



</details>

<a name="0x3_dwallet_network_key_new_encrypted_network_decryption_key_shares"></a>

## Function `new_encrypted_network_decryption_key_shares`

Function to create a new NetworkDecryptionKeyShares.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_new_encrypted_network_decryption_key_shares">new_encrypted_network_decryption_key_shares</a>(epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, current_epoch_shares: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, previous_epoch_shares: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, protocol_public_parameters: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, decryption_public_parameters: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, encryption_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, reconstructed_commitments_to_sharing: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="dwallet_network_key.md#0x3_dwallet_network_key_NetworkDecryptionKeyShares">dwallet_network_key::NetworkDecryptionKeyShares</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_new_encrypted_network_decryption_key_shares">new_encrypted_network_decryption_key_shares</a>(
    epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    current_epoch_shares: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    previous_epoch_shares: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    protocol_public_parameters: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    decryption_public_parameters: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    encryption_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    reconstructed_commitments_to_sharing: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
): <a href="dwallet_network_key.md#0x3_dwallet_network_key_NetworkDecryptionKeyShares">NetworkDecryptionKeyShares</a> {
    <a href="dwallet_network_key.md#0x3_dwallet_network_key_NetworkDecryptionKeyShares">NetworkDecryptionKeyShares</a> {
        epoch,
        current_epoch_shares,
        previous_epoch_shares,
        protocol_public_parameters,
        decryption_public_parameters,
        encryption_key,
        reconstructed_commitments_to_sharing,
    }
}
</code></pre>



</details>

<a name="0x3_dwallet_network_key_update_new_shares"></a>

## Function `update_new_shares`

Function to update the shares of the network encryption of decryption key.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_update_new_shares">update_new_shares</a>(self: &<b>mut</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_NetworkDecryptionKeyShares">dwallet_network_key::NetworkDecryptionKeyShares</a>, new_shares: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_update_new_shares">update_new_shares</a>(
    self: &<b>mut</b> <a href="dwallet_network_key.md#0x3_dwallet_network_key_NetworkDecryptionKeyShares">NetworkDecryptionKeyShares</a>,
    new_shares: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
) {
    self.previous_epoch_shares = self.current_epoch_shares;
    self.current_epoch_shares = new_shares;
    self.epoch = epoch;
}
</code></pre>



</details>
