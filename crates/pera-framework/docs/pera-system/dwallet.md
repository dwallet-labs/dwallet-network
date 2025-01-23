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
-  [Struct `MessageApproval`](#0x3_dwallet_MessageApproval)
-  [Constants](#@Constants_7)
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
-  [Function `create_message_approval`](#0x3_dwallet_create_message_approval)
-  [Function `approve_messages`](#0x3_dwallet_approve_messages)
-  [Function `remove_message_approval`](#0x3_dwallet_remove_message_approval)
-  [Function `pop_and_verify_message_approval`](#0x3_dwallet_pop_and_verify_message_approval)
-  [Function `hash_messages`](#0x3_dwallet_hash_messages)
-  [Function `hash_message`](#0x3_dwallet_hash_message)
-  [Function `is_supported_hash_scheme`](#0x3_dwallet_is_supported_hash_scheme)


<pre><code><b>use</b> <a href="../move-stdlib/hash.md#0x1_hash">0x1::hash</a>;
<b>use</b> <a href="../move-stdlib/vector.md#0x1_vector">0x1::vector</a>;
<b>use</b> <a href="../pera-framework/ed25519.md#0x2_ed25519">0x2::ed25519</a>;
<b>use</b> <a href="../pera-framework/event.md#0x2_event">0x2::event</a>;
<b>use</b> <a href="../pera-framework/hash.md#0x2_hash">0x2::hash</a>;
<b>use</b> <a href="../pera-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../pera-framework/table.md#0x2_table">0x2::table</a>;
<b>use</b> <a href="../pera-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../pera-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
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

<a name="0x3_dwallet_MessageApproval"></a>

## Struct `MessageApproval`

Represents a message that was approved as part of a dWallet process.

This struct binds the message to a specific <code><a href="dwallet.md#0x3_dwallet_DWalletCap">DWalletCap</a></code> for
traceability and accountability within the system.


<a name="@Fields_6"></a>

##### Fields

- **<code>dwallet_cap_id</code>**: The identifier of the dWallet capability
associated with this approval.
- **<code>hash_scheme</code>**: The message hash scheme.
- **<code>message</code>**: The message that has been approved.


<pre><code><b>struct</b> <a href="dwallet.md#0x3_dwallet_MessageApproval">MessageApproval</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>hash_scheme: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>message: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_7"></a>

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



<a name="0x3_dwallet_EInvalidHashScheme"></a>



<pre><code><b>const</b> <a href="dwallet.md#0x3_dwallet_EInvalidHashScheme">EInvalidHashScheme</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 6;
</code></pre>



<a name="0x3_dwallet_EMessageApprovalDWalletMismatch"></a>



<pre><code><b>const</b> <a href="dwallet.md#0x3_dwallet_EMessageApprovalDWalletMismatch">EMessageApprovalDWalletMismatch</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 4;
</code></pre>



<a name="0x3_dwallet_EMissingApprovalOrWrongApprovalOrder"></a>



<pre><code><b>const</b> <a href="dwallet.md#0x3_dwallet_EMissingApprovalOrWrongApprovalOrder">EMissingApprovalOrWrongApprovalOrder</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 5;
</code></pre>



<a name="0x3_dwallet_KECCAK256"></a>

Supported hash schemes for message signing.


<pre><code><b>const</b> <a href="dwallet.md#0x3_dwallet_KECCAK256">KECCAK256</a>: u8 = 0;
</code></pre>



<a name="0x3_dwallet_SHA256"></a>



<pre><code><b>const</b> <a href="dwallet.md#0x3_dwallet_SHA256">SHA256</a>: u8 = 1;
</code></pre>



<a name="0x3_dwallet_SYSTEM_ADDRESS"></a>



<pre><code><b>const</b> <a href="dwallet.md#0x3_dwallet_SYSTEM_ADDRESS">SYSTEM_ADDRESS</a>: <b>address</b> = 0;
</code></pre>



<a name="0x3_dwallet_get_encryption_key"></a>

## Function `get_encryption_key`

Retrieves the encryption key from an <code><a href="dwallet.md#0x3_dwallet_EncryptionKey">EncryptionKey</a></code> object.


<a name="@Parameters_8"></a>

##### Parameters

- <code>key</code>: A read reference to the <code><a href="dwallet.md#0x3_dwallet_EncryptionKey">EncryptionKey</a></code> object.


<a name="@Returns_9"></a>

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


<a name="@Parameters_10"></a>

##### Parameters

- <code>session_id</code>: Session ID that generated this dWallet.
- <code>dwallet_cap_id</code>: Capability ID associated with this dWallet.
- <code>decentralized_output</code>: Decentralized output of the second DKG round.
- <code>dwallet_mpc_network_key_version</code>: Version of the MPC network key.
- <code>dkg_centralized_public_output</code>: Centralized public output of the DKG round.
- <code>ctx</code>: Mutable transaction context.


<a name="@Returns_11"></a>

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

<a name="@Parameters_12"></a>

##### Parameters

- <code>encryption_key</code>: Serialized encryption key.
- <code>signed_encryption_key</code>: Signed encryption key.
- <code>sender_pubkey</code>: Public key of the sender.
- <code>encryption_key_scheme</code>: Scheme of the encryption key.
- <code>ctx</code>: Mutable transaction context.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet.md#0x3_dwallet_register_encryption_key">register_encryption_key</a>(encryption_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, signed_encryption_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, sender_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, encryption_key_scheme: u8, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet.md#0x3_dwallet_register_encryption_key">register_encryption_key</a>(
    encryption_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    signed_encryption_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    sender_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    encryption_key_scheme: u8,
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


<a name="@Parameters_13"></a>

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


<a name="@Parameters_14"></a>

##### Parameters

- <code>ctx</code>: A mutable transaction context used to create the <code><a href="dwallet.md#0x3_dwallet_DWalletCap">DWalletCap</a></code> object.


<a name="@Returns_15"></a>

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


<a name="@Parameters_16"></a>

##### Parameters

- <code><a href="dwallet.md#0x3_dwallet">dwallet</a></code>: A reference to the [<code><a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a></code>] object whose capability ID is to be retrieved.


<a name="@Returns_17"></a>

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


<a name="@Parameters_18"></a>

##### Parameters

- <code><a href="dwallet.md#0x3_dwallet">dwallet</a></code>: A reference to the [<code><a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a></code>] object whose DKG output is to be retrieved.


<a name="@Returns_19"></a>

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

<a name="0x3_dwallet_create_message_approval"></a>

## Function `create_message_approval`

Creates a <code><a href="dwallet.md#0x3_dwallet_MessageApproval">MessageApproval</a></code> object.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet.md#0x3_dwallet_create_message_approval">create_message_approval</a>(dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, hash_scheme: u8, message: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="dwallet.md#0x3_dwallet_MessageApproval">dwallet::MessageApproval</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet.md#0x3_dwallet_create_message_approval">create_message_approval</a>(
    dwallet_cap_id: ID,
    hash_scheme: u8,
    message: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
): <a href="dwallet.md#0x3_dwallet_MessageApproval">MessageApproval</a> {
    <b>assert</b>!(<a href="dwallet.md#0x3_dwallet_is_supported_hash_scheme">is_supported_hash_scheme</a>(hash_scheme), <a href="dwallet.md#0x3_dwallet_EInvalidHashScheme">EInvalidHashScheme</a>);
    <b>let</b> approval = <a href="dwallet.md#0x3_dwallet_MessageApproval">MessageApproval</a> {
        dwallet_cap_id,
        hash_scheme,
        message,
    };
    approval
}
</code></pre>



</details>

<a name="0x3_dwallet_approve_messages"></a>

## Function `approve_messages`

Approves a set of messages for a specific dWallet capability.

This function creates a list of <code><a href="dwallet.md#0x3_dwallet_MessageApproval">MessageApproval</a></code> objects for a given set of messages.
Each message is associated with the same <code>dWalletCap</code> and <code>hash_scheme</code>. The messages
must be approved in the same order as they were created to maintain their sequence.


<a name="@Parameters_20"></a>

##### Parameters

- <code>dwallet_cap</code>: A reference to the <code><a href="dwallet.md#0x3_dwallet_DWalletCap">DWalletCap</a></code> object representing the capability for which
the messages are being approved.
- <code>hash_scheme</code>: The hash scheme to be used for hashing the messages. For example:
- <code><a href="dwallet.md#0x3_dwallet_KECCAK256">KECCAK256</a></code>
- <code><a href="dwallet.md#0x3_dwallet_SHA256">SHA256</a></code>
- <code>messages</code>: A mutable vector containing the messages to be approved. The messages are removed
from this vector as they are processed and added to the approvals list.


<a name="@Returns_21"></a>

##### Returns

A vector of <code><a href="dwallet.md#0x3_dwallet_MessageApproval">MessageApproval</a></code> objects corresponding to the approved messages.


<a name="@Behavior_22"></a>

##### Behavior

- The function iterates over the provided <code>messages</code> vector, processes each message by creating
a <code><a href="dwallet.md#0x3_dwallet_MessageApproval">MessageApproval</a></code> object, and pushes it into the <code>message_approvals</code> vector.
- The messages are approved in reverse order and then reversed again to preserve their original order.


<a name="@Aborts_23"></a>

##### Aborts

- Aborts if the provided <code>hash_scheme</code> is not supported by the system (checked during <code>create_message_approval</code>).


<pre><code><b>public</b> <b>fun</b> <a href="dwallet.md#0x3_dwallet_approve_messages">approve_messages</a>(dwallet_cap: &<a href="dwallet.md#0x3_dwallet_DWalletCap">dwallet::DWalletCap</a>, hash_scheme: u8, messages: &<b>mut</b> <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet.md#0x3_dwallet_MessageApproval">dwallet::MessageApproval</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet.md#0x3_dwallet_approve_messages">approve_messages</a>(
    dwallet_cap: &<a href="dwallet.md#0x3_dwallet_DWalletCap">DWalletCap</a>,
    hash_scheme: u8,
    messages: &<b>mut</b> <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;
): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet.md#0x3_dwallet_MessageApproval">MessageApproval</a>&gt; {
    <b>let</b> dwallet_cap_id = <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(dwallet_cap);
    <b>let</b> <b>mut</b> message_approvals = <a href="../move-stdlib/vector.md#0x1_vector_empty">vector::empty</a>&lt;<a href="dwallet.md#0x3_dwallet_MessageApproval">MessageApproval</a>&gt;();

    // Approve all messages and maintain their order.
    <b>let</b> messages_length = <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(messages);
    <b>let</b> <b>mut</b> i: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
    <b>while</b> (i &lt; messages_length) {
        <b>let</b> message = <a href="../move-stdlib/vector.md#0x1_vector_pop_back">vector::pop_back</a>(messages);
        <a href="../move-stdlib/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> message_approvals, <a href="dwallet.md#0x3_dwallet_create_message_approval">create_message_approval</a> (
            dwallet_cap_id,
            hash_scheme,
            message,
        ));
        i = i + 1;
    };
    <a href="../move-stdlib/vector.md#0x1_vector_reverse">vector::reverse</a>(&<b>mut</b> message_approvals);
    message_approvals
}
</code></pre>



</details>

<a name="0x3_dwallet_remove_message_approval"></a>

## Function `remove_message_approval`

Remove a <code><a href="dwallet.md#0x3_dwallet_MessageApproval">MessageApproval</a></code> and return the <code>dwallet_cap_id</code>,
<code>hash_scheme</code> and the <code>message</code>.


<pre><code><b>fun</b> <a href="dwallet.md#0x3_dwallet_remove_message_approval">remove_message_approval</a>(message_approval: <a href="dwallet.md#0x3_dwallet_MessageApproval">dwallet::MessageApproval</a>): (<a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, u8, <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="dwallet.md#0x3_dwallet_remove_message_approval">remove_message_approval</a>(message_approval: <a href="dwallet.md#0x3_dwallet_MessageApproval">MessageApproval</a>): (ID, u8, <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;) {
    <b>let</b> <a href="dwallet.md#0x3_dwallet_MessageApproval">MessageApproval</a> {
        dwallet_cap_id,
        hash_scheme,
        message
    } = message_approval;
    (dwallet_cap_id, hash_scheme, message)
}
</code></pre>



</details>

<a name="0x3_dwallet_pop_and_verify_message_approval"></a>

## Function `pop_and_verify_message_approval`

Pops the last message approval from the vector and verifies it against the given message & dwallet_cap_id.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet.md#0x3_dwallet_pop_and_verify_message_approval">pop_and_verify_message_approval</a>(dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, message_hash: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, message_approvals: &<b>mut</b> <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet.md#0x3_dwallet_MessageApproval">dwallet::MessageApproval</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet.md#0x3_dwallet_pop_and_verify_message_approval">pop_and_verify_message_approval</a>(
    dwallet_cap_id: ID,
    message_hash: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    message_approvals: &<b>mut</b> <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet.md#0x3_dwallet_MessageApproval">MessageApproval</a>&gt;
) {
    <b>let</b> message_approval = <a href="../move-stdlib/vector.md#0x1_vector_pop_back">vector::pop_back</a>(message_approvals);
    <b>let</b> (message_approval_dwallet_cap_id, _hash_scheme, approved_message) = <a href="dwallet.md#0x3_dwallet_remove_message_approval">remove_message_approval</a>(message_approval);
    <b>assert</b>!(dwallet_cap_id == message_approval_dwallet_cap_id, <a href="dwallet.md#0x3_dwallet_EMessageApprovalDWalletMismatch">EMessageApprovalDWalletMismatch</a>);
    <b>assert</b>!(&message_hash == &approved_message, <a href="dwallet.md#0x3_dwallet_EMissingApprovalOrWrongApprovalOrder">EMissingApprovalOrWrongApprovalOrder</a>);
}
</code></pre>



</details>

<a name="0x3_dwallet_hash_messages"></a>

## Function `hash_messages`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet.md#0x3_dwallet_hash_messages">hash_messages</a>(message_approvals: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet.md#0x3_dwallet_MessageApproval">dwallet::MessageApproval</a>&gt;): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet.md#0x3_dwallet_hash_messages">hash_messages</a>(message_approvals: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet.md#0x3_dwallet_MessageApproval">MessageApproval</a>&gt;): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;{
    <b>let</b> <b>mut</b> hashed_messages = <a href="../move-stdlib/vector.md#0x1_vector_empty">vector::empty</a>();
    <b>let</b> messages_length = <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(message_approvals);
    <b>let</b> <b>mut</b> i: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
    <b>while</b> (i &lt; messages_length) {
        <b>let</b> message = &message_approvals[i].message;
        <b>let</b> hash_scheme = message_approvals[i].hash_scheme;
        <b>let</b> hashed_message = <a href="dwallet.md#0x3_dwallet_hash_message">hash_message</a>(*message, hash_scheme);
        <a href="../move-stdlib/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> hashed_messages, hashed_message);
        i = i + 1;
    };
    hashed_messages
}
</code></pre>



</details>

<a name="0x3_dwallet_hash_message"></a>

## Function `hash_message`

Hashes the given message using the specified hash scheme.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet.md#0x3_dwallet_hash_message">hash_message</a>(message: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, hash_scheme: u8): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet.md#0x3_dwallet_hash_message">hash_message</a>(message: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, hash_scheme: u8): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <b>assert</b>!(<a href="dwallet.md#0x3_dwallet_is_supported_hash_scheme">is_supported_hash_scheme</a>(hash_scheme), <a href="dwallet.md#0x3_dwallet_EInvalidHashScheme">EInvalidHashScheme</a>);
    <b>return</b> match (hash_scheme) {
            <a href="dwallet.md#0x3_dwallet_KECCAK256">KECCAK256</a> =&gt; <a href="../pera-framework/hash.md#0x2_hash_keccak256">hash::keccak256</a>(&message),
            <a href="dwallet.md#0x3_dwallet_SHA256">SHA256</a> =&gt; std::hash::sha2_256(message),
            _ =&gt; <a href="../move-stdlib/vector.md#0x1_vector_empty">vector::empty</a>(),
    }
}
</code></pre>



</details>

<a name="0x3_dwallet_is_supported_hash_scheme"></a>

## Function `is_supported_hash_scheme`

Checks if the given hash scheme is supported for message signing.


<pre><code><b>fun</b> <a href="dwallet.md#0x3_dwallet_is_supported_hash_scheme">is_supported_hash_scheme</a>(val: u8): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="dwallet.md#0x3_dwallet_is_supported_hash_scheme">is_supported_hash_scheme</a>(val: u8): bool {
    <b>return</b> match (val) {
            <a href="dwallet.md#0x3_dwallet_KECCAK256">KECCAK256</a> | <a href="dwallet.md#0x3_dwallet_SHA256">SHA256</a> =&gt; <b>true</b>,
    _ =&gt; <b>false</b>,
}
}
</code></pre>



</details>
