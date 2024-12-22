---
title: Module `0x3::dwallet`
---

This module defines the core data structures and functions for
working with dWallets in the pera system.


<a name="@Overview_0"></a>

### Overview


- A **dWallet** (<code><a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a></code>) represents a wallet that is created after the Distributed Key Generation (DKG) process.
It encapsulates the session ID, capability ID, and the output of the DKG's second round.
- A **dWallet capability** (<code><a href="dwallet.md#0x3_dwallet_DWalletCap">DWalletCap</a></code>) represents a capability that grants
ownership and control over a corresponding <code><a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a></code>.


<a name="@Key_Concepts_1"></a>

### Key Concepts


- **DWallet**: A generic wallet structure with a phantom type <code>T</code>.
- **DWalletCap**: A capability object granting control over a specific dWallet.
- **Session ID**: A unique identifier for the DKG session.


    -  [Overview](#@Overview_0)
    -  [Key Concepts](#@Key_Concepts_1)
-  [Resource `DWallet`](#0x3_dwallet_DWallet)
-  [Resource `DWalletCap`](#0x3_dwallet_DWalletCap)
-  [Function `create_dwallet`](#0x3_dwallet_create_dwallet)
-  [Function `create_dwallet_cap`](#0x3_dwallet_create_dwallet_cap)
-  [Function `get_dwallet_cap_id`](#0x3_dwallet_get_dwallet_cap_id)
-  [Function `get_dwallet_output`](#0x3_dwallet_get_dwallet_output)


<pre><code><b>use</b> <a href="../pera-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../pera-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
</code></pre>



<a name="0x3_dwallet_DWallet"></a>

## Resource `DWallet`

<code><a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a></code> represents a wallet that is created after the DKG process.


<a name="@Fields_2"></a>

##### Fields

- <code>id</code>: The unique identifier for the dWallet object.
- <code>session_id</code>: The ID of the session that generated this dWallet.
- <code>dwallet_cap_id</code>: The ID of the dWallet capability associated with this wallet.
- <code>output</code>: The output of the second DKG round, represented as a <code><a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>.


<pre><code><b>struct</b> <a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a>&lt;T&gt; <b>has</b> store, key
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
<code>dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
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

<a name="0x3_dwallet_DWalletCap"></a>

## Resource `DWalletCap`

<code><a href="dwallet.md#0x3_dwallet_DWalletCap">DWalletCap</a></code> holder controls a corresponding <code><a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a></code>.


<a name="@Fields_3"></a>

##### Fields

- <code>id</code>: The unique identifier for the dWallet capability object.


<pre><code><b>struct</b> <a href="dwallet.md#0x3_dwallet_DWalletCap">DWalletCap</a> <b>has</b> store, key
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

<a name="0x3_dwallet_create_dwallet"></a>

## Function `create_dwallet`

A generic function to create a new [<code><a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a></code>] object of type <code>T</code>.


<a name="@Parameters_4"></a>

##### Parameters

- <code>session_id</code>: The ID of the session that generates this dWallet.
- <code>dwallet_cap_id</code>: The ID of the dWallet capability associated with this dWallet.
- <code>output</code>: The output of the second DKG round, represented as a <code><a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>.
- <code>ctx</code>: A mutable transaction context used to create the dWallet object.


<a name="@Returns_5"></a>

##### Returns

A new [<code><a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a></code>] object of the specified type <code>T</code>.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet.md#0x3_dwallet_create_dwallet">create_dwallet</a>&lt;T: drop&gt;(session_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, dwallet_cap_id: <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>, output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, dwallet_mpc_network_key_version: u8, ctx: &<b>mut</b> <a href="../pera-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet.md#0x3_dwallet_create_dwallet">create_dwallet</a>&lt;T: drop&gt;(
    session_id: ID,
    dwallet_cap_id: ID,
    output: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    dwallet_mpc_network_key_version: u8,
    ctx: &<b>mut</b> TxContext
): <a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a>&lt;T&gt; {
    <a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a>&lt;T&gt; {
        id: <a href="../pera-framework/object.md#0x2_object_new">object::new</a>(ctx),
        session_id,
        dwallet_cap_id,
        output,
        dwallet_mpc_network_key_version,
    }
}
</code></pre>



</details>

<a name="0x3_dwallet_create_dwallet_cap"></a>

## Function `create_dwallet_cap`

Create a new [<code><a href="dwallet.md#0x3_dwallet_DWalletCap">DWalletCap</a></code>] object.

The holder of the <code><a href="dwallet.md#0x3_dwallet_DWalletCap">DWalletCap</a></code> has control and ownership over
the associated <code><a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a></code>.


<a name="@Parameters_6"></a>

##### Parameters

- <code>ctx</code>: A mutable transaction context used to create the <code><a href="dwallet.md#0x3_dwallet_DWalletCap">DWalletCap</a></code> object.


<a name="@Returns_7"></a>

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

<a name="0x3_dwallet_get_dwallet_cap_id"></a>

## Function `get_dwallet_cap_id`

Retrieve the ID of the <code><a href="dwallet.md#0x3_dwallet_DWalletCap">DWalletCap</a></code> associated with a given dWallet.


<a name="@Parameters_8"></a>

##### Parameters

- <code><a href="dwallet.md#0x3_dwallet">dwallet</a></code>: A reference to the [<code><a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a></code>] object whose capability ID is to be retrieved.


<a name="@Returns_9"></a>

##### Returns

The ID of the <code><a href="dwallet.md#0x3_dwallet_DWalletCap">DWalletCap</a></code> associated with the provided dWallet.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet.md#0x3_dwallet_get_dwallet_cap_id">get_dwallet_cap_id</a>&lt;T: drop&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;T&gt;): <a href="../pera-framework/object.md#0x2_object_ID">object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet.md#0x3_dwallet_get_dwallet_cap_id">get_dwallet_cap_id</a>&lt;T: drop&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a>&lt;T&gt;): ID {
    <a href="dwallet.md#0x3_dwallet">dwallet</a>.dwallet_cap_id
}
</code></pre>



</details>

<a name="0x3_dwallet_get_dwallet_output"></a>

## Function `get_dwallet_output`

Retrieve the output of the second DKG round for a given dWallet.


<a name="@Parameters_10"></a>

##### Parameters

- <code><a href="dwallet.md#0x3_dwallet">dwallet</a></code>: A reference to the [<code><a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a></code>] object whose DKG output is to be retrieved.


<a name="@Returns_11"></a>

##### Returns

A <code><a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code> representing the output of the second DKG round for the specified dWallet.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="dwallet.md#0x3_dwallet_get_dwallet_output">get_dwallet_output</a>&lt;T: drop&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">dwallet::DWallet</a>&lt;T&gt;): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="dwallet.md#0x3_dwallet_get_dwallet_output">get_dwallet_output</a>&lt;T: drop&gt;(<a href="dwallet.md#0x3_dwallet">dwallet</a>: &<a href="dwallet.md#0x3_dwallet_DWallet">DWallet</a>&lt;T&gt;): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <a href="dwallet.md#0x3_dwallet">dwallet</a>.output
}
</code></pre>



</details>
