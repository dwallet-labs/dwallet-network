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
-  [Resource `PartialCentralizedSignedMessages`](#0x3_dwallet_PartialCentralizedSignedMessages)
-  [Resource `DWalletCap`](#0x3_dwallet_DWalletCap)
-  [Resource `EncryptionKey`](#0x3_dwallet_EncryptionKey)
-  [Struct `CreatedEncryptionKeyEvent`](#0x3_dwallet_CreatedEncryptionKeyEvent)
-  [Struct `StartEncryptionKeyVerificationEvent`](#0x3_dwallet_StartEncryptionKeyVerificationEvent)
-  [Resource `ActiveEncryptionKeys`](#0x3_dwallet_ActiveEncryptionKeys)
-  [Struct `MessageApproval`](#0x3_dwallet_MessageApproval)
-  [Constants](#@Constants_3)
-  [Function `create_dwallet`](#0x3_dwallet_create_dwallet)
-  [Function `create_partial_centralized_signed_messages`](#0x3_dwallet_create_partial_centralized_signed_messages)
-  [Function `transfer_partial_centralized_signed_messages`](#0x3_dwallet_transfer_partial_centralized_signed_messages)
-  [Function `unpack_partial_centralized_signed_messages`](#0x3_dwallet_unpack_partial_centralized_signed_messages)
-  [Function `get_dwallet_cap_id`](#0x3_dwallet_get_dwallet_cap_id)
-  [Function `get_dwallet_decentralized_public_output`](#0x3_dwallet_get_dwallet_decentralized_public_output)
-  [Function `get_dwallet_centralized_public_output`](#0x3_dwallet_get_dwallet_centralized_public_output)
-  [Function `get_dwallet_mpc_network_decryption_key_version`](#0x3_dwallet_get_dwallet_mpc_network_decryption_key_version)
-  [Function `create_dwallet_cap`](#0x3_dwallet_create_dwallet_cap)
-  [Function `get_encryption_key`](#0x3_dwallet_get_encryption_key)
-  [Function `create_active_encryption_keys`](#0x3_dwallet_create_active_encryption_keys)
-  [Function `get_active_encryption_key`](#0x3_dwallet_get_active_encryption_key)
-  [Function `upsert_active_encryption_key`](#0x3_dwallet_upsert_active_encryption_key)
-  [Function `register_encryption_key`](#0x3_dwallet_register_encryption_key)
-  [Function `create_encryption_key`](#0x3_dwallet_create_encryption_key)
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

<code><a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a></code> represents a decentralized wallet that is
created after the DKG process.


<pre><code><b>struct</b> <a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a>&lt;T&gt; <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../pera-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>
 Unique identifier for the dWallet.
</dd>
<dt>
<code>session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The session ID that generated this dWallet.
</dd>
<dt>
<code>dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The ID of the capability associated with this dWallet.
</dd>
<dt>
<code>decentralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The decentralized party public output in the DKG process.
</dd>
<dt>
<code>centralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The centralized party public output in the DKG process.
</dd>
<dt>
<code>dwallet_mpc_network_decryption_key_version: u8</code>
</dt>
<dd>
 The MPC network decryption key version that is used to decrypt this dWallet.
</dd>
</dl>


</details>

<a name="0x3_dwallet_PartialCentralizedSignedMessages"></a>

## Resource `PartialCentralizedSignedMessages`

Messages that have been signed by a user, a.k.a the centralized party,
but not yet by the blockchain.
Used for scenarios where the user needs to first agree to sign some transaction,
and the blockchain signs this transaction only later,
when some other conditions are met.

Can be used to implement an order-book-based exchange, for example.
User A first agrees to buy BTC with ETH at price X, and signs a transaction with this information.
When a matching user B, that agrees to sell BTC for ETH at price X,
signs a transaction with this information,
the blockchain can sign both transactions, and the exchange is completed.


<pre><code><b>struct</b> <a href="dwallet.md#0x3_dwallet_PartialCentralizedSignedMessages">PartialCentralizedSignedMessages</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../pera-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>
 A unique identifier for this <code><a href="dwallet.md#0x3_dwallet_PartialCentralizedSignedMessages">PartialCentralizedSignedMessages</a></code> object.
</dd>
<dt>
<code>presigns: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>
 The presigns bytes for each message.
 The matching presign objects are being "burned" before this object is created.
</dd>
<dt>
<code>presign_session_ids: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>&gt;</code>
</dt>
<dd>
 The presigns session IDs.
</dd>
<dt>
<code>messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>
 The messages that are being signed.
</dd>
<dt>
<code>signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>
 The user centralized signatures for each message.
</dd>
<dt>
<code>dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The unique identifier of the associated dWallet.
</dd>
<dt>
<code>dwallet_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The DKG output of the dWallet.
</dd>
<dt>
<code>dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The unique identifier of the dWallet capability.
</dd>
<dt>
<code>dwallet_mpc_network_decryption_key_version: u8</code>
</dt>
<dd>
 The MPC network decryption key version that is used to decrypt the associated dWallet.
</dd>
</dl>


</details>

<a name="0x3_dwallet_DWalletCap"></a>

## Resource `DWalletCap`

Represents a capability granting control over a specific dWallet.


<pre><code><b>struct</b> <a href="dwallet.md#0x3_dwallet_DWalletCap">DWalletCap</a> <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../pera-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>
 Unique identifier for the dWallet capability.
</dd>
</dl>


</details>

<a name="0x3_dwallet_EncryptionKey"></a>

## Resource `EncryptionKey`

todo(zeev): check why we transfer both public key and address.
Represents an encryption key used to encrypt a dWallet centralized (user) secret key share.

Encryption keys facilitate secure data transfer between accounts on the
dWallet Network by ensuring that sensitive information remains confidential during transmission.
Each address on the dWallet Network is associated with a unique encryption key.
When an external party intends to send encrypted data to a particular account, they use the recipient’s
encryption key to encrypt the data. The recipient is then the sole entity capable of decrypting
and accessing this information, ensuring secure, end-to-end encryption.


<pre><code><b>struct</b> <a href="dwallet.md#0x3_dwallet_EncryptionKey">EncryptionKey</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../pera-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>
 Unique identifier for the <code><a href="dwallet.md#0x3_dwallet_EncryptionKey">EncryptionKey</a></code>.
</dd>
<dt>
<code>encryption_key_scheme: u8</code>
</dt>
<dd>
 Scheme identifier for the encryption key (e.g., Class Groups).
</dd>
<dt>
<code>encryption_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 Serialized encryption key.
</dd>
<dt>
<code>key_owner_address: <b>address</b></code>
</dt>
<dd>
 Address of the encryption key owner.
</dd>
<dt>
<code>encryption_key_signature: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 Signature for the encryption key, signed by the <code>key_signer_public_key</code>.
</dd>
<dt>
<code>key_signer_public_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The public key that was used to sign the <code>encryption_key</code>.
</dd>
</dl>


</details>

<a name="0x3_dwallet_CreatedEncryptionKeyEvent"></a>

## Struct `CreatedEncryptionKeyEvent`

Event emitted when an encryption key is created.

This event is emitted after the blockchain verifies the encryption key's validity
and creates the corresponding <code><a href="dwallet.md#0x3_dwallet_EncryptionKey">EncryptionKey</a></code> object.


<pre><code><b>struct</b> <a href="dwallet.md#0x3_dwallet_CreatedEncryptionKeyEvent">CreatedEncryptionKeyEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 A unique identifier for the session related to the encryption key creation.
</dd>
<dt>
<code>encryption_key_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The unique identifier of the created <code><a href="dwallet.md#0x3_dwallet_EncryptionKey">EncryptionKey</a></code> object.
</dd>
</dl>


</details>

<a name="0x3_dwallet_StartEncryptionKeyVerificationEvent"></a>

## Struct `StartEncryptionKeyVerificationEvent`

Event emitted to start an encryption key verification process.

Since Ika does not support native functions, this event is emitted and
caught by the blockchain to initiate the verification process.
This process ensures the encryption key's validity and compliance with the system's requirements.


<pre><code><b>struct</b> <a href="dwallet.md#0x3_dwallet_StartEncryptionKeyVerificationEvent">StartEncryptionKeyVerificationEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>encryption_key_scheme: u8</code>
</dt>
<dd>
 Scheme identifier for the encryption key (e.g., Class Groups).
</dd>
<dt>
<code>encryption_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 Serialized encryption key to be verified.
</dd>
<dt>
<code>encryption_key_signature: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 Signature for the encryption key.
</dd>
<dt>
<code>key_signer_public_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The public key of the signer, used to verify
 the signature on the encryption key.
</dd>
<dt>
<code>initiator: <b>address</b></code>
</dt>
<dd>
 The address of the user initiating the verification process.
</dd>
<dt>
<code>session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 A unique identifier for the session related to this verification.
</dd>
</dl>


</details>

<a name="0x3_dwallet_ActiveEncryptionKeys"></a>

## Resource `ActiveEncryptionKeys`

Shared object that holds the active encryption keys per user.

This object maintains a mapping between user addresses and their active encryption keys,
enabling efficient retrieval and management of encryption keys within the Ika blockchain.


<pre><code><b>struct</b> <a href="dwallet.md#0x3_dwallet_ActiveEncryptionKeys">ActiveEncryptionKeys</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../pera-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>
 Unique identifier for the <code><a href="dwallet.md#0x3_dwallet_ActiveEncryptionKeys">ActiveEncryptionKeys</a></code> object.
</dd>
<dt>
<code>encryption_keys: <a href="../pera-framework/table.md#0x2_table_Table">table::Table</a>&lt;<b>address</b>, <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>&gt;</code>
</dt>
<dd>
 A table mapping user addresses to encryption key object IDs.
</dd>
</dl>


</details>

<a name="0x3_dwallet_MessageApproval"></a>

## Struct `MessageApproval`

Represents a message that was approved as part of a dWallet process.

This struct binds the message to a specific <code><a href="dwallet.md#0x3_dwallet_DWalletCap">DWalletCap</a></code> for
traceability and accountability within the system.


<a name="@Fields_2"></a>

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

<a name="@Constants_3"></a>

## Constants


<a name="0x3_dwallet_ENotSystemAddress"></a>



<pre><code><b>const</b> <a href="dwallet.md#0x3_dwallet_ENotSystemAddress">ENotSystemAddress</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 3;
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



<a name="0x3_dwallet_KEY_SCHEME_CLASS_GROUPS"></a>



<pre><code><b>const</b> <a href="dwallet.md#0x3_dwallet_KEY_SCHEME_CLASS_GROUPS">KEY_SCHEME_CLASS_GROUPS</a>: u8 = 0;
</code></pre>



<a name="0x3_dwallet_SHA256"></a>



<pre><code><b>const</b> <a href="dwallet.md#0x3_dwallet_SHA256">SHA256</a>: u8 = 1;
</code></pre>



<a name="0x3_dwallet_SYSTEM_ADDRESS"></a>



<pre><code><b>const</b> <a href="dwallet.md#0x3_dwallet_SYSTEM_ADDRESS">SYSTEM_ADDRESS</a>: <b>address</b> = 0;
</code></pre>



<a name="0x3_dwallet_create_dwallet"></a>

## Function `create_dwallet`

Creates a new [<code><a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a></code>] object of type <code>T</code>.

This function initializes a decentralized wallet (<code><a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a></code>) after the second DKG round,
linking it to the appropriate capability ID and storing the outputs from the DKG process.


<a name="@Parameters_4"></a>

##### Parameters

- <code>session_id</code>: A unique identifier for the session that generated this dWallet.
- <code>dwallet_cap_id</code>: The unique identifier for the capability associated with this dWallet.
- <code>decentralized_public_output</code>: The decentralized public output produced during the second DKG round.
- <code>dwallet_mpc_network_decryption_key_version</code>: The version of the MPC network decryption key
used for this dWallet.
- <code>centralized_public_output</code>: The centralized public output produced during the DKG process.
- <code>ctx</code>: A mutable transaction context used to create the dWallet object.


<a name="@Returns_5"></a>

##### Returns

A new [<code><a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a></code>] object of type <code>T</code>.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet.md#0x3_dwallet_create_dwallet">create_dwallet</a>&lt;T: drop&gt;(session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, decentralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, dwallet_mpc_network_decryption_key_version: u8, centralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet.md#0x3_dwallet_create_dwallet">create_dwallet</a>&lt;T: drop&gt;(
    session_id: ID,
    dwallet_cap_id: ID,
    decentralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    dwallet_mpc_network_decryption_key_version: u8,
    centralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext
): <a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a>&lt;T&gt; {
    <a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a>&lt;T&gt; {
        id: <a href="../pera-framework/object.md#0x2_object_new">object::new</a>(ctx),
        session_id,
        dwallet_cap_id,
        decentralized_public_output,
        dwallet_mpc_network_decryption_key_version,
        centralized_public_output,
    }
}
</code></pre>



</details>

<a name="0x3_dwallet_create_partial_centralized_signed_messages"></a>

## Function `create_partial_centralized_signed_messages`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet.md#0x3_dwallet_create_partial_centralized_signed_messages">create_partial_centralized_signed_messages</a>&lt;T: drop&gt;(presigns_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, presign_session_ids: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>&gt;, messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, <a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;T&gt;, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="dwallet.md#0x3_dwallet_PartialCentralizedSignedMessages">dwallet::PartialCentralizedSignedMessages</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet.md#0x3_dwallet_create_partial_centralized_signed_messages">create_partial_centralized_signed_messages</a>&lt;T: drop&gt;(
    presigns_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    presign_session_ids: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;ID&gt;,
    messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    <a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a>&lt;T&gt;,
    ctx: &<b>mut</b> TxContext
): <a href="dwallet.md#0x3_dwallet_PartialCentralizedSignedMessages">PartialCentralizedSignedMessages</a> {
    <a href="dwallet.md#0x3_dwallet_PartialCentralizedSignedMessages">PartialCentralizedSignedMessages</a> {
        id: <a href="../pera-framework/object.md#0x2_object_new">object::new</a>(ctx),
        presigns: presigns_bytes,
        presign_session_ids,
        messages,
        signatures,
        dwallet_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(<a href="dwallet.md#0x3_dwallet">dwallet</a>),
        dwallet_output: <a href="dwallet.md#0x3_dwallet">dwallet</a>.decentralized_public_output,
        dwallet_cap_id: <a href="dwallet.md#0x3_dwallet">dwallet</a>.dwallet_cap_id,
        dwallet_mpc_network_decryption_key_version: <a href="dwallet.md#0x3_dwallet">dwallet</a>.dwallet_mpc_network_decryption_key_version,
    }
}
</code></pre>



</details>

<a name="0x3_dwallet_transfer_partial_centralized_signed_messages"></a>

## Function `transfer_partial_centralized_signed_messages`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet.md#0x3_dwallet_transfer_partial_centralized_signed_messages">transfer_partial_centralized_signed_messages</a>(partial_signatures: <a href="dwallet.md#0x3_dwallet_PartialCentralizedSignedMessages">dwallet::PartialCentralizedSignedMessages</a>, target: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet.md#0x3_dwallet_transfer_partial_centralized_signed_messages">transfer_partial_centralized_signed_messages</a>(
    partial_signatures: <a href="dwallet.md#0x3_dwallet_PartialCentralizedSignedMessages">PartialCentralizedSignedMessages</a>,
    target: <b>address</b>,
) {
    <a href="../pera-framework/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(partial_signatures, target);
}
</code></pre>



</details>

<a name="0x3_dwallet_unpack_partial_centralized_signed_messages"></a>

## Function `unpack_partial_centralized_signed_messages`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet.md#0x3_dwallet_unpack_partial_centralized_signed_messages">unpack_partial_centralized_signed_messages</a>(partial_centralized_signed_messages: <a href="dwallet.md#0x3_dwallet_PartialCentralizedSignedMessages">dwallet::PartialCentralizedSignedMessages</a>): (<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>&gt;, <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet.md#0x3_dwallet_unpack_partial_centralized_signed_messages">unpack_partial_centralized_signed_messages</a>(
    partial_centralized_signed_messages: <a href="dwallet.md#0x3_dwallet_PartialCentralizedSignedMessages">PartialCentralizedSignedMessages</a>
): ( <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;ID&gt;, <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, ID, <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ID, u8) {

    <b>let</b> <a href="dwallet.md#0x3_dwallet_PartialCentralizedSignedMessages">PartialCentralizedSignedMessages</a> {
        id,
        presigns,
        presign_session_ids,
        messages,
        signatures,
        dwallet_id,
        dwallet_output,
        dwallet_cap_id,
        dwallet_mpc_network_decryption_key_version,
    } = partial_centralized_signed_messages;

    <a href="../pera-framework/object.md#0x2_object_delete">object::delete</a>(id);
    (
        presigns,
        presign_session_ids,
        messages,
        signatures,
        dwallet_id,
        dwallet_output,
        dwallet_cap_id,
        dwallet_mpc_network_decryption_key_version,
    )
}
</code></pre>



</details>

<a name="0x3_dwallet_get_dwallet_cap_id"></a>

## Function `get_dwallet_cap_id`

Retrieve the ID of the <code><a href="dwallet.md#0x3_dwallet_DWalletCap">DWalletCap</a></code> associated with a given dWallet.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet.md#0x3_dwallet_get_dwallet_cap_id">get_dwallet_cap_id</a>&lt;T: drop&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;T&gt;): <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet.md#0x3_dwallet_get_dwallet_cap_id">get_dwallet_cap_id</a>&lt;T: drop&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a>&lt;T&gt;): ID {
    <a href="dwallet.md#0x3_dwallet">dwallet</a>.dwallet_cap_id
}
</code></pre>



</details>

<a name="0x3_dwallet_get_dwallet_decentralized_public_output"></a>

## Function `get_dwallet_decentralized_public_output`

Retrieves the decentralized public output of the second DKG round for a given dWallet..


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet.md#0x3_dwallet_get_dwallet_decentralized_public_output">get_dwallet_decentralized_public_output</a>&lt;T: drop&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;T&gt;): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet.md#0x3_dwallet_get_dwallet_decentralized_public_output">get_dwallet_decentralized_public_output</a>&lt;T: drop&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a>&lt;T&gt;): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <a href="dwallet.md#0x3_dwallet">dwallet</a>.decentralized_public_output
}
</code></pre>



</details>

<a name="0x3_dwallet_get_dwallet_centralized_public_output"></a>

## Function `get_dwallet_centralized_public_output`

Retrieves the centralized public output for a given dWallet.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet.md#0x3_dwallet_get_dwallet_centralized_public_output">get_dwallet_centralized_public_output</a>&lt;T: drop&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;T&gt;): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet.md#0x3_dwallet_get_dwallet_centralized_public_output">get_dwallet_centralized_public_output</a>&lt;T: drop&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a>&lt;T&gt;): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <a href="dwallet.md#0x3_dwallet">dwallet</a>.centralized_public_output
}
</code></pre>



</details>

<a name="0x3_dwallet_get_dwallet_mpc_network_decryption_key_version"></a>

## Function `get_dwallet_mpc_network_decryption_key_version`

Retrieves the MPC network decryption key version for a given dWallet.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet.md#0x3_dwallet_get_dwallet_mpc_network_decryption_key_version">get_dwallet_mpc_network_decryption_key_version</a>&lt;T: drop&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;T&gt;): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet.md#0x3_dwallet_get_dwallet_mpc_network_decryption_key_version">get_dwallet_mpc_network_decryption_key_version</a>&lt;T: drop&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a>&lt;T&gt;): u8 {
    <a href="dwallet.md#0x3_dwallet">dwallet</a>.dwallet_mpc_network_decryption_key_version
}
</code></pre>



</details>

<a name="0x3_dwallet_create_dwallet_cap"></a>

## Function `create_dwallet_cap`

Create a new [<code><a href="dwallet.md#0x3_dwallet_DWalletCap">DWalletCap</a></code>] object.

The holder of the <code><a href="dwallet.md#0x3_dwallet_DWalletCap">DWalletCap</a></code> has control and ownership over
the associated <code><a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a></code>.


<a name="@Returns_6"></a>

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

<a name="0x3_dwallet_get_encryption_key"></a>

## Function `get_encryption_key`

Retrieves the encryption key from an <code><a href="dwallet.md#0x3_dwallet_EncryptionKey">EncryptionKey</a></code> object.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet.md#0x3_dwallet_get_encryption_key">get_encryption_key</a>(key: &<a href="dwallet.md#0x3_dwallet_EncryptionKey">dwallet::EncryptionKey</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet.md#0x3_dwallet_get_encryption_key">get_encryption_key</a>(key: &<a href="dwallet.md#0x3_dwallet_EncryptionKey">EncryptionKey</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    key.encryption_key
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

Registers an encryption key to be used later for encrypting a
centralized secret key share.

The encryption key is saved as an immutable object after verification.
This function emits an event (<code><a href="dwallet.md#0x3_dwallet_StartEncryptionKeyVerificationEvent">StartEncryptionKeyVerificationEvent</a></code>) that is caught
by the blockchain.
The blockchain then performs necessary verifications and invokes <code><a href="dwallet.md#0x3_dwallet_create_encryption_key">create_encryption_key</a>()</code>
to finalize and store the key. This flow is required because verification can only
be implemented in Rust, as native functions are not supported in Ika.


<a name="@Parameters_7"></a>

##### Parameters

- <code>encryption_key</code>: The serialized encryption key to be registered.
- <code>encryption_key_signature</code>: The signature of the encryption key, signed by the signer.
- <code>key_signer_public_key</code>: The public key of the signer used to verify the encryption key signature.
- <code>encryption_key_scheme</code>: The scheme of the encryption key (e.g., Class Groups).


<pre><code><b>public</b> <b>fun</b> <a href="dwallet.md#0x3_dwallet_register_encryption_key">register_encryption_key</a>(encryption_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, encryption_key_signature: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, key_signer_public_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, encryption_key_scheme: u8, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet.md#0x3_dwallet_register_encryption_key">register_encryption_key</a>(
    encryption_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    encryption_key_signature: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    key_signer_public_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    encryption_key_scheme: u8,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(<a href="dwallet.md#0x3_dwallet_is_valid_encryption_key_scheme">is_valid_encryption_key_scheme</a>(encryption_key_scheme), <a href="dwallet.md#0x3_dwallet_EInvalidEncryptionKeyScheme">EInvalidEncryptionKeyScheme</a>);
    <b>assert</b>!(
        ed25519_verify(&encryption_key_signature, &key_signer_public_key, &encryption_key),
        <a href="dwallet.md#0x3_dwallet_EInvalidEncryptionKeySignature">EInvalidEncryptionKeySignature</a>
    );
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(
        <a href="dwallet.md#0x3_dwallet_StartEncryptionKeyVerificationEvent">StartEncryptionKeyVerificationEvent</a> {
            encryption_key_scheme,
            encryption_key,
            encryption_key_signature,
            key_signer_public_key,
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

This function is called by the blockchain after it verifies that the
<code>key_signer_public_key</code> matches the <code>initiator</code> address. This flow ensures
that verification is handled securely in Rust since native functions are
not supported in Ika.

The created encryption key object is immutable.
An event (<code><a href="dwallet.md#0x3_dwallet_CreatedEncryptionKeyEvent">CreatedEncryptionKeyEvent</a></code>) is emitted to signal the successful
creation of the encryption key.


<a name="@Parameters_8"></a>

##### Parameters

- <code>encryption_key</code>: The serialized encryption key to be created.
- <code>encryption_key_signature</code>: The signature of the encryption key, signed by the signer.
- <code>signer_public_key</code>: The public key of the signer used to verify the encryption key.
- <code>encryption_key_scheme</code>: The scheme of the encryption key (e.g., Class Groups).
- <code>initiator</code>: The address of the user initiating the encryption key creation.
- <code>session_id</code>: A unique identifier for the session associated with this encryption key.
- <code>ctx</code>: A mutable transaction context used to create and freeze the encryption key object.


<pre><code><b>fun</b> <a href="dwallet.md#0x3_dwallet_create_encryption_key">create_encryption_key</a>(encryption_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, encryption_key_signature: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, key_signer_public_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, encryption_key_scheme: u8, initiator: <b>address</b>, session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="dwallet.md#0x3_dwallet_create_encryption_key">create_encryption_key</a>(
    encryption_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    encryption_key_signature: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    key_signer_public_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    encryption_key_scheme: u8,
    initiator: <b>address</b>,
    session_id: ID,
    ctx: &<b>mut</b> TxContext
) {
    // Ensure the caller is the system <b>address</b>
    <b>assert</b>!(<a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == <a href="dwallet.md#0x3_dwallet_SYSTEM_ADDRESS">SYSTEM_ADDRESS</a>, <a href="dwallet.md#0x3_dwallet_ENotSystemAddress">ENotSystemAddress</a>);

    // Create the encryption key <a href="../pera-framework/object.md#0x2_object">object</a>
    <b>let</b> encryption_key_obj = <a href="dwallet.md#0x3_dwallet_EncryptionKey">EncryptionKey</a> {
        id: <a href="../pera-framework/object.md#0x2_object_new">object::new</a>(ctx),
        encryption_key_scheme,
        encryption_key,
        key_owner_address: initiator,
        encryption_key_signature,
        key_signer_public_key,
    };

    // Emit an <a href="../pera-framework/event.md#0x2_event">event</a> <b>to</b> signal the creation of the encryption key
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet.md#0x3_dwallet_CreatedEncryptionKeyEvent">CreatedEncryptionKeyEvent</a> {
        encryption_key_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(&encryption_key_obj),
        session_id,
    });

    // Freeze the encryption key <a href="../pera-framework/object.md#0x2_object">object</a> <b>to</b> make it immutable
    <a href="../pera-framework/transfer.md#0x2_transfer_freeze_object">transfer::freeze_object</a>(encryption_key_obj);
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
    scheme == <a href="dwallet.md#0x3_dwallet_KEY_SCHEME_CLASS_GROUPS">KEY_SCHEME_CLASS_GROUPS</a>
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


<a name="@Parameters_9"></a>

##### Parameters

- <code>dwallet_cap</code>: A reference to the <code><a href="dwallet.md#0x3_dwallet_DWalletCap">DWalletCap</a></code> object representing the capability for which
the messages are being approved.
- <code>hash_scheme</code>: The hash scheme to be used for hashing the messages. For example:
- <code><a href="dwallet.md#0x3_dwallet_KECCAK256">KECCAK256</a></code>
- <code><a href="dwallet.md#0x3_dwallet_SHA256">SHA256</a></code>
- <code>messages</code>: A mutable vector containing the messages to be approved. The messages are removed
from this vector as they are processed and added to the approvals list.


<a name="@Returns_10"></a>

##### Returns

A vector of <code><a href="dwallet.md#0x3_dwallet_MessageApproval">MessageApproval</a></code> objects corresponding to the approved messages.


<a name="@Behavior_11"></a>

##### Behavior

- The function iterates over the provided <code>messages</code> vector, processes each message by creating
a <code><a href="dwallet.md#0x3_dwallet_MessageApproval">MessageApproval</a></code> object, and pushes it into the <code>message_approvals</code> vector.
- The messages are approved in reverse order and then reversed again to preserve their original order.


<a name="@Aborts_12"></a>

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
        <a href="../move-stdlib/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> message_approvals, <a href="dwallet.md#0x3_dwallet_create_message_approval">create_message_approval</a>(
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
    <b>let</b> (message_approval_dwallet_cap_id, _hash_scheme, approved_message) = <a href="dwallet.md#0x3_dwallet_remove_message_approval">remove_message_approval</a>(
        message_approval
    );
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


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet.md#0x3_dwallet_hash_messages">hash_messages</a>(message_approvals: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet.md#0x3_dwallet_MessageApproval">MessageApproval</a>&gt;): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt; {
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
