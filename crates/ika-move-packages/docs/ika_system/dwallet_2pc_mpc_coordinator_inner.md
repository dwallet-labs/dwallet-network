---
title: Module `(ika_system=0x0)::dwallet_2pc_mpc_coordinator_inner`
---

This module handles the logic for creating and managing dWallets using the Secp256K1 signature scheme
and the DKG process. It leverages validators to execute MPC (Multi-Party Computation)
protocols to ensure trustless and decentralized wallet creation and key management.


-  [Struct `DWalletCoordinatorInner`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner)
-  [Struct `DWalletSessionEventKey`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSessionEventKey)
-  [Struct `DWalletSession`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSession)
-  [Struct `DWalletCap`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap)
-  [Struct `ImportedKeyDWalletCap`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyDWalletCap)
-  [Struct `DWalletNetworkDecryptionKeyCap`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDecryptionKeyCap)
-  [Struct `DWalletNetworkDecryptionKey`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDecryptionKey)
-  [Struct `EncryptionKey`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptionKey)
-  [Struct `EncryptedUserSecretKeyShare`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShare)
-  [Struct `UnverifiedPartialUserSignatureCap`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPartialUserSignatureCap)
-  [Struct `VerifiedPartialUserSignatureCap`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap)
-  [Struct `PartialUserSignature`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignature)
-  [Struct `DWallet`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet)
-  [Struct `UnverifiedPresignCap`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap)
-  [Struct `VerifiedPresignCap`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap)
-  [Struct `Presign`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_Presign)
-  [Struct `Sign`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_Sign)
-  [Struct `DWalletEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletEvent)
-  [Struct `CreatedEncryptionKeyEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CreatedEncryptionKeyEvent)
-  [Struct `DWalletNetworkDKGDecryptionKeyRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDKGDecryptionKeyRequestEvent)
-  [Struct `DWalletDecryptionKeyReshareRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDecryptionKeyReshareRequestEvent)
-  [Struct `CompletedDWalletDecryptionKeyReshareEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletDecryptionKeyReshareEvent)
-  [Struct `CompletedDWalletNetworkDKGDecryptionKeyEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletNetworkDKGDecryptionKeyEvent)
-  [Struct `DWalletDKGFirstRoundRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGFirstRoundRequestEvent)
-  [Struct `CompletedDWalletDKGFirstdRoundEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletDKGFirstdRoundEvent)
-  [Struct `RejectedDWalletDKGFirstRoundEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletDKGFirstRoundEvent)
-  [Struct `DWalletDKGSecondRoundRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGSecondRoundRequestEvent)
-  [Struct `CompletedDWalletDKGSecondRoundEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletDKGSecondRoundEvent)
-  [Struct `RejectedDWalletDKGSecondRoundEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletDKGSecondRoundEvent)
-  [Struct `DWalletImportedKeyVerificationRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletImportedKeyVerificationRequestEvent)
-  [Struct `CompletedDWalletImportedKeyVerificationEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletImportedKeyVerificationEvent)
-  [Struct `RejectedDWalletImportedKeyVerificationEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletImportedKeyVerificationEvent)
-  [Struct `EncryptedShareVerificationRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedShareVerificationRequestEvent)
-  [Struct `CompletedEncryptedShareVerificationEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedEncryptedShareVerificationEvent)
-  [Struct `RejectedEncryptedShareVerificationEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedEncryptedShareVerificationEvent)
-  [Struct `AcceptReEncryptedUserShareEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_AcceptReEncryptedUserShareEvent)
-  [Struct `MakeDWalletUserSecretKeySharesPublicRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MakeDWalletUserSecretKeySharesPublicRequestEvent)
-  [Struct `CompletedMakeDWalletUserSecretKeySharesPublicEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedMakeDWalletUserSecretKeySharesPublicEvent)
-  [Struct `RejectedMakeDWalletUserSecretKeySharesPublicEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedMakeDWalletUserSecretKeySharesPublicEvent)
-  [Struct `PresignRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignRequestEvent)
-  [Struct `CompletedPresignEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedPresignEvent)
-  [Struct `RejectedPresignEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedPresignEvent)
-  [Struct `SignRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignRequestEvent)
-  [Struct `FutureSignRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_FutureSignRequestEvent)
-  [Struct `CompletedFutureSignEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedFutureSignEvent)
-  [Struct `RejectedFutureSignEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedFutureSignEvent)
-  [Struct `CompletedSignEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedSignEvent)
-  [Struct `RejectedSignEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedSignEvent)
-  [Struct `DWalletCheckpointInfoEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCheckpointInfoEvent)
-  [Struct `MessageApproval`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval)
-  [Struct `ImportedKeyMessageApproval`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyMessageApproval)
-  [Enum `DWalletNetworkDecryptionKeyState`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDecryptionKeyState)
-  [Enum `EncryptedUserSecretKeyShareState`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShareState)
-  [Enum `PartialUserSignatureState`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignatureState)
-  [Enum `DWalletState`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletState)
-  [Enum `PresignState`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignState)
-  [Enum `SignState`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignState)
-  [Enum `SessionType`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionType)
-  [Constants](#@Constants_1)
-  [Function `lock_last_active_session_sequence_number`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_lock_last_active_session_sequence_number)
-  [Function `create_dwallet_coordinator_inner`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_create_dwallet_coordinator_inner)
-  [Function `request_dwallet_network_decryption_key_dkg`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_network_decryption_key_dkg)
-  [Function `respond_dwallet_network_decryption_key_dkg`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_network_decryption_key_dkg)
-  [Function `respond_dwallet_network_decryption_key_reconfiguration`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_network_decryption_key_reconfiguration)
-  [Function `advance_epoch_dwallet_network_decryption_key`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_advance_epoch_dwallet_network_decryption_key)
-  [Function `emit_start_reshare_event`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_emit_start_reshare_event)
-  [Function `get_active_dwallet_network_decryption_key`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_network_decryption_key)
-  [Function `advance_epoch`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_advance_epoch)
-  [Function `get_dwallet`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet)
-  [Function `get_dwallet_mut`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut)
-  [Function `validate_active_and_get_public_output`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_active_and_get_public_output)
-  [Function `charge_and_create_current_epoch_dwallet_event`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event)
-  [Function `create_system_dwallet_event`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_create_system_dwallet_event)
-  [Function `get_active_dwallet_and_public_output`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output)
-  [Function `get_active_dwallet_and_public_output_mut`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output_mut)
-  [Function `get_active_encryption_key`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_encryption_key)
-  [Function `validate_supported_curve`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_supported_curve)
-  [Function `validate_supported_curve_and_signature_algorithm`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_supported_curve_and_signature_algorithm)
-  [Function `validate_supported_curve_and_signature_algorithm_and_hash_scheme`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_supported_curve_and_signature_algorithm_and_hash_scheme)
-  [Function `register_encryption_key`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_register_encryption_key)
-  [Function `approve_message`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_approve_message)
-  [Function `approve_imported_key_message`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_approve_imported_key_message)
-  [Function `validate_approve_message`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_approve_message)
-  [Function `request_dwallet_dkg_first_round`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_dkg_first_round)
-  [Function `update_last_session_to_complete_in_current_epoch`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_update_last_session_to_complete_in_current_epoch)
-  [Function `all_current_epoch_sessions_completed`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_all_current_epoch_sessions_completed)
-  [Function `remove_session_and_charge`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_session_and_charge)
-  [Function `respond_dwallet_dkg_first_round`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_dkg_first_round)
-  [Function `request_dwallet_dkg_second_round`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_dkg_second_round)
-  [Function `respond_dwallet_dkg_second_round`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_dkg_second_round)
-  [Function `request_re_encrypt_user_share_for`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_re_encrypt_user_share_for)
-  [Function `respond_re_encrypt_user_share_for`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_re_encrypt_user_share_for)
-  [Function `accept_encrypted_user_share`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_accept_encrypted_user_share)
-  [Function `new_imported_key_dwallet`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_new_imported_key_dwallet)
-  [Function `request_imported_key_dwallet_verification`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_imported_key_dwallet_verification)
-  [Function `respond_imported_key_dwallet_verification`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_imported_key_dwallet_verification)
-  [Function `request_make_dwallet_user_secret_key_shares_public`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_make_dwallet_user_secret_key_shares_public)
-  [Function `respond_make_dwallet_user_secret_key_shares_public`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_make_dwallet_user_secret_key_shares_public)
-  [Function `request_presign`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_presign)
-  [Function `request_global_presign`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_global_presign)
-  [Function `respond_presign`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_presign)
-  [Function `is_presign_valid`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_is_presign_valid)
-  [Function `verify_presign_cap`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_verify_presign_cap)
-  [Function `validate_and_initiate_sign`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_and_initiate_sign)
-  [Function `request_sign`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_sign)
    -  [Effects](#@Effects_21)
    -  [Aborts](#@Aborts_22)
    -  [Parameters](#@Parameters_23)
    -  [Type Parameters](#@Type_Parameters_24)
-  [Function `request_imported_key_sign`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_imported_key_sign)
-  [Function `request_future_sign`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_future_sign)
-  [Function `respond_future_sign`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_future_sign)
-  [Function `is_partial_user_signature_valid`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_is_partial_user_signature_valid)
-  [Function `verify_partial_user_signature_cap`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_verify_partial_user_signature_cap)
-  [Function `request_sign_with_partial_user_signature`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_sign_with_partial_user_signature)
        -  [Type Parameters](#@Type_Parameters_25)
        -  [Parameters](#@Parameters_26)
        -  [Notes](#@Notes_27)
-  [Function `request_imported_key_sign_with_partial_user_signature`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_imported_key_sign_with_partial_user_signature)
-  [Function `match_partial_user_signature_with_message_approval`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_match_partial_user_signature_with_message_approval)
-  [Function `match_partial_user_signature_with_imported_key_message_approval`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_match_partial_user_signature_with_imported_key_message_approval)
-  [Function `respond_sign`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_sign)
-  [Function `process_checkpoint_message_by_quorum`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_process_checkpoint_message_by_quorum)
-  [Function `process_checkpoint_message`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_process_checkpoint_message)
-  [Function `set_supported_curves_to_signature_algorithms_to_hash_schemes`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_set_supported_curves_to_signature_algorithms_to_hash_schemes)
-  [Function `set_paused_curves_and_signature_algorithms`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_set_paused_curves_and_signature_algorithms)


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
<b>use</b> <a href="../sui/vec_map.md#sui_vec_map">sui::vec_map</a>;
<b>use</b> <a href="../sui/vec_set.md#sui_vec_set">sui::vec_set</a>;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner"></a>

## Struct `DWalletCoordinatorInner`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a> <b>has</b> store
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
<code>sessions: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;u64, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSession">dwallet_2pc_mpc_coordinator_inner::DWalletSession</a>&gt;</code>
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
<code>started_system_sessions_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>completed_system_sessions_count: u64</code>
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
<code>dwallets: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">dwallet_2pc_mpc_coordinator_inner::DWallet</a>&gt;</code>
</dt>
<dd>
 The key is the ID of <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a></code>.
</dd>
<dt>
<code>dwallet_network_decryption_keys: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDecryptionKey">dwallet_2pc_mpc_coordinator_inner::DWalletNetworkDecryptionKey</a>&gt;</code>
</dt>
<dd>
 The key is the ID of <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDecryptionKey">DWalletNetworkDecryptionKey</a></code>.
</dd>
<dt>
<code>encryption_keys: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;<b>address</b>, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptionKey">dwallet_2pc_mpc_coordinator_inner::EncryptionKey</a>&gt;</code>
</dt>
<dd>
 A table mapping user addresses to encryption key object IDs.
</dd>
<dt>
<code>presigns: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_Presign">dwallet_2pc_mpc_coordinator_inner::Presign</a>&gt;</code>
</dt>
<dd>
 A table mapping id to their presigns.
</dd>
<dt>
<code>partial_centralized_signed_messages: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignature">dwallet_2pc_mpc_coordinator_inner::PartialUserSignature</a>&gt;</code>
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
<code>previous_epoch_last_checkpoint_sequence_number: u64</code>
</dt>
<dd>
 The last checkpoint sequence number processed in the previous epoch.
</dd>
<dt>
<code>supported_curves_to_signature_algorithms_to_hash_schemes: <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u32, <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u32, vector&lt;u32&gt;&gt;&gt;</code>
</dt>
<dd>
 A nested map of supported curves to signature algorithms to hash schemes.
 e.g. secp256k1 -> (ecdsa -> [sha256, keccak256])
</dd>
<dt>
<code>paused_curves: vector&lt;u32&gt;</code>
</dt>
<dd>
 A list of paused curves in case of emergency.
 e.g. [secp256k1]
</dd>
<dt>
<code>paused_signature_algorithms: vector&lt;u32&gt;</code>
</dt>
<dd>
 A list of paused signature algorithms in case of emergency.
 e.g. [ecdsa]
</dd>
<dt>
<code>paused_hash_schemes: vector&lt;u32&gt;</code>
</dt>
<dd>
 A list of paused hash schemes in case of emergency.
 e.g. [sha256, keccak256]
</dd>
<dt>
<code>signature_algorithms_allowed_global_presign: vector&lt;u32&gt;</code>
</dt>
<dd>
 A list of signature algorithms that are allowed for global presign.
</dd>
<dt>
<code>extra_fields: <a href="../sui/bag.md#sui_bag_Bag">sui::bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSessionEventKey"></a>

## Struct `DWalletSessionEventKey`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSessionEventKey">DWalletSessionEventKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSession"></a>

## Struct `DWalletSession`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSession">DWalletSession</a> <b>has</b> key, store
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap"></a>

## Struct `DWalletCap`

Represents a capability granting control over a specific dWallet.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">DWalletCap</a> <b>has</b> key, store
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyDWalletCap"></a>

## Struct `ImportedKeyDWalletCap`

Represents a capability granting control over a specific imported key dWallet.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyDWalletCap">ImportedKeyDWalletCap</a> <b>has</b> key, store
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDecryptionKeyCap"></a>

## Struct `DWalletNetworkDecryptionKeyCap`

Represents a capability granting control over a specific dWallet network decryption key.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDecryptionKeyCap">DWalletNetworkDecryptionKeyCap</a> <b>has</b> key, store
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDecryptionKey"></a>

## Struct `DWalletNetworkDecryptionKey`

<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDecryptionKey">DWalletNetworkDecryptionKey</a></code> represents a network decryption key of
the homomorphically encrypted network share.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDecryptionKey">DWalletNetworkDecryptionKey</a> <b>has</b> key, store
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
<code>state: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDecryptionKeyState">dwallet_2pc_mpc_coordinator_inner::DWalletNetworkDecryptionKeyState</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptionKey"></a>

## Struct `EncryptionKey`

Represents an encryption key used to encrypt a dWallet centralized (user) secret key share.

Encryption keys facilitate secure data transfer between accounts on the
Ika by ensuring that sensitive information remains confidential during transmission.
Each address on the Ika is associated with a unique encryption key.
When an external party intends to send encrypted data to a particular account, they use the recipient's
encryption key to encrypt the data. The recipient is then the sole entity capable of decrypting
and accessing this information, ensuring secure, end-to-end encryption.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptionKey">EncryptionKey</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
 Unique identifier for the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptionKey">EncryptionKey</a></code>.
</dd>
<dt>
<code>created_at_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>curve: u32</code>
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShare"></a>

## Struct `EncryptedUserSecretKeyShare`

A verified Encrypted dWallet centralized secret key share.

This struct represents an encrypted centralized secret key share tied to
a specific dWallet (<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a></code>).
It includes cryptographic proof that the encryption is valid and securely linked
to the associated <code>dWallet</code>.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a> <b>has</b> key, store
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
 The ID of the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptionKey">EncryptionKey</a></code> object used to encrypt the secret share.
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
 The ID of the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a></code> the secret was re-encrypted from (None if created during dkg).
</dd>
<dt>
<code>state: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShareState">dwallet_2pc_mpc_coordinator_inner::EncryptedUserSecretKeyShareState</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPartialUserSignatureCap"></a>

## Struct `UnverifiedPartialUserSignatureCap`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPartialUserSignatureCap">UnverifiedPartialUserSignatureCap</a> <b>has</b> key, store
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap"></a>

## Struct `VerifiedPartialUserSignatureCap`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap">VerifiedPartialUserSignatureCap</a> <b>has</b> key, store
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignature"></a>

## Struct `PartialUserSignature`

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


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignature">PartialUserSignature</a> <b>has</b> key, store
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
<code>presign_cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">dwallet_2pc_mpc_coordinator_inner::VerifiedPresignCap</a></code>
</dt>
<dd>
</dd>
<dt>
<code>dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>cap_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>curve: u32</code>
</dt>
<dd>
</dd>
<dt>
<code>signature_algorithm: u32</code>
</dt>
<dd>
</dd>
<dt>
<code>hash_scheme: u32</code>
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
<code>state: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignatureState">dwallet_2pc_mpc_coordinator_inner::PartialUserSignatureState</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet"></a>

## Struct `DWallet`

<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a></code> represents a decentralized wallet (dWallet) that is
created after the Distributed key generation (DKG) process.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a> <b>has</b> key, store
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
<code>curve: u32</code>
</dt>
<dd>
 The elliptic curve used for the dWallet.
</dd>
<dt>
<code>public_user_secret_key_shares: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
 If not set, the user secret key shares is not public, and the user will need to
 keep it encrypted using encrypted user secret key shares. It is
 the case where we have zero trust for the dWallet becuase the
 user particiation is required.
 If set, the user secret key shares is public, the network can sign
 without the user participation. In this case, it is trust minimalized
 security for the user.
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
<code>is_imported_key_dwallet: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>encrypted_user_secret_key_shares: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShare">dwallet_2pc_mpc_coordinator_inner::EncryptedUserSecretKeyShare</a>&gt;</code>
</dt>
<dd>
 A table mapping id to their encryption key object.
</dd>
<dt>
<code>signs: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_Sign">dwallet_2pc_mpc_coordinator_inner::Sign</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>state: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletState">dwallet_2pc_mpc_coordinator_inner::DWalletState</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap"></a>

## Struct `UnverifiedPresignCap`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">UnverifiedPresignCap</a> <b>has</b> key, store
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
<code>dwallet_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;</code>
</dt>
<dd>
 The ID of the dWallet for which this Presign has been created and can be used by exclusively, if set.
 Optional, since some key signature algorithms (e.g., Schnorr and EdDSA) can support global presigns,
 which can be used for any dWallet (under the same network key).
</dd>
<dt>
<code>presign_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The ID of the presign.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap"></a>

## Struct `VerifiedPresignCap`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">VerifiedPresignCap</a> <b>has</b> key, store
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
<code>dwallet_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;</code>
</dt>
<dd>
 The ID of the dWallet for which this Presign has been created and can be used by exclusively, if set.
 Optional, since some key signature algorithms (e.g., Schnorr and EdDSA) can support global presigns,
 which can be used for any dWallet (under the same network key).
</dd>
<dt>
<code>presign_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The ID of the presign.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_Presign"></a>

## Struct `Presign`

Represents the result of the second and final presign round.
This struct links the results of both presign rounds to a specific dWallet ID.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_Presign">Presign</a> <b>has</b> key, store
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
<code>curve: u32</code>
</dt>
<dd>
 The elliptic curve used for the dWallet.
</dd>
<dt>
<code>signature_algorithm: u32</code>
</dt>
<dd>
 The signature algorithm for the presign.
</dd>
<dt>
<code>dwallet_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;</code>
</dt>
<dd>
 The ID of the dWallet for which this Presign has been created and can be used by exclusively, if set.
 Optional, since some key signature algorithms (e.g., Schnorr and EdDSA) can support global presigns,
 which can be used for any dWallet (under the same network key).
</dd>
<dt>
<code>cap_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>state: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignState">dwallet_2pc_mpc_coordinator_inner::PresignState</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_Sign"></a>

## Struct `Sign`

The output of a batched Sign session.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_Sign">Sign</a> <b>has</b> key, store
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
<code>state: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignState">dwallet_2pc_mpc_coordinator_inner::SignState</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletEvent"></a>

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
<code>session_type: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionType">dwallet_2pc_mpc_coordinator_inner::SessionType</a></code>
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CreatedEncryptionKeyEvent"></a>

## Struct `CreatedEncryptionKeyEvent`

Event emitted when an encryption key is created.

This event is emitted after the blockchain verifies the encryption key's validity
and creates the corresponding <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptionKey">EncryptionKey</a></code> object.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CreatedEncryptionKeyEvent">CreatedEncryptionKeyEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The unique identifier of the created <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptionKey">EncryptionKey</a></code> object.
</dd>
<dt>
<code>signer_address: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDKGDecryptionKeyRequestEvent"></a>

## Struct `DWalletNetworkDKGDecryptionKeyRequestEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDKGDecryptionKeyRequestEvent">DWalletNetworkDKGDecryptionKeyRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDecryptionKeyReshareRequestEvent"></a>

## Struct `DWalletDecryptionKeyReshareRequestEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDecryptionKeyReshareRequestEvent">DWalletDecryptionKeyReshareRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletDecryptionKeyReshareEvent"></a>

## Struct `CompletedDWalletDecryptionKeyReshareEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletDecryptionKeyReshareEvent">CompletedDWalletDecryptionKeyReshareEvent</a> <b>has</b> <b>copy</b>, drop, store
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletNetworkDKGDecryptionKeyEvent"></a>

## Struct `CompletedDWalletNetworkDKGDecryptionKeyEvent`

An event emitted when the first round of the DKG process is completed.

This event is emitted by the blockchain to notify the user about
the completion of the first round.
The user should catch this event to generate inputs for
the second round and call the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_dkg_second_round">request_dwallet_dkg_second_round</a>()</code> function.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletNetworkDKGDecryptionKeyEvent">CompletedDWalletNetworkDKGDecryptionKeyEvent</a> <b>has</b> <b>copy</b>, drop, store
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGFirstRoundRequestEvent"></a>

## Struct `DWalletDKGFirstRoundRequestEvent`

Event emitted to start the first round of the DKG process.

This event is caught by the blockchain, which is then using it to
initiate the first round of the DKG.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGFirstRoundRequestEvent">DWalletDKGFirstRoundRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
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
<dt>
<code>curve: u32</code>
</dt>
<dd>
 The elliptic curve used for the dWallet.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletDKGFirstdRoundEvent"></a>

## Struct `CompletedDWalletDKGFirstdRoundEvent`

An event emitted when the first round of the DKG process is completed.

This event is emitted by the blockchain to notify the user about
the completion of the first round.
The user should catch this event to generate inputs for
the second round and call the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_dkg_second_round">request_dwallet_dkg_second_round</a>()</code> function.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletDKGFirstdRoundEvent">CompletedDWalletDKGFirstdRoundEvent</a> <b>has</b> <b>copy</b>, drop, store
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletDKGFirstRoundEvent"></a>

## Struct `RejectedDWalletDKGFirstRoundEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletDKGFirstRoundEvent">RejectedDWalletDKGFirstRoundEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGSecondRoundRequestEvent"></a>

## Struct `DWalletDKGSecondRoundRequestEvent`

Event emitted to initiate the second round of the DKG process.

This event is emitted to notify Validators to begin the second round of the DKG.
It contains all necessary data to ensure proper continuation of the process.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGSecondRoundRequestEvent">DWalletDKGSecondRoundRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
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
 The <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptionKey">EncryptionKey</a></code> object used for encrypting the secret key share.
</dd>
<dt>
<code>encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The unique identifier of the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptionKey">EncryptionKey</a></code> object.
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
<code>signer_public_key: vector&lt;u8&gt;</code>
</dt>
<dd>
 The Ed25519 public key of the initiator,
 used to verify the signature on the centralized public output.
</dd>
<dt>
<code>dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The MPC network decryption key id that is used to decrypt associated dWallet.
</dd>
<dt>
<code>curve: u32</code>
</dt>
<dd>
 The elliptic curve used for the dWallet.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletDKGSecondRoundEvent"></a>

## Struct `CompletedDWalletDKGSecondRoundEvent`

Event emitted upon the completion of the second (and final) round of the
Distributed Key Generation (DKG).

This event provides all necessary data generated from the second
round of the DKG process.
Emitted to notify the centralized party.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletDKGSecondRoundEvent">CompletedDWalletDKGSecondRoundEvent</a> <b>has</b> <b>copy</b>, drop, store
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletDKGSecondRoundEvent"></a>

## Struct `RejectedDWalletDKGSecondRoundEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletDKGSecondRoundEvent">RejectedDWalletDKGSecondRoundEvent</a> <b>has</b> <b>copy</b>, drop, store
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletImportedKeyVerificationRequestEvent"></a>

## Struct `DWalletImportedKeyVerificationRequestEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletImportedKeyVerificationRequestEvent">DWalletImportedKeyVerificationRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
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
<code>centralized_party_message: vector&lt;u8&gt;</code>
</dt>
<dd>
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
 The <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptionKey">EncryptionKey</a></code> object used for encrypting the secret key share.
</dd>
<dt>
<code>encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The unique identifier of the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptionKey">EncryptionKey</a></code> object.
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
<code>signer_public_key: vector&lt;u8&gt;</code>
</dt>
<dd>
 The Ed25519 public key of the initiator,
 used to verify the signature on the centralized public output.
</dd>
<dt>
<code>dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The MPC network decryption key id that is used to decrypt associated dWallet.
</dd>
<dt>
<code>curve: u32</code>
</dt>
<dd>
 The elliptic curve used for the dWallet.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletImportedKeyVerificationEvent"></a>

## Struct `CompletedDWalletImportedKeyVerificationEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletImportedKeyVerificationEvent">CompletedDWalletImportedKeyVerificationEvent</a> <b>has</b> <b>copy</b>, drop, store
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
<code>public_output: vector&lt;u8&gt;</code>
</dt>
<dd>
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletImportedKeyVerificationEvent"></a>

## Struct `RejectedDWalletImportedKeyVerificationEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletImportedKeyVerificationEvent">RejectedDWalletImportedKeyVerificationEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedShareVerificationRequestEvent"></a>

## Struct `EncryptedShareVerificationRequestEvent`

Event emitted to start an encrypted dWallet centralized (user) key share
verification process.
Ika does not support native functions, so an event is emitted and
caught by the blockchain, which then starts the verification process,
similar to the MPC processes.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedShareVerificationRequestEvent">EncryptedShareVerificationRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
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
 The <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptionKey">EncryptionKey</a></code> Move object ID.
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
<code>dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>curve: u32</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedEncryptedShareVerificationEvent"></a>

## Struct `CompletedEncryptedShareVerificationEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedEncryptedShareVerificationEvent">CompletedEncryptedShareVerificationEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The ID of the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a></code> Move object.
</dd>
<dt>
<code>dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The ID of the dWallet associated with this encrypted secret share.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedEncryptedShareVerificationEvent"></a>

## Struct `RejectedEncryptedShareVerificationEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedEncryptedShareVerificationEvent">RejectedEncryptedShareVerificationEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The ID of the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a></code> Move object.
</dd>
<dt>
<code>dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The ID of the dWallet associated with this encrypted secret share.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_AcceptReEncryptedUserShareEvent"></a>

## Struct `AcceptReEncryptedUserShareEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_AcceptReEncryptedUserShareEvent">AcceptReEncryptedUserShareEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The ID of the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a></code> Move object.
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MakeDWalletUserSecretKeySharesPublicRequestEvent"></a>

## Struct `MakeDWalletUserSecretKeySharesPublicRequestEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MakeDWalletUserSecretKeySharesPublicRequestEvent">MakeDWalletUserSecretKeySharesPublicRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>public_user_secret_key_shares: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>public_output: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>curve: u32</code>
</dt>
<dd>
</dd>
<dt>
<code>dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedMakeDWalletUserSecretKeySharesPublicEvent"></a>

## Struct `CompletedMakeDWalletUserSecretKeySharesPublicEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedMakeDWalletUserSecretKeySharesPublicEvent">CompletedMakeDWalletUserSecretKeySharesPublicEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedMakeDWalletUserSecretKeySharesPublicEvent"></a>

## Struct `RejectedMakeDWalletUserSecretKeySharesPublicEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedMakeDWalletUserSecretKeySharesPublicEvent">RejectedMakeDWalletUserSecretKeySharesPublicEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignRequestEvent"></a>

## Struct `PresignRequestEvent`

Event emitted to initiate the first round of a Presign session.

This event is used to signal Validators to start the
first round of the Presign process.
The event includes all necessary details to link
the session to the corresponding dWallet
and DKG process.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignRequestEvent">PresignRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dwallet_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;</code>
</dt>
<dd>
 The ID of the dWallet for which this Presign has been created and can be used by exclusively, if set.
 Optional, since some key signature algorithms (e.g., Schnorr and EdDSA) can support global presigns,
 which can be used for any dWallet (under the same network key).
</dd>
<dt>
<code>presign_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The ID of the presign.
</dd>
<dt>
<code>dwallet_public_output: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
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
<dt>
<code>curve: u32</code>
</dt>
<dd>
 The curve used for the presign.
</dd>
<dt>
<code>signature_algorithm: u32</code>
</dt>
<dd>
 The signature algorithm for the presign.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedPresignEvent"></a>

## Struct `CompletedPresignEvent`

Event emitted when the presign batch is completed.

This event indicates the successful completion of a batched presign process.
It provides details about the presign objects created and their associated metadata.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedPresignEvent">CompletedPresignEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dwallet_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;</code>
</dt>
<dd>
 The ID of the dWallet for which this Presign has been created and can be used by exclusively, if set.
 Optional, since some key signature algorithms (e.g., Schnorr and EdDSA) can support global presigns,
 which can be used for any dWallet (under the same network key).
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedPresignEvent"></a>

## Struct `RejectedPresignEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedPresignEvent">RejectedPresignEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dwallet_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;</code>
</dt>
<dd>
 The ID of the dWallet for which this Presign has been created and can be used by exclusively, if set.
 Optional, since some key signature algorithms (e.g., Schnorr and EdDSA) can support global presigns,
 which can be used for any dWallet (under the same network key).
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignRequestEvent"></a>

## Struct `SignRequestEvent`

Event emitted to initiate the signing process.

This event is captured by Validators to start the signing protocol.
It includes all the necessary information to link the signing process
to a specific dWallet, and batched process.
D: The type of data that can be stored with the object,
specific to each Digital Signature Algorithm.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignRequestEvent">SignRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
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
<code>curve: u32</code>
</dt>
<dd>
 The elliptic curve used for the dWallet.
</dd>
<dt>
<code>signature_algorithm: u32</code>
</dt>
<dd>
 The signature algorithm used for the signing process.
</dd>
<dt>
<code>hash_scheme: u32</code>
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_FutureSignRequestEvent"></a>

## Struct `FutureSignRequestEvent`

Event emitted when a [<code>PartialCentralizedSignedMessages</code>] object is created.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_FutureSignRequestEvent">FutureSignRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
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
<code>curve: u32</code>
</dt>
<dd>
</dd>
<dt>
<code>signature_algorithm: u32</code>
</dt>
<dd>
</dd>
<dt>
<code>hash_scheme: u32</code>
</dt>
<dd>
</dd>
<dt>
<code>message_centralized_signature: vector&lt;u8&gt;</code>
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedFutureSignEvent"></a>

## Struct `CompletedFutureSignEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedFutureSignEvent">CompletedFutureSignEvent</a> <b>has</b> <b>copy</b>, drop, store
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedFutureSignEvent"></a>

## Struct `RejectedFutureSignEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedFutureSignEvent">RejectedFutureSignEvent</a> <b>has</b> <b>copy</b>, drop, store
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedSignEvent"></a>

## Struct `CompletedSignEvent`

Event emitted to signal the completion of a Sign process.

This event contains signatures for all signed messages in the batch.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedSignEvent">CompletedSignEvent</a> <b>has</b> <b>copy</b>, drop, store
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedSignEvent"></a>

## Struct `RejectedSignEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedSignEvent">RejectedSignEvent</a> <b>has</b> <b>copy</b>, drop, store
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCheckpointInfoEvent"></a>

## Struct `DWalletCheckpointInfoEvent`

Event containing dwallet 2pc-mpc checkpoint information, emitted during
the checkpoint submmision message.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCheckpointInfoEvent">DWalletCheckpointInfoEvent</a> <b>has</b> <b>copy</b>, drop, store
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval"></a>

## Struct `MessageApproval`

Represents a message that was approved as part of a dWallet process.

This struct binds the message to a specific <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">DWalletCap</a></code> for
traceability and accountability within the system.


<a name="@Fields_0"></a>

##### Fields

- **<code>dwallet_cap_id</code>**: The identifier of the dWallet capability
associated with this approval.
- **<code>hash_scheme</code>**: The message hash scheme.
- **<code>message</code>**: The message that has been approved.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">MessageApproval</a> <b>has</b> drop, store
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
<code>signature_algorithm: u32</code>
</dt>
<dd>
</dd>
<dt>
<code>hash_scheme: u32</code>
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyMessageApproval"></a>

## Struct `ImportedKeyMessageApproval`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyMessageApproval">ImportedKeyMessageApproval</a> <b>has</b> drop, store
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
<code>signature_algorithm: u32</code>
</dt>
<dd>
</dd>
<dt>
<code>hash_scheme: u32</code>
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDecryptionKeyState"></a>

## Enum `DWalletNetworkDecryptionKeyState`



<pre><code><b>public</b> <b>enum</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDecryptionKeyState">DWalletNetworkDecryptionKeyState</a> <b>has</b> <b>copy</b>, drop, store
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShareState"></a>

## Enum `EncryptedUserSecretKeyShareState`



<pre><code><b>public</b> <b>enum</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShareState">EncryptedUserSecretKeyShareState</a> <b>has</b> <b>copy</b>, drop, store
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignatureState"></a>

## Enum `PartialUserSignatureState`



<pre><code><b>public</b> <b>enum</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignatureState">PartialUserSignatureState</a> <b>has</b> <b>copy</b>, drop, store
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletState"></a>

## Enum `DWalletState`



<pre><code><b>public</b> <b>enum</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletState">DWalletState</a> <b>has</b> <b>copy</b>, drop, store
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
Variant <code>AwaitingNetworkDKGVerification</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>AwaitingNetworkImportedKeyVerification</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>AwaitingUserDKGVerificationInitiation</code>
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
Variant <code>AwaitingUserImportedKeyInitiation</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>DKGRequested</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>NetworkRejectedDKGRequest</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>NetworkRejectedDKGVerification</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>NetworkRejectedImportedKeyVerification</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignState"></a>

## Enum `PresignState`



<pre><code><b>public</b> <b>enum</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignState">PresignState</a> <b>has</b> <b>copy</b>, drop, store
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignState"></a>

## Enum `SignState`



<pre><code><b>public</b> <b>enum</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignState">SignState</a> <b>has</b> <b>copy</b>, drop, store
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionType"></a>

## Enum `SessionType`

The dWallet MPC session type
User initiated sessions have a sequence number, which is used to determine in which epoch
the session will get completed.
System sessions are guaranteed to always get completed in the epoch they were created in.


<pre><code><b>public</b> <b>enum</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionType">SessionType</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Variants</summary>


<dl>
<dt>
Variant <code>System</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>User</code>
</dt>
<dd>
</dd>

<dl>
<dt>
<code>sequence_number: u64</code>
</dt>
<dd>
</dd>
</dl>

</dl>


</details>

<a name="@Constants_1"></a>

## Constants


<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CHECKPOINT_MESSAGE_INTENT"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CHECKPOINT_MESSAGE_INTENT">CHECKPOINT_MESSAGE_INTENT</a>: vector&lt;u8&gt; = vector[1, 0, 0];
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EActiveBlsCommitteeMustInitialize"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EActiveBlsCommitteeMustInitialize">EActiveBlsCommitteeMustInitialize</a>: vector&lt;u8&gt; = b"First active committee must initialize.";
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ECannotAdvanceEpoch"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ECannotAdvanceEpoch">ECannotAdvanceEpoch</a>: u64 = 16;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ECurvePaused"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ECurvePaused">ECurvePaused</a>: u64 = 19;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletInactive"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletInactive">EDWalletInactive</a>: u64 = 2;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletMismatch"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletMismatch">EDWalletMismatch</a>: u64 = 1;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkDecryptionKeyNotActive"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkDecryptionKeyNotActive">EDWalletNetworkDecryptionKeyNotActive</a>: u64 = 14;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkDecryptionKeyNotExist"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkDecryptionKeyNotExist">EDWalletNetworkDecryptionKeyNotExist</a>: u64 = 5;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNotExists"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNotExists">EDWalletNotExists</a>: u64 = 3;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletUserSecretKeySharesAlreadyPublic"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletUserSecretKeySharesAlreadyPublic">EDWalletUserSecretKeySharesAlreadyPublic</a>: u64 = 21;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EHashSchemePaused"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EHashSchemePaused">EHashSchemePaused</a>: u64 = 25;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EImportedKeyDWallet"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EImportedKeyDWallet">EImportedKeyDWallet</a>: u64 = 23;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EIncorrectCap"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EIncorrectCap">EIncorrectCap</a>: u64 = 11;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EIncorrectEpochInCheckpoint"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EIncorrectEpochInCheckpoint">EIncorrectEpochInCheckpoint</a>: vector&lt;u8&gt; = b"The checkpoint epoch is incorrect.";
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidCurve"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidCurve">EInvalidCurve</a>: u64 = 17;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidEncryptionKeySignature"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidEncryptionKeySignature">EInvalidEncryptionKeySignature</a>: u64 = 6;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidHashScheme"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidHashScheme">EInvalidHashScheme</a>: u64 = 8;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidPresign"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidPresign">EInvalidPresign</a>: u64 = 15;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidSignatureAlgorithm"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidSignatureAlgorithm">EInvalidSignatureAlgorithm</a>: u64 = 18;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidSource"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidSource">EInvalidSource</a>: u64 = 13;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMessageApprovalMismatch"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMessageApprovalMismatch">EMessageApprovalMismatch</a>: u64 = 7;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMismatchCurve"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMismatchCurve">EMismatchCurve</a>: u64 = 22;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ENotImportedKeyDWallet"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ENotImportedKeyDWallet">ENotImportedKeyDWallet</a>: u64 = 24;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EPresignNotExist"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EPresignNotExist">EPresignNotExist</a>: u64 = 10;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ESignWrongState"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ESignWrongState">ESignWrongState</a>: u64 = 9;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ESignatureAlgorithmPaused"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ESignatureAlgorithmPaused">ESignatureAlgorithmPaused</a>: u64 = 20;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EUnverifiedCap"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EUnverifiedCap">EUnverifiedCap</a>: u64 = 12;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongCheckpointSequenceNumber"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongCheckpointSequenceNumber">EWrongCheckpointSequenceNumber</a>: vector&lt;u8&gt; = b"The checkpoint sequence number should be the expected next one.";
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongState"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongState">EWrongState</a>: u64 = 4;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_lock_last_active_session_sequence_number"></a>

## Function `lock_last_active_session_sequence_number`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_lock_last_active_session_sequence_number">lock_last_active_session_sequence_number</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_lock_last_active_session_sequence_number">lock_last_active_session_sequence_number</a>(self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>) {
    self.locked_last_session_to_complete_in_current_epoch = <b>true</b>;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_create_dwallet_coordinator_inner"></a>

## Function `create_dwallet_coordinator_inner`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_create_dwallet_coordinator_inner">create_dwallet_coordinator_inner</a>(current_epoch: u64, active_committee: (ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">bls_committee::BlsCommittee</a>, pricing: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing2PcMpcSecp256K1">dwallet_pricing::DWalletPricing2PcMpcSecp256K1</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_create_dwallet_coordinator_inner">create_dwallet_coordinator_inner</a>(
    current_epoch: u64,
    active_committee: BlsCommittee,
    pricing: DWalletPricing2PcMpcSecp256K1,
    ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a> {
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a> {
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
        presigns: object_table::new(ctx),
        partial_centralized_signed_messages: object_table::new(ctx),
        pricing,
        gas_fee_reimbursement_sui: balance::zero(),
        consensus_validation_fee_charged_ika: balance::zero(),
        active_committee,
        previous_committee: <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_empty">bls_committee::empty</a>(),
        total_messages_processed: 0,
        last_processed_checkpoint_sequence_number: option::none(),
        completed_system_sessions_count: 0,
        started_system_sessions_count: 0,
        previous_epoch_last_checkpoint_sequence_number: 0,
        supported_curves_to_signature_algorithms_to_hash_schemes: vec_map::empty(),
        paused_curves: vector[],
        paused_signature_algorithms: vector[],
        paused_hash_schemes: vector[],
        signature_algorithms_allowed_global_presign: vector[],
        extra_fields: bag::new(ctx),
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_network_decryption_key_dkg"></a>

## Function `request_dwallet_network_decryption_key_dkg`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_network_decryption_key_dkg">request_dwallet_network_decryption_key_dkg</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDecryptionKeyCap">dwallet_2pc_mpc_coordinator_inner::DWalletNetworkDecryptionKeyCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_network_decryption_key_dkg">request_dwallet_network_decryption_key_dkg</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDecryptionKeyCap">DWalletNetworkDecryptionKeyCap</a> {
    <b>let</b> id = object::new(ctx);
    <b>let</b> dwallet_network_decryption_key_id = id.to_inner();
    <b>let</b> cap = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDecryptionKeyCap">DWalletNetworkDecryptionKeyCap</a> {
        id: object::new(ctx),
        dwallet_network_decryption_key_id,
    };
    self.dwallet_network_decryption_keys.add(dwallet_network_decryption_key_id, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDecryptionKey">DWalletNetworkDecryptionKey</a> {
        id,
        dwallet_network_decryption_key_cap_id: object::id(&cap),
        current_epoch: self.current_epoch,
        reconfiguration_public_outputs: <a href="../sui/table.md#sui_table_new">sui::table::new</a>(ctx),
        network_dkg_public_output: table_vec::empty(ctx),
        computation_fee_charged_ika: balance::zero(),
        state: DWalletNetworkDecryptionKeyState::AwaitingNetworkDKG,
    });
    event::emit(self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_create_system_dwallet_event">create_system_dwallet_event</a>(
        dwallet_network_decryption_key_id,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDKGDecryptionKeyRequestEvent">DWalletNetworkDKGDecryptionKeyRequestEvent</a> {
            dwallet_network_decryption_key_id
        },
        ctx,
    ));
    cap
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_network_decryption_key_dkg"></a>

## Function `respond_dwallet_network_decryption_key_dkg`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_network_decryption_key_dkg">respond_dwallet_network_decryption_key_dkg</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, network_public_output: vector&lt;u8&gt;, is_last_chunk: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_network_decryption_key_dkg">respond_dwallet_network_decryption_key_dkg</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_network_decryption_key_id: ID,
    network_public_output: vector&lt;u8&gt;,
    is_last_chunk: bool,
) {
    <b>if</b> (is_last_chunk) {
        self.completed_system_sessions_count = self.completed_system_sessions_count + 1;
    };
    <b>let</b> dwallet_network_decryption_key = self.dwallet_network_decryption_keys.borrow_mut(dwallet_network_decryption_key_id);
    dwallet_network_decryption_key.network_dkg_public_output.push_back(network_public_output);
    dwallet_network_decryption_key.state = match (&dwallet_network_decryption_key.state) {
        DWalletNetworkDecryptionKeyState::AwaitingNetworkDKG =&gt; {
            <b>if</b> (is_last_chunk) {
                event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletNetworkDKGDecryptionKeyEvent">CompletedDWalletNetworkDKGDecryptionKeyEvent</a> {
                    dwallet_network_decryption_key_id,
                });
                DWalletNetworkDecryptionKeyState::NetworkDKGCompleted
            } <b>else</b> {
                DWalletNetworkDecryptionKeyState::AwaitingNetworkDKG
            }
        },
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongState">EWrongState</a>
    };
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_network_decryption_key_reconfiguration"></a>

## Function `respond_dwallet_network_decryption_key_reconfiguration`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_network_decryption_key_reconfiguration">respond_dwallet_network_decryption_key_reconfiguration</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, public_output: vector&lt;u8&gt;, is_last_chunk: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_network_decryption_key_reconfiguration">respond_dwallet_network_decryption_key_reconfiguration</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_network_decryption_key_id: ID,
    public_output: vector&lt;u8&gt;,
    is_last_chunk: bool,
) {
    <b>if</b> (is_last_chunk) {
        self.completed_system_sessions_count = self.completed_system_sessions_count + 1;
    };
    <b>let</b> dwallet_network_decryption_key = self.dwallet_network_decryption_keys.borrow_mut(dwallet_network_decryption_key_id);
    <b>let</b> next_reconfiguration_public_output = dwallet_network_decryption_key.reconfiguration_public_outputs.borrow_mut(dwallet_network_decryption_key.current_epoch + 1);
    next_reconfiguration_public_output.push_back(public_output);
    dwallet_network_decryption_key.state = match (&dwallet_network_decryption_key.state) {
        DWalletNetworkDecryptionKeyState::AwaitingNetworkReconfiguration =&gt; {
            <b>if</b> (is_last_chunk) {
                event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletDecryptionKeyReshareEvent">CompletedDWalletDecryptionKeyReshareEvent</a> {
                    dwallet_network_decryption_key_id,
                });
                DWalletNetworkDecryptionKeyState::AwaitingNextEpochReconfiguration
            } <b>else</b> {
                DWalletNetworkDecryptionKeyState::AwaitingNetworkReconfiguration
            }
        },
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongState">EWrongState</a>
    };
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_advance_epoch_dwallet_network_decryption_key"></a>

## Function `advance_epoch_dwallet_network_decryption_key`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_advance_epoch_dwallet_network_decryption_key">advance_epoch_dwallet_network_decryption_key</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, cap: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDecryptionKeyCap">dwallet_2pc_mpc_coordinator_inner::DWalletNetworkDecryptionKeyCap</a>): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_advance_epoch_dwallet_network_decryption_key">advance_epoch_dwallet_network_decryption_key</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    cap: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDecryptionKeyCap">DWalletNetworkDecryptionKeyCap</a>,
): Balance&lt;IKA&gt; {
    <b>let</b> dwallet_network_decryption_key = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_network_decryption_key">get_active_dwallet_network_decryption_key</a>(
        cap.dwallet_network_decryption_key_id
    );
    <b>assert</b>!(dwallet_network_decryption_key.dwallet_network_decryption_key_cap_id == cap.id.to_inner(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EIncorrectCap">EIncorrectCap</a>);
    <b>assert</b>!(dwallet_network_decryption_key.state == DWalletNetworkDecryptionKeyState::AwaitingNextEpochReconfiguration, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongState">EWrongState</a>);
    dwallet_network_decryption_key.current_epoch = dwallet_network_decryption_key.current_epoch + 1;
    dwallet_network_decryption_key.state = DWalletNetworkDecryptionKeyState::NetworkReconfigurationCompleted;
    <b>let</b> <b>mut</b> epoch_computation_fee_charged_ika = <a href="../sui/balance.md#sui_balance_zero">sui::balance::zero</a>&lt;IKA&gt;();
    epoch_computation_fee_charged_ika.join(dwallet_network_decryption_key.computation_fee_charged_ika.withdraw_all());
    <b>return</b> epoch_computation_fee_charged_ika
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_emit_start_reshare_event"></a>

## Function `emit_start_reshare_event`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_emit_start_reshare_event">emit_start_reshare_event</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, key_cap: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDecryptionKeyCap">dwallet_2pc_mpc_coordinator_inner::DWalletNetworkDecryptionKeyCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_emit_start_reshare_event">emit_start_reshare_event</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>, key_cap: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDecryptionKeyCap">DWalletNetworkDecryptionKeyCap</a>, ctx: &<b>mut</b> TxContext
) {
    <b>let</b> dwallet_network_decryption_key = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_network_decryption_key">get_active_dwallet_network_decryption_key</a>(key_cap.dwallet_network_decryption_key_id);
    dwallet_network_decryption_key.state = DWalletNetworkDecryptionKeyState::AwaitingNetworkReconfiguration;
    dwallet_network_decryption_key.reconfiguration_public_outputs.add(dwallet_network_decryption_key.current_epoch + 1, table_vec::empty(ctx));
    event::emit(self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_create_system_dwallet_event">create_system_dwallet_event</a>(
        key_cap.dwallet_network_decryption_key_id,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDecryptionKeyReshareRequestEvent">DWalletDecryptionKeyReshareRequestEvent</a> {
            dwallet_network_decryption_key_id: key_cap.dwallet_network_decryption_key_id
        },
        ctx,
    ));
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_network_decryption_key"></a>

## Function `get_active_dwallet_network_decryption_key`



<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_network_decryption_key">get_active_dwallet_network_decryption_key</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDecryptionKey">dwallet_2pc_mpc_coordinator_inner::DWalletNetworkDecryptionKey</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_network_decryption_key">get_active_dwallet_network_decryption_key</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_network_decryption_key_id: ID,
): &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDecryptionKey">DWalletNetworkDecryptionKey</a> {
    <b>let</b> dwallet_network_decryption_key = self.dwallet_network_decryption_keys.borrow_mut(dwallet_network_decryption_key_id);
    <b>assert</b>!(dwallet_network_decryption_key.state != DWalletNetworkDecryptionKeyState::AwaitingNetworkDKG, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkDecryptionKeyNotActive">EDWalletNetworkDecryptionKeyNotActive</a>);
    dwallet_network_decryption_key
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_advance_epoch"></a>

## Function `advance_epoch`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_advance_epoch">advance_epoch</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, next_committee: (ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">bls_committee::BlsCommittee</a>): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_advance_epoch">advance_epoch</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    next_committee: BlsCommittee
): Balance&lt;IKA&gt; {
    <b>assert</b>!(self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_all_current_epoch_sessions_completed">all_current_epoch_sessions_completed</a>(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ECannotAdvanceEpoch">ECannotAdvanceEpoch</a>);
    <b>if</b> (self.last_processed_checkpoint_sequence_number.is_some()) {
        <b>let</b> last_processed_checkpoint_sequence_number = *self.last_processed_checkpoint_sequence_number.borrow();
        self.previous_epoch_last_checkpoint_sequence_number = last_processed_checkpoint_sequence_number;
    };
    self.locked_last_session_to_complete_in_current_epoch = <b>false</b>;
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_update_last_session_to_complete_in_current_epoch">update_last_session_to_complete_in_current_epoch</a>();
    self.current_epoch = self.current_epoch + 1;
    self.previous_committee = self.active_committee;
    self.active_committee = next_committee;
    <b>let</b> <b>mut</b> epoch_consensus_validation_fee_charged_ika = <a href="../sui/balance.md#sui_balance_zero">sui::balance::zero</a>&lt;IKA&gt;();
    epoch_consensus_validation_fee_charged_ika.join(self.consensus_validation_fee_charged_ika.withdraw_all());
    <b>return</b> epoch_consensus_validation_fee_charged_ika
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet"></a>

## Function `get_dwallet`



<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet">get_dwallet</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">dwallet_2pc_mpc_coordinator_inner::DWallet</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet">get_dwallet</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
): &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a> {
    <b>assert</b>!(self.dwallets.contains(dwallet_id), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNotExists">EDWalletNotExists</a>);
    self.dwallets.borrow(dwallet_id)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut"></a>

## Function `get_dwallet_mut`



<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut">get_dwallet_mut</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">dwallet_2pc_mpc_coordinator_inner::DWallet</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut">get_dwallet_mut</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
): &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a> {
    <b>assert</b>!(self.dwallets.contains(dwallet_id), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNotExists">EDWalletNotExists</a>);
    self.dwallets.borrow_mut(dwallet_id)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_active_and_get_public_output"></a>

## Function `validate_active_and_get_public_output`



<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_active_and_get_public_output">validate_active_and_get_public_output</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">dwallet_2pc_mpc_coordinator_inner::DWallet</a>): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_active_and_get_public_output">validate_active_and_get_public_output</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a>,
): &vector&lt;u8&gt; {
    match (&self.state) {
        DWalletState::Active {
            public_output,
        } =&gt; {
            public_output
        },
        DWalletState::DKGRequested |
        DWalletState::NetworkRejectedDKGRequest |
        DWalletState::AwaitingUserDKGVerificationInitiation { .. } |
        DWalletState::AwaitingNetworkDKGVerification |
        DWalletState::NetworkRejectedDKGVerification |
        DWalletState::AwaitingUserImportedKeyInitiation |
        DWalletState::AwaitingNetworkImportedKeyVerification |
        DWalletState::NetworkRejectedImportedKeyVerification =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletInactive">EDWalletInactive</a>,
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event"></a>

## Function `charge_and_create_current_epoch_dwallet_event`



<pre><code><b>fun</b> charge_and_create_current_epoch_dwallet_eventE(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, pricing: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">dwallet_pricing::PricingPerOperation</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, event_data: E, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletEvent">dwallet_2pc_mpc_coordinator_inner::DWalletEvent</a>&lt;E&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>&lt;E: <b>copy</b> + drop + store&gt;(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_network_decryption_key_id: ID,
    pricing: PricingPerOperation,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    event_data: E,
    ctx: &<b>mut</b> TxContext,
): <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletEvent">DWalletEvent</a>&lt;E&gt; {
    <b>assert</b>!(self.dwallet_network_decryption_keys.contains(dwallet_network_decryption_key_id), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkDecryptionKeyNotExist">EDWalletNetworkDecryptionKeyNotExist</a>);
    <b>let</b> computation_fee_charged_ika = payment_ika.split(pricing.computation_ika(), ctx).into_balance();
    <b>let</b> consensus_validation_fee_charged_ika = payment_ika.split(pricing.consensus_validation_ika(), ctx).into_balance();
    <b>let</b> gas_fee_reimbursement_sui = payment_sui.split(pricing.gas_fee_reimbursement_sui(), ctx).into_balance();
    <b>let</b> session_sequence_number = self.next_session_sequence_number;
    <b>let</b> session = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSession">DWalletSession</a> {
        id: object::new(ctx),
        session_sequence_number,
        dwallet_network_decryption_key_id,
        consensus_validation_fee_charged_ika,
        computation_fee_charged_ika,
        gas_fee_reimbursement_sui,
    };
    <b>let</b> event = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletEvent">DWalletEvent</a> {
        epoch: self.current_epoch,
        session_type: {
            SessionType::User {
                sequence_number: session_sequence_number,
            }
        },
        session_id: object::id(&session),
        event_data,
    };
    self.session_start_events.add(session.id.to_inner(), event);
    self.sessions.add(session_sequence_number, session);
    self.next_session_sequence_number = session_sequence_number + 1;
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_update_last_session_to_complete_in_current_epoch">update_last_session_to_complete_in_current_epoch</a>();
    event
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_create_system_dwallet_event"></a>

## Function `create_system_dwallet_event`



<pre><code><b>fun</b> create_system_dwallet_eventE(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, event_data: E, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletEvent">dwallet_2pc_mpc_coordinator_inner::DWalletEvent</a>&lt;E&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_create_system_dwallet_event">create_system_dwallet_event</a>&lt;E: <b>copy</b> + drop + store&gt;(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_network_decryption_key_id: ID,
    event_data: E,
    ctx: &<b>mut</b> TxContext,
): <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletEvent">DWalletEvent</a>&lt;E&gt; {
    <b>assert</b>!(self.dwallet_network_decryption_keys.contains(dwallet_network_decryption_key_id), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkDecryptionKeyNotExist">EDWalletNetworkDecryptionKeyNotExist</a>);
    self.started_system_sessions_count = self.started_system_sessions_count + 1;
    <b>let</b> event = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletEvent">DWalletEvent</a> {
        epoch: self.current_epoch,
        session_type: SessionType::System,
        session_id: object::id_from_address(tx_context::fresh_object_address(ctx)),
        event_data,
    };
    event
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output"></a>

## Function `get_active_dwallet_and_public_output`



<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output">get_active_dwallet_and_public_output</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): (&(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">dwallet_2pc_mpc_coordinator_inner::DWallet</a>, vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output">get_active_dwallet_and_public_output</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
): (&<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a>, vector&lt;u8&gt;) {
    <b>assert</b>!(self.dwallets.contains(dwallet_id), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNotExists">EDWalletNotExists</a>);
    <b>let</b> dwallet = self.dwallets.borrow(dwallet_id);
    <b>let</b> public_output = dwallet.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_active_and_get_public_output">validate_active_and_get_public_output</a>();
    (dwallet, *public_output)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output_mut"></a>

## Function `get_active_dwallet_and_public_output_mut`



<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output_mut">get_active_dwallet_and_public_output_mut</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): (&<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">dwallet_2pc_mpc_coordinator_inner::DWallet</a>, vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output_mut">get_active_dwallet_and_public_output_mut</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
): (&<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a>, vector&lt;u8&gt;) {
    <b>assert</b>!(self.dwallets.contains(dwallet_id), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNotExists">EDWalletNotExists</a>);
    <b>let</b> dwallet = self.dwallets.borrow_mut(dwallet_id);
    <b>let</b> public_output = dwallet.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_active_and_get_public_output">validate_active_and_get_public_output</a>();
    (dwallet, *public_output)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_encryption_key"></a>

## Function `get_active_encryption_key`

Get the active encryption key ID by its address.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_encryption_key">get_active_encryption_key</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, <b>address</b>: <b>address</b>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_encryption_key">get_active_encryption_key</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    <b>address</b>: <b>address</b>,
): ID {
    self.encryption_keys.borrow(<b>address</b>).id.to_inner()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_supported_curve"></a>

## Function `validate_supported_curve`



<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_supported_curve">validate_supported_curve</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, curve: u32)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_supported_curve">validate_supported_curve</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    curve: u32,
) {
    <b>assert</b>!(self.supported_curves_to_signature_algorithms_to_hash_schemes.contains(&curve), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidCurve">EInvalidCurve</a>);
    <b>assert</b>!(!self.paused_curves.contains(&curve), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ECurvePaused">ECurvePaused</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_supported_curve_and_signature_algorithm"></a>

## Function `validate_supported_curve_and_signature_algorithm`



<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_supported_curve_and_signature_algorithm">validate_supported_curve_and_signature_algorithm</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, curve: u32, signature_algorithm: u32)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_supported_curve_and_signature_algorithm">validate_supported_curve_and_signature_algorithm</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    curve: u32,
    signature_algorithm: u32,
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_supported_curve">validate_supported_curve</a>(curve);
    <b>let</b> supported_curve_to_signature_algorithms = self.supported_curves_to_signature_algorithms_to_hash_schemes[&curve];
    <b>assert</b>!(supported_curve_to_signature_algorithms.contains(&signature_algorithm), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidSignatureAlgorithm">EInvalidSignatureAlgorithm</a>);
    <b>assert</b>!(!self.paused_signature_algorithms.contains(&signature_algorithm), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ESignatureAlgorithmPaused">ESignatureAlgorithmPaused</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_supported_curve_and_signature_algorithm_and_hash_scheme"></a>

## Function `validate_supported_curve_and_signature_algorithm_and_hash_scheme`



<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_supported_curve_and_signature_algorithm_and_hash_scheme">validate_supported_curve_and_signature_algorithm_and_hash_scheme</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, curve: u32, signature_algorithm: u32, hash_scheme: u32)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_supported_curve_and_signature_algorithm_and_hash_scheme">validate_supported_curve_and_signature_algorithm_and_hash_scheme</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    curve: u32,
    signature_algorithm: u32,
    hash_scheme: u32,
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_supported_curve_and_signature_algorithm">validate_supported_curve_and_signature_algorithm</a>(curve, signature_algorithm);
    <b>let</b> supported_hash_schemes = self.supported_curves_to_signature_algorithms_to_hash_schemes[&curve][&signature_algorithm];
    <b>assert</b>!(supported_hash_schemes.contains(&hash_scheme), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidHashScheme">EInvalidHashScheme</a>);
    <b>assert</b>!(!self.paused_hash_schemes.contains(&hash_scheme), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EHashSchemePaused">EHashSchemePaused</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_register_encryption_key"></a>

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


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_register_encryption_key">register_encryption_key</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, curve: u32, encryption_key: vector&lt;u8&gt;, encryption_key_signature: vector&lt;u8&gt;, signer_public_key: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_register_encryption_key">register_encryption_key</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    curve: u32,
    encryption_key: vector&lt;u8&gt;,
    encryption_key_signature: vector&lt;u8&gt;,
    signer_public_key: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_supported_curve">validate_supported_curve</a>(curve);
    <b>assert</b>!(
        ed25519_verify(&encryption_key_signature, &signer_public_key, &encryption_key),
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidEncryptionKeySignature">EInvalidEncryptionKeySignature</a>
    );
    <b>let</b> signer_address = <a href="../ika_system/address.md#(ika_system=0x0)_address_ed25519_address">address::ed25519_address</a>(signer_public_key);
    <b>let</b> id = object::new(ctx);
    <b>let</b> encryption_key_id = id.to_inner();
    self.encryption_keys.add(signer_address, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptionKey">EncryptionKey</a> {
        id,
        created_at_epoch: self.current_epoch,
        curve,
        encryption_key,
        encryption_key_signature,
        signer_public_key,
        signer_address,
    });
    // Emit an event to signal the creation of the encryption key
    event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CreatedEncryptionKeyEvent">CreatedEncryptionKeyEvent</a> {
        encryption_key_id,
        signer_address,
    });
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_approve_message"></a>

## Function `approve_message`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_approve_message">approve_message</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_cap: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">dwallet_2pc_mpc_coordinator_inner::DWalletCap</a>, signature_algorithm: u32, hash_scheme: u32, message: vector&lt;u8&gt;): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">dwallet_2pc_mpc_coordinator_inner::MessageApproval</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_approve_message">approve_message</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_cap: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">DWalletCap</a>,
    signature_algorithm: u32,
    hash_scheme: u32,
    message: vector&lt;u8&gt;
): <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">MessageApproval</a> {
    <b>let</b> dwallet_id = dwallet_cap.dwallet_id;
    <b>let</b> is_imported_key_dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_approve_message">validate_approve_message</a>(dwallet_id, signature_algorithm, hash_scheme);
    <b>assert</b>!(!is_imported_key_dwallet, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EImportedKeyDWallet">EImportedKeyDWallet</a>);
    <b>let</b> approval = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">MessageApproval</a> {
        dwallet_id,
        signature_algorithm,
        hash_scheme,
        message,
    };
    approval
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_approve_imported_key_message"></a>

## Function `approve_imported_key_message`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_approve_imported_key_message">approve_imported_key_message</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, imported_key_dwallet_cap: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyDWalletCap">dwallet_2pc_mpc_coordinator_inner::ImportedKeyDWalletCap</a>, signature_algorithm: u32, hash_scheme: u32, message: vector&lt;u8&gt;): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyMessageApproval">dwallet_2pc_mpc_coordinator_inner::ImportedKeyMessageApproval</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_approve_imported_key_message">approve_imported_key_message</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    imported_key_dwallet_cap: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyDWalletCap">ImportedKeyDWalletCap</a>,
    signature_algorithm: u32,
    hash_scheme: u32,
    message: vector&lt;u8&gt;
): <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyMessageApproval">ImportedKeyMessageApproval</a> {
    <b>let</b> dwallet_id = imported_key_dwallet_cap.dwallet_id;
    <b>let</b> is_imported_key_dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_approve_message">validate_approve_message</a>(dwallet_id, signature_algorithm, hash_scheme);
    <b>assert</b>!(is_imported_key_dwallet, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ENotImportedKeyDWallet">ENotImportedKeyDWallet</a>);
    <b>let</b> approval = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyMessageApproval">ImportedKeyMessageApproval</a> {
        dwallet_id,
        signature_algorithm,
        hash_scheme,
        message,
    };
    approval
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_approve_message"></a>

## Function `validate_approve_message`



<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_approve_message">validate_approve_message</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, signature_algorithm: u32, hash_scheme: u32): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_approve_message">validate_approve_message</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
    signature_algorithm: u32,
    hash_scheme: u32,
): bool {
    <b>let</b> (dwallet, _) = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output">get_active_dwallet_and_public_output</a>(dwallet_id);
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_supported_curve_and_signature_algorithm_and_hash_scheme">validate_supported_curve_and_signature_algorithm_and_hash_scheme</a>(dwallet.curve, signature_algorithm, hash_scheme);
    dwallet.is_imported_key_dwallet
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_dkg_first_round"></a>

## Function `request_dwallet_dkg_first_round`

Starts the first Distributed Key Generation (DKG) session.

This function creates a new <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">DWalletCap</a></code> object,
transfers it to the session initiator,
and emits a <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGFirstRoundRequestEvent">DWalletDKGFirstRoundRequestEvent</a></code> to signal
the beginning of the DKG process.


<a name="@Parameters_3"></a>

##### Parameters



<a name="@Effects_4"></a>

##### Effects

- Generates a new <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">DWalletCap</a></code> object.
- Transfers the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">DWalletCap</a></code> to the session initiator (<code>ctx.sender</code>).
- Emits a <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGFirstRoundRequestEvent">DWalletDKGFirstRoundRequestEvent</a></code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_dkg_first_round">request_dwallet_dkg_first_round</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, curve: u32, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">dwallet_2pc_mpc_coordinator_inner::DWalletCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_dkg_first_round">request_dwallet_dkg_first_round</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_network_decryption_key_id: ID,
    curve: u32,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">DWalletCap</a> {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_supported_curve">validate_supported_curve</a>(curve);
    <b>let</b> pricing = self.pricing.dkg_first_round();
    <b>assert</b>!(self.dwallet_network_decryption_keys.contains(dwallet_network_decryption_key_id), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkDecryptionKeyNotExist">EDWalletNetworkDecryptionKeyNotExist</a>);
    <b>let</b> id = object::new(ctx);
    <b>let</b> dwallet_id = id.to_inner();
    <b>let</b> dwallet_cap = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">DWalletCap</a> {
        id: object::new(ctx),
        dwallet_id,
    };
    <b>let</b> dwallet_cap_id = object::id(&dwallet_cap);
    self.dwallets.add(dwallet_id, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a> {
        id,
        created_at_epoch: self.current_epoch,
        curve,
        public_user_secret_key_shares: option::none(),
        dwallet_cap_id,
        dwallet_network_decryption_key_id,
        is_imported_key_dwallet: <b>false</b>,
        encrypted_user_secret_key_shares: object_table::new(ctx),
        signs: object_table::new(ctx),
        state: DWalletState::DKGRequested,
    });
    event::emit(self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
                dwallet_network_decryption_key_id,
        pricing,
        payment_ika,
        payment_sui,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGFirstRoundRequestEvent">DWalletDKGFirstRoundRequestEvent</a> {
            dwallet_id,
            dwallet_cap_id,
            dwallet_network_decryption_key_id,
            curve,
        },
        ctx,
    ));
    dwallet_cap
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_update_last_session_to_complete_in_current_epoch"></a>

## Function `update_last_session_to_complete_in_current_epoch`

Updates the <code>last_session_to_complete_in_current_epoch</code> field.
We do this to ensure that the last session to complete in the current epoch is equal
to the desired completed sessions count.
This is part of the epoch switch logic.


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_update_last_session_to_complete_in_current_epoch">update_last_session_to_complete_in_current_epoch</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_update_last_session_to_complete_in_current_epoch">update_last_session_to_complete_in_current_epoch</a>(self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>) {
    <b>if</b> (self.locked_last_session_to_complete_in_current_epoch) {
        <b>return</b>
    };
    <b>let</b> new_last_session_to_complete_in_current_epoch = (
        self.number_of_completed_sessions + self.max_active_sessions_buffer
    ).min(
        // Setting it to the `next_session_sequence_number` and not `next_session_sequence_number - 1`,
        // <b>as</b> we compare this index against the `number_of_completed_sessions` counter, that starts counting from 1.
        self.next_session_sequence_number,
    );
    <b>if</b> (self.last_session_to_complete_in_current_epoch &gt;= new_last_session_to_complete_in_current_epoch) {
        <b>return</b>
    };
    self.last_session_to_complete_in_current_epoch = new_last_session_to_complete_in_current_epoch;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_all_current_epoch_sessions_completed"></a>

## Function `all_current_epoch_sessions_completed`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_all_current_epoch_sessions_completed">all_current_epoch_sessions_completed</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_all_current_epoch_sessions_completed">all_current_epoch_sessions_completed</a>(self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>): bool {
    <b>return</b> self.locked_last_session_to_complete_in_current_epoch &&
        self.number_of_completed_sessions == self.last_session_to_complete_in_current_epoch &&
        self.completed_system_sessions_count == self.started_system_sessions_count
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_session_and_charge"></a>

## Function `remove_session_and_charge`



<pre><code><b>fun</b> remove_session_and_chargeE(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, session_sequence_number: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_session_and_charge">remove_session_and_charge</a>&lt;E: <b>copy</b> + drop + store&gt;(self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>, session_sequence_number: u64) {
    self.number_of_completed_sessions = self.number_of_completed_sessions + 1;
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_update_last_session_to_complete_in_current_epoch">update_last_session_to_complete_in_current_epoch</a>();
    <b>let</b> session = self.sessions.remove(session_sequence_number);
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSession">DWalletSession</a> {
        computation_fee_charged_ika,
        gas_fee_reimbursement_sui,
        consensus_validation_fee_charged_ika,
        dwallet_network_decryption_key_id,
        id,
        ..
    } = session;
    <b>let</b> dwallet_network_decryption_key = self.dwallet_network_decryption_keys.borrow_mut(dwallet_network_decryption_key_id);
    <b>let</b> _: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletEvent">DWalletEvent</a>&lt;E&gt; = self.session_start_events.remove(id.to_inner());
    object::delete(id);
    dwallet_network_decryption_key.computation_fee_charged_ika.join(computation_fee_charged_ika);
    self.consensus_validation_fee_charged_ika.join(consensus_validation_fee_charged_ika);
    self.gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_dkg_first_round"></a>

## Function `respond_dwallet_dkg_first_round`

Creates the output of the first DKG round.

This function transfers the output of the first DKG round
to the session initiator and ensures it is securely linked
to the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">DWalletCap</a></code> of the session.
This function is called by blockchain itself.
Validators call it, it's part of the blockchain logic.


<a name="@Effects_5"></a>

##### Effects

- Transfers the output of the first round to the initiator.
- Emits necessary metadata and links it to the associated session.


<a name="@Parameters_6"></a>

##### Parameters

- <code>initiator</code>: The address of the user who initiated the DKG session.
- <code>session_id</code>: The ID of the DKG session.
- <code>decentralized_public_output</code>: The public output data from the first round.
- <code>dwallet_cap_id</code>: The ID of the associated <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">DWalletCap</a></code>.
- <code>ctx</code>: The transaction context.


<a name="@Panics_7"></a>

##### Panics

- Panics with <code>ENotSystemAddress</code> if the sender is not the system address.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_dkg_first_round">respond_dwallet_dkg_first_round</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, first_round_output: vector&lt;u8&gt;, rejected: bool, session_sequence_number: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_dkg_first_round">respond_dwallet_dkg_first_round</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
    first_round_output: vector&lt;u8&gt;,
    rejected: bool,
    session_sequence_number: u64,
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_session_and_charge">remove_session_and_charge</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGFirstRoundRequestEvent">DWalletDKGFirstRoundRequestEvent</a>&gt;(session_sequence_number);
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut">get_dwallet_mut</a>(dwallet_id);
    dwallet.state = match (dwallet.state) {
        DWalletState::DKGRequested =&gt; {
            <b>if</b> (rejected) {
                event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletDKGFirstRoundEvent">RejectedDWalletDKGFirstRoundEvent</a> {
                    dwallet_id,
                });
                DWalletState::NetworkRejectedDKGRequest
            } <b>else</b> {
                event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletDKGFirstdRoundEvent">CompletedDWalletDKGFirstdRoundEvent</a> {
                    dwallet_id,
                    first_round_output,
                });
                DWalletState::AwaitingUserDKGVerificationInitiation {
                    first_round_output
                }
            }
        },
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongState">EWrongState</a>
    };
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_dkg_second_round"></a>

## Function `request_dwallet_dkg_second_round`

Initiates the second round of the Distributed Key Generation (DKG) process
and emits an event for validators to begin their participation in this round.

This function handles the creation of a new DKG session ID and emits an event containing
all the necessary parameters to continue the DKG process.

<a name="@Parameters_8"></a>

##### Parameters

- <code>dwallet_cap</code>: A reference to the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">DWalletCap</a></code>, representing the capability associated with the dWallet.
- <code>centralized_public_key_share_and_proof</code>: The user (centralized) public key share and proof.
- <code>first_round_output</code>: A reference to the <code>DWalletDKGFirstRoundOutput</code> structure containing the output of the first DKG round.
- <code>encrypted_centralized_secret_share_and_proof</code>: Encrypted centralized secret key share and its proof.
- <code>encryption_key</code>: The <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptionKey">EncryptionKey</a></code> object used for encrypting the secret key share.
- <code>centralized_public_output</code>: The public output of the centralized party in the DKG process.
- <code>decentralized_user_output_signature</code>: The signature for the public output of the centralized party in the DKG process.
- <code>signer_public_key</code>: The Ed25519 public key of the initiator,
used to verify the signature on the public output.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_dkg_second_round">request_dwallet_dkg_second_round</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_cap: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">dwallet_2pc_mpc_coordinator_inner::DWalletCap</a>, centralized_public_key_share_and_proof: vector&lt;u8&gt;, encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;, encryption_key_address: <b>address</b>, user_public_output: vector&lt;u8&gt;, signer_public_key: vector&lt;u8&gt;, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_dkg_second_round">request_dwallet_dkg_second_round</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_cap: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">DWalletCap</a>,
    centralized_public_key_share_and_proof: vector&lt;u8&gt;,
    encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;,
    encryption_key_address: <b>address</b>,
    user_public_output: vector&lt;u8&gt;,
    signer_public_key: vector&lt;u8&gt;,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> encryption_key = self.encryption_keys.borrow(encryption_key_address);
    <b>let</b> encryption_key_curve = encryption_key.curve;
    <b>let</b> encryption_key_id = encryption_key.id.to_inner();
    <b>let</b> encryption_key = encryption_key.encryption_key;
    <b>let</b> created_at_epoch: u64 = self.current_epoch;
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet">get_dwallet</a>(dwallet_cap.dwallet_id);
    <b>assert</b>!(!dwallet.is_imported_key_dwallet, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EImportedKeyDWallet">EImportedKeyDWallet</a>);
    <b>assert</b>!(encryption_key_curve == dwallet.curve, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMismatchCurve">EMismatchCurve</a>);
    <b>let</b> first_round_output = match (&dwallet.state) {
        DWalletState::AwaitingUserDKGVerificationInitiation {
            first_round_output,
        } =&gt; {
            *first_round_output
        },
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongState">EWrongState</a>
    };
    <b>let</b> dwallet_id = dwallet.id.to_inner();
    <b>let</b> dwallet_network_decryption_key_id = dwallet.dwallet_network_decryption_key_id;
    <b>let</b> encrypted_user_share = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a> {
        id: object::new(ctx),
        created_at_epoch,
        dwallet_id,
        encrypted_centralized_secret_share_and_proof,
        encryption_key_id,
        encryption_key_address,
        source_encrypted_user_secret_key_share_id: option::none(),
        state: EncryptedUserSecretKeyShareState::AwaitingNetworkVerification
    };
    <b>let</b> encrypted_user_secret_key_share_id = object::id(&encrypted_user_share);
    <b>let</b> pricing = self.pricing.dkg_second_round();
    <b>let</b> emit_event = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
        dwallet_network_decryption_key_id,
        pricing,
        payment_ika,
        payment_sui,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGSecondRoundRequestEvent">DWalletDKGSecondRoundRequestEvent</a> {
            encrypted_user_secret_key_share_id,
            dwallet_id: dwallet_cap.dwallet_id,
            first_round_output,
            centralized_public_key_share_and_proof,
            dwallet_cap_id: object::id(dwallet_cap),
            encrypted_centralized_secret_share_and_proof,
            encryption_key,
            encryption_key_id,
            encryption_key_address,
            user_public_output,
            signer_public_key,
            dwallet_network_decryption_key_id,
            curve: dwallet.curve,
        },
        ctx,
    );
    event::emit(emit_event);
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut">get_dwallet_mut</a>(dwallet_cap.dwallet_id);
    dwallet.encrypted_user_secret_key_shares.add(encrypted_user_secret_key_share_id, encrypted_user_share);
    dwallet.state = DWalletState::AwaitingNetworkDKGVerification;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_dkg_second_round"></a>

## Function `respond_dwallet_dkg_second_round`

Completes the second round of the Distributed Key Generation (DKG) process and
creates the [<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a></code>].

This function finalizes the DKG process by creating a <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a></code> object and associating it with the
cryptographic outputs of the second round. It also generates an encrypted user share and emits
events to record the results of the process.
This function is called by the blockchain.


<a name="@Parameters_9"></a>

##### Parameters

- **<code>session_id</code>**: A unique identifier for the current DKG session.
- **<code>decentralized_public_output</code>**: The public output of the second round of the DKG process,
representing the decentralized computation result.
- **<code>dwallet_cap_id</code>**: The unique identifier of the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">DWalletCap</a></code> associated with this session.
- **<code>dwallet_mpc_network_decryption_key_version</code>**: The version of the MPC network key for the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a></code>.
- **<code>encrypted_secret_share_and_proof</code>**: The encrypted user secret key share and associated cryptographic proof.
- **<code>encryption_key_id</code>**: The ID of the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptionKey">EncryptionKey</a></code> used for encrypting the secret key share.
- **<code>signed_public_share</code>**: The signed public share corresponding to the secret key share.
- **<code>encryptor_ed25519_pubkey</code>**: The Ed25519 public key of the entity that encrypted the secret key share.
- **<code>centralized_public_output</code>**: The centralized public output from the DKG process.


<a name="@Effects_10"></a>

##### Effects

- Creates a new <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a></code> object using the provided session ID, DKG outputs, and other metadata.
- Creates an encrypted user share and associates it with the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a></code>.
- Emits a <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletDKGSecondRoundEvent">CompletedDWalletDKGSecondRoundEvent</a></code> to record the completion of the second DKG round.
- Freezes the created <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a></code> object to make it immutable.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_dkg_second_round">respond_dwallet_dkg_second_round</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, public_output: vector&lt;u8&gt;, encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, session_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, rejected: bool, session_sequence_number: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_dkg_second_round">respond_dwallet_dkg_second_round</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
    public_output: vector&lt;u8&gt;,
    encrypted_user_secret_key_share_id: ID,
    session_id: ID,
    rejected: bool,
    session_sequence_number: u64,
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_session_and_charge">remove_session_and_charge</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGSecondRoundRequestEvent">DWalletDKGSecondRoundRequestEvent</a>&gt;(session_sequence_number);
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut">get_dwallet_mut</a>(dwallet_id);
    dwallet.state = match (&dwallet.state) {
        DWalletState::AwaitingNetworkDKGVerification =&gt; {
            <b>if</b> (rejected) {
                event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletDKGSecondRoundEvent">RejectedDWalletDKGSecondRoundEvent</a> {
                    dwallet_id,
                    public_output,
                });
                DWalletState::NetworkRejectedDKGVerification
            } <b>else</b> {
                <b>let</b> encrypted_user_share = dwallet.encrypted_user_secret_key_shares.borrow_mut(encrypted_user_secret_key_share_id);
                encrypted_user_share.state = EncryptedUserSecretKeyShareState::NetworkVerificationCompleted;
                event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletDKGSecondRoundEvent">CompletedDWalletDKGSecondRoundEvent</a> {
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
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongState">EWrongState</a>
    };
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_re_encrypt_user_share_for"></a>

## Function `request_re_encrypt_user_share_for`

Transfers an encrypted dWallet user secret key share from a source entity to destination entity.

This function emits an event with the encrypted user secret key share, along with its cryptographic proof,
to the blockchain. The chain verifies that the encrypted data matches the expected secret key share
associated with the dWallet before creating an [<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a></code>] object.


<a name="@Parameters_11"></a>

##### Parameters

- **<code>dwallet</code>**: A reference to the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a>&lt;Secp256K1&gt;</code> object to which the secret share is linked.
- **<code>destination_encryption_key</code>**: A reference to the encryption key used for encrypting the secret key share.
- **<code>encrypted_centralized_secret_share_and_proof</code>**: The encrypted secret key share, accompanied by a cryptographic proof.
- **<code>source_signed_centralized_public_output</code>**: The signed centralized public output corresponding to the secret share.
- **<code>source_ed25519_pubkey</code>**: The Ed25519 public key of the source (encryptor) used for verifying the signature.


<a name="@Effects_12"></a>

##### Effects

- Emits a <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedShareVerificationRequestEvent">EncryptedShareVerificationRequestEvent</a></code>,
which is captured by the blockchain to initiate the verification process.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_re_encrypt_user_share_for">request_re_encrypt_user_share_for</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, destination_encryption_key_address: <b>address</b>, encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;, source_encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_re_encrypt_user_share_for">request_re_encrypt_user_share_for</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
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
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut">get_dwallet_mut</a>(dwallet_id);
    <b>let</b> public_output = *dwallet.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_active_and_get_public_output">validate_active_and_get_public_output</a>();
    <b>let</b> dwallet_network_decryption_key_id = dwallet.dwallet_network_decryption_key_id;
    <b>let</b> curve = dwallet.curve;
    <b>assert</b>!(dwallet.encrypted_user_secret_key_shares.contains(source_encrypted_user_secret_key_share_id), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidSource">EInvalidSource</a>);
    <b>let</b> encrypted_user_share = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a> {
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
    <b>let</b> pricing = self.pricing.re_encrypt_user_share();
    event::emit(
        self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
            dwallet_network_decryption_key_id,
            pricing,
            payment_ika,
            payment_sui,
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedShareVerificationRequestEvent">EncryptedShareVerificationRequestEvent</a> {
                encrypted_centralized_secret_share_and_proof,
                public_output,
                dwallet_id,
                encryption_key: destination_encryption_key,
                encryption_key_id: destination_encryption_key_id,
                encrypted_user_secret_key_share_id,
                source_encrypted_user_secret_key_share_id,
                dwallet_network_decryption_key_id,
                curve,
            },
            ctx,
        )
    );
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_re_encrypt_user_share_for"></a>

## Function `respond_re_encrypt_user_share_for`

Creates an encrypted user secret key share after it has been verified by the blockchain.

This function is invoked by the blockchain to generate an [<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a></code>] object
once the associated encryption and cryptographic proofs have been verified.
It finalizes the process by storing the encrypted user share on-chain and emitting the relevant event.


<a name="@Parameters_13"></a>

##### Parameters

- <code>dwallet_id</code>: The unique identifier of the dWallet associated with the encrypted user share.
- <code>encrypted_centralized_secret_share_and_proof</code>: The encrypted centralized secret key share along with its cryptographic proof.
- <code>encryption_key_id</code>: The <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptionKey">EncryptionKey</a></code> Move object ID used to encrypt the secret key share.
- <code>centralized_user_output_signature</code>: The signed public share corresponding to the encrypted secret share.
- <code>signer_public_key</code>: The Ed25519 public key of the encryptor, used for signing.
- <code>initiator</code>: The address of the entity that performed the encryption operation of this secret key share.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_re_encrypt_user_share_for">respond_re_encrypt_user_share_for</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, rejected: bool, session_sequence_number: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_re_encrypt_user_share_for">respond_re_encrypt_user_share_for</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
    encrypted_user_secret_key_share_id: ID,
    rejected: bool,
    session_sequence_number: u64
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_session_and_charge">remove_session_and_charge</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedShareVerificationRequestEvent">EncryptedShareVerificationRequestEvent</a>&gt;(session_sequence_number);
    <b>let</b> (dwallet, _) = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output_mut">get_active_dwallet_and_public_output_mut</a>(dwallet_id);
    <b>let</b> encrypted_user_secret_key_share = dwallet.encrypted_user_secret_key_shares.borrow_mut(encrypted_user_secret_key_share_id);
    encrypted_user_secret_key_share.state = match(encrypted_user_secret_key_share.state) {
        EncryptedUserSecretKeyShareState::AwaitingNetworkVerification =&gt; {
            <b>if</b>(rejected) {
                event::emit(
                    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedEncryptedShareVerificationEvent">RejectedEncryptedShareVerificationEvent</a> {
                        encrypted_user_secret_key_share_id,
                        dwallet_id,
                    }
                );
                EncryptedUserSecretKeyShareState::NetworkVerificationRejected
            } <b>else</b> {
                event::emit(
                    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedEncryptedShareVerificationEvent">CompletedEncryptedShareVerificationEvent</a> {
                        encrypted_user_secret_key_share_id,
                        dwallet_id,
                    }
                );
                EncryptedUserSecretKeyShareState::NetworkVerificationCompleted
            }
        },
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongState">EWrongState</a>
    };
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_accept_encrypted_user_share"></a>

## Function `accept_encrypted_user_share`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_accept_encrypted_user_share">accept_encrypted_user_share</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, user_output_signature: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_accept_encrypted_user_share">accept_encrypted_user_share</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
    encrypted_user_secret_key_share_id: ID,
    user_output_signature: vector&lt;u8&gt;,
) {
    <b>let</b> (dwallet, public_output) = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output">get_active_dwallet_and_public_output</a>(dwallet_id);
    <b>let</b> encrypted_user_secret_key_share = dwallet.encrypted_user_secret_key_shares.borrow(encrypted_user_secret_key_share_id);
    <b>let</b> encryption_key = self.encryption_keys.borrow(encrypted_user_secret_key_share.encryption_key_address);
    <b>let</b> encryption_key_id = encrypted_user_secret_key_share.encryption_key_id;
    <b>let</b> encryption_key_address = encrypted_user_secret_key_share.encryption_key_address;
    <b>assert</b>!(
        ed25519_verify(&user_output_signature, &encryption_key.signer_public_key, &public_output),
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidEncryptionKeySignature">EInvalidEncryptionKeySignature</a>
    );
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut">get_dwallet_mut</a>(dwallet_id);
    <b>let</b> encrypted_user_secret_key_share = dwallet.encrypted_user_secret_key_shares.borrow_mut(encrypted_user_secret_key_share_id);
    encrypted_user_secret_key_share.state = match (encrypted_user_secret_key_share.state) {
        EncryptedUserSecretKeyShareState::NetworkVerificationCompleted =&gt; EncryptedUserSecretKeyShareState::KeyHolderSiged {
            user_output_signature
        },
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongState">EWrongState</a>
    };
    event::emit(
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_AcceptReEncryptedUserShareEvent">AcceptReEncryptedUserShareEvent</a> {
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_new_imported_key_dwallet"></a>

## Function `new_imported_key_dwallet`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_new_imported_key_dwallet">new_imported_key_dwallet</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, curve: u32, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyDWalletCap">dwallet_2pc_mpc_coordinator_inner::ImportedKeyDWalletCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_new_imported_key_dwallet">new_imported_key_dwallet</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_network_decryption_key_id: ID,
    curve: u32,
    ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyDWalletCap">ImportedKeyDWalletCap</a> {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_supported_curve">validate_supported_curve</a>(curve);
    <b>assert</b>!(self.dwallet_network_decryption_keys.contains(dwallet_network_decryption_key_id), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkDecryptionKeyNotExist">EDWalletNetworkDecryptionKeyNotExist</a>);
    <b>let</b> id = object::new(ctx);
    <b>let</b> dwallet_id = id.to_inner();
    <b>let</b> dwallet_cap = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyDWalletCap">ImportedKeyDWalletCap</a> {
        id: object::new(ctx),
        dwallet_id,
    };
    <b>let</b> dwallet_cap_id = object::id(&dwallet_cap);
    self.dwallets.add(dwallet_id, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a> {
        id,
        created_at_epoch: self.current_epoch,
        curve,
        public_user_secret_key_shares: option::none(),
        dwallet_cap_id,
        dwallet_network_decryption_key_id,
        is_imported_key_dwallet: <b>true</b>,
        encrypted_user_secret_key_shares: object_table::new(ctx),
        signs: object_table::new(ctx),
        state: DWalletState::AwaitingUserImportedKeyInitiation,
    });
    dwallet_cap
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_imported_key_dwallet_verification"></a>

## Function `request_imported_key_dwallet_verification`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_imported_key_dwallet_verification">request_imported_key_dwallet_verification</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_cap: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyDWalletCap">dwallet_2pc_mpc_coordinator_inner::ImportedKeyDWalletCap</a>, centralized_party_message: vector&lt;u8&gt;, encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;, encryption_key_address: <b>address</b>, user_public_output: vector&lt;u8&gt;, signer_public_key: vector&lt;u8&gt;, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_imported_key_dwallet_verification">request_imported_key_dwallet_verification</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_cap: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyDWalletCap">ImportedKeyDWalletCap</a>,
    centralized_party_message: vector&lt;u8&gt;,
    encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;,
    encryption_key_address: <b>address</b>,
    user_public_output: vector&lt;u8&gt;,
    signer_public_key: vector&lt;u8&gt;,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> encryption_key = self.encryption_keys.borrow(encryption_key_address);
    <b>let</b> encryption_key_id = encryption_key.id.to_inner();
    <b>let</b> encryption_key = encryption_key.encryption_key;
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut">get_dwallet_mut</a>(dwallet_cap.dwallet_id);
    <b>assert</b>!(dwallet.is_imported_key_dwallet, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ENotImportedKeyDWallet">ENotImportedKeyDWallet</a>);
    dwallet.state = match (&dwallet.state) {
        DWalletState::AwaitingUserImportedKeyInitiation =&gt; {
            DWalletState::AwaitingNetworkImportedKeyVerification
        },
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongState">EWrongState</a>
    };
    <b>let</b> dwallet_network_decryption_key_id = dwallet.dwallet_network_decryption_key_id;
    <b>let</b> curve = dwallet.curve;
    <b>let</b> pricing = self.pricing.imported_key_dwallet_verification();
    <b>let</b> emit_event = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
        dwallet_network_decryption_key_id,
        pricing,
        payment_ika,
        payment_sui,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletImportedKeyVerificationRequestEvent">DWalletImportedKeyVerificationRequestEvent</a> {
            dwallet_id: dwallet_cap.dwallet_id,
            centralized_party_message,
            dwallet_cap_id: object::id(dwallet_cap),
            encrypted_centralized_secret_share_and_proof,
            encryption_key,
            encryption_key_id,
            encryption_key_address,
            user_public_output,
            signer_public_key,
            dwallet_network_decryption_key_id,
            curve,
        },
        ctx,
    );
    event::emit(emit_event);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_imported_key_dwallet_verification"></a>

## Function `respond_imported_key_dwallet_verification`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_imported_key_dwallet_verification">respond_imported_key_dwallet_verification</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, public_output: vector&lt;u8&gt;, encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;, encryption_key_address: <b>address</b>, session_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, rejected: bool, session_sequence_number: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_imported_key_dwallet_verification">respond_imported_key_dwallet_verification</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
    public_output: vector&lt;u8&gt;,
    encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;,
    encryption_key_address: <b>address</b>,
    session_id: ID,
    rejected: bool,
    session_sequence_number: u64,
    ctx: &<b>mut</b> TxContext
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_session_and_charge">remove_session_and_charge</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletImportedKeyVerificationRequestEvent">DWalletImportedKeyVerificationRequestEvent</a>&gt;(session_sequence_number);
    <b>let</b> encryption_key = self.encryption_keys.borrow(encryption_key_address);
    <b>let</b> encryption_key_id = encryption_key.id.to_inner();
    <b>let</b> created_at_epoch = self.current_epoch;
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut">get_dwallet_mut</a>(dwallet_id);
    dwallet.state = match (&dwallet.state) {
        DWalletState::AwaitingNetworkImportedKeyVerification =&gt; {
            <b>if</b> (rejected) {
                event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletImportedKeyVerificationEvent">RejectedDWalletImportedKeyVerificationEvent</a> {
                    dwallet_id,
                });
                DWalletState::NetworkRejectedImportedKeyVerification
            } <b>else</b> {
                <b>let</b> encrypted_user_share = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a> {
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
                event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletImportedKeyVerificationEvent">CompletedDWalletImportedKeyVerificationEvent</a> {
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
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongState">EWrongState</a>
    };
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_make_dwallet_user_secret_key_shares_public"></a>

## Function `request_make_dwallet_user_secret_key_shares_public`

Requests to make the user secret key shares of a dWallet public.
*IMPORTANT*: If you make the dWallet user secret key shares public, you remove
the zero trust security of the dWallet and you can't revert it.

This function emits a <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MakeDWalletUserSecretKeySharesPublicRequestEvent">MakeDWalletUserSecretKeySharesPublicRequestEvent</a></code> event to initiate the
process of making the user secret key shares of a dWallet public. It charges the initiator for
the operation and creates a new event to record the request.


<a name="@Parameters_14"></a>

##### Parameters

- <code>dwallet_id</code>: The ID of the dWallet to make the user secret key shares public.
- <code>public_user_secret_key_shares</code>: The public user secret key shares to be made public.
- <code>payment_ika</code>: The IKA payment for the operation.
- <code>payment_sui</code>: The SUI payment for the operation.
- <code>ctx</code>: The transaction context.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_make_dwallet_user_secret_key_shares_public">request_make_dwallet_user_secret_key_shares_public</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, public_user_secret_key_shares: vector&lt;u8&gt;, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_make_dwallet_user_secret_key_shares_public">request_make_dwallet_user_secret_key_shares_public</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
    public_user_secret_key_shares: vector&lt;u8&gt;,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> (dwallet, public_output) = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output">get_active_dwallet_and_public_output</a>(dwallet_id);
    <b>let</b> dwallet_network_decryption_key_id = dwallet.dwallet_network_decryption_key_id;
    <b>let</b> curve = dwallet.curve;
    <b>assert</b>!(dwallet.public_user_secret_key_shares.is_none(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletUserSecretKeySharesAlreadyPublic">EDWalletUserSecretKeySharesAlreadyPublic</a>);
    <b>let</b> pricing = self.pricing.make_dwallet_user_secret_key_shares_public();
    event::emit(
        self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
            dwallet_network_decryption_key_id,
            pricing,
            payment_ika,
            payment_sui,
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MakeDWalletUserSecretKeySharesPublicRequestEvent">MakeDWalletUserSecretKeySharesPublicRequestEvent</a> {
                public_user_secret_key_shares,
                public_output,
                curve,
                dwallet_id,
                dwallet_network_decryption_key_id,
            },
            ctx,
        )
    );
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_make_dwallet_user_secret_key_shares_public"></a>

## Function `respond_make_dwallet_user_secret_key_shares_public`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_make_dwallet_user_secret_key_shares_public">respond_make_dwallet_user_secret_key_shares_public</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, public_user_secret_key_shares: vector&lt;u8&gt;, rejected: bool, session_sequence_number: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_make_dwallet_user_secret_key_shares_public">respond_make_dwallet_user_secret_key_shares_public</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
    public_user_secret_key_shares: vector&lt;u8&gt;,
    rejected: bool,
    session_sequence_number: u64,
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_session_and_charge">remove_session_and_charge</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MakeDWalletUserSecretKeySharesPublicRequestEvent">MakeDWalletUserSecretKeySharesPublicRequestEvent</a>&gt;(session_sequence_number);
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut">get_dwallet_mut</a>(dwallet_id);
    <b>if</b> (rejected) {
        event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedMakeDWalletUserSecretKeySharesPublicEvent">RejectedMakeDWalletUserSecretKeySharesPublicEvent</a> {
            dwallet_id,
        });
    } <b>else</b> {
        dwallet.public_user_secret_key_shares.fill(public_user_secret_key_shares);
        event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedMakeDWalletUserSecretKeySharesPublicEvent">CompletedMakeDWalletUserSecretKeySharesPublicEvent</a> {
            dwallet_id,
        });
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_presign"></a>

## Function `request_presign`

Starts a batched presign session.

This function emits a <code>RequestedBatchedPresignEvent</code> for the entire batch and a
<code>RequestedPresignFirstRoundEvent</code> for each presign in the batch. These events signal
validators to begin processing the first round of the presign process for each session.
- A unique <code>batch_session_id</code> is generated for the batch.
- A loop creates and emits a <code>RequestedPresignFirstRoundEvent</code> for each session in the batch.
- Each session is linked to the parent batch via <code>batch_session_id</code>.


<a name="@Effects_15"></a>

##### Effects

- Associates the batched presign session with the specified dWallet.
- Emits a <code>RequestedBatchedPresignEvent</code> containing the batch session details.
- Emits a <code>RequestedPresignFirstRoundEvent</code> for each presign in the batch, with relevant details.


<a name="@Parameters_16"></a>

##### Parameters

- <code>dwallet_id</code>: The dWallet's ID to resquest presign.
- <code>ctx</code>: The mutable transaction context, used to generate unique object IDs and retrieve the initiator.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_presign">request_presign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, signature_algorithm: u32, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">dwallet_2pc_mpc_coordinator_inner::UnverifiedPresignCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_presign">request_presign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
    signature_algorithm: u32,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">UnverifiedPresignCap</a> {
    <b>let</b> created_at_epoch = self.current_epoch;
    <b>assert</b>!(!self.signature_algorithms_allowed_global_presign.contains(&signature_algorithm), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidSignatureAlgorithm">EInvalidSignatureAlgorithm</a>);
    <b>let</b> (dwallet, public_output) = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output">get_active_dwallet_and_public_output</a>(dwallet_id);
    <b>let</b> curve = dwallet.curve;
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_supported_curve_and_signature_algorithm">validate_supported_curve_and_signature_algorithm</a>(curve, signature_algorithm);
    <b>let</b> dwallet_network_decryption_key_id = dwallet.dwallet_network_decryption_key_id;
    <b>let</b> id = object::new(ctx);
    <b>let</b> presign_id = id.to_inner();
    <b>let</b> cap = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">UnverifiedPresignCap</a> {
        id: object::new(ctx),
        dwallet_id: option::some(dwallet_id),
        presign_id,
    };
    self.presigns.add(presign_id, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_Presign">Presign</a> {
        id,
        created_at_epoch,
        signature_algorithm,
        curve,
        dwallet_id: option::some(dwallet_id),
        cap_id: object::id(&cap),
        state: PresignState::Requested,
    });
    <b>let</b> pricing = self.pricing.presign();
    event::emit(
        self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
            dwallet_network_decryption_key_id,
            pricing,
            payment_ika,
            payment_sui,
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignRequestEvent">PresignRequestEvent</a> {
                dwallet_id: option::some(dwallet_id),
                presign_id,
                dwallet_public_output: option::some(public_output),
                dwallet_network_decryption_key_id,
                curve,
                signature_algorithm,
            },
            ctx,
        )
    );
    cap
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_global_presign"></a>

## Function `request_global_presign`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_global_presign">request_global_presign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_network_decryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, curve: u32, signature_algorithm: u32, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">dwallet_2pc_mpc_coordinator_inner::UnverifiedPresignCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_global_presign">request_global_presign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_network_decryption_key_id: ID,
    curve: u32,
    signature_algorithm: u32,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">UnverifiedPresignCap</a> {
    <b>let</b> created_at_epoch = self.current_epoch;
    <b>assert</b>!(self.signature_algorithms_allowed_global_presign.contains(&signature_algorithm), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidSignatureAlgorithm">EInvalidSignatureAlgorithm</a>);
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_supported_curve_and_signature_algorithm">validate_supported_curve_and_signature_algorithm</a>(curve, signature_algorithm);
    <b>let</b> id = object::new(ctx);
    <b>let</b> presign_id = id.to_inner();
    <b>let</b> cap = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">UnverifiedPresignCap</a> {
        id: object::new(ctx),
        dwallet_id: option::none(),
        presign_id,
    };
    self.presigns.add(presign_id, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_Presign">Presign</a> {
        id,
        created_at_epoch,
        signature_algorithm,
        curve,
        dwallet_id: option::none(),
        cap_id: object::id(&cap),
        state: PresignState::Requested,
    });
    <b>let</b> pricing = self.pricing.presign();
    event::emit(
        self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
            dwallet_network_decryption_key_id,
            pricing,
            payment_ika,
            payment_sui,
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignRequestEvent">PresignRequestEvent</a> {
                dwallet_id: option::none(),
                presign_id,
                dwallet_public_output: option::none(),
                dwallet_network_decryption_key_id,
                curve,
                signature_algorithm,
            },
            ctx,
        )
    );
    cap
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_presign"></a>

## Function `respond_presign`

Completes the presign session by creating the output of the
second presign round and transferring it to the session initiator.

This function is called by validators as part of the blockchain logic.
It creates a <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_Presign">Presign</a></code> object representing the second presign round output,
emits a <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedPresignEvent">CompletedPresignEvent</a></code>, and transfers the result to the initiating user.


<a name="@Parameters_17"></a>

##### Parameters

- <code>initiator</code>: The address of the user who initiated the presign session.
- <code>session_id</code>: The ID of the presign session.
- <code>output</code>: The presign result data.
- <code>dwallet_cap_id</code>: The ID of the associated <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">DWalletCap</a></code>.
- <code>dwallet_id</code>: The ID of the associated <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a></code>.
- <code>ctx</code>: The transaction context.


<a name="@Emits_18"></a>

##### Emits

- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedPresignEvent">CompletedPresignEvent</a></code>: Includes the initiator, dWallet ID, and presign ID.


<a name="@Panics_19"></a>

##### Panics

- Panics with <code>ENotSystemAddress</code> if the sender of the transaction is not the system address.


<a name="@Effects_20"></a>

##### Effects

- Creates a <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_Presign">Presign</a></code> object and transfers it to the session initiator.
- Emits a <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedPresignEvent">CompletedPresignEvent</a></code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_presign">respond_presign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;, presign_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, session_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, presign: vector&lt;u8&gt;, rejected: bool, session_sequence_number: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_presign">respond_presign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: Option&lt;ID&gt;,
    presign_id: ID,
    session_id: ID,
    presign: vector&lt;u8&gt;,
    rejected: bool,
    session_sequence_number: u64
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_session_and_charge">remove_session_and_charge</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignRequestEvent">PresignRequestEvent</a>&gt;(session_sequence_number);
    <b>let</b> presign_obj = self.presigns.borrow_mut(presign_id);
    presign_obj.state = match(presign_obj.state) {
        PresignState::Requested =&gt; {
            <b>if</b>(rejected) {
                event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedPresignEvent">RejectedPresignEvent</a> {
                    dwallet_id,
                    session_id,
                    presign_id
                });
                PresignState::NetworkRejected
            } <b>else</b> {
                event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedPresignEvent">CompletedPresignEvent</a> {
                    dwallet_id,
                    session_id,
                    presign_id,
                    presign
                });
                PresignState::Completed {
                    presign
                }
            }
        },
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongState">EWrongState</a>
    };
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_is_presign_valid"></a>

## Function `is_presign_valid`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_is_presign_valid">is_presign_valid</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, cap: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">dwallet_2pc_mpc_coordinator_inner::UnverifiedPresignCap</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_is_presign_valid">is_presign_valid</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    cap: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">UnverifiedPresignCap</a>,
): bool {
    <b>let</b> presign = self.presigns.borrow(cap.presign_id);
    match(&presign.state) {
        PresignState::Completed { .. } =&gt; {
            cap.id.to_inner() == presign.cap_id
        },
        _ =&gt; <b>false</b>
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_verify_presign_cap"></a>

## Function `verify_presign_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_verify_presign_cap">verify_presign_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">dwallet_2pc_mpc_coordinator_inner::UnverifiedPresignCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">dwallet_2pc_mpc_coordinator_inner::VerifiedPresignCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_verify_presign_cap">verify_presign_cap</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    cap: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">UnverifiedPresignCap</a>,
    ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">VerifiedPresignCap</a> {
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">UnverifiedPresignCap</a> {
        id,
        dwallet_id,
        presign_id
    } = cap;
    <b>let</b> cap_id = id.to_inner();
    id.delete();
    <b>let</b> presign = self.presigns.borrow_mut(presign_id);
    <b>assert</b>!(presign.cap_id == cap_id, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EIncorrectCap">EIncorrectCap</a>);
        match(&presign.state) {
        PresignState::Completed { .. } =&gt; {},
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EUnverifiedCap">EUnverifiedCap</a>
    };
    <b>let</b> cap = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">VerifiedPresignCap</a> {
        id: object::new(ctx),
        dwallet_id,
        presign_id,
    };
    presign.cap_id = cap.id.to_inner();
    cap
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_and_initiate_sign"></a>

## Function `validate_and_initiate_sign`

This function is a shared logic for both the normal and future sign flows.
It checks the presign is valid and removes it, thus assuring it is never used twice.
Finally it emits the sign event.


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_and_initiate_sign">validate_and_initiate_sign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, pricing: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_PricingPerOperation">dwallet_pricing::PricingPerOperation</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, signature_algorithm: u32, hash_scheme: u32, message: vector&lt;u8&gt;, presign_cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">dwallet_2pc_mpc_coordinator_inner::VerifiedPresignCap</a>, message_centralized_signature: vector&lt;u8&gt;, is_future_sign: bool, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_and_initiate_sign">validate_and_initiate_sign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    pricing: PricingPerOperation,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    dwallet_id: ID,
    signature_algorithm: u32,
    hash_scheme: u32,
    message: vector&lt;u8&gt;,
    presign_cap: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">VerifiedPresignCap</a>,
    message_centralized_signature: vector&lt;u8&gt;,
    is_future_sign: bool,
    ctx: &<b>mut</b> TxContext
): bool {
    <b>let</b> created_at_epoch = self.current_epoch;
    <b>assert</b>!(self.presigns.contains(presign_cap.presign_id), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EPresignNotExist">EPresignNotExist</a>);
    <b>let</b> presign = self.presigns.remove(presign_cap.presign_id);
    <b>let</b> (dwallet, dwallet_public_output) = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output_mut">get_active_dwallet_and_public_output_mut</a>(dwallet_id);
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">VerifiedPresignCap</a> {
        id,
        dwallet_id: presign_cap_dwallet_id,
        presign_id: presign_cap_presign_id,
    } = presign_cap;
    <b>let</b> presign_cap_id = id.to_inner();
    id.delete();
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_Presign">Presign</a> {
        id,
        created_at_epoch: _,
        dwallet_id: presign_dwallet_id,
        cap_id,
        state,
        curve,
        signature_algorithm: presign_signature_algorithm,
    } = presign;
    <b>let</b> presign = match(state) {
        PresignState::Completed { presign } =&gt; {
            presign
        },
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidPresign">EInvalidPresign</a>
    };
    <b>let</b> presign_id = id.to_inner();
    id.delete();
    <b>assert</b>!(presign_dwallet_id.is_none() || presign_dwallet_id.is_some_and!(|id| id == dwallet_id), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMessageApprovalMismatch">EMessageApprovalMismatch</a>);
    <b>assert</b>!(presign_signature_algorithm == signature_algorithm, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMessageApprovalMismatch">EMessageApprovalMismatch</a>);
    <b>assert</b>!(presign_cap_id == cap_id, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EPresignNotExist">EPresignNotExist</a>);
    <b>assert</b>!(presign_id == presign_cap_presign_id, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EPresignNotExist">EPresignNotExist</a>);
    <b>assert</b>!(presign_cap_dwallet_id == presign_dwallet_id, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EPresignNotExist">EPresignNotExist</a>);
    <b>assert</b>!(dwallet.curve == curve, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletMismatch">EDWalletMismatch</a>);
    <b>let</b> id = object::new(ctx);
    <b>let</b> sign_id = id.to_inner();
    <b>let</b> dwallet_network_decryption_key_id = dwallet.dwallet_network_decryption_key_id;
    <b>let</b> emit_event = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
        dwallet_network_decryption_key_id,
        pricing,
        payment_ika,
        payment_sui,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignRequestEvent">SignRequestEvent</a> {
            sign_id,
            dwallet_id,
            dwallet_public_output,
            curve,
            signature_algorithm,
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
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut">get_dwallet_mut</a>(dwallet_id);
    dwallet.signs.add(sign_id, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_Sign">Sign</a> {
        id,
        created_at_epoch,
        dwallet_id,
        session_id,
        state: SignState::Requested,
    });
    <b>let</b> is_imported_key_dwallet = dwallet.is_imported_key_dwallet;
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_supported_curve_and_signature_algorithm_and_hash_scheme">validate_supported_curve_and_signature_algorithm_and_hash_scheme</a>(curve, signature_algorithm, hash_scheme);
    event::emit(emit_event);
    is_imported_key_dwallet
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_sign"></a>

## Function `request_sign`

Initiates the signing process for a given dWallet of type T.

This function emits a <code>RequestedSignEvent</code> and a <code>RequestedBatchedSignEvent</code>,
providing all necessary metadata to ensure the integrity of the signing process.
It validates the linkage between the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a></code>, <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">DWalletCap</a></code>, and <code>SignatureAlgorithmData</code> objects.


<a name="@Effects_21"></a>

### Effects

- Ensures a valid linkage between <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a></code>, <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">DWalletCap</a></code>, and <code>SignatureAlgorithmData</code>.
- Validates that <code>signature_algorithm_data</code> and <code>message_approvals</code> have the same length.
- Emits the following events:
- <code>RequestedBatchedSignEvent</code>: Contains the session details and the list of hashed messages.
- <code>RequestedSignEvent</code>: Includes details for each message signing process.


<a name="@Aborts_22"></a>

### Aborts

- **<code>EExtraDataAndMessagesLenMismatch</code>**: If the number of <code>hashed_messages</code> does not
match the number of <code>signature_algorithm_data</code>.
- **<code>EMissingApprovalOrWrongApprovalOrder</code>**: If the approvals are missing or provided in the incorrect order.


<a name="@Parameters_23"></a>

### Parameters

- <code>message_approvals</code>: A vector of <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">MessageApproval</a></code> objects representing
approvals for the messages, which are destroyed at the end of the transaction.
- <code>dwallet</code>: A reference to the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a></code> object being used for signing.
- <code>signature_algorithm_data</code>: A vector of <code>SignatureAlgorithmData</code> objects containing intermediate signing outputs,
which are unpacked and then destroyed at the end of the transaction.


<a name="@Type_Parameters_24"></a>

### Type Parameters

- <code>T</code>: The elliptic curve type used for the dWallet.
D: The type of data that can be stored with the object,
specific to each Digital Signature Algorithm.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_sign">request_sign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, message_approval: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">dwallet_2pc_mpc_coordinator_inner::MessageApproval</a>, presign_cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">dwallet_2pc_mpc_coordinator_inner::VerifiedPresignCap</a>, message_centralized_signature: vector&lt;u8&gt;, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_sign">request_sign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    message_approval: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">MessageApproval</a>,
    presign_cap: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">VerifiedPresignCap</a>,
    message_centralized_signature: vector&lt;u8&gt;,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">MessageApproval</a> {
        dwallet_id,
        signature_algorithm,
        hash_scheme,
        message
    } = message_approval;
    <b>let</b> pricing = self.pricing.sign();
    <b>let</b> is_imported_key_dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_and_initiate_sign">validate_and_initiate_sign</a>(
        pricing,
        payment_ika,
        payment_sui,
        dwallet_id,
        signature_algorithm,
        hash_scheme,
        message,
        presign_cap,
        message_centralized_signature,
        <b>false</b>,
        ctx
    );
    <b>assert</b>!(!is_imported_key_dwallet, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EImportedKeyDWallet">EImportedKeyDWallet</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_imported_key_sign"></a>

## Function `request_imported_key_sign`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_imported_key_sign">request_imported_key_sign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, message_approval: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyMessageApproval">dwallet_2pc_mpc_coordinator_inner::ImportedKeyMessageApproval</a>, presign_cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">dwallet_2pc_mpc_coordinator_inner::VerifiedPresignCap</a>, message_centralized_signature: vector&lt;u8&gt;, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_imported_key_sign">request_imported_key_sign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    message_approval: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyMessageApproval">ImportedKeyMessageApproval</a>,
    presign_cap: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">VerifiedPresignCap</a>,
    message_centralized_signature: vector&lt;u8&gt;,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyMessageApproval">ImportedKeyMessageApproval</a> {
        dwallet_id,
        signature_algorithm,
        hash_scheme,
        message
    } = message_approval;
    <b>let</b> pricing = self.pricing.sign();
    <b>let</b> is_imported_key_dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_and_initiate_sign">validate_and_initiate_sign</a>(
        pricing,
        payment_ika,
        payment_sui,
        dwallet_id,
        signature_algorithm,
        hash_scheme,
        message,
        presign_cap,
        message_centralized_signature,
        <b>false</b>,
        ctx
    );
    <b>assert</b>!(is_imported_key_dwallet, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ENotImportedKeyDWallet">ENotImportedKeyDWallet</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_future_sign"></a>

## Function `request_future_sign`

A function to publish messages signed by the user on chain with on-chain verification,
without launching the chain's sign flow immediately.

See the docs of [<code>PartialCentralizedSignedMessages</code>] for
more details on when this may be used.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_future_sign">request_future_sign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, presign_cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">dwallet_2pc_mpc_coordinator_inner::VerifiedPresignCap</a>, message: vector&lt;u8&gt;, hash_scheme: u32, message_centralized_signature: vector&lt;u8&gt;, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPartialUserSignatureCap">dwallet_2pc_mpc_coordinator_inner::UnverifiedPartialUserSignatureCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_future_sign">request_future_sign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
    presign_cap: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">VerifiedPresignCap</a>,
    message: vector&lt;u8&gt;,
    hash_scheme: u32,
    message_centralized_signature: vector&lt;u8&gt;,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPartialUserSignatureCap">UnverifiedPartialUserSignatureCap</a> {
    <b>let</b> pricing = self.pricing.future_sign();
    <b>assert</b>!(!self.paused_hash_schemes.contains(&hash_scheme), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EHashSchemePaused">EHashSchemePaused</a>);
    <b>assert</b>!(presign_cap.dwallet_id.is_none() || presign_cap.dwallet_id.is_some_and!(|id| id == dwallet_id), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMessageApprovalMismatch">EMessageApprovalMismatch</a>);
    <b>let</b> (dwallet, dwallet_public_output) = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output_mut">get_active_dwallet_and_public_output_mut</a>(dwallet_id);
    <b>let</b> dwallet_network_decryption_key_id = dwallet.dwallet_network_decryption_key_id;
    <b>let</b> curve = dwallet.curve;
    <b>assert</b>!(self.presigns.contains(presign_cap.presign_id), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EPresignNotExist">EPresignNotExist</a>);
    <b>let</b> presign_obj = self.presigns.borrow(presign_cap.presign_id);
    <b>assert</b>!(presign_obj.curve == curve, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletMismatch">EDWalletMismatch</a>);
    <b>let</b> presign = match(presign_obj.state) {
        PresignState::Completed { presign } =&gt; {
            presign
        },
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidPresign">EInvalidPresign</a>
    };
    <b>let</b> id = object::new(ctx);
    <b>let</b> partial_centralized_signed_message_id = id.to_inner();
    <b>let</b> cap = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPartialUserSignatureCap">UnverifiedPartialUserSignatureCap</a> {
        id: object::new(ctx),
        partial_centralized_signed_message_id,
    };
    <b>let</b> signature_algorithm = presign_obj.signature_algorithm;
    <b>let</b> emit_event = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
        dwallet_network_decryption_key_id,
        pricing,
        payment_ika,
        payment_sui,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_FutureSignRequestEvent">FutureSignRequestEvent</a> {
                dwallet_id,
                partial_centralized_signed_message_id,
                message,
                presign: presign,
                dwallet_public_output,
                curve,
                signature_algorithm,
                hash_scheme,
                message_centralized_signature,
                dwallet_network_decryption_key_id,
        },
        ctx,
    );
    self.partial_centralized_signed_messages.add(partial_centralized_signed_message_id, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignature">PartialUserSignature</a> {
        id: id,
        created_at_epoch: self.current_epoch,
        presign_cap,
        dwallet_id,
        cap_id: object::id(&cap),
        hash_scheme,
        message,
        message_centralized_signature,
        state: PartialUserSignatureState::AwaitingNetworkVerification,
        curve,
        signature_algorithm,
    });
    event::emit(emit_event);
    cap
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_future_sign"></a>

## Function `respond_future_sign`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_future_sign">respond_future_sign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, session_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, partial_centralized_signed_message_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, rejected: bool, session_sequence_number: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_future_sign">respond_future_sign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    session_id: ID,
    dwallet_id: ID,
    partial_centralized_signed_message_id: ID,
    rejected: bool,
    session_sequence_number: u64
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_session_and_charge">remove_session_and_charge</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_FutureSignRequestEvent">FutureSignRequestEvent</a>&gt;(session_sequence_number);
    <b>let</b> partial_centralized_signed_message = self.partial_centralized_signed_messages.borrow_mut(partial_centralized_signed_message_id);
    <b>assert</b>!(partial_centralized_signed_message.presign_cap.dwallet_id.is_none() || partial_centralized_signed_message.presign_cap.dwallet_id.is_some_and!(|id| id == dwallet_id), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletMismatch">EDWalletMismatch</a>);
    partial_centralized_signed_message.state = match(partial_centralized_signed_message.state) {
        PartialUserSignatureState::AwaitingNetworkVerification =&gt; {
            <b>if</b>(rejected) {
                event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedFutureSignEvent">RejectedFutureSignEvent</a> {
                    session_id,
                    dwallet_id,
                    partial_centralized_signed_message_id
                });
                PartialUserSignatureState::NetworkVerificationRejected
            } <b>else</b> {
                event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedFutureSignEvent">CompletedFutureSignEvent</a> {
                    session_id,
                    dwallet_id,
                    partial_centralized_signed_message_id
                });
                PartialUserSignatureState::NetworkVerificationCompleted
            }
        },
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongState">EWrongState</a>
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_is_partial_user_signature_valid"></a>

## Function `is_partial_user_signature_valid`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_is_partial_user_signature_valid">is_partial_user_signature_valid</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, cap: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPartialUserSignatureCap">dwallet_2pc_mpc_coordinator_inner::UnverifiedPartialUserSignatureCap</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_is_partial_user_signature_valid">is_partial_user_signature_valid</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    cap: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPartialUserSignatureCap">UnverifiedPartialUserSignatureCap</a>,
): bool {
    <b>let</b> partial_centralized_signed_message = self.partial_centralized_signed_messages.borrow(cap.partial_centralized_signed_message_id);
    partial_centralized_signed_message.cap_id == cap.id.to_inner() && partial_centralized_signed_message.state == PartialUserSignatureState::NetworkVerificationCompleted
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_verify_partial_user_signature_cap"></a>

## Function `verify_partial_user_signature_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_verify_partial_user_signature_cap">verify_partial_user_signature_cap</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPartialUserSignatureCap">dwallet_2pc_mpc_coordinator_inner::UnverifiedPartialUserSignatureCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap">dwallet_2pc_mpc_coordinator_inner::VerifiedPartialUserSignatureCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_verify_partial_user_signature_cap">verify_partial_user_signature_cap</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    cap: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPartialUserSignatureCap">UnverifiedPartialUserSignatureCap</a>,
    ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap">VerifiedPartialUserSignatureCap</a> {
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPartialUserSignatureCap">UnverifiedPartialUserSignatureCap</a> {
        id,
        partial_centralized_signed_message_id
    } = cap;
    <b>let</b> cap_id = id.to_inner();
    id.delete();
    <b>let</b> partial_centralized_signed_message = self.partial_centralized_signed_messages.borrow_mut(partial_centralized_signed_message_id);
    <b>assert</b>!(partial_centralized_signed_message.cap_id == cap_id, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EIncorrectCap">EIncorrectCap</a>);
    <b>assert</b>!(partial_centralized_signed_message.state == PartialUserSignatureState::NetworkVerificationCompleted, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EUnverifiedCap">EUnverifiedCap</a>);
    <b>let</b> cap = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap">VerifiedPartialUserSignatureCap</a> {
        id: object::new(ctx),
        partial_centralized_signed_message_id,
    };
    partial_centralized_signed_message.cap_id = cap.id.to_inner();
    cap
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_sign_with_partial_user_signature"></a>

## Function `request_sign_with_partial_user_signature`

Initiates a signing flow using a previously published [<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignature">PartialUserSignature</a></code>].

This function takes a partial signature object, validates approvals for each message,
and emits the necessary signing events.


<a name="@Type_Parameters_25"></a>

#### Type Parameters

- <code>D</code>: Represents additional data fields specific for each implementation.


<a name="@Parameters_26"></a>

#### Parameters

- <code>partial_signature</code>: A previously published <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignature">PartialUserSignature</a>&lt;D&gt;</code> object
containing messages that require approval.
- <code>message_approvals</code>: A list of approvals corresponding to the messages in <code>partial_signature</code>.
- <code>ctx</code>: The transaction context.

<a name="@Notes_27"></a>

#### Notes

- See [<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignature">PartialUserSignature</a></code>] documentation for more details on usage scenarios.
- The function ensures that messages and approvals have a one-to-one correspondence before proceeding.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_sign_with_partial_user_signature">request_sign_with_partial_user_signature</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, partial_user_signature_cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap">dwallet_2pc_mpc_coordinator_inner::VerifiedPartialUserSignatureCap</a>, message_approval: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">dwallet_2pc_mpc_coordinator_inner::MessageApproval</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_sign_with_partial_user_signature">request_sign_with_partial_user_signature</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    partial_user_signature_cap: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap">VerifiedPartialUserSignatureCap</a>,
    message_approval: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">MessageApproval</a>,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> pricing = self.pricing.sign_with_partial_user_signature();
    // Ensure that each partial user signature <b>has</b> a corresponding message approval; otherwise, <b>abort</b>.
    <b>let</b> is_match = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_match_partial_user_signature_with_message_approval">match_partial_user_signature_with_message_approval</a>(&partial_user_signature_cap, &message_approval);
    <b>assert</b>!(is_match, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMessageApprovalMismatch">EMessageApprovalMismatch</a>);
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap">VerifiedPartialUserSignatureCap</a> {
        id,
        partial_centralized_signed_message_id,
    } = partial_user_signature_cap;
    <b>let</b> verified_cap_id = id.to_inner();
    id.delete();
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignature">PartialUserSignature</a> {
        id,
        created_at_epoch: _,
        presign_cap,
        dwallet_id: _,
        cap_id,
        curve: _,
        signature_algorithm: _,
        hash_scheme: _,
        message: _,
        message_centralized_signature,
        state
    } = self.partial_centralized_signed_messages.remove(partial_centralized_signed_message_id);
    id.delete();
    <b>assert</b>!(cap_id == verified_cap_id && state == PartialUserSignatureState::NetworkVerificationCompleted, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EIncorrectCap">EIncorrectCap</a>);
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">MessageApproval</a> {
        dwallet_id,
        signature_algorithm,
        hash_scheme,
        message
    } = message_approval;
    // Emit signing events to finalize the signing process.
    <b>let</b> is_imported_key_dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_and_initiate_sign">validate_and_initiate_sign</a>(
        pricing,
        payment_ika,
        payment_sui,
        dwallet_id,
        signature_algorithm,
        hash_scheme,
        message,
        presign_cap,
        message_centralized_signature,
        <b>true</b>,
        ctx
    );
    <b>assert</b>!(!is_imported_key_dwallet, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EImportedKeyDWallet">EImportedKeyDWallet</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_imported_key_sign_with_partial_user_signature"></a>

## Function `request_imported_key_sign_with_partial_user_signature`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_imported_key_sign_with_partial_user_signature">request_imported_key_sign_with_partial_user_signature</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, partial_user_signature_cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap">dwallet_2pc_mpc_coordinator_inner::VerifiedPartialUserSignatureCap</a>, message_approval: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyMessageApproval">dwallet_2pc_mpc_coordinator_inner::ImportedKeyMessageApproval</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_imported_key_sign_with_partial_user_signature">request_imported_key_sign_with_partial_user_signature</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    partial_user_signature_cap: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap">VerifiedPartialUserSignatureCap</a>,
    message_approval: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyMessageApproval">ImportedKeyMessageApproval</a>,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> pricing = self.pricing.sign_with_partial_user_signature();
    // Ensure that each partial user signature <b>has</b> a corresponding imported key message approval; otherwise, <b>abort</b>.
    <b>let</b> is_match = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_match_partial_user_signature_with_imported_key_message_approval">match_partial_user_signature_with_imported_key_message_approval</a>(&partial_user_signature_cap, &message_approval);
    <b>assert</b>!(is_match, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMessageApprovalMismatch">EMessageApprovalMismatch</a>);
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap">VerifiedPartialUserSignatureCap</a> {
        id,
        partial_centralized_signed_message_id,
    } = partial_user_signature_cap;
    <b>let</b> verified_cap_id = id.to_inner();
    id.delete();
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignature">PartialUserSignature</a> {
        id,
        created_at_epoch: _,
        presign_cap,
        dwallet_id: _,
        cap_id,
        curve: _,
        signature_algorithm: _,
        hash_scheme: _,
        message: _,
        message_centralized_signature,
        state
    } = self.partial_centralized_signed_messages.remove(partial_centralized_signed_message_id);
    id.delete();
    <b>assert</b>!(cap_id == verified_cap_id && state == PartialUserSignatureState::NetworkVerificationCompleted, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EIncorrectCap">EIncorrectCap</a>);
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyMessageApproval">ImportedKeyMessageApproval</a> {
        dwallet_id,
        signature_algorithm,
        hash_scheme,
        message
    } = message_approval;
    // Emit signing events to finalize the signing process.
    <b>let</b> is_imported_key_dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_and_initiate_sign">validate_and_initiate_sign</a>(
        pricing,
        payment_ika,
        payment_sui,
        dwallet_id,
        signature_algorithm,
        hash_scheme,
        message,
        presign_cap,
        message_centralized_signature,
        <b>true</b>,
        ctx
    );
    <b>assert</b>!(is_imported_key_dwallet, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ENotImportedKeyDWallet">ENotImportedKeyDWallet</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_match_partial_user_signature_with_message_approval"></a>

## Function `match_partial_user_signature_with_message_approval`

Matches partial user signature with message approval to ensure they are consistent.
This function can be called by the user to verify before calling
the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_sign_with_partial_user_signature">request_sign_with_partial_user_signature</a></code> function.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_match_partial_user_signature_with_message_approval">match_partial_user_signature_with_message_approval</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, partial_user_signature_cap: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap">dwallet_2pc_mpc_coordinator_inner::VerifiedPartialUserSignatureCap</a>, message_approval: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">dwallet_2pc_mpc_coordinator_inner::MessageApproval</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_match_partial_user_signature_with_message_approval">match_partial_user_signature_with_message_approval</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    partial_user_signature_cap: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap">VerifiedPartialUserSignatureCap</a>,
    message_approval: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">MessageApproval</a>,
): bool {
    <b>let</b> partial_signature = self.partial_centralized_signed_messages.borrow(partial_user_signature_cap.partial_centralized_signed_message_id);
    partial_signature.dwallet_id == message_approval.dwallet_id &&
    partial_signature.message == message_approval.message &&
    partial_signature.signature_algorithm == message_approval.signature_algorithm &&
    partial_signature.hash_scheme == message_approval.hash_scheme
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_match_partial_user_signature_with_imported_key_message_approval"></a>

## Function `match_partial_user_signature_with_imported_key_message_approval`

Matches partial user signature with imported key message approval to ensure they are consistent.
This function can be called by the user to verify before calling
the <code>request_imported_key_sign_with_partial_user_signatures</code> function.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_match_partial_user_signature_with_imported_key_message_approval">match_partial_user_signature_with_imported_key_message_approval</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, partial_user_signature_cap: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap">dwallet_2pc_mpc_coordinator_inner::VerifiedPartialUserSignatureCap</a>, message_approval: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyMessageApproval">dwallet_2pc_mpc_coordinator_inner::ImportedKeyMessageApproval</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_match_partial_user_signature_with_imported_key_message_approval">match_partial_user_signature_with_imported_key_message_approval</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    partial_user_signature_cap: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap">VerifiedPartialUserSignatureCap</a>,
    message_approval: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyMessageApproval">ImportedKeyMessageApproval</a>,
): bool {
    <b>let</b> partial_signature = self.partial_centralized_signed_messages.borrow(partial_user_signature_cap.partial_centralized_signed_message_id);
    partial_signature.dwallet_id == message_approval.dwallet_id &&
    partial_signature.message == message_approval.message &&
    partial_signature.signature_algorithm == message_approval.signature_algorithm &&
    partial_signature.hash_scheme == message_approval.hash_scheme
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_sign"></a>

## Function `respond_sign`

Emits a <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedSignEvent">CompletedSignEvent</a></code> with the MPC Sign protocol output.

This function is called by the blockchain itself and is part of the core
blockchain logic executed by validators. The emitted event contains the
completed sign output that should be consumed by the initiating user.


<a name="@Parameters_28"></a>

##### Parameters

- **<code>signed_messages</code>**: A vector containing the signed message outputs.
- **<code>batch_session_id</code>**: The unique identifier for the batch signing session.
- **<code>ctx</code>**: The transaction context used for event emission.


<a name="@Requirements_29"></a>

##### Requirements

- The caller **must be the system address** (<code>@0x0</code>). If this condition is not met,
the function will abort with <code>ENotSystemAddress</code>.


<a name="@Events_30"></a>

##### Events

- **<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedSignEvent">CompletedSignEvent</a></code>**: Emitted with the <code>session_id</code> and <code>signed_messages</code>,
signaling the completion of the sign process for the batch session.


<a name="@Errors_31"></a>

##### Errors

- **<code>ENotSystemAddress</code>**: If the caller is not the system address (<code>@0x0</code>),
the function will abort with this error.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_sign">respond_sign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, sign_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, session_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, signature: vector&lt;u8&gt;, is_future_sign: bool, rejected: bool, session_sequence_number: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_sign">respond_sign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
    sign_id: ID,
    session_id: ID,
    signature: vector&lt;u8&gt;,
    is_future_sign: bool,
    rejected: bool,
    session_sequence_number: u64
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_session_and_charge">remove_session_and_charge</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignRequestEvent">SignRequestEvent</a>&gt;(session_sequence_number);
    <b>let</b> (dwallet, _) = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output_mut">get_active_dwallet_and_public_output_mut</a>(dwallet_id);
    <b>let</b> sign = dwallet.signs.borrow_mut(sign_id);
    sign.state = match(sign.state) {
        SignState::Requested =&gt; {
            <b>if</b>(rejected) {
                event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedSignEvent">RejectedSignEvent</a> {
                    sign_id,
                    session_id,
                    is_future_sign,
                });
                SignState::NetworkRejected
            } <b>else</b> {
                event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedSignEvent">CompletedSignEvent</a> {
                    sign_id,
                    session_id,
                    signature,
                    is_future_sign,
                });
                SignState::Completed { signature }
            }
        },
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ESignWrongState">ESignWrongState</a>
    };
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_process_checkpoint_message_by_quorum"></a>

## Function `process_checkpoint_message_by_quorum`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, signature: vector&lt;u8&gt;, signers_bitmap: vector&lt;u8&gt;, message: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    signature: vector&lt;u8&gt;,
    signers_bitmap: vector&lt;u8&gt;,
    message: vector&lt;u8&gt;,
) {
    <b>let</b> <b>mut</b> intent_bytes = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CHECKPOINT_MESSAGE_INTENT">CHECKPOINT_MESSAGE_INTENT</a>;
    intent_bytes.append(message);
    intent_bytes.append(bcs::to_bytes(&self.current_epoch));
    self.active_committee.verify_certificate(self.current_epoch, &signature, &signers_bitmap, &intent_bytes);
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_process_checkpoint_message">process_checkpoint_message</a>(message);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_process_checkpoint_message"></a>

## Function `process_checkpoint_message`



<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_process_checkpoint_message">process_checkpoint_message</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, message: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_process_checkpoint_message">process_checkpoint_message</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    message: vector&lt;u8&gt;,
) {
    <b>assert</b>!(!self.active_committee.members().is_empty(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EActiveBlsCommitteeMustInitialize">EActiveBlsCommitteeMustInitialize</a>);
    <b>let</b> <b>mut</b> bcs_body = bcs::new(<b>copy</b> message);
    <b>let</b> epoch = bcs_body.peel_u64();
    <b>assert</b>!(epoch == self.current_epoch, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EIncorrectEpochInCheckpoint">EIncorrectEpochInCheckpoint</a>);
    <b>let</b> sequence_number = bcs_body.peel_u64();
    <b>if</b>(self.last_processed_checkpoint_sequence_number.is_none()) {
        <b>assert</b>!(sequence_number == 0, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongCheckpointSequenceNumber">EWrongCheckpointSequenceNumber</a>);
        self.last_processed_checkpoint_sequence_number.fill(sequence_number);
    } <b>else</b> {
        <b>assert</b>!(sequence_number &gt; 0 && *self.last_processed_checkpoint_sequence_number.borrow() + 1 == sequence_number, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongCheckpointSequenceNumber">EWrongCheckpointSequenceNumber</a>);
        self.last_processed_checkpoint_sequence_number.swap(sequence_number);
    };
    <b>let</b> timestamp_ms = bcs_body.peel_u64();
    event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCheckpointInfoEvent">DWalletCheckpointInfoEvent</a> {
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
                <b>let</b> rejected = <b>false</b>;
                // TODO: Use this once we have a proper way to reject the first round
                //<b>let</b> rejected = bcs_body.peel_bool();
                <b>let</b> session_sequence_number = bcs_body.peel_u64();
                self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_dkg_first_round">respond_dwallet_dkg_first_round</a>(dwallet_id, first_round_output, rejected, session_sequence_number);
            } <b>else</b> <b>if</b> (message_data_type == 1) {
                <b>let</b> dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> session_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> encrypted_user_secret_key_share_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> public_output = bcs_body.peel_vec_u8();
                <b>let</b> rejected = bcs_body.peel_bool();
                <b>let</b> session_sequence_number = bcs_body.peel_u64();
                self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_dkg_second_round">respond_dwallet_dkg_second_round</a>(
                    dwallet_id,
                    public_output,
                    encrypted_user_secret_key_share_id,
                    session_id,
                    rejected,
                    session_sequence_number,
                );
            } <b>else</b> <b>if</b> (message_data_type == 2) {
                <b>let</b> dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> encrypted_user_secret_key_share_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> rejected = bcs_body.peel_bool();
                <b>let</b> session_sequence_number = bcs_body.peel_u64();
                self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_re_encrypt_user_share_for">respond_re_encrypt_user_share_for</a>(
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
                self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_sign">respond_sign</a>(
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
                self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_future_sign">respond_future_sign</a>(
                    session_id,
                    dwallet_id,
                    partial_centralized_signed_message_id,
                    rejected,
                    session_sequence_number
                );
            } <b>else</b> <b>if</b> (message_data_type == 4) {
                <b>let</b> dwallet_id = bcs_body.peel_option!(|bcs_option| object::id_from_bytes(bcs_option.peel_vec_u8()));
                <b>let</b> presign_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> session_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> presign = bcs_body.peel_vec_u8();
                <b>let</b> rejected = bcs_body.peel_bool();
                <b>let</b> session_sequence_number = bcs_body.peel_u64();
                self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_presign">respond_presign</a>(dwallet_id, presign_id, session_id, presign, rejected, session_sequence_number);
            } <b>else</b> <b>if</b> (message_data_type == 6) {
                <b>let</b> dwallet_network_decryption_key_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> public_output = bcs_body.peel_vec_u8();
                <b>let</b> is_last = bcs_body.peel_bool();
                self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_network_decryption_key_dkg">respond_dwallet_network_decryption_key_dkg</a>(dwallet_network_decryption_key_id, public_output, is_last);
            } <b>else</b> <b>if</b> (message_data_type == 7) {
                <b>let</b> dwallet_network_decryption_key_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> public_output = bcs_body.peel_vec_u8();
                <b>let</b> is_last = bcs_body.peel_bool();
                self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_network_decryption_key_reconfiguration">respond_dwallet_network_decryption_key_reconfiguration</a>(dwallet_network_decryption_key_id, public_output, is_last);
            };
        i = i + 1;
    };
    self.total_messages_processed = self.total_messages_processed + i;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_set_supported_curves_to_signature_algorithms_to_hash_schemes"></a>

## Function `set_supported_curves_to_signature_algorithms_to_hash_schemes`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_set_supported_curves_to_signature_algorithms_to_hash_schemes">set_supported_curves_to_signature_algorithms_to_hash_schemes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, supported_curves_to_signature_algorithms_to_hash_schemes: <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u32, <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u32, vector&lt;u32&gt;&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_set_supported_curves_to_signature_algorithms_to_hash_schemes">set_supported_curves_to_signature_algorithms_to_hash_schemes</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    supported_curves_to_signature_algorithms_to_hash_schemes: VecMap&lt;u32, VecMap&lt;u32, vector&lt;u32&gt;&gt;&gt;,
) {
    self.supported_curves_to_signature_algorithms_to_hash_schemes = supported_curves_to_signature_algorithms_to_hash_schemes;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_set_paused_curves_and_signature_algorithms"></a>

## Function `set_paused_curves_and_signature_algorithms`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_set_paused_curves_and_signature_algorithms">set_paused_curves_and_signature_algorithms</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, paused_curves: vector&lt;u32&gt;, paused_signature_algorithms: vector&lt;u32&gt;, paused_hash_schemes: vector&lt;u32&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_set_paused_curves_and_signature_algorithms">set_paused_curves_and_signature_algorithms</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    paused_curves: vector&lt;u32&gt;,
    paused_signature_algorithms: vector&lt;u32&gt;,
    paused_hash_schemes: vector&lt;u32&gt;,
) {
    self.paused_curves = paused_curves;
    self.paused_signature_algorithms = paused_signature_algorithms;
    self.paused_hash_schemes = paused_hash_schemes;
}
</code></pre>



</details>
