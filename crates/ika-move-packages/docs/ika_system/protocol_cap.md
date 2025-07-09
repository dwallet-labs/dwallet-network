---
title: Module `(ika_system=0x0)::protocol_cap`
---



-  [Struct `ProtocolCap`](#(ika_system=0x0)_protocol_cap_ProtocolCap)
-  [Struct `VerifiedProtocolCap`](#(ika_system=0x0)_protocol_cap_VerifiedProtocolCap)
-  [Function `create`](#(ika_system=0x0)_protocol_cap_create)
-  [Function `create_verified`](#(ika_system=0x0)_protocol_cap_create_verified)


<pre><code><b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../sui/address.md#sui_address">sui::address</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
</code></pre>



<a name="(ika_system=0x0)_protocol_cap_ProtocolCap"></a>

## Struct `ProtocolCap`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_ProtocolCap">ProtocolCap</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_protocol_cap_VerifiedProtocolCap"></a>

## Struct `VerifiedProtocolCap`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_VerifiedProtocolCap">VerifiedProtocolCap</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="(ika_system=0x0)_protocol_cap_create"></a>

## Function `create`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_create">create</a>(ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_ProtocolCap">protocol_cap::ProtocolCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_create">create</a>(
    ctx: &<b>mut</b> TxContext,
): <a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_ProtocolCap">ProtocolCap</a> {
    <a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_ProtocolCap">ProtocolCap</a> {
        id: object::new(ctx),
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_protocol_cap_create_verified"></a>

## Function `create_verified`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_create_verified">create_verified</a>(): (ika_system=0x0)::<a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_VerifiedProtocolCap">protocol_cap::VerifiedProtocolCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_create_verified">create_verified</a>(): <a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_VerifiedProtocolCap">VerifiedProtocolCap</a> {
    <a href="../ika_system/protocol_cap.md#(ika_system=0x0)_protocol_cap_VerifiedProtocolCap">VerifiedProtocolCap</a> {}
}
</code></pre>



</details>
