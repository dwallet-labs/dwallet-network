---
title: Module `(ika_system=0x0)::validator_cap`
---



-  [Struct `ValidatorCap`](#(ika_system=0x0)_validator_cap_ValidatorCap)
-  [Struct `ValidatorOperationCap`](#(ika_system=0x0)_validator_cap_ValidatorOperationCap)
-  [Function `new_validator_cap`](#(ika_system=0x0)_validator_cap_new_validator_cap)
-  [Function `validator_id`](#(ika_system=0x0)_validator_cap_validator_id)
-  [Function `validator_operation_cap_validator_id`](#(ika_system=0x0)_validator_cap_validator_operation_cap_validator_id)
-  [Function `new_validator_operation_cap`](#(ika_system=0x0)_validator_cap_new_validator_operation_cap)


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



<a name="(ika_system=0x0)_validator_cap_ValidatorCap"></a>

## Struct `ValidatorCap`

A capability for controlling the validator, cannot be revoked.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">ValidatorCap</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_validator_cap_ValidatorOperationCap"></a>

## Struct `ValidatorOperationCap`

A capability for validator operations, can be revoked using <code><a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">ValidatorCap</a></code>.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">ValidatorOperationCap</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_validator_cap_new_validator_cap"></a>

## Function `new_validator_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_new_validator_cap">new_validator_cap</a>(<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_new_validator_cap">new_validator_cap</a>(
    <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>: ID,
    ctx: &<b>mut</b> TxContext,
): <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">ValidatorCap</a> {
    <b>let</b> cap = <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">ValidatorCap</a> {
        id: object::new(ctx),
        <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>
    };
    cap
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_cap_validator_id"></a>

## Function `validator_id`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>(cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>(
    cap: &<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">ValidatorCap</a>,
): ID {
    cap.<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_cap_validator_operation_cap_validator_id"></a>

## Function `validator_operation_cap_validator_id`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_operation_cap_validator_id">validator_operation_cap_validator_id</a>(cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_operation_cap_validator_id">validator_operation_cap_validator_id</a>(cap: &<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">ValidatorOperationCap</a>): ID {
    cap.<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_cap_new_validator_operation_cap"></a>

## Function `new_validator_operation_cap`

Should be only called by the friend modules when adding a <code>Validator</code>
or rotating an existing validaotr's <code>operation_cap_id</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_new_validator_operation_cap">new_validator_operation_cap</a>(<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_new_validator_operation_cap">new_validator_operation_cap</a>(
    <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>: ID,
    ctx: &<b>mut</b> TxContext,
): <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">ValidatorOperationCap</a> {
    <b>let</b> operation_cap = <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">ValidatorOperationCap</a> {
        id: object::new(ctx),
        <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>,
    };
    operation_cap
}
</code></pre>



</details>
