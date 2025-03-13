---
title: Module `(ika_system=0x0)::validator_inner_v1`
---



-  [Struct `ValidatorMetadata`](#(ika_system=0x0)_validator_inner_v1_ValidatorMetadata)
-  [Struct `ValidatorInnerV1`](#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1)
-  [Struct `StakingRequestEvent`](#(ika_system=0x0)_validator_inner_v1_StakingRequestEvent)
-  [Struct `UnstakingRequestEvent`](#(ika_system=0x0)_validator_inner_v1_UnstakingRequestEvent)
-  [Struct `ConvertingToFungibleStakedIkaEvent`](#(ika_system=0x0)_validator_inner_v1_ConvertingToFungibleStakedIkaEvent)
-  [Struct `RedeemingFungibleStakedIkaEvent`](#(ika_system=0x0)_validator_inner_v1_RedeemingFungibleStakedIkaEvent)
-  [Constants](#@Constants_0)
-  [Function `create_metadata`](#(ika_system=0x0)_validator_inner_v1_create_metadata)
-  [Function `create`](#(ika_system=0x0)_validator_inner_v1_create)
-  [Function `deactivate`](#(ika_system=0x0)_validator_inner_v1_deactivate)
-  [Function `activate`](#(ika_system=0x0)_validator_inner_v1_activate)
-  [Function `adjust_stake_and_computation_price`](#(ika_system=0x0)_validator_inner_v1_adjust_stake_and_computation_price)
-  [Function `request_add_stake`](#(ika_system=0x0)_validator_inner_v1_request_add_stake)
-  [Function `convert_to_fungible_staked_ika`](#(ika_system=0x0)_validator_inner_v1_convert_to_fungible_staked_ika)
-  [Function `redeem_fungible_staked_ika`](#(ika_system=0x0)_validator_inner_v1_redeem_fungible_staked_ika)
-  [Function `request_withdraw_stake`](#(ika_system=0x0)_validator_inner_v1_request_withdraw_stake)
-  [Function `request_set_computation_price`](#(ika_system=0x0)_validator_inner_v1_request_set_computation_price)
-  [Function `set_candidate_computation_price`](#(ika_system=0x0)_validator_inner_v1_set_candidate_computation_price)
-  [Function `request_set_commission_rate`](#(ika_system=0x0)_validator_inner_v1_request_set_commission_rate)
-  [Function `set_candidate_commission_rate`](#(ika_system=0x0)_validator_inner_v1_set_candidate_commission_rate)
-  [Function `deposit_stake_rewards`](#(ika_system=0x0)_validator_inner_v1_deposit_stake_rewards)
-  [Function `process_pending_stakes_and_withdraws`](#(ika_system=0x0)_validator_inner_v1_process_pending_stakes_and_withdraws)
-  [Function `is_candidate`](#(ika_system=0x0)_validator_inner_v1_is_candidate)
-  [Function `is_inactive`](#(ika_system=0x0)_validator_inner_v1_is_inactive)
-  [Function `validator_id`](#(ika_system=0x0)_validator_inner_v1_validator_id)
-  [Function `metadata`](#(ika_system=0x0)_validator_inner_v1_metadata)
-  [Function `payment_address`](#(ika_system=0x0)_validator_inner_v1_payment_address)
-  [Function `name`](#(ika_system=0x0)_validator_inner_v1_name)
-  [Function `description`](#(ika_system=0x0)_validator_inner_v1_description)
-  [Function `image_url`](#(ika_system=0x0)_validator_inner_v1_image_url)
-  [Function `project_url`](#(ika_system=0x0)_validator_inner_v1_project_url)
-  [Function `network_address`](#(ika_system=0x0)_validator_inner_v1_network_address)
-  [Function `p2p_address`](#(ika_system=0x0)_validator_inner_v1_p2p_address)
-  [Function `consensus_address`](#(ika_system=0x0)_validator_inner_v1_consensus_address)
-  [Function `protocol_pubkey_bytes`](#(ika_system=0x0)_validator_inner_v1_protocol_pubkey_bytes)
-  [Function `protocol_pubkey`](#(ika_system=0x0)_validator_inner_v1_protocol_pubkey)
-  [Function `proof_of_possession_bytes`](#(ika_system=0x0)_validator_inner_v1_proof_of_possession_bytes)
-  [Function `network_pubkey_bytes`](#(ika_system=0x0)_validator_inner_v1_network_pubkey_bytes)
-  [Function `consensus_pubkey_bytes`](#(ika_system=0x0)_validator_inner_v1_consensus_pubkey_bytes)
-  [Function `class_groups_pubkey_and_proof_bytes`](#(ika_system=0x0)_validator_inner_v1_class_groups_pubkey_and_proof_bytes)
-  [Function `next_epoch_network_address`](#(ika_system=0x0)_validator_inner_v1_next_epoch_network_address)
-  [Function `next_epoch_p2p_address`](#(ika_system=0x0)_validator_inner_v1_next_epoch_p2p_address)
-  [Function `next_epoch_consensus_address`](#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_address)
-  [Function `next_epoch_protocol_pubkey_bytes`](#(ika_system=0x0)_validator_inner_v1_next_epoch_protocol_pubkey_bytes)
-  [Function `next_epoch_proof_of_possession_bytes`](#(ika_system=0x0)_validator_inner_v1_next_epoch_proof_of_possession_bytes)
-  [Function `next_epoch_network_pubkey_bytes`](#(ika_system=0x0)_validator_inner_v1_next_epoch_network_pubkey_bytes)
-  [Function `next_epoch_consensus_pubkey_bytes`](#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_pubkey_bytes)
-  [Function `next_epoch_class_groups_pubkey_and_proof_bytes`](#(ika_system=0x0)_validator_inner_v1_next_epoch_class_groups_pubkey_and_proof_bytes)
-  [Function `operation_cap_id`](#(ika_system=0x0)_validator_inner_v1_operation_cap_id)
-  [Function `next_epoch_computation_price`](#(ika_system=0x0)_validator_inner_v1_next_epoch_computation_price)
-  [Function `total_stake_amount`](#(ika_system=0x0)_validator_inner_v1_total_stake_amount)
-  [Function `pending_stake_amount`](#(ika_system=0x0)_validator_inner_v1_pending_stake_amount)
-  [Function `pending_stake_withdraw_amount`](#(ika_system=0x0)_validator_inner_v1_pending_stake_withdraw_amount)
-  [Function `computation_price`](#(ika_system=0x0)_validator_inner_v1_computation_price)
-  [Function `commission_rate`](#(ika_system=0x0)_validator_inner_v1_commission_rate)
-  [Function `pool_token_exchange_rate_at_epoch`](#(ika_system=0x0)_validator_inner_v1_pool_token_exchange_rate_at_epoch)
-  [Function `is_duplicate`](#(ika_system=0x0)_validator_inner_v1_is_duplicate)
-  [Function `is_equal_some_and_value`](#(ika_system=0x0)_validator_inner_v1_is_equal_some_and_value)
-  [Function `is_equal_some`](#(ika_system=0x0)_validator_inner_v1_is_equal_some)
-  [Function `new_validator_operation_cap`](#(ika_system=0x0)_validator_inner_v1_new_validator_operation_cap)
-  [Function `update_payment_address`](#(ika_system=0x0)_validator_inner_v1_update_payment_address)
-  [Function `update_name`](#(ika_system=0x0)_validator_inner_v1_update_name)
-  [Function `update_description`](#(ika_system=0x0)_validator_inner_v1_update_description)
-  [Function `update_image_url`](#(ika_system=0x0)_validator_inner_v1_update_image_url)
-  [Function `update_project_url`](#(ika_system=0x0)_validator_inner_v1_update_project_url)
-  [Function `update_next_epoch_network_address`](#(ika_system=0x0)_validator_inner_v1_update_next_epoch_network_address)
-  [Function `update_candidate_network_address`](#(ika_system=0x0)_validator_inner_v1_update_candidate_network_address)
-  [Function `update_next_epoch_p2p_address`](#(ika_system=0x0)_validator_inner_v1_update_next_epoch_p2p_address)
-  [Function `update_candidate_p2p_address`](#(ika_system=0x0)_validator_inner_v1_update_candidate_p2p_address)
-  [Function `update_next_epoch_consensus_address`](#(ika_system=0x0)_validator_inner_v1_update_next_epoch_consensus_address)
-  [Function `update_candidate_consensus_address`](#(ika_system=0x0)_validator_inner_v1_update_candidate_consensus_address)
-  [Function `update_next_epoch_protocol_pubkey_bytes`](#(ika_system=0x0)_validator_inner_v1_update_next_epoch_protocol_pubkey_bytes)
-  [Function `update_candidate_protocol_pubkey_bytes`](#(ika_system=0x0)_validator_inner_v1_update_candidate_protocol_pubkey_bytes)
-  [Function `update_next_epoch_network_pubkey_bytes`](#(ika_system=0x0)_validator_inner_v1_update_next_epoch_network_pubkey_bytes)
-  [Function `update_candidate_network_pubkey_bytes`](#(ika_system=0x0)_validator_inner_v1_update_candidate_network_pubkey_bytes)
-  [Function `update_next_epoch_consensus_pubkey_bytes`](#(ika_system=0x0)_validator_inner_v1_update_next_epoch_consensus_pubkey_bytes)
-  [Function `update_next_epoch_class_groups_pubkey_and_proof_bytes`](#(ika_system=0x0)_validator_inner_v1_update_next_epoch_class_groups_pubkey_and_proof_bytes)
-  [Function `update_candidate_consensus_pubkey_bytes`](#(ika_system=0x0)_validator_inner_v1_update_candidate_consensus_pubkey_bytes)
-  [Function `update_candidate_class_groups_pubkey_and_proof_bytes`](#(ika_system=0x0)_validator_inner_v1_update_candidate_class_groups_pubkey_and_proof_bytes)
-  [Function `effectuate_staged_metadata`](#(ika_system=0x0)_validator_inner_v1_effectuate_staged_metadata)
-  [Function `update_class_groups_key_and_proof`](#(ika_system=0x0)_validator_inner_v1_update_class_groups_key_and_proof)
-  [Function `verify_proof_of_possession`](#(ika_system=0x0)_validator_inner_v1_verify_proof_of_possession)
-  [Function `validate_metadata`](#(ika_system=0x0)_validator_inner_v1_validate_metadata)
-  [Function `get_staking_pool_ref`](#(ika_system=0x0)_validator_inner_v1_get_staking_pool_ref)
-  [Function `create_from_metadata`](#(ika_system=0x0)_validator_inner_v1_create_from_metadata)


<pre><code><b>use</b> (ika=0x0)::ika;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof">class_groups_public_key_and_proof</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap">validator_cap</a>;
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
<b>use</b> <a href="../sui/event.md#sui_event">sui::event</a>;
<b>use</b> <a href="../sui/group_ops.md#sui_group_ops">sui::group_ops</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/table.md#sui_table">sui::table</a>;
<b>use</b> <a href="../sui/table_vec.md#sui_table_vec">sui::table_vec</a>;
<b>use</b> <a href="../sui/transfer.md#sui_transfer">sui::transfer</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
<b>use</b> <a href="../sui/types.md#sui_types">sui::types</a>;
<b>use</b> <a href="../sui/url.md#sui_url">sui::url</a>;
<b>use</b> <a href="../sui/vec_set.md#sui_vec_set">sui::vec_set</a>;
</code></pre>



<a name="(ika_system=0x0)_validator_inner_v1_ValidatorMetadata"></a>

## Struct `ValidatorMetadata`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorMetadata">ValidatorMetadata</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_payment_address">payment_address</a>: <b>address</b></code>
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
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey_bytes">protocol_pubkey_bytes</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
 The public key bytes corresponding to the private key that the validator
 holds to sign checkpoint messages.
</dd>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey">protocol_pubkey</a>: <a href="../sui/group_ops.md#sui_group_ops_Element">sui::group_ops::Element</a>&lt;<a href="../sui/bls12381.md#sui_bls12381_UncompressedG1">sui::bls12381::UncompressedG1</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_proof_of_possession_bytes">proof_of_possession_bytes</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
 This is a proof that the validator has ownership of the protocol private key
</dd>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_pubkey_bytes">network_pubkey_bytes</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
 The public key bytes corresponding to the private key that the validator
 uses to establish TLS connections
</dd>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_pubkey_bytes">consensus_pubkey_bytes</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
 The public key bytes correstponding to the consensus
</dd>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_class_groups_pubkey_and_proof_bytes">class_groups_pubkey_and_proof_bytes</a>: <a href="../sui/table_vec.md#sui_table_vec_TableVec">sui::table_vec::TableVec</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
 The validator's Class Groups public key and its associated proof.
 This key is used for the network DKG process and for resharing the network MPC key.
</dd>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_name">name</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 A unique human-readable name of this validator.
</dd>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_description">description</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_image_url">image_url</a>: <a href="../sui/url.md#sui_url_Url">sui::url::Url</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_project_url">project_url</a>: <a href="../sui/url.md#sui_url_Url">sui::url::Url</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_address">network_address</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 The network address of the validator (could also contain extra info such as port, DNS and etc.).
</dd>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_p2p_address">p2p_address</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 The address of the validator used for p2p activities such as state sync (could also contain extra info such as port, DNS and etc.).
</dd>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_address">consensus_address</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 The address of the consensus
</dd>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
 "next_epoch" metadata only takes effects in the next epoch.
 If none, current value will stay unchanged.
</dd>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_proof_of_possession_bytes">next_epoch_proof_of_possession_bytes</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_class_groups_pubkey_and_proof_bytes">next_epoch_class_groups_pubkey_and_proof_bytes</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;(ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_address">next_epoch_network_address</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_p2p_address">next_epoch_p2p_address</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_address">next_epoch_consensus_address</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>extra_fields: <a href="../sui/bag.md#sui_bag_Bag">sui::bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1"></a>

## Struct `ValidatorInnerV1`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>: (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorMetadata">validator_inner_v1::ValidatorMetadata</a></code>
</dt>
<dd>
 Summary of the validator.
</dd>
<dt>
<code>cap_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The ID of this validator's <code>ValidatorCap</code>
</dd>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_operation_cap_id">operation_cap_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The ID of this validator's current valid <code>UnverifiedValidatorOperationCap</code>
</dd>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_computation_price">computation_price</a>: u64</code>
</dt>
<dd>
 Gas price quote, updated only at end of epoch.
</dd>
<dt>
<code><a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a>: (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a></code>
</dt>
<dd>
 Staking pool for this validator.
</dd>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_commission_rate">commission_rate</a>: u16</code>
</dt>
<dd>
 Commission rate of the validator, in basis point.
</dd>
<dt>
<code>next_epoch_stake: u64</code>
</dt>
<dd>
 Total amount of stake that would be active in the next epoch.
</dd>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_computation_price">next_epoch_computation_price</a>: u64</code>
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
<code>extra_fields: <a href="../sui/bag.md#sui_bag_Bag">sui::bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_validator_inner_v1_StakingRequestEvent"></a>

## Struct `StakingRequestEvent`

Event emitted when a new stake request is received.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_StakingRequestEvent">StakingRequestEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>staked_ika_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_validator_inner_v1_UnstakingRequestEvent"></a>

## Struct `UnstakingRequestEvent`

Event emitted when a new unstake request is received.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_UnstakingRequestEvent">UnstakingRequestEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>staked_ika_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>stake_activation_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>unstaking_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>principal_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reward_amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_validator_inner_v1_ConvertingToFungibleStakedIkaEvent"></a>

## Struct `ConvertingToFungibleStakedIkaEvent`

Event emitted when a staked IKA is converted to a fungible staked IKA.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ConvertingToFungibleStakedIkaEvent">ConvertingToFungibleStakedIkaEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>stake_activation_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>staked_ika_principal_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>fungible_staked_ika_amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_validator_inner_v1_RedeemingFungibleStakedIkaEvent"></a>

## Struct `RedeemingFungibleStakedIkaEvent`

Event emitted when a fungible staked IKA is redeemed.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_RedeemingFungibleStakedIkaEvent">RedeemingFungibleStakedIkaEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>fungible_staked_ika_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>ika_amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="(ika_system=0x0)_validator_inner_v1_BLS_KEY_LEN"></a>



<pre><code><b>const</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_BLS_KEY_LEN">BLS_KEY_LEN</a>: u64 = 48;
</code></pre>



<a name="(ika_system=0x0)_validator_inner_v1_CLASS_GROUPS_BYTES_LEN"></a>



<pre><code><b>const</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_CLASS_GROUPS_BYTES_LEN">CLASS_GROUPS_BYTES_LEN</a>: u64 = 241722;
</code></pre>



<a name="(ika_system=0x0)_validator_inner_v1_DEFAULT_EPOCH_ID"></a>



<pre><code><b>const</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_DEFAULT_EPOCH_ID">DEFAULT_EPOCH_ID</a>: u64 = 0;
</code></pre>



<a name="(ika_system=0x0)_validator_inner_v1_ECommissionRateTooHigh"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ECommissionRateTooHigh">ECommissionRateTooHigh</a>: vector&lt;u8&gt; = b"Commission rate set by the <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> is higher than the threshold.";
</code></pre>



<a name="(ika_system=0x0)_validator_inner_v1_ED25519_KEY_LEN"></a>



<pre><code><b>const</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ED25519_KEY_LEN">ED25519_KEY_LEN</a>: u64 = 32;
</code></pre>



<a name="(ika_system=0x0)_validator_inner_v1_EGasPriceHigherThanThreshold"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EGasPriceHigherThanThreshold">EGasPriceHigherThanThreshold</a>: vector&lt;u8&gt; = b"Validator trying to set computation price higher than threshold.";
</code></pre>



<a name="(ika_system=0x0)_validator_inner_v1_EInactiveValidator"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EInactiveValidator">EInactiveValidator</a>: vector&lt;u8&gt; = b"The <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> is inactive.";
</code></pre>



<a name="(ika_system=0x0)_validator_inner_v1_EInvalidCap"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EInvalidCap">EInvalidCap</a>: vector&lt;u8&gt; = b"Cap is not valid.";
</code></pre>



<a name="(ika_system=0x0)_validator_inner_v1_EInvalidProofOfPossession"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EInvalidProofOfPossession">EInvalidProofOfPossession</a>: vector&lt;u8&gt; = b"Invalid <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_proof_of_possession_bytes">proof_of_possession_bytes</a> field in <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorMetadata">ValidatorMetadata</a>.";
</code></pre>



<a name="(ika_system=0x0)_validator_inner_v1_EInvalidStakeAmount"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EInvalidStakeAmount">EInvalidStakeAmount</a>: vector&lt;u8&gt; = b"Stake amount is invalid or wrong.";
</code></pre>



<a name="(ika_system=0x0)_validator_inner_v1_EMetadataInvalidClassGroupsPubkey"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EMetadataInvalidClassGroupsPubkey">EMetadataInvalidClassGroupsPubkey</a>: vector&lt;u8&gt; = b"Invalid <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_class_groups_pubkey_and_proof_bytes">class_groups_pubkey_and_proof_bytes</a> field in <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorMetadata">ValidatorMetadata</a>.";
</code></pre>



<a name="(ika_system=0x0)_validator_inner_v1_EMetadataInvalidConsensusAddress"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EMetadataInvalidConsensusAddress">EMetadataInvalidConsensusAddress</a>: vector&lt;u8&gt; = b"Invalid <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_address">consensus_address</a> field in <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorMetadata">ValidatorMetadata</a>.";
</code></pre>



<a name="(ika_system=0x0)_validator_inner_v1_EMetadataInvalidConsensusPubkey"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EMetadataInvalidConsensusPubkey">EMetadataInvalidConsensusPubkey</a>: vector&lt;u8&gt; = b"Invalid <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_pubkey_bytes">consensus_pubkey_bytes</a> field in <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorMetadata">ValidatorMetadata</a>.";
</code></pre>



<a name="(ika_system=0x0)_validator_inner_v1_EMetadataInvalidNetworkAddress"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EMetadataInvalidNetworkAddress">EMetadataInvalidNetworkAddress</a>: vector&lt;u8&gt; = b"Invalid <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_address">network_address</a> field in <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorMetadata">ValidatorMetadata</a>.";
</code></pre>



<a name="(ika_system=0x0)_validator_inner_v1_EMetadataInvalidNetworkPubkey"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EMetadataInvalidNetworkPubkey">EMetadataInvalidNetworkPubkey</a>: vector&lt;u8&gt; = b"Invalid <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_pubkey_bytes">network_pubkey_bytes</a> field in <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorMetadata">ValidatorMetadata</a>.";
</code></pre>



<a name="(ika_system=0x0)_validator_inner_v1_EMetadataInvalidP2pAddress"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EMetadataInvalidP2pAddress">EMetadataInvalidP2pAddress</a>: vector&lt;u8&gt; = b"Invalid <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_p2p_address">p2p_address</a> field in <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorMetadata">ValidatorMetadata</a>.";
</code></pre>



<a name="(ika_system=0x0)_validator_inner_v1_EMetadataInvalidProtocolPubkey"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EMetadataInvalidProtocolPubkey">EMetadataInvalidProtocolPubkey</a>: vector&lt;u8&gt; = b"Invalid <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey_bytes">protocol_pubkey_bytes</a> field in <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorMetadata">ValidatorMetadata</a>.";
</code></pre>



<a name="(ika_system=0x0)_validator_inner_v1_ENewCapNotCreatedByValidatorItself"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ENewCapNotCreatedByValidatorItself">ENewCapNotCreatedByValidatorItself</a>: vector&lt;u8&gt; = b"New Capability is not created by the <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> itself.";
</code></pre>



<a name="(ika_system=0x0)_validator_inner_v1_ENotValidatorCandidate"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ENotValidatorCandidate">ENotValidatorCandidate</a>: vector&lt;u8&gt; = b"Intended <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> is not a candidate one.";
</code></pre>



<a name="(ika_system=0x0)_validator_inner_v1_EValidatorMetadataExceedingLengthLimit"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>: vector&lt;u8&gt; = b"Validator Metadata is too long.";
</code></pre>



<a name="(ika_system=0x0)_validator_inner_v1_MAX_COMMISSION_RATE"></a>



<pre><code><b>const</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_MAX_COMMISSION_RATE">MAX_COMMISSION_RATE</a>: u16 = 2000;
</code></pre>



<a name="(ika_system=0x0)_validator_inner_v1_MAX_VALIDATOR_COMPUTATION_PRICE"></a>

Max computation price a validator can set is 100K NIKA.


<pre><code><b>const</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_MAX_VALIDATOR_COMPUTATION_PRICE">MAX_VALIDATOR_COMPUTATION_PRICE</a>: u64 = 100000;
</code></pre>



<a name="(ika_system=0x0)_validator_inner_v1_MAX_VALIDATOR_METADATA_LENGTH"></a>



<pre><code><b>const</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>: u64 = 256;
</code></pre>



<a name="(ika_system=0x0)_validator_inner_v1_PROOF_OF_POSSESSION_INTENT"></a>



<pre><code><b>const</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_PROOF_OF_POSSESSION_INTENT">PROOF_OF_POSSESSION_INTENT</a>: vector&lt;u8&gt; = vector[0, 0, 0];
</code></pre>



<a name="(ika_system=0x0)_validator_inner_v1_create_metadata"></a>

## Function `create_metadata`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_create_metadata">create_metadata</a>(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_payment_address">payment_address</a>: <b>address</b>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey_bytes">protocol_pubkey_bytes</a>: vector&lt;u8&gt;, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_pubkey_bytes">network_pubkey_bytes</a>: vector&lt;u8&gt;, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_pubkey_bytes">consensus_pubkey_bytes</a>: vector&lt;u8&gt;, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_class_groups_pubkey_and_proof_bytes">class_groups_pubkey_and_proof_bytes</a>: (ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_proof_of_possession_bytes">proof_of_possession_bytes</a>: vector&lt;u8&gt;, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_name">name</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_description">description</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_image_url">image_url</a>: <a href="../sui/url.md#sui_url_Url">sui::url::Url</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_project_url">project_url</a>: <a href="../sui/url.md#sui_url_Url">sui::url::Url</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_address">network_address</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_p2p_address">p2p_address</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_address">consensus_address</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, extra_fields: <a href="../sui/bag.md#sui_bag_Bag">sui::bag::Bag</a>, ctx: &<a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorMetadata">validator_inner_v1::ValidatorMetadata</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_create_metadata">create_metadata</a>(
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_payment_address">payment_address</a>: <b>address</b>,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey_bytes">protocol_pubkey_bytes</a>: vector&lt;u8&gt;,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_pubkey_bytes">network_pubkey_bytes</a>: vector&lt;u8&gt;,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_pubkey_bytes">consensus_pubkey_bytes</a>: vector&lt;u8&gt;,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_class_groups_pubkey_and_proof_bytes">class_groups_pubkey_and_proof_bytes</a>: ClassGroupsPublicKeyAndProof,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_proof_of_possession_bytes">proof_of_possession_bytes</a>: vector&lt;u8&gt;,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_name">name</a>: String,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_description">description</a>: String,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_image_url">image_url</a>: Url,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_project_url">project_url</a>: Url,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_address">network_address</a>: String,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_p2p_address">p2p_address</a>: String,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_address">consensus_address</a>: String,
    extra_fields: Bag,
    ctx: &TxContext,
): <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorMetadata">ValidatorMetadata</a> {
    <b>let</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey">protocol_pubkey</a> = g1_to_uncompressed_g1(&g1_from_bytes(&<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey_bytes">protocol_pubkey_bytes</a>));
    <b>let</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_class_groups_pubkey_and_proof_bytes">class_groups_pubkey_and_proof_bytes</a> = <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_class_groups_pubkey_and_proof_bytes">class_groups_pubkey_and_proof_bytes</a>.destroy();
    <b>let</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a> = <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorMetadata">ValidatorMetadata</a> {
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_payment_address">payment_address</a>,
        proof_of_possession_sender: ctx.sender(),
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey_bytes">protocol_pubkey_bytes</a>,
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey">protocol_pubkey</a>,
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_pubkey_bytes">network_pubkey_bytes</a>,
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_pubkey_bytes">consensus_pubkey_bytes</a>,
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_class_groups_pubkey_and_proof_bytes">class_groups_pubkey_and_proof_bytes</a>,
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_proof_of_possession_bytes">proof_of_possession_bytes</a>,
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_name">name</a>,
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_description">description</a>,
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_image_url">image_url</a>,
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_project_url">project_url</a>,
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_address">network_address</a>,
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_p2p_address">p2p_address</a>,
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_address">consensus_address</a>,
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>: option::none(),
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>: option::none(),
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>: option::none(),
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_class_groups_pubkey_and_proof_bytes">next_epoch_class_groups_pubkey_and_proof_bytes</a>: option::none(),
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_proof_of_possession_bytes">next_epoch_proof_of_possession_bytes</a>: option::none(),
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_address">next_epoch_network_address</a>: option::none(),
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_p2p_address">next_epoch_p2p_address</a>: option::none(),
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_address">next_epoch_consensus_address</a>: option::none(),
        extra_fields,
    };
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_create"></a>

## Function `create`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_create">create</a>(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, cap_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_operation_cap_id">operation_cap_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_payment_address">payment_address</a>: <b>address</b>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey_bytes">protocol_pubkey_bytes</a>: vector&lt;u8&gt;, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_pubkey_bytes">network_pubkey_bytes</a>: vector&lt;u8&gt;, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_pubkey_bytes">consensus_pubkey_bytes</a>: vector&lt;u8&gt;, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_class_groups_pubkey_and_proof_bytes">class_groups_pubkey_and_proof_bytes</a>: (ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_proof_of_possession_bytes">proof_of_possession_bytes</a>: vector&lt;u8&gt;, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_name">name</a>: vector&lt;u8&gt;, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_description">description</a>: vector&lt;u8&gt;, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_image_url">image_url</a>: vector&lt;u8&gt;, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_project_url">project_url</a>: vector&lt;u8&gt;, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_address">network_address</a>: vector&lt;u8&gt;, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_p2p_address">p2p_address</a>: vector&lt;u8&gt;, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_address">consensus_address</a>: vector&lt;u8&gt;, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_computation_price">computation_price</a>: u64, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_commission_rate">commission_rate</a>: u16, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_create">create</a>(
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>: ID,
    cap_id: ID,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_operation_cap_id">operation_cap_id</a>: ID,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_payment_address">payment_address</a>: <b>address</b>,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey_bytes">protocol_pubkey_bytes</a>: vector&lt;u8&gt;,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_pubkey_bytes">network_pubkey_bytes</a>: vector&lt;u8&gt;,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_pubkey_bytes">consensus_pubkey_bytes</a>: vector&lt;u8&gt;,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_class_groups_pubkey_and_proof_bytes">class_groups_pubkey_and_proof_bytes</a>: ClassGroupsPublicKeyAndProof,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_proof_of_possession_bytes">proof_of_possession_bytes</a>: vector&lt;u8&gt;,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_name">name</a>: vector&lt;u8&gt;,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_description">description</a>: vector&lt;u8&gt;,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_image_url">image_url</a>: vector&lt;u8&gt;,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_project_url">project_url</a>: vector&lt;u8&gt;,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_address">network_address</a>: vector&lt;u8&gt;,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_p2p_address">p2p_address</a>: vector&lt;u8&gt;,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_address">consensus_address</a>: vector&lt;u8&gt;,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_computation_price">computation_price</a>: u64,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_commission_rate">commission_rate</a>: u16,
    ctx: &<b>mut</b> TxContext,
): <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a> {
    <b>assert</b>!(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_commission_rate">commission_rate</a> &lt;= <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_MAX_COMMISSION_RATE">MAX_COMMISSION_RATE</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ECommissionRateTooHigh">ECommissionRateTooHigh</a>);
    <b>assert</b>!(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_computation_price">computation_price</a> &lt; <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_MAX_VALIDATOR_COMPUTATION_PRICE">MAX_VALIDATOR_COMPUTATION_PRICE</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EGasPriceHigherThanThreshold">EGasPriceHigherThanThreshold</a>);
    <b>let</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a> = <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_create_metadata">create_metadata</a>(
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_payment_address">payment_address</a>,
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey_bytes">protocol_pubkey_bytes</a>,
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_pubkey_bytes">network_pubkey_bytes</a>,
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_pubkey_bytes">consensus_pubkey_bytes</a>,
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_class_groups_pubkey_and_proof_bytes">class_groups_pubkey_and_proof_bytes</a>,
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_proof_of_possession_bytes">proof_of_possession_bytes</a>,
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_name">name</a>.to_ascii_string().to_string(),
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_description">description</a>.to_ascii_string().to_string(),
        url::new_unsafe_from_bytes(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_image_url">image_url</a>),
        url::new_unsafe_from_bytes(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_project_url">project_url</a>),
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_address">network_address</a>.to_ascii_string().to_string(),
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_p2p_address">p2p_address</a>.to_ascii_string().to_string(),
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_address">consensus_address</a>.to_ascii_string().to_string(),
        bag::new(ctx),
        ctx,
    );
    // Checks that the keys & addresses & PoP are valid.
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validate_metadata">validate_metadata</a>(&<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>);
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_create_from_metadata">create_from_metadata</a>(
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>,
        cap_id,
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_operation_cap_id">operation_cap_id</a>,
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>,
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_computation_price">computation_price</a>,
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_commission_rate">commission_rate</a>,
        ctx,
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_deactivate"></a>

## Function `deactivate`

Deactivate this validator's staking pool


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_deactivate">deactivate</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, deactivation_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_deactivate">deactivate</a>(self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>, deactivation_epoch: u64) {
    self.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a>.deactivate_staking_pool(deactivation_epoch)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_activate"></a>

## Function `activate`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_activate">activate</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, activation_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_activate">activate</a>(self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>, activation_epoch: u64) {
    self.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a>.activate_staking_pool(activation_epoch);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_adjust_stake_and_computation_price"></a>

## Function `adjust_stake_and_computation_price`

Process pending stake and pending withdraws, and update the computation price.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_adjust_stake_and_computation_price">adjust_stake_and_computation_price</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_adjust_stake_and_computation_price">adjust_stake_and_computation_price</a>(self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>) {
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_computation_price">computation_price</a> = self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_computation_price">next_epoch_computation_price</a>;
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_commission_rate">commission_rate</a> = self.next_epoch_commission_rate;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_request_add_stake"></a>

## Function `request_add_stake`

Request to add stake to the validator's staking pool, processed at the end of the epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_request_add_stake">request_add_stake</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, epoch: u64, stake: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_request_add_stake">request_add_stake</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    epoch: u64,
    stake: Balance&lt;IKA&gt;,
    ctx: &<b>mut</b> TxContext,
): StakedIka {
    <b>let</b> stake_amount = stake.value();
    <b>let</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a> = self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>();
    <b>assert</b>!(stake_amount &gt; 0, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EInvalidStakeAmount">EInvalidStakeAmount</a>);
    <b>let</b> stake_epoch = epoch + 1;
    <b>let</b> <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a> = self.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_request_add_stake">request_add_stake</a>(stake, stake_epoch, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>, ctx);
    // Process stake right away <b>if</b> staking pool is preactive.
    <b>if</b> (self.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_candidate">is_candidate</a>()) {
        self.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a>.process_pending_stake();
    };
    self.next_epoch_stake = self.next_epoch_stake + stake_amount;
    event::emit(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_StakingRequestEvent">StakingRequestEvent</a> {
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>,
        staked_ika_id: object::id(&<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>),
        epoch: epoch,
        amount: stake_amount,
    });
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_convert_to_fungible_staked_ika"></a>

## Function `convert_to_fungible_staked_ika`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_convert_to_fungible_staked_ika">convert_to_fungible_staked_ika</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, epoch: u64, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_convert_to_fungible_staked_ika">convert_to_fungible_staked_ika</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    epoch: u64,
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: StakedIka,
    ctx: &<b>mut</b> TxContext,
): FungibleStakedIka {
    <b>let</b> stake_activation_epoch = <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.stake_activation_epoch();
    <b>let</b> staked_ika_principal_amount = <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.staked_ika_amount();
    <b>let</b> fungible_staked_ika = self.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_convert_to_fungible_staked_ika">convert_to_fungible_staked_ika</a>(epoch, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>, ctx);
    event::emit(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ConvertingToFungibleStakedIkaEvent">ConvertingToFungibleStakedIkaEvent</a> {
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>: self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>(),
        stake_activation_epoch,
        staked_ika_principal_amount,
        fungible_staked_ika_amount: fungible_staked_ika.value(),
    });
    fungible_staked_ika
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_redeem_fungible_staked_ika"></a>

## Function `redeem_fungible_staked_ika`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_redeem_fungible_staked_ika">redeem_fungible_staked_ika</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, epoch: u64, fungible_staked_ika: (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_FungibleStakedIka">staked_ika::FungibleStakedIka</a>): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_redeem_fungible_staked_ika">redeem_fungible_staked_ika</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    epoch: u64,
    fungible_staked_ika: FungibleStakedIka,
): Balance&lt;IKA&gt; {
    <b>let</b> fungible_staked_ika_amount = fungible_staked_ika.value();
    <b>let</b> ika = self.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_redeem_fungible_staked_ika">redeem_fungible_staked_ika</a>(epoch, fungible_staked_ika);
    self.next_epoch_stake = self.next_epoch_stake - ika.value();
    event::emit(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_RedeemingFungibleStakedIkaEvent">RedeemingFungibleStakedIkaEvent</a> {
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>: self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>(),
        fungible_staked_ika_amount,
        ika_amount: ika.value(),
    });
    ika
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_request_withdraw_stake"></a>

## Function `request_withdraw_stake`

Request to withdraw stake from the validator's staking pool, processed at the end of the epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_request_withdraw_stake">request_withdraw_stake</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, epoch: u64, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: (ika_system=0x0)::<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika_StakedIka">staked_ika::StakedIka</a>): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_request_withdraw_stake">request_withdraw_stake</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    epoch: u64,
    <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>: StakedIka,
): Balance&lt;IKA&gt; {
    <b>let</b> principal_amount = <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.staked_ika_amount();
    <b>let</b> stake_activation_epoch = <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>.stake_activation_epoch();
    <b>let</b> staked_ika_id = object::id(&<a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>);
    <b>let</b> withdrawn_stake = self.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_request_withdraw_stake">request_withdraw_stake</a>(epoch, <a href="../ika_system/staked_ika.md#(ika_system=0x0)_staked_ika">staked_ika</a>);
    <b>let</b> withdraw_amount = withdrawn_stake.value();
    <b>let</b> reward_amount = withdraw_amount - principal_amount;
    self.next_epoch_stake = self.next_epoch_stake - withdraw_amount;
    event::emit(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_UnstakingRequestEvent">UnstakingRequestEvent</a> {
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>: self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>(),
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

<a name="(ika_system=0x0)_validator_inner_v1_request_set_computation_price"></a>

## Function `request_set_computation_price`

Request to set new computation price for the next epoch.
Need to present a <code>ValidatorOperationCap</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_request_set_computation_price">request_set_computation_price</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, operation_cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, new_price: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_request_set_computation_price">request_set_computation_price</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    operation_cap: &ValidatorOperationCap,
    new_price: u64,
) {
    <b>assert</b>!(!<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_inactive">is_inactive</a>(self), <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EInactiveValidator">EInactiveValidator</a>);
    <b>assert</b>!(new_price &lt; <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_MAX_VALIDATOR_COMPUTATION_PRICE">MAX_VALIDATOR_COMPUTATION_PRICE</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EGasPriceHigherThanThreshold">EGasPriceHigherThanThreshold</a>);
    <b>let</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a> = operation_cap.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>();
    <b>assert</b>!(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a> == self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>(), <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EInvalidCap">EInvalidCap</a>);
    <b>assert</b>!(object::id(operation_cap) == self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_operation_cap_id">operation_cap_id</a>(), <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EInvalidCap">EInvalidCap</a>);
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_computation_price">next_epoch_computation_price</a> = new_price;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_set_candidate_computation_price"></a>

## Function `set_candidate_computation_price`

Set new computation price for the candidate validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_set_candidate_computation_price">set_candidate_computation_price</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, operation_cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, new_price: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_set_candidate_computation_price">set_candidate_computation_price</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    operation_cap: &ValidatorOperationCap,
    new_price: u64,
) {
    <b>assert</b>!(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_candidate">is_candidate</a>(self), <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    <b>assert</b>!(new_price &lt; <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_MAX_VALIDATOR_COMPUTATION_PRICE">MAX_VALIDATOR_COMPUTATION_PRICE</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EGasPriceHigherThanThreshold">EGasPriceHigherThanThreshold</a>);
    <b>let</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a> = operation_cap.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>();
    <b>assert</b>!(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a> == self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>(), <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EInvalidCap">EInvalidCap</a>);
    <b>assert</b>!(object::id(operation_cap) == self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_operation_cap_id">operation_cap_id</a>(), <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EInvalidCap">EInvalidCap</a>);
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_computation_price">next_epoch_computation_price</a> = new_price;
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_computation_price">computation_price</a> = new_price;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_request_set_commission_rate"></a>

## Function `request_set_commission_rate`

Request to set new commission rate for the next epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_request_set_commission_rate">request_set_commission_rate</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, new_commission_rate: u16)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_request_set_commission_rate">request_set_commission_rate</a>(self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>, new_commission_rate: u16) {
    <b>assert</b>!(!<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_inactive">is_inactive</a>(self), <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EInactiveValidator">EInactiveValidator</a>);
    <b>assert</b>!(new_commission_rate &lt;= <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_MAX_COMMISSION_RATE">MAX_COMMISSION_RATE</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ECommissionRateTooHigh">ECommissionRateTooHigh</a>);
    self.next_epoch_commission_rate = new_commission_rate;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_set_candidate_commission_rate"></a>

## Function `set_candidate_commission_rate`

Set new commission rate for the candidate validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_set_candidate_commission_rate">set_candidate_commission_rate</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, new_commission_rate: u16)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_set_candidate_commission_rate">set_candidate_commission_rate</a>(self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>, new_commission_rate: u16) {
    <b>assert</b>!(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_candidate">is_candidate</a>(self), <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    <b>assert</b>!(new_commission_rate &lt;= <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_MAX_COMMISSION_RATE">MAX_COMMISSION_RATE</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ECommissionRateTooHigh">ECommissionRateTooHigh</a>);
    self.next_epoch_commission_rate = new_commission_rate;
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_commission_rate">commission_rate</a> = new_commission_rate;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_deposit_stake_rewards"></a>

## Function `deposit_stake_rewards`

Deposit stakes rewards into the validator's staking pool, called at the end of the epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_deposit_stake_rewards">deposit_stake_rewards</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, reward: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_deposit_stake_rewards">deposit_stake_rewards</a>(self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>, reward: Balance&lt;IKA&gt;) {
    self.next_epoch_stake = self.next_epoch_stake + reward.value();
    self.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a>.deposit_rewards(reward);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_process_pending_stakes_and_withdraws"></a>

## Function `process_pending_stakes_and_withdraws`

Process pending stakes and withdraws, called at the end of the epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, new_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>(self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>, new_epoch: u64) {
    self.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>(new_epoch);
    // TODO: bring this assertion back when we are ready.
    <b>assert</b>!(self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_total_stake_amount">total_stake_amount</a>() == self.next_epoch_stake, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EInvalidStakeAmount">EInvalidStakeAmount</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_is_candidate"></a>

## Function `is_candidate`

Returns true if the validator is candidate.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_candidate">is_candidate</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_candidate">is_candidate</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): bool {
    self.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_candidate">is_candidate</a>()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_is_inactive"></a>

## Function `is_inactive`

Returns true if the validator is inactive.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_inactive">is_inactive</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_inactive">is_inactive</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): bool {
    self.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_inactive">is_inactive</a>()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_validator_id"></a>

## Function `validator_id`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): ID {
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_metadata"></a>

## Function `metadata`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorMetadata">validator_inner_v1::ValidatorMetadata</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorMetadata">ValidatorMetadata</a> {
    &self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_payment_address"></a>

## Function `payment_address`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_payment_address">payment_address</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_payment_address">payment_address</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): <b>address</b> {
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_payment_address">payment_address</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_name"></a>

## Function `name`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_name">name</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_name">name</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &String {
    &self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_name">name</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_description"></a>

## Function `description`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_description">description</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_description">description</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &String {
    &self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_description">description</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_image_url"></a>

## Function `image_url`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_image_url">image_url</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../sui/url.md#sui_url_Url">sui::url::Url</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_image_url">image_url</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &Url {
    &self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_image_url">image_url</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_project_url"></a>

## Function `project_url`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_project_url">project_url</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../sui/url.md#sui_url_Url">sui::url::Url</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_project_url">project_url</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &Url {
    &self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_project_url">project_url</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_network_address"></a>

## Function `network_address`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_address">network_address</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_address">network_address</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &String {
    &self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_address">network_address</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_p2p_address"></a>

## Function `p2p_address`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_p2p_address">p2p_address</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_p2p_address">p2p_address</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &String {
    &self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_p2p_address">p2p_address</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_consensus_address"></a>

## Function `consensus_address`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_address">consensus_address</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_address">consensus_address</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &String {
    &self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_address">consensus_address</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_protocol_pubkey_bytes"></a>

## Function `protocol_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey_bytes">protocol_pubkey_bytes</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey_bytes">protocol_pubkey_bytes</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &vector&lt;u8&gt; {
    &self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey_bytes">protocol_pubkey_bytes</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_protocol_pubkey"></a>

## Function `protocol_pubkey`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey">protocol_pubkey</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../sui/group_ops.md#sui_group_ops_Element">sui::group_ops::Element</a>&lt;<a href="../sui/bls12381.md#sui_bls12381_UncompressedG1">sui::bls12381::UncompressedG1</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey">protocol_pubkey</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &Element&lt;UncompressedG1&gt; {
    &self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey">protocol_pubkey</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_proof_of_possession_bytes"></a>

## Function `proof_of_possession_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_proof_of_possession_bytes">proof_of_possession_bytes</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_proof_of_possession_bytes">proof_of_possession_bytes</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &vector&lt;u8&gt; {
    &self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_proof_of_possession_bytes">proof_of_possession_bytes</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_network_pubkey_bytes"></a>

## Function `network_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_pubkey_bytes">network_pubkey_bytes</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_pubkey_bytes">network_pubkey_bytes</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &vector&lt;u8&gt; {
    &self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_pubkey_bytes">network_pubkey_bytes</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_consensus_pubkey_bytes"></a>

## Function `consensus_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_pubkey_bytes">consensus_pubkey_bytes</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_pubkey_bytes">consensus_pubkey_bytes</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &vector&lt;u8&gt; {
    &self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_pubkey_bytes">consensus_pubkey_bytes</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_class_groups_pubkey_and_proof_bytes"></a>

## Function `class_groups_pubkey_and_proof_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_class_groups_pubkey_and_proof_bytes">class_groups_pubkey_and_proof_bytes</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../sui/table_vec.md#sui_table_vec_TableVec">sui::table_vec::TableVec</a>&lt;vector&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_class_groups_pubkey_and_proof_bytes">class_groups_pubkey_and_proof_bytes</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &TableVec&lt;vector&lt;u8&gt;&gt; {
    &self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_class_groups_pubkey_and_proof_bytes">class_groups_pubkey_and_proof_bytes</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_next_epoch_network_address"></a>

## Function `next_epoch_network_address`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_address">next_epoch_network_address</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_address">next_epoch_network_address</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &Option&lt;String&gt; {
    &self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_address">next_epoch_network_address</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_next_epoch_p2p_address"></a>

## Function `next_epoch_p2p_address`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_p2p_address">next_epoch_p2p_address</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_p2p_address">next_epoch_p2p_address</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &Option&lt;String&gt; {
    &self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_p2p_address">next_epoch_p2p_address</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_address"></a>

## Function `next_epoch_consensus_address`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_address">next_epoch_consensus_address</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_address">next_epoch_consensus_address</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &Option&lt;String&gt; {
    &self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_address">next_epoch_consensus_address</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_next_epoch_protocol_pubkey_bytes"></a>

## Function `next_epoch_protocol_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &Option&lt;vector&lt;u8&gt;&gt; {
    &self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_next_epoch_proof_of_possession_bytes"></a>

## Function `next_epoch_proof_of_possession_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_proof_of_possession_bytes">next_epoch_proof_of_possession_bytes</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_proof_of_possession_bytes">next_epoch_proof_of_possession_bytes</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &Option&lt;vector&lt;u8&gt;&gt; {
    &self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_proof_of_possession_bytes">next_epoch_proof_of_possession_bytes</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_next_epoch_network_pubkey_bytes"></a>

## Function `next_epoch_network_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &Option&lt;vector&lt;u8&gt;&gt; {
    &self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_pubkey_bytes"></a>

## Function `next_epoch_consensus_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &Option&lt;vector&lt;u8&gt;&gt; {
    &self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_next_epoch_class_groups_pubkey_and_proof_bytes"></a>

## Function `next_epoch_class_groups_pubkey_and_proof_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_class_groups_pubkey_and_proof_bytes">next_epoch_class_groups_pubkey_and_proof_bytes</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;(ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_class_groups_pubkey_and_proof_bytes">next_epoch_class_groups_pubkey_and_proof_bytes</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &Option&lt;ClassGroupsPublicKeyAndProof&gt; {
    &self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_class_groups_pubkey_and_proof_bytes">next_epoch_class_groups_pubkey_and_proof_bytes</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_operation_cap_id"></a>

## Function `operation_cap_id`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_operation_cap_id">operation_cap_id</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_operation_cap_id">operation_cap_id</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &ID {
    &self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_operation_cap_id">operation_cap_id</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_next_epoch_computation_price"></a>

## Function `next_epoch_computation_price`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_computation_price">next_epoch_computation_price</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_computation_price">next_epoch_computation_price</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): u64 {
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_computation_price">next_epoch_computation_price</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_total_stake_amount"></a>

## Function `total_stake_amount`

Return the total amount staked with this validator


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_total_stake_amount">total_stake_amount</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_total_stake_amount">total_stake_amount</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): u64 {
    self.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a>.ika_balance()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_pending_stake_amount"></a>

## Function `pending_stake_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_pending_stake_amount">pending_stake_amount</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_pending_stake_amount">pending_stake_amount</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): u64 {
    self.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_pending_stake_amount">pending_stake_amount</a>()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_pending_stake_withdraw_amount"></a>

## Function `pending_stake_withdraw_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_pending_stake_withdraw_amount">pending_stake_withdraw_amount</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_pending_stake_withdraw_amount">pending_stake_withdraw_amount</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): u64 {
    self.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_pending_stake_withdraw_amount">pending_stake_withdraw_amount</a>()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_computation_price"></a>

## Function `computation_price`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_computation_price">computation_price</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_computation_price">computation_price</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): u64 {
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_computation_price">computation_price</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_commission_rate"></a>

## Function `commission_rate`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_commission_rate">commission_rate</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): u16
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_commission_rate">commission_rate</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): u16 {
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_commission_rate">commission_rate</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_pool_token_exchange_rate_at_epoch"></a>

## Function `pool_token_exchange_rate_at_epoch`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, epoch: u64): (ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>, epoch: u64): PoolTokenExchangeRate {
    self.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(epoch)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_is_duplicate"></a>

## Function `is_duplicate`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_duplicate">is_duplicate</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, other: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_duplicate">is_duplicate</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>, other: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): bool {
            self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_name">name</a> == other.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_name">name</a>
            || self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_address">network_address</a> == other.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_address">network_address</a>
            || self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_p2p_address">p2p_address</a> == other.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_p2p_address">p2p_address</a>
            || self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey_bytes">protocol_pubkey_bytes</a> == other.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey_bytes">protocol_pubkey_bytes</a>
            || self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_pubkey_bytes">network_pubkey_bytes</a> == other.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_pubkey_bytes">network_pubkey_bytes</a>
            || self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_pubkey_bytes">network_pubkey_bytes</a> == other.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_pubkey_bytes">consensus_pubkey_bytes</a>
            || self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_pubkey_bytes">consensus_pubkey_bytes</a> == other.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_pubkey_bytes">consensus_pubkey_bytes</a>
            || self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_pubkey_bytes">consensus_pubkey_bytes</a> == other.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_pubkey_bytes">network_pubkey_bytes</a>
            // All next epoch parameters.
            || <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_equal_some">is_equal_some</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_address">next_epoch_network_address</a>, &other.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_address">next_epoch_network_address</a>)
            || <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_equal_some">is_equal_some</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_p2p_address">next_epoch_p2p_address</a>, &other.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_p2p_address">next_epoch_p2p_address</a>)
            || <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_equal_some">is_equal_some</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>, &other.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>)
            || <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_equal_some">is_equal_some</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>, &other.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>)
            || <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_equal_some">is_equal_some</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>, &other.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>)
            || <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_equal_some">is_equal_some</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>, &other.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>)
            || <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_equal_some">is_equal_some</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>, &other.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>)
            // My next epoch parameters with other current epoch parameters.
            || <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_address">next_epoch_network_address</a>, &other.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_address">network_address</a>)
            || <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_p2p_address">next_epoch_p2p_address</a>, &other.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_p2p_address">p2p_address</a>)
            || <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>, &other.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey_bytes">protocol_pubkey_bytes</a>)
            || <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>, &other.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_pubkey_bytes">network_pubkey_bytes</a>)
            || <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>, &other.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_pubkey_bytes">consensus_pubkey_bytes</a>)
            || <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>, &other.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_pubkey_bytes">consensus_pubkey_bytes</a>)
            || <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>, &other.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_pubkey_bytes">network_pubkey_bytes</a>)
            // Other next epoch parameters with my current epoch parameters.
            || <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>(&other.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_address">next_epoch_network_address</a>, &self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_address">network_address</a>)
            || <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>(&other.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_p2p_address">next_epoch_p2p_address</a>, &self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_p2p_address">p2p_address</a>)
            || <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>(&other.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>, &self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey_bytes">protocol_pubkey_bytes</a>)
            || <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>(&other.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>, &self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_pubkey_bytes">network_pubkey_bytes</a>)
            || <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>(&other.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>, &self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_pubkey_bytes">consensus_pubkey_bytes</a>)
            || <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>(&other.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>, &self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_pubkey_bytes">consensus_pubkey_bytes</a>)
            || <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>(&other.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>, &self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_pubkey_bytes">network_pubkey_bytes</a>)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_is_equal_some_and_value"></a>

## Function `is_equal_some_and_value`



<pre><code><b>fun</b> is_equal_some_and_valueT(a: &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;T&gt;, b: &T): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_equal_some_and_value">is_equal_some_and_value</a>&lt;T&gt;(a: &Option&lt;T&gt;, b: &T): bool {
    <b>if</b> (a.is_none()) {
        <b>false</b>
    } <b>else</b> {
        a.borrow() == b
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_is_equal_some"></a>

## Function `is_equal_some`



<pre><code><b>fun</b> is_equal_someT(a: &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;T&gt;, b: &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;T&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_equal_some">is_equal_some</a>&lt;T&gt;(a: &Option&lt;T&gt;, b: &Option&lt;T&gt;): bool {
    <b>if</b> (a.is_none() || b.is_none()) {
        <b>false</b>
    } <b>else</b> {
        a.borrow() == b.borrow()
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_new_validator_operation_cap"></a>

## Function `new_validator_operation_cap`

Create a new <code>ValidatorOperationCap</code>, and registers it,
thus revoking the previous cap's permission.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_new_validator_operation_cap">new_validator_operation_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, cap: &(ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorCap">validator_cap::ValidatorCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_new_validator_operation_cap">new_validator_operation_cap</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    cap: &ValidatorCap,
    ctx: &<b>mut</b> TxContext,
): ValidatorOperationCap {
    <b>let</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a> = cap.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>();
    <b>assert</b>!(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a> == self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>(), <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ENewCapNotCreatedByValidatorItself">ENewCapNotCreatedByValidatorItself</a>);
    <b>let</b> operation_cap = <a href="../ika_system/validator_cap.md#(ika_system=0x0)_validator_cap_new_validator_operation_cap">validator_cap::new_validator_operation_cap</a>(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>, ctx);
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_operation_cap_id">operation_cap_id</a> = object::id(&operation_cap);
    operation_cap
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_update_payment_address"></a>

## Function `update_payment_address`

Update payment address of the validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_payment_address">update_payment_address</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_payment_address">payment_address</a>: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_payment_address">update_payment_address</a>(self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_payment_address">payment_address</a>: <b>address</b>) {
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_payment_address">payment_address</a> = <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_payment_address">payment_address</a>;
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_update_name"></a>

## Function `update_name`

Update name of the validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_name">update_name</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_name">name</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_name">update_name</a>(self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_name">name</a>: vector&lt;u8&gt;) {
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_name">name</a> = <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_name">name</a>.to_ascii_string().to_string();
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_update_description"></a>

## Function `update_description`

Update description of the validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_description">update_description</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_description">description</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_description">update_description</a>(self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_description">description</a>: vector&lt;u8&gt;) {
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_description">description</a> = <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_description">description</a>.to_ascii_string().to_string();
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_update_image_url"></a>

## Function `update_image_url`

Update image url of the validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_image_url">update_image_url</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_image_url">image_url</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_image_url">update_image_url</a>(self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_image_url">image_url</a>: vector&lt;u8&gt;) {
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_image_url">image_url</a> = url::new_unsafe_from_bytes(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_image_url">image_url</a>);
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_update_project_url"></a>

## Function `update_project_url`

Update project url of the validator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_project_url">update_project_url</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_project_url">project_url</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_project_url">update_project_url</a>(self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_project_url">project_url</a>: vector&lt;u8&gt;) {
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_project_url">project_url</a> = url::new_unsafe_from_bytes(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_project_url">project_url</a>);
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_update_next_epoch_network_address"></a>

## Function `update_next_epoch_network_address`

Update network address of this validator, taking effects from next epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_next_epoch_network_address">update_next_epoch_network_address</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_address">network_address</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_next_epoch_network_address">update_next_epoch_network_address</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_address">network_address</a>: vector&lt;u8&gt;,
) {
    <b>assert</b>!(!<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_inactive">is_inactive</a>(self), <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EInactiveValidator">EInactiveValidator</a>);
    <b>let</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_address">network_address</a> = <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_address">network_address</a>.to_ascii_string().to_string();
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_address">next_epoch_network_address</a> = option::some(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_address">network_address</a>);
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_update_candidate_network_address"></a>

## Function `update_candidate_network_address`

Update network address of this candidate validator


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_candidate_network_address">update_candidate_network_address</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_address">network_address</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_candidate_network_address">update_candidate_network_address</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_address">network_address</a>: vector&lt;u8&gt;,
) {
    <b>assert</b>!(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_candidate">is_candidate</a>(self), <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    <b>let</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_address">network_address</a> = <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_address">network_address</a>.to_ascii_string().to_string();
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_address">network_address</a> = <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_address">network_address</a>;
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_update_next_epoch_p2p_address"></a>

## Function `update_next_epoch_p2p_address`

Update p2p address of this validator, taking effects from next epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_next_epoch_p2p_address">update_next_epoch_p2p_address</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_p2p_address">p2p_address</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_next_epoch_p2p_address">update_next_epoch_p2p_address</a>(self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_p2p_address">p2p_address</a>: vector&lt;u8&gt;) {
    <b>assert</b>!(!<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_inactive">is_inactive</a>(self), <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EInactiveValidator">EInactiveValidator</a>);
    <b>let</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_p2p_address">p2p_address</a> = <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_p2p_address">p2p_address</a>.to_ascii_string().to_string();
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_p2p_address">next_epoch_p2p_address</a> = option::some(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_p2p_address">p2p_address</a>);
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_update_candidate_p2p_address"></a>

## Function `update_candidate_p2p_address`

Update p2p address of this candidate validator


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_candidate_p2p_address">update_candidate_p2p_address</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_p2p_address">p2p_address</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_candidate_p2p_address">update_candidate_p2p_address</a>(self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_p2p_address">p2p_address</a>: vector&lt;u8&gt;) {
    <b>assert</b>!(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_candidate">is_candidate</a>(self), <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    <b>let</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_p2p_address">p2p_address</a> = <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_p2p_address">p2p_address</a>.to_ascii_string().to_string();
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_p2p_address">p2p_address</a> = <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_p2p_address">p2p_address</a>;
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_update_next_epoch_consensus_address"></a>

## Function `update_next_epoch_consensus_address`

Update primary address of this validator, taking effects from next epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_next_epoch_consensus_address">update_next_epoch_consensus_address</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_address">consensus_address</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_next_epoch_consensus_address">update_next_epoch_consensus_address</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_address">consensus_address</a>: vector&lt;u8&gt;,
) {
    <b>assert</b>!(!<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_inactive">is_inactive</a>(self), <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EInactiveValidator">EInactiveValidator</a>);
    <b>let</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_address">consensus_address</a> = <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_address">consensus_address</a>.to_ascii_string().to_string();
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_address">next_epoch_consensus_address</a> = option::some(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_address">consensus_address</a>);
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_update_candidate_consensus_address"></a>

## Function `update_candidate_consensus_address`

Update primary address of this candidate validator


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_candidate_consensus_address">update_candidate_consensus_address</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_address">consensus_address</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_candidate_consensus_address">update_candidate_consensus_address</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_address">consensus_address</a>: vector&lt;u8&gt;,
) {
    <b>assert</b>!(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_candidate">is_candidate</a>(self), <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    <b>let</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_address">consensus_address</a> = <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_address">consensus_address</a>.to_ascii_string().to_string();
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_address">consensus_address</a> = <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_address">consensus_address</a>;
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_update_next_epoch_protocol_pubkey_bytes"></a>

## Function `update_next_epoch_protocol_pubkey_bytes`

Update protocol public key of this validator, taking effects from next epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_next_epoch_protocol_pubkey_bytes">update_next_epoch_protocol_pubkey_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey_bytes">protocol_pubkey_bytes</a>: vector&lt;u8&gt;, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_proof_of_possession_bytes">proof_of_possession_bytes</a>: vector&lt;u8&gt;, ctx: &<a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_next_epoch_protocol_pubkey_bytes">update_next_epoch_protocol_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey_bytes">protocol_pubkey_bytes</a>: vector&lt;u8&gt;,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_proof_of_possession_bytes">proof_of_possession_bytes</a>: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>assert</b>!(!<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_inactive">is_inactive</a>(self), <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EInactiveValidator">EInactiveValidator</a>);
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.proof_of_possession_sender = ctx.sender();
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a> = option::some(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey_bytes">protocol_pubkey_bytes</a>);
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_proof_of_possession_bytes">next_epoch_proof_of_possession_bytes</a> = option::some(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_proof_of_possession_bytes">proof_of_possession_bytes</a>);
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_update_candidate_protocol_pubkey_bytes"></a>

## Function `update_candidate_protocol_pubkey_bytes`

Update protocol public key of this candidate validator


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_candidate_protocol_pubkey_bytes">update_candidate_protocol_pubkey_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey_bytes">protocol_pubkey_bytes</a>: vector&lt;u8&gt;, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_proof_of_possession_bytes">proof_of_possession_bytes</a>: vector&lt;u8&gt;, ctx: &<a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_candidate_protocol_pubkey_bytes">update_candidate_protocol_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey_bytes">protocol_pubkey_bytes</a>: vector&lt;u8&gt;,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_proof_of_possession_bytes">proof_of_possession_bytes</a>: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>assert</b>!(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_candidate">is_candidate</a>(self), <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.proof_of_possession_sender = ctx.sender();
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey_bytes">protocol_pubkey_bytes</a> = <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey_bytes">protocol_pubkey_bytes</a>;
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_proof_of_possession_bytes">proof_of_possession_bytes</a> = <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_proof_of_possession_bytes">proof_of_possession_bytes</a>;
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_update_next_epoch_network_pubkey_bytes"></a>

## Function `update_next_epoch_network_pubkey_bytes`

Update network public key of this validator, taking effects from next epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_next_epoch_network_pubkey_bytes">update_next_epoch_network_pubkey_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_pubkey_bytes">network_pubkey_bytes</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_next_epoch_network_pubkey_bytes">update_next_epoch_network_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_pubkey_bytes">network_pubkey_bytes</a>: vector&lt;u8&gt;,
) {
    <b>assert</b>!(!<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_inactive">is_inactive</a>(self), <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EInactiveValidator">EInactiveValidator</a>);
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a> = option::some(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_pubkey_bytes">network_pubkey_bytes</a>);
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_update_candidate_network_pubkey_bytes"></a>

## Function `update_candidate_network_pubkey_bytes`

Update network public key of this candidate validator


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_candidate_network_pubkey_bytes">update_candidate_network_pubkey_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_pubkey_bytes">network_pubkey_bytes</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_candidate_network_pubkey_bytes">update_candidate_network_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_pubkey_bytes">network_pubkey_bytes</a>: vector&lt;u8&gt;,
) {
    <b>assert</b>!(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_candidate">is_candidate</a>(self), <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_pubkey_bytes">network_pubkey_bytes</a> = <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_pubkey_bytes">network_pubkey_bytes</a>;
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_update_next_epoch_consensus_pubkey_bytes"></a>

## Function `update_next_epoch_consensus_pubkey_bytes`

Update consensus public key of this validator, taking effects from next epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_next_epoch_consensus_pubkey_bytes">update_next_epoch_consensus_pubkey_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_pubkey_bytes">consensus_pubkey_bytes</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_next_epoch_consensus_pubkey_bytes">update_next_epoch_consensus_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_pubkey_bytes">consensus_pubkey_bytes</a>: vector&lt;u8&gt;,
) {
    <b>assert</b>!(!<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_inactive">is_inactive</a>(self), <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EInactiveValidator">EInactiveValidator</a>);
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a> = option::some(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_pubkey_bytes">consensus_pubkey_bytes</a>);
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_update_next_epoch_class_groups_pubkey_and_proof_bytes"></a>

## Function `update_next_epoch_class_groups_pubkey_and_proof_bytes`

Update class groups public key and its associated proof of this validator, taking effects from next epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_next_epoch_class_groups_pubkey_and_proof_bytes">update_next_epoch_class_groups_pubkey_and_proof_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, class_groups_pubkey_and_proof: (ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_next_epoch_class_groups_pubkey_and_proof_bytes">update_next_epoch_class_groups_pubkey_and_proof_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    class_groups_pubkey_and_proof: ClassGroupsPublicKeyAndProof
) {
    <b>assert</b>!(!<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_inactive">is_inactive</a>(self), <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EInactiveValidator">EInactiveValidator</a>);
    <b>let</b> old_value = self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_class_groups_pubkey_and_proof_bytes">next_epoch_class_groups_pubkey_and_proof_bytes</a>.swap_or_fill(class_groups_pubkey_and_proof);
    old_value.destroy!(|v| {
        v.drop();
    });
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_update_candidate_consensus_pubkey_bytes"></a>

## Function `update_candidate_consensus_pubkey_bytes`

Update consensus public key of this candidate validator


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_candidate_consensus_pubkey_bytes">update_candidate_consensus_pubkey_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_pubkey_bytes">consensus_pubkey_bytes</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_candidate_consensus_pubkey_bytes">update_candidate_consensus_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_pubkey_bytes">consensus_pubkey_bytes</a>: vector&lt;u8&gt;,
) {
    <b>assert</b>!(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_candidate">is_candidate</a>(self), <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_pubkey_bytes">consensus_pubkey_bytes</a> = <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_pubkey_bytes">consensus_pubkey_bytes</a>;
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_update_candidate_class_groups_pubkey_and_proof_bytes"></a>

## Function `update_candidate_class_groups_pubkey_and_proof_bytes`

Update class groups public key and its associated proof of this candidate validator


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_candidate_class_groups_pubkey_and_proof_bytes">update_candidate_class_groups_pubkey_and_proof_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>, class_groups_pubkey_and_proof: (ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_candidate_class_groups_pubkey_and_proof_bytes">update_candidate_class_groups_pubkey_and_proof_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>,
    class_groups_pubkey_and_proof: ClassGroupsPublicKeyAndProof,
) {
    <b>assert</b>!(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_is_candidate">is_candidate</a>(self), <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_class_groups_key_and_proof">update_class_groups_key_and_proof</a>(&<b>mut</b> self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_class_groups_pubkey_and_proof_bytes">class_groups_pubkey_and_proof_bytes</a>, class_groups_pubkey_and_proof);
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validate_metadata">validate_metadata</a>(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_effectuate_staged_metadata"></a>

## Function `effectuate_staged_metadata`

Effectutate all staged next epoch metadata for this validator.
NOTE: this function SHOULD ONLY be called by validator_set when
advancing an epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_effectuate_staged_metadata">effectuate_staged_metadata</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_effectuate_staged_metadata">effectuate_staged_metadata</a>(self: &<b>mut</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>) {
    <b>if</b> (<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_address">next_epoch_network_address</a>(self).is_some()) {
        self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_address">network_address</a> = self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_address">next_epoch_network_address</a>.extract();
        self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_address">next_epoch_network_address</a> = option::none();
    };
    <b>if</b> (<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_p2p_address">next_epoch_p2p_address</a>(self).is_some()) {
        self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_p2p_address">p2p_address</a> = self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_p2p_address">next_epoch_p2p_address</a>.extract();
        self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_p2p_address">next_epoch_p2p_address</a> = option::none();
    };
    <b>if</b> (<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_address">next_epoch_consensus_address</a>(self).is_some()) {
        self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_address">consensus_address</a> = self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_address">next_epoch_consensus_address</a>.extract();
        self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_address">next_epoch_consensus_address</a> = option::none();
    };
    <b>if</b> (<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>(self).is_some()) {
        self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey_bytes">protocol_pubkey_bytes</a> =
        self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>.extract();
        self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a> = option::none();
        self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey">protocol_pubkey</a> = g1_to_uncompressed_g1(&g1_from_bytes(&self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey_bytes">protocol_pubkey_bytes</a>));
        self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_proof_of_possession_bytes">proof_of_possession_bytes</a> = self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_proof_of_possession_bytes">next_epoch_proof_of_possession_bytes</a>.extract();
        self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_proof_of_possession_bytes">next_epoch_proof_of_possession_bytes</a> = option::none();
    };
    <b>if</b> (<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>(self).is_some()) {
        self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_pubkey_bytes">network_pubkey_bytes</a> =
            self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>.extract();
        self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a> = option::none();
    };
    <b>if</b> (<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>(self).is_some()) {
        self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_pubkey_bytes">consensus_pubkey_bytes</a> = self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>.extract();
        self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a> = option::none();
    };
    <b>if</b> (<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_class_groups_pubkey_and_proof_bytes">next_epoch_class_groups_pubkey_and_proof_bytes</a>(self).is_some()) {
        <b>let</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_class_groups_pubkey_and_proof_bytes">next_epoch_class_groups_pubkey_and_proof_bytes</a> = self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_class_groups_pubkey_and_proof_bytes">next_epoch_class_groups_pubkey_and_proof_bytes</a>.extract();
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_class_groups_key_and_proof">update_class_groups_key_and_proof</a>(&<b>mut</b> self.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_class_groups_pubkey_and_proof_bytes">class_groups_pubkey_and_proof_bytes</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_class_groups_pubkey_and_proof_bytes">next_epoch_class_groups_pubkey_and_proof_bytes</a>);
    };
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_update_class_groups_key_and_proof"></a>

## Function `update_class_groups_key_and_proof`



<pre><code><b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_class_groups_key_and_proof">update_class_groups_key_and_proof</a>(class_groups_pubkey_and_proof: &<b>mut</b> <a href="../sui/table_vec.md#sui_table_vec_TableVec">sui::table_vec::TableVec</a>&lt;vector&lt;u8&gt;&gt;, new_class_groups_key_and_proof: (ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_update_class_groups_key_and_proof">update_class_groups_key_and_proof</a> (
    class_groups_pubkey_and_proof: &<b>mut</b> TableVec&lt;vector&lt;u8&gt;&gt;,
    new_class_groups_key_and_proof: ClassGroupsPublicKeyAndProof,
) {
    <b>let</b> <b>mut</b> new_class_groups_key_and_proof = new_class_groups_key_and_proof.destroy();
    <b>let</b> <b>mut</b> i = class_groups_pubkey_and_proof.length() - 1;
    <b>while</b> (!new_class_groups_key_and_proof.is_empty()) {
        *class_groups_pubkey_and_proof.borrow_mut(i) = new_class_groups_key_and_proof.pop_back();
        i = i  - 1;
    };
    new_class_groups_key_and_proof.destroy_empty();
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_verify_proof_of_possession"></a>

## Function `verify_proof_of_possession`

Verify the provided proof of possession using the contained public key and the provided
signature.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_verify_proof_of_possession">verify_proof_of_possession</a>(epoch: u64, sender_address: <b>address</b>, bls_key: vector&lt;u8&gt;, pop_signature: vector&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_verify_proof_of_possession">verify_proof_of_possession</a>(
    epoch: u64,
    sender_address: <b>address</b>,
    bls_key: vector&lt;u8&gt;,
    pop_signature: vector&lt;u8&gt;,
): bool {
    <b>let</b> <b>mut</b> intent_bytes = <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_PROOF_OF_POSSESSION_INTENT">PROOF_OF_POSSESSION_INTENT</a>;
    <b>let</b> <b>mut</b> message = vector&lt;u8&gt;[];
    message.append(bls_key);
    message.append(<a href="../sui/address.md#sui_address_to_bytes">sui::address::to_bytes</a>(sender_address));
    intent_bytes.append(bcs::to_bytes(&message));
    intent_bytes.append(bcs::to_bytes(&epoch));
    bls12381_min_pk_verify(
        &pop_signature,
        &bls_key,
        &intent_bytes,
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_validate_metadata"></a>

## Function `validate_metadata`

Aborts if validator metadata is invalid


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validate_metadata">validate_metadata</a>(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorMetadata">validator_inner_v1::ValidatorMetadata</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validate_metadata">validate_metadata</a>(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorMetadata">ValidatorMetadata</a>) {
    <b>assert</b>!(
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_address">network_address</a>.length() &lt;= <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
                && <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_p2p_address">p2p_address</a>.length() &lt;= <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
                && <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_address">consensus_address</a>.length() &lt;= <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
                && <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_name">name</a>.length() &lt;= <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
                && <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_description">description</a>.length() &lt;= <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
                && <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_image_url">image_url</a>.inner_url().length() &lt;= <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
                && <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_project_url">project_url</a>.inner_url().length() &lt;= <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>,
    );
    <b>if</b> (<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_address">next_epoch_network_address</a>.is_some()) {
        <b>assert</b>!(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_address">next_epoch_network_address</a>.borrow().length() &lt;= <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>);
    };
    <b>if</b> (<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_p2p_address">next_epoch_p2p_address</a>.is_some()) {
        <b>assert</b>!(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_p2p_address">next_epoch_p2p_address</a>.borrow().length() &lt;= <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>);
    };
    <b>if</b> (<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_address">next_epoch_consensus_address</a>.is_some()) {
        <b>assert</b>!(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_address">next_epoch_consensus_address</a>.borrow().length() &lt;= <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>);
    };
    <b>assert</b>!(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_network_pubkey_bytes">network_pubkey_bytes</a>.length() == <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ED25519_KEY_LEN">ED25519_KEY_LEN</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EMetadataInvalidNetworkPubkey">EMetadataInvalidNetworkPubkey</a>);
    <b>if</b> (<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>.is_some()) {
        <b>assert</b>!(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>.borrow().length() == <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ED25519_KEY_LEN">ED25519_KEY_LEN</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EMetadataInvalidNetworkPubkey">EMetadataInvalidNetworkPubkey</a>);
    };
    <b>assert</b>!(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_consensus_pubkey_bytes">consensus_pubkey_bytes</a>.length() == <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ED25519_KEY_LEN">ED25519_KEY_LEN</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EMetadataInvalidConsensusPubkey">EMetadataInvalidConsensusPubkey</a>);
    <b>if</b> (<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>.is_some()) {
        <b>assert</b>!(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>.borrow().length() == <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ED25519_KEY_LEN">ED25519_KEY_LEN</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EMetadataInvalidConsensusPubkey">EMetadataInvalidConsensusPubkey</a>);
    };
    <b>assert</b>!(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey_bytes">protocol_pubkey_bytes</a>.length() == <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_BLS_KEY_LEN">BLS_KEY_LEN</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EMetadataInvalidProtocolPubkey">EMetadataInvalidProtocolPubkey</a>);
    <b>assert</b>!(
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_verify_proof_of_possession">verify_proof_of_possession</a>(
            <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_DEFAULT_EPOCH_ID">DEFAULT_EPOCH_ID</a>,
            <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.proof_of_possession_sender,
            <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_protocol_pubkey_bytes">protocol_pubkey_bytes</a>,
            <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_proof_of_possession_bytes">proof_of_possession_bytes</a>
        ),
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EInvalidProofOfPossession">EInvalidProofOfPossession</a>
    );
    <b>if</b> (<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>.is_some()) {
        <b>assert</b>!(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>.borrow().length() == <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_BLS_KEY_LEN">BLS_KEY_LEN</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EMetadataInvalidProtocolPubkey">EMetadataInvalidProtocolPubkey</a>);
        <b>assert</b>!(
            <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_verify_proof_of_possession">verify_proof_of_possession</a>(
                <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_DEFAULT_EPOCH_ID">DEFAULT_EPOCH_ID</a>,
                <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.proof_of_possession_sender,
                *<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>.borrow(),
                *<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>.<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_proof_of_possession_bytes">next_epoch_proof_of_possession_bytes</a>.borrow()
            ),
            <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_EInvalidProofOfPossession">EInvalidProofOfPossession</a>
        );
    };
    // TODO(omersadika): add <a href="../ika_system/test.md#(ika_system=0x0)_test">test</a> <b>for</b> next epoch
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_get_staking_pool_ref"></a>

## Function `get_staking_pool_ref`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_get_staking_pool_ref">get_staking_pool_ref</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>): &(ika_system=0x0)::<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_StakingPool">staking_pool::StakingPool</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_get_staking_pool_ref">get_staking_pool_ref</a>(self: &<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a>): &StakingPool {
    &self.<a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_inner_v1_create_from_metadata"></a>

## Function `create_from_metadata`

Create a new validator from the given <code><a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorMetadata">ValidatorMetadata</a></code>, called by both <code>new</code> and <code>new_for_testing</code>.


<pre><code><b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_create_from_metadata">create_from_metadata</a>(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, cap_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_operation_cap_id">operation_cap_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>: (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorMetadata">validator_inner_v1::ValidatorMetadata</a>, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_computation_price">computation_price</a>: u64, <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_commission_rate">commission_rate</a>: u16, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">validator_inner_v1::ValidatorInnerV1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_create_from_metadata">create_from_metadata</a>(
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>: ID,
    cap_id: ID,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_operation_cap_id">operation_cap_id</a>: ID,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>: <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorMetadata">ValidatorMetadata</a>,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_computation_price">computation_price</a>: u64,
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_commission_rate">commission_rate</a>: u16,
    ctx: &<b>mut</b> TxContext,
): <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a> {
    <b>let</b> <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a> = <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool_new">staking_pool::new</a>(<a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>, ctx);
    <b>let</b> <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1">validator_inner_v1</a> = <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_ValidatorInnerV1">ValidatorInnerV1</a> {
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_validator_id">validator_id</a>,
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_metadata">metadata</a>,
        // Initialize the voting power to be 0.
        // At the epoch change where this <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> is actually added to the
        // active <a href="../ika_system/validator.md#(ika_system=0x0)_validator">validator</a> set, the voting power will be updated accordingly.
        cap_id,
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_operation_cap_id">operation_cap_id</a>,
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_computation_price">computation_price</a>,
        <a href="../ika_system/staking_pool.md#(ika_system=0x0)_staking_pool">staking_pool</a>,
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_commission_rate">commission_rate</a>,
        next_epoch_stake: 0,
        <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_next_epoch_computation_price">next_epoch_computation_price</a>: <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_computation_price">computation_price</a>,
        next_epoch_commission_rate: <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1_commission_rate">commission_rate</a>,
        extra_fields: bag::new(ctx),
    };
    <a href="../ika_system/validator_inner.md#(ika_system=0x0)_validator_inner_v1">validator_inner_v1</a>
}
</code></pre>



</details>
