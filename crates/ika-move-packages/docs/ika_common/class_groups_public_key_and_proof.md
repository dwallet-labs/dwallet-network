---
title: Module `ika_common::class_groups_public_key_and_proof`
---

This module defines the data structures for the Class Groups public key and proof.

The full Class Groups public key consists of 13 public keys, each with a corresponding proof.
Due to Sui's limitations on object size (≤ 250KB) and transaction size (≤ 128KB),
the full key must be split into parts and stored dynamically using <code>table_vec</code>.


-  [Struct `ClassGroupsPublicKeyAndProofBuilder`](#ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProofBuilder)
-  [Struct `ClassGroupsPublicKeyAndProof`](#ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof)
-  [Function `empty`](#ika_common_class_groups_public_key_and_proof_empty)
-  [Function `add_public_key_and_proof`](#ika_common_class_groups_public_key_and_proof_add_public_key_and_proof)
-  [Function `finish`](#ika_common_class_groups_public_key_and_proof_finish)
-  [Function `drop`](#ika_common_class_groups_public_key_and_proof_drop)
-  [Function `destroy`](#ika_common_class_groups_public_key_and_proof_destroy)


<pre><code><b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../sui/address.md#sui_address">sui::address</a>;
<b>use</b> <a href="../sui/dynamic_field.md#sui_dynamic_field">sui::dynamic_field</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/table.md#sui_table">sui::table</a>;
<b>use</b> <a href="../sui/table_vec.md#sui_table_vec">sui::table_vec</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
</code></pre>



<a name="ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProofBuilder"></a>

## Struct `ClassGroupsPublicKeyAndProofBuilder`

<code><a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProofBuilder">ClassGroupsPublicKeyAndProofBuilder</a></code> is used to construct a <code><a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">ClassGroupsPublicKeyAndProof</a></code> object.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProofBuilder">ClassGroupsPublicKeyAndProofBuilder</a> <b>has</b> key, store
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
<code>public_keys_and_proofs: <a href="../sui/table_vec.md#sui_table_vec_TableVec">sui::table_vec::TableVec</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
 A <code>TableVec</code> that dynamically stores public keys and their corresponding proofs.
</dd>
</dl>


</details>

<a name="ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof"></a>

## Struct `ClassGroupsPublicKeyAndProof`

<code><a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">ClassGroupsPublicKeyAndProof</a></code> stores the full Class Groups public key and proof.
This object can only be created using <code><a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProofBuilder">ClassGroupsPublicKeyAndProofBuilder</a></code>.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">ClassGroupsPublicKeyAndProof</a> <b>has</b> key, store
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
<code>public_keys_and_proofs: <a href="../sui/table_vec.md#sui_table_vec_TableVec">sui::table_vec::TableVec</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
 A <code>TableVec</code> that dynamically stores public keys and their corresponding proofs.
</dd>
</dl>


</details>

<a name="ika_common_class_groups_public_key_and_proof_empty"></a>

## Function `empty`

Creates a new <code><a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProofBuilder">ClassGroupsPublicKeyAndProofBuilder</a></code> instance.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_empty">empty</a>(ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProofBuilder">ika_common::class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProofBuilder</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_empty">empty</a>(
    ctx: &<b>mut</b> TxContext,
): <a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProofBuilder">ClassGroupsPublicKeyAndProofBuilder</a> {
    <a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProofBuilder">ClassGroupsPublicKeyAndProofBuilder</a> {
        id: object::new(ctx),
        public_keys_and_proofs: table_vec::empty(ctx),
    }
}
</code></pre>



</details>

<a name="ika_common_class_groups_public_key_and_proof_add_public_key_and_proof"></a>

## Function `add_public_key_and_proof`

Adds a public key and its corresponding proof to the <code><a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProofBuilder">ClassGroupsPublicKeyAndProofBuilder</a></code>.

Due to Sui's transaction argument size limit (≤ 16KB), each public key-proof pair
must be split into two parts before being stored.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_add_public_key_and_proof">add_public_key_and_proof</a>(self: &<b>mut</b> <a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProofBuilder">ika_common::class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProofBuilder</a>, public_key_and_proof_first_part: vector&lt;u8&gt;, public_key_and_proof_second_part: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_add_public_key_and_proof">add_public_key_and_proof</a>(
    self: &<b>mut</b> <a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProofBuilder">ClassGroupsPublicKeyAndProofBuilder</a>,
    public_key_and_proof_first_part: vector&lt;u8&gt;,
    public_key_and_proof_second_part: vector&lt;u8&gt;,
) {
    <b>let</b> <b>mut</b> full_public_key_and_proof = vector::empty();
    full_public_key_and_proof.append(public_key_and_proof_first_part);
    full_public_key_and_proof.append(public_key_and_proof_second_part);
    self.public_keys_and_proofs.push_back(full_public_key_and_proof);
}
</code></pre>



</details>

<a name="ika_common_class_groups_public_key_and_proof_finish"></a>

## Function `finish`

Finalizes the construction of a <code><a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">ClassGroupsPublicKeyAndProof</a></code> object.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_finish">finish</a>(self: <a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProofBuilder">ika_common::class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProofBuilder</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">ika_common::class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_finish">finish</a>(
    self: <a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProofBuilder">ClassGroupsPublicKeyAndProofBuilder</a>,
    ctx: &<b>mut</b> TxContext,
): <a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">ClassGroupsPublicKeyAndProof</a> {
    <b>let</b> <a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProofBuilder">ClassGroupsPublicKeyAndProofBuilder</a> { id, public_keys_and_proofs } = self;
    id.delete();
    <a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">ClassGroupsPublicKeyAndProof</a> {
        id: object::new(ctx),
        public_keys_and_proofs
    }
}
</code></pre>



</details>

<a name="ika_common_class_groups_public_key_and_proof_drop"></a>

## Function `drop`

Drops the <code><a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">ClassGroupsPublicKeyAndProof</a></code> object, removing all public keys and proofs before deletion.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_drop">drop</a>(self: <a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">ika_common::class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_drop">drop</a>(self: <a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">ClassGroupsPublicKeyAndProof</a>) {
    <b>let</b> <a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">ClassGroupsPublicKeyAndProof</a> { id, <b>mut</b> public_keys_and_proofs } = self;
    <b>while</b> (!public_keys_and_proofs.is_empty()) {
        public_keys_and_proofs.pop_back();
    };
    public_keys_and_proofs.destroy_empty();
    id.delete();
}
</code></pre>



</details>

<a name="ika_common_class_groups_public_key_and_proof_destroy"></a>

## Function `destroy`

Destroys the <code><a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">ClassGroupsPublicKeyAndProof</a></code> object, returning the stored public keys and proofs.

This function removes the object from storage and returns the <code>TableVec</code> containing
the public keys and their corresponding proofs.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_destroy">destroy</a>(self: <a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">ika_common::class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof</a>): <a href="../sui/table_vec.md#sui_table_vec_TableVec">sui::table_vec::TableVec</a>&lt;vector&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_destroy">destroy</a>(
    self: <a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">ClassGroupsPublicKeyAndProof</a>,
): table_vec::TableVec&lt;vector&lt;u8&gt;&gt; {
    <b>let</b> <a href="../ika_common/class_groups_public_key_and_proof.md#ika_common_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">ClassGroupsPublicKeyAndProof</a> { id, public_keys_and_proofs } = self;
    id.delete();
    public_keys_and_proofs
}
</code></pre>



</details>
