---
title: Module `0x0::validator`
---



-  [Resource `Validator`](#0x0_validator_Validator)
-  [Constants](#@Constants_0)
-  [Function `create`](#0x0_validator_create)
-  [Function `load_validator_maybe_upgrade`](#0x0_validator_load_validator_maybe_upgrade)
-  [Function `destroy`](#0x0_validator_destroy)
-  [Function `upgrade_to_latest`](#0x0_validator_upgrade_to_latest)
-  [Function `version`](#0x0_validator_version)


<pre><code><b>use</b> <a href="validator_cap.md#0x0_validator_cap">0x0::validator_cap</a>;
<b>use</b> <a href="validator_inner.md#0x0_validator_inner_v1">0x0::validator_inner_v1</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../sui-framework/versioned.md#0x2_versioned">0x2::versioned</a>;
</code></pre>



<a name="0x0_validator_Validator"></a>

## Resource `Validator`



<pre><code><b>struct</b> <a href="validator.md#0x0_validator_Validator">Validator</a> <b>has</b> store, key
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
<code>inner: <a href="../sui-framework/versioned.md#0x2_versioned_Versioned">versioned::Versioned</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x0_validator_EInvalidVersion"></a>



<pre><code><b>const</b> <a href="validator.md#0x0_validator_EInvalidVersion">EInvalidVersion</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
</code></pre>



<a name="0x0_validator_VERSION"></a>

Flag to indicate the version of the ika validator.


<pre><code><b>const</b> <a href="validator.md#0x0_validator_VERSION">VERSION</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1;
</code></pre>



<a name="0x0_validator_create"></a>

## Function `create`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x0_validator_create">create</a>(payment_address: <b>address</b>, protocol_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, network_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, consensus_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, proof_of_possession_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, name: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, description: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, image_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, project_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, network_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, p2p_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, consensus_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, computation_price: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, commission_rate: u16, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): (<a href="validator.md#0x0_validator_Validator">validator::Validator</a>, <a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, <a href="validator_cap.md#0x0_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator.md#0x0_validator_create">create</a>(
    payment_address: <b>address</b>,
    protocol_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    network_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    consensus_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    proof_of_possession_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    name: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    description: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    image_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    project_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    network_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    p2p_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    consensus_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    computation_price: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    commission_rate: u16,
    ctx: &<b>mut</b> TxContext,
): (<a href="validator.md#0x0_validator_Validator">Validator</a>, ValidatorCap, ValidatorOperationCap) {
    <b>let</b> validator_uid = <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx);

    <b>let</b> validator_id = validator_uid.to_inner();

    <b>let</b> cap = <a href="validator_cap.md#0x0_validator_cap_new_validator_cap">validator_cap::new_validator_cap</a>(validator_id, ctx);

    <b>let</b> operation_cap = <a href="validator_cap.md#0x0_validator_cap_new_validator_operation_cap">validator_cap::new_validator_operation_cap</a>(
        validator_id,
        ctx,
    );

    <b>let</b> cap_id = <a href="../sui-framework/object.md#0x2_object_id">object::id</a>(&cap);
    <b>let</b> operation_cap_id = <a href="../sui-framework/object.md#0x2_object_id">object::id</a>(&operation_cap);

    <b>let</b> <a href="validator_inner.md#0x0_validator_inner_v1">validator_inner_v1</a> = <a href="validator_inner.md#0x0_validator_inner_v1_create">validator_inner_v1::create</a>(
        validator_id,
        cap_id,
        operation_cap_id,
        payment_address,
        protocol_pubkey_bytes,
        network_pubkey_bytes,
        consensus_pubkey_bytes,
        proof_of_possession_bytes,
        name,
        description,
        image_url,
        project_url,
        network_address,
        p2p_address,
        consensus_address,
        computation_price,
        commission_rate,
        ctx,
    );

    <b>let</b> <a href="validator.md#0x0_validator">validator</a> = <a href="validator.md#0x0_validator_Validator">Validator</a> {
        id: validator_uid,
        inner: <a href="../sui-framework/versioned.md#0x2_versioned_create">versioned::create</a>(<a href="validator.md#0x0_validator_VERSION">VERSION</a>, <a href="validator_inner.md#0x0_validator_inner_v1">validator_inner_v1</a>, ctx),
    };
    (<a href="validator.md#0x0_validator">validator</a>, cap, operation_cap)
}
</code></pre>



</details>

<a name="0x0_validator_load_validator_maybe_upgrade"></a>

## Function `load_validator_maybe_upgrade`

This function should always return the latest supported version.
If the inner version is old, we upgrade it lazily in-place.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x0_validator_load_validator_maybe_upgrade">load_validator_maybe_upgrade</a>(self: &<b>mut</b> <a href="validator.md#0x0_validator_Validator">validator::Validator</a>): &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator.md#0x0_validator_load_validator_maybe_upgrade">load_validator_maybe_upgrade</a>(self: &<b>mut</b> <a href="validator.md#0x0_validator_Validator">Validator</a>): &<b>mut</b> ValidatorInnerV1 {
    <a href="validator.md#0x0_validator_upgrade_to_latest">upgrade_to_latest</a>(self);
    <a href="../sui-framework/versioned.md#0x2_versioned_load_value_mut">versioned::load_value_mut</a>(&<b>mut</b> self.inner)
}
</code></pre>



</details>

<a name="0x0_validator_destroy"></a>

## Function `destroy`

Destroy the wrapper and retrieve the inner validator object.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x0_validator_destroy">destroy</a>(self: <a href="validator.md#0x0_validator_Validator">validator::Validator</a>): <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator.md#0x0_validator_destroy">destroy</a>(self: <a href="validator.md#0x0_validator_Validator">Validator</a>): ValidatorInnerV1 {
    <a href="validator.md#0x0_validator_upgrade_to_latest">upgrade_to_latest</a>(&self);
    <b>let</b> <a href="validator.md#0x0_validator_Validator">Validator</a> { id, inner } = self;
    id.delete();
    <a href="../sui-framework/versioned.md#0x2_versioned_destroy">versioned::destroy</a>(inner)
}
</code></pre>



</details>

<a name="0x0_validator_upgrade_to_latest"></a>

## Function `upgrade_to_latest`



<pre><code><b>fun</b> <a href="validator.md#0x0_validator_upgrade_to_latest">upgrade_to_latest</a>(self: &<a href="validator.md#0x0_validator_Validator">validator::Validator</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator.md#0x0_validator_upgrade_to_latest">upgrade_to_latest</a>(self: &<a href="validator.md#0x0_validator_Validator">Validator</a>) {
    <b>let</b> version = <a href="validator.md#0x0_validator_version">version</a>(self);
    // TODO: When new versions are added, we need <b>to</b> explicitly upgrade here.
    <b>assert</b>!(version == <a href="validator.md#0x0_validator_VERSION">VERSION</a>, <a href="validator.md#0x0_validator_EInvalidVersion">EInvalidVersion</a>);
}
</code></pre>



</details>

<a name="0x0_validator_version"></a>

## Function `version`



<pre><code><b>fun</b> <a href="validator.md#0x0_validator_version">version</a>(self: &<a href="validator.md#0x0_validator_Validator">validator::Validator</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator.md#0x0_validator_version">version</a>(self: &<a href="validator.md#0x0_validator_Validator">Validator</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <a href="../sui-framework/versioned.md#0x2_versioned_version">versioned::version</a>(&self.inner)
}
</code></pre>



</details>
