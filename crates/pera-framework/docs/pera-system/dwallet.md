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
-  [Resource `DWalletCap`](#0x3_dwallet_DWalletCap)
-  [Resource `EncryptionKey`](#0x3_dwallet_EncryptionKey)
-  [Struct `CreatedEncryptionKeyEvent`](#0x3_dwallet_CreatedEncryptionKeyEvent)
-  [Struct `StartEncryptionKeyVerificationEvent`](#0x3_dwallet_StartEncryptionKeyVerificationEvent)
-  [Resource `ActiveEncryptionKeys`](#0x3_dwallet_ActiveEncryptionKeys)
-  [Struct `MessageApproval`](#0x3_dwallet_MessageApproval)
-  [Resource `PartialCentralizedSignedMessages`](#0x3_dwallet_PartialCentralizedSignedMessages)
-  [Struct `StartBatchedSignEvent`](#0x3_dwallet_StartBatchedSignEvent)
-  [Struct `CompletedSignEvent`](#0x3_dwallet_CompletedSignEvent)
-  [Resource `BatchedSignOutput`](#0x3_dwallet_BatchedSignOutput)
-  [Struct `StartSignEvent`](#0x3_dwallet_StartSignEvent)
-  [Struct `SignatureAlgorithmData`](#0x3_dwallet_SignatureAlgorithmData)
-  [Struct `CreatedPartialCentralizedSignedMessagesEvent`](#0x3_dwallet_CreatedPartialCentralizedSignedMessagesEvent)
-  [Constants](#@Constants_3)
-  [Function `create_dwallet`](#0x3_dwallet_create_dwallet)
-  [Function `get_dwallet_cap_id`](#0x3_dwallet_get_dwallet_cap_id)
-  [Function `get_dwallet_decentralized_public_output`](#0x3_dwallet_get_dwallet_decentralized_public_output)
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
-  [Function `hash_messages`](#0x3_dwallet_hash_messages)
-  [Function `hash_message`](#0x3_dwallet_hash_message)
-  [Function `is_supported_hash_scheme`](#0x3_dwallet_is_supported_hash_scheme)
-  [Function `create_partial_centralized_signed_messages`](#0x3_dwallet_create_partial_centralized_signed_messages)
-  [Function `sign`](#0x3_dwallet_sign)
    -  [Effects](#@Effects_13)
    -  [Aborts](#@Aborts_14)
    -  [Parameters](#@Parameters_15)
    -  [Type Parameters](#@Type_Parameters_16)
-  [Function `emit_sign_events`](#0x3_dwallet_emit_sign_events)
    -  [Effects](#@Effects_17)
    -  [Aborts](#@Aborts_18)
-  [Function `create_sign_output`](#0x3_dwallet_create_sign_output)
-  [Function `request_future_sign`](#0x3_dwallet_request_future_sign)
-  [Function `create_signature_algorithm_data`](#0x3_dwallet_create_signature_algorithm_data)
-  [Function `sign_with_partial_centralized_message_signatures`](#0x3_dwallet_sign_with_partial_centralized_message_signatures)
        -  [Type Parameters](#@Type_Parameters_23)
        -  [Parameters](#@Parameters_24)
        -  [Notes](#@Notes_25)
-  [Function `compare_messages_with_approvals`](#0x3_dwallet_compare_messages_with_approvals)


<pre><code><b>use</b> <a href="../move-stdlib/hash.md#0x1_hash">0x1::hash</a>;
<b>use</b> <a href="../move-stdlib/vector.md#0x1_vector">0x1::vector</a>;
<b>use</b> <a href="../pera-framework/ed25519.md#0x2_ed25519">0x2::ed25519</a>;
<b>use</b> <a href="../pera-framework/event.md#0x2_event">0x2::event</a>;
<b>use</b> <a href="../pera-framework/hash.md#0x2_hash">0x2::hash</a>;
<b>use</b> <a href="../pera-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../pera-framework/table.md#0x2_table">0x2::table</a>;
<b>use</b> <a href="../pera-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../pera-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="pera_system.md#0x3_pera_system">0x3::pera_system</a>;
</code></pre>



<a name="0x3_dwallet_DWallet"></a>

## Resource `DWallet`

<code><a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a></code> represents a decentralized wallet (dWallet) that is
created after the Distributed key generation (DKG) process.


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
<code>dwallet_mpc_network_decryption_key_version: u8</code>
</dt>
<dd>
 The MPC network decryption key version that is used to decrypt this dWallet.
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

<a name="0x3_dwallet_PartialCentralizedSignedMessages"></a>

## Resource `PartialCentralizedSignedMessages`

Messages that have been signed by a user, a.k.a the centralized party,
but not yet by the blockchain.
Used for scenarios where the user needs to first agree to sign some transaction,
and the blockchain signs this transaction later,
when some other conditions are met.

Can be used to implement an order-book-based exchange, for example.
User <code>A</code> first agrees to buy BTC with ETH at price X, and signs a transaction with this information.
When a matching user <code>B</code>, that agrees to sell BTC for ETH at price X,
signs a transaction with this information,
the blockchain can sign both transactions, and the exchange is completed.

D: The type of data that can be stored with the object,
specific to each Digital Signature Algorithm.


<pre><code><b>struct</b> <a href="dwallet.md#0x3_dwallet_PartialCentralizedSignedMessages">PartialCentralizedSignedMessages</a>&lt;D: store&gt; <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../pera-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>
 A unique identifier for this object.
</dd>
<dt>
<code>messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>
 The messages that are being signed.
</dd>
<dt>
<code>dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The unique identifier of the associated dWallet.
</dd>
<dt>
<code>dwallet_decentralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The DKG decentralized party public output for the dWallet.
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
<dt>
<code>signature_algorithm_data: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;D&gt;</code>
</dt>
<dd>
 Data specific to every DSA implementation.
 Every message (from <code>messages</code>) corresponds with a <code>D</code>, so the order of
 the messages and the data fields must be maintained.
</dd>
</dl>


</details>

<a name="0x3_dwallet_StartBatchedSignEvent"></a>

## Struct `StartBatchedSignEvent`

Event emitted to start a batched sign process.


<pre><code><b>struct</b> <a href="dwallet.md#0x3_dwallet_StartBatchedSignEvent">StartBatchedSignEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The session identifier for the batched sign process.
</dd>
<dt>
<code>hashed_messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>
 A list of hashed messages to be signed.
 Todo (#565): Move the hash calculation into the Rust code.
</dd>
<dt>
<code>initiator: <b>address</b></code>
</dt>
<dd>
 The address of the user who initiated the protocol.
</dd>
</dl>


</details>

<a name="0x3_dwallet_CompletedSignEvent"></a>

## Struct `CompletedSignEvent`

Event emitted to signal the completion of a Sign process.

This event contains signatures for all signed messages in the batch.


<pre><code><b>struct</b> <a href="dwallet.md#0x3_dwallet_CompletedSignEvent">CompletedSignEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The session identifier for the signing process.
</dd>
<dt>
<code>output_object_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The ID of the batched sign output object (<code><a href="dwallet.md#0x3_dwallet_BatchedSignOutput">BatchedSignOutput</a></code>).
</dd>
<dt>
<code>signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>
 List of signatures in the same order as the sign function message approvals input.
</dd>
<dt>
<code>is_future_sign: bool</code>
</dt>
<dd>
 Indicates whether the future sign feature was used to start the session.
</dd>
</dl>


</details>

<a name="0x3_dwallet_BatchedSignOutput"></a>

## Resource `BatchedSignOutput`

The output of a batched Sign session.


<pre><code><b>struct</b> <a href="dwallet.md#0x3_dwallet_BatchedSignOutput">BatchedSignOutput</a> <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../pera-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>
 A unique identifier for the batched sign output.
</dd>
<dt>
<code>session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The session identifier for the batched sign process.
</dd>
<dt>
<code>signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>
 A collection of signatures (of the sign process messages)
 generated in the batched sign session.
</dd>
<dt>
<code>dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The unique identifier of the associated dWallet.
</dd>
</dl>


</details>

<a name="0x3_dwallet_StartSignEvent"></a>

## Struct `StartSignEvent`

Event emitted to initiate the signing process.

This event is captured by Validators to start the signing protocol.
It includes all the necessary information to link the signing process
to a specific dWallet, and batched process.
D: The type of data that can be stored with the object,
specific to each Digital Signature Algorithm.


<pre><code><b>struct</b> <a href="dwallet.md#0x3_dwallet_StartSignEvent">StartSignEvent</a>&lt;D: <b>copy</b>, drop&gt; <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 A unique identifier for this signing session.
</dd>
<dt>
<code>initiator: <b>address</b></code>
</dt>
<dd>
 The address of the user who initiated the signing event.
</dd>
<dt>
<code>batched_session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 A unique identifier for the batched signing process this session belongs to.
</dd>
<dt>
<code>dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The unique identifier for the dWallet used in the session.
</dd>
<dt>
<code>dwallet_decentralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The output from the dWallet DKG process used in this session.
</dd>
<dt>
<code>hashed_message: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The hashed message to be signed in this session.
</dd>
<dt>
<code>dwallet_mpc_network_decryption_key_version: u8</code>
</dt>
<dd>
 The MPC network decryption key version that is used to decrypt the associated dWallet.
</dd>
<dt>
<code>signature_algorithm_data: D</code>
</dt>
<dd>
 Data that can be stored with the object,
 specific to each Digital Signature Algorithm.
</dd>
<dt>
<code>is_future_sign: bool</code>
</dt>
<dd>
 Indicates whether the future sign feature was used to start the session.
</dd>
</dl>


</details>

<a name="0x3_dwallet_SignatureAlgorithmData"></a>

## Struct `SignatureAlgorithmData`

Stores data essential for the signing process and specific to every signature algorithm implementation.
Must be passed to the signing functions and used in the programmable transaction block;
otherwise, the transaction will fail due to the "Hot Potato" pattern.
D: The type of data that can be stored with the object,
specific to each Digital Signature Algorithm.


<pre><code><b>struct</b> <a href="dwallet.md#0x3_dwallet_SignatureAlgorithmData">SignatureAlgorithmData</a>&lt;D: <b>copy</b>, drop, store&gt;
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>data: D</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_CreatedPartialCentralizedSignedMessagesEvent"></a>

## Struct `CreatedPartialCentralizedSignedMessagesEvent`

Event emitted when a [<code><a href="dwallet.md#0x3_dwallet_PartialCentralizedSignedMessages">PartialCentralizedSignedMessages</a></code>] object is created.


<pre><code><b>struct</b> <a href="dwallet.md#0x3_dwallet_CreatedPartialCentralizedSignedMessagesEvent">CreatedPartialCentralizedSignedMessagesEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>partial_signatures_object_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The unique identifier of the created <code><a href="dwallet.md#0x3_dwallet_PartialCentralizedSignedMessages">PartialCentralizedSignedMessages</a></code> object.
</dd>
</dl>


</details>

<a name="@Constants_3"></a>

## Constants


<a name="0x3_dwallet_ENotSystemAddress"></a>



<pre><code><b>const</b> <a href="dwallet.md#0x3_dwallet_ENotSystemAddress">ENotSystemAddress</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 3;
</code></pre>



<a name="0x3_dwallet_EExtraDataAndMessagesLenMismatch"></a>



<pre><code><b>const</b> <a href="dwallet.md#0x3_dwallet_EExtraDataAndMessagesLenMismatch">EExtraDataAndMessagesLenMismatch</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 7;
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


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet.md#0x3_dwallet_create_dwallet">create_dwallet</a>&lt;T: drop&gt;(session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, decentralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, dwallet_mpc_network_decryption_key_version: u8, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet.md#0x3_dwallet_create_dwallet">create_dwallet</a>&lt;T: drop&gt;(
    session_id: ID,
    dwallet_cap_id: ID,
    decentralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    dwallet_mpc_network_decryption_key_version: u8,
    ctx: &<b>mut</b> TxContext
): <a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a>&lt;T&gt; {
    <a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a>&lt;T&gt; {
        id: <a href="../pera-framework/object.md#0x2_object_new">object::new</a>(ctx),
        session_id,
        dwallet_cap_id,
        decentralized_public_output,
        dwallet_mpc_network_decryption_key_version,
    }
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
- <code>_pera_system_state</code>: The Pera system state object. Its ID is always 0x5.
Needed so the TX will get ordered in consensus before getting executed.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet.md#0x3_dwallet_register_encryption_key">register_encryption_key</a>(encryption_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, encryption_key_signature: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, key_signer_public_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, encryption_key_scheme: u8, _pera_system_state: &<a href="pera_system.md#0x3_pera_system_PeraSystemState">pera_system::PeraSystemState</a>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet.md#0x3_dwallet_register_encryption_key">register_encryption_key</a>(
    encryption_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    encryption_key_signature: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    key_signer_public_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    encryption_key_scheme: u8,
    // TODO (#529): Create a dedicated, small shared <a href="../pera-framework/object.md#0x2_object">object</a> instead of using the system state.
    _pera_system_state: &PeraSystemState,
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

<a name="0x3_dwallet_create_partial_centralized_signed_messages"></a>

## Function `create_partial_centralized_signed_messages`

Creates a new [<code><a href="dwallet.md#0x3_dwallet_PartialCentralizedSignedMessages">PartialCentralizedSignedMessages</a></code>] object.
This object is used to store messages that have been signed by a user,
but not yet by the blockchain.
T: The eliptic curve type used for the dWallet (for example <code>Secp256k1</code>).
D: The type of data that can be stored with the object,
specific to each Digital Signature Algorithm.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet.md#0x3_dwallet_create_partial_centralized_signed_messages">create_partial_centralized_signed_messages</a>&lt;T: drop, D: <b>copy</b>, drop, store&gt;(messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, <a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;T&gt;, signature_algorithm_data: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;D&gt;, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="dwallet.md#0x3_dwallet_PartialCentralizedSignedMessages">dwallet::PartialCentralizedSignedMessages</a>&lt;D&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet.md#0x3_dwallet_create_partial_centralized_signed_messages">create_partial_centralized_signed_messages</a>&lt;T: drop, D: store + drop + <b>copy</b>&gt;(
    messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    <a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a>&lt;T&gt;,
    signature_algorithm_data: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;D&gt;,
    ctx: &<b>mut</b> TxContext
): <a href="dwallet.md#0x3_dwallet_PartialCentralizedSignedMessages">PartialCentralizedSignedMessages</a>&lt;D&gt; {
    <a href="dwallet.md#0x3_dwallet_PartialCentralizedSignedMessages">PartialCentralizedSignedMessages</a>&lt;D&gt; {
        id: <a href="../pera-framework/object.md#0x2_object_new">object::new</a>(ctx),
        messages,
        dwallet_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(<a href="dwallet.md#0x3_dwallet">dwallet</a>),
        dwallet_decentralized_public_output: <a href="dwallet.md#0x3_dwallet">dwallet</a>.decentralized_public_output,
        dwallet_cap_id: <a href="dwallet.md#0x3_dwallet">dwallet</a>.dwallet_cap_id,
        dwallet_mpc_network_decryption_key_version: <a href="dwallet.md#0x3_dwallet">dwallet</a>.dwallet_mpc_network_decryption_key_version,
        signature_algorithm_data,
    }
}
</code></pre>



</details>

<a name="0x3_dwallet_sign"></a>

## Function `sign`

Initiates the signing process for a given dWallet of type T.

This function emits a <code><a href="dwallet.md#0x3_dwallet_StartSignEvent">StartSignEvent</a></code> and a <code><a href="dwallet.md#0x3_dwallet_StartBatchedSignEvent">StartBatchedSignEvent</a></code>,
providing all necessary metadata to ensure the integrity of the signing process.
It validates the linkage between the <code><a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a></code>, <code><a href="dwallet.md#0x3_dwallet_DWalletCap">DWalletCap</a></code>, and <code><a href="dwallet.md#0x3_dwallet_SignatureAlgorithmData">SignatureAlgorithmData</a></code> objects.


<a name="@Effects_13"></a>

### Effects

- Ensures a valid linkage between <code><a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a></code>, <code><a href="dwallet.md#0x3_dwallet_DWalletCap">DWalletCap</a></code>, and <code><a href="dwallet.md#0x3_dwallet_SignatureAlgorithmData">SignatureAlgorithmData</a></code>.
- Validates that <code>signature_algorithm_data</code> and <code>message_approvals</code> have the same length.
- Emits the following events:
- <code><a href="dwallet.md#0x3_dwallet_StartBatchedSignEvent">StartBatchedSignEvent</a></code>: Contains the session details and the list of hashed messages.
- <code><a href="dwallet.md#0x3_dwallet_StartSignEvent">StartSignEvent</a></code>: Includes details for each message signing process.


<a name="@Aborts_14"></a>

### Aborts

- **<code><a href="dwallet.md#0x3_dwallet_EExtraDataAndMessagesLenMismatch">EExtraDataAndMessagesLenMismatch</a></code>**: If the number of <code>hashed_messages</code> does not
match the number of <code>signature_algorithm_data</code>.
- **<code><a href="dwallet.md#0x3_dwallet_EMissingApprovalOrWrongApprovalOrder">EMissingApprovalOrWrongApprovalOrder</a></code>**: If the approvals are missing or provided in the incorrect order.


<a name="@Parameters_15"></a>

### Parameters

- <code>message_approvals</code>: A vector of <code><a href="dwallet.md#0x3_dwallet_MessageApproval">MessageApproval</a></code> objects representing
approvals for the messages, which are destroyed at the end of the transaction.
- <code><a href="dwallet.md#0x3_dwallet">dwallet</a></code>: A reference to the <code><a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a></code> object being used for signing.
- <code>signature_algorithm_data</code>: A vector of <code><a href="dwallet.md#0x3_dwallet_SignatureAlgorithmData">SignatureAlgorithmData</a></code> objects containing intermediate signing outputs,
which are unpacked and then destroyed at the end of the transaction.


<a name="@Type_Parameters_16"></a>

### Type Parameters

- <code>T</code>: The elliptic curve type used for the dWallet.
D: The type of data that can be stored with the object,
specific to each Digital Signature Algorithm.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet.md#0x3_dwallet_sign">sign</a>&lt;T: drop, D: <b>copy</b>, drop, store&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;T&gt;, message_approvals: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet.md#0x3_dwallet_MessageApproval">dwallet::MessageApproval</a>&gt;, signature_algorithm_data: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet.md#0x3_dwallet_SignatureAlgorithmData">dwallet::SignatureAlgorithmData</a>&lt;D&gt;&gt;, _pera_system_state: &<a href="pera_system.md#0x3_pera_system_PeraSystemState">pera_system::PeraSystemState</a>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet.md#0x3_dwallet_sign">sign</a>&lt;T: drop, D: <b>copy</b> + drop + store&gt;(
    <a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a>&lt;T&gt;,
    message_approvals: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet.md#0x3_dwallet_MessageApproval">MessageApproval</a>&gt;,
    signature_algorithm_data: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet.md#0x3_dwallet_SignatureAlgorithmData">SignatureAlgorithmData</a>&lt;D&gt;&gt;,
    _pera_system_state: &PeraSystemState,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> signature_algorithm_data_unpacked = vector::map!(signature_algorithm_data, | <a href="dwallet.md#0x3_dwallet_SignatureAlgorithmData">SignatureAlgorithmData</a> { data } | data);
    <a href="dwallet.md#0x3_dwallet_emit_sign_events">emit_sign_events</a>&lt;D&gt;(
        message_approvals,
        <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(<a href="dwallet.md#0x3_dwallet">dwallet</a>),
        <a href="dwallet.md#0x3_dwallet">dwallet</a>.dwallet_cap_id,
        <a href="dwallet.md#0x3_dwallet">dwallet</a>.decentralized_public_output,
        <a href="dwallet.md#0x3_dwallet">dwallet</a>.dwallet_mpc_network_decryption_key_version,
        signature_algorithm_data_unpacked,
        <b>false</b>,
        ctx
    );
}
</code></pre>



</details>

<a name="0x3_dwallet_emit_sign_events"></a>

## Function `emit_sign_events`

Emits events to initiate the signing process for each message.

This function ensures that all messages have the correct approvals, calculates
their hashes, and emits signing events.


<a name="@Effects_17"></a>

### Effects

- Checks that the number of <code>signature_algorithm_data</code> items matches <code>message_approvals</code>.
- Generates a new session ID for batch signing.
- Emits <code><a href="dwallet.md#0x3_dwallet_StartBatchedSignEvent">StartBatchedSignEvent</a></code> containing session details and hashed messages.
- Iterates through <code>signature_algorithm_data</code>, verifying approvals and emitting <code><a href="dwallet.md#0x3_dwallet_StartSignEvent">StartSignEvent</a></code> for each.


<a name="@Aborts_18"></a>

### Aborts

- **<code><a href="dwallet.md#0x3_dwallet_EExtraDataAndMessagesLenMismatch">EExtraDataAndMessagesLenMismatch</a></code>**: If <code>signature_algorithm_data</code> and <code>message_approvals</code> have different lengths.
- **<code><a href="dwallet.md#0x3_dwallet_EMissingApprovalOrWrongApprovalOrder">EMissingApprovalOrWrongApprovalOrder</a></code>**: If message approvals are incorrect or missing.


<pre><code><b>fun</b> <a href="dwallet.md#0x3_dwallet_emit_sign_events">emit_sign_events</a>&lt;D: <b>copy</b>, drop, store&gt;(message_approvals: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet.md#0x3_dwallet_MessageApproval">dwallet::MessageApproval</a>&gt;, dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, dwallet_decentralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, dwallet_mpc_network_decryption_key_version: u8, signature_algorithm_data: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;D&gt;, is_future_sign: bool, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="dwallet.md#0x3_dwallet_emit_sign_events">emit_sign_events</a>&lt;D: <b>copy</b> + drop + store&gt;(
    message_approvals: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet.md#0x3_dwallet_MessageApproval">MessageApproval</a>&gt;,
    dwallet_id: ID,
    dwallet_cap_id: ID,
    dwallet_decentralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    dwallet_mpc_network_decryption_key_version: u8,
    signature_algorithm_data: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;D&gt;,
    is_future_sign: bool,
    ctx: &<b>mut</b> TxContext
){
    <b>assert</b>!(<a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&signature_algorithm_data) == <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&message_approvals), <a href="dwallet.md#0x3_dwallet_EExtraDataAndMessagesLenMismatch">EExtraDataAndMessagesLenMismatch</a>);
    // Todo (#565): Move the <a href="../pera-framework/hash.md#0x2_hash">hash</a> calculation into the rust code.
    <b>let</b> hashed_messages = <a href="dwallet.md#0x3_dwallet_hash_messages">hash_messages</a>(&message_approvals);

    <b>let</b> batch_session_id = <a href="../pera-framework/object.md#0x2_object_id_from_address">object::id_from_address</a>(<a href="../pera-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx));
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet.md#0x3_dwallet_StartBatchedSignEvent">StartBatchedSignEvent</a> {
        session_id: batch_session_id,
        hashed_messages,
        initiator: <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx)
    });

    vector::zip_map!(signature_algorithm_data, message_approvals, |data, message_approval| {
        <b>let</b> <a href="dwallet.md#0x3_dwallet_MessageApproval">MessageApproval</a> {
            dwallet_cap_id:  message_approval_dwallet_cap_id,
            hash_scheme,
            message
        } = message_approval;
        <b>assert</b>!(dwallet_cap_id == message_approval_dwallet_cap_id, <a href="dwallet.md#0x3_dwallet_EMessageApprovalDWalletMismatch">EMessageApprovalDWalletMismatch</a>);
        // Todo (#565): Move the <a href="../pera-framework/hash.md#0x2_hash">hash</a> calculation into the rust code.
        <b>let</b> hashed_message = <a href="dwallet.md#0x3_dwallet_hash_message">hash_message</a>(message, hash_scheme);

        <b>let</b> id = <a href="../pera-framework/object.md#0x2_object_id_from_address">object::id_from_address</a>(<a href="../pera-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx));
        <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet.md#0x3_dwallet_StartSignEvent">StartSignEvent</a>&lt;D&gt; {
            session_id: id,
            initiator: <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx),
            batched_session_id: batch_session_id,
            dwallet_id,
            dwallet_decentralized_public_output,
            hashed_message,
            dwallet_mpc_network_decryption_key_version,
            signature_algorithm_data: data,
            is_future_sign,
        });
        <b>true</b>
    });
}
</code></pre>



</details>

<a name="0x3_dwallet_create_sign_output"></a>

## Function `create_sign_output`

Emits a <code><a href="dwallet.md#0x3_dwallet_CompletedSignEvent">CompletedSignEvent</a></code> with the MPC Sign protocol output.

This function is called by the blockchain itself and is part of the core
blockchain logic executed by validators. The emitted event contains the
completed sign output that should be consumed by the initiating user.


<a name="@Parameters_19"></a>

##### Parameters

- **<code>signed_messages</code>**: A vector containing the signed message outputs.
- **<code>batch_session_id</code>**: The unique identifier for the batch signing session.
- **<code>ctx</code>**: The transaction context used for event emission.


<a name="@Requirements_20"></a>

##### Requirements

- The caller **must be the system address** (<code>@0x0</code>). If this condition is not met,
the function will abort with <code><a href="dwallet.md#0x3_dwallet_ENotSystemAddress">ENotSystemAddress</a></code>.


<a name="@Events_21"></a>

##### Events

- **<code><a href="dwallet.md#0x3_dwallet_CompletedSignEvent">CompletedSignEvent</a></code>**: Emitted with the <code>session_id</code> and <code>signed_messages</code>,
signaling the completion of the sign process for the batch session.


<a name="@Errors_22"></a>

##### Errors

- **<code><a href="dwallet.md#0x3_dwallet_ENotSystemAddress">ENotSystemAddress</a></code>**: If the caller is not the system address (<code>@0x0</code>),
the function will abort with this error.


<pre><code><b>fun</b> <a href="dwallet.md#0x3_dwallet_create_sign_output">create_sign_output</a>(signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, batch_session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, initiator: <b>address</b>, dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, is_future_sign: bool, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="dwallet.md#0x3_dwallet_create_sign_output">create_sign_output</a>(
    signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    batch_session_id: ID,
    initiator: <b>address</b>,
    dwallet_id: ID,
    is_future_sign: bool,
    ctx: &<b>mut</b> TxContext
) {
    // Ensure that only the system <b>address</b> can call this function.
    <b>assert</b>!(<a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == <a href="dwallet.md#0x3_dwallet_SYSTEM_ADDRESS">SYSTEM_ADDRESS</a>, <a href="dwallet.md#0x3_dwallet_ENotSystemAddress">ENotSystemAddress</a>);

    <b>let</b> sign_output = <a href="dwallet.md#0x3_dwallet_BatchedSignOutput">BatchedSignOutput</a> {
        id: <a href="../pera-framework/object.md#0x2_object_new">object::new</a>(ctx),
        signatures: signatures,
        dwallet_id,
        session_id: batch_session_id,
    };

    // Emit the <a href="dwallet.md#0x3_dwallet_CompletedSignEvent">CompletedSignEvent</a> <b>with</b> session ID and signed messages.
    // This is only used <b>to</b> notify the user that the signing process is complete.
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet.md#0x3_dwallet_CompletedSignEvent">CompletedSignEvent</a> {
        session_id: batch_session_id,
        output_object_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(&sign_output),
        signatures,
        is_future_sign,
    });

    <a href="../pera-framework/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(sign_output, initiator);
}
</code></pre>



</details>

<a name="0x3_dwallet_request_future_sign"></a>

## Function `request_future_sign`

A function to publish messages signed by the user on chain with on-chain verification,
without launching the chain's sign flow immediately.

See the docs of [<code><a href="dwallet.md#0x3_dwallet_PartialCentralizedSignedMessages">PartialCentralizedSignedMessages</a></code>] for
more details on when this may be used.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet.md#0x3_dwallet_request_future_sign">request_future_sign</a>&lt;T: drop, D: <b>copy</b>, drop, store&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;T&gt;, messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, signature_algorithm_data: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet.md#0x3_dwallet_SignatureAlgorithmData">dwallet::SignatureAlgorithmData</a>&lt;D&gt;&gt;, _pera_system_state: &<a href="pera_system.md#0x3_pera_system_PeraSystemState">pera_system::PeraSystemState</a>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet.md#0x3_dwallet_request_future_sign">request_future_sign</a>&lt;T: drop, D: <b>copy</b> + drop + store&gt;(
    <a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a>&lt;T&gt;,
    messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    signature_algorithm_data: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet.md#0x3_dwallet_SignatureAlgorithmData">SignatureAlgorithmData</a>&lt;D&gt;&gt;,
    _pera_system_state: &PeraSystemState,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> messages_len = <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&messages);
    <b>let</b> signature_algorithm_data_len = <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&signature_algorithm_data);
    <b>assert</b>!(messages_len == signature_algorithm_data_len, <a href="dwallet.md#0x3_dwallet_EExtraDataAndMessagesLenMismatch">EExtraDataAndMessagesLenMismatch</a>);

    <b>let</b> signature_algorithm_data_unpacked = vector::map!(signature_algorithm_data, |<a href="dwallet.md#0x3_dwallet_SignatureAlgorithmData">SignatureAlgorithmData</a> { data }| data);
    <b>let</b> partial_signatures = <a href="dwallet.md#0x3_dwallet_create_partial_centralized_signed_messages">create_partial_centralized_signed_messages</a>&lt;T, D&gt;(
        messages,
        <a href="dwallet.md#0x3_dwallet">dwallet</a>,
        signature_algorithm_data_unpacked,
        ctx,
    );

    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet.md#0x3_dwallet_CreatedPartialCentralizedSignedMessagesEvent">CreatedPartialCentralizedSignedMessagesEvent</a> {
        partial_signatures_object_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(&partial_signatures),
    });

    // Todo (#415): Add the <a href="../pera-framework/event.md#0x2_event">event</a> for the verify_partially_signed_signatures
    // Todo (#415): <a href="dwallet.md#0x3_dwallet_PartialCentralizedSignedMessages">PartialCentralizedSignedMessages</a> will be created & retured <b>to</b> the user only after the verification is done.
    <a href="../pera-framework/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(partial_signatures, <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx));
}
</code></pre>



</details>

<a name="0x3_dwallet_create_signature_algorithm_data"></a>

## Function `create_signature_algorithm_data`

A function to create a [<code><a href="dwallet.md#0x3_dwallet_SignatureAlgorithmData">SignatureAlgorithmData</a></code>] object.
Extra fields are used to store additional data with the object, specific to every protocol implementation.
D: The type of the extra fields that can be stored with the object.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet.md#0x3_dwallet_create_signature_algorithm_data">create_signature_algorithm_data</a>&lt;D: <b>copy</b>, drop, store&gt;(data: D): <a href="dwallet.md#0x3_dwallet_SignatureAlgorithmData">dwallet::SignatureAlgorithmData</a>&lt;D&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet.md#0x3_dwallet_create_signature_algorithm_data">create_signature_algorithm_data</a>&lt;D: store + <b>copy</b> + drop&gt;(data: D): <a href="dwallet.md#0x3_dwallet_SignatureAlgorithmData">SignatureAlgorithmData</a>&lt;D&gt; {
    <a href="dwallet.md#0x3_dwallet_SignatureAlgorithmData">SignatureAlgorithmData</a> { data }
}
</code></pre>



</details>

<a name="0x3_dwallet_sign_with_partial_centralized_message_signatures"></a>

## Function `sign_with_partial_centralized_message_signatures`

Initiates a signing flow using a previously published [<code><a href="dwallet.md#0x3_dwallet_PartialCentralizedSignedMessages">PartialCentralizedSignedMessages</a></code>].

This function takes a partial signature object, validates approvals for each message,
and emits the necessary signing events.


<a name="@Type_Parameters_23"></a>

#### Type Parameters

- <code>D</code>: Represents additional data fields specific for each implementation.


<a name="@Parameters_24"></a>

#### Parameters

- <code>partial_signature</code>: A previously published <code><a href="dwallet.md#0x3_dwallet_PartialCentralizedSignedMessages">PartialCentralizedSignedMessages</a>&lt;D&gt;</code> object
containing messages that require approval.
- <code>message_approvals</code>: A list of approvals corresponding to the messages in <code>partial_signature</code>.
- <code>ctx</code>: The transaction context.

<a name="@Notes_25"></a>

#### Notes

- See [<code><a href="dwallet.md#0x3_dwallet_PartialCentralizedSignedMessages">PartialCentralizedSignedMessages</a></code>] documentation for more details on usage scenarios.
- The function ensures that messages and approvals have a one-to-one correspondence before proceeding.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet.md#0x3_dwallet_sign_with_partial_centralized_message_signatures">sign_with_partial_centralized_message_signatures</a>&lt;D: <b>copy</b>, drop, store&gt;(partial_signature: <a href="dwallet.md#0x3_dwallet_PartialCentralizedSignedMessages">dwallet::PartialCentralizedSignedMessages</a>&lt;D&gt;, message_approvals: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet.md#0x3_dwallet_MessageApproval">dwallet::MessageApproval</a>&gt;, _pera_system_state: &<a href="pera_system.md#0x3_pera_system_PeraSystemState">pera_system::PeraSystemState</a>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet.md#0x3_dwallet_sign_with_partial_centralized_message_signatures">sign_with_partial_centralized_message_signatures</a>&lt;D: store + <b>copy</b> + drop&gt;(
    partial_signature: <a href="dwallet.md#0x3_dwallet_PartialCentralizedSignedMessages">PartialCentralizedSignedMessages</a>&lt;D&gt;,
    message_approvals: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet.md#0x3_dwallet_MessageApproval">MessageApproval</a>&gt;,
    _pera_system_state: &PeraSystemState,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> <a href="dwallet.md#0x3_dwallet_PartialCentralizedSignedMessages">PartialCentralizedSignedMessages</a>&lt;D&gt; {
        id,
        messages,
        dwallet_id,
        dwallet_decentralized_public_output,
        dwallet_cap_id,
        dwallet_mpc_network_decryption_key_version,
        signature_algorithm_data,
    } = partial_signature;
    <a href="../pera-framework/object.md#0x2_object_delete">object::delete</a>(id);

    // Ensure that each message <b>has</b> a corresponding approval; otherwise, <b>abort</b>.
    <a href="dwallet.md#0x3_dwallet_compare_messages_with_approvals">compare_messages_with_approvals</a>(&messages, &message_approvals);

    // Emit signing events <b>to</b> finalize the signing process.
    <a href="dwallet.md#0x3_dwallet_emit_sign_events">emit_sign_events</a>&lt;D&gt;(
        message_approvals,
        dwallet_id,
        dwallet_cap_id,
        dwallet_decentralized_public_output,
        dwallet_mpc_network_decryption_key_version,
        signature_algorithm_data,
        <b>true</b>,
        ctx
    );
}
</code></pre>



</details>

<a name="0x3_dwallet_compare_messages_with_approvals"></a>

## Function `compare_messages_with_approvals`

Compares the messages with the approvals to ensure they match.
This function can be called by the user to verify that the messages and approvals match,
before calling the <code>sign_with_partial_centralized_message_signatures</code> function.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet.md#0x3_dwallet_compare_messages_with_approvals">compare_messages_with_approvals</a>(messages: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, approvals: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet.md#0x3_dwallet_MessageApproval">dwallet::MessageApproval</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet.md#0x3_dwallet_compare_messages_with_approvals">compare_messages_with_approvals</a>(
    messages: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    approvals: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet.md#0x3_dwallet_MessageApproval">MessageApproval</a>&gt;
) {
    <b>let</b> compare_vector = vector::zip_map_ref!(messages, approvals, |message, approval| {
        <b>if</b> (message == approval.message) {<b>true</b>} <b>else</b> {<b>false</b>}
    });
    <b>assert</b>!(vector::all!(&compare_vector, |v| { v == <b>true</b> }), <a href="dwallet.md#0x3_dwallet_EMissingApprovalOrWrongApprovalOrder">EMissingApprovalOrWrongApprovalOrder</a>);
}
</code></pre>



</details>
