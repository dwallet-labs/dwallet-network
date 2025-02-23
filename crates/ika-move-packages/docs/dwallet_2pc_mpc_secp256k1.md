---
title: Module `(ika_system=0x0)::dwallet_2pc_mpc_secp256k1`
---



-  [Struct `DWallet2PcMpcSecp256K1`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1)
-  [Constants](#@Constants_0)
-  [Function `create`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_create)
-  [Function `set_active_committee`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_set_active_committee)
-  [Function `get_active_encryption_key`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_get_active_encryption_key)
-  [Function `register_encryption_key`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_register_encryption_key)
-  [Function `request_dkg_first_round`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_dkg_first_round)
-  [Function `request_dkg_second_round`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_dkg_second_round)
-  [Function `request_re_encrypt_user_share_for`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_re_encrypt_user_share_for)
-  [Function `accept_encrypted_user_share`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_accept_encrypted_user_share)
-  [Function `request_ecdsa_presign`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_ecdsa_presign)
-  [Function `request_ecdsa_sign`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_ecdsa_sign)
-  [Function `request_ecdsa_future_sign`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_ecdsa_future_sign)
-  [Function `verifiy_ecdsa_partial_user_signature_cap`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_verifiy_ecdsa_partial_user_signature_cap)
-  [Function `request_ecdsa_sign_with_partial_user_signatures`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_ecdsa_sign_with_partial_user_signatures)
-  [Function `compare_ecdsa_partial_user_signatures_with_approvals`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_compare_ecdsa_partial_user_signatures_with_approvals)
-  [Function `process_checkpoint_message_by_quorum`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_process_checkpoint_message_by_quorum)
-  [Function `migrate`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_migrate)
-  [Function `inner_mut`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_mut)
-  [Function `inner`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner)


<pre><code><b>use</b> (ika=0x0)::ika;
<b>use</b> (ika_system=0x0)::<b>address</b>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/committee.md#(ika_system=0x0)_committee">committee</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner">dwallet_2pc_mpc_secp256k1_inner</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing">dwallet_pricing</a>;
<b>use</b> <a href="../../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../../std/u64.md#std_u64">std::u64</a>;
<b>use</b> <a href="../../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../../sui/address.md#sui_address">sui::address</a>;
<b>use</b> <a href="../../sui/bag.md#sui_bag">sui::bag</a>;
<b>use</b> <a href="../../sui/balance.md#sui_balance">sui::balance</a>;
<b>use</b> <a href="../../sui/bcs.md#sui_bcs">sui::bcs</a>;
<b>use</b> <a href="../../sui/bls12381.md#sui_bls12381">sui::bls12381</a>;
<b>use</b> <a href="../../sui/coin.md#sui_coin">sui::coin</a>;
<b>use</b> <a href="../../sui/config.md#sui_config">sui::config</a>;
<b>use</b> <a href="../../sui/deny_list.md#sui_deny_list">sui::deny_list</a>;
<b>use</b> <a href="../../sui/dynamic_field.md#sui_dynamic_field">sui::dynamic_field</a>;
<b>use</b> <a href="../../sui/dynamic_object_field.md#sui_dynamic_object_field">sui::dynamic_object_field</a>;
<b>use</b> <a href="../../sui/ed25519.md#sui_ed25519">sui::ed25519</a>;
<b>use</b> <a href="../../sui/event.md#sui_event">sui::event</a>;
<b>use</b> <a href="../../sui/group_ops.md#sui_group_ops">sui::group_ops</a>;
<b>use</b> <a href="../../sui/hash.md#sui_hash">sui::hash</a>;
<b>use</b> <a href="../../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../../sui/object_table.md#sui_object_table">sui::object_table</a>;
<b>use</b> <a href="../../sui/sui.md#sui_sui">sui::sui</a>;
<b>use</b> <a href="../../sui/table.md#sui_table">sui::table</a>;
<b>use</b> <a href="../../sui/transfer.md#sui_transfer">sui::transfer</a>;
<b>use</b> <a href="../../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
<b>use</b> <a href="../../sui/types.md#sui_types">sui::types</a>;
<b>use</b> <a href="../../sui/url.md#sui_url">sui::url</a>;
<b>use</b> <a href="../../sui/vec_set.md#sui_vec_set">sui::vec_set</a>;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1"></a>

## Struct `DWallet2PcMpcSecp256K1`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">DWallet2PcMpcSecp256K1</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>package_id: <a href="../../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>new_package_id: <a href="../../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_EInvalidMigration"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_EInvalidMigration">EInvalidMigration</a>: u64 = 1;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_EWrongInnerVersion"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_EWrongInnerVersion">EWrongInnerVersion</a>: u64 = 0;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_VERSION"></a>

Flag to indicate the version of the ika system.


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_VERSION">VERSION</a>: u64 = 1;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_create"></a>

## Function `create`

Create a new System object and make it shared.
This function will be called only once in init.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_create">create</a>(package_id: <a href="../../sui/object.md#sui_object_ID">sui::object::ID</a>, epoch: u64, active_committee: (ika_system=0x0)::<a href="../ika_system/committee.md#(ika_system=0x0)_committee_Committee">committee::Committee</a>, pricing: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">dwallet_pricing::DWalletPricing2PcMpcSecp256K1</a>, ctx: &<b>mut</b> <a href="../../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (<a href="../../sui/object.md#sui_object_ID">sui::object::ID</a>, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletNetworkDecryptionKeyCap">dwallet_2pc_mpc_secp256k1_inner::DWalletNetworkDecryptionKeyCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_create">create</a>(
    package_id: ID,
    epoch: u64,
    active_committee: Committee,
    pricing: DWalletPricing2PcMpcSecp256K1,
    ctx: &<b>mut</b> TxContext
): (ID, DWalletNetworkDecryptionKeyCap) {
    <b>let</b> <b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1">dwallet_2pc_mpc_secp256k1</a> = <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_create">dwallet_2pc_mpc_secp256k1_inner::create</a>(
        epoch,
        active_committee,
        pricing,
        ctx,
    );
        // TODO: remove this code!
    <b>let</b> cap = <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1">dwallet_2pc_mpc_secp256k1</a>.create_dwallet_network_decryption_key(ctx);
    <b>let</b> <b>mut</b> self = <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">DWallet2PcMpcSecp256K1</a> {
        id: object::new(ctx),
        version: <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_VERSION">VERSION</a>,
        package_id,
        new_package_id: option::none(),
    };
    <b>let</b> self_id = object::id(&self);
    dynamic_field::add(&<b>mut</b> self.id, <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_VERSION">VERSION</a>, <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1">dwallet_2pc_mpc_secp256k1</a>);
    transfer::share_object(self);
    (self_id, cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_set_active_committee"></a>

## Function `set_active_committee`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_set_active_committee">set_active_committee</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">dwallet_2pc_mpc_secp256k1::DWallet2PcMpcSecp256K1</a>, <a href="../ika_system/committee.md#(ika_system=0x0)_committee">committee</a>: (ika_system=0x0)::<a href="../ika_system/committee.md#(ika_system=0x0)_committee_Committee">committee::Committee</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_set_active_committee">set_active_committee</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">DWallet2PcMpcSecp256K1</a>,
    <a href="../ika_system/committee.md#(ika_system=0x0)_committee">committee</a>: Committee,
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_mut">inner_mut</a>().<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_set_active_committee">set_active_committee</a>(<a href="../ika_system/committee.md#(ika_system=0x0)_committee">committee</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_get_active_encryption_key"></a>

## Function `get_active_encryption_key`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_get_active_encryption_key">get_active_encryption_key</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">dwallet_2pc_mpc_secp256k1::DWallet2PcMpcSecp256K1</a>, <b>address</b>: <b>address</b>): <a href="../../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_get_active_encryption_key">get_active_encryption_key</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">DWallet2PcMpcSecp256K1</a>,
    <b>address</b>: <b>address</b>,
): ID {
    self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner">inner</a>().<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_get_active_encryption_key">get_active_encryption_key</a>(<b>address</b>)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_register_encryption_key"></a>

## Function `register_encryption_key`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_register_encryption_key">register_encryption_key</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">dwallet_2pc_mpc_secp256k1::DWallet2PcMpcSecp256K1</a>, encryption_key: vector&lt;u8&gt;, encryption_key_signature: vector&lt;u8&gt;, signer_public_key: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_register_encryption_key">register_encryption_key</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">DWallet2PcMpcSecp256K1</a>,
    encryption_key: vector&lt;u8&gt;,
    encryption_key_signature: vector&lt;u8&gt;,
    signer_public_key: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_mut">inner_mut</a>().<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_register_encryption_key">register_encryption_key</a>(
        encryption_key,
        encryption_key_signature,
        signer_public_key,
        ctx
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_dkg_first_round"></a>

## Function `request_dkg_first_round`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_dkg_first_round">request_dkg_first_round</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">dwallet_2pc_mpc_secp256k1::DWallet2PcMpcSecp256K1</a>, dwallet_network_decryption_key_id: <a href="../../sui/object.md#sui_object_ID">sui::object::ID</a>, payment_ika: &<b>mut</b> <a href="../../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCap">dwallet_2pc_mpc_secp256k1_inner::DWalletCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_dkg_first_round">request_dkg_first_round</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">DWallet2PcMpcSecp256K1</a>,
    dwallet_network_decryption_key_id: ID,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
): DWalletCap {
    self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_mut">inner_mut</a>().<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_dkg_first_round">request_dkg_first_round</a>(
        dwallet_network_decryption_key_id,
        payment_ika,
        payment_sui,
        ctx
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_dkg_second_round"></a>

## Function `request_dkg_second_round`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_dkg_second_round">request_dkg_second_round</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">dwallet_2pc_mpc_secp256k1::DWallet2PcMpcSecp256K1</a>, dwallet_cap: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCap">dwallet_2pc_mpc_secp256k1_inner::DWalletCap</a>, centralized_public_key_share_and_proof: vector&lt;u8&gt;, encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;, encryption_key_address: <b>address</b>, user_public_output: vector&lt;u8&gt;, singer_public_key: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_dkg_second_round">request_dkg_second_round</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">DWallet2PcMpcSecp256K1</a>,
    dwallet_cap: &DWalletCap,
    centralized_public_key_share_and_proof: vector&lt;u8&gt;,
    encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;,
    encryption_key_address: <b>address</b>,
    user_public_output: vector&lt;u8&gt;,
    singer_public_key: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_mut">inner_mut</a>().<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_dkg_second_round">request_dkg_second_round</a>(
        dwallet_cap,
        centralized_public_key_share_and_proof,
        encrypted_centralized_secret_share_and_proof,
        encryption_key_address,
        user_public_output,
        singer_public_key,
        ctx
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_re_encrypt_user_share_for"></a>

## Function `request_re_encrypt_user_share_for`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_re_encrypt_user_share_for">request_re_encrypt_user_share_for</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">dwallet_2pc_mpc_secp256k1::DWallet2PcMpcSecp256K1</a>, dwallet_id: <a href="../../sui/object.md#sui_object_ID">sui::object::ID</a>, destination_encryption_key_address: <b>address</b>, encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;, source_encrypted_user_secret_key_share_id: <a href="../../sui/object.md#sui_object_ID">sui::object::ID</a>, payment_ika: &<b>mut</b> <a href="../../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_re_encrypt_user_share_for">request_re_encrypt_user_share_for</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">DWallet2PcMpcSecp256K1</a>,
    dwallet_id: ID,
    destination_encryption_key_address: <b>address</b>,
    encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;,
    source_encrypted_user_secret_key_share_id: ID,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_mut">inner_mut</a>().<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_re_encrypt_user_share_for">request_re_encrypt_user_share_for</a>(
        dwallet_id,
        destination_encryption_key_address,
        encrypted_centralized_secret_share_and_proof,
        source_encrypted_user_secret_key_share_id,
        payment_ika,
        payment_sui,
        ctx,
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_accept_encrypted_user_share"></a>

## Function `accept_encrypted_user_share`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_accept_encrypted_user_share">accept_encrypted_user_share</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">dwallet_2pc_mpc_secp256k1::DWallet2PcMpcSecp256K1</a>, dwallet_id: <a href="../../sui/object.md#sui_object_ID">sui::object::ID</a>, encrypted_user_secret_key_share_id: <a href="../../sui/object.md#sui_object_ID">sui::object::ID</a>, user_output_signature: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_accept_encrypted_user_share">accept_encrypted_user_share</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">DWallet2PcMpcSecp256K1</a>,
    dwallet_id: ID,
    encrypted_user_secret_key_share_id: ID,
    user_output_signature: vector&lt;u8&gt;,
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_mut">inner_mut</a>().<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_accept_encrypted_user_share">accept_encrypted_user_share</a>(
        dwallet_id,
        encrypted_user_secret_key_share_id,
        user_output_signature,
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_ecdsa_presign"></a>

## Function `request_ecdsa_presign`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_ecdsa_presign">request_ecdsa_presign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">dwallet_2pc_mpc_secp256k1::DWallet2PcMpcSecp256K1</a>, dwallet_id: <a href="../../sui/object.md#sui_object_ID">sui::object::ID</a>, payment_ika: &<b>mut</b> <a href="../../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_ecdsa_presign">request_ecdsa_presign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">DWallet2PcMpcSecp256K1</a>,
    dwallet_id: ID,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_mut">inner_mut</a>().<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_ecdsa_presign">request_ecdsa_presign</a>(
        dwallet_id,
        payment_ika,
        payment_sui,
        ctx,
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_ecdsa_sign"></a>

## Function `request_ecdsa_sign`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_ecdsa_sign">request_ecdsa_sign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">dwallet_2pc_mpc_secp256k1::DWallet2PcMpcSecp256K1</a>, dwallet_id: <a href="../../sui/object.md#sui_object_ID">sui::object::ID</a>, message_approval: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_MessageApproval">dwallet_2pc_mpc_secp256k1_inner::MessageApproval</a>, presign_id: <a href="../../sui/object.md#sui_object_ID">sui::object::ID</a>, message_centralized_signature: vector&lt;u8&gt;, payment_ika: &<b>mut</b> <a href="../../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_ecdsa_sign">request_ecdsa_sign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">DWallet2PcMpcSecp256K1</a>,
    dwallet_id: ID,
    message_approval: MessageApproval,
    presign_id: ID,
    message_centralized_signature: vector&lt;u8&gt;,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_mut">inner_mut</a>().<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_ecdsa_sign">request_ecdsa_sign</a>(
        dwallet_id,
        message_approval,
        presign_id,
        message_centralized_signature,
        payment_ika,
        payment_sui,
        ctx
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_ecdsa_future_sign"></a>

## Function `request_ecdsa_future_sign`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_ecdsa_future_sign">request_ecdsa_future_sign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">dwallet_2pc_mpc_secp256k1::DWallet2PcMpcSecp256K1</a>, dwallet_id: <a href="../../sui/object.md#sui_object_ID">sui::object::ID</a>, message: vector&lt;u8&gt;, presign_id: <a href="../../sui/object.md#sui_object_ID">sui::object::ID</a>, hash_scheme: u8, message_centralized_signature: vector&lt;u8&gt;, payment_ika: &<b>mut</b> <a href="../../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_UnverifiedECDSAPartialUserSignatureCap">dwallet_2pc_mpc_secp256k1_inner::UnverifiedECDSAPartialUserSignatureCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_ecdsa_future_sign">request_ecdsa_future_sign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">DWallet2PcMpcSecp256K1</a>,
    dwallet_id: ID,
    message: vector&lt;u8&gt;,
    presign_id: ID,
    hash_scheme: u8,
    message_centralized_signature: vector&lt;u8&gt;,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
): UnverifiedECDSAPartialUserSignatureCap {
    self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_mut">inner_mut</a>().<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_ecdsa_future_sign">request_ecdsa_future_sign</a>(
        dwallet_id,
        message,
        presign_id,
        hash_scheme,
        message_centralized_signature,
        payment_ika,
        payment_sui,
        ctx,
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_verifiy_ecdsa_partial_user_signature_cap"></a>

## Function `verifiy_ecdsa_partial_user_signature_cap`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_verifiy_ecdsa_partial_user_signature_cap">verifiy_ecdsa_partial_user_signature_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">dwallet_2pc_mpc_secp256k1::DWallet2PcMpcSecp256K1</a>, cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_UnverifiedECDSAPartialUserSignatureCap">dwallet_2pc_mpc_secp256k1_inner::UnverifiedECDSAPartialUserSignatureCap</a>, ctx: &<b>mut</b> <a href="../../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_VerifiedECDSAPartialUserSignatureCap">dwallet_2pc_mpc_secp256k1_inner::VerifiedECDSAPartialUserSignatureCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_verifiy_ecdsa_partial_user_signature_cap">verifiy_ecdsa_partial_user_signature_cap</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">DWallet2PcMpcSecp256K1</a>,
    cap: UnverifiedECDSAPartialUserSignatureCap,
    ctx: &<b>mut</b> TxContext
): VerifiedECDSAPartialUserSignatureCap {
    self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_mut">inner_mut</a>().<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_verifiy_ecdsa_partial_user_signature_cap">verifiy_ecdsa_partial_user_signature_cap</a>(
        cap,
        ctx,
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_ecdsa_sign_with_partial_user_signatures"></a>

## Function `request_ecdsa_sign_with_partial_user_signatures`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_ecdsa_sign_with_partial_user_signatures">request_ecdsa_sign_with_partial_user_signatures</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">dwallet_2pc_mpc_secp256k1::DWallet2PcMpcSecp256K1</a>, dwallet_id: <a href="../../sui/object.md#sui_object_ID">sui::object::ID</a>, partial_user_signature_cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_VerifiedECDSAPartialUserSignatureCap">dwallet_2pc_mpc_secp256k1_inner::VerifiedECDSAPartialUserSignatureCap</a>, message_approval: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_MessageApproval">dwallet_2pc_mpc_secp256k1_inner::MessageApproval</a>, payment_ika: &<b>mut</b> <a href="../../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_ecdsa_sign_with_partial_user_signatures">request_ecdsa_sign_with_partial_user_signatures</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">DWallet2PcMpcSecp256K1</a>,
    dwallet_id: ID,
    partial_user_signature_cap: VerifiedECDSAPartialUserSignatureCap,
    message_approval: MessageApproval,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_mut">inner_mut</a>().<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_request_ecdsa_sign_with_partial_user_signatures">request_ecdsa_sign_with_partial_user_signatures</a>(
        dwallet_id,
        partial_user_signature_cap,
        message_approval,
        payment_ika,
        payment_sui,
        ctx,
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_compare_ecdsa_partial_user_signatures_with_approvals"></a>

## Function `compare_ecdsa_partial_user_signatures_with_approvals`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_compare_ecdsa_partial_user_signatures_with_approvals">compare_ecdsa_partial_user_signatures_with_approvals</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">dwallet_2pc_mpc_secp256k1::DWallet2PcMpcSecp256K1</a>, partial_user_signature_cap: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_VerifiedECDSAPartialUserSignatureCap">dwallet_2pc_mpc_secp256k1_inner::VerifiedECDSAPartialUserSignatureCap</a>, message_approval: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_MessageApproval">dwallet_2pc_mpc_secp256k1_inner::MessageApproval</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_compare_ecdsa_partial_user_signatures_with_approvals">compare_ecdsa_partial_user_signatures_with_approvals</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">DWallet2PcMpcSecp256K1</a>,
    partial_user_signature_cap: &VerifiedECDSAPartialUserSignatureCap,
    message_approval: &MessageApproval,
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner">inner</a>().<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_compare_ecdsa_partial_user_signatures_with_approvals">compare_ecdsa_partial_user_signatures_with_approvals</a>(
        partial_user_signature_cap,
        message_approval,
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_process_checkpoint_message_by_quorum"></a>

## Function `process_checkpoint_message_by_quorum`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">dwallet_2pc_mpc_secp256k1::DWallet2PcMpcSecp256K1</a>, signature: vector&lt;u8&gt;, signers_bitmap: vector&lt;u8&gt;, message: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">DWallet2PcMpcSecp256K1</a>,
    signature: vector&lt;u8&gt;,
    signers_bitmap: vector&lt;u8&gt;,
    message: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_mut">inner_mut</a>();
    self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(signature, signers_bitmap, message, ctx);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_migrate"></a>

## Function `migrate`

Migrate the dwallet_2pc_mpc_secp256k1 object to the new package id.

This function sets the new package id and version and can be modified in future versions
to migrate changes in the <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner">dwallet_2pc_mpc_secp256k1_inner</a></code> object if needed.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_migrate">migrate</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">dwallet_2pc_mpc_secp256k1::DWallet2PcMpcSecp256K1</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_migrate">migrate</a>(
        self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">DWallet2PcMpcSecp256K1</a>,
) {
    <b>assert</b>!(self.version &lt; <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_VERSION">VERSION</a>, <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_EInvalidMigration">EInvalidMigration</a>);
    // Move the old <a href="../ika_system/system.md#(ika_system=0x0)_system">system</a> state <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner">inner</a> to the new version.
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner">dwallet_2pc_mpc_secp256k1_inner</a>: DWallet2PcMpcSecp256K1InnerV1 = dynamic_field::remove(&<b>mut</b> self.id, self.version);
    dynamic_field::add(&<b>mut</b> self.id, <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_VERSION">VERSION</a>, <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner">dwallet_2pc_mpc_secp256k1_inner</a>);
    self.version = <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_VERSION">VERSION</a>;
    // Set the new package id.
    <b>assert</b>!(self.new_package_id.is_some(), <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_EInvalidMigration">EInvalidMigration</a>);
    self.package_id = self.new_package_id.extract();
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_mut"></a>

## Function `inner_mut`

Get a mutable reference to <code>DWallet2PcMpcSecp256K1InnerVX</code> from the <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">DWallet2PcMpcSecp256K1</a></code>.


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_mut">inner_mut</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">dwallet_2pc_mpc_secp256k1::DWallet2PcMpcSecp256K1</a>): &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet2PcMpcSecp256K1InnerV1">dwallet_2pc_mpc_secp256k1_inner::DWallet2PcMpcSecp256K1InnerV1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_mut">inner_mut</a>(self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">DWallet2PcMpcSecp256K1</a>): &<b>mut</b> DWallet2PcMpcSecp256K1InnerV1 {
    <b>assert</b>!(self.version == <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_VERSION">VERSION</a>, <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_EWrongInnerVersion">EWrongInnerVersion</a>);
    dynamic_field::borrow_mut(&<b>mut</b> self.id, <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_VERSION">VERSION</a>)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner"></a>

## Function `inner`

Get an immutable reference to <code>DWallet2PcMpcSecp256K1VX</code> from the <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">DWallet2PcMpcSecp256K1</a></code>.


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner">inner</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">dwallet_2pc_mpc_secp256k1::DWallet2PcMpcSecp256K1</a>): &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet2PcMpcSecp256K1InnerV1">dwallet_2pc_mpc_secp256k1_inner::DWallet2PcMpcSecp256K1InnerV1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner">inner</a>(self: &<a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_DWallet2PcMpcSecp256K1">DWallet2PcMpcSecp256K1</a>): &DWallet2PcMpcSecp256K1InnerV1 {
    <b>assert</b>!(self.version == <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_VERSION">VERSION</a>, <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_EWrongInnerVersion">EWrongInnerVersion</a>);
    dynamic_field::borrow(&self.id, <a href="../ika_system/dwallet_2pc_mpc_secp256k1.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_VERSION">VERSION</a>)
}
</code></pre>



</details>
