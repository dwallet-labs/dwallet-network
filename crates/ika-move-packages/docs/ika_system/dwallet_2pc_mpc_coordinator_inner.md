---
title: Module `(ika_system=0x0)::dwallet_2pc_mpc_coordinator_inner`
---

This module handles the logic for creating and managing dWallets using the Secp256K1 signature scheme
and the DKG process. It leverages validators to execute MPC (Multi-Party Computation)
protocols to ensure trustless and decentralized wallet creation and key management.


-  [Struct `SessionManagement`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionManagement)
-  [Struct `SupportConfig`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SupportConfig)
-  [Struct `PricingAndFeeManagement`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PricingAndFeeManagement)
-  [Struct `DWalletCoordinatorInner`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner)
-  [Struct `DWalletSessionEventKey`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSessionEventKey)
-  [Struct `DWalletSession`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSession)
-  [Struct `DWalletCap`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap)
-  [Struct `ImportedKeyDWalletCap`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyDWalletCap)
-  [Struct `DWalletNetworkEncryptionKeyCap`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyCap)
-  [Struct `DWalletNetworkEncryptionKey`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKey)
-  [Struct `EncryptionKey`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptionKey)
-  [Struct `EncryptedUserSecretKeyShare`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShare)
-  [Struct `UnverifiedPartialUserSignatureCap`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPartialUserSignatureCap)
-  [Struct `VerifiedPartialUserSignatureCap`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap)
-  [Struct `PartialUserSignature`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignature)
-  [Struct `DWallet`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet)
-  [Struct `UnverifiedPresignCap`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap)
-  [Struct `VerifiedPresignCap`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap)
-  [Struct `PresignSession`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignSession)
-  [Struct `SignSession`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignSession)
-  [Struct `DWalletEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletEvent)
-  [Struct `CreatedEncryptionKeyEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CreatedEncryptionKeyEvent)
-  [Struct `DWalletNetworkDKGEncryptionKeyRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDKGEncryptionKeyRequestEvent)
-  [Struct `CompletedDWalletNetworkDKGEncryptionKeyEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletNetworkDKGEncryptionKeyEvent)
-  [Struct `RejectedDWalletNetworkDKGEncryptionKeyEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletNetworkDKGEncryptionKeyEvent)
-  [Struct `DWalletEncryptionKeyReconfigurationRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletEncryptionKeyReconfigurationRequestEvent)
-  [Struct `CompletedDWalletEncryptionKeyReconfigurationEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletEncryptionKeyReconfigurationEvent)
-  [Struct `RejectedDWalletEncryptionKeyReconfigurationEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletEncryptionKeyReconfigurationEvent)
-  [Struct `DWalletDKGFirstRoundRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGFirstRoundRequestEvent)
-  [Struct `CompletedDWalletDKGFirstRoundEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletDKGFirstRoundEvent)
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
-  [Struct `AcceptEncryptedUserShareEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_AcceptEncryptedUserShareEvent)
-  [Struct `MakeDWalletUserSecretKeySharePublicRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MakeDWalletUserSecretKeySharePublicRequestEvent)
-  [Struct `CompletedMakeDWalletUserSecretKeySharePublicEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedMakeDWalletUserSecretKeySharePublicEvent)
-  [Struct `RejectedMakeDWalletUserSecretKeySharePublicEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedMakeDWalletUserSecretKeySharePublicEvent)
-  [Struct `PresignRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignRequestEvent)
-  [Struct `CompletedPresignEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedPresignEvent)
-  [Struct `RejectedPresignEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedPresignEvent)
-  [Struct `SignRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignRequestEvent)
-  [Struct `FutureSignRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_FutureSignRequestEvent)
-  [Struct `CompletedFutureSignEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedFutureSignEvent)
-  [Struct `RejectedFutureSignEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedFutureSignEvent)
-  [Struct `SetMaxActiveSessionsBufferEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SetMaxActiveSessionsBufferEvent)
-  [Struct `SetGasFeeReimbursementSuiSystemCallValueEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SetGasFeeReimbursementSuiSystemCallValueEvent)
-  [Struct `CompletedSignEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedSignEvent)
-  [Struct `RejectedSignEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedSignEvent)
-  [Struct `DWalletCheckpointInfoEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCheckpointInfoEvent)
-  [Struct `MessageApproval`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval)
-  [Struct `ImportedKeyMessageApproval`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyMessageApproval)
-  [Struct `NewImportedKeyDWalletEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_NewImportedKeyDWalletEvent)
-  [Enum `DWalletNetworkEncryptionKeyState`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyState)
-  [Enum `EncryptedUserSecretKeyShareState`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShareState)
-  [Enum `PartialUserSignatureState`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignatureState)
-  [Enum `DWalletState`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletState)
-  [Enum `PresignState`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignState)
-  [Enum `SignState`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignState)
-  [Enum `SessionType`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionType)
-  [Constants](#@Constants_2)
-  [Function `lock_last_active_session_sequence_number`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_lock_last_active_session_sequence_number)
-  [Function `create_dwallet_coordinator_inner`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_create_dwallet_coordinator_inner)
-  [Function `request_dwallet_network_encryption_key_dkg`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_network_encryption_key_dkg)
-  [Function `charge_gas_fee_reimbursement_sui_for_system_calls`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_gas_fee_reimbursement_sui_for_system_calls)
-  [Function `respond_dwallet_network_encryption_key_dkg`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_network_encryption_key_dkg)
-  [Function `respond_dwallet_network_encryption_key_reconfiguration`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_network_encryption_key_reconfiguration)
-  [Function `advance_epoch_dwallet_network_encryption_key`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_advance_epoch_dwallet_network_encryption_key)
-  [Function `mid_epoch_reconfiguration`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_mid_epoch_reconfiguration)
-  [Function `calculate_pricing_votes`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_calculate_pricing_votes)
-  [Function `emit_start_reconfiguration_event`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_emit_start_reconfiguration_event)
-  [Function `get_active_dwallet_network_encryption_key`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_network_encryption_key)
-  [Function `advance_epoch`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_advance_epoch)
-  [Function `get_dwallet`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet)
-  [Function `get_dwallet_mut`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut)
-  [Function `validate_active_and_get_public_output`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_active_and_get_public_output)
-  [Function `charge_and_create_current_epoch_dwallet_event`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event)
-  [Function `initiate_system_dwallet_session`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_initiate_system_dwallet_session)
-  [Function `get_active_dwallet_and_public_output`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output)
-  [Function `get_active_dwallet_and_public_output_mut`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output_mut)
-  [Function `get_active_encryption_key`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_encryption_key)
-  [Function `validate_curve`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_curve)
-  [Function `validate_curve_and_signature_algorithm`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_curve_and_signature_algorithm)
-  [Function `validate_curve_and_signature_algorithm_and_hash_scheme`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_curve_and_signature_algorithm_and_hash_scheme)
-  [Function `register_encryption_key`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_register_encryption_key)
-  [Function `approve_message`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_approve_message)
-  [Function `approve_imported_key_message`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_approve_imported_key_message)
-  [Function `validate_approve_message`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_approve_message)
-  [Function `request_dwallet_dkg_first_round`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_dkg_first_round)
-  [Function `update_last_user_initiated_session_to_complete_in_current_epoch`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_update_last_user_initiated_session_to_complete_in_current_epoch)
-  [Function `all_current_epoch_user_initiated_sessions_completed`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_all_current_epoch_user_initiated_sessions_completed)
-  [Function `remove_user_initiated_session_and_charge`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_user_initiated_session_and_charge)
-  [Function `respond_dwallet_dkg_first_round`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_dkg_first_round)
-  [Function `request_dwallet_dkg_second_round`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_dkg_second_round)
-  [Function `respond_dwallet_dkg_second_round`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_dkg_second_round)
-  [Function `request_re_encrypt_user_share_for`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_re_encrypt_user_share_for)
-  [Function `respond_re_encrypt_user_share_for`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_re_encrypt_user_share_for)
-  [Function `accept_encrypted_user_share`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_accept_encrypted_user_share)
-  [Function `new_imported_key_dwallet`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_new_imported_key_dwallet)
-  [Function `request_imported_key_dwallet_verification`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_imported_key_dwallet_verification)
-  [Function `respond_imported_key_dwallet_verification`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_imported_key_dwallet_verification)
-  [Function `request_make_dwallet_user_secret_key_share_public`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_make_dwallet_user_secret_key_share_public)
-  [Function `respond_make_dwallet_user_secret_key_share_public`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_make_dwallet_user_secret_key_share_public)
-  [Function `request_presign`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_presign)
-  [Function `request_global_presign`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_global_presign)
-  [Function `respond_presign`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_presign)
-  [Function `is_presign_valid`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_is_presign_valid)
-  [Function `verify_presign_cap`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_verify_presign_cap)
-  [Function `validate_and_initiate_sign`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_and_initiate_sign)
-  [Function `request_sign`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_sign)
-  [Function `request_imported_key_sign`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_imported_key_sign)
-  [Function `request_future_sign`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_future_sign)
-  [Function `respond_future_sign`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_future_sign)
-  [Function `is_partial_user_signature_valid`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_is_partial_user_signature_valid)
-  [Function `verify_partial_user_signature_cap`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_verify_partial_user_signature_cap)
-  [Function `request_sign_with_partial_user_signature`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_sign_with_partial_user_signature)
-  [Function `request_imported_key_sign_with_partial_user_signature`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_imported_key_sign_with_partial_user_signature)
-  [Function `match_partial_user_signature_with_message_approval`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_match_partial_user_signature_with_message_approval)
-  [Function `match_partial_user_signature_with_imported_key_message_approval`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_match_partial_user_signature_with_imported_key_message_approval)
-  [Function `respond_sign`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_sign)
-  [Function `process_checkpoint_message_by_quorum`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_process_checkpoint_message_by_quorum)
-  [Function `process_checkpoint_message`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_process_checkpoint_message)
-  [Function `set_max_active_sessions_buffer`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_set_max_active_sessions_buffer)
-  [Function `set_gas_fee_reimbursement_sui_system_call_value`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_set_gas_fee_reimbursement_sui_system_call_value)
-  [Function `set_supported_and_pricing`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_set_supported_and_pricing)
-  [Function `verify_pricing_exists_for_all_protocols`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_verify_pricing_exists_for_all_protocols)
-  [Function `set_paused_curves_and_signature_algorithms`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_set_paused_curves_and_signature_algorithms)
-  [Function `set_pricing_vote`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_set_pricing_vote)
-  [Function `subsidize_coordinator_with_sui`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_subsidize_coordinator_with_sui)
-  [Function `subsidize_coordinator_with_ika`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_subsidize_coordinator_with_ika)


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



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionManagement"></a>

## Struct `SessionManagement`

Session management data for the dWallet coordinator.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionManagement">SessionManagement</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>sessions: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;u64, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSession">dwallet_2pc_mpc_coordinator_inner::DWalletSession</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>user_requested_sessions_events: <a href="../sui/bag.md#sui_bag_Bag">sui::bag::Bag</a></code>
</dt>
<dd>
</dd>
<dt>
<code>number_of_completed_user_initiated_sessions: u64</code>
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
 The sequence number to assign to the next user-requested session.
 Initialized to <code>1</code> and incremented at every new session creation.
</dd>
<dt>
<code>last_user_initiated_session_to_complete_in_current_epoch: u64</code>
</dt>
<dd>
 The last MPC session to process in the current epoch.
 The validators of the Ika network must always begin sessions,
 when they become available to them, so long their sequence number is lesser or equal to this value.
 Initialized to <code>0</code>, as when the system is initialized no user-requested session exists so none should be started
 and we shouldn't wait for any to complete before advancing epoch (until the first session is created),
 and updated at every new session creation or completion, and when advancing epochs,
 to the latest session whilst assuring a maximum of <code>max_active_sessions_buffer</code> sessions to be completed in the current epoch.
 Validators should complete every session they start before switching epochs.
</dd>
<dt>
<code>locked_last_user_initiated_session_to_complete_in_current_epoch: bool</code>
</dt>
<dd>
 Denotes whether the <code>last_user_initiated_session_to_complete_in_current_epoch</code> field is locked or not.
 This field gets locked before performing the epoch switch.
</dd>
<dt>
<code>max_active_sessions_buffer: u64</code>
</dt>
<dd>
 The maximum number of active MPC sessions Ika nodes may run during an epoch.
 Validators should complete every session they start before switching epochs.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SupportConfig"></a>

## Struct `SupportConfig`

Support data for the dWallet coordinator, including curve and algorithm configurations.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SupportConfig">SupportConfig</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>supported_curves_to_signature_algorithms_to_hash_schemes: <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u32, <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u32, vector&lt;u32&gt;&gt;&gt;</code>
</dt>
<dd>
 A nested map of supported curves to signature algorithms to hash schemes.
 e.g. secp256k1 -> [(ecdsa -> [sha256, keccak256]), (schnorr -> [sha256])]
</dd>
<dt>
<code>paused_curves: vector&lt;u32&gt;</code>
</dt>
<dd>
 A list of paused curves in case of emergency.
 e.g. [secp256k1, ristretto]
</dd>
<dt>
<code>paused_signature_algorithms: vector&lt;u32&gt;</code>
</dt>
<dd>
 A list of paused signature algorithms in case of emergency.
 e.g. [ecdsa, schnorr]
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
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PricingAndFeeManagement"></a>

## Struct `PricingAndFeeManagement`

Pricing and fee management data for the dWallet coordinator.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PricingAndFeeManagement">PricingAndFeeManagement</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>current: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing">dwallet_pricing::DWalletPricing</a></code>
</dt>
<dd>
 The pricing for the current epoch.
</dd>
<dt>
<code>default: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing">dwallet_pricing::DWalletPricing</a></code>
</dt>
<dd>
 The default pricing.
</dd>
<dt>
<code>validator_votes: <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing">dwallet_pricing::DWalletPricing</a>&gt;</code>
</dt>
<dd>
 The votes for the pricing set by validators.
 The key is the validator ID to their votes.
</dd>
<dt>
<code>calculation_votes: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;(ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricingCalculationVotes">dwallet_pricing::DWalletPricingCalculationVotes</a>&gt;</code>
</dt>
<dd>
 The votes for the pricing calculation, if set, we have to complete the pricing
 calculation before we advance to the next epoch.
</dd>
<dt>
<code>gas_fee_reimbursement_sui_system_call_value: u64</code>
</dt>
<dd>
 The value of the gas fee reimbursement for system calls.
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
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner"></a>

## Struct `DWalletCoordinatorInner`

A shared object that holds all the Ika system object used to manage dWallets:

Most importantly, the <code>dwallets</code> themselves, which holds the public key and public key shares,
and the encryption of the network's share under the network's threshold encryption key.
The encryption of the network's secret key share for every dWallet points to an encryption key in <code>dwallet_network_encryption_keys</code>,
which also stores the encrypted encryption key shares of each validator and their public verification keys.

For the user side, the secret key share is stored encrypted to the user encryption key (in <code>encryption_keys</code>) inside the dWallet,
together with a signature on the public key (shares).
Together, these constitute the necessary information to create a signature with the user.

Next, <code>presign_sessions</code> holds the outputs of the Presign protocol which are later used for the signing protocol,
and <code>partial_centralized_signed_messages</code> holds the partial signatures of users awaiting for a future sign once a <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">MessageApproval</a></code> is presented.

Additionally, this structure holds management information, like the <code>previous_committee</code> and <code>active_committee</code> committees,
information regarding <code>pricing</code>, all the <code>sessions</code> and the <code>next_session_sequence_number</code> that will be used for the next session,
and various other fields, like the supported and paused curves, signing algorithms and hashes.


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
<code>session_management: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionManagement">dwallet_2pc_mpc_coordinator_inner::SessionManagement</a></code>
</dt>
<dd>
</dd>
<dt>
<code>dwallets: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">dwallet_2pc_mpc_coordinator_inner::DWallet</a>&gt;</code>
</dt>
<dd>
 The key is the ID of <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a></code>.
</dd>
<dt>
<code>dwallet_network_encryption_keys: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKey">dwallet_2pc_mpc_coordinator_inner::DWalletNetworkEncryptionKey</a>&gt;</code>
</dt>
<dd>
 The key is the ID of <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKey">DWalletNetworkEncryptionKey</a></code>.
</dd>
<dt>
<code>encryption_keys: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;<b>address</b>, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptionKey">dwallet_2pc_mpc_coordinator_inner::EncryptionKey</a>&gt;</code>
</dt>
<dd>
 A table mapping user addresses to encryption key object IDs.
</dd>
<dt>
<code>presign_sessions: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignSession">dwallet_2pc_mpc_coordinator_inner::PresignSession</a>&gt;</code>
</dt>
<dd>
 A table mapping id to their presign sessions.
</dd>
<dt>
<code>partial_centralized_signed_messages: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignature">dwallet_2pc_mpc_coordinator_inner::PartialUserSignature</a>&gt;</code>
</dt>
<dd>
 A table mapping id to their partial centralized signed messages.
</dd>
<dt>
<code>pricing_and_fee_management: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PricingAndFeeManagement">dwallet_2pc_mpc_coordinator_inner::PricingAndFeeManagement</a></code>
</dt>
<dd>
 Pricing and fee management data.
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
<code>support_config: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SupportConfig">dwallet_2pc_mpc_coordinator_inner::SupportConfig</a></code>
</dt>
<dd>
 Support data for curves, algorithms, and their configurations.
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

An Ika MPC session.


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
<code>dwallet_network_encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyCap"></a>

## Struct `DWalletNetworkEncryptionKeyCap`

Represents a capability granting control over a specific dWallet network encryption key.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyCap">DWalletNetworkEncryptionKeyCap</a> <b>has</b> key, store
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
<code>dwallet_network_encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKey"></a>

## Struct `DWalletNetworkEncryptionKey`

<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKey">DWalletNetworkEncryptionKey</a></code> represents a (threshold) encryption key owned by the network.
It stores the <code>network_dkg_public_output</code>, which in turn stores the encryption key itself (divided to chunks, due to space limitations).
Before the first reconfiguration (which happens at every epoch switch,)
<code>network_dkg_public_output</code> also holds the encryption of the current encryption key shares
(encrypted to each validator's encryption key, and decrypted by them whenever they start)
and the public verification keys of all validators, from which the public parameters of the threshold encryption scheme
can be generated.
After the first reconfiguration, <code>reconfiguration_public_outputs</code> holds this information updated for the <code>current_epoch</code>.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKey">DWalletNetworkEncryptionKey</a> <b>has</b> key, store
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
<code>dwallet_network_encryption_key_cap_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
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
<code>state: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyState">dwallet_2pc_mpc_coordinator_inner::DWalletNetworkEncryptionKeyState</a></code>
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
When a user intends to send encrypted data (i.e. when sharing the secret key share to grant access and/or transfer a dWallet) to another user,
they use the recipient's encryption key to encrypt the data.
The recipient is then the sole entity capable of decrypting and accessing this information, ensuring secure, end-to-end encryption.


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
 Used to verify the data originated from the <code>signer_address</code>.
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
<code>public_user_secret_key_share: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
 If not set, the user secret key shares is not public, and the user will need to
 keep it encrypted using encrypted user secret key shares. It is
 the case where we have zero trust for the dWallet because the
 user participation is required.
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
<code>dwallet_network_encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The MPC network encryption key id that is used to encrypt this dWallet network secret key share.
</dd>
<dt>
<code>is_imported_key_dwallet: bool</code>
</dt>
<dd>
 Key was imported.
</dd>
<dt>
<code>encrypted_user_secret_key_shares: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShare">dwallet_2pc_mpc_coordinator_inner::EncryptedUserSecretKeyShare</a>&gt;</code>
</dt>
<dd>
 A table mapping id to their encryption key object.
</dd>
<dt>
<code>sign_sessions: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignSession">dwallet_2pc_mpc_coordinator_inner::SignSession</a>&gt;</code>
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
 which can be used for any dWallet (under the same network key). Others, like ECDSA, must have this set.
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
 which can be used for any dWallet (under the same network key). Others, like ECDSA, must have this set.
</dd>
<dt>
<code>presign_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The ID of the presign.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignSession"></a>

## Struct `PresignSession`

A session of the Presign protocol.
When <code>state</code> is <code>PresignState::Completed</code>, holds a presign:
a single-use precomputation that does not depend on the message,
used to speed up the (online) Sign protocol.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignSession">PresignSession</a> <b>has</b> key, store
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignSession"></a>

## Struct `SignSession`

A Sign session. When <code>state</code> is <code>SignState::Completed</code>, holds the <code>signature</code>.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignSession">SignSession</a> <b>has</b> key, store
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



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletEvent">DWalletEvent</a>&lt;E: <b>copy</b>, drop, store&gt; <b>has</b> <b>copy</b>, drop, store
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDKGEncryptionKeyRequestEvent"></a>

## Struct `DWalletNetworkDKGEncryptionKeyRequestEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDKGEncryptionKeyRequestEvent">DWalletNetworkDKGEncryptionKeyRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dwallet_network_encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletNetworkDKGEncryptionKeyEvent"></a>

## Struct `CompletedDWalletNetworkDKGEncryptionKeyEvent`

An event emitted when the first round of the DKG process is completed.

This event is emitted by the blockchain to notify the user about
the completion of the first round.
The user should catch this event to generate inputs for
the second round and call the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_dkg_second_round">request_dwallet_dkg_second_round</a>()</code> function.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletNetworkDKGEncryptionKeyEvent">CompletedDWalletNetworkDKGEncryptionKeyEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dwallet_network_encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletNetworkDKGEncryptionKeyEvent"></a>

## Struct `RejectedDWalletNetworkDKGEncryptionKeyEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletNetworkDKGEncryptionKeyEvent">RejectedDWalletNetworkDKGEncryptionKeyEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dwallet_network_encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletEncryptionKeyReconfigurationRequestEvent"></a>

## Struct `DWalletEncryptionKeyReconfigurationRequestEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletEncryptionKeyReconfigurationRequestEvent">DWalletEncryptionKeyReconfigurationRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dwallet_network_encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletEncryptionKeyReconfigurationEvent"></a>

## Struct `CompletedDWalletEncryptionKeyReconfigurationEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletEncryptionKeyReconfigurationEvent">CompletedDWalletEncryptionKeyReconfigurationEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dwallet_network_encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletEncryptionKeyReconfigurationEvent"></a>

## Struct `RejectedDWalletEncryptionKeyReconfigurationEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletEncryptionKeyReconfigurationEvent">RejectedDWalletEncryptionKeyReconfigurationEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dwallet_network_encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
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
<code>dwallet_network_encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The MPC network encryption key id that is used to encrypt associated dWallet network secret key share.
</dd>
<dt>
<code>curve: u32</code>
</dt>
<dd>
 The elliptic curve used for the dWallet.
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletDKGFirstRoundEvent"></a>

## Struct `CompletedDWalletDKGFirstRoundEvent`

An event emitted when the first round of the DKG process is completed.

This event is emitted by the blockchain to notify the user about
the completion of the first round.
The user should catch this event to generate inputs for
the second round and call the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_dkg_second_round">request_dwallet_dkg_second_round</a>()</code> function.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletDKGFirstRoundEvent">CompletedDWalletDKGFirstRoundEvent</a> <b>has</b> <b>copy</b>, drop, store
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
<code>dwallet_network_encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The MPC network encryption key id that is used to encrypt associated dWallet network secret key share.
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
<code>encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
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
<code>dwallet_network_encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The MPC network encryption key id that is used to encrypt associated dWallet network secret key share.
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
<code>dwallet_network_encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_AcceptEncryptedUserShareEvent"></a>

## Struct `AcceptEncryptedUserShareEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_AcceptEncryptedUserShareEvent">AcceptEncryptedUserShareEvent</a> <b>has</b> <b>copy</b>, drop, store
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MakeDWalletUserSecretKeySharePublicRequestEvent"></a>

## Struct `MakeDWalletUserSecretKeySharePublicRequestEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MakeDWalletUserSecretKeySharePublicRequestEvent">MakeDWalletUserSecretKeySharePublicRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>public_user_secret_key_share: vector&lt;u8&gt;</code>
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
<code>dwallet_network_encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedMakeDWalletUserSecretKeySharePublicEvent"></a>

## Struct `CompletedMakeDWalletUserSecretKeySharePublicEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedMakeDWalletUserSecretKeySharePublicEvent">CompletedMakeDWalletUserSecretKeySharePublicEvent</a> <b>has</b> <b>copy</b>, drop, store
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedMakeDWalletUserSecretKeySharePublicEvent"></a>

## Struct `RejectedMakeDWalletUserSecretKeySharePublicEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedMakeDWalletUserSecretKeySharePublicEvent">RejectedMakeDWalletUserSecretKeySharePublicEvent</a> <b>has</b> <b>copy</b>, drop, store
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
<code>dwallet_network_encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The MPC network encryption key id that is used to encrypt associated dWallet network secret key share.
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
<code>dwallet_network_encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The MPC network encryption key id that is used to encrypt associated dWallet network secret key share.
</dd>
<dt>
<code>presign_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The presign object ID, this ID will
 be used as the signature MPC protocol ID.
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
<code>dwallet_network_encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SetMaxActiveSessionsBufferEvent"></a>

## Struct `SetMaxActiveSessionsBufferEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SetMaxActiveSessionsBufferEvent">SetMaxActiveSessionsBufferEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>max_active_sessions_buffer: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SetGasFeeReimbursementSuiSystemCallValueEvent"></a>

## Struct `SetGasFeeReimbursementSuiSystemCallValueEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SetGasFeeReimbursementSuiSystemCallValueEvent">SetGasFeeReimbursementSuiSystemCallValueEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>gas_fee_reimbursement_sui_system_call_value: u64</code>
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
 The signature that was generated in this session.
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
the checkpoint submission message.


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

Represents a message that was approved to be signed by the dWallet corresponding to <code>dwallet_id</code>.


<a name="@Fields_0"></a>

##### Fields

- **<code>dwallet_id</code>**: The identifier of the dWallet
associated with this approval.
- **<code>hash_scheme</code>**: The message hash scheme to use for signing.
- **<code>signature_algorithm</code>**: The signature algorithm with which the message can be signed.
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

Represents a message that was approved to be signed by the imported key dWallet corresponding to <code>dwallet_id</code>.


<a name="@Fields_1"></a>

##### Fields

- **<code>dwallet_id</code>**: The identifier of the dWallet
associated with this approval.
- **<code>hash_scheme</code>**: The message hash scheme to use for signing.
- **<code>signature_algorithm</code>**: The signature algorithm with which the message can be signed.
- **<code>message</code>**: The message that has been approved.


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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_NewImportedKeyDWalletEvent"></a>

## Struct `NewImportedKeyDWalletEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_NewImportedKeyDWalletEvent">NewImportedKeyDWalletEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>dwallet_cap_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyState"></a>

## Enum `DWalletNetworkEncryptionKeyState`



<pre><code><b>public</b> <b>enum</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyState">DWalletNetworkEncryptionKeyState</a> <b>has</b> <b>copy</b>, drop, store
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
Variant <code>NetworkDKGCompleted</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>AwaitingNetworkReconfiguration</code>
</dt>
<dd>
 Reconfiguration request was sent to the network, but didn't finish yet.
 <code>is_first</code> is true if this is the first reconfiguration request, false otherwise.
</dd>

<dl>
<dt>
<code>is_first: bool</code>
</dt>
<dd>
</dd>
</dl>

<dt>
Variant <code>AwaitingNextEpochToUpdateReconfiguration</code>
</dt>
<dd>
 Reconfiguration request finished, but we didn't switch an epoch yet.
 We need to wait for the next epoch to update the reconfiguration public outputs.
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
Variant <code>NetworkVerificationCompleted</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>NetworkVerificationRejected</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>KeyHolderSigned</code>
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
Variant <code>AwaitingNetworkDKGVerification</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>NetworkRejectedDKGVerification</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>AwaitingUserImportedKeyInitiation</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>AwaitingNetworkImportedKeyVerification</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>NetworkRejectedImportedKeyVerification</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>AwaitingKeyHolderSignature</code>
</dt>
<dd>
</dd>

<dl>
<dt>
<code>public_output: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>

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
Variant <code>Requested</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>NetworkRejected</code>
</dt>
<dd>
</dd>
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
Variant <code>Requested</code>
</dt>
<dd>
</dd>
<dt>
Variant <code>NetworkRejected</code>
</dt>
<dd>
</dd>
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

<dt>
Variant <code>System</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_2"></a>

## Constants


<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CHECKPOINT_MESSAGE_INTENT"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CHECKPOINT_MESSAGE_INTENT">CHECKPOINT_MESSAGE_INTENT</a>: vector&lt;u8&gt; = vector[1, 0, 0];
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DKG_FIRST_ROUND_PROTOCOL_FLAG"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DKG_FIRST_ROUND_PROTOCOL_FLAG">DKG_FIRST_ROUND_PROTOCOL_FLAG</a>: u32 = 0;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DKG_SECOND_ROUND_PROTOCOL_FLAG"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DKG_SECOND_ROUND_PROTOCOL_FLAG">DKG_SECOND_ROUND_PROTOCOL_FLAG</a>: u32 = 1;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RE_ENCRYPT_USER_SHARE_PROTOCOL_FLAG"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RE_ENCRYPT_USER_SHARE_PROTOCOL_FLAG">RE_ENCRYPT_USER_SHARE_PROTOCOL_FLAG</a>: u32 = 2;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MAKE_DWALLET_USER_SECRET_KEY_SHARE_PUBLIC_PROTOCOL_FLAG"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MAKE_DWALLET_USER_SECRET_KEY_SHARE_PUBLIC_PROTOCOL_FLAG">MAKE_DWALLET_USER_SECRET_KEY_SHARE_PUBLIC_PROTOCOL_FLAG</a>: u32 = 3;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_IMPORTED_KEY_DWALLET_VERIFICATION_PROTOCOL_FLAG"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_IMPORTED_KEY_DWALLET_VERIFICATION_PROTOCOL_FLAG">IMPORTED_KEY_DWALLET_VERIFICATION_PROTOCOL_FLAG</a>: u32 = 4;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PRESIGN_PROTOCOL_FLAG"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PRESIGN_PROTOCOL_FLAG">PRESIGN_PROTOCOL_FLAG</a>: u32 = 5;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SIGN_PROTOCOL_FLAG"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SIGN_PROTOCOL_FLAG">SIGN_PROTOCOL_FLAG</a>: u32 = 6;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_FUTURE_SIGN_PROTOCOL_FLAG"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_FUTURE_SIGN_PROTOCOL_FLAG">FUTURE_SIGN_PROTOCOL_FLAG</a>: u32 = 7;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SIGN_WITH_PARTIAL_USER_SIGNATURE_PROTOCOL_FLAG"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SIGN_WITH_PARTIAL_USER_SIGNATURE_PROTOCOL_FLAG">SIGN_WITH_PARTIAL_USER_SIGNATURE_PROTOCOL_FLAG</a>: u32 = 8;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_DKG_FIRST_ROUND_OUTPUT_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_DKG_FIRST_ROUND_OUTPUT_MESSAGE_TYPE">RESPOND_DWALLET_DKG_FIRST_ROUND_OUTPUT_MESSAGE_TYPE</a>: u64 = 0;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_DKG_SECOND_ROUND_OUTPUT_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_DKG_SECOND_ROUND_OUTPUT_MESSAGE_TYPE">RESPOND_DWALLET_DKG_SECOND_ROUND_OUTPUT_MESSAGE_TYPE</a>: u64 = 1;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_ENCRYPTED_USER_SHARE_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_ENCRYPTED_USER_SHARE_MESSAGE_TYPE">RESPOND_DWALLET_ENCRYPTED_USER_SHARE_MESSAGE_TYPE</a>: u64 = 2;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_MAKE_DWALLET_USER_SECRET_KEY_SHARES_PUBLIC_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_MAKE_DWALLET_USER_SECRET_KEY_SHARES_PUBLIC_MESSAGE_TYPE">RESPOND_MAKE_DWALLET_USER_SECRET_KEY_SHARES_PUBLIC_MESSAGE_TYPE</a>: u64 = 3;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_IMPORTED_KEY_VERIFICATION_OUTPUT_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_IMPORTED_KEY_VERIFICATION_OUTPUT_MESSAGE_TYPE">RESPOND_DWALLET_IMPORTED_KEY_VERIFICATION_OUTPUT_MESSAGE_TYPE</a>: u64 = 4;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_PRESIGN_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_PRESIGN_MESSAGE_TYPE">RESPOND_DWALLET_PRESIGN_MESSAGE_TYPE</a>: u64 = 5;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_SIGN_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_SIGN_MESSAGE_TYPE">RESPOND_DWALLET_SIGN_MESSAGE_TYPE</a>: u64 = 6;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_PARTIAL_SIGNATURE_VERIFICATION_OUTPUT_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_PARTIAL_SIGNATURE_VERIFICATION_OUTPUT_MESSAGE_TYPE">RESPOND_DWALLET_PARTIAL_SIGNATURE_VERIFICATION_OUTPUT_MESSAGE_TYPE</a>: u64 = 7;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_MPC_NETWORK_DKG_OUTPUT_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_MPC_NETWORK_DKG_OUTPUT_MESSAGE_TYPE">RESPOND_DWALLET_MPC_NETWORK_DKG_OUTPUT_MESSAGE_TYPE</a>: u64 = 8;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_MPC_NETWORK_RECONFIGURATION_OUTPUT_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_MPC_NETWORK_RECONFIGURATION_OUTPUT_MESSAGE_TYPE">RESPOND_DWALLET_MPC_NETWORK_RECONFIGURATION_OUTPUT_MESSAGE_TYPE</a>: u64 = 9;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SET_MAX_ACTIVE_SESSIONS_BUFFER_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SET_MAX_ACTIVE_SESSIONS_BUFFER_MESSAGE_TYPE">SET_MAX_ACTIVE_SESSIONS_BUFFER_MESSAGE_TYPE</a>: u64 = 10;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SET_GAS_FEE_REIMBURSEMENT_SUI_SYSTEM_CALL_VALUE_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SET_GAS_FEE_REIMBURSEMENT_SUI_SYSTEM_CALL_VALUE_MESSAGE_TYPE">SET_GAS_FEE_REIMBURSEMENT_SUI_SYSTEM_CALL_VALUE_MESSAGE_TYPE</a>: u64 = 11;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletMismatch"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletMismatch">EDWalletMismatch</a>: u64 = 1;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletInactive"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletInactive">EDWalletInactive</a>: u64 = 2;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNotExists"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNotExists">EDWalletNotExists</a>: u64 = 3;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongState"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongState">EWrongState</a>: u64 = 4;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkEncryptionKeyNotExist"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkEncryptionKeyNotExist">EDWalletNetworkEncryptionKeyNotExist</a>: u64 = 5;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidEncryptionKeySignature"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidEncryptionKeySignature">EInvalidEncryptionKeySignature</a>: u64 = 6;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMessageApprovalMismatch"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMessageApprovalMismatch">EMessageApprovalMismatch</a>: u64 = 7;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidHashScheme"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidHashScheme">EInvalidHashScheme</a>: u64 = 8;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ESignWrongState"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ESignWrongState">ESignWrongState</a>: u64 = 9;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EPresignNotExist"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EPresignNotExist">EPresignNotExist</a>: u64 = 10;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EIncorrectCap"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EIncorrectCap">EIncorrectCap</a>: u64 = 11;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EUnverifiedCap"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EUnverifiedCap">EUnverifiedCap</a>: u64 = 12;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidSource"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidSource">EInvalidSource</a>: u64 = 13;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkEncryptionKeyNotActive"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkEncryptionKeyNotActive">EDWalletNetworkEncryptionKeyNotActive</a>: u64 = 14;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidPresign"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidPresign">EInvalidPresign</a>: u64 = 15;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ECannotAdvanceEpoch"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ECannotAdvanceEpoch">ECannotAdvanceEpoch</a>: u64 = 16;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidCurve"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidCurve">EInvalidCurve</a>: u64 = 17;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidSignatureAlgorithm"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidSignatureAlgorithm">EInvalidSignatureAlgorithm</a>: u64 = 18;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ECurvePaused"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ECurvePaused">ECurvePaused</a>: u64 = 19;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ESignatureAlgorithmPaused"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ESignatureAlgorithmPaused">ESignatureAlgorithmPaused</a>: u64 = 20;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletUserSecretKeySharesAlreadyPublic"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletUserSecretKeySharesAlreadyPublic">EDWalletUserSecretKeySharesAlreadyPublic</a>: u64 = 21;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMismatchCurve"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMismatchCurve">EMismatchCurve</a>: u64 = 22;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EImportedKeyDWallet"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EImportedKeyDWallet">EImportedKeyDWallet</a>: u64 = 23;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ENotImportedKeyDWallet"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ENotImportedKeyDWallet">ENotImportedKeyDWallet</a>: u64 = 24;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EHashSchemePaused"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EHashSchemePaused">EHashSchemePaused</a>: u64 = 25;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EEncryptionKeyNotExist"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EEncryptionKeyNotExist">EEncryptionKeyNotExist</a>: u64 = 26;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMissingProtocolPricing"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMissingProtocolPricing">EMissingProtocolPricing</a>: u64 = 27;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EPricingCalculationVotesHasNotBeenStarted"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EPricingCalculationVotesHasNotBeenStarted">EPricingCalculationVotesHasNotBeenStarted</a>: u64 = 28;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EPricingCalculationVotesMustBeCompleted"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EPricingCalculationVotesMustBeCompleted">EPricingCalculationVotesMustBeCompleted</a>: u64 = 29;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ECannotSetDuringVotesCalculation"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ECannotSetDuringVotesCalculation">ECannotSetDuringVotesCalculation</a>: u64 = 30;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EIncorrectEpochInCheckpoint"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EIncorrectEpochInCheckpoint">EIncorrectEpochInCheckpoint</a>: vector&lt;u8&gt; = b"The checkpoint epoch is incorrect.";
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongCheckpointSequenceNumber"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongCheckpointSequenceNumber">EWrongCheckpointSequenceNumber</a>: vector&lt;u8&gt; = b"The checkpoint sequence number should be the expected next one.";
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EActiveBlsCommitteeMustInitialize"></a>



<pre><code>#[error]
<b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EActiveBlsCommitteeMustInitialize">EActiveBlsCommitteeMustInitialize</a>: vector&lt;u8&gt; = b"First active committee must initialize.";
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_lock_last_active_session_sequence_number"></a>

## Function `lock_last_active_session_sequence_number`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_lock_last_active_session_sequence_number">lock_last_active_session_sequence_number</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_lock_last_active_session_sequence_number">lock_last_active_session_sequence_number</a>(self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>) {
    self.session_management.locked_last_user_initiated_session_to_complete_in_current_epoch = <b>true</b>;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_create_dwallet_coordinator_inner"></a>

## Function `create_dwallet_coordinator_inner`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_create_dwallet_coordinator_inner">create_dwallet_coordinator_inner</a>(current_epoch: u64, active_committee: (ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">bls_committee::BlsCommittee</a>, pricing: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing">dwallet_pricing::DWalletPricing</a>, supported_curves_to_signature_algorithms_to_hash_schemes: <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u32, <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u32, vector&lt;u32&gt;&gt;&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_create_dwallet_coordinator_inner">create_dwallet_coordinator_inner</a>(
    current_epoch: u64,
    active_committee: BlsCommittee,
    pricing: DWalletPricing,
    supported_curves_to_signature_algorithms_to_hash_schemes: VecMap&lt;u32, VecMap&lt;u32, vector&lt;u32&gt;&gt;&gt;,
    ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a> {
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_verify_pricing_exists_for_all_protocols">verify_pricing_exists_for_all_protocols</a>(&supported_curves_to_signature_algorithms_to_hash_schemes, &pricing);
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a> {
        current_epoch,
        session_management: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionManagement">SessionManagement</a> {
            sessions: object_table::new(ctx),
            user_requested_sessions_events: bag::new(ctx),
            number_of_completed_user_initiated_sessions: 0,
            started_system_sessions_count: 0,
            completed_system_sessions_count: 0,
            next_session_sequence_number: 1,
            last_user_initiated_session_to_complete_in_current_epoch: 0,
            locked_last_user_initiated_session_to_complete_in_current_epoch: <b>true</b>,
            max_active_sessions_buffer: 100,
        },
        dwallets: object_table::new(ctx),
        dwallet_network_encryption_keys: object_table::new(ctx),
        encryption_keys: object_table::new(ctx),
        presign_sessions: object_table::new(ctx),
        partial_centralized_signed_messages: object_table::new(ctx),
        pricing_and_fee_management: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PricingAndFeeManagement">PricingAndFeeManagement</a> {
            current: pricing,
            default: pricing,
            validator_votes: table::new(ctx),
            calculation_votes: option::none(),
            gas_fee_reimbursement_sui_system_call_value: 0,
            gas_fee_reimbursement_sui: balance::zero(),
            consensus_validation_fee_charged_ika: balance::zero(),
        },
        active_committee,
        previous_committee: <a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_empty">bls_committee::empty</a>(),
        total_messages_processed: 0,
        last_processed_checkpoint_sequence_number: option::none(),
        previous_epoch_last_checkpoint_sequence_number: 0,
        support_config: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SupportConfig">SupportConfig</a> {
            supported_curves_to_signature_algorithms_to_hash_schemes,
            paused_curves: vector[],
            paused_signature_algorithms: vector[],
            paused_hash_schemes: vector[],
            signature_algorithms_allowed_global_presign: vector[],
        },
        extra_fields: bag::new(ctx),
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_network_encryption_key_dkg"></a>

## Function `request_dwallet_network_encryption_key_dkg`

Start a Distributed Key Generation (DKG) session for the network (threshold) encryption key.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_network_encryption_key_dkg">request_dwallet_network_encryption_key_dkg</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyCap">dwallet_2pc_mpc_coordinator_inner::DWalletNetworkEncryptionKeyCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_network_encryption_key_dkg">request_dwallet_network_encryption_key_dkg</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyCap">DWalletNetworkEncryptionKeyCap</a> {
    // Create a new capability to control this encryption key.
    <b>let</b> id = object::new(ctx);
    <b>let</b> dwallet_network_encryption_key_id = id.to_inner();
    <b>let</b> cap = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyCap">DWalletNetworkEncryptionKeyCap</a> {
        id: object::new(ctx),
        dwallet_network_encryption_key_id,
    };
    // Create a new network encryption key and add it to the shared state.
    self.dwallet_network_encryption_keys.add(dwallet_network_encryption_key_id, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKey">DWalletNetworkEncryptionKey</a> {
        id,
        dwallet_network_encryption_key_cap_id: object::id(&cap),
        current_epoch: self.current_epoch,
        reconfiguration_public_outputs: <a href="../sui/table.md#sui_table_new">sui::table::new</a>(ctx),
        network_dkg_public_output: table_vec::empty(ctx),
        computation_fee_charged_ika: balance::zero(),
        state: DWalletNetworkEncryptionKeyState::AwaitingNetworkDKG,
    });
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_initiate_system_dwallet_session">initiate_system_dwallet_session</a>(
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDKGEncryptionKeyRequestEvent">DWalletNetworkDKGEncryptionKeyRequestEvent</a> {
            dwallet_network_encryption_key_id
        },
        ctx,
    );
    // Return the capability.
    cap
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_gas_fee_reimbursement_sui_for_system_calls"></a>

## Function `charge_gas_fee_reimbursement_sui_for_system_calls`



<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_gas_fee_reimbursement_sui_for_system_calls">charge_gas_fee_reimbursement_sui_for_system_calls</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_gas_fee_reimbursement_sui_for_system_calls">charge_gas_fee_reimbursement_sui_for_system_calls</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
): Balance&lt;SUI&gt; {
    <b>let</b> gas_fee_reimbursement_sui_value = self.pricing_and_fee_management.gas_fee_reimbursement_sui.value();
    <b>let</b> gas_fee_reimbursement_sui_system_call_value = self.pricing_and_fee_management.gas_fee_reimbursement_sui_system_call_value;
    <b>if</b>(gas_fee_reimbursement_sui_value &gt; 0 && gas_fee_reimbursement_sui_system_call_value &gt; 0) {
        <b>if</b>(gas_fee_reimbursement_sui_value &gt; gas_fee_reimbursement_sui_system_call_value) {
            self.pricing_and_fee_management.gas_fee_reimbursement_sui.split(gas_fee_reimbursement_sui_system_call_value)
        } <b>else</b> {
            self.pricing_and_fee_management.gas_fee_reimbursement_sui.split(gas_fee_reimbursement_sui_value)
        }
    } <b>else</b> {
        balance::zero()
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_network_encryption_key_dkg"></a>

## Function `respond_dwallet_network_encryption_key_dkg`

Complete the Distributed Key Generation (DKG) session
and store the public output corresponding to the newly created network (threshold) encryption key.

Note: assumes the public output is divided into chunks and each <code>network_public_output_chunk</code> is delivered in order,
with <code>is_last_chunk</code> set for the last call.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_network_encryption_key_dkg">respond_dwallet_network_encryption_key_dkg</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_network_encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, network_public_output_chunk: vector&lt;u8&gt;, is_last_chunk: bool, rejected: bool, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_network_encryption_key_dkg">respond_dwallet_network_encryption_key_dkg</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_network_encryption_key_id: ID,
    network_public_output_chunk: vector&lt;u8&gt;,
    is_last_chunk: bool,
    rejected: bool,
    ctx: &<b>mut</b> TxContext,
): Balance&lt;SUI&gt; {
    <b>if</b> (is_last_chunk) {
        self.session_management.completed_system_sessions_count = self.session_management.completed_system_sessions_count + 1;
    };
    <b>let</b> dwallet_network_encryption_key = self.dwallet_network_encryption_keys.borrow_mut(
        dwallet_network_encryption_key_id
    );
    <b>if</b> (rejected) {
        dwallet_network_encryption_key.state = DWalletNetworkEncryptionKeyState::AwaitingNetworkDKG;
        // TODO(@scaly): should we empty dwallet_network_encryption_key.network_dkg_public_output?
        event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletNetworkDKGEncryptionKeyEvent">RejectedDWalletNetworkDKGEncryptionKeyEvent</a> {
            dwallet_network_encryption_key_id,
        });
        self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_initiate_system_dwallet_session">initiate_system_dwallet_session</a>(
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDKGEncryptionKeyRequestEvent">DWalletNetworkDKGEncryptionKeyRequestEvent</a> {
                dwallet_network_encryption_key_id,
            },
            ctx,
        );
    } <b>else</b> {
        dwallet_network_encryption_key.network_dkg_public_output.push_back(network_public_output_chunk);
        dwallet_network_encryption_key.state = match (&dwallet_network_encryption_key.state) {
            DWalletNetworkEncryptionKeyState::AwaitingNetworkDKG =&gt; {
            <b>if</b> (is_last_chunk) {
                event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletNetworkDKGEncryptionKeyEvent">CompletedDWalletNetworkDKGEncryptionKeyEvent</a> {
                    dwallet_network_encryption_key_id,
                });
                DWalletNetworkEncryptionKeyState::NetworkDKGCompleted
            } <b>else</b> {
                DWalletNetworkEncryptionKeyState::AwaitingNetworkDKG
            }
        },
            _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongState">EWrongState</a>
        };
    };
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_gas_fee_reimbursement_sui_for_system_calls">charge_gas_fee_reimbursement_sui_for_system_calls</a>()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_network_encryption_key_reconfiguration"></a>

## Function `respond_dwallet_network_encryption_key_reconfiguration`

Complete the Reconfiguration session
and store the public output corresponding to the reconfigured network (threshold) encryption key.

Note: assumes the public output is divided into chunks and each <code>network_public_output_chunk</code> is delivered in order,
with <code>is_last_chunk</code> set for the last call.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_network_encryption_key_reconfiguration">respond_dwallet_network_encryption_key_reconfiguration</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_network_encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, public_output: vector&lt;u8&gt;, is_last_chunk: bool, rejected: bool, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_network_encryption_key_reconfiguration">respond_dwallet_network_encryption_key_reconfiguration</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_network_encryption_key_id: ID,
    public_output: vector&lt;u8&gt;,
    is_last_chunk: bool,
    rejected: bool,
    ctx: &<b>mut</b> TxContext,
): Balance&lt;SUI&gt; {
    // The Reconfiguration output can be large, so it is seperated into chunks.
    // We should only update the count once, so we check it is the last chunk before we do.
    <b>if</b> (is_last_chunk) {
        self.session_management.completed_system_sessions_count = self.session_management.completed_system_sessions_count + 1;
    };
    // Store this chunk <b>as</b> the last chunk in the chunks vector corresponding to the upcoming's epoch in the <b>public</b> outputs map.
    <b>let</b> dwallet_network_encryption_key = self.dwallet_network_encryption_keys.borrow_mut(dwallet_network_encryption_key_id);
    <b>if</b> (rejected) {
        dwallet_network_encryption_key.state = match (&dwallet_network_encryption_key.state) {
            DWalletNetworkEncryptionKeyState::AwaitingNetworkReconfiguration { is_first } =&gt; {
                DWalletNetworkEncryptionKeyState::AwaitingNetworkReconfiguration { is_first: *is_first }
            },
            _ =&gt; DWalletNetworkEncryptionKeyState::AwaitingNetworkReconfiguration { is_first: <b>false</b> }
        };
        // TODO(@scaly): should we empty next_reconfiguration_public_output?
        event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletEncryptionKeyReconfigurationEvent">RejectedDWalletEncryptionKeyReconfigurationEvent</a> {
            dwallet_network_encryption_key_id,
        });
        self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_initiate_system_dwallet_session">initiate_system_dwallet_session</a>(
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletEncryptionKeyReconfigurationRequestEvent">DWalletEncryptionKeyReconfigurationRequestEvent</a> {
                dwallet_network_encryption_key_id,
            },
            ctx,
        );
    } <b>else</b> {
        <b>let</b> next_reconfiguration_public_output = dwallet_network_encryption_key.reconfiguration_public_outputs.borrow_mut(dwallet_network_encryption_key.current_epoch + 1);
        // Change state to complete and emit an event to signify that only <b>if</b> it is the last chunk.
        next_reconfiguration_public_output.push_back(public_output);
        dwallet_network_encryption_key.state = match (&dwallet_network_encryption_key.state) {
            DWalletNetworkEncryptionKeyState::AwaitingNetworkReconfiguration { is_first } =&gt; {
                <b>if</b> (is_last_chunk) {
                        event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletEncryptionKeyReconfigurationEvent">CompletedDWalletEncryptionKeyReconfigurationEvent</a> {
                            dwallet_network_encryption_key_id,
                        });
                        DWalletNetworkEncryptionKeyState::AwaitingNextEpochToUpdateReconfiguration
                    } <b>else</b> {
                        DWalletNetworkEncryptionKeyState::AwaitingNetworkReconfiguration { is_first: *is_first }
                    }
                },
            _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongState">EWrongState</a>
        };
    };
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_gas_fee_reimbursement_sui_for_system_calls">charge_gas_fee_reimbursement_sui_for_system_calls</a>()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_advance_epoch_dwallet_network_encryption_key"></a>

## Function `advance_epoch_dwallet_network_encryption_key`

Advance the <code>current_epoch</code> and <code>state</code> of the network encryption key corresponding to <code>cap</code>,
finalizing the reconfiguration of that key, and readying it for use in the next epoch.


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_advance_epoch_dwallet_network_encryption_key">advance_epoch_dwallet_network_encryption_key</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, cap: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyCap">dwallet_2pc_mpc_coordinator_inner::DWalletNetworkEncryptionKeyCap</a>): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_advance_epoch_dwallet_network_encryption_key">advance_epoch_dwallet_network_encryption_key</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    cap: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyCap">DWalletNetworkEncryptionKeyCap</a>,
): Balance&lt;IKA&gt; {
    // Get the corresponding network encryption key.
    <b>let</b> dwallet_network_encryption_key = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_network_encryption_key">get_active_dwallet_network_encryption_key</a>(
        cap.dwallet_network_encryption_key_id
    );
    // Sanity checks: check the capability is the right one, and that the key is in the right state.
    <b>assert</b>!(dwallet_network_encryption_key.dwallet_network_encryption_key_cap_id == cap.id.to_inner(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EIncorrectCap">EIncorrectCap</a>);
    <b>assert</b>!(dwallet_network_encryption_key.state == DWalletNetworkEncryptionKeyState::AwaitingNextEpochToUpdateReconfiguration, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongState">EWrongState</a>);
    // Advance the current epoch and state.
    dwallet_network_encryption_key.current_epoch = dwallet_network_encryption_key.current_epoch + 1;
    dwallet_network_encryption_key.state = DWalletNetworkEncryptionKeyState::NetworkReconfigurationCompleted;
    // Return the fees.
    <b>let</b> <b>mut</b> epoch_computation_fee_charged_ika = <a href="../sui/balance.md#sui_balance_zero">sui::balance::zero</a>&lt;IKA&gt;();
    epoch_computation_fee_charged_ika.join(dwallet_network_encryption_key.computation_fee_charged_ika.withdraw_all());
    <b>return</b> epoch_computation_fee_charged_ika
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_mid_epoch_reconfiguration"></a>

## Function `mid_epoch_reconfiguration`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_mid_epoch_reconfiguration">mid_epoch_reconfiguration</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, next_epoch_active_committee: (ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">bls_committee::BlsCommittee</a>, dwallet_network_encryption_key_caps: &vector&lt;(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyCap">dwallet_2pc_mpc_coordinator_inner::DWalletNetworkEncryptionKeyCap</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_mid_epoch_reconfiguration">mid_epoch_reconfiguration</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    next_epoch_active_committee: BlsCommittee,
    dwallet_network_encryption_key_caps: &vector&lt;<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyCap">DWalletNetworkEncryptionKeyCap</a>&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> pricing_calculation_votes = <a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_new_pricing_calculation">dwallet_pricing::new_pricing_calculation</a>(next_epoch_active_committee, self.pricing_and_fee_management.default);
    self.pricing_and_fee_management.calculation_votes = option::some(pricing_calculation_votes);
    dwallet_network_encryption_key_caps.do_ref!(|cap| self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_emit_start_reconfiguration_event">emit_start_reconfiguration_event</a>(cap, ctx));
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_calculate_pricing_votes"></a>

## Function `calculate_pricing_votes`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_calculate_pricing_votes">calculate_pricing_votes</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, curve: u32, signature_algorithm: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u32&gt;, protocol: u32)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_calculate_pricing_votes">calculate_pricing_votes</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    curve: u32,
    signature_algorithm: Option&lt;u32&gt;,
    protocol: u32,
) {
    <b>let</b> pricing_and_fee_management = &<b>mut</b> self.pricing_and_fee_management;
    <b>assert</b>!(pricing_and_fee_management.calculation_votes.is_some(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EPricingCalculationVotesHasNotBeenStarted">EPricingCalculationVotesHasNotBeenStarted</a>);
    <b>let</b> pricing_calculation_votes = pricing_and_fee_management.calculation_votes.borrow_mut();
    <b>let</b> pricing_votes = pricing_calculation_votes.committee_members_for_pricing_calculation_votes().map!(|id| {
        <b>if</b> (pricing_and_fee_management.validator_votes.contains(id)) {
            pricing_and_fee_management.validator_votes[id]
        } <b>else</b> {
            pricing_and_fee_management.default
        }
    });
    pricing_calculation_votes.calculate_pricing_quorum_below(pricing_votes, curve, signature_algorithm, protocol);
    <b>if</b>(pricing_calculation_votes.is_calculation_completed()) {
        pricing_and_fee_management.current = pricing_calculation_votes.calculated_pricing();
        pricing_and_fee_management.calculation_votes = option::none();
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_emit_start_reconfiguration_event"></a>

## Function `emit_start_reconfiguration_event`

Emit an event to the Ika network to request a reconfiguration session for the network encryption key corresponding to <code>cap</code>.


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_emit_start_reconfiguration_event">emit_start_reconfiguration_event</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, cap: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyCap">dwallet_2pc_mpc_coordinator_inner::DWalletNetworkEncryptionKeyCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_emit_start_reconfiguration_event">emit_start_reconfiguration_event</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>, cap: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyCap">DWalletNetworkEncryptionKeyCap</a>, ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(self.dwallet_network_encryption_keys.contains(cap.dwallet_network_encryption_key_id), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkEncryptionKeyNotExist">EDWalletNetworkEncryptionKeyNotExist</a>);
    <b>let</b> dwallet_network_encryption_key = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_network_encryption_key">get_active_dwallet_network_encryption_key</a>(cap.dwallet_network_encryption_key_id);
    dwallet_network_encryption_key.state = match (&dwallet_network_encryption_key.state) {
        DWalletNetworkEncryptionKeyState::NetworkDKGCompleted =&gt; {
            DWalletNetworkEncryptionKeyState::AwaitingNetworkReconfiguration { is_first: <b>true</b> }
        },
        DWalletNetworkEncryptionKeyState::NetworkReconfigurationCompleted =&gt; {
            DWalletNetworkEncryptionKeyState::AwaitingNetworkReconfiguration { is_first: <b>false</b> }
        },
        _ =&gt; <b>return</b>, // TODO(@scaly): should not happen, what do you think?
    };
    // Initialize the chunks vector corresponding to the upcoming's epoch in the <b>public</b> outputs map.
    dwallet_network_encryption_key.reconfiguration_public_outputs.add(dwallet_network_encryption_key.current_epoch + 1, table_vec::empty(ctx));
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_initiate_system_dwallet_session">initiate_system_dwallet_session</a>(
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletEncryptionKeyReconfigurationRequestEvent">DWalletEncryptionKeyReconfigurationRequestEvent</a> {
            dwallet_network_encryption_key_id: cap.dwallet_network_encryption_key_id
        },
        ctx,
    );
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_network_encryption_key"></a>

## Function `get_active_dwallet_network_encryption_key`



<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_network_encryption_key">get_active_dwallet_network_encryption_key</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_network_encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKey">dwallet_2pc_mpc_coordinator_inner::DWalletNetworkEncryptionKey</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_network_encryption_key">get_active_dwallet_network_encryption_key</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_network_encryption_key_id: ID,
): &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKey">DWalletNetworkEncryptionKey</a> {
    <b>let</b> dwallet_network_encryption_key = self.dwallet_network_encryption_keys.borrow_mut(dwallet_network_encryption_key_id);
    <b>assert</b>!(dwallet_network_encryption_key.state != DWalletNetworkEncryptionKeyState::AwaitingNetworkDKG, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkEncryptionKeyNotActive">EDWalletNetworkEncryptionKeyNotActive</a>);
    dwallet_network_encryption_key
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_advance_epoch"></a>

## Function `advance_epoch`

Advance the epoch.

Checks that all the current epoch sessions are completed,
and updates the required metadata for the next epoch's sessions management.

Sets the current and previous committees.

Unlocks and updates <code>last_user_initiated_session_to_complete_in_current_epoch</code>.

And finally increments the <code>current_epoch</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_advance_epoch">advance_epoch</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, next_committee: (ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">bls_committee::BlsCommittee</a>, dwallet_network_encryption_key_caps: &vector&lt;(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyCap">dwallet_2pc_mpc_coordinator_inner::DWalletNetworkEncryptionKeyCap</a>&gt;): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_advance_epoch">advance_epoch</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    next_committee: BlsCommittee,
    dwallet_network_encryption_key_caps: &vector&lt;<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyCap">DWalletNetworkEncryptionKeyCap</a>&gt;,
): Balance&lt;IKA&gt; {
    <b>assert</b>!(self.pricing_and_fee_management.calculation_votes.is_none(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EPricingCalculationVotesMustBeCompleted">EPricingCalculationVotesMustBeCompleted</a>);
    <b>assert</b>!(self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_all_current_epoch_user_initiated_sessions_completed">all_current_epoch_user_initiated_sessions_completed</a>(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ECannotAdvanceEpoch">ECannotAdvanceEpoch</a>);
    <b>if</b> (self.last_processed_checkpoint_sequence_number.is_some()) {
        <b>let</b> last_processed_checkpoint_sequence_number = *self.last_processed_checkpoint_sequence_number.borrow();
        self.previous_epoch_last_checkpoint_sequence_number = last_processed_checkpoint_sequence_number;
    };
    self.session_management.locked_last_user_initiated_session_to_complete_in_current_epoch = <b>false</b>;
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_update_last_user_initiated_session_to_complete_in_current_epoch">update_last_user_initiated_session_to_complete_in_current_epoch</a>();
    self.current_epoch = self.current_epoch + 1;
    self.previous_committee = self.active_committee;
    self.active_committee = next_committee;
    <b>let</b> <b>mut</b> balance = balance::zero&lt;IKA&gt;();
    dwallet_network_encryption_key_caps.do_ref!(|cap| {
        balance.join(self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_advance_epoch_dwallet_network_encryption_key">advance_epoch_dwallet_network_encryption_key</a>(cap));
    });
    balance.join(self.pricing_and_fee_management.consensus_validation_fee_charged_ika.withdraw_all());
    balance
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
        DWalletState::NetworkRejectedImportedKeyVerification |
        DWalletState::AwaitingKeyHolderSignature { .. } =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletInactive">EDWalletInactive</a>,
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event"></a>

## Function `charge_and_create_current_epoch_dwallet_event`

Creates a new MPC session and charges the user for it.

Payment is done in both Ika (for the MPC computation by the Ika network)
and Sui (for storing the public output in Sui).
The payment is saved in the session object, for it is to be distributed only upon the completion of the session.

The newly created session has its sequence number set to <code>next_session_sequence_number</code>, which is then incremented.
Finally, the last session to complete in current epoch is updated, if needed.


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>&lt;E: <b>copy</b>, drop, store&gt;(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_network_encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, pricing_value: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricingValue">dwallet_pricing::DWalletPricingValue</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, event_data: E, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletEvent">dwallet_2pc_mpc_coordinator_inner::DWalletEvent</a>&lt;E&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>&lt;E: <b>copy</b> + drop + store&gt;(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_network_encryption_key_id: ID,
    pricing_value: DWalletPricingValue,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    event_data: E,
    ctx: &<b>mut</b> TxContext,
): <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletEvent">DWalletEvent</a>&lt;E&gt; {
    <b>assert</b>!(self.dwallet_network_encryption_keys.contains(dwallet_network_encryption_key_id), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkEncryptionKeyNotExist">EDWalletNetworkEncryptionKeyNotExist</a>);
    <b>let</b> computation_fee_charged_ika = payment_ika.split(pricing_value.computation_ika(), ctx).into_balance();
    <b>let</b> consensus_validation_fee_charged_ika = payment_ika.split(pricing_value.consensus_validation_ika(), ctx).into_balance();
    <b>let</b> gas_fee_reimbursement_sui = payment_sui.split(pricing_value.gas_fee_reimbursement_sui(), ctx).into_balance();
    self.pricing_and_fee_management.gas_fee_reimbursement_sui.join(payment_sui.split(pricing_value.gas_fee_reimbursement_sui_for_system_calls(), ctx).into_balance());
    <b>let</b> session_sequence_number = self.session_management.next_session_sequence_number;
    <b>let</b> session = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSession">DWalletSession</a> {
        id: object::new(ctx),
        session_sequence_number,
        dwallet_network_encryption_key_id,
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
    self.session_management.user_requested_sessions_events.add(session.id.to_inner(), event);
    self.session_management.sessions.add(session_sequence_number, session);
    self.session_management.next_session_sequence_number = session_sequence_number + 1;
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_update_last_user_initiated_session_to_complete_in_current_epoch">update_last_user_initiated_session_to_complete_in_current_epoch</a>();
    event
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_initiate_system_dwallet_session"></a>

## Function `initiate_system_dwallet_session`

Initiate a new MPC session that serves the system (i.e. the Ika network).
The current protocols that are supported for such is network DKG and Reconfiguration,
both of which are related to a particular <code>dwallet_network_encryption_key_id</code>.
No funds are charged, since there is no user to charge.


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_initiate_system_dwallet_session">initiate_system_dwallet_session</a>&lt;E: <b>copy</b>, drop, store&gt;(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, event_data: E, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_initiate_system_dwallet_session">initiate_system_dwallet_session</a>&lt;E: <b>copy</b> + drop + store&gt;(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    event_data: E,
    ctx: &<b>mut</b> TxContext,
) {
    self.session_management.started_system_sessions_count = self.session_management.started_system_sessions_count + 1;
    <b>let</b> event = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletEvent">DWalletEvent</a> {
        epoch: self.current_epoch,
        session_type: SessionType::System,
        session_id: object::id_from_address(tx_context::fresh_object_address(ctx)),
        event_data,
    };
    event::emit(event);
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
    <b>assert</b>!(self.encryption_keys.contains(<b>address</b>), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EEncryptionKeyNotExist">EEncryptionKeyNotExist</a>);
    self.encryption_keys.borrow(<b>address</b>).id.to_inner()
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_curve"></a>

## Function `validate_curve`

Validates the <code>curve</code> selection is both supported, and not paused.


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_curve">validate_curve</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, curve: u32)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_curve">validate_curve</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    curve: u32,
) {
    <b>assert</b>!(self.support_config.supported_curves_to_signature_algorithms_to_hash_schemes.contains(&curve), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidCurve">EInvalidCurve</a>);
    <b>assert</b>!(!self.support_config.paused_curves.contains(&curve), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ECurvePaused">ECurvePaused</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_curve_and_signature_algorithm"></a>

## Function `validate_curve_and_signature_algorithm`

Validates the <code>curve</code> and <code>signature_algorithm</code> selection is supported, and not paused.


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_curve_and_signature_algorithm">validate_curve_and_signature_algorithm</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, curve: u32, signature_algorithm: u32)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_curve_and_signature_algorithm">validate_curve_and_signature_algorithm</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    curve: u32,
    signature_algorithm: u32,
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_curve">validate_curve</a>(curve);
    <b>let</b> supported_curve_to_signature_algorithms = self.support_config.supported_curves_to_signature_algorithms_to_hash_schemes[&curve];
    <b>assert</b>!(supported_curve_to_signature_algorithms.contains(&signature_algorithm), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidSignatureAlgorithm">EInvalidSignatureAlgorithm</a>);
    <b>assert</b>!(!self.support_config.paused_signature_algorithms.contains(&signature_algorithm), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ESignatureAlgorithmPaused">ESignatureAlgorithmPaused</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_curve_and_signature_algorithm_and_hash_scheme"></a>

## Function `validate_curve_and_signature_algorithm_and_hash_scheme`

Validates the <code>curve</code>, <code>signature_algorithm</code> and <code>hash_scheme</code> selection is supported, and not paused.


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_curve_and_signature_algorithm_and_hash_scheme">validate_curve_and_signature_algorithm_and_hash_scheme</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, curve: u32, signature_algorithm: u32, hash_scheme: u32)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_curve_and_signature_algorithm_and_hash_scheme">validate_curve_and_signature_algorithm_and_hash_scheme</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    curve: u32,
    signature_algorithm: u32,
    hash_scheme: u32,
) {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_curve_and_signature_algorithm">validate_curve_and_signature_algorithm</a>(curve, signature_algorithm);
    <b>let</b> supported_hash_schemes = self.support_config.supported_curves_to_signature_algorithms_to_hash_schemes[&curve][&signature_algorithm];
    <b>assert</b>!(supported_hash_schemes.contains(&hash_scheme), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidHashScheme">EInvalidHashScheme</a>);
    <b>assert</b>!(!self.support_config.paused_hash_schemes.contains(&hash_scheme), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EHashSchemePaused">EHashSchemePaused</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_register_encryption_key"></a>

## Function `register_encryption_key`

Registers an encryption key to be used later for encrypting a
centralized secret key share.


<a name="@Parameters_3"></a>

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
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_curve">validate_curve</a>(curve);
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

Approves <code>message</code> to be signed by the dWallet corresponding to <code>dwallet_cap</code>.
Binds the approval for a specific <code>signature_algorithm</code> and <code>hash_scheme</code> choice.


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

Approves <code>message</code> to be signed by the imported key dWallet corresponding to <code>imported_key_dwallet_cap</code>.
Binds the approval for a specific <code>signature_algorithm</code> and <code>hash_scheme</code> choice.


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

Perform shared validation for both the dWallet and imported key dWallet's variants of <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_approve_message">approve_message</a>()</code>.
Verify the <code>curve</code>, <code>signature_algorithm</code> and <code>hash_scheme</code> choice, and that the dWallet exists.
Returns whether this is an imported key dWallet, to be verified by the caller.


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
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_curve_and_signature_algorithm_and_hash_scheme">validate_curve_and_signature_algorithm_and_hash_scheme</a>(dwallet.curve, signature_algorithm, hash_scheme);
    dwallet.is_imported_key_dwallet
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_dkg_first_round"></a>

## Function `request_dwallet_dkg_first_round`

Starts the first Distributed Key Generation (DKG) session.

This function creates a new <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">DWalletCap</a></code> object,
transfers it to the session initiator (the user),
and emits a <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGFirstRoundRequestEvent">DWalletDKGFirstRoundRequestEvent</a></code> to signal
the beginning of the DKG process.


<a name="@Parameters_4"></a>

##### Parameters



<a name="@Effects_5"></a>

##### Effects

- Generates a new <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">DWalletCap</a></code> object.
- Transfers the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">DWalletCap</a></code> to the session initiator (<code>ctx.sender</code>).
- Creates a new <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a></code> object and inserts it into the <code>dwallets</code> map.
- Emits a <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGFirstRoundRequestEvent">DWalletDKGFirstRoundRequestEvent</a></code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_dkg_first_round">request_dwallet_dkg_first_round</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_network_encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, curve: u32, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">dwallet_2pc_mpc_coordinator_inner::DWalletCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_dkg_first_round">request_dwallet_dkg_first_round</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_network_encryption_key_id: ID,
    curve: u32,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">DWalletCap</a> {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_curve">validate_curve</a>(curve);
    <b>let</b> <b>mut</b> pricing_value = self.pricing_and_fee_management.default.try_get_dwallet_pricing_value(curve, option::none(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DKG_FIRST_ROUND_PROTOCOL_FLAG">DKG_FIRST_ROUND_PROTOCOL_FLAG</a>);
    <b>assert</b>!(pricing_value.is_some(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMissingProtocolPricing">EMissingProtocolPricing</a>);
    // TODO(@Omer): check the state of the dWallet (i.e., not waiting <b>for</b> dkg.)
    // TODO(@Omer): I believe the best thing would be to always <b>use</b> the latest key. I'm not sure why the user should even supply the id.
    <b>assert</b>!(self.dwallet_network_encryption_keys.contains(dwallet_network_encryption_key_id), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkEncryptionKeyNotExist">EDWalletNetworkEncryptionKeyNotExist</a>);
    // Create a new `<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">DWalletCap</a>` object.
    <b>let</b> id = object::new(ctx);
    <b>let</b> dwallet_id = id.to_inner();
    <b>let</b> dwallet_cap = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">DWalletCap</a> {
        id: object::new(ctx),
        dwallet_id,
    };
    <b>let</b> dwallet_cap_id = object::id(&dwallet_cap);
    // Create a new `<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a>` object,
    // link it to the `dwallet_cap` we just created by id,
    // and insert it into the `dwallets` map.
    self.dwallets.add(dwallet_id, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a> {
        id,
        created_at_epoch: self.current_epoch,
        curve,
        public_user_secret_key_share: option::none(),
        dwallet_cap_id,
        dwallet_network_encryption_key_id,
        is_imported_key_dwallet: <b>false</b>,
        encrypted_user_secret_key_shares: object_table::new(ctx),
        sign_sessions: object_table::new(ctx),
        state: DWalletState::DKGRequested,
    });
    // Emit an event to request the Ika network to start DKG <b>for</b> this dWallet.
    event::emit(self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
                dwallet_network_encryption_key_id,
        pricing_value.extract(),
        payment_ika,
        payment_sui,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGFirstRoundRequestEvent">DWalletDKGFirstRoundRequestEvent</a> {
            dwallet_id,
            dwallet_cap_id,
            dwallet_network_encryption_key_id,
            curve,
        },
        ctx,
    ));
    dwallet_cap
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_update_last_user_initiated_session_to_complete_in_current_epoch"></a>

## Function `update_last_user_initiated_session_to_complete_in_current_epoch`

Updates the <code>last_user_initiated_session_to_complete_in_current_epoch</code> field:
- If we already locked this field, we do nothing.
- Otherwise, we take the latest session whilst assuring
a maximum of <code>max_active_sessions_buffer</code> sessions to be completed in the current epoch.


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_update_last_user_initiated_session_to_complete_in_current_epoch">update_last_user_initiated_session_to_complete_in_current_epoch</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_update_last_user_initiated_session_to_complete_in_current_epoch">update_last_user_initiated_session_to_complete_in_current_epoch</a>(self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>) {
    <b>if</b> (self.session_management.locked_last_user_initiated_session_to_complete_in_current_epoch) {
        <b>return</b>
    };
    <b>let</b> new_last_user_initiated_session_to_complete_in_current_epoch = (
        self.session_management.number_of_completed_user_initiated_sessions + self.session_management.max_active_sessions_buffer
    ).min(
        self.session_management.next_session_sequence_number - 1
    );
    // Sanity check: only update this field <b>if</b> we need to.
    <b>if</b> (self.session_management.last_user_initiated_session_to_complete_in_current_epoch &gt;= new_last_user_initiated_session_to_complete_in_current_epoch) {
        <b>return</b>
    };
    self.session_management.last_user_initiated_session_to_complete_in_current_epoch = new_last_user_initiated_session_to_complete_in_current_epoch;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_all_current_epoch_user_initiated_sessions_completed"></a>

## Function `all_current_epoch_user_initiated_sessions_completed`

Check whether all the user-initiated session that should complete in the current epoch are in fact completed.
This check is only relevant after <code>last_user_initiated_session_to_complete_in_current_epoch</code> is locked, and is called
as a requirement to advance the epoch.
Session sequence numbers are sequential, so ch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_all_current_epoch_user_initiated_sessions_completed">all_current_epoch_user_initiated_sessions_completed</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_all_current_epoch_user_initiated_sessions_completed">all_current_epoch_user_initiated_sessions_completed</a>(self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>): bool {
    <b>return</b> (self.session_management.locked_last_user_initiated_session_to_complete_in_current_epoch &&
        (self.session_management.number_of_completed_user_initiated_sessions == self.session_management.last_user_initiated_session_to_complete_in_current_epoch) &&
        (self.session_management.completed_system_sessions_count == self.session_management.started_system_sessions_count))
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_user_initiated_session_and_charge"></a>

## Function `remove_user_initiated_session_and_charge`

Removes a user-initiated session and its corresponding event, charging the pre-paid gas amounts in both Sui and Ika
to be later distributed as part of the consensus validation and gas reimbursement fees.

Increments <code>number_of_completed_user_initiated_sessions</code>.

Notice: never called for a system session.


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_user_initiated_session_and_charge">remove_user_initiated_session_and_charge</a>&lt;E: <b>copy</b>, drop, store&gt;(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, session_sequence_number: u64): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_user_initiated_session_and_charge">remove_user_initiated_session_and_charge</a>&lt;E: <b>copy</b> + drop + store&gt;(self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>, session_sequence_number: u64): Balance&lt;SUI&gt; {
    self.session_management.number_of_completed_user_initiated_sessions = self.session_management.number_of_completed_user_initiated_sessions + 1;
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_update_last_user_initiated_session_to_complete_in_current_epoch">update_last_user_initiated_session_to_complete_in_current_epoch</a>();
    <b>let</b> session = self.session_management.sessions.remove(session_sequence_number);
    // Unpack and delete the `<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSession">DWalletSession</a>` object.
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSession">DWalletSession</a> {
        computation_fee_charged_ika,
        gas_fee_reimbursement_sui,
        consensus_validation_fee_charged_ika,
        dwallet_network_encryption_key_id,
        id,
        ..
    } = session;
    // Remove the corresponding event.
    <b>let</b> dwallet_network_encryption_key = self.dwallet_network_encryption_keys.borrow_mut(dwallet_network_encryption_key_id);
    <b>let</b> _: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletEvent">DWalletEvent</a>&lt;E&gt; = self.session_management.user_requested_sessions_events.remove(id.to_inner());
    object::delete(id);
    dwallet_network_encryption_key.computation_fee_charged_ika.join(computation_fee_charged_ika);
    self.pricing_and_fee_management.consensus_validation_fee_charged_ika.join(consensus_validation_fee_charged_ika);
    //self.gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
    gas_fee_reimbursement_sui
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_dkg_first_round"></a>

## Function `respond_dwallet_dkg_first_round`

This function is called by the Ika network to respond to the dWallet DKG first round request made by the user.
Advances the dWallet's state and registers the output in it.
Also emits an event with the output.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_dkg_first_round">respond_dwallet_dkg_first_round</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, first_round_output: vector&lt;u8&gt;, rejected: bool, session_sequence_number: u64): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_dkg_first_round">respond_dwallet_dkg_first_round</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
    first_round_output: vector&lt;u8&gt;,
    rejected: bool,
    session_sequence_number: u64,
): Balance&lt;SUI&gt; {
    <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_user_initiated_session_and_charge">remove_user_initiated_session_and_charge</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGFirstRoundRequestEvent">DWalletDKGFirstRoundRequestEvent</a>&gt;(session_sequence_number);
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut">get_dwallet_mut</a>(dwallet_id);
    dwallet.state = match (dwallet.state) {
        DWalletState::DKGRequested =&gt; {
            <b>if</b> (rejected) {
                event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletDKGFirstRoundEvent">RejectedDWalletDKGFirstRoundEvent</a> {
                    dwallet_id,
                });
                DWalletState::NetworkRejectedDKGRequest
            } <b>else</b> {
                event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletDKGFirstRoundEvent">CompletedDWalletDKGFirstRoundEvent</a> {
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
    gas_fee_reimbursement_sui
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_dkg_second_round"></a>

## Function `request_dwallet_dkg_second_round`

Initiates the second round of the Distributed Key Generation (DKG) protocol
by emitting an event for the Ika validators to request the execution of this round.

Creates a new <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a></code> object, with the state awaiting the network verification
that the user encrypted its user share correctly (the network will verify it as part of the second round).

Sets the state of the dWallet to <code>AwaitingNetworkDKGVerification</code>.


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
    <b>let</b> dwallet_id = dwallet_cap.dwallet_id;
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet">get_dwallet</a>(dwallet_id);
    <b>let</b> curve = dwallet.curve;
    <b>assert</b>!(!dwallet.is_imported_key_dwallet, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EImportedKeyDWallet">EImportedKeyDWallet</a>);
    <b>assert</b>!(encryption_key_curve == curve, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMismatchCurve">EMismatchCurve</a>);
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_curve">validate_curve</a>(curve);
    <b>let</b> first_round_output = match (&dwallet.state) {
        DWalletState::AwaitingUserDKGVerificationInitiation {
            first_round_output,
        } =&gt; {
            *first_round_output
        },
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongState">EWrongState</a>
    };
    <b>let</b> dwallet_network_encryption_key_id = dwallet.dwallet_network_encryption_key_id;
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
    <b>let</b> <b>mut</b> pricing_value = self.pricing_and_fee_management.current.try_get_dwallet_pricing_value(curve, option::none(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DKG_SECOND_ROUND_PROTOCOL_FLAG">DKG_SECOND_ROUND_PROTOCOL_FLAG</a>);
    <b>assert</b>!(pricing_value.is_some(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMissingProtocolPricing">EMissingProtocolPricing</a>);
    <b>let</b> emit_event = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
        dwallet_network_encryption_key_id,
        pricing_value.extract(),
        payment_ika,
        payment_sui,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGSecondRoundRequestEvent">DWalletDKGSecondRoundRequestEvent</a> {
            encrypted_user_secret_key_share_id,
            dwallet_id,
            first_round_output,
            centralized_public_key_share_and_proof,
            dwallet_cap_id: object::id(dwallet_cap),
            encrypted_centralized_secret_share_and_proof,
            encryption_key,
            encryption_key_id,
            encryption_key_address,
            user_public_output,
            signer_public_key,
            dwallet_network_encryption_key_id,
            curve,
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

This function is called by the Ika network to respond to the dWallet DKG second round request made by the user.

Completes the second round of the Distributed Key Generation (DKG) process and
advances the [<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a></code>] state to <code>AwaitingKeyHolderSignature</code> with the DKG public output registered in it.

Advances the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShareState">EncryptedUserSecretKeyShareState</a></code> to <code>NetworkVerificationCompleted</code>.

Also emits an event with the public output.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_dkg_second_round">respond_dwallet_dkg_second_round</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, public_output: vector&lt;u8&gt;, encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, session_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, rejected: bool, session_sequence_number: u64): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;
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
): Balance&lt;SUI&gt; {
    <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_user_initiated_session_and_charge">remove_user_initiated_session_and_charge</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGSecondRoundRequestEvent">DWalletDKGSecondRoundRequestEvent</a>&gt;(session_sequence_number);
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
                DWalletState::AwaitingKeyHolderSignature {
                    public_output
                }
            }
        },
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongState">EWrongState</a>
    };
    gas_fee_reimbursement_sui
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_re_encrypt_user_share_for"></a>

## Function `request_re_encrypt_user_share_for`

Requests a re-encryption of the user share of the dWallet by having the Ika network
verify a zk-proof that the encryption matches the public share of the dWallet.

This can be used as part of granting access or transferring the dWallet.

Creates a new <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a></code> object, with the state awaiting the network verification.
Emits an event to request the verification by the network.


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
    <b>let</b> dwallet_network_encryption_key_id = dwallet.dwallet_network_encryption_key_id;
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
    <b>let</b> <b>mut</b> pricing_value = self.pricing_and_fee_management.current.try_get_dwallet_pricing_value(curve, option::none(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RE_ENCRYPT_USER_SHARE_PROTOCOL_FLAG">RE_ENCRYPT_USER_SHARE_PROTOCOL_FLAG</a>);
    <b>assert</b>!(pricing_value.is_some(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMissingProtocolPricing">EMissingProtocolPricing</a>);
    event::emit(
        self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
            dwallet_network_encryption_key_id,
            pricing_value.extract(),
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
                dwallet_network_encryption_key_id,
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

This function is called by the Ika network to respond to a re-encryption request of the user share of the dWallet
by setting the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShareState">EncryptedUserSecretKeyShareState</a></code> object's state according to the verification result.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_re_encrypt_user_share_for">respond_re_encrypt_user_share_for</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, rejected: bool, session_sequence_number: u64): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_re_encrypt_user_share_for">respond_re_encrypt_user_share_for</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
    encrypted_user_secret_key_share_id: ID,
    rejected: bool,
    session_sequence_number: u64
): Balance&lt;SUI&gt; {
    <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_user_initiated_session_and_charge">remove_user_initiated_session_and_charge</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedShareVerificationRequestEvent">EncryptedShareVerificationRequestEvent</a>&gt;(session_sequence_number);
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
    gas_fee_reimbursement_sui
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_accept_encrypted_user_share"></a>

## Function `accept_encrypted_user_share`

Accept the encryption of the user share of a dWallet.

Called after the user verified the signature of the sender (who re-encrypted the user share for them)
on the public output of the dWallet, and that the decrypted share matches the public key share of the dWallet.

Register the user's own signature on the public output <code>user_output_signature</code> for an easy way to perform self-verification in the future.

Finalizes the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShareState">EncryptedUserSecretKeyShareState</a></code> object's state as <code>KeyHolderSigned</code>.


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
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut">get_dwallet_mut</a>(dwallet_id);
    match(&dwallet.state) {
        DWalletState::AwaitingKeyHolderSignature {
            public_output
        } =&gt; {
            dwallet.state = DWalletState::Active {
                public_output: *public_output
            };
        },
        DWalletState::Active { .. } =&gt; { },
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongState">EWrongState</a>
    };
    <b>let</b> public_output = *dwallet.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_active_and_get_public_output">validate_active_and_get_public_output</a>();
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
        EncryptedUserSecretKeyShareState::NetworkVerificationCompleted =&gt; EncryptedUserSecretKeyShareState::KeyHolderSigned {
            user_output_signature
        },
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongState">EWrongState</a>
    };
    event::emit(
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_AcceptEncryptedUserShareEvent">AcceptEncryptedUserShareEvent</a> {
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

Creates a new imported key dWallet, by creating a new <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a></code> object with <code>is_imported_key_dwallet</code> set and the state at <code>AwaitingUserImportedKeyInitiation</code>,
alongside a corresponding <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyDWalletCap">ImportedKeyDWalletCap</a></code>.

Required as a first step before the user can call <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_imported_key_dwallet_verification">request_imported_key_dwallet_verification</a>()</code>,
which requires the user to know the <code>dwallet_id</code> for a unique identifier used by the user to prove the imported key is valid.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_new_imported_key_dwallet">new_imported_key_dwallet</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_network_encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, curve: u32, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyDWalletCap">dwallet_2pc_mpc_coordinator_inner::ImportedKeyDWalletCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_new_imported_key_dwallet">new_imported_key_dwallet</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_network_encryption_key_id: ID,
    curve: u32,
    ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyDWalletCap">ImportedKeyDWalletCap</a> {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_curve">validate_curve</a>(curve);
    <b>assert</b>!(self.dwallet_network_encryption_keys.contains(dwallet_network_encryption_key_id), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkEncryptionKeyNotExist">EDWalletNetworkEncryptionKeyNotExist</a>);
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
        public_user_secret_key_share: option::none(),
        dwallet_cap_id,
        dwallet_network_encryption_key_id,
        is_imported_key_dwallet: <b>true</b>,
        encrypted_user_secret_key_shares: object_table::new(ctx),
        sign_sessions: object_table::new(ctx),
        state: DWalletState::AwaitingUserImportedKeyInitiation,
    });
    event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_NewImportedKeyDWalletEvent">NewImportedKeyDWalletEvent</a> {
        dwallet_id,
        dwallet_cap_id,
    });
    dwallet_cap
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_imported_key_dwallet_verification"></a>

## Function `request_imported_key_dwallet_verification`

Request verification of the imported key dWallet from the Ika network.

Sets the state of the dWallet to <code>AwaitingNetworkImportedKeyVerification</code> and creates a new <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a></code> object, with the state awaiting the network verification
that the user encrypted its user share correctly (the network will verify it as part of the second round).

Emits an event with the user's message and encrypted user share proof to the Ika network.


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
    <b>let</b> created_at_epoch: u64 = self.current_epoch;
    <b>let</b> dwallet_id = dwallet_cap.dwallet_id;
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut">get_dwallet_mut</a>(dwallet_cap.dwallet_id);
    <b>assert</b>!(dwallet.is_imported_key_dwallet, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ENotImportedKeyDWallet">ENotImportedKeyDWallet</a>);
    dwallet.state = match (&dwallet.state) {
        DWalletState::AwaitingUserImportedKeyInitiation =&gt; {
            DWalletState::AwaitingNetworkImportedKeyVerification
        },
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongState">EWrongState</a>
    };
    <b>let</b> dwallet_network_encryption_key_id = dwallet.dwallet_network_encryption_key_id;
    <b>let</b> curve = dwallet.curve;
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
    dwallet.encrypted_user_secret_key_shares.add(encrypted_user_secret_key_share_id, encrypted_user_share);
    <b>let</b> <b>mut</b> pricing_value = self.pricing_and_fee_management.current.try_get_dwallet_pricing_value(curve, option::none(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_IMPORTED_KEY_DWALLET_VERIFICATION_PROTOCOL_FLAG">IMPORTED_KEY_DWALLET_VERIFICATION_PROTOCOL_FLAG</a>);
    <b>assert</b>!(pricing_value.is_some(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMissingProtocolPricing">EMissingProtocolPricing</a>);
    <b>let</b> emit_event = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
        dwallet_network_encryption_key_id,
        pricing_value.extract(),
        payment_ika,
        payment_sui,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletImportedKeyVerificationRequestEvent">DWalletImportedKeyVerificationRequestEvent</a> {
            dwallet_id,
            encrypted_user_secret_key_share_id,
            centralized_party_message,
            dwallet_cap_id: object::id(dwallet_cap),
            encrypted_centralized_secret_share_and_proof,
            encryption_key,
            encryption_key_id,
            encryption_key_address,
            user_public_output,
            signer_public_key,
            dwallet_network_encryption_key_id,
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

This function is called by the Ika network to respond to the import key dWallet verification request made by the user.

Completes the verification of an imported key dWallet and
advances the [<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a></code>] state to <code>AwaitingKeyHolderSignature</code> with the DKG public output registered in it.
Also emits an event with the public output.

Advances the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShareState">EncryptedUserSecretKeyShareState</a></code> to <code>NetworkVerificationCompleted</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_imported_key_dwallet_verification">respond_imported_key_dwallet_verification</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, public_output: vector&lt;u8&gt;, encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, session_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, rejected: bool, session_sequence_number: u64): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_imported_key_dwallet_verification">respond_imported_key_dwallet_verification</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
    public_output: vector&lt;u8&gt;,
    encrypted_user_secret_key_share_id: ID,
    session_id: ID,
    rejected: bool,
    session_sequence_number: u64,
): Balance&lt;SUI&gt; {
    <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_user_initiated_session_and_charge">remove_user_initiated_session_and_charge</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletImportedKeyVerificationRequestEvent">DWalletImportedKeyVerificationRequestEvent</a>&gt;(session_sequence_number);
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut">get_dwallet_mut</a>(dwallet_id);
    dwallet.state = match (&dwallet.state) {
        DWalletState::AwaitingNetworkImportedKeyVerification =&gt; {
            <b>if</b> (rejected) {
                event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletImportedKeyVerificationEvent">RejectedDWalletImportedKeyVerificationEvent</a> {
                    dwallet_id,
                });
                DWalletState::NetworkRejectedImportedKeyVerification
            } <b>else</b> {
                <b>let</b> encrypted_user_share = dwallet.encrypted_user_secret_key_shares.borrow_mut(encrypted_user_secret_key_share_id);
                encrypted_user_share.state = EncryptedUserSecretKeyShareState::NetworkVerificationCompleted;
                event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletImportedKeyVerificationEvent">CompletedDWalletImportedKeyVerificationEvent</a> {
                    dwallet_id,
                    public_output,
                    encrypted_user_secret_key_share_id,
                    session_id,
                });
                DWalletState::AwaitingKeyHolderSignature {
                    public_output
                }
            }
        },
        _ =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongState">EWrongState</a>
    };
    gas_fee_reimbursement_sui
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_make_dwallet_user_secret_key_share_public"></a>

## Function `request_make_dwallet_user_secret_key_share_public`

Requests to make the user secret key shares of a dWallet public.
*IMPORTANT*: If you make the dWallet user secret key shares public, you remove
the zero trust security of the dWallet and you can't revert it.

This function emits a <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MakeDWalletUserSecretKeySharePublicRequestEvent">MakeDWalletUserSecretKeySharePublicRequestEvent</a></code> event to initiate the
process of making the user secret key shares of a dWallet public. It charges the initiator for
the operation and creates a new event to record the request.


<a name="@Parameters_6"></a>

##### Parameters

- <code>dwallet_id</code>: The ID of the dWallet to make the user secret key shares public.
- <code>public_user_secret_key_share</code>: The public user secret key shares to be made public.
- <code>payment_ika</code>: The IKA payment for the operation.
- <code>payment_sui</code>: The SUI payment for the operation.
- <code>ctx</code>: The transaction context.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_make_dwallet_user_secret_key_share_public">request_make_dwallet_user_secret_key_share_public</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, public_user_secret_key_share: vector&lt;u8&gt;, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_make_dwallet_user_secret_key_share_public">request_make_dwallet_user_secret_key_share_public</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
    public_user_secret_key_share: vector&lt;u8&gt;,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> (dwallet, public_output) = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output">get_active_dwallet_and_public_output</a>(dwallet_id);
    <b>let</b> dwallet_network_encryption_key_id = dwallet.dwallet_network_encryption_key_id;
    <b>let</b> curve = dwallet.curve;
    <b>assert</b>!(dwallet.public_user_secret_key_share.is_none(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletUserSecretKeySharesAlreadyPublic">EDWalletUserSecretKeySharesAlreadyPublic</a>);
    <b>let</b> <b>mut</b> pricing_value = self.pricing_and_fee_management.current.try_get_dwallet_pricing_value(curve, option::none(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MAKE_DWALLET_USER_SECRET_KEY_SHARE_PUBLIC_PROTOCOL_FLAG">MAKE_DWALLET_USER_SECRET_KEY_SHARE_PUBLIC_PROTOCOL_FLAG</a>);
    <b>assert</b>!(pricing_value.is_some(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMissingProtocolPricing">EMissingProtocolPricing</a>);
    event::emit(
        self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
            dwallet_network_encryption_key_id,
            pricing_value.extract(),
            payment_ika,
            payment_sui,
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MakeDWalletUserSecretKeySharePublicRequestEvent">MakeDWalletUserSecretKeySharePublicRequestEvent</a> {
                public_user_secret_key_share,
                public_output,
                curve,
                dwallet_id,
                dwallet_network_encryption_key_id,
            },
            ctx,
        )
    );
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_make_dwallet_user_secret_key_share_public"></a>

## Function `respond_make_dwallet_user_secret_key_share_public`

This function is called by the Ika network to respond to the request to make the dWallet's user share public.
Sets <code>public_user_secret_key_share</code> to the verified value.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_make_dwallet_user_secret_key_share_public">respond_make_dwallet_user_secret_key_share_public</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, public_user_secret_key_share: vector&lt;u8&gt;, rejected: bool, session_sequence_number: u64): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_make_dwallet_user_secret_key_share_public">respond_make_dwallet_user_secret_key_share_public</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_id: ID,
    public_user_secret_key_share: vector&lt;u8&gt;,
    rejected: bool,
    session_sequence_number: u64,
): Balance&lt;SUI&gt; {
    <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_user_initiated_session_and_charge">remove_user_initiated_session_and_charge</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MakeDWalletUserSecretKeySharePublicRequestEvent">MakeDWalletUserSecretKeySharePublicRequestEvent</a>&gt;(session_sequence_number);
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut">get_dwallet_mut</a>(dwallet_id);
    <b>if</b> (rejected) {
        event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedMakeDWalletUserSecretKeySharePublicEvent">RejectedMakeDWalletUserSecretKeySharePublicEvent</a> {
            dwallet_id,
        });
    } <b>else</b> {
        dwallet.public_user_secret_key_share.fill(public_user_secret_key_share);
        event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedMakeDWalletUserSecretKeySharePublicEvent">CompletedMakeDWalletUserSecretKeySharePublicEvent</a> {
            dwallet_id,
        });
    };
    gas_fee_reimbursement_sui
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_presign"></a>

## Function `request_presign`

Initiates the Presign protocol by creating a new <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignSession">PresignSession</a></code> in <code>self.presign_sessions</code>
and emitting an event for the Ika validators to request its execution.

Creates an <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">UnverifiedPresignCap</a></code> for the new <code>presign_id</code> that can be exclusively used with this <code>dwallet_id</code>.


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
    <b>let</b> (dwallet, public_output) = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output">get_active_dwallet_and_public_output</a>(dwallet_id);
    <b>let</b> curve = dwallet.curve;
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_curve_and_signature_algorithm">validate_curve_and_signature_algorithm</a>(curve, signature_algorithm);
    <b>let</b> dwallet_network_encryption_key_id = dwallet.dwallet_network_encryption_key_id;
    <b>let</b> id = object::new(ctx);
    <b>let</b> presign_id = id.to_inner();
    <b>let</b> cap = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">UnverifiedPresignCap</a> {
        id: object::new(ctx),
        dwallet_id: option::some(dwallet_id),
        presign_id,
    };
    self.presign_sessions.add(presign_id, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignSession">PresignSession</a> {
        id,
        created_at_epoch,
        signature_algorithm,
        curve,
        dwallet_id: option::some(dwallet_id),
        cap_id: object::id(&cap),
        state: PresignState::Requested,
    });
    <b>let</b> <b>mut</b> pricing_value = self.pricing_and_fee_management.current.try_get_dwallet_pricing_value(curve, option::some(signature_algorithm), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PRESIGN_PROTOCOL_FLAG">PRESIGN_PROTOCOL_FLAG</a>);
    <b>assert</b>!(pricing_value.is_some(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMissingProtocolPricing">EMissingProtocolPricing</a>);
    event::emit(
        self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
            dwallet_network_encryption_key_id,
            pricing_value.extract(),
            payment_ika,
            payment_sui,
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignRequestEvent">PresignRequestEvent</a> {
                dwallet_id: option::some(dwallet_id),
                presign_id,
                dwallet_public_output: option::some(public_output),
                dwallet_network_encryption_key_id,
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

Initiates the Presign protocol by creating a new <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignSession">PresignSession</a></code> in <code>self.presign_sessions</code>
and emitting an event for the Ika validators to request its execution.

Creates an <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">UnverifiedPresignCap</a></code> for the new <code>presign_id</code> that can be used with any dWallet.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_global_presign">request_global_presign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_network_encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, curve: u32, signature_algorithm: u32, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">dwallet_2pc_mpc_coordinator_inner::UnverifiedPresignCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_global_presign">request_global_presign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    dwallet_network_encryption_key_id: ID,
    curve: u32,
    signature_algorithm: u32,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">UnverifiedPresignCap</a> {
    <b>let</b> created_at_epoch = self.current_epoch;
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_curve_and_signature_algorithm">validate_curve_and_signature_algorithm</a>(curve, signature_algorithm);
    <b>let</b> id = object::new(ctx);
    <b>let</b> presign_id = id.to_inner();
    <b>let</b> cap = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">UnverifiedPresignCap</a> {
        id: object::new(ctx),
        dwallet_id: option::none(),
        presign_id,
    };
    self.presign_sessions.add(presign_id, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignSession">PresignSession</a> {
        id,
        created_at_epoch,
        signature_algorithm,
        curve,
        dwallet_id: option::none(),
        cap_id: object::id(&cap),
        state: PresignState::Requested,
    });
    <b>let</b> <b>mut</b> pricing_value = self.pricing_and_fee_management.current.try_get_dwallet_pricing_value(curve, option::some(signature_algorithm), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PRESIGN_PROTOCOL_FLAG">PRESIGN_PROTOCOL_FLAG</a>);
    <b>assert</b>!(pricing_value.is_some(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMissingProtocolPricing">EMissingProtocolPricing</a>);
    event::emit(
        self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
            dwallet_network_encryption_key_id,
            pricing_value.extract(),
            payment_ika,
            payment_sui,
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignRequestEvent">PresignRequestEvent</a> {
                dwallet_id: option::none(),
                presign_id,
                dwallet_public_output: option::none(),
                dwallet_network_encryption_key_id,
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

This function is called by the Ika network to respond to the Presign request made by the user.
Advances the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignSession">PresignSession</a></code> state to <code>Completed</code> and registers the output (the presign) in it.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_presign">respond_presign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;, presign_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, session_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, presign: vector&lt;u8&gt;, rejected: bool, session_sequence_number: u64): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;
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
): Balance&lt;SUI&gt; {
    <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_user_initiated_session_and_charge">remove_user_initiated_session_and_charge</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignRequestEvent">PresignRequestEvent</a>&gt;(session_sequence_number);
    <b>let</b> presign_obj = self.presign_sessions.borrow_mut(presign_id);
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
    gas_fee_reimbursement_sui
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_is_presign_valid"></a>

## Function `is_presign_valid`

Checks that the presign corresponding to <code>cap</code> is valid by ensuring it is in the <code>Completed</code> state and that the IDs match.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_is_presign_valid">is_presign_valid</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, cap: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">dwallet_2pc_mpc_coordinator_inner::UnverifiedPresignCap</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_is_presign_valid">is_presign_valid</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    cap: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">UnverifiedPresignCap</a>,
): bool {
    <b>let</b> presign = self.presign_sessions.borrow(cap.presign_id);
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

Verify <code>cap</code> by deleting the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">UnverifiedPresignCap</a></code> object and replacing it with a new <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">VerifiedPresignCap</a></code>,
if <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_is_presign_valid">is_presign_valid</a>()</code>.


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
    <b>let</b> presign = self.presign_sessions.borrow_mut(presign_id);
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

This function is a shared logic for both the standard and future sign flows.

It checks the presign is valid and deletes it (and its <code>presign_cap</code>), thus assuring it is not used twice.

Creates a <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignSession">SignSession</a></code> object and register it in <code>sign_sessions</code>.

Finally it emits the sign event.


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_and_initiate_sign">validate_and_initiate_sign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, pricing_value: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricingValue">dwallet_pricing::DWalletPricingValue</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, signature_algorithm: u32, hash_scheme: u32, message: vector&lt;u8&gt;, presign_cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">dwallet_2pc_mpc_coordinator_inner::VerifiedPresignCap</a>, message_centralized_signature: vector&lt;u8&gt;, is_future_sign: bool, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_and_initiate_sign">validate_and_initiate_sign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    pricing_value: DWalletPricingValue,
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
    <b>assert</b>!(self.presign_sessions.contains(presign_cap.presign_id), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EPresignNotExist">EPresignNotExist</a>);
    <b>let</b> presign = self.presign_sessions.remove(presign_cap.presign_id);
    <b>let</b> (dwallet, dwallet_public_output) = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output_mut">get_active_dwallet_and_public_output_mut</a>(dwallet_id);
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">VerifiedPresignCap</a> {
        id,
        dwallet_id: presign_cap_dwallet_id,
        presign_id: presign_cap_presign_id,
    } = presign_cap;
    <b>let</b> presign_cap_id = id.to_inner();
    id.delete();
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignSession">PresignSession</a> {
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
    // Check that the presign is global, or that it belongs to this dWallet.
    <b>assert</b>!(presign_dwallet_id.is_none() || presign_dwallet_id.is_some_and!(|id| id == dwallet_id), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMessageApprovalMismatch">EMessageApprovalMismatch</a>);
    // Sanity checks: check that the IDs of the capability and presign match, and that they point to this dWallet.
    <b>assert</b>!(presign_cap_id == cap_id, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EPresignNotExist">EPresignNotExist</a>);
    <b>assert</b>!(presign_id == presign_cap_presign_id, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EPresignNotExist">EPresignNotExist</a>);
    <b>assert</b>!(presign_cap_dwallet_id == presign_dwallet_id, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EPresignNotExist">EPresignNotExist</a>);
    // Check that the curve of the dWallet matches that of the presign, and that the signature algorithm matches.
    <b>assert</b>!(dwallet.curve == curve, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletMismatch">EDWalletMismatch</a>);
    <b>assert</b>!(presign_signature_algorithm == signature_algorithm, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMessageApprovalMismatch">EMessageApprovalMismatch</a>);
    // Emit a `<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignRequestEvent">SignRequestEvent</a>` to request the Ika network to sign `message`.
    <b>let</b> id = object::new(ctx);
    <b>let</b> sign_id = id.to_inner();
    <b>let</b> dwallet_network_encryption_key_id = dwallet.dwallet_network_encryption_key_id;
    <b>let</b> emit_event = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
        dwallet_network_encryption_key_id,
        pricing_value,
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
            dwallet_network_encryption_key_id,
            presign_id,
            presign,
            message_centralized_signature,
            is_future_sign,
        },
        ctx,
    );
    // Create a `<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignSession">SignSession</a>` object and register it in `sign_sessions`.
    <b>let</b> session_id = emit_event.session_id;
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut">get_dwallet_mut</a>(dwallet_id);
    dwallet.sign_sessions.add(sign_id, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignSession">SignSession</a> {
        id,
        created_at_epoch,
        dwallet_id,
        session_id,
        state: SignState::Requested,
    });
    <b>let</b> is_imported_key_dwallet = dwallet.is_imported_key_dwallet;
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_curve_and_signature_algorithm_and_hash_scheme">validate_curve_and_signature_algorithm_and_hash_scheme</a>(curve, signature_algorithm, hash_scheme);
    event::emit(emit_event);
    is_imported_key_dwallet
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_sign"></a>

## Function `request_sign`

Initiates the Sign protocol for this dWallet.
Requires a <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">MessageApproval</a></code>, which approves a message for signing and is unpacked and deleted to ensure it is never used twice.


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
    <b>let</b> (dwallet, _) = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output">get_active_dwallet_and_public_output</a>(dwallet_id);
    <b>let</b> curve = dwallet.curve;
    <b>let</b> <b>mut</b> pricing_value = self.pricing_and_fee_management.current.try_get_dwallet_pricing_value(curve, option::some(signature_algorithm), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SIGN_PROTOCOL_FLAG">SIGN_PROTOCOL_FLAG</a>);
    <b>assert</b>!(pricing_value.is_some(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMissingProtocolPricing">EMissingProtocolPricing</a>);
    <b>let</b> is_imported_key_dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_and_initiate_sign">validate_and_initiate_sign</a>(
        pricing_value.extract(),
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

Initiates the Sign protocol for this imported key dWallet.
Requires an <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyMessageApproval">ImportedKeyMessageApproval</a></code>, which approves a message for signing and is unpacked and deleted to ensure it is never used twice.


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
    <b>let</b> (dwallet, _) = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output">get_active_dwallet_and_public_output</a>(dwallet_id);
    <b>let</b> curve = dwallet.curve;
    <b>let</b> <b>mut</b> pricing_value = self.pricing_and_fee_management.current.try_get_dwallet_pricing_value(curve, option::some(signature_algorithm), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SIGN_PROTOCOL_FLAG">SIGN_PROTOCOL_FLAG</a>);
    <b>assert</b>!(pricing_value.is_some(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMissingProtocolPricing">EMissingProtocolPricing</a>);
    <b>let</b> is_imported_key_dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_and_initiate_sign">validate_and_initiate_sign</a>(
        pricing_value.extract(),
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

Request the Ika network verify the user-side sign protocol (in other words, that <code>message</code> is partially signed by the user),
without (yet) executing the network side sign-protocol.

Used for future sign use-cases, in which the user share isn't required to sign <code>message</code>;
instead, anyone that holds a <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap">VerifiedPartialUserSignatureCap</a></code> capability and a <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">MessageApproval</a></code> can sign <code>message</code> by calling <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_sign_with_partial_user_signature">request_sign_with_partial_user_signature</a>()</code> at any time.

Creates a new <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignature">PartialUserSignature</a></code> in the <code>AwaitingNetworkVerification</code> state and registered it into <code>partial_centralized_signed_messages</code>. Moves <code>presign_cap</code> to it,
ensuring it can be used for anything other than signing this <code>message</code> using <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_sign_with_partial_user_signature">request_sign_with_partial_user_signature</a>()</code> (which will in turn ensure it can only be signed once).

Creates a new <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPartialUserSignatureCap">UnverifiedPartialUserSignatureCap</a></code> object and returns it to the caller.

See the doc of [<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignature">PartialUserSignature</a></code>] for
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
    // Check that the presign is global, or that it belongs to this dWallet.
    <b>assert</b>!(presign_cap.dwallet_id.is_none() || presign_cap.dwallet_id.is_some_and!(|id| id == dwallet_id), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMessageApprovalMismatch">EMessageApprovalMismatch</a>);
    <b>let</b> (dwallet, dwallet_public_output) = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output_mut">get_active_dwallet_and_public_output_mut</a>(dwallet_id);
    <b>let</b> dwallet_network_encryption_key_id = dwallet.dwallet_network_encryption_key_id;
    <b>let</b> curve = dwallet.curve;
    <b>assert</b>!(self.presign_sessions.contains(presign_cap.presign_id), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EPresignNotExist">EPresignNotExist</a>);
    <b>let</b> presign_obj = self.presign_sessions.borrow(presign_cap.presign_id);
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
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_curve_and_signature_algorithm_and_hash_scheme">validate_curve_and_signature_algorithm_and_hash_scheme</a>(curve, signature_algorithm, hash_scheme);
    <b>let</b> <b>mut</b> pricing_value = self.pricing_and_fee_management.current.try_get_dwallet_pricing_value(curve, option::some(signature_algorithm), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_FUTURE_SIGN_PROTOCOL_FLAG">FUTURE_SIGN_PROTOCOL_FLAG</a>);
    <b>assert</b>!(pricing_value.is_some(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMissingProtocolPricing">EMissingProtocolPricing</a>);
    <b>let</b> emit_event = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
        dwallet_network_encryption_key_id,
        pricing_value.extract(),
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
                dwallet_network_encryption_key_id,
        },
        ctx,
    );
    // Create a new `<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignature">PartialUserSignature</a>` that wraps around `presign_cap` to ensure it can't be used twice.
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

Called by the Ika network to respond with the verification result of the user-side sign protocol (in other words, whether <code>message</code> is partially signed by the user).

Advances the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignature">PartialUserSignature</a></code> state to <code>NetworkVerificationCompleted</code>.

See the doc of [<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignature">PartialUserSignature</a></code>] for
more details on when this may be used.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_future_sign">respond_future_sign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, session_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, partial_centralized_signed_message_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, rejected: bool, session_sequence_number: u64): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;
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
): Balance&lt;SUI&gt; {
    <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_user_initiated_session_and_charge">remove_user_initiated_session_and_charge</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_FutureSignRequestEvent">FutureSignRequestEvent</a>&gt;(session_sequence_number);
    <b>let</b> partial_centralized_signed_message = self.partial_centralized_signed_messages.borrow_mut(partial_centralized_signed_message_id);
    // Check that the presign is global, or that it belongs to this dWallet.
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
    };
    gas_fee_reimbursement_sui
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_is_partial_user_signature_valid"></a>

## Function `is_partial_user_signature_valid`

Checks that the partial user signature corresponding to <code>cap</code> is valid, by assuring it is in the <code>NetworkVerificationCompleted</code>.


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

Verifies that the partial user signature corresponding to <code>cap</code> is valid,
deleting the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPartialUserSignatureCap">UnverifiedPartialUserSignatureCap</a></code> object and returning a new <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap">VerifiedPartialUserSignatureCap</a></code> in its place.


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

Requests the Ika network to complete the signing session on a message that was already partially-signed by the user (i.e. a message with a verified [<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignature">PartialUserSignature</a></code>]).
Useful is <code>message_approval</code> was only acquired after <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignature">PartialUserSignature</a></code> was created, and the caller does not own the user-share of this dWallet.

Takes the <code>presign_cap</code> from the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignature">PartialUserSignature</a></code> object, and destroys it in <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_and_initiate_sign">validate_and_initiate_sign</a>()</code>,
ensuring the presign was not used for any other purpose than signing this message once.

See the doc of [<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignature">PartialUserSignature</a></code>] for
more details on when this may be used.


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
        curve,
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
    <b>let</b> <b>mut</b> pricing_value = self.pricing_and_fee_management.current.try_get_dwallet_pricing_value(curve, option::some(signature_algorithm), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SIGN_WITH_PARTIAL_USER_SIGNATURE_PROTOCOL_FLAG">SIGN_WITH_PARTIAL_USER_SIGNATURE_PROTOCOL_FLAG</a>);
    <b>assert</b>!(pricing_value.is_some(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMissingProtocolPricing">EMissingProtocolPricing</a>);
    // Emit signing events to finalize the signing process.
    <b>let</b> is_imported_key_dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_and_initiate_sign">validate_and_initiate_sign</a>(
        pricing_value.extract(),
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

The imported key variant of [<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_sign_with_partial_user_signature">request_sign_with_partial_user_signature</a>()</code>] (see for documentation).


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
        curve,
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
    <b>let</b> <b>mut</b> pricing_value = self.pricing_and_fee_management.current.try_get_dwallet_pricing_value(curve, option::some(signature_algorithm), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SIGN_WITH_PARTIAL_USER_SIGNATURE_PROTOCOL_FLAG">SIGN_WITH_PARTIAL_USER_SIGNATURE_PROTOCOL_FLAG</a>);
    <b>assert</b>!(pricing_value.is_some(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMissingProtocolPricing">EMissingProtocolPricing</a>);
    // Emit signing events to finalize the signing process.
    <b>let</b> is_imported_key_dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_and_initiate_sign">validate_and_initiate_sign</a>(
        pricing_value.extract(),
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
It is also called before requesting the Ika network to complete the signing.


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

Called by the Ika network to respond to (and complete) a Sign protocol request.

Sets the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignSession">SignSession</a></code> to <code>Completed</code> and stores in it the <code>signature</code>.
Also emits an event with the <code>signature</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_sign">respond_sign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, sign_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, session_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, signature: vector&lt;u8&gt;, is_future_sign: bool, rejected: bool, session_sequence_number: u64): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;
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
): Balance&lt;SUI&gt; {
    <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_user_initiated_session_and_charge">remove_user_initiated_session_and_charge</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignRequestEvent">SignRequestEvent</a>&gt;(session_sequence_number);
    <b>let</b> (dwallet, _) = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output_mut">get_active_dwallet_and_public_output_mut</a>(dwallet_id);
    <b>let</b> sign = dwallet.sign_sessions.borrow_mut(sign_id);
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
    gas_fee_reimbursement_sui
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_process_checkpoint_message_by_quorum"></a>

## Function `process_checkpoint_message_by_quorum`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, signature: vector&lt;u8&gt;, signers_bitmap: vector&lt;u8&gt;, message: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_process_checkpoint_message_by_quorum">process_checkpoint_message_by_quorum</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    signature: vector&lt;u8&gt;,
    signers_bitmap: vector&lt;u8&gt;,
    message: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext,
): Coin&lt;SUI&gt; {
    <b>let</b> <b>mut</b> intent_bytes = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CHECKPOINT_MESSAGE_INTENT">CHECKPOINT_MESSAGE_INTENT</a>;
    intent_bytes.append(message);
    intent_bytes.append(bcs::to_bytes(&self.current_epoch));
    self.active_committee.verify_certificate(self.current_epoch, &signature, &signers_bitmap, &intent_bytes);
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_process_checkpoint_message">process_checkpoint_message</a>(message, ctx)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_process_checkpoint_message"></a>

## Function `process_checkpoint_message`



<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_process_checkpoint_message">process_checkpoint_message</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, message: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_process_checkpoint_message">process_checkpoint_message</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    message: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext,
): Coin&lt;SUI&gt; {
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
    <b>let</b> <b>mut</b> total_gas_fee_reimbursement_sui = balance::zero();
    <b>while</b> (i &lt; len) {
        <b>let</b> message_data_type = bcs_body.peel_vec_length();
        // Parses checkpoint BCS bytes directly.
        // Messages with `message_data_type` 1 & 2 are handled by the <a href="../ika_system/system.md#(ika_system=0x0)_system">system</a> <b>module</b>,
        // but their bytes must be extracted here to allow correct parsing of types 3 and above.
        // This step only extracts the bytes without further processing.
        match (message_data_type) {
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_DKG_FIRST_ROUND_OUTPUT_MESSAGE_TYPE">RESPOND_DWALLET_DKG_FIRST_ROUND_OUTPUT_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> first_round_output = bcs_body.peel_vec_u8();
                <b>let</b> rejected = bcs_body.peel_bool();
                <b>let</b> session_sequence_number = bcs_body.peel_u64();
                <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_dkg_first_round">respond_dwallet_dkg_first_round</a>(dwallet_id, first_round_output, rejected, session_sequence_number);
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            },
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_DKG_SECOND_ROUND_OUTPUT_MESSAGE_TYPE">RESPOND_DWALLET_DKG_SECOND_ROUND_OUTPUT_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> session_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> encrypted_user_secret_key_share_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> public_output = bcs_body.peel_vec_u8();
                <b>let</b> rejected = bcs_body.peel_bool();
                <b>let</b> session_sequence_number = bcs_body.peel_u64();
                <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_dkg_second_round">respond_dwallet_dkg_second_round</a>(
                    dwallet_id,
                    public_output,
                    encrypted_user_secret_key_share_id,
                    session_id,
                    rejected,
                    session_sequence_number,
                );
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            },
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_ENCRYPTED_USER_SHARE_MESSAGE_TYPE">RESPOND_DWALLET_ENCRYPTED_USER_SHARE_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> encrypted_user_secret_key_share_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> rejected = bcs_body.peel_bool();
                <b>let</b> session_sequence_number = bcs_body.peel_u64();
                <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_re_encrypt_user_share_for">respond_re_encrypt_user_share_for</a>(
                    dwallet_id,
                    encrypted_user_secret_key_share_id,
                    rejected,
                    session_sequence_number,
                );
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            },
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_MAKE_DWALLET_USER_SECRET_KEY_SHARES_PUBLIC_MESSAGE_TYPE">RESPOND_MAKE_DWALLET_USER_SECRET_KEY_SHARES_PUBLIC_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> public_user_secret_key_shares = bcs_body.peel_vec_u8();
                <b>let</b> rejected = bcs_body.peel_bool();
                <b>let</b> session_sequence_number = bcs_body.peel_u64();
                <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_make_dwallet_user_secret_key_share_public">respond_make_dwallet_user_secret_key_share_public</a>(dwallet_id, public_user_secret_key_shares, rejected, session_sequence_number);
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            },
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_IMPORTED_KEY_VERIFICATION_OUTPUT_MESSAGE_TYPE">RESPOND_DWALLET_IMPORTED_KEY_VERIFICATION_OUTPUT_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> public_output = bcs_body.peel_vec_u8();
                <b>let</b> encrypted_user_secret_key_share_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> session_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> rejected = bcs_body.peel_bool();
                <b>let</b> session_sequence_number = bcs_body.peel_u64();
                <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_imported_key_dwallet_verification">respond_imported_key_dwallet_verification</a>(
                    dwallet_id,
                    public_output,
                    encrypted_user_secret_key_share_id,
                    session_id,
                    rejected,
                    session_sequence_number
                );
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            },
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_PRESIGN_MESSAGE_TYPE">RESPOND_DWALLET_PRESIGN_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> dwallet_id = bcs_body.peel_option!(|bcs_option| object::id_from_bytes(bcs_option.peel_vec_u8()));
                <b>let</b> presign_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> session_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> presign = bcs_body.peel_vec_u8();
                <b>let</b> rejected = bcs_body.peel_bool();
                <b>let</b> session_sequence_number = bcs_body.peel_u64();
                <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_presign">respond_presign</a>(
                    dwallet_id,
                    presign_id,
                    session_id,
                    presign,
                    rejected,
                    session_sequence_number);
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            },
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_SIGN_MESSAGE_TYPE">RESPOND_DWALLET_SIGN_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> sign_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> session_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> signature = bcs_body.peel_vec_u8();
                <b>let</b> is_future_sign = bcs_body.peel_bool();
                <b>let</b> rejected = bcs_body.peel_bool();
                <b>let</b> session_sequence_number = bcs_body.peel_u64();
                <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_sign">respond_sign</a>(
                    dwallet_id,
                    sign_id,
                    session_id,
                    signature,
                    is_future_sign,
                    rejected,
                    session_sequence_number
                );
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            },
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_PARTIAL_SIGNATURE_VERIFICATION_OUTPUT_MESSAGE_TYPE">RESPOND_DWALLET_PARTIAL_SIGNATURE_VERIFICATION_OUTPUT_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> session_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> dwallet_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> partial_centralized_signed_message_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> rejected = bcs_body.peel_bool();
                <b>let</b> session_sequence_number = bcs_body.peel_u64();
                <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_future_sign">respond_future_sign</a>(
                    session_id,
                    dwallet_id,
                    partial_centralized_signed_message_id,
                    rejected,
                    session_sequence_number
                );
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            },
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_MPC_NETWORK_DKG_OUTPUT_MESSAGE_TYPE">RESPOND_DWALLET_MPC_NETWORK_DKG_OUTPUT_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> dwallet_network_encryption_key_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> public_output = bcs_body.peel_vec_u8();
                <b>let</b> is_last = bcs_body.peel_bool();
                <b>let</b> rejected = bcs_body.peel_bool();
                <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_network_encryption_key_dkg">respond_dwallet_network_encryption_key_dkg</a>(dwallet_network_encryption_key_id, public_output, is_last, rejected, ctx);
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            },
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_MPC_NETWORK_RECONFIGURATION_OUTPUT_MESSAGE_TYPE">RESPOND_DWALLET_MPC_NETWORK_RECONFIGURATION_OUTPUT_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> dwallet_network_encryption_key_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> public_output = bcs_body.peel_vec_u8();
                <b>let</b> is_last = bcs_body.peel_bool();
                <b>let</b> rejected = bcs_body.peel_bool();
                <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_network_encryption_key_reconfiguration">respond_dwallet_network_encryption_key_reconfiguration</a>(dwallet_network_encryption_key_id, public_output, is_last, rejected, ctx);
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            },
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SET_MAX_ACTIVE_SESSIONS_BUFFER_MESSAGE_TYPE">SET_MAX_ACTIVE_SESSIONS_BUFFER_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> max_active_sessions_buffer = bcs_body.peel_u64();
                self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_set_max_active_sessions_buffer">set_max_active_sessions_buffer</a>(max_active_sessions_buffer);
            },
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SET_GAS_FEE_REIMBURSEMENT_SUI_SYSTEM_CALL_VALUE_MESSAGE_TYPE">SET_GAS_FEE_REIMBURSEMENT_SUI_SYSTEM_CALL_VALUE_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> gas_fee_reimbursement_sui_system_call_value = bcs_body.peel_u64();
                self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_set_gas_fee_reimbursement_sui_system_call_value">set_gas_fee_reimbursement_sui_system_call_value</a>(gas_fee_reimbursement_sui_system_call_value);
            },
            _ =&gt; {},
        };
        i = i + 1;
    };
    self.total_messages_processed = self.total_messages_processed + i;
    total_gas_fee_reimbursement_sui.into_coin(ctx)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_set_max_active_sessions_buffer"></a>

## Function `set_max_active_sessions_buffer`



<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_set_max_active_sessions_buffer">set_max_active_sessions_buffer</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, max_active_sessions_buffer: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_set_max_active_sessions_buffer">set_max_active_sessions_buffer</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    max_active_sessions_buffer: u64,
) {
    self.session_management.max_active_sessions_buffer = max_active_sessions_buffer;
    event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SetMaxActiveSessionsBufferEvent">SetMaxActiveSessionsBufferEvent</a> {
        max_active_sessions_buffer
    });
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_set_gas_fee_reimbursement_sui_system_call_value"></a>

## Function `set_gas_fee_reimbursement_sui_system_call_value`



<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_set_gas_fee_reimbursement_sui_system_call_value">set_gas_fee_reimbursement_sui_system_call_value</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, gas_fee_reimbursement_sui_system_call_value: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_set_gas_fee_reimbursement_sui_system_call_value">set_gas_fee_reimbursement_sui_system_call_value</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    gas_fee_reimbursement_sui_system_call_value: u64,
) {
    self.pricing_and_fee_management.gas_fee_reimbursement_sui_system_call_value = gas_fee_reimbursement_sui_system_call_value;
    event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SetGasFeeReimbursementSuiSystemCallValueEvent">SetGasFeeReimbursementSuiSystemCallValueEvent</a> {
        gas_fee_reimbursement_sui_system_call_value
    });
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_set_supported_and_pricing"></a>

## Function `set_supported_and_pricing`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_set_supported_and_pricing">set_supported_and_pricing</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, default_pricing: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing">dwallet_pricing::DWalletPricing</a>, supported_curves_to_signature_algorithms_to_hash_schemes: <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u32, <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u32, vector&lt;u32&gt;&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_set_supported_and_pricing">set_supported_and_pricing</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    default_pricing: DWalletPricing,
    supported_curves_to_signature_algorithms_to_hash_schemes: VecMap&lt;u32, VecMap&lt;u32, vector&lt;u32&gt;&gt;&gt;,
) {
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_verify_pricing_exists_for_all_protocols">verify_pricing_exists_for_all_protocols</a>(&supported_curves_to_signature_algorithms_to_hash_schemes, &default_pricing);
    self.pricing_and_fee_management.default = default_pricing;
    self.support_config.supported_curves_to_signature_algorithms_to_hash_schemes = supported_curves_to_signature_algorithms_to_hash_schemes;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_verify_pricing_exists_for_all_protocols"></a>

## Function `verify_pricing_exists_for_all_protocols`

Verifies that pricing exists for all protocols for all curves.
Aborts if pricing is missing for any protocol or curve.
IMPORTANT: every time a new protocol is added, this function must be updated with verifying the new protocol pricing.


<a name="@Parameters_7"></a>

##### Parameters

- **<code>supported_curves_to_signature_algorithms_to_hash_schemes</code>**: A map of curves to signature algorithms to hash schemes.
- **<code>default_pricing</code>**: The default pricing to use if pricing is missing for a protocol or curve.


<a name="@Errors_8"></a>

##### Errors

- **<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMissingProtocolPricing">EMissingProtocolPricing</a></code>**: If pricing is missing for any protocol or curve.


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_verify_pricing_exists_for_all_protocols">verify_pricing_exists_for_all_protocols</a>(supported_curves_to_signature_algorithms_to_hash_schemes: &<a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u32, <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u32, vector&lt;u32&gt;&gt;&gt;, default_pricing: &(ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing">dwallet_pricing::DWalletPricing</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_verify_pricing_exists_for_all_protocols">verify_pricing_exists_for_all_protocols</a>(supported_curves_to_signature_algorithms_to_hash_schemes: &VecMap&lt;u32, VecMap&lt;u32, vector&lt;u32&gt;&gt;&gt;, default_pricing: &DWalletPricing) {
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> curves = supported_curves_to_signature_algorithms_to_hash_schemes.keys();
    <b>while</b> (i &lt; curves.length()) {
        <b>let</b> <b>mut</b> is_missing_pricing = <b>false</b>;
        <b>let</b> curve = curves[i];
        <b>let</b> signature_algorithms = &supported_curves_to_signature_algorithms_to_hash_schemes[&curve];
        <b>let</b> signature_algorithms = signature_algorithms.keys();
        is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(curve, option::none(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DKG_FIRST_ROUND_PROTOCOL_FLAG">DKG_FIRST_ROUND_PROTOCOL_FLAG</a>).is_none();
        is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(curve, option::none(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DKG_SECOND_ROUND_PROTOCOL_FLAG">DKG_SECOND_ROUND_PROTOCOL_FLAG</a>).is_none();
        is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(curve, option::none(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RE_ENCRYPT_USER_SHARE_PROTOCOL_FLAG">RE_ENCRYPT_USER_SHARE_PROTOCOL_FLAG</a>).is_none();
        is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(curve, option::none(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MAKE_DWALLET_USER_SECRET_KEY_SHARE_PUBLIC_PROTOCOL_FLAG">MAKE_DWALLET_USER_SECRET_KEY_SHARE_PUBLIC_PROTOCOL_FLAG</a>).is_none();
        is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(curve, option::none(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_IMPORTED_KEY_DWALLET_VERIFICATION_PROTOCOL_FLAG">IMPORTED_KEY_DWALLET_VERIFICATION_PROTOCOL_FLAG</a>).is_none();
        // Add here pricing validation <b>for</b> new protocols per curve.
        signature_algorithms.do_ref!(|signature_algorithm| {
            is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(curve, option::some(*signature_algorithm), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PRESIGN_PROTOCOL_FLAG">PRESIGN_PROTOCOL_FLAG</a>).is_none();
            is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(curve, option::some(*signature_algorithm), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SIGN_PROTOCOL_FLAG">SIGN_PROTOCOL_FLAG</a>).is_none();
            is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(curve, option::some(*signature_algorithm), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_FUTURE_SIGN_PROTOCOL_FLAG">FUTURE_SIGN_PROTOCOL_FLAG</a>).is_none();
            is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(curve, option::some(*signature_algorithm), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SIGN_WITH_PARTIAL_USER_SIGNATURE_PROTOCOL_FLAG">SIGN_WITH_PARTIAL_USER_SIGNATURE_PROTOCOL_FLAG</a>).is_none();
            // Add here pricing validation <b>for</b> new protocols per curve per signature algorithm.
        });
        <b>assert</b>!(!is_missing_pricing, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMissingProtocolPricing">EMissingProtocolPricing</a>);
        i = i + 1;
    };
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
    self.support_config.paused_curves = paused_curves;
    self.support_config.paused_signature_algorithms = paused_signature_algorithms;
    self.support_config.paused_hash_schemes = paused_hash_schemes;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_set_pricing_vote"></a>

## Function `set_pricing_vote`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_set_pricing_vote">set_pricing_vote</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, validator_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, pricing_vote: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing">dwallet_pricing::DWalletPricing</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_set_pricing_vote">set_pricing_vote</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    validator_id: ID,
    pricing_vote: DWalletPricing,
) {
    <b>assert</b>!(self.pricing_and_fee_management.calculation_votes.is_none(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ECannotSetDuringVotesCalculation">ECannotSetDuringVotesCalculation</a>);
    <b>if</b>(self.pricing_and_fee_management.validator_votes.contains(validator_id)) {
        <b>let</b> vote = self.pricing_and_fee_management.validator_votes.borrow_mut(validator_id);
        *vote = pricing_vote;
    } <b>else</b> {
        self.pricing_and_fee_management.validator_votes.add(validator_id, pricing_vote);
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_subsidize_coordinator_with_sui"></a>

## Function `subsidize_coordinator_with_sui`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_subsidize_coordinator_with_sui">subsidize_coordinator_with_sui</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, sui: <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_subsidize_coordinator_with_sui">subsidize_coordinator_with_sui</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    sui: Coin&lt;SUI&gt;,
) {
    self.pricing_and_fee_management.gas_fee_reimbursement_sui.join(sui.into_balance());
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_subsidize_coordinator_with_ika"></a>

## Function `subsidize_coordinator_with_ika`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_subsidize_coordinator_with_ika">subsidize_coordinator_with_ika</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, ika: <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_subsidize_coordinator_with_ika">subsidize_coordinator_with_ika</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    ika: Coin&lt;IKA&gt;,
) {
    self.pricing_and_fee_management.consensus_validation_fee_charged_ika.join(ika.into_balance());
}
</code></pre>



</details>
