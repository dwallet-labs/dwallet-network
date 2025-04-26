---
title: Module `(ika_system=0x0)::pending_values`
---



-  [Struct `PendingValues`](#(ika_system=0x0)_pending_values_PendingValues)
-  [Constants](#@Constants_0)
-  [Function `empty`](#(ika_system=0x0)_pending_values_empty)
-  [Function `insert_or_add`](#(ika_system=0x0)_pending_values_insert_or_add)
-  [Function `insert_or_replace`](#(ika_system=0x0)_pending_values_insert_or_replace)
-  [Function `reduce`](#(ika_system=0x0)_pending_values_reduce)
-  [Function `value_at`](#(ika_system=0x0)_pending_values_value_at)
-  [Function `flush`](#(ika_system=0x0)_pending_values_flush)
-  [Function `inner`](#(ika_system=0x0)_pending_values_inner)
-  [Function `inner_mut`](#(ika_system=0x0)_pending_values_inner_mut)
-  [Function `unwrap`](#(ika_system=0x0)_pending_values_unwrap)
-  [Function `is_empty`](#(ika_system=0x0)_pending_values_is_empty)


<pre><code><b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../sui/vec_map.md#sui_vec_map">sui::vec_map</a>;
</code></pre>



<a name="(ika_system=0x0)_pending_values_PendingValues"></a>

## Struct `PendingValues`

Represents a map of pending values. The key is the epoch when the value is
pending, and the value is the amount of IKAs or pool shares.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">PendingValues</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>0: <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u64, u64&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="(ika_system=0x0)_pending_values_EMissingEpochValue"></a>

No value for the provided epoch exists.


<pre><code><b>const</b> <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_EMissingEpochValue">EMissingEpochValue</a>: u64 = 0;
</code></pre>



<a name="(ika_system=0x0)_pending_values_EReduceValueTooLarge"></a>

The value that the pending values should be reduced by for an epoch is too large.


<pre><code><b>const</b> <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_EReduceValueTooLarge">EReduceValueTooLarge</a>: u64 = 1;
</code></pre>



<a name="(ika_system=0x0)_pending_values_empty"></a>

## Function `empty`

Create a new empty <code><a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">PendingValues</a></code> instance.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_empty">empty</a>(): (ika_system=0x0)::<a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">pending_values::PendingValues</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_empty">empty</a>(): <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">PendingValues</a> { <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">PendingValues</a>(vec_map::empty()) }
</code></pre>



</details>

<a name="(ika_system=0x0)_pending_values_insert_or_add"></a>

## Function `insert_or_add`

Insert a new pending value for the given epoch, or add to the existing value.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_insert_or_add">insert_or_add</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">pending_values::PendingValues</a>, epoch: u64, value: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_insert_or_add">insert_or_add</a>(self: &<b>mut</b> <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">PendingValues</a>, epoch: u64, value: u64) {
    <b>let</b> map = &<b>mut</b> self.0;
    <b>if</b> (!map.contains(&epoch)) {
        map.insert(epoch, value);
    } <b>else</b> {
        <b>let</b> curr = map[&epoch];
        *&<b>mut</b> map[&epoch] = curr + value;
    };
}
</code></pre>



</details>

<a name="(ika_system=0x0)_pending_values_insert_or_replace"></a>

## Function `insert_or_replace`

Insert a new pending value for the given epoch, or replace the existing.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_insert_or_replace">insert_or_replace</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">pending_values::PendingValues</a>, epoch: u64, value: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_insert_or_replace">insert_or_replace</a>(self: &<b>mut</b> <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">PendingValues</a>, epoch: u64, value: u64) {
    <b>let</b> map = &<b>mut</b> self.0;
    <b>if</b> (!map.contains(&epoch)) {
        map.insert(epoch, value);
    } <b>else</b> {
        *&<b>mut</b> map[&epoch] = value;
    };
}
</code></pre>



</details>

<a name="(ika_system=0x0)_pending_values_reduce"></a>

## Function `reduce`

Reduce the pending value for the given epoch by the given value.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_reduce">reduce</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">pending_values::PendingValues</a>, epoch: u64, value: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_reduce">reduce</a>(self: &<b>mut</b> <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">PendingValues</a>, epoch: u64, value: u64) {
    <b>let</b> map = &<b>mut</b> self.0;
    <b>if</b> (!map.contains(&epoch)) {
        <b>abort</b> <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_EMissingEpochValue">EMissingEpochValue</a>
    } <b>else</b> {
        <b>let</b> curr = map[&epoch];
        <b>assert</b>!(curr &gt;= value, <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_EReduceValueTooLarge">EReduceValueTooLarge</a>);
        *&<b>mut</b> map[&epoch] = curr - value;
    };
}
</code></pre>



</details>

<a name="(ika_system=0x0)_pending_values_value_at"></a>

## Function `value_at`

Get the total value of the pending values up to the given epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_value_at">value_at</a>(self: &(ika_system=0x0)::<a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">pending_values::PendingValues</a>, epoch: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_value_at">value_at</a>(self: &<a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">PendingValues</a>, epoch: u64): u64 {
    self.0.keys().fold!(0, |<b>mut</b> value, e| {
        <b>if</b> (e &lt;= epoch) value = value + self.0[&e];
        value
    })
}
</code></pre>



</details>

<a name="(ika_system=0x0)_pending_values_flush"></a>

## Function `flush`

Reduce the pending values to the given epoch. This method removes all the
values that are pending for epochs less than or equal to the given epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_flush">flush</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">pending_values::PendingValues</a>, to_epoch: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_flush">flush</a>(self: &<b>mut</b> <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">PendingValues</a>, to_epoch: u64): u64 {
    <b>let</b> <b>mut</b> value = 0;
    self.0.keys().do!(|epoch| <b>if</b> (epoch &lt;= to_epoch) {
        <b>let</b> (_, epoch_value) = self.0.remove(&epoch);
        value = value + epoch_value;
    });
    value
}
</code></pre>



</details>

<a name="(ika_system=0x0)_pending_values_inner"></a>

## Function `inner`

Get a reference to the inner <code>VecMap&lt;u64, u64&gt;</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_inner">inner</a>(self: &(ika_system=0x0)::<a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">pending_values::PendingValues</a>): &<a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u64, u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_inner">inner</a>(self: &<a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">PendingValues</a>): &VecMap&lt;u64, u64&gt; { &self.0 }
</code></pre>



</details>

<a name="(ika_system=0x0)_pending_values_inner_mut"></a>

## Function `inner_mut`

Get a mutable reference to the inner <code>VecMap&lt;u64, u64&gt;</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_inner_mut">inner_mut</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">pending_values::PendingValues</a>): &<b>mut</b> <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u64, u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_inner_mut">inner_mut</a>(self: &<b>mut</b> <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">PendingValues</a>): &<b>mut</b> VecMap&lt;u64, u64&gt; { &<b>mut</b> self.0 }
</code></pre>



</details>

<a name="(ika_system=0x0)_pending_values_unwrap"></a>

## Function `unwrap`

Unwrap the <code><a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">PendingValues</a></code> into a <code>VecMap&lt;u64, u64&gt;</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_unwrap">unwrap</a>(self: (ika_system=0x0)::<a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">pending_values::PendingValues</a>): <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u64, u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_unwrap">unwrap</a>(self: <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">PendingValues</a>): VecMap&lt;u64, u64&gt; {
    <b>let</b> <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">PendingValues</a>(map) = self;
    map
}
</code></pre>



</details>

<a name="(ika_system=0x0)_pending_values_is_empty"></a>

## Function `is_empty`

Check if the <code><a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">PendingValues</a></code> is empty.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_is_empty">is_empty</a>(self: &(ika_system=0x0)::<a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">pending_values::PendingValues</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_is_empty">is_empty</a>(self: &<a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_PendingValues">PendingValues</a>): bool { self.0.<a href="../ika_system/pending_values.md#(ika_system=0x0)_pending_values_is_empty">is_empty</a>() }
</code></pre>



</details>
