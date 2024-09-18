---
title: Module `0x3::proof`
---

The proof module
Responsible to start and manage the Proof generation MPC flow
Used only for testing the way we launch & manage an MPC flow.


-  [Struct `CreatedProofMPCSessionEvent`](#0x3_proof_CreatedProofMPCSessionEvent)
-  [Resource `ProofSessionData`](#0x3_proof_ProofSessionData)
-  [Function `launch_proof_mpc_flow`](#0x3_proof_launch_proof_mpc_flow)


<pre><code><b>use</b> <a href="../pera-framework/event.md#0x2_event">0x2::event</a>;
<b>use</b> <a href="../pera-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../pera-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../pera-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
</code></pre>



<a name="0x3_proof_CreatedProofMPCSessionEvent"></a>

## Struct `CreatedProofMPCSessionEvent`

Event to start a <code>MockMPCSession</code>, caught by the Validators.


<pre><code><b>struct</b> <a href="proof.md#0x3_proof_CreatedProofMPCSessionEvent">CreatedProofMPCSessionEvent</a> <b>has</b> <b>copy</b>, drop
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

<a name="0x3_proof_ProofSessionData"></a>

## Resource `ProofSessionData`



<pre><code><b>struct</b> <a href="proof.md#0x3_proof_ProofSessionData">ProofSessionData</a> <b>has</b> key
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

<a name="0x3_proof_launch_proof_mpc_flow"></a>

## Function `launch_proof_mpc_flow`

Function to launch proof MPC flow.


<pre><code><b>public</b> <b>fun</b> <a href="proof.md#0x3_proof_launch_proof_mpc_flow">launch_proof_mpc_flow</a>(ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="proof.md#0x3_proof_launch_proof_mpc_flow">launch_proof_mpc_flow</a>(ctx: &<b>mut</b> TxContext) {
    <b>let</b> session_data = <a href="proof.md#0x3_proof_ProofSessionData">ProofSessionData</a> {
        id: <a href="../pera-framework/object.md#0x2_object_new">object::new</a>(ctx),
    };
    // Emit <a href="../pera-framework/event.md#0x2_event">event</a> <b>to</b> start MPC flow.
    // Part of the implementation of section 3.2.1 in the Pera Async paper.
    // When iterating over the transactions in the batch, MPC transactions will get exectuted locally
    // <b>to</b> catch the <a href="../pera-framework/event.md#0x2_event">event</a> <b>with</b> all the needed data <b>to</b> start the MPC flow.
    <b>let</b> created_proof_mpc_session_event = <a href="proof.md#0x3_proof_CreatedProofMPCSessionEvent">CreatedProofMPCSessionEvent</a> {
        // The session ID is a <a href="../pera-framework/random.md#0x2_random">random</a>, unique ID of the <a href="proof.md#0x3_proof_ProofSessionData">ProofSessionData</a> <a href="../pera-framework/object.md#0x2_object">object</a>.
        // It is needed so the user will be able <b>to</b> know, when fetching the generated <a href="proof.md#0x3_proof">proof</a> data,
        // that the <a href="proof.md#0x3_proof">proof</a> was generated for this specific session.
        session_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(&session_data),
        sender: <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx),
    };
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(created_proof_mpc_session_event);
    <a href="../pera-framework/transfer.md#0x2_transfer_freeze_object">transfer::freeze_object</a>(session_data);
}
</code></pre>



</details>
