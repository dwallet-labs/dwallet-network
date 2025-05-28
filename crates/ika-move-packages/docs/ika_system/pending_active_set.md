---
title: Module `(ika_system=0x0)::pending_active_set`
---

Contains an active set of validators. The active set is a smart collection
that only stores up to a max size of validators. The active set tracks the total amount of staked
IKA to make the calculation of the rewards and voting power distribution easier.


-  [Struct `PendingActiveSetEntry`](#(ika_system=0x0)_pending_active_set_PendingActiveSetEntry)
-  [Struct `PendingActiveSet`](#(ika_system=0x0)_pending_active_set_PendingActiveSet)
-  [Constants](#@Constants_0)
-  [Function `new`](#(ika_system=0x0)_pending_active_set_new)
-  [Function `insert_or_update_or_remove`](#(ika_system=0x0)_pending_active_set_insert_or_update_or_remove)
-  [Function `update_or_remove`](#(ika_system=0x0)_pending_active_set_update_or_remove)
-  [Function `update`](#(ika_system=0x0)_pending_active_set_update)
-  [Function `insert`](#(ika_system=0x0)_pending_active_set_insert)
-  [Function `remove`](#(ika_system=0x0)_pending_active_set_remove)
-  [Function `find_validator_index`](#(ika_system=0x0)_pending_active_set_find_validator_index)
-  [Function `insert_sorted`](#(ika_system=0x0)_pending_active_set_insert_sorted)
-  [Function `reposition_validator`](#(ika_system=0x0)_pending_active_set_reposition_validator)
-  [Function `set_max_validator_count`](#(ika_system=0x0)_pending_active_set_set_max_validator_count)
-  [Function `set_min_validator_count`](#(ika_system=0x0)_pending_active_set_set_min_validator_count)
-  [Function `set_max_validator_change_count`](#(ika_system=0x0)_pending_active_set_set_max_validator_change_count)
-  [Function `reset_validator_changes`](#(ika_system=0x0)_pending_active_set_reset_validator_changes)
-  [Function `max_validator_count`](#(ika_system=0x0)_pending_active_set_max_validator_count)
-  [Function `min_validator_count`](#(ika_system=0x0)_pending_active_set_min_validator_count)
-  [Function `max_validator_change_count`](#(ika_system=0x0)_pending_active_set_max_validator_change_count)
-  [Function `size`](#(ika_system=0x0)_pending_active_set_size)
-  [Function `active_ids`](#(ika_system=0x0)_pending_active_set_active_ids)
-  [Function `active_ids_and_stake`](#(ika_system=0x0)_pending_active_set_active_ids_and_stake)
-  [Function `set_min_validator_joining_stake`](#(ika_system=0x0)_pending_active_set_set_min_validator_joining_stake)
-  [Function `min_validator_joining_stake`](#(ika_system=0x0)_pending_active_set_min_validator_joining_stake)
-  [Function `total_stake`](#(ika_system=0x0)_pending_active_set_total_stake)


<pre><code><b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../sui/address.md#sui_address">sui::address</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
<b>use</b> <a href="../sui/vec_set.md#sui_vec_set">sui::vec_set</a>;
</code></pre>



<a name="(ika_system=0x0)_pending_active_set_PendingActiveSetEntry"></a>

## Struct `PendingActiveSetEntry`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSetEntry">PendingActiveSetEntry</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>staked_amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_pending_active_set_PendingActiveSet"></a>

## Struct `PendingActiveSet`

The active set of validators, a smart collection that only stores up
to a max size of validators.
Additionally, the active set tracks the total amount of staked IKA to make
the calculation of the rewards and voting power distribution easier.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">PendingActiveSet</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_count">min_validator_count</a>: u64</code>
</dt>
<dd>
 The minimum number of validators required in the active set.
</dd>
<dt>
<code><a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_count">max_validator_count</a>: u64</code>
</dt>
<dd>
 The maximum number of validators in the active set.
</dd>
<dt>
<code><a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_joining_stake">min_validator_joining_stake</a>: u64</code>
</dt>
<dd>
 The minimum amount of staked IKA needed to enter the active set. This is used to
 determine if a storage validator can be added to the active set.
</dd>
<dt>
<code><a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_change_count">max_validator_change_count</a>: u64</code>
</dt>
<dd>
 The maximum number of validators that can be added or removed to the active set in an epoch.
</dd>
<dt>
<code>validators: vector&lt;(ika_system=0x0)::<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSetEntry">pending_active_set::PendingActiveSetEntry</a>&gt;</code>
</dt>
<dd>
 The list of validators in the active set and their stake.
</dd>
<dt>
<code><a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_total_stake">total_stake</a>: u64</code>
</dt>
<dd>
 The total amount of staked IKA in the active set.
</dd>
<dt>
<code>validator_changes: <a href="../sui/vec_set.md#sui_vec_set_VecSet">sui::vec_set::VecSet</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;</code>
</dt>
<dd>
 The list of validators that have been added or removed to the active set in the current epoch.
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="(ika_system=0x0)_pending_active_set_EZeroMaxSize"></a>

The maximum size of an ActiveSet must be strictly larger than zero.


<pre><code><b>const</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_EZeroMaxSize">EZeroMaxSize</a>: u64 = 0;
</code></pre>



<a name="(ika_system=0x0)_pending_active_set_EDuplicateInsertion"></a>

The validator is already part of the active set.


<pre><code><b>const</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_EDuplicateInsertion">EDuplicateInsertion</a>: u64 = 1;
</code></pre>



<a name="(ika_system=0x0)_pending_active_set_EBelowMinValidatorCount"></a>

The minimum active set size must be maintained.


<pre><code><b>const</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_EBelowMinValidatorCount">EBelowMinValidatorCount</a>: u64 = 3;
</code></pre>



<a name="(ika_system=0x0)_pending_active_set_EMaxValidatorChangeReached"></a>

The maximum number of validator changes has been reached.


<pre><code><b>const</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_EMaxValidatorChangeReached">EMaxValidatorChangeReached</a>: u64 = 4;
</code></pre>



<a name="(ika_system=0x0)_pending_active_set_new"></a>

## Function `new`

Creates a new active set with the given <code><a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_count">min_validator_count</a></code>, <code><a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_count">max_validator_count</a></code>, <code><a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_joining_stake">min_validator_joining_stake</a></code>,
and <code><a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_change_count">max_validator_change_count</a></code>.
The <code><a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_joining_stake">min_validator_joining_stake</a></code> is used to filter out validators that do not have enough staked
IKA to be included in the active set initially. The <code><a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_change_count">max_validator_change_count</a></code> limits the number
of validator additions/removals per epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_new">new</a>(<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_count">min_validator_count</a>: u64, <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_count">max_validator_count</a>: u64, <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_joining_stake">min_validator_joining_stake</a>: u64, <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_change_count">max_validator_change_count</a>: u64): (ika_system=0x0)::<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">pending_active_set::PendingActiveSet</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_new">new</a>(
    <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_count">min_validator_count</a>: u64,
    <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_count">max_validator_count</a>: u64,
    <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_joining_stake">min_validator_joining_stake</a>: u64,
    <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_change_count">max_validator_change_count</a>: u64
): <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">PendingActiveSet</a> {
    <b>assert</b>!(<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_count">max_validator_count</a> &gt; 0, <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_EZeroMaxSize">EZeroMaxSize</a>);
    <b>assert</b>!(<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_count">min_validator_count</a> &lt;= <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_count">max_validator_count</a>, <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_EBelowMinValidatorCount">EBelowMinValidatorCount</a>);
    <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">PendingActiveSet</a> {
        <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_count">min_validator_count</a>,
        <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_count">max_validator_count</a>,
        <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_joining_stake">min_validator_joining_stake</a>,
        <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_change_count">max_validator_change_count</a>,
        validators: vector[],
        <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_total_stake">total_stake</a>: 0,
        validator_changes: vec_set::empty(),
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_pending_active_set_insert_or_update_or_remove"></a>

## Function `insert_or_update_or_remove`

Inserts the validator if it is not already in the active set, otherwise updates its stake.
If the validator's stake is below the threshold value, it attempts to remove it from the set
unless that would violate the minimum validator count.
Returns true if the validator is in the set after the operation, false otherwise.
Also returns the ID of any validator that was removed during the operation, or None if no validator was removed.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_insert_or_update_or_remove">insert_or_update_or_remove</a>(set: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">pending_active_set::PendingActiveSet</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, staked_amount: u64): (bool, <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_insert_or_update_or_remove">insert_or_update_or_remove</a>(set: &<b>mut</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">PendingActiveSet</a>, validator_id: ID, staked_amount: u64): (bool, Option&lt;ID&gt;) {
    // Currently, the `<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_joining_stake">min_validator_joining_stake</a>` is set to `0`, so we need to account <b>for</b> that.
    <b>if</b> (staked_amount == 0 || staked_amount &lt; set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_joining_stake">min_validator_joining_stake</a>) {
        <b>if</b> (set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_remove">remove</a>(validator_id)) {
            (<b>false</b>, option::some(validator_id))
        } <b>else</b> {
            (<b>false</b>, option::none())
        }
    } <b>else</b> <b>if</b> (set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_update">update</a>(validator_id, staked_amount)) {
        (<b>true</b>, option::none())
    } <b>else</b> {
        set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_insert">insert</a>(validator_id, staked_amount)
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_pending_active_set_update_or_remove"></a>

## Function `update_or_remove`

Updates the staked amount of the storage validator with the given <code>validator_id</code> in
the active set. Returns true if the validator is in the set.
Also returns the ID of any validator that was removed during the operation, or None if no validator was removed.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_update_or_remove">update_or_remove</a>(set: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">pending_active_set::PendingActiveSet</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, staked_amount: u64): (bool, <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_update_or_remove">update_or_remove</a>(set: &<b>mut</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">PendingActiveSet</a>, validator_id: ID, staked_amount: u64): (bool, Option&lt;ID&gt;) {
    <b>if</b> (staked_amount == 0 || staked_amount &lt; set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_joining_stake">min_validator_joining_stake</a>) {
        <b>if</b> (set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_remove">remove</a>(validator_id)) {
            (<b>false</b>, option::some(validator_id))
        } <b>else</b> {
            (<b>false</b>, option::none())
        }
    } <b>else</b> {
        (set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_update">update</a>(validator_id, staked_amount), option::none())
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_pending_active_set_update"></a>

## Function `update`

Updates the staked amount of the storage validator with the given <code>validator_id</code> in
the active set. Returns true if the validator is in the set.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_update">update</a>(set: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">pending_active_set::PendingActiveSet</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, staked_amount: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_update">update</a>(set: &<b>mut</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">PendingActiveSet</a>, validator_id: ID, staked_amount: u64): bool {
    <b>let</b> index = set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_find_validator_index">find_validator_index</a>(validator_id);
    <b>if</b> (index.is_none()) {
        <b>return</b> <b>false</b>
    };
    index.do!(|idx| {
        set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_total_stake">total_stake</a> = set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_total_stake">total_stake</a> + staked_amount - set.validators[idx].staked_amount;
        set.validators[idx].staked_amount = staked_amount;
        // Re-sort the <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> in its <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_new">new</a> position
        set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_reposition_validator">reposition_validator</a>(idx);
    });
    <b>true</b>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_pending_active_set_insert"></a>

## Function `insert`

Inserts a storage validator with the given <code>validator_id</code> and <code>staked_amount</code> into the
active set. The validator is only added if it has enough staked IKA to be included
in the active set. If the active set is full, the validator with the smallest
staked IKA is removed to make space for the new validator.
Returns true if the validator was inserted, false otherwise.
Also returns the ID of any validator that was removed during the operation, or None if no validator was removed.


<pre><code><b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_insert">insert</a>(set: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">pending_active_set::PendingActiveSet</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, staked_amount: u64): (bool, <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_insert">insert</a>(set: &<b>mut</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">PendingActiveSet</a>, validator_id: ID, staked_amount: u64): (bool, Option&lt;ID&gt;) {
    <b>assert</b>!(set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_find_validator_index">find_validator_index</a>(validator_id).is_none(), <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_EDuplicateInsertion">EDuplicateInsertion</a>);
    // If the validators are less than the max <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_size">size</a>, <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_insert">insert</a> the <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.
    <b>if</b> (set.validators.length() &lt; set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_count">max_validator_count</a>) {
        set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_total_stake">total_stake</a> = set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_total_stake">total_stake</a> + staked_amount;
        <b>let</b> new_entry = <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSetEntry">PendingActiveSetEntry</a> { validator_id, staked_amount };
        set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_insert_sorted">insert_sorted</a>(new_entry);
        <b>if</b> (!set.validator_changes.contains(&validator_id)) {
            set.validator_changes.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_insert">insert</a>(validator_id);
        };
        <b>assert</b>!(set.validator_changes.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_size">size</a>() &lt;= set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_change_count">max_validator_change_count</a>, <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_EMaxValidatorChangeReached">EMaxValidatorChangeReached</a>);
        <b>return</b> (<b>true</b>, option::none())
    };
    // If the <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_new">new</a> <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>'s stake is less than the smallest stake in the set, don't <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_insert">insert</a>
    <b>if</b> (staked_amount &lt;= set.validators[0].staked_amount) {
        <b>return</b> (<b>false</b>, option::none())
    };
    // Remove the <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> with smallest stake and <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_insert">insert</a> the <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_new">new</a> one
    <b>let</b> removed_validator_id = set.validators[0].validator_id;
    <b>let</b> removed_stake = set.validators[0].staked_amount;
    set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_total_stake">total_stake</a> = set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_total_stake">total_stake</a> - removed_stake + staked_amount;
    set.validators.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_remove">remove</a>(0);
    <b>let</b> new_entry = <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSetEntry">PendingActiveSetEntry</a> { validator_id, staked_amount };
    set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_insert_sorted">insert_sorted</a>(new_entry);
    <b>if</b> (!set.validator_changes.contains(&validator_id)) {
        set.validator_changes.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_insert">insert</a>(validator_id);
    };
    <b>assert</b>!(set.validator_changes.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_size">size</a>() &lt;= set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_change_count">max_validator_change_count</a>, <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_EMaxValidatorChangeReached">EMaxValidatorChangeReached</a>);
    (<b>true</b>, option::some(removed_validator_id))
}
</code></pre>



</details>

<a name="(ika_system=0x0)_pending_active_set_remove"></a>

## Function `remove`

Removes the storage validator with the given <code>validator_id</code> from the active set.
Will abort with EBelowMinValidatorCount if removing would bring the set below min_validator_count.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_remove">remove</a>(set: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">pending_active_set::PendingActiveSet</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_remove">remove</a>(set: &<b>mut</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">PendingActiveSet</a>, validator_id: ID): bool {
    <b>let</b> is_under_min_validator_count = set.validators.length() &lt; set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_count">min_validator_count</a>;
    <b>let</b> index = set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_find_validator_index">find_validator_index</a>(validator_id);
    <b>let</b> removed = index.is_some();
    index.do!(|idx| {
        <b>let</b> <b>entry</b> = set.validators.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_remove">remove</a>(idx);
        set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_total_stake">total_stake</a> = set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_total_stake">total_stake</a> - <b>entry</b>.staked_amount;
    });
    // Abort <b>if</b> removal would violate the minimum <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> count
    <b>assert</b>!(is_under_min_validator_count || set.validators.length() &gt;= set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_count">min_validator_count</a>, <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_EBelowMinValidatorCount">EBelowMinValidatorCount</a>);
    // Only track the change <b>if</b> the <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> was actually removed
    <b>if</b> (removed) {
        <b>if</b> (!set.validator_changes.contains(&validator_id)) {
            set.validator_changes.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_insert">insert</a>(validator_id);
        };
        <b>assert</b>!(set.validator_changes.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_size">size</a>() &lt;= set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_change_count">max_validator_change_count</a>, <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_EMaxValidatorChangeReached">EMaxValidatorChangeReached</a>);
    };
    removed
}
</code></pre>



</details>

<a name="(ika_system=0x0)_pending_active_set_find_validator_index"></a>

## Function `find_validator_index`

Finds the index of a validator in the active set using linear search.
Returns None if the validator is not found.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_find_validator_index">find_validator_index</a>(set: &(ika_system=0x0)::<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">pending_active_set::PendingActiveSet</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_find_validator_index">find_validator_index</a>(set: &<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">PendingActiveSet</a>, validator_id: ID): Option&lt;u64&gt; {
    <b>let</b> len = set.validators.length();
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>if</b> (set.validators[i].validator_id == validator_id) {
            <b>return</b> option::some(i)
        };
        i = i + 1;
    };
    option::none()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_pending_active_set_insert_sorted"></a>

## Function `insert_sorted`

Inserts a validator entry into the sorted vector at the correct position.


<pre><code><b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_insert_sorted">insert_sorted</a>(set: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">pending_active_set::PendingActiveSet</a>, <b>entry</b>: (ika_system=0x0)::<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSetEntry">pending_active_set::PendingActiveSetEntry</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_insert_sorted">insert_sorted</a>(set: &<b>mut</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">PendingActiveSet</a>, <b>entry</b>: <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSetEntry">PendingActiveSetEntry</a>) {
    <b>let</b> <b>mut</b> left = 0u64;
    <b>let</b> <b>mut</b> right = set.validators.length();
    <b>while</b> (left &lt; right) {
        <b>let</b> mid = (left + right) / 2;
        <b>if</b> (set.validators[mid].staked_amount &lt; <b>entry</b>.staked_amount) {
            left = mid + 1
        } <b>else</b> {
            right = mid
        }
    };
    // Manual <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_insert">insert</a> implementation:
    // Push to end, then shift elements to make space and place the <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_new">new</a> element
    vector::push_back(&<b>mut</b> set.validators, <b>entry</b>); // Temporarily add to end
    <b>let</b> len = set.validators.length();
    <b>if</b> (len &gt; 1) {
        <b>let</b> <b>mut</b> i = len - 1;
        <b>while</b> (i &gt; left) {
            vector::swap(&<b>mut</b> set.validators, i, i - 1);
            i = i - 1;
        }
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_pending_active_set_reposition_validator"></a>

## Function `reposition_validator`

Repositions a validator in the sorted vector after its stake has been updated.


<pre><code><b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_reposition_validator">reposition_validator</a>(set: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">pending_active_set::PendingActiveSet</a>, index: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_reposition_validator">reposition_validator</a>(set: &<b>mut</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">PendingActiveSet</a>, index: u64) {
    <b>let</b> <b>entry</b> = vector::remove(&<b>mut</b> set.validators, index);
    set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_insert_sorted">insert_sorted</a>(<b>entry</b>)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_pending_active_set_set_max_validator_count"></a>

## Function `set_max_validator_count`

Sets the maximum size of the active set.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_set_max_validator_count">set_max_validator_count</a>(set: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">pending_active_set::PendingActiveSet</a>, <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_count">max_validator_count</a>: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_set_max_validator_count">set_max_validator_count</a>(set: &<b>mut</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">PendingActiveSet</a>, <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_count">max_validator_count</a>: u64) {
    set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_count">max_validator_count</a> = <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_count">max_validator_count</a>;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_pending_active_set_set_min_validator_count"></a>

## Function `set_min_validator_count`

Sets the minimum number of validators required in the active set.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_set_min_validator_count">set_min_validator_count</a>(set: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">pending_active_set::PendingActiveSet</a>, <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_count">min_validator_count</a>: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_set_min_validator_count">set_min_validator_count</a>(set: &<b>mut</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">PendingActiveSet</a>, <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_count">min_validator_count</a>: u64) {
    set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_count">min_validator_count</a> = <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_count">min_validator_count</a>;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_pending_active_set_set_max_validator_change_count"></a>

## Function `set_max_validator_change_count`

Sets the maximum number of validator changes allowed per epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_set_max_validator_change_count">set_max_validator_change_count</a>(set: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">pending_active_set::PendingActiveSet</a>, <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_change_count">max_validator_change_count</a>: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_set_max_validator_change_count">set_max_validator_change_count</a>(set: &<b>mut</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">PendingActiveSet</a>, <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_change_count">max_validator_change_count</a>: u64) {
    set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_change_count">max_validator_change_count</a> = <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_change_count">max_validator_change_count</a>;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_pending_active_set_reset_validator_changes"></a>

## Function `reset_validator_changes`

Resets the validator changes count (typically called at the start of a new epoch).


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_reset_validator_changes">reset_validator_changes</a>(set: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">pending_active_set::PendingActiveSet</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_reset_validator_changes">reset_validator_changes</a>(set: &<b>mut</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">PendingActiveSet</a>) {
    set.validator_changes = vec_set::empty();
}
</code></pre>



</details>

<a name="(ika_system=0x0)_pending_active_set_max_validator_count"></a>

## Function `max_validator_count`

The maximum size of the active set.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_count">max_validator_count</a>(set: &(ika_system=0x0)::<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">pending_active_set::PendingActiveSet</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_count">max_validator_count</a>(set: &<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">PendingActiveSet</a>): u64 { set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_count">max_validator_count</a> }
</code></pre>



</details>

<a name="(ika_system=0x0)_pending_active_set_min_validator_count"></a>

## Function `min_validator_count`

The minimum number of validators required in the active set.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_count">min_validator_count</a>(set: &(ika_system=0x0)::<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">pending_active_set::PendingActiveSet</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_count">min_validator_count</a>(set: &<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">PendingActiveSet</a>): u64 { set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_count">min_validator_count</a> }
</code></pre>



</details>

<a name="(ika_system=0x0)_pending_active_set_max_validator_change_count"></a>

## Function `max_validator_change_count`

The maximum number of validator changes allowed per epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_change_count">max_validator_change_count</a>(set: &(ika_system=0x0)::<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">pending_active_set::PendingActiveSet</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_change_count">max_validator_change_count</a>(set: &<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">PendingActiveSet</a>): u64 { set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_max_validator_change_count">max_validator_change_count</a> }
</code></pre>



</details>

<a name="(ika_system=0x0)_pending_active_set_size"></a>

## Function `size`

The current size of the active set.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_size">size</a>(set: &(ika_system=0x0)::<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">pending_active_set::PendingActiveSet</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_size">size</a>(set: &<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">PendingActiveSet</a>): u64 { set.validators.length() }
</code></pre>



</details>

<a name="(ika_system=0x0)_pending_active_set_active_ids"></a>

## Function `active_ids`

The IDs of the validators in the active set.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_active_ids">active_ids</a>(set: &(ika_system=0x0)::<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">pending_active_set::PendingActiveSet</a>): vector&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_active_ids">active_ids</a>(set: &<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">PendingActiveSet</a>): vector&lt;ID&gt; {
    set.validators.map_ref!(|<a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>| <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a>.validator_id)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_pending_active_set_active_ids_and_stake"></a>

## Function `active_ids_and_stake`

The IDs and stake of the validators in the active set.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_active_ids_and_stake">active_ids_and_stake</a>(set: &(ika_system=0x0)::<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">pending_active_set::PendingActiveSet</a>): (vector&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;, vector&lt;u64&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_active_ids_and_stake">active_ids_and_stake</a>(set: &<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">PendingActiveSet</a>): (vector&lt;ID&gt;, vector&lt;u64&gt;) {
    <b>let</b> <b>mut</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_active_ids">active_ids</a> = vector[];
    <b>let</b> <b>mut</b> stake = vector[];
    set.validators.do_ref!(|<b>entry</b>| {
        <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_active_ids">active_ids</a>.push_back(<b>entry</b>.validator_id);
        stake.push_back(<b>entry</b>.staked_amount);
    });
    (<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_active_ids">active_ids</a>, stake)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_pending_active_set_set_min_validator_joining_stake"></a>

## Function `set_min_validator_joining_stake`

Sets the minimum amount of staked IKA in the active set.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_set_min_validator_joining_stake">set_min_validator_joining_stake</a>(set: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">pending_active_set::PendingActiveSet</a>, <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_joining_stake">min_validator_joining_stake</a>: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_set_min_validator_joining_stake">set_min_validator_joining_stake</a>(set: &<b>mut</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">PendingActiveSet</a>, <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_joining_stake">min_validator_joining_stake</a>: u64) {
    set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_joining_stake">min_validator_joining_stake</a> = <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_joining_stake">min_validator_joining_stake</a>;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_pending_active_set_min_validator_joining_stake"></a>

## Function `min_validator_joining_stake`

The minimum amount of staked IKA in the active set.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_joining_stake">min_validator_joining_stake</a>(set: &(ika_system=0x0)::<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">pending_active_set::PendingActiveSet</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_joining_stake">min_validator_joining_stake</a>(set: &<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">PendingActiveSet</a>): u64 { set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_min_validator_joining_stake">min_validator_joining_stake</a> }
</code></pre>



</details>

<a name="(ika_system=0x0)_pending_active_set_total_stake"></a>

## Function `total_stake`

The total amount of staked IKA in the active set.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_total_stake">total_stake</a>(set: &(ika_system=0x0)::<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">pending_active_set::PendingActiveSet</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_total_stake">total_stake</a>(set: &<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_PendingActiveSet">PendingActiveSet</a>): u64 { set.<a href="../ika_system/pending_active_set.md#(ika_system=0x0)_pending_active_set_total_stake">total_stake</a> }
</code></pre>



</details>
