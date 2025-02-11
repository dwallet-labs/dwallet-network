---
title: Module `(ika_system=0x0)::validator`
---



-  [Struct `Validator`](#(ika_system=0x0)_validator_Validator)
-  [Constants](#@Constants_0)
-  [Function `create`](#(ika_system=0x0)_validator_create)
-  [Function `load_validator_maybe_upgrade`](#(ika_system=0x0)_validator_load_validator_maybe_upgrade)
-  [Function `destroy`](#(ika_system=0x0)_validator_destroy)
-  [Function `upgrade_to_latest`](#(ika_system=0x0)_validator_upgrade_to_latest)
-  [Function `version`](#(ika_system=0x0)_validator_version)


<pre><code><b>use</b> (ika=0x0)::ika;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/extended_field.md#(ika_system=0x0)_extended_field">extended_field</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap">validator_cap</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1">validator_inner_v1</a>;
<b>use</b> <a href="../../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../../std/u64.md#std_u64">std::u64</a>;
<b>use</b> <a href="../../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../../sui/address.md#sui_address">sui::address</a>;
<b>use</b> <a href="../../sui/bag.md#sui_bag">sui::bag</a>;
<b>use</b> <a href="../../sui/balance.md#sui_balance">sui::balance</a>;
<b>use</b> <a href="../../sui/bcs.md#sui_bcs">sui::bcs</a>;
<b>use</b> <a href="../../sui/bls12381.md#sui_bls12381">sui::bls12381</a>;
<b>use</b> <a href="../../sui/coin.md#sui_coin">sui::coin</a>;
<b>use</b> <a href="../../sui/config.md#sui_config">sui::config</a>;
<b>use</b> <a href="../../sui/deny_list.md#sui_deny_list">sui::deny_list</a>;
<b>use</b> <a href="../../sui/dynamic_field.md#sui_dynamic_field">sui::dynamic_field</a>;
<b>use</b> <a href="../../sui/dynamic_object_field.md#sui_dynamic_object_field">sui::dynamic_object_field</a>;
<b>use</b> <a href="../../sui/event.md#sui_event">sui::event</a>;
<b>use</b> <a href="../../sui/group_ops.md#sui_group_ops">sui::group_ops</a>;
<b>use</b> <a href="../../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../../sui/table.md#sui_table">sui::table</a>;
<b>use</b> <a href="../../sui/transfer.md#sui_transfer">sui::transfer</a>;
<b>use</b> <a href="../../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
<b>use</b> <a href="../../sui/types.md#sui_types">sui::types</a>;
<b>use</b> <a href="../../sui/url.md#sui_url">sui::url</a>;
<b>use</b> <a href="../../sui/vec_set.md#sui_vec_set">sui::vec_set</a>;
<b>use</b> <a href="../../sui/versioned.md#sui_versioned">sui::versioned</a>;
</code></pre>



<a name="(ika_system=0x0)_validator_Validator"></a>

## Struct `Validator`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>inner: <a href="../../sui/versioned.md#sui_versioned_Versioned">sui::versioned::Versioned</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="(ika_system=0x0)_validator_EInvalidVersion"></a>



<pre><code><b>const</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EInvalidVersion">EInvalidVersion</a>: u64 = 0;
</code></pre>



<a name="(ika_system=0x0)_validator_VERSION"></a>

Flag to indicate the version of the ika validator.


<pre><code><b>const</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_VERSION">VERSION</a>: u64 = 1;
</code></pre>



<a name="(ika_system=0x0)_validator_create"></a>

## Function `create`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_create">create</a>(payment_address: <b>address</b>, protocol_pubkey_bytes: vector&lt;u8&gt;, network_pubkey_bytes: vector&lt;u8&gt;, consensus_pubkey_bytes: vector&lt;u8&gt;, class_groups_pubkey_and_proof_bytes: vector&lt;u8&gt;, proof_of_possession_bytes: vector&lt;u8&gt;, name: vector&lt;u8&gt;, description: vector&lt;u8&gt;, image_url: vector&lt;u8&gt;, project_url: vector&lt;u8&gt;, network_address: vector&lt;u8&gt;, p2p_address: vector&lt;u8&gt;, consensus_address: vector&lt;u8&gt;, computation_price: u64, commission_rate: u16, ctx: &<b>mut</b> <a href="../../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): ((ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>, (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_create">create</a>(
    payment_address: <b>address</b>,
    protocol_pubkey_bytes: vector&lt;u8&gt;,
    network_pubkey_bytes: vector&lt;u8&gt;,
    consensus_pubkey_bytes: vector&lt;u8&gt;,
    class_groups_pubkey_and_proof_bytes: vector&lt;u8&gt;,
    proof_of_possession_bytes: vector&lt;u8&gt;,
    name: vector&lt;u8&gt;,
    description: vector&lt;u8&gt;,
    image_url: vector&lt;u8&gt;,
    project_url: vector&lt;u8&gt;,
    network_address: vector&lt;u8&gt;,
    p2p_address: vector&lt;u8&gt;,
    consensus_address: vector&lt;u8&gt;,
    computation_price: u64,
    commission_rate: u16,
    ctx: &<b>mut</b> TxContext,
): (<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>, ValidatorCap, ValidatorOperationCap) {
    <b>let</b> validator_uid = object::new(ctx);
    <b>let</b> validator_id = validator_uid.to_inner();
    <b>let</b> cap = <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_new_validator_cap">validator_cap::new_validator_cap</a>(validator_id, ctx);
    <b>let</b> operation_cap = <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_new_validator_operation_cap">validator_cap::new_validator_operation_cap</a>(
        validator_id,
        ctx,
    );
    <b>let</b> cap_id = object::id(&cap);
    <b>let</b> operation_cap_id = object::id(&operation_cap);
    <b>let</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1">validator_inner_v1</a> = <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_create">validator_inner_v1::create</a>(
        validator_id,
        cap_id,
        operation_cap_id,
        payment_address,
        protocol_pubkey_bytes,
        network_pubkey_bytes,
        consensus_pubkey_bytes,
        class_groups_pubkey_and_proof_bytes,
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
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> = <a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a> {
        id: validator_uid,
        inner: versioned::create(<a href="../ika_system/validator.md#(ika_system=0x0)_validator_VERSION">VERSION</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1">validator_inner_v1</a>, ctx),
    };
    (<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>, cap, operation_cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_load_validator_maybe_upgrade"></a>

## Function `load_validator_maybe_upgrade`

This function should always return the latest supported version.
If the inner version is old, we upgrade it lazily in-place.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_load_validator_maybe_upgrade">load_validator_maybe_upgrade</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>): &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_load_validator_maybe_upgrade">load_validator_maybe_upgrade</a>(self: &<b>mut</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>): &<b>mut</b> ValidatorInnerV1 {
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator_upgrade_to_latest">upgrade_to_latest</a>(self);
    versioned::load_value_mut(&<b>mut</b> self.inner)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_destroy"></a>

## Function `destroy`

Destroy the wrapper and retrieve the inner validator object.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_destroy">destroy</a>(self: (ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>): (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_destroy">destroy</a>(self: <a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>): ValidatorInnerV1 {
    <a href="../ika_system/validator.md#(ika_system=0x0)_validator_upgrade_to_latest">upgrade_to_latest</a>(&self);
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a> { id, inner } = self;
    id.delete();
    versioned::destroy(inner)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_upgrade_to_latest"></a>

## Function `upgrade_to_latest`



<pre><code><b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_upgrade_to_latest">upgrade_to_latest</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_upgrade_to_latest">upgrade_to_latest</a>(self: &<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>) {
    <b>let</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_version">version</a> = <a href="../ika_system/validator.md#(ika_system=0x0)_validator_version">version</a>(self);
    // TODO: When new versions are added, we need to explicitly upgrade here.
    <b>assert</b>!(<a href="../ika_system/validator.md#(ika_system=0x0)_validator_version">version</a> == <a href="../ika_system/validator.md#(ika_system=0x0)_validator_VERSION">VERSION</a>, <a href="../ika_system/validator.md#(ika_system=0x0)_validator_EInvalidVersion">EInvalidVersion</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_version"></a>

## Function `version`



<pre><code><b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_version">version</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/validator.md#(ika_system=0x0)_validator_version">version</a>(self: &<a href="../ika_system/validator.md#(ika_system=0x0)_validator_Validator">Validator</a>): u64 {
    versioned::version(&self.inner)
}
</code></pre>



</details>
