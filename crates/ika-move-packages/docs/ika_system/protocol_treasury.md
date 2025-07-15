---
title: Module `(ika_system=0x0)::protocol_treasury`
---



-  [Struct `ProtocolTreasury`](#(ika_system=0x0)_protocol_treasury_ProtocolTreasury)
-  [Constants](#@Constants_0)
-  [Function `create`](#(ika_system=0x0)_protocol_treasury_create)
-  [Function `stake_subsidy_for_distribution`](#(ika_system=0x0)_protocol_treasury_stake_subsidy_for_distribution)
-  [Function `set_stake_subsidy_rate`](#(ika_system=0x0)_protocol_treasury_set_stake_subsidy_rate)
-  [Function `set_stake_subsidy_period_length`](#(ika_system=0x0)_protocol_treasury_set_stake_subsidy_period_length)
-  [Function `stake_subsidy_amount_per_distribution`](#(ika_system=0x0)_protocol_treasury_stake_subsidy_amount_per_distribution)
-  [Function `get_stake_subsidy_distribution_counter`](#(ika_system=0x0)_protocol_treasury_get_stake_subsidy_distribution_counter)
-  [Function `calculate_stake_subsidy_amount_per_distribution`](#(ika_system=0x0)_protocol_treasury_calculate_stake_subsidy_amount_per_distribution)


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



<a name="(ika_system=0x0)_protocol_treasury_ProtocolTreasury"></a>

## Struct `ProtocolTreasury`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_ProtocolTreasury">ProtocolTreasury</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>treasury_cap: <a href="../sui/coin.md#sui_coin_TreasuryCap">sui::coin::TreasuryCap</a>&lt;(ika=0x0)::ika::IKA&gt;</code>
</dt>
<dd>
 TreasuryCap of IKA tokens.
</dd>
<dt>
<code>stake_subsidy_distribution_counter: u64</code>
</dt>
<dd>
 Count of the number of times stake subsidies have been distributed.
</dd>
<dt>
<code>stake_subsidy_rate: u16</code>
</dt>
<dd>
 The rate at which the amount per distribution is calculated based on
 period nad total supply. Expressed in basis points.
</dd>
<dt>
<code><a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_stake_subsidy_amount_per_distribution">stake_subsidy_amount_per_distribution</a>: u64</code>
</dt>
<dd>
 The amount of stake subsidy to be destructured per distribution.
 This amount changes based on <code>stake_subsidy_rate</code>.
</dd>
<dt>
<code>stake_subsidy_period_length: u64</code>
</dt>
<dd>
 Number of distributions to occur before the amount per distribution will be recalculated.
</dd>
<dt>
<code>total_supply_at_period_start: u64</code>
</dt>
<dd>
 The total supply of IKA tokens at the start of the current period.
</dd>
<dt>
<code>extra_fields: <a href="../sui/bag.md#sui_bag_Bag">sui::bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="(ika_system=0x0)_protocol_treasury_ESubsidyDecreaseRateTooLarge"></a>

The stake subsidy rate is too large.


<pre><code><b>const</b> <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_ESubsidyDecreaseRateTooLarge">ESubsidyDecreaseRateTooLarge</a>: u64 = 0;
</code></pre>



<a name="(ika_system=0x0)_protocol_treasury_BASIS_POINT_DENOMINATOR"></a>



<pre><code><b>const</b> <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a>: u128 = 10000;
</code></pre>



<a name="(ika_system=0x0)_protocol_treasury_create"></a>

## Function `create`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_create">create</a>(treasury_cap: <a href="../sui/coin.md#sui_coin_TreasuryCap">sui::coin::TreasuryCap</a>&lt;(ika=0x0)::ika::IKA&gt;, stake_subsidy_rate: u16, stake_subsidy_period_length: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_ProtocolTreasury">protocol_treasury::ProtocolTreasury</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_create">create</a>(
    treasury_cap: TreasuryCap&lt;IKA&gt;,
    stake_subsidy_rate: u16,
    stake_subsidy_period_length: u64,
    ctx: &<b>mut</b> TxContext,
): <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_ProtocolTreasury">ProtocolTreasury</a> {
    // Rate can't be higher than 100%.
    <b>assert</b>!(stake_subsidy_rate &lt;= <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a> <b>as</b> u16, <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_ESubsidyDecreaseRateTooLarge">ESubsidyDecreaseRateTooLarge</a>);
    <b>let</b> total_supply_at_period_start = treasury_cap.total_supply();
    <b>let</b> <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_stake_subsidy_amount_per_distribution">stake_subsidy_amount_per_distribution</a> = <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_calculate_stake_subsidy_amount_per_distribution">calculate_stake_subsidy_amount_per_distribution</a>(
        total_supply_at_period_start,
        stake_subsidy_rate,
        stake_subsidy_period_length,
    );
    <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_ProtocolTreasury">ProtocolTreasury</a> {
        treasury_cap,
        stake_subsidy_distribution_counter: 0,
        stake_subsidy_rate,
        <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_stake_subsidy_amount_per_distribution">stake_subsidy_amount_per_distribution</a>,
        stake_subsidy_period_length,
        total_supply_at_period_start,
        extra_fields: bag::new(ctx),
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_protocol_treasury_stake_subsidy_for_distribution"></a>

## Function `stake_subsidy_for_distribution`

Advance the distribution counter and return the stake subsidy.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_stake_subsidy_for_distribution">stake_subsidy_for_distribution</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_ProtocolTreasury">protocol_treasury::ProtocolTreasury</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_stake_subsidy_for_distribution">stake_subsidy_for_distribution</a>(
    self: &<b>mut</b> <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_ProtocolTreasury">ProtocolTreasury</a>,
    ctx: &<b>mut</b> TxContext,
): Balance&lt;IKA&gt; {
    // Mint the reward amount <b>for</b> this stake subsidy
    <b>let</b> stake_subsidy = self.treasury_cap.mint(self.<a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_stake_subsidy_amount_per_distribution">stake_subsidy_amount_per_distribution</a>, ctx);
    self.stake_subsidy_distribution_counter = self.stake_subsidy_distribution_counter + 1;
    // Recalculate subsidy amount per distribution only when the current period ends.
    <b>if</b> (self.stake_subsidy_distribution_counter % self.stake_subsidy_period_length == 0) {
        <b>let</b> total_supply_at_period_start = self.treasury_cap.total_supply();
        self.<a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_stake_subsidy_amount_per_distribution">stake_subsidy_amount_per_distribution</a> =
            <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_calculate_stake_subsidy_amount_per_distribution">calculate_stake_subsidy_amount_per_distribution</a>(
                total_supply_at_period_start,
                self.stake_subsidy_rate,
                self.stake_subsidy_period_length,
            );
        self.total_supply_at_period_start = total_supply_at_period_start;
    };
    stake_subsidy.into_balance()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_protocol_treasury_set_stake_subsidy_rate"></a>

## Function `set_stake_subsidy_rate`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_set_stake_subsidy_rate">set_stake_subsidy_rate</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_ProtocolTreasury">protocol_treasury::ProtocolTreasury</a>, stake_subsidy_rate: u16)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_set_stake_subsidy_rate">set_stake_subsidy_rate</a>(self: &<b>mut</b> <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_ProtocolTreasury">ProtocolTreasury</a>, stake_subsidy_rate: u16) {
    // Rate can't be higher than 100%.
    <b>assert</b>!(stake_subsidy_rate &lt;= <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a> <b>as</b> u16, <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_ESubsidyDecreaseRateTooLarge">ESubsidyDecreaseRateTooLarge</a>);
    // Update the stored rate
    self.stake_subsidy_rate = stake_subsidy_rate;
    // Recalculate the stake subsidy amount per distribution with the new rate
    self.<a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_stake_subsidy_amount_per_distribution">stake_subsidy_amount_per_distribution</a> = <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_calculate_stake_subsidy_amount_per_distribution">calculate_stake_subsidy_amount_per_distribution</a>(
        self.total_supply_at_period_start,
        stake_subsidy_rate,
        self.stake_subsidy_period_length,
    );
}
</code></pre>



</details>

<a name="(ika_system=0x0)_protocol_treasury_set_stake_subsidy_period_length"></a>

## Function `set_stake_subsidy_period_length`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_set_stake_subsidy_period_length">set_stake_subsidy_period_length</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_ProtocolTreasury">protocol_treasury::ProtocolTreasury</a>, stake_subsidy_period_length: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_set_stake_subsidy_period_length">set_stake_subsidy_period_length</a>(self: &<b>mut</b> <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_ProtocolTreasury">ProtocolTreasury</a>, stake_subsidy_period_length: u64) {
    self.stake_subsidy_period_length = stake_subsidy_period_length;
    // Recalculate the stake subsidy amount per distribution with the new period length
    self.<a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_stake_subsidy_amount_per_distribution">stake_subsidy_amount_per_distribution</a> = <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_calculate_stake_subsidy_amount_per_distribution">calculate_stake_subsidy_amount_per_distribution</a>(
        self.total_supply_at_period_start,
        self.stake_subsidy_rate,
        stake_subsidy_period_length,
    );
}
</code></pre>



</details>

<a name="(ika_system=0x0)_protocol_treasury_stake_subsidy_amount_per_distribution"></a>

## Function `stake_subsidy_amount_per_distribution`

Returns the stake subsidy amount per distribution.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_stake_subsidy_amount_per_distribution">stake_subsidy_amount_per_distribution</a>(self: &(ika_system=0x0)::<a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_ProtocolTreasury">protocol_treasury::ProtocolTreasury</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_stake_subsidy_amount_per_distribution">stake_subsidy_amount_per_distribution</a>(self: &<a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_ProtocolTreasury">ProtocolTreasury</a>): u64 {
    self.<a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_stake_subsidy_amount_per_distribution">stake_subsidy_amount_per_distribution</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_protocol_treasury_get_stake_subsidy_distribution_counter"></a>

## Function `get_stake_subsidy_distribution_counter`

Returns the number of distributions that have occurred.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_get_stake_subsidy_distribution_counter">get_stake_subsidy_distribution_counter</a>(self: &(ika_system=0x0)::<a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_ProtocolTreasury">protocol_treasury::ProtocolTreasury</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_get_stake_subsidy_distribution_counter">get_stake_subsidy_distribution_counter</a>(self: &<a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_ProtocolTreasury">ProtocolTreasury</a>): u64 {
    self.stake_subsidy_distribution_counter
}
</code></pre>



</details>

<a name="(ika_system=0x0)_protocol_treasury_calculate_stake_subsidy_amount_per_distribution"></a>

## Function `calculate_stake_subsidy_amount_per_distribution`



<pre><code><b>fun</b> <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_calculate_stake_subsidy_amount_per_distribution">calculate_stake_subsidy_amount_per_distribution</a>(total_supply_at_period_start: u64, stake_subsidy_rate: u16, stake_subsidy_period_length: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_calculate_stake_subsidy_amount_per_distribution">calculate_stake_subsidy_amount_per_distribution</a>(
    total_supply_at_period_start: u64,
    stake_subsidy_rate: u16,
    stake_subsidy_period_length: u64,
): u64 {
    <b>let</b> stake_subsidy_total_period_distribution_amount =
        total_supply_at_period_start <b>as</b> u128
                * (stake_subsidy_rate <b>as</b> u128) / <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a>;
    <b>let</b> <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_stake_subsidy_amount_per_distribution">stake_subsidy_amount_per_distribution</a> =
        stake_subsidy_total_period_distribution_amount / (stake_subsidy_period_length <b>as</b> u128);
    <a href="../ika_system/protocol_treasury.md#(ika_system=0x0)_protocol_treasury_stake_subsidy_amount_per_distribution">stake_subsidy_amount_per_distribution</a> <b>as</b> u64
}
</code></pre>



</details>
