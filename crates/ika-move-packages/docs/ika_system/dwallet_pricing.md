---
title: Module `(ika_system=0x0)::dwallet_pricing`
---

This module provides structures and functions for managing pricing information for a dWallet.
Each operation (e.g., DKG, re-encrypt user share, ECDSA presign, etc.) has its own pricing data,
represented by a <code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a></code>. Each <code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a></code> holds three values:
- **consensus_validation_ika**: The consensus validation IKA price.
- **computation_ika**: The computation_ika IKA price.
- **gas_fee_reimbursement_sui**: The SUI reimbursement.

The main struct, <code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a></code>, now holds one <code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a></code> per operation.
The DKG operation is split into two separate rounds:
- <code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_dkg_first_round">dkg_first_round</a></code>
- <code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_dkg_second_round">dkg_second_round</a></code>


-  [Struct `PricingPerOperation`](#(ika_system=0x0)_dwallet_pricing_PricingPerOperation)
-  [Struct `DWalletPricing2PcMpcSecp256K1`](#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1)
-  [Function `create_dwallet_pricing_2pc_mpc_secp256k1`](#(ika_system=0x0)_dwallet_pricing_create_dwallet_pricing_2pc_mpc_secp256k1)
    -  [Parameters](#@Parameters_0)
    -  [Returns](#@Returns_1)
-  [Function `zero`](#(ika_system=0x0)_dwallet_pricing_zero)
-  [Function `dkg_first_round`](#(ika_system=0x0)_dwallet_pricing_dkg_first_round)
-  [Function `dkg_second_round`](#(ika_system=0x0)_dwallet_pricing_dkg_second_round)
-  [Function `re_encrypt_user_share`](#(ika_system=0x0)_dwallet_pricing_re_encrypt_user_share)
-  [Function `presign`](#(ika_system=0x0)_dwallet_pricing_presign)
-  [Function `sign`](#(ika_system=0x0)_dwallet_pricing_sign)
-  [Function `future_sign`](#(ika_system=0x0)_dwallet_pricing_future_sign)
-  [Function `sign_with_partial_user_signature`](#(ika_system=0x0)_dwallet_pricing_sign_with_partial_user_signature)
-  [Function `make_dwallet_user_secret_key_shares_public`](#(ika_system=0x0)_dwallet_pricing_make_dwallet_user_secret_key_shares_public)
-  [Function `consensus_validation_ika`](#(ika_system=0x0)_dwallet_pricing_consensus_validation_ika)
-  [Function `computation_ika`](#(ika_system=0x0)_dwallet_pricing_computation_ika)
-  [Function `gas_fee_reimbursement_sui`](#(ika_system=0x0)_dwallet_pricing_gas_fee_reimbursement_sui)


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



<a name="(ika_system=0x0)_dwallet_pricing_PricingPerOperation"></a>

## Struct `PricingPerOperation`

Holds pricing information for a single operation.
The fields are ordered so that the consensus validation price is first.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_consensus_validation_ika">consensus_validation_ika</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika">computation_ika</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_gas_fee_reimbursement_sui">gas_fee_reimbursement_sui</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1"></a>

## Struct `DWalletPricing2PcMpcSecp256K1`

Represents pricing information for various operations in a dWallet.
Each operation is represented by its own <code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a></code>:
- <code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_dkg_first_round">dkg_first_round</a></code>: Pricing for the first round of distributed key generation.
- <code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_dkg_second_round">dkg_second_round</a></code>: Pricing for the second round of distributed key generation.
- <code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_re_encrypt_user_share">re_encrypt_user_share</a></code>: Pricing for re-encrypting user shares.
- <code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_presign">presign</a></code>: Pricing for ECDSA presigning.
- <code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_sign">sign</a></code>: Pricing for ECDSA signing.
- <code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_future_sign">future_sign</a></code>: Pricing for ECDSA future signing.
- <code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_sign_with_partial_user_signature">sign_with_partial_user_signature</a></code>: Pricing for ECDSA signing with partial user signature.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_dkg_first_round">dkg_first_round</a>: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">dwallet_pricing::PricingPerOperation</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_dkg_second_round">dkg_second_round</a>: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">dwallet_pricing::PricingPerOperation</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_re_encrypt_user_share">re_encrypt_user_share</a>: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">dwallet_pricing::PricingPerOperation</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_presign">presign</a>: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">dwallet_pricing::PricingPerOperation</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_sign">sign</a>: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">dwallet_pricing::PricingPerOperation</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_future_sign">future_sign</a>: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">dwallet_pricing::PricingPerOperation</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_sign_with_partial_user_signature">sign_with_partial_user_signature</a>: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">dwallet_pricing::PricingPerOperation</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_make_dwallet_user_secret_key_shares_public">make_dwallet_user_secret_key_shares_public</a>: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">dwallet_pricing::PricingPerOperation</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_pricing_create_dwallet_pricing_2pc_mpc_secp256k1"></a>

## Function `create_dwallet_pricing_2pc_mpc_secp256k1`

Creates a new [<code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a></code>] object.

Initializes pricing data for each operation by providing values for the three pricing fields for each operation.


<a name="@Parameters_0"></a>

### Parameters


- **DKG First Round Pricing:**
- <code>dkg_first_round_consensus_validation_ika</code>: Consensus validation IKA price.
- <code>dkg_first_round_computation_ika</code>: Computation IKA price.
- <code>dkg_first_round_gas_fee_reimbursement_sui</code>: SUI reimbursement.

- **DKG Second Round Pricing:**
- <code>dkg_second_round_consensus_validation_ika</code>: Consensus validation IKA price.
- <code>dkg_second_round_computation_ika</code>: Computation IKA price.
- <code>dkg_second_round_gas_fee_reimbursement_sui</code>: SUI reimbursement.

- **Re-encrypt User Share Pricing:**
- <code>re_encrypt_consensus_validation_ika</code>: Consensus validation IKA price.
- <code>re_encrypt_computation_ika</code>: Computation IKA price.
- <code>re_encrypt_gas_fee_reimbursement_sui</code>: SUI reimbursement.

- **ECDSA Presign Pricing:**
- <code>presign_consensus_validation_ika</code>: Consensus validation IKA price.
- <code>presign_computation_ika</code>: Computation IKA price.
- <code>presign_gas_fee_reimbursement_sui</code>: SUI reimbursement.

- **ECDSA Sign Pricing:**
- <code>sign_consensus_validation_ika</code>: Consensus validation IKA price.
- <code>sign_computation_ika</code>: Computation IKA price.
- <code>sign_gas_fee_reimbursement_sui</code>: SUI reimbursement.

- **ECDSA Future Sign Pricing:**
- <code>future_sign_consensus_validation_ika</code>: Consensus validation IKA price.
- <code>future_sign_computation_ika</code>: Computation IKA price.
- <code>future_sign_gas_fee_reimbursement_sui</code>: SUI reimbursement.

- **ECDSA Sign with Partial User Signature Pricing:**
- <code>partial_sign_consensus_validation_ika</code>: Consensus validation IKA price.
- <code>partial_sign_computation_ika</code>: Computation IKA price.
- <code>partial_sign_gas_fee_reimbursement_sui</code>: SUI reimbursement.

- <code>ctx</code>: The transaction context.


<a name="@Returns_1"></a>

### Returns


A newly created instance of <code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_create_dwallet_pricing_2pc_mpc_secp256k1">create_dwallet_pricing_2pc_mpc_secp256k1</a>(dkg_first_round_consensus_validation_ika: u64, dkg_first_round_computation_ika: u64, dkg_first_round_gas_fee_reimbursement_sui: u64, dkg_second_round_consensus_validation_ika: u64, dkg_second_round_computation_ika: u64, dkg_second_round_gas_fee_reimbursement_sui: u64, re_encrypt_consensus_validation_ika: u64, re_encrypt_computation_ika: u64, re_encrypt_gas_fee_reimbursement_sui: u64, presign_consensus_validation_ika: u64, presign_computation_ika: u64, presign_gas_fee_reimbursement_sui: u64, sign_consensus_validation_ika: u64, sign_computation_ika: u64, sign_gas_fee_reimbursement_sui: u64, future_sign_consensus_validation_ika: u64, future_sign_computation_ika: u64, future_sign_gas_fee_reimbursement_sui: u64, partial_sign_consensus_validation_ika: u64, partial_sign_computation_ika: u64, partial_sign_gas_fee_reimbursement_sui: u64, make_dwallet_user_secret_key_shares_public_consensus_validation_ika: u64, make_dwallet_user_secret_key_shares_public_computation_ika: u64, make_dwallet_user_secret_key_shares_public_gas_fee_reimbursement_sui: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">dwallet_pricing::DWalletPricing2PcMpcSecp256K1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_create_dwallet_pricing_2pc_mpc_secp256k1">create_dwallet_pricing_2pc_mpc_secp256k1</a>(
    // DKG First Round Pricing
    dkg_first_round_consensus_validation_ika: u64,
    dkg_first_round_computation_ika: u64,
    dkg_first_round_gas_fee_reimbursement_sui: u64,
    // DKG Second Round Pricing
    dkg_second_round_consensus_validation_ika: u64,
    dkg_second_round_computation_ika: u64,
    dkg_second_round_gas_fee_reimbursement_sui: u64,
    // Re-encrypt User Share Pricing
    re_encrypt_consensus_validation_ika: u64,
    re_encrypt_computation_ika: u64,
    re_encrypt_gas_fee_reimbursement_sui: u64,
    // ECDSA Presign Pricing
    presign_consensus_validation_ika: u64,
    presign_computation_ika: u64,
    presign_gas_fee_reimbursement_sui: u64,
    // ECDSA Sign Pricing
    sign_consensus_validation_ika: u64,
    sign_computation_ika: u64,
    sign_gas_fee_reimbursement_sui: u64,
    // ECDSA Future Sign Pricing
    future_sign_consensus_validation_ika: u64,
    future_sign_computation_ika: u64,
    future_sign_gas_fee_reimbursement_sui: u64,
    // ECDSA Sign with Partial User Signature Pricing
    partial_sign_consensus_validation_ika: u64,
    partial_sign_computation_ika: u64,
    partial_sign_gas_fee_reimbursement_sui: u64,
    // Make DWallet User Secret Key Shares Public Pricing
    make_dwallet_user_secret_key_shares_public_consensus_validation_ika: u64,
    make_dwallet_user_secret_key_shares_public_computation_ika: u64,
    make_dwallet_user_secret_key_shares_public_gas_fee_reimbursement_sui: u64,
    ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a> {
    <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a> {
        id: object::new(ctx),
        <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_dkg_first_round">dkg_first_round</a>: <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a> {
            <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_consensus_validation_ika">consensus_validation_ika</a>: dkg_first_round_consensus_validation_ika,
            <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika">computation_ika</a>: dkg_first_round_computation_ika,
            <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_gas_fee_reimbursement_sui">gas_fee_reimbursement_sui</a>: dkg_first_round_gas_fee_reimbursement_sui,
        },
        <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_dkg_second_round">dkg_second_round</a>: <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a> {
            <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_consensus_validation_ika">consensus_validation_ika</a>: dkg_second_round_consensus_validation_ika,
            <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika">computation_ika</a>: dkg_second_round_computation_ika,
            <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_gas_fee_reimbursement_sui">gas_fee_reimbursement_sui</a>: dkg_second_round_gas_fee_reimbursement_sui,
        },
        <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_re_encrypt_user_share">re_encrypt_user_share</a>: <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a> {
            <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_consensus_validation_ika">consensus_validation_ika</a>: re_encrypt_consensus_validation_ika,
            <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika">computation_ika</a>: re_encrypt_computation_ika,
            <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_gas_fee_reimbursement_sui">gas_fee_reimbursement_sui</a>: re_encrypt_gas_fee_reimbursement_sui,
        },
        <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_presign">presign</a>: <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a> {
            <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_consensus_validation_ika">consensus_validation_ika</a>: presign_consensus_validation_ika,
            <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika">computation_ika</a>: presign_computation_ika,
            <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_gas_fee_reimbursement_sui">gas_fee_reimbursement_sui</a>: presign_gas_fee_reimbursement_sui,
        },
        <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_sign">sign</a>: <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a> {
            <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_consensus_validation_ika">consensus_validation_ika</a>: sign_consensus_validation_ika,
            <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika">computation_ika</a>: sign_computation_ika,
            <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_gas_fee_reimbursement_sui">gas_fee_reimbursement_sui</a>: sign_gas_fee_reimbursement_sui,
        },
        <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_future_sign">future_sign</a>: <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a> {
            <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_consensus_validation_ika">consensus_validation_ika</a>: future_sign_consensus_validation_ika,
            <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika">computation_ika</a>: future_sign_computation_ika,
            <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_gas_fee_reimbursement_sui">gas_fee_reimbursement_sui</a>: future_sign_gas_fee_reimbursement_sui,
        },
        <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_sign_with_partial_user_signature">sign_with_partial_user_signature</a>: <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a> {
            <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_consensus_validation_ika">consensus_validation_ika</a>: partial_sign_consensus_validation_ika,
            <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika">computation_ika</a>: partial_sign_computation_ika,
            <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_gas_fee_reimbursement_sui">gas_fee_reimbursement_sui</a>: partial_sign_gas_fee_reimbursement_sui,
        },
        <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_make_dwallet_user_secret_key_shares_public">make_dwallet_user_secret_key_shares_public</a>: <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a> {
            <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_consensus_validation_ika">consensus_validation_ika</a>: make_dwallet_user_secret_key_shares_public_consensus_validation_ika,
            <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika">computation_ika</a>: make_dwallet_user_secret_key_shares_public_computation_ika,
            <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_gas_fee_reimbursement_sui">gas_fee_reimbursement_sui</a>: make_dwallet_user_secret_key_shares_public_gas_fee_reimbursement_sui,
        },
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_pricing_zero"></a>

## Function `zero`

Returns zero <code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_zero">zero</a>(): (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">dwallet_pricing::PricingPerOperation</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_zero">zero</a>(): <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a> {
    <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a> {
        <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_consensus_validation_ika">consensus_validation_ika</a>: 0,
        <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika">computation_ika</a>: 0,
        <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_gas_fee_reimbursement_sui">gas_fee_reimbursement_sui</a>: 0,
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_pricing_dkg_first_round"></a>

## Function `dkg_first_round`

Returns <code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a></code> for the DKG first round.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_dkg_first_round">dkg_first_round</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">dwallet_pricing::DWalletPricing2PcMpcSecp256K1</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">dwallet_pricing::PricingPerOperation</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_dkg_first_round">dkg_first_round</a>(self: &<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a>): <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a> {
    self.<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_dkg_first_round">dkg_first_round</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_pricing_dkg_second_round"></a>

## Function `dkg_second_round`

Returns <code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a></code> for the DKG second round.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_dkg_second_round">dkg_second_round</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">dwallet_pricing::DWalletPricing2PcMpcSecp256K1</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">dwallet_pricing::PricingPerOperation</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_dkg_second_round">dkg_second_round</a>(self: &<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a>): <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a> {
    self.<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_dkg_second_round">dkg_second_round</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_pricing_re_encrypt_user_share"></a>

## Function `re_encrypt_user_share`

Returns <code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a></code> for the re-encrypt user share.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_re_encrypt_user_share">re_encrypt_user_share</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">dwallet_pricing::DWalletPricing2PcMpcSecp256K1</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">dwallet_pricing::PricingPerOperation</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_re_encrypt_user_share">re_encrypt_user_share</a>(self: &<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a>): <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a> {
    self.<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_re_encrypt_user_share">re_encrypt_user_share</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_pricing_presign"></a>

## Function `presign`

Returns <code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a></code> for the ECDSA presign.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_presign">presign</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">dwallet_pricing::DWalletPricing2PcMpcSecp256K1</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">dwallet_pricing::PricingPerOperation</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_presign">presign</a>(self: &<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a>): <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a> {
    self.<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_presign">presign</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_pricing_sign"></a>

## Function `sign`

Returns <code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a></code> for the ECDSA sign.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_sign">sign</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">dwallet_pricing::DWalletPricing2PcMpcSecp256K1</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">dwallet_pricing::PricingPerOperation</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_sign">sign</a>(self: &<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a>): <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a> {
    self.<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_sign">sign</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_pricing_future_sign"></a>

## Function `future_sign`

Returns <code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a></code> for the ECDSA future sign.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_future_sign">future_sign</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">dwallet_pricing::DWalletPricing2PcMpcSecp256K1</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">dwallet_pricing::PricingPerOperation</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_future_sign">future_sign</a>(self: &<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a>): <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a> {
    self.<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_future_sign">future_sign</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_pricing_sign_with_partial_user_signature"></a>

## Function `sign_with_partial_user_signature`

Returns <code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a></code> for the ECDSA sign with partial user signature.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_sign_with_partial_user_signature">sign_with_partial_user_signature</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">dwallet_pricing::DWalletPricing2PcMpcSecp256K1</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">dwallet_pricing::PricingPerOperation</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_sign_with_partial_user_signature">sign_with_partial_user_signature</a>(self: &<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a>): <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a> {
    self.<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_sign_with_partial_user_signature">sign_with_partial_user_signature</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_pricing_make_dwallet_user_secret_key_shares_public"></a>

## Function `make_dwallet_user_secret_key_shares_public`

Returns <code><a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a></code> for the make dWallet user secret key shares public.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_make_dwallet_user_secret_key_shares_public">make_dwallet_user_secret_key_shares_public</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">dwallet_pricing::DWalletPricing2PcMpcSecp256K1</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">dwallet_pricing::PricingPerOperation</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_make_dwallet_user_secret_key_shares_public">make_dwallet_user_secret_key_shares_public</a>(self: &<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">DWalletPricing2PcMpcSecp256K1</a>): <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a> {
    self.<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_make_dwallet_user_secret_key_shares_public">make_dwallet_user_secret_key_shares_public</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_pricing_consensus_validation_ika"></a>

## Function `consensus_validation_ika`

Getter for the consensus_validation_ika field of a PricingPerOperation.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_consensus_validation_ika">consensus_validation_ika</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">dwallet_pricing::PricingPerOperation</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_consensus_validation_ika">consensus_validation_ika</a>(self: &<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a>): u64 {
    self.<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_consensus_validation_ika">consensus_validation_ika</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_pricing_computation_ika"></a>

## Function `computation_ika`

Getter for the computation_ika field of a PricingPerOperation.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika">computation_ika</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">dwallet_pricing::PricingPerOperation</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika">computation_ika</a>(self: &<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a>): u64 {
    self.<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_computation_ika">computation_ika</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_pricing_gas_fee_reimbursement_sui"></a>

## Function `gas_fee_reimbursement_sui`

Getter for the gas_fee_reimbursement_sui field of a PricingPerOperation.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_gas_fee_reimbursement_sui">gas_fee_reimbursement_sui</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">dwallet_pricing::PricingPerOperation</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_gas_fee_reimbursement_sui">gas_fee_reimbursement_sui</a>(self: &<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">PricingPerOperation</a>): u64 {
    self.<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_gas_fee_reimbursement_sui">gas_fee_reimbursement_sui</a>
}
</code></pre>



</details>
