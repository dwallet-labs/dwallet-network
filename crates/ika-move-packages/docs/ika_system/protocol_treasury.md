---
title: Module `0x0::protocol_treasury`
---



-  [Struct `ProtocolTreasury`](#0x0_protocol_treasury_ProtocolTreasury)
-  [Constants](#@Constants_0)
-  [Function `create`](#0x0_protocol_treasury_create)
-  [Function `stake_subsidy_for_distribution`](#0x0_protocol_treasury_stake_subsidy_for_distribution)
-  [Function `calculate_stake_subsidy_amount_per_distribution`](#0x0_protocol_treasury_calculate_stake_subsidy_amount_per_distribution)
-  [Function `set_stake_subsidy_rate`](#0x0_protocol_treasury_set_stake_subsidy_rate)
-  [Function `stake_subsidy_amount_per_distribution`](#0x0_protocol_treasury_stake_subsidy_amount_per_distribution)
-  [Function `get_stake_subsidy_distribution_counter`](#0x0_protocol_treasury_get_stake_subsidy_distribution_counter)


<pre><code><b>use</b> <a href="../ika/ika.md#0x0_ika">0x0::ika</a>;
<b>use</b> <a href="../sui-framework/bag.md#0x2_bag">0x2::bag</a>;
<b>use</b> <a href="../sui-framework/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../sui-framework/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
</code></pre>



<a name="0x0_protocol_treasury_ProtocolTreasury"></a>

## Struct `ProtocolTreasury`



<pre><code><b>struct</b> <a href="protocol_treasury.md#0x0_protocol_treasury_ProtocolTreasury">ProtocolTreasury</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>treasury_cap: <a href="../sui-framework/coin.md#0x2_coin_TreasuryCap">coin::TreasuryCap</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;</code>
</dt>
<dd>
 TreasuryCap of IKA tokens.
</dd>
<dt>
<code>stake_subsidy_distribution_counter: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
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
<code>stake_subsidy_amount_per_distribution: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 The amount of stake subsidy to be distrabtured per distribution.
 This amount changes based on <code>stake_subsidy_rate</code>.
</dd>
<dt>
<code>stake_subsidy_period_length: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 Number of distributions to occur before the amount per distribution will be recalculated.
</dd>
<dt>
<code>extra_fields: <a href="../sui-framework/bag.md#0x2_bag_Bag">bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x0_protocol_treasury_BASIS_POINT_DENOMINATOR"></a>



<pre><code><b>const</b> <a href="protocol_treasury.md#0x0_protocol_treasury_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a>: u128 = 10000;
</code></pre>



<a name="0x0_protocol_treasury_ESubsidyDecreaseRateTooLarge"></a>



<pre><code><b>const</b> <a href="protocol_treasury.md#0x0_protocol_treasury_ESubsidyDecreaseRateTooLarge">ESubsidyDecreaseRateTooLarge</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
</code></pre>



<a name="0x0_protocol_treasury_create"></a>

## Function `create`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="protocol_treasury.md#0x0_protocol_treasury_create">create</a>(treasury_cap: <a href="../sui-framework/coin.md#0x2_coin_TreasuryCap">coin::TreasuryCap</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;, stake_subsidy_rate: u16, stake_subsidy_period_length: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="protocol_treasury.md#0x0_protocol_treasury_ProtocolTreasury">protocol_treasury::ProtocolTreasury</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="protocol_treasury.md#0x0_protocol_treasury_create">create</a>(
    treasury_cap: TreasuryCap&lt;IKA&gt;,
    stake_subsidy_rate: u16,
    stake_subsidy_period_length: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    ctx: &<b>mut</b> TxContext,
): <a href="protocol_treasury.md#0x0_protocol_treasury_ProtocolTreasury">ProtocolTreasury</a> {
    // Rate can't be higher than 100%.
    <b>assert</b>!(stake_subsidy_rate &lt;= <a href="protocol_treasury.md#0x0_protocol_treasury_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a> <b>as</b> u16, <a href="protocol_treasury.md#0x0_protocol_treasury_ESubsidyDecreaseRateTooLarge">ESubsidyDecreaseRateTooLarge</a>);

    <b>let</b> stake_subsidy_amount_per_distribution = <a href="protocol_treasury.md#0x0_protocol_treasury_calculate_stake_subsidy_amount_per_distribution">calculate_stake_subsidy_amount_per_distribution</a>(
        &treasury_cap,
        stake_subsidy_rate,
        stake_subsidy_period_length,
    );

    <a href="protocol_treasury.md#0x0_protocol_treasury_ProtocolTreasury">ProtocolTreasury</a> {
        treasury_cap,
        stake_subsidy_distribution_counter: 0,
        stake_subsidy_rate,
        stake_subsidy_amount_per_distribution,
        stake_subsidy_period_length,
        extra_fields: <a href="../sui-framework/bag.md#0x2_bag_new">bag::new</a>(ctx),
    }
}
</code></pre>



</details>

<a name="0x0_protocol_treasury_stake_subsidy_for_distribution"></a>

## Function `stake_subsidy_for_distribution`

Advance the distribution counter and return the stake subsidy.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="protocol_treasury.md#0x0_protocol_treasury_stake_subsidy_for_distribution">stake_subsidy_for_distribution</a>(self: &<b>mut</b> <a href="protocol_treasury.md#0x0_protocol_treasury_ProtocolTreasury">protocol_treasury::ProtocolTreasury</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="protocol_treasury.md#0x0_protocol_treasury_stake_subsidy_for_distribution">stake_subsidy_for_distribution</a>(
    self: &<b>mut</b> <a href="protocol_treasury.md#0x0_protocol_treasury_ProtocolTreasury">ProtocolTreasury</a>,
    ctx: &<b>mut</b> TxContext,
): Balance&lt;IKA&gt; {
    // Mint the reward amount for this stake subsidy
    <b>let</b> stake_subsidy = self.treasury_cap.mint(self.stake_subsidy_amount_per_distribution, ctx);

    self.stake_subsidy_distribution_counter = self.stake_subsidy_distribution_counter + 1;

    // Recalculate subsidy amount per distribution only when the current period ends.
    <b>if</b> (self.stake_subsidy_distribution_counter % self.stake_subsidy_period_length == 0) {
        self.stake_subsidy_amount_per_distribution =
            <a href="protocol_treasury.md#0x0_protocol_treasury_calculate_stake_subsidy_amount_per_distribution">calculate_stake_subsidy_amount_per_distribution</a>(
                &self.treasury_cap,
                self.stake_subsidy_rate,
                self.stake_subsidy_period_length,
            );
    };

    stake_subsidy.into_balance()
}
</code></pre>



</details>

<a name="0x0_protocol_treasury_calculate_stake_subsidy_amount_per_distribution"></a>

## Function `calculate_stake_subsidy_amount_per_distribution`



<pre><code><b>fun</b> <a href="protocol_treasury.md#0x0_protocol_treasury_calculate_stake_subsidy_amount_per_distribution">calculate_stake_subsidy_amount_per_distribution</a>(treasury_cap: &<a href="../sui-framework/coin.md#0x2_coin_TreasuryCap">coin::TreasuryCap</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;, stake_subsidy_rate: u16, stake_subsidy_period_length: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="protocol_treasury.md#0x0_protocol_treasury_calculate_stake_subsidy_amount_per_distribution">calculate_stake_subsidy_amount_per_distribution</a>(
    treasury_cap: &TreasuryCap&lt;IKA&gt;,
    stake_subsidy_rate: u16,
    stake_subsidy_period_length: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <b>let</b> stake_subsidy_total_period_distribution_amount =
        treasury_cap.total_supply() <b>as</b> u128
                * (stake_subsidy_rate <b>as</b> u128) / <a href="protocol_treasury.md#0x0_protocol_treasury_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a>;
    <b>let</b> stake_subsidy_amount_per_distribution =
        stake_subsidy_total_period_distribution_amount / (stake_subsidy_period_length <b>as</b> u128);
    stake_subsidy_amount_per_distribution <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
}
</code></pre>



</details>

<a name="0x0_protocol_treasury_set_stake_subsidy_rate"></a>

## Function `set_stake_subsidy_rate`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="protocol_treasury.md#0x0_protocol_treasury_set_stake_subsidy_rate">set_stake_subsidy_rate</a>(self: &<b>mut</b> <a href="protocol_treasury.md#0x0_protocol_treasury_ProtocolTreasury">protocol_treasury::ProtocolTreasury</a>, stake_subsidy_rate: u16)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="protocol_treasury.md#0x0_protocol_treasury_set_stake_subsidy_rate">set_stake_subsidy_rate</a>(self: &<b>mut</b> <a href="protocol_treasury.md#0x0_protocol_treasury_ProtocolTreasury">ProtocolTreasury</a>, stake_subsidy_rate: u16) {
    // When stake subsidy rate <b>decreases</b>
    <b>if</b> (self.stake_subsidy_rate &gt; stake_subsidy_rate) {
        <b>let</b> stake_subsidy_rate_diff = self.stake_subsidy_rate - stake_subsidy_rate;
        <b>let</b> stake_subsidy_diff =
            (self.stake_subsidy_amount_per_distribution <b>as</b> u128) * (stake_subsidy_rate_diff <b>as</b> u128) / <a href="protocol_treasury.md#0x0_protocol_treasury_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a>;
        self.stake_subsidy_amount_per_distribution =
            self.stake_subsidy_amount_per_distribution - (stake_subsidy_diff <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>);
        // When stake subsidy rate increases
    } <b>else</b> <b>if</b> (self.stake_subsidy_rate &lt; stake_subsidy_rate) {
        <b>let</b> stake_subsidy_rate_diff = stake_subsidy_rate - self.stake_subsidy_rate;
        <b>let</b> stake_subsidy_diff =
            (self.stake_subsidy_amount_per_distribution <b>as</b> u128) * (stake_subsidy_rate_diff <b>as</b> u128) / <a href="protocol_treasury.md#0x0_protocol_treasury_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a>;
        self.stake_subsidy_amount_per_distribution =
            self.stake_subsidy_amount_per_distribution + (stake_subsidy_diff <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>);
    }
}
</code></pre>



</details>

<a name="0x0_protocol_treasury_stake_subsidy_amount_per_distribution"></a>

## Function `stake_subsidy_amount_per_distribution`

Returns the stake subsidy amount per distribution.


<pre><code><b>public</b> <b>fun</b> <a href="protocol_treasury.md#0x0_protocol_treasury_stake_subsidy_amount_per_distribution">stake_subsidy_amount_per_distribution</a>(self: &<a href="protocol_treasury.md#0x0_protocol_treasury_ProtocolTreasury">protocol_treasury::ProtocolTreasury</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="protocol_treasury.md#0x0_protocol_treasury_stake_subsidy_amount_per_distribution">stake_subsidy_amount_per_distribution</a>(self: &<a href="protocol_treasury.md#0x0_protocol_treasury_ProtocolTreasury">ProtocolTreasury</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    self.stake_subsidy_amount_per_distribution
}
</code></pre>



</details>

<a name="0x0_protocol_treasury_get_stake_subsidy_distribution_counter"></a>

## Function `get_stake_subsidy_distribution_counter`

Returns the number of distributions that have occurred.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="protocol_treasury.md#0x0_protocol_treasury_get_stake_subsidy_distribution_counter">get_stake_subsidy_distribution_counter</a>(self: &<a href="protocol_treasury.md#0x0_protocol_treasury_ProtocolTreasury">protocol_treasury::ProtocolTreasury</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="protocol_treasury.md#0x0_protocol_treasury_get_stake_subsidy_distribution_counter">get_stake_subsidy_distribution_counter</a>(self: &<a href="protocol_treasury.md#0x0_protocol_treasury_ProtocolTreasury">ProtocolTreasury</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    self.stake_subsidy_distribution_counter
}
</code></pre>



</details>
