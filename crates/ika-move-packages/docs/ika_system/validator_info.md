---
title: Module `(ika_system=0x0)::validator_info`
---



-  [Struct `ValidatorInfo`](#(ika_system=0x0)_validator_info_ValidatorInfo)
-  [Constants](#@Constants_0)
-  [Function `new`](#(ika_system=0x0)_validator_info_new)
-  [Function `metadata`](#(ika_system=0x0)_validator_info_metadata)
-  [Function `validator_id`](#(ika_system=0x0)_validator_info_validator_id)
-  [Function `network_address`](#(ika_system=0x0)_validator_info_network_address)
-  [Function `p2p_address`](#(ika_system=0x0)_validator_info_p2p_address)
-  [Function `consensus_address`](#(ika_system=0x0)_validator_info_consensus_address)
-  [Function `protocol_pubkey_bytes`](#(ika_system=0x0)_validator_info_protocol_pubkey_bytes)
-  [Function `protocol_pubkey`](#(ika_system=0x0)_validator_info_protocol_pubkey)
-  [Function `network_pubkey_bytes`](#(ika_system=0x0)_validator_info_network_pubkey_bytes)
-  [Function `consensus_pubkey_bytes`](#(ika_system=0x0)_validator_info_consensus_pubkey_bytes)
-  [Function `class_groups_pubkey_and_proof_bytes`](#(ika_system=0x0)_validator_info_class_groups_pubkey_and_proof_bytes)
-  [Function `next_epoch_network_address`](#(ika_system=0x0)_validator_info_next_epoch_network_address)
-  [Function `next_epoch_p2p_address`](#(ika_system=0x0)_validator_info_next_epoch_p2p_address)
-  [Function `next_epoch_consensus_address`](#(ika_system=0x0)_validator_info_next_epoch_consensus_address)
-  [Function `next_epoch_protocol_pubkey_bytes`](#(ika_system=0x0)_validator_info_next_epoch_protocol_pubkey_bytes)
-  [Function `next_epoch_network_pubkey_bytes`](#(ika_system=0x0)_validator_info_next_epoch_network_pubkey_bytes)
-  [Function `next_epoch_consensus_pubkey_bytes`](#(ika_system=0x0)_validator_info_next_epoch_consensus_pubkey_bytes)
-  [Function `next_epoch_class_groups_pubkey_and_proof_bytes`](#(ika_system=0x0)_validator_info_next_epoch_class_groups_pubkey_and_proof_bytes)
-  [Function `set_name`](#(ika_system=0x0)_validator_info_set_name)
-  [Function `set_network_address`](#(ika_system=0x0)_validator_info_set_network_address)
-  [Function `set_validator_metadata`](#(ika_system=0x0)_validator_info_set_validator_metadata)
-  [Function `set_next_epoch_network_address`](#(ika_system=0x0)_validator_info_set_next_epoch_network_address)
-  [Function `set_next_epoch_p2p_address`](#(ika_system=0x0)_validator_info_set_next_epoch_p2p_address)
-  [Function `set_next_epoch_consensus_address`](#(ika_system=0x0)_validator_info_set_next_epoch_consensus_address)
-  [Function `set_next_epoch_protocol_pubkey_bytes`](#(ika_system=0x0)_validator_info_set_next_epoch_protocol_pubkey_bytes)
-  [Function `set_next_epoch_network_pubkey_bytes`](#(ika_system=0x0)_validator_info_set_next_epoch_network_pubkey_bytes)
-  [Function `set_next_epoch_consensus_pubkey_bytes`](#(ika_system=0x0)_validator_info_set_next_epoch_consensus_pubkey_bytes)
-  [Function `set_next_epoch_class_groups_pubkey_and_proof_bytes`](#(ika_system=0x0)_validator_info_set_next_epoch_class_groups_pubkey_and_proof_bytes)
-  [Function `roatate_next_epoch_info`](#(ika_system=0x0)_validator_info_roatate_next_epoch_info)
-  [Function `update_class_groups_key_and_proof`](#(ika_system=0x0)_validator_info_update_class_groups_key_and_proof)
-  [Function `verify_proof_of_possession`](#(ika_system=0x0)_validator_info_verify_proof_of_possession)
-  [Function `validate`](#(ika_system=0x0)_validator_info_validate)
-  [Function `destroy`](#(ika_system=0x0)_validator_info_destroy)
-  [Function `is_duplicate`](#(ika_system=0x0)_validator_info_is_duplicate)
-  [Function `is_equal_some_and_value`](#(ika_system=0x0)_validator_info_is_equal_some_and_value)
-  [Function `is_equal_some`](#(ika_system=0x0)_validator_info_is_equal_some)


<pre><code><b>use</b> (ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof">class_groups_public_key_and_proof</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/extended_field.md#(ika_system=0x0)_extended_field">extended_field</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/multiaddr.md#(ika_system=0x0)_multiaddr">multiaddr</a>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata">validator_metadata</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../sui/address.md#sui_address">sui::address</a>;
<b>use</b> <a href="../sui/bcs.md#sui_bcs">sui::bcs</a>;
<b>use</b> <a href="../sui/bls12381.md#sui_bls12381">sui::bls12381</a>;
<b>use</b> <a href="../sui/dynamic_field.md#sui_dynamic_field">sui::dynamic_field</a>;
<b>use</b> <a href="../sui/group_ops.md#sui_group_ops">sui::group_ops</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/table.md#sui_table">sui::table</a>;
<b>use</b> <a href="../sui/table_vec.md#sui_table_vec">sui::table_vec</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
<b>use</b> <a href="../sui/vec_map.md#sui_vec_map">sui::vec_map</a>;
</code></pre>



<a name="(ika_system=0x0)_validator_info_ValidatorInfo"></a>

## Struct `ValidatorInfo`

Represents a validator info in the system.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>name: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_validator_id">validator_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_address">network_address</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 The network address of the validator (could also contain extra info such as port, DNS and etc.).
</dd>
<dt>
<code><a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_p2p_address">p2p_address</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 The address of the validator used for p2p activities such as state sync (could also contain extra info such as port, DNS and etc.).
</dd>
<dt>
<code><a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_address">consensus_address</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 The address of the consensus
</dd>
<dt>
<code><a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_protocol_pubkey_bytes">protocol_pubkey_bytes</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
 The public key bytes corresponding to the private key that the validator
 holds to sign checkpoint messages.
</dd>
<dt>
<code><a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_protocol_pubkey">protocol_pubkey</a>: <a href="../sui/group_ops.md#sui_group_ops_Element">sui::group_ops::Element</a>&lt;<a href="../sui/bls12381.md#sui_bls12381_UncompressedG1">sui::bls12381::UncompressedG1</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_pubkey_bytes">network_pubkey_bytes</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
 The public key bytes corresponding to the private key that the validator
 uses to establish TLS connections
</dd>
<dt>
<code><a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_pubkey_bytes">consensus_pubkey_bytes</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
 The public key bytes correstponding to the consensus
</dd>
<dt>
<code><a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_class_groups_pubkey_and_proof_bytes">class_groups_pubkey_and_proof_bytes</a>: <a href="../sui/table_vec.md#sui_table_vec_TableVec">sui::table_vec::TableVec</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
 The validator's Class Groups public key and its associated proof.
 This key is used for the network DKG process and for resharing the network MPC key.
</dd>
<dt>
<code><a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
 "next_epoch" metadata only takes effects in the next epoch.
 If none, current value will stay unchanged.
</dd>
<dt>
<code><a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_class_groups_pubkey_and_proof_bytes">next_epoch_class_groups_pubkey_and_proof_bytes</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;(ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_address">next_epoch_network_address</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_p2p_address">next_epoch_p2p_address</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_address">next_epoch_consensus_address</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_metadata">metadata</a>: (ika_system=0x0)::<a href="../ika_system/extended_field.md#(ika_system=0x0)_extended_field_ExtendedField">extended_field::ExtendedField</a>&lt;(ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">validator_metadata::ValidatorMetadata</a>&gt;</code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="(ika_system=0x0)_validator_info_BLS_KEY_LEN"></a>



<pre><code><b>const</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_BLS_KEY_LEN">BLS_KEY_LEN</a>: u64 = 48;
</code></pre>



<a name="(ika_system=0x0)_validator_info_DEFAULT_EPOCH_ID"></a>



<pre><code><b>const</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_DEFAULT_EPOCH_ID">DEFAULT_EPOCH_ID</a>: u64 = 0;
</code></pre>



<a name="(ika_system=0x0)_validator_info_ED25519_KEY_LEN"></a>



<pre><code><b>const</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ED25519_KEY_LEN">ED25519_KEY_LEN</a>: u64 = 32;
</code></pre>



<a name="(ika_system=0x0)_validator_info_EInvalidNameLength"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_EInvalidNameLength">EInvalidNameLength</a>: vector&lt;u8&gt; = b"Validator name length exceeds maximum allowed length.";
</code></pre>



<a name="(ika_system=0x0)_validator_info_EInvalidProofOfPossession"></a>

The network public key length is invalid.


<pre><code>#[error]
<b>const</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_EInvalidProofOfPossession">EInvalidProofOfPossession</a>: vector&lt;u8&gt; = b"Invalid proof_of_possession_bytes field in ValidatorMetadata.";
</code></pre>



<a name="(ika_system=0x0)_validator_info_EMetadataInvalidConsensusAddress"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_EMetadataInvalidConsensusAddress">EMetadataInvalidConsensusAddress</a>: vector&lt;u8&gt; = b"Invalid <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_address">consensus_address</a> field in ValidatorMetadata.";
</code></pre>



<a name="(ika_system=0x0)_validator_info_EMetadataInvalidConsensusPubkey"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_EMetadataInvalidConsensusPubkey">EMetadataInvalidConsensusPubkey</a>: vector&lt;u8&gt; = b"Invalid <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_pubkey_bytes">consensus_pubkey_bytes</a> field in ValidatorMetadata.";
</code></pre>



<a name="(ika_system=0x0)_validator_info_EMetadataInvalidNetworkAddress"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_EMetadataInvalidNetworkAddress">EMetadataInvalidNetworkAddress</a>: vector&lt;u8&gt; = b"Invalid <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_address">network_address</a> field in ValidatorMetadata.";
</code></pre>



<a name="(ika_system=0x0)_validator_info_EMetadataInvalidNetworkPubkey"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_EMetadataInvalidNetworkPubkey">EMetadataInvalidNetworkPubkey</a>: vector&lt;u8&gt; = b"Invalid <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_pubkey_bytes">network_pubkey_bytes</a> field in ValidatorMetadata.";
</code></pre>



<a name="(ika_system=0x0)_validator_info_EMetadataInvalidP2pAddress"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_EMetadataInvalidP2pAddress">EMetadataInvalidP2pAddress</a>: vector&lt;u8&gt; = b"Invalid <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_p2p_address">p2p_address</a> field in ValidatorMetadata.";
</code></pre>



<a name="(ika_system=0x0)_validator_info_EMetadataInvalidProtocolPubkey"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_EMetadataInvalidProtocolPubkey">EMetadataInvalidProtocolPubkey</a>: vector&lt;u8&gt; = b"Invalid <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_protocol_pubkey_bytes">protocol_pubkey_bytes</a> field in ValidatorMetadata.";
</code></pre>



<a name="(ika_system=0x0)_validator_info_EValidatorMetadataExceedingLengthLimit"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>: vector&lt;u8&gt; = b"Validator Metadata is too long.";
</code></pre>



<a name="(ika_system=0x0)_validator_info_MAX_NODE_NAME_LENGTH"></a>



<pre><code><b>const</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_MAX_NODE_NAME_LENGTH">MAX_NODE_NAME_LENGTH</a>: u64 = 100;
</code></pre>



<a name="(ika_system=0x0)_validator_info_MAX_VALIDATOR_TEXT_FIELD_LENGTH"></a>



<pre><code><b>const</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_MAX_VALIDATOR_TEXT_FIELD_LENGTH">MAX_VALIDATOR_TEXT_FIELD_LENGTH</a>: u64 = 259;
</code></pre>



<a name="(ika_system=0x0)_validator_info_PROOF_OF_POSSESSION_INTENT"></a>



<pre><code><b>const</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_PROOF_OF_POSSESSION_INTENT">PROOF_OF_POSSESSION_INTENT</a>: vector&lt;u8&gt; = vector[0, 0, 0];
</code></pre>



<a name="(ika_system=0x0)_validator_info_new"></a>

## Function `new`

A public constructor for the ValidatorInfo.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_new">new</a>(name: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_validator_id">validator_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_protocol_pubkey_bytes">protocol_pubkey_bytes</a>: vector&lt;u8&gt;, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_pubkey_bytes">network_pubkey_bytes</a>: vector&lt;u8&gt;, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_pubkey_bytes">consensus_pubkey_bytes</a>: vector&lt;u8&gt;, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_class_groups_pubkey_and_proof_bytes">class_groups_pubkey_and_proof_bytes</a>: (ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof</a>, proof_of_possession_bytes: vector&lt;u8&gt;, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_address">network_address</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_p2p_address">p2p_address</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_address">consensus_address</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_metadata">metadata</a>: (ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">validator_metadata::ValidatorMetadata</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_new">new</a>(
    name: String,
    <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_validator_id">validator_id</a>: ID,
    <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_protocol_pubkey_bytes">protocol_pubkey_bytes</a>: vector&lt;u8&gt;,
    <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_pubkey_bytes">network_pubkey_bytes</a>: vector&lt;u8&gt;,
    <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_pubkey_bytes">consensus_pubkey_bytes</a>: vector&lt;u8&gt;,
    <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_class_groups_pubkey_and_proof_bytes">class_groups_pubkey_and_proof_bytes</a>: ClassGroupsPublicKeyAndProof,
    proof_of_possession_bytes: vector&lt;u8&gt;,
    <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_address">network_address</a>: String,
    <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_p2p_address">p2p_address</a>: String,
    <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_address">consensus_address</a>: String,
    <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_metadata">metadata</a>: ValidatorMetadata,
    ctx: &<b>mut</b> TxContext,
): <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a> {
    <b>let</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_protocol_pubkey">protocol_pubkey</a> = g1_to_uncompressed_g1(&g1_from_bytes(&<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_protocol_pubkey_bytes">protocol_pubkey_bytes</a>));
    <b>let</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_class_groups_pubkey_and_proof_bytes">class_groups_pubkey_and_proof_bytes</a> = <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_class_groups_pubkey_and_proof_bytes">class_groups_pubkey_and_proof_bytes</a>.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_destroy">destroy</a>();
    <b>assert</b>!(
        <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_verify_proof_of_possession">verify_proof_of_possession</a>(
            <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_DEFAULT_EPOCH_ID">DEFAULT_EPOCH_ID</a>,
            ctx.sender(),
            <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_protocol_pubkey_bytes">protocol_pubkey_bytes</a>,
            proof_of_possession_bytes
        ),
        <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_EInvalidProofOfPossession">EInvalidProofOfPossession</a>
    );
    <b>let</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a> = <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a> {
        <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_validator_id">validator_id</a>,
        name,
        <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_protocol_pubkey_bytes">protocol_pubkey_bytes</a>,
        <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_protocol_pubkey">protocol_pubkey</a>,
        <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_pubkey_bytes">network_pubkey_bytes</a>,
        <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_pubkey_bytes">consensus_pubkey_bytes</a>,
        <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_class_groups_pubkey_and_proof_bytes">class_groups_pubkey_and_proof_bytes</a>,
        <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_address">network_address</a>,
        <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_p2p_address">p2p_address</a>,
        <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_address">consensus_address</a>,
        <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>: option::none(),
        <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>: option::none(),
        <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>: option::none(),
        <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_class_groups_pubkey_and_proof_bytes">next_epoch_class_groups_pubkey_and_proof_bytes</a>: option::none(),
        <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_address">next_epoch_network_address</a>: option::none(),
        <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_p2p_address">next_epoch_p2p_address</a>: option::none(),
        <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_address">next_epoch_consensus_address</a>: option::none(),
        <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_metadata">metadata</a>: <a href="../ika_system/extended_field.md#(ika_system=0x0)_extended_field_new">extended_field::new</a>(<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_metadata">metadata</a>, ctx),
    };
    <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_validate">validate</a>();
    <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info">validator_info</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_metadata"></a>

## Function `metadata`

Return the name of the validator info.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_metadata">metadata</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>): (ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">validator_metadata::ValidatorMetadata</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_metadata">metadata</a>(self: &<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>): ValidatorMetadata {
    *self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_metadata">metadata</a>.borrow()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_validator_id"></a>

## Function `validator_id`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_validator_id">validator_id</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_validator_id">validator_id</a>(self: &<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>): ID {
    self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_validator_id">validator_id</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_network_address"></a>

## Function `network_address`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_address">network_address</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>): &<a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_address">network_address</a>(self: &<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>): &String {
    &self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_address">network_address</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_p2p_address"></a>

## Function `p2p_address`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_p2p_address">p2p_address</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>): &<a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_p2p_address">p2p_address</a>(self: &<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>): &String {
    &self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_p2p_address">p2p_address</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_consensus_address"></a>

## Function `consensus_address`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_address">consensus_address</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>): &<a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_address">consensus_address</a>(self: &<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>): &String {
    &self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_address">consensus_address</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_protocol_pubkey_bytes"></a>

## Function `protocol_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_protocol_pubkey_bytes">protocol_pubkey_bytes</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_protocol_pubkey_bytes">protocol_pubkey_bytes</a>(self: &<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>): &vector&lt;u8&gt; {
    &self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_protocol_pubkey_bytes">protocol_pubkey_bytes</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_protocol_pubkey"></a>

## Function `protocol_pubkey`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_protocol_pubkey">protocol_pubkey</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>): &<a href="../sui/group_ops.md#sui_group_ops_Element">sui::group_ops::Element</a>&lt;<a href="../sui/bls12381.md#sui_bls12381_UncompressedG1">sui::bls12381::UncompressedG1</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_protocol_pubkey">protocol_pubkey</a>(self: &<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>): &Element&lt;UncompressedG1&gt; {
    &self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_protocol_pubkey">protocol_pubkey</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_network_pubkey_bytes"></a>

## Function `network_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_pubkey_bytes">network_pubkey_bytes</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_pubkey_bytes">network_pubkey_bytes</a>(self: &<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>): &vector&lt;u8&gt; {
    &self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_pubkey_bytes">network_pubkey_bytes</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_consensus_pubkey_bytes"></a>

## Function `consensus_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_pubkey_bytes">consensus_pubkey_bytes</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_pubkey_bytes">consensus_pubkey_bytes</a>(self: &<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>): &vector&lt;u8&gt; {
    &self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_pubkey_bytes">consensus_pubkey_bytes</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_class_groups_pubkey_and_proof_bytes"></a>

## Function `class_groups_pubkey_and_proof_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_class_groups_pubkey_and_proof_bytes">class_groups_pubkey_and_proof_bytes</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>): &<a href="../sui/table_vec.md#sui_table_vec_TableVec">sui::table_vec::TableVec</a>&lt;vector&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_class_groups_pubkey_and_proof_bytes">class_groups_pubkey_and_proof_bytes</a>(self: &<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>): &TableVec&lt;vector&lt;u8&gt;&gt; {
    &self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_class_groups_pubkey_and_proof_bytes">class_groups_pubkey_and_proof_bytes</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_next_epoch_network_address"></a>

## Function `next_epoch_network_address`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_address">next_epoch_network_address</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_address">next_epoch_network_address</a>(self: &<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>): &Option&lt;String&gt; {
    &self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_address">next_epoch_network_address</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_next_epoch_p2p_address"></a>

## Function `next_epoch_p2p_address`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_p2p_address">next_epoch_p2p_address</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_p2p_address">next_epoch_p2p_address</a>(self: &<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>): &Option&lt;String&gt; {
    &self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_p2p_address">next_epoch_p2p_address</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_next_epoch_consensus_address"></a>

## Function `next_epoch_consensus_address`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_address">next_epoch_consensus_address</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_address">next_epoch_consensus_address</a>(self: &<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>): &Option&lt;String&gt; {
    &self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_address">next_epoch_consensus_address</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_next_epoch_protocol_pubkey_bytes"></a>

## Function `next_epoch_protocol_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>(self: &<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>): &Option&lt;vector&lt;u8&gt;&gt; {
    &self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_next_epoch_network_pubkey_bytes"></a>

## Function `next_epoch_network_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>(self: &<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>): &Option&lt;vector&lt;u8&gt;&gt; {
    &self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_next_epoch_consensus_pubkey_bytes"></a>

## Function `next_epoch_consensus_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>(self: &<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>): &Option&lt;vector&lt;u8&gt;&gt; {
    &self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_next_epoch_class_groups_pubkey_and_proof_bytes"></a>

## Function `next_epoch_class_groups_pubkey_and_proof_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_class_groups_pubkey_and_proof_bytes">next_epoch_class_groups_pubkey_and_proof_bytes</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;(ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_class_groups_pubkey_and_proof_bytes">next_epoch_class_groups_pubkey_and_proof_bytes</a>(self: &<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>): &Option&lt;ClassGroupsPublicKeyAndProof&gt; {
    &self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_class_groups_pubkey_and_proof_bytes">next_epoch_class_groups_pubkey_and_proof_bytes</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_set_name"></a>

## Function `set_name`

Sets the name of the validator info.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_set_name">set_name</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>, name: <a href="../std/string.md#std_string_String">std::string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_set_name">set_name</a>(self: &<b>mut</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>, name: String) {
    self.name = name;
    self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_validate">validate</a>();
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_set_network_address"></a>

## Function `set_network_address`

Sets the network address or host of the validator info.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_set_network_address">set_network_address</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_address">network_address</a>: <a href="../std/string.md#std_string_String">std::string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_set_network_address">set_network_address</a>(self: &<b>mut</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_address">network_address</a>: String) {
    self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_address">network_address</a> = <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_address">network_address</a>;
    self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_validate">validate</a>();
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_set_validator_metadata"></a>

## Function `set_validator_metadata`

Sets the metadata of the validator info.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_set_validator_metadata">set_validator_metadata</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_metadata">metadata</a>: (ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">validator_metadata::ValidatorMetadata</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_set_validator_metadata">set_validator_metadata</a>(self: &<b>mut</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_metadata">metadata</a>: ValidatorMetadata) {
    self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_metadata">metadata</a>.swap(<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_metadata">metadata</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_set_next_epoch_network_address"></a>

## Function `set_next_epoch_network_address`

Sets network address of this validator, taking effects from next epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_set_next_epoch_network_address">set_next_epoch_network_address</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_address">network_address</a>: <a href="../std/string.md#std_string_String">std::string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_set_next_epoch_network_address">set_next_epoch_network_address</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>,
    <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_address">network_address</a>: String,
) {
    self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_address">next_epoch_network_address</a> = option::some(<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_address">network_address</a>);
    self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_validate">validate</a>();
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_set_next_epoch_p2p_address"></a>

## Function `set_next_epoch_p2p_address`

Sets p2p address of this validator, taking effects from next epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_set_next_epoch_p2p_address">set_next_epoch_p2p_address</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_p2p_address">p2p_address</a>: <a href="../std/string.md#std_string_String">std::string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_set_next_epoch_p2p_address">set_next_epoch_p2p_address</a>(self: &<b>mut</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_p2p_address">p2p_address</a>: String) {
    self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_p2p_address">next_epoch_p2p_address</a> = option::some(<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_p2p_address">p2p_address</a>);
    self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_validate">validate</a>();
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_set_next_epoch_consensus_address"></a>

## Function `set_next_epoch_consensus_address`

Sets primary address of this validator, taking effects from next epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_set_next_epoch_consensus_address">set_next_epoch_consensus_address</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_address">consensus_address</a>: <a href="../std/string.md#std_string_String">std::string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_set_next_epoch_consensus_address">set_next_epoch_consensus_address</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>,
    <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_address">consensus_address</a>: String,
) {
    self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_address">next_epoch_consensus_address</a> = option::some(<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_address">consensus_address</a>);
    self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_validate">validate</a>();
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_set_next_epoch_protocol_pubkey_bytes"></a>

## Function `set_next_epoch_protocol_pubkey_bytes`

Sets protocol public key of this validator, taking effects from next epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_set_next_epoch_protocol_pubkey_bytes">set_next_epoch_protocol_pubkey_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_protocol_pubkey_bytes">protocol_pubkey_bytes</a>: vector&lt;u8&gt;, proof_of_possession_bytes: vector&lt;u8&gt;, ctx: &<a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_set_next_epoch_protocol_pubkey_bytes">set_next_epoch_protocol_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>,
    <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_protocol_pubkey_bytes">protocol_pubkey_bytes</a>: vector&lt;u8&gt;,
    proof_of_possession_bytes: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>assert</b>!(
        <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_verify_proof_of_possession">verify_proof_of_possession</a>(
            <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_DEFAULT_EPOCH_ID">DEFAULT_EPOCH_ID</a>,
            ctx.sender(),
            <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_protocol_pubkey_bytes">protocol_pubkey_bytes</a>,
            proof_of_possession_bytes
        ),
        <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_EInvalidProofOfPossession">EInvalidProofOfPossession</a>
    );
    self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a> = option::some(<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_protocol_pubkey_bytes">protocol_pubkey_bytes</a>);
    self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_validate">validate</a>();
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_set_next_epoch_network_pubkey_bytes"></a>

## Function `set_next_epoch_network_pubkey_bytes`

Sets network public key of this validator, taking effects from next epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_set_next_epoch_network_pubkey_bytes">set_next_epoch_network_pubkey_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_pubkey_bytes">network_pubkey_bytes</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_set_next_epoch_network_pubkey_bytes">set_next_epoch_network_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>,
    <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_pubkey_bytes">network_pubkey_bytes</a>: vector&lt;u8&gt;,
) {
    self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a> = option::some(<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_pubkey_bytes">network_pubkey_bytes</a>);
    self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_validate">validate</a>();
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_set_next_epoch_consensus_pubkey_bytes"></a>

## Function `set_next_epoch_consensus_pubkey_bytes`

Sets consensus public key of this validator, taking effects from next epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_set_next_epoch_consensus_pubkey_bytes">set_next_epoch_consensus_pubkey_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_pubkey_bytes">consensus_pubkey_bytes</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_set_next_epoch_consensus_pubkey_bytes">set_next_epoch_consensus_pubkey_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>,
    <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_pubkey_bytes">consensus_pubkey_bytes</a>: vector&lt;u8&gt;,
) {
    self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a> = option::some(<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_pubkey_bytes">consensus_pubkey_bytes</a>);
    self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_validate">validate</a>();
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_set_next_epoch_class_groups_pubkey_and_proof_bytes"></a>

## Function `set_next_epoch_class_groups_pubkey_and_proof_bytes`

Sets class groups public key and its associated proof of this validator, taking effects from next epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_set_next_epoch_class_groups_pubkey_and_proof_bytes">set_next_epoch_class_groups_pubkey_and_proof_bytes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>, class_groups_pubkey_and_proof: (ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_set_next_epoch_class_groups_pubkey_and_proof_bytes">set_next_epoch_class_groups_pubkey_and_proof_bytes</a>(
    self: &<b>mut</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>,
    class_groups_pubkey_and_proof: ClassGroupsPublicKeyAndProof
) {
    <b>let</b> old_value = self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_class_groups_pubkey_and_proof_bytes">next_epoch_class_groups_pubkey_and_proof_bytes</a>.swap_or_fill(class_groups_pubkey_and_proof);
    old_value.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_destroy">destroy</a>!(|v| {
        v.drop();
    });
    self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_validate">validate</a>();
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_roatate_next_epoch_info"></a>

## Function `roatate_next_epoch_info`

Effectutate all staged next epoch metadata for this validator.
NOTE: this function SHOULD ONLY be called by validator_set when
advancing an epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_roatate_next_epoch_info">roatate_next_epoch_info</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_roatate_next_epoch_info">roatate_next_epoch_info</a>(self: &<b>mut</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>) {
    <b>if</b> (self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_address">next_epoch_network_address</a>.is_some()) {
        self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_address">network_address</a> = self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_address">next_epoch_network_address</a>.extract();
        self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_address">next_epoch_network_address</a> = option::none();
    };
    <b>if</b> (self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_p2p_address">next_epoch_p2p_address</a>.is_some()) {
        self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_p2p_address">p2p_address</a> = self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_p2p_address">next_epoch_p2p_address</a>.extract();
        self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_p2p_address">next_epoch_p2p_address</a> = option::none();
    };
    <b>if</b> (self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_address">next_epoch_consensus_address</a>.is_some()) {
        self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_address">consensus_address</a> = self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_address">next_epoch_consensus_address</a>.extract();
        self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_address">next_epoch_consensus_address</a> = option::none();
    };
    <b>if</b> (self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>.is_some()) {
        self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_protocol_pubkey_bytes">protocol_pubkey_bytes</a> =
        self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>.extract();
        self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a> = option::none();
        self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_protocol_pubkey">protocol_pubkey</a> = g1_to_uncompressed_g1(&g1_from_bytes(&self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_protocol_pubkey_bytes">protocol_pubkey_bytes</a>));
    };
    <b>if</b> (self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>.is_some()) {
        self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_pubkey_bytes">network_pubkey_bytes</a> =
            self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>.extract();
        self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a> = option::none();
    };
    <b>if</b> (self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>.is_some()) {
        self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_pubkey_bytes">consensus_pubkey_bytes</a> = self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>.extract();
        self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a> = option::none();
    };
    <b>if</b> (self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_class_groups_pubkey_and_proof_bytes">next_epoch_class_groups_pubkey_and_proof_bytes</a>.is_some()) {
        <b>let</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_class_groups_pubkey_and_proof_bytes">next_epoch_class_groups_pubkey_and_proof_bytes</a> = self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_class_groups_pubkey_and_proof_bytes">next_epoch_class_groups_pubkey_and_proof_bytes</a>.extract();
        <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_update_class_groups_key_and_proof">update_class_groups_key_and_proof</a>(&<b>mut</b> self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_class_groups_pubkey_and_proof_bytes">class_groups_pubkey_and_proof_bytes</a>, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_class_groups_pubkey_and_proof_bytes">next_epoch_class_groups_pubkey_and_proof_bytes</a>);
    };
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_update_class_groups_key_and_proof"></a>

## Function `update_class_groups_key_and_proof`



<pre><code><b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_update_class_groups_key_and_proof">update_class_groups_key_and_proof</a>(class_groups_pubkey_and_proof: &<b>mut</b> <a href="../sui/table_vec.md#sui_table_vec_TableVec">sui::table_vec::TableVec</a>&lt;vector&lt;u8&gt;&gt;, new_class_groups_key_and_proof: (ika_system=0x0)::<a href="../ika_system/class_groups_public_key_and_proof.md#(ika_system=0x0)_class_groups_public_key_and_proof_ClassGroupsPublicKeyAndProof">class_groups_public_key_and_proof::ClassGroupsPublicKeyAndProof</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_update_class_groups_key_and_proof">update_class_groups_key_and_proof</a> (
    class_groups_pubkey_and_proof: &<b>mut</b> TableVec&lt;vector&lt;u8&gt;&gt;,
    new_class_groups_key_and_proof: ClassGroupsPublicKeyAndProof,
) {
    <b>let</b> <b>mut</b> new_class_groups_key_and_proof = new_class_groups_key_and_proof.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_destroy">destroy</a>();
    <b>let</b> <b>mut</b> i = class_groups_pubkey_and_proof.length() - 1;
    <b>while</b> (!new_class_groups_key_and_proof.is_empty()) {
        *class_groups_pubkey_and_proof.borrow_mut(i) = new_class_groups_key_and_proof.pop_back();
        i = i  - 1;
    };
    new_class_groups_key_and_proof.destroy_empty();
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_verify_proof_of_possession"></a>

## Function `verify_proof_of_possession`

Verify the provided proof of possession using the contained public key and the provided
signature.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_verify_proof_of_possession">verify_proof_of_possession</a>(epoch: u64, sender_address: <b>address</b>, bls_key: vector&lt;u8&gt;, pop_signature: vector&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_verify_proof_of_possession">verify_proof_of_possession</a>(
    epoch: u64,
    sender_address: <b>address</b>,
    bls_key: vector&lt;u8&gt;,
    pop_signature: vector&lt;u8&gt;,
): bool {
    <b>let</b> <b>mut</b> intent_bytes = <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_PROOF_OF_POSSESSION_INTENT">PROOF_OF_POSSESSION_INTENT</a>;
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

<a name="(ika_system=0x0)_validator_info_validate"></a>

## Function `validate`

Aborts if validator info is invalid


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_validate">validate</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_validate">validate</a>(self: &<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>) {
        // Verify name length.
    <b>assert</b>!(self.name.length() &lt;= <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_MAX_NODE_NAME_LENGTH">MAX_NODE_NAME_LENGTH</a>, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_EInvalidNameLength">EInvalidNameLength</a>);
    // Verify <b>address</b> length.
    <b>assert</b>!(
        self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_address">network_address</a>.length() &lt;= <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_MAX_VALIDATOR_TEXT_FIELD_LENGTH">MAX_VALIDATOR_TEXT_FIELD_LENGTH</a>
                && self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_p2p_address">p2p_address</a>.length() &lt;= <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_MAX_VALIDATOR_TEXT_FIELD_LENGTH">MAX_VALIDATOR_TEXT_FIELD_LENGTH</a>
                && self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_address">consensus_address</a>.length() &lt;= <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_MAX_VALIDATOR_TEXT_FIELD_LENGTH">MAX_VALIDATOR_TEXT_FIELD_LENGTH</a>
                && self.name.length() &lt;= <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_MAX_VALIDATOR_TEXT_FIELD_LENGTH">MAX_VALIDATOR_TEXT_FIELD_LENGTH</a>,
        <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>,
    );
    <b>assert</b>!(<a href="../ika_system/multiaddr.md#(ika_system=0x0)_multiaddr_validate_tcp">multiaddr::validate_tcp</a>(&self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_address">network_address</a>), <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_EMetadataInvalidNetworkAddress">EMetadataInvalidNetworkAddress</a>);
    <b>if</b> (self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_address">next_epoch_network_address</a>.is_some()) {
        <b>assert</b>!(self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_address">next_epoch_network_address</a>.borrow().length() &lt;= <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_MAX_VALIDATOR_TEXT_FIELD_LENGTH">MAX_VALIDATOR_TEXT_FIELD_LENGTH</a>, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>);
        <b>assert</b>!(<a href="../ika_system/multiaddr.md#(ika_system=0x0)_multiaddr_validate_tcp">multiaddr::validate_tcp</a>(self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_address">next_epoch_network_address</a>.borrow()), <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_EMetadataInvalidNetworkAddress">EMetadataInvalidNetworkAddress</a>);
    };
    <b>assert</b>!(<a href="../ika_system/multiaddr.md#(ika_system=0x0)_multiaddr_validate_udp">multiaddr::validate_udp</a>(&self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_p2p_address">p2p_address</a>), <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_EMetadataInvalidP2pAddress">EMetadataInvalidP2pAddress</a>);
    <b>if</b> (self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_p2p_address">next_epoch_p2p_address</a>.is_some()) {
        <b>assert</b>!(self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_p2p_address">next_epoch_p2p_address</a>.borrow().length() &lt;= <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_MAX_VALIDATOR_TEXT_FIELD_LENGTH">MAX_VALIDATOR_TEXT_FIELD_LENGTH</a>, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>);
        <b>assert</b>!(<a href="../ika_system/multiaddr.md#(ika_system=0x0)_multiaddr_validate_udp">multiaddr::validate_udp</a>(self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_p2p_address">next_epoch_p2p_address</a>.borrow()), <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_EMetadataInvalidP2pAddress">EMetadataInvalidP2pAddress</a>);
    };
    <b>assert</b>!(<a href="../ika_system/multiaddr.md#(ika_system=0x0)_multiaddr_validate_udp">multiaddr::validate_udp</a>(&self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_address">consensus_address</a>), <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_EMetadataInvalidConsensusAddress">EMetadataInvalidConsensusAddress</a>);
    <b>if</b> (self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_address">next_epoch_consensus_address</a>.is_some()) {
        <b>assert</b>!(self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_address">next_epoch_consensus_address</a>.borrow().length() &lt;= <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_MAX_VALIDATOR_TEXT_FIELD_LENGTH">MAX_VALIDATOR_TEXT_FIELD_LENGTH</a>, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>);
        <b>assert</b>!(<a href="../ika_system/multiaddr.md#(ika_system=0x0)_multiaddr_validate_udp">multiaddr::validate_udp</a>(self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_address">next_epoch_consensus_address</a>.borrow()), <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_EMetadataInvalidConsensusAddress">EMetadataInvalidConsensusAddress</a>);
    };
    <b>assert</b>!(self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_pubkey_bytes">network_pubkey_bytes</a>.length() == <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ED25519_KEY_LEN">ED25519_KEY_LEN</a>, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_EMetadataInvalidNetworkPubkey">EMetadataInvalidNetworkPubkey</a>);
    <b>if</b> (self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>.is_some()) {
        <b>assert</b>!(self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>.borrow().length() == <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ED25519_KEY_LEN">ED25519_KEY_LEN</a>, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_EMetadataInvalidNetworkPubkey">EMetadataInvalidNetworkPubkey</a>);
    };
    <b>assert</b>!(self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_pubkey_bytes">consensus_pubkey_bytes</a>.length() == <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ED25519_KEY_LEN">ED25519_KEY_LEN</a>, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_EMetadataInvalidConsensusPubkey">EMetadataInvalidConsensusPubkey</a>);
    <b>if</b> (self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>.is_some()) {
        <b>assert</b>!(self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>.borrow().length() == <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ED25519_KEY_LEN">ED25519_KEY_LEN</a>, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_EMetadataInvalidConsensusPubkey">EMetadataInvalidConsensusPubkey</a>);
    };
    <b>assert</b>!(self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_protocol_pubkey_bytes">protocol_pubkey_bytes</a>.length() == <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_BLS_KEY_LEN">BLS_KEY_LEN</a>, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_EMetadataInvalidProtocolPubkey">EMetadataInvalidProtocolPubkey</a>);
    <b>if</b> (self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>.is_some()) {
        <b>assert</b>!(self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>.borrow().length() == <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_BLS_KEY_LEN">BLS_KEY_LEN</a>, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_EMetadataInvalidProtocolPubkey">EMetadataInvalidProtocolPubkey</a>);
    };
    // TODO(omersadika): add test <b>for</b> next epoch
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_destroy"></a>

## Function `destroy`

Destroy the validator info.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_destroy">destroy</a>(self: (ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_destroy">destroy</a>(self: <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>) {
    <b>let</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a> { <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_metadata">metadata</a>, <b>mut</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_class_groups_pubkey_and_proof_bytes">class_groups_pubkey_and_proof_bytes</a>, <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_class_groups_pubkey_and_proof_bytes">next_epoch_class_groups_pubkey_and_proof_bytes</a>, .. } = self;
    <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_metadata">metadata</a>.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_destroy">destroy</a>();
    <b>while</b>(<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_class_groups_pubkey_and_proof_bytes">class_groups_pubkey_and_proof_bytes</a>.length() != 0) {
        <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_class_groups_pubkey_and_proof_bytes">class_groups_pubkey_and_proof_bytes</a>.pop_back();
    };
    <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_class_groups_pubkey_and_proof_bytes">class_groups_pubkey_and_proof_bytes</a>.destroy_empty();
    <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_class_groups_pubkey_and_proof_bytes">next_epoch_class_groups_pubkey_and_proof_bytes</a>.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_destroy">destroy</a>!(|c| c.drop());
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_is_duplicate"></a>

## Function `is_duplicate`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_is_duplicate">is_duplicate</a>(self: &(ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>, other: &(ika_system=0x0)::<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">validator_info::ValidatorInfo</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_is_duplicate">is_duplicate</a>(self: &<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>, other: &<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_ValidatorInfo">ValidatorInfo</a>): bool {
            self.name == other.name
            || self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_address">network_address</a> == other.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_address">network_address</a>
            || self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_p2p_address">p2p_address</a> == other.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_p2p_address">p2p_address</a>
            || self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_protocol_pubkey_bytes">protocol_pubkey_bytes</a> == other.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_protocol_pubkey_bytes">protocol_pubkey_bytes</a>
            || self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_pubkey_bytes">network_pubkey_bytes</a> == other.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_pubkey_bytes">network_pubkey_bytes</a>
            || self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_pubkey_bytes">network_pubkey_bytes</a> == other.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_pubkey_bytes">consensus_pubkey_bytes</a>
            || self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_pubkey_bytes">consensus_pubkey_bytes</a> == other.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_pubkey_bytes">consensus_pubkey_bytes</a>
            || self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_pubkey_bytes">consensus_pubkey_bytes</a> == other.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_pubkey_bytes">network_pubkey_bytes</a>
            // All next epoch parameters.
            || <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_is_equal_some">is_equal_some</a>(&self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_address">next_epoch_network_address</a>, &other.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_address">next_epoch_network_address</a>)
            || <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_is_equal_some">is_equal_some</a>(&self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_p2p_address">next_epoch_p2p_address</a>, &other.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_p2p_address">next_epoch_p2p_address</a>)
            || <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_is_equal_some">is_equal_some</a>(&self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>, &other.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>)
            || <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_is_equal_some">is_equal_some</a>(&self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>, &other.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>)
            || <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_is_equal_some">is_equal_some</a>(&self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>, &other.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>)
            || <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_is_equal_some">is_equal_some</a>(&self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>, &other.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>)
            || <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_is_equal_some">is_equal_some</a>(&self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>, &other.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>)
            // My next epoch parameters with other current epoch parameters.
            || <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_is_equal_some_and_value">is_equal_some_and_value</a>(&self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_address">next_epoch_network_address</a>, &other.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_address">network_address</a>)
            || <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_is_equal_some_and_value">is_equal_some_and_value</a>(&self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_p2p_address">next_epoch_p2p_address</a>, &other.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_p2p_address">p2p_address</a>)
            || <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_is_equal_some_and_value">is_equal_some_and_value</a>(&self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>, &other.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_protocol_pubkey_bytes">protocol_pubkey_bytes</a>)
            || <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_is_equal_some_and_value">is_equal_some_and_value</a>(&self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>, &other.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_pubkey_bytes">network_pubkey_bytes</a>)
            || <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_is_equal_some_and_value">is_equal_some_and_value</a>(&self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>, &other.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_pubkey_bytes">consensus_pubkey_bytes</a>)
            || <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_is_equal_some_and_value">is_equal_some_and_value</a>(&self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>, &other.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_pubkey_bytes">consensus_pubkey_bytes</a>)
            || <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_is_equal_some_and_value">is_equal_some_and_value</a>(&self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>, &other.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_pubkey_bytes">network_pubkey_bytes</a>)
            // Other next epoch parameters with my current epoch parameters.
            || <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_is_equal_some_and_value">is_equal_some_and_value</a>(&other.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_address">next_epoch_network_address</a>, &self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_address">network_address</a>)
            || <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_is_equal_some_and_value">is_equal_some_and_value</a>(&other.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_p2p_address">next_epoch_p2p_address</a>, &self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_p2p_address">p2p_address</a>)
            || <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_is_equal_some_and_value">is_equal_some_and_value</a>(&other.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>, &self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_protocol_pubkey_bytes">protocol_pubkey_bytes</a>)
            || <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_is_equal_some_and_value">is_equal_some_and_value</a>(&other.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>, &self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_pubkey_bytes">network_pubkey_bytes</a>)
            || <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_is_equal_some_and_value">is_equal_some_and_value</a>(&other.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>, &self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_pubkey_bytes">consensus_pubkey_bytes</a>)
            || <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_is_equal_some_and_value">is_equal_some_and_value</a>(&other.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>, &self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_consensus_pubkey_bytes">consensus_pubkey_bytes</a>)
            || <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_is_equal_some_and_value">is_equal_some_and_value</a>(&other.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_next_epoch_consensus_pubkey_bytes">next_epoch_consensus_pubkey_bytes</a>, &self.<a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_network_pubkey_bytes">network_pubkey_bytes</a>)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_is_equal_some_and_value"></a>

## Function `is_equal_some_and_value`



<pre><code><b>fun</b> is_equal_some_and_valueT(a: &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;T&gt;, b: &T): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_is_equal_some_and_value">is_equal_some_and_value</a>&lt;T&gt;(a: &Option&lt;T&gt;, b: &T): bool {
    <b>if</b> (a.is_none()) {
        <b>false</b>
    } <b>else</b> {
        a.borrow() == b
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_info_is_equal_some"></a>

## Function `is_equal_some`



<pre><code><b>fun</b> is_equal_someT(a: &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;T&gt;, b: &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;T&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/validator_info.md#(ika_system=0x0)_validator_info_is_equal_some">is_equal_some</a>&lt;T&gt;(a: &Option&lt;T&gt;, b: &Option&lt;T&gt;): bool {
    <b>if</b> (a.is_none() || b.is_none()) {
        <b>false</b>
    } <b>else</b> {
        a.borrow() == b.borrow()
    }
}
</code></pre>



</details>
