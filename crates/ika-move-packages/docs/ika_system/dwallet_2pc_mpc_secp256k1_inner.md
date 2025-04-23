---
title: Module `(ika_system=0x0)::dwallet_2pc_mpc_secp256k1_inner`
---

This module handles the logic for creating and managing dWallets using the Secp256K1 signature scheme
and the DKG process. It leverages validators to execute MPC (Multi-Party Computation)
protocols to ensure trustless and decentralized wallet creation and key management.


-  [Struct `DWalletCoordinatorInner`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner)
-  [Struct `DWalletSessionEventKey`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletSessionEventKey)
-  [Struct `DWalletSession`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletSession)
-  [Struct `DWalletCap`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCap)
-  [Struct `DWalletNetworkDecryptionKeyCap`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletNetworkDecryptionKeyCap)
-  [Struct `DWalletNetworkDecryptionKey`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletNetworkDecryptionKey)
-  [Struct `EncryptionKey`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptionKey)
-  [Struct `EncryptedUserSecretKeyShare`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptedUserSecretKeyShare)
-  [Struct `UnverifiedECDSAPartialUserSignatureCap`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_UnverifiedECDSAPartialUserSignatureCap)
-  [Struct `VerifiedECDSAPartialUserSignatureCap`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_VerifiedECDSAPartialUserSignatureCap)
-  [Struct `ECDSAPartialUserSignature`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPartialUserSignature)
-  [Struct `DWallet`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet)
-  [Struct `ECDSAPresignCap`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresignCap)
-  [Struct `ECDSAPresign`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresign)
-  [Struct `ECDSASign`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSASign)
-  [Struct `DWalletEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletEvent)
-  [Struct `CreatedEncryptionKeyEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CreatedEncryptionKeyEvent)
-  [Struct `DWalletNetworkDKGDecryptionKeyRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletNetworkDKGDecryptionKeyRequestEvent)
-  [Struct `DWalletDecryptionKeyReshareRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletDecryptionKeyReshareRequestEvent)
-  [Struct `CompletedDWalletDecryptionKeyReshareEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedDWalletDecryptionKeyReshareEvent)
-  [Struct `CompletedDWalletNetworkDKGDecryptionKeyEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedDWalletNetworkDKGDecryptionKeyEvent)
-  [Struct `DWalletDKGFirstRoundRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletDKGFirstRoundRequestEvent)
-  [Struct `CompletedDKGFirstdRoundEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedDKGFirstdRoundEvent)
-  [Struct `DWalletDKGSecondRoundRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletDKGSecondRoundRequestEvent)
-  [Struct `CompletedDWalletDKGSecondRoundEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedDWalletDKGSecondRoundEvent)
-  [Struct `RejectedDWalletDKGSecondRoundEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_RejectedDWalletDKGSecondRoundEvent)
-  [Struct `EncryptedShareVerificationRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptedShareVerificationRequestEvent)
-  [Struct `CompletedEncryptedShareVerificationEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedEncryptedShareVerificationEvent)
-  [Struct `RejectedEncryptedShareVerificationEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_RejectedEncryptedShareVerificationEvent)
-  [Struct `AcceptReEncryptedUserShareEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_AcceptReEncryptedUserShareEvent)
-  [Struct `ECDSAPresignRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresignRequestEvent)
-  [Struct `CompletedECDSAPresignEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedECDSAPresignEvent)
-  [Struct `RejectedECDSAPresignEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_RejectedECDSAPresignEvent)
-  [Struct `ECDSASignRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSASignRequestEvent)
-  [Struct `ECDSAFutureSignRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAFutureSignRequestEvent)
-  [Struct `CompletedECDSAFutureSignEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedECDSAFutureSignEvent)
-  [Struct `RejectedECDSAFutureSignEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_RejectedECDSAFutureSignEvent)
-  [Struct `CompletedECDSASignEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedECDSASignEvent)
-  [Struct `RejectedECDSASignEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_RejectedECDSASignEvent)
-  [Struct `SystemCheckpointInfoEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_SystemCheckpointInfoEvent)
-  [Struct `MessageApproval`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_MessageApproval)
-  [Enum `DWalletNetworkDecryptionKeyState`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletNetworkDecryptionKeyState)
-  [Enum `EncryptedUserSecretKeyShareState`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptedUserSecretKeyShareState)
-  [Enum `ECDSAPartialUserSignatureState`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPartialUserSignatureState)
-  [Enum `DWalletState`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletState)
-  [Enum `ECDSAPresignState`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresignState)
-  [Enum `ECDSASignState`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSASignState)
-  [Constants](#@Constants_1)
-  [Function `lock_last_active_session_sequence_number`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_lock_last_active_session_sequence_number)
-  [Function `create_dwallet_coordinator_inner`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_create_dwallet_coordinator_inner)
-  [Function `request_dwallet_network_decryption_key_dkg`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_dwallet_network_decryption_key_dkg)
-  [Function `respond_dwallet_network_decryption_key_dkg`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_dwallet_network_decryption_key_dkg)
-  [Function `respond_dwallet_network_decryption_key_reconfiguration`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_dwallet_network_decryption_key_reconfiguration)
-  [Function `advance_epoch_dwallet_network_decryption_key`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_advance_epoch_dwallet_network_decryption_key)
-  [Function `emit_start_reshare_event`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_emit_start_reshare_event)
-  [Function `get_active_dwallet_network_decryption_key`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_active_dwallet_network_decryption_key)
-  [Function `advance_epoch`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_advance_epoch)
-  [Function `get_dwallet`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_dwallet)
-  [Function `get_dwallet_mut`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_dwallet_mut)
-  [Function `validate_active_and_get_public_output`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_validate_active_and_get_public_output)
-  [Function `charge_and_create_current_epoch_dwallet_event`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_charge_and_create_current_epoch_dwallet_event)
-  [Function `create_immediate_dwallet_event`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_create_immediate_dwallet_event)
-  [Function `get_active_dwallet_and_public_output`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_active_dwallet_and_public_output)
-  [Function `get_active_dwallet_and_public_output_mut`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_active_dwallet_and_public_output_mut)
-  [Function `get_active_encryption_key`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_active_encryption_key)
-  [Function `register_encryption_key`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_register_encryption_key)
-  [Function `create_message_approval`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_create_message_approval)
-  [Function `approve_message`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_approve_message)
-  [Function `is_supported_hash_scheme`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_is_supported_hash_scheme)
-  [Function `request_dwallet_dkg_first_round`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_dwallet_dkg_first_round)
-  [Function `update_last_session_to_complete_in_current_epoch`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_update_last_session_to_complete_in_current_epoch)
-  [Function `all_current_epoch_sessions_completed`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_all_current_epoch_sessions_completed)
-  [Function `remove_session_and_charge`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_remove_session_and_charge)
-  [Function `respond_dwallet_dkg_first_round`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_dwallet_dkg_first_round)
-  [Function `create_first_round_dwallet_mock`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_create_first_round_dwallet_mock)
-  [Function `mock_create_dwallet`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_mock_create_dwallet)
-  [Function `request_dwallet_dkg_second_round`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_dwallet_dkg_second_round)
-  [Function `respond_dwallet_dkg_second_round`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_dwallet_dkg_second_round)
-  [Function `request_re_encrypt_user_share_for`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_re_encrypt_user_share_for)
-  [Function `respond_re_encrypt_user_share_for`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_re_encrypt_user_share_for)
-  [Function `accept_encrypted_user_share`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_accept_encrypted_user_share)
-  [Function `request_ecdsa_presign`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_ecdsa_presign)
-  [Function `mock_create_presign`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_mock_create_presign)
-  [Function `respond_ecdsa_presign`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_ecdsa_presign)
-  [Function `is_ecdsa_presign_valid`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_is_ecdsa_presign_valid)
-  [Function `emit_ecdsa_sign_event`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_emit_ecdsa_sign_event)
    -  [Effects](#@Effects_24)
    -  [Aborts](#@Aborts_25)
-  [Function `request_ecdsa_sign`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_ecdsa_sign)
    -  [Effects](#@Effects_26)
    -  [Aborts](#@Aborts_27)
    -  [Parameters](#@Parameters_28)
    -  [Type Parameters](#@Type_Parameters_29)
-  [Function `request_ecdsa_future_sign`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_ecdsa_future_sign)
-  [Function `respond_ecdsa_future_sign`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_ecdsa_future_sign)
-  [Function `verify_ecdsa_partial_user_signature_cap`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_verify_ecdsa_partial_user_signature_cap)
-  [Function `request_ecdsa_sign_with_partial_user_signatures`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_ecdsa_sign_with_partial_user_signatures)
        -  [Type Parameters](#@Type_Parameters_30)
        -  [Parameters](#@Parameters_31)
        -  [Notes](#@Notes_32)
-  [Function `compare_ecdsa_partial_user_signatures_with_approvals`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_compare_ecdsa_partial_user_signatures_with_approvals)
-  [Function `respond_ecdsa_sign`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_ecdsa_sign)
-  [Function `process_checkpoint_message_by_quorum`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_process_checkpoint_message_by_quorum)
-  [Function `process_checkpoint_message`](#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_process_checkpoint_message)


<pre><code><b>use</b> (ika=0x0)::ika;
<b>use</b> (ika_system=0x0)::<b>address</b>;
<b>use</b> (ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee">bls_committee</a>;
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
<b>use</b> <a href="../sui/vec_set.md#sui_vec_set">sui::vec_set</a>;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner"></a>

## Struct `DWalletCoordinatorInner`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>current_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>sessions: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;u64, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletSession">dwallet_2pc_mpc_secp256k1_inner::DWalletSession</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>session_start_events: <a href="../sui/bag.md#sui_bag_Bag">sui::bag::Bag</a></code>
</dt>
<dd>
</dd>
<dt>
<code>number_of_completed_sessions: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>started_immediate_sessions_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>completed_immediate_sessions_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>next_session_sequence_number: u64</code>
</dt>
<dd>
 The last session sequence number that an event was emitted for.
 i.e, the user requested this session, and the event was emitted for it.
</dd>
<dt>
<code>last_session_to_complete_in_current_epoch: u64</code>
</dt>
<dd>
 The last MPC session to process in the current epoch.
 Validators should complete every session they start before switching epochs.
</dd>
<dt>
<code>locked_last_session_to_complete_in_current_epoch: bool</code>
</dt>
<dd>
 Denotes whether the <code>last_session_to_complete_in_current_epoch</code> field is locked or not.
 This field gets locked before performing the epoch switch.
</dd>
<dt>
<code>max_active_sessions_buffer: u64</code>
</dt>
<dd>
 The maximum number of active MPC sessions Ika nodes may run during an epoch.
 Validators should complete every session they start before switching epochs.
</dd>
<dt>
<code>dwallets: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet">dwallet_2pc_mpc_secp256k1_inner::DWallet</a>&gt;</code>
</dt>
<dd>
 The key is the ID of <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet">DWallet</a></code>.
</dd>
<dt>
<code>dwallet_network_decryption_keys: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletNetworkDecryptionKey">dwallet_2pc_mpc_secp256k1_inner::DWalletNetworkDecryptionKey</a>&gt;</code>
</dt>
<dd>
 The key is the ID of <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletNetworkDecryptionKey">DWalletNetworkDecryptionKey</a></code>.
</dd>
<dt>
<code>encryption_keys: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;<b>address</b>, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptionKey">dwallet_2pc_mpc_secp256k1_inner::EncryptionKey</a>&gt;</code>
</dt>
<dd>
 A table mapping user addresses to encryption key object IDs.
</dd>
<dt>
<code>ecdsa_partial_centralized_signed_messages: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPartialUserSignature">dwallet_2pc_mpc_secp256k1_inner::ECDSAPartialUserSignature</a>&gt;</code>
</dt>
<dd>
 A table mapping id to their partial centralized signed messages.
</dd>
<dt>
<code>pricing: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">dwallet_pricing::DWalletPricing2PcMpcSecp256K1</a></code>
</dt>
<dd>
 The computation IKA price per unit size for the current epoch.
</dd>
<dt>
<code>gas_fee_reimbursement_sui: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;</code>
</dt>
<dd>
 Sui gas fee reimbursement to fund the network writing tx responses to sui.
</dd>
<dt>
<code>consensus_validation_fee_charged_ika: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;</code>
</dt>
<dd>
 The fees paid for consensus validation in IKA.
</dd>
<dt>
<code>active_committee: (ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">bls_committee::BlsCommittee</a></code>
</dt>
<dd>
 The active committees.
</dd>
<dt>
<code>previous_committee: (ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">bls_committee::BlsCommittee</a></code>
</dt>
<dd>
 The previous committee.
</dd>
<dt>
<code>total_messages_processed: u64</code>
</dt>
<dd>
 The total messages processed.
</dd>
<dt>
<code>last_processed_checkpoint_sequence_number: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
 The last checkpoint sequence number processed.
</dd>
<dt>
<code>extra_fields: <a href="../sui/bag.md#sui_bag_Bag">sui::bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletSessionEventKey"></a>

## Struct `DWalletSessionEventKey`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletSessionEventKey">DWalletSessionEventKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletSession"></a>

## Struct `DWalletSession`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletSession">DWalletSession</a> <b>has</b> key, store
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
<code>session_sequence_number: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>consensus_validation_fee_charged_ika: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;</code>
</dt>
<dd>
 The fees paid for consensus validation in IKA.
</dd>
<dt>
<code>computation_fee_charged_ika: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;</code>
</dt>
<dd>
 The fees paid for computation in IKA.
</dd>
<dt>
<code>gas_fee_reimbursement_sui: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;</code>
</dt>
<dd>
 Sui gas fee reimbursement to fund the network writing tx responses to sui.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCap"></a>

## Struct `DWalletCap`

Represents a capability granting control over a specific dWallet.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCap">DWalletCap</a> <b>has</b> key, store
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
<code>dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletNetworkDecryptionKeyCap"></a>

## Struct `DWalletNetworkDecryptionKeyCap`

Represents a capability granting control over a specific dWallet network decryption key.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletNetworkDecryptionKeyCap">DWalletNetworkDecryptionKeyCap</a> <b>has</b> key, store
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
<code>dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletNetworkDecryptionKey"></a>

## Struct `DWalletNetworkDecryptionKey`

<code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletNetworkDecryptionKey">DWalletNetworkDecryptionKey</a></code> represents a network decryption key of
the homomorphically encrypted network share.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletNetworkDecryptionKey">DWalletNetworkDecryptionKey</a> <b>has</b> key, store
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
<code>dwallet_network_decryption_key_cap_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>current_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reconfiguration_public_outputs: <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, <a href="../sui/table_vec.md#sui_table_vec_TableVec">sui::table_vec::TableVec</a>&lt;vector&lt;u8&gt;&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>network_dkg_public_output: <a href="../sui/table_vec.md#sui_table_vec_TableVec">sui::table_vec::TableVec</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>computation_fee_charged_ika: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;</code>
</dt>
<dd>
 The fees paid for computation in IKA.
</dd>
<dt>
<code>state: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletNetworkDecryptionKeyState">dwallet_2pc_mpc_secp256k1_inner::DWalletNetworkDecryptionKeyState</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptionKey"></a>

## Struct `EncryptionKey`

Represents an encryption key used to encrypt a dWallet centralized (user) secret key share.

Encryption keys facilitate secure data transfer between accounts on the
Ika by ensuring that sensitive information remains confidential during transmission.
Each address on the Ika is associated with a unique encryption key.
When an external party intends to send encrypted data to a particular account, they use the recipient’s
encryption key to encrypt the data. The recipient is then the sole entity capable of decrypting
and accessing this information, ensuring secure, end-to-end encryption.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptionKey">EncryptionKey</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
 Unique identifier for the <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptionKey">EncryptionKey</a></code>.
</dd>
<dt>
<code>created_at_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>encryption_key: vector&lt;u8&gt;</code>
</dt>
<dd>
 Serialized encryption key.
</dd>
<dt>
<code>encryption_key_signature: vector&lt;u8&gt;</code>
</dt>
<dd>
 Signature for the encryption key, signed by the <code>signer_public_key</code>.
</dd>
<dt>
<code>signer_public_key: vector&lt;u8&gt;</code>
</dt>
<dd>
 The public key that was used to sign the <code>encryption_key</code>.
</dd>
<dt>
<code>signer_address: <b>address</b></code>
</dt>
<dd>
 Address of the encryption key owner.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptedUserSecretKeyShare"></a>

## Struct `EncryptedUserSecretKeyShare`

A verified Encrypted dWallet centralized secret key share.

This struct represents an encrypted centralized secret key share tied to
a specific dWallet (<code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet">DWallet</a></code>).
It includes cryptographic proof that the encryption is valid and securely linked
to the associated <code>dWallet</code>.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
 A unique identifier for this encrypted user share object.
</dd>
<dt>
<code>created_at_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The ID of the dWallet associated with this encrypted secret share.
</dd>
<dt>
<code>encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;</code>
</dt>
<dd>
 The encrypted centralized secret key share along with a cryptographic proof
 that the encryption corresponds to the dWallet's secret key share.
</dd>
<dt>
<code>encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The ID of the <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptionKey">EncryptionKey</a></code> object used to encrypt the secret share.
</dd>
<dt>
<code>encryption_key_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>source_encrypted_user_secret_key_share_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;</code>
</dt>
<dd>
 The ID of the <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a></code> the secret was re-encrypted from (None if created during dkg).
</dd>
<dt>
<code>state: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptedUserSecretKeyShareState">dwallet_2pc_mpc_secp256k1_inner::EncryptedUserSecretKeyShareState</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_UnverifiedECDSAPartialUserSignatureCap"></a>

## Struct `UnverifiedECDSAPartialUserSignatureCap`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_UnverifiedECDSAPartialUserSignatureCap">UnverifiedECDSAPartialUserSignatureCap</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
 A unique identifier for this object.
</dd>
<dt>
<code>partial_centralized_signed_message_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The unique identifier of the associated PartialCentralizedSignedMessage.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_VerifiedECDSAPartialUserSignatureCap"></a>

## Struct `VerifiedECDSAPartialUserSignatureCap`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_VerifiedECDSAPartialUserSignatureCap">VerifiedECDSAPartialUserSignatureCap</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
 A unique identifier for this object.
</dd>
<dt>
<code>partial_centralized_signed_message_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The unique identifier of the associated PartialCentralizedSignedMessage.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPartialUserSignature"></a>

## Struct `ECDSAPartialUserSignature`

Message that have been signed by a user, a.k.a the centralized party,
but not yet by the blockchain.
Used for scenarios where the user needs to first agree to sign some transaction,
and the blockchain signs this transaction later,
when some other conditions are met.

Can be used to implement an order-book-based exchange, for example.
User <code>A</code> first agrees to buy BTC with ETH at price X, and signs a transaction with this information.
When a matching user <code>B</code>, that agrees to sell BTC for ETH at price X,
signs a transaction with this information,
the blockchain can sign both transactions, and the exchange is completed.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPartialUserSignature">ECDSAPartialUserSignature</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
 A unique identifier for this object.
</dd>
<dt>
<code>created_at_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>presign_cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresignCap">dwallet_2pc_mpc_secp256k1_inner::ECDSAPresignCap</a></code>
</dt>
<dd>
</dd>
<dt>
<code>cap_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>hash_scheme: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>message: vector&lt;u8&gt;</code>
</dt>
<dd>
 The messages that are being signed.
</dd>
<dt>
<code>message_centralized_signature: vector&lt;u8&gt;</code>
</dt>
<dd>
 The centralized party signature of a message.
</dd>
<dt>
<code>state: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPartialUserSignatureState">dwallet_2pc_mpc_secp256k1_inner::ECDSAPartialUserSignatureState</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet"></a>

## Struct `DWallet`

<code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet">DWallet</a></code> represents a decentralized wallet (dWallet) that is
created after the Distributed key generation (DKG) process.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet">DWallet</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
 Unique identifier for the dWallet.
</dd>
<dt>
<code>created_at_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>dwallet_cap_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The ID of the capability associated with this dWallet.
</dd>
<dt>
<code>dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The MPC network decryption key id that is used to decrypt this dWallet.
</dd>
<dt>
<code>encrypted_user_secret_key_shares: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptedUserSecretKeyShare">dwallet_2pc_mpc_secp256k1_inner::EncryptedUserSecretKeyShare</a>&gt;</code>
</dt>
<dd>
 A table mapping id to their encryption key object.
</dd>
<dt>
<code>ecdsa_presigns: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresign">dwallet_2pc_mpc_secp256k1_inner::ECDSAPresign</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>ecdsa_signs: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSASign">dwallet_2pc_mpc_secp256k1_inner::ECDSASign</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>state: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletState">dwallet_2pc_mpc_secp256k1_inner::DWalletState</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresignCap"></a>

## Struct `ECDSAPresignCap`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresignCap">ECDSAPresignCap</a> <b>has</b> key, store
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
<code>dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the associated dWallet.
</dd>
<dt>
<code>presign_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresign"></a>

## Struct `ECDSAPresign`

Represents the result of the second and final presign round.
This struct links the results of both presign rounds to a specific dWallet ID.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresign">ECDSAPresign</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
 Unique identifier for the presign object.
</dd>
<dt>
<code>created_at_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the associated dWallet.
</dd>
<dt>
<code>cap_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>state: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresignState">dwallet_2pc_mpc_secp256k1_inner::ECDSAPresignState</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSASign"></a>

## Struct `ECDSASign`

The output of a batched Sign session.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSASign">ECDSASign</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
 A unique identifier for the batched sign output.
</dd>
<dt>
<code>created_at_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The unique identifier of the associated dWallet.
</dd>
<dt>
<code>session_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The session identifier for the sign process.
</dd>
<dt>
<code>state: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSASignState">dwallet_2pc_mpc_secp256k1_inner::ECDSASignState</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletEvent"></a>

## Struct `DWalletEvent`



<pre><code><b>public</b> <b>struct</b> DWalletEventE <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>session_sequence_number: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>session_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>event_data: E</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CreatedEncryptionKeyEvent"></a>

## Struct `CreatedEncryptionKeyEvent`

Event emitted when an encryption key is created.

This event is emitted after the blockchain verifies the encryption key's validity
and creates the corresponding <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptionKey">EncryptionKey</a></code> object.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CreatedEncryptionKeyEvent">CreatedEncryptionKeyEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The unique identifier of the created <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptionKey">EncryptionKey</a></code> object.
</dd>
<dt>
<code>signer_address: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletNetworkDKGDecryptionKeyRequestEvent"></a>

## Struct `DWalletNetworkDKGDecryptionKeyRequestEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletNetworkDKGDecryptionKeyRequestEvent">DWalletNetworkDKGDecryptionKeyRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletDecryptionKeyReshareRequestEvent"></a>

## Struct `DWalletDecryptionKeyReshareRequestEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletDecryptionKeyReshareRequestEvent">DWalletDecryptionKeyReshareRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedDWalletDecryptionKeyReshareEvent"></a>

## Struct `CompletedDWalletDecryptionKeyReshareEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedDWalletDecryptionKeyReshareEvent">CompletedDWalletDecryptionKeyReshareEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedDWalletNetworkDKGDecryptionKeyEvent"></a>

## Struct `CompletedDWalletNetworkDKGDecryptionKeyEvent`

An event emitted when the first round of the DKG process is completed.

This event is emitted by the blockchain to notify the user about
the completion of the first round.
The user should catch this event to generate inputs for
the second round and call the <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_dwallet_dkg_second_round">request_dwallet_dkg_second_round</a>()</code> function.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedDWalletNetworkDKGDecryptionKeyEvent">CompletedDWalletNetworkDKGDecryptionKeyEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletDKGFirstRoundRequestEvent"></a>

## Struct `DWalletDKGFirstRoundRequestEvent`

Event emitted to start the first round of the DKG process.

This event is caught by the blockchain, which is then using it to
initiate the first round of the DKG.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletDKGFirstRoundRequestEvent">DWalletDKGFirstRoundRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The unique session identifier for the DKG process.
</dd>
<dt>
<code>dwallet_cap_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The identifier for the dWallet capability.
</dd>
<dt>
<code>dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The MPC network decryption key id that is used to decrypt associated dWallet.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedDKGFirstdRoundEvent"></a>

## Struct `CompletedDKGFirstdRoundEvent`

An event emitted when the first round of the DKG process is completed.

This event is emitted by the blockchain to notify the user about
the completion of the first round.
The user should catch this event to generate inputs for
the second round and call the <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_dwallet_dkg_second_round">request_dwallet_dkg_second_round</a>()</code> function.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedDKGFirstdRoundEvent">CompletedDKGFirstdRoundEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The unique session identifier for the DKG process.
</dd>
<dt>
<code>first_round_output: vector&lt;u8&gt;</code>
</dt>
<dd>
 The decentralized public output data produced by the first round of the DKG process.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletDKGSecondRoundRequestEvent"></a>

## Struct `DWalletDKGSecondRoundRequestEvent`

Event emitted to initiate the second round of the DKG process.

This event is emitted to notify Validators to begin the second round of the DKG.
It contains all necessary data to ensure proper continuation of the process.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletDKGSecondRoundRequestEvent">DWalletDKGSecondRoundRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The unique session identifier for the DWallet.
</dd>
<dt>
<code>first_round_output: vector&lt;u8&gt;</code>
</dt>
<dd>
 The output from the first round of the DKG process.
</dd>
<dt>
<code>centralized_public_key_share_and_proof: vector&lt;u8&gt;</code>
</dt>
<dd>
 A serialized vector containing the centralized public key share and its proof.
</dd>
<dt>
<code>dwallet_cap_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The unique identifier of the dWallet capability associated with this session.
</dd>
<dt>
<code>encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;</code>
</dt>
<dd>
 Encrypted centralized secret key share and the associated cryptographic proof of encryption.
</dd>
<dt>
<code>encryption_key: vector&lt;u8&gt;</code>
</dt>
<dd>
 The <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptionKey">EncryptionKey</a></code> object used for encrypting the secret key share.
</dd>
<dt>
<code>encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The unique identifier of the <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptionKey">EncryptionKey</a></code> object.
</dd>
<dt>
<code>encryption_key_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>user_public_output: vector&lt;u8&gt;</code>
</dt>
<dd>
 The public output of the centralized party in the DKG process.
</dd>
<dt>
<code>singer_public_key: vector&lt;u8&gt;</code>
</dt>
<dd>
 The Ed25519 public key of the initiator,
 used to verify the signature on the centralized public output.
</dd>
<dt>
<code>dwallet_mpc_network_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The MPC network decryption key id that is used to decrypt associated dWallet.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedDWalletDKGSecondRoundEvent"></a>

## Struct `CompletedDWalletDKGSecondRoundEvent`

Event emitted upon the completion of the second (and final) round of the
Distributed Key Generation (DKG).

This event provides all necessary data generated from the second
round of the DKG process.
Emitted to notify the centralized party.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedDWalletDKGSecondRoundEvent">CompletedDWalletDKGSecondRoundEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The identifier of the dWallet created as a result of the DKG process.
</dd>
<dt>
<code>public_output: vector&lt;u8&gt;</code>
</dt>
<dd>
 The public output for the second round of the DKG process.
</dd>
<dt>
<code>encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>session_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_RejectedDWalletDKGSecondRoundEvent"></a>

## Struct `RejectedDWalletDKGSecondRoundEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_RejectedDWalletDKGSecondRoundEvent">RejectedDWalletDKGSecondRoundEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The identifier of the dWallet created as a result of the DKG process.
</dd>
<dt>
<code>public_output: vector&lt;u8&gt;</code>
</dt>
<dd>
 The public output for the second round of the DKG process.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptedShareVerificationRequestEvent"></a>

## Struct `EncryptedShareVerificationRequestEvent`

Event emitted to start an encrypted dWallet centralized (user) key share
verification process.
Ika does not support native functions, so an event is emitted and
caught by the blockchain, which then starts the verification process,
similar to the MPC processes.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptedShareVerificationRequestEvent">EncryptedShareVerificationRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;</code>
</dt>
<dd>
 Encrypted centralized secret key share and the associated cryptographic proof of encryption.
</dd>
<dt>
<code>public_output: vector&lt;u8&gt;</code>
</dt>
<dd>
 The public output of the centralized party,
 belongs to the dWallet that its centralized
 secret share is being encrypted.
 This is not passed by the user,
 but taken from the blockchain during event creation.
</dd>
<dt>
<code>dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The ID of the dWallet that this encrypted secret key share belongs to.
</dd>
<dt>
<code>encryption_key: vector&lt;u8&gt;</code>
</dt>
<dd>
 The encryption key used to encrypt the secret key share with.
</dd>
<dt>
<code>encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptionKey">EncryptionKey</a></code> Move object ID.
</dd>
<dt>
<code>encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>source_encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>dwallet_mpc_network_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedEncryptedShareVerificationEvent"></a>

## Struct `CompletedEncryptedShareVerificationEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedEncryptedShareVerificationEvent">CompletedEncryptedShareVerificationEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The ID of the <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a></code> Move object.
</dd>
<dt>
<code>dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The ID of the dWallet associated with this encrypted secret share.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_RejectedEncryptedShareVerificationEvent"></a>

## Struct `RejectedEncryptedShareVerificationEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_RejectedEncryptedShareVerificationEvent">RejectedEncryptedShareVerificationEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The ID of the <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a></code> Move object.
</dd>
<dt>
<code>dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The ID of the dWallet associated with this encrypted secret share.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_AcceptReEncryptedUserShareEvent"></a>

## Struct `AcceptReEncryptedUserShareEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_AcceptReEncryptedUserShareEvent">AcceptReEncryptedUserShareEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The ID of the <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a></code> Move object.
</dd>
<dt>
<code>dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The ID of the dWallet associated with this encrypted secret share.
</dd>
<dt>
<code>user_output_signature: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>encryption_key_address: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresignRequestEvent"></a>

## Struct `ECDSAPresignRequestEvent`

Event emitted to initiate the first round of a Presign session.

This event is used to signal Validators to start the
first round of the Presign process.
The event includes all necessary details to link
the session to the corresponding dWallet
and DKG process.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresignRequestEvent">ECDSAPresignRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the associated dWallet.
</dd>
<dt>
<code>presign_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>dwallet_public_output: vector&lt;u8&gt;</code>
</dt>
<dd>
 The output produced by the DKG process,
 used as input for the Presign session.
</dd>
<dt>
<code>dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The MPC network decryption key id that is used to decrypt associated dWallet.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedECDSAPresignEvent"></a>

## Struct `CompletedECDSAPresignEvent`

Event emitted when the presign batch is completed.

This event indicates the successful completion of a batched presign process.
It provides details about the presign objects created and their associated metadata.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedECDSAPresignEvent">CompletedECDSAPresignEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The ID of the dWallet associated with this batch.
</dd>
<dt>
<code>session_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The session ID.
</dd>
<dt>
<code>presign_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>presign: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_RejectedECDSAPresignEvent"></a>

## Struct `RejectedECDSAPresignEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_RejectedECDSAPresignEvent">RejectedECDSAPresignEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The ID of the dWallet associated with this batch.
</dd>
<dt>
<code>session_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The session ID.
</dd>
<dt>
<code>presign_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSASignRequestEvent"></a>

## Struct `ECDSASignRequestEvent`

Event emitted to initiate the signing process.

This event is captured by Validators to start the signing protocol.
It includes all the necessary information to link the signing process
to a specific dWallet, and batched process.
D: The type of data that can be stored with the object,
specific to each Digital Signature Algorithm.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSASignRequestEvent">ECDSASignRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>sign_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The unique identifier for the dWallet used in the session.
</dd>
<dt>
<code>dwallet_public_output: vector&lt;u8&gt;</code>
</dt>
<dd>
 The output from the dWallet DKG process used in this session.
</dd>
<dt>
<code>hash_scheme: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>message: vector&lt;u8&gt;</code>
</dt>
<dd>
 The message to be signed in this session.
</dd>
<dt>
<code>dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The MPC network decryption key id that is used to decrypt associated dWallet.
</dd>
<dt>
<code>presign_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The presign object ID, this ID will
 be used as the singature MPC protocol ID.
</dd>
<dt>
<code>presign: vector&lt;u8&gt;</code>
</dt>
<dd>
 The presign protocol output as bytes.
</dd>
<dt>
<code>message_centralized_signature: vector&lt;u8&gt;</code>
</dt>
<dd>
 The centralized party signature of a message.
</dd>
<dt>
<code>is_future_sign: bool</code>
</dt>
<dd>
 Indicates whether the future sign feature was used to start the session.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAFutureSignRequestEvent"></a>

## Struct `ECDSAFutureSignRequestEvent`

Event emitted when a [<code>PartialCentralizedSignedMessages</code>] object is created.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAFutureSignRequestEvent">ECDSAFutureSignRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>partial_centralized_signed_message_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>message: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>presign: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>dwallet_public_output: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>hash_scheme: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>message_centralized_signature: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>dwallet_mpc_network_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedECDSAFutureSignEvent"></a>

## Struct `CompletedECDSAFutureSignEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedECDSAFutureSignEvent">CompletedECDSAFutureSignEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>session_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>partial_centralized_signed_message_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_RejectedECDSAFutureSignEvent"></a>

## Struct `RejectedECDSAFutureSignEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_RejectedECDSAFutureSignEvent">RejectedECDSAFutureSignEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>session_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>partial_centralized_signed_message_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedECDSASignEvent"></a>

## Struct `CompletedECDSASignEvent`

Event emitted to signal the completion of a Sign process.

This event contains signatures for all signed messages in the batch.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedECDSASignEvent">CompletedECDSASignEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>sign_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>session_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The session identifier for the signing process.
</dd>
<dt>
<code>signature: vector&lt;u8&gt;</code>
</dt>
<dd>
 List of signatures in the same order as the sign function message approvals input.
</dd>
<dt>
<code>is_future_sign: bool</code>
</dt>
<dd>
 Indicates whether the future sign feature was used to start the session.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_RejectedECDSASignEvent"></a>

## Struct `RejectedECDSASignEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_RejectedECDSASignEvent">RejectedECDSASignEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>sign_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>session_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The session identifier for the signing process.
</dd>
<dt>
<code>is_future_sign: bool</code>
</dt>
<dd>
 Indicates whether the future sign feature was used to start the session.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_SystemCheckpointInfoEvent"></a>

## Struct `SystemCheckpointInfoEvent`

Event containing system-level checkpoint information, emitted during
the checkpoint submmision message.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_SystemCheckpointInfoEvent">SystemCheckpointInfoEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>sequence_number: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_MessageApproval"></a>

## Struct `MessageApproval`

Represents a message that was approved as part of a dWallet process.

This struct binds the message to a specific <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCap">DWalletCap</a></code> for
traceability and accountability within the system.


<a name="@Fields_0"></a>

##### Fields

- **<code>dwallet_cap_id</code>**: The identifier of the dWallet capability
associated with this approval.
- **<code>hash_scheme</code>**: The message hash scheme.
- **<code>message</code>**: The message that has been approved.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_MessageApproval">MessageApproval</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>hash_scheme: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>message: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletNetworkDecryptionKeyState"></a>

## Enum `DWalletNetworkDecryptionKeyState`



<pre><code><b>public</b> <b>enum</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletNetworkDecryptionKeyState">DWalletNetworkDecryptionKeyState</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Variants</summary>


<dl>
<dt>
Variant <code>AwaitingNetworkDKG</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>AwaitingNetworkReconfiguration</code>
</dt>
<dd>
 Reconfiguration request was sent to the network, but didn't finish yet.
</dd>
<dt>
Variant <code>AwaitingNextEpochReconfiguration</code>
</dt>
<dd>
 Reconfiguration request finished, but we didn't switch an epoch yet.
</dd>
<dt>
Variant <code>NetworkDKGCompleted</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>NetworkReconfigurationCompleted</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptedUserSecretKeyShareState"></a>

## Enum `EncryptedUserSecretKeyShareState`



<pre><code><b>public</b> <b>enum</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptedUserSecretKeyShareState">EncryptedUserSecretKeyShareState</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Variants</summary>


<dl>
<dt>
Variant <code>AwaitingNetworkVerification</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>KeyHolderSiged</code>
</dt>
<dd>
</dd>

<dl>
<dt>
<code>user_output_signature: vector&lt;u8&gt;</code>
</dt>
<dd>
 The signed public share corresponding to the encrypted secret key share,
 used to verify its authenticity.
</dd>
</dl>

<dt>
Variant <code>NetworkVerificationCompleted</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>NetworkVerificationRejected</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPartialUserSignatureState"></a>

## Enum `ECDSAPartialUserSignatureState`



<pre><code><b>public</b> <b>enum</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPartialUserSignatureState">ECDSAPartialUserSignatureState</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Variants</summary>


<dl>
<dt>
Variant <code>AwaitingNetworkVerification</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>NetworkVerificationCompleted</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>NetworkVerificationRejected</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletState"></a>

## Enum `DWalletState`



<pre><code><b>public</b> <b>enum</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletState">DWalletState</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Variants</summary>


<dl>
<dt>
Variant <code>Active</code>
</dt>
<dd>
</dd>

<dl>
<dt>
<code>public_output: vector&lt;u8&gt;</code>
</dt>
<dd>
 The output of the DKG process.
</dd>
</dl>

<dt>
Variant <code>AwaitingNetworkVerification</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>AwaitingUser</code>
</dt>
<dd>
</dd>

<dl>
<dt>
<code>first_round_output: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>

<dt>
Variant <code>NetworkRejectedSecondRound</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>Requested</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresignState"></a>

## Enum `ECDSAPresignState`



<pre><code><b>public</b> <b>enum</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresignState">ECDSAPresignState</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Variants</summary>


<dl>
<dt>
Variant <code>Completed</code>
</dt>
<dd>
</dd>

<dl>
<dt>
<code>presign: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>

<dt>
Variant <code>NetworkRejected</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>Requested</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSASignState"></a>

## Enum `ECDSASignState`



<pre><code><b>public</b> <b>enum</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSASignState">ECDSASignState</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Variants</summary>


<dl>
<dt>
Variant <code>Completed</code>
</dt>
<dd>
</dd>

<dl>
<dt>
<code>signature: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>

<dt>
Variant <code>NetworkRejected</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>Requested</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_1"></a>

## Constants


<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CHECKPOINT_MESSAGE_INTENT"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CHECKPOINT_MESSAGE_INTENT">CHECKPOINT_MESSAGE_INTENT</a>: vector&lt;u8&gt; = vector[1, 0, 0];
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EActiveBlsCommitteeMustInitialize"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EActiveBlsCommitteeMustInitialize">EActiveBlsCommitteeMustInitialize</a>: vector&lt;u8&gt; = b"First active committee must initialize.";
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECannotAdvanceEpoch"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECannotAdvanceEpoch">ECannotAdvanceEpoch</a>: u64 = 16;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EDWalletInactive"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EDWalletInactive">EDWalletInactive</a>: u64 = 2;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EDWalletMismatch"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EDWalletMismatch">EDWalletMismatch</a>: u64 = 1;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EDWalletNetworkDecryptionKeyNotActive"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EDWalletNetworkDecryptionKeyNotActive">EDWalletNetworkDecryptionKeyNotActive</a>: u64 = 14;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EDWalletNetworkDecryptionKeyNotExist"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EDWalletNetworkDecryptionKeyNotExist">EDWalletNetworkDecryptionKeyNotExist</a>: u64 = 5;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EDWalletNotExists"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EDWalletNotExists">EDWalletNotExists</a>: u64 = 3;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EIncorrectCap"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EIncorrectCap">EIncorrectCap</a>: u64 = 11;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EIncorrectEpochInCheckpoint"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EIncorrectEpochInCheckpoint">EIncorrectEpochInCheckpoint</a>: vector&lt;u8&gt; = b"The checkpoint epoch is incorrect.";
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EInvalidEncryptionKeySignature"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EInvalidEncryptionKeySignature">EInvalidEncryptionKeySignature</a>: u64 = 6;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EInvalidHashScheme"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EInvalidHashScheme">EInvalidHashScheme</a>: u64 = 8;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EInvalidPresign"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EInvalidPresign">EInvalidPresign</a>: u64 = 15;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EInvalidSource"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EInvalidSource">EInvalidSource</a>: u64 = 13;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EMessageApprovalMismatch"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EMessageApprovalMismatch">EMessageApprovalMismatch</a>: u64 = 7;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EPresignNotExist"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EPresignNotExist">EPresignNotExist</a>: u64 = 10;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ESignWrongState"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ESignWrongState">ESignWrongState</a>: u64 = 9;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EUnverifiedCap"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EUnverifiedCap">EUnverifiedCap</a>: u64 = 12;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EWrongCheckpointSequenceNumber"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EWrongCheckpointSequenceNumber">EWrongCheckpointSequenceNumber</a>: vector&lt;u8&gt; = b"The checkpoint sequence number should be the expected next one.";
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EWrongState"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EWrongState">EWrongState</a>: u64 = 4;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_KECCAK256"></a>

Supported hash schemes for message signing.


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_KECCAK256">KECCAK256</a>: u8 = 0;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_SHA256"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_SHA256">SHA256</a>: u8 = 1;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_lock_last_active_session_sequence_number"></a>

## Function `lock_last_active_session_sequence_number`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_lock_last_active_session_sequence_number">lock_last_active_session_sequence_number</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_lock_last_active_session_sequence_number">lock_last_active_session_sequence_number</a>(self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>) {
    self.locked_last_session_to_complete_in_current_epoch = <b>true</b>;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_create_dwallet_coordinator_inner"></a>

## Function `create_dwallet_coordinator_inner`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_create_dwallet_coordinator_inner">create_dwallet_coordinator_inner</a>(current_epoch: u64, active_committee: (ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">bls_committee::BlsCommittee</a>, pricing: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">dwallet_pricing::DWalletPricing2PcMpcSecp256K1</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_create_dwallet_coordinator_inner">create_dwallet_coordinator_inner</a>(
    current_epoch: u64,
    active_committee: BlsCommittee,
    pricing: DWalletPricing2PcMpcSecp256K1,
    ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a> {
    <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a> {
        current_epoch,
        sessions: object_table::new(ctx),
        session_start_events: bag::new(ctx),
        number_of_completed_sessions: 0,
        next_session_sequence_number: 0,
        last_session_to_complete_in_current_epoch: 0,
        // TODO (#856): Allow configuring the max_active_session_buffer field
        max_active_sessions_buffer: 100,
        locked_last_session_to_complete_in_current_epoch: <b>false</b>,
        dwallets: object_table::new(ctx),
        dwallet_network_decryption_keys: object_table::new(ctx),
        encryption_keys: object_table::new(ctx),
        ecdsa_partial_centralized_signed_messages: object_table::new(ctx),
        pricing,
        gas_fee_reimbursement_sui: balance::zero(),
        consensus_validation_fee_charged_ika: balance::zero(),
        active_committee,
        previous_committee: <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_empty">bls_committee::empty</a>(),
        total_messages_processed: 0,
        last_processed_checkpoint_sequence_number: option::none(),
        completed_immediate_sessions_count: 0,
        started_immediate_sessions_count: 0,
        extra_fields: bag::new(ctx),
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_dwallet_network_decryption_key_dkg"></a>

## Function `request_dwallet_network_decryption_key_dkg`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_dwallet_network_decryption_key_dkg">request_dwallet_network_decryption_key_dkg</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletNetworkDecryptionKeyCap">dwallet_2pc_mpc_secp256k1_inner::DWalletNetworkDecryptionKeyCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_dwallet_network_decryption_key_dkg">request_dwallet_network_decryption_key_dkg</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletNetworkDecryptionKeyCap">DWalletNetworkDecryptionKeyCap</a> {
    <b>let</b> id = object::new(ctx);
    <b>let</b> dwallet_network_decryption_key_id = id.to_inner();
    <b>let</b> cap = <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletNetworkDecryptionKeyCap">DWalletNetworkDecryptionKeyCap</a> {
        id: object::new(ctx),
        dwallet_network_decryption_key_id,
    };
    self.dwallet_network_decryption_keys.add(dwallet_network_decryption_key_id, <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletNetworkDecryptionKey">DWalletNetworkDecryptionKey</a> {
        id,
        dwallet_network_decryption_key_cap_id: object::id(&cap),
        current_epoch: self.current_epoch,
        reconfiguration_public_outputs: <a href="../sui/table.md#sui_table_new">sui::table::new</a>(ctx),
        network_dkg_public_output: table_vec::empty(ctx),
        computation_fee_charged_ika: balance::zero(),
        state: DWalletNetworkDecryptionKeyState::AwaitingNetworkDKG,
    });
    event::emit(self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_create_immediate_dwallet_event">create_immediate_dwallet_event</a>(
        dwallet_network_decryption_key_id,
        <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletNetworkDKGDecryptionKeyRequestEvent">DWalletNetworkDKGDecryptionKeyRequestEvent</a> {
            dwallet_network_decryption_key_id
        },
        ctx,
    ));
    cap
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_dwallet_network_decryption_key_dkg"></a>

## Function `respond_dwallet_network_decryption_key_dkg`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_dwallet_network_decryption_key_dkg">respond_dwallet_network_decryption_key_dkg</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, network_public_output: vector&lt;u8&gt;, is_last_chunk: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_dwallet_network_decryption_key_dkg">respond_dwallet_network_decryption_key_dkg</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_network_decryption_key_id: ID,
    network_public_output: vector&lt;u8&gt;,
    is_last_chunk: bool,
) {
    <b>if</b> (is_last_chunk) {
        self.completed_immediate_sessions_count = self.completed_immediate_sessions_count + 1;
    };
    <b>let</b> dwallet_network_decryption_key = self.dwallet_network_decryption_keys.borrow_mut(dwallet_network_decryption_key_id);
    dwallet_network_decryption_key.network_dkg_public_output.push_back(network_public_output);
    dwallet_network_decryption_key.state = match (&dwallet_network_decryption_key.state) {
        DWalletNetworkDecryptionKeyState::AwaitingNetworkDKG =&gt; {
            <b>if</b> (is_last_chunk) {
                event::emit(<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedDWalletNetworkDKGDecryptionKeyEvent">CompletedDWalletNetworkDKGDecryptionKeyEvent</a> {
                    dwallet_network_decryption_key_id,
                });
                DWalletNetworkDecryptionKeyState::NetworkDKGCompleted
            } <b>else</b> {
                DWalletNetworkDecryptionKeyState::AwaitingNetworkDKG
            }
        },
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EWrongState">EWrongState</a>
    };
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_dwallet_network_decryption_key_reconfiguration"></a>

## Function `respond_dwallet_network_decryption_key_reconfiguration`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_dwallet_network_decryption_key_reconfiguration">respond_dwallet_network_decryption_key_reconfiguration</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, public_output: vector&lt;u8&gt;, is_last_chunk: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_dwallet_network_decryption_key_reconfiguration">respond_dwallet_network_decryption_key_reconfiguration</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_network_decryption_key_id: ID,
    public_output: vector&lt;u8&gt;,
    is_last_chunk: bool,
) {
    <b>if</b> (is_last_chunk) {
        self.completed_immediate_sessions_count = self.completed_immediate_sessions_count + 1;
    };
    <b>let</b> dwallet_network_decryption_key = self.dwallet_network_decryption_keys.borrow_mut(dwallet_network_decryption_key_id);
    <b>let</b> next_reconfiguration_public_output = dwallet_network_decryption_key.reconfiguration_public_outputs.borrow_mut(dwallet_network_decryption_key.current_epoch + 1);
    next_reconfiguration_public_output.push_back(public_output);
    dwallet_network_decryption_key.state = match (&dwallet_network_decryption_key.state) {
        DWalletNetworkDecryptionKeyState::AwaitingNetworkReconfiguration =&gt; {
            <b>if</b> (is_last_chunk) {
                event::emit(<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedDWalletDecryptionKeyReshareEvent">CompletedDWalletDecryptionKeyReshareEvent</a> {
                    dwallet_network_decryption_key_id,
                });
                DWalletNetworkDecryptionKeyState::AwaitingNextEpochReconfiguration
            } <b>else</b> {
                DWalletNetworkDecryptionKeyState::AwaitingNetworkReconfiguration
            }
        },
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EWrongState">EWrongState</a>
    };
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_advance_epoch_dwallet_network_decryption_key"></a>

## Function `advance_epoch_dwallet_network_decryption_key`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_advance_epoch_dwallet_network_decryption_key">advance_epoch_dwallet_network_decryption_key</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, cap: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletNetworkDecryptionKeyCap">dwallet_2pc_mpc_secp256k1_inner::DWalletNetworkDecryptionKeyCap</a>): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_advance_epoch_dwallet_network_decryption_key">advance_epoch_dwallet_network_decryption_key</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    cap: &<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletNetworkDecryptionKeyCap">DWalletNetworkDecryptionKeyCap</a>,
): Balance&lt;IKA&gt; {
    <b>let</b> dwallet_network_decryption_key = self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_active_dwallet_network_decryption_key">get_active_dwallet_network_decryption_key</a>(
        cap.dwallet_network_decryption_key_id
    );
    <b>assert</b>!(dwallet_network_decryption_key.dwallet_network_decryption_key_cap_id == cap.id.to_inner(), <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EIncorrectCap">EIncorrectCap</a>);
    dwallet_network_decryption_key.current_epoch = dwallet_network_decryption_key.current_epoch + 1;
    dwallet_network_decryption_key.state = DWalletNetworkDecryptionKeyState::NetworkReconfigurationCompleted;
    <b>let</b> <b>mut</b> epoch_computation_fee_charged_ika = <a href="../sui/balance.md#sui_balance_zero">sui::balance::zero</a>&lt;IKA&gt;();
    epoch_computation_fee_charged_ika.join(dwallet_network_decryption_key.computation_fee_charged_ika.withdraw_all());
    <b>return</b> epoch_computation_fee_charged_ika
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_emit_start_reshare_event"></a>

## Function `emit_start_reshare_event`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_emit_start_reshare_event">emit_start_reshare_event</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, key_cap: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletNetworkDecryptionKeyCap">dwallet_2pc_mpc_secp256k1_inner::DWalletNetworkDecryptionKeyCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_emit_start_reshare_event">emit_start_reshare_event</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>, key_cap: &<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletNetworkDecryptionKeyCap">DWalletNetworkDecryptionKeyCap</a>, ctx: &<b>mut</b> TxContext
) {
    <b>let</b> dwallet_network_decryption_key = self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_active_dwallet_network_decryption_key">get_active_dwallet_network_decryption_key</a>(key_cap.dwallet_network_decryption_key_id);
    dwallet_network_decryption_key.state = DWalletNetworkDecryptionKeyState::AwaitingNetworkReconfiguration;
    dwallet_network_decryption_key.reconfiguration_public_outputs.add(dwallet_network_decryption_key.current_epoch + 1, table_vec::empty(ctx));
    event::emit(self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_create_immediate_dwallet_event">create_immediate_dwallet_event</a>(
        key_cap.dwallet_network_decryption_key_id,
        <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletDecryptionKeyReshareRequestEvent">DWalletDecryptionKeyReshareRequestEvent</a> {
            dwallet_network_decryption_key_id: key_cap.dwallet_network_decryption_key_id
        },
        ctx,
    ));
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_active_dwallet_network_decryption_key"></a>

## Function `get_active_dwallet_network_decryption_key`



<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_active_dwallet_network_decryption_key">get_active_dwallet_network_decryption_key</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletNetworkDecryptionKey">dwallet_2pc_mpc_secp256k1_inner::DWalletNetworkDecryptionKey</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_active_dwallet_network_decryption_key">get_active_dwallet_network_decryption_key</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_network_decryption_key_id: ID,
): &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletNetworkDecryptionKey">DWalletNetworkDecryptionKey</a> {
    <b>let</b> dwallet_network_decryption_key = self.dwallet_network_decryption_keys.borrow_mut(dwallet_network_decryption_key_id);
    <b>assert</b>!(dwallet_network_decryption_key.state != DWalletNetworkDecryptionKeyState::AwaitingNetworkDKG, <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EDWalletNetworkDecryptionKeyNotActive">EDWalletNetworkDecryptionKeyNotActive</a>);
    dwallet_network_decryption_key
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_advance_epoch"></a>

## Function `advance_epoch`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_advance_epoch">advance_epoch</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, next_committee: (ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">bls_committee::BlsCommittee</a>): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_advance_epoch">advance_epoch</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    next_committee: BlsCommittee
): Balance&lt;IKA&gt; {
    <b>assert</b>!(self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_all_current_epoch_sessions_completed">all_current_epoch_sessions_completed</a>(), <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECannotAdvanceEpoch">ECannotAdvanceEpoch</a>);
    self.locked_last_session_to_complete_in_current_epoch = <b>false</b>;
    self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_update_last_session_to_complete_in_current_epoch">update_last_session_to_complete_in_current_epoch</a>();
    self.current_epoch = self.current_epoch + 1;
    self.previous_committee = self.active_committee;
    self.active_committee = next_committee;
    <b>let</b> <b>mut</b> epoch_consensus_validation_fee_charged_ika = <a href="../sui/balance.md#sui_balance_zero">sui::balance::zero</a>&lt;IKA&gt;();
    epoch_consensus_validation_fee_charged_ika.join(self.consensus_validation_fee_charged_ika.withdraw_all());
    <b>return</b> epoch_consensus_validation_fee_charged_ika
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_dwallet"></a>

## Function `get_dwallet`



<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_dwallet">get_dwallet</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet">dwallet_2pc_mpc_secp256k1_inner::DWallet</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_dwallet">get_dwallet</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
): &<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet">DWallet</a> {
    <b>assert</b>!(self.dwallets.contains(dwallet_id), <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EDWalletNotExists">EDWalletNotExists</a>);
    self.dwallets.borrow(dwallet_id)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_dwallet_mut"></a>

## Function `get_dwallet_mut`



<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_dwallet_mut">get_dwallet_mut</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet">dwallet_2pc_mpc_secp256k1_inner::DWallet</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_dwallet_mut">get_dwallet_mut</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
): &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet">DWallet</a> {
    <b>assert</b>!(self.dwallets.contains(dwallet_id), <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EDWalletNotExists">EDWalletNotExists</a>);
    self.dwallets.borrow_mut(dwallet_id)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_validate_active_and_get_public_output"></a>

## Function `validate_active_and_get_public_output`



<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_validate_active_and_get_public_output">validate_active_and_get_public_output</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet">dwallet_2pc_mpc_secp256k1_inner::DWallet</a>): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_validate_active_and_get_public_output">validate_active_and_get_public_output</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet">DWallet</a>,
): &vector&lt;u8&gt; {
    match (&self.state) {
        DWalletState::Active {
            public_output,
        } =&gt; {
            public_output
        },
        DWalletState::Requested | DWalletState::AwaitingUser { .. } | DWalletState::AwaitingNetworkVerification | DWalletState::NetworkRejectedSecondRound =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EDWalletInactive">EDWalletInactive</a>,
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_charge_and_create_current_epoch_dwallet_event"></a>

## Function `charge_and_create_current_epoch_dwallet_event`



<pre><code><b>fun</b> charge_and_create_current_epoch_dwallet_eventE(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, pricing: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">dwallet_pricing::PricingPerOperation</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, event_data: E, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletEvent">dwallet_2pc_mpc_secp256k1_inner::DWalletEvent</a>&lt;E&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>&lt;E: <b>copy</b> + drop + store&gt;(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_network_decryption_key_id: ID,
    pricing: PricingPerOperation,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    event_data: E,
    ctx: &<b>mut</b> TxContext,
): <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletEvent">DWalletEvent</a>&lt;E&gt; {
    <b>assert</b>!(self.dwallet_network_decryption_keys.contains(dwallet_network_decryption_key_id), <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EDWalletNetworkDecryptionKeyNotExist">EDWalletNetworkDecryptionKeyNotExist</a>);
    <b>let</b> computation_fee_charged_ika = payment_ika.split(pricing.computation_ika(), ctx).into_balance();
    <b>let</b> consensus_validation_fee_charged_ika = payment_ika.split(pricing.consensus_validation_ika(), ctx).into_balance();
    <b>let</b> gas_fee_reimbursement_sui = payment_sui.split(pricing.gas_fee_reimbursement_sui(), ctx).into_balance();
    <b>let</b> session_sequence_number = self.next_session_sequence_number;
    <b>let</b> session = <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletSession">DWalletSession</a> {
        id: object::new(ctx),
        session_sequence_number,
        dwallet_network_decryption_key_id,
        consensus_validation_fee_charged_ika,
        computation_fee_charged_ika,
        gas_fee_reimbursement_sui,
    };
    <b>let</b> event = <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletEvent">DWalletEvent</a> {
        epoch: self.current_epoch,
        session_sequence_number,
        session_id: object::id(&session),
        event_data,
    };
    self.session_start_events.add(session.id.to_inner(), event);
    self.sessions.add(session_sequence_number, session);
    self.next_session_sequence_number = session_sequence_number + 1;
    self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_update_last_session_to_complete_in_current_epoch">update_last_session_to_complete_in_current_epoch</a>();
    event
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_create_immediate_dwallet_event"></a>

## Function `create_immediate_dwallet_event`



<pre><code><b>fun</b> create_immediate_dwallet_eventE(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, event_data: E, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletEvent">dwallet_2pc_mpc_secp256k1_inner::DWalletEvent</a>&lt;E&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_create_immediate_dwallet_event">create_immediate_dwallet_event</a>&lt;E: <b>copy</b> + drop + store&gt;(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_network_decryption_key_id: ID,
    event_data: E,
    ctx: &<b>mut</b> TxContext,
): <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletEvent">DWalletEvent</a>&lt;E&gt; {
    <b>assert</b>!(self.dwallet_network_decryption_keys.contains(dwallet_network_decryption_key_id), <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EDWalletNetworkDecryptionKeyNotExist">EDWalletNetworkDecryptionKeyNotExist</a>);
    self.started_immediate_sessions_count = self.started_immediate_sessions_count + 1;
    <b>let</b> event = <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletEvent">DWalletEvent</a> {
        epoch: self.current_epoch,
        session_sequence_number: self.next_session_sequence_number,
        session_id: object::id_from_address(tx_context::fresh_object_address(ctx)),
        event_data,
    };
    // This special logic is here to allow the immediate session have a unique session sequenece number on the one hand,
    // yet ignore it when deciding the last session to complete in the current epoch, <b>as</b> immediate sessions
    // are special sessions that must get completed in the current epoch.
    self.next_session_sequence_number = self.next_session_sequence_number + 1;
    self.number_of_completed_sessions = self.number_of_completed_sessions + 1;
    self.last_session_to_complete_in_current_epoch = self.last_session_to_complete_in_current_epoch + 1;
    event
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_active_dwallet_and_public_output"></a>

## Function `get_active_dwallet_and_public_output`



<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_active_dwallet_and_public_output">get_active_dwallet_and_public_output</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): (&(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet">dwallet_2pc_mpc_secp256k1_inner::DWallet</a>, vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_active_dwallet_and_public_output">get_active_dwallet_and_public_output</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
): (&<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet">DWallet</a>, vector&lt;u8&gt;) {
    <b>assert</b>!(self.dwallets.contains(dwallet_id), <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EDWalletNotExists">EDWalletNotExists</a>);
    <b>let</b> dwallet = self.dwallets.borrow(dwallet_id);
    <b>let</b> public_output = dwallet.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_validate_active_and_get_public_output">validate_active_and_get_public_output</a>();
    (dwallet, *public_output)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_active_dwallet_and_public_output_mut"></a>

## Function `get_active_dwallet_and_public_output_mut`



<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_active_dwallet_and_public_output_mut">get_active_dwallet_and_public_output_mut</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): (&<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet">dwallet_2pc_mpc_secp256k1_inner::DWallet</a>, vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_active_dwallet_and_public_output_mut">get_active_dwallet_and_public_output_mut</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
): (&<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet">DWallet</a>, vector&lt;u8&gt;) {
    <b>assert</b>!(self.dwallets.contains(dwallet_id), <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EDWalletNotExists">EDWalletNotExists</a>);
    <b>let</b> dwallet = self.dwallets.borrow_mut(dwallet_id);
    <b>let</b> public_output = dwallet.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_validate_active_and_get_public_output">validate_active_and_get_public_output</a>();
    (dwallet, *public_output)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_active_encryption_key"></a>

## Function `get_active_encryption_key`

Get the active encryption key ID by its address.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_active_encryption_key">get_active_encryption_key</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, <b>address</b>: <b>address</b>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_active_encryption_key">get_active_encryption_key</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    <b>address</b>: <b>address</b>,
): ID {
    self.encryption_keys.borrow(<b>address</b>).id.to_inner()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_register_encryption_key"></a>

## Function `register_encryption_key`

Registers an encryption key to be used later for encrypting a
centralized secret key share.


<a name="@Parameters_2"></a>

##### Parameters

- <code>encryption_key</code>: The serialized encryption key to be registered.
- <code>encryption_key_signature</code>: The signature of the encryption key, signed by the signer.
- <code>signer_public_key</code>: The public key of the signer used to verify the encryption key signature.
- <code>encryption_key_scheme</code>: The scheme of the encryption key (e.g., Class Groups).
Needed so the TX will get ordered in consensus before getting executed.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_register_encryption_key">register_encryption_key</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, encryption_key: vector&lt;u8&gt;, encryption_key_signature: vector&lt;u8&gt;, signer_public_key: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_register_encryption_key">register_encryption_key</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    encryption_key: vector&lt;u8&gt;,
    encryption_key_signature: vector&lt;u8&gt;,
    signer_public_key: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(
        ed25519_verify(&encryption_key_signature, &signer_public_key, &encryption_key),
        <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EInvalidEncryptionKeySignature">EInvalidEncryptionKeySignature</a>
    );
    <b>let</b> signer_address = <a href="../ika_system/address.md#(ika_system=0x0)_address_ed25519_address">address::ed25519_address</a>(signer_public_key);
    <b>let</b> id = object::new(ctx);
    <b>let</b> encryption_key_id = id.to_inner();
    self.encryption_keys.add(signer_address, <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptionKey">EncryptionKey</a> {
        id,
        created_at_epoch: self.current_epoch,
        encryption_key,
        encryption_key_signature,
        signer_public_key,
        signer_address,
    });
    // Emit an event to signal the creation of the encryption key
    event::emit(<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CreatedEncryptionKeyEvent">CreatedEncryptionKeyEvent</a> {
        encryption_key_id,
        signer_address,
    });
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_create_message_approval"></a>

## Function `create_message_approval`

Creates a <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_MessageApproval">MessageApproval</a></code> object.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_create_message_approval">create_message_approval</a>(dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, hash_scheme: u8, message: vector&lt;u8&gt;): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_MessageApproval">dwallet_2pc_mpc_secp256k1_inner::MessageApproval</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_create_message_approval">create_message_approval</a>(
    dwallet_id: ID,
    hash_scheme: u8,
    message: vector&lt;u8&gt;,
): <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_MessageApproval">MessageApproval</a> {
    <b>assert</b>!(<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_is_supported_hash_scheme">is_supported_hash_scheme</a>(hash_scheme), <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EInvalidHashScheme">EInvalidHashScheme</a>);
    <b>let</b> approval = <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_MessageApproval">MessageApproval</a> {
        dwallet_id,
        hash_scheme,
        message,
    };
    approval
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_approve_message"></a>

## Function `approve_message`

Approves a set of messages for a specific dWallet capability.

This function creates a list of <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_MessageApproval">MessageApproval</a></code> objects for a given set of messages.
Each message is associated with the same <code>dWalletCap</code> and <code>hash_scheme</code>. The messages
must be approved in the same order as they were created to maintain their sequence.


<a name="@Parameters_3"></a>

##### Parameters

- <code>dwallet_cap</code>: A reference to the <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCap">DWalletCap</a></code> object representing the capability for which
the messages are being approved.
- <code>hash_scheme</code>: The hash scheme to be used for hashing the messages. For example:
- <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_KECCAK256">KECCAK256</a></code>
- <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_SHA256">SHA256</a></code>
- <code>messages</code>: A mutable vector containing the messages to be approved. The messages are removed
from this vector as they are processed and added to the approvals list.


<a name="@Returns_4"></a>

##### Returns

A vector of <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_MessageApproval">MessageApproval</a></code> objects corresponding to the approved messages.


<a name="@Behavior_5"></a>

##### Behavior

- The function iterates over the provided <code>messages</code> vector, processes each message by creating
a <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_MessageApproval">MessageApproval</a></code> object, and pushes it into the <code>message_approvals</code> vector.
- The messages are approved in reverse order and then reversed again to preserve their original order.


<a name="@Aborts_6"></a>

##### Aborts

- Aborts if the provided <code>hash_scheme</code> is not supported by the system (checked during <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_create_message_approval">create_message_approval</a></code>).


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_approve_message">approve_message</a>(dwallet_cap: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCap">dwallet_2pc_mpc_secp256k1_inner::DWalletCap</a>, hash_scheme: u8, message: vector&lt;u8&gt;): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_MessageApproval">dwallet_2pc_mpc_secp256k1_inner::MessageApproval</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_approve_message">approve_message</a>(
    dwallet_cap: &<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCap">DWalletCap</a>,
    hash_scheme: u8,
    message: vector&lt;u8&gt;
): <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_MessageApproval">MessageApproval</a> {
    <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_create_message_approval">create_message_approval</a>(
        dwallet_cap.dwallet_id,
        hash_scheme,
        message,
    )
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_is_supported_hash_scheme"></a>

## Function `is_supported_hash_scheme`

Checks if the given hash scheme is supported for message signing.


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_is_supported_hash_scheme">is_supported_hash_scheme</a>(val: u8): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_is_supported_hash_scheme">is_supported_hash_scheme</a>(val: u8): bool {
    <b>return</b> match (val) {
            <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_KECCAK256">KECCAK256</a> | <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_SHA256">SHA256</a> =&gt; <b>true</b>,
    _ =&gt; <b>false</b>,
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_dwallet_dkg_first_round"></a>

## Function `request_dwallet_dkg_first_round`

Starts the first Distributed Key Generation (DKG) session.

This function creates a new <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCap">DWalletCap</a></code> object,
transfers it to the session initiator,
and emits a <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletDKGFirstRoundRequestEvent">DWalletDKGFirstRoundRequestEvent</a></code> to signal
the beginning of the DKG process.


<a name="@Parameters_7"></a>

##### Parameters



<a name="@Effects_8"></a>

##### Effects

- Generates a new <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCap">DWalletCap</a></code> object.
- Transfers the <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCap">DWalletCap</a></code> to the session initiator (<code>ctx.sender</code>).
- Emits a <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletDKGFirstRoundRequestEvent">DWalletDKGFirstRoundRequestEvent</a></code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_dwallet_dkg_first_round">request_dwallet_dkg_first_round</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCap">dwallet_2pc_mpc_secp256k1_inner::DWalletCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_dwallet_dkg_first_round">request_dwallet_dkg_first_round</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_network_decryption_key_id: ID,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCap">DWalletCap</a> {
    <b>let</b> pricing = self.pricing.dkg_first_round();
    <b>assert</b>!(self.dwallet_network_decryption_keys.contains(dwallet_network_decryption_key_id), <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EDWalletNetworkDecryptionKeyNotExist">EDWalletNetworkDecryptionKeyNotExist</a>);
    <b>let</b> id = object::new(ctx);
    <b>let</b> dwallet_id = id.to_inner();
    <b>let</b> dwallet_cap = <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCap">DWalletCap</a> {
        id: object::new(ctx),
        dwallet_id,
    };
    <b>let</b> dwallet_cap_id = object::id(&dwallet_cap);
    self.dwallets.add(dwallet_id, <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet">DWallet</a> {
        id,
        created_at_epoch: self.current_epoch,
        dwallet_cap_id,
        dwallet_network_decryption_key_id,
        encrypted_user_secret_key_shares: object_table::new(ctx),
        ecdsa_presigns: object_table::new(ctx),
        ecdsa_signs: object_table::new(ctx),
        state: DWalletState::Requested,
    });
    event::emit(self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
                dwallet_network_decryption_key_id,
        pricing,
        payment_ika,
        payment_sui,
        <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletDKGFirstRoundRequestEvent">DWalletDKGFirstRoundRequestEvent</a> {
            dwallet_id,
            dwallet_cap_id,
            dwallet_network_decryption_key_id,
        },
        ctx,
    ));
    dwallet_cap
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_update_last_session_to_complete_in_current_epoch"></a>

## Function `update_last_session_to_complete_in_current_epoch`

Updates the <code>last_session_to_complete_in_current_epoch</code> field.
We do this to ensure that the last session to complete in the current epoch is equal
to the desired completed sessions count.
This is part of the epoch switch logic.


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_update_last_session_to_complete_in_current_epoch">update_last_session_to_complete_in_current_epoch</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_update_last_session_to_complete_in_current_epoch">update_last_session_to_complete_in_current_epoch</a>(self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>) {
    <b>if</b> (self.locked_last_session_to_complete_in_current_epoch) {
        <b>return</b>
    };
    <b>let</b> new_last_session_to_complete_in_current_epoch = (
        self.number_of_completed_sessions + self.max_active_sessions_buffer
    ).min(
        self.next_session_sequence_number - 1,
    );
    <b>if</b> (self.last_session_to_complete_in_current_epoch &gt;= new_last_session_to_complete_in_current_epoch) {
        <b>return</b>
    };
    self.last_session_to_complete_in_current_epoch = new_last_session_to_complete_in_current_epoch;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_all_current_epoch_sessions_completed"></a>

## Function `all_current_epoch_sessions_completed`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_all_current_epoch_sessions_completed">all_current_epoch_sessions_completed</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_all_current_epoch_sessions_completed">all_current_epoch_sessions_completed</a>(self: &<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>): bool {
    <b>return</b> self.locked_last_session_to_complete_in_current_epoch &&
        self.number_of_completed_sessions == self.last_session_to_complete_in_current_epoch &&
        self.completed_immediate_sessions_count == self.started_immediate_sessions_count
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_remove_session_and_charge"></a>

## Function `remove_session_and_charge`



<pre><code><b>fun</b> remove_session_and_chargeE(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, session_sequence_number: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_remove_session_and_charge">remove_session_and_charge</a>&lt;E: <b>copy</b> + drop + store&gt;(self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>, session_sequence_number: u64) {
    self.number_of_completed_sessions = self.number_of_completed_sessions + 1;
    self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_update_last_session_to_complete_in_current_epoch">update_last_session_to_complete_in_current_epoch</a>();
    <b>let</b> session = self.sessions.remove(session_sequence_number);
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletSession">DWalletSession</a> {
        computation_fee_charged_ika,
        gas_fee_reimbursement_sui,
        consensus_validation_fee_charged_ika,
        dwallet_network_decryption_key_id,
        id,
        ..
    } = session;
    <b>let</b> dwallet_network_decryption_key = self.dwallet_network_decryption_keys.borrow_mut(dwallet_network_decryption_key_id);
    <b>let</b> _: <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletEvent">DWalletEvent</a>&lt;E&gt; = self.session_start_events.remove(id.to_inner());
    object::delete(id);
    dwallet_network_decryption_key.computation_fee_charged_ika.join(computation_fee_charged_ika);
    self.consensus_validation_fee_charged_ika.join(consensus_validation_fee_charged_ika);
    self.gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_dwallet_dkg_first_round"></a>

## Function `respond_dwallet_dkg_first_round`

Creates the output of the first DKG round.

This function transfers the output of the first DKG round
to the session initiator and ensures it is securely linked
to the <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCap">DWalletCap</a></code> of the session.
This function is called by blockchain itself.
Validators call it, it's part of the blockchain logic.


<a name="@Effects_9"></a>

##### Effects

- Transfers the output of the first round to the initiator.
- Emits necessary metadata and links it to the associated session.


<a name="@Parameters_10"></a>

##### Parameters

- <code>initiator</code>: The address of the user who initiated the DKG session.
- <code>session_id</code>: The ID of the DKG session.
- <code>decentralized_public_output</code>: The public output data from the first round.
- <code>dwallet_cap_id</code>: The ID of the associated <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCap">DWalletCap</a></code>.
- <code>ctx</code>: The transaction context.


<a name="@Panics_11"></a>

##### Panics

- Panics with <code>ENotSystemAddress</code> if the sender is not the system address.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_dwallet_dkg_first_round">respond_dwallet_dkg_first_round</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, first_round_output: vector&lt;u8&gt;, session_sequence_number: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_dwallet_dkg_first_round">respond_dwallet_dkg_first_round</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
    first_round_output: vector&lt;u8&gt;,
    session_sequence_number: u64,
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_remove_session_and_charge">remove_session_and_charge</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletDKGFirstRoundRequestEvent">DWalletDKGFirstRoundRequestEvent</a>&gt;(session_sequence_number);
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_dwallet_mut">get_dwallet_mut</a>(dwallet_id);
    dwallet.state = match (dwallet.state) {
        DWalletState::Requested =&gt; {
            event::emit(<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedDKGFirstdRoundEvent">CompletedDKGFirstdRoundEvent</a> {
                dwallet_id,
                first_round_output,
            });
            DWalletState::AwaitingUser {
                first_round_output
            }
        },
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EWrongState">EWrongState</a>
    };
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_create_first_round_dwallet_mock"></a>

## Function `create_first_round_dwallet_mock`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_create_first_round_dwallet_mock">create_first_round_dwallet_mock</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, first_round_output: vector&lt;u8&gt;, dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCap">dwallet_2pc_mpc_secp256k1_inner::DWalletCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_create_first_round_dwallet_mock">create_first_round_dwallet_mock</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>, first_round_output: vector&lt;u8&gt;, dwallet_network_decryption_key_id: ID, ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCap">DWalletCap</a> {
    <b>let</b> id = object::new(ctx);
    <b>let</b> dwallet_id = id.to_inner();
    <b>let</b> dwallet_cap = <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCap">DWalletCap</a> {
        id: object::new(ctx),
        dwallet_id,
    };
    <b>let</b> dwallet_cap_id = object::id(&dwallet_cap);
    self.dwallets.add(dwallet_id, <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet">DWallet</a> {
        id,
        created_at_epoch: self.current_epoch,
        dwallet_cap_id,
        dwallet_network_decryption_key_id,
        encrypted_user_secret_key_shares: object_table::new(ctx),
        ecdsa_presigns: object_table::new(ctx),
        ecdsa_signs: object_table::new(ctx),
        state: DWalletState::AwaitingUser {
            first_round_output
        },
    });
    dwallet_cap
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_mock_create_dwallet"></a>

## Function `mock_create_dwallet`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_mock_create_dwallet">mock_create_dwallet</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, output: vector&lt;u8&gt;, dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCap">dwallet_2pc_mpc_secp256k1_inner::DWalletCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_mock_create_dwallet">mock_create_dwallet</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>, output: vector&lt;u8&gt;, dwallet_network_decryption_key_id: ID, ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCap">DWalletCap</a> {
    <b>let</b> id = object::new(ctx);
    <b>let</b> dwallet_id = id.to_inner();
    <b>let</b> dwallet_cap = <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCap">DWalletCap</a> {
        id: object::new(ctx),
        dwallet_id,
    };
    <b>let</b> dwallet_cap_id = object::id(&dwallet_cap);
    self.dwallets.add(dwallet_id, <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet">DWallet</a> {
        id,
        created_at_epoch: self.current_epoch,
        dwallet_cap_id,
        dwallet_network_decryption_key_id,
        encrypted_user_secret_key_shares: object_table::new(ctx),
        ecdsa_presigns: object_table::new(ctx),
        ecdsa_signs: object_table::new(ctx),
        state: DWalletState::Active {
            public_output: output
        },
    });
    dwallet_cap
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_dwallet_dkg_second_round"></a>

## Function `request_dwallet_dkg_second_round`

Initiates the second round of the Distributed Key Generation (DKG) process
and emits an event for validators to begin their participation in this round.

This function handles the creation of a new DKG session ID and emits an event containing
all the necessary parameters to continue the DKG process.

<a name="@Parameters_12"></a>

##### Parameters

- <code>dwallet_cap</code>: A reference to the <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCap">DWalletCap</a></code>, representing the capability associated with the dWallet.
- <code>centralized_public_key_share_and_proof</code>: The user (centralized) public key share and proof.
- <code>first_round_output</code>: A reference to the <code>DWalletDKGFirstRoundOutput</code> structure containing the output of the first DKG round.
- <code>encrypted_centralized_secret_share_and_proof</code>: Encrypted centralized secret key share and its proof.
- <code>encryption_key</code>: The <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptionKey">EncryptionKey</a></code> object used for encrypting the secret key share.
- <code>centralized_public_output</code>: The public output of the centralized party in the DKG process.
- <code>decentralized_user_output_signature</code>: The signature for the public output of the centralized party in the DKG process.
- <code>singer_public_key</code>: The Ed25519 public key of the initiator,
used to verify the signature on the public output.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_dwallet_dkg_second_round">request_dwallet_dkg_second_round</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, dwallet_cap: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCap">dwallet_2pc_mpc_secp256k1_inner::DWalletCap</a>, centralized_public_key_share_and_proof: vector&lt;u8&gt;, encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;, encryption_key_address: <b>address</b>, user_public_output: vector&lt;u8&gt;, singer_public_key: vector&lt;u8&gt;, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_dwallet_dkg_second_round">request_dwallet_dkg_second_round</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_cap: &<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCap">DWalletCap</a>,
    centralized_public_key_share_and_proof: vector&lt;u8&gt;,
    encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;,
    encryption_key_address: <b>address</b>,
    user_public_output: vector&lt;u8&gt;,
    singer_public_key: vector&lt;u8&gt;,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> encryption_key = self.encryption_keys.borrow(encryption_key_address);
    <b>let</b> encryption_key_id = encryption_key.id.to_inner();
    <b>let</b> encryption_key = encryption_key.encryption_key;
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_dwallet">get_dwallet</a>(dwallet_cap.dwallet_id);
    <b>let</b> first_round_output = match (&dwallet.state) {
        DWalletState::AwaitingUser {
            first_round_output,
        } =&gt; {
            *first_round_output
        },
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EWrongState">EWrongState</a>
    };
    <b>let</b> pricing = self.pricing.dkg_second_round();
    <b>let</b> dwallet_network_decryption_key_id = dwallet.dwallet_network_decryption_key_id;
    <b>let</b> emit_event = self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
        dwallet_network_decryption_key_id,
        pricing,
        payment_ika,
        payment_sui,
        <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletDKGSecondRoundRequestEvent">DWalletDKGSecondRoundRequestEvent</a> {
            dwallet_id: dwallet_cap.dwallet_id,
            first_round_output,
            centralized_public_key_share_and_proof,
            dwallet_cap_id: object::id(dwallet_cap),
            encrypted_centralized_secret_share_and_proof,
            encryption_key,
            encryption_key_id,
            encryption_key_address,
            user_public_output,
            singer_public_key,
            dwallet_mpc_network_key_id: dwallet_network_decryption_key_id,
        },
        ctx,
    );
    event::emit(emit_event);
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_dwallet_mut">get_dwallet_mut</a>(dwallet_cap.dwallet_id);
    dwallet.state = DWalletState::AwaitingNetworkVerification;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_dwallet_dkg_second_round"></a>

## Function `respond_dwallet_dkg_second_round`

Completes the second round of the Distributed Key Generation (DKG) process and
creates the [<code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet">DWallet</a></code>].

This function finalizes the DKG process by creating a <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet">DWallet</a></code> object and associating it with the
cryptographic outputs of the second round. It also generates an encrypted user share and emits
events to record the results of the process.
This function is called by the blockchain.


<a name="@Parameters_13"></a>

##### Parameters

- **<code>session_id</code>**: A unique identifier for the current DKG session.
- **<code>decentralized_public_output</code>**: The public output of the second round of the DKG process,
representing the decentralized computation result.
- **<code>dwallet_cap_id</code>**: The unique identifier of the <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCap">DWalletCap</a></code> associated with this session.
- **<code>dwallet_mpc_network_decryption_key_version</code>**: The version of the MPC network key for the <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet">DWallet</a></code>.
- **<code>encrypted_secret_share_and_proof</code>**: The encrypted user secret key share and associated cryptographic proof.
- **<code>encryption_key_id</code>**: The ID of the <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptionKey">EncryptionKey</a></code> used for encrypting the secret key share.
- **<code>signed_public_share</code>**: The signed public share corresponding to the secret key share.
- **<code>encryptor_ed25519_pubkey</code>**: The Ed25519 public key of the entity that encrypted the secret key share.
- **<code>centralized_public_output</code>**: The centralized public output from the DKG process.


<a name="@Effects_14"></a>

##### Effects

- Creates a new <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet">DWallet</a></code> object using the provided session ID, DKG outputs, and other metadata.
- Creates an encrypted user share and associates it with the <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet">DWallet</a></code>.
- Emits a <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedDWalletDKGSecondRoundEvent">CompletedDWalletDKGSecondRoundEvent</a></code> to record the completion of the second DKG round.
- Freezes the created <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet">DWallet</a></code> object to make it immutable.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_dwallet_dkg_second_round">respond_dwallet_dkg_second_round</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, public_output: vector&lt;u8&gt;, encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;, encryption_key_address: <b>address</b>, session_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, rejected: bool, session_sequence_number: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_dwallet_dkg_second_round">respond_dwallet_dkg_second_round</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
    public_output: vector&lt;u8&gt;,
    encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;,
    encryption_key_address: <b>address</b>,
    session_id: ID,
    rejected: bool,
    session_sequence_number: u64,
    ctx: &<b>mut</b> TxContext
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_remove_session_and_charge">remove_session_and_charge</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletDKGSecondRoundRequestEvent">DWalletDKGSecondRoundRequestEvent</a>&gt;(session_sequence_number);
    <b>let</b> encryption_key = self.encryption_keys.borrow(encryption_key_address);
    <b>let</b> encryption_key_id = encryption_key.id.to_inner();
    <b>let</b> created_at_epoch = self.current_epoch;
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_dwallet_mut">get_dwallet_mut</a>(dwallet_id);
    dwallet.state = match (&dwallet.state) {
        DWalletState::AwaitingNetworkVerification =&gt; {
            <b>if</b> (rejected) {
                event::emit(<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_RejectedDWalletDKGSecondRoundEvent">RejectedDWalletDKGSecondRoundEvent</a> {
                    dwallet_id,
                    public_output,
                });
                DWalletState::NetworkRejectedSecondRound
            } <b>else</b> {
                <b>let</b> encrypted_user_share = <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a> {
                    id: object::new(ctx),
                    created_at_epoch,
                    dwallet_id,
                    encrypted_centralized_secret_share_and_proof,
                    encryption_key_id,
                    encryption_key_address,
                    source_encrypted_user_secret_key_share_id: option::none(),
                    state: EncryptedUserSecretKeyShareState::NetworkVerificationCompleted
                };
                <b>let</b> encrypted_user_secret_key_share_id = object::id(&encrypted_user_share);
                dwallet.encrypted_user_secret_key_shares.add(encrypted_user_secret_key_share_id, encrypted_user_share);
                event::emit(<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedDWalletDKGSecondRoundEvent">CompletedDWalletDKGSecondRoundEvent</a> {
                    dwallet_id,
                    public_output,
                    encrypted_user_secret_key_share_id,
                    session_id,
                });
                DWalletState::Active {
                    public_output
                }
            }
        },
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EWrongState">EWrongState</a>
    };
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_re_encrypt_user_share_for"></a>

## Function `request_re_encrypt_user_share_for`

Transfers an encrypted dWallet user secret key share from a source entity to destination entity.

This function emits an event with the encrypted user secret key share, along with its cryptographic proof,
to the blockchain. The chain verifies that the encrypted data matches the expected secret key share
associated with the dWallet before creating an [<code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a></code>] object.


<a name="@Parameters_15"></a>

##### Parameters

- **<code>dwallet</code>**: A reference to the <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet">DWallet</a>&lt;Secp256K1&gt;</code> object to which the secret share is linked.
- **<code>destination_encryption_key</code>**: A reference to the encryption key used for encrypting the secret key share.
- **<code>encrypted_centralized_secret_share_and_proof</code>**: The encrypted secret key share, accompanied by a cryptographic proof.
- **<code>source_signed_centralized_public_output</code>**: The signed centralized public output corresponding to the secret share.
- **<code>source_ed25519_pubkey</code>**: The Ed25519 public key of the source (encryptor) used for verifying the signature.


<a name="@Effects_16"></a>

##### Effects

- Emits a <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptedShareVerificationRequestEvent">EncryptedShareVerificationRequestEvent</a></code>,
which is captured by the blockchain to initiate the verification process.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_re_encrypt_user_share_for">request_re_encrypt_user_share_for</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, destination_encryption_key_address: <b>address</b>, encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;, source_encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_re_encrypt_user_share_for">request_re_encrypt_user_share_for</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
    destination_encryption_key_address: <b>address</b>,
    encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;,
    source_encrypted_user_secret_key_share_id: ID,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> created_at_epoch = self.current_epoch;
    <b>let</b> destination_encryption_key = self.encryption_keys.borrow(destination_encryption_key_address);
    <b>let</b> destination_encryption_key_id = destination_encryption_key.id.to_inner();
    <b>let</b> destination_encryption_key = destination_encryption_key.encryption_key;
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_dwallet_mut">get_dwallet_mut</a>(dwallet_id);
    <b>let</b> public_output = *dwallet.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_validate_active_and_get_public_output">validate_active_and_get_public_output</a>();
    <b>let</b> dwallet_mpc_network_key_id = dwallet.dwallet_network_decryption_key_id;
    <b>assert</b>!(dwallet.encrypted_user_secret_key_shares.contains(source_encrypted_user_secret_key_share_id), <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EInvalidSource">EInvalidSource</a>);
    <b>let</b> encrypted_user_share = <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a> {
        id: object::new(ctx),
        created_at_epoch,
        dwallet_id,
        encrypted_centralized_secret_share_and_proof,
        encryption_key_id: destination_encryption_key_id,
        encryption_key_address: destination_encryption_key_address,
        source_encrypted_user_secret_key_share_id: option::some(source_encrypted_user_secret_key_share_id),
        state: EncryptedUserSecretKeyShareState::AwaitingNetworkVerification
    };
    <b>let</b> encrypted_user_secret_key_share_id = object::id(&encrypted_user_share);
    dwallet.encrypted_user_secret_key_shares.add(encrypted_user_secret_key_share_id, encrypted_user_share);
    <b>let</b> dwallet_network_decryption_key_id = dwallet.dwallet_network_decryption_key_id;
    <b>let</b> pricing = self.pricing.re_encrypt_user_share();
    event::emit(
        self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
            dwallet_network_decryption_key_id,
            pricing,
            payment_ika,
            payment_sui,
            <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptedShareVerificationRequestEvent">EncryptedShareVerificationRequestEvent</a> {
                encrypted_centralized_secret_share_and_proof,
                public_output,
                dwallet_id,
                encryption_key: destination_encryption_key,
                encryption_key_id: destination_encryption_key_id,
                encrypted_user_secret_key_share_id,
                source_encrypted_user_secret_key_share_id,
                dwallet_mpc_network_key_id,
            },
            ctx,
        )
    );
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_re_encrypt_user_share_for"></a>

## Function `respond_re_encrypt_user_share_for`

Creates an encrypted user secret key share after it has been verified by the blockchain.

This function is invoked by the blockchain to generate an [<code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a></code>] object
once the associated encryption and cryptographic proofs have been verified.
It finalizes the process by storing the encrypted user share on-chain and emitting the relevant event.


<a name="@Parameters_17"></a>

##### Parameters

- <code>dwallet_id</code>: The unique identifier of the dWallet associated with the encrypted user share.
- <code>encrypted_centralized_secret_share_and_proof</code>: The encrypted centralized secret key share along with its cryptographic proof.
- <code>encryption_key_id</code>: The <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptionKey">EncryptionKey</a></code> Move object ID used to encrypt the secret key share.
- <code>centralized_user_output_signature</code>: The signed public share corresponding to the encrypted secret share.
- <code>singer_public_key</code>: The Ed25519 public key of the encryptor, used for signing.
- <code>initiator</code>: The address of the entity that performed the encryption operation of this secret key share.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_re_encrypt_user_share_for">respond_re_encrypt_user_share_for</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, rejected: bool, session_sequence_number: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_re_encrypt_user_share_for">respond_re_encrypt_user_share_for</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
    encrypted_user_secret_key_share_id: ID,
    rejected: bool,
    session_sequence_number: u64
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_remove_session_and_charge">remove_session_and_charge</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EncryptedShareVerificationRequestEvent">EncryptedShareVerificationRequestEvent</a>&gt;(session_sequence_number);
    <b>let</b> (dwallet, _) = self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_active_dwallet_and_public_output_mut">get_active_dwallet_and_public_output_mut</a>(dwallet_id);
    <b>let</b> encrypted_user_secret_key_share = dwallet.encrypted_user_secret_key_shares.borrow_mut(encrypted_user_secret_key_share_id);
    encrypted_user_secret_key_share.state = match(encrypted_user_secret_key_share.state) {
        EncryptedUserSecretKeyShareState::AwaitingNetworkVerification =&gt; {
            <b>if</b>(rejected) {
                event::emit(
                    <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_RejectedEncryptedShareVerificationEvent">RejectedEncryptedShareVerificationEvent</a> {
                        encrypted_user_secret_key_share_id,
                        dwallet_id,
                    }
                );
                EncryptedUserSecretKeyShareState::NetworkVerificationRejected
            } <b>else</b> {
                event::emit(
                    <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedEncryptedShareVerificationEvent">CompletedEncryptedShareVerificationEvent</a> {
                        encrypted_user_secret_key_share_id,
                        dwallet_id,
                    }
                );
                EncryptedUserSecretKeyShareState::NetworkVerificationCompleted
            }
        },
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EWrongState">EWrongState</a>
    };
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_accept_encrypted_user_share"></a>

## Function `accept_encrypted_user_share`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_accept_encrypted_user_share">accept_encrypted_user_share</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, user_output_signature: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_accept_encrypted_user_share">accept_encrypted_user_share</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
    encrypted_user_secret_key_share_id: ID,
    user_output_signature: vector&lt;u8&gt;,
) {
    <b>let</b> (dwallet, public_output) = self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_active_dwallet_and_public_output">get_active_dwallet_and_public_output</a>(dwallet_id);
    <b>let</b> encrypted_user_secret_key_share = dwallet.encrypted_user_secret_key_shares.borrow(encrypted_user_secret_key_share_id);
    <b>let</b> encryption_key = self.encryption_keys.borrow(encrypted_user_secret_key_share.encryption_key_address);
    <b>let</b> encryption_key_id = encrypted_user_secret_key_share.encryption_key_id;
    <b>let</b> encryption_key_address = encrypted_user_secret_key_share.encryption_key_address;
    <b>assert</b>!(
        ed25519_verify(&user_output_signature, &encryption_key.signer_public_key, &public_output),
        <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EInvalidEncryptionKeySignature">EInvalidEncryptionKeySignature</a>
    );
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_dwallet_mut">get_dwallet_mut</a>(dwallet_id);
    <b>let</b> encrypted_user_secret_key_share = dwallet.encrypted_user_secret_key_shares.borrow_mut(encrypted_user_secret_key_share_id);
    encrypted_user_secret_key_share.state = match (encrypted_user_secret_key_share.state) {
        EncryptedUserSecretKeyShareState::NetworkVerificationCompleted =&gt; EncryptedUserSecretKeyShareState::KeyHolderSiged {
            user_output_signature
        },
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EWrongState">EWrongState</a>
    };
    event::emit(
        <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_AcceptReEncryptedUserShareEvent">AcceptReEncryptedUserShareEvent</a> {
            encrypted_user_secret_key_share_id,
            dwallet_id,
            user_output_signature,
            encryption_key_id,
            encryption_key_address,
        }
    );
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_ecdsa_presign"></a>

## Function `request_ecdsa_presign`

Starts a batched presign session.

This function emits a <code>RequestedBatchedPresignEvent</code> for the entire batch and a
<code>RequestedPresignFirstRoundEvent</code> for each presign in the batch. These events signal
validators to begin processing the first round of the presign process for each session.
- A unique <code>batch_session_id</code> is generated for the batch.
- A loop creates and emits a <code>RequestedPresignFirstRoundEvent</code> for each session in the batch.
- Each session is linked to the parent batch via <code>batch_session_id</code>.


<a name="@Effects_18"></a>

##### Effects

- Associates the batched presign session with the specified dWallet.
- Emits a <code>RequestedBatchedPresignEvent</code> containing the batch session details.
- Emits a <code>RequestedPresignFirstRoundEvent</code> for each presign in the batch, with relevant details.


<a name="@Parameters_19"></a>

##### Parameters

- <code>dwallet_id</code>: The dWallet's ID to resquest presign.
- <code>ctx</code>: The mutable transaction context, used to generate unique object IDs and retrieve the initiator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_ecdsa_presign">request_ecdsa_presign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresignCap">dwallet_2pc_mpc_secp256k1_inner::ECDSAPresignCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_ecdsa_presign">request_ecdsa_presign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresignCap">ECDSAPresignCap</a> {
    <b>let</b> created_at_epoch = self.current_epoch;
    <b>let</b> (dwallet, public_output) = self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_active_dwallet_and_public_output_mut">get_active_dwallet_and_public_output_mut</a>(dwallet_id);
    <b>let</b> dwallet_network_decryption_key_id = dwallet.dwallet_network_decryption_key_id;
    <b>let</b> id = object::new(ctx);
    <b>let</b> presign_id = id.to_inner();
    <b>let</b> cap = <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresignCap">ECDSAPresignCap</a> {
        id: object::new(ctx),
        dwallet_id,
        presign_id,
    };
    dwallet.ecdsa_presigns.add(presign_id, <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresign">ECDSAPresign</a> {
        id,
        created_at_epoch,
        dwallet_id,
        cap_id: object::id(&cap),
        state: ECDSAPresignState::Requested,
    });
    <b>let</b> pricing = self.pricing.ecdsa_presign();
    event::emit(
        self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
            dwallet_network_decryption_key_id,
            pricing,
            payment_ika,
            payment_sui,
            <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresignRequestEvent">ECDSAPresignRequestEvent</a> {
                dwallet_id,
                presign_id,
                dwallet_public_output: public_output,
                dwallet_network_decryption_key_id: dwallet_network_decryption_key_id,
            },
            ctx,
        )
    );
    cap
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_mock_create_presign"></a>

## Function `mock_create_presign`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_mock_create_presign">mock_create_presign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, presign: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresignCap">dwallet_2pc_mpc_secp256k1_inner::ECDSAPresignCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_mock_create_presign">mock_create_presign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
    presign: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresignCap">ECDSAPresignCap</a> {
    <b>let</b> (dwallet, _) = self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_active_dwallet_and_public_output_mut">get_active_dwallet_and_public_output_mut</a>(dwallet_id);
    <b>let</b> id = object::new(ctx);
    <b>let</b> presign_id = id.to_inner();
    <b>let</b> cap = <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresignCap">ECDSAPresignCap</a> {
        id: object::new(ctx),
        dwallet_id,
        presign_id,
    };
    dwallet.ecdsa_presigns.add(presign_id, <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresign">ECDSAPresign</a> {
        id,
        created_at_epoch: 0,
        dwallet_id,
        cap_id: object::id(&cap),
        state: ECDSAPresignState::Completed {
            presign
        }
    });
    event::emit(<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedECDSAPresignEvent">CompletedECDSAPresignEvent</a> {
        dwallet_id,
        session_id: object::id_from_address(tx_context::fresh_object_address(ctx)),
        presign_id,
        presign
    });
    cap
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_ecdsa_presign"></a>

## Function `respond_ecdsa_presign`

Completes the presign session by creating the output of the
second presign round and transferring it to the session initiator.

This function is called by validators as part of the blockchain logic.
It creates a <code>Presign</code> object representing the second presign round output,
emits a <code>CompletedPresignEvent</code>, and transfers the result to the initiating user.


<a name="@Parameters_20"></a>

##### Parameters

- <code>initiator</code>: The address of the user who initiated the presign session.
- <code>session_id</code>: The ID of the presign session.
- <code>output</code>: The presign result data.
- <code>dwallet_cap_id</code>: The ID of the associated <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCap">DWalletCap</a></code>.
- <code>dwallet_id</code>: The ID of the associated <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet">DWallet</a></code>.
- <code>ctx</code>: The transaction context.


<a name="@Emits_21"></a>

##### Emits

- <code>CompletedPresignEvent</code>: Includes the initiator, dWallet ID, and presign ID.


<a name="@Panics_22"></a>

##### Panics

- Panics with <code>ENotSystemAddress</code> if the sender of the transaction is not the system address.


<a name="@Effects_23"></a>

##### Effects

- Creates a <code>Presign</code> object and transfers it to the session initiator.
- Emits a <code>CompletedPresignEvent</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_ecdsa_presign">respond_ecdsa_presign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, presign_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, session_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, presign: vector&lt;u8&gt;, rejected: bool, session_sequence_number: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_ecdsa_presign">respond_ecdsa_presign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
    presign_id: ID,
    session_id: ID,
    presign: vector&lt;u8&gt;,
    rejected: bool,
    session_sequence_number: u64
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_remove_session_and_charge">remove_session_and_charge</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresignRequestEvent">ECDSAPresignRequestEvent</a>&gt;(session_sequence_number);
    <b>let</b> (dwallet, _) = self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_active_dwallet_and_public_output_mut">get_active_dwallet_and_public_output_mut</a>(dwallet_id);
    <b>let</b> presign_obj = dwallet.ecdsa_presigns.borrow_mut(presign_id);
    presign_obj.state = match(presign_obj.state) {
        ECDSAPresignState::Requested =&gt; {
            <b>if</b>(rejected) {
                event::emit(<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_RejectedECDSAPresignEvent">RejectedECDSAPresignEvent</a> {
                    dwallet_id,
                    session_id,
                    presign_id
                });
                ECDSAPresignState::NetworkRejected
            } <b>else</b> {
                event::emit(<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedECDSAPresignEvent">CompletedECDSAPresignEvent</a> {
                    dwallet_id,
                    session_id,
                    presign_id,
                    presign
                });
                ECDSAPresignState::Completed {
                    presign
                }
            }
        },
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EWrongState">EWrongState</a>
    };
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_is_ecdsa_presign_valid"></a>

## Function `is_ecdsa_presign_valid`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_is_ecdsa_presign_valid">is_ecdsa_presign_valid</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, presign_cap: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresignCap">dwallet_2pc_mpc_secp256k1_inner::ECDSAPresignCap</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_is_ecdsa_presign_valid">is_ecdsa_presign_valid</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    presign_cap: &<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresignCap">ECDSAPresignCap</a>,
): bool {
    <b>let</b> (dwallet, _) = self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_active_dwallet_and_public_output">get_active_dwallet_and_public_output</a>(presign_cap.dwallet_id);
    <b>let</b> presign = dwallet.ecdsa_presigns.borrow(presign_cap.presign_id);
    match(&presign.state) {
        ECDSAPresignState::Completed { .. } =&gt; {
            <b>true</b>
        },
        _ =&gt; <b>false</b>
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_emit_ecdsa_sign_event"></a>

## Function `emit_ecdsa_sign_event`

Emits events to initiate the signing process for each message.

This function ensures that all messages have the correct approvals, calculates
their hashes, and emits signing events.


<a name="@Effects_24"></a>

### Effects

- Checks that the number of <code>signature_algorithm_data</code> items matches <code>message_approvals</code>.
- Generates a new session ID for batch signing.
- Emits <code>RequestedBatchedSignEvent</code> containing session details and hashed messages.
- Iterates through <code>signature_algorithm_data</code>, verifying approvals and emitting <code>RequestedSignEvent</code> for each.


<a name="@Aborts_25"></a>

### Aborts

- **<code>EExtraDataAndMessagesLenMismatch</code>**: If <code>signature_algorithm_data</code> and <code>message_approvals</code> have different lengths.
- **<code>EMissingApprovalOrWrongApprovalOrder</code>**: If message approvals are incorrect or missing.


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_emit_ecdsa_sign_event">emit_ecdsa_sign_event</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, pricing: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">dwallet_pricing::PricingPerOperation</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, message_approval: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_MessageApproval">dwallet_2pc_mpc_secp256k1_inner::MessageApproval</a>, presign_cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresignCap">dwallet_2pc_mpc_secp256k1_inner::ECDSAPresignCap</a>, message_centralized_signature: vector&lt;u8&gt;, is_future_sign: bool, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_emit_ecdsa_sign_event">emit_ecdsa_sign_event</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    pricing: PricingPerOperation,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    message_approval: <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_MessageApproval">MessageApproval</a>,
    presign_cap: <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresignCap">ECDSAPresignCap</a>,
    message_centralized_signature: vector&lt;u8&gt;,
    is_future_sign: bool,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> created_at_epoch = self.current_epoch;
    <b>let</b> (dwallet, public_output) = self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_active_dwallet_and_public_output_mut">get_active_dwallet_and_public_output_mut</a>(presign_cap.dwallet_id);
    <b>assert</b>!(dwallet.ecdsa_presigns.contains(presign_cap.presign_id), <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EPresignNotExist">EPresignNotExist</a>);
    <b>let</b> presign = dwallet.ecdsa_presigns.remove(presign_cap.presign_id);
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_MessageApproval">MessageApproval</a> {
        dwallet_id: message_approval_dwallet_id,
        hash_scheme,
        message
    } = message_approval;
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresignCap">ECDSAPresignCap</a> {
        id,
        dwallet_id: presign_cap_dwallet_id,
        presign_id: presign_cap_presign_id,
    } = presign_cap;
    <b>let</b> presign_cap_id = id.to_inner();
    id.delete();
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresign">ECDSAPresign</a> {
        id,
        created_at_epoch: _,
        dwallet_id: presign_dwallet_id,
        cap_id,
        state,
    } = presign;
    <b>let</b> presign = match(state) {
        ECDSAPresignState::Completed { presign } =&gt; {
            presign
        },
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EInvalidPresign">EInvalidPresign</a>
    };
    <b>let</b> presign_id = id.to_inner();
    id.delete();
    <b>assert</b>!(presign_dwallet_id == message_approval_dwallet_id, <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EMessageApprovalMismatch">EMessageApprovalMismatch</a>);
    <b>assert</b>!(presign_cap_id == cap_id, <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EPresignNotExist">EPresignNotExist</a>);
    <b>assert</b>!(presign_id == presign_cap_presign_id, <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EPresignNotExist">EPresignNotExist</a>);
    <b>assert</b>!(presign_cap_dwallet_id == presign_dwallet_id, <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EPresignNotExist">EPresignNotExist</a>);
    <b>let</b> id = object::new(ctx);
    <b>let</b> sign_id = id.to_inner();
    <b>let</b> dwallet_network_decryption_key_id = dwallet.dwallet_network_decryption_key_id;
    <b>let</b> emit_event = self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
        dwallet_network_decryption_key_id,
        pricing,
        payment_ika,
        payment_sui,
        <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSASignRequestEvent">ECDSASignRequestEvent</a> {
            sign_id,
            dwallet_id: presign_dwallet_id,
            dwallet_public_output: public_output,
            hash_scheme,
            message,
            dwallet_network_decryption_key_id,
            presign_id,
            presign,
            message_centralized_signature,
            is_future_sign,
        },
        ctx,
    );
    <b>let</b> session_id = emit_event.session_id;
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_dwallet_mut">get_dwallet_mut</a>(presign_dwallet_id);
    dwallet.ecdsa_signs.add(sign_id, <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSASign">ECDSASign</a> {
        id,
        created_at_epoch,
        dwallet_id: presign_dwallet_id,
        session_id,
        state: ECDSASignState::Requested,
    });
    event::emit(emit_event);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_ecdsa_sign"></a>

## Function `request_ecdsa_sign`

Initiates the signing process for a given dWallet of type T.

This function emits a <code>RequestedSignEvent</code> and a <code>RequestedBatchedSignEvent</code>,
providing all necessary metadata to ensure the integrity of the signing process.
It validates the linkage between the <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet">DWallet</a></code>, <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCap">DWalletCap</a></code>, and <code>SignatureAlgorithmData</code> objects.


<a name="@Effects_26"></a>

### Effects

- Ensures a valid linkage between <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet">DWallet</a></code>, <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCap">DWalletCap</a></code>, and <code>SignatureAlgorithmData</code>.
- Validates that <code>signature_algorithm_data</code> and <code>message_approvals</code> have the same length.
- Emits the following events:
- <code>RequestedBatchedSignEvent</code>: Contains the session details and the list of hashed messages.
- <code>RequestedSignEvent</code>: Includes details for each message signing process.


<a name="@Aborts_27"></a>

### Aborts

- **<code>EExtraDataAndMessagesLenMismatch</code>**: If the number of <code>hashed_messages</code> does not
match the number of <code>signature_algorithm_data</code>.
- **<code>EMissingApprovalOrWrongApprovalOrder</code>**: If the approvals are missing or provided in the incorrect order.


<a name="@Parameters_28"></a>

### Parameters

- <code>message_approvals</code>: A vector of <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_MessageApproval">MessageApproval</a></code> objects representing
approvals for the messages, which are destroyed at the end of the transaction.
- <code>dwallet</code>: A reference to the <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWallet">DWallet</a></code> object being used for signing.
- <code>signature_algorithm_data</code>: A vector of <code>SignatureAlgorithmData</code> objects containing intermediate signing outputs,
which are unpacked and then destroyed at the end of the transaction.


<a name="@Type_Parameters_29"></a>

### Type Parameters

- <code>T</code>: The elliptic curve type used for the dWallet.
D: The type of data that can be stored with the object,
specific to each Digital Signature Algorithm.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_ecdsa_sign">request_ecdsa_sign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, message_approval: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_MessageApproval">dwallet_2pc_mpc_secp256k1_inner::MessageApproval</a>, presign_cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresignCap">dwallet_2pc_mpc_secp256k1_inner::ECDSAPresignCap</a>, message_centralized_signature: vector&lt;u8&gt;, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_ecdsa_sign">request_ecdsa_sign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    message_approval: <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_MessageApproval">MessageApproval</a>,
    presign_cap: <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresignCap">ECDSAPresignCap</a>,
    message_centralized_signature: vector&lt;u8&gt;,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> (dwallet, _) = self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_active_dwallet_and_public_output_mut">get_active_dwallet_and_public_output_mut</a>(presign_cap.dwallet_id);
    <b>assert</b>!(dwallet.ecdsa_presigns.contains(presign_cap.presign_id), <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EPresignNotExist">EPresignNotExist</a>);
    <b>let</b> pricing = self.pricing.ecdsa_sign();
    self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_emit_ecdsa_sign_event">emit_ecdsa_sign_event</a>(
        pricing,
        payment_ika,
        payment_sui,
        message_approval,
        presign_cap,
        message_centralized_signature,
        <b>false</b>,
        ctx
    );
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_ecdsa_future_sign"></a>

## Function `request_ecdsa_future_sign`

A function to publish messages signed by the user on chain with on-chain verification,
without launching the chain's sign flow immediately.

See the docs of [<code>PartialCentralizedSignedMessages</code>] for
more details on when this may be used.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_ecdsa_future_sign">request_ecdsa_future_sign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, presign_cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresignCap">dwallet_2pc_mpc_secp256k1_inner::ECDSAPresignCap</a>, message: vector&lt;u8&gt;, hash_scheme: u8, message_centralized_signature: vector&lt;u8&gt;, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_UnverifiedECDSAPartialUserSignatureCap">dwallet_2pc_mpc_secp256k1_inner::UnverifiedECDSAPartialUserSignatureCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_ecdsa_future_sign">request_ecdsa_future_sign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    presign_cap: <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPresignCap">ECDSAPresignCap</a>,
    message: vector&lt;u8&gt;,
    hash_scheme: u8,
    message_centralized_signature: vector&lt;u8&gt;,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_UnverifiedECDSAPartialUserSignatureCap">UnverifiedECDSAPartialUserSignatureCap</a> {
    <b>let</b> pricing = self.pricing.ecdsa_future_sign();
    <b>let</b> (dwallet, public_dwallet_output) = self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_active_dwallet_and_public_output_mut">get_active_dwallet_and_public_output_mut</a>(presign_cap.dwallet_id);
    <b>let</b> dwallet_network_decryption_key_id = dwallet.dwallet_network_decryption_key_id;
    // TODO: Change error
    <b>assert</b>!(dwallet.ecdsa_presigns.contains(presign_cap.presign_id), <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EPresignNotExist">EPresignNotExist</a>);
    <b>let</b> presign_obj = dwallet.ecdsa_presigns.borrow(presign_cap.presign_id);
    <b>let</b> presign = match(presign_obj.state) {
        ECDSAPresignState::Completed { presign } =&gt; {
            presign
        },
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EInvalidPresign">EInvalidPresign</a>
    };
    <b>let</b> id = object::new(ctx);
    <b>let</b> partial_centralized_signed_message_id = id.to_inner();
    <b>let</b> cap = <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_UnverifiedECDSAPartialUserSignatureCap">UnverifiedECDSAPartialUserSignatureCap</a> {
        id: object::new(ctx),
        partial_centralized_signed_message_id,
    };
    <b>let</b> emit_event = self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
        dwallet_network_decryption_key_id,
        pricing,
        payment_ika,
        payment_sui,
        <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAFutureSignRequestEvent">ECDSAFutureSignRequestEvent</a> {
                dwallet_id: presign_cap.dwallet_id,
                partial_centralized_signed_message_id,
                message,
                presign: presign,
                dwallet_public_output: public_dwallet_output,
                hash_scheme,
                message_centralized_signature,
                dwallet_mpc_network_key_id: dwallet_network_decryption_key_id,
        },
        ctx,
    );
    self.ecdsa_partial_centralized_signed_messages.add(partial_centralized_signed_message_id, <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPartialUserSignature">ECDSAPartialUserSignature</a> {
        id: id,
        created_at_epoch: self.current_epoch,
        presign_cap,
        cap_id: object::id(&cap),
        hash_scheme,
        message,
        message_centralized_signature,
        state: ECDSAPartialUserSignatureState::AwaitingNetworkVerification,
    });
    event::emit(emit_event);
    cap
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_ecdsa_future_sign"></a>

## Function `respond_ecdsa_future_sign`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_ecdsa_future_sign">respond_ecdsa_future_sign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, session_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, partial_centralized_signed_message_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, rejected: bool, session_sequence_number: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_ecdsa_future_sign">respond_ecdsa_future_sign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    session_id: ID,
    dwallet_id: ID,
    partial_centralized_signed_message_id: ID,
    rejected: bool,
    session_sequence_number: u64
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_remove_session_and_charge">remove_session_and_charge</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAFutureSignRequestEvent">ECDSAFutureSignRequestEvent</a>&gt;(session_sequence_number);
    <b>let</b> partial_centralized_signed_message = self.ecdsa_partial_centralized_signed_messages.borrow_mut(partial_centralized_signed_message_id);
    <b>assert</b>!(partial_centralized_signed_message.presign_cap.dwallet_id == dwallet_id, <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EDWalletMismatch">EDWalletMismatch</a>);
    partial_centralized_signed_message.state = match(partial_centralized_signed_message.state) {
        ECDSAPartialUserSignatureState::AwaitingNetworkVerification =&gt; {
            <b>if</b>(rejected) {
                event::emit(<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_RejectedECDSAFutureSignEvent">RejectedECDSAFutureSignEvent</a> {
                    session_id,
                    dwallet_id,
                    partial_centralized_signed_message_id
                });
                ECDSAPartialUserSignatureState::NetworkVerificationRejected
            } <b>else</b> {
                event::emit(<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedECDSAFutureSignEvent">CompletedECDSAFutureSignEvent</a> {
                    session_id,
                    dwallet_id,
                    partial_centralized_signed_message_id
                });
                ECDSAPartialUserSignatureState::NetworkVerificationCompleted
            }
        },
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EWrongState">EWrongState</a>
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_verify_ecdsa_partial_user_signature_cap"></a>

## Function `verify_ecdsa_partial_user_signature_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_verify_ecdsa_partial_user_signature_cap">verify_ecdsa_partial_user_signature_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_UnverifiedECDSAPartialUserSignatureCap">dwallet_2pc_mpc_secp256k1_inner::UnverifiedECDSAPartialUserSignatureCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_VerifiedECDSAPartialUserSignatureCap">dwallet_2pc_mpc_secp256k1_inner::VerifiedECDSAPartialUserSignatureCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_verify_ecdsa_partial_user_signature_cap">verify_ecdsa_partial_user_signature_cap</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    cap: <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_UnverifiedECDSAPartialUserSignatureCap">UnverifiedECDSAPartialUserSignatureCap</a>,
    ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_VerifiedECDSAPartialUserSignatureCap">VerifiedECDSAPartialUserSignatureCap</a> {
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_UnverifiedECDSAPartialUserSignatureCap">UnverifiedECDSAPartialUserSignatureCap</a> {
        id,
        partial_centralized_signed_message_id
    } = cap;
    <b>let</b> cap_id = id.to_inner();
    id.delete();
    <b>let</b> partial_centralized_signed_message = self.ecdsa_partial_centralized_signed_messages.borrow_mut(partial_centralized_signed_message_id);
    <b>assert</b>!(partial_centralized_signed_message.cap_id == cap_id, <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EIncorrectCap">EIncorrectCap</a>);
    <b>assert</b>!(partial_centralized_signed_message.state == ECDSAPartialUserSignatureState::NetworkVerificationCompleted, <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EUnverifiedCap">EUnverifiedCap</a>);
    <b>let</b> cap = <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_VerifiedECDSAPartialUserSignatureCap">VerifiedECDSAPartialUserSignatureCap</a> {
        id: object::new(ctx),
        partial_centralized_signed_message_id,
    };
    partial_centralized_signed_message.cap_id = cap.id.to_inner();
    cap
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_ecdsa_sign_with_partial_user_signatures"></a>

## Function `request_ecdsa_sign_with_partial_user_signatures`

Initiates a signing flow using a previously published [<code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPartialUserSignature">ECDSAPartialUserSignature</a></code>].

This function takes a partial signature object, validates approvals for each message,
and emits the necessary signing events.


<a name="@Type_Parameters_30"></a>

#### Type Parameters

- <code>D</code>: Represents additional data fields specific for each implementation.


<a name="@Parameters_31"></a>

#### Parameters

- <code>partial_signature</code>: A previously published <code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPartialUserSignature">ECDSAPartialUserSignature</a>&lt;D&gt;</code> object
containing messages that require approval.
- <code>message_approvals</code>: A list of approvals corresponding to the messages in <code>partial_signature</code>.
- <code>ctx</code>: The transaction context.

<a name="@Notes_32"></a>

#### Notes

- See [<code><a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPartialUserSignature">ECDSAPartialUserSignature</a></code>] documentation for more details on usage scenarios.
- The function ensures that messages and approvals have a one-to-one correspondence before proceeding.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_ecdsa_sign_with_partial_user_signatures">request_ecdsa_sign_with_partial_user_signatures</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, partial_user_signature_cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_VerifiedECDSAPartialUserSignatureCap">dwallet_2pc_mpc_secp256k1_inner::VerifiedECDSAPartialUserSignatureCap</a>, message_approval: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_MessageApproval">dwallet_2pc_mpc_secp256k1_inner::MessageApproval</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_request_ecdsa_sign_with_partial_user_signatures">request_ecdsa_sign_with_partial_user_signatures</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    partial_user_signature_cap: <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_VerifiedECDSAPartialUserSignatureCap">VerifiedECDSAPartialUserSignatureCap</a>,
    message_approval: <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_MessageApproval">MessageApproval</a>,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> pricing = self.pricing.ecdsa_sign_with_partial_user_signature();
    // Ensure that each message <b>has</b> a corresponding approval; otherwise, <b>abort</b>.
    self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_compare_ecdsa_partial_user_signatures_with_approvals">compare_ecdsa_partial_user_signatures_with_approvals</a>(&partial_user_signature_cap, &message_approval);
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_VerifiedECDSAPartialUserSignatureCap">VerifiedECDSAPartialUserSignatureCap</a> {
        id,
        partial_centralized_signed_message_id,
    } = partial_user_signature_cap;
    <b>let</b> verified_cap_id = id.to_inner();
    id.delete();
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSAPartialUserSignature">ECDSAPartialUserSignature</a> {
        id,
        created_at_epoch: _,
        presign_cap,
        cap_id,
        hash_scheme: _,
        message: _,
        message_centralized_signature,
        state
    } = self.ecdsa_partial_centralized_signed_messages.remove(partial_centralized_signed_message_id);
    id.delete();
    <b>assert</b>!(cap_id == verified_cap_id && state == ECDSAPartialUserSignatureState::NetworkVerificationCompleted, <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EIncorrectCap">EIncorrectCap</a>);
    // Emit signing events to finalize the signing process.
    self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_emit_ecdsa_sign_event">emit_ecdsa_sign_event</a>(
        pricing,
        payment_ika,
        payment_sui,
        message_approval,
        presign_cap,
        message_centralized_signature,
        <b>true</b>,
        ctx
    );
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_compare_ecdsa_partial_user_signatures_with_approvals"></a>

## Function `compare_ecdsa_partial_user_signatures_with_approvals`

Compares partial user signatures with message approvals to ensure they match.
This function can be called by the user to verify that the messages and approvals match,
before calling the <code>sign_with_partial_centralized_message_signatures</code> function.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_compare_ecdsa_partial_user_signatures_with_approvals">compare_ecdsa_partial_user_signatures_with_approvals</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, partial_user_signature_cap: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_VerifiedECDSAPartialUserSignatureCap">dwallet_2pc_mpc_secp256k1_inner::VerifiedECDSAPartialUserSignatureCap</a>, message_approval: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_MessageApproval">dwallet_2pc_mpc_secp256k1_inner::MessageApproval</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_compare_ecdsa_partial_user_signatures_with_approvals">compare_ecdsa_partial_user_signatures_with_approvals</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    partial_user_signature_cap: &<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_VerifiedECDSAPartialUserSignatureCap">VerifiedECDSAPartialUserSignatureCap</a>,
    message_approval: &<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_MessageApproval">MessageApproval</a>,
) {
    <b>let</b> partial_signature = self.ecdsa_partial_centralized_signed_messages.borrow(partial_user_signature_cap.partial_centralized_signed_message_id);
    <b>assert</b>!(partial_signature.presign_cap.dwallet_id == message_approval.dwallet_id && message_approval.message == partial_signature.message && partial_signature.hash_scheme == message_approval.hash_scheme, <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EMessageApprovalMismatch">EMessageApprovalMismatch</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_ecdsa_sign"></a>

## Function `respond_ecdsa_sign`

Emits a <code>CompletedSignEvent</code> with the MPC Sign protocol output.

This function is called by the blockchain itself and is part of the core
blockchain logic executed by validators. The emitted event contains the
completed sign output that should be consumed by the initiating user.


<a name="@Parameters_33"></a>

##### Parameters

- **<code>signed_messages</code>**: A vector containing the signed message outputs.
- **<code>batch_session_id</code>**: The unique identifier for the batch signing session.
- **<code>ctx</code>**: The transaction context used for event emission.


<a name="@Requirements_34"></a>

##### Requirements

- The caller **must be the system address** (<code>@0x0</code>). If this condition is not met,
the function will abort with <code>ENotSystemAddress</code>.


<a name="@Events_35"></a>

##### Events

- **<code>CompletedSignEvent</code>**: Emitted with the <code>session_id</code> and <code>signed_messages</code>,
signaling the completion of the sign process for the batch session.


<a name="@Errors_36"></a>

##### Errors

- **<code>ENotSystemAddress</code>**: If the caller is not the system address (<code>@0x0</code>),
the function will abort with this error.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_ecdsa_sign">respond_ecdsa_sign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, sign_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, session_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, signature: vector&lt;u8&gt;, is_future_sign: bool, rejected: bool, session_sequence_number: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_ecdsa_sign">respond_ecdsa_sign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
    sign_id: ID,
    session_id: ID,
    signature: vector&lt;u8&gt;,
    is_future_sign: bool,
    rejected: bool,
    session_sequence_number: u64
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_remove_session_and_charge">remove_session_and_charge</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ECDSASignRequestEvent">ECDSASignRequestEvent</a>&gt;(session_sequence_number);
    <b>let</b> (dwallet, _) = self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_get_active_dwallet_and_public_output_mut">get_active_dwallet_and_public_output_mut</a>(dwallet_id);
    <b>let</b> sign = dwallet.ecdsa_signs.borrow_mut(sign_id);
    sign.state = match(sign.state) {
        ECDSASignState::Requested =&gt; {
            <b>if</b>(rejected) {
                event::emit(<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_RejectedECDSASignEvent">RejectedECDSASignEvent</a> {
                    sign_id,
                    session_id,
                    is_future_sign,
                });
                ECDSASignState::NetworkRejected
            } <b>else</b> {
                event::emit(<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CompletedECDSASignEvent">CompletedECDSASignEvent</a> {
                    sign_id,
                    session_id,
                    signature,
                    is_future_sign,
                });
                ECDSASignState::Completed { signature }
            }
        },
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_ESignWrongState">ESignWrongState</a>
    };
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_process_checkpoint_message_by_quorum"></a>

## Function `process_checkpoint_message_by_quorum`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, signature: vector&lt;u8&gt;, signers_bitmap: vector&lt;u8&gt;, message: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    signature: vector&lt;u8&gt;,
    signers_bitmap: vector&lt;u8&gt;,
    message: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> <b>mut</b> intent_bytes = <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_CHECKPOINT_MESSAGE_INTENT">CHECKPOINT_MESSAGE_INTENT</a>;
    intent_bytes.append(message);
    intent_bytes.append(bcs::to_bytes(&self.current_epoch));
    self.active_committee.verify_certificate(self.current_epoch, &signature, &signers_bitmap, &intent_bytes);
    self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_process_checkpoint_message">process_checkpoint_message</a>(message, ctx);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_process_checkpoint_message"></a>

## Function `process_checkpoint_message`



<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_process_checkpoint_message">process_checkpoint_message</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_secp256k1_inner::DWalletCoordinatorInner</a>, message: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_process_checkpoint_message">process_checkpoint_message</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    message: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(!self.active_committee.members().is_empty(), <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EActiveBlsCommitteeMustInitialize">EActiveBlsCommitteeMustInitialize</a>);
    <b>let</b> <b>mut</b> bcs_body = bcs::new(<b>copy</b> message);
    <b>let</b> epoch = bcs_body.peel_u64();
    <b>assert</b>!(epoch == self.current_epoch, <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EIncorrectEpochInCheckpoint">EIncorrectEpochInCheckpoint</a>);
    <b>let</b> sequence_number = bcs_body.peel_u64();
    <b>if</b>(self.last_processed_checkpoint_sequence_number.is_none()) {
        <b>assert</b>!(sequence_number == 0, <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EWrongCheckpointSequenceNumber">EWrongCheckpointSequenceNumber</a>);
        self.last_processed_checkpoint_sequence_number.fill(sequence_number);
    } <b>else</b> {
        <b>assert</b>!(sequence_number &gt; 0 && *self.last_processed_checkpoint_sequence_number.borrow() + 1 == sequence_number, <a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_EWrongCheckpointSequenceNumber">EWrongCheckpointSequenceNumber</a>);
        self.last_processed_checkpoint_sequence_number.swap(sequence_number);
    };
    <b>let</b> timestamp_ms = bcs_body.peel_u64();
    event::emit(<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_SystemCheckpointInfoEvent">SystemCheckpointInfoEvent</a> {
        epoch,
        sequence_number,
        timestamp_ms,
    });
    <b>let</b> len = bcs_body.peel_vec_length();
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> message_data_type = bcs_body.peel_vec_length();
            // Parses checkpoint BCS bytes directly.
            // Messages with `message_data_type` 1 & 2 are handled by the <a href="../ika_system/system.md#(ika_system=0x0)_system">system</a> <b>module</b>,
            // but their bytes must be extracted here to allow correct parsing of types 3 and above.
            // This step only extracts the bytes without further processing.
            <b>if</b> (message_data_type == 0) {
                <b>let</b> dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> first_round_output = bcs_body.peel_vec_u8();
                <b>let</b> session_sequence_number = bcs_body.peel_u64();
                self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_dwallet_dkg_first_round">respond_dwallet_dkg_first_round</a>(dwallet_id, first_round_output, session_sequence_number);
            } <b>else</b> <b>if</b> (message_data_type == 1) {
                <b>let</b> dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> session_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> public_output = bcs_body.peel_vec_u8();
                <b>let</b> encrypted_centralized_secret_share_and_proof = bcs_body.peel_vec_u8();
                <b>let</b> encryption_key_address = <a href="../sui/address.md#sui_address_from_bytes">sui::address::from_bytes</a>(bcs_body.peel_vec_u8());
                <b>let</b> rejected = bcs_body.peel_bool();
                <b>let</b> session_sequence_number = bcs_body.peel_u64();
                self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_dwallet_dkg_second_round">respond_dwallet_dkg_second_round</a>(
                    dwallet_id,
                    public_output,
                    encrypted_centralized_secret_share_and_proof,
                    encryption_key_address,
                    session_id,
                    rejected,
                    session_sequence_number,
                    ctx,
                );
            } <b>else</b> <b>if</b> (message_data_type == 2) {
                <b>let</b> dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> encrypted_user_secret_key_share_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> rejected = bcs_body.peel_bool();
                <b>let</b> session_sequence_number = bcs_body.peel_u64();
                self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_re_encrypt_user_share_for">respond_re_encrypt_user_share_for</a>(
                    dwallet_id,
                    encrypted_user_secret_key_share_id,
                    rejected,
                    session_sequence_number,
                );
            } <b>else</b> <b>if</b> (message_data_type == 3) {
                <b>let</b> dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> sign_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> session_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> signature = bcs_body.peel_vec_u8();
                <b>let</b> is_future_sign = bcs_body.peel_bool();
                <b>let</b> rejected = bcs_body.peel_bool();
                <b>let</b> session_sequence_number = bcs_body.peel_u64();
                self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_ecdsa_sign">respond_ecdsa_sign</a>(
                    dwallet_id,
                    sign_id,
                    session_id,
                    signature,
                    is_future_sign,
                    rejected,
                    session_sequence_number
                );
            } <b>else</b> <b>if</b> (message_data_type == 5) {
                <b>let</b> session_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> partial_centralized_signed_message_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> rejected = bcs_body.peel_bool();
                <b>let</b> session_sequence_number = bcs_body.peel_u64();
                self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_ecdsa_future_sign">respond_ecdsa_future_sign</a>(
                    session_id,
                    dwallet_id,
                    partial_centralized_signed_message_id,
                    rejected,
                    session_sequence_number
                );
            } <b>else</b> <b>if</b> (message_data_type == 4) {
                <b>let</b> dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> presign_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> session_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> presign = bcs_body.peel_vec_u8();
                <b>let</b> rejected = bcs_body.peel_bool();
                <b>let</b> session_sequence_number = bcs_body.peel_u64();
                self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_ecdsa_presign">respond_ecdsa_presign</a>(dwallet_id, presign_id, session_id, presign, rejected, session_sequence_number);
            } <b>else</b> <b>if</b> (message_data_type == 6) {
                <b>let</b> dwallet_network_decryption_key_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> public_output = bcs_body.peel_vec_u8();
                <b>let</b> is_last = bcs_body.peel_bool();
                self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_dwallet_network_decryption_key_dkg">respond_dwallet_network_decryption_key_dkg</a>(dwallet_network_decryption_key_id, public_output, is_last);
            } <b>else</b> <b>if</b> (message_data_type == 7) {
                <b>let</b> dwallet_network_decryption_key_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> public_output = bcs_body.peel_vec_u8();
                <b>let</b> is_last = bcs_body.peel_bool();
                self.<a href="../ika_system/dwallet_2pc_mpc_secp256k1_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_secp256k1_inner_respond_dwallet_network_decryption_key_reconfiguration">respond_dwallet_network_decryption_key_reconfiguration</a>(dwallet_network_decryption_key_id, public_output, is_last);
            };
        i = i + 1;
    };
    self.total_messages_processed = self.total_messages_processed + i;
}
</code></pre>



</details>
