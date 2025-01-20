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
-  [Resource `EncryptedUserSecretKeyShare`](#0x3_dwallet_2pc_mpc_ecdsa_k1_EncryptedUserSecretKeyShare)
-  [Struct `StartDKGSecondRoundEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGSecondRoundEvent)
-  [Struct `CompletedDKGSecondRoundEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedDKGSecondRoundEvent)
-  [Struct `StartPresignFirstRoundEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_StartPresignFirstRoundEvent)
-  [Struct `StartPresignSecondRoundEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_StartPresignSecondRoundEvent)
-  [Struct `CompletedBatchedPresignEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedBatchedPresignEvent)
-  [Struct `StartSignEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_StartSignEvent)
-  [Struct `StartBatchedSignEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_StartBatchedSignEvent)
-  [Struct `StartBatchedPresignEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_StartBatchedPresignEvent)
-  [Constants](#@Constants_12)
-  [Function `launch_dkg_first_round`](#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_dkg_first_round)
-  [Function `create_dkg_first_round_output`](#0x3_dwallet_2pc_mpc_ecdsa_k1_create_dkg_first_round_output)
-  [Function `launch_dkg_second_round`](#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_dkg_second_round)
-  [Function `transfer_encrypted_user_share`](#0x3_dwallet_2pc_mpc_ecdsa_k1_transfer_encrypted_user_share)
-  [Function `create_encrypted_user_share`](#0x3_dwallet_2pc_mpc_ecdsa_k1_create_encrypted_user_share)
-  [Function `create_dkg_second_round_output`](#0x3_dwallet_2pc_mpc_ecdsa_k1_create_dkg_second_round_output)
-  [Function `launch_batched_presign`](#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_batched_presign)
-  [Function `launch_presign_second_round`](#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_presign_second_round)
-  [Function `create_batched_presign_output`](#0x3_dwallet_2pc_mpc_ecdsa_k1_create_batched_presign_output)
-  [Function `sign`](#0x3_dwallet_2pc_mpc_ecdsa_k1_sign)
-  [Function `create_sign_output`](#0x3_dwallet_2pc_mpc_ecdsa_k1_create_sign_output)
-  [Function `create_mock_dwallet_for_testing`](#0x3_dwallet_2pc_mpc_ecdsa_k1_create_mock_dwallet_for_testing)
-  [Function `create_mock_dwallet`](#0x3_dwallet_2pc_mpc_ecdsa_k1_create_mock_dwallet)
-  [Function `create_mock_presign`](#0x3_dwallet_2pc_mpc_ecdsa_k1_create_mock_presign)
-  [Function `publish_partially_signed_messages`](#0x3_dwallet_2pc_mpc_ecdsa_k1_publish_partially_signed_messages)
-  [Function `future_sign`](#0x3_dwallet_2pc_mpc_ecdsa_k1_future_sign)
-  [Function `verify_partially_signed_signatures`](#0x3_dwallet_2pc_mpc_ecdsa_k1_verify_partially_signed_signatures)


<pre><code><b>use</b> <a href="../move-stdlib/vector.md#0x1_vector">0x1::vector</a>;
<b>use</b> <a href="../pera-framework/event.md#0x2_event">0x2::event</a>;
<b>use</b> <a href="../pera-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../pera-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../pera-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="dwallet.md#0x3_dwallet">0x3::dwallet</a>;
<b>use</b> <a href="pera_system.md#0x3_pera_system">0x3::pera_system</a>;
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

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_DKGFirstRoundOutputEvent"></a>

## Struct `DKGFirstRoundOutputEvent`

An event emitted when the first round of the DKG process is completed.

This event is emitted by the blockchain to notify the user about the completion of the first round.
The user should catch this event to generate inputs for the second round and call the <code>launch_dkg_second_round</code> function.


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

The output of the first round of the dWallet creation DKG process.


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

Represents the result of the second and final presign round.

This struct links the results of both presign rounds to a specific dWallet ID.


<a name="@Fields_2"></a>

##### Fields

- <code>id</code>: Unique identifier for the presign object.
- <code>dwallet_id</code>: ID of the associated dWallet.
- <code>first_round_session_id</code>: Session ID for the first presign round.
- <code>presign</code>: Serialized output of the presign process.


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

Event emitted to start an encrypted dWallet centralized (user) key share
verification process.
Ika does not support native functions, so an event is emitted and
caught by the blockchain, which then starts the verification process,
similar to the MPC processes.


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
<code>dwallet_centralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The DKG centralized output of the dwallet that its secret is being encrypted.
</dd>
<dt>
<code>dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The dWallet that this encrypted secret key share belongs to.
</dd>
<dt>
<code>encryption_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The encryption key used to encrypt the secret key share with.
</dd>
<dt>
<code>encryption_key_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The <code>EncryptionKey</code> Move object ID.
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
 The signed dWallet public_centralized_output,
 signed by the secret key that corresponds to <code>encryptor_ed25519_pubkey</code>.
</dd>
<dt>
<code>encryptor_ed25519_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The public key of the encryptor.
 Used to verify the signature on the public share (centralized public output).
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

Event emitted when an encrypted share is created by the system transaction.


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
 The ID of the encrypted secret key share.
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
 The encrypted secret key share and proof of encryption.
</dd>
<dt>
<code>encryption_key_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The Encryption (Public) Key that was used to encrypt the secret key share.
</dd>
<dt>
<code>encryptor_address: <b>address</b></code>
</dt>
<dd>
 Source address of the entity that encrypted this secret key share.
</dd>
<dt>
<code>encryptor_ed25519_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 Source Public Key of the entity that encrypted this secret key share (used for verifications).
</dd>
<dt>
<code>signed_public_share: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 Signed dWallet public centralized (user) output (singed by the encryptor entity).
 todo(zeev): rename.
</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_PartiallySignedMessages"></a>

## Resource `PartiallySignedMessages`

Messages that have been signed by a user, a.k.a the centralized party,
but not yet by the blockchain.
Used for scenarios where the user needs to first agree to sign some transaction,
and the blockchain signs this transaction only later,
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
<code>hashed_messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
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

</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGFirstRoundEvent"></a>

## Struct `StartDKGFirstRoundEvent`

Event emitted to start the first round of the DKG process.

This event is caught by the blockchain, which is then using it to
initiate the first round of the DKG.


<a name="@Fields_3"></a>

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


<a name="@Fields_4"></a>

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

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_EncryptedUserSecretKeyShare"></a>

## Resource `EncryptedUserSecretKeyShare`

A verified encrypted dWallet secret user key share.

This struct represents an encrypted user secret key share tied to a specific dWallet.
It includes cryptographic proof that the encryption is valid and securely linked
to the associated <code>dWallet</code>.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../pera-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>
 A unique identifier for this encrypted user share object.
</dd>
<dt>
<code>dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The ID of the dWallet associated with this encrypted secret share.
</dd>
<dt>
<code>encrypted_secret_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The encrypted secret key share along with a cryptographic proof
 that the encryption corresponds to the dWallet's secret key share.
</dd>
<dt>
<code>encryption_key_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The ID of the <code>EncryptionKey</code> object used to encrypt the secret share.
</dd>
<dt>
<code>signed_public_share: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The signed public share corresponding to the encrypted secret key share,
 used to verify its authenticity.
</dd>
<dt>
<code>encryptor_ed25519_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The Ed25519 public key of the encryptor, used to verify the signature
 on the encrypted secret share.
 todo(zeev): I think it's used to verify the signed_public_share?
</dd>
<dt>
<code>encryptor_address: <b>address</b></code>
</dt>
<dd>
 The address of the encryptor, identifying who performed the encryption.
 If the key is transferred to someone else, this is the source entity.
 If the key is re-encrypted by an entity, then this is the Ika address of this entity.
</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGSecondRoundEvent"></a>

## Struct `StartDKGSecondRoundEvent`

Event emitted to initiate the second round of the DKG process.

This event is emitted to notify Validators to begin the second round of the DKG.
It contains all necessary data to ensure proper continuation of the process.


<a name="@Fields_5"></a>

##### Fields

- **<code>session_id</code>**: The unique identifier for the DKG session.
- **<code>initiator</code>**: The address of the user who initiated the second round.
- **<code>first_round_output</code>**: The output from the first round of the DKG process.
- **<code>public_key_share_and_proof</code>**: A serialized vector containing the public key share and
its proof from the first round.
- **<code>dwallet_cap_id</code>**: The unique identifier of the dWallet capability associated with this session.
- **<code>first_round_session_id</code>**: The session ID of the first round of the DKG process.
- **<code>encrypted_secret_share_and_proof</code>**: Encrypted secret share and the associated cryptographic proof.
- **<code>encryption_key</code>**: The encryption key used in the process.
- **<code>encryption_key_id</code>**: The unique identifier of the EncryptionKey object.
- **<code>signed_public_share</code>**: The signed public share corresponding to the encrypted secret key share.
- **<code>encryptor_ed25519_pubkey</code>**: The Ed25519 public key of the entity that performed the encryption.
- **<code>dkg_centralized_public_output</code>**: The centralized public output of the DKG process.


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
<dt>
<code>encrypted_secret_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>encryption_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>encryption_key_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

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
<code>dkg_centralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedDKGSecondRoundEvent"></a>

## Struct `CompletedDKGSecondRoundEvent`

Event emitted upon the completion of the second round of the
Distributed Key Generation (DKG).

This event provides all necessary data generated from the second
round of the DKG process.
This event is emitted to notify Validators to finalize the DKG
process and store its results.


<a name="@Fields_6"></a>

##### Fields

- **<code>session_id</code>**: A unique identifier for the DKG session, linking all related events and actions.
- **<code>initiator</code>**: The address of the user who initiated the DKG process.
- **<code>dwallet_cap_id</code>**: The unique identifier of the dWallet capability associated with the session.
- **<code>dwallet_id</code>**: The identifier of the dWallet created as a result of the DKG process.
- **<code>value</code>**: The output value from the second round of the DKG process,
representing the validated and combined result from all participants.


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

This event is used to signal Validators to start the first round of the Presign process.
The event includes all necessary details to link the session to the corresponding dWallet
and DKG process.


<a name="@Fields_7"></a>

##### Fields

- **<code>session_id</code>**: A unique identifier for the Presign session.
- **<code>initiator</code>**: The address of the user who initiated the Presign session.
- **<code>dwallet_id</code>**: The unique identifier of the associated dWallet.
- **<code>dkg_output</code>**: The output produced by the DKG process, used as input for the Presign session.
- **<code>batch_session_id</code>**: A unique identifier for the Presign batch session.
- **<code>dwallet_mpc_network_key_version</code>**: The version of the dWallet's MPC network key.


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

This event signals Validators to begin the second round of the Presign process.
The second round is a critical step in the multi-party computation (MPC) protocol,
enabling the generation of pre-signatures required for ECDSA signing.


<a name="@Fields_8"></a>

##### Fields

- **<code>session_id</code>**: A unique identifier for the current Presign session.
- **<code>initiator</code>**: The address of the user who initiated the Presign session.
- **<code>dwallet_id</code>**: The unique identifier of the DWallet associated with this Presign session.
- **<code>dkg_output</code>**: The output from the Distributed Key Generation (DKG) process, used as input for the Presign session.
- **<code>first_round_output</code>**: The output generated from the first round of the Presign session.
- **<code>first_round_session_id</code>**: The session identifier for the first round of the Presign process.
- **<code>batch_session_id</code>**: A unique identifier linking this session to a batched Presign process.
- **<code>dwallet_mpc_network_key_version</code>**: The version of the dWallet's MPC network key.


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

Event emitted to initiate the signing process.

This event is captured by Validators to start the signing protocol.
It includes all the necessary information to link the signing process to a specific dWallet,
Presign session, and batched process.


<a name="@Fields_9"></a>

##### Fields

- **<code>session_id</code>**: A unique identifier for this signing session.
- **<code>presign_session_id</code>**: A unique identifier for the associated Presign session.
- **<code>initiator</code>**: The address of the user who initiated the signing event.
- **<code>batched_session_id</code>**: A unique identifier for the batched signing process this session belongs to.
- **<code>dwallet_id</code>**: The unique identifier for the dWallet used in the session.
- **<code>dkg_output</code>**: The output from the Distributed Key Generation (DKG) process used in this session.
- **<code>hashed_message</code>**: The hashed message to be signed in this session.
- **<code>presign</code>**: The Presign output used to assist in the signing process.
- **<code>centralized_signed_message</code>**: The final signed message generated by the centralized signing process.
- **<code>dwallet_mpc_network_key_version</code>**: The version of the dWallet's MPC network key.


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


<a name="@Fields_10"></a>

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


<a name="@Fields_11"></a>

##### Fields

- **<code>session_id</code>**: The session identifier for the batched sign process.
- **<code>batch_size</code>**: The number of presign sessions to be started.
- **<code>initiator</code>**: The address of the user who initiated the protocol.


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

<a name="@Constants_12"></a>

## Constants


<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress"></a>

Error raised when the sender is not the system address.


<pre><code><b>const</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1;
</code></pre>



<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_SYSTEM_ADDRESS"></a>

System address for asserting system-level actions.


<pre><code><b>const</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_SYSTEM_ADDRESS">SYSTEM_ADDRESS</a>: <b>address</b> = 0;
</code></pre>



<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_EApprovalsAndMessagesLenMismatch"></a>



<pre><code><b>const</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EApprovalsAndMessagesLenMismatch">EApprovalsAndMessagesLenMismatch</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 3;
</code></pre>



<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_EApprovalsAndSignaturesLenMismatch"></a>



<pre><code><b>const</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EApprovalsAndSignaturesLenMismatch">EApprovalsAndSignaturesLenMismatch</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 7;
</code></pre>



<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_ECentralizedSignedMessagesAndMessagesLenMismatch"></a>



<pre><code><b>const</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ECentralizedSignedMessagesAndMessagesLenMismatch">ECentralizedSignedMessagesAndMessagesLenMismatch</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 5;
</code></pre>



<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_EDwalletMismatch"></a>



<pre><code><b>const</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EDwalletMismatch">EDwalletMismatch</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 2;
</code></pre>



<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_EInvalidSignatures"></a>



<pre><code><b>const</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EInvalidSignatures">EInvalidSignatures</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 8;
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


<a name="@Effects_13"></a>

##### Effects

- Generates a new <code>DWalletCap</code> object.
- Transfers the <code>DWalletCap</code> to the session initiator (<code>ctx.sender</code>).
- Emits a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGFirstRoundEvent">StartDKGFirstRoundEvent</a></code>.


<a name="@Emits_14"></a>

##### Emits

- <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGFirstRoundEvent">StartDKGFirstRoundEvent</a></code>:
- <code>session_id</code>: The generated session ID.
- <code>initiator</code>: The address of the transaction sender.
- <code>dwallet_cap_id</code>: The ID of the created <code>DWalletCap</code>.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_dkg_first_round">launch_dkg_first_round</a>(_pera_system_state: &<a href="pera_system.md#0x3_pera_system_PeraSystemState">pera_system::PeraSystemState</a>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_dkg_first_round">launch_dkg_first_round</a>(_pera_system_state: &PeraSystemState, ctx: &<b>mut</b> TxContext) {
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


<a name="@Effects_15"></a>

##### Effects

- Transfers the output of the first round to the initiator.
- Emits necessary metadata and links it to the associated session.


<a name="@Parameters_16"></a>

##### Parameters

- <code>initiator</code>: The address of the user who initiated the DKG session.
- <code>session_id</code>: The ID of the DKG session.
- <code>output</code>: The output data from the first round.
- <code>dwallet_cap_id</code>: The ID of the associated <code>DWalletCap</code>.
- <code>ctx</code>: The transaction context.


<a name="@Panics_17"></a>

##### Panics

- Panics with <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a></code> if the sender is not the system address.


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

Initiates the second round of the Distributed Key Generation (DKG) process
and emits an event for validators to begin their participation in this round.

This function handles the creation of a new DKG session ID and emits an event containing
all the necessary parameters to continue the DKG process.

<a name="@Parameters_18"></a>

##### Parameters

- <code>dwallet_cap</code>: A reference to the <code>DWalletCap</code>, representing the capability associated with the dWallet.
- <code>centralized_public_key_share_and_proof</code>: The user (centralized) public key share and proof from the first round.
- <code>first_round_output</code>: A reference to the <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_DKGFirstRoundOutput">DKGFirstRoundOutput</a></code> structure containing the output of the first DKG round.
- <code>first_round_session_id</code>: The session ID associated with the first DKG round.
- <code>encrypted_secret_share_and_proof</code>: Encrypted user secret key share and its proof.
- <code>encryption_key</code>: A reference to the <code>EncryptionKey</code> object used for encrypting the secret key share.
- <code>signed_centralized_public_output</code>: The centralized public output signed by the caller.
- <code>encryptor_ed25519_pubkey</code>: The Ed25519 public key of the encryptor.
- <code>centralized_public_output</code>: The centralized public output of the DKG process.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_dkg_second_round">launch_dkg_second_round</a>(dwallet_cap: &<a href="dwallet.md#0x3_dwallet_DWalletCap">dwallet::DWalletCap</a>, centralized_public_key_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, first_round_output: &<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_DKGFirstRoundOutput">dwallet_2pc_mpc_ecdsa_k1::DKGFirstRoundOutput</a>, first_round_session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, encrypted_secret_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, encryption_key: &<a href="dwallet.md#0x3_dwallet_EncryptionKey">dwallet::EncryptionKey</a>, signed_centralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, encryptor_ed25519_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, centralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, _pera_system_state: &<a href="pera_system.md#0x3_pera_system_PeraSystemState">pera_system::PeraSystemState</a>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_dkg_second_round">launch_dkg_second_round</a>(
    dwallet_cap: &DWalletCap,
    centralized_public_key_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    first_round_output: &<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_DKGFirstRoundOutput">DKGFirstRoundOutput</a>,
    first_round_session_id: ID,
    encrypted_secret_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    encryption_key: &EncryptionKey,
    // todo(scaly): is it the <b>public</b> key?
    signed_centralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    encryptor_ed25519_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    // todo(scaly): is it the <b>public</b> eky?
    centralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    _pera_system_state: &PeraSystemState,
    ctx: &<b>mut</b> TxContext
): <b>address</b> {
    // todo(zeev): rename the <a href="../pera-framework/event.md#0x2_event">event</a> fields.
    <b>let</b> session_id = <a href="../pera-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx);
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGSecondRoundEvent">StartDKGSecondRoundEvent</a> {
        session_id,
        initiator: <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx),
        first_round_output: first_round_output.output,
        public_key_share_and_proof: centralized_public_key_share_and_proof,
        dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(dwallet_cap),
        first_round_session_id,
        encrypted_secret_share_and_proof,
        encryption_key: get_encryption_key(encryption_key),
        encryption_key_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(encryption_key),
        signed_public_share: signed_centralized_public_output,
        encryptor_ed25519_pubkey,
        dkg_centralized_public_output: centralized_public_output
    });
    session_id
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_transfer_encrypted_user_share"></a>

## Function `transfer_encrypted_user_share`

Transfers an encrypted dWallet user secret key share from a source entity to destination entity.

This function emits an event with the encrypted user secret key share, along with its cryptographic proof,
to the blockchain. The chain verifies that the encrypted data matches the expected secret key share
associated with the dWallet before creating an [<code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a></code>] object.


<a name="@Parameters_19"></a>

##### Parameters

- **<code><a href="dwallet.md#0x3_dwallet">dwallet</a></code>**: A reference to the <code>DWallet&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;</code> object to which the secret share is linked.
- **<code>destination_encryption_key</code>**: A reference to the encryption key used for encrypting the secret key share.
- **<code>encrypted_secret_share_and_proof</code>**: The encrypted secret key share, accompanied by a cryptographic proof.
- **<code>source_signed_centralized_public_output</code>**: The signed centralized public output corresponding to the secret share.
- **<code>source_ed25519_pubkey</code>**: The Ed25519 public key of the source (encryptor) used for verifying the signature.


<a name="@Effects_20"></a>

##### Effects

- Emits a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartEncryptedShareVerificationEvent">StartEncryptedShareVerificationEvent</a></code>,
which is captured by the blockchain to initiate the verification process.


<a name="@Emits_21"></a>

##### Emits

- **<code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartEncryptedShareVerificationEvent">StartEncryptedShareVerificationEvent</a></code>**:
- Includes the encrypted secret share, cryptographic proof,
encryption key, and additional metadata required for verification.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_transfer_encrypted_user_share">transfer_encrypted_user_share</a>(<a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">dwallet_2pc_mpc_ecdsa_k1::Secp256K1</a>&gt;, destination_encryption_key: &<a href="dwallet.md#0x3_dwallet_EncryptionKey">dwallet::EncryptionKey</a>, encrypted_secret_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, source_signed_centralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, source_ed25519_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, _pera_system_state: &<a href="pera_system.md#0x3_pera_system_PeraSystemState">pera_system::PeraSystemState</a>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_transfer_encrypted_user_share">transfer_encrypted_user_share</a>(
    <a href="dwallet.md#0x3_dwallet">dwallet</a>: &DWallet&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;,
    destination_encryption_key: &EncryptionKey,
    encrypted_secret_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    source_signed_centralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    source_ed25519_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    _pera_system_state: &PeraSystemState,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> session_id = <a href="../pera-framework/object.md#0x2_object_id_from_address">object::id_from_address</a>(<a href="../pera-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx));
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartEncryptedShareVerificationEvent">StartEncryptedShareVerificationEvent</a> {
        encrypted_secret_share_and_proof,
        dwallet_centralized_public_output: get_dwallet_centralized_output&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>),
        dwallet_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(<a href="dwallet.md#0x3_dwallet">dwallet</a>),
        encryption_key: get_encryption_key(destination_encryption_key),
        encryption_key_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(destination_encryption_key),
        session_id,
        signed_public_share: source_signed_centralized_public_output,
        encryptor_ed25519_pubkey: source_ed25519_pubkey,
        initiator: <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx),
    });
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_create_encrypted_user_share"></a>

## Function `create_encrypted_user_share`

Creates an encrypted user secret key share after it has been verified by the blockchain.

This function is invoked by the blockchain to generate an [<code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a></code>] object
once the associated encryption and cryptographic proofs have been verified.
It finalizes the process by storing the encrypted user share on-chain and emitting the relevant event.


<a name="@Parameters_22"></a>

##### Parameters

- **<code>dwallet_id</code>**: The unique identifier of the dWallet associated with the encrypted user share.
- **<code>encrypted_secret_share_and_proof</code>**: The encrypted secret share along with its cryptographic proof.
- **<code>encryption_key_id</code>**: The unique identifier of the encryption key used for the share.
- **<code>session_id</code>**: A unique identifier for the session related to this operation.
- **<code>signed_public_share</code>**: The signed public share corresponding to the encrypted secret share.
- **<code>encryptor_ed25519_pubkey</code>**: The Ed25519 public key of the encryptor used for signing.
- **<code>initiator</code>**: The address of the entity that performed the encryption.

<a name="@Effects_23"></a>

##### Effects

- Creates an <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a></code> object and stores it on-chain.
- Transfers the newly created share to the initiator // todo(itay): mistake?
- Emits a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_CreatedEncryptedSecretShareEvent">CreatedEncryptedSecretShareEvent</a></code> which includes details about the encrypted share,
cryptographic proofs, and related metadata.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_encrypted_user_share">create_encrypted_user_share</a>(dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, encrypted_secret_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, encryption_key_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, signed_public_share: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, encryptor_ed25519_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, initiator: <b>address</b>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_encrypted_user_share">create_encrypted_user_share</a>(
    dwallet_id: ID,
    encrypted_secret_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    encryption_key_id: ID,
    session_id: ID,
    // todo(zeev): rename
    signed_public_share: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    encryptor_ed25519_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    // todo(itay): this name is not correct? should be dest?
    initiator: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(<a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_SYSTEM_ADDRESS">SYSTEM_ADDRESS</a>, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a>);
    <b>let</b> encrypted_user_share = <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a> {
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
    <a href="../pera-framework/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(encrypted_user_share, initiator);
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_create_dkg_second_round_output"></a>

## Function `create_dkg_second_round_output`

Completes the second round of the Distributed Key Generation (DKG) process and
creates the [<code>DWallet</code>].

This function finalizes the DKG process by creating a <code>DWallet</code> object and associating it with the
cryptographic outputs of the second round. It also generates an encrypted user share and emits
events to record the results of the process.
This function is called by the blockchain.


<a name="@Parameters_24"></a>

##### Parameters

- **<code>initiator</code>**: The address of the user who initiated the DKG session.
- **<code>session_id</code>**: A unique identifier for the current DKG session.
- **<code>decentralized_output</code>**: The output of the second round of the DKG process,
representing the decentralized computation result.
- **<code>dwallet_cap_id</code>**: The unique identifier of the <code>DWalletCap</code> associated with this session.
- **<code>dwallet_mpc_network_key_version</code>**: The version of the MPC network key for the <code>DWallet</code>.
- **<code>encrypted_secret_share_and_proof</code>**: The encrypted user secret key share and associated cryptographic proof.
- **<code>encryption_key_id</code>**: The ID of the <code>EncryptionKey</code> used for encrypting the secret key share.
- **<code>signed_public_share</code>**: The signed public share corresponding to the secret key share.
- **<code>encryptor_ed25519_pubkey</code>**: The Ed25519 public key of the entity that encrypted the secret key share.
- **<code>centralized_public_output</code>**: The centralized public output from the DKG process.


<a name="@Effects_25"></a>

##### Effects

- Creates a new <code>DWallet</code> object using the provided session ID, DKG outputs, and other metadata.
- Creates an encrypted user share and associates it with the <code>DWallet</code>.
- Emits a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedDKGSecondRoundEvent">CompletedDKGSecondRoundEvent</a></code> to record the completion of the second DKG round.
- Freezes the created <code>DWallet</code> object to make it immutable.


<a name="@Panics_26"></a>

##### Panics

- **<code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a></code>**: If the function is not called by the system address.


<pre><code><b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_dkg_second_round_output">create_dkg_second_round_output</a>(initiator: <b>address</b>, session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, decentralized_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, dwallet_mpc_network_key_version: u8, encrypted_secret_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, encryption_key_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, signed_public_share: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, encryptor_ed25519_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, centralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_dkg_second_round_output">create_dkg_second_round_output</a>(
    initiator: <b>address</b>,
    session_id: ID,
    decentralized_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    dwallet_cap_id: ID,
    dwallet_mpc_network_key_version: u8,
    encrypted_secret_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    encryption_key_id: ID,
    signed_public_share: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    encryptor_ed25519_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    centralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(<a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_SYSTEM_ADDRESS">SYSTEM_ADDRESS</a>, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a>);

    <b>let</b> <a href="dwallet.md#0x3_dwallet">dwallet</a> = <a href="dwallet.md#0x3_dwallet_create_dwallet">dwallet::create_dwallet</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;(
        session_id,
        dwallet_cap_id,
        decentralized_output,
        dwallet_mpc_network_key_version,
        centralized_public_output,
        ctx
    );

    <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_encrypted_user_share">create_encrypted_user_share</a>(<a href="../pera-framework/object.md#0x2_object_id">object::id</a>(&<a href="dwallet.md#0x3_dwallet">dwallet</a>),
        encrypted_secret_share_and_proof,
        encryption_key_id,
        session_id,
        signed_public_share,
        encryptor_ed25519_pubkey,
        initiator,
        ctx
    );

    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedDKGSecondRoundEvent">CompletedDKGSecondRoundEvent</a> {
        session_id,
        initiator,
        dwallet_cap_id,
        dwallet_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(&<a href="dwallet.md#0x3_dwallet">dwallet</a>),
        value: decentralized_output,
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


<a name="@Effects_27"></a>

##### Effects

- Associates the batched presign session with the specified dWallet.
- Emits a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartBatchedPresignEvent">StartBatchedPresignEvent</a></code> containing the batch session details.
- Emits a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartPresignFirstRoundEvent">StartPresignFirstRoundEvent</a></code> for each presign in the batch, with relevant details.


<a name="@Parameters_28"></a>

##### Parameters

- <code><a href="dwallet.md#0x3_dwallet">dwallet</a></code>: A reference to the target dWallet. This is used to retrieve the dWallet's ID and output.
- <code>batch_size</code>: The number of presign sessions to be created in this batch.
- <code>ctx</code>: The mutable transaction context, used to generate unique object IDs and retrieve the initiator.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_batched_presign">launch_batched_presign</a>(<a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">dwallet_2pc_mpc_ecdsa_k1::Secp256K1</a>&gt;, batch_size: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, _pera_system_state: &<a href="pera_system.md#0x3_pera_system_PeraSystemState">pera_system::PeraSystemState</a>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_batched_presign">launch_batched_presign</a>(
    <a href="dwallet.md#0x3_dwallet">dwallet</a>: &DWallet&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;,
    batch_size: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    _pera_system_state: &PeraSystemState,
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
            dkg_output: get_dwallet_decentralized_output&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>),
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


<a name="@Parameters_29"></a>

##### Parameters

- <code>initiator</code>: The address of the user initiating the presign session.
- <code>dwallet_id</code>: The ID of the associated dWallet.
- <code>dkg_output</code>: The output from the DKG process.
- <code>dwallet_cap_id</code>: The ID of the associated <code>DWalletCap</code>.
- <code>first_round_output</code>: The output from the first round of the presign process.
- <code>first_round_session_id</code>: The session ID of the first presign round.
- <code>ctx</code>: The transaction context used to emit the event.


<a name="@Panics_30"></a>

##### Panics

- Panics with <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a></code> if the sender of the transaction is not the system address.


<a name="@Emits_31"></a>

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


<a name="@Parameters_32"></a>

##### Parameters

- <code>initiator</code>: The address of the user who initiated the presign session.
- <code>session_id</code>: The ID of the presign session.
- <code>output</code>: The presign result data.
- <code>dwallet_cap_id</code>: The ID of the associated <code>DWalletCap</code>.
- <code>dwallet_id</code>: The ID of the associated <code>DWallet</code>.
- <code>ctx</code>: The transaction context.


<a name="@Emits_33"></a>

##### Emits

- <code>CompletedPresignEvent</code>: Includes the initiator, dWallet ID, and presign ID.


<a name="@Panics_34"></a>

##### Panics

- Panics with <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a></code> if the sender of the transaction is not the system address.


<a name="@Effects_35"></a>

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

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_sign"></a>

## Function `sign`

Initiates the signing process for a given dWallet.

This function emits a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartSignEvent">StartSignEvent</a></code>, providing all necessary
metadata and ensuring the integrity of the signing process.
It validates the linkage between the <code>DWallet</code>, <code>DWalletCap</code>, and <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a></code>.
It also "burns" the [<code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a></code>] object, by sending it to the system address,
as every presign can only be used to sign only one message.


<a name="@Effects_36"></a>

##### Effects

- Validates the linkage between dWallet components.
- Verifies that the number of <code>hashed_messages</code>, <code>message_approvals</code>, and
<code>centralized_signed_messages</code> are equal.
- Emits a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartSignEvent">StartSignEvent</a></code> with the hashed message, presign outputs,
and additional metadata.


<a name="@Emits_37"></a>

##### Emits

- <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartBatchedSignEvent">StartBatchedSignEvent</a></code>:
- Contains the session details and the list of hashed messages.
- <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartSignEvent">StartSignEvent</a></code>:
- Includes session details, hashed message, presign outputs,
and DKG output.


<a name="@Aborts_38"></a>

##### Aborts

- **<code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EDwalletMismatch">EDwalletMismatch</a></code>**: If the <code><a href="dwallet.md#0x3_dwallet">dwallet</a></code> object does not match the ID
in the <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a></code> object.
- **<code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EApprovalsAndMessagesLenMismatch">EApprovalsAndMessagesLenMismatch</a></code>**: If the length of the <code>hashed_messages</code>
does not match the length of the <code>message_approvals</code>.
- **<code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ECentralizedSignedMessagesAndMessagesLenMismatch">ECentralizedSignedMessagesAndMessagesLenMismatch</a></code>**: If the length of
<code>hashed_messages</code> does not match the length of <code>centralized_signed_messages</code>.
- **<code>EMessageApprovalDWalletMismatch</code>**: If the DWalletCap ID does not match
the expected DWalletCap ID for any of the message approvals.
- **<code>EMissingApprovalOrWrongApprovalOrder</code>**: If the approved messages are not
in the same order as the <code>hashed_messages</code>.


<a name="@Parameters_39"></a>

##### Parameters

- <code>dwallet_cap</code>: The capability associated with the dWallet.
- <code>hashed_messages</code>: The list of hashed messages to be signed.
- <code>message_approvals</code>: The approvals for the messages.
- <code>presign</code>: The presign object containing intermediate outputs.
- <code><a href="dwallet.md#0x3_dwallet">dwallet</a></code>: The dWallet object.
- <code>centralized_signed_messages</code>: The list of centralized signatures.
- <code>presign_session_id</code>: The session ID of the presign process.
- <code>ctx</code>: The mutable transaction context.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_sign">sign</a>(message_approvals: &<b>mut</b> <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet.md#0x3_dwallet_MessageApproval">dwallet::MessageApproval</a>&gt;, hashed_messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, presigns: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">dwallet_2pc_mpc_ecdsa_k1::Presign</a>&gt;, <a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">dwallet_2pc_mpc_ecdsa_k1::Secp256K1</a>&gt;, centralized_signed_messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, _pera_system_state: &<a href="pera_system.md#0x3_pera_system_PeraSystemState">pera_system::PeraSystemState</a>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_sign">sign</a>(
    message_approvals: &<b>mut</b> <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;MessageApproval&gt;,
    <b>mut</b> hashed_messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    <b>mut</b> presigns: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a>&gt;,
    <a href="dwallet.md#0x3_dwallet">dwallet</a>: &DWallet&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;,
    <b>mut</b> centralized_signed_messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    _pera_system_state: &PeraSystemState,
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
        pop_and_verify_message_approval(expected_dwallet_cap_id, message, message_approvals);
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
            dkg_output: get_dwallet_decentralized_output&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>),
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


<a name="@Parameters_40"></a>

##### Parameters

- **<code>signed_messages</code>**: A vector containing the signed message outputs.
- **<code>batch_session_id</code>**: The unique identifier for the batch signing session.
- **<code>ctx</code>**: The transaction context used for event emission.


<a name="@Requirements_41"></a>

##### Requirements

- The caller **must be the system address** (<code>@0x0</code>). If this condition is not met,
the function will abort with <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a></code>.


<a name="@Events_42"></a>

##### Events

- **<code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedSignEvent">CompletedSignEvent</a></code>**: Emitted with the <code>session_id</code> and <code>signed_messages</code>,
signaling the completion of the sign process for the batch session.


<a name="@Errors_43"></a>

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


<a name="@Parameters_44"></a>

##### Parameters

- <code>ctx</code>: The transaction context for generating IDs and objects.
- <code>dwallet_cap_id</code>: The ID of the capability associated with the mock dWallet.
- <code>dkg_output</code>: The decentralized DKG output.


<a name="@Effects_45"></a>

##### Effects

- Creates and initializes a <code>DWallet&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;</code> object.
- Links the dWallet to the provided capability.


<a name="@Returns_46"></a>

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
    <b>let</b> dwallet_mpc_network_key_version: u8 = 0;
    <a href="dwallet.md#0x3_dwallet_create_dwallet">dwallet::create_dwallet</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;(
        session_id,
        dwallet_cap_id,
        dkg_output,
        dwallet_mpc_network_key_version,
        <a href="../move-stdlib/vector.md#0x1_vector">vector</a>[],
        ctx
    )
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_create_mock_dwallet"></a>

## Function `create_mock_dwallet`

Created an immutable [<code>DWallet</code>] object with the given DKG output.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_mock_dwallet">create_mock_dwallet</a>(dkg_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, dkg_centralized_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_mock_dwallet">create_mock_dwallet</a>(
    dkg_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    dkg_centralized_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> dwallet_cap = create_dwallet_cap(ctx);
    <b>let</b> dwallet_cap_id = <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(&dwallet_cap);
    <a href="../pera-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(dwallet_cap, <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx));
    <b>let</b> session_id = <a href="../pera-framework/object.md#0x2_object_id_from_address">object::id_from_address</a>(<a href="../pera-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx));
    <b>let</b> dwallet_mpc_network_key_version: u8 = 0;
    <b>let</b> <a href="dwallet.md#0x3_dwallet">dwallet</a> = <a href="dwallet.md#0x3_dwallet_create_dwallet">dwallet::create_dwallet</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;(
        session_id,
        dwallet_cap_id,
        dkg_output,
        dwallet_mpc_network_key_version,
        dkg_centralized_output,
        ctx
    );
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


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_publish_partially_signed_messages">publish_partially_signed_messages</a>(signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, hashed_messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, presigns: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">dwallet_2pc_mpc_ecdsa_k1::Presign</a>&gt;, <a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">dwallet_2pc_mpc_ecdsa_k1::Secp256K1</a>&gt;, _pera_system_state: &<a href="pera_system.md#0x3_pera_system_PeraSystemState">pera_system::PeraSystemState</a>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_publish_partially_signed_messages">publish_partially_signed_messages</a>(
    signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    hashed_messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    <b>mut</b> presigns: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a>&gt;,
    <a href="dwallet.md#0x3_dwallet">dwallet</a>: &DWallet&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;,
    _pera_system_state: &PeraSystemState,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> messages_len = <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&hashed_messages);
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
            hashed_messages,
            presigns_bytes,
            get_dwallet_decentralized_output(<a href="dwallet.md#0x3_dwallet">dwallet</a>)
        ),
        <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EInvalidSignatures">EInvalidSignatures</a>
    );
    <b>let</b> partial_signatures = <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_PartiallySignedMessages">PartiallySignedMessages</a> {
        id: <a href="../pera-framework/object.md#0x2_object_new">object::new</a>(ctx),
        presigns: presigns_bytes,
        presign_session_ids,
        hashed_messages,
        signatures,
        dwallet_output: get_dwallet_decentralized_output(<a href="dwallet.md#0x3_dwallet">dwallet</a>),
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


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_future_sign">future_sign</a>(partial_signature: <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_PartiallySignedMessages">dwallet_2pc_mpc_ecdsa_k1::PartiallySignedMessages</a>, message_approvals: &<b>mut</b> <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet.md#0x3_dwallet_MessageApproval">dwallet::MessageApproval</a>&gt;, _pera_system_state: &<a href="pera_system.md#0x3_pera_system_PeraSystemState">pera_system::PeraSystemState</a>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_future_sign">future_sign</a>(
    partial_signature: <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_PartiallySignedMessages">PartiallySignedMessages</a>,
    message_approvals: &<b>mut</b> <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;MessageApproval&gt;,
    _pera_system_state: &PeraSystemState,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_PartiallySignedMessages">PartiallySignedMessages</a> {
        id,
        <b>mut</b> presigns,
        <b>mut</b> presign_session_ids,
        <b>mut</b> hashed_messages,
        <b>mut</b> signatures,
        dwallet_id,
        dwallet_cap_id,
        dwallet_output,
        dwallet_mpc_network_key_version,
    } = partial_signature;
    <a href="../pera-framework/object.md#0x2_object_delete">object::delete</a>(id);
    <b>let</b> message_approvals_len = <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(message_approvals);
    <b>let</b> messages_len = <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&hashed_messages);
    <b>assert</b>!(message_approvals_len == messages_len, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EApprovalsAndMessagesLenMismatch">EApprovalsAndMessagesLenMismatch</a>);
    <b>let</b> batch_session_id = <a href="../pera-framework/object.md#0x2_object_id_from_address">object::id_from_address</a>(<a href="../pera-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx));
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartBatchedSignEvent">StartBatchedSignEvent</a> {
        session_id: batch_session_id,
        hashed_messages,
        initiator: <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx)
    });
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; message_approvals_len) {
        <b>let</b> message = <a href="../move-stdlib/vector.md#0x1_vector_pop_back">vector::pop_back</a>(&<b>mut</b> hashed_messages);
        pop_and_verify_message_approval(dwallet_cap_id, message, message_approvals);
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
