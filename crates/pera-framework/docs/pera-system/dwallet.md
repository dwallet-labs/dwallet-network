---
title: Module `0x3::dwallet`
---

This module defines the core data structures and functions for
working with dWallets in the Ika system.


<a name="@Overview_0"></a>

### Overview


- A **dWallet** (<code><a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a></code>) represents a wallet created after the Distributed Key Generation (DKG) process.
It encapsulates the session ID, capability ID, and the outputs from the DKG rounds.
- A **dWallet capability** (<code><a href="dwallet.md#0x3_dwallet_DWalletCap">DWalletCap</a></code>) grants ownership and control over a corresponding <code><a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a></code>.


<a name="@Key_Concepts_1"></a>

### Key Concepts


- **DWallet**: A generic wallet structure with a phantom type <code>T</code>.
- **DWalletCap**: A capability object that grants control over a specific dWallet.
- **Session ID**: A unique identifier for the DKG session.


    -  [Overview](#@Overview_0)
    -  [Key Concepts](#@Key_Concepts_1)
-  [Resource `DWallet`](#0x3_dwallet_DWallet)
-  [Resource `EncryptionKey`](#0x3_dwallet_EncryptionKey)
-  [Struct `CreatedEncryptionKeyEvent`](#0x3_dwallet_CreatedEncryptionKeyEvent)
-  [Struct `StartEncryptionKeyVerificationEvent`](#0x3_dwallet_StartEncryptionKeyVerificationEvent)
-  [Resource `DWalletCap`](#0x3_dwallet_DWalletCap)
-  [Resource `ActiveEncryptionKeys`](#0x3_dwallet_ActiveEncryptionKeys)
-  [Constants](#@Constants_6)
-  [Function `get_encryption_key`](#0x3_dwallet_get_encryption_key)
-  [Function `create_dwallet`](#0x3_dwallet_create_dwallet)
-  [Function `create_active_encryption_keys`](#0x3_dwallet_create_active_encryption_keys)
-  [Function `get_active_encryption_key`](#0x3_dwallet_get_active_encryption_key)
-  [Function `upsert_active_encryption_key`](#0x3_dwallet_upsert_active_encryption_key)
-  [Function `register_encryption_key`](#0x3_dwallet_register_encryption_key)
-  [Function `create_encryption_key`](#0x3_dwallet_create_encryption_key)
-  [Function `create_dwallet_cap`](#0x3_dwallet_create_dwallet_cap)
-  [Function `get_dwallet_cap_id`](#0x3_dwallet_get_dwallet_cap_id)
-  [Function `get_dwallet_decentralized_output`](#0x3_dwallet_get_dwallet_decentralized_output)
-  [Function `get_dwallet_centralized_output`](#0x3_dwallet_get_dwallet_centralized_output)
-  [Function `get_dwallet_mpc_network_key_version`](#0x3_dwallet_get_dwallet_mpc_network_key_version)
-  [Function `is_valid_encryption_key_scheme`](#0x3_dwallet_is_valid_encryption_key_scheme)


<pre><code><b>use</b> <a href="../pera-framework/ed25519.md#0x2_ed25519">0x2::ed25519</a>;
<b>use</b> <a href="../pera-framework/event.md#0x2_event">0x2::event</a>;
<b>use</b> <a href="../pera-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../pera-framework/table.md#0x2_table">0x2::table</a>;
<b>use</b> <a href="../pera-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../pera-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="pera_system.md#0x3_pera_system">0x3::pera_system</a>;
</code></pre>



<a name="0x3_dwallet_DWallet"></a>

## Resource `DWallet`

<code><a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a></code> represents a wallet that is created after the DKG process.


<a name="@Fields_2"></a>

##### Fields

- <code>id</code>: Unique identifier for the dWallet.
- <code>session_id</code>: The session ID that generated this dWallet.
- <code>dwallet_cap_id</code>: The ID of the capability associated with this dWallet.
- <code>decentralized_output</code>: Decentralized public output of the second DKG round.
- <code>centralized_output</code>: Centralized public output.
- <code>dwallet_mpc_network_key_version</code>: Version of the MPC network key.


<pre><code><b>struct</b> <a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a>&lt;T&gt; <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../pera-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>decentralized_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>centralized_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>dwallet_mpc_network_key_version: u8</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_EncryptionKey"></a>

## Resource `EncryptionKey`

todo(zeev): check why we transfer both public key and address.
An Encryption key that is used to encrypt a dWallet centralized (user) secret key share.
Encryption keys facilitate secure data transfer between accounts on the
dWallet Network by ensuring that sensitive information remains confidential during transmission.
Each address on the dWallet Network is associated with a unique encryption key.
When an external party intends to send encrypted data to a particular account, they use the recipient’s
encryption key to encrypt the data.
The recipient is then the sole entity capable of decrypting and accessing this information, ensuring secure,
end-to-end encryption.

<a name="@Fields_3"></a>

##### Fields

- <code>id</code>: Unique identifier for the encryption key.
- <code>scheme</code>: Scheme identifier for the encryption key (e.g., Class Groups).
- <code>encryption_key</code>: Serialized encryption key.
- <code>key_owner_address</code>: Address of the encryption key owner.
- <code>encryption_key_signature</code>: Signature for the encryption key, signed by the owner.
- <code>key_owner_pubkey</code>: Public key of the encryption key owner.


<pre><code><b>struct</b> <a href="dwallet.md#0x3_dwallet_EncryptionKey">EncryptionKey</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../pera-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>scheme: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>encryption_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>key_owner_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>encryption_key_signature: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>key_owner_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_CreatedEncryptionKeyEvent"></a>

## Struct `CreatedEncryptionKeyEvent`

Event emitted when an encryption key is created.


<pre><code><b>struct</b> <a href="dwallet.md#0x3_dwallet_CreatedEncryptionKeyEvent">CreatedEncryptionKeyEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>scheme: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>encryption_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>key_owner_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>encryption_key_signature: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>key_owner_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>encryption_key_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_StartEncryptionKeyVerificationEvent"></a>

## Struct `StartEncryptionKeyVerificationEvent`

An event emitted to start an encryption key verification process.
Ika does not support native functions, so an event is emitted and
caught by the blockchain, which then starts the verification process,
similar to the MPC processes.


<pre><code><b>struct</b> <a href="dwallet.md#0x3_dwallet_StartEncryptionKeyVerificationEvent">StartEncryptionKeyVerificationEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>scheme: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>encryption_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>key_owner_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>encryption_key_signature: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>sender_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>initiator: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_DWalletCap"></a>

## Resource `DWalletCap`

Represents a capability granting control over a specific dWallet.


<a name="@Fields_4"></a>

##### Fields

- <code>id</code>: Unique identifier for the dWallet capability.


<pre><code><b>struct</b> <a href="dwallet.md#0x3_dwallet_DWalletCap">DWalletCap</a> <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../pera-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_ActiveEncryptionKeys"></a>

## Resource `ActiveEncryptionKeys`

Shared object that holds the active encryption keys per user.


<a name="@Fields_5"></a>

##### Fields

- <code>id</code>: Unique identifier for the object.
- <code>encryption_keys</code>: Table mapping user addresses to encryption key object IDs.


<pre><code><b>struct</b> <a href="dwallet.md#0x3_dwallet_ActiveEncryptionKeys">ActiveEncryptionKeys</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../pera-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>encryption_keys: <a href="../pera-framework/table.md#0x2_table_Table">table::Table</a>&lt;<b>address</b>, <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_6"></a>

## Constants


<a name="0x3_dwallet_ENotSystemAddress"></a>



<pre><code><b>const</b> <a href="dwallet.md#0x3_dwallet_ENotSystemAddress">ENotSystemAddress</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 3;
</code></pre>



<a name="0x3_dwallet_CLASS_GROUPS"></a>



<pre><code><b>const</b> <a href="dwallet.md#0x3_dwallet_CLASS_GROUPS">CLASS_GROUPS</a>: u8 = 0;
</code></pre>



<a name="0x3_dwallet_EInvalidEncryptionKeyOwner"></a>



<pre><code><b>const</b> <a href="dwallet.md#0x3_dwallet_EInvalidEncryptionKeyOwner">EInvalidEncryptionKeyOwner</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 2;
</code></pre>



<a name="0x3_dwallet_EInvalidEncryptionKeyScheme"></a>



<pre><code><b>const</b> <a href="dwallet.md#0x3_dwallet_EInvalidEncryptionKeyScheme">EInvalidEncryptionKeyScheme</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
</code></pre>



<a name="0x3_dwallet_EInvalidEncryptionKeySignature"></a>



<pre><code><b>const</b> <a href="dwallet.md#0x3_dwallet_EInvalidEncryptionKeySignature">EInvalidEncryptionKeySignature</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1;
</code></pre>



<a name="0x3_dwallet_SYSTEM_ADDRESS"></a>



<pre><code><b>const</b> <a href="dwallet.md#0x3_dwallet_SYSTEM_ADDRESS">SYSTEM_ADDRESS</a>: <b>address</b> = 0;
</code></pre>



<a name="0x3_dwallet_get_encryption_key"></a>

## Function `get_encryption_key`

Retrieves the encryption key from an <code><a href="dwallet.md#0x3_dwallet_EncryptionKey">EncryptionKey</a></code> object.


<a name="@Parameters_7"></a>

##### Parameters

- <code>key</code>: A read reference to the <code><a href="dwallet.md#0x3_dwallet_EncryptionKey">EncryptionKey</a></code> object.


<a name="@Returns_8"></a>

##### Returns

A <code><a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code> containing the encryption key.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet.md#0x3_dwallet_get_encryption_key">get_encryption_key</a>(key: &<a href="dwallet.md#0x3_dwallet_EncryptionKey">dwallet::EncryptionKey</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet.md#0x3_dwallet_get_encryption_key">get_encryption_key</a>(key: &<a href="dwallet.md#0x3_dwallet_EncryptionKey">EncryptionKey</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    key.encryption_key
}
</code></pre>



</details>

<a name="0x3_dwallet_create_dwallet"></a>

## Function `create_dwallet`

Creates a new [<code><a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a></code>] object of type <code>T</code>.


<a name="@Parameters_9"></a>

##### Parameters

- <code>session_id</code>: Session ID that generated this dWallet.
- <code>dwallet_cap_id</code>: Capability ID associated with this dWallet.
- <code>decentralized_output</code>: Decentralized output of the second DKG round.
- <code>dwallet_mpc_network_key_version</code>: Version of the MPC network key.
- <code>dkg_centralized_public_output</code>: Centralized public output of the DKG round.
- <code>ctx</code>: Mutable transaction context.


<a name="@Returns_10"></a>

##### Returns

A new [<code><a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a></code>] object of type <code>T</code>.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet.md#0x3_dwallet_create_dwallet">create_dwallet</a>&lt;T: drop&gt;(session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, decentralized_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, dwallet_mpc_network_key_version: u8, dkg_centralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet.md#0x3_dwallet_create_dwallet">create_dwallet</a>&lt;T: drop&gt;(
    session_id: ID,
    dwallet_cap_id: ID,
    decentralized_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    dwallet_mpc_network_key_version: u8,
    dkg_centralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext
): <a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a>&lt;T&gt; {
    <a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a>&lt;T&gt; {
        id: <a href="../pera-framework/object.md#0x2_object_new">object::new</a>(ctx),
        session_id,
        dwallet_cap_id,
        decentralized_output,
        dwallet_mpc_network_key_version,
        centralized_output: dkg_centralized_public_output,
    }
}
</code></pre>



</details>

<a name="0x3_dwallet_create_active_encryption_keys"></a>

## Function `create_active_encryption_keys`

Create a shared object that holds the active encryption keys per user.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet.md#0x3_dwallet_create_active_encryption_keys">create_active_encryption_keys</a>(ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet.md#0x3_dwallet_create_active_encryption_keys">create_active_encryption_keys</a>(ctx: &<b>mut</b> TxContext) {
    <a href="../pera-framework/transfer.md#0x2_transfer_share_object">transfer::share_object</a>(<a href="dwallet.md#0x3_dwallet_ActiveEncryptionKeys">ActiveEncryptionKeys</a> {
        id: <a href="../pera-framework/object.md#0x2_object_new">object::new</a>(ctx),
        encryption_keys: <a href="../pera-framework/table.md#0x2_table_new">table::new</a>(ctx),
    });
}
</code></pre>



</details>

<a name="0x3_dwallet_get_active_encryption_key"></a>

## Function `get_active_encryption_key`

Get the active encryption key ID by user adders.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet.md#0x3_dwallet_get_active_encryption_key">get_active_encryption_key</a>(active_encryption_keys: &<a href="dwallet.md#0x3_dwallet_ActiveEncryptionKeys">dwallet::ActiveEncryptionKeys</a>, key_owner: <b>address</b>): &<a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet.md#0x3_dwallet_get_active_encryption_key">get_active_encryption_key</a>(
    active_encryption_keys: &<a href="dwallet.md#0x3_dwallet_ActiveEncryptionKeys">ActiveEncryptionKeys</a>,
    key_owner: <b>address</b>,
): &ID {
    <a href="../pera-framework/table.md#0x2_table_borrow">table::borrow</a>(&active_encryption_keys.encryption_keys, key_owner)
}
</code></pre>



</details>

<a name="0x3_dwallet_upsert_active_encryption_key"></a>

## Function `upsert_active_encryption_key`

Updates or inserts an encryption key as the active key for a user.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet.md#0x3_dwallet_upsert_active_encryption_key">upsert_active_encryption_key</a>(active_encryption_keys: &<b>mut</b> <a href="dwallet.md#0x3_dwallet_ActiveEncryptionKeys">dwallet::ActiveEncryptionKeys</a>, encryption_key: &<a href="dwallet.md#0x3_dwallet_EncryptionKey">dwallet::EncryptionKey</a>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet.md#0x3_dwallet_upsert_active_encryption_key">upsert_active_encryption_key</a>(
    active_encryption_keys: &<b>mut</b> <a href="dwallet.md#0x3_dwallet_ActiveEncryptionKeys">ActiveEncryptionKeys</a>,
    encryption_key: &<a href="dwallet.md#0x3_dwallet_EncryptionKey">EncryptionKey</a>,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(encryption_key.key_owner_address == <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx), <a href="dwallet.md#0x3_dwallet_EInvalidEncryptionKeyOwner">EInvalidEncryptionKeyOwner</a>);
    <b>if</b> (<a href="../pera-framework/table.md#0x2_table_contains">table::contains</a>(&active_encryption_keys.encryption_keys, encryption_key.key_owner_address)) {
        <a href="../pera-framework/table.md#0x2_table_remove">table::remove</a>(&<b>mut</b> active_encryption_keys.encryption_keys, encryption_key.key_owner_address);
    };
    <a href="../pera-framework/table.md#0x2_table_add">table::add</a>(
        &<b>mut</b> active_encryption_keys.encryption_keys,
        encryption_key.key_owner_address,
        <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(encryption_key)
    );
}
</code></pre>



</details>

<a name="0x3_dwallet_register_encryption_key"></a>

## Function `register_encryption_key`

Register an encryption key, to later use for encrypting a user share.
The key is saved as an immutable object.
The event emitted by this function is caught by the chain.
The chain then calls "create_encryption_key" after verifications, in order to save it.
We need to run the flow this way as this verification can only be done in Rust,
and we can't use Native functions.

<a name="@Parameters_11"></a>

##### Parameters

- <code>encryption_key</code>: Serialized encryption key.
- <code>signed_encryption_key</code>: Signed encryption key.
- <code>sender_pubkey</code>: Public key of the sender.
- <code>encryption_key_scheme</code>: Scheme of the encryption key.
- <code>ctx</code>: Mutable transaction context.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet.md#0x3_dwallet_register_encryption_key">register_encryption_key</a>(encryption_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, signed_encryption_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, sender_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, encryption_key_scheme: u8, _pera_system_state: &<a href="pera_system.md#0x3_pera_system_PeraSystemState">pera_system::PeraSystemState</a>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet.md#0x3_dwallet_register_encryption_key">register_encryption_key</a>(
    encryption_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    signed_encryption_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    sender_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    encryption_key_scheme: u8,
    _pera_system_state: &PeraSystemState,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(<a href="dwallet.md#0x3_dwallet_is_valid_encryption_key_scheme">is_valid_encryption_key_scheme</a>(encryption_key_scheme), <a href="dwallet.md#0x3_dwallet_EInvalidEncryptionKeyScheme">EInvalidEncryptionKeyScheme</a>);
    <b>assert</b>!(
        ed25519_verify(&signed_encryption_key, &sender_pubkey, &encryption_key),
        <a href="dwallet.md#0x3_dwallet_EInvalidEncryptionKeySignature">EInvalidEncryptionKeySignature</a>
    );
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(
        <a href="dwallet.md#0x3_dwallet_StartEncryptionKeyVerificationEvent">StartEncryptionKeyVerificationEvent</a> {
            scheme: encryption_key_scheme,
            encryption_key,
            key_owner_address: <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx),
            encryption_key_signature: signed_encryption_key,
            // todo(zeev): rename.
            sender_pubkey,
            initiator: <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx),
            session_id: <a href="../pera-framework/object.md#0x2_object_id_from_address">object::id_from_address</a>(<a href="../pera-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx)),
        }
    );
}
</code></pre>



</details>

<a name="0x3_dwallet_create_encryption_key"></a>

## Function `create_encryption_key`

Creates an encryption key object.
Being called by the blockchain after it verifies
the <code>sender_pubkey</code> matches the initiator address.
// todo(zeev): validate this claim.
We need to run the flow this way as this verification can only be done in rust.


<a name="@Parameters_12"></a>

##### Parameters

- <code>key</code>: Serialized encryption key.
- <code>signature</code>: Encryption key signature.
- <code>sender_pubkey</code>: Sender's Ika public key.
- <code>scheme</code>: Encryption key scheme.
- <code>initiator</code>: Initiator's address.
- <code>session_id</code>: ID of the session.
- <code>ctx</code>: Mutable transaction context.


<pre><code><b>fun</b> <a href="dwallet.md#0x3_dwallet_create_encryption_key">create_encryption_key</a>(key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, signature: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, sender_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, scheme: u8, initiator: <b>address</b>, session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="dwallet.md#0x3_dwallet_create_encryption_key">create_encryption_key</a>(
    key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    signature: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    sender_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    scheme: u8,
    initiator: <b>address</b>,
    session_id: ID,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(<a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == <a href="dwallet.md#0x3_dwallet_SYSTEM_ADDRESS">SYSTEM_ADDRESS</a>, <a href="dwallet.md#0x3_dwallet_ENotSystemAddress">ENotSystemAddress</a>);
    <b>let</b> encryption_key = <a href="dwallet.md#0x3_dwallet_EncryptionKey">EncryptionKey</a> {
        id: <a href="../pera-framework/object.md#0x2_object_new">object::new</a>(ctx),
        scheme,
        encryption_key: key,
        key_owner_address: initiator,
        encryption_key_signature: signature,
        key_owner_pubkey: sender_pubkey,
    };
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet.md#0x3_dwallet_CreatedEncryptionKeyEvent">CreatedEncryptionKeyEvent</a> {
        scheme,
        encryption_key: key,
        key_owner_address: initiator,
        encryption_key_signature: signature,
        key_owner_pubkey: sender_pubkey,
        encryption_key_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(&encryption_key),
        session_id,
    });
    <a href="../pera-framework/transfer.md#0x2_transfer_freeze_object">transfer::freeze_object</a>(encryption_key);
}
</code></pre>



</details>

<a name="0x3_dwallet_create_dwallet_cap"></a>

## Function `create_dwallet_cap`

Create a new [<code><a href="dwallet.md#0x3_dwallet_DWalletCap">DWalletCap</a></code>] object.

The holder of the <code><a href="dwallet.md#0x3_dwallet_DWalletCap">DWalletCap</a></code> has control and ownership over
the associated <code><a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a></code>.


<a name="@Parameters_13"></a>

##### Parameters

- <code>ctx</code>: A mutable transaction context used to create the <code><a href="dwallet.md#0x3_dwallet_DWalletCap">DWalletCap</a></code> object.


<a name="@Returns_14"></a>

##### Returns

The newly created <code><a href="dwallet.md#0x3_dwallet_DWalletCap">DWalletCap</a></code> object.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet.md#0x3_dwallet_create_dwallet_cap">create_dwallet_cap</a>(ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="dwallet.md#0x3_dwallet_DWalletCap">dwallet::DWalletCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet.md#0x3_dwallet_create_dwallet_cap">create_dwallet_cap</a>(ctx: &<b>mut</b> TxContext): <a href="dwallet.md#0x3_dwallet_DWalletCap">DWalletCap</a> {
    <a href="dwallet.md#0x3_dwallet_DWalletCap">DWalletCap</a> {
        id: <a href="../pera-framework/object.md#0x2_object_new">object::new</a>(ctx),
    }
}
</code></pre>



</details>

<a name="0x3_dwallet_get_dwallet_cap_id"></a>

## Function `get_dwallet_cap_id`

Retrieve the ID of the <code><a href="dwallet.md#0x3_dwallet_DWalletCap">DWalletCap</a></code> associated with a given dWallet.


<a name="@Parameters_15"></a>

##### Parameters

- <code><a href="dwallet.md#0x3_dwallet">dwallet</a></code>: A reference to the [<code><a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a></code>] object whose capability ID is to be retrieved.


<a name="@Returns_16"></a>

##### Returns

The ID of the <code><a href="dwallet.md#0x3_dwallet_DWalletCap">DWalletCap</a></code> associated with the provided dWallet.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet.md#0x3_dwallet_get_dwallet_cap_id">get_dwallet_cap_id</a>&lt;T: drop&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;T&gt;): <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet.md#0x3_dwallet_get_dwallet_cap_id">get_dwallet_cap_id</a>&lt;T: drop&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a>&lt;T&gt;): ID {
    <a href="dwallet.md#0x3_dwallet">dwallet</a>.dwallet_cap_id
}
</code></pre>



</details>

<a name="0x3_dwallet_get_dwallet_decentralized_output"></a>

## Function `get_dwallet_decentralized_output`

Retrieve the output of the second DKG round for a given dWallet.


<a name="@Parameters_17"></a>

##### Parameters

- <code><a href="dwallet.md#0x3_dwallet">dwallet</a></code>: A reference to the [<code><a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a></code>] object whose DKG output is to be retrieved.


<a name="@Returns_18"></a>

##### Returns

A <code><a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code> representing the output of the second DKG round for the specified dWallet.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet.md#0x3_dwallet_get_dwallet_decentralized_output">get_dwallet_decentralized_output</a>&lt;T: drop&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;T&gt;): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet.md#0x3_dwallet_get_dwallet_decentralized_output">get_dwallet_decentralized_output</a>&lt;T: drop&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a>&lt;T&gt;): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <a href="dwallet.md#0x3_dwallet">dwallet</a>.decentralized_output
}
</code></pre>



</details>

<a name="0x3_dwallet_get_dwallet_centralized_output"></a>

## Function `get_dwallet_centralized_output`

Retrieve the centralized public DKG output for a given dWallet.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet.md#0x3_dwallet_get_dwallet_centralized_output">get_dwallet_centralized_output</a>&lt;T: drop&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;T&gt;): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet.md#0x3_dwallet_get_dwallet_centralized_output">get_dwallet_centralized_output</a>&lt;T: drop&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a>&lt;T&gt;): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <a href="dwallet.md#0x3_dwallet">dwallet</a>.centralized_output
}
</code></pre>



</details>

<a name="0x3_dwallet_get_dwallet_mpc_network_key_version"></a>

## Function `get_dwallet_mpc_network_key_version`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet.md#0x3_dwallet_get_dwallet_mpc_network_key_version">get_dwallet_mpc_network_key_version</a>&lt;T: drop&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;T&gt;): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet.md#0x3_dwallet_get_dwallet_mpc_network_key_version">get_dwallet_mpc_network_key_version</a>&lt;T: drop&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a>&lt;T&gt;): u8 {
    <a href="dwallet.md#0x3_dwallet">dwallet</a>.dwallet_mpc_network_key_version
}
</code></pre>



</details>

<a name="0x3_dwallet_is_valid_encryption_key_scheme"></a>

## Function `is_valid_encryption_key_scheme`

Validates encryption key schemes.


<pre><code><b>fun</b> <a href="dwallet.md#0x3_dwallet_is_valid_encryption_key_scheme">is_valid_encryption_key_scheme</a>(scheme: u8): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="dwallet.md#0x3_dwallet_is_valid_encryption_key_scheme">is_valid_encryption_key_scheme</a>(scheme: u8): bool {
    scheme == <a href="dwallet.md#0x3_dwallet_CLASS_GROUPS">CLASS_GROUPS</a>
}
</code></pre>



</details>
