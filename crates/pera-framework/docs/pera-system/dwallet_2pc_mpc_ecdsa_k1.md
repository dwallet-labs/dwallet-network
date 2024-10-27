---
title: Module `0x3::dwallet_2pc_mpc_ecdsa_k1`
---



-  [Resource `DKGSession`](#0x3_dwallet_2pc_mpc_ecdsa_k1_DKGSession)
-  [Struct `CreatedDKGSessionEvent`](#0x3_dwallet_2pc_mpc_ecdsa_k1_CreatedDKGSessionEvent)
-  [Function `start_first_dkg_session`](#0x3_dwallet_2pc_mpc_ecdsa_k1_start_first_dkg_session)


<pre><code><b>use</b> <a href="../pera-framework/event.md#0x2_event">0x2::event</a>;
<b>use</b> <a href="../pera-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../pera-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../pera-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="dwallet.md#0x3_dwallet">0x3::dwallet</a>;
</code></pre>



<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_DKGSession"></a>

## Resource `DKGSession`



<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_DKGSession">DKGSession</a> <b>has</b> key
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
<code>dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
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

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_CreatedDKGSessionEvent"></a>

## Struct `CreatedDKGSessionEvent`

Event to start a <code>DKG</code> session, caught by the Validators.


<pre><code><b>struct</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_CreatedDKGSessionEvent">CreatedDKGSessionEvent</a> <b>has</b> <b>copy</b>, drop
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

<a name="0x3_dwallet_2pc_mpc_ecdsa_k1_start_first_dkg_session"></a>

## Function `start_first_dkg_session`

Starts the first Distributed Key Generation (DKG) session. Two MPC sessions are required to
create a Dwallet.
Capabilities are used to control access to the Dwallet.
This function start the DKG proccess in the Validators.


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_start_first_dkg_session">start_first_dkg_session</a>(ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="dwallet.md#0x3_dwallet_DWalletCap">dwallet::DWalletCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_start_first_dkg_session">start_first_dkg_session</a>(
    ctx: &<b>mut</b> TxContext
): DWalletCap {
    <b>let</b> cap = create_dwallet_cap(ctx);
    <b>let</b> sender = <a href="../pera-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx);
    <b>let</b> session = <a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_DKGSession">DKGSession</a> {
        id: <a href="../pera-framework/object.md#0x2_object_new">object::new</a>(ctx),
        dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(&cap),
        sender,
    };
    <a href="../pera-framework/event.md#0x2_event_emit">event::emit</a>(<a href="dwallet_2pc_mpc_ecdsa_k1.md#0x3_dwallet_2pc_mpc_ecdsa_k1_CreatedDKGSessionEvent">CreatedDKGSessionEvent</a> {
        session_id: <a href="../pera-framework/object.md#0x2_object_id">object::id</a>(&session),
        sender,
    });
    <a href="../pera-framework/transfer.md#0x2_transfer_freeze_object">transfer::freeze_object</a>(session);
    cap
}
</code></pre>



</details>
