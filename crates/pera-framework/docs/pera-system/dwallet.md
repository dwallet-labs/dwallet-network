---
title: Module `0x3::dwallet`
---



-  [Struct `InitiateDKGSessionEvent`](#0x3_dwallet_InitiateDKGSessionEvent)
-  [Resource `InitiateDKGSessionData`](#0x3_dwallet_InitiateDKGSessionData)
-  [Struct `StartDKGSecondRoundEvent`](#0x3_dwallet_StartDKGSecondRoundEvent)
-  [Resource `DKGSecondRoundData`](#0x3_dwallet_DKGSecondRoundData)
-  [Resource `CompletedFirstDKGRoundData`](#0x3_dwallet_CompletedFirstDKGRoundData)
-  [Struct `CompletedDKGRoundEvent`](#0x3_dwallet_CompletedDKGRoundEvent)
-  [Constants](#@Constants_0)
-  [Function `launch_initiate_dkg_session`](#0x3_dwallet_launch_initiate_dkg_session)
-  [Function `launch_dkg_second_round`](#0x3_dwallet_launch_dkg_second_round)
-  [Function `create_first_dkg_round_output`](#0x3_dwallet_create_first_dkg_round_output)


<pre><code><b>use</b> <a href="../pera-framework/event.md#0x2_event">0x2::event</a>;
<b>use</b> <a href="../pera-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../pera-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../pera-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
</code></pre>



<a name="0x3_dwallet_InitiateDKGSessionEvent"></a>

## Struct `InitiateDKGSessionEvent`



<pre><code><b>struct</b> <a href="dwallet.md#0x3_dwallet_InitiateDKGSessionEvent">InitiateDKGSessionEvent</a> <b>has</b> <b>copy</b>, drop
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
</dl>


</details>

<a name="0x3_dwallet_InitiateDKGSessionData"></a>

## Resource `InitiateDKGSessionData`



<pre><code><b>struct</b> <a href="dwallet.md#0x3_dwallet_InitiateDKGSessionData">InitiateDKGSessionData</a> <b>has</b> key
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
<code>sender: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_StartDKGSecondRoundEvent"></a>

## Struct `StartDKGSecondRoundEvent`



<pre><code><b>struct</b> <a href="dwallet.md#0x3_dwallet_StartDKGSecondRoundEvent">StartDKGSecondRoundEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>input: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_DKGSecondRoundData"></a>

## Resource `DKGSecondRoundData`



<pre><code><b>struct</b> <a href="dwallet.md#0x3_dwallet_DKGSecondRoundData">DKGSecondRoundData</a> <b>has</b> key
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
<code>sender: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>input: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_CompletedFirstDKGRoundData"></a>

## Resource `CompletedFirstDKGRoundData`



<pre><code><b>struct</b> <a href="dwallet.md#0x3_dwallet_CompletedFirstDKGRoundData">CompletedFirstDKGRoundData</a> <b>has</b> key
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
<code>value: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_dwallet_CompletedDKGRoundEvent"></a>

## Struct `CompletedDKGRoundEvent`



<pre><code><b>struct</b> <a href="dwallet.md#0x3_dwallet_CompletedDKGRoundEvent">CompletedDKGRoundEvent</a> <b>has</b> <b>copy</b>, drop
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
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x3_dwallet_ENotSystemAddress"></a>



<pre><code><b>const</b> <a href="dwallet.md#0x3_dwallet_ENotSystemAddress">ENotSystemAddress</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
</code></pre>



<a name="0x3_dwallet_launch_initiate_dkg_session"></a>

## Function `launch_initiate_dkg_session`

Function to launch proof MPC flow.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet.md#0x3_dwallet_launch_initiate_dkg_session">launch_initiate_dkg_session</a>(ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet.md#0x3_dwallet_launch_initiate_dkg_session">launch_initiate_dkg_session</a>(ctx: &<b>mut</b> TxContext) {
    <b>let</b> session_data = <a href="dwallet.md#0x3_dwallet_InitiateDKGSessionData">InitiateDKGSessionData</a> {
        id: <a href="../pera-framework/object.md#0x2_object_new">object::new</a>(ctx),
        sender: <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx)
    };
    <b>let</b> created_proof_mpc_session_event = <a href="dwallet.md#0x3_dwallet_InitiateDKGSessionEvent">InitiateDKGSessionEvent</a> {
        session_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(&session_data),
        sender: <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx)
    };
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(created_proof_mpc_session_event);
    <a href="../pera-framework/transfer.md#0x2_transfer_freeze_object">transfer::freeze_object</a>(session_data);
}
</code></pre>



</details>

<a name="0x3_dwallet_launch_dkg_second_round"></a>

## Function `launch_dkg_second_round`

Function to launch proof MPC flow.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet.md#0x3_dwallet_launch_dkg_second_round">launch_dkg_second_round</a>(input: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet.md#0x3_dwallet_launch_dkg_second_round">launch_dkg_second_round</a>(input: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<b>mut</b> TxContext) {
    <b>let</b> session_data = <a href="dwallet.md#0x3_dwallet_DKGSecondRoundData">DKGSecondRoundData</a> {
        id: <a href="../pera-framework/object.md#0x2_object_new">object::new</a>(ctx),
        sender: <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx),
        input
    };
    <b>let</b> created_proof_mpc_session_event = <a href="dwallet.md#0x3_dwallet_StartDKGSecondRoundEvent">StartDKGSecondRoundEvent</a> {
        session_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(&session_data),
        sender: <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx),
        input
    };
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(created_proof_mpc_session_event);
    <a href="../pera-framework/transfer.md#0x2_transfer_freeze_object">transfer::freeze_object</a>(session_data);
}
</code></pre>



</details>

<a name="0x3_dwallet_create_first_dkg_round_output"></a>

## Function `create_first_dkg_round_output`



<pre><code><b>public</b> <b>fun</b> <a href="dwallet.md#0x3_dwallet_create_first_dkg_round_output">create_first_dkg_round_output</a>(session_initiator: <b>address</b>, session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet.md#0x3_dwallet_create_first_dkg_round_output">create_first_dkg_round_output</a>(session_initiator: <b>address</b>, session_id: ID, output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<b>mut</b> TxContext) {
   <b>assert</b>!(<a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == @0x0, <a href="dwallet.md#0x3_dwallet_ENotSystemAddress">ENotSystemAddress</a>);
   <b>let</b> proof_session_result = <a href="dwallet.md#0x3_dwallet_CompletedFirstDKGRoundData">CompletedFirstDKGRoundData</a> {
       id: <a href="../pera-framework/object.md#0x2_object_new">object::new</a>(ctx),
       session_id: session_id,
       value: output,
   };
   <a href="../pera-framework/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(proof_session_result, session_initiator);

   <b>let</b> completed_proof_mpc_session_event = <a href="dwallet.md#0x3_dwallet_CompletedDKGRoundEvent">CompletedDKGRoundEvent</a> {
       session_id: session_id,
       sender: session_initiator,
   };

   <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(completed_proof_mpc_session_event);
}
</code></pre>



</details>