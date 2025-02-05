---
title: Module `std::address`
---

Provides a way to get address length since it's a
platform-specific parameter.


-  [Function `length`](#std_address_length)


<pre><code></code></pre>



<a name="std_address_length"></a>

## Function `length`

Should be converted to a native function.
Current implementation only works for Sui.


<pre><code><b>public</b> <b>fun</b> <a href="../std/address.md#std_address_length">length</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../std/address.md#std_address_length">length</a>(): u64 {
    32
}
</code></pre>



</details>
