---
title: Module `0x0::validator_cap`
---



-  [Resource `ValidatorCap`](#0x0_validator_cap_ValidatorCap)
-  [Resource `ValidatorOperationCap`](#0x0_validator_cap_ValidatorOperationCap)
-  [Function `new_validator_cap`](#0x0_validator_cap_new_validator_cap)
-  [Function `validator_id`](#0x0_validator_cap_validator_id)
-  [Function `validator_operation_cap_validator_id`](#0x0_validator_cap_validator_operation_cap_validator_id)
-  [Function `new_validator_operation_cap`](#0x0_validator_cap_new_validator_operation_cap)


<pre><code><b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
</code></pre>



<a name="0x0_validator_cap_ValidatorCap"></a>

## Resource `ValidatorCap`

A capability for controlling the validator, cannot be revoked.


<pre><code><b>struct</b> <a href="validator_cap.md#0x0_validator_cap_ValidatorCap">ValidatorCap</a> <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x0_validator_cap_ValidatorOperationCap"></a>

## Resource `ValidatorOperationCap`

A capability for validator operations, can be revoked using <code><a href="validator_cap.md#0x0_validator_cap_ValidatorCap">ValidatorCap</a></code>.


<pre><code><b>struct</b> <a href="validator_cap.md#0x0_validator_cap_ValidatorOperationCap">ValidatorOperationCap</a> <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x0_validator_cap_new_validator_cap"></a>

## Function `new_validator_cap`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_cap.md#0x0_validator_cap_new_validator_cap">new_validator_cap</a>(validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_cap.md#0x0_validator_cap_new_validator_cap">new_validator_cap</a>(
    validator_id: ID,
    ctx: &<b>mut</b> TxContext,
): <a href="validator_cap.md#0x0_validator_cap_ValidatorCap">ValidatorCap</a> {
    <b>let</b> cap = <a href="validator_cap.md#0x0_validator_cap_ValidatorCap">ValidatorCap</a> {
        id: <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx),
        validator_id
    };
    cap
}
</code></pre>



</details>

<a name="0x0_validator_cap_validator_id"></a>

## Function `validator_id`



<pre><code><b>public</b> <b>fun</b> <a href="validator_cap.md#0x0_validator_cap_validator_id">validator_id</a>(cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>): <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_cap.md#0x0_validator_cap_validator_id">validator_id</a>(
    cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">ValidatorCap</a>,
): ID {
    cap.validator_id
}
</code></pre>



</details>

<a name="0x0_validator_cap_validator_operation_cap_validator_id"></a>

## Function `validator_operation_cap_validator_id`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_cap.md#0x0_validator_cap_validator_operation_cap_validator_id">validator_operation_cap_validator_id</a>(cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>): <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_cap.md#0x0_validator_cap_validator_operation_cap_validator_id">validator_operation_cap_validator_id</a>(cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorOperationCap">ValidatorOperationCap</a>): ID {
    cap.validator_id
}
</code></pre>



</details>

<a name="0x0_validator_cap_new_validator_operation_cap"></a>

## Function `new_validator_operation_cap`

Should be only called by the friend modules when adding a <code>Validator</code>
or rotating an existing validaotr's <code>operation_cap_id</code>.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_cap.md#0x0_validator_cap_new_validator_operation_cap">new_validator_operation_cap</a>(validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="validator_cap.md#0x0_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_cap.md#0x0_validator_cap_new_validator_operation_cap">new_validator_operation_cap</a>(
    validator_id: ID,
    ctx: &<b>mut</b> TxContext,
): <a href="validator_cap.md#0x0_validator_cap_ValidatorOperationCap">ValidatorOperationCap</a> {
    <b>let</b> operation_cap = <a href="validator_cap.md#0x0_validator_cap_ValidatorOperationCap">ValidatorOperationCap</a> {
        id: <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx),
        validator_id,
    };
    operation_cap
}
</code></pre>



</details>
