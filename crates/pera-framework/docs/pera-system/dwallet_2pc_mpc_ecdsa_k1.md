---
title: Module `0x3::dwallet_2pc_mpc_ecdsa_k1`
---

This module handles the logic for creating and managing dWallets using the Secp256K1 signature scheme
and the DKG process. It leverages validators to execute MPC (Multi-Party Computation)
protocols to ensure trustless and decentralized wallet creation and key management.


-  [Struct `Secp256K1`](#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1)
-  [Struct `StartDKGFirstRoundEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGFirstRoundEvent)
-  [Struct `DKGFirstRoundOutputEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_DKGFirstRoundOutputEvent)
-  [Resource `DKGFirstRoundOutput`](#0x3_dwallet_2pc_mpc_ecdsa_k1_DKGFirstRoundOutput)
-  [Struct `StartDKGSecondRoundEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGSecondRoundEvent)
-  [Struct `CompletedDKGSecondRoundEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedDKGSecondRoundEvent)
-  [Resource `EncryptedUserSecretKeyShare`](#0x3_dwallet_2pc_mpc_ecdsa_k1_EncryptedUserSecretKeyShare)
-  [Struct `StartEncryptedShareVerificationEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_StartEncryptedShareVerificationEvent)
-  [Struct `CreatedEncryptedSecretShareEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_CreatedEncryptedSecretShareEvent)
-  [Resource `Presign`](#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign)
-  [Struct `StartBatchedPresignEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_StartBatchedPresignEvent)
-  [Struct `StartPresignFirstRoundEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_StartPresignFirstRoundEvent)
-  [Struct `StartPresignSecondRoundEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_StartPresignSecondRoundEvent)
-  [Struct `CompletedBatchedPresignEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedBatchedPresignEvent)
-  [Struct `AlgorithmSpecificData`](#0x3_dwallet_2pc_mpc_ecdsa_k1_AlgorithmSpecificData)
-  [Constants](#@Constants_0)
-  [Function `launch_dkg_first_round`](#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_dkg_first_round)
-  [Function `create_dkg_first_round_output`](#0x3_dwallet_2pc_mpc_ecdsa_k1_create_dkg_first_round_output)
-  [Function `launch_dkg_second_round`](#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_dkg_second_round)
-  [Function `transfer_encrypted_user_share`](#0x3_dwallet_2pc_mpc_ecdsa_k1_transfer_encrypted_user_share)
-  [Function `create_encrypted_user_share`](#0x3_dwallet_2pc_mpc_ecdsa_k1_create_encrypted_user_share)
-  [Function `create_dkg_second_round_output`](#0x3_dwallet_2pc_mpc_ecdsa_k1_create_dkg_second_round_output)
-  [Function `launch_batched_presign`](#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_batched_presign)
-  [Function `launch_presign_second_round`](#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_presign_second_round)
-  [Function `create_batched_presign_output`](#0x3_dwallet_2pc_mpc_ecdsa_k1_create_batched_presign_output)
-  [Function `create_signing_algorithm_data`](#0x3_dwallet_2pc_mpc_ecdsa_k1_create_signing_algorithm_data)
-  [Function `create_mock_dwallet_for_testing`](#0x3_dwallet_2pc_mpc_ecdsa_k1_create_mock_dwallet_for_testing)
-  [Function `create_mock_dwallet`](#0x3_dwallet_2pc_mpc_ecdsa_k1_create_mock_dwallet)
-  [Function `create_mock_presign`](#0x3_dwallet_2pc_mpc_ecdsa_k1_create_mock_presign)


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

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGFirstRoundEvent"></a>

## Struct `StartDKGFirstRoundEvent`

Event emitted to start the first round of the DKG process.

This event is caught by the blockchain, which is then using it to
initiate the first round of the DKG.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGFirstRoundEvent">StartDKGFirstRoundEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>session_id: <b>address</b></code>
</dt>
<dd>
 The unique session identifier for the DKG process.
</dd>
<dt>
<code>initiator: <b>address</b></code>
</dt>
<dd>
 The address of the user who initiated the DKG process.
</dd>
<dt>
<code>dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The identifier for the dWallet capability.
</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_DKGFirstRoundOutputEvent"></a>

## Struct `DKGFirstRoundOutputEvent`

An event emitted when the first round of the DKG process is completed.

This event is emitted by the blockchain to notify the user about
the completion of the first round.
The user should catch this event to generate inputs for
the second round and call the <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_dkg_second_round">launch_dkg_second_round</a>()</code> function.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_DKGFirstRoundOutputEvent">DKGFirstRoundOutputEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The unique session identifier for the DKG process.
</dd>
<dt>
<code>output_object_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The unique identifier of the output object created in the first round.
</dd>
<dt>
<code>decentralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The decentralized public output data produced by the first round of the DKG process.
</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_DKGFirstRoundOutput"></a>

## Resource `DKGFirstRoundOutput`

The output of the first round of the dWallet creation from the DKG process.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_DKGFirstRoundOutput">DKGFirstRoundOutput</a> <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../pera-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>
 A unique identifier for the DKG first round output.
</dd>
<dt>
<code>session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The unique session identifier for the DKG process.
</dd>
<dt>
<code>decentralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The decentralized public output data produced by the first round of the DKG process.
</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGSecondRoundEvent"></a>

## Struct `StartDKGSecondRoundEvent`

Event emitted to initiate the second round of the DKG process.

This event is emitted to notify Validators to begin the second round of the DKG.
It contains all necessary data to ensure proper continuation of the process.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGSecondRoundEvent">StartDKGSecondRoundEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>session_id: <b>address</b></code>
</dt>
<dd>
 The unique identifier for the DKG session.
</dd>
<dt>
<code>initiator: <b>address</b></code>
</dt>
<dd>
 The address of the user who initiated the dWallet creation.
</dd>
<dt>
<code>first_round_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The output from the first round of the DKG process.
</dd>
<dt>
<code>centralized_public_key_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 A serialized vector containing the centralized public key share and its proof.
</dd>
<dt>
<code>dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The unique identifier of the dWallet capability associated with this session.
</dd>
<dt>
<code>first_round_session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The session ID of the first round of the DKG process.
</dd>
<dt>
<code>encrypted_centralized_secret_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 Encrypted centralized secret key share and the associated cryptographic proof of encryption.
</dd>
<dt>
<code>encryption_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The <code>EncryptionKey</code> object used for encrypting the secret key share.
</dd>
<dt>
<code>encryption_key_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The unique identifier of the <code>EncryptionKey</code> object.
</dd>
<dt>
<code>centralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The public output of the centralized party in the DKG process.
</dd>
<dt>
<code>centralized_public_output_signature: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The signature for the public output of the centralized party in the DKG process.
</dd>
<dt>
<code>initiator_public_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The Ed25519 public key of the initiator,
 used to verify the signature on the centralized public output.
</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedDKGSecondRoundEvent"></a>

## Struct `CompletedDKGSecondRoundEvent`

Event emitted upon the completion of the second (and final) round of the
Distributed Key Generation (DKG).

This event provides all necessary data generated from the second
round of the DKG process.
Emitted to notify the centralized party.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedDKGSecondRoundEvent">CompletedDKGSecondRoundEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 A unique identifier for the DKG session.
</dd>
<dt>
<code>initiator: <b>address</b></code>
</dt>
<dd>
 The address of the user who initiated the DKG process.
</dd>
<dt>
<code>dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The unique identifier of the dWallet capability associated with the session.
</dd>
<dt>
<code>dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The identifier of the dWallet created as a result of the DKG process.
</dd>
<dt>
<code>decentralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The public decentralized output for the second round of the DKG process.
</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_EncryptedUserSecretKeyShare"></a>

## Resource `EncryptedUserSecretKeyShare`

A verified Encrypted dWallet centralized secret key share.

This struct represents an encrypted centralized secret key share tied to
a specific dWallet (<code>DWallet</code>).
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
<code>encrypted_centralized_secret_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The encrypted centralized secret key share along with a cryptographic proof
 that the encryption corresponds to the dWallet's secret key share.
</dd>
<dt>
<code>encryption_key_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The ID of the <code>EncryptionKey</code> object used to encrypt the secret share.
</dd>
<dt>
<code>centralized_public_output_signature: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
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
<code>encrypted_centralized_secret_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 Encrypted centralized secret key share and the associated cryptographic proof of encryption.
</dd>
<dt>
<code>centralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The public output of the centralized party,
 belongs to the dWallet that its centralized
 secret share is being encrypted.
 todo(zeev): we should not trust this, don't pass it.
</dd>
<dt>
<code>centralized_public_output_signature: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The signature of the dWallet <code>centralized_public_output</code>,
 signed by the secret key that corresponds to <code>encryptor_ed25519_pubkey</code>.
</dd>
<dt>
<code>dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The ID of the dWallet that this encrypted secret key share belongs to.
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
 A unique identifier for the session related to this operation.
</dd>
<dt>
<code>encryptor_ed25519_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 Public key of the entity that performed the encryption operation
 Used to verify the signature on the dWallet <code>centralized_public_output</code>.
 Note that the "encryptor" is the entity that performed the encryption,
 and the encryption can be done with another public key, this may not be
 the public key that was used for encryption.
</dd>
<dt>
<code>initiator: <b>address</b></code>
</dt>
<dd>
 The address of the entity that performed the encryption
 operation of this secret key share.
</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_CreatedEncryptedSecretShareEvent"></a>

## Struct `CreatedEncryptedSecretShareEvent`

Emitted when an encrypted share is created by the system transaction.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_CreatedEncryptedSecretShareEvent">CreatedEncryptedSecretShareEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 A unique identifier for the session related to this operation.
</dd>
<dt>
<code>encrypted_share_obj_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The ID of the <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a></code> Move object.
</dd>
<dt>
<code>dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The ID of the dWallet associated with this encrypted secret share.
</dd>
<dt>
<code>encrypted_centralized_secret_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The encrypted centralized secret key share along with a cryptographic proof
 that the encryption corresponds to the dWallet's secret key share.
</dd>
<dt>
<code>encryption_key_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The <code>EncryptionKey</code> Move object ID that was used to encrypt the secret key share.
</dd>
<dt>
<code>encryptor_address: <b>address</b></code>
</dt>
<dd>
 The address of the entity that performed the encryption operation of this secret key share.
</dd>
<dt>
<code>encryptor_ed25519_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 Public key of the entity that performed the encryption operation
 (with some encryption key — depends on the context)
 and signed the <code>centralized_public_output</code>.
 Used for verifications.
</dd>
<dt>
<code>centralized_public_output_signature: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 Signed dWallet public centralized output (signed by the <code>encryptor</code> entity).
</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_Presign"></a>

## Resource `Presign`

Represents the result of the second and final presign round.
This struct links the results of both presign rounds to a specific dWallet ID.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a> <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../pera-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>
 Unique identifier for the presign object.
</dd>
<dt>
<code>dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 ID of the associated dWallet.
</dd>
<dt>
<code>first_round_session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 Session ID for the first presign round.
</dd>
<dt>
<code>presign: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 Serialized output of the presign process.
</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_StartBatchedPresignEvent"></a>

## Struct `StartBatchedPresignEvent`

Event emitted to start a batched presign flow,
creating multiple presigns at once.

This event signals the initiation of a batch presign process,
where multiple presign
sessions are started simultaneously.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartBatchedPresignEvent">StartBatchedPresignEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The session identifier for the batched presign process.
</dd>
<dt>
<code>batch_size: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 The number of presign sessions to be started in this batch.
</dd>
<dt>
<code>initiator: <b>address</b></code>
</dt>
<dd>
 The address of the user who initiated the protocol.
</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_StartPresignFirstRoundEvent"></a>

## Struct `StartPresignFirstRoundEvent`

Event emitted to initiate the first round of a Presign session.

This event is used to signal Validators to start the
first round of the Presign process.
The event includes all necessary details to link
the session to the corresponding dWallet
and DKG process.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartPresignFirstRoundEvent">StartPresignFirstRoundEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 A unique identifier for the Presign session.
</dd>
<dt>
<code>initiator: <b>address</b></code>
</dt>
<dd>
 The address of the user who initiated the Presign session.
</dd>
<dt>
<code>dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The ID of the associated dWallet.
</dd>
<dt>
<code>dkg_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The output produced by the DKG process,
 used as input for the Presign session.
</dd>
<dt>
<code>batch_session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 A unique identifier for the Presign batch session.
</dd>
<dt>
<code>dwallet_mpc_network_decryption_key_version: u8</code>
</dt>
<dd>
 The MPC network decryption key version that is used to decrypt the associated dWallet.
</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_StartPresignSecondRoundEvent"></a>

## Struct `StartPresignSecondRoundEvent`

Event emitted to initiate the second round of a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a></code> session.

This event signals Validators to begin the second round of the Presign process.
The second round is a critical step in the multi-party computation (MPC) protocol,
enabling the generation of pre-signatures required for ECDSA signing.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartPresignSecondRoundEvent">StartPresignSecondRoundEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 A unique identifier for the current Presign session.
</dd>
<dt>
<code>initiator: <b>address</b></code>
</dt>
<dd>
 The address of the user who initiated the Presign session.
</dd>
<dt>
<code>dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The ID of the DWallet associated with this Presign session.
</dd>
<dt>
<code>dkg_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The output from the Distributed Key Generation (DKG) process,
 used as input for the Presign session.
</dd>
<dt>
<code>first_round_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The output generated from the first
 round of the Presign session.
</dd>
<dt>
<code>first_round_session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The session identifier for the first round of the Presign process.
</dd>
<dt>
<code>batch_session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 A unique identifier linking this session to a batched Presign process.
</dd>
<dt>
<code>dwallet_mpc_network_decryption_key_version: u8</code>
</dt>
<dd>
 The MPC network decryption key version that is used to decrypt the associated dWallet.
</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedBatchedPresignEvent"></a>

## Struct `CompletedBatchedPresignEvent`

Event emitted when the presign batch is completed.

This event indicates the successful completion of a batched presign process.
It provides details about the presign objects created and their associated metadata.


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
 The ID of the dWallet associated with this batch.
</dd>
<dt>
<code>session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The batch session ID.
</dd>
<dt>
<code>presign_ids: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>&gt;</code>
</dt>
<dd>
 The IDs of all the presign objects created in this batch.
 Each presign can be used to sign only one message.
</dd>
<dt>
<code>first_round_session_ids: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>&gt;</code>
</dt>
<dd>
 The first-round session IDs for each presign.
 The order of the session IDs corresponds to the order of the presigns.
 These IDs are needed for the centralized sign process.
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

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_AlgorithmSpecificData"></a>

## Struct `AlgorithmSpecificData`



<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_AlgorithmSpecificData">AlgorithmSpecificData</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>presign_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The presign object ID, the presign ID will be used as the sign MPC protocol ID.
</dd>
<dt>
<code>presign_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The presign protocol output as bytes.
</dd>
<dt>
<code>messages_centralized_signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>
 The centralized signatures for each message.
 The order of the signatures corresponds to the order of the messages.
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress"></a>

Error raised when the sender is not the system address.


<pre><code><b>const</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1;
</code></pre>



<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_SYSTEM_ADDRESS"></a>

System address for asserting system-level actions.


<pre><code><b>const</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_SYSTEM_ADDRESS">SYSTEM_ADDRESS</a>: <b>address</b> = 0;
</code></pre>



<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_EDwalletMismatch"></a>



<pre><code><b>const</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EDwalletMismatch">EDwalletMismatch</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 2;
</code></pre>



<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_launch_dkg_first_round"></a>

## Function `launch_dkg_first_round`

Starts the first Distributed Key Generation (DKG) session.

This function creates a new <code>DWalletCap</code> object,
transfers it to the session initiator,
and emits a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGFirstRoundEvent">StartDKGFirstRoundEvent</a></code> to signal
the beginning of the DKG process.


<a name="@Effects_1"></a>

##### Effects

- Generates a new <code>DWalletCap</code> object.
- Transfers the <code>DWalletCap</code> to the session initiator (<code>ctx.sender</code>).
- Emits a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGFirstRoundEvent">StartDKGFirstRoundEvent</a></code>.


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


<a name="@Effects_2"></a>

##### Effects

- Transfers the output of the first round to the initiator.
- Emits necessary metadata and links it to the associated session.


<a name="@Parameters_3"></a>

##### Parameters

- <code>initiator</code>: The address of the user who initiated the DKG session.
- <code>session_id</code>: The ID of the DKG session.
- <code>decentralized_public_output</code>: The public output data from the first round.
- <code>dwallet_cap_id</code>: The ID of the associated <code>DWalletCap</code>.
- <code>ctx</code>: The transaction context.


<a name="@Panics_4"></a>

##### Panics

- Panics with <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a></code> if the sender is not the system address.


<pre><code><b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_dkg_first_round_output">create_dkg_first_round_output</a>(session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, decentralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, initiator: <b>address</b>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_dkg_first_round_output">create_dkg_first_round_output</a>(
    session_id: ID,
    decentralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    initiator: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(<a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_SYSTEM_ADDRESS">SYSTEM_ADDRESS</a>, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a>);
    <b>let</b> dkg_output = <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_DKGFirstRoundOutput">DKGFirstRoundOutput</a> {
        session_id,
        id: <a href="../pera-framework/object.md#0x2_object_new">object::new</a>(ctx),
        decentralized_public_output,
    };
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_DKGFirstRoundOutputEvent">DKGFirstRoundOutputEvent</a> {
        session_id,
        output_object_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(&dkg_output),
        decentralized_public_output,
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

<a name="@Parameters_5"></a>

##### Parameters

- <code>dwallet_cap</code>: A reference to the <code>DWalletCap</code>, representing the capability associated with the dWallet.
- <code>centralized_public_key_share_and_proof</code>: The user (centralized) public key share and proof.
- <code>first_round_output</code>: A reference to the <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_DKGFirstRoundOutput">DKGFirstRoundOutput</a></code> structure containing the output of the first DKG round.
- <code>first_round_session_id</code>: The session ID associated with the first DKG round.
- <code>encrypted_centralized_secret_share_and_proof</code>: Encrypted centralized secret key share and its proof.
- <code>encryption_key</code>: The <code>EncryptionKey</code> object used for encrypting the secret key share.
- <code>centralized_public_output</code>: The public output of the centralized party in the DKG process.
- <code>centralized_public_output_signature</code>: The signature for the public output of the centralized party in the DKG process.
- <code>initiator_public_key</code>: The Ed25519 public key of the initiator,
used to verify the signature on the public output.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_dkg_second_round">launch_dkg_second_round</a>(dwallet_cap: &<a href="dwallet.md#0x3_dwallet_DWalletCap">dwallet::DWalletCap</a>, centralized_public_key_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, first_round_output: &<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_DKGFirstRoundOutput">dwallet_2pc_mpc_ecdsa_k1::DKGFirstRoundOutput</a>, first_round_session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, encrypted_centralized_secret_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, encryption_key: &<a href="dwallet.md#0x3_dwallet_EncryptionKey">dwallet::EncryptionKey</a>, centralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, centralized_public_output_signature: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, initiator_public_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_dkg_second_round">launch_dkg_second_round</a>(
    dwallet_cap: &DWalletCap,
    centralized_public_key_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    first_round_output: &<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_DKGFirstRoundOutput">DKGFirstRoundOutput</a>,
    first_round_session_id: ID,
    encrypted_centralized_secret_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    encryption_key: &EncryptionKey,
    centralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    centralized_public_output_signature: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    initiator_public_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext
): <b>address</b> {
    <b>let</b> session_id = <a href="../pera-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx);
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGSecondRoundEvent">StartDKGSecondRoundEvent</a> {
        session_id,
        initiator: <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx),
        first_round_output: first_round_output.decentralized_public_output,
        centralized_public_key_share_and_proof,
        dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(dwallet_cap),
        first_round_session_id,
        encrypted_centralized_secret_share_and_proof,
        encryption_key: get_encryption_key(encryption_key),
        encryption_key_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(encryption_key),
        centralized_public_output,
        centralized_public_output_signature,
        initiator_public_key,
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


<a name="@Parameters_6"></a>

##### Parameters

- **<code><a href="dwallet.md#0x3_dwallet">dwallet</a></code>**: A reference to the <code>DWallet&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;</code> object to which the secret share is linked.
- **<code>destination_encryption_key</code>**: A reference to the encryption key used for encrypting the secret key share.
- **<code>encrypted_secret_share_and_proof</code>**: The encrypted secret key share, accompanied by a cryptographic proof.
- **<code>source_signed_centralized_public_output</code>**: The signed centralized public output corresponding to the secret share.
- **<code>source_ed25519_pubkey</code>**: The Ed25519 public key of the source (encryptor) used for verifying the signature.


<a name="@Effects_7"></a>

##### Effects

- Emits a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartEncryptedShareVerificationEvent">StartEncryptedShareVerificationEvent</a></code>,
which is captured by the blockchain to initiate the verification process.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_transfer_encrypted_user_share">transfer_encrypted_user_share</a>(<a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">dwallet_2pc_mpc_ecdsa_k1::Secp256K1</a>&gt;, destination_encryption_key: &<a href="dwallet.md#0x3_dwallet_EncryptionKey">dwallet::EncryptionKey</a>, encrypted_centralized_secret_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, source_centralized_public_output_signature: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, source_ed25519_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_transfer_encrypted_user_share">transfer_encrypted_user_share</a>(
    <a href="dwallet.md#0x3_dwallet">dwallet</a>: &DWallet&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;,
    destination_encryption_key: &EncryptionKey,
    encrypted_centralized_secret_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    source_centralized_public_output_signature: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    source_ed25519_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> session_id = <a href="../pera-framework/object.md#0x2_object_id_from_address">object::id_from_address</a>(<a href="../pera-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx));
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartEncryptedShareVerificationEvent">StartEncryptedShareVerificationEvent</a> {
        encrypted_centralized_secret_share_and_proof,
        centralized_public_output: get_dwallet_centralized_public_output&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>),
        dwallet_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(<a href="dwallet.md#0x3_dwallet">dwallet</a>),
        encryption_key: get_encryption_key(destination_encryption_key),
        encryption_key_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(destination_encryption_key),
        session_id,
        centralized_public_output_signature: source_centralized_public_output_signature,
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


<a name="@Parameters_8"></a>

##### Parameters

- <code>dwallet_id</code>: The unique identifier of the dWallet associated with the encrypted user share.
- <code>encrypted_centralized_secret_share_and_proof</code>: The encrypted centralized secret key share along with its cryptographic proof.
- <code>encryption_key_id</code>: The <code>EncryptionKey</code> Move object ID used to encrypt the secret key share.
- <code>session_id</code>: A unique identifier for the session related to this operation.
- <code>centralized_public_output_signature</code>: The signed public share corresponding to the encrypted secret share.
- <code>encryptor_ed25519_pubkey</code>: The Ed25519 public key of the encryptor used for signing.
- <code>initiator</code>: The address of the entity that performed the encryption operation of this secret key share.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_encrypted_user_share">create_encrypted_user_share</a>(dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, encrypted_centralized_secret_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, encryption_key_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, centralized_public_output_signature: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, encryptor_ed25519_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, initiator: <b>address</b>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_encrypted_user_share">create_encrypted_user_share</a>(
    dwallet_id: ID,
    encrypted_centralized_secret_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    encryption_key_id: ID,
    session_id: ID,
    centralized_public_output_signature: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    encryptor_ed25519_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    initiator: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(<a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_SYSTEM_ADDRESS">SYSTEM_ADDRESS</a>, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a>);

    <b>let</b> encrypted_user_share = <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a> {
        id: <a href="../pera-framework/object.md#0x2_object_new">object::new</a>(ctx),
        dwallet_id,
        encrypted_centralized_secret_share_and_proof,
        encryption_key_id,
        centralized_public_output_signature,
        encryptor_ed25519_pubkey,
        encryptor_address: initiator,
    };
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_CreatedEncryptedSecretShareEvent">CreatedEncryptedSecretShareEvent</a> {
        session_id,
        encrypted_share_obj_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(&encrypted_user_share),
        dwallet_id,
        encrypted_centralized_secret_share_and_proof,
        encryption_key_id,
        centralized_public_output_signature,
        encryptor_ed25519_pubkey,
        encryptor_address: initiator,
    });
    // TODO (#527): Transfer the encrypted user share <b>move</b> <a href="../pera-framework/object.md#0x2_object">object</a> <b>to</b> the destination
    // TODO (#527): <b>address</b> instead of the initiating user.
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


<a name="@Parameters_9"></a>

##### Parameters

- **<code>initiator</code>**: The address of the user who initiated the DKG session.
- **<code>session_id</code>**: A unique identifier for the current DKG session.
- **<code>decentralized_public_output</code>**: The public output of the second round of the DKG process,
representing the decentralized computation result.
- **<code>dwallet_cap_id</code>**: The unique identifier of the <code>DWalletCap</code> associated with this session.
- **<code>dwallet_mpc_network_decryption_key_version</code>**: The version of the MPC network key for the <code>DWallet</code>.
- **<code>encrypted_secret_share_and_proof</code>**: The encrypted user secret key share and associated cryptographic proof.
- **<code>encryption_key_id</code>**: The ID of the <code>EncryptionKey</code> used for encrypting the secret key share.
- **<code>signed_public_share</code>**: The signed public share corresponding to the secret key share.
- **<code>encryptor_ed25519_pubkey</code>**: The Ed25519 public key of the entity that encrypted the secret key share.
- **<code>centralized_public_output</code>**: The centralized public output from the DKG process.


<a name="@Effects_10"></a>

##### Effects

- Creates a new <code>DWallet</code> object using the provided session ID, DKG outputs, and other metadata.
- Creates an encrypted user share and associates it with the <code>DWallet</code>.
- Emits a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedDKGSecondRoundEvent">CompletedDKGSecondRoundEvent</a></code> to record the completion of the second DKG round.
- Freezes the created <code>DWallet</code> object to make it immutable.


<a name="@Panics_11"></a>

##### Panics

- **<code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a></code>**: If the function is not called by the system address.


<pre><code><b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_dkg_second_round_output">create_dkg_second_round_output</a>(initiator: <b>address</b>, session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, decentralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, dwallet_mpc_network_decryption_key_version: u8, encrypted_secret_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, encryption_key_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, signed_public_share: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, encryptor_ed25519_pubkey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, centralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_dkg_second_round_output">create_dkg_second_round_output</a>(
    initiator: <b>address</b>,
    session_id: ID,
    decentralized_public_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    dwallet_cap_id: ID,
    dwallet_mpc_network_decryption_key_version: u8,
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
        decentralized_public_output,
        dwallet_mpc_network_decryption_key_version,
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
        decentralized_public_output,
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


<a name="@Effects_12"></a>

##### Effects

- Associates the batched presign session with the specified dWallet.
- Emits a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartBatchedPresignEvent">StartBatchedPresignEvent</a></code> containing the batch session details.
- Emits a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartPresignFirstRoundEvent">StartPresignFirstRoundEvent</a></code> for each presign in the batch, with relevant details.


<a name="@Parameters_13"></a>

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
            dkg_output: get_dwallet_decentralized_public_output&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>),
            batch_session_id,
            dwallet_mpc_network_decryption_key_version: get_dwallet_mpc_network_decryption_key_version(<a href="dwallet.md#0x3_dwallet">dwallet</a>),
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


<a name="@Parameters_14"></a>

##### Parameters

- <code>initiator</code>: The address of the user initiating the presign session.
- <code>dwallet_id</code>: The ID of the associated dWallet.
- <code>dkg_output</code>: The output from the DKG process.
- <code>dwallet_cap_id</code>: The ID of the associated <code>DWalletCap</code>.
- <code>first_round_output</code>: The output from the first round of the presign process.
- <code>first_round_session_id</code>: The session ID of the first presign round.
- <code>ctx</code>: The transaction context used to emit the event.


<a name="@Panics_15"></a>

##### Panics

- Panics with <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a></code> if the sender of the transaction is not the system address.


<a name="@Emits_16"></a>

##### Emits

- <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartPresignSecondRoundEvent">StartPresignSecondRoundEvent</a></code>: Includes session ID, initiator address, dWallet ID, dWallet capability ID,
DKG output, first round output, and first round session ID.


<pre><code><b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_presign_second_round">launch_presign_second_round</a>(initiator: <b>address</b>, dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, dkg_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, first_round_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, first_round_session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, batch_session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, dwallet_mpc_network_decryption_key_version: u8, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
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
    dwallet_mpc_network_decryption_key_version: u8,
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
        dwallet_mpc_network_decryption_key_version,
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


<a name="@Parameters_17"></a>

##### Parameters

- <code>initiator</code>: The address of the user who initiated the presign session.
- <code>session_id</code>: The ID of the presign session.
- <code>output</code>: The presign result data.
- <code>dwallet_cap_id</code>: The ID of the associated <code>DWalletCap</code>.
- <code>dwallet_id</code>: The ID of the associated <code>DWallet</code>.
- <code>ctx</code>: The transaction context.


<a name="@Emits_18"></a>

##### Emits

- <code>CompletedPresignEvent</code>: Includes the initiator, dWallet ID, and presign ID.


<a name="@Panics_19"></a>

##### Panics

- Panics with <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a></code> if the sender of the transaction is not the system address.


<a name="@Effects_20"></a>

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

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_create_signing_algorithm_data"></a>

## Function `create_signing_algorithm_data`

Creates a vector of <code>SigningAlgorithmData</code> objects from a vector of <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a></code> objects.

This function constructs the necessary data structures for the signing process using ECDSA K1.
It takes a vector of <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a></code> objects, extracts the relevant data, and removes the original objects.
Additionally, it ensures that the <code>DWallet</code> associated with the <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a></code> objects matches the provided <code>DWallet</code>.

The function returns a vector of <code>SigningAlgorithmData</code> objects, which are essential for the signing process.
The returned value must be used in a PTB; otherwise, the transaction will fail due to the "Hot Potato" pattern.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_signing_algorithm_data">create_signing_algorithm_data</a>(presigns: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">dwallet_2pc_mpc_ecdsa_k1::Presign</a>&gt;, messages_centralized_signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, <a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">dwallet_2pc_mpc_ecdsa_k1::Secp256K1</a>&gt;): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet.md#0x3_dwallet_SigningAlgorithmData">dwallet::SigningAlgorithmData</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_AlgorithmSpecificData">dwallet_2pc_mpc_ecdsa_k1::AlgorithmSpecificData</a>&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_signing_algorithm_data">create_signing_algorithm_data</a>(
    presigns: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a>&gt;,
    <b>mut</b> messages_centralized_signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    <a href="dwallet.md#0x3_dwallet">dwallet</a>: &DWallet&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;,
): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;SigningAlgorithmData&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_AlgorithmSpecificData">AlgorithmSpecificData</a>&gt;&gt; {
    vector::map!(presigns, |presign| {
        <b>let</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a> {id, presign, first_round_session_id, dwallet_id} = presign;
        <a href="../move-stdlib/vector.md#0x1_vector_reverse">vector::reverse</a>(&<b>mut</b> messages_centralized_signatures);
        <b>assert</b>!(<a href="../pera-framework/object.md#0x2_object_id">object::id</a>(<a href="dwallet.md#0x3_dwallet">dwallet</a>) == dwallet_id, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EDwalletMismatch">EDwalletMismatch</a>);
        <b>let</b> extra_data_per_sign = <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_AlgorithmSpecificData">AlgorithmSpecificData</a> {
            presign_id: first_round_session_id,
            presign_output: presign,
            messages_centralized_signatures,
        };
        <a href="../pera-framework/object.md#0x2_object_delete">object::delete</a>(id);
        <a href="dwallet.md#0x3_dwallet_create_signing_algorithm_data">dwallet::create_signing_algorithm_data</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_AlgorithmSpecificData">AlgorithmSpecificData</a>&gt;(extra_data_per_sign)
    })
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_create_mock_dwallet_for_testing"></a>

## Function `create_mock_dwallet_for_testing`

Generates a mock <code>DWallet&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;</code> object for testing purposes.

This function creates a dWallet object with random data,
useful for testing or initialization in non-production environments.


<a name="@Parameters_21"></a>

##### Parameters

- <code>ctx</code>: The transaction context for generating IDs and objects.
- <code>dwallet_cap_id</code>: The ID of the capability associated with the mock dWallet.
- <code>dkg_output</code>: The decentralized DKG output.


<a name="@Effects_22"></a>

##### Effects

- Creates and initializes a <code>DWallet&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;</code> object.
- Links the dWallet to the provided capability.


<a name="@Returns_23"></a>

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
    <b>let</b> dwallet_mpc_network_decryption_key_version: u8 = 0;
    <a href="dwallet.md#0x3_dwallet_create_dwallet">dwallet::create_dwallet</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;(
        session_id,
        dwallet_cap_id,
        dkg_output,
        dwallet_mpc_network_decryption_key_version,
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
    <b>let</b> dwallet_mpc_network_decryption_key_version: u8 = 0;
    <b>let</b> <a href="dwallet.md#0x3_dwallet">dwallet</a> = <a href="dwallet.md#0x3_dwallet_create_dwallet">dwallet::create_dwallet</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;(
        session_id,
        dwallet_cap_id,
        dkg_output,
        dwallet_mpc_network_decryption_key_version,
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
