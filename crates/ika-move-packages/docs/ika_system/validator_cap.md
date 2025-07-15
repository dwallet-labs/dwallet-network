---
title: Module `(ika_system=0x0)::validator_cap`
---



-  [Struct `ValidatorCap`](#(ika_system=0x0)_validator_cap_ValidatorCap)
-  [Struct `ValidatorOperationCap`](#(ika_system=0x0)_validator_cap_ValidatorOperationCap)
-  [Struct `ValidatorCommissionCap`](#(ika_system=0x0)_validator_cap_ValidatorCommissionCap)
-  [Struct `VerifiedValidatorCap`](#(ika_system=0x0)_validator_cap_VerifiedValidatorCap)
-  [Struct `VerifiedValidatorOperationCap`](#(ika_system=0x0)_validator_cap_VerifiedValidatorOperationCap)
-  [Struct `VerifiedValidatorCommissionCap`](#(ika_system=0x0)_validator_cap_VerifiedValidatorCommissionCap)
-  [Function `new_validator_cap`](#(ika_system=0x0)_validator_cap_new_validator_cap)
-  [Function `new_validator_operation_cap`](#(ika_system=0x0)_validator_cap_new_validator_operation_cap)
-  [Function `new_validator_commission_cap`](#(ika_system=0x0)_validator_cap_new_validator_commission_cap)
-  [Function `create_verified_validator_cap`](#(ika_system=0x0)_validator_cap_create_verified_validator_cap)
-  [Function `create_verified_validator_operation_cap`](#(ika_system=0x0)_validator_cap_create_verified_validator_operation_cap)
-  [Function `create_verified_validator_commission_cap`](#(ika_system=0x0)_validator_cap_create_verified_validator_commission_cap)
-  [Function `validator_id`](#(ika_system=0x0)_validator_cap_validator_id)
-  [Function `validator_operation_cap_validator_id`](#(ika_system=0x0)_validator_cap_validator_operation_cap_validator_id)
-  [Function `validator_commission_cap_validator_id`](#(ika_system=0x0)_validator_cap_validator_commission_cap_validator_id)
-  [Function `verified_validator_cap_validator_id`](#(ika_system=0x0)_validator_cap_verified_validator_cap_validator_id)
-  [Function `verified_validator_operation_cap_validator_id`](#(ika_system=0x0)_validator_cap_verified_validator_operation_cap_validator_id)
-  [Function `verified_validator_commission_cap_validator_id`](#(ika_system=0x0)_validator_cap_verified_validator_commission_cap_validator_id)


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

<a name="(ika_system=0x0)_validator_cap_ValidatorCommissionCap"></a>

## Struct `ValidatorCommissionCap`

A capability for validator commission, can be revoked using <code><a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">ValidatorCap</a></code>.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCommissionCap">ValidatorCommissionCap</a> <b>has</b> key, store
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

<a name="(ika_system=0x0)_validator_cap_VerifiedValidatorCap"></a>

## Struct `VerifiedValidatorCap`

A one time witness for the validator capability.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_VerifiedValidatorCap">VerifiedValidatorCap</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_validator_cap_VerifiedValidatorOperationCap"></a>

## Struct `VerifiedValidatorOperationCap`

A one time witness for the validator operation capability.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_VerifiedValidatorOperationCap">VerifiedValidatorOperationCap</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_validator_cap_VerifiedValidatorCommissionCap"></a>

## Struct `VerifiedValidatorCommissionCap`

A one time witness for the validator commission capability.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_VerifiedValidatorCommissionCap">VerifiedValidatorCommissionCap</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
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


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_new_validator_cap">new_validator_cap</a>(<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>: ID, ctx: &<b>mut</b> TxContext): <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">ValidatorCap</a> {
    <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">ValidatorCap</a> {
        id: object::new(ctx),
        <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>,
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_cap_new_validator_operation_cap"></a>

## Function `new_validator_operation_cap`

Should be only called by the friend modules when adding a <code>Validator</code>
or rotating an existing validator's <code>operation_cap_id</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_new_validator_operation_cap">new_validator_operation_cap</a>(<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_new_validator_operation_cap">new_validator_operation_cap</a>(
    <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>: ID,
    ctx: &<b>mut</b> TxContext,
): <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">ValidatorOperationCap</a> {
    <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">ValidatorOperationCap</a> {
        id: object::new(ctx),
        <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>,
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_cap_new_validator_commission_cap"></a>

## Function `new_validator_commission_cap`

Should be only called by the friend modules when adding a <code>Validator</code>
or rotating an existing validator's <code>commission_cap_id</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_new_validator_commission_cap">new_validator_commission_cap</a>(<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCommissionCap">validator_cap::ValidatorCommissionCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_new_validator_commission_cap">new_validator_commission_cap</a>(
    <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>: ID,
    ctx: &<b>mut</b> TxContext,
): <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCommissionCap">ValidatorCommissionCap</a> {
    <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCommissionCap">ValidatorCommissionCap</a> {
        id: object::new(ctx),
        <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>,
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_cap_create_verified_validator_cap"></a>

## Function `create_verified_validator_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_create_verified_validator_cap">create_verified_validator_cap</a>(cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>): (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_VerifiedValidatorCap">validator_cap::VerifiedValidatorCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_create_verified_validator_cap">create_verified_validator_cap</a>(cap: &<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">ValidatorCap</a>): <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_VerifiedValidatorCap">VerifiedValidatorCap</a> {
    <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_VerifiedValidatorCap">VerifiedValidatorCap</a> {
        <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>: cap.<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>,
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_cap_create_verified_validator_operation_cap"></a>

## Function `create_verified_validator_operation_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_create_verified_validator_operation_cap">create_verified_validator_operation_cap</a>(cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>): (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_VerifiedValidatorOperationCap">validator_cap::VerifiedValidatorOperationCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_create_verified_validator_operation_cap">create_verified_validator_operation_cap</a>(cap: &<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">ValidatorOperationCap</a>): <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_VerifiedValidatorOperationCap">VerifiedValidatorOperationCap</a> {
    <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_VerifiedValidatorOperationCap">VerifiedValidatorOperationCap</a> {
        <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>: cap.<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>,
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_cap_create_verified_validator_commission_cap"></a>

## Function `create_verified_validator_commission_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_create_verified_validator_commission_cap">create_verified_validator_commission_cap</a>(cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCommissionCap">validator_cap::ValidatorCommissionCap</a>): (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_VerifiedValidatorCommissionCap">validator_cap::VerifiedValidatorCommissionCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_create_verified_validator_commission_cap">create_verified_validator_commission_cap</a>(cap: &<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCommissionCap">ValidatorCommissionCap</a>): <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_VerifiedValidatorCommissionCap">VerifiedValidatorCommissionCap</a> {
    <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_VerifiedValidatorCommissionCap">VerifiedValidatorCommissionCap</a> {
        <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>: cap.<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>,
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_cap_validator_id"></a>

## Function `validator_id`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>(cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>(cap: &<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">ValidatorCap</a>): ID {
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

<a name="(ika_system=0x0)_validator_cap_validator_commission_cap_validator_id"></a>

## Function `validator_commission_cap_validator_id`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_commission_cap_validator_id">validator_commission_cap_validator_id</a>(cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCommissionCap">validator_cap::ValidatorCommissionCap</a>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_commission_cap_validator_id">validator_commission_cap_validator_id</a>(cap: &<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCommissionCap">ValidatorCommissionCap</a>): ID {
    cap.<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_cap_verified_validator_cap_validator_id"></a>

## Function `verified_validator_cap_validator_id`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_verified_validator_cap_validator_id">verified_validator_cap_validator_id</a>(cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_VerifiedValidatorCap">validator_cap::VerifiedValidatorCap</a>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_verified_validator_cap_validator_id">verified_validator_cap_validator_id</a>(cap: &<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_VerifiedValidatorCap">VerifiedValidatorCap</a>): ID {
    cap.<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_cap_verified_validator_operation_cap_validator_id"></a>

## Function `verified_validator_operation_cap_validator_id`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_verified_validator_operation_cap_validator_id">verified_validator_operation_cap_validator_id</a>(cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_VerifiedValidatorOperationCap">validator_cap::VerifiedValidatorOperationCap</a>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_verified_validator_operation_cap_validator_id">verified_validator_operation_cap_validator_id</a>(cap: &<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_VerifiedValidatorOperationCap">VerifiedValidatorOperationCap</a>): ID {
    cap.<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_cap_verified_validator_commission_cap_validator_id"></a>

## Function `verified_validator_commission_cap_validator_id`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_verified_validator_commission_cap_validator_id">verified_validator_commission_cap_validator_id</a>(cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_VerifiedValidatorCommissionCap">validator_cap::VerifiedValidatorCommissionCap</a>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_verified_validator_commission_cap_validator_id">verified_validator_commission_cap_validator_id</a>(cap: &<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_VerifiedValidatorCommissionCap">VerifiedValidatorCommissionCap</a>): ID {
    cap.<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_validator_id">validator_id</a>
}
</code></pre>



</details>
