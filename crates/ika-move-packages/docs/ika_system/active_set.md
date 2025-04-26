---
title: Module `(ika_system=0x0)::active_set`
---

Contains an active set of validators. The active set is a smart collection
that only stores up to a 1000 validators. The active set tracks the total amount of staked
IKA to make the calculation of the rewards and voting power distribution easier.


-  [Struct `ActiveSetEntry`](#(ika_system=0x0)_active_set_ActiveSetEntry)
-  [Struct `ActiveSet`](#(ika_system=0x0)_active_set_ActiveSet)
-  [Constants](#@Constants_0)
-  [Function `new`](#(ika_system=0x0)_active_set_new)
-  [Function `insert_or_update`](#(ika_system=0x0)_active_set_insert_or_update)
-  [Function `update`](#(ika_system=0x0)_active_set_update)
-  [Function `insert`](#(ika_system=0x0)_active_set_insert)
-  [Function `remove`](#(ika_system=0x0)_active_set_remove)
-  [Function `find_validator_index`](#(ika_system=0x0)_active_set_find_validator_index)
-  [Function `insert_sorted`](#(ika_system=0x0)_active_set_insert_sorted)
-  [Function `reposition_validator`](#(ika_system=0x0)_active_set_reposition_validator)
-  [Function `max_size`](#(ika_system=0x0)_active_set_max_size)
-  [Function `size`](#(ika_system=0x0)_active_set_size)
-  [Function `active_ids`](#(ika_system=0x0)_active_set_active_ids)
-  [Function `active_ids_and_stake`](#(ika_system=0x0)_active_set_active_ids_and_stake)
-  [Function `min_stake`](#(ika_system=0x0)_active_set_min_stake)
-  [Function `total_stake`](#(ika_system=0x0)_active_set_total_stake)


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



<a name="(ika_system=0x0)_active_set_ActiveSetEntry"></a>

## Struct `ActiveSetEntry`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSetEntry">ActiveSetEntry</a> <b>has</b> <b>copy</b>, drop, store
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

<a name="(ika_system=0x0)_active_set_ActiveSet"></a>

## Struct `ActiveSet`

The active set of validators, a smart collection that only stores up
to a 1000 validators.
Additionally, the active set tracks the total amount of staked IKA to make
the calculation of the rewards and voting power distribution easier.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSet">ActiveSet</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_max_size">max_size</a>: u16</code>
</dt>
<dd>
 The maximum number of validators in the active set.
 Potentially remove this field.
</dd>
<dt>
<code><a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_min_stake">min_stake</a>: u64</code>
</dt>
<dd>
 The minimum amount of staked IKA needed to enter the active set. This is used to
 determine if a storage validator can be added to the active set.
</dd>
<dt>
<code>validators: vector&lt;(ika_system=0x0)::<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSetEntry">active_set::ActiveSetEntry</a>&gt;</code>
</dt>
<dd>
 The list of validators in the active set and their stake.
</dd>
<dt>
<code><a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_total_stake">total_stake</a>: u64</code>
</dt>
<dd>
 The total amount of staked IKA in the active set.
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="(ika_system=0x0)_active_set_EDuplicateInsertion"></a>

The validator is already part of the active set.


<pre><code><b>const</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_EDuplicateInsertion">EDuplicateInsertion</a>: u64 = 1;
</code></pre>



<a name="(ika_system=0x0)_active_set_EZeroMaxSize"></a>

The maximum size of an ActiveSet must be strictly larger than zero.


<pre><code><b>const</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_EZeroMaxSize">EZeroMaxSize</a>: u64 = 0;
</code></pre>



<a name="(ika_system=0x0)_active_set_new"></a>

## Function `new`

Creates a new active set with the given <code><a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_size">size</a></code> and <code><a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_min_stake">min_stake</a></code>. The
latter is used to filter out validators that do not have enough staked
IKA to be included in the active set initially.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_new">new</a>(<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_max_size">max_size</a>: u16, <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_min_stake">min_stake</a>: u64): (ika_system=0x0)::<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSet">active_set::ActiveSet</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_new">new</a>(<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_max_size">max_size</a>: u16, <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_min_stake">min_stake</a>: u64): <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSet">ActiveSet</a> {
    <b>assert</b>!(<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_max_size">max_size</a> &gt; 0, <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_EZeroMaxSize">EZeroMaxSize</a>);
    <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSet">ActiveSet</a> {
        <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_max_size">max_size</a>,
        <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_min_stake">min_stake</a>,
        validators: vector[],
        <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_total_stake">total_stake</a>: 0,
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_active_set_insert_or_update"></a>

## Function `insert_or_update`

Inserts the validator if it is not already in the active set, otherwise updates its stake.
If the validator's stake is below the threshold value, it is removed from the set.
Returns true if the validator is in the set after the operation, false otherwise.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_insert_or_update">insert_or_update</a>(set: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSet">active_set::ActiveSet</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, staked_amount: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_insert_or_update">insert_or_update</a>(set: &<b>mut</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSet">ActiveSet</a>, validator_id: ID, staked_amount: u64): bool {
    // Currently, the `<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_min_stake">min_stake</a>` is set to `0`, so we need to account <b>for</b> that.
    <b>if</b> (staked_amount == 0 || staked_amount &lt; set.<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_min_stake">min_stake</a>) {
        set.<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_remove">remove</a>(validator_id);
        <b>return</b> <b>false</b>
    };
    <b>if</b> (set.<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_update">update</a>(validator_id, staked_amount)) <b>true</b>
    <b>else</b> set.<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_insert">insert</a>(validator_id, staked_amount)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_active_set_update"></a>

## Function `update`

Updates the staked amount of the storage validator with the given <code>validator_id</code> in
the active set. Returns true if the validator is in the set.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_update">update</a>(set: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSet">active_set::ActiveSet</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, staked_amount: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_update">update</a>(set: &<b>mut</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSet">ActiveSet</a>, validator_id: ID, staked_amount: u64): bool {
    <b>let</b> index = set.<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_find_validator_index">find_validator_index</a>(validator_id);
    <b>if</b> (index.is_none()) {
        <b>return</b> <b>false</b>
    };
    index.do!(|idx| {
        set.<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_total_stake">total_stake</a> = set.<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_total_stake">total_stake</a> + staked_amount - set.validators[idx].staked_amount;
        set.validators[idx].staked_amount = staked_amount;
        // Re-sort the validator in its <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_new">new</a> position
        set.<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_reposition_validator">reposition_validator</a>(idx);
    });
    <b>true</b>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_active_set_insert"></a>

## Function `insert`

Inserts a storage validator with the given <code>validator_id</code> and <code>staked_amount</code> into the
active set. The validator is only added if it has enough staked IKA to be included
in the active set. If the active set is full, the validator with the smallest
staked IKA is removed to make space for the new validator.
Returns true if the validator was inserted, false otherwise.


<pre><code><b>fun</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_insert">insert</a>(set: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSet">active_set::ActiveSet</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, staked_amount: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_insert">insert</a>(set: &<b>mut</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSet">ActiveSet</a>, validator_id: ID, staked_amount: u64): bool {
    <b>assert</b>!(set.<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_find_validator_index">find_validator_index</a>(validator_id).is_none(), <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_EDuplicateInsertion">EDuplicateInsertion</a>);
    // If the validators are less than the max <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_size">size</a>, <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_insert">insert</a> the validator.
    <b>if</b> (set.validators.length() <b>as</b> u16 &lt; set.<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_max_size">max_size</a>) {
        set.<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_total_stake">total_stake</a> = set.<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_total_stake">total_stake</a> + staked_amount;
        <b>let</b> new_entry = <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSetEntry">ActiveSetEntry</a> { validator_id, staked_amount };
        set.<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_insert_sorted">insert_sorted</a>(new_entry);
        <b>return</b> <b>true</b>
    };
    // If the <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_new">new</a> validator's stake is less than the smallest stake in the set, don't <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_insert">insert</a>
    <b>if</b> (staked_amount &lt;= set.validators[0].staked_amount) {
        <b>return</b> <b>false</b>
    };
    // Remove the validator with smallest stake and <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_insert">insert</a> the <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_new">new</a> one
    <b>let</b> removed_stake = set.validators[0].staked_amount;
    set.<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_total_stake">total_stake</a> = set.<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_total_stake">total_stake</a> - removed_stake + staked_amount;
    vector::remove(&<b>mut</b> set.validators, 0);
    <b>let</b> new_entry = <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSetEntry">ActiveSetEntry</a> { validator_id, staked_amount };
    set.<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_insert_sorted">insert_sorted</a>(new_entry);
    <b>true</b>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_active_set_remove"></a>

## Function `remove`

Removes the storage validator with the given <code>validator_id</code> from the active set.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_remove">remove</a>(set: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSet">active_set::ActiveSet</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_remove">remove</a>(set: &<b>mut</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSet">ActiveSet</a>, validator_id: ID) {
    <b>let</b> index = set.<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_find_validator_index">find_validator_index</a>(validator_id);
    index.do!(|idx| {
        <b>let</b> <b>entry</b> = vector::remove(&<b>mut</b> set.validators, idx);
        set.<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_total_stake">total_stake</a> = set.<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_total_stake">total_stake</a> - <b>entry</b>.staked_amount;
    });
}
</code></pre>



</details>

<a name="(ika_system=0x0)_active_set_find_validator_index"></a>

## Function `find_validator_index`

Finds the index of a validator in the active set using linear search.
Returns None if the validator is not found.


<pre><code><b>fun</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_find_validator_index">find_validator_index</a>(set: &(ika_system=0x0)::<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSet">active_set::ActiveSet</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_find_validator_index">find_validator_index</a>(set: &<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSet">ActiveSet</a>, validator_id: ID): Option&lt;u64&gt; {
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

<a name="(ika_system=0x0)_active_set_insert_sorted"></a>

## Function `insert_sorted`

Inserts a validator entry into the sorted vector at the correct position.


<pre><code><b>fun</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_insert_sorted">insert_sorted</a>(set: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSet">active_set::ActiveSet</a>, <b>entry</b>: (ika_system=0x0)::<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSetEntry">active_set::ActiveSetEntry</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_insert_sorted">insert_sorted</a>(set: &<b>mut</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSet">ActiveSet</a>, <b>entry</b>: <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSetEntry">ActiveSetEntry</a>) {
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
    // Manual <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_insert">insert</a> implementation:
    // Push to end, then shift elements to make space and place the <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_new">new</a> element
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

<a name="(ika_system=0x0)_active_set_reposition_validator"></a>

## Function `reposition_validator`

Repositions a validator in the sorted vector after its stake has been updated.


<pre><code><b>fun</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_reposition_validator">reposition_validator</a>(set: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSet">active_set::ActiveSet</a>, index: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_reposition_validator">reposition_validator</a>(set: &<b>mut</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSet">ActiveSet</a>, index: u64) {
    <b>let</b> <b>entry</b> = vector::remove(&<b>mut</b> set.validators, index);
    set.<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_insert_sorted">insert_sorted</a>(<b>entry</b>)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_active_set_max_size"></a>

## Function `max_size`

The maximum size of the active set.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_max_size">max_size</a>(set: &(ika_system=0x0)::<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSet">active_set::ActiveSet</a>): u16
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_max_size">max_size</a>(set: &<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSet">ActiveSet</a>): u16 { set.<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_max_size">max_size</a> }
</code></pre>



</details>

<a name="(ika_system=0x0)_active_set_size"></a>

## Function `size`

The current size of the active set.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_size">size</a>(set: &(ika_system=0x0)::<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSet">active_set::ActiveSet</a>): u16
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_size">size</a>(set: &<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSet">ActiveSet</a>): u16 { set.validators.length() <b>as</b> u16 }
</code></pre>



</details>

<a name="(ika_system=0x0)_active_set_active_ids"></a>

## Function `active_ids`

The IDs of the validators in the active set.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_active_ids">active_ids</a>(set: &(ika_system=0x0)::<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSet">active_set::ActiveSet</a>): vector&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_active_ids">active_ids</a>(set: &<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSet">ActiveSet</a>): vector&lt;ID&gt; {
    set.validators.map_ref!(|validator| validator.validator_id)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_active_set_active_ids_and_stake"></a>

## Function `active_ids_and_stake`

The IDs and stake of the validators in the active set.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_active_ids_and_stake">active_ids_and_stake</a>(set: &(ika_system=0x0)::<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSet">active_set::ActiveSet</a>): (vector&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;, vector&lt;u64&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_active_ids_and_stake">active_ids_and_stake</a>(set: &<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSet">ActiveSet</a>): (vector&lt;ID&gt;, vector&lt;u64&gt;) {
    <b>let</b> <b>mut</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_active_ids">active_ids</a> = vector[];
    <b>let</b> <b>mut</b> stake = vector[];
    set.validators.do_ref!(|<b>entry</b>| {
        <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_active_ids">active_ids</a>.push_back(<b>entry</b>.validator_id);
        stake.push_back(<b>entry</b>.staked_amount);
    });
    (<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_active_ids">active_ids</a>, stake)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_active_set_min_stake"></a>

## Function `min_stake`

The minimum amount of staked IKA in the active set.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_min_stake">min_stake</a>(set: &(ika_system=0x0)::<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSet">active_set::ActiveSet</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_min_stake">min_stake</a>(set: &<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSet">ActiveSet</a>): u64 { set.<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_min_stake">min_stake</a> }
</code></pre>



</details>

<a name="(ika_system=0x0)_active_set_total_stake"></a>

## Function `total_stake`

The total amount of staked IKA in the active set.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_total_stake">total_stake</a>(set: &(ika_system=0x0)::<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSet">active_set::ActiveSet</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_total_stake">total_stake</a>(set: &<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_ActiveSet">ActiveSet</a>): u64 { set.<a href="../ika_system/active_set.md#(ika_system=0x0)_active_set_total_stake">total_stake</a> }
</code></pre>



</details>
