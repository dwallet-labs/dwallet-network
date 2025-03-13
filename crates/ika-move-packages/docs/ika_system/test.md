---
title: Module `(ika_system=0x0)::test`
---



-  [Struct `TestEvent`](#(ika_system=0x0)_test_TestEvent)
-  [Function `test_event`](#(ika_system=0x0)_test_test_event)


<pre><code><b>use</b> <a href="../sui/event.md#sui_event">sui::event</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
</code></pre>



<a name="(ika_system=0x0)_test_TestEvent"></a>

## Struct `TestEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/test.md#(ika_system=0x0)_test_TestEvent">TestEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>sender: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>tx_hash: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>epoch_timestamp_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_test_test_event"></a>

## Function `test_event`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/test.md#(ika_system=0x0)_test_test_event">test_event</a>(ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../ika_system/test.md#(ika_system=0x0)_test_test_event">test_event</a>(ctx: &<b>mut</b> TxContext) {
    event::emit(<a href="../ika_system/test.md#(ika_system=0x0)_test_TestEvent">TestEvent</a> {
        sender: ctx.sender(),
        tx_hash: *ctx.digest(),
        epoch: ctx.epoch(),
        epoch_timestamp_ms: ctx.epoch_timestamp_ms(),
    });
}
</code></pre>



</details>
