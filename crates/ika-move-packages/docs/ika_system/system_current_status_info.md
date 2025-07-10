---
title: Module `(ika_system=0x0)::system_current_status_info`
---



-  [Struct `SystemCurrentStatusInfo`](#(ika_system=0x0)_system_current_status_info_SystemCurrentStatusInfo)
-  [Function `create`](#(ika_system=0x0)_system_current_status_info_create)
-  [Function `current_epoch`](#(ika_system=0x0)_system_current_status_info_current_epoch)
-  [Function `is_mid_epoch_time`](#(ika_system=0x0)_system_current_status_info_is_mid_epoch_time)
-  [Function `is_end_epoch_time`](#(ika_system=0x0)_system_current_status_info_is_end_epoch_time)
-  [Function `current_epoch_active_committee`](#(ika_system=0x0)_system_current_status_info_current_epoch_active_committee)
-  [Function `next_epoch_active_committee`](#(ika_system=0x0)_system_current_status_info_next_epoch_active_committee)


<pre><code><b>use</b> (ika_common=0x0)::bls_committee;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/u64.md#std_u64">std::u64</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../sui/address.md#sui_address">sui::address</a>;
<b>use</b> <a href="../sui/bcs.md#sui_bcs">sui::bcs</a>;
<b>use</b> <a href="../sui/bls12381.md#sui_bls12381">sui::bls12381</a>;
<b>use</b> <a href="../sui/event.md#sui_event">sui::event</a>;
<b>use</b> <a href="../sui/group_ops.md#sui_group_ops">sui::group_ops</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
</code></pre>



<a name="(ika_system=0x0)_system_current_status_info_SystemCurrentStatusInfo"></a>

## Struct `SystemCurrentStatusInfo`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_SystemCurrentStatusInfo">SystemCurrentStatusInfo</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_current_epoch">current_epoch</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_is_mid_epoch_time">is_mid_epoch_time</a>: bool</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_is_end_epoch_time">is_end_epoch_time</a>: bool</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_current_epoch_active_committee">current_epoch_active_committee</a>: (ika_common=0x0)::bls_committee::BlsCommittee</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_next_epoch_active_committee">next_epoch_active_committee</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;(ika_common=0x0)::bls_committee::BlsCommittee&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_system_current_status_info_create"></a>

## Function `create`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_create">create</a>(<a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_current_epoch">current_epoch</a>: u64, <a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_is_mid_epoch_time">is_mid_epoch_time</a>: bool, <a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_is_end_epoch_time">is_end_epoch_time</a>: bool, <a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_current_epoch_active_committee">current_epoch_active_committee</a>: (ika_common=0x0)::bls_committee::BlsCommittee, <a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_next_epoch_active_committee">next_epoch_active_committee</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;(ika_common=0x0)::bls_committee::BlsCommittee&gt;): (ika_system=0x0)::<a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_SystemCurrentStatusInfo">system_current_status_info::SystemCurrentStatusInfo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_create">create</a>(
    <a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_current_epoch">current_epoch</a>: u64,
    <a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_is_mid_epoch_time">is_mid_epoch_time</a>: bool,
    <a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_is_end_epoch_time">is_end_epoch_time</a>: bool,
    <a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_current_epoch_active_committee">current_epoch_active_committee</a>: BlsCommittee,
    <a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_next_epoch_active_committee">next_epoch_active_committee</a>: Option&lt;BlsCommittee&gt;,
): <a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_SystemCurrentStatusInfo">SystemCurrentStatusInfo</a> {
    <a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_SystemCurrentStatusInfo">SystemCurrentStatusInfo</a> {
        <a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_current_epoch">current_epoch</a>,
        <a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_is_mid_epoch_time">is_mid_epoch_time</a>,
        <a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_is_end_epoch_time">is_end_epoch_time</a>,
        <a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_current_epoch_active_committee">current_epoch_active_committee</a>,
        <a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_next_epoch_active_committee">next_epoch_active_committee</a>,
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_current_status_info_current_epoch"></a>

## Function `current_epoch`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_current_epoch">current_epoch</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_SystemCurrentStatusInfo">system_current_status_info::SystemCurrentStatusInfo</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_current_epoch">current_epoch</a>(self: &<a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_SystemCurrentStatusInfo">SystemCurrentStatusInfo</a>): u64 {
    self.<a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_current_epoch">current_epoch</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_current_status_info_is_mid_epoch_time"></a>

## Function `is_mid_epoch_time`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_is_mid_epoch_time">is_mid_epoch_time</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_SystemCurrentStatusInfo">system_current_status_info::SystemCurrentStatusInfo</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_is_mid_epoch_time">is_mid_epoch_time</a>(self: &<a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_SystemCurrentStatusInfo">SystemCurrentStatusInfo</a>): bool {
    self.<a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_is_mid_epoch_time">is_mid_epoch_time</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_current_status_info_is_end_epoch_time"></a>

## Function `is_end_epoch_time`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_is_end_epoch_time">is_end_epoch_time</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_SystemCurrentStatusInfo">system_current_status_info::SystemCurrentStatusInfo</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_is_end_epoch_time">is_end_epoch_time</a>(self: &<a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_SystemCurrentStatusInfo">SystemCurrentStatusInfo</a>): bool {
    self.<a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_is_end_epoch_time">is_end_epoch_time</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_current_status_info_current_epoch_active_committee"></a>

## Function `current_epoch_active_committee`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_current_epoch_active_committee">current_epoch_active_committee</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_SystemCurrentStatusInfo">system_current_status_info::SystemCurrentStatusInfo</a>): (ika_common=0x0)::bls_committee::BlsCommittee
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_current_epoch_active_committee">current_epoch_active_committee</a>(self: &<a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_SystemCurrentStatusInfo">SystemCurrentStatusInfo</a>): BlsCommittee {
    self.<a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_current_epoch_active_committee">current_epoch_active_committee</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_system_current_status_info_next_epoch_active_committee"></a>

## Function `next_epoch_active_committee`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_next_epoch_active_committee">next_epoch_active_committee</a>(self: &(ika_system=0x0)::<a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_SystemCurrentStatusInfo">system_current_status_info::SystemCurrentStatusInfo</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;(ika_common=0x0)::bls_committee::BlsCommittee&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_next_epoch_active_committee">next_epoch_active_committee</a>(self: &<a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_SystemCurrentStatusInfo">SystemCurrentStatusInfo</a>): Option&lt;BlsCommittee&gt; {
    self.<a href="../ika_system/system_current_status_info.md#(ika_system=0x0)_system_current_status_info_next_epoch_active_committee">next_epoch_active_committee</a>
}
</code></pre>



</details>
