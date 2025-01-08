---
title: Module `0x3::dwallet_2pc_mpc_ecdsa_k1`
---

This module handles the logic for creating and managing dWallets using the Secp256K1 signature scheme
and the DKG process. It leverages validators to execute MPC (Multi-Party Computation)
protocols to ensure trustless and decentralized wallet creation and key management.


<a name="@Overview_0"></a>

### Overview


- **Secp256K1**: The cryptographic curve used for this implementation.
- dWallets are created through two phases of DKG:
1. The first phase outputs partial results for the user.
2. The second phase generates the dWallet.
- **Capabilities**: Access to dWallets is controlled via capabilities (<code>DWalletCap</code>).


<a name="@Features_1"></a>

### Features


- Emit events for validators to coordinate DKG rounds.
- Transfer intermediate results and final outputs to the initiating user.
- Ensure secure and decentralized key generation and management.


    -  [Overview](#@Overview_0)
    -  [Features](#@Features_1)
-  [Struct `Secp256K1`](#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1)
-  [Struct `MessageApproval`](#0x3_dwallet_2pc_mpc_ecdsa_k1_MessageApproval)
-  [Struct `DKGFirstRoundOutputEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_DKGFirstRoundOutputEvent)
-  [Resource `DKGFirstRoundOutput`](#0x3_dwallet_2pc_mpc_ecdsa_k1_DKGFirstRoundOutput)
-  [Resource `BatchedSignOutput`](#0x3_dwallet_2pc_mpc_ecdsa_k1_BatchedSignOutput)
-  [Resource `Presign`](#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign)
-  [Struct `StartEncryptedShareVerificationEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_StartEncryptedShareVerificationEvent)
-  [Struct `CreatedEncryptedSecretShareEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_CreatedEncryptedSecretShareEvent)
-  [Resource `PartiallySignedMessages`](#0x3_dwallet_2pc_mpc_ecdsa_k1_PartiallySignedMessages)
-  [Struct `CreatedPartiallySignedMessagesEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_CreatedPartiallySignedMessagesEvent)
-  [Struct `StartDKGFirstRoundEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGFirstRoundEvent)
-  [Struct `CompletedSignEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedSignEvent)
-  [Resource `EncryptedUserShare`](#0x3_dwallet_2pc_mpc_ecdsa_k1_EncryptedUserShare)
-  [Struct `StartDKGSecondRoundEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGSecondRoundEvent)
-  [Struct `CompletedDKGSecondRoundEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedDKGSecondRoundEvent)
-  [Struct `StartPresignFirstRoundEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_StartPresignFirstRoundEvent)
-  [Struct `StartPresignSecondRoundEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_StartPresignSecondRoundEvent)
-  [Struct `CompletedBatchedPresignEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedBatchedPresignEvent)
-  [Struct `StartSignEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_StartSignEvent)
-  [Struct `StartBatchedSignEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_StartBatchedSignEvent)
-  [Struct `StartBatchedPresignEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_StartBatchedPresignEvent)
-  [Constants](#@Constants_13)
-  [Function `launch_dkg_first_round`](#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_dkg_first_round)
-  [Function `create_dkg_first_round_output`](#0x3_dwallet_2pc_mpc_ecdsa_k1_create_dkg_first_round_output)
-  [Function `launch_dkg_second_round`](#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_dkg_second_round)
-  [Function `publish_encrypted_user_share`](#0x3_dwallet_2pc_mpc_ecdsa_k1_publish_encrypted_user_share)
-  [Function `create_encrypted_user_share`](#0x3_dwallet_2pc_mpc_ecdsa_k1_create_encrypted_user_share)
-  [Function `create_dkg_second_round_output`](#0x3_dwallet_2pc_mpc_ecdsa_k1_create_dkg_second_round_output)
-  [Function `launch_batched_presign`](#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_batched_presign)
-  [Function `launch_presign_second_round`](#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_presign_second_round)
-  [Function `create_batched_presign_output`](#0x3_dwallet_2pc_mpc_ecdsa_k1_create_batched_presign_output)
-  [Function `approve_messages`](#0x3_dwallet_2pc_mpc_ecdsa_k1_approve_messages)
-  [Function `remove_message_approval`](#0x3_dwallet_2pc_mpc_ecdsa_k1_remove_message_approval)
-  [Function `sign`](#0x3_dwallet_2pc_mpc_ecdsa_k1_sign)
-  [Function `create_sign_output`](#0x3_dwallet_2pc_mpc_ecdsa_k1_create_sign_output)
-  [Function `create_mock_dwallet_for_testing`](#0x3_dwallet_2pc_mpc_ecdsa_k1_create_mock_dwallet_for_testing)
-  [Function `create_mock_dwallet`](#0x3_dwallet_2pc_mpc_ecdsa_k1_create_mock_dwallet)
-  [Function `create_mock_presign`](#0x3_dwallet_2pc_mpc_ecdsa_k1_create_mock_presign)
-  [Function `publish_partially_signed_messages`](#0x3_dwallet_2pc_mpc_ecdsa_k1_publish_partially_signed_messages)
-  [Function `future_sign`](#0x3_dwallet_2pc_mpc_ecdsa_k1_future_sign)
-  [Function `pop_and_verify_message_approval`](#0x3_dwallet_2pc_mpc_ecdsa_k1_pop_and_verify_message_approval)
-  [Function `verify_partially_signed_signatures`](#0x3_dwallet_2pc_mpc_ecdsa_k1_verify_partially_signed_signatures)


<pre><code><b>use</b> <a href="../move-stdlib/vector.md#0x1_vector">0x1::vector</a>;
<b>use</b> <a href="../pera-framework/event.md#0x2_event">0x2::event</a>;
<b>use</b> <a href="../pera-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../pera-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../pera-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="dwallet.md#0x3_dwallet">0x3::dwallet</a>;
</code></pre>



<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1"></a>

## Struct `Secp256K1`

Represents the <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a></code> dWallet type.

This struct serves as a marker to identify and signify
the dWallet cryptographic scheme used for ECDSA
(Elliptic Curve Digital Signature Algorithm)
based on the <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a></code> curve.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a> <b>has</b> drop
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

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_MessageApproval"></a>

## Struct `MessageApproval`

Represents a message that was approved as part of a dWallet process.

This struct binds the message to a specific <code>DWalletCap</code> for
traceability and accountability within the system.


<a name="@Fields_2"></a>

##### Fields

- **<code>dwallet_cap_id</code>**: The identifier of the DWallet capability
associated with this approval.
- **<code>message</code>**: The message that has been approved.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_MessageApproval">MessageApproval</a> <b>has</b> drop, store
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
<code>message: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_DKGFirstRoundOutputEvent"></a>

## Struct `DKGFirstRoundOutputEvent`

An event being emitted when the DKG first round is completed by the blockchain.
The user should catch this event to get the output of the first round,
use it to generate the needed input for the second round, and then call the
[<code>launch_dkg_second_round</code>] function to start the second round.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_DKGFirstRoundOutputEvent">DKGFirstRoundOutputEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>output_object_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_DKGFirstRoundOutput"></a>

## Resource `DKGFirstRoundOutput`

The output of the dWallet creation DKG first round.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_DKGFirstRoundOutput">DKGFirstRoundOutput</a> <b>has</b> store, key
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
<code>output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_BatchedSignOutput"></a>

## Resource `BatchedSignOutput`

The output of a batched Sign session.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_BatchedSignOutput">BatchedSignOutput</a> <b>has</b> store, key
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
<code>signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_Presign"></a>

## Resource `Presign`

Represents the result of the second and final
presign round.

This struct holds information on both rounds of the presign process,
linking them to the corresponding DWallet session.


<a name="@Fields_3"></a>

##### Fields

- **<code>id</code>**: Unique identifier for this presign object.
- **<code>session_id</code>**: The session ID of this presign process.
- **<code>dwallet_id</code>**: The DWallet identifier associated with the presign.
- **<code>dwallet_cap_id</code>**: The DWallet capability identifier for this presign.
- **<code>first_round_session_id</code>**: The session ID for the first round of the presign.
- **<code>first_round_output</code>**: The output from the first round of the presign.
- **<code>second_round_output</code>**: The output from the second round of the presign.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a> <b>has</b> store, key
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
<code>dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>first_round_session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>presign: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_StartEncryptedShareVerificationEvent"></a>

## Struct `StartEncryptedShareVerificationEvent`

An event emitted to start an encrypted share verification process.
Since we cannot use native functions if we depend on Sui to hold our state,
we need to emit an event to start the verification process, like we start the other MPC processes.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartEncryptedShareVerificationEvent">StartEncryptedShareVerificationEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>encrypted_secret_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>dwallet_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The DKG decentralized output of the dwallet that its secret is being encrypted.
</dd>
<dt>
<code>dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>encryption_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The encryption key used to encrypt the secret share to.
</dd>
<dt>
<code>encryption_key_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The encryption key Move object ID.
</dd>
<dt>
<code>session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>signed_public_share: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The signed public share of the dwallet that its secret is being encrypted.
</dd>
<dt>
<code>encryptor_ed25519_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The public key of the encryptor. Used to verify the signature on the public share.
</dd>
<dt>
<code>initiator: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_CreatedEncryptedSecretShareEvent"></a>

## Struct `CreatedEncryptedSecretShareEvent`

An event emitted when an encrypted share is created by the system transaction.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_CreatedEncryptedSecretShareEvent">CreatedEncryptedSecretShareEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>encrypted_share_obj_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>encrypted_secret_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>encryption_key_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>encryptor_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>encryptor_ed25519_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>signed_public_share: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_PartiallySignedMessages"></a>

## Resource `PartiallySignedMessages`

Messages that has been signed by a user, a.k.a the centralized party, but not yet by the blockchain.
Used for scenarios where the user need to first agree to sign some transaction, and the blockchain signs this transaction only later,
when some other conditions are met.

Can be used to implement an order-book based exchange, for example.
User A first agrees to buy BTC with ETH at price X, and signs a transaction with this information.
When a matching user B, that agrees to sell BTC for ETH at price X, signs a transaction with this information,
the blockchain can sign both transactions, and the exchange is completed.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_PartiallySignedMessages">PartiallySignedMessages</a> <b>has</b> key
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
 The hashed messages that are being signed.
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

</dd>
<dt>
<code>dwallet_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The DKG output of the DWallet.
</dd>
<dt>
<code>dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
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

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_CreatedPartiallySignedMessagesEvent"></a>

## Struct `CreatedPartiallySignedMessagesEvent`

Event emitted when a [<code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_PartiallySignedMessages">PartiallySignedMessages</a></code>] object is created.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_CreatedPartiallySignedMessagesEvent">CreatedPartiallySignedMessagesEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>partial_signatures_object_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The object's ID.
</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGFirstRoundEvent"></a>

## Struct `StartDKGFirstRoundEvent`

Event emitted to start the first DKG round.

This event is caught by Validators, who use it to initiate the first round of the DKG.


<a name="@Fields_4"></a>

##### Fields

- **<code>session_id</code>**: The unique session identifier for the DKG process.
- **<code>initiator</code>**: The address of the user who initiated the DKG process.
- **<code>dwallet_cap_id</code>**: The identifier for the DWallet capability.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGFirstRoundEvent">StartDKGFirstRoundEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>session_id: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>initiator: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedSignEvent"></a>

## Struct `CompletedSignEvent`

Event emitted to signal the completion of a Sign process.

This event contains signatures for all signed messages in the batch.


<a name="@Fields_5"></a>

##### Fields

- **<code>session_id</code>**: The session identifier for the signing process.
- **<code>signed_messages</code>**: A collection of signed messages.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedSignEvent">CompletedSignEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>signed_messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_EncryptedUserShare"></a>

## Resource `EncryptedUserShare`

A verified encrypted user share.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EncryptedUserShare">EncryptedUserShare</a> <b>has</b> key
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
<code>dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The id of the DWallet that its secret share is encrypted.
</dd>
<dt>
<code>encrypted_secret_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The encrypted secret share and a proof that the encryption is actually the encryption of the <code>dwallet_id</code>'s secret share.
</dd>
<dt>
<code>encryption_key_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The encryption key used to encrypt the secret share to.
</dd>
<dt>
<code>signed_public_share: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>encryptor_ed25519_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>encryptor_address: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGSecondRoundEvent"></a>

## Struct `StartDKGSecondRoundEvent`

Event emitted to start the second round of the DKG process.

This event is caught by Validators to start the second round of DKG.


<a name="@Fields_6"></a>

##### Fields

- **<code>session_id</code>**: The session identifier.
- **<code>initiator</code>**: The address of the user who initiated the event.
- **<code>first_round_output</code>**: The output from the first round of the DKG.
- **<code>public_key_share_and_proof</code>**: The public key share and its proof.
- **<code>dwallet_cap_id</code>**: The DWallet capability identifier.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGSecondRoundEvent">StartDKGSecondRoundEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>session_id: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>initiator: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>first_round_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>public_key_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>first_round_session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedDKGSecondRoundEvent"></a>

## Struct `CompletedDKGSecondRoundEvent`

Event emitted when the second round of the
Distributed Key Generation (DKG) is completed.

This event contains all relevant data produced from the
second round of the DKG process.
Validators and users utilize this event to
finalize and store the results of the DKG.


<a name="@Fields_7"></a>

##### Fields

- **<code>session_id</code>**: The unique identifier for the DKG session.
- **<code>initiator</code>**: The address of the user who initiated the DKG process.
- **<code>dwallet_cap_id</code>**: The identifier of the DWallet capability used in the DKG process.
- **<code>dwallet_id</code>**: The identifier of the DWallet created as a result of the DKG process.
- **<code>value</code>**: The value produced by the second round of the DKG, typically representing
the combined and validated output from all participating parties.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedDKGSecondRoundEvent">CompletedDKGSecondRoundEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>initiator: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>value: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_StartPresignFirstRoundEvent"></a>

## Struct `StartPresignFirstRoundEvent`

Event emitted to initiate the first round of a Presign session.


<a name="@Fields_8"></a>

##### Fields

- **<code>session_id</code>**: The session identifier.
- **<code>initiator</code>**: The address of the user who initiated the event.
- **<code>dwallet_id</code>**: The DWallet identifier.
- **<code>dwallet_cap_id</code>**: The DWallet capability identifier.
- **<code>dkg_output</code>**: The output from the DKG process.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartPresignFirstRoundEvent">StartPresignFirstRoundEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>initiator: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>dkg_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>batch_session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
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

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_StartPresignSecondRoundEvent"></a>

## Struct `StartPresignSecondRoundEvent`

Event emitted to initiate the second round of a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a></code> session.

This event is caught by Validators to initiate the second round of the Presign process.
The second round is a crucial step in the multi-party computation (MPC) protocol
to generate pre-signatures for ECDSA signing.


<a name="@Fields_9"></a>

##### Fields

- **<code>session_id</code>**: The unique identifier for the current presign session.
- **<code>initiator</code>**: The address of the user who initiated the presign session.
- **<code>dwallet_id</code>**: The identifier of the DWallet associated with this presign session.
- **<code>dwallet_cap_id</code>**: The identifier of the DWallet capability used in this session.
- **<code>dkg_output</code>**: The output produced from the Distributed Key Generation (DKG) process.
- **<code>first_round_output</code>**: The output of the first round of the presign session.
- **<code>first_round_session_id</code>**: The session identifier for the first round of the presign.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartPresignSecondRoundEvent">StartPresignSecondRoundEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>initiator: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>dkg_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>first_round_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>first_round_session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>batch_session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
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

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedBatchedPresignEvent"></a>

## Struct `CompletedBatchedPresignEvent`

Event emitted when the presign batch is completed.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedBatchedPresignEvent">CompletedBatchedPresignEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>initiator: <b>address</b></code>
</dt>
<dd>
 The address of the user who initiated the batch.
</dd>
<dt>
<code>dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 Tha batch session ID.
</dd>
<dt>
<code>presign_ids: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>&gt;</code>
</dt>
<dd>
 The ID of all the presign objects created in this batch.
 Each presign can be used to sign only one message.
</dd>
<dt>
<code>first_round_session_ids: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>&gt;</code>
</dt>
<dd>
 The first round session IDs for each presign.
 The order of the session IDs corresponds to the order of the presigns.
 The first round session ID is needed for the centralized sign process.
</dd>
<dt>
<code>presigns: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>
 The serialized presign objects created in this batch.
 The order of the presigns corresponds to the order of the presign IDs.
</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_StartSignEvent"></a>

## Struct `StartSignEvent`

Event emitted to start the signing process.
The event is caught by the validators to initiate the signing protocol.


<a name="@Fields_10"></a>

##### Fields

- **<code>session_id</code>**: The unique identifier for this sign session.
- **<code>presign_session_id</code>**: The unique identifier for the associated presign session.
- **<code>initiator</code>**: The address of the user who initiated the sign event.
- **<code>batched_session_id</code>**: The session identifier for the batched sign process.
- **<code>dwallet_id</code>**: The unique identifier for the DWallet used in the session.
- **<code>dwallet_cap_id</code>**: The identifier for the DWallet's capability.
- **<code>dkg_output</code>**: The output of the DKG process used for the session.
- **<code>hashed_message</code>**: The hashed message that will be signed during this session.
- **<code>presign_first_round_output</code>**: The output from the first round of the presign process.
- **<code>presign_second_round_output</code>**: The output from the second round of the presign process.
- **<code>centralized_signed_message</code>**: The final signed message produced by the centralized sign process.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartSignEvent">StartSignEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>presign_session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>initiator: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>batched_session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>dkg_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>hashed_message: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>presign: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>centralized_signed_message: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
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

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_StartBatchedSignEvent"></a>

## Struct `StartBatchedSignEvent`

Event emitted to start a batched sign process.


<a name="@Fields_11"></a>

##### Fields

- **<code>session_id</code>**: The session identifier for the batched sign process.
- **<code>hashed_messages</code>**: A list of hashed messages to be signed.
- **<code>initiator</code>**: The address of the user who initiated the protocol.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartBatchedSignEvent">StartBatchedSignEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>hashed_messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>initiator: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_StartBatchedPresignEvent"></a>

## Struct `StartBatchedPresignEvent`

Event emitted to start a batched presign flow, i.e. a flow that creates multiple presigns at once.


<a name="@Fields_12"></a>

##### Fields

- **<code>session_id</code>**: The session identifier for the batched sign process.
- **<code>batch_size</code>**: The number of presign sessions to be started.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartBatchedPresignEvent">StartBatchedPresignEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>batch_size: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>initiator: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_13"></a>

## Constants


<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress"></a>

Error raised when the sender is not the system address.


<pre><code><b>const</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
</code></pre>



<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_SYSTEM_ADDRESS"></a>

System address for asserting system-level actions.


<pre><code><b>const</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_SYSTEM_ADDRESS">SYSTEM_ADDRESS</a>: <b>address</b> = 0;
</code></pre>



<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_EApprovalsAndMessagesLenMismatch"></a>



<pre><code><b>const</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EApprovalsAndMessagesLenMismatch">EApprovalsAndMessagesLenMismatch</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 3;
</code></pre>



<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_EApprovalsAndSignaturesLenMismatch"></a>



<pre><code><b>const</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EApprovalsAndSignaturesLenMismatch">EApprovalsAndSignaturesLenMismatch</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 8;
</code></pre>



<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_ECentralizedSignedMessagesAndMessagesLenMismatch"></a>



<pre><code><b>const</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ECentralizedSignedMessagesAndMessagesLenMismatch">ECentralizedSignedMessagesAndMessagesLenMismatch</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 5;
</code></pre>



<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_EDwalletMismatch"></a>



<pre><code><b>const</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EDwalletMismatch">EDwalletMismatch</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 2;
</code></pre>



<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_EInvalidSignatures"></a>



<pre><code><b>const</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EInvalidSignatures">EInvalidSignatures</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 7;
</code></pre>



<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_EMessageApprovalDWalletMismatch"></a>



<pre><code><b>const</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EMessageApprovalDWalletMismatch">EMessageApprovalDWalletMismatch</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1;
</code></pre>



<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_EMissingApprovalOrWrongApprovalOrder"></a>



<pre><code><b>const</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EMissingApprovalOrWrongApprovalOrder">EMissingApprovalOrWrongApprovalOrder</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 4;
</code></pre>



<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_EPresignsAndMessagesLenMismatch"></a>



<pre><code><b>const</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EPresignsAndMessagesLenMismatch">EPresignsAndMessagesLenMismatch</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 6;
</code></pre>



<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_launch_dkg_first_round"></a>

## Function `launch_dkg_first_round`

Starts the first Distributed Key Generation (DKG) session.

This function creates a new <code>DWalletCap</code> object,
transfers it to the session initiator,
and emits a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGFirstRoundEvent">StartDKGFirstRoundEvent</a></code> to signal
the beginning of the DKG process.


<a name="@Effects_14"></a>

##### Effects

- Generates a new <code>DWalletCap</code> object.
- Transfers the <code>DWalletCap</code> to the session initiator (<code>ctx.sender</code>).
- Emits a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGFirstRoundEvent">StartDKGFirstRoundEvent</a></code>.


<a name="@Emits_15"></a>

##### Emits

- <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGFirstRoundEvent">StartDKGFirstRoundEvent</a></code>:
- <code>session_id</code>: The generated session ID.
- <code>initiator</code>: The address of the transaction sender.
- <code>dwallet_cap_id</code>: The ID of the created <code>DWalletCap</code>.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_dkg_first_round">launch_dkg_first_round</a>(ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_dkg_first_round">launch_dkg_first_round</a>(ctx: &<b>mut</b> TxContext) {
    <b>let</b> dwallet_cap = create_dwallet_cap(ctx);
    <b>let</b> dwallet_cap_id = <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(&dwallet_cap);
    <a href="../pera-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(dwallet_cap, <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx));
    <b>let</b> initiator = <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx);
    <b>let</b> session_id = <a href="../pera-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx);
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGFirstRoundEvent">StartDKGFirstRoundEvent</a> {
        session_id,
        initiator,
        dwallet_cap_id,
    });
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_create_dkg_first_round_output"></a>

## Function `create_dkg_first_round_output`

Creates the output of the first DKG round.

This function transfers the output of the first DKG round
to the session initiator and ensures it is securely linked
to the <code>DWalletCap</code> of the session.
This function is called by blockchain itself.
Validators call it, it's part of the blockchain logic.


<a name="@Effects_16"></a>

##### Effects

- Transfers the output of the first round to the initiator.
- Emits necessary metadata and links it to the associated session.


<a name="@Parameters_17"></a>

##### Parameters

- <code>initiator</code>: The address of the user who initiated the DKG session.
- <code>session_id</code>: The ID of the DKG session.
- <code>output</code>: The output data from the first round.
- <code>dwallet_cap_id</code>: The ID of the associated <code>DWalletCap</code>.
- <code>ctx</code>: The transaction context.


<a name="@Panics_18"></a>

##### Panics

- Panics with <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a></code> if the sender is not the system address.
Creates the output of the first round of the DKG MPC, transferring it to the initiating user.
This function is called by the blockchain itself.
Validators call it as part of the blockchain logic.


<pre><code><b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_dkg_first_round_output">create_dkg_first_round_output</a>(session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, initiator: <b>address</b>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_dkg_first_round_output">create_dkg_first_round_output</a>(
    session_id: ID,
    output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    initiator: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(<a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_SYSTEM_ADDRESS">SYSTEM_ADDRESS</a>, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a>);
    <b>let</b> dkg_output = <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_DKGFirstRoundOutput">DKGFirstRoundOutput</a> {
        session_id,
        id: <a href="../pera-framework/object.md#0x2_object_new">object::new</a>(ctx),
        output,
    };
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_DKGFirstRoundOutputEvent">DKGFirstRoundOutputEvent</a> {
        session_id,
        output_object_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(&dkg_output),
        output,
    });
    <a href="../pera-framework/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(dkg_output, initiator);
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_launch_dkg_second_round"></a>

## Function `launch_dkg_second_round`

Starts the second DKG round.
Emits an event for validators to begin the second round of the DKG process.


<a name="@Parameters_19"></a>

##### Parameters

- <code>dwallet_cap</code>: The capability for the associated dWallet.
- <code>public_key_share_and_proof</code>: Public key share and proof from the first round.
- <code>first_round_output</code>: Output from the first DKG round.
- <code>first_round_session_id</code>: Session ID of the first DKG round.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_dkg_second_round">launch_dkg_second_round</a>(dwallet_cap: &<a href="dwallet.md#0x3_dwallet_DWalletCap">dwallet::DWalletCap</a>, public_key_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, first_round_output: &<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_DKGFirstRoundOutput">dwallet_2pc_mpc_ecdsa_k1::DKGFirstRoundOutput</a>, first_round_session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_dkg_second_round">launch_dkg_second_round</a>(
    dwallet_cap: &DWalletCap,
    public_key_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    first_round_output: &<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_DKGFirstRoundOutput">DKGFirstRoundOutput</a>,
    first_round_session_id: ID,
    ctx: &<b>mut</b> TxContext
): <b>address</b> {
    <b>let</b> session_id = <a href="../pera-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx);
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGSecondRoundEvent">StartDKGSecondRoundEvent</a> {
        session_id,
        initiator: <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx),
        first_round_output: first_round_output.output,
        public_key_share_and_proof,
        dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(dwallet_cap),
        first_round_session_id,
    });
    session_id
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_publish_encrypted_user_share"></a>

## Function `publish_encrypted_user_share`

Submits the given secret share encryption to the chain.
The chain verifies that the encryption is actually the encryption of the secret share before creating the [<code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EncryptedUserShare">EncryptedUserShare</a></code>] object.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_publish_encrypted_user_share">publish_encrypted_user_share</a>(<a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">dwallet_2pc_mpc_ecdsa_k1::Secp256K1</a>&gt;, encryption_key: &<a href="dwallet.md#0x3_dwallet_EncryptionKey">dwallet::EncryptionKey</a>, encrypted_secret_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, signed_public_share: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, encryptor_ed25519_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_publish_encrypted_user_share">publish_encrypted_user_share</a>(
    <a href="dwallet.md#0x3_dwallet">dwallet</a>: &DWallet&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;,
    encryption_key: &EncryptionKey,
    encrypted_secret_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    signed_public_share: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    encryptor_ed25519_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext,
){
    <b>let</b> session_id = <a href="../pera-framework/object.md#0x2_object_id_from_address">object::id_from_address</a>(<a href="../pera-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx));
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartEncryptedShareVerificationEvent">StartEncryptedShareVerificationEvent</a> {
        encrypted_secret_share_and_proof,
        dwallet_output: get_dwallet_output&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>),
        dwallet_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(<a href="dwallet.md#0x3_dwallet">dwallet</a>),
        encryption_key: get_encryption_key(encryption_key),
        encryption_key_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(encryption_key),
        session_id,
        signed_public_share,
        encryptor_ed25519_pubkey,
        initiator: <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx),
    });
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_create_encrypted_user_share"></a>

## Function `create_encrypted_user_share`

This function is called by the blockchain itself to create the encrypted user share after it has been verified.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_encrypted_user_share">create_encrypted_user_share</a>(dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, encrypted_secret_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, encryption_key_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, signed_public_share: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, encryptor_ed25519_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, initiator: <b>address</b>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_encrypted_user_share">create_encrypted_user_share</a>(
    dwallet_id: ID,
    encrypted_secret_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    encryption_key_id: ID,
    session_id: ID,
    signed_public_share: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    encryptor_ed25519_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    initiator: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(<a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_SYSTEM_ADDRESS">SYSTEM_ADDRESS</a>, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a>);
    <b>let</b> encrypted_user_share = <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EncryptedUserShare">EncryptedUserShare</a> {
        id: <a href="../pera-framework/object.md#0x2_object_new">object::new</a>(ctx),
        dwallet_id,
        encrypted_secret_share_and_proof,
        encryption_key_id,
        signed_public_share,
        encryptor_ed25519_pubkey,
        encryptor_address: initiator,
    };
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_CreatedEncryptedSecretShareEvent">CreatedEncryptedSecretShareEvent</a> {
        session_id,
        encrypted_share_obj_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(&encrypted_user_share),
        dwallet_id,
        encrypted_secret_share_and_proof,
        encryption_key_id,
        signed_public_share,
        encryptor_ed25519_pubkey,
        encryptor_address: initiator,
    });
    <a href="../pera-framework/transfer.md#0x2_transfer_freeze_object">transfer::freeze_object</a>(encrypted_user_share);
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_create_dkg_second_round_output"></a>

## Function `create_dkg_second_round_output`

Completes the second DKG round and creates the final [<code>DWallet</code>].
This function finalizes the DKG process and emits an event with all relevant data.
This function is called by blockchain itself.
Validators call it, it's part of the blockchain logic.


<a name="@Parameters_20"></a>

##### Parameters

- <code>initiator</code>: The address of the user who initiated the DKG session.
- <code>session_id</code>: The ID of the current DKG session.
- <code>output</code>: The decentralized output of the second DKG round.
- <code>dwallet_cap_id</code>: The ID of the associated dWallet capability.
- <code>ctx</code>: The transaction context.


<pre><code><b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_dkg_second_round_output">create_dkg_second_round_output</a>(initiator: <b>address</b>, session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, dwallet_mpc_network_key_version: u8, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_dkg_second_round_output">create_dkg_second_round_output</a>(
    initiator: <b>address</b>,
    session_id: ID,
    output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    dwallet_cap_id: ID,
    dwallet_mpc_network_key_version: u8,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(<a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_SYSTEM_ADDRESS">SYSTEM_ADDRESS</a>, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a>);
    <b>let</b> <a href="dwallet.md#0x3_dwallet">dwallet</a> = <a href="dwallet.md#0x3_dwallet_create_dwallet">dwallet::create_dwallet</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;(
        session_id,
        dwallet_cap_id,
        output,
        dwallet_mpc_network_key_version,
        ctx
    );
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedDKGSecondRoundEvent">CompletedDKGSecondRoundEvent</a> {
        session_id,
        initiator,
        dwallet_cap_id,
        dwallet_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(&<a href="dwallet.md#0x3_dwallet">dwallet</a>),
        value: output,
    });
    <a href="../pera-framework/transfer.md#0x2_transfer_public_freeze_object">transfer::public_freeze_object</a>(<a href="dwallet.md#0x3_dwallet">dwallet</a>);
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_launch_batched_presign"></a>

## Function `launch_batched_presign`

Starts a batched presign session.

This function emits a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartBatchedPresignEvent">StartBatchedPresignEvent</a></code> for the entire batch and a
<code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartPresignFirstRoundEvent">StartPresignFirstRoundEvent</a></code> for each presign in the batch. These events signal
validators to begin processing the first round of the presign process for each session.
- A unique <code>batch_session_id</code> is generated for the batch.
- A loop creates and emits a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartPresignFirstRoundEvent">StartPresignFirstRoundEvent</a></code> for each session in the batch.
- Each session is linked to the parent batch via <code>batch_session_id</code>.


<a name="@Effects_21"></a>

##### Effects

- Associates the batched presign session with the specified dWallet.
- Emits a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartBatchedPresignEvent">StartBatchedPresignEvent</a></code> containing the batch session details.
- Emits a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartPresignFirstRoundEvent">StartPresignFirstRoundEvent</a></code> for each presign in the batch, with relevant details.


<a name="@Parameters_22"></a>

##### Parameters

- <code><a href="dwallet.md#0x3_dwallet">dwallet</a></code>: A reference to the target dWallet. This is used to retrieve the dWallet's ID and output.
- <code>batch_size</code>: The number of presign sessions to be created in this batch.
- <code>ctx</code>: The mutable transaction context, used to generate unique object IDs and retrieve the initiator.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_batched_presign">launch_batched_presign</a>(<a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">dwallet_2pc_mpc_ecdsa_k1::Secp256K1</a>&gt;, batch_size: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_batched_presign">launch_batched_presign</a>(
    <a href="dwallet.md#0x3_dwallet">dwallet</a>: &DWallet&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;,
    batch_size: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> batch_session_id = <a href="../pera-framework/object.md#0x2_object_id_from_address">object::id_from_address</a>(<a href="../pera-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx));
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartBatchedPresignEvent">StartBatchedPresignEvent</a> {
        session_id: batch_session_id,
        batch_size,
        initiator: <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx)
    });
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; batch_size) {
        <b>let</b> session_id = <a href="../pera-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx);
        i = i + 1;
        <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartPresignFirstRoundEvent">StartPresignFirstRoundEvent</a> {
            session_id: <a href="../pera-framework/object.md#0x2_object_id_from_address">object::id_from_address</a>(session_id),
            initiator: <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx),
            dwallet_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(<a href="dwallet.md#0x3_dwallet">dwallet</a>),
            dkg_output: get_dwallet_output&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>),
            batch_session_id,
            dwallet_mpc_network_key_version: get_dwallet_mpc_network_key_version(<a href="dwallet.md#0x3_dwallet">dwallet</a>),
        });
    };
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_launch_presign_second_round"></a>

## Function `launch_presign_second_round`

Launches the second round of the presign session.

This function emits a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartPresignSecondRoundEvent">StartPresignSecondRoundEvent</a></code>, which is caught by validators
to begin the second round of the presign process.


<a name="@Parameters_23"></a>

##### Parameters

- <code>initiator</code>: The address of the user initiating the presign session.
- <code>dwallet_id</code>: The ID of the associated dWallet.
- <code>dkg_output</code>: The output from the DKG process.
- <code>dwallet_cap_id</code>: The ID of the associated <code>DWalletCap</code>.
- <code>first_round_output</code>: The output from the first round of the presign process.
- <code>first_round_session_id</code>: The session ID of the first presign round.
- <code>ctx</code>: The transaction context used to emit the event.


<a name="@Panics_24"></a>

##### Panics

- Panics with <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a></code> if the sender of the transaction is not the system address.


<a name="@Emits_25"></a>

##### Emits

- <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartPresignSecondRoundEvent">StartPresignSecondRoundEvent</a></code>: Includes session ID, initiator address, dWallet ID, dWallet capability ID,
DKG output, first round output, and first round session ID.


<pre><code><b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_presign_second_round">launch_presign_second_round</a>(initiator: <b>address</b>, dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, dkg_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, first_round_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, first_round_session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, batch_session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, dwallet_mpc_network_key_version: u8, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_presign_second_round">launch_presign_second_round</a>(
    initiator: <b>address</b>,
    dwallet_id: ID,
    dkg_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    first_round_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    first_round_session_id: ID,
    batch_session_id: ID,
    dwallet_mpc_network_key_version: u8,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(<a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_SYSTEM_ADDRESS">SYSTEM_ADDRESS</a>, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a>);

    <b>let</b> session_id = <a href="../pera-framework/object.md#0x2_object_id_from_address">object::id_from_address</a>(<a href="../pera-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx));

    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartPresignSecondRoundEvent">StartPresignSecondRoundEvent</a> {
        session_id,
        initiator,
        dwallet_id,
        dkg_output,
        first_round_output,
        first_round_session_id,
        batch_session_id,
        dwallet_mpc_network_key_version,
    });
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_create_batched_presign_output"></a>

## Function `create_batched_presign_output`

Completes the presign session by creating the output of the
second presign round and transferring it to the session initiator.

This function is called by validators as part of the blockchain logic.
It creates a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a></code> object representing the second presign round output,
emits a <code>CompletedPresignEvent</code>, and transfers the result to the initiating user.


<a name="@Parameters_26"></a>

##### Parameters

- <code>initiator</code>: The address of the user who initiated the presign session.
- <code>session_id</code>: The ID of the presign session.
- <code>output</code>: The presign result data.
- <code>dwallet_cap_id</code>: The ID of the associated <code>DWalletCap</code>.
- <code>dwallet_id</code>: The ID of the associated <code>DWallet</code>.
- <code>ctx</code>: The transaction context.


<a name="@Emits_27"></a>

##### Emits

- <code>CompletedPresignEvent</code>: Includes the initiator, dWallet ID, and presign ID.


<a name="@Panics_28"></a>

##### Panics

- Panics with <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a></code> if the sender of the transaction is not the system address.


<a name="@Effects_29"></a>

##### Effects

- Creates a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a></code> object and transfers it to the session initiator.
- Emits a <code>CompletedPresignEvent</code>.


<pre><code><b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_batched_presign_output">create_batched_presign_output</a>(initiator: <b>address</b>, batch_session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, first_round_session_ids: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>&gt;, presigns: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_batched_presign_output">create_batched_presign_output</a>(
    initiator: <b>address</b>,
    batch_session_id: ID,
    first_round_session_ids: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;ID&gt;,
    presigns: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    dwallet_id: ID,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(<a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_SYSTEM_ADDRESS">SYSTEM_ADDRESS</a>, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a>);
    <b>let</b> <b>mut</b> i: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
    <b>let</b> <b>mut</b> batch_presigns_ids: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;ID&gt; = <a href="../move-stdlib/vector.md#0x1_vector_empty">vector::empty</a>();
    <b>let</b> first_round_session_ids_len = <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&first_round_session_ids);
    <b>while</b> (i &lt; first_round_session_ids_len) {
        <b>let</b> first_round_session_id = first_round_session_ids[i];
        <b>let</b> presign = presigns[i];
        <b>let</b> output = <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a> {
            id: <a href="../pera-framework/object.md#0x2_object_new">object::new</a>(ctx),
            first_round_session_id,
            dwallet_id,
            presign,
        };
        batch_presigns_ids.push_back(<a href="../pera-framework/object.md#0x2_object_id">object::id</a>(&output));
        <a href="../pera-framework/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(output, initiator);
        i = i + 1;
    };

    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedBatchedPresignEvent">CompletedBatchedPresignEvent</a> {
        initiator,
        dwallet_id,
        session_id: batch_session_id,
        presign_ids: batch_presigns_ids,
        presigns,
        first_round_session_ids,
    });
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_approve_messages"></a>

## Function `approve_messages`

Create a set of message approvals.
The messages must be approved in the same order as they were created.
The messages must be approved by the same <code>dwallet_cap_id</code>.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_approve_messages">approve_messages</a>(dwallet_cap: &<a href="dwallet.md#0x3_dwallet_DWalletCap">dwallet::DWalletCap</a>, messages: &<b>mut</b> <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_MessageApproval">dwallet_2pc_mpc_ecdsa_k1::MessageApproval</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_approve_messages">approve_messages</a>(
    dwallet_cap: &DWalletCap,
    messages: &<b>mut</b> <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;
): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_MessageApproval">MessageApproval</a>&gt; {
    <b>let</b> dwallet_cap_id = <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(dwallet_cap);
    <b>let</b> <b>mut</b> message_approvals = <a href="../move-stdlib/vector.md#0x1_vector_empty">vector::empty</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_MessageApproval">MessageApproval</a>&gt;();

    // Approve all messages and maintain their order.
    <b>let</b> messages_length = <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(messages);
    <b>let</b> <b>mut</b> i: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
    <b>while</b> (i &lt; messages_length) {
        <b>let</b> message = <a href="../move-stdlib/vector.md#0x1_vector_pop_back">vector::pop_back</a>(messages);
        <a href="../move-stdlib/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> message_approvals, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_MessageApproval">MessageApproval</a> {
            dwallet_cap_id,
            message,
        });
        i = i + 1;
    };
    <a href="../move-stdlib/vector.md#0x1_vector_reverse">vector::reverse</a>(&<b>mut</b> message_approvals);
    message_approvals
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_remove_message_approval"></a>

## Function `remove_message_approval`

Remove a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_MessageApproval">MessageApproval</a></code> and return the <code>dwallet_cap_id</code>
and the <code>message</code>.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_remove_message_approval">remove_message_approval</a>(message_approval: <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_MessageApproval">dwallet_2pc_mpc_ecdsa_k1::MessageApproval</a>): (<a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_remove_message_approval">remove_message_approval</a>(message_approval: <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_MessageApproval">MessageApproval</a>): (ID, <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;) {
    <b>let</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_MessageApproval">MessageApproval</a> {
        dwallet_cap_id,
        message
    } = message_approval;
    (dwallet_cap_id, message)
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_sign"></a>

## Function `sign`

Initiates the signing process for a given dWallet.

This function emits a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartSignEvent">StartSignEvent</a></code>, providing all necessary
metadata and ensuring the integrity of the signing process.
It validates the linkage between the <code>DWallet</code>, <code>DWalletCap</code>, and <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a></code>.
It also "burns" the [<code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a></code>] object, by sending it to the system address,
as every presign can only be used to sign only one message.


<a name="@Effects_30"></a>

##### Effects

- Validates the linkage between dWallet components.
- Verifies that the number of <code>hashed_messages</code>, <code>message_approvals</code>, and
<code>centralized_signed_messages</code> are equal.
- Emits a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartSignEvent">StartSignEvent</a></code> with the hashed message, presign outputs,
and additional metadata.


<a name="@Emits_31"></a>

##### Emits

- <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartBatchedSignEvent">StartBatchedSignEvent</a></code>:
- Contains the session details and the list of hashed messages.
- <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartSignEvent">StartSignEvent</a></code>:
- Includes session details, hashed message, presign outputs,
and DKG output.


<a name="@Aborts_32"></a>

##### Aborts

- **<code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EDwalletMismatch">EDwalletMismatch</a></code>**: If the <code><a href="dwallet.md#0x3_dwallet">dwallet</a></code> object does not match the ID
in the <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a></code> object.
- **<code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EApprovalsAndMessagesLenMismatch">EApprovalsAndMessagesLenMismatch</a></code>**: If the length of the <code>hashed_messages</code>
does not match the length of the <code>message_approvals</code>.
- **<code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ECentralizedSignedMessagesAndMessagesLenMismatch">ECentralizedSignedMessagesAndMessagesLenMismatch</a></code>**: If the length of
<code>hashed_messages</code> does not match the length of <code>centralized_signed_messages</code>.
- **<code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EMessageApprovalDWalletMismatch">EMessageApprovalDWalletMismatch</a></code>**: If the DWalletCap ID does not match
the expected DWalletCap ID for any of the message approvals.
- **<code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EMissingApprovalOrWrongApprovalOrder">EMissingApprovalOrWrongApprovalOrder</a></code>**: If the approved messages are not
in the same order as the <code>hashed_messages</code>.


<a name="@Parameters_33"></a>

##### Parameters

- <code>dwallet_cap</code>: The capability associated with the dWallet.
- <code>hashed_messages</code>: The list of hashed messages to be signed.
- <code>message_approvals</code>: The approvals for the messages.
- <code>presign</code>: The presign object containing intermediate outputs.
- <code><a href="dwallet.md#0x3_dwallet">dwallet</a></code>: The dWallet object.
- <code>centralized_signed_messages</code>: The list of centralized signatures.
- <code>presign_session_id</code>: The session ID of the presign process.
- <code>ctx</code>: The mutable transaction context.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_sign">sign</a>(message_approvals: &<b>mut</b> <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_MessageApproval">dwallet_2pc_mpc_ecdsa_k1::MessageApproval</a>&gt;, hashed_messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, presigns: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">dwallet_2pc_mpc_ecdsa_k1::Presign</a>&gt;, <a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">dwallet_2pc_mpc_ecdsa_k1::Secp256K1</a>&gt;, centralized_signed_messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_sign">sign</a>(
    message_approvals: &<b>mut</b> <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_MessageApproval">MessageApproval</a>&gt;,
    <b>mut</b> hashed_messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    <b>mut</b> presigns: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a>&gt;,
    <a href="dwallet.md#0x3_dwallet">dwallet</a>: &DWallet&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;,
    <b>mut</b> centralized_signed_messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> messages_len: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&hashed_messages);
    <b>let</b> presigns_len: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&presigns);
    <b>let</b> approvals_len: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(message_approvals);
    <b>let</b> centralized_signed_len: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&centralized_signed_messages);
    <b>assert</b>!(messages_len == approvals_len, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EApprovalsAndMessagesLenMismatch">EApprovalsAndMessagesLenMismatch</a>);
    <b>assert</b>!(messages_len == centralized_signed_len, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ECentralizedSignedMessagesAndMessagesLenMismatch">ECentralizedSignedMessagesAndMessagesLenMismatch</a>);
    <b>assert</b>!(messages_len == presigns_len, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EPresignsAndMessagesLenMismatch">EPresignsAndMessagesLenMismatch</a>);
    <b>let</b> expected_dwallet_cap_id = get_dwallet_cap_id(<a href="dwallet.md#0x3_dwallet">dwallet</a>);
    <b>let</b> batch_session_id = <a href="../pera-framework/object.md#0x2_object_id_from_address">object::id_from_address</a>(<a href="../pera-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx));
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartBatchedSignEvent">StartBatchedSignEvent</a> {
        session_id: batch_session_id,
        hashed_messages,
        initiator: <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx)
    });
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> message_approvals_len = <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(message_approvals);
    <b>while</b> (i &lt; message_approvals_len) {
        <b>let</b> presign = <a href="../move-stdlib/vector.md#0x1_vector_pop_back">vector::pop_back</a>(&<b>mut</b> presigns);
        <b>assert</b>!(<a href="../pera-framework/object.md#0x2_object_id">object::id</a>(<a href="dwallet.md#0x3_dwallet">dwallet</a>) == presign.dwallet_id, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EDwalletMismatch">EDwalletMismatch</a>);
        <b>let</b> message = <a href="../move-stdlib/vector.md#0x1_vector_pop_back">vector::pop_back</a>(&<b>mut</b> hashed_messages);
        <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_pop_and_verify_message_approval">pop_and_verify_message_approval</a>(expected_dwallet_cap_id, message, message_approvals);
        <b>let</b> id = <a href="../pera-framework/object.md#0x2_object_id_from_address">object::id_from_address</a>(<a href="../pera-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx));
        <b>let</b> centralized_signed_message = <a href="../move-stdlib/vector.md#0x1_vector_pop_back">vector::pop_back</a>(&<b>mut</b> centralized_signed_messages);
        <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartSignEvent">StartSignEvent</a> {
            session_id: id,
            presign_session_id: presign.first_round_session_id,
            initiator: <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx),
            batched_session_id: batch_session_id,
            dwallet_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(<a href="dwallet.md#0x3_dwallet">dwallet</a>),
            presign: presign.presign,
            centralized_signed_message,
            dkg_output: get_dwallet_output&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>),
            hashed_message: message,
            dwallet_mpc_network_key_version: get_dwallet_mpc_network_key_version&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>),
        });
        <a href="../pera-framework/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(presign, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_SYSTEM_ADDRESS">SYSTEM_ADDRESS</a>);
        i = i + 1;
    };
    presigns.destroy_empty();
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_create_sign_output"></a>

## Function `create_sign_output`

Emits a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedSignEvent">CompletedSignEvent</a></code> with the MPC Sign protocol output.

This function is called by the blockchain itself and is part of the core
blockchain logic executed by validators. The emitted event contains the
completed sign output that should be consumed by the initiating user.


<a name="@Parameters_34"></a>

##### Parameters

- **<code>signed_messages</code>**: A vector containing the signed message outputs.
- **<code>batch_session_id</code>**: The unique identifier for the batch signing session.
- **<code>ctx</code>**: The transaction context used for event emission.


<a name="@Requirements_35"></a>

##### Requirements

- The caller **must be the system address** (<code>@0x0</code>). If this condition is not met,
the function will abort with <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a></code>.


<a name="@Events_36"></a>

##### Events

- **<code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedSignEvent">CompletedSignEvent</a></code>**: Emitted with the <code>session_id</code> and <code>signed_messages</code>,
signaling the completion of the sign process for the batch session.


<a name="@Errors_37"></a>

##### Errors

- **<code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a></code>**: If the caller is not the system address (<code>@0x0</code>),
the function will abort with this error.


<pre><code><b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_sign_output">create_sign_output</a>(signed_messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, batch_session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, initiator: <b>address</b>, dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_sign_output">create_sign_output</a>(
    signed_messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    batch_session_id: ID,
    initiator: <b>address</b>,
    dwallet_id: ID,
    ctx: &<b>mut</b> TxContext
) {
    // Ensure that only the system <b>address</b> can call this function.
    <b>assert</b>!(<a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_SYSTEM_ADDRESS">SYSTEM_ADDRESS</a>, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a>);

    <b>let</b> sign_output = <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_BatchedSignOutput">BatchedSignOutput</a> {
        id: <a href="../pera-framework/object.md#0x2_object_new">object::new</a>(ctx),
        signatures: signed_messages,
        dwallet_id,
        session_id: batch_session_id
    };
    <a href="../pera-framework/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(sign_output, initiator);

    // Emit the <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedSignEvent">CompletedSignEvent</a> <b>with</b> session ID and signed messages.
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedSignEvent">CompletedSignEvent</a> {
        session_id: batch_session_id,
        signed_messages,
    });
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_create_mock_dwallet_for_testing"></a>

## Function `create_mock_dwallet_for_testing`

Generates a mock <code>DWallet&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;</code> object for testing purposes.

This function creates a dWallet object with random data,
useful for testing or initialization in non-production environments.


<a name="@Parameters_38"></a>

##### Parameters

- <code>ctx</code>: The transaction context for generating IDs and objects.
- <code>dwallet_cap_id</code>: The ID of the capability associated with the mock dWallet.
- <code>dkg_output</code>: The decentralized DKG output.


<a name="@Effects_39"></a>

##### Effects

- Creates and initializes a <code>DWallet&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;</code> object.
- Links the dWallet to the provided capability.


<a name="@Returns_40"></a>

##### Returns

- <code>DWallet&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;</code>: A mock dWallet object.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_mock_dwallet_for_testing">create_mock_dwallet_for_testing</a>(dkg_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">dwallet_2pc_mpc_ecdsa_k1::Secp256K1</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_mock_dwallet_for_testing">create_mock_dwallet_for_testing</a>(
    dkg_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext
): DWallet&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt; {
    <b>let</b> dwallet_cap = create_dwallet_cap(ctx);
    <b>let</b> dwallet_cap_id = <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(&dwallet_cap);
    <a href="../pera-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(dwallet_cap, <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx));
    <b>let</b> session_id = <a href="../pera-framework/object.md#0x2_object_id_from_address">object::id_from_address</a>(<a href="../pera-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx));
    <b>let</b> dwallet_mpc_network_key_version: u8 = 1;
    <a href="dwallet.md#0x3_dwallet_create_dwallet">dwallet::create_dwallet</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;(session_id, dwallet_cap_id, dkg_output, dwallet_mpc_network_key_version, ctx)
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_create_mock_dwallet"></a>

## Function `create_mock_dwallet`

Created an immutable [<code>DWallet</code>] object with the given DKG output.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_mock_dwallet">create_mock_dwallet</a>(dkg_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_mock_dwallet">create_mock_dwallet</a>(
    dkg_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> dwallet_cap = create_dwallet_cap(ctx);
    <b>let</b> dwallet_cap_id = <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(&dwallet_cap);
    <a href="../pera-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(dwallet_cap, <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx));
    <b>let</b> session_id = <a href="../pera-framework/object.md#0x2_object_id_from_address">object::id_from_address</a>(<a href="../pera-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx));
    <b>let</b> dwallet_mpc_network_key_version: u8 = 1;
    <b>let</b> <a href="dwallet.md#0x3_dwallet">dwallet</a> = <a href="dwallet.md#0x3_dwallet_create_dwallet">dwallet::create_dwallet</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;(session_id, dwallet_cap_id, dkg_output, dwallet_mpc_network_key_version, ctx);
    <a href="../pera-framework/transfer.md#0x2_transfer_public_freeze_object">transfer::public_freeze_object</a>(<a href="dwallet.md#0x3_dwallet">dwallet</a>);
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_create_mock_presign"></a>

## Function `create_mock_presign`

Generates a new mock <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a></code> object with random IDs and data.
This function is useful for testing or initializing Presign objects.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_mock_presign">create_mock_presign</a>(dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, presign: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, first_round_session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">dwallet_2pc_mpc_ecdsa_k1::Presign</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_mock_presign">create_mock_presign</a>(
    dwallet_id: ID,
    presign: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    first_round_session_id: ID,
    ctx: &<b>mut</b> TxContext,
): <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a> {
    <b>let</b> id = <a href="../pera-framework/object.md#0x2_object_new">object::new</a>(ctx);

    // Create and <b>return</b> the <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a> <a href="../pera-framework/object.md#0x2_object">object</a>.
    <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a> {
        id,
        dwallet_id,
        presign,
        first_round_session_id,
    }
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_publish_partially_signed_messages"></a>

## Function `publish_partially_signed_messages`

A function to publish messages signed by the user on chain with on-chain verification,
without launching the chain's sign flow immediately.

See the docs of [<code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_PartiallySignedMessages">PartiallySignedMessages</a></code>] for more details on when this may be used.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_publish_partially_signed_messages">publish_partially_signed_messages</a>(signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, presigns: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">dwallet_2pc_mpc_ecdsa_k1::Presign</a>&gt;, <a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">dwallet_2pc_mpc_ecdsa_k1::Secp256K1</a>&gt;, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_publish_partially_signed_messages">publish_partially_signed_messages</a>(
    signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    <b>mut</b> presigns: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a>&gt;,
    <a href="dwallet.md#0x3_dwallet">dwallet</a>: &DWallet&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> messages_len = <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&messages);
    <b>let</b> signatures_len = <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&signatures);
    <b>let</b> presigns_len = <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&presigns);
    <b>assert</b>!(messages_len == signatures_len, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EApprovalsAndSignaturesLenMismatch">EApprovalsAndSignaturesLenMismatch</a>);
    <b>assert</b>!(messages_len == presigns_len, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EPresignsAndMessagesLenMismatch">EPresignsAndMessagesLenMismatch</a>);
    <b>let</b> <b>mut</b> presigns_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt; = <a href="../move-stdlib/vector.md#0x1_vector_empty">vector::empty</a>();
    <b>let</b> <b>mut</b> presign_session_ids: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;ID&gt; = <a href="../move-stdlib/vector.md#0x1_vector_empty">vector::empty</a>();
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; messages_len) {
        <b>let</b> presign = <a href="../move-stdlib/vector.md#0x1_vector_pop_back">vector::pop_back</a>(&<b>mut</b> presigns);
        <b>assert</b>!(presign.dwallet_id == <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(<a href="dwallet.md#0x3_dwallet">dwallet</a>), <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EDwalletMismatch">EDwalletMismatch</a>);
        presigns_bytes.push_back(presign.presign);
        presign_session_ids.push_back(presign.first_round_session_id);
        <a href="../pera-framework/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(presign, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_SYSTEM_ADDRESS">SYSTEM_ADDRESS</a>);
        i = i + 1;
    };
    presigns_bytes.reverse();
    presign_session_ids.reverse();
    presigns.destroy_empty();
    <b>assert</b>!(
        <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_verify_partially_signed_signatures">verify_partially_signed_signatures</a>(
            signatures,
            messages,
            presigns_bytes,
            get_dwallet_output(<a href="dwallet.md#0x3_dwallet">dwallet</a>)
        ),
        <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EInvalidSignatures">EInvalidSignatures</a>
    );
    <b>let</b> partial_signatures = <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_PartiallySignedMessages">PartiallySignedMessages</a> {
        id: <a href="../pera-framework/object.md#0x2_object_new">object::new</a>(ctx),
        presigns: presigns_bytes,
        presign_session_ids,
        messages,
        signatures,
        dwallet_output: get_dwallet_output(<a href="dwallet.md#0x3_dwallet">dwallet</a>),
        dwallet_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(<a href="dwallet.md#0x3_dwallet">dwallet</a>),
        dwallet_cap_id: get_dwallet_cap_id(<a href="dwallet.md#0x3_dwallet">dwallet</a>),
        dwallet_mpc_network_key_version: get_dwallet_mpc_network_key_version(<a href="dwallet.md#0x3_dwallet">dwallet</a>),
    };
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_CreatedPartiallySignedMessagesEvent">CreatedPartiallySignedMessagesEvent</a> {
        partial_signatures_object_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(&partial_signatures),
    });
    <a href="../pera-framework/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(partial_signatures, <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx));
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_future_sign"></a>

## Function `future_sign`

A function to launch a sign flow with a previously published [<code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_PartiallySignedMessages">PartiallySignedMessages</a></code>].

See the docs of [<code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_PartiallySignedMessages">PartiallySignedMessages</a></code>] for more details on when this may be used.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_future_sign">future_sign</a>(partial_signature: <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_PartiallySignedMessages">dwallet_2pc_mpc_ecdsa_k1::PartiallySignedMessages</a>, message_approvals: &<b>mut</b> <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_MessageApproval">dwallet_2pc_mpc_ecdsa_k1::MessageApproval</a>&gt;, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_future_sign">future_sign</a>(
    partial_signature: <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_PartiallySignedMessages">PartiallySignedMessages</a>,
    message_approvals: &<b>mut</b> <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_MessageApproval">MessageApproval</a>&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_PartiallySignedMessages">PartiallySignedMessages</a> {
        id,
        <b>mut</b> presigns,
        <b>mut</b> presign_session_ids,
        <b>mut</b> messages,
        <b>mut</b> signatures,
        dwallet_id,
        dwallet_cap_id,
        dwallet_output,
        dwallet_mpc_network_key_version,
    } = partial_signature;
    <a href="../pera-framework/object.md#0x2_object_delete">object::delete</a>(id);
    <b>let</b> message_approvals_len = <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(message_approvals);
    <b>let</b> messages_len = <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&messages);
    <b>assert</b>!(message_approvals_len == messages_len, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EApprovalsAndMessagesLenMismatch">EApprovalsAndMessagesLenMismatch</a>);
    <b>let</b> batch_session_id = <a href="../pera-framework/object.md#0x2_object_id_from_address">object::id_from_address</a>(<a href="../pera-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx));
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartBatchedSignEvent">StartBatchedSignEvent</a> {
        session_id: batch_session_id,
        hashed_messages: messages,
        initiator: <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx)
    });
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; message_approvals_len) {
        <b>let</b> message = <a href="../move-stdlib/vector.md#0x1_vector_pop_back">vector::pop_back</a>(&<b>mut</b> messages);
        <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_pop_and_verify_message_approval">pop_and_verify_message_approval</a>(dwallet_cap_id, message, message_approvals);
        <b>let</b> id = <a href="../pera-framework/object.md#0x2_object_id_from_address">object::id_from_address</a>(<a href="../pera-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx));
        <b>let</b> centralized_signed_message = <a href="../move-stdlib/vector.md#0x1_vector_pop_back">vector::pop_back</a>(&<b>mut</b> signatures);
        <b>let</b> presign = <a href="../move-stdlib/vector.md#0x1_vector_pop_back">vector::pop_back</a>(&<b>mut</b> presigns);
        <b>let</b> presign_session_id = <a href="../move-stdlib/vector.md#0x1_vector_pop_back">vector::pop_back</a>(&<b>mut</b> presign_session_ids);
        <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartSignEvent">StartSignEvent</a> {
            session_id: id,
            presign_session_id,
            initiator: <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx),
            batched_session_id: batch_session_id,
            dwallet_id,
            presign,
            centralized_signed_message,
            dkg_output: dwallet_output,
            hashed_message: message,
            dwallet_mpc_network_key_version,
        });
        i = i + 1;
    };
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_pop_and_verify_message_approval"></a>

## Function `pop_and_verify_message_approval`

Pops the last message approval from the vector and verifies it against tje given message & dwallet_cap_id.


<pre><code><b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_pop_and_verify_message_approval">pop_and_verify_message_approval</a>(dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, message: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, message_approvals: &<b>mut</b> <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_MessageApproval">dwallet_2pc_mpc_ecdsa_k1::MessageApproval</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_pop_and_verify_message_approval">pop_and_verify_message_approval</a>(
    dwallet_cap_id: ID,
    message: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    message_approvals: &<b>mut</b> <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_MessageApproval">MessageApproval</a>&gt;
) {
    <b>let</b> message_approval = <a href="../move-stdlib/vector.md#0x1_vector_pop_back">vector::pop_back</a>(message_approvals);
    <b>let</b> (message_approval_dwallet_cap_id, approved_message) = <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_remove_message_approval">remove_message_approval</a>(message_approval);
    <b>assert</b>!(dwallet_cap_id == message_approval_dwallet_cap_id, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EMessageApprovalDWalletMismatch">EMessageApprovalDWalletMismatch</a>);
    <b>assert</b>!(&message == &approved_message, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EMissingApprovalOrWrongApprovalOrder">EMissingApprovalOrWrongApprovalOrder</a>);
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_verify_partially_signed_signatures"></a>

## Function `verify_partially_signed_signatures`

Verifies that the user's centralized party signatures are valid.


<pre><code><b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_verify_partially_signed_signatures">verify_partially_signed_signatures</a>(partial_signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, presigns: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, dkg_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_verify_partially_signed_signatures">verify_partially_signed_signatures</a>(
    partial_signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    presigns: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    dkg_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
): bool;
</code></pre>



</details>
