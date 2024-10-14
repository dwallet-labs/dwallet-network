---
title: Module `0x3::dwallet`
---



-  [Struct `InitiateDKGSessionEvent`](#0x3_dwallet_InitiateDKGSessionEvent)
-  [Resource `InitiateDKGSessionData`](#0x3_dwallet_InitiateDKGSessionData)
-  [Function `launch_initiate_dkg_session`](#0x3_dwallet_launch_initiate_dkg_session)


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
    };
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(created_proof_mpc_session_event);
    <a href="../pera-framework/transfer.md#0x2_transfer_freeze_object">transfer::freeze_object</a>(session_data);
}
</code></pre>



</details>
