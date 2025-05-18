---
title: Module `(ika_system=0x0)::dwallet_2pc_mpc_coordinator`
---



-  [Struct `DWalletCoordinator`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator)
-  [Constants](#@Constants_0)
-  [Function `create_dwallet_coordinator`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_create_dwallet_coordinator)
-  [Function `share_dwallet_coordinator`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_share_dwallet_coordinator)
-  [Function `process_checkpoint_message_by_quorum`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_process_checkpoint_message_by_quorum)
-  [Function `request_dwallet_network_encryption_key_dkg`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_dwallet_network_encryption_key_dkg)
-  [Function `advance_epoch`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_advance_epoch)
-  [Function `get_active_encryption_key`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_get_active_encryption_key)
-  [Function `register_encryption_key`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_register_encryption_key)
-  [Function `approve_message`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_approve_message)
-  [Function `approve_imported_key_message`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_approve_imported_key_message)
-  [Function `request_dwallet_dkg_first_round`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_dwallet_dkg_first_round)
-  [Function `request_dwallet_dkg_second_round`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_dwallet_dkg_second_round)
-  [Function `new_imported_key_dwallet`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_new_imported_key_dwallet)
-  [Function `request_imported_key_dwallet_verification`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_imported_key_dwallet_verification)
-  [Function `request_re_encrypt_user_share_for`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_re_encrypt_user_share_for)
-  [Function `accept_encrypted_user_share`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_accept_encrypted_user_share)
-  [Function `request_presign`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_presign)
-  [Function `request_global_presign`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_global_presign)
-  [Function `is_presign_valid`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_is_presign_valid)
-  [Function `verify_presign_cap`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_verify_presign_cap)
-  [Function `request_sign`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_sign)
-  [Function `request_imported_key_sign`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_imported_key_sign)
-  [Function `request_future_sign`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_future_sign)
-  [Function `is_partial_user_signature_valid`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_is_partial_user_signature_valid)
-  [Function `verify_partial_user_signature_cap`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_verify_partial_user_signature_cap)
-  [Function `request_sign_with_partial_user_signature`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_sign_with_partial_user_signature)
-  [Function `request_imported_key_sign_with_partial_user_signature`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_imported_key_sign_with_partial_user_signature)
-  [Function `match_partial_user_signature_with_message_approval`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_match_partial_user_signature_with_message_approval)
-  [Function `match_partial_user_signature_with_imported_key_message_approval`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_match_partial_user_signature_with_imported_key_message_approval)
-  [Function `migrate`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_migrate)
-  [Function `inner_mut`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_mut)
-  [Function `inner`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner)


<pre><code><b>use</b> (ika=0x0)::ika;
<b>use</b> (ika_system=0x0)::<b>address</b>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee">bls_committee</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner">dwallet_2pc_mpc_coordinator_inner</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing">dwallet_pricing</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u64.md#std_u64">std::u64</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../sui/address.md#sui_address">sui::address</a>;
<b>use</b> <a href="../sui/bag.md#sui_bag">sui::bag</a>;
<b>use</b> <a href="../sui/balance.md#sui_balance">sui::balance</a>;
<b>use</b> <a href="../sui/bcs.md#sui_bcs">sui::bcs</a>;
<b>use</b> <a href="../sui/bls12381.md#sui_bls12381">sui::bls12381</a>;
<b>use</b> <a href="../sui/coin.md#sui_coin">sui::coin</a>;
<b>use</b> <a href="../sui/config.md#sui_config">sui::config</a>;
<b>use</b> <a href="../sui/deny_list.md#sui_deny_list">sui::deny_list</a>;
<b>use</b> <a href="../sui/dynamic_field.md#sui_dynamic_field">sui::dynamic_field</a>;
<b>use</b> <a href="../sui/dynamic_object_field.md#sui_dynamic_object_field">sui::dynamic_object_field</a>;
<b>use</b> <a href="../sui/ed25519.md#sui_ed25519">sui::ed25519</a>;
<b>use</b> <a href="../sui/event.md#sui_event">sui::event</a>;
<b>use</b> <a href="../sui/group_ops.md#sui_group_ops">sui::group_ops</a>;
<b>use</b> <a href="../sui/hash.md#sui_hash">sui::hash</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/object_table.md#sui_object_table">sui::object_table</a>;
<b>use</b> <a href="../sui/sui.md#sui_sui">sui::sui</a>;
<b>use</b> <a href="../sui/table.md#sui_table">sui::table</a>;
<b>use</b> <a href="../sui/table_vec.md#sui_table_vec">sui::table_vec</a>;
<b>use</b> <a href="../sui/transfer.md#sui_transfer">sui::transfer</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
<b>use</b> <a href="../sui/types.md#sui_types">sui::types</a>;
<b>use</b> <a href="../sui/url.md#sui_url">sui::url</a>;
<b>use</b> <a href="../sui/vec_map.md#sui_vec_map">sui::vec_map</a>;
<b>use</b> <a href="../sui/vec_set.md#sui_vec_set">sui::vec_set</a>;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator"></a>

## Struct `DWalletCoordinator`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a> <b>has</b> key
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
<code>version: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>package_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>new_package_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_EInvalidMigration"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_EInvalidMigration">EInvalidMigration</a>: u64 = 1;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_EWrongInnerVersion"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_EWrongInnerVersion">EWrongInnerVersion</a>: u64 = 0;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_VERSION"></a>

Flag to indicate the version of the ika system.


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_VERSION">VERSION</a>: u64 = 1;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_create_dwallet_coordinator"></a>

## Function `create_dwallet_coordinator`

Create a new System object and make it shared.
This function will be called only once in init.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_create_dwallet_coordinator">create_dwallet_coordinator</a>(package_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, epoch: u64, active_committee: (ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">bls_committee::BlsCommittee</a>, pricing: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">dwallet_pricing::DWalletPricing2PcMpcSecp256K1</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_create_dwallet_coordinator">create_dwallet_coordinator</a>(
    package_id: ID,
    epoch: u64,
    active_committee: BlsCommittee,
    pricing: DWalletPricing2PcMpcSecp256K1,
    ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a> {
    <b>let</b> dwallet_coordinator_inner = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_create_dwallet_coordinator_inner">dwallet_2pc_mpc_coordinator_inner::create_dwallet_coordinator_inner</a>(
        epoch,
        active_committee,
        pricing,
        ctx,
    );
    <b>let</b> <b>mut</b> self = <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a> {
        id: object::new(ctx),
        version: <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_VERSION">VERSION</a>,
        package_id,
        new_package_id: option::none(),
    };
    dynamic_field::add(&<b>mut</b> self.id, <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_VERSION">VERSION</a>, dwallet_coordinator_inner);
    self
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_share_dwallet_coordinator"></a>

## Function `share_dwallet_coordinator`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_share_dwallet_coordinator">share_dwallet_coordinator</a>(dwallet_coordinator: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_share_dwallet_coordinator">share_dwallet_coordinator</a>(
    dwallet_coordinator: <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
) {
    transfer::share_object(dwallet_coordinator);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_process_checkpoint_message_by_quorum"></a>

## Function `process_checkpoint_message_by_quorum`

Being called by the Ika network to store outputs of completed MPC sessions to Sui.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator">dwallet_2pc_mpc_coordinator</a>: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, signature: vector&lt;u8&gt;, signers_bitmap: vector&lt;u8&gt;, message: vector&lt;u8&gt;, message2: vector&lt;u8&gt;, message3: vector&lt;u8&gt;, message4: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(
    <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator">dwallet_2pc_mpc_coordinator</a>: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    signature: vector&lt;u8&gt;,
    signers_bitmap: vector&lt;u8&gt;,
    <b>mut</b> message: vector&lt;u8&gt;,
    message2: vector&lt;u8&gt;,
    message3: vector&lt;u8&gt;,
    message4: vector&lt;u8&gt;,
) {
    message.append(message2);
    message.append(message3);
    message.append(message4);
    <b>let</b> dwallet_inner = <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator">dwallet_2pc_mpc_coordinator</a>.<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_mut">inner_mut</a>();
    dwallet_inner.<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(signature, signers_bitmap, message);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_dwallet_network_encryption_key_dkg"></a>

## Function `request_dwallet_network_encryption_key_dkg`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_dwallet_network_encryption_key_dkg">request_dwallet_network_encryption_key_dkg</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyCap">dwallet_2pc_mpc_coordinator_inner::DWalletNetworkEncryptionKeyCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_dwallet_network_encryption_key_dkg">request_dwallet_network_encryption_key_dkg</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    ctx: &<b>mut</b> TxContext
): DWalletNetworkEncryptionKeyCap {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_dwallet_network_encryption_key_dkg">request_dwallet_network_encryption_key_dkg</a>(ctx)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_advance_epoch"></a>

## Function `advance_epoch`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_advance_epoch">advance_epoch</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, committee: (ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">bls_committee::BlsCommittee</a>): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_advance_epoch">advance_epoch</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    committee: BlsCommittee,
): Balance&lt;IKA&gt; {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_advance_epoch">advance_epoch</a>(committee)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_get_active_encryption_key"></a>

## Function `get_active_encryption_key`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_get_active_encryption_key">get_active_encryption_key</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, <b>address</b>: <b>address</b>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_get_active_encryption_key">get_active_encryption_key</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    <b>address</b>: <b>address</b>,
): ID {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner">inner</a>().<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_get_active_encryption_key">get_active_encryption_key</a>(<b>address</b>)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_register_encryption_key"></a>

## Function `register_encryption_key`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_register_encryption_key">register_encryption_key</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, curve: u32, encryption_key: vector&lt;u8&gt;, encryption_key_signature: vector&lt;u8&gt;, signer_public_key: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_register_encryption_key">register_encryption_key</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    curve: u32,
    encryption_key: vector&lt;u8&gt;,
    encryption_key_signature: vector&lt;u8&gt;,
    signer_public_key: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_register_encryption_key">register_encryption_key</a>(
        curve,
        encryption_key,
        encryption_key_signature,
        signer_public_key,
        ctx
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_approve_message"></a>

## Function `approve_message`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_approve_message">approve_message</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, dwallet_cap: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">dwallet_2pc_mpc_coordinator_inner::DWalletCap</a>, signature_algorithm: u32, hash_scheme: u32, message: vector&lt;u8&gt;): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">dwallet_2pc_mpc_coordinator_inner::MessageApproval</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_approve_message">approve_message</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    dwallet_cap: &DWalletCap,
    signature_algorithm: u32,
    hash_scheme: u32,
    message: vector&lt;u8&gt;
): MessageApproval {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner">inner</a>().<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_approve_message">approve_message</a>(
        dwallet_cap,
        signature_algorithm,
        hash_scheme,
        message,
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_approve_imported_key_message"></a>

## Function `approve_imported_key_message`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_approve_imported_key_message">approve_imported_key_message</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, imported_key_dwallet_cap: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyDWalletCap">dwallet_2pc_mpc_coordinator_inner::ImportedKeyDWalletCap</a>, signature_algorithm: u32, hash_scheme: u32, message: vector&lt;u8&gt;): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyMessageApproval">dwallet_2pc_mpc_coordinator_inner::ImportedKeyMessageApproval</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_approve_imported_key_message">approve_imported_key_message</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    imported_key_dwallet_cap: &ImportedKeyDWalletCap,
    signature_algorithm: u32,
    hash_scheme: u32,
    message: vector&lt;u8&gt;
): ImportedKeyMessageApproval {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner">inner</a>().<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_approve_imported_key_message">approve_imported_key_message</a>(
        imported_key_dwallet_cap,
        signature_algorithm,
        hash_scheme,
        message,
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_dwallet_dkg_first_round"></a>

## Function `request_dwallet_dkg_first_round`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_dwallet_dkg_first_round">request_dwallet_dkg_first_round</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, curve: u32, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">dwallet_2pc_mpc_coordinator_inner::DWalletCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_dwallet_dkg_first_round">request_dwallet_dkg_first_round</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    dwallet_network_decryption_key_id: ID,
    curve: u32,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
): DWalletCap {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_dwallet_dkg_first_round">request_dwallet_dkg_first_round</a>(
        dwallet_network_decryption_key_id,
        curve,
        payment_ika,
        payment_sui,
        ctx
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_dwallet_dkg_second_round"></a>

## Function `request_dwallet_dkg_second_round`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_dwallet_dkg_second_round">request_dwallet_dkg_second_round</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, dwallet_cap: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">dwallet_2pc_mpc_coordinator_inner::DWalletCap</a>, centralized_public_key_share_and_proof: vector&lt;u8&gt;, encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;, encryption_key_address: <b>address</b>, user_public_output: vector&lt;u8&gt;, singer_public_key: vector&lt;u8&gt;, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_dwallet_dkg_second_round">request_dwallet_dkg_second_round</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    dwallet_cap: &DWalletCap,
    centralized_public_key_share_and_proof: vector&lt;u8&gt;,
    encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;,
    encryption_key_address: <b>address</b>,
    user_public_output: vector&lt;u8&gt;,
    singer_public_key: vector&lt;u8&gt;,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_dwallet_dkg_second_round">request_dwallet_dkg_second_round</a>(
        dwallet_cap,
        centralized_public_key_share_and_proof,
        encrypted_centralized_secret_share_and_proof,
        encryption_key_address,
        user_public_output,
        singer_public_key,
        payment_ika,
        payment_sui,
        ctx
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_new_imported_key_dwallet"></a>

## Function `new_imported_key_dwallet`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_new_imported_key_dwallet">new_imported_key_dwallet</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, curve: u32, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyDWalletCap">dwallet_2pc_mpc_coordinator_inner::ImportedKeyDWalletCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_new_imported_key_dwallet">new_imported_key_dwallet</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    dwallet_network_decryption_key_id: ID,
    curve: u32,
    ctx: &<b>mut</b> TxContext
): ImportedKeyDWalletCap {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_new_imported_key_dwallet">new_imported_key_dwallet</a>(
        dwallet_network_decryption_key_id,
        curve,
        ctx,
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_imported_key_dwallet_verification"></a>

## Function `request_imported_key_dwallet_verification`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_imported_key_dwallet_verification">request_imported_key_dwallet_verification</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, dwallet_cap: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyDWalletCap">dwallet_2pc_mpc_coordinator_inner::ImportedKeyDWalletCap</a>, centralized_party_message: vector&lt;u8&gt;, encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;, encryption_key_address: <b>address</b>, user_public_output: vector&lt;u8&gt;, signer_public_key: vector&lt;u8&gt;, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_imported_key_dwallet_verification">request_imported_key_dwallet_verification</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    dwallet_cap: &ImportedKeyDWalletCap,
    centralized_party_message: vector&lt;u8&gt;,
    encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;,
    encryption_key_address: <b>address</b>,
    user_public_output: vector&lt;u8&gt;,
    signer_public_key: vector&lt;u8&gt;,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_imported_key_dwallet_verification">request_imported_key_dwallet_verification</a>(
        dwallet_cap,
        centralized_party_message,
        encrypted_centralized_secret_share_and_proof,
        encryption_key_address,
        user_public_output,
        signer_public_key,
        payment_ika,
        payment_sui,
        ctx,
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_re_encrypt_user_share_for"></a>

## Function `request_re_encrypt_user_share_for`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_re_encrypt_user_share_for">request_re_encrypt_user_share_for</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, destination_encryption_key_address: <b>address</b>, encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;, source_encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_re_encrypt_user_share_for">request_re_encrypt_user_share_for</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    dwallet_id: ID,
    destination_encryption_key_address: <b>address</b>,
    encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;,
    source_encrypted_user_secret_key_share_id: ID,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_re_encrypt_user_share_for">request_re_encrypt_user_share_for</a>(
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_accept_encrypted_user_share"></a>

## Function `accept_encrypted_user_share`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_accept_encrypted_user_share">accept_encrypted_user_share</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, user_output_signature: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_accept_encrypted_user_share">accept_encrypted_user_share</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    dwallet_id: ID,
    encrypted_user_secret_key_share_id: ID,
    user_output_signature: vector&lt;u8&gt;,
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_accept_encrypted_user_share">accept_encrypted_user_share</a>(
        dwallet_id,
        encrypted_user_secret_key_share_id,
        user_output_signature,
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_presign"></a>

## Function `request_presign`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_presign">request_presign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, signature_algorithm: u32, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">dwallet_2pc_mpc_coordinator_inner::UnverifiedPresignCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_presign">request_presign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    dwallet_id: ID,
    signature_algorithm: u32,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
): UnverifiedPresignCap {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_presign">request_presign</a>(
        dwallet_id,
        signature_algorithm,
        payment_ika,
        payment_sui,
        ctx,
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_global_presign"></a>

## Function `request_global_presign`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_global_presign">request_global_presign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, curve: u32, signature_algorithm: u32, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">dwallet_2pc_mpc_coordinator_inner::UnverifiedPresignCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_global_presign">request_global_presign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    dwallet_network_decryption_key_id: ID,
    curve: u32,
    signature_algorithm: u32,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
): UnverifiedPresignCap {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_global_presign">request_global_presign</a>(
        dwallet_network_decryption_key_id,
        curve,
        signature_algorithm,
        payment_ika,
        payment_sui,
        ctx,
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_is_presign_valid"></a>

## Function `is_presign_valid`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_is_presign_valid">is_presign_valid</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, presign_cap: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">dwallet_2pc_mpc_coordinator_inner::UnverifiedPresignCap</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_is_presign_valid">is_presign_valid</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    presign_cap: &UnverifiedPresignCap,
): bool {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner">inner</a>().<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_is_presign_valid">is_presign_valid</a>(
        presign_cap,
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_verify_presign_cap"></a>

## Function `verify_presign_cap`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_verify_presign_cap">verify_presign_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">dwallet_2pc_mpc_coordinator_inner::UnverifiedPresignCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">dwallet_2pc_mpc_coordinator_inner::VerifiedPresignCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_verify_presign_cap">verify_presign_cap</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    cap: UnverifiedPresignCap,
    ctx: &<b>mut</b> TxContext
): VerifiedPresignCap {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_verify_presign_cap">verify_presign_cap</a>(cap, ctx)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_sign"></a>

## Function `request_sign`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_sign">request_sign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, presign_cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">dwallet_2pc_mpc_coordinator_inner::VerifiedPresignCap</a>, message_approval: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">dwallet_2pc_mpc_coordinator_inner::MessageApproval</a>, message_centralized_signature: vector&lt;u8&gt;, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_sign">request_sign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    presign_cap: VerifiedPresignCap,
    message_approval: MessageApproval,
    message_centralized_signature: vector&lt;u8&gt;,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_sign">request_sign</a>(
        message_approval,
        presign_cap,
        message_centralized_signature,
        payment_ika,
        payment_sui,
        ctx
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_imported_key_sign"></a>

## Function `request_imported_key_sign`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_imported_key_sign">request_imported_key_sign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, presign_cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">dwallet_2pc_mpc_coordinator_inner::VerifiedPresignCap</a>, message_approval: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyMessageApproval">dwallet_2pc_mpc_coordinator_inner::ImportedKeyMessageApproval</a>, message_centralized_signature: vector&lt;u8&gt;, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_imported_key_sign">request_imported_key_sign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    presign_cap: VerifiedPresignCap,
    message_approval: ImportedKeyMessageApproval,
    message_centralized_signature: vector&lt;u8&gt;,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_imported_key_sign">request_imported_key_sign</a>(
        message_approval,
        presign_cap,
        message_centralized_signature,
        payment_ika,
        payment_sui,
        ctx
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_future_sign"></a>

## Function `request_future_sign`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_future_sign">request_future_sign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, presign_cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">dwallet_2pc_mpc_coordinator_inner::VerifiedPresignCap</a>, message: vector&lt;u8&gt;, hash_scheme: u32, message_centralized_signature: vector&lt;u8&gt;, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPartialUserSignatureCap">dwallet_2pc_mpc_coordinator_inner::UnverifiedPartialUserSignatureCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_future_sign">request_future_sign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    dwallet_id: ID,
    presign_cap: VerifiedPresignCap,
    message: vector&lt;u8&gt;,
    hash_scheme: u32,
    message_centralized_signature: vector&lt;u8&gt;,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
): UnverifiedPartialUserSignatureCap {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_future_sign">request_future_sign</a>(
        dwallet_id,
        presign_cap,
        message,
        hash_scheme,
        message_centralized_signature,
        payment_ika,
        payment_sui,
        ctx,
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_is_partial_user_signature_valid"></a>

## Function `is_partial_user_signature_valid`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_is_partial_user_signature_valid">is_partial_user_signature_valid</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, cap: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPartialUserSignatureCap">dwallet_2pc_mpc_coordinator_inner::UnverifiedPartialUserSignatureCap</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_is_partial_user_signature_valid">is_partial_user_signature_valid</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    cap: &UnverifiedPartialUserSignatureCap,
): bool {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner">inner</a>().<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_is_partial_user_signature_valid">is_partial_user_signature_valid</a>(cap)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_verify_partial_user_signature_cap"></a>

## Function `verify_partial_user_signature_cap`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_verify_partial_user_signature_cap">verify_partial_user_signature_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPartialUserSignatureCap">dwallet_2pc_mpc_coordinator_inner::UnverifiedPartialUserSignatureCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap">dwallet_2pc_mpc_coordinator_inner::VerifiedPartialUserSignatureCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_verify_partial_user_signature_cap">verify_partial_user_signature_cap</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    cap: UnverifiedPartialUserSignatureCap,
    ctx: &<b>mut</b> TxContext
): VerifiedPartialUserSignatureCap {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_verify_partial_user_signature_cap">verify_partial_user_signature_cap</a>(
        cap,
        ctx,
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_sign_with_partial_user_signature"></a>

## Function `request_sign_with_partial_user_signature`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_sign_with_partial_user_signature">request_sign_with_partial_user_signature</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, partial_user_signature_cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap">dwallet_2pc_mpc_coordinator_inner::VerifiedPartialUserSignatureCap</a>, message_approval: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">dwallet_2pc_mpc_coordinator_inner::MessageApproval</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_sign_with_partial_user_signature">request_sign_with_partial_user_signature</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    partial_user_signature_cap: VerifiedPartialUserSignatureCap,
    message_approval: MessageApproval,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_sign_with_partial_user_signature">request_sign_with_partial_user_signature</a>(
        partial_user_signature_cap,
        message_approval,
        payment_ika,
        payment_sui,
        ctx,
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_imported_key_sign_with_partial_user_signature"></a>

## Function `request_imported_key_sign_with_partial_user_signature`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_imported_key_sign_with_partial_user_signature">request_imported_key_sign_with_partial_user_signature</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, partial_user_signature_cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap">dwallet_2pc_mpc_coordinator_inner::VerifiedPartialUserSignatureCap</a>, message_approval: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyMessageApproval">dwallet_2pc_mpc_coordinator_inner::ImportedKeyMessageApproval</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_imported_key_sign_with_partial_user_signature">request_imported_key_sign_with_partial_user_signature</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    partial_user_signature_cap: VerifiedPartialUserSignatureCap,
    message_approval: ImportedKeyMessageApproval,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_request_imported_key_sign_with_partial_user_signature">request_imported_key_sign_with_partial_user_signature</a>(
        partial_user_signature_cap,
        message_approval,
        payment_ika,
        payment_sui,
        ctx,
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_match_partial_user_signature_with_message_approval"></a>

## Function `match_partial_user_signature_with_message_approval`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_match_partial_user_signature_with_message_approval">match_partial_user_signature_with_message_approval</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, partial_user_signature_cap: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap">dwallet_2pc_mpc_coordinator_inner::VerifiedPartialUserSignatureCap</a>, message_approval: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">dwallet_2pc_mpc_coordinator_inner::MessageApproval</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_match_partial_user_signature_with_message_approval">match_partial_user_signature_with_message_approval</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    partial_user_signature_cap: &VerifiedPartialUserSignatureCap,
    message_approval: &MessageApproval,
): bool {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner">inner</a>().<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_match_partial_user_signature_with_message_approval">match_partial_user_signature_with_message_approval</a>(
        partial_user_signature_cap,
        message_approval,
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_match_partial_user_signature_with_imported_key_message_approval"></a>

## Function `match_partial_user_signature_with_imported_key_message_approval`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_match_partial_user_signature_with_imported_key_message_approval">match_partial_user_signature_with_imported_key_message_approval</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>, partial_user_signature_cap: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap">dwallet_2pc_mpc_coordinator_inner::VerifiedPartialUserSignatureCap</a>, message_approval: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyMessageApproval">dwallet_2pc_mpc_coordinator_inner::ImportedKeyMessageApproval</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_match_partial_user_signature_with_imported_key_message_approval">match_partial_user_signature_with_imported_key_message_approval</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    partial_user_signature_cap: &VerifiedPartialUserSignatureCap,
    message_approval: &ImportedKeyMessageApproval,
): bool {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner">inner</a>().<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_match_partial_user_signature_with_imported_key_message_approval">match_partial_user_signature_with_imported_key_message_approval</a>(
        partial_user_signature_cap,
        message_approval,
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_migrate"></a>

## Function `migrate`

Migrate the dwallet_2pc_mpc_coordinator object to the new package id.

This function sets the new package id and version and can be modified in future versions
to migrate changes in the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner">dwallet_2pc_mpc_coordinator_inner</a></code> object if needed.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_migrate">migrate</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_migrate">migrate</a>(
        self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
) {
    <b>assert</b>!(self.version &lt; <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_VERSION">VERSION</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_EInvalidMigration">EInvalidMigration</a>);
    // Move the old <a href="../ika_system/system.md#(ika_system=0x0)_system">system</a> state <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner">inner</a> to the new version.
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner">dwallet_2pc_mpc_coordinator_inner</a>: DWalletCoordinatorInner = dynamic_field::remove(&<b>mut</b> self.id, self.version);
    dynamic_field::add(&<b>mut</b> self.id, <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_VERSION">VERSION</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner">dwallet_2pc_mpc_coordinator_inner</a>);
    self.version = <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_VERSION">VERSION</a>;
    // Set the new package id.
    <b>assert</b>!(self.new_package_id.is_some(), <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_EInvalidMigration">EInvalidMigration</a>);
    self.package_id = self.new_package_id.extract();
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_mut"></a>

## Function `inner_mut`

Get a mutable reference to <code>DWalletCoordinatorInnerVX</code> from the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a></code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_mut">inner_mut</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>): &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_mut">inner_mut</a>(self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a>): &<b>mut</b> DWalletCoordinatorInner {
    <b>assert</b>!(self.version == <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_VERSION">VERSION</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_EWrongInnerVersion">EWrongInnerVersion</a>);
    dynamic_field::borrow_mut(&<b>mut</b> self.id, <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_VERSION">VERSION</a>)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner"></a>

## Function `inner`

Get an immutable reference to <code>DWalletCoordinatorVX</code> from the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a></code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner">inner</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">dwallet_2pc_mpc_coordinator::DWalletCoordinator</a>): &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner">inner</a>(self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_DWalletCoordinator">DWalletCoordinator</a>): &DWalletCoordinatorInner {
    <b>assert</b>!(self.version == <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_VERSION">VERSION</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_EWrongInnerVersion">EWrongInnerVersion</a>);
    dynamic_field::borrow(&self.id, <a href="../ika_system/dwallet_2pc_mpc_coordinator.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_VERSION">VERSION</a>)
}
</code></pre>



</details>
