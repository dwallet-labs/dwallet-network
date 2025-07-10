---
title: Module `(ika_system=0x0)::advance_epoch_approver`
---



-  [Struct `AdvanceEpochApprover`](#(ika_system=0x0)_advance_epoch_approver_AdvanceEpochApprover)
-  [Constants](#@Constants_0)
-  [Function `create`](#(ika_system=0x0)_advance_epoch_approver_create)
-  [Function `assert_all_witnesses_approved`](#(ika_system=0x0)_advance_epoch_approver_assert_all_witnesses_approved)
-  [Function `destroy`](#(ika_system=0x0)_advance_epoch_approver_destroy)
-  [Function `approve_advance_epoch_by_witness`](#(ika_system=0x0)_advance_epoch_approver_approve_advance_epoch_by_witness)


<pre><code><b>use</b> (ika=0x0)::ika;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../sui/address.md#sui_address">sui::address</a>;
<b>use</b> <a href="../sui/bag.md#sui_bag">sui::bag</a>;
<b>use</b> <a href="../sui/balance.md#sui_balance">sui::balance</a>;
<b>use</b> <a href="../sui/coin.md#sui_coin">sui::coin</a>;
<b>use</b> <a href="../sui/config.md#sui_config">sui::config</a>;
<b>use</b> <a href="../sui/deny_list.md#sui_deny_list">sui::deny_list</a>;
<b>use</b> <a href="../sui/dynamic_field.md#sui_dynamic_field">sui::dynamic_field</a>;
<b>use</b> <a href="../sui/dynamic_object_field.md#sui_dynamic_object_field">sui::dynamic_object_field</a>;
<b>use</b> <a href="../sui/event.md#sui_event">sui::event</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/party.md#sui_party">sui::party</a>;
<b>use</b> <a href="../sui/table.md#sui_table">sui::table</a>;
<b>use</b> <a href="../sui/transfer.md#sui_transfer">sui::transfer</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
<b>use</b> <a href="../sui/types.md#sui_types">sui::types</a>;
<b>use</b> <a href="../sui/url.md#sui_url">sui::url</a>;
<b>use</b> <a href="../sui/vec_map.md#sui_vec_map">sui::vec_map</a>;
<b>use</b> <a href="../sui/vec_set.md#sui_vec_set">sui::vec_set</a>;
</code></pre>



<a name="(ika_system=0x0)_advance_epoch_approver_AdvanceEpochApprover"></a>

## Struct `AdvanceEpochApprover`

This struct is an Hot-Potato that is passed around during epoch advancement.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver_AdvanceEpochApprover">AdvanceEpochApprover</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>remaining_witnesses_to_approve: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>balance_ika: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="(ika_system=0x0)_advance_epoch_approver_EWitnessIsNotInApprover"></a>

Witness is not in the approver.


<pre><code><b>const</b> <a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver_EWitnessIsNotInApprover">EWitnessIsNotInApprover</a>: u64 = 0;
</code></pre>



<a name="(ika_system=0x0)_advance_epoch_approver_create"></a>

## Function `create`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver_create">create</a>(remaining_witnesses_to_approve: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, balance_ika: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;): (ika_system=0x0)::<a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver_AdvanceEpochApprover">advance_epoch_approver::AdvanceEpochApprover</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver_create">create</a>(
    remaining_witnesses_to_approve: vector&lt;String&gt;,
    balance_ika: Balance&lt;IKA&gt;,
): <a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver_AdvanceEpochApprover">AdvanceEpochApprover</a> {
    <a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver_AdvanceEpochApprover">AdvanceEpochApprover</a> {
        remaining_witnesses_to_approve,
        balance_ika,
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_advance_epoch_approver_assert_all_witnesses_approved"></a>

## Function `assert_all_witnesses_approved`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver_assert_all_witnesses_approved">assert_all_witnesses_approved</a>(self: &(ika_system=0x0)::<a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver_AdvanceEpochApprover">advance_epoch_approver::AdvanceEpochApprover</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver_assert_all_witnesses_approved">assert_all_witnesses_approved</a>(self: &<a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver_AdvanceEpochApprover">AdvanceEpochApprover</a>) {
    <b>assert</b>!(self.remaining_witnesses_to_approve.is_empty(), <a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver_EWitnessIsNotInApprover">EWitnessIsNotInApprover</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_advance_epoch_approver_destroy"></a>

## Function `destroy`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver_destroy">destroy</a>(self: (ika_system=0x0)::<a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver_AdvanceEpochApprover">advance_epoch_approver::AdvanceEpochApprover</a>): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver_destroy">destroy</a>(self: <a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver_AdvanceEpochApprover">AdvanceEpochApprover</a>): Balance&lt;IKA&gt; {
    <b>let</b> <a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver_AdvanceEpochApprover">AdvanceEpochApprover</a> {
        balance_ika,
        ..
    } = self;
    balance_ika
}
</code></pre>



</details>

<a name="(ika_system=0x0)_advance_epoch_approver_approve_advance_epoch_by_witness"></a>

## Function `approve_advance_epoch_by_witness`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver_approve_advance_epoch_by_witness">approve_advance_epoch_by_witness</a>&lt;Witness: drop&gt;(<a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver">advance_epoch_approver</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver_AdvanceEpochApprover">advance_epoch_approver::AdvanceEpochApprover</a>, _: Witness, balance_ika: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver_approve_advance_epoch_by_witness">approve_advance_epoch_by_witness</a>&lt;Witness: drop&gt;(
    <a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver">advance_epoch_approver</a>: &<b>mut</b> <a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver_AdvanceEpochApprover">AdvanceEpochApprover</a>,
    _: Witness,
    balance_ika: Balance&lt;IKA&gt;,
) {
    <b>let</b> witness_type = type_name::get_with_original_ids&lt;Witness&gt;();
    <b>let</b> witness_type_name = witness_type.into_string().to_string();
    <b>let</b> (is_found, index) = <a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver">advance_epoch_approver</a>.remaining_witnesses_to_approve.index_of(&witness_type_name);
    <b>assert</b>!(is_found, <a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver_EWitnessIsNotInApprover">EWitnessIsNotInApprover</a>);
    <a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver">advance_epoch_approver</a>.remaining_witnesses_to_approve.remove(index);
    <a href="../ika_system/advance_epoch_approver.md#(ika_system=0x0)_advance_epoch_approver">advance_epoch_approver</a>.balance_ika.join(balance_ika);
}
</code></pre>



</details>
