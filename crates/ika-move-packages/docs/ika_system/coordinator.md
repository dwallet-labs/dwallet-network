---
title: Module `(ika_dwallet_2pc_mpc=0x0)::coordinator`
---



-  [Struct `DWalletCoordinator`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator)
-  [Constants](#@Constants_0)
-  [Function `create`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_create)
-  [Function `request_dwallet_network_encryption_key_dkg_by_cap`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_dwallet_network_encryption_key_dkg_by_cap)
-  [Function `set_supported_and_pricing`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_set_supported_and_pricing)
-  [Function `set_paused_curves_and_signature_algorithms`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_set_paused_curves_and_signature_algorithms)
-  [Function `request_lock_epoch_sessions`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_lock_epoch_sessions)
-  [Function `set_pricing_vote`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_set_pricing_vote)
-  [Function `register_session_identifier`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_register_session_identifier)
-  [Function `process_checkpoint_message_by_quorum`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_process_checkpoint_message_by_quorum)
-  [Function `get_active_encryption_key`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_get_active_encryption_key)
-  [Function `register_encryption_key`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_register_encryption_key)
-  [Function `approve_message`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_approve_message)
-  [Function `approve_imported_key_message`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_approve_imported_key_message)
-  [Function `request_dwallet_dkg_first_round`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_dwallet_dkg_first_round)
-  [Function `request_dwallet_dkg_second_round`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_dwallet_dkg_second_round)
-  [Function `calculate_pricing_votes`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_calculate_pricing_votes)
-  [Function `request_imported_key_dwallet_verification`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_imported_key_dwallet_verification)
-  [Function `request_make_dwallet_user_secret_key_shares_public`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_make_dwallet_user_secret_key_shares_public)
-  [Function `request_re_encrypt_user_share_for`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_re_encrypt_user_share_for)
-  [Function `accept_encrypted_user_share`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_accept_encrypted_user_share)
-  [Function `request_presign`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_presign)
-  [Function `request_global_presign`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_global_presign)
-  [Function `is_presign_valid`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_is_presign_valid)
-  [Function `verify_presign_cap`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_verify_presign_cap)
-  [Function `request_sign`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_sign)
-  [Function `request_imported_key_sign`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_imported_key_sign)
-  [Function `request_future_sign`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_future_sign)
-  [Function `is_partial_user_signature_valid`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_is_partial_user_signature_valid)
-  [Function `verify_partial_user_signature_cap`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_verify_partial_user_signature_cap)
-  [Function `request_sign_with_partial_user_signature`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_sign_with_partial_user_signature)
-  [Function `request_imported_key_sign_with_partial_user_signature`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_imported_key_sign_with_partial_user_signature)
-  [Function `match_partial_user_signature_with_message_approval`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_match_partial_user_signature_with_message_approval)
-  [Function `match_partial_user_signature_with_imported_key_message_approval`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_match_partial_user_signature_with_imported_key_message_approval)
-  [Function `current_pricing`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_current_pricing)
-  [Function `subsidize_coordinator_with_sui`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_subsidize_coordinator_with_sui)
-  [Function `subsidize_coordinator_with_ika`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_subsidize_coordinator_with_ika)
-  [Function `migrate`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_migrate)
-  [Function `inner_mut`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_mut)
-  [Function `inner`](#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner)


<pre><code><b>use</b> (ika=0x0)::ika;
<b>use</b> (ika_common=0x0)::<b>address</b>;
<b>use</b> (ika_common=0x0)::bls_committee;
<b>use</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner">coordinator_inner</a>;
<b>use</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing">dwallet_pricing</a>;
<b>use</b> (ika_system=0x0)::advance_epoch_approver;
<b>use</b> (ika_system=0x0)::protocol_cap;
<b>use</b> (ika_system=0x0)::system_current_status_info;
<b>use</b> (ika_system=0x0)::validator_cap;
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
<b>use</b> <a href="../sui/party.md#sui_party">sui::party</a>;
<b>use</b> <a href="../sui/priority_queue.md#sui_priority_queue">sui::priority_queue</a>;
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



<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator"></a>

## Struct `DWalletCoordinator`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a> <b>has</b> key
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


<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_EWrongInnerVersion"></a>

The inner version is incorrect.


<pre><code><b>const</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_EWrongInnerVersion">EWrongInnerVersion</a>: u64 = 0;
</code></pre>



<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_EInvalidMigration"></a>

The migration is invalid.


<pre><code><b>const</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_EInvalidMigration">EInvalidMigration</a>: u64 = 1;
</code></pre>



<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_VERSION"></a>

Flag to indicate the version of the ika system.


<pre><code><b>const</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_VERSION">VERSION</a>: u64 = 1;
</code></pre>



<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_create"></a>

## Function `create`

Create a new System object and make it shared.
This function will be called only once in init.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_create">create</a>(package_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, advance_epoch_approver: &<b>mut</b> (ika_system=0x0)::advance_epoch_approver::AdvanceEpochApprover, pricing: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">dwallet_pricing::DWalletPricing</a>, supported_curves_to_signature_algorithms_to_hash_schemes: <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u32, <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u32, vector&lt;u32&gt;&gt;&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_create">create</a>(
    package_id: ID,
    advance_epoch_approver: &<b>mut</b> AdvanceEpochApprover,
    pricing: DWalletPricing,
    supported_curves_to_signature_algorithms_to_hash_schemes: VecMap&lt;u32, VecMap&lt;u32, vector&lt;u32&gt;&gt;&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> dwallet_coordinator_inner = <a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_create">coordinator_inner::create</a>(
        advance_epoch_approver,
        pricing,
        supported_curves_to_signature_algorithms_to_hash_schemes,
        ctx,
    );
    <b>let</b> <b>mut</b> self = <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a> {
        id: object::new(ctx),
        version: <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_VERSION">VERSION</a>,
        package_id,
        new_package_id: option::none(),
    };
    dynamic_field::add(&<b>mut</b> self.id, <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_VERSION">VERSION</a>, dwallet_coordinator_inner);
    transfer::share_object(self);
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_request_dwallet_network_encryption_key_dkg_by_cap"></a>

## Function `request_dwallet_network_encryption_key_dkg_by_cap`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_dwallet_network_encryption_key_dkg_by_cap">request_dwallet_network_encryption_key_dkg_by_cap</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, params_for_network: vector&lt;u8&gt;, cap: &(ika_system=0x0)::protocol_cap::VerifiedProtocolCap, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_dwallet_network_encryption_key_dkg_by_cap">request_dwallet_network_encryption_key_dkg_by_cap</a>(
    self: &<b>mut</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    params_for_network: vector&lt;u8&gt;,
    cap: &VerifiedProtocolCap,
    ctx: &<b>mut</b> TxContext,
) {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_mut">inner_mut</a>().request_dwallet_network_encryption_key_dkg(params_for_network, cap, ctx);
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_set_supported_and_pricing"></a>

## Function `set_supported_and_pricing`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_set_supported_and_pricing">set_supported_and_pricing</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, default_pricing: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">dwallet_pricing::DWalletPricing</a>, supported_curves_to_signature_algorithms_to_hash_schemes: <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u32, <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u32, vector&lt;u32&gt;&gt;&gt;, cap: &(ika_system=0x0)::protocol_cap::VerifiedProtocolCap)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_set_supported_and_pricing">set_supported_and_pricing</a>(
    self: &<b>mut</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    default_pricing: DWalletPricing,
    supported_curves_to_signature_algorithms_to_hash_schemes: VecMap&lt;u32, VecMap&lt;u32, vector&lt;u32&gt;&gt;&gt;,
    cap: &VerifiedProtocolCap,
) {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_set_supported_and_pricing">set_supported_and_pricing</a>(default_pricing, supported_curves_to_signature_algorithms_to_hash_schemes, cap);
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_set_paused_curves_and_signature_algorithms"></a>

## Function `set_paused_curves_and_signature_algorithms`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_set_paused_curves_and_signature_algorithms">set_paused_curves_and_signature_algorithms</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, paused_curves: vector&lt;u32&gt;, paused_signature_algorithms: vector&lt;u32&gt;, paused_hash_schemes: vector&lt;u32&gt;, cap: &(ika_system=0x0)::protocol_cap::VerifiedProtocolCap)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_set_paused_curves_and_signature_algorithms">set_paused_curves_and_signature_algorithms</a>(
    self: &<b>mut</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    paused_curves: vector&lt;u32&gt;,
    paused_signature_algorithms: vector&lt;u32&gt;,
    paused_hash_schemes: vector&lt;u32&gt;,
    cap: &VerifiedProtocolCap,
) {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_set_paused_curves_and_signature_algorithms">set_paused_curves_and_signature_algorithms</a>(paused_curves, paused_signature_algorithms, paused_hash_schemes, cap);
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_request_lock_epoch_sessions"></a>

## Function `request_lock_epoch_sessions`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_lock_epoch_sessions">request_lock_epoch_sessions</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, system_current_status_info: &(ika_system=0x0)::system_current_status_info::SystemCurrentStatusInfo)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_lock_epoch_sessions">request_lock_epoch_sessions</a>(
    self: &<b>mut</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    system_current_status_info: &SystemCurrentStatusInfo,
) {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_lock_epoch_sessions">request_lock_epoch_sessions</a>(system_current_status_info);
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_set_pricing_vote"></a>

## Function `set_pricing_vote`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_set_pricing_vote">set_pricing_vote</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, pricing: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">dwallet_pricing::DWalletPricing</a>, cap: &(ika_system=0x0)::validator_cap::VerifiedValidatorOperationCap)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_set_pricing_vote">set_pricing_vote</a>(
    self: &<b>mut</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    pricing: DWalletPricing,
    cap: &VerifiedValidatorOperationCap,
) {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_set_pricing_vote">set_pricing_vote</a>(pricing, cap);
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_register_session_identifier"></a>

## Function `register_session_identifier`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_register_session_identifier">register_session_identifier</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, identifier: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_SessionIdentifier">coordinator_inner::SessionIdentifier</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_register_session_identifier">register_session_identifier</a>(
    self: &<b>mut</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    identifier: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext,
): SessionIdentifier {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_register_session_identifier">register_session_identifier</a>(identifier, ctx)
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_process_checkpoint_message_by_quorum"></a>

## Function `process_checkpoint_message_by_quorum`

Being called by the Ika network to store outputs of completed MPC sessions to Sui.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(dwallet_2pc_mpc_coordinator: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, signature: vector&lt;u8&gt;, signers_bitmap: vector&lt;u8&gt;, message: vector&lt;u8&gt;, message2: vector&lt;u8&gt;, message3: vector&lt;u8&gt;, message4: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(
    dwallet_2pc_mpc_coordinator: &<b>mut</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    signature: vector&lt;u8&gt;,
    signers_bitmap: vector&lt;u8&gt;,
    <b>mut</b> message: vector&lt;u8&gt;,
    message2: vector&lt;u8&gt;,
    message3: vector&lt;u8&gt;,
    message4: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext,
): Coin&lt;SUI&gt; {
    message.append(message2);
    message.append(message3);
    message.append(message4);
    <b>let</b> dwallet_inner = dwallet_2pc_mpc_coordinator.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_mut">inner_mut</a>();
    dwallet_inner.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(signature, signers_bitmap, message, ctx)
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_get_active_encryption_key"></a>

## Function `get_active_encryption_key`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_get_active_encryption_key">get_active_encryption_key</a>(self: &(ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, <b>address</b>: <b>address</b>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_get_active_encryption_key">get_active_encryption_key</a>(
    self: &<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    <b>address</b>: <b>address</b>,
): ID {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner">inner</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_get_active_encryption_key">get_active_encryption_key</a>(<b>address</b>)
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_register_encryption_key"></a>

## Function `register_encryption_key`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_register_encryption_key">register_encryption_key</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, curve: u32, encryption_key: vector&lt;u8&gt;, encryption_key_signature: vector&lt;u8&gt;, signer_public_key: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_register_encryption_key">register_encryption_key</a>(
    self: &<b>mut</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    curve: u32,
    encryption_key: vector&lt;u8&gt;,
    encryption_key_signature: vector&lt;u8&gt;,
    signer_public_key: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext
) {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_register_encryption_key">register_encryption_key</a>(
        curve,
        encryption_key,
        encryption_key_signature,
        signer_public_key,
        ctx
    )
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_approve_message"></a>

## Function `approve_message`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_approve_message">approve_message</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, dwallet_cap: &(ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_DWalletCap">coordinator_inner::DWalletCap</a>, signature_algorithm: u32, hash_scheme: u32, message: vector&lt;u8&gt;): (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_MessageApproval">coordinator_inner::MessageApproval</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_approve_message">approve_message</a>(
    self: &<b>mut</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    dwallet_cap: &DWalletCap,
    signature_algorithm: u32,
    hash_scheme: u32,
    message: vector&lt;u8&gt;
): MessageApproval {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner">inner</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_approve_message">approve_message</a>(
        dwallet_cap,
        signature_algorithm,
        hash_scheme,
        message,
    )
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_approve_imported_key_message"></a>

## Function `approve_imported_key_message`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_approve_imported_key_message">approve_imported_key_message</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, imported_key_dwallet_cap: &(ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_ImportedKeyDWalletCap">coordinator_inner::ImportedKeyDWalletCap</a>, signature_algorithm: u32, hash_scheme: u32, message: vector&lt;u8&gt;): (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_ImportedKeyMessageApproval">coordinator_inner::ImportedKeyMessageApproval</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_approve_imported_key_message">approve_imported_key_message</a>(
    self: &<b>mut</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    imported_key_dwallet_cap: &ImportedKeyDWalletCap,
    signature_algorithm: u32,
    hash_scheme: u32,
    message: vector&lt;u8&gt;
): ImportedKeyMessageApproval {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner">inner</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_approve_imported_key_message">approve_imported_key_message</a>(
        imported_key_dwallet_cap,
        signature_algorithm,
        hash_scheme,
        message,
    )
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_request_dwallet_dkg_first_round"></a>

## Function `request_dwallet_dkg_first_round`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_dwallet_dkg_first_round">request_dwallet_dkg_first_round</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, dwallet_network_encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, curve: u32, session_identifier: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_SessionIdentifier">coordinator_inner::SessionIdentifier</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_DWalletCap">coordinator_inner::DWalletCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_dwallet_dkg_first_round">request_dwallet_dkg_first_round</a>(
    self: &<b>mut</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    dwallet_network_encryption_key_id: ID,
    curve: u32,
    session_identifier: SessionIdentifier,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
): DWalletCap {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_dwallet_dkg_first_round">request_dwallet_dkg_first_round</a>(
        dwallet_network_encryption_key_id,
        curve,
        session_identifier,
        payment_ika,
        payment_sui,
        ctx
    )
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_request_dwallet_dkg_second_round"></a>

## Function `request_dwallet_dkg_second_round`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_dwallet_dkg_second_round">request_dwallet_dkg_second_round</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, dwallet_cap: &(ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_DWalletCap">coordinator_inner::DWalletCap</a>, centralized_public_key_share_and_proof: vector&lt;u8&gt;, encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;, encryption_key_address: <b>address</b>, user_public_output: vector&lt;u8&gt;, singer_public_key: vector&lt;u8&gt;, session_identifier: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_SessionIdentifier">coordinator_inner::SessionIdentifier</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_dwallet_dkg_second_round">request_dwallet_dkg_second_round</a>(
    self: &<b>mut</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    dwallet_cap: &DWalletCap,
    centralized_public_key_share_and_proof: vector&lt;u8&gt;,
    encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;,
    encryption_key_address: <b>address</b>,
    user_public_output: vector&lt;u8&gt;,
    singer_public_key: vector&lt;u8&gt;,
    session_identifier: SessionIdentifier,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
) {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_dwallet_dkg_second_round">request_dwallet_dkg_second_round</a>(
        dwallet_cap,
        centralized_public_key_share_and_proof,
        encrypted_centralized_secret_share_and_proof,
        encryption_key_address,
        user_public_output,
        singer_public_key,
        session_identifier,
        payment_ika,
        payment_sui,
        ctx
    )
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_calculate_pricing_votes"></a>

## Function `calculate_pricing_votes`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_calculate_pricing_votes">calculate_pricing_votes</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, curve: u32, signature_algorithm: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u32&gt;, protocol: u32)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_calculate_pricing_votes">calculate_pricing_votes</a>(
    self: &<b>mut</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    curve: u32,
    signature_algorithm: Option&lt;u32&gt;,
    protocol: u32,
) {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_calculate_pricing_votes">calculate_pricing_votes</a>(curve, signature_algorithm, protocol);
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_request_imported_key_dwallet_verification"></a>

## Function `request_imported_key_dwallet_verification`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_imported_key_dwallet_verification">request_imported_key_dwallet_verification</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, dwallet_network_encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, curve: u32, centralized_party_message: vector&lt;u8&gt;, encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;, encryption_key_address: <b>address</b>, user_public_output: vector&lt;u8&gt;, signer_public_key: vector&lt;u8&gt;, session_identifier: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_SessionIdentifier">coordinator_inner::SessionIdentifier</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_ImportedKeyDWalletCap">coordinator_inner::ImportedKeyDWalletCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_imported_key_dwallet_verification">request_imported_key_dwallet_verification</a>(
    self: &<b>mut</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    dwallet_network_encryption_key_id: ID,
    curve: u32,
    centralized_party_message: vector&lt;u8&gt;,
    encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;,
    encryption_key_address: <b>address</b>,
    user_public_output: vector&lt;u8&gt;,
    signer_public_key: vector&lt;u8&gt;,
    session_identifier: SessionIdentifier,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
): ImportedKeyDWalletCap {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_imported_key_dwallet_verification">request_imported_key_dwallet_verification</a>(
        dwallet_network_encryption_key_id,
        curve,
        centralized_party_message,
        encrypted_centralized_secret_share_and_proof,
        encryption_key_address,
        user_public_output,
        signer_public_key,
        session_identifier,
        payment_ika,
        payment_sui,
        ctx,
    )
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_request_make_dwallet_user_secret_key_shares_public"></a>

## Function `request_make_dwallet_user_secret_key_shares_public`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_make_dwallet_user_secret_key_shares_public">request_make_dwallet_user_secret_key_shares_public</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, public_user_secret_key_shares: vector&lt;u8&gt;, session_identifier: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_SessionIdentifier">coordinator_inner::SessionIdentifier</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_make_dwallet_user_secret_key_shares_public">request_make_dwallet_user_secret_key_shares_public</a>(
    self: &<b>mut</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    dwallet_id: ID,
    public_user_secret_key_shares: vector&lt;u8&gt;,
    session_identifier: SessionIdentifier,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_mut">inner_mut</a>().request_make_dwallet_user_secret_key_share_public(
        dwallet_id,
        public_user_secret_key_shares,
        session_identifier,
        payment_ika,
        payment_sui,
        ctx,
    )
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_request_re_encrypt_user_share_for"></a>

## Function `request_re_encrypt_user_share_for`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_re_encrypt_user_share_for">request_re_encrypt_user_share_for</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, destination_encryption_key_address: <b>address</b>, encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;, source_encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, session_identifier: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_SessionIdentifier">coordinator_inner::SessionIdentifier</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_re_encrypt_user_share_for">request_re_encrypt_user_share_for</a>(
    self: &<b>mut</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    dwallet_id: ID,
    destination_encryption_key_address: <b>address</b>,
    encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;,
    source_encrypted_user_secret_key_share_id: ID,
    session_identifier: SessionIdentifier,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_re_encrypt_user_share_for">request_re_encrypt_user_share_for</a>(
        dwallet_id,
        destination_encryption_key_address,
        encrypted_centralized_secret_share_and_proof,
        source_encrypted_user_secret_key_share_id,
        session_identifier,
        payment_ika,
        payment_sui,
        ctx,
    )
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_accept_encrypted_user_share"></a>

## Function `accept_encrypted_user_share`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_accept_encrypted_user_share">accept_encrypted_user_share</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, user_output_signature: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_accept_encrypted_user_share">accept_encrypted_user_share</a>(
    self: &<b>mut</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    dwallet_id: ID,
    encrypted_user_secret_key_share_id: ID,
    user_output_signature: vector&lt;u8&gt;,
) {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_accept_encrypted_user_share">accept_encrypted_user_share</a>(
        dwallet_id,
        encrypted_user_secret_key_share_id,
        user_output_signature,
    )
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_request_presign"></a>

## Function `request_presign`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_presign">request_presign</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, signature_algorithm: u32, session_identifier: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_SessionIdentifier">coordinator_inner::SessionIdentifier</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_UnverifiedPresignCap">coordinator_inner::UnverifiedPresignCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_presign">request_presign</a>(
    self: &<b>mut</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    dwallet_id: ID,
    signature_algorithm: u32,
    session_identifier: SessionIdentifier,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
): UnverifiedPresignCap {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_presign">request_presign</a>(
        dwallet_id,
        signature_algorithm,
        session_identifier,
        payment_ika,
        payment_sui,
        ctx,
    )
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_request_global_presign"></a>

## Function `request_global_presign`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_global_presign">request_global_presign</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, dwallet_network_encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, curve: u32, signature_algorithm: u32, session_identifier: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_SessionIdentifier">coordinator_inner::SessionIdentifier</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_UnverifiedPresignCap">coordinator_inner::UnverifiedPresignCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_global_presign">request_global_presign</a>(
    self: &<b>mut</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    dwallet_network_encryption_key_id: ID,
    curve: u32,
    signature_algorithm: u32,
    session_identifier: SessionIdentifier,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
): UnverifiedPresignCap {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_global_presign">request_global_presign</a>(
        dwallet_network_encryption_key_id,
        curve,
        signature_algorithm,
        session_identifier,
        payment_ika,
        payment_sui,
        ctx,
    )
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_is_presign_valid"></a>

## Function `is_presign_valid`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_is_presign_valid">is_presign_valid</a>(self: &(ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, presign_cap: &(ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_UnverifiedPresignCap">coordinator_inner::UnverifiedPresignCap</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_is_presign_valid">is_presign_valid</a>(
    self: &<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    presign_cap: &UnverifiedPresignCap,
): bool {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner">inner</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_is_presign_valid">is_presign_valid</a>(
        presign_cap,
    )
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_verify_presign_cap"></a>

## Function `verify_presign_cap`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_verify_presign_cap">verify_presign_cap</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, cap: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_UnverifiedPresignCap">coordinator_inner::UnverifiedPresignCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_VerifiedPresignCap">coordinator_inner::VerifiedPresignCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_verify_presign_cap">verify_presign_cap</a>(
    self: &<b>mut</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    cap: UnverifiedPresignCap,
    ctx: &<b>mut</b> TxContext
): VerifiedPresignCap {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_verify_presign_cap">verify_presign_cap</a>(cap, ctx)
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_request_sign"></a>

## Function `request_sign`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_sign">request_sign</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, presign_cap: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_VerifiedPresignCap">coordinator_inner::VerifiedPresignCap</a>, message_approval: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_MessageApproval">coordinator_inner::MessageApproval</a>, message_centralized_signature: vector&lt;u8&gt;, session_identifier: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_SessionIdentifier">coordinator_inner::SessionIdentifier</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_sign">request_sign</a>(
    self: &<b>mut</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    presign_cap: VerifiedPresignCap,
    message_approval: MessageApproval,
    message_centralized_signature: vector&lt;u8&gt;,
    session_identifier: SessionIdentifier,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
) {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_sign">request_sign</a>(
        message_approval,
        presign_cap,
        message_centralized_signature,
        session_identifier,
        payment_ika,
        payment_sui,
        ctx
    )
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_request_imported_key_sign"></a>

## Function `request_imported_key_sign`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_imported_key_sign">request_imported_key_sign</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, presign_cap: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_VerifiedPresignCap">coordinator_inner::VerifiedPresignCap</a>, message_approval: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_ImportedKeyMessageApproval">coordinator_inner::ImportedKeyMessageApproval</a>, message_centralized_signature: vector&lt;u8&gt;, session_identifier: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_SessionIdentifier">coordinator_inner::SessionIdentifier</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_imported_key_sign">request_imported_key_sign</a>(
    self: &<b>mut</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    presign_cap: VerifiedPresignCap,
    message_approval: ImportedKeyMessageApproval,
    message_centralized_signature: vector&lt;u8&gt;,
    session_identifier: SessionIdentifier,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
) {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_imported_key_sign">request_imported_key_sign</a>(
        message_approval,
        presign_cap,
        message_centralized_signature,
        session_identifier,
        payment_ika,
        payment_sui,
        ctx
    )
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_request_future_sign"></a>

## Function `request_future_sign`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_future_sign">request_future_sign</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, presign_cap: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_VerifiedPresignCap">coordinator_inner::VerifiedPresignCap</a>, message: vector&lt;u8&gt;, hash_scheme: u32, message_centralized_signature: vector&lt;u8&gt;, session_identifier: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_SessionIdentifier">coordinator_inner::SessionIdentifier</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_UnverifiedPartialUserSignatureCap">coordinator_inner::UnverifiedPartialUserSignatureCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_future_sign">request_future_sign</a>(
    self: &<b>mut</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    dwallet_id: ID,
    presign_cap: VerifiedPresignCap,
    message: vector&lt;u8&gt;,
    hash_scheme: u32,
    message_centralized_signature: vector&lt;u8&gt;,
    session_identifier: SessionIdentifier,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
): UnverifiedPartialUserSignatureCap {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_future_sign">request_future_sign</a>(
        dwallet_id,
        presign_cap,
        message,
        hash_scheme,
        message_centralized_signature,
        session_identifier,
        payment_ika,
        payment_sui,
        ctx,
    )
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_is_partial_user_signature_valid"></a>

## Function `is_partial_user_signature_valid`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_is_partial_user_signature_valid">is_partial_user_signature_valid</a>(self: &(ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, cap: &(ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_UnverifiedPartialUserSignatureCap">coordinator_inner::UnverifiedPartialUserSignatureCap</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_is_partial_user_signature_valid">is_partial_user_signature_valid</a>(
    self: &<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    cap: &UnverifiedPartialUserSignatureCap,
): bool {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner">inner</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_is_partial_user_signature_valid">is_partial_user_signature_valid</a>(cap)
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_verify_partial_user_signature_cap"></a>

## Function `verify_partial_user_signature_cap`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_verify_partial_user_signature_cap">verify_partial_user_signature_cap</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, cap: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_UnverifiedPartialUserSignatureCap">coordinator_inner::UnverifiedPartialUserSignatureCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_VerifiedPartialUserSignatureCap">coordinator_inner::VerifiedPartialUserSignatureCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_verify_partial_user_signature_cap">verify_partial_user_signature_cap</a>(
    self: &<b>mut</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    cap: UnverifiedPartialUserSignatureCap,
    ctx: &<b>mut</b> TxContext
): VerifiedPartialUserSignatureCap {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_verify_partial_user_signature_cap">verify_partial_user_signature_cap</a>(
        cap,
        ctx,
    )
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_request_sign_with_partial_user_signature"></a>

## Function `request_sign_with_partial_user_signature`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_sign_with_partial_user_signature">request_sign_with_partial_user_signature</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, partial_user_signature_cap: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_VerifiedPartialUserSignatureCap">coordinator_inner::VerifiedPartialUserSignatureCap</a>, message_approval: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_MessageApproval">coordinator_inner::MessageApproval</a>, session_identifier: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_SessionIdentifier">coordinator_inner::SessionIdentifier</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_sign_with_partial_user_signature">request_sign_with_partial_user_signature</a>(
    self: &<b>mut</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    partial_user_signature_cap: VerifiedPartialUserSignatureCap,
    message_approval: MessageApproval,
    session_identifier: SessionIdentifier,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
) {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_sign_with_partial_user_signature">request_sign_with_partial_user_signature</a>(
        partial_user_signature_cap,
        message_approval,
        session_identifier,
        payment_ika,
        payment_sui,
        ctx,
    )
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_request_imported_key_sign_with_partial_user_signature"></a>

## Function `request_imported_key_sign_with_partial_user_signature`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_imported_key_sign_with_partial_user_signature">request_imported_key_sign_with_partial_user_signature</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, partial_user_signature_cap: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_VerifiedPartialUserSignatureCap">coordinator_inner::VerifiedPartialUserSignatureCap</a>, message_approval: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_ImportedKeyMessageApproval">coordinator_inner::ImportedKeyMessageApproval</a>, session_identifier: (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_SessionIdentifier">coordinator_inner::SessionIdentifier</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_imported_key_sign_with_partial_user_signature">request_imported_key_sign_with_partial_user_signature</a>(
    self: &<b>mut</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    partial_user_signature_cap: VerifiedPartialUserSignatureCap,
    message_approval: ImportedKeyMessageApproval,
    session_identifier: SessionIdentifier,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
) {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_request_imported_key_sign_with_partial_user_signature">request_imported_key_sign_with_partial_user_signature</a>(
        partial_user_signature_cap,
        message_approval,
        session_identifier,
        payment_ika,
        payment_sui,
        ctx,
    )
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_match_partial_user_signature_with_message_approval"></a>

## Function `match_partial_user_signature_with_message_approval`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_match_partial_user_signature_with_message_approval">match_partial_user_signature_with_message_approval</a>(self: &(ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, partial_user_signature_cap: &(ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_VerifiedPartialUserSignatureCap">coordinator_inner::VerifiedPartialUserSignatureCap</a>, message_approval: &(ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_MessageApproval">coordinator_inner::MessageApproval</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_match_partial_user_signature_with_message_approval">match_partial_user_signature_with_message_approval</a>(
    self: &<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    partial_user_signature_cap: &VerifiedPartialUserSignatureCap,
    message_approval: &MessageApproval,
): bool {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner">inner</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_match_partial_user_signature_with_message_approval">match_partial_user_signature_with_message_approval</a>(
        partial_user_signature_cap,
        message_approval,
    )
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_match_partial_user_signature_with_imported_key_message_approval"></a>

## Function `match_partial_user_signature_with_imported_key_message_approval`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_match_partial_user_signature_with_imported_key_message_approval">match_partial_user_signature_with_imported_key_message_approval</a>(self: &(ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, partial_user_signature_cap: &(ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_VerifiedPartialUserSignatureCap">coordinator_inner::VerifiedPartialUserSignatureCap</a>, message_approval: &(ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_ImportedKeyMessageApproval">coordinator_inner::ImportedKeyMessageApproval</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_match_partial_user_signature_with_imported_key_message_approval">match_partial_user_signature_with_imported_key_message_approval</a>(
    self: &<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    partial_user_signature_cap: &VerifiedPartialUserSignatureCap,
    message_approval: &ImportedKeyMessageApproval,
): bool {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner">inner</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_match_partial_user_signature_with_imported_key_message_approval">match_partial_user_signature_with_imported_key_message_approval</a>(
        partial_user_signature_cap,
        message_approval,
    )
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_current_pricing"></a>

## Function `current_pricing`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_current_pricing">current_pricing</a>(self: &(ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>): (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_dwallet_2pc_mpc=0x0)_dwallet_pricing_DWalletPricing">dwallet_pricing::DWalletPricing</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_current_pricing">current_pricing</a>(self: &<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>): DWalletPricing {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner">inner</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_current_pricing">current_pricing</a>()
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_subsidize_coordinator_with_sui"></a>

## Function `subsidize_coordinator_with_sui`

Fund the coordinator with SUI - this let you subsidize the protocol.
IMPORTANT: YOU WON'T BE ABLE TO WITHDRAW THE FUNDS OR GET ANYTHING IN RETURN.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_subsidize_coordinator_with_sui">subsidize_coordinator_with_sui</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, sui: <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_subsidize_coordinator_with_sui">subsidize_coordinator_with_sui</a>(
    self: &<b>mut</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    sui: Coin&lt;SUI&gt;,
) {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_subsidize_coordinator_with_sui">subsidize_coordinator_with_sui</a>(sui);
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_subsidize_coordinator_with_ika"></a>

## Function `subsidize_coordinator_with_ika`

Fund the coordinator with IKA - this let you subsidize the protocol.
IMPORTANT: YOU WON'T BE ABLE TO WITHDRAW THE FUNDS OR GET ANYTHING IN RETURN.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_subsidize_coordinator_with_ika">subsidize_coordinator_with_ika</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>, ika: <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_subsidize_coordinator_with_ika">subsidize_coordinator_with_ika</a>(
    self: &<b>mut</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
    ika: Coin&lt;IKA&gt;,
) {
    self.<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_mut">inner_mut</a>().<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_subsidize_coordinator_with_ika">subsidize_coordinator_with_ika</a>(ika);
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_migrate"></a>

## Function `migrate`

Migrate the dwallet_2pc_mpc_coordinator object to the new package id.

This function sets the new package id and version and can be modified in future versions
to migrate changes in the <code>dwallet_2pc_mpc_coordinator_inner</code> object if needed.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_migrate">migrate</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_migrate">migrate</a>(
        self: &<b>mut</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>,
) {
    <b>assert</b>!(self.version &lt; <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_VERSION">VERSION</a>, <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_EInvalidMigration">EInvalidMigration</a>);
    // Move the old system state <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner">inner</a> to the new version.
    <b>let</b> dwallet_2pc_mpc_coordinator_inner: DWalletCoordinatorInner = dynamic_field::remove(&<b>mut</b> self.id, self.version);
    dynamic_field::add(&<b>mut</b> self.id, <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_VERSION">VERSION</a>, dwallet_2pc_mpc_coordinator_inner);
    self.version = <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_VERSION">VERSION</a>;
    // Set the new package id.
    <b>assert</b>!(self.new_package_id.is_some(), <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_EInvalidMigration">EInvalidMigration</a>);
    self.package_id = self.new_package_id.extract();
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_mut"></a>

## Function `inner_mut`

Get a mutable reference to <code>DWalletCoordinatorInnerVX</code> from the <code><a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a></code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_mut">inner_mut</a>(self: &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>): &<b>mut</b> (ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_DWalletCoordinatorInner">coordinator_inner::DWalletCoordinatorInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_mut">inner_mut</a>(self: &<b>mut</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>): &<b>mut</b> DWalletCoordinatorInner {
    <b>assert</b>!(self.version == <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_VERSION">VERSION</a>, <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_EWrongInnerVersion">EWrongInnerVersion</a>);
    dynamic_field::borrow_mut(&<b>mut</b> self.id, <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_VERSION">VERSION</a>)
}
</code></pre>



</details>

<a name="(ika_dwallet_2pc_mpc=0x0)_coordinator_inner"></a>

## Function `inner`

Get an immutable reference to <code>DWalletCoordinatorVX</code> from the <code><a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a></code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner">inner</a>(self: &(ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">coordinator::DWalletCoordinator</a>): &(ika_dwallet_2pc_mpc=0x0)::<a href="../ika_system/coordinator_inner.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner_DWalletCoordinatorInner">coordinator_inner::DWalletCoordinatorInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_inner">inner</a>(self: &<a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_DWalletCoordinator">DWalletCoordinator</a>): &DWalletCoordinatorInner {
    <b>assert</b>!(self.version == <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_VERSION">VERSION</a>, <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_EWrongInnerVersion">EWrongInnerVersion</a>);
    dynamic_field::borrow(&self.id, <a href="../ika_system/coordinator.md#(ika_dwallet_2pc_mpc=0x0)_coordinator_VERSION">VERSION</a>)
}
</code></pre>



</details>
