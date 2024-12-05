---
title: Module `0x3::dwallet_2pc_mpc_ecdsa_k1`
---



-  [Struct `MessageApproval`](#0x3_dwallet_2pc_mpc_ecdsa_k1_MessageApproval)
-  [Struct `Secp256K1`](#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1)
-  [Resource `DKGFirstRoundOutput`](#0x3_dwallet_2pc_mpc_ecdsa_k1_DKGFirstRoundOutput)
-  [Struct `StartDKGFirstRoundEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGFirstRoundEvent)
-  [Struct `CompletedSignEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedSignEvent)
-  [Struct `StartDKGSecondRoundEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGSecondRoundEvent)
-  [Struct `CompletedDKGSecondRoundEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedDKGSecondRoundEvent)
-  [Struct `StartPresignFirstRoundEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_StartPresignFirstRoundEvent)
-  [Struct `StartPresignSecondRoundEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_StartPresignSecondRoundEvent)
-  [Resource `Presign`](#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign)
-  [Struct `CompletedPresignEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedPresignEvent)
-  [Struct `StartBatchedSignEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_StartBatchedSignEvent)
-  [Struct `StartSignEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_StartSignEvent)
-  [Resource `SignOutput`](#0x3_dwallet_2pc_mpc_ecdsa_k1_SignOutput)
-  [Constants](#@Constants_0)
-  [Function `launch_dkg_first_round`](#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_dkg_first_round)
-  [Function `create_dkg_first_round_output`](#0x3_dwallet_2pc_mpc_ecdsa_k1_create_dkg_first_round_output)
-  [Function `launch_dkg_second_round`](#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_dkg_second_round)
-  [Function `approve_messages`](#0x3_dwallet_2pc_mpc_ecdsa_k1_approve_messages)
-  [Function `create_dkg_second_round_output`](#0x3_dwallet_2pc_mpc_ecdsa_k1_create_dkg_second_round_output)
-  [Function `launch_presign_first_round`](#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_presign_first_round)
-  [Function `launch_presign_second_round`](#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_presign_second_round)
-  [Function `create_second_presign_round_output`](#0x3_dwallet_2pc_mpc_ecdsa_k1_create_second_presign_round_output)
-  [Function `remove_message_approval`](#0x3_dwallet_2pc_mpc_ecdsa_k1_remove_message_approval)
-  [Function `sign`](#0x3_dwallet_2pc_mpc_ecdsa_k1_sign)
-  [Function `mock_sign`](#0x3_dwallet_2pc_mpc_ecdsa_k1_mock_sign)
-  [Function `create_sign_output`](#0x3_dwallet_2pc_mpc_ecdsa_k1_create_sign_output)


<pre><code><b>use</b> <a href="../pera-framework/event.md#0x2_event">0x2::event</a>;
<b>use</b> <a href="../pera-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../pera-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../pera-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="dwallet.md#0x3_dwallet">0x3::dwallet</a>;
</code></pre>



<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_MessageApproval"></a>

## Struct `MessageApproval`

<code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_MessageApproval">MessageApproval</a></code> represents a message that was approved.
Bound to a <code>DWalletCap</code>.


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

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1"></a>

## Struct `Secp256K1`

Represents the Secp256K1 DWallet type.


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

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_DKGFirstRoundOutput"></a>

## Resource `DKGFirstRoundOutput`

A struct to hold the output of the first round of the DKG.
An instance of this struct is being transferred to the user that initiated the DKG after
the first round is completed.
The user can then use this output to start the second round of the DKG.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_DKGFirstRoundOutput">DKGFirstRoundOutput</a> <b>has</b> key
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
<dt>
<code>dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGFirstRoundEvent"></a>

## Struct `StartDKGFirstRoundEvent`

Event to start a <code>DKG</code> session, caught by the Validators.


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
<code>sender: <b>address</b></code>
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

Event that indicates the completion of a Sign flow and includes signatures on all the messages.


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

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGSecondRoundEvent"></a>

## Struct `StartDKGSecondRoundEvent`

Event emitted to start the validator's second DKG round.
Each validator catches this event and starts the second round of the DKG.


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
<code>sender: <b>address</b></code>
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

Event emitted when the second round of the DKG is completed,
containing all relevant data from that round.


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
<code>sender: <b>address</b></code>
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

Event emitted to initiate a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a></code> session, caught by the Validators.


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
<code>sender: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>dkg_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_StartPresignSecondRoundEvent"></a>

## Struct `StartPresignSecondRoundEvent`

Event emitted to initiate the second round of a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a></code> session, caught by the Validators.


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
<code>sender: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
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
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_Presign"></a>

## Resource `Presign`

Represents the presign result of a the second and final presign round.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a> <b>has</b> key
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
<code>dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
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
<code>first_round_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>second_round_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedPresignEvent"></a>

## Struct `CompletedPresignEvent`

Event emitted when the presign second round is completed.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedPresignEvent">CompletedPresignEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>sender: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>presign_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_StartBatchedSignEvent"></a>

## Struct `StartBatchedSignEvent`



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
<code>initiating_user: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_StartSignEvent"></a>

## Struct `StartSignEvent`

Event emitted by the user to start the signing process.


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
<code>sender: <b>address</b></code>
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
<code>dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
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
<code>presign_first_round_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>presign_second_round_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>centralized_signed_message: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_SignOutput"></a>

## Resource `SignOutput`

Object representing the output of the signing process.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_SignOutput">SignOutput</a> <b>has</b> key
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

<a name="@Constants_0"></a>

## Constants


<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress"></a>



<pre><code><b>const</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
</code></pre>



<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_EMesssageApprovalDWalletMismatch"></a>



<pre><code><b>const</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EMesssageApprovalDWalletMismatch">EMesssageApprovalDWalletMismatch</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1;
</code></pre>



<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_SYSTEM_ADDRESS"></a>



<pre><code><b>const</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_SYSTEM_ADDRESS">SYSTEM_ADDRESS</a>: <b>address</b> = 0;
</code></pre>



<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_launch_dkg_first_round"></a>

## Function `launch_dkg_first_round`

Starts the first Distributed Key Generation (DKG) session. Two MPC sessions are required to
create a Dwallet.
Capabilities are used to control access to the Dwallet.
This function starts the DKG process within Validators.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_dkg_first_round">launch_dkg_first_round</a>(ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_dkg_first_round">launch_dkg_first_round</a>(
    ctx: &<b>mut</b> TxContext
): <b>address</b> {
    <b>let</b> cap_id = create_dwallet_cap(ctx);
    <b>let</b> sender = <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx);
    <b>let</b> session_id = <a href="../pera-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx);
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGFirstRoundEvent">StartDKGFirstRoundEvent</a> {
        session_id,
        sender,
        dwallet_cap_id: cap_id,
    });
    session_id
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_create_dkg_first_round_output"></a>

## Function `create_dkg_first_round_output`

Creates the output of the first round of the DKG MPC, transferring it to the initiating user.
This function is called by the blockchain itself.
Validators call it as part of the blockchain logic.


<pre><code><b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_dkg_first_round_output">create_dkg_first_round_output</a>(sender: <b>address</b>, session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_dkg_first_round_output">create_dkg_first_round_output</a>(
    sender: <b>address</b>,
    session_id: ID,
    output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    dwallet_cap_id: ID,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(<a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_SYSTEM_ADDRESS">SYSTEM_ADDRESS</a>, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a>);
    <b>let</b> output = <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_DKGFirstRoundOutput">DKGFirstRoundOutput</a> {
        id: <a href="../pera-framework/object.md#0x2_object_new">object::new</a>(ctx),
        session_id,
        output,
        dwallet_cap_id,
    };
    <a href="../pera-framework/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(output, sender);
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_launch_dkg_second_round"></a>

## Function `launch_dkg_second_round`

Launches the second DKG round, emitting an event with all required data.
Each validator catches this event and starts the second round of the DKG.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_dkg_second_round">launch_dkg_second_round</a>(dwallet_cap: &<a href="dwallet.md#0x3_dwallet_DWalletCap">dwallet::DWalletCap</a>, public_key_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, first_round_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, first_round_session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_dkg_second_round">launch_dkg_second_round</a>(
    dwallet_cap: &DWalletCap,
    public_key_share_and_proof: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    first_round_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    first_round_session_id: ID,
    ctx: &<b>mut</b> TxContext
): <b>address</b> {
    <b>let</b> session_id = <a href="../pera-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx);
    <b>let</b> created_proof_mpc_session_event = <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartDKGSecondRoundEvent">StartDKGSecondRoundEvent</a> {
        session_id,
        sender: <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx),
        first_round_output,
        public_key_share_and_proof,
        dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(dwallet_cap),
        first_round_session_id
    };
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(created_proof_mpc_session_event);
    session_id
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_approve_messages"></a>

## Function `approve_messages`

Create a set of message approvals.
The messages must be approved in the same order as they were created.
The messages must be approved by the same <code>dwallet_cap_id</code>.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_approve_messages">approve_messages</a>(dwallet_cap: &<a href="dwallet.md#0x3_dwallet_DWalletCap">dwallet::DWalletCap</a>, messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_MessageApproval">dwallet_2pc_mpc_ecdsa_k1::MessageApproval</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_approve_messages">approve_messages</a>(
    dwallet_cap: &DWalletCap,
    <b>mut</b> messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;
): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_MessageApproval">MessageApproval</a>&gt; {
    <b>let</b> dwallet_cap_id = <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(dwallet_cap);
    <b>let</b> <b>mut</b> message_approvals = <a href="../move-stdlib/vector.md#0x1_vector_empty">vector::empty</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_MessageApproval">MessageApproval</a>&gt;();
    <b>while</b> (<a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&messages) &gt; 0) {
        <b>let</b> message = <a href="../move-stdlib/vector.md#0x1_vector_pop_back">vector::pop_back</a>(&<b>mut</b> messages);
        <a href="../move-stdlib/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> message_approvals, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_MessageApproval">MessageApproval</a> {
            dwallet_cap_id,
            message,
        });
    };
    message_approvals
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_create_dkg_second_round_output"></a>

## Function `create_dkg_second_round_output`

Completes the second DKG MPC round by creating the actual [<code><a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a></code>].
This function is called by the blockchain itself.
Validators call it as part of the blockchain logic.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_dkg_second_round_output">create_dkg_second_round_output</a>(session_initiator: <b>address</b>, session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_dkg_second_round_output">create_dkg_second_round_output</a>(
    session_initiator: <b>address</b>,
    session_id: ID,
    output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    dwallet_cap_id: ID,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(<a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == @0x0, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a>);
    <b>let</b> <a href="dwallet.md#0x3_dwallet">dwallet</a> = <a href="dwallet.md#0x3_dwallet_create_dwallet">dwallet::create_dwallet</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;(session_id, dwallet_cap_id, output, ctx);
    <b>let</b> completed_proof_mpc_session_event = <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedDKGSecondRoundEvent">CompletedDKGSecondRoundEvent</a> {
        session_id,
        sender: session_initiator,
        dwallet_cap_id,
        dwallet_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(&<a href="dwallet.md#0x3_dwallet">dwallet</a>),
        value: output,
    };

    <a href="../pera-framework/transfer.md#0x2_transfer_public_freeze_object">transfer::public_freeze_object</a>(<a href="dwallet.md#0x3_dwallet">dwallet</a>);
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(completed_proof_mpc_session_event);
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_launch_presign_first_round"></a>

## Function `launch_presign_first_round`

Starts the first round of the presign session.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_presign_first_round">launch_presign_first_round</a>(<a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">dwallet_2pc_mpc_ecdsa_k1::Secp256K1</a>&gt;, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_presign_first_round">launch_presign_first_round</a>(
    <a href="dwallet.md#0x3_dwallet">dwallet</a>: &DWallet&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> session_id = <a href="../pera-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx);
    <b>let</b> <a href="../pera-framework/event.md#0x2_event">event</a> = <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartPresignFirstRoundEvent">StartPresignFirstRoundEvent</a> {
        session_id: <a href="../pera-framework/object.md#0x2_object_id_from_address">object::id_from_address</a>(session_id),
        sender: <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx),
        dwallet_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(<a href="dwallet.md#0x3_dwallet">dwallet</a>),
        dwallet_cap_id: get_dwallet_cap_id&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>),
        dkg_output: get_dwallet_output&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>),
    };
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="../pera-framework/event.md#0x2_event">event</a>);
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_launch_presign_second_round"></a>

## Function `launch_presign_second_round`

Launches the second round of the presign session.
Validators catch this event and begin processing for the second round.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_presign_second_round">launch_presign_second_round</a>(session_initiator: <b>address</b>, dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, dkg_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, first_round_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, first_round_session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_launch_presign_second_round">launch_presign_second_round</a>(
    session_initiator: <b>address</b>,
    dwallet_id: ID,
    dkg_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    dwallet_cap_id: ID,
    first_round_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    first_round_session_id: ID,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(<a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == @0x0, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a>);

    <b>let</b> session_id = <a href="../pera-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx);
    <b>let</b> session_id = <a href="../pera-framework/object.md#0x2_object_id_from_address">object::id_from_address</a>(session_id);

    <b>let</b> <a href="../pera-framework/event.md#0x2_event">event</a> = <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartPresignSecondRoundEvent">StartPresignSecondRoundEvent</a> {
        session_id,
        sender: session_initiator,
        dwallet_id,
        dwallet_cap_id,
        dkg_output,
        first_round_output: first_round_output,
        first_round_session_id: first_round_session_id,
    };
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="../pera-framework/event.md#0x2_event">event</a>);
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_create_second_presign_round_output"></a>

## Function `create_second_presign_round_output`

Completes the presign session by creating the output of the second presign round
and transferring it to the session initiator.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_second_presign_round_output">create_second_presign_round_output</a>(session_initiator: <b>address</b>, session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, first_round_session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, first_round_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, second_round_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, dwallet_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_second_presign_round_output">create_second_presign_round_output</a>(
    session_initiator: <b>address</b>,
    session_id: ID,
    first_round_session_id: ID,
    first_round_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    second_round_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    dwallet_cap_id: ID,
    dwallet_id: ID,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(<a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == @0x0, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a>);

    <b>let</b> output = <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a> {
        id: <a href="../pera-framework/object.md#0x2_object_new">object::new</a>(ctx),
        session_id,
        first_round_session_id,
        dwallet_id,
        dwallet_cap_id,
        first_round_output,
        second_round_output,
    };

    <b>let</b> <a href="../pera-framework/event.md#0x2_event">event</a> = <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedPresignEvent">CompletedPresignEvent</a> {
        sender: session_initiator,
        dwallet_id,
        presign_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(&output),
    };

    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="../pera-framework/event.md#0x2_event">event</a>);
    <a href="../pera-framework/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(output, session_initiator);
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_remove_message_approval"></a>

## Function `remove_message_approval`

Remove a <code><a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_MessageApproval">MessageApproval</a></code> and return the <code>dwallet_cap_id</code> and the <code>message</code>.


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

Starts the signing process.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_sign">sign</a>(dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, message_approvals: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_MessageApproval">dwallet_2pc_mpc_ecdsa_k1::MessageApproval</a>&gt;, hashed_messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, presign: &<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">dwallet_2pc_mpc_ecdsa_k1::Presign</a>, <a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">dwallet_2pc_mpc_ecdsa_k1::Secp256K1</a>&gt;, centralized_signed_messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, presign_session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_sign">sign</a>(
    dwallet_cap_id: ID,
    <b>mut</b> message_approvals: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_MessageApproval">MessageApproval</a>&gt;,
    hashed_messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    presign: &<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Presign">Presign</a>,
    <a href="dwallet.md#0x3_dwallet">dwallet</a>: &DWallet&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;,
    centralized_signed_messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    presign_session_id: ID,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> messages_len: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&hashed_messages);
    <b>let</b> approval_len: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&message_approvals);
    <b>assert</b>!(messages_len == approval_len, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EMesssageApprovalDWalletMismatch">EMesssageApprovalDWalletMismatch</a>);

    <b>let</b> <b>mut</b> i: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
    <b>while</b> (i &lt; messages_len) {
        <b>let</b> message_approval = <a href="../move-stdlib/vector.md#0x1_vector_pop_back">vector::pop_back</a>(&<b>mut</b> message_approvals);
        <b>let</b> (message_approval_dwallet_cap_id, approved_message) = <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_remove_message_approval">remove_message_approval</a>(message_approval);
        <b>assert</b>!(get_dwallet_cap_id(<a href="dwallet.md#0x3_dwallet">dwallet</a>) == message_approval_dwallet_cap_id, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EMesssageApprovalDWalletMismatch">EMesssageApprovalDWalletMismatch</a>);
        <b>let</b> message = <a href="../move-stdlib/vector.md#0x1_vector_borrow">vector::borrow</a>(&hashed_messages, i);
        <b>assert</b>!(message == &approved_message, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_EMesssageApprovalDWalletMismatch">EMesssageApprovalDWalletMismatch</a>);
        i = i + 1;
    };

    <b>let</b> batch_session_id = <a href="../pera-framework/object.md#0x2_object_id_from_address">object::id_from_address</a>(<a href="../pera-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx));
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartBatchedSignEvent">StartBatchedSignEvent</a> {
        session_id: batch_session_id,
        hashed_messages,
        initiating_user: <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx)
    });
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> messages_length = <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&hashed_messages);
    <b>while</b> (i &lt; messages_length) {
        <b>let</b> id = <a href="../pera-framework/object.md#0x2_object_id_from_address">object::id_from_address</a>(<a href="../pera-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx));
        <b>let</b> <a href="../pera-framework/event.md#0x2_event">event</a> = <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartSignEvent">StartSignEvent</a> {
            session_id: id,
            presign_session_id,
            sender: <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx),
            batched_session_id: batch_session_id,
            dwallet_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(<a href="dwallet.md#0x3_dwallet">dwallet</a>),
            dwallet_cap_id,
            presign_first_round_output: presign.first_round_output,
            presign_second_round_output: presign.second_round_output,
            centralized_signed_message: centralized_signed_messages[i],
            dkg_output: get_dwallet_output&lt;<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_Secp256K1">Secp256K1</a>&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>),
            hashed_message: hashed_messages[i],
        };
        <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="../pera-framework/event.md#0x2_event">event</a>);
        i = i + 1;
    };
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_mock_sign"></a>

## Function `mock_sign`

Todo: Remove this function


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_mock_sign">mock_sign</a>(hashed_messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, presign_first_round_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, presign_second_round_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, dkg_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, centralized_signed_messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, presign_session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_mock_sign">mock_sign</a>(
    hashed_messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    presign_first_round_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    presign_second_round_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    dkg_output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    centralized_signed_messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    presign_session_id: ID,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> batch_session_id = <a href="../pera-framework/object.md#0x2_object_id_from_address">object::id_from_address</a>(<a href="../pera-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx));
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartBatchedSignEvent">StartBatchedSignEvent</a> {
        session_id: batch_session_id,
        hashed_messages,
        initiating_user: <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx)
    });
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> messages_length = <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&hashed_messages);
    <b>while</b> (i &lt; messages_length) {
        <b>let</b> id = <a href="../pera-framework/object.md#0x2_object_id_from_address">object::id_from_address</a>(<a href="../pera-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx));
        <b>let</b> <a href="../pera-framework/event.md#0x2_event">event</a> = <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_StartSignEvent">StartSignEvent</a> {
            session_id: id,
            presign_session_id,
            sender: <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx),
            batched_session_id: batch_session_id,
            dwallet_id: id,
            dwallet_cap_id: id,
            presign_first_round_output: presign_first_round_output,
            presign_second_round_output: presign_second_round_output,
            centralized_signed_message: centralized_signed_messages[i],
            dkg_output,
            hashed_message: hashed_messages[i],
        };
        <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="../pera-framework/event.md#0x2_event">event</a>);
        i = i + 1;
    };
}
</code></pre>



</details>

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_create_sign_output"></a>

## Function `create_sign_output`

Emit event with the MPC Sign protocol output.
The initiating user should consume the emitted event.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_sign_output">create_sign_output</a>(signed_messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, batch_session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_create_sign_output">create_sign_output</a>(signed_messages: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, batch_session_id: ID, ctx: &<b>mut</b> TxContext) {
    <b>assert</b>!(<a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == @0x0, <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_ENotSystemAddress">ENotSystemAddress</a>);
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_CompletedSignEvent">CompletedSignEvent</a> {
        session_id: batch_session_id,
        signed_messages,
    });
}
</code></pre>



</details>
