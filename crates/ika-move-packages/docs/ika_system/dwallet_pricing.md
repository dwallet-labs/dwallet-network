---
title: Module `(ika_dwallet_2pc_mpc=0x0)::dwallet_pricing`
---

This module provides structures and functions for managing pricing information for a dWallet.
Each operation (e.g., DKG, re-encrypt user share, ECDSA presign, etc.) has its own pricing data,
represented by a <code>PricingPerOperation</code>. Each <code>PricingPerOperation</code> holds three values:
- **fee_ika**: The IKA fee for the operation.
- **gas_fee_reimbursement_sui**: The SUI reimbursement.
- **gas_fee_reimbursement_sui_for_system_calls**: The SUI reimbursement for system calls.

The main struct, <code><a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">DWalletPricing</a></code>, now holds one <code>PricingPerOperation</code> per operation.
The DKG operation is split into two separate rounds:
- <code>dkg_first_round</code>
- <code>dkg_second_round</code>


-  [Struct `DWalletPricing`](#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing)
-  [Struct `DWalletPricingKey`](#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingKey)
-  [Struct `DWalletPricingValue`](#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingValue)
-  [Struct `DWalletPricingCalculationVotes`](#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingCalculationVotes)
-  [Function `empty`](#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_empty)
    -  [Parameters](#@Parameters_0)
    -  [Returns](#@Returns_1)
-  [Function `insert_or_update_dwallet_pricing`](#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_insert_or_update_dwallet_pricing)
    -  [Parameters](#@Parameters_2)
    -  [Returns](#@Returns_3)
-  [Function `try_get_dwallet_pricing_value`](#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_try_get_dwallet_pricing_value)
    -  [Parameters](#@Parameters_4)
    -  [Returns](#@Returns_5)
-  [Function `fee_ika`](#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_fee_ika)
-  [Function `gas_fee_reimbursement_sui`](#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_gas_fee_reimbursement_sui)
-  [Function `gas_fee_reimbursement_sui_for_system_calls`](#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_gas_fee_reimbursement_sui_for_system_calls)
-  [Function `new_pricing_calculation`](#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_new_pricing_calculation)
-  [Function `committee_members_for_pricing_calculation_votes`](#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_committee_members_for_pricing_calculation_votes)
-  [Function `calculate_pricing_quorum_below`](#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_calculate_pricing_quorum_below)
-  [Function `pricing_value_quorum_below`](#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_pricing_value_quorum_below)
-  [Function `is_calculation_completed`](#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_is_calculation_completed)
-  [Function `calculated_pricing`](#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_calculated_pricing)
-  [Function `insert_or_update_dwallet_pricing_value`](#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_insert_or_update_dwallet_pricing_value)
-  [Function `quorum_below`](#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_quorum_below)


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
<b>use</b> <a href="../sui/priority_queue.md#sui_priority_queue">sui::priority_queue</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
<b>use</b> <a href="../sui/vec_map.md#sui_vec_map">sui::vec_map</a>;
</code></pre>



<a name="(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing"></a>

## Struct `DWalletPricing`

Holds pricing information for a dWallet.
The vector is indexed by the curve and signature algorithm and protocol.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">DWalletPricing</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>pricing_map: <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;(ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingKey">dwallet_pricing::DWalletPricingKey</a>, (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingValue">dwallet_pricing::DWalletPricingValue</a>&gt;</code>
</dt>
<dd>
 The pricing for each curve and signature algorithm and protocol.
 The first key is the curve, the second is the signature algorithm, the third is the protocol.
</dd>
</dl>


</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingKey"></a>

## Struct `DWalletPricingKey`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingKey">DWalletPricingKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>curve: u32</code>
</dt>
<dd>
</dd>
<dt>
<code>signature_algorithm: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u32&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>protocol: u32</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingValue"></a>

## Struct `DWalletPricingValue`

Holds pricing information for a single operation.
The fields are ordered so that the consensus validation price is first.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingValue">DWalletPricingValue</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_fee_ika">fee_ika</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_gas_fee_reimbursement_sui">gas_fee_reimbursement_sui</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_gas_fee_reimbursement_sui_for_system_calls">gas_fee_reimbursement_sui_for_system_calls</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingCalculationVotes"></a>

## Struct `DWalletPricingCalculationVotes`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingCalculationVotes">DWalletPricingCalculationVotes</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>bls_committee: (ika_common=0x0)::bls_committee::BlsCommittee</code>
</dt>
<dd>
</dd>
<dt>
<code>default_pricing: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">dwallet_pricing::DWalletPricing</a></code>
</dt>
<dd>
</dd>
<dt>
<code>working_pricing: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">dwallet_pricing::DWalletPricing</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_empty"></a>

## Function `empty`

Creates a new [<code><a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">DWalletPricing</a></code>] object.

Initializes the table with the given pricing values for each operation.


<a name="@Parameters_0"></a>

### Parameters


- <code>ctx</code>: The transaction context.


<a name="@Returns_1"></a>

### Returns


A newly created instance of <code><a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">DWalletPricing</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_empty">empty</a>(): (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">dwallet_pricing::DWalletPricing</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_empty">empty</a>(): <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">DWalletPricing</a> {
    <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">DWalletPricing</a> {
        pricing_map: vec_map::empty(),
    }
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_insert_or_update_dwallet_pricing"></a>

## Function `insert_or_update_dwallet_pricing`

Inserts pricing information for a specific operation into the [<code><a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">DWalletPricing</a></code>] table.


<a name="@Parameters_2"></a>

### Parameters


- <code>self</code>: The [<code><a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">DWalletPricing</a></code>] object.
- <code>key</code>: The key for the operation.
- <code>value</code>: The pricing information for the operation.


<a name="@Returns_3"></a>

### Returns


The [<code><a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">DWalletPricing</a></code>] object.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_insert_or_update_dwallet_pricing">insert_or_update_dwallet_pricing</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">dwallet_pricing::DWalletPricing</a>, curve: u32, signature_algorithm: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u32&gt;, protocol: u32, <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_fee_ika">fee_ika</a>: u64, <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_gas_fee_reimbursement_sui">gas_fee_reimbursement_sui</a>: u64, <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_gas_fee_reimbursement_sui_for_system_calls">gas_fee_reimbursement_sui_for_system_calls</a>: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_insert_or_update_dwallet_pricing">insert_or_update_dwallet_pricing</a>(self: &<b>mut</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">DWalletPricing</a>, curve: u32, signature_algorithm: Option&lt;u32&gt;, protocol: u32, <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_fee_ika">fee_ika</a>: u64, <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_gas_fee_reimbursement_sui">gas_fee_reimbursement_sui</a>: u64, <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_gas_fee_reimbursement_sui_for_system_calls">gas_fee_reimbursement_sui_for_system_calls</a>: u64) {
    self.<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_insert_or_update_dwallet_pricing_value">insert_or_update_dwallet_pricing_value</a>(curve, signature_algorithm, protocol, <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingValue">DWalletPricingValue</a> {
        <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_fee_ika">fee_ika</a>,
        <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_gas_fee_reimbursement_sui">gas_fee_reimbursement_sui</a>,
        <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_gas_fee_reimbursement_sui_for_system_calls">gas_fee_reimbursement_sui_for_system_calls</a>,
    })
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_try_get_dwallet_pricing_value"></a>

## Function `try_get_dwallet_pricing_value`

Returns the pricing information for a specific operation from the [<code><a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">DWalletPricing</a></code>] table.


<a name="@Parameters_4"></a>

### Parameters


- <code>self</code>: The [<code><a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">DWalletPricing</a></code>] object.
- <code>key</code>: The key for the operation.


<a name="@Returns_5"></a>

### Returns


The pricing information for the operation.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_try_get_dwallet_pricing_value">try_get_dwallet_pricing_value</a>(self: &(ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">dwallet_pricing::DWalletPricing</a>, curve: u32, signature_algorithm: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u32&gt;, protocol: u32): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;(ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingValue">dwallet_pricing::DWalletPricingValue</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_try_get_dwallet_pricing_value">try_get_dwallet_pricing_value</a>(self: &<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">DWalletPricing</a>, curve: u32, signature_algorithm: Option&lt;u32&gt;, protocol: u32): Option&lt;<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingValue">DWalletPricingValue</a>&gt; {
    <b>let</b> key = <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingKey">DWalletPricingKey</a> {
        curve,
        signature_algorithm,
        protocol,
    };
    self.pricing_map.try_get(&key)
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_fee_ika"></a>

## Function `fee_ika`

Getter for the fee_ika field of a DWalletPricingValue.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_fee_ika">fee_ika</a>(self: &(ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingValue">dwallet_pricing::DWalletPricingValue</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_fee_ika">fee_ika</a>(self: &<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingValue">DWalletPricingValue</a>): u64 {
    self.<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_fee_ika">fee_ika</a>
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_gas_fee_reimbursement_sui"></a>

## Function `gas_fee_reimbursement_sui`

Getter for the gas_fee_reimbursement_sui field of a DWalletPricingValue.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_gas_fee_reimbursement_sui">gas_fee_reimbursement_sui</a>(self: &(ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingValue">dwallet_pricing::DWalletPricingValue</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_gas_fee_reimbursement_sui">gas_fee_reimbursement_sui</a>(self: &<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingValue">DWalletPricingValue</a>): u64 {
    self.<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_gas_fee_reimbursement_sui">gas_fee_reimbursement_sui</a>
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_gas_fee_reimbursement_sui_for_system_calls"></a>

## Function `gas_fee_reimbursement_sui_for_system_calls`

Getter for the gas_fee_reimbursement_sui_for_system_calls field of a DWalletPricingValue.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_gas_fee_reimbursement_sui_for_system_calls">gas_fee_reimbursement_sui_for_system_calls</a>(self: &(ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingValue">dwallet_pricing::DWalletPricingValue</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_gas_fee_reimbursement_sui_for_system_calls">gas_fee_reimbursement_sui_for_system_calls</a>(self: &<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingValue">DWalletPricingValue</a>): u64 {
    self.<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_gas_fee_reimbursement_sui_for_system_calls">gas_fee_reimbursement_sui_for_system_calls</a>
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_new_pricing_calculation"></a>

## Function `new_pricing_calculation`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_new_pricing_calculation">new_pricing_calculation</a>(bls_committee: (ika_common=0x0)::bls_committee::BlsCommittee, default_pricing: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">dwallet_pricing::DWalletPricing</a>): (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingCalculationVotes">dwallet_pricing::DWalletPricingCalculationVotes</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_new_pricing_calculation">new_pricing_calculation</a>(bls_committee: BlsCommittee, default_pricing: <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">DWalletPricing</a>): <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingCalculationVotes">DWalletPricingCalculationVotes</a> {
    <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingCalculationVotes">DWalletPricingCalculationVotes</a> {
        bls_committee,
        default_pricing,
        working_pricing: <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_empty">empty</a>(),
    }
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_committee_members_for_pricing_calculation_votes"></a>

## Function `committee_members_for_pricing_calculation_votes`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_committee_members_for_pricing_calculation_votes">committee_members_for_pricing_calculation_votes</a>(calculation: &(ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingCalculationVotes">dwallet_pricing::DWalletPricingCalculationVotes</a>): vector&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_committee_members_for_pricing_calculation_votes">committee_members_for_pricing_calculation_votes</a>(calculation: &<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingCalculationVotes">DWalletPricingCalculationVotes</a>): vector&lt;ID&gt; {
    calculation.bls_committee.members().map_ref!(|member| {
        member.validator_id()
    })
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_calculate_pricing_quorum_below"></a>

## Function `calculate_pricing_quorum_below`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_calculate_pricing_quorum_below">calculate_pricing_quorum_below</a>(calculation: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingCalculationVotes">dwallet_pricing::DWalletPricingCalculationVotes</a>, pricing: vector&lt;(ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">dwallet_pricing::DWalletPricing</a>&gt;, curve: u32, signature_algorithm: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u32&gt;, protocol: u32)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_calculate_pricing_quorum_below">calculate_pricing_quorum_below</a>(calculation: &<b>mut</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingCalculationVotes">DWalletPricingCalculationVotes</a>, pricing: vector&lt;<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">DWalletPricing</a>&gt;, curve: u32, signature_algorithm: Option&lt;u32&gt;, protocol: u32) {
    <b>let</b> <b>mut</b> values = vector[];
    pricing.do_ref!(|pricing| {
        <b>let</b> value = pricing.<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_try_get_dwallet_pricing_value">try_get_dwallet_pricing_value</a>(curve, signature_algorithm, protocol);
        values.push_back(value.get_with_default(calculation.default_pricing.<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_try_get_dwallet_pricing_value">try_get_dwallet_pricing_value</a>(curve, signature_algorithm, protocol).extract()));
    });
    <b>let</b> value = <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_pricing_value_quorum_below">pricing_value_quorum_below</a>(calculation.bls_committee, values);
    calculation.working_pricing.<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_insert_or_update_dwallet_pricing_value">insert_or_update_dwallet_pricing_value</a>(curve, signature_algorithm, protocol, value);
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_pricing_value_quorum_below"></a>

## Function `pricing_value_quorum_below`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_pricing_value_quorum_below">pricing_value_quorum_below</a>(bls_committee: (ika_common=0x0)::bls_committee::BlsCommittee, values: vector&lt;(ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingValue">dwallet_pricing::DWalletPricingValue</a>&gt;): (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingValue">dwallet_pricing::DWalletPricingValue</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_pricing_value_quorum_below">pricing_value_quorum_below</a>(bls_committee: BlsCommittee, values: vector&lt;<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingValue">DWalletPricingValue</a>&gt;): <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingValue">DWalletPricingValue</a> {
    <b>let</b> <b>mut</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_fee_ika">fee_ika</a> = priority_queue::new(vector[]);
    <b>let</b> <b>mut</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_gas_fee_reimbursement_sui">gas_fee_reimbursement_sui</a> = priority_queue::new(vector[]);
    <b>let</b> <b>mut</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_gas_fee_reimbursement_sui_for_system_calls">gas_fee_reimbursement_sui_for_system_calls</a> = priority_queue::new(vector[]);
    values.do_ref!(|value| {
        <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_fee_ika">fee_ika</a>.insert(value.<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_fee_ika">fee_ika</a>(), 1);
        <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_gas_fee_reimbursement_sui">gas_fee_reimbursement_sui</a>.insert(value.<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_gas_fee_reimbursement_sui">gas_fee_reimbursement_sui</a>(), 1);
        <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_gas_fee_reimbursement_sui_for_system_calls">gas_fee_reimbursement_sui_for_system_calls</a>.insert(value.<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_gas_fee_reimbursement_sui_for_system_calls">gas_fee_reimbursement_sui_for_system_calls</a>(), 1);
    });
    <b>let</b> fee_ika_quorum_below = <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_quorum_below">quorum_below</a>(bls_committee, &<b>mut</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_fee_ika">fee_ika</a>);
    <b>let</b> gas_fee_reimbursement_sui_quorum_below = <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_quorum_below">quorum_below</a>(bls_committee, &<b>mut</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_gas_fee_reimbursement_sui">gas_fee_reimbursement_sui</a>);
    <b>let</b> gas_fee_reimbursement_sui_for_system_calls_quorum_below = <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_quorum_below">quorum_below</a>(bls_committee, &<b>mut</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_gas_fee_reimbursement_sui_for_system_calls">gas_fee_reimbursement_sui_for_system_calls</a>);
    <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingValue">DWalletPricingValue</a> {
        <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_fee_ika">fee_ika</a>: fee_ika_quorum_below,
        <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_gas_fee_reimbursement_sui">gas_fee_reimbursement_sui</a>: gas_fee_reimbursement_sui_quorum_below,
        <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_gas_fee_reimbursement_sui_for_system_calls">gas_fee_reimbursement_sui_for_system_calls</a>: gas_fee_reimbursement_sui_for_system_calls_quorum_below,
    }
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_is_calculation_completed"></a>

## Function `is_calculation_completed`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_is_calculation_completed">is_calculation_completed</a>(calculation: &(ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingCalculationVotes">dwallet_pricing::DWalletPricingCalculationVotes</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_is_calculation_completed">is_calculation_completed</a>(calculation: &<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingCalculationVotes">DWalletPricingCalculationVotes</a>): bool {
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> (keys, _) = calculation.default_pricing.pricing_map.into_keys_values();
    <b>while</b> (i &lt; keys.length()) {
        <b>let</b> key = keys[i];
        <b>if</b>(calculation.working_pricing.pricing_map.try_get(&key).is_none()) {
            <b>return</b> <b>false</b>
        };
        i = i + 1;
    };
    <b>true</b>
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_calculated_pricing"></a>

## Function `calculated_pricing`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_calculated_pricing">calculated_pricing</a>(calculation: &(ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingCalculationVotes">dwallet_pricing::DWalletPricingCalculationVotes</a>): (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">dwallet_pricing::DWalletPricing</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_calculated_pricing">calculated_pricing</a>(calculation: &<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingCalculationVotes">DWalletPricingCalculationVotes</a>): <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">DWalletPricing</a> {
    calculation.working_pricing
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_insert_or_update_dwallet_pricing_value"></a>

## Function `insert_or_update_dwallet_pricing_value`



<pre><code><b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_insert_or_update_dwallet_pricing_value">insert_or_update_dwallet_pricing_value</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">dwallet_pricing::DWalletPricing</a>, curve: u32, signature_algorithm: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u32&gt;, protocol: u32, value: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingValue">dwallet_pricing::DWalletPricingValue</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_insert_or_update_dwallet_pricing_value">insert_or_update_dwallet_pricing_value</a>(self: &<b>mut</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">DWalletPricing</a>, curve: u32, signature_algorithm: Option&lt;u32&gt;, protocol: u32, value: <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingValue">DWalletPricingValue</a>) {
    <b>let</b> key = <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricingKey">DWalletPricingKey</a> {
        curve,
        signature_algorithm,
        protocol,
    };
    <b>if</b>(self.pricing_map.contains(&key)) {
        <b>let</b> existing_value = &<b>mut</b> self.pricing_map[&key];
        *existing_value = value;
    } <b>else</b> {
        self.pricing_map.insert(key, value);
    };
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_quorum_below"></a>

## Function `quorum_below`

Take the lowest value, s.t. a quorum  (2f + 1) voted for a value lower or equal to this.


<pre><code><b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_quorum_below">quorum_below</a>(bls_committee: (ika_common=0x0)::bls_committee::BlsCommittee, vote_queue: &<b>mut</b> <a href="../sui/priority_queue.md#sui_priority_queue_PriorityQueue">sui::priority_queue::PriorityQueue</a>&lt;u64&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_quorum_below">quorum_below</a>(bls_committee: BlsCommittee, vote_queue: &<b>mut</b> PriorityQueue&lt;u64&gt;): u64 {
    <b>let</b> <b>mut</b> sum_votes = bls_committee.total_voting_power();
    // We have a quorum initially, so we remove nodes until doing so breaks the quorum.
    // The value at that point is the minimum value with support from a quorum.
    <b>loop</b> {
        <b>let</b> (value, votes) = vote_queue.pop_max();
        sum_votes = sum_votes - votes;
        <b>if</b> (!bls_committee.is_quorum_threshold(sum_votes)) {
            <b>return</b> value
        };
    }
}
</code></pre>



</details>
