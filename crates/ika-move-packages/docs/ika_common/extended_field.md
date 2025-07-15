---
title: Module `ika_common::extended_field`
---



-  [Struct `ExtendedField`](#ika_common_extended_field_ExtendedField)
-  [Struct `Key`](#ika_common_extended_field_Key)
-  [Function `new`](#ika_common_extended_field_new)
-  [Function `borrow`](#ika_common_extended_field_borrow)
-  [Function `borrow_mut`](#ika_common_extended_field_borrow_mut)
-  [Function `swap`](#ika_common_extended_field_swap)
-  [Function `destroy`](#ika_common_extended_field_destroy)


<pre><code><b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../sui/address.md#sui_address">sui::address</a>;
<b>use</b> <a href="../sui/dynamic_field.md#sui_dynamic_field">sui::dynamic_field</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
</code></pre>



<a name="ika_common_extended_field_ExtendedField"></a>

## Struct `ExtendedField`

Extended field acts as a field, but stored in a dynamic field, hence, it does
not bloat the original object's storage, storing only <code>UID</code> of the extended
field.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_common/extended_field.md#ika_common_extended_field_ExtendedField">ExtendedField</a>&lt;<b>phantom</b> T: store&gt; <b>has</b> key, store
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

<a name="ika_common_extended_field_Key"></a>

## Struct `Key`

Key to store the value in the extended field. Never changes.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_common/extended_field.md#ika_common_extended_field_Key">Key</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="ika_common_extended_field_new"></a>

## Function `new`

Creates a new extended field with the given value.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/extended_field.md#ika_common_extended_field_new">new</a>&lt;T: store&gt;(value: T, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../ika_common/extended_field.md#ika_common_extended_field_ExtendedField">ika_common::extended_field::ExtendedField</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/extended_field.md#ika_common_extended_field_new">new</a>&lt;T: store&gt;(value: T, ctx: &<b>mut</b> TxContext): <a href="../ika_common/extended_field.md#ika_common_extended_field_ExtendedField">ExtendedField</a>&lt;T&gt; {
    <b>let</b> <b>mut</b> id = object::new(ctx);
    df::add(&<b>mut</b> id, <a href="../ika_common/extended_field.md#ika_common_extended_field_Key">Key</a>(), value);
    <a href="../ika_common/extended_field.md#ika_common_extended_field_ExtendedField">ExtendedField</a> { id }
}
</code></pre>



</details>

<a name="ika_common_extended_field_borrow"></a>

## Function `borrow`

Borrows the value stored in the extended field.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/extended_field.md#ika_common_extended_field_borrow">borrow</a>&lt;T: store&gt;(field: &<a href="../ika_common/extended_field.md#ika_common_extended_field_ExtendedField">ika_common::extended_field::ExtendedField</a>&lt;T&gt;): &T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/extended_field.md#ika_common_extended_field_borrow">borrow</a>&lt;T: store&gt;(field: &<a href="../ika_common/extended_field.md#ika_common_extended_field_ExtendedField">ExtendedField</a>&lt;T&gt;): &T {
    df::borrow(&field.id, <a href="../ika_common/extended_field.md#ika_common_extended_field_Key">Key</a>())
}
</code></pre>



</details>

<a name="ika_common_extended_field_borrow_mut"></a>

## Function `borrow_mut`

Borrows the value stored in the extended field mutably.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/extended_field.md#ika_common_extended_field_borrow_mut">borrow_mut</a>&lt;T: store&gt;(field: &<b>mut</b> <a href="../ika_common/extended_field.md#ika_common_extended_field_ExtendedField">ika_common::extended_field::ExtendedField</a>&lt;T&gt;): &<b>mut</b> T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/extended_field.md#ika_common_extended_field_borrow_mut">borrow_mut</a>&lt;T: store&gt;(field: &<b>mut</b> <a href="../ika_common/extended_field.md#ika_common_extended_field_ExtendedField">ExtendedField</a>&lt;T&gt;): &<b>mut</b> T {
    df::borrow_mut(&<b>mut</b> field.id, <a href="../ika_common/extended_field.md#ika_common_extended_field_Key">Key</a>())
}
</code></pre>



</details>

<a name="ika_common_extended_field_swap"></a>

## Function `swap`

Swaps the value stored in the extended field with the given value.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/extended_field.md#ika_common_extended_field_swap">swap</a>&lt;T: store&gt;(field: &<b>mut</b> <a href="../ika_common/extended_field.md#ika_common_extended_field_ExtendedField">ika_common::extended_field::ExtendedField</a>&lt;T&gt;, value: T): T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/extended_field.md#ika_common_extended_field_swap">swap</a>&lt;T: store&gt;(field: &<b>mut</b> <a href="../ika_common/extended_field.md#ika_common_extended_field_ExtendedField">ExtendedField</a>&lt;T&gt;, value: T): T {
    <b>let</b> old = df::remove(&<b>mut</b> field.id, <a href="../ika_common/extended_field.md#ika_common_extended_field_Key">Key</a>());
    df::add(&<b>mut</b> field.id, <a href="../ika_common/extended_field.md#ika_common_extended_field_Key">Key</a>(), value);
    old
}
</code></pre>



</details>

<a name="ika_common_extended_field_destroy"></a>

## Function `destroy`

Destroys the extended field and returns the value stored in it.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/extended_field.md#ika_common_extended_field_destroy">destroy</a>&lt;T: store&gt;(field: <a href="../ika_common/extended_field.md#ika_common_extended_field_ExtendedField">ika_common::extended_field::ExtendedField</a>&lt;T&gt;): T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/extended_field.md#ika_common_extended_field_destroy">destroy</a>&lt;T: store&gt;(field: <a href="../ika_common/extended_field.md#ika_common_extended_field_ExtendedField">ExtendedField</a>&lt;T&gt;): T {
    <b>let</b> <a href="../ika_common/extended_field.md#ika_common_extended_field_ExtendedField">ExtendedField</a> { <b>mut</b> id } = field;
    <b>let</b> value = df::remove(&<b>mut</b> id, <a href="../ika_common/extended_field.md#ika_common_extended_field_Key">Key</a>());
    id.delete();
    value
}
</code></pre>



</details>
