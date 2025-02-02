---
title: Module `0x0::test`
---



-  [Struct `TestEvent`](#0x0_test_TestEvent)
-  [Function `test_event`](#0x0_test_test_event)


<pre><code><b>use</b> <a href="../sui-framework/event.md#0x2_event">0x2::event</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
</code></pre>



<a name="0x0_test_TestEvent"></a>

## Struct `TestEvent`



<pre><code><b>struct</b> <a href="test.md#0x0_test_TestEvent">TestEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>tx_hash: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>epoch_timestamp_ms: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x0_test_test_event"></a>

## Function `test_event`



<pre><code><b>public</b> entry <b>fun</b> <a href="test.md#0x0_test_test_event">test_event</a>(ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="test.md#0x0_test_test_event">test_event</a>(ctx: &<b>mut</b> TxContext) {
    <a href="../sui-framework/event.md#0x2_event_emit">event::emit</a>(<a href="test.md#0x0_test_TestEvent">TestEvent</a> {
        sender: ctx.sender(),
        tx_hash: *ctx.digest(),
        epoch: ctx.epoch(),
        epoch_timestamp_ms: ctx.epoch_timestamp_ms(),
    });
}
</code></pre>



</details>
