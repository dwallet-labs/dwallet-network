---
title: Module `0x0::protocol_cap`
---



-  [Resource `ProtocolCap`](#0x0_protocol_cap_ProtocolCap)
-  [Function `new_protocol_cap`](#0x0_protocol_cap_new_protocol_cap)


<pre><code><b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
</code></pre>



<a name="0x0_protocol_cap_ProtocolCap"></a>

## Resource `ProtocolCap`



<pre><code><b>struct</b> <a href="protocol_cap.md#0x0_protocol_cap_ProtocolCap">ProtocolCap</a> <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x0_protocol_cap_new_protocol_cap"></a>

## Function `new_protocol_cap`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="protocol_cap.md#0x0_protocol_cap_new_protocol_cap">new_protocol_cap</a>(ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="protocol_cap.md#0x0_protocol_cap_ProtocolCap">protocol_cap::ProtocolCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="protocol_cap.md#0x0_protocol_cap_new_protocol_cap">new_protocol_cap</a>(
    ctx: &<b>mut</b> TxContext,
): <a href="protocol_cap.md#0x0_protocol_cap_ProtocolCap">ProtocolCap</a> {
    <b>let</b> cap = <a href="protocol_cap.md#0x0_protocol_cap_ProtocolCap">ProtocolCap</a> {
        id: <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx),
    };
    cap
}
</code></pre>



</details>
