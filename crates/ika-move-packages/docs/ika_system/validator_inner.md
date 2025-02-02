---
title: Module `0x0::validator_inner_v1`
---



-  [Struct `ValidatorMetadata`](#0x0_validator_inner_v1_ValidatorMetadata)
-  [Struct `ValidatorInnerV1`](#0x0_validator_inner_v1_ValidatorInnerV1)
-  [Struct `StakingRequestEvent`](#0x0_validator_inner_v1_StakingRequestEvent)
-  [Struct `UnstakingRequestEvent`](#0x0_validator_inner_v1_UnstakingRequestEvent)
-  [Struct `ConvertingToFungibleStakedIkaEvent`](#0x0_validator_inner_v1_ConvertingToFungibleStakedIkaEvent)
-  [Struct `RedeemingFungibleStakedIkaEvent`](#0x0_validator_inner_v1_RedeemingFungibleStakedIkaEvent)
-  [Constants](#@Constants_0)
-  [Function `create_metadata`](#0x0_validator_inner_v1_create_metadata)
-  [Function `create`](#0x0_validator_inner_v1_create)
-  [Function `deactivate`](#0x0_validator_inner_v1_deactivate)
-  [Function `activate`](#0x0_validator_inner_v1_activate)
-  [Function `adjust_stake_and_computation_price`](#0x0_validator_inner_v1_adjust_stake_and_computation_price)
-  [Function `request_add_stake`](#0x0_validator_inner_v1_request_add_stake)
-  [Function `convert_to_fungible_staked_ika`](#0x0_validator_inner_v1_convert_to_fungible_staked_ika)
-  [Function `redeem_fungible_staked_ika`](#0x0_validator_inner_v1_redeem_fungible_staked_ika)
-  [Function `request_withdraw_stake`](#0x0_validator_inner_v1_request_withdraw_stake)
-  [Function `request_set_computation_price`](#0x0_validator_inner_v1_request_set_computation_price)
-  [Function `set_candidate_computation_price`](#0x0_validator_inner_v1_set_candidate_computation_price)
-  [Function `request_set_commission_rate`](#0x0_validator_inner_v1_request_set_commission_rate)
-  [Function `set_candidate_commission_rate`](#0x0_validator_inner_v1_set_candidate_commission_rate)
-  [Function `deposit_stake_rewards`](#0x0_validator_inner_v1_deposit_stake_rewards)
-  [Function `process_pending_stakes_and_withdraws`](#0x0_validator_inner_v1_process_pending_stakes_and_withdraws)
-  [Function `is_candidate`](#0x0_validator_inner_v1_is_candidate)
-  [Function `is_inactive`](#0x0_validator_inner_v1_is_inactive)
-  [Function `validator_id`](#0x0_validator_inner_v1_validator_id)
-  [Function `metadata`](#0x0_validator_inner_v1_metadata)
-  [Function `payment_address`](#0x0_validator_inner_v1_payment_address)
-  [Function `name`](#0x0_validator_inner_v1_name)
-  [Function `description`](#0x0_validator_inner_v1_description)
-  [Function `image_url`](#0x0_validator_inner_v1_image_url)
-  [Function `project_url`](#0x0_validator_inner_v1_project_url)
-  [Function `network_address`](#0x0_validator_inner_v1_network_address)
-  [Function `p2p_address`](#0x0_validator_inner_v1_p2p_address)
-  [Function `consensus_address`](#0x0_validator_inner_v1_consensus_address)
-  [Function `protocol_pubkey_bytes`](#0x0_validator_inner_v1_protocol_pubkey_bytes)
-  [Function `protocol_pubkey`](#0x0_validator_inner_v1_protocol_pubkey)
-  [Function `proof_of_possession_bytes`](#0x0_validator_inner_v1_proof_of_possession_bytes)
-  [Function `network_pubkey_bytes`](#0x0_validator_inner_v1_network_pubkey_bytes)
-  [Function `consensus_pubkey_bytes`](#0x0_validator_inner_v1_consensus_pubkey_bytes)
-  [Function `next_epoch_network_address`](#0x0_validator_inner_v1_next_epoch_network_address)
-  [Function `next_epoch_p2p_address`](#0x0_validator_inner_v1_next_epoch_p2p_address)
-  [Function `next_epoch_consensus_address`](#0x0_validator_inner_v1_next_epoch_consensus_address)
-  [Function `next_epoch_protocol_pubkey_bytes`](#0x0_validator_inner_v1_next_epoch_protocol_pubkey_bytes)
-  [Function `next_epoch_proof_of_possession_bytes`](#0x0_validator_inner_v1_next_epoch_proof_of_possession_bytes)
-  [Function `next_epoch_network_pubkey_bytes`](#0x0_validator_inner_v1_next_epoch_network_pubkey_bytes)
-  [Function `next_epoch_consensus_pubkey_bytes`](#0x0_validator_inner_v1_next_epoch_consensus_pubkey_bytes)
-  [Function `operation_cap_id`](#0x0_validator_inner_v1_operation_cap_id)
-  [Function `next_epoch_computation_price`](#0x0_validator_inner_v1_next_epoch_computation_price)
-  [Function `total_stake_amount`](#0x0_validator_inner_v1_total_stake_amount)
-  [Function `pending_stake_amount`](#0x0_validator_inner_v1_pending_stake_amount)
-  [Function `pending_stake_withdraw_amount`](#0x0_validator_inner_v1_pending_stake_withdraw_amount)
-  [Function `computation_price`](#0x0_validator_inner_v1_computation_price)
-  [Function `commission_rate`](#0x0_validator_inner_v1_commission_rate)
-  [Function `pool_token_exchange_rate_at_epoch`](#0x0_validator_inner_v1_pool_token_exchange_rate_at_epoch)
-  [Function `is_duplicate`](#0x0_validator_inner_v1_is_duplicate)
-  [Function `is_equal_some_and_value`](#0x0_validator_inner_v1_is_equal_some_and_value)
-  [Function `is_equal_some`](#0x0_validator_inner_v1_is_equal_some)
-  [Function `new_validator_operation_cap`](#0x0_validator_inner_v1_new_validator_operation_cap)
-  [Function `update_payment_address`](#0x0_validator_inner_v1_update_payment_address)
-  [Function `update_name`](#0x0_validator_inner_v1_update_name)
-  [Function `update_description`](#0x0_validator_inner_v1_update_description)
-  [Function `update_image_url`](#0x0_validator_inner_v1_update_image_url)
-  [Function `update_project_url`](#0x0_validator_inner_v1_update_project_url)
-  [Function `update_next_epoch_network_address`](#0x0_validator_inner_v1_update_next_epoch_network_address)
-  [Function `update_candidate_network_address`](#0x0_validator_inner_v1_update_candidate_network_address)
-  [Function `update_next_epoch_p2p_address`](#0x0_validator_inner_v1_update_next_epoch_p2p_address)
-  [Function `update_candidate_p2p_address`](#0x0_validator_inner_v1_update_candidate_p2p_address)
-  [Function `update_next_epoch_consensus_address`](#0x0_validator_inner_v1_update_next_epoch_consensus_address)
-  [Function `update_candidate_consensus_address`](#0x0_validator_inner_v1_update_candidate_consensus_address)
-  [Function `update_next_epoch_protocol_pubkey_bytes`](#0x0_validator_inner_v1_update_next_epoch_protocol_pubkey_bytes)
-  [Function `update_candidate_protocol_pubkey_bytes`](#0x0_validator_inner_v1_update_candidate_protocol_pubkey_bytes)
-  [Function `update_next_epoch_network_pubkey_bytes`](#0x0_validator_inner_v1_update_next_epoch_network_pubkey_bytes)
-  [Function `update_candidate_network_pubkey_bytes`](#0x0_validator_inner_v1_update_candidate_network_pubkey_bytes)
-  [Function `update_next_epoch_consensus_pubkey_bytes`](#0x0_validator_inner_v1_update_next_epoch_consensus_pubkey_bytes)
-  [Function `update_candidate_consensus_pubkey_bytes`](#0x0_validator_inner_v1_update_candidate_consensus_pubkey_bytes)
-  [Function `effectuate_staged_metadata`](#0x0_validator_inner_v1_effectuate_staged_metadata)
-  [Function `verify_proof_of_possession`](#0x0_validator_inner_v1_verify_proof_of_possession)
-  [Function `validate_metadata`](#0x0_validator_inner_v1_validate_metadata)
-  [Function `get_staking_pool_ref`](#0x0_validator_inner_v1_get_staking_pool_ref)
-  [Function `create_from_metadata`](#0x0_validator_inner_v1_create_from_metadata)


<pre><code><b>use</b> <a href="../ika/ika.md#0x0_ika">0x0::ika</a>;
<b>use</b> <a href="staked_ika.md#0x0_staked_ika">0x0::staked_ika</a>;
<b>use</b> <a href="staking_pool.md#0x0_staking_pool">0x0::staking_pool</a>;
<b>use</b> <a href="validator_cap.md#0x0_validator_cap">0x0::validator_cap</a>;
<b>use</b> <a href="../move-stdlib/ascii.md#0x1_ascii">0x1::ascii</a>;
<b>use</b> <a href="../move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../move-stdlib/string.md#0x1_string">0x1::string</a>;
<b>use</b> <a href="../move-stdlib/vector.md#0x1_vector">0x1::vector</a>;
<b>use</b> <a href="../sui-framework/address.md#0x2_address">0x2::address</a>;
<b>use</b> <a href="../sui-framework/bag.md#0x2_bag">0x2::bag</a>;
<b>use</b> <a href="../sui-framework/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../sui-framework/bcs.md#0x2_bcs">0x2::bcs</a>;
<b>use</b> <a href="../sui-framework/bls12381.md#0x2_bls12381">0x2::bls12381</a>;
<b>use</b> <a href="../sui-framework/event.md#0x2_event">0x2::event</a>;
<b>use</b> <a href="../sui-framework/group_ops.md#0x2_group_ops">0x2::group_ops</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../sui-framework/url.md#0x2_url">0x2::url</a>;
</code></pre>



<a name="0x0_validator_inner_v1_ValidatorMetadata"></a>

## Struct `ValidatorMetadata`



<pre><code><b>struct</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorMetadata">ValidatorMetadata</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>payment_address: <b>address</b></code>
</dt>
<dd>
 The address to receive the payments
</dd>
<dt>
<code>proof_of_possession_sender: <b>address</b></code>
</dt>
<dd>
 The address of the proof of possesion sender
</dd>
<dt>
<code>protocol_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The public key bytes corresponding to the private key that the validator
 holds to sign checkpoint messages.
</dd>
<dt>
<code>protocol_pubkey: <a href="../sui-framework/group_ops.md#0x2_group_ops_Element">group_ops::Element</a>&lt;<a href="../sui-framework/bls12381.md#0x2_bls12381_UncompressedG1">bls12381::UncompressedG1</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>proof_of_possession_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 This is a proof that the validator has ownership of the protocol private key
</dd>
<dt>
<code>network_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The public key bytes corresponding to the private key that the validator
 uses to establish TLS connections
</dd>
<dt>
<code>consensus_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The public key bytes correstponding to the consensus
</dd>
<dt>
<code>name: <a href="../move-stdlib/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>
 A unique human-readable name of this validator.
</dd>
<dt>
<code>description: <a href="../move-stdlib/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>image_url: <a href="../sui-framework/url.md#0x2_url_Url">url::Url</a></code>
</dt>
<dd>

</dd>
<dt>
<code>project_url: <a href="../sui-framework/url.md#0x2_url_Url">url::Url</a></code>
</dt>
<dd>

</dd>
<dt>
<code>network_address: <a href="../move-stdlib/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>
 The network address of the validator (could also contain extra info such as port, DNS and etc.).
</dd>
<dt>
<code>p2p_address: <a href="../move-stdlib/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>
 The address of the validator used for p2p activities such as state sync (could also contain extra info such as port, DNS and etc.).
</dd>
<dt>
<code>consensus_address: <a href="../move-stdlib/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>
 The address of the consensus
</dd>
<dt>
<code>next_epoch_protocol_pubkey_bytes: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>
 "next_epoch" metadata only takes effects in the next epoch.
 If none, current value will stay unchanged.
</dd>
<dt>
<code>next_epoch_proof_of_possession_bytes: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>next_epoch_network_pubkey_bytes: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>next_epoch_consensus_pubkey_bytes: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>next_epoch_network_address: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/string.md#0x1_string_String">string::String</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>next_epoch_p2p_address: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/string.md#0x1_string_String">string::String</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>next_epoch_consensus_address: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/string.md#0x1_string_String">string::String</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>extra_fields: <a href="../sui-framework/bag.md#0x2_bag_Bag">bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="0x0_validator_inner_v1_ValidatorInnerV1"></a>

## Struct `ValidatorInnerV1`



<pre><code><b>struct</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>metadata: <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorMetadata">validator_inner_v1::ValidatorMetadata</a></code>
</dt>
<dd>
 Summary of the validator.
</dd>
<dt>
<code>cap_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The ID of this validator's <code>ValidatorCap</code>
</dd>
<dt>
<code>operation_cap_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The ID of this validator's current valid <code>UnverifiedValidatorOperationCap</code>
</dd>
<dt>
<code>computation_price: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 Gas price quote, updated only at end of epoch.
</dd>
<dt>
<code><a href="staking_pool.md#0x0_staking_pool">staking_pool</a>: <a href="staking_pool.md#0x0_staking_pool_StakingPool">staking_pool::StakingPool</a></code>
</dt>
<dd>
 Staking pool for this validator.
</dd>
<dt>
<code>commission_rate: u16</code>
</dt>
<dd>
 Commission rate of the validator, in basis point.
</dd>
<dt>
<code>next_epoch_stake: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 Total amount of stake that would be active in the next epoch.
</dd>
<dt>
<code>next_epoch_computation_price: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 This validator's computation price quote for the next epoch.
</dd>
<dt>
<code>next_epoch_commission_rate: u16</code>
</dt>
<dd>
 The commission rate of the validator starting the next epoch, in basis point.
</dd>
<dt>
<code>extra_fields: <a href="../sui-framework/bag.md#0x2_bag_Bag">bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="0x0_validator_inner_v1_StakingRequestEvent"></a>

## Struct `StakingRequestEvent`

Event emitted when a new stake request is received.


<pre><code><b>struct</b> <a href="validator_inner.md#0x0_validator_inner_v1_StakingRequestEvent">StakingRequestEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>staked_ika_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x0_validator_inner_v1_UnstakingRequestEvent"></a>

## Struct `UnstakingRequestEvent`

Event emitted when a new unstake request is received.


<pre><code><b>struct</b> <a href="validator_inner.md#0x0_validator_inner_v1_UnstakingRequestEvent">UnstakingRequestEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>staked_ika_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>stake_activation_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>unstaking_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>principal_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>reward_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x0_validator_inner_v1_ConvertingToFungibleStakedIkaEvent"></a>

## Struct `ConvertingToFungibleStakedIkaEvent`

Event emitted when a staked IKA is converted to a fungible staked IKA.


<pre><code><b>struct</b> <a href="validator_inner.md#0x0_validator_inner_v1_ConvertingToFungibleStakedIkaEvent">ConvertingToFungibleStakedIkaEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>stake_activation_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>staked_ika_principal_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>fungible_staked_ika_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x0_validator_inner_v1_RedeemingFungibleStakedIkaEvent"></a>

## Struct `RedeemingFungibleStakedIkaEvent`

Event emitted when a fungible staked IKA is redeemed.


<pre><code><b>struct</b> <a href="validator_inner.md#0x0_validator_inner_v1_RedeemingFungibleStakedIkaEvent">RedeemingFungibleStakedIkaEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>fungible_staked_ika_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>ika_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x0_validator_inner_v1_BLS_KEY_LEN"></a>



<pre><code><b>const</b> <a href="validator_inner.md#0x0_validator_inner_v1_BLS_KEY_LEN">BLS_KEY_LEN</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 48;
</code></pre>



<a name="0x0_validator_inner_v1_DEFAULT_EPOCH_ID"></a>



<pre><code><b>const</b> <a href="validator_inner.md#0x0_validator_inner_v1_DEFAULT_EPOCH_ID">DEFAULT_EPOCH_ID</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
</code></pre>



<a name="0x0_validator_inner_v1_ECommissionRateTooHigh"></a>



<pre><code>#[error]
<b>const</b> <a href="validator_inner.md#0x0_validator_inner_v1_ECommissionRateTooHigh">ECommissionRateTooHigh</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = b"Commission rate set by the <a href="validator.md#0x0_validator">validator</a> is higher than the threshold.";
</code></pre>



<a name="0x0_validator_inner_v1_ED25519_KEY_LEN"></a>



<pre><code><b>const</b> <a href="validator_inner.md#0x0_validator_inner_v1_ED25519_KEY_LEN">ED25519_KEY_LEN</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 32;
</code></pre>



<a name="0x0_validator_inner_v1_EGasPriceHigherThanThreshold"></a>



<pre><code>#[error]
<b>const</b> <a href="validator_inner.md#0x0_validator_inner_v1_EGasPriceHigherThanThreshold">EGasPriceHigherThanThreshold</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = b"Validator trying <b>to</b> set computation price higher than threshold.";
</code></pre>



<a name="0x0_validator_inner_v1_EInactiveValidator"></a>



<pre><code>#[error]
<b>const</b> <a href="validator_inner.md#0x0_validator_inner_v1_EInactiveValidator">EInactiveValidator</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = b"The <a href="validator.md#0x0_validator">validator</a> is inactive.";
</code></pre>



<a name="0x0_validator_inner_v1_EInvalidCap"></a>



<pre><code>#[error]
<b>const</b> <a href="validator_inner.md#0x0_validator_inner_v1_EInvalidCap">EInvalidCap</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = b"Cap is not valid.";
</code></pre>



<a name="0x0_validator_inner_v1_EInvalidProofOfPossession"></a>



<pre><code>#[error]
<b>const</b> <a href="validator_inner.md#0x0_validator_inner_v1_EInvalidProofOfPossession">EInvalidProofOfPossession</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = b"Invalid proof_of_possession_bytes field in <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorMetadata">ValidatorMetadata</a>.";
</code></pre>



<a name="0x0_validator_inner_v1_EInvalidStakeAmount"></a>



<pre><code>#[error]
<b>const</b> <a href="validator_inner.md#0x0_validator_inner_v1_EInvalidStakeAmount">EInvalidStakeAmount</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = b"Stake amount is invalid or wrong.";
</code></pre>



<a name="0x0_validator_inner_v1_EMetadataInvalidConsensusAddress"></a>



<pre><code>#[error]
<b>const</b> <a href="validator_inner.md#0x0_validator_inner_v1_EMetadataInvalidConsensusAddress">EMetadataInvalidConsensusAddress</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = b"Invalid consensus_address field in <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorMetadata">ValidatorMetadata</a>.";
</code></pre>



<a name="0x0_validator_inner_v1_EMetadataInvalidConsensusPubkey"></a>



<pre><code>#[error]
<b>const</b> <a href="validator_inner.md#0x0_validator_inner_v1_EMetadataInvalidConsensusPubkey">EMetadataInvalidConsensusPubkey</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = b"Invalid consensus_pubkey_bytes field in <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorMetadata">ValidatorMetadata</a>.";
</code></pre>



<a name="0x0_validator_inner_v1_EMetadataInvalidNetworkAddress"></a>



<pre><code>#[error]
<b>const</b> <a href="validator_inner.md#0x0_validator_inner_v1_EMetadataInvalidNetworkAddress">EMetadataInvalidNetworkAddress</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = b"Invalid network_address field in <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorMetadata">ValidatorMetadata</a>.";
</code></pre>



<a name="0x0_validator_inner_v1_EMetadataInvalidNetworkPubkey"></a>



<pre><code>#[error]
<b>const</b> <a href="validator_inner.md#0x0_validator_inner_v1_EMetadataInvalidNetworkPubkey">EMetadataInvalidNetworkPubkey</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = b"Invalid network_pubkey_bytes field in <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorMetadata">ValidatorMetadata</a>.";
</code></pre>



<a name="0x0_validator_inner_v1_EMetadataInvalidP2pAddress"></a>



<pre><code>#[error]
<b>const</b> <a href="validator_inner.md#0x0_validator_inner_v1_EMetadataInvalidP2pAddress">EMetadataInvalidP2pAddress</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = b"Invalid p2p_address field in <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorMetadata">ValidatorMetadata</a>.";
</code></pre>



<a name="0x0_validator_inner_v1_EMetadataInvalidProtocolPubkey"></a>



<pre><code>#[error]
<b>const</b> <a href="validator_inner.md#0x0_validator_inner_v1_EMetadataInvalidProtocolPubkey">EMetadataInvalidProtocolPubkey</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = b"Invalid protocol_pubkey_bytes field in <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorMetadata">ValidatorMetadata</a>.";
</code></pre>



<a name="0x0_validator_inner_v1_ENewCapNotCreatedByValidatorItself"></a>



<pre><code>#[error]
<b>const</b> <a href="validator_inner.md#0x0_validator_inner_v1_ENewCapNotCreatedByValidatorItself">ENewCapNotCreatedByValidatorItself</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = b"New Capability is not created by the <a href="validator.md#0x0_validator">validator</a> itself.";
</code></pre>



<a name="0x0_validator_inner_v1_ENotValidatorCandidate"></a>



<pre><code>#[error]
<b>const</b> <a href="validator_inner.md#0x0_validator_inner_v1_ENotValidatorCandidate">ENotValidatorCandidate</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = b"Intended <a href="validator.md#0x0_validator">validator</a> is not a candidate one.";
</code></pre>



<a name="0x0_validator_inner_v1_EValidatorMetadataExceedingLengthLimit"></a>



<pre><code>#[error]
<b>const</b> <a href="validator_inner.md#0x0_validator_inner_v1_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = b"Validator Metadata is too long.";
</code></pre>



<a name="0x0_validator_inner_v1_MAX_COMMISSION_RATE"></a>



<pre><code><b>const</b> <a href="validator_inner.md#0x0_validator_inner_v1_MAX_COMMISSION_RATE">MAX_COMMISSION_RATE</a>: u16 = 2000;
</code></pre>



<a name="0x0_validator_inner_v1_MAX_VALIDATOR_COMPUTATION_PRICE"></a>

Max computation price a validator can set is 100K NIKA.


<pre><code><b>const</b> <a href="validator_inner.md#0x0_validator_inner_v1_MAX_VALIDATOR_COMPUTATION_PRICE">MAX_VALIDATOR_COMPUTATION_PRICE</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 100000;
</code></pre>



<a name="0x0_validator_inner_v1_MAX_VALIDATOR_METADATA_LENGTH"></a>



<pre><code><b>const</b> <a href="validator_inner.md#0x0_validator_inner_v1_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 256;
</code></pre>



<a name="0x0_validator_inner_v1_PROOF_OF_POSSESSION_INTENT"></a>



<pre><code><b>const</b> <a href="validator_inner.md#0x0_validator_inner_v1_PROOF_OF_POSSESSION_INTENT">PROOF_OF_POSSESSION_INTENT</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [0, 0, 0];
</code></pre>



<a name="0x0_validator_inner_v1_create_metadata"></a>

## Function `create_metadata`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_create_metadata">create_metadata</a>(payment_address: <b>address</b>, protocol_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, network_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, consensus_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, proof_of_possession_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, name: <a href="../move-stdlib/string.md#0x1_string_String">string::String</a>, description: <a href="../move-stdlib/string.md#0x1_string_String">string::String</a>, image_url: <a href="../sui-framework/url.md#0x2_url_Url">url::Url</a>, project_url: <a href="../sui-framework/url.md#0x2_url_Url">url::Url</a>, network_address: <a href="../move-stdlib/string.md#0x1_string_String">string::String</a>, p2p_address: <a href="../move-stdlib/string.md#0x1_string_String">string::String</a>, consensus_address: <a href="../move-stdlib/string.md#0x1_string_String">string::String</a>, extra_fields: <a href="../sui-framework/bag.md#0x2_bag_Bag">bag::Bag</a>, ctx: &<a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorMetadata">validator_inner_v1::ValidatorMetadata</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_create_metadata">create_metadata</a>(
    payment_address: <b>address</b>,
    protocol_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    network_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    consensus_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    proof_of_possession_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    name: String,
    description: String,
    image_url: Url,
    project_url: Url,
    network_address: String,
    p2p_address: String,
    consensus_address: String,
    extra_fields: Bag,
    ctx: &TxContext,
): <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorMetadata">ValidatorMetadata</a> {
    <b>let</b> protocol_pubkey = g1_to_uncompressed_g1(&g1_from_bytes(&protocol_pubkey_bytes));
    <b>let</b> metadata = <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorMetadata">ValidatorMetadata</a> {
        payment_address,
        proof_of_possession_sender: ctx.sender(),
        protocol_pubkey_bytes,
        protocol_pubkey,
        network_pubkey_bytes,
        consensus_pubkey_bytes,
        proof_of_possession_bytes,
        name,
        description,
        image_url,
        project_url,
        network_address,
        p2p_address,
        consensus_address,
        next_epoch_protocol_pubkey_bytes: <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(),
        next_epoch_network_pubkey_bytes: <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(),
        next_epoch_consensus_pubkey_bytes: <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(),
        next_epoch_proof_of_possession_bytes: <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(),
        next_epoch_network_address: <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(),
        next_epoch_p2p_address: <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(),
        next_epoch_consensus_address: <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(),
        extra_fields,
    };
    metadata
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_create"></a>

## Function `create`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_create">create</a>(validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, cap_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, operation_cap_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, payment_address: <b>address</b>, protocol_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, network_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, consensus_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, proof_of_possession_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, name: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, description: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, image_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, project_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, network_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, p2p_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, consensus_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, computation_price: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, commission_rate: u16, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_create">create</a>(
    validator_id: ID,
    cap_id: ID,
    operation_cap_id: ID,
    payment_address: <b>address</b>,
    protocol_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    network_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    consensus_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    proof_of_possession_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    name: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    description: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    image_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    project_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    network_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    p2p_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    consensus_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    computation_price: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    commission_rate: u16,
    ctx: &<b>mut</b> TxContext,
): <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a> {
    <b>assert</b>!(<a href="validator_inner.md#0x0_validator_inner_v1_commission_rate">commission_rate</a> &lt;= <a href="validator_inner.md#0x0_validator_inner_v1_MAX_COMMISSION_RATE">MAX_COMMISSION_RATE</a>, <a href="validator_inner.md#0x0_validator_inner_v1_ECommissionRateTooHigh">ECommissionRateTooHigh</a>);
    <b>assert</b>!(<a href="validator_inner.md#0x0_validator_inner_v1_computation_price">computation_price</a> &lt; <a href="validator_inner.md#0x0_validator_inner_v1_MAX_VALIDATOR_COMPUTATION_PRICE">MAX_VALIDATOR_COMPUTATION_PRICE</a>, <a href="validator_inner.md#0x0_validator_inner_v1_EGasPriceHigherThanThreshold">EGasPriceHigherThanThreshold</a>);

    <b>let</b> metadata = <a href="validator_inner.md#0x0_validator_inner_v1_create_metadata">create_metadata</a>(
        payment_address,
        protocol_pubkey_bytes,
        network_pubkey_bytes,
        consensus_pubkey_bytes,
        proof_of_possession_bytes,
        name.to_ascii_string().to_string(),
        description.to_ascii_string().to_string(),
        <a href="../sui-framework/url.md#0x2_url_new_unsafe_from_bytes">url::new_unsafe_from_bytes</a>(image_url),
        <a href="../sui-framework/url.md#0x2_url_new_unsafe_from_bytes">url::new_unsafe_from_bytes</a>(project_url),
        network_address.to_ascii_string().to_string(),
        p2p_address.to_ascii_string().to_string(),
        consensus_address.to_ascii_string().to_string(),
        <a href="../sui-framework/bag.md#0x2_bag_new">bag::new</a>(ctx),
        ctx,
    );

    // Checks that the keys & addresses & PoP are valid.
    <a href="validator_inner.md#0x0_validator_inner_v1_validate_metadata">validate_metadata</a>(&metadata);

    <a href="validator_inner.md#0x0_validator_inner_v1_create_from_metadata">create_from_metadata</a>(
        validator_id,
        cap_id,
        operation_cap_id,
        metadata,
        computation_price,
        commission_rate,
        ctx,
    )
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_deactivate"></a>

## Function `deactivate`

Deactivate this validator's staking pool


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_deactivate">deactivate</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, deactivation_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_deactivate">deactivate</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>, deactivation_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>) {
    self.<a href="staking_pool.md#0x0_staking_pool">staking_pool</a>.deactivate_staking_pool(deactivation_epoch)
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_activate"></a>

## Function `activate`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_activate">activate</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, activation_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_activate">activate</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>, activation_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>) {
    self.<a href="staking_pool.md#0x0_staking_pool">staking_pool</a>.activate_staking_pool(activation_epoch);
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_adjust_stake_and_computation_price"></a>

## Function `adjust_stake_and_computation_price`

Process pending stake and pending withdraws, and update the computation price.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_adjust_stake_and_computation_price">adjust_stake_and_computation_price</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_adjust_stake_and_computation_price">adjust_stake_and_computation_price</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>) {
    self.computation_price = self.next_epoch_computation_price;
    self.commission_rate = self.next_epoch_commission_rate;
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_request_add_stake"></a>

## Function `request_add_stake`

Request to add stake to the validator's staking pool, processed at the end of the epoch.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_request_add_stake">request_add_stake</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, stake: <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="staked_ika.md#0x0_staked_ika_StakedIka">staked_ika::StakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_request_add_stake">request_add_stake</a>(
    self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    stake: Balance&lt;IKA&gt;,
    ctx: &<b>mut</b> TxContext,
): StakedIka {
    <b>let</b> stake_amount = stake.value();
    <b>let</b> validator_id = self.<a href="validator_inner.md#0x0_validator_inner_v1_validator_id">validator_id</a>();
    <b>assert</b>!(stake_amount &gt; 0, <a href="validator_inner.md#0x0_validator_inner_v1_EInvalidStakeAmount">EInvalidStakeAmount</a>);
    <b>let</b> stake_epoch = epoch + 1;
    <b>let</b> <a href="staked_ika.md#0x0_staked_ika">staked_ika</a> = self.<a href="staking_pool.md#0x0_staking_pool">staking_pool</a>.<a href="validator_inner.md#0x0_validator_inner_v1_request_add_stake">request_add_stake</a>(stake, stake_epoch, validator_id, ctx);
    // Process stake right away <b>if</b> staking pool is preactive.
    <b>if</b> (self.<a href="staking_pool.md#0x0_staking_pool">staking_pool</a>.<a href="validator_inner.md#0x0_validator_inner_v1_is_candidate">is_candidate</a>()) {
        self.<a href="staking_pool.md#0x0_staking_pool">staking_pool</a>.process_pending_stake();
    };
    self.next_epoch_stake = self.next_epoch_stake + stake_amount;
    <a href="../sui-framework/event.md#0x2_event_emit">event::emit</a>(<a href="validator_inner.md#0x0_validator_inner_v1_StakingRequestEvent">StakingRequestEvent</a> {
        validator_id,
        staked_ika_id: <a href="../sui-framework/object.md#0x2_object_id">object::id</a>(&<a href="staked_ika.md#0x0_staked_ika">staked_ika</a>),
        epoch: epoch,
        amount: stake_amount,
    });
    <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_convert_to_fungible_staked_ika"></a>

## Function `convert_to_fungible_staked_ika`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_convert_to_fungible_staked_ika">convert_to_fungible_staked_ika</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: <a href="staked_ika.md#0x0_staked_ika_StakedIka">staked_ika::StakedIka</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_convert_to_fungible_staked_ika">convert_to_fungible_staked_ika</a>(
    self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: StakedIka,
    ctx: &<b>mut</b> TxContext,
): FungibleStakedIka {
    <b>let</b> stake_activation_epoch = <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>.stake_activation_epoch();
    <b>let</b> staked_ika_principal_amount = <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>.staked_ika_amount();

    <b>let</b> fungible_staked_ika = self.<a href="staking_pool.md#0x0_staking_pool">staking_pool</a>.<a href="validator_inner.md#0x0_validator_inner_v1_convert_to_fungible_staked_ika">convert_to_fungible_staked_ika</a>(epoch, <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>, ctx);

    <a href="../sui-framework/event.md#0x2_event_emit">event::emit</a>(<a href="validator_inner.md#0x0_validator_inner_v1_ConvertingToFungibleStakedIkaEvent">ConvertingToFungibleStakedIkaEvent</a> {
        validator_id: self.<a href="validator_inner.md#0x0_validator_inner_v1_validator_id">validator_id</a>(),
        stake_activation_epoch,
        staked_ika_principal_amount,
        fungible_staked_ika_amount: fungible_staked_ika.value(),
    });

    fungible_staked_ika
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_redeem_fungible_staked_ika"></a>

## Function `redeem_fungible_staked_ika`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_redeem_fungible_staked_ika">redeem_fungible_staked_ika</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, fungible_staked_ika: <a href="staked_ika.md#0x0_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>): <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_redeem_fungible_staked_ika">redeem_fungible_staked_ika</a>(
    self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    fungible_staked_ika: FungibleStakedIka,
): Balance&lt;IKA&gt; {
    <b>let</b> fungible_staked_ika_amount = fungible_staked_ika.value();

    <b>let</b> <a href="../ika/ika.md#0x0_ika">ika</a> = self.<a href="staking_pool.md#0x0_staking_pool">staking_pool</a>.<a href="validator_inner.md#0x0_validator_inner_v1_redeem_fungible_staked_ika">redeem_fungible_staked_ika</a>(epoch, fungible_staked_ika);

    self.next_epoch_stake = self.next_epoch_stake - <a href="../ika/ika.md#0x0_ika">ika</a>.value();

    <a href="../sui-framework/event.md#0x2_event_emit">event::emit</a>(<a href="validator_inner.md#0x0_validator_inner_v1_RedeemingFungibleStakedIkaEvent">RedeemingFungibleStakedIkaEvent</a> {
        validator_id: self.<a href="validator_inner.md#0x0_validator_inner_v1_validator_id">validator_id</a>(),
        fungible_staked_ika_amount,
        ika_amount: <a href="../ika/ika.md#0x0_ika">ika</a>.value(),
    });

    <a href="../ika/ika.md#0x0_ika">ika</a>
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_request_withdraw_stake"></a>

## Function `request_withdraw_stake`

Request to withdraw stake from the validator's staking pool, processed at the end of the epoch.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_request_withdraw_stake">request_withdraw_stake</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: <a href="staked_ika.md#0x0_staked_ika_StakedIka">staked_ika::StakedIka</a>): <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_request_withdraw_stake">request_withdraw_stake</a>(
    self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>: StakedIka,
): Balance&lt;IKA&gt; {
    <b>let</b> principal_amount = <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>.staked_ika_amount();
    <b>let</b> stake_activation_epoch = <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>.stake_activation_epoch();
    <b>let</b> staked_ika_id = <a href="../sui-framework/object.md#0x2_object_id">object::id</a>(&<a href="staked_ika.md#0x0_staked_ika">staked_ika</a>);
    <b>let</b> withdrawn_stake = self.<a href="staking_pool.md#0x0_staking_pool">staking_pool</a>.<a href="validator_inner.md#0x0_validator_inner_v1_request_withdraw_stake">request_withdraw_stake</a>(epoch, <a href="staked_ika.md#0x0_staked_ika">staked_ika</a>);
    <b>let</b> withdraw_amount = withdrawn_stake.value();
    <b>let</b> reward_amount = withdraw_amount - principal_amount;
    self.next_epoch_stake = self.next_epoch_stake - withdraw_amount;
    <a href="../sui-framework/event.md#0x2_event_emit">event::emit</a>(<a href="validator_inner.md#0x0_validator_inner_v1_UnstakingRequestEvent">UnstakingRequestEvent</a> {
        validator_id: self.<a href="validator_inner.md#0x0_validator_inner_v1_validator_id">validator_id</a>(),
        staked_ika_id,
        stake_activation_epoch,
        unstaking_epoch: epoch,
        principal_amount,
        reward_amount,
    });
    withdrawn_stake
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_request_set_computation_price"></a>

## Function `request_set_computation_price`

Request to set new computation price for the next epoch.
Need to present a <code>ValidatorOperationCap</code>.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_request_set_computation_price">request_set_computation_price</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, operation_cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, new_price: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_request_set_computation_price">request_set_computation_price</a>(
    self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    operation_cap: &ValidatorOperationCap,
    new_price: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
) {
    <b>assert</b>!(!<a href="validator_inner.md#0x0_validator_inner_v1_is_inactive">is_inactive</a>(self), <a href="validator_inner.md#0x0_validator_inner_v1_EInactiveValidator">EInactiveValidator</a>);
    <b>assert</b>!(new_price &lt; <a href="validator_inner.md#0x0_validator_inner_v1_MAX_VALIDATOR_COMPUTATION_PRICE">MAX_VALIDATOR_COMPUTATION_PRICE</a>, <a href="validator_inner.md#0x0_validator_inner_v1_EGasPriceHigherThanThreshold">EGasPriceHigherThanThreshold</a>);
    <b>let</b> validator_id = operation_cap.<a href="validator_inner.md#0x0_validator_inner_v1_validator_id">validator_id</a>();
    <b>assert</b>!(validator_id == self.<a href="validator_inner.md#0x0_validator_inner_v1_validator_id">validator_id</a>(), <a href="validator_inner.md#0x0_validator_inner_v1_EInvalidCap">EInvalidCap</a>);
    <b>assert</b>!(<a href="../sui-framework/object.md#0x2_object_id">object::id</a>(operation_cap) == self.<a href="validator_inner.md#0x0_validator_inner_v1_operation_cap_id">operation_cap_id</a>(), <a href="validator_inner.md#0x0_validator_inner_v1_EInvalidCap">EInvalidCap</a>);
    self.next_epoch_computation_price = new_price;
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_set_candidate_computation_price"></a>

## Function `set_candidate_computation_price`

Set new computation price for the candidate validator.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_set_candidate_computation_price">set_candidate_computation_price</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, operation_cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, new_price: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_set_candidate_computation_price">set_candidate_computation_price</a>(
    self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    operation_cap: &ValidatorOperationCap,
    new_price: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
) {
    <b>assert</b>!(<a href="validator_inner.md#0x0_validator_inner_v1_is_candidate">is_candidate</a>(self), <a href="validator_inner.md#0x0_validator_inner_v1_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    <b>assert</b>!(new_price &lt; <a href="validator_inner.md#0x0_validator_inner_v1_MAX_VALIDATOR_COMPUTATION_PRICE">MAX_VALIDATOR_COMPUTATION_PRICE</a>, <a href="validator_inner.md#0x0_validator_inner_v1_EGasPriceHigherThanThreshold">EGasPriceHigherThanThreshold</a>);
    <b>let</b> validator_id = operation_cap.<a href="validator_inner.md#0x0_validator_inner_v1_validator_id">validator_id</a>();
    <b>assert</b>!(validator_id == self.<a href="validator_inner.md#0x0_validator_inner_v1_validator_id">validator_id</a>(), <a href="validator_inner.md#0x0_validator_inner_v1_EInvalidCap">EInvalidCap</a>);
    <b>assert</b>!(<a href="../sui-framework/object.md#0x2_object_id">object::id</a>(operation_cap) == self.<a href="validator_inner.md#0x0_validator_inner_v1_operation_cap_id">operation_cap_id</a>(), <a href="validator_inner.md#0x0_validator_inner_v1_EInvalidCap">EInvalidCap</a>);
    self.next_epoch_computation_price = new_price;
    self.computation_price = new_price;
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_request_set_commission_rate"></a>

## Function `request_set_commission_rate`

Request to set new commission rate for the next epoch.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_request_set_commission_rate">request_set_commission_rate</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, new_commission_rate: u16)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_request_set_commission_rate">request_set_commission_rate</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>, new_commission_rate: u16) {
    <b>assert</b>!(!<a href="validator_inner.md#0x0_validator_inner_v1_is_inactive">is_inactive</a>(self), <a href="validator_inner.md#0x0_validator_inner_v1_EInactiveValidator">EInactiveValidator</a>);
    <b>assert</b>!(new_commission_rate &lt;= <a href="validator_inner.md#0x0_validator_inner_v1_MAX_COMMISSION_RATE">MAX_COMMISSION_RATE</a>, <a href="validator_inner.md#0x0_validator_inner_v1_ECommissionRateTooHigh">ECommissionRateTooHigh</a>);
    self.next_epoch_commission_rate = new_commission_rate;
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_set_candidate_commission_rate"></a>

## Function `set_candidate_commission_rate`

Set new commission rate for the candidate validator.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_set_candidate_commission_rate">set_candidate_commission_rate</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, new_commission_rate: u16)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_set_candidate_commission_rate">set_candidate_commission_rate</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>, new_commission_rate: u16) {
    <b>assert</b>!(<a href="validator_inner.md#0x0_validator_inner_v1_is_candidate">is_candidate</a>(self), <a href="validator_inner.md#0x0_validator_inner_v1_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    <b>assert</b>!(new_commission_rate &lt;= <a href="validator_inner.md#0x0_validator_inner_v1_MAX_COMMISSION_RATE">MAX_COMMISSION_RATE</a>, <a href="validator_inner.md#0x0_validator_inner_v1_ECommissionRateTooHigh">ECommissionRateTooHigh</a>);
    self.next_epoch_commission_rate = new_commission_rate;
    self.commission_rate = new_commission_rate;
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_deposit_stake_rewards"></a>

## Function `deposit_stake_rewards`

Deposit stakes rewards into the validator's staking pool, called at the end of the epoch.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_deposit_stake_rewards">deposit_stake_rewards</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, reward: <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../ika/ika.md#0x0_ika_IKA">ika::IKA</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_deposit_stake_rewards">deposit_stake_rewards</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>, reward: Balance&lt;IKA&gt;) {
    self.next_epoch_stake = self.next_epoch_stake + reward.value();
    self.<a href="staking_pool.md#0x0_staking_pool">staking_pool</a>.deposit_rewards(reward);
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_process_pending_stakes_and_withdraws"></a>

## Function `process_pending_stakes_and_withdraws`

Process pending stakes and withdraws, called at the end of the epoch.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, new_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>, new_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>) {
    self.<a href="staking_pool.md#0x0_staking_pool">staking_pool</a>.<a href="validator_inner.md#0x0_validator_inner_v1_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>(new_epoch);
    // TODO: bring this assertion back when we are ready.
    <b>assert</b>!(self.<a href="validator_inner.md#0x0_validator_inner_v1_total_stake_amount">total_stake_amount</a>() == self.next_epoch_stake, <a href="validator_inner.md#0x0_validator_inner_v1_EInvalidStakeAmount">EInvalidStakeAmount</a>);
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_is_candidate"></a>

## Function `is_candidate`

Returns true if the validator is candidate.


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_is_candidate">is_candidate</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_is_candidate">is_candidate</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): bool {
    self.<a href="staking_pool.md#0x0_staking_pool">staking_pool</a>.<a href="validator_inner.md#0x0_validator_inner_v1_is_candidate">is_candidate</a>()
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_is_inactive"></a>

## Function `is_inactive`

Returns true if the validator is inactive.


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_is_inactive">is_inactive</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_is_inactive">is_inactive</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): bool {
    self.<a href="staking_pool.md#0x0_staking_pool">staking_pool</a>.<a href="validator_inner.md#0x0_validator_inner_v1_is_inactive">is_inactive</a>()
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_validator_id"></a>

## Function `validator_id`



<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_validator_id">validator_id</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_validator_id">validator_id</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): ID {
    self.validator_id
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_metadata"></a>

## Function `metadata`



<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_metadata">metadata</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorMetadata">validator_inner_v1::ValidatorMetadata</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_metadata">metadata</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorMetadata">ValidatorMetadata</a> {
    &self.metadata
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_payment_address"></a>

## Function `payment_address`



<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_payment_address">payment_address</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_payment_address">payment_address</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): <b>address</b> {
    self.metadata.payment_address
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_name"></a>

## Function `name`



<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_name">name</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../move-stdlib/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_name">name</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &String {
    &self.metadata.name
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_description"></a>

## Function `description`



<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_description">description</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../move-stdlib/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_description">description</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &String {
    &self.metadata.description
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_image_url"></a>

## Function `image_url`



<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_image_url">image_url</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../sui-framework/url.md#0x2_url_Url">url::Url</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_image_url">image_url</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &Url {
    &self.metadata.image_url
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_project_url"></a>

## Function `project_url`



<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_project_url">project_url</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../sui-framework/url.md#0x2_url_Url">url::Url</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_project_url">project_url</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &Url {
    &self.metadata.project_url
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_network_address"></a>

## Function `network_address`



<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_network_address">network_address</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../move-stdlib/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_network_address">network_address</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &String {
    &self.metadata.network_address
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_p2p_address"></a>

## Function `p2p_address`



<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_p2p_address">p2p_address</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../move-stdlib/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_p2p_address">p2p_address</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &String {
    &self.metadata.p2p_address
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_consensus_address"></a>

## Function `consensus_address`



<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_consensus_address">consensus_address</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../move-stdlib/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_consensus_address">consensus_address</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &String {
    &self.metadata.consensus_address
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_protocol_pubkey_bytes"></a>

## Function `protocol_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_protocol_pubkey_bytes">protocol_pubkey_bytes</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_protocol_pubkey_bytes">protocol_pubkey_bytes</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    &self.metadata.protocol_pubkey_bytes
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_protocol_pubkey"></a>

## Function `protocol_pubkey`



<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_protocol_pubkey">protocol_pubkey</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../sui-framework/group_ops.md#0x2_group_ops_Element">group_ops::Element</a>&lt;<a href="../sui-framework/bls12381.md#0x2_bls12381_UncompressedG1">bls12381::UncompressedG1</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_protocol_pubkey">protocol_pubkey</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &Element&lt;UncompressedG1&gt; {
    &self.metadata.protocol_pubkey
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_proof_of_possession_bytes"></a>

## Function `proof_of_possession_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_proof_of_possession_bytes">proof_of_possession_bytes</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_proof_of_possession_bytes">proof_of_possession_bytes</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    &self.metadata.proof_of_possession_bytes
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_network_pubkey_bytes"></a>

## Function `network_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_network_pubkey_bytes">network_pubkey_bytes</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_network_pubkey_bytes">network_pubkey_bytes</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    &self.metadata.network_pubkey_bytes
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_consensus_pubkey_bytes"></a>

## Function `consensus_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_consensus_pubkey_bytes">consensus_pubkey_bytes</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_consensus_pubkey_bytes">consensus_pubkey_bytes</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    &self.metadata.consensus_pubkey_bytes
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_next_epoch_network_address"></a>

## Function `next_epoch_network_address`



<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_next_epoch_network_address">next_epoch_network_address</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/string.md#0x1_string_String">string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_next_epoch_network_address">next_epoch_network_address</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &Option&lt;String&gt; {
    &self.metadata.next_epoch_network_address
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_next_epoch_p2p_address"></a>

## Function `next_epoch_p2p_address`



<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_next_epoch_p2p_address">next_epoch_p2p_address</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/string.md#0x1_string_String">string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_next_epoch_p2p_address">next_epoch_p2p_address</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &Option&lt;String&gt; {
    &self.metadata.next_epoch_p2p_address
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_next_epoch_consensus_address"></a>

## Function `next_epoch_consensus_address`



<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_next_epoch_consensus_address">next_epoch_consensus_address</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/string.md#0x1_string_String">string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_next_epoch_consensus_address">next_epoch_consensus_address</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &Option&lt;String&gt; {
    &self.metadata.next_epoch_consensus_address
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_next_epoch_protocol_pubkey_bytes"></a>

## Function `next_epoch_protocol_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &Option&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt; {
    &self.metadata.next_epoch_protocol_pubkey_bytes
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_next_epoch_proof_of_possession_bytes"></a>

## Function `next_epoch_proof_of_possession_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_next_epoch_proof_of_possession_bytes">next_epoch_proof_of_possession_bytes</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_next_epoch_proof_of_possession_bytes">next_epoch_proof_of_possession_bytes</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &Option&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt; {
    &self.metadata.next_epoch_proof_of_possession_bytes
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_next_epoch_network_pubkey_bytes"></a>

## Function `next_epoch_network_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &Option&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt; {
    &self.metadata.next_epoch_network_pubkey_bytes
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_next_epoch_consensus_pubkey_bytes"></a>

## Function `next_epoch_consensus_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &Option&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt; {
    &self.metadata.next_epoch_consensus_pubkey_bytes
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_operation_cap_id"></a>

## Function `operation_cap_id`



<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_operation_cap_id">operation_cap_id</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_operation_cap_id">operation_cap_id</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &ID {
    &self.operation_cap_id
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_next_epoch_computation_price"></a>

## Function `next_epoch_computation_price`



<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_next_epoch_computation_price">next_epoch_computation_price</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_next_epoch_computation_price">next_epoch_computation_price</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    self.next_epoch_computation_price
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_total_stake_amount"></a>

## Function `total_stake_amount`

Return the total amount staked with this validator


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_total_stake_amount">total_stake_amount</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_total_stake_amount">total_stake_amount</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    self.<a href="staking_pool.md#0x0_staking_pool">staking_pool</a>.ika_balance()
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_pending_stake_amount"></a>

## Function `pending_stake_amount`



<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_pending_stake_amount">pending_stake_amount</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_pending_stake_amount">pending_stake_amount</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    self.<a href="staking_pool.md#0x0_staking_pool">staking_pool</a>.<a href="validator_inner.md#0x0_validator_inner_v1_pending_stake_amount">pending_stake_amount</a>()
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_pending_stake_withdraw_amount"></a>

## Function `pending_stake_withdraw_amount`



<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_pending_stake_withdraw_amount">pending_stake_withdraw_amount</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_pending_stake_withdraw_amount">pending_stake_withdraw_amount</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    self.<a href="staking_pool.md#0x0_staking_pool">staking_pool</a>.<a href="validator_inner.md#0x0_validator_inner_v1_pending_stake_withdraw_amount">pending_stake_withdraw_amount</a>()
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_computation_price"></a>

## Function `computation_price`



<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_computation_price">computation_price</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_computation_price">computation_price</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    self.computation_price
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_commission_rate"></a>

## Function `commission_rate`



<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_commission_rate">commission_rate</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): u16
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_commission_rate">commission_rate</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): u16 {
    self.commission_rate
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_pool_token_exchange_rate_at_epoch"></a>

## Function `pool_token_exchange_rate_at_epoch`



<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="staking_pool.md#0x0_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): PoolTokenExchangeRate {
    self.<a href="staking_pool.md#0x0_staking_pool">staking_pool</a>.<a href="validator_inner.md#0x0_validator_inner_v1_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(epoch)
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_is_duplicate"></a>

## Function `is_duplicate`



<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_is_duplicate">is_duplicate</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, other: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_is_duplicate">is_duplicate</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>, other: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): bool {
            self.metadata.name == other.metadata.name
            || self.metadata.network_address == other.metadata.network_address
            || self.metadata.p2p_address == other.metadata.p2p_address
            || self.metadata.protocol_pubkey_bytes == other.metadata.protocol_pubkey_bytes
            || self.metadata.network_pubkey_bytes == other.metadata.network_pubkey_bytes
            || self.metadata.network_pubkey_bytes == other.metadata.consensus_pubkey_bytes
            || self.metadata.consensus_pubkey_bytes == other.metadata.consensus_pubkey_bytes
            || self.metadata.consensus_pubkey_bytes == other.metadata.network_pubkey_bytes
            // All next epoch parameters.
            || <a href="validator_inner.md#0x0_validator_inner_v1_is_equal_some">is_equal_some</a>(&self.metadata.next_epoch_network_address, &other.metadata.next_epoch_network_address)
            || <a href="validator_inner.md#0x0_validator_inner_v1_is_equal_some">is_equal_some</a>(&self.metadata.next_epoch_p2p_address, &other.metadata.next_epoch_p2p_address)
            || <a href="validator_inner.md#0x0_validator_inner_v1_is_equal_some">is_equal_some</a>(&self.metadata.next_epoch_protocol_pubkey_bytes, &other.metadata.next_epoch_protocol_pubkey_bytes)
            || <a href="validator_inner.md#0x0_validator_inner_v1_is_equal_some">is_equal_some</a>(&self.metadata.next_epoch_network_pubkey_bytes, &other.metadata.next_epoch_network_pubkey_bytes)
            || <a href="validator_inner.md#0x0_validator_inner_v1_is_equal_some">is_equal_some</a>(&self.metadata.next_epoch_network_pubkey_bytes, &other.metadata.next_epoch_consensus_pubkey_bytes)
            || <a href="validator_inner.md#0x0_validator_inner_v1_is_equal_some">is_equal_some</a>(&self.metadata.next_epoch_consensus_pubkey_bytes, &other.metadata.next_epoch_consensus_pubkey_bytes)
            || <a href="validator_inner.md#0x0_validator_inner_v1_is_equal_some">is_equal_some</a>(&self.metadata.next_epoch_consensus_pubkey_bytes, &other.metadata.next_epoch_network_pubkey_bytes)
            // My next epoch parameters <b>with</b> other current epoch parameters.
            || <a href="validator_inner.md#0x0_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>(&self.metadata.next_epoch_network_address, &other.metadata.network_address)
            || <a href="validator_inner.md#0x0_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>(&self.metadata.next_epoch_p2p_address, &other.metadata.p2p_address)
            || <a href="validator_inner.md#0x0_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>(&self.metadata.next_epoch_protocol_pubkey_bytes, &other.metadata.protocol_pubkey_bytes)
            || <a href="validator_inner.md#0x0_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>(&self.metadata.next_epoch_network_pubkey_bytes, &other.metadata.network_pubkey_bytes)
            || <a href="validator_inner.md#0x0_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>(&self.metadata.next_epoch_network_pubkey_bytes, &other.metadata.consensus_pubkey_bytes)
            || <a href="validator_inner.md#0x0_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>(&self.metadata.next_epoch_consensus_pubkey_bytes, &other.metadata.consensus_pubkey_bytes)
            || <a href="validator_inner.md#0x0_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>(&self.metadata.next_epoch_consensus_pubkey_bytes, &other.metadata.network_pubkey_bytes)
            // Other next epoch parameters <b>with</b> my current epoch parameters.
            || <a href="validator_inner.md#0x0_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>(&other.metadata.next_epoch_network_address, &self.metadata.network_address)
            || <a href="validator_inner.md#0x0_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>(&other.metadata.next_epoch_p2p_address, &self.metadata.p2p_address)
            || <a href="validator_inner.md#0x0_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>(&other.metadata.next_epoch_protocol_pubkey_bytes, &self.metadata.protocol_pubkey_bytes)
            || <a href="validator_inner.md#0x0_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>(&other.metadata.next_epoch_network_pubkey_bytes, &self.metadata.network_pubkey_bytes)
            || <a href="validator_inner.md#0x0_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>(&other.metadata.next_epoch_network_pubkey_bytes, &self.metadata.consensus_pubkey_bytes)
            || <a href="validator_inner.md#0x0_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>(&other.metadata.next_epoch_consensus_pubkey_bytes, &self.metadata.consensus_pubkey_bytes)
            || <a href="validator_inner.md#0x0_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>(&other.metadata.next_epoch_consensus_pubkey_bytes, &self.metadata.network_pubkey_bytes)
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_is_equal_some_and_value"></a>

## Function `is_equal_some_and_value`



<pre><code><b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>&lt;T&gt;(a: &<a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;T&gt;, b: &T): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>&lt;T&gt;(a: &Option&lt;T&gt;, b: &T): bool {
    <b>if</b> (a.is_none()) {
        <b>false</b>
    } <b>else</b> {
        a.borrow() == b
    }
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_is_equal_some"></a>

## Function `is_equal_some`



<pre><code><b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_is_equal_some">is_equal_some</a>&lt;T&gt;(a: &<a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;T&gt;, b: &<a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;T&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_is_equal_some">is_equal_some</a>&lt;T&gt;(a: &Option&lt;T&gt;, b: &Option&lt;T&gt;): bool {
    <b>if</b> (a.is_none() || b.is_none()) {
        <b>false</b>
    } <b>else</b> {
        a.borrow() == b.borrow()
    }
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_new_validator_operation_cap"></a>

## Function `new_validator_operation_cap`

Create a new <code>ValidatorOperationCap</code>, and registers it,
thus revoking the previous cap's permission.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_new_validator_operation_cap">new_validator_operation_cap</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, cap: &<a href="validator_cap.md#0x0_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="validator_cap.md#0x0_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_new_validator_operation_cap">new_validator_operation_cap</a>(
    self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    cap: &ValidatorCap,
    ctx: &<b>mut</b> TxContext,
): ValidatorOperationCap {
    <b>let</b> validator_id = cap.<a href="validator_inner.md#0x0_validator_inner_v1_validator_id">validator_id</a>();
    <b>assert</b>!(validator_id == self.<a href="validator_inner.md#0x0_validator_inner_v1_validator_id">validator_id</a>(), <a href="validator_inner.md#0x0_validator_inner_v1_ENewCapNotCreatedByValidatorItself">ENewCapNotCreatedByValidatorItself</a>);
    <b>let</b> operation_cap = <a href="validator_cap.md#0x0_validator_cap_new_validator_operation_cap">validator_cap::new_validator_operation_cap</a>(validator_id, ctx);
    self.operation_cap_id = <a href="../sui-framework/object.md#0x2_object_id">object::id</a>(&operation_cap);
    operation_cap
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_update_payment_address"></a>

## Function `update_payment_address`

Update payment address of the validator.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_payment_address">update_payment_address</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, payment_address: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_payment_address">update_payment_address</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>, payment_address: <b>address</b>) {
    self.metadata.payment_address = payment_address;
    <a href="validator_inner.md#0x0_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_update_name"></a>

## Function `update_name`

Update name of the validator.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_name">update_name</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, name: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_name">update_name</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>, name: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;) {
    self.metadata.name = name.to_ascii_string().to_string();
    <a href="validator_inner.md#0x0_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_update_description"></a>

## Function `update_description`

Update description of the validator.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_description">update_description</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, description: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_description">update_description</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>, description: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;) {
    self.metadata.description = description.to_ascii_string().to_string();
    <a href="validator_inner.md#0x0_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_update_image_url"></a>

## Function `update_image_url`

Update image url of the validator.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_image_url">update_image_url</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, image_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_image_url">update_image_url</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>, image_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;) {
    self.metadata.image_url = <a href="../sui-framework/url.md#0x2_url_new_unsafe_from_bytes">url::new_unsafe_from_bytes</a>(image_url);
    <a href="validator_inner.md#0x0_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_update_project_url"></a>

## Function `update_project_url`

Update project url of the validator.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_project_url">update_project_url</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, project_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_project_url">update_project_url</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>, project_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;) {
    self.metadata.project_url = <a href="../sui-framework/url.md#0x2_url_new_unsafe_from_bytes">url::new_unsafe_from_bytes</a>(project_url);
    <a href="validator_inner.md#0x0_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_update_next_epoch_network_address"></a>

## Function `update_next_epoch_network_address`

Update network address of this validator, taking effects from next epoch


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_next_epoch_network_address">update_next_epoch_network_address</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, network_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_next_epoch_network_address">update_next_epoch_network_address</a>(
    self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    network_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
) {
    <b>assert</b>!(!<a href="validator_inner.md#0x0_validator_inner_v1_is_inactive">is_inactive</a>(self), <a href="validator_inner.md#0x0_validator_inner_v1_EInactiveValidator">EInactiveValidator</a>);
    <b>let</b> network_address = network_address.to_ascii_string().to_string();
    self.metadata.next_epoch_network_address = <a href="../move-stdlib/option.md#0x1_option_some">option::some</a>(network_address);
    <a href="validator_inner.md#0x0_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_update_candidate_network_address"></a>

## Function `update_candidate_network_address`

Update network address of this candidate validator


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_candidate_network_address">update_candidate_network_address</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, network_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_candidate_network_address">update_candidate_network_address</a>(
    self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    network_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
) {
    <b>assert</b>!(<a href="validator_inner.md#0x0_validator_inner_v1_is_candidate">is_candidate</a>(self), <a href="validator_inner.md#0x0_validator_inner_v1_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    <b>let</b> network_address = network_address.to_ascii_string().to_string();
    self.metadata.network_address = network_address;
    <a href="validator_inner.md#0x0_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_update_next_epoch_p2p_address"></a>

## Function `update_next_epoch_p2p_address`

Update p2p address of this validator, taking effects from next epoch


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_next_epoch_p2p_address">update_next_epoch_p2p_address</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, p2p_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_next_epoch_p2p_address">update_next_epoch_p2p_address</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>, p2p_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;) {
    <b>assert</b>!(!<a href="validator_inner.md#0x0_validator_inner_v1_is_inactive">is_inactive</a>(self), <a href="validator_inner.md#0x0_validator_inner_v1_EInactiveValidator">EInactiveValidator</a>);
    <b>let</b> p2p_address = p2p_address.to_ascii_string().to_string();
    self.metadata.next_epoch_p2p_address = <a href="../move-stdlib/option.md#0x1_option_some">option::some</a>(p2p_address);
    <a href="validator_inner.md#0x0_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_update_candidate_p2p_address"></a>

## Function `update_candidate_p2p_address`

Update p2p address of this candidate validator


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_candidate_p2p_address">update_candidate_p2p_address</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, p2p_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_candidate_p2p_address">update_candidate_p2p_address</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>, p2p_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;) {
    <b>assert</b>!(<a href="validator_inner.md#0x0_validator_inner_v1_is_candidate">is_candidate</a>(self), <a href="validator_inner.md#0x0_validator_inner_v1_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    <b>let</b> p2p_address = p2p_address.to_ascii_string().to_string();
    self.metadata.p2p_address = p2p_address;
    <a href="validator_inner.md#0x0_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_update_next_epoch_consensus_address"></a>

## Function `update_next_epoch_consensus_address`

Update primary address of this validator, taking effects from next epoch


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_next_epoch_consensus_address">update_next_epoch_consensus_address</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, consensus_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_next_epoch_consensus_address">update_next_epoch_consensus_address</a>(
    self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    consensus_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
) {
    <b>assert</b>!(!<a href="validator_inner.md#0x0_validator_inner_v1_is_inactive">is_inactive</a>(self), <a href="validator_inner.md#0x0_validator_inner_v1_EInactiveValidator">EInactiveValidator</a>);
    <b>let</b> consensus_address = consensus_address.to_ascii_string().to_string();
    self.metadata.next_epoch_consensus_address = <a href="../move-stdlib/option.md#0x1_option_some">option::some</a>(consensus_address);
    <a href="validator_inner.md#0x0_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_update_candidate_consensus_address"></a>

## Function `update_candidate_consensus_address`

Update primary address of this candidate validator


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_candidate_consensus_address">update_candidate_consensus_address</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, consensus_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_candidate_consensus_address">update_candidate_consensus_address</a>(
    self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    consensus_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
) {
    <b>assert</b>!(<a href="validator_inner.md#0x0_validator_inner_v1_is_candidate">is_candidate</a>(self), <a href="validator_inner.md#0x0_validator_inner_v1_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    <b>let</b> consensus_address = consensus_address.to_ascii_string().to_string();
    self.metadata.consensus_address = consensus_address;
    <a href="validator_inner.md#0x0_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_update_next_epoch_protocol_pubkey_bytes"></a>

## Function `update_next_epoch_protocol_pubkey_bytes`

Update protocol public key of this validator, taking effects from next epoch


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_next_epoch_protocol_pubkey_bytes">update_next_epoch_protocol_pubkey_bytes</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, protocol_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, proof_of_possession_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_next_epoch_protocol_pubkey_bytes">update_next_epoch_protocol_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    protocol_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    proof_of_possession_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>assert</b>!(!<a href="validator_inner.md#0x0_validator_inner_v1_is_inactive">is_inactive</a>(self), <a href="validator_inner.md#0x0_validator_inner_v1_EInactiveValidator">EInactiveValidator</a>);
    self.metadata.proof_of_possession_sender = ctx.sender();
    self.metadata.next_epoch_protocol_pubkey_bytes = <a href="../move-stdlib/option.md#0x1_option_some">option::some</a>(protocol_pubkey_bytes);
    self.metadata.next_epoch_proof_of_possession_bytes = <a href="../move-stdlib/option.md#0x1_option_some">option::some</a>(proof_of_possession_bytes);
    <a href="validator_inner.md#0x0_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_update_candidate_protocol_pubkey_bytes"></a>

## Function `update_candidate_protocol_pubkey_bytes`

Update protocol public key of this candidate validator


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_candidate_protocol_pubkey_bytes">update_candidate_protocol_pubkey_bytes</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, protocol_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, proof_of_possession_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_candidate_protocol_pubkey_bytes">update_candidate_protocol_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    protocol_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    proof_of_possession_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>assert</b>!(<a href="validator_inner.md#0x0_validator_inner_v1_is_candidate">is_candidate</a>(self), <a href="validator_inner.md#0x0_validator_inner_v1_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    self.metadata.proof_of_possession_sender = ctx.sender();
    self.metadata.protocol_pubkey_bytes = protocol_pubkey_bytes;
    self.metadata.proof_of_possession_bytes = proof_of_possession_bytes;
    <a href="validator_inner.md#0x0_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_update_next_epoch_network_pubkey_bytes"></a>

## Function `update_next_epoch_network_pubkey_bytes`

Update network public key of this validator, taking effects from next epoch


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_next_epoch_network_pubkey_bytes">update_next_epoch_network_pubkey_bytes</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, network_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_next_epoch_network_pubkey_bytes">update_next_epoch_network_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    network_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
) {
    <b>assert</b>!(!<a href="validator_inner.md#0x0_validator_inner_v1_is_inactive">is_inactive</a>(self), <a href="validator_inner.md#0x0_validator_inner_v1_EInactiveValidator">EInactiveValidator</a>);
    self.metadata.next_epoch_network_pubkey_bytes = <a href="../move-stdlib/option.md#0x1_option_some">option::some</a>(network_pubkey_bytes);
    <a href="validator_inner.md#0x0_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_update_candidate_network_pubkey_bytes"></a>

## Function `update_candidate_network_pubkey_bytes`

Update network public key of this candidate validator


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_candidate_network_pubkey_bytes">update_candidate_network_pubkey_bytes</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, network_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_candidate_network_pubkey_bytes">update_candidate_network_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    network_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
) {
    <b>assert</b>!(<a href="validator_inner.md#0x0_validator_inner_v1_is_candidate">is_candidate</a>(self), <a href="validator_inner.md#0x0_validator_inner_v1_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    self.metadata.network_pubkey_bytes = network_pubkey_bytes;
    <a href="validator_inner.md#0x0_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_update_next_epoch_consensus_pubkey_bytes"></a>

## Function `update_next_epoch_consensus_pubkey_bytes`

Update consensus public key of this validator, taking effects from next epoch


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_next_epoch_consensus_pubkey_bytes">update_next_epoch_consensus_pubkey_bytes</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, consensus_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_next_epoch_consensus_pubkey_bytes">update_next_epoch_consensus_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    consensus_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
) {
    <b>assert</b>!(!<a href="validator_inner.md#0x0_validator_inner_v1_is_inactive">is_inactive</a>(self), <a href="validator_inner.md#0x0_validator_inner_v1_EInactiveValidator">EInactiveValidator</a>);
    self.metadata.next_epoch_consensus_pubkey_bytes = <a href="../move-stdlib/option.md#0x1_option_some">option::some</a>(consensus_pubkey_bytes);
    <a href="validator_inner.md#0x0_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_update_candidate_consensus_pubkey_bytes"></a>

## Function `update_candidate_consensus_pubkey_bytes`

Update consensus public key of this candidate validator


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_candidate_consensus_pubkey_bytes">update_candidate_consensus_pubkey_bytes</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, consensus_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_update_candidate_consensus_pubkey_bytes">update_candidate_consensus_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    consensus_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
) {
    <b>assert</b>!(<a href="validator_inner.md#0x0_validator_inner_v1_is_candidate">is_candidate</a>(self), <a href="validator_inner.md#0x0_validator_inner_v1_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    self.metadata.consensus_pubkey_bytes = consensus_pubkey_bytes;
    <a href="validator_inner.md#0x0_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_effectuate_staged_metadata"></a>

## Function `effectuate_staged_metadata`

Effectutate all staged next epoch metadata for this validator.
NOTE: this function SHOULD ONLY be called by validator_set when
advancing an epoch.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_effectuate_staged_metadata">effectuate_staged_metadata</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_effectuate_staged_metadata">effectuate_staged_metadata</a>(self: &<b>mut</b> <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>) {
    <b>if</b> (<a href="validator_inner.md#0x0_validator_inner_v1_next_epoch_network_address">next_epoch_network_address</a>(self).is_some()) {
        self.metadata.network_address = self.metadata.next_epoch_network_address.extract();
        self.metadata.next_epoch_network_address = <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>();
    };

    <b>if</b> (<a href="validator_inner.md#0x0_validator_inner_v1_next_epoch_p2p_address">next_epoch_p2p_address</a>(self).is_some()) {
        self.metadata.p2p_address = self.metadata.next_epoch_p2p_address.extract();
        self.metadata.next_epoch_p2p_address = <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>();
    };

    <b>if</b> (<a href="validator_inner.md#0x0_validator_inner_v1_next_epoch_consensus_address">next_epoch_consensus_address</a>(self).is_some()) {
        self.metadata.consensus_address = self.metadata.next_epoch_consensus_address.extract();
        self.metadata.next_epoch_consensus_address = <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>();
    };

    <b>if</b> (<a href="validator_inner.md#0x0_validator_inner_v1_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>(self).is_some()) {
        self.metadata.protocol_pubkey_bytes =
        self.metadata.next_epoch_protocol_pubkey_bytes.extract();
        self.metadata.next_epoch_protocol_pubkey_bytes = <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>();
        self.metadata.protocol_pubkey = g1_to_uncompressed_g1(&g1_from_bytes(&self.metadata.protocol_pubkey_bytes));
        self.metadata.proof_of_possession_bytes = self.metadata.next_epoch_proof_of_possession_bytes.extract();
        self.metadata.next_epoch_proof_of_possession_bytes = <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>();
    };

    <b>if</b> (<a href="validator_inner.md#0x0_validator_inner_v1_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>(self).is_some()) {
        self.metadata.network_pubkey_bytes =
            self.metadata.next_epoch_network_pubkey_bytes.extract();
        self.metadata.next_epoch_network_pubkey_bytes = <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>();
    };

    <b>if</b> (<a href="validator_inner.md#0x0_validator_inner_v1_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>(self).is_some()) {
        self.metadata.consensus_pubkey_bytes = self.metadata.next_epoch_consensus_pubkey_bytes.extract();
        self.metadata.next_epoch_consensus_pubkey_bytes = <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>();
    };
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_verify_proof_of_possession"></a>

## Function `verify_proof_of_possession`

Verify the provided proof of possession using the contained public key and the provided
signature.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_verify_proof_of_possession">verify_proof_of_possession</a>(epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, sender_address: <b>address</b>, bls_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, pop_signature: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_verify_proof_of_possession">verify_proof_of_possession</a>(
    epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    sender_address: <b>address</b>,
    bls_key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    pop_signature: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
): bool {
    <b>let</b> <b>mut</b> intent_bytes = <a href="validator_inner.md#0x0_validator_inner_v1_PROOF_OF_POSSESSION_INTENT">PROOF_OF_POSSESSION_INTENT</a>;
    <b>let</b> <b>mut</b> message = <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;[];
    message.append(bls_key);
    message.append(sui::address::to_bytes(sender_address));
    intent_bytes.append(<a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&message));
    intent_bytes.append(<a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&epoch));
    bls12381_min_pk_verify(
        &pop_signature,
        &bls_key,
        &intent_bytes,
    )
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_validate_metadata"></a>

## Function `validate_metadata`

Aborts if validator metadata is invalid


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_validate_metadata">validate_metadata</a>(metadata: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorMetadata">validator_inner_v1::ValidatorMetadata</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_validate_metadata">validate_metadata</a>(metadata: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorMetadata">ValidatorMetadata</a>) {
    <b>assert</b>!(
        metadata.network_address.length() &lt;= <a href="validator_inner.md#0x0_validator_inner_v1_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
                && metadata.p2p_address.length() &lt;= <a href="validator_inner.md#0x0_validator_inner_v1_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
                && metadata.consensus_address.length() &lt;= <a href="validator_inner.md#0x0_validator_inner_v1_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
                && metadata.name.length() &lt;= <a href="validator_inner.md#0x0_validator_inner_v1_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
                && metadata.description.length() &lt;= <a href="validator_inner.md#0x0_validator_inner_v1_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
                && metadata.image_url.inner_url().length() &lt;= <a href="validator_inner.md#0x0_validator_inner_v1_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
                && metadata.project_url.inner_url().length() &lt;= <a href="validator_inner.md#0x0_validator_inner_v1_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="validator_inner.md#0x0_validator_inner_v1_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>,
    );
    <b>if</b> (metadata.next_epoch_network_address.is_some()) {
        <b>assert</b>!(metadata.next_epoch_network_address.borrow().length() &lt;= <a href="validator_inner.md#0x0_validator_inner_v1_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>, <a href="validator_inner.md#0x0_validator_inner_v1_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>);
    };
    <b>if</b> (metadata.next_epoch_p2p_address.is_some()) {
        <b>assert</b>!(metadata.next_epoch_p2p_address.borrow().length() &lt;= <a href="validator_inner.md#0x0_validator_inner_v1_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>, <a href="validator_inner.md#0x0_validator_inner_v1_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>);
    };
    <b>if</b> (metadata.next_epoch_consensus_address.is_some()) {
        <b>assert</b>!(metadata.next_epoch_consensus_address.borrow().length() &lt;= <a href="validator_inner.md#0x0_validator_inner_v1_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>, <a href="validator_inner.md#0x0_validator_inner_v1_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>);
    };

    <b>assert</b>!(metadata.network_pubkey_bytes.length() == <a href="validator_inner.md#0x0_validator_inner_v1_ED25519_KEY_LEN">ED25519_KEY_LEN</a>, <a href="validator_inner.md#0x0_validator_inner_v1_EMetadataInvalidNetworkPubkey">EMetadataInvalidNetworkPubkey</a>);
    <b>if</b> (metadata.next_epoch_network_pubkey_bytes.is_some()) {
        <b>assert</b>!(metadata.next_epoch_network_pubkey_bytes.borrow().length() == <a href="validator_inner.md#0x0_validator_inner_v1_ED25519_KEY_LEN">ED25519_KEY_LEN</a>, <a href="validator_inner.md#0x0_validator_inner_v1_EMetadataInvalidNetworkPubkey">EMetadataInvalidNetworkPubkey</a>);
    };
    <b>assert</b>!(metadata.consensus_pubkey_bytes.length() == <a href="validator_inner.md#0x0_validator_inner_v1_ED25519_KEY_LEN">ED25519_KEY_LEN</a>, <a href="validator_inner.md#0x0_validator_inner_v1_EMetadataInvalidConsensusPubkey">EMetadataInvalidConsensusPubkey</a>);
    <b>if</b> (metadata.next_epoch_consensus_pubkey_bytes.is_some()) {
        <b>assert</b>!(metadata.next_epoch_consensus_pubkey_bytes.borrow().length() == <a href="validator_inner.md#0x0_validator_inner_v1_ED25519_KEY_LEN">ED25519_KEY_LEN</a>, <a href="validator_inner.md#0x0_validator_inner_v1_EMetadataInvalidConsensusPubkey">EMetadataInvalidConsensusPubkey</a>);
    };

    <b>assert</b>!(metadata.protocol_pubkey_bytes.length() == <a href="validator_inner.md#0x0_validator_inner_v1_BLS_KEY_LEN">BLS_KEY_LEN</a>, <a href="validator_inner.md#0x0_validator_inner_v1_EMetadataInvalidProtocolPubkey">EMetadataInvalidProtocolPubkey</a>);
    <b>assert</b>!(
        <a href="validator_inner.md#0x0_validator_inner_v1_verify_proof_of_possession">verify_proof_of_possession</a>(
            <a href="validator_inner.md#0x0_validator_inner_v1_DEFAULT_EPOCH_ID">DEFAULT_EPOCH_ID</a>,
            metadata.proof_of_possession_sender,
            metadata.protocol_pubkey_bytes,
            metadata.proof_of_possession_bytes
        ),
        <a href="validator_inner.md#0x0_validator_inner_v1_EInvalidProofOfPossession">EInvalidProofOfPossession</a>
    );
    <b>if</b> (metadata.next_epoch_protocol_pubkey_bytes.is_some()) {
        <b>assert</b>!(metadata.next_epoch_protocol_pubkey_bytes.borrow().length() == <a href="validator_inner.md#0x0_validator_inner_v1_BLS_KEY_LEN">BLS_KEY_LEN</a>, <a href="validator_inner.md#0x0_validator_inner_v1_EMetadataInvalidProtocolPubkey">EMetadataInvalidProtocolPubkey</a>);
        <b>assert</b>!(
            <a href="validator_inner.md#0x0_validator_inner_v1_verify_proof_of_possession">verify_proof_of_possession</a>(
                <a href="validator_inner.md#0x0_validator_inner_v1_DEFAULT_EPOCH_ID">DEFAULT_EPOCH_ID</a>,
                metadata.proof_of_possession_sender,
                *metadata.next_epoch_protocol_pubkey_bytes.borrow(),
                *metadata.next_epoch_proof_of_possession_bytes.borrow()
            ),
            <a href="validator_inner.md#0x0_validator_inner_v1_EInvalidProofOfPossession">EInvalidProofOfPossession</a>
        );
    };

    // TODO(omersadika): add <a href="test.md#0x0_test">test</a> for next epoch
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_get_staking_pool_ref"></a>

## Function `get_staking_pool_ref`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_get_staking_pool_ref">get_staking_pool_ref</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="staking_pool.md#0x0_staking_pool_StakingPool">staking_pool::StakingPool</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_get_staking_pool_ref">get_staking_pool_ref</a>(self: &<a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &StakingPool {
    &self.<a href="staking_pool.md#0x0_staking_pool">staking_pool</a>
}
</code></pre>



</details>

<a name="0x0_validator_inner_v1_create_from_metadata"></a>

## Function `create_from_metadata`

Create a new validator from the given <code><a href="validator_inner.md#0x0_validator_inner_v1_ValidatorMetadata">ValidatorMetadata</a></code>, called by both <code>new</code> and <code>new_for_testing</code>.


<pre><code><b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_create_from_metadata">create_from_metadata</a>(validator_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, cap_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, operation_cap_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, metadata: <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorMetadata">validator_inner_v1::ValidatorMetadata</a>, computation_price: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, commission_rate: u16, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator_inner.md#0x0_validator_inner_v1_create_from_metadata">create_from_metadata</a>(
    validator_id: ID,
    cap_id: ID,
    operation_cap_id: ID,
    metadata: <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorMetadata">ValidatorMetadata</a>,
    computation_price: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    commission_rate: u16,
    ctx: &<b>mut</b> TxContext,
): <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a> {
    <b>let</b> <a href="staking_pool.md#0x0_staking_pool">staking_pool</a> = <a href="staking_pool.md#0x0_staking_pool_new">staking_pool::new</a>(validator_id, ctx);

    <b>let</b> <a href="validator_inner.md#0x0_validator_inner_v1">validator_inner_v1</a> = <a href="validator_inner.md#0x0_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a> {
        validator_id,
        metadata,
        // Initialize the voting power <b>to</b> be 0.
        // At the epoch change <b>where</b> this <a href="validator.md#0x0_validator">validator</a> is actually added <b>to</b> the
        // active <a href="validator.md#0x0_validator">validator</a> set, the voting power will be updated accordingly.
        cap_id,
        operation_cap_id,
        computation_price,
        <a href="staking_pool.md#0x0_staking_pool">staking_pool</a>,
        commission_rate,
        next_epoch_stake: 0,
        next_epoch_computation_price: computation_price,
        next_epoch_commission_rate: commission_rate,
        extra_fields: <a href="../sui-framework/bag.md#0x2_bag_new">bag::new</a>(ctx),
    };
    <a href="validator_inner.md#0x0_validator_inner_v1">validator_inner_v1</a>
}
</code></pre>



</details>
