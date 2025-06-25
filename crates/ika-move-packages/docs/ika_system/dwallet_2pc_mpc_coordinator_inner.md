---
title: Module `(ika_system=0x0)::dwallet_2pc_mpc_coordinator_inner`
---


<a name="@dWallet_2PC-MPC_Coordinator_Inner_Module_0"></a>

## dWallet 2PC-MPC Coordinator Inner Module


This module implements the core logic for creating and managing dWallets using
Multi-Party Computation (MPC) protocols. It provides a trustless and decentralized
approach to wallet creation and key management through distributed key generation (DKG)
and threshold signing protocols.


<a name="@Key_Features_1"></a>

### Key Features

- Distributed Key Generation (DKG) for secure key creation
- Threshold signing with presign optimization
- Network encryption key management and reconfiguration
- User encryption key registration and management
- Session-based MPC protocol coordination
- Epoch-based validator committee transitions
- Comprehensive pricing and fee management
- Support for multiple cryptographic curves and algorithms


<a name="@Architecture_2"></a>

### Architecture

The module is organized around the <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a></code> struct which manages:
- dWallet lifecycle and state transitions
- MPC session coordination and scheduling
- Validator committee management
- Cryptographic algorithm support and emergency controls
- Economic incentives through pricing and fee collection


-  [dWallet 2PC-MPC Coordinator Inner Module](#@dWallet_2PC-MPC_Coordinator_Inner_Module_0)
    -  [Key Features](#@Key_Features_1)
    -  [Architecture](#@Architecture_2)
-  [Struct `SessionManagement`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionManagement)
-  [Struct `SupportConfig`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SupportConfig)
-  [Struct `PricingAndFeeManagement`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PricingAndFeeManagement)
-  [Struct `DWalletCoordinatorInner`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner)
        -  [Key Components:](#@Key_Components:_3)
-  [Struct `DWalletSession`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSession)
-  [Struct `DWalletCap`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap)
-  [Struct `ImportedKeyDWalletCap`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyDWalletCap)
-  [Struct `DWalletNetworkEncryptionKeyCap`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyCap)
-  [Struct `DWalletNetworkEncryptionKey`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKey)
        -  [Lifecycle Phases](#@Lifecycle_Phases_4)
        -  [Data Storage Strategy](#@Data_Storage_Strategy_7)
        -  [Security Properties](#@Security_Properties_8)
-  [Struct `EncryptionKey`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptionKey)
        -  [Security Model](#@Security_Model_9)
        -  [Use Cases](#@Use_Cases_10)
-  [Struct `EncryptedUserSecretKeyShare`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShare)
        -  [Verification Process](#@Verification_Process_11)
        -  [Creation Methods](#@Creation_Methods_12)
        -  [Security Properties](#@Security_Properties_13)
-  [Struct `UnverifiedPartialUserSignatureCap`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPartialUserSignatureCap)
        -  [Verification Process](#@Verification_Process_14)
        -  [Security Properties](#@Security_Properties_15)
-  [Struct `VerifiedPartialUserSignatureCap`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap)
        -  [Usage in Conditional Signing](#@Usage_in_Conditional_Signing_16)
        -  [Security Guarantees](#@Security_Guarantees_17)
-  [Struct `PartialUserSignature`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignature)
        -  [Use Cases](#@Use_Cases_18)
        -  [Security Properties](#@Security_Properties_21)
-  [Struct `DWallet`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet)
        -  [Security Models](#@Security_Models_22)
        -  [State Lifecycle](#@State_Lifecycle_23)
-  [Struct `UnverifiedPresignCap`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap)
        -  [Verification Process](#@Verification_Process_24)
        -  [Security Model](#@Security_Model_25)
-  [Struct `VerifiedPresignCap`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap)
        -  [Usage Constraints](#@Usage_Constraints_26)
        -  [Security Properties](#@Security_Properties_27)
-  [Struct `PresignSession`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignSession)
        -  [Types of Presigns](#@Types_of_Presigns_28)
        -  [Performance Benefits](#@Performance_Benefits_31)
        -  [Security Properties](#@Security_Properties_32)
-  [Struct `SignSession`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignSession)
        -  [Signing Process](#@Signing_Process_33)
        -  [Types of Signing](#@Types_of_Signing_34)
        -  [Performance Optimization](#@Performance_Optimization_35)
-  [Struct `SessionIdentifier`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier)
-  [Struct `MessageApproval`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval)
        -  [Security Properties](#@Security_Properties_36)
        -  [Usage Pattern](#@Usage_Pattern_37)
-  [Struct `ImportedKeyMessageApproval`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyMessageApproval)
        -  [Differences from Standard MessageApproval](#@Differences_from_Standard_MessageApproval_38)
        -  [Security Considerations](#@Security_Considerations_39)
-  [Struct `SessionIdentifierRegisteredEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifierRegisteredEvent)
-  [Struct `DWalletSessionEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSessionEvent)
-  [Struct `DWalletSessionResultEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSessionResultEvent)
-  [Struct `DWalletNetworkDKGEncryptionKeyRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDKGEncryptionKeyRequestEvent)
-  [Struct `CompletedDWalletNetworkDKGEncryptionKeyEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletNetworkDKGEncryptionKeyEvent)
        -  [Next Steps](#@Next_Steps_40)
-  [Struct `RejectedDWalletNetworkDKGEncryptionKeyEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletNetworkDKGEncryptionKeyEvent)
-  [Struct `DWalletEncryptionKeyReconfigurationRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletEncryptionKeyReconfigurationRequestEvent)
-  [Struct `CompletedDWalletEncryptionKeyReconfigurationEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletEncryptionKeyReconfigurationEvent)
-  [Struct `RejectedDWalletEncryptionKeyReconfigurationEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletEncryptionKeyReconfigurationEvent)
-  [Struct `DWalletDKGFirstRoundRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGFirstRoundRequestEvent)
-  [Struct `CompletedDWalletDKGFirstRoundEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletDKGFirstRoundEvent)
        -  [Next Steps](#@Next_Steps_41)
-  [Struct `RejectedDWalletDKGFirstRoundEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletDKGFirstRoundEvent)
-  [Struct `DWalletDKGSecondRoundRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGSecondRoundRequestEvent)
        -  [Process Flow](#@Process_Flow_42)
        -  [Security Properties](#@Security_Properties_43)
-  [Struct `CompletedDWalletDKGSecondRoundEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletDKGSecondRoundEvent)
        -  [Next Steps for Users](#@Next_Steps_for_Users_44)
        -  [Security Verification](#@Security_Verification_45)
-  [Struct `RejectedDWalletDKGSecondRoundEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletDKGSecondRoundEvent)
        -  [Common Rejection Reasons](#@Common_Rejection_Reasons_46)
-  [Struct `DWalletImportedKeyVerificationRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletImportedKeyVerificationRequestEvent)
        -  [Imported Key Flow](#@Imported_Key_Flow_47)
        -  [Security Considerations](#@Security_Considerations_48)
-  [Struct `CompletedDWalletImportedKeyVerificationEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletImportedKeyVerificationEvent)
        -  [Next Steps for Users](#@Next_Steps_for_Users_49)
-  [Struct `RejectedDWalletImportedKeyVerificationEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletImportedKeyVerificationEvent)
        -  [Common Rejection Reasons](#@Common_Rejection_Reasons_50)
-  [Struct `CreatedEncryptionKeyEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CreatedEncryptionKeyEvent)
-  [Struct `EncryptedShareVerificationRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedShareVerificationRequestEvent)
        -  [Re-encryption Use Cases](#@Re-encryption_Use_Cases_51)
        -  [Verification Process](#@Verification_Process_52)
        -  [Security Properties](#@Security_Properties_53)
-  [Struct `CompletedEncryptedShareVerificationEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedEncryptedShareVerificationEvent)
        -  [Next Steps for Recipient](#@Next_Steps_for_Recipient_54)
-  [Struct `RejectedEncryptedShareVerificationEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedEncryptedShareVerificationEvent)
        -  [Common Rejection Reasons](#@Common_Rejection_Reasons_55)
-  [Struct `AcceptEncryptedUserShareEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_AcceptEncryptedUserShareEvent)
        -  [Acceptance Process](#@Acceptance_Process_56)
        -  [Security Verification](#@Security_Verification_57)
-  [Struct `MakeDWalletUserSecretKeySharePublicRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MakeDWalletUserSecretKeySharePublicRequestEvent)
        -  [⚠️ CRITICAL SECURITY WARNING](#@⚠️_CRITICAL_SECURITY_WARNING_58)
-  [Struct `CompletedMakeDWalletUserSecretKeySharePublicEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedMakeDWalletUserSecretKeySharePublicEvent)
        -  [Post-Transition Capabilities](#@Post-Transition_Capabilities_62)
        -  [⚠️ Security Reminder](#@⚠️_Security_Reminder_63)
-  [Struct `RejectedMakeDWalletUserSecretKeySharePublicEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedMakeDWalletUserSecretKeySharePublicEvent)
        -  [Common Rejection Reasons](#@Common_Rejection_Reasons_64)
-  [Struct `PresignRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignRequestEvent)
        -  [Presign Types](#@Presign_Types_65)
        -  [Performance Benefits](#@Performance_Benefits_68)
-  [Struct `CompletedPresignEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedPresignEvent)
        -  [Next Steps](#@Next_Steps_69)
        -  [Security Properties](#@Security_Properties_70)
-  [Struct `RejectedPresignEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedPresignEvent)
        -  [Common Rejection Reasons](#@Common_Rejection_Reasons_71)
-  [Struct `SignRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignRequestEvent)
        -  [Signing Process Flow](#@Signing_Process_Flow_72)
        -  [Signature Types](#@Signature_Types_73)
        -  [Performance Optimization](#@Performance_Optimization_76)
-  [Struct `CompletedSignEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedSignEvent)
        -  [Signature Properties](#@Signature_Properties_77)
        -  [Next Steps](#@Next_Steps_78)
        -  [Performance Metrics](#@Performance_Metrics_79)
-  [Struct `RejectedSignEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedSignEvent)
        -  [Common Rejection Reasons](#@Common_Rejection_Reasons_80)
        -  [Recovery Steps](#@Recovery_Steps_81)
-  [Struct `FutureSignRequestEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_FutureSignRequestEvent)
        -  [Future Sign Use Cases](#@Future_Sign_Use_Cases_82)
        -  [Security Benefits](#@Security_Benefits_86)
-  [Struct `CompletedFutureSignEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedFutureSignEvent)
        -  [Next Steps](#@Next_Steps_87)
-  [Struct `RejectedFutureSignEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedFutureSignEvent)
        -  [Common Rejection Reasons](#@Common_Rejection_Reasons_88)
-  [Struct `DWalletCheckpointInfoEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCheckpointInfoEvent)
-  [Struct `SetMaxActiveSessionsBufferEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SetMaxActiveSessionsBufferEvent)
-  [Struct `SetGasFeeReimbursementSuiSystemCallValueEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SetGasFeeReimbursementSuiSystemCallValueEvent)
-  [Enum `DWalletNetworkEncryptionKeyState`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyState)
-  [Enum `EncryptedUserSecretKeyShareState`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShareState)
-  [Enum `PartialUserSignatureState`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignatureState)
-  [Enum `DWalletState`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletState)
-  [Enum `PresignState`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignState)
-  [Enum `SignState`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignState)
-  [Enum `SessionType`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionType)
-  [Enum `DWalletSessionStatusEvent`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSessionStatusEvent)
-  [Constants](#@Constants_89)
-  [Function `create_dwallet_coordinator_inner`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_create_dwallet_coordinator_inner)
-  [Function `lock_last_active_session_sequence_number`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_lock_last_active_session_sequence_number)
-  [Function `register_session_identifier`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_register_session_identifier)
-  [Function `request_dwallet_network_encryption_key_dkg`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_network_encryption_key_dkg)
-  [Function `charge_gas_fee_reimbursement_sui_for_system_calls`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_gas_fee_reimbursement_sui_for_system_calls)
-  [Function `handle_completed_system_session`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_handle_completed_system_session)
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
-  [Function `validate_network_encryption_key_supports_curve`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_network_encryption_key_supports_curve)
-  [Function `register_encryption_key`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_register_encryption_key)
-  [Function `approve_message`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_approve_message)
-  [Function `approve_imported_key_message`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_approve_imported_key_message)
-  [Function `validate_approve_message`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_approve_message)
-  [Function `update_last_user_initiated_session_to_complete_in_current_epoch`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_update_last_user_initiated_session_to_complete_in_current_epoch)
-  [Function `all_current_epoch_sessions_completed`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_all_current_epoch_sessions_completed)
-  [Function `remove_user_initiated_session_and_charge`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_user_initiated_session_and_charge)
-  [Function `request_dwallet_dkg_first_round`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_dkg_first_round)
-  [Function `respond_dwallet_dkg_first_round`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_dkg_first_round)
-  [Function `request_dwallet_dkg_second_round`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_dkg_second_round)
-  [Function `respond_dwallet_dkg_second_round`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_dkg_second_round)
-  [Function `request_re_encrypt_user_share_for`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_re_encrypt_user_share_for)
-  [Function `respond_re_encrypt_user_share_for`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_re_encrypt_user_share_for)
-  [Function `accept_encrypted_user_share`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_accept_encrypted_user_share)
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
-  [Function `dwallet_network_encryption_key_id`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id)
-  [Function `current_pricing`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_current_pricing)
-  [Function `get_network_encryption_key_supported_curves`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_network_encryption_key_supported_curves)
-  [Function `dwallet_id`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id)
-  [Function `imported_key_dwallet_id`](#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_imported_key_dwallet_id)


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



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionManagement"></a>

## Struct `SessionManagement`

Session management data for the dWallet coordinator.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionManagement">SessionManagement</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>registered_session_identifiers: <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;vector&lt;u8&gt;, <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;</code>
</dt>
<dd>
 Registered session identifiers, keyed by the session identifier bytes -> to session object ID
</dd>
<dt>
<code>sessions: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;u64, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSession">dwallet_2pc_mpc_coordinator_inner::DWalletSession</a>&gt;</code>
</dt>
<dd>
 Active sessions indexed by sequence number
</dd>
<dt>
<code>session_events: <a href="../sui/bag.md#sui_bag_Bag">sui::bag::Bag</a></code>
</dt>
<dd>
 Events for user-requested sessions, keyed by session ID
</dd>
<dt>
<code>number_of_completed_user_initiated_sessions: u64</code>
</dt>
<dd>
 Number of completed user-initiated sessions
</dd>
<dt>
<code>started_system_sessions_count: u64</code>
</dt>
<dd>
 Count of started system sessions
</dd>
<dt>
<code>completed_system_sessions_count: u64</code>
</dt>
<dd>
 Count of completed system sessions
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
 List of paused curves in case of emergency (e.g. [secp256k1, ristretto])
</dd>
<dt>
<code>paused_signature_algorithms: vector&lt;u32&gt;</code>
</dt>
<dd>
 List of paused signature algorithms in case of emergency (e.g. [ecdsa, schnorr])
</dd>
<dt>
<code>paused_hash_schemes: vector&lt;u32&gt;</code>
</dt>
<dd>
 List of paused hash schemes in case of emergency (e.g. [sha256, keccak256])
</dd>
<dt>
<code>signature_algorithms_allowed_global_presign: vector&lt;u32&gt;</code>
</dt>
<dd>
 Signature algorithms that are allowed for global presign
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
 Pricing for the current epoch
</dd>
<dt>
<code>default: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing">dwallet_pricing::DWalletPricing</a></code>
</dt>
<dd>
 Default pricing configuration
</dd>
<dt>
<code>validator_votes: <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing">dwallet_pricing::DWalletPricing</a>&gt;</code>
</dt>
<dd>
 Validator votes for pricing (validator ID -> pricing vote)
</dd>
<dt>
<code>calculation_votes: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;(ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricingCalculationVotes">dwallet_pricing::DWalletPricingCalculationVotes</a>&gt;</code>
</dt>
<dd>
 Pricing calculation votes - if set, must complete before epoch advance
</dd>
<dt>
<code>gas_fee_reimbursement_sui_system_call_value: u64</code>
</dt>
<dd>
 Gas fee reimbursement value for system calls
</dd>
<dt>
<code>gas_fee_reimbursement_sui: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;</code>
</dt>
<dd>
 SUI balance for gas fee reimbursement to fund network tx responses
</dd>
<dt>
<code>consensus_validation_fee_charged_ika: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;</code>
</dt>
<dd>
 IKA fees charged for consensus validation
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner"></a>

## Struct `DWalletCoordinatorInner`

Core coordinator for dWallet 2PC-MPC operations.

This shared object manages all aspects of dWallet creation and operation:
- dWallet lifecycle (DKG, signing, presigning)
- Network encryption keys and user encryption
- Session management and epoch transitions
- Pricing and fee collection
- Committee management and consensus

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
information regarding <code>pricing_and_fee_management</code>, all the <code>session_management</code> and the <code>next_session_sequence_number</code> that will be used for the next session,
and various other fields, like the supported and paused curves, signing algorithms and hashes.


<a name="@Key_Components:_3"></a>

#### Key Components:

- <code>dwallets</code>: Core dWallet objects with public keys and encrypted shares
- <code>dwallet_network_encryption_keys</code>: Network threshold encryption keys
- <code>encryption_keys</code>: User encryption keys for secure share storage
- <code>presign_sessions</code>: Precomputed signing materials
- <code>partial_centralized_signed_messages</code>: Future sign capabilities
- <code>session_management</code>: MPC session coordination
- <code>pricing_and_fee_management</code>: Economic incentives and fee collection
- <code>active_committee</code>/<code>previous_committee</code>: Validator consensus groups
- <code>support_config</code>: Cryptographic algorithm support and emergency controls


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>current_epoch: u64</code>
</dt>
<dd>
 Current epoch number
</dd>
<dt>
<code>session_management: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionManagement">dwallet_2pc_mpc_coordinator_inner::SessionManagement</a></code>
</dt>
<dd>
 Session management and coordination
</dd>
<dt>
<code>dwallets: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">dwallet_2pc_mpc_coordinator_inner::DWallet</a>&gt;</code>
</dt>
<dd>
 All dWallet instances (DWallet ID -> DWallet)
</dd>
<dt>
<code>dwallet_network_encryption_keys: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKey">dwallet_2pc_mpc_coordinator_inner::DWalletNetworkEncryptionKey</a>&gt;</code>
</dt>
<dd>
 Network encryption keys (Network encryption key ID -> DWalletNetworkEncryptionKey)
</dd>
<dt>
<code>encryption_keys: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;<b>address</b>, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptionKey">dwallet_2pc_mpc_coordinator_inner::EncryptionKey</a>&gt;</code>
</dt>
<dd>
 User encryption keys (User encryption key address -> EncryptionKey)
</dd>
<dt>
<code>presign_sessions: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignSession">dwallet_2pc_mpc_coordinator_inner::PresignSession</a>&gt;</code>
</dt>
<dd>
 Presign sessions for signature optimization (Presign session ID -> PresignSession)
</dd>
<dt>
<code>partial_centralized_signed_messages: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignature">dwallet_2pc_mpc_coordinator_inner::PartialUserSignature</a>&gt;</code>
</dt>
<dd>
 Partial user signatures for future signing (Partial user signature ID -> PartialUserSignature)
</dd>
<dt>
<code>pricing_and_fee_management: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PricingAndFeeManagement">dwallet_2pc_mpc_coordinator_inner::PricingAndFeeManagement</a></code>
</dt>
<dd>
 Pricing and fee management
</dd>
<dt>
<code>active_committee: (ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">bls_committee::BlsCommittee</a></code>
</dt>
<dd>
 Current active validator committee
</dd>
<dt>
<code>previous_committee: (ika_system=0x0)::<a href="../ika_system/bls_committee.md#(ika_system=0x0)_bls_committee_BlsCommittee">bls_committee::BlsCommittee</a></code>
</dt>
<dd>
 Previous validator committee
</dd>
<dt>
<code>total_messages_processed: u64</code>
</dt>
<dd>
 Total number of messages processed
</dd>
<dt>
<code>last_processed_checkpoint_sequence_number: u64</code>
</dt>
<dd>
 Last processed checkpoint sequence number
</dd>
<dt>
<code>previous_epoch_last_checkpoint_sequence_number: u64</code>
</dt>
<dd>
 Last checkpoint sequence number from previous epoch
</dd>
<dt>
<code>support_config: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SupportConfig">dwallet_2pc_mpc_coordinator_inner::SupportConfig</a></code>
</dt>
<dd>
 Cryptographic algorithm support configuration
</dd>
<dt>
<code>received_end_of_publish: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>extra_fields: <a href="../sui/bag.md#sui_bag_Bag">sui::bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSession"></a>

## Struct `DWalletSession`

Represents an active MPC session in the Ika network.

Each session tracks fees and is associated with a network encryption key.
Sessions are sequentially numbered for epoch management.


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
<code>session_identifier: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">dwallet_2pc_mpc_coordinator_inner::SessionIdentifier</a></code>
</dt>
<dd>
 Session identifier
</dd>
<dt>
<code>session_sequence_number: u64</code>
</dt>
<dd>
 Sequential number for session ordering
</dd>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 Associated network encryption key
</dd>
<dt>
<code>consensus_validation_fee_charged_ika: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;</code>
</dt>
<dd>
 IKA fees for consensus validation
</dd>
<dt>
<code>computation_fee_charged_ika: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;</code>
</dt>
<dd>
 IKA fees for computation
</dd>
<dt>
<code>gas_fee_reimbursement_sui: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;</code>
</dt>
<dd>
 SUI balance for gas reimbursement
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap"></a>

## Struct `DWalletCap`

Capability granting control over a specific dWallet.

This capability allows the holder to perform operations on the associated dWallet,
such as requesting signatures, managing encryption keys, and approving messages.


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
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the controlled dWallet
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyDWalletCap"></a>

## Struct `ImportedKeyDWalletCap`

Capability granting control over a specific imported key dWallet.

Similar to DWalletCap but specifically for dWallets created from imported keys
rather than through the DKG process.


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
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the controlled imported key dWallet
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyCap"></a>

## Struct `DWalletNetworkEncryptionKeyCap`

Capability granting control over a specific dWallet network encryption key.

This capability allows management of network-level encryption keys used
for threshold encryption in the MPC protocols.


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
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the controlled network encryption key
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKey"></a>

## Struct `DWalletNetworkEncryptionKey`

Network-owned threshold encryption key for dWallet MPC protocols.

This key enables the validator network to securely store and manage encrypted
shares of dWallet secret keys. It supports reconfiguration across epochs to
maintain security as the validator set changes.


<a name="@Lifecycle_Phases_4"></a>

#### Lifecycle Phases



<a name="@Initial_Creation_5"></a>

##### Initial Creation

- Network DKG generates the initial threshold encryption key
- <code>network_dkg_public_output</code> contains the key and validator shares


<a name="@Reconfiguration_6"></a>

##### Reconfiguration

- Triggered before epoch transitions when validator set changes
- <code>reconfiguration_public_outputs</code> stores updated keys per epoch
- Ensures continuous security across validator set changes


<a name="@Data_Storage_Strategy_7"></a>

#### Data Storage Strategy

- Large cryptographic outputs are chunked due to storage limitations
- Chunked data is reconstructed during verification and usage
- Supports both initial DKG and ongoing reconfiguration outputs


<a name="@Security_Properties_8"></a>

#### Security Properties

- Threshold encryption protects against individual validator compromise
- Reconfiguration maintains security across validator set changes
- Cryptographic proofs ensure data integrity


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
 ID of the capability that controls this encryption key
</dd>
<dt>
<code>current_epoch: u64</code>
</dt>
<dd>
 Current epoch for this encryption key
</dd>
<dt>
<code>reconfiguration_public_outputs: <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, <a href="../sui/table_vec.md#sui_table_vec_TableVec">sui::table_vec::TableVec</a>&lt;vector&lt;u8&gt;&gt;&gt;</code>
</dt>
<dd>
 Reconfiguration outputs indexed by epoch (Epoch -> Chunked Output)
</dd>
<dt>
<code>network_dkg_public_output: <a href="../sui/table_vec.md#sui_table_vec_TableVec">sui::table_vec::TableVec</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
 Initial network DKG output (chunked for storage efficiency)
</dd>
<dt>
<code>computation_fee_charged_ika: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;(ika=0x0)::ika::IKA&gt;</code>
</dt>
<dd>
 IKA fees accumulated for computation services
</dd>
<dt>
<code>dkg_params_for_network: vector&lt;u8&gt;</code>
</dt>
<dd>
 Parameters for network dkg
</dd>
<dt>
<code>supported_curves: vector&lt;u32&gt;</code>
</dt>
<dd>
 Curves supported by this network encryption key
</dd>
<dt>
<code>state: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyState">dwallet_2pc_mpc_coordinator_inner::DWalletNetworkEncryptionKeyState</a></code>
</dt>
<dd>
 Current operational state
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptionKey"></a>

## Struct `EncryptionKey`

User encryption key for secure dWallet secret key share storage.

Encryption keys enable secure transfer and storage of encrypted user secret key shares
between accounts. Each user address has an associated encryption key that allows
others to encrypt data specifically for that user to ensure sensitive information
remains confidential during transmission.

Each address on the Ika is associated with a unique encryption key.
When a user intends to send encrypted data (i.e. when sharing the secret key share to grant access and/or transfer a dWallet) to another user,
they use the recipient's encryption key to encrypt the data.
The recipient is then the sole entity capable of decrypting and accessing this information, ensuring secure, end-to-end encryption.


<a name="@Security_Model_9"></a>

#### Security Model

- Keys are Ed25519-signed to prove authenticity
- Each address maintains one active encryption key
- Keys support various cryptographic curves
- Encrypted shares can only be decrypted by the key owner


<a name="@Use_Cases_10"></a>

#### Use Cases

- Encrypting user secret key shares during dWallet creation
- Re-encrypting shares for access transfer or dWallet sharing


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptionKey">EncryptionKey</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
 Unique identifier for this encryption key
</dd>
<dt>
<code>created_at_epoch: u64</code>
</dt>
<dd>
 Epoch when this key was created
</dd>
<dt>
<code>curve: u32</code>
</dt>
<dd>
 Cryptographic curve this key supports
</dd>
<dt>
<code>encryption_key: vector&lt;u8&gt;</code>
</dt>
<dd>
 Serialized encryption key data
</dd>
<dt>
<code>encryption_key_signature: vector&lt;u8&gt;</code>
</dt>
<dd>
 Ed25519 signature proving encryption key authenticity, signed by the <code>signer_public_key</code>.
 Used to verify the data originated from the <code>signer_address</code>.
</dd>
<dt>
<code>signer_public_key: vector&lt;u8&gt;</code>
</dt>
<dd>
 Ed25519 public key used to create the signature
</dd>
<dt>
<code>signer_address: <b>address</b></code>
</dt>
<dd>
 Address of the encryption key owner
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShare"></a>

## Struct `EncryptedUserSecretKeyShare`

Encrypted user secret key share with cryptographic verification.

Represents a user's secret key share that has been encrypted to a specific
user's encryption key. Includes zero-knowledge proofs that the encryption
is valid and corresponds to the dWallet's public key share.


<a name="@Verification_Process_11"></a>

#### Verification Process

1. Network verifies the encryption proof
2. User decrypts and verifies the share matches the public output
3. User signs the public output to accept the share


<a name="@Creation_Methods_12"></a>

#### Creation Methods

- **Direct**: Created during DKG second round
- **Re-encryption**: Created when transferring access to another user


<a name="@Security_Properties_13"></a>

#### Security Properties

- Zero-knowledge proof ensures encryption correctness
- Only the target user can decrypt the share
- Cryptographically linked to the associated dWallet


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
 Unique identifier for this encrypted share
</dd>
<dt>
<code>created_at_epoch: u64</code>
</dt>
<dd>
 Epoch when this share was created
</dd>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the dWallet this share belongs to
</dd>
<dt>
<code>encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;</code>
</dt>
<dd>
 Encrypted secret share with zero-knowledge proof of correctness
 for the dWallet's secret key share (of <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a></code>).
</dd>
<dt>
<code>encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the encryption key used for encryption
</dd>
<dt>
<code>encryption_key_address: <b>address</b></code>
</dt>
<dd>
 Address of the encryption key owner
</dd>
<dt>
<code>source_encrypted_user_secret_key_share_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;</code>
</dt>
<dd>
 Source share ID if this was created via re-encryption (None for DKG-created)
</dd>
<dt>
<code>state: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShareState">dwallet_2pc_mpc_coordinator_inner::EncryptedUserSecretKeyShareState</a></code>
</dt>
<dd>
 Current verification and acceptance state
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPartialUserSignatureCap"></a>

## Struct `UnverifiedPartialUserSignatureCap`

Unverified capability for a partial user signature requiring network validation.

This capability is issued when a user creates a partial signature but must be
verified by the network before it can be used for conditional signing.


<a name="@Verification_Process_14"></a>

#### Verification Process

1. Network validates the user's partial signature
2. Network verifies the signature matches the message and dWallet
3. Network confirms the presign material is valid
4. Capability becomes verified and ready for use


<a name="@Security_Properties_15"></a>

#### Security Properties

- Prevents use of invalid partial signatures
- Ensures network validation before conditional signing
- Capability-based authorization for future signing


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPartialUserSignatureCap">UnverifiedPartialUserSignatureCap</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
 Unique identifier for this capability
</dd>
<dt>
<code>partial_centralized_signed_message_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the associated partial user signature
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap"></a>

## Struct `VerifiedPartialUserSignatureCap`

Verified capability for a network-validated partial user signature.

This capability proves that:
- The user's partial signature has been validated by the network
- The signature matches the intended message and dWallet
- The associated presign material is valid and reserved
- The holder is authorized to request signature completion


<a name="@Usage_in_Conditional_Signing_16"></a>

#### Usage in Conditional Signing

- Can be combined with <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">MessageApproval</a></code> to complete signatures
- Enables conditional execution when multiple conditions are met
- Supports atomic multi-party transactions


<a name="@Security_Guarantees_17"></a>

#### Security Guarantees

- Network has verified the partial signature authenticity
- Presign material is reserved and cannot be double-spent
- Only the capability holder can trigger signature completion


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap">VerifiedPartialUserSignatureCap</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
 Unique identifier for this capability
</dd>
<dt>
<code>partial_centralized_signed_message_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the associated verified partial user signature
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignature"></a>

## Struct `PartialUserSignature`

Partial user signature for future/conditional signing scenarios.

Represents a message that has been signed by the user (centralized party) but not
yet by the network. This enables conditional signing patterns where user consent
is obtained first, and network signing occurs later when conditions are met.


<a name="@Use_Cases_18"></a>

#### Use Cases



<a name="@Decentralized_Exchange_(DEX)_19"></a>

##### Decentralized Exchange (DEX)

1. User A creates a partial signature to buy BTC with ETH at price X
2. User B creates a matching partial signature to sell BTC for ETH at price X
3. When both conditions are met, the network completes both signatures
4. Atomic swap is executed


<a name="@Conditional_Payments_20"></a>

##### Conditional Payments

- Pre-authorize payments that execute when specific conditions are met
- Escrow-like functionality with delayed execution
- Multi-party agreement protocols


<a name="@Security_Properties_21"></a>

#### Security Properties

- User signature proves intent and authorization
- Presign capability ensures single-use semantics
- Network verification prevents malicious signatures
- Capability-based access control for completion


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignature">PartialUserSignature</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
 Unique identifier for this partial signature
</dd>
<dt>
<code>created_at_epoch: u64</code>
</dt>
<dd>
 Epoch when this partial signature was created
</dd>
<dt>
<code>presign_cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">dwallet_2pc_mpc_coordinator_inner::VerifiedPresignCap</a></code>
</dt>
<dd>
 Presign capability (consumed to prevent reuse)
</dd>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the dWallet that will complete the signature
</dd>
<dt>
<code>cap_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the capability that controls completion
</dd>
<dt>
<code>curve: u32</code>
</dt>
<dd>
 Cryptographic curve for the signature
</dd>
<dt>
<code>signature_algorithm: u32</code>
</dt>
<dd>
 Signature algorithm to be used
</dd>
<dt>
<code>hash_scheme: u32</code>
</dt>
<dd>
 Hash scheme to apply to the message
</dd>
<dt>
<code>message: vector&lt;u8&gt;</code>
</dt>
<dd>
 Raw message bytes to be signed
</dd>
<dt>
<code>message_centralized_signature: vector&lt;u8&gt;</code>
</dt>
<dd>
 User's partial signature on the message
</dd>
<dt>
<code>state: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignatureState">dwallet_2pc_mpc_coordinator_inner::PartialUserSignatureState</a></code>
</dt>
<dd>
 Current verification state
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet"></a>

## Struct `DWallet`

Represents a decentralized wallet (dWallet) created through DKG or key import.

A dWallet encapsulates cryptographic key material and provides secure signing
capabilities through Multi-Party Computation. It can operate in two security models:

1. **Zero-trust mode**: User secret key share remains encrypted, requiring user
participation for every signature. Maximum security.
2. **Trust-minimized mode**: User secret key share is made public, allowing
network-only signing. Reduced security but improved UX.


<a name="@Security_Models_22"></a>

#### Security Models

- **DKG dWallets**: Created through distributed key generation
- **Imported Key dWallets**: Created from existing private keys


<a name="@State_Lifecycle_23"></a>

#### State Lifecycle

The dWallet progresses through various states from creation to active use,
with different paths for DKG and imported key variants.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
 Unique identifier for the dWallet
</dd>
<dt>
<code>created_at_epoch: u64</code>
</dt>
<dd>
 Epoch when this dWallet was created
</dd>
<dt>
<code>curve: u32</code>
</dt>
<dd>
 Elliptic curve used for cryptographic operations
</dd>
<dt>
<code>public_user_secret_key_share: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
 Public user secret key share (if trust-minimized mode is enabled)
 - <code>None</code>: Zero-trust mode - user participation required for signing
 - <code>Some(share)</code>: Trust-minimized mode - network can sign independently
</dd>
<dt>
<code>dwallet_cap_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the capability that controls this dWallet
</dd>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 Network encryption key used for securing this dWallet's network share
</dd>
<dt>
<code>is_imported_key_dwallet: bool</code>
</dt>
<dd>
 Whether this dWallet was created from an imported key
</dd>
<dt>
<code>encrypted_user_secret_key_shares: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShare">dwallet_2pc_mpc_coordinator_inner::EncryptedUserSecretKeyShare</a>&gt;</code>
</dt>
<dd>
 Encrypted user secret key shares (Encryption user secret key share ID -> EncryptedUserSecretKeyShare)
</dd>
<dt>
<code>sign_sessions: <a href="../sui/object_table.md#sui_object_table_ObjectTable">sui::object_table::ObjectTable</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignSession">dwallet_2pc_mpc_coordinator_inner::SignSession</a>&gt;</code>
</dt>
<dd>
 Signing sessions (Sign ID -> SignSession)
</dd>
<dt>
<code>state: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletState">dwallet_2pc_mpc_coordinator_inner::DWalletState</a></code>
</dt>
<dd>
 Current state of the dWallet
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap"></a>

## Struct `UnverifiedPresignCap`

Unverified capability for a presign session requiring validation.

This capability is issued when a presign is requested but must be verified
as completed before it can be used for signing operations.


<a name="@Verification_Process_24"></a>

#### Verification Process

1. Check that the referenced presign session is completed
2. Validate capability ID matches the session
3. Convert to <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">VerifiedPresignCap</a></code> for actual use


<a name="@Security_Model_25"></a>

#### Security Model

- Cannot be used for signing until verified
- Prevents use of incomplete or invalid presigns
- Capability-based access control


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
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;</code>
</dt>
<dd>
 Target dWallet ID for dWallet-specific presigns
 - <code>Some(id)</code>: Can only be used with the specified dWallet (e.g. ECDSA requirement)
 - <code>None</code>: Global presign, can be used with any compatible dWallet (e.g. Schnorr and EdDSA)
</dd>
<dt>
<code>presign_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the associated presign session
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap"></a>

## Struct `VerifiedPresignCap`

Verified capability for a completed presign session ready for signing.

This capability proves that:
- The associated presign session has completed successfully
- The capability holder has authorization to use the presign
- The presign matches the cryptographic requirements


<a name="@Usage_Constraints_26"></a>

#### Usage Constraints

- Single-use: Consumed during signature generation
- Algorithm-specific: Must match the target signature algorithm
- Expiration: May have epoch-based validity limits


<a name="@Security_Properties_27"></a>

#### Security Properties

- Cryptographically bound to specific presign output
- Prevents double-spending of presign material
- Enforces proper authorization flow


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
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;</code>
</dt>
<dd>
 Target dWallet ID for dWallet-specific presigns
 - <code>Some(id)</code>: Can only be used with the specified dWallet (e.g. ECDSA requirement)
 - <code>None</code>: Global presign, can be used with any compatible dWallet (e.g. Schnorr and EdDSA)
</dd>
<dt>
<code>presign_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the associated presign session
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignSession"></a>

## Struct `PresignSession`

Presign session for optimized signature generation.

Presigns are cryptographic precomputations that enable faster online signing
by performing expensive computations offline, before the message is known.
This significantly reduces signing latency in real-time applications.


<a name="@Types_of_Presigns_28"></a>

#### Types of Presigns



<a name="@dWallet-Specific_Presigns_29"></a>

##### dWallet-Specific Presigns

- Bound to a specific dWallet ID
- Required for algorithms like ECDSA
- Higher security isolation


<a name="@Global_Presigns_30"></a>

##### Global Presigns

- Can be used with any dWallet under the same network key
- Supported by algorithms like Schnorr and EdDSA
- Better resource efficiency


<a name="@Performance_Benefits_31"></a>

#### Performance Benefits

- Reduces online full signing flow time significantly
- Enables high-frequency trading and real-time applications
- Improves user experience with instant signatures


<a name="@Security_Properties_32"></a>

#### Security Properties

- Single-use: Each presign can only be used once
- Algorithm-specific: Tailored to the signature algorithm
- Network-secured: Protected by threshold cryptography


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignSession">PresignSession</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
 Unique identifier for this presign session
</dd>
<dt>
<code>created_at_epoch: u64</code>
</dt>
<dd>
 Epoch when this presign was created
</dd>
<dt>
<code>curve: u32</code>
</dt>
<dd>
 Elliptic curve used for the presign
</dd>
<dt>
<code>signature_algorithm: u32</code>
</dt>
<dd>
 Signature algorithm this presign supports
</dd>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;</code>
</dt>
<dd>
 Target dWallet ID (None for global presigns)
 - <code>Some(id)</code>: dWallet-specific presign (e.g. required for ECDSA)
 - <code>None</code>: Global presign (e.g. available for Schnorr, EdDSA)
</dd>
<dt>
<code>cap_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the capability that controls this presign
</dd>
<dt>
<code>state: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignState">dwallet_2pc_mpc_coordinator_inner::PresignState</a></code>
</dt>
<dd>
 Current state of the presign computation
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignSession"></a>

## Struct `SignSession`

Signing session for generating dWallet signatures.

Represents an ongoing or completed signature generation process using
the 2PC-MPC protocol. Combines user and network contributions to create
a complete signature.


<a name="@Signing_Process_33"></a>

#### Signing Process

1. User provides message approval and presign capability
2. Network validates the request and user's partial signature
3. Network combines with its share to generate the full signature
4. Session transitions to completed state with the final signature


<a name="@Types_of_Signing_34"></a>

#### Types of Signing

- **Standard**: Direct signing with immediate user participation
- **Future**: Conditional signing using pre-validated partial signatures
- **Imported Key**: Signing with imported key dWallets


<a name="@Performance_Optimization_35"></a>

#### Performance Optimization

- Uses presign material to accelerate the online signing process
- Reduces latency from seconds to milliseconds for real-time applications
- Enables high-frequency trading and interactive applications


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
 Epoch when this signing session was initiated
</dd>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the dWallet performing the signature
</dd>
<dt>
<code>state: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignState">dwallet_2pc_mpc_coordinator_inner::SignState</a></code>
</dt>
<dd>
 Current state of the signing process
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier"></a>

## Struct `SessionIdentifier`

The preimage is used to create the session identifier.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">SessionIdentifier</a> <b>has</b> key, store
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
<code>identifier_preimage: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval"></a>

## Struct `MessageApproval`

Authorization to sign a specific message with a dWallet.

This approval object grants permission to sign a message using a dWallet's
secret key material. It specifies the exact cryptographic parameters and
message content that has been authorized.


<a name="@Security_Properties_36"></a>

#### Security Properties

- Single-use: Consumed during signature generation to prevent replay
- Cryptographically bound: Specifies exact algorithm and hash scheme
- Message-specific: Tied to specific message content
- dWallet-specific: Can only be used with the designated dWallet


<a name="@Usage_Pattern_37"></a>

#### Usage Pattern

1. User creates approval for specific message and dWallet
2. Approval is combined with presign capability
3. Network validates and generates signature
4. Approval is consumed and cannot be reused


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">MessageApproval</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the dWallet authorized to sign this message
</dd>
<dt>
<code>signature_algorithm: u32</code>
</dt>
<dd>
 Cryptographic signature algorithm to use
</dd>
<dt>
<code>hash_scheme: u32</code>
</dt>
<dd>
 Hash scheme to apply to the message before signing
</dd>
<dt>
<code>message: vector&lt;u8&gt;</code>
</dt>
<dd>
 Raw message bytes to be signed
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyMessageApproval"></a>

## Struct `ImportedKeyMessageApproval`

Authorization to sign a specific message with an imported key dWallet.

Similar to <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">MessageApproval</a></code> but specifically for dWallets created from
imported private keys rather than through distributed key generation.


<a name="@Differences_from_Standard_MessageApproval_38"></a>

#### Differences from Standard MessageApproval

- Used with <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyDWalletCap">ImportedKeyDWalletCap</a></code> instead of <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">DWalletCap</a></code>
- May have different security assumptions due to key import process
- Supports the same cryptographic algorithms and operations


<a name="@Security_Considerations_39"></a>

#### Security Considerations

- Imported key dWallets may have different trust models
- Users should understand the provenance of imported keys
- Same single-use and message-binding properties apply


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyMessageApproval">ImportedKeyMessageApproval</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the imported key dWallet authorized to sign this message
</dd>
<dt>
<code>signature_algorithm: u32</code>
</dt>
<dd>
 Cryptographic signature algorithm to use
</dd>
<dt>
<code>hash_scheme: u32</code>
</dt>
<dd>
 Hash scheme to apply to the message before signing
</dd>
<dt>
<code>message: vector&lt;u8&gt;</code>
</dt>
<dd>
 Raw message bytes to be signed
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifierRegisteredEvent"></a>

## Struct `SessionIdentifierRegisteredEvent`

Event emitted when a session identifier is registered.

This event signals that a new session identifier has been registered and is
ready for use in the dWallet system.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifierRegisteredEvent">SessionIdentifierRegisteredEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>session_object_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the session object
</dd>
<dt>
<code>session_identifier_preimage: vector&lt;u8&gt;</code>
</dt>
<dd>
 Unique session identifier
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSessionEvent"></a>

## Struct `DWalletSessionEvent`

Generic wrapper for dWallet-related events with session context.

Provides standardized metadata for all dWallet operations including
epoch information, session type, and session ID for tracking and debugging.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSessionEvent">DWalletSessionEvent</a>&lt;E: <b>copy</b>, drop, store&gt; <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>epoch: u64</code>
</dt>
<dd>
 Epoch when the event occurred
</dd>
<dt>
<code>session_object_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the session object
</dd>
<dt>
<code>session_type: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionType">dwallet_2pc_mpc_coordinator_inner::SessionType</a></code>
</dt>
<dd>
 Type of session (User or System)
</dd>
<dt>
<code>session_identifier_preimage: vector&lt;u8&gt;</code>
</dt>
<dd>
 Unique session identifier
</dd>
<dt>
<code>event_data: E</code>
</dt>
<dd>
 Event-specific data
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSessionResultEvent"></a>

## Struct `DWalletSessionResultEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSessionResultEvent">DWalletSessionResultEvent</a>&lt;Success: <b>copy</b>, drop, store, Rejected: <b>copy</b>, drop, store&gt; <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>session_identifier_preimage: vector&lt;u8&gt;</code>
</dt>
<dd>
 The identifier of the session
</dd>
<dt>
<code>status: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSessionStatusEvent">dwallet_2pc_mpc_coordinator_inner::DWalletSessionStatusEvent</a>&lt;Success, Rejected&gt;</code>
</dt>
<dd>
 The status of the event
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDKGEncryptionKeyRequestEvent"></a>

## Struct `DWalletNetworkDKGEncryptionKeyRequestEvent`

Event requesting network DKG for a new encryption key.

Initiates the distributed key generation process for creating a new
network threshold encryption key used by the validator committee.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDKGEncryptionKeyRequestEvent">DWalletNetworkDKGEncryptionKeyRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the network encryption key to be generated
</dd>
<dt>
<code>params_for_network: vector&lt;u8&gt;</code>
</dt>
<dd>
 Parameters for the network
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletNetworkDKGEncryptionKeyEvent"></a>

## Struct `CompletedDWalletNetworkDKGEncryptionKeyEvent`

Event emitted when network DKG for an encryption key completes successfully.

Signals that the validator network has successfully generated a new
threshold encryption key and it's ready for use in securing dWallet shares.


<a name="@Next_Steps_40"></a>

#### Next Steps

The encryption key can now be used for:
- Encrypting dWallet network shares
- Securing validator committee communications
- Supporting MPC protocol operations


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletNetworkDKGEncryptionKeyEvent">CompletedDWalletNetworkDKGEncryptionKeyEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the successfully generated network encryption key
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletNetworkDKGEncryptionKeyEvent"></a>

## Struct `RejectedDWalletNetworkDKGEncryptionKeyEvent`

Event emitted when network DKG for an encryption key is rejected.

Indicates that the validator network could not complete the DKG process
for the requested encryption key, typically due to insufficient participation
or validation failures.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletNetworkDKGEncryptionKeyEvent">RejectedDWalletNetworkDKGEncryptionKeyEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the rejected network encryption key
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletEncryptionKeyReconfigurationRequestEvent"></a>

## Struct `DWalletEncryptionKeyReconfigurationRequestEvent`

Event requesting reconfiguration of a network encryption key.

Initiates the process to update a network encryption key for a new
validator committee, ensuring continuity of service across epoch transitions.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletEncryptionKeyReconfigurationRequestEvent">DWalletEncryptionKeyReconfigurationRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the network encryption key to be reconfigured
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletEncryptionKeyReconfigurationEvent"></a>

## Struct `CompletedDWalletEncryptionKeyReconfigurationEvent`

Event emitted when encryption key reconfiguration completes successfully.

Signals that the network encryption key has been successfully updated
for the new validator committee and is ready for the next epoch.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletEncryptionKeyReconfigurationEvent">CompletedDWalletEncryptionKeyReconfigurationEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the successfully reconfigured network encryption key
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletEncryptionKeyReconfigurationEvent"></a>

## Struct `RejectedDWalletEncryptionKeyReconfigurationEvent`

Event emitted when encryption key reconfiguration is rejected.

Indicates that the validator network could not complete the reconfiguration
process, potentially requiring retry or manual intervention.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletEncryptionKeyReconfigurationEvent">RejectedDWalletEncryptionKeyReconfigurationEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the network encryption key that failed reconfiguration
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGFirstRoundRequestEvent"></a>

## Struct `DWalletDKGFirstRoundRequestEvent`

Event requesting the start of DKG first round from the validator network.

Initiates the distributed key generation process for a new dWallet.
Validators respond by executing the first round of the DKG protocol.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGFirstRoundRequestEvent">DWalletDKGFirstRoundRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the dWallet being created
</dd>
<dt>
<code>dwallet_cap_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the capability that controls the dWallet
</dd>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 Network encryption key for securing the dWallet's network share
</dd>
<dt>
<code>curve: u32</code>
</dt>
<dd>
 Elliptic curve for the dWallet's cryptographic operations
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletDKGFirstRoundEvent"></a>

## Struct `CompletedDWalletDKGFirstRoundEvent`

Event emitted when DKG first round completes successfully.

Signals that the validator network has completed the first round of DKG
and provides the output needed for the user to proceed with the second round.


<a name="@Next_Steps_41"></a>

#### Next Steps

Users should:
1. Process the <code>first_round_output</code>
2. Generate their contribution to the DKG
3. Call <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_dkg_second_round">request_dwallet_dkg_second_round</a>()</code> to continue


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletDKGFirstRoundEvent">CompletedDWalletDKGFirstRoundEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the dWallet being created
</dd>
<dt>
<code>first_round_output: vector&lt;u8&gt;</code>
</dt>
<dd>
 Public output from the first round of DKG
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletDKGFirstRoundEvent"></a>

## Struct `RejectedDWalletDKGFirstRoundEvent`

Event emitted when DKG first round is rejected by the network.

Indicates that the validator network could not complete the first round
of DKG for the requested dWallet, typically due to validation failures
or insufficient validator participation.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletDKGFirstRoundEvent">RejectedDWalletDKGFirstRoundEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the dWallet whose DKG first round was rejected
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGSecondRoundRequestEvent"></a>

## Struct `DWalletDKGSecondRoundRequestEvent`

Event requesting the second round of DKG from the validator network.

This event initiates the final phase of distributed key generation where
the user's contribution is combined with the network's first round output
to complete the dWallet creation process.


<a name="@Process_Flow_42"></a>

#### Process Flow

1. User processes the first round output from validators
2. User generates their cryptographic contribution
3. User encrypts their secret key share
4. Network validates and completes the DKG process


<a name="@Security_Properties_43"></a>

#### Security Properties

- User contribution ensures the user controls part of the key
- Network validation prevents malicious key generation
- Encrypted shares ensure proper key distribution


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGSecondRoundRequestEvent">DWalletDKGSecondRoundRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the encrypted user secret key share being created
</dd>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the dWallet being created through DKG
</dd>
<dt>
<code>first_round_output: vector&lt;u8&gt;</code>
</dt>
<dd>
 Cryptographic output from the network's first round of DKG
</dd>
<dt>
<code>centralized_public_key_share_and_proof: vector&lt;u8&gt;</code>
</dt>
<dd>
 User's public key share with cryptographic proof of correctness
</dd>
<dt>
<code>dwallet_cap_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the dWallet capability that authorizes this operation
</dd>
<dt>
<code>encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;</code>
</dt>
<dd>
 User's encrypted secret key share with zero-knowledge proof
</dd>
<dt>
<code>encryption_key: vector&lt;u8&gt;</code>
</dt>
<dd>
 Serialized encryption key used to encrypt the user's secret share
</dd>
<dt>
<code>encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the encryption key object
</dd>
<dt>
<code>encryption_key_address: <b>address</b></code>
</dt>
<dd>
 Address of the encryption key owner
</dd>
<dt>
<code>user_public_output: vector&lt;u8&gt;</code>
</dt>
<dd>
 User's contribution to the DKG public output
</dd>
<dt>
<code>signer_public_key: vector&lt;u8&gt;</code>
</dt>
<dd>
 Ed25519 public key for verifying the user's signature
</dd>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the network encryption key for securing network shares
</dd>
<dt>
<code>curve: u32</code>
</dt>
<dd>
 Elliptic curve for the dWallet's cryptographic operations
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletDKGSecondRoundEvent"></a>

## Struct `CompletedDWalletDKGSecondRoundEvent`

Event emitted when DKG second round completes successfully.

Signals the successful completion of the distributed key generation process.
The dWallet is now ready for user acceptance and can begin signing operations
once the user validates and accepts their encrypted key share.


<a name="@Next_Steps_for_Users_44"></a>

#### Next Steps for Users

1. Validate the public output matches expected values
2. Decrypt and verify the received encrypted key share
3. Sign the public output to accept the dWallet
4. Begin using the dWallet for signing operations


<a name="@Security_Verification_45"></a>

#### Security Verification

Users should verify that the public key corresponds to their expected
contribution and that the encrypted share can be properly decrypted.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletDKGSecondRoundEvent">CompletedDWalletDKGSecondRoundEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the successfully created dWallet
</dd>
<dt>
<code>public_output: vector&lt;u8&gt;</code>
</dt>
<dd>
 Complete public output from the DKG process (public key and metadata)
</dd>
<dt>
<code>encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the user's encrypted secret key share
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletDKGSecondRoundEvent"></a>

## Struct `RejectedDWalletDKGSecondRoundEvent`

Event emitted when DKG second round is rejected by the network.

Indicates that the validator network rejected the user's contribution
to the DKG process, typically due to invalid proofs or malformed data.


<a name="@Common_Rejection_Reasons_46"></a>

#### Common Rejection Reasons

- Invalid cryptographic proofs
- Malformed user contribution
- Encryption verification failures
- Network consensus issues


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletDKGSecondRoundEvent">RejectedDWalletDKGSecondRoundEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the dWallet whose DKG second round was rejected
</dd>
<dt>
<code>public_output: vector&lt;u8&gt;</code>
</dt>
<dd>
 Public output that was being processed when rejection occurred
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletImportedKeyVerificationRequestEvent"></a>

## Struct `DWalletImportedKeyVerificationRequestEvent`

Event requesting verification of an imported key dWallet from the network.

This event initiates the validation process for a dWallet created from an
existing private key rather than through distributed key generation.


<a name="@Imported_Key_Flow_47"></a>

#### Imported Key Flow

1. User creates an imported key dWallet object
2. User provides cryptographic proof of key ownership
3. Network validates the proof and key authenticity
4. If valid, the dWallet becomes active for signing


<a name="@Security_Considerations_48"></a>

#### Security Considerations

- Imported keys may have different security assumptions than DKG keys
- Network validates proof of ownership but cannot verify key generation process
- Users should understand the provenance and security of imported keys


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletImportedKeyVerificationRequestEvent">DWalletImportedKeyVerificationRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the imported key dWallet being verified
</dd>
<dt>
<code>encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the encrypted user secret key share being created
</dd>
<dt>
<code>centralized_party_message: vector&lt;u8&gt;</code>
</dt>
<dd>
 User's cryptographic message for importing computation
</dd>
<dt>
<code>dwallet_cap_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the imported key dWallet capability
</dd>
<dt>
<code>encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;</code>
</dt>
<dd>
 User's encrypted secret key share with proof of correctness
</dd>
<dt>
<code>encryption_key: vector&lt;u8&gt;</code>
</dt>
<dd>
 Serialized encryption key used for user share encryption
</dd>
<dt>
<code>encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the encryption key object
</dd>
<dt>
<code>encryption_key_address: <b>address</b></code>
</dt>
<dd>
 Address of the encryption key owner
</dd>
<dt>
<code>user_public_output: vector&lt;u8&gt;</code>
</dt>
<dd>
 User's public key contribution and verification data
</dd>
<dt>
<code>signer_public_key: vector&lt;u8&gt;</code>
</dt>
<dd>
 Ed25519 public key for signature verification, used to verify the user's signature on the public output
</dd>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the network encryption key for securing network shares
</dd>
<dt>
<code>curve: u32</code>
</dt>
<dd>
 Elliptic curve for the imported key dWallet
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletImportedKeyVerificationEvent"></a>

## Struct `CompletedDWalletImportedKeyVerificationEvent`

Event emitted when imported key verification completes successfully.

Signals that the network has validated the user's imported key and the
dWallet is ready for user acceptance and subsequent signing operations.


<a name="@Next_Steps_for_Users_49"></a>

#### Next Steps for Users

1. Verify the public output matches the imported key
2. Validate the encrypted key share can be properly decrypted
3. Sign the public output to accept the dWallet
4. Begin using the imported key dWallet for signatures


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletImportedKeyVerificationEvent">CompletedDWalletImportedKeyVerificationEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the successfully verified imported key dWallet
</dd>
<dt>
<code>public_output: vector&lt;u8&gt;</code>
</dt>
<dd>
 Public output from the verification process
</dd>
<dt>
<code>encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the user's encrypted secret key share
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletImportedKeyVerificationEvent"></a>

## Struct `RejectedDWalletImportedKeyVerificationEvent`

Event emitted when imported key verification is rejected by the network.

Indicates that the validator network could not validate the imported key,
typically due to invalid proofs or malformed verification data.


<a name="@Common_Rejection_Reasons_50"></a>

#### Common Rejection Reasons

- Invalid cryptographic proofs of key ownership
- Malformed imported key data
- Verification signature failures
- Incompatible curve parameters


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletImportedKeyVerificationEvent">RejectedDWalletImportedKeyVerificationEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the imported key dWallet that failed verification
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CreatedEncryptionKeyEvent"></a>

## Struct `CreatedEncryptionKeyEvent`

Event emitted when an encryption key is successfully created and registered.

This event signals that a new encryption key has been validated and is available
for use in encrypting user secret key shares.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CreatedEncryptionKeyEvent">CreatedEncryptionKeyEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the newly created encryption key
</dd>
<dt>
<code>signer_address: <b>address</b></code>
</dt>
<dd>
 Address of the encryption key owner
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedShareVerificationRequestEvent"></a>

## Struct `EncryptedShareVerificationRequestEvent`

Event requesting verification of an encrypted user secret key share.

This event initiates the validation process for re-encrypted user shares,
typically used when transferring dWallet access to another user or when
creating additional encrypted copies for backup purposes.


<a name="@Re-encryption_Use_Cases_51"></a>

#### Re-encryption Use Cases

- **Access Transfer**: Share dWallet access with another user
- **Access Granting**: Allow multiple users to control the same dWallet
- **Backup Creation**: Create additional encrypted copies for redundancy
- **Key Recovery**: Re-encrypt shares for recovery scenarios


<a name="@Verification_Process_52"></a>

#### Verification Process

1. User re-encrypts their secret key share to a new encryption key
2. User provides zero-knowledge proof of correct re-encryption
3. Network validates the proof against the dWallet's public output
4. If valid, the new encrypted share becomes available for use


<a name="@Security_Properties_53"></a>

#### Security Properties

- Zero-knowledge proofs ensure re-encryption correctness
- Original share remains secure during the process
- Network cannot learn the secret key material
- Destination user must decrypt and validate the share


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedShareVerificationRequestEvent">EncryptedShareVerificationRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;</code>
</dt>
<dd>
 User's encrypted secret key share with zero-knowledge proof of correctness
</dd>
<dt>
<code>public_output: vector&lt;u8&gt;</code>
</dt>
<dd>
 Public output of the dWallet (used for verification), this is the
 public output of the dWallet that the user's share is being encrypted to.
 This value is taken from the the dWallet object during event creation, and
 we cannot get it from the user's side.
</dd>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the dWallet this encrypted share belongs to
</dd>
<dt>
<code>encryption_key: vector&lt;u8&gt;</code>
</dt>
<dd>
 Serialized encryption key used for the re-encryption
</dd>
<dt>
<code>encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the encryption key object
</dd>
<dt>
<code>encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the new encrypted user secret key share being created
</dd>
<dt>
<code>source_encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the source encrypted share (if this is a re-encryption)
</dd>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the network encryption key securing network shares
</dd>
<dt>
<code>curve: u32</code>
</dt>
<dd>
 Elliptic curve for the dWallet
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedEncryptedShareVerificationEvent"></a>

## Struct `CompletedEncryptedShareVerificationEvent`

Event emitted when encrypted share verification completes successfully.

Signals that the network has validated the re-encryption proof and the
new encrypted share is ready for the destination user to accept.


<a name="@Next_Steps_for_Recipient_54"></a>

#### Next Steps for Recipient

1. Decrypt the encrypted share using their private encryption key
2. Verify the decrypted share matches the dWallet's public output
3. Sign the public output to accept and activate the share
4. Use the share for dWallet operations


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedEncryptedShareVerificationEvent">CompletedEncryptedShareVerificationEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the successfully verified encrypted user secret key share
</dd>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the dWallet associated with this encrypted share
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedEncryptedShareVerificationEvent"></a>

## Struct `RejectedEncryptedShareVerificationEvent`

Event emitted when encrypted share verification is rejected.

Indicates that the network could not validate the re-encryption proof,
typically due to invalid cryptographic proofs or verification failures.


<a name="@Common_Rejection_Reasons_55"></a>

#### Common Rejection Reasons

- Invalid zero-knowledge proof of re-encryption
- Mismatch between encrypted share and public output
- Corrupted or malformed encryption data
- Incompatible encryption key parameters


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedEncryptedShareVerificationEvent">RejectedEncryptedShareVerificationEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the encrypted user secret key share that failed verification
</dd>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the dWallet associated with the failed share
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_AcceptEncryptedUserShareEvent"></a>

## Struct `AcceptEncryptedUserShareEvent`

Event emitted when a user accepts an encrypted secret key share.

This event signals the final step in the share transfer process where
the recipient has validated and accepted their encrypted share, making
the dWallet fully accessible to them.


<a name="@Acceptance_Process_56"></a>

#### Acceptance Process

1. User decrypts the share with their private encryption key
2. User verifies the share produces the correct public key
3. User signs the public output to prove acceptance
4. Share becomes active and usable for signing operations


<a name="@Security_Verification_57"></a>

#### Security Verification

The user's signature on the public output serves as cryptographic proof that:
- They successfully decrypted the share
- The share is mathematically correct
- They accept responsibility for the dWallet


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_AcceptEncryptedUserShareEvent">AcceptEncryptedUserShareEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the accepted encrypted user secret key share
</dd>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the dWallet associated with this share
</dd>
<dt>
<code>user_output_signature: vector&lt;u8&gt;</code>
</dt>
<dd>
 User's signature on the public output proving acceptance
</dd>
<dt>
<code>encryption_key_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the encryption key used for this share
</dd>
<dt>
<code>encryption_key_address: <b>address</b></code>
</dt>
<dd>
 Address of the user who accepted the share
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MakeDWalletUserSecretKeySharePublicRequestEvent"></a>

## Struct `MakeDWalletUserSecretKeySharePublicRequestEvent`

Event requesting to make a dWallet's user secret key share public.

This event initiates the transition from zero-trust mode to trust-minimized mode,
where the user's secret key share becomes publicly visible, allowing the network
to sign independently without user participation.


<a name="@⚠️_CRITICAL_SECURITY_WARNING_58"></a>

#### ⚠️ CRITICAL SECURITY WARNING

**This operation is IRREVERSIBLE and reduces security!**


<a name="@Security_Trade-offs_59"></a>

##### Security Trade-offs

- **Before**: Zero-trust - user participation required for every signature
- **After**: Trust-minimized - network can sign independently
- **Risk**: Compromised validators could potentially misuse the dWallet


<a name="@When_to_Consider_This_60"></a>

##### When to Consider This

- High-frequency automated trading where latency is critical
- Applications requiring instant signature generation
- When convenience outweighs the security reduction
- Smart contract automation that needs independent signing


<a name="@Use_Cases_61"></a>

##### Use Cases

- DeFi protocols with automated rebalancing
- Gaming applications with instant transactions
- IoT devices requiring autonomous signing
- Bot trading with microsecond latency requirements


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MakeDWalletUserSecretKeySharePublicRequestEvent">MakeDWalletUserSecretKeySharePublicRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>public_user_secret_key_share: vector&lt;u8&gt;</code>
</dt>
<dd>
 The user's secret key share to be made public
</dd>
<dt>
<code>public_output: vector&lt;u8&gt;</code>
</dt>
<dd>
 dWallet's public output for verification
</dd>
<dt>
<code>curve: u32</code>
</dt>
<dd>
 Elliptic curve for the dWallet
</dd>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the dWallet being transitioned to trust-minimized mode
</dd>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the network encryption key
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedMakeDWalletUserSecretKeySharePublicEvent"></a>

## Struct `CompletedMakeDWalletUserSecretKeySharePublicEvent`

Event emitted when user secret key share is successfully made public.

Signals that the dWallet has transitioned to trust-minimized mode where
the network can now sign independently without user participation.


<a name="@Post-Transition_Capabilities_62"></a>

#### Post-Transition Capabilities

- Network can generate signatures autonomously
- Reduced latency for signing operations
- No user interaction required for each signature
- Suitable for high-frequency automated applications


<a name="@⚠️_Security_Reminder_63"></a>

#### ⚠️ Security Reminder

The dWallet now operates in trust-minimized mode. Monitor validator
behavior and consider the implications for your security model.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedMakeDWalletUserSecretKeySharePublicEvent">CompletedMakeDWalletUserSecretKeySharePublicEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the dWallet that successfully transitioned to trust-minimized mode
</dd>
<dt>
<code>public_user_secret_key_share: vector&lt;u8&gt;</code>
</dt>
<dd>
 The user's secret key share that was made public
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedMakeDWalletUserSecretKeySharePublicEvent"></a>

## Struct `RejectedMakeDWalletUserSecretKeySharePublicEvent`

Event emitted when the request to make user secret key share public is rejected.

Indicates that the network could not validate or complete the transition
to trust-minimized mode.


<a name="@Common_Rejection_Reasons_64"></a>

#### Common Rejection Reasons

- Invalid user secret key share provided
- Mismatch between share and public output
- dWallet already in trust-minimized mode
- Network validation failures


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedMakeDWalletUserSecretKeySharePublicEvent">RejectedMakeDWalletUserSecretKeySharePublicEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the dWallet that failed to transition to trust-minimized mode
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignRequestEvent"></a>

## Struct `PresignRequestEvent`

Event requesting the generation of a presign from the validator network.

This event initiates the precomputation of cryptographic material that will
be used to accelerate future signature generation. Presigns are a key
optimization in the 2PC-MPC protocol, reducing online signing time by 80-90%.


<a name="@Presign_Types_65"></a>

#### Presign Types



<a name="@dWallet-Specific_Presigns_66"></a>

##### dWallet-Specific Presigns

- Required for algorithms like ECDSA that need key-specific precomputation
- Bound to a specific dWallet and cannot be used elsewhere
- Higher security isolation but less resource efficiency


<a name="@Global_Presigns_67"></a>

##### Global Presigns

- Supported by algorithms like Schnorr and EdDSA
- Can be used with any compatible dWallet under the same network key
- Better resource utilization and batching efficiency


<a name="@Performance_Benefits_68"></a>

#### Performance Benefits

- **Latency Reduction**: From seconds to milliseconds for signing
- **Throughput Increase**: Enables high-frequency trading applications
- **User Experience**: Near-instant signature generation
- **Scalability**: Batch presign generation during low activity periods


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignRequestEvent">PresignRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;</code>
</dt>
<dd>
 Target dWallet ID for dWallet-specific presigns
 - <code>Some(id)</code>: dWallet-specific presign (required for ECDSA)
 - <code>None</code>: Global presign (available for Schnorr, EdDSA)
</dd>
<dt>
<code>presign_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 Unique identifier for this presign session
</dd>
<dt>
<code>dwallet_public_output: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
 dWallet's public output for verification (None for global presigns)
</dd>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the network encryption key securing the presign
</dd>
<dt>
<code>curve: u32</code>
</dt>
<dd>
 Elliptic curve for the presign computation
</dd>
<dt>
<code>signature_algorithm: u32</code>
</dt>
<dd>
 Signature algorithm for the presign (determines presign type)
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedPresignEvent"></a>

## Struct `CompletedPresignEvent`

Event emitted when a presign generation completes successfully.

Signals that the validator network has successfully generated the
cryptographic precomputation material and it's ready for use in
accelerated signature generation.


<a name="@Next_Steps_69"></a>

#### Next Steps

1. User receives a <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">VerifiedPresignCap</a></code> capability
2. Presign can be combined with message approval for fast signing
3. Single-use: Each presign can only be used once
4. Expiration: Presigns may have validity time limits


<a name="@Security_Properties_70"></a>

#### Security Properties

- Cryptographically bound to specific algorithm and curve
- Cannot be used for different signature types
- Single-use prevents double-spending of presign material
- Network validation ensures correctness


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedPresignEvent">CompletedPresignEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;</code>
</dt>
<dd>
 Target dWallet ID (None for global presigns)
</dd>
<dt>
<code>presign_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 Unique identifier for the completed presign
</dd>
<dt>
<code>presign: vector&lt;u8&gt;</code>
</dt>
<dd>
 Precomputed cryptographic material for signature acceleration
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedPresignEvent"></a>

## Struct `RejectedPresignEvent`

Event emitted when presign generation is rejected by the network.

Indicates that the validator network could not complete the presign
generation, typically due to validation failures or resource constraints.


<a name="@Common_Rejection_Reasons_71"></a>

#### Common Rejection Reasons

- Insufficient validator participation
- Invalid cryptographic parameters
- Network resource constraints
- Validation failures during precomputation
- Incompatible algorithm/curve combinations


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedPresignEvent">RejectedPresignEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;</code>
</dt>
<dd>
 Target dWallet ID (None for global presigns)
</dd>
<dt>
<code>presign_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the presign that failed generation
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignRequestEvent"></a>

## Struct `SignRequestEvent`

Event requesting signature generation from the validator network.

This event initiates the final phase of the 2PC-MPC signing protocol where
the network combines user authorization with precomputed material to generate
a complete cryptographic signature.


<a name="@Signing_Process_Flow_72"></a>

#### Signing Process Flow

1. User provides message approval and presign capability
2. Network validates the user's authorization
3. Network combines presign with user's partial signature
4. Complete signature is generated and returned


<a name="@Signature_Types_73"></a>

#### Signature Types



<a name="@Standard_Signing_(<code>is_future_sign:_<b>false</b></code>)_74"></a>

##### Standard Signing (<code>is_future_sign: <b>false</b></code>)

- Immediate user participation required
- User signature computed in real-time
- Highest security with fresh user authorization


<a name="@Future_Signing_(<code>is_future_sign:_<b>true</b></code>)_75"></a>

##### Future Signing (<code>is_future_sign: <b>true</b></code>)

- Uses pre-validated partial user signatures
- Enables conditional and delayed execution
- Supports complex multi-party transaction patterns


<a name="@Performance_Optimization_76"></a>

#### Performance Optimization

- Presign material enables sub-second signature generation
- Critical for high-frequency trading and real-time applications
- Reduces network round-trips and computational overhead


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignRequestEvent">SignRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>sign_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 Unique identifier for this signing session
</dd>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the dWallet performing the signature
</dd>
<dt>
<code>dwallet_public_output: vector&lt;u8&gt;</code>
</dt>
<dd>
 dWallet's public output for signature verification
</dd>
<dt>
<code>curve: u32</code>
</dt>
<dd>
 Elliptic curve for the signature
</dd>
<dt>
<code>signature_algorithm: u32</code>
</dt>
<dd>
 Cryptographic signature algorithm
</dd>
<dt>
<code>hash_scheme: u32</code>
</dt>
<dd>
 Hash scheme applied to the message
</dd>
<dt>
<code>message: vector&lt;u8&gt;</code>
</dt>
<dd>
 Raw message bytes to be signed
</dd>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the network encryption key securing network shares
</dd>
<dt>
<code>presign_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the presign used for acceleration
</dd>
<dt>
<code>presign: vector&lt;u8&gt;</code>
</dt>
<dd>
 Precomputed cryptographic material for fast signing
</dd>
<dt>
<code>message_centralized_signature: vector&lt;u8&gt;</code>
</dt>
<dd>
 User's partial signature on the message
</dd>
<dt>
<code>is_future_sign: bool</code>
</dt>
<dd>
 Whether this uses future sign capabilities
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedSignEvent"></a>

## Struct `CompletedSignEvent`

Event emitted when signature generation completes successfully.

This event signals the successful completion of the 2PC-MPC signing protocol
and provides the final cryptographic signature that can be used in transactions.


<a name="@Signature_Properties_77"></a>

#### Signature Properties

- **Mathematically Valid**: Verifiable against the dWallet's public key
- **Cryptographically Secure**: Generated using threshold cryptography
- **Single-Use Presign**: Associated presign material is consumed
- **User Authorized**: Includes validated user consent


<a name="@Next_Steps_78"></a>

#### Next Steps

1. Extract the signature from the event
2. Combine with transaction data for blockchain submission
3. Verify signature matches expected format for target blockchain
4. Submit transaction to the destination network


<a name="@Performance_Metrics_79"></a>

#### Performance Metrics

With presigns, signature generation typically completes in:
- **Standard Networks**: 100-500ms
- **High-Performance Setup**: 50-100ms
- **Without Presigns**: 2-5 seconds


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedSignEvent">CompletedSignEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>sign_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 Unique identifier for the completed signing session
</dd>
<dt>
<code>signature: vector&lt;u8&gt;</code>
</dt>
<dd>
 Complete cryptographic signature ready for use
</dd>
<dt>
<code>is_future_sign: bool</code>
</dt>
<dd>
 Whether this signature used future sign capabilities
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedSignEvent"></a>

## Struct `RejectedSignEvent`

Event emitted when signature generation is rejected by the network.

Indicates that the validator network could not complete the signature
generation, typically due to validation failures or protocol errors.


<a name="@Common_Rejection_Reasons_80"></a>

#### Common Rejection Reasons

- **Invalid Presign**: Presign material is corrupted or expired
- **Authorization Failure**: User signature validation failed
- **Network Issues**: Insufficient validator participation
- **Protocol Errors**: Cryptographic validation failures
- **Resource Constraints**: Network overload or rate limiting


<a name="@Recovery_Steps_81"></a>

#### Recovery Steps

1. Check presign validity and obtain new presign if needed
2. Verify message approval is correctly formatted
3. Ensure dWallet is in active state
4. Retry with fresh authorization if temporary failure


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedSignEvent">RejectedSignEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>sign_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the signing session that failed
</dd>
<dt>
<code>is_future_sign: bool</code>
</dt>
<dd>
 Whether this rejection involved future sign capabilities
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_FutureSignRequestEvent"></a>

## Struct `FutureSignRequestEvent`

Event requesting validation of a partial user signature for future signing.

This event initiates the creation of a conditional signature capability where
the user's authorization is validated upfront but the network signature is
deferred until specific conditions are met.


<a name="@Future_Sign_Use_Cases_82"></a>

#### Future Sign Use Cases



<a name="@Decentralized_Exchange_(DEX)_Orders_83"></a>

##### Decentralized Exchange (DEX) Orders

```
1. User A: "I'll sell 1 BTC for 50,000 USDC"
2. User B: "I'll buy 1 BTC for 50,000 USDC"
3. When both conditions match → automatic execution
```


<a name="@Conditional_Payments_84"></a>

##### Conditional Payments

```
1. User: "Pay 1000 USDC to Alice when she delivers the goods"
2. Oracle confirms delivery → automatic payment
```


<a name="@Multi-Party_Atomic_Swaps_85"></a>

##### Multi-Party Atomic Swaps

```
1. Multiple users create conditional signatures
2. When all conditions are met → atomic execution
```


<a name="@Security_Benefits_86"></a>

#### Security Benefits

- User authorization is cryptographically committed upfront
- Network validation prevents invalid partial signatures
- Conditions can be verified before execution
- Atomic execution reduces counterparty risk


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_FutureSignRequestEvent">FutureSignRequestEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the dWallet that will complete the future signature
</dd>
<dt>
<code>partial_centralized_signed_message_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the partial user signature being validated
</dd>
<dt>
<code>message: vector&lt;u8&gt;</code>
</dt>
<dd>
 Message that will be signed when conditions are met
</dd>
<dt>
<code>presign: vector&lt;u8&gt;</code>
</dt>
<dd>
 Precomputed cryptographic material for the future signature
</dd>
<dt>
<code>dwallet_public_output: vector&lt;u8&gt;</code>
</dt>
<dd>
 dWallet's public output for verification
</dd>
<dt>
<code>curve: u32</code>
</dt>
<dd>
 Elliptic curve for the signature
</dd>
<dt>
<code>signature_algorithm: u32</code>
</dt>
<dd>
 Signature algorithm for the future signature
</dd>
<dt>
<code>hash_scheme: u32</code>
</dt>
<dd>
 Hash scheme to be applied to the message
</dd>
<dt>
<code>message_centralized_signature: vector&lt;u8&gt;</code>
</dt>
<dd>
 User's partial signature proving authorization
</dd>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the network encryption key
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedFutureSignEvent"></a>

## Struct `CompletedFutureSignEvent`

Event emitted when future sign validation completes successfully.

Signals that the network has validated the user's partial signature and
the future sign capability is ready for conditional execution.


<a name="@Next_Steps_87"></a>

#### Next Steps

1. User receives a <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap">VerifiedPartialUserSignatureCap</a></code>
2. Capability can be combined with <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">MessageApproval</a></code> for execution
3. Network will complete the signature when both are presented
4. Enables complex conditional signing workflows


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedFutureSignEvent">CompletedFutureSignEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the dWallet associated with the future signature
</dd>
<dt>
<code>partial_centralized_signed_message_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the validated partial user signature
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedFutureSignEvent"></a>

## Struct `RejectedFutureSignEvent`

Event emitted when future sign validation is rejected.

Indicates that the network could not validate the user's partial signature,
preventing the creation of the conditional signing capability.


<a name="@Common_Rejection_Reasons_88"></a>

#### Common Rejection Reasons

- Invalid user partial signature
- Mismatch between signature and message
- Incompatible presign material
- dWallet validation failures


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedFutureSignEvent">RejectedFutureSignEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the dWallet associated with the failed request
</dd>
<dt>
<code>partial_centralized_signed_message_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 ID of the partial user signature that failed validation
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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SetMaxActiveSessionsBufferEvent"></a>

## Struct `SetMaxActiveSessionsBufferEvent`

Event requesting to set the maximum number of active sessions buffer.

This event is used to configure the maximum number of active sessions that
can be created at any given time. This is used to prevent the network from
creating too many sessions and causing the validators to become out of sync.


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

Event requesting to set the gas fee reimbursement SUI system call value.

This event is used to configure the gas fee reimbursement SUI system call value.


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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyState"></a>

## Enum `DWalletNetworkEncryptionKeyState`

State of a dWallet network encryption key throughout its lifecycle


<pre><code><b>public</b> <b>enum</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyState">DWalletNetworkEncryptionKeyState</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Variants</summary>


<dl>
<dt>
Variant <code>AwaitingNetworkDKG</code>
</dt>
<dd>
 Waiting for network DKG to complete
</dd>
<dt>
Variant <code>NetworkDKGCompleted</code>
</dt>
<dd>
 Network DKG has completed successfully
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
 Network reconfiguration has completed successfully
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShareState"></a>

## Enum `EncryptedUserSecretKeyShareState`

State of an encrypted user secret key share throughout verification and acceptance


<pre><code><b>public</b> <b>enum</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShareState">EncryptedUserSecretKeyShareState</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Variants</summary>


<dl>
<dt>
Variant <code>AwaitingNetworkVerification</code>
</dt>
<dd>
 Waiting for network to verify the encryption proof
</dd>
<dt>
Variant <code>NetworkVerificationCompleted</code>
</dt>
<dd>
 Network has successfully verified the encryption
</dd>
<dt>
Variant <code>NetworkVerificationRejected</code>
</dt>
<dd>
 Network has rejected the encryption verification
</dd>
<dt>
Variant <code>KeyHolderSigned</code>
</dt>
<dd>
 Key holder has signed and accepted the share
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

State of a dWallet throughout its creation and operational lifecycle.

dWallets can be created through two paths:
1. **DKG Path**: Distributed Key Generation with validator participation
2. **Import Path**: Importing existing private keys with network verification

Both paths converge to the <code>Active</code> state where signing operations can be performed.


<pre><code><b>public</b> <b>enum</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletState">DWalletState</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Variants</summary>


<dl>
<dt>
Variant <code>DKGRequested</code>
</dt>
<dd>
 DKG first round has been requested from the network
</dd>
<dt>
Variant <code>NetworkRejectedDKGRequest</code>
</dt>
<dd>
 Network rejected the DKG first round request
</dd>
<dt>
Variant <code>AwaitingUserDKGVerificationInitiation</code>
</dt>
<dd>
 DKG first round completed, waiting for user to initiate second round
</dd>

<dl>
<dt>
<code>first_round_output: vector&lt;u8&gt;</code>
</dt>
<dd>
 Output from the first round of DKG
</dd>
</dl>

<dt>
Variant <code>AwaitingNetworkDKGVerification</code>
</dt>
<dd>
 DKG second round has been requested, waiting for network verification
</dd>
<dt>
Variant <code>NetworkRejectedDKGVerification</code>
</dt>
<dd>
 Network rejected the DKG second round verification
</dd>
<dt>
Variant <code>AwaitingNetworkImportedKeyVerification</code>
</dt>
<dd>
 Imported key verification requested, waiting for network verification
</dd>
<dt>
Variant <code>NetworkRejectedImportedKeyVerification</code>
</dt>
<dd>
 Network rejected the imported key verification
</dd>
<dt>
Variant <code>AwaitingKeyHolderSignature</code>
</dt>
<dd>
 DKG/Import completed, waiting for key holder to sign and accept
</dd>

<dl>
<dt>
<code>public_output: vector&lt;u8&gt;</code>
</dt>
<dd>
 Public output from DKG or import verification
</dd>
</dl>

<dt>
Variant <code>Active</code>
</dt>
<dd>
 dWallet is fully operational and ready for signing
</dd>

<dl>
<dt>
<code>public_output: vector&lt;u8&gt;</code>
</dt>
<dd>
 The verified public output
</dd>
</dl>

</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignState"></a>

## Enum `PresignState`

State progression of a presign session through its lifecycle.

Presign sessions follow a linear progression from request to completion,
with potential rejection at the network validation stage.


<pre><code><b>public</b> <b>enum</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignState">PresignState</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Variants</summary>


<dl>
<dt>
Variant <code>Requested</code>
</dt>
<dd>
 Presign has been requested and is awaiting network processing
</dd>
<dt>
Variant <code>NetworkRejected</code>
</dt>
<dd>
 Network rejected the presign request (invalid parameters, insufficient resources, etc.)
</dd>
<dt>
Variant <code>Completed</code>
</dt>
<dd>
 Presign completed successfully with cryptographic material ready for use
</dd>

<dl>
<dt>
<code>presign: vector&lt;u8&gt;</code>
</dt>
<dd>
 Precomputed cryptographic material for accelerated signing
</dd>
</dl>

</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignState"></a>

## Enum `SignState`

State progression of a signing session through its lifecycle.

Signing sessions combine user authorization with network cryptographic operations
to produce final signatures.


<pre><code><b>public</b> <b>enum</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignState">SignState</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Variants</summary>


<dl>
<dt>
Variant <code>Requested</code>
</dt>
<dd>
 Signature has been requested and is awaiting network processing
</dd>
<dt>
Variant <code>NetworkRejected</code>
</dt>
<dd>
 Network rejected the signature request (invalid presign, unauthorized message, etc.)
</dd>
<dt>
Variant <code>Completed</code>
</dt>
<dd>
 Signature completed successfully and ready for use
</dd>

<dl>
<dt>
<code>signature: vector&lt;u8&gt;</code>
</dt>
<dd>
 Final cryptographic signature that can be verified against the public key
</dd>
</dl>

</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionType"></a>

## Enum `SessionType`

Type of dWallet MPC session for scheduling and epoch management.

User-initiated sessions have sequence numbers for multi-epoch completion scheduling.
System sessions are guaranteed to complete within their creation epoch.


<pre><code><b>public</b> <b>enum</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionType">SessionType</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Variants</summary>


<dl>
<dt>
Variant <code>User</code>
</dt>
<dd>
 User-initiated session with sequence number for epoch scheduling
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
 System-initiated session (always completes in current epoch)
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSessionStatusEvent"></a>

## Enum `DWalletSessionStatusEvent`



<pre><code><b>public</b> <b>enum</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSessionStatusEvent">DWalletSessionStatusEvent</a>&lt;Success: <b>copy</b>, drop, store, Rejected: <b>copy</b>, drop, store&gt; <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Variants</summary>


<dl>
<dt>
Variant <code>Success</code>
</dt>
<dd>
 The event was successful
</dd>

<dl>
<dt>
<code>0: Success</code>
</dt>
<dd>
</dd>
</dl>

<dt>
Variant <code>Rejected</code>
</dt>
<dd>
 The event was rejected
</dd>

<dl>
<dt>
<code>0: Rejected</code>
</dt>
<dd>
</dd>
</dl>

</dl>


</details>

<a name="@Constants_89"></a>

## Constants


<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CHECKPOINT_MESSAGE_INTENT"></a>

Intent bytes for checkpoint message verification to prevent replay attacks


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CHECKPOINT_MESSAGE_INTENT">CHECKPOINT_MESSAGE_INTENT</a>: vector&lt;u8&gt; = vector[1, 0, 0];
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SESSION_IDENTIFIER_LENGTH"></a>

Session identifier length


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SESSION_IDENTIFIER_LENGTH">SESSION_IDENTIFIER_LENGTH</a>: u64 = 32;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DKG_FIRST_ROUND_PROTOCOL_FLAG"></a>

DKG first round protocol identifier


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DKG_FIRST_ROUND_PROTOCOL_FLAG">DKG_FIRST_ROUND_PROTOCOL_FLAG</a>: u32 = 0;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DKG_SECOND_ROUND_PROTOCOL_FLAG"></a>

DKG second round protocol identifier


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DKG_SECOND_ROUND_PROTOCOL_FLAG">DKG_SECOND_ROUND_PROTOCOL_FLAG</a>: u32 = 1;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RE_ENCRYPT_USER_SHARE_PROTOCOL_FLAG"></a>

User share re-encryption protocol identifier


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RE_ENCRYPT_USER_SHARE_PROTOCOL_FLAG">RE_ENCRYPT_USER_SHARE_PROTOCOL_FLAG</a>: u32 = 2;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MAKE_DWALLET_USER_SECRET_KEY_SHARE_PUBLIC_PROTOCOL_FLAG"></a>

Make user secret key share public protocol identifier


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MAKE_DWALLET_USER_SECRET_KEY_SHARE_PUBLIC_PROTOCOL_FLAG">MAKE_DWALLET_USER_SECRET_KEY_SHARE_PUBLIC_PROTOCOL_FLAG</a>: u32 = 3;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_IMPORTED_KEY_DWALLET_VERIFICATION_PROTOCOL_FLAG"></a>

Imported key dWallet verification protocol identifier


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_IMPORTED_KEY_DWALLET_VERIFICATION_PROTOCOL_FLAG">IMPORTED_KEY_DWALLET_VERIFICATION_PROTOCOL_FLAG</a>: u32 = 4;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PRESIGN_PROTOCOL_FLAG"></a>

Presign generation protocol identifier


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PRESIGN_PROTOCOL_FLAG">PRESIGN_PROTOCOL_FLAG</a>: u32 = 5;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SIGN_PROTOCOL_FLAG"></a>

Standard signing protocol identifier


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SIGN_PROTOCOL_FLAG">SIGN_PROTOCOL_FLAG</a>: u32 = 6;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_FUTURE_SIGN_PROTOCOL_FLAG"></a>

Future/conditional signing protocol identifier


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_FUTURE_SIGN_PROTOCOL_FLAG">FUTURE_SIGN_PROTOCOL_FLAG</a>: u32 = 7;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SIGN_WITH_PARTIAL_USER_SIGNATURE_PROTOCOL_FLAG"></a>

Signing with partial user signature protocol identifier


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SIGN_WITH_PARTIAL_USER_SIGNATURE_PROTOCOL_FLAG">SIGN_WITH_PARTIAL_USER_SIGNATURE_PROTOCOL_FLAG</a>: u32 = 8;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_DKG_FIRST_ROUND_OUTPUT_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_DKG_FIRST_ROUND_OUTPUT_MESSAGE_TYPE">RESPOND_DWALLET_DKG_FIRST_ROUND_OUTPUT_MESSAGE_TYPE</a>: u32 = 0;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_DKG_SECOND_ROUND_OUTPUT_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_DKG_SECOND_ROUND_OUTPUT_MESSAGE_TYPE">RESPOND_DWALLET_DKG_SECOND_ROUND_OUTPUT_MESSAGE_TYPE</a>: u32 = 1;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_ENCRYPTED_USER_SHARE_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_ENCRYPTED_USER_SHARE_MESSAGE_TYPE">RESPOND_DWALLET_ENCRYPTED_USER_SHARE_MESSAGE_TYPE</a>: u32 = 2;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_MAKE_DWALLET_USER_SECRET_KEY_SHARES_PUBLIC_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_MAKE_DWALLET_USER_SECRET_KEY_SHARES_PUBLIC_MESSAGE_TYPE">RESPOND_MAKE_DWALLET_USER_SECRET_KEY_SHARES_PUBLIC_MESSAGE_TYPE</a>: u32 = 3;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_IMPORTED_KEY_VERIFICATION_OUTPUT_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_IMPORTED_KEY_VERIFICATION_OUTPUT_MESSAGE_TYPE">RESPOND_DWALLET_IMPORTED_KEY_VERIFICATION_OUTPUT_MESSAGE_TYPE</a>: u32 = 4;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_PRESIGN_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_PRESIGN_MESSAGE_TYPE">RESPOND_DWALLET_PRESIGN_MESSAGE_TYPE</a>: u32 = 5;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_SIGN_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_SIGN_MESSAGE_TYPE">RESPOND_DWALLET_SIGN_MESSAGE_TYPE</a>: u32 = 6;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_PARTIAL_SIGNATURE_VERIFICATION_OUTPUT_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_PARTIAL_SIGNATURE_VERIFICATION_OUTPUT_MESSAGE_TYPE">RESPOND_DWALLET_PARTIAL_SIGNATURE_VERIFICATION_OUTPUT_MESSAGE_TYPE</a>: u32 = 7;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_MPC_NETWORK_DKG_OUTPUT_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_MPC_NETWORK_DKG_OUTPUT_MESSAGE_TYPE">RESPOND_DWALLET_MPC_NETWORK_DKG_OUTPUT_MESSAGE_TYPE</a>: u32 = 8;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_MPC_NETWORK_RECONFIGURATION_OUTPUT_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_MPC_NETWORK_RECONFIGURATION_OUTPUT_MESSAGE_TYPE">RESPOND_DWALLET_MPC_NETWORK_RECONFIGURATION_OUTPUT_MESSAGE_TYPE</a>: u32 = 9;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SET_MAX_ACTIVE_SESSIONS_BUFFER_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SET_MAX_ACTIVE_SESSIONS_BUFFER_MESSAGE_TYPE">SET_MAX_ACTIVE_SESSIONS_BUFFER_MESSAGE_TYPE</a>: u32 = 10;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SET_GAS_FEE_REIMBURSEMENT_SUI_SYSTEM_CALL_VALUE_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SET_GAS_FEE_REIMBURSEMENT_SUI_SYSTEM_CALL_VALUE_MESSAGE_TYPE">SET_GAS_FEE_REIMBURSEMENT_SUI_SYSTEM_CALL_VALUE_MESSAGE_TYPE</a>: u32 = 11;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_END_OF_EPOCH_MESSAGE_TYPE"></a>



<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_END_OF_EPOCH_MESSAGE_TYPE">END_OF_EPOCH_MESSAGE_TYPE</a>: u32 = 12;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletMismatch"></a>

dWallet parameters do not match expected values


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletMismatch">EDWalletMismatch</a>: u64 = 1;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletInactive"></a>

dWallet is not in active state for requested operation


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletInactive">EDWalletInactive</a>: u64 = 2;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNotExists"></a>

Referenced dWallet does not exist


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNotExists">EDWalletNotExists</a>: u64 = 3;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongState"></a>

Object is in wrong state for requested operation


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongState">EWrongState</a>: u64 = 4;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkEncryptionKeyNotExist"></a>

Referenced network encryption key does not exist


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkEncryptionKeyNotExist">EDWalletNetworkEncryptionKeyNotExist</a>: u64 = 5;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidEncryptionKeySignature"></a>

Encryption key signature verification failed


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidEncryptionKeySignature">EInvalidEncryptionKeySignature</a>: u64 = 6;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMessageApprovalMismatch"></a>

Message approval parameters do not match partial signature


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMessageApprovalMismatch">EMessageApprovalMismatch</a>: u64 = 7;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidHashScheme"></a>

Specified hash scheme is not supported


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidHashScheme">EInvalidHashScheme</a>: u64 = 8;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ESignWrongState"></a>

Signing session is in wrong state


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ESignWrongState">ESignWrongState</a>: u64 = 9;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EPresignNotExist"></a>

Referenced presign does not exist


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EPresignNotExist">EPresignNotExist</a>: u64 = 10;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EIncorrectCap"></a>

Capability does not match expected object


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EIncorrectCap">EIncorrectCap</a>: u64 = 11;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EUnverifiedCap"></a>

Capability has not been verified


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EUnverifiedCap">EUnverifiedCap</a>: u64 = 12;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidSource"></a>

Invalid source for re-encryption operation


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidSource">EInvalidSource</a>: u64 = 13;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkEncryptionKeyNotActive"></a>

Network encryption key is not in active state


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkEncryptionKeyNotActive">EDWalletNetworkEncryptionKeyNotActive</a>: u64 = 14;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidPresign"></a>

Presign is invalid or incomplete


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidPresign">EInvalidPresign</a>: u64 = 15;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ECannotAdvanceEpoch"></a>

Cannot advance epoch due to incomplete sessions


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ECannotAdvanceEpoch">ECannotAdvanceEpoch</a>: u64 = 16;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidCurve"></a>

Specified cryptographic curve is not supported


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidCurve">EInvalidCurve</a>: u64 = 17;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidSignatureAlgorithm"></a>

Specified signature algorithm is not supported


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidSignatureAlgorithm">EInvalidSignatureAlgorithm</a>: u64 = 18;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ECurvePaused"></a>

Cryptographic curve is temporarily paused


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ECurvePaused">ECurvePaused</a>: u64 = 19;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ESignatureAlgorithmPaused"></a>

Signature algorithm is temporarily paused


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ESignatureAlgorithmPaused">ESignatureAlgorithmPaused</a>: u64 = 20;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletUserSecretKeySharesAlreadyPublic"></a>

dWallet user secret key shares are already public


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletUserSecretKeySharesAlreadyPublic">EDWalletUserSecretKeySharesAlreadyPublic</a>: u64 = 21;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMismatchCurve"></a>

Cryptographic curve mismatch between objects


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMismatchCurve">EMismatchCurve</a>: u64 = 22;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EImportedKeyDWallet"></a>

Operation not allowed on imported key dWallet


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EImportedKeyDWallet">EImportedKeyDWallet</a>: u64 = 23;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ENotImportedKeyDWallet"></a>

Operation requires imported key dWallet


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ENotImportedKeyDWallet">ENotImportedKeyDWallet</a>: u64 = 24;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EHashSchemePaused"></a>

Hash scheme is temporarily paused


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EHashSchemePaused">EHashSchemePaused</a>: u64 = 25;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EEncryptionKeyNotExist"></a>

Referenced encryption key does not exist


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EEncryptionKeyNotExist">EEncryptionKeyNotExist</a>: u64 = 26;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMissingProtocolPricing"></a>

Pricing configuration missing for protocol


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMissingProtocolPricing">EMissingProtocolPricing</a>: u64 = 27;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EPricingCalculationVotesHasNotBeenStarted"></a>

Pricing calculation votes have not been initiated


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EPricingCalculationVotesHasNotBeenStarted">EPricingCalculationVotesHasNotBeenStarted</a>: u64 = 28;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EPricingCalculationVotesMustBeCompleted"></a>

Pricing calculation votes must complete before epoch advance


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EPricingCalculationVotesMustBeCompleted">EPricingCalculationVotesMustBeCompleted</a>: u64 = 29;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ECannotSetDuringVotesCalculation"></a>

Cannot modify settings during active pricing calculation


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ECannotSetDuringVotesCalculation">ECannotSetDuringVotesCalculation</a>: u64 = 30;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInsufficientIKAPayment"></a>

Insufficient IKA payment


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInsufficientIKAPayment">EInsufficientIKAPayment</a>: u64 = 31;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInsufficientSUIPayment"></a>

Insufficient SUI payment


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInsufficientSUIPayment">EInsufficientSUIPayment</a>: u64 = 32;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ENetworkEncryptionKeyUnsupportedCurve"></a>

Network encryption key does not support the requested curve


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ENetworkEncryptionKeyUnsupportedCurve">ENetworkEncryptionKeyUnsupportedCurve</a>: u64 = 33;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ESessionIdentifierAlreadyRegistered"></a>

Session identifier already registered


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ESessionIdentifierAlreadyRegistered">ESessionIdentifierAlreadyRegistered</a>: u64 = 34;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ESessionIdentifierNotExist"></a>

Session identifier does not exist


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ESessionIdentifierNotExist">ESessionIdentifierNotExist</a>: u64 = 35;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ESessionIdentifierInvalidLength"></a>

Session identifier is invalid


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ESessionIdentifierInvalidLength">ESessionIdentifierInvalidLength</a>: u64 = 36;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EIncorrectEpochInCheckpoint"></a>

The checkpoint epoch is incorrect


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EIncorrectEpochInCheckpoint">EIncorrectEpochInCheckpoint</a>: u64 = 37;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongCheckpointSequenceNumber"></a>

The checkpoint sequence number should be the expected next one


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongCheckpointSequenceNumber">EWrongCheckpointSequenceNumber</a>: u64 = 38;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EActiveBlsCommitteeMustInitialize"></a>

First active committee must initialize


<pre><code><b>const</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EActiveBlsCommitteeMustInitialize">EActiveBlsCommitteeMustInitialize</a>: u64 = 39;
</code></pre>



<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_create_dwallet_coordinator_inner"></a>

## Function `create_dwallet_coordinator_inner`

Creates a new DWalletCoordinatorInner instance with initial configuration.

Validates that pricing exists for all supported protocols and curves before creation.
Initializes all internal data structures with default values.


<a name="@Parameters_90"></a>

##### Parameters

- <code>current_epoch</code>: Starting epoch number
- <code>active_committee</code>: Initial validator committee
- <code>pricing</code>: Default pricing configuration
- <code>supported_curves_to_signature_algorithms_to_hash_schemes</code>: Supported cryptographic configurations
- <code>ctx</code>: Transaction context for object creation


<a name="@Returns_91"></a>

##### Returns

A new DWalletCoordinatorInner instance ready for use


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
            registered_session_identifiers: table::new(ctx),
            sessions: object_table::new(ctx),
            session_events: bag::new(ctx),
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
        last_processed_checkpoint_sequence_number: 0,
        previous_epoch_last_checkpoint_sequence_number: 0,
        support_config: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SupportConfig">SupportConfig</a> {
            supported_curves_to_signature_algorithms_to_hash_schemes,
            paused_curves: vector[],
            paused_signature_algorithms: vector[],
            paused_hash_schemes: vector[],
            signature_algorithms_allowed_global_presign: vector[],
        },
        received_end_of_publish: <b>true</b>,
        extra_fields: bag::new(ctx),
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_lock_last_active_session_sequence_number"></a>

## Function `lock_last_active_session_sequence_number`

Locks the last active session sequence number to prevent further updates.

This function is called before epoch transitions to ensure session scheduling
stability during the epoch switch process.


<a name="@Parameters_92"></a>

##### Parameters

- <code>self</code>: Mutable reference to the coordinator


<a name="@Effects_93"></a>

##### Effects

- Prevents further updates to <code>last_user_initiated_session_to_complete_in_current_epoch</code>
- Ensures session completion targets remain stable during epoch transitions


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_lock_last_active_session_sequence_number">lock_last_active_session_sequence_number</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_lock_last_active_session_sequence_number">lock_last_active_session_sequence_number</a>(self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>) {
    self.session_management.locked_last_user_initiated_session_to_complete_in_current_epoch = <b>true</b>;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_register_session_identifier"></a>

## Function `register_session_identifier`

Registers a new session identifier.

This function is used to register a new session identifier.


<a name="@Parameters_94"></a>

##### Parameters

- <code>self</code>: Mutable reference to the coordinator.
- <code>identifier_preimage</code>: The preimage bytes for creating the session identifier.
- <code>ctx</code>: Transaction context for object creation.


<a name="@Returns_95"></a>

##### Returns

A new session identifier object.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_register_session_identifier">register_session_identifier</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, identifier_preimage: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">dwallet_2pc_mpc_coordinator_inner::SessionIdentifier</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_register_session_identifier">register_session_identifier</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    identifier_preimage: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext,
): <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">SessionIdentifier</a> {
    <b>assert</b>!(identifier_preimage.length() == <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SESSION_IDENTIFIER_LENGTH">SESSION_IDENTIFIER_LENGTH</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ESessionIdentifierInvalidLength">ESessionIdentifierInvalidLength</a>);
    <b>assert</b>!(!self.session_management.registered_session_identifiers.contains(identifier_preimage), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ESessionIdentifierAlreadyRegistered">ESessionIdentifierAlreadyRegistered</a>);
    <b>let</b> id = object::new(ctx);
    self.session_management.registered_session_identifiers.add(identifier_preimage, id.to_inner());
    event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifierRegisteredEvent">SessionIdentifierRegisteredEvent</a> {
        session_object_id: id.to_inner(),
        session_identifier_preimage: identifier_preimage,
    });
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">SessionIdentifier</a> {
        id,
        identifier_preimage,
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_network_encryption_key_dkg"></a>

## Function `request_dwallet_network_encryption_key_dkg`

Starts a Distributed Key Generation (DKG) session for the network (threshold) encryption key.

Creates a new network encryption key and initiates the DKG process through the validator network.
Returns a capability that grants control over the created encryption key.


<a name="@Parameters_96"></a>

##### Parameters

- <code>self</code>: Mutable reference to the coordinator
- <code>ctx</code>: Transaction context for object creation


<a name="@Returns_97"></a>

##### Returns

A capability granting control over the new network encryption key


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_network_encryption_key_dkg">request_dwallet_network_encryption_key_dkg</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, params_for_network: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyCap">dwallet_2pc_mpc_coordinator_inner::DWalletNetworkEncryptionKeyCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_network_encryption_key_dkg">request_dwallet_network_encryption_key_dkg</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    params_for_network: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyCap">DWalletNetworkEncryptionKeyCap</a> {
    // Create a new capability to control this encryption key.
    <b>let</b> id = object::new(ctx);
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a> = id.to_inner();
    <b>let</b> cap = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyCap">DWalletNetworkEncryptionKeyCap</a> {
        id: object::new(ctx),
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
    };
    // Create a new network encryption key and add it to the shared state.
    self.dwallet_network_encryption_keys.add(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKey">DWalletNetworkEncryptionKey</a> {
        id,
        dwallet_network_encryption_key_cap_id: object::id(&cap),
        current_epoch: self.current_epoch,
        reconfiguration_public_outputs: <a href="../sui/table.md#sui_table_new">sui::table::new</a>(ctx),
        network_dkg_public_output: table_vec::empty(ctx),
        computation_fee_charged_ika: balance::zero(),
        dkg_params_for_network: params_for_network,
        supported_curves: vector::empty(),
        state: DWalletNetworkEncryptionKeyState::AwaitingNetworkDKG,
    });
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_initiate_system_dwallet_session">initiate_system_dwallet_session</a>(
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDKGEncryptionKeyRequestEvent">DWalletNetworkDKGEncryptionKeyRequestEvent</a> {
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
            params_for_network,
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

Charges gas fee reimbursement for system-initiated operations.

Allocates SUI from the coordinator's gas reimbursement pool to cover
transaction costs for system operations like network DKG and reconfiguration.


<a name="@Parameters_98"></a>

##### Parameters

- <code>self</code>: Mutable reference to the coordinator


<a name="@Returns_99"></a>

##### Returns

SUI balance to reimburse gas costs for system operations


<a name="@Logic_100"></a>

##### Logic

- Returns zero if no reimbursement funds or value configured
- Takes the minimum of available funds and configured system call value
- Ensures system operations don't exhaust the entire reimbursement pool


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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_handle_completed_system_session"></a>

## Function `handle_completed_system_session`



<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_handle_completed_system_session">handle_completed_system_session</a>&lt;E: <b>copy</b>, drop, store&gt;(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, session_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_handle_completed_system_session">handle_completed_system_session</a>&lt;E: <b>copy</b> + drop + store&gt;(self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>, session_id: ID) {
    self.session_management.completed_system_sessions_count = self.session_management.completed_system_sessions_count + 1;
    <b>let</b> _: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSessionEvent">DWalletSessionEvent</a>&lt;E&gt; = self.session_management.session_events.remove(session_id);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_network_encryption_key_dkg"></a>

## Function `respond_dwallet_network_encryption_key_dkg`

Complete the Distributed Key Generation (DKG) session
and store the public output corresponding to the newly created network (threshold) encryption key.

Note: assumes the public output is divided into chunks and each <code>network_public_output_chunk</code> is delivered in order,
with <code>is_last_chunk</code> set for the last call.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_network_encryption_key_dkg">respond_dwallet_network_encryption_key_dkg</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, session_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, network_public_output_chunk: vector&lt;u8&gt;, supported_curves: vector&lt;u32&gt;, is_last_chunk: bool, rejected: bool, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_network_encryption_key_dkg">respond_dwallet_network_encryption_key_dkg</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    session_id: ID,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: ID,
    network_public_output_chunk: vector&lt;u8&gt;,
    supported_curves: vector&lt;u32&gt;,
    is_last_chunk: bool,
    rejected: bool,
    ctx: &<b>mut</b> TxContext,
): Balance&lt;SUI&gt; {
    <b>if</b> (is_last_chunk) {
        self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_handle_completed_system_session">handle_completed_system_session</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDKGEncryptionKeyRequestEvent">DWalletNetworkDKGEncryptionKeyRequestEvent</a>&gt;(session_id);
        <b>let</b> dwallet_network_encryption_key = self.dwallet_network_encryption_keys.borrow_mut(
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>
        );
        dwallet_network_encryption_key.supported_curves = supported_curves;
    };
    <b>let</b> dwallet_network_encryption_key = self.dwallet_network_encryption_keys.borrow_mut(
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>
    );
    <b>if</b> (rejected) {
        dwallet_network_encryption_key.state = DWalletNetworkEncryptionKeyState::AwaitingNetworkDKG;
        // TODO(@scaly): should we empty dwallet_network_encryption_key.network_dkg_public_output?
        event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletNetworkDKGEncryptionKeyEvent">RejectedDWalletNetworkDKGEncryptionKeyEvent</a> {
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
        });
        self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_initiate_system_dwallet_session">initiate_system_dwallet_session</a>(
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkDKGEncryptionKeyRequestEvent">DWalletNetworkDKGEncryptionKeyRequestEvent</a> {
                <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
                params_for_network: dwallet_network_encryption_key.dkg_params_for_network,
            },
            ctx,
        );
    } <b>else</b> {
        dwallet_network_encryption_key.network_dkg_public_output.push_back(network_public_output_chunk);
        dwallet_network_encryption_key.state = match (&dwallet_network_encryption_key.state) {
            DWalletNetworkEncryptionKeyState::AwaitingNetworkDKG =&gt; {
            <b>if</b> (is_last_chunk) {
                event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletNetworkDKGEncryptionKeyEvent">CompletedDWalletNetworkDKGEncryptionKeyEvent</a> {
                    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
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


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_network_encryption_key_reconfiguration">respond_dwallet_network_encryption_key_reconfiguration</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, session_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, public_output: vector&lt;u8&gt;, supported_curves: vector&lt;u32&gt;, is_last_chunk: bool, rejected: bool, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_network_encryption_key_reconfiguration">respond_dwallet_network_encryption_key_reconfiguration</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    session_id: ID,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: ID,
    public_output: vector&lt;u8&gt;,
    supported_curves: vector&lt;u32&gt;,
    is_last_chunk: bool,
    rejected: bool,
    ctx: &<b>mut</b> TxContext,
): Balance&lt;SUI&gt; {
    // The Reconfiguration output can be large, so it is seperated into chunks.
    // We should only update the count once, so we check it is the last chunk before we do.
    <b>if</b> (is_last_chunk) {
        self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_handle_completed_system_session">handle_completed_system_session</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletEncryptionKeyReconfigurationRequestEvent">DWalletEncryptionKeyReconfigurationRequestEvent</a>&gt;(session_id);
        <b>let</b> dwallet_network_encryption_key = self.dwallet_network_encryption_keys.borrow_mut(
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>
        );
        dwallet_network_encryption_key.supported_curves = supported_curves;
    };
    <b>let</b> dwallet_network_encryption_key = self.dwallet_network_encryption_keys.borrow_mut(
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>
    );
    // Store this chunk <b>as</b> the last chunk in the chunks vector corresponding to the upcoming's epoch in the <b>public</b> outputs map.
    <b>if</b> (rejected) {
        dwallet_network_encryption_key.state = match (&dwallet_network_encryption_key.state) {
            DWalletNetworkEncryptionKeyState::AwaitingNetworkReconfiguration { is_first } =&gt; {
                DWalletNetworkEncryptionKeyState::AwaitingNetworkReconfiguration { is_first: *is_first }
            },
            _ =&gt; DWalletNetworkEncryptionKeyState::AwaitingNetworkReconfiguration { is_first: <b>false</b> }
        };
        // TODO(@scaly): should we empty next_reconfiguration_public_output?
        event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletEncryptionKeyReconfigurationEvent">RejectedDWalletEncryptionKeyReconfigurationEvent</a> {
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
        });
        self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_initiate_system_dwallet_session">initiate_system_dwallet_session</a>(
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletEncryptionKeyReconfigurationRequestEvent">DWalletEncryptionKeyReconfigurationRequestEvent</a> {
                <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
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
                            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
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
        cap.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>
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
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    cap: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyCap">DWalletNetworkEncryptionKeyCap</a>,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(self.dwallet_network_encryption_keys.contains(cap.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkEncryptionKeyNotExist">EDWalletNetworkEncryptionKeyNotExist</a>);
    <b>let</b> dwallet_network_encryption_key = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_network_encryption_key">get_active_dwallet_network_encryption_key</a>(cap.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>);
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
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: cap.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
        },
        ctx,
    );
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_network_encryption_key"></a>

## Function `get_active_dwallet_network_encryption_key`



<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_network_encryption_key">get_active_dwallet_network_encryption_key</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKey">dwallet_2pc_mpc_coordinator_inner::DWalletNetworkEncryptionKey</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_network_encryption_key">get_active_dwallet_network_encryption_key</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: ID,
): &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKey">DWalletNetworkEncryptionKey</a> {
    <b>let</b> dwallet_network_encryption_key = self.dwallet_network_encryption_keys.borrow_mut(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>);
    <b>assert</b>!(dwallet_network_encryption_key.state != DWalletNetworkEncryptionKeyState::AwaitingNetworkDKG, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkEncryptionKeyNotActive">EDWalletNetworkEncryptionKeyNotActive</a>);
    dwallet_network_encryption_key
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_advance_epoch"></a>

## Function `advance_epoch`

Advances the coordinator to the next epoch with comprehensive state transitions.

Performs a complete epoch transition including session management updates,
committee transitions, and network encryption key advancement. This is a
critical operation that must be executed atomically.


<a name="@Parameters_101"></a>

##### Parameters

- <code>self</code>: Mutable reference to the coordinator
- <code>next_committee</code>: New validator committee for the upcoming epoch
- <code>dwallet_network_encryption_key_caps</code>: Capabilities for network encryption keys to advance


<a name="@Returns_102"></a>

##### Returns

Combined IKA balance from fees collected during the epoch


<a name="@Effects_103"></a>

##### Effects

- Validates all current epoch sessions are completed
- Updates session management metadata for the next epoch
- Transitions validator committees (current -> previous, next -> current)
- Advances network encryption key epochs
- Unlocks session sequence number management
- Increments the current epoch counter
- Collects and returns accumulated fees


<a name="@Aborts_104"></a>

##### Aborts

- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EPricingCalculationVotesMustBeCompleted">EPricingCalculationVotesMustBeCompleted</a></code>: If pricing votes are still in progress
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ECannotAdvanceEpoch">ECannotAdvanceEpoch</a></code>: If not all current epoch sessions are completed
- Various network encryption key related errors from capability validation


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
    <b>assert</b>!(self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_all_current_epoch_sessions_completed">all_current_epoch_sessions_completed</a>(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ECannotAdvanceEpoch">ECannotAdvanceEpoch</a>);
    // We advance the first epoch `0` immediately during initialization, the network doesn't participate in it and therefore, it did not send an `END_OF_PUBLISH`. For any other epoch, don't advance before the network sent an `END_OF_PUBLISH`.
    <b>assert</b>!(self.received_end_of_publish, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ECannotAdvanceEpoch">ECannotAdvanceEpoch</a>);
    self.received_end_of_publish = <b>false</b>;
    self.previous_epoch_last_checkpoint_sequence_number = self.last_processed_checkpoint_sequence_number;
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

Gets an immutable reference to a dWallet by ID.


<a name="@Parameters_105"></a>

##### Parameters

- <code>self</code>: Reference to the coordinator
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a></code>: ID of the dWallet to retrieve


<a name="@Returns_106"></a>

##### Returns

Immutable reference to the dWallet


<a name="@Aborts_107"></a>

##### Aborts

- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNotExists">EDWalletNotExists</a></code>: If the dWallet doesn't exist


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet">get_dwallet</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">dwallet_2pc_mpc_coordinator_inner::DWallet</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet">get_dwallet</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: ID,
): &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a> {
    <b>assert</b>!(self.dwallets.contains(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNotExists">EDWalletNotExists</a>);
    self.dwallets.borrow(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut"></a>

## Function `get_dwallet_mut`

Gets a mutable reference to a dWallet by ID.


<a name="@Parameters_108"></a>

##### Parameters

- <code>self</code>: Mutable reference to the coordinator
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a></code>: ID of the dWallet to retrieve


<a name="@Returns_109"></a>

##### Returns

Mutable reference to the dWallet


<a name="@Aborts_110"></a>

##### Aborts

- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNotExists">EDWalletNotExists</a></code>: If the dWallet doesn't exist


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut">get_dwallet_mut</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">dwallet_2pc_mpc_coordinator_inner::DWallet</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut">get_dwallet_mut</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: ID,
): &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a> {
    <b>assert</b>!(self.dwallets.contains(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNotExists">EDWalletNotExists</a>);
    self.dwallets.borrow_mut(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_active_and_get_public_output"></a>

## Function `validate_active_and_get_public_output`

Validates that a dWallet is in active state and returns its public output.

This function ensures that a dWallet has completed its creation process
(either DKG or imported key verification) and is ready for cryptographic
operations like signing.


<a name="@Parameters_111"></a>

##### Parameters

- <code>self</code>: Reference to the dWallet to validate


<a name="@Returns_112"></a>

##### Returns

Reference to the dWallet's public output


<a name="@Aborts_113"></a>

##### Aborts

- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletInactive">EDWalletInactive</a></code>: If the dWallet is not in the <code>Active</code> state


<a name="@Active_State_Requirements_114"></a>

##### Active State Requirements

A dWallet is considered active when:
- DKG process has completed successfully, OR
- Imported key verification has completed successfully
- User has accepted their encrypted key share
- Public output is available for cryptographic operations


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
        DWalletState::AwaitingNetworkImportedKeyVerification |
        DWalletState::NetworkRejectedImportedKeyVerification |
        DWalletState::AwaitingKeyHolderSignature { .. } =&gt; <b>abort</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletInactive">EDWalletInactive</a>,
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event"></a>

## Function `charge_and_create_current_epoch_dwallet_event`

Creates and charges a user-initiated MPC session for the current epoch.

This function implements the core session creation and payment logic for all
user-initiated dWallet operations. It handles fee collection, session sequencing,
and epoch management in a unified manner.


<a name="@Fee_Structure_115"></a>

##### Fee Structure

- **Computation IKA**: Paid to validators for MPC computation
- **Consensus Validation IKA**: Paid for validator consensus on results
- **Gas Reimbursement SUI**: Covers blockchain transaction costs
- **System Call SUI**: Reserved for internal system operations


<a name="@Session_Management_116"></a>

##### Session Management

1. Assigns sequential session number for epoch ordering
2. Creates session object with collected fees
3. Updates session completion tracking for epoch transitions
4. Stores event for retrieval during session completion


<a name="@Epoch_Coordination_117"></a>

##### Epoch Coordination

- Sessions are sequentially numbered for deterministic epoch management
- Last session completion target is updated to manage epoch transitions
- Fee distribution occurs only upon successful session completion


<a name="@Security_Properties_118"></a>

##### Security Properties

- Fees are escrowed until session completion
- Session sequence numbers prevent replay attacks
- Network encryption key validation ensures proper key usage


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>&lt;E: <b>copy</b>, drop, store&gt;(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, session_identifier: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">dwallet_2pc_mpc_coordinator_inner::SessionIdentifier</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, pricing_value: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricingValue">dwallet_pricing::DWalletPricingValue</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, event_data: E, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSessionEvent">dwallet_2pc_mpc_coordinator_inner::DWalletSessionEvent</a>&lt;E&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>&lt;E: <b>copy</b> + drop + store&gt;(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    session_identifier: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">SessionIdentifier</a>,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: ID,
    pricing_value: DWalletPricingValue,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    event_data: E,
    ctx: &<b>mut</b> TxContext,
): <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSessionEvent">DWalletSessionEvent</a>&lt;E&gt; {
    <b>assert</b>!(self.dwallet_network_encryption_keys.contains(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkEncryptionKeyNotExist">EDWalletNetworkEncryptionKeyNotExist</a>);
    <b>assert</b>!(payment_ika.value() &gt;= pricing_value.computation_ika() + pricing_value.consensus_validation_ika(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInsufficientIKAPayment">EInsufficientIKAPayment</a>);
    <b>assert</b>!(payment_sui.value() &gt;= pricing_value.gas_fee_reimbursement_sui() + pricing_value.gas_fee_reimbursement_sui_for_system_calls(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInsufficientSUIPayment">EInsufficientSUIPayment</a>);
    <b>let</b> computation_fee_charged_ika = payment_ika.split(pricing_value.computation_ika(), ctx).into_balance();
    <b>let</b> consensus_validation_fee_charged_ika = payment_ika.split(pricing_value.consensus_validation_ika(), ctx).into_balance();
    <b>let</b> gas_fee_reimbursement_sui = payment_sui.split(pricing_value.gas_fee_reimbursement_sui(), ctx).into_balance();
    self.pricing_and_fee_management.gas_fee_reimbursement_sui.join(payment_sui.split(pricing_value.gas_fee_reimbursement_sui_for_system_calls(), ctx).into_balance());
    <b>let</b> identifier_preimage = session_identifier.identifier_preimage;
    <b>assert</b>!(self.session_management.registered_session_identifiers.contains(identifier_preimage), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ESessionIdentifierNotExist">ESessionIdentifierNotExist</a>);
    <b>assert</b>!(self.session_management.registered_session_identifiers.borrow(identifier_preimage) == session_identifier.id.to_inner(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ESessionIdentifierNotExist">ESessionIdentifierNotExist</a>);
    <b>let</b> session_sequence_number = self.session_management.next_session_sequence_number;
    <b>let</b> session = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSession">DWalletSession</a> {
        id: object::new(ctx),
        session_identifier,
        session_sequence_number,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
        consensus_validation_fee_charged_ika,
        computation_fee_charged_ika,
        gas_fee_reimbursement_sui,
    };
    <b>let</b> event = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSessionEvent">DWalletSessionEvent</a> {
        epoch: self.current_epoch,
        session_object_id: session.id.to_inner(),
        session_type: {
            SessionType::User {
                sequence_number: session_sequence_number,
            }
        },
        session_identifier_preimage: identifier_preimage,
        event_data,
    };
    self.session_management.session_events.add(session.id.to_inner(), event);
    self.session_management.sessions.add(session_sequence_number, session);
    self.session_management.next_session_sequence_number = session_sequence_number + 1;
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_update_last_user_initiated_session_to_complete_in_current_epoch">update_last_user_initiated_session_to_complete_in_current_epoch</a>();
    event
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_initiate_system_dwallet_session"></a>

## Function `initiate_system_dwallet_session`

Initiates a system-managed MPC session for network operations.

System sessions are initiated by the protocol itself for critical
network maintenance operations that don't involve direct user interaction.
These sessions are essential for network health and security.


<a name="@Supported_System_Operations_119"></a>

##### Supported System Operations

- **Network DKG**: Distributed Key Generation for encryption keys
- **Key Reconfiguration**: Updating existing network encryption keys
- **Network Maintenance**: Other validator network coordination tasks


<a name="@Key_Differences_from_User_Sessions_120"></a>

##### Key Differences from User Sessions

- **No Payment Required**: System operations don't charge users
- **No Sequential Numbering**: System sessions use generated IDs
- **Immediate Emission**: Events are emitted immediately rather than stored
- **Network Priority**: These sessions have priority in validator processing


<a name="@Session_Tracking_121"></a>

##### Session Tracking

- Increments <code>started_system_sessions_count</code> for network monitoring
- Uses fresh object addresses for unique session identification
- Maintains epoch association for proper network coordination


<a name="@Security_Properties_122"></a>

##### Security Properties

- System sessions cannot be initiated by external users
- Session IDs are cryptographically unique to prevent conflicts
- Epoch tracking ensures proper network state consistency


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
    <b>let</b> session_id = object::id_from_address(tx_context::fresh_object_address(ctx));
    <b>let</b> event = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSessionEvent">DWalletSessionEvent</a> {
        epoch: self.current_epoch,
        session_object_id: session_id,
        session_type: SessionType::System,
        // Notice that `session_identifier_preimage` is only the pre-image.
        // For user-initiated events, we guarantee uniqueness by guaranteeing it never repeats (which guarantees the hash is unique).
        // For <a href="../ika_system/system.md#(ika_system=0x0)_system">system</a> events, we guarantee uniqueness by creating an object <b>address</b>, which can never repeat in Move (<a href="../ika_system/system.md#(ika_system=0x0)_system">system</a>-wide).
        // To avoid user-initiated events colliding with <a href="../ika_system/system.md#(ika_system=0x0)_system">system</a> events,
        // we pad the `session_identifier_preimage` differently <b>for</b> user and <a href="../ika_system/system.md#(ika_system=0x0)_system">system</a> events before hashing it.
        session_identifier_preimage: tx_context::fresh_object_address(ctx).to_bytes(),
        event_data,
    };
    self.session_management.session_events.add(session_id, event);
    event::emit(event);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output"></a>

## Function `get_active_dwallet_and_public_output`

Retrieves an active dWallet and its public output for read-only operations.

This helper function safely accesses a dWallet ensuring it exists and is in
an active state suitable for cryptographic operations. The public output
represents the cryptographic public key material.


<a name="@Parameters_123"></a>

##### Parameters

- <code>self</code>: Reference to the coordinator
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a></code>: Unique identifier of the target dWallet


<a name="@Returns_124"></a>

##### Returns

A tuple containing:
- Reference to the validated dWallet object
- Copy of the public output (cryptographic public key data)


<a name="@Validation_Performed_125"></a>

##### Validation Performed

- Confirms dWallet exists in the coordinator's registry
- Validates dWallet is in <code>Active</code> state (DKG completed)
- Ensures public output is available for cryptographic operations


<a name="@Aborts_126"></a>

##### Aborts

- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNotExists">EDWalletNotExists</a></code>: If the dWallet ID is not found
- <code>EDWalletNotActive</code>: If the dWallet is not in active state


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output">get_active_dwallet_and_public_output</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): (&(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">dwallet_2pc_mpc_coordinator_inner::DWallet</a>, vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output">get_active_dwallet_and_public_output</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: ID,
): (&<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a>, vector&lt;u8&gt;) {
    <b>assert</b>!(self.dwallets.contains(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNotExists">EDWalletNotExists</a>);
    <b>let</b> dwallet = self.dwallets.borrow(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>);
    <b>let</b> public_output = dwallet.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_active_and_get_public_output">validate_active_and_get_public_output</a>();
    (dwallet, *public_output)
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output_mut"></a>

## Function `get_active_dwallet_and_public_output_mut`

Retrieves an active dWallet and its public output for mutable operations.

Similar to <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output">get_active_dwallet_and_public_output</a></code> but returns a mutable reference
to the dWallet for operations that need to modify the dWallet state, such as
updating session counts or state transitions.


<a name="@Parameters_127"></a>

##### Parameters

- <code>self</code>: Mutable reference to the coordinator
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a></code>: Unique identifier of the target dWallet


<a name="@Returns_128"></a>

##### Returns

A tuple containing:
- Mutable reference to the validated dWallet object
- Copy of the public output (cryptographic public key data)


<a name="@Common_Use_Cases_129"></a>

##### Common Use Cases

- Updating presign session counters
- Modifying dWallet state during operations
- Recording operational history or metrics
- Managing active session associations


<a name="@Validation_Performed_130"></a>

##### Validation Performed

- Confirms dWallet exists in the coordinator's registry
- Validates dWallet is in <code>Active</code> state (DKG completed)
- Ensures public output is available for cryptographic operations


<a name="@Aborts_131"></a>

##### Aborts

- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNotExists">EDWalletNotExists</a></code>: If the dWallet ID is not found
- <code>EDWalletNotActive</code>: If the dWallet is not in active state


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output_mut">get_active_dwallet_and_public_output_mut</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): (&<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">dwallet_2pc_mpc_coordinator_inner::DWallet</a>, vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output_mut">get_active_dwallet_and_public_output_mut</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: ID,
): (&<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a>, vector&lt;u8&gt;) {
    <b>assert</b>!(self.dwallets.contains(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNotExists">EDWalletNotExists</a>);
    <b>let</b> dwallet = self.dwallets.borrow_mut(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>);
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

Validates that a curve is supported and not paused.


<a name="@Parameters_132"></a>

##### Parameters

- <code>self</code>: Reference to the coordinator
- <code>curve</code>: Curve identifier to validate


<a name="@Aborts_133"></a>

##### Aborts

- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidCurve">EInvalidCurve</a></code>: If the curve is not supported
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ECurvePaused">ECurvePaused</a></code>: If the curve is currently paused


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

Validates that a curve and signature algorithm combination is supported and not paused.


<a name="@Parameters_134"></a>

##### Parameters

- <code>self</code>: Reference to the coordinator
- <code>curve</code>: Curve identifier to validate
- <code>signature_algorithm</code>: Signature algorithm to validate


<a name="@Aborts_135"></a>

##### Aborts

- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidCurve">EInvalidCurve</a></code>: If the curve is not supported
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ECurvePaused">ECurvePaused</a></code>: If the curve is currently paused
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidSignatureAlgorithm">EInvalidSignatureAlgorithm</a></code>: If the signature algorithm is not supported for this curve
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ESignatureAlgorithmPaused">ESignatureAlgorithmPaused</a></code>: If the signature algorithm is currently paused


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

Validates that a curve, signature algorithm, and hash scheme combination is supported and not paused.


<a name="@Parameters_136"></a>

##### Parameters

- <code>self</code>: Reference to the coordinator
- <code>curve</code>: Curve identifier to validate
- <code>signature_algorithm</code>: Signature algorithm to validate
- <code>hash_scheme</code>: Hash scheme to validate


<a name="@Aborts_137"></a>

##### Aborts

- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidCurve">EInvalidCurve</a></code>: If the curve is not supported
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ECurvePaused">ECurvePaused</a></code>: If the curve is currently paused
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidSignatureAlgorithm">EInvalidSignatureAlgorithm</a></code>: If the signature algorithm is not supported for this curve
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ESignatureAlgorithmPaused">ESignatureAlgorithmPaused</a></code>: If the signature algorithm is currently paused
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidHashScheme">EInvalidHashScheme</a></code>: If the hash scheme is not supported for this combination
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EHashSchemePaused">EHashSchemePaused</a></code>: If the hash scheme is currently paused


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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_network_encryption_key_supports_curve"></a>

## Function `validate_network_encryption_key_supports_curve`

Validates that a curve is supported by the network encryption key.


<a name="@Parameters_138"></a>

##### Parameters

- <code>self</code>: Reference to the coordinator
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a></code>: ID of the network encryption key to validate
- <code>curve</code>: Curve identifier to validate


<a name="@Aborts_139"></a>

##### Aborts

- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkEncryptionKeyNotExist">EDWalletNetworkEncryptionKeyNotExist</a></code>: If the network encryption key doesn't exist
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ENetworkEncryptionKeyUnsupportedCurve">ENetworkEncryptionKeyUnsupportedCurve</a></code>: If the curve is not supported by the network encryption key


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_network_encryption_key_supports_curve">validate_network_encryption_key_supports_curve</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, curve: u32)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_network_encryption_key_supports_curve">validate_network_encryption_key_supports_curve</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: ID,
    curve: u32,
) {
    <b>assert</b>!(self.dwallet_network_encryption_keys.contains(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkEncryptionKeyNotExist">EDWalletNetworkEncryptionKeyNotExist</a>);
    <b>let</b> dwallet_network_encryption_key = self.dwallet_network_encryption_keys.borrow(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>);
    <b>assert</b>!(dwallet_network_encryption_key.supported_curves.contains(&curve), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ENetworkEncryptionKeyUnsupportedCurve">ENetworkEncryptionKeyUnsupportedCurve</a>);
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_register_encryption_key"></a>

## Function `register_encryption_key`

Registers an encryption key for secure dWallet share storage.

Creates and validates a new encryption key that can be used to encrypt
centralized secret key shares. The key signature is verified before registration.


<a name="@Parameters_140"></a>

##### Parameters

- <code>self</code>: Mutable reference to the coordinator
- <code>curve</code>: Cryptographic curve for the encryption key
- <code>encryption_key</code>: Serialized encryption key data
- <code>encryption_key_signature</code>: Ed25519 signature of the encryption key
- <code>signer_public_key</code>: Public key used to create the signature
- <code>ctx</code>: Transaction context for object creation


<a name="@Effects_141"></a>

##### Effects

- Creates a new <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptionKey">EncryptionKey</a></code> object
- Emits a <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CreatedEncryptionKeyEvent">CreatedEncryptionKeyEvent</a></code>


<a name="@Aborts_142"></a>

##### Aborts

- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidCurve">EInvalidCurve</a></code>: If the curve is not supported
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ECurvePaused">ECurvePaused</a></code>: If the curve is currently paused
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidEncryptionKeySignature">EInvalidEncryptionKeySignature</a></code>: If the signature verification fails


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

Approves a message for signing by a dWallet.

Creates a message approval that authorizes the specified message to be signed
using the given signature algorithm and hash scheme. This approval can later
be used to initiate a signing session.


<a name="@Parameters_143"></a>

##### Parameters

- <code>self</code>: Reference to the coordinator
- <code>dwallet_cap</code>: Capability proving control over the dWallet
- <code>signature_algorithm</code>: Algorithm to use for signing
- <code>hash_scheme</code>: Hash scheme to apply to the message
- <code>message</code>: Raw message bytes to be signed


<a name="@Returns_144"></a>

##### Returns

A <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">MessageApproval</a></code> that can be used to request signing


<a name="@Aborts_145"></a>

##### Aborts

- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EImportedKeyDWallet">EImportedKeyDWallet</a></code>: If this is an imported key dWallet (use <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_approve_imported_key_message">approve_imported_key_message</a></code> instead)
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNotExists">EDWalletNotExists</a></code>: If the dWallet doesn't exist
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletInactive">EDWalletInactive</a></code>: If the dWallet is not in active state
- Various validation errors for unsupported/paused algorithms


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
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a> = dwallet_cap.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>;
    <b>let</b> is_imported_key_dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_approve_message">validate_approve_message</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>, signature_algorithm, hash_scheme);
    <b>assert</b>!(!is_imported_key_dwallet, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EImportedKeyDWallet">EImportedKeyDWallet</a>);
    <b>let</b> approval = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">MessageApproval</a> {
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
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

Approves a message for signing by an imported key dWallet.

Creates a message approval that authorizes the specified message to be signed
using the given signature algorithm and hash scheme. This approval can later
be used to initiate a signing session.


<a name="@Parameters_146"></a>

##### Parameters

- <code>self</code>: Reference to the coordinator
- <code>imported_key_dwallet_cap</code>: Capability proving control over the dWallet
- <code>signature_algorithm</code>: Algorithm to use for signing
- <code>hash_scheme</code>: Hash scheme to apply to the message
- <code>message</code>: Raw message bytes to be signed


<a name="@Returns_147"></a>

##### Returns

A <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyMessageApproval">ImportedKeyMessageApproval</a></code> that can be used to request signing


<a name="@Aborts_148"></a>

##### Aborts

- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ENotImportedKeyDWallet">ENotImportedKeyDWallet</a></code>: If this is not an imported key dWallet (use <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_approve_message">approve_message</a></code> instead)
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNotExists">EDWalletNotExists</a></code>: If the dWallet doesn't exist
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletInactive">EDWalletInactive</a></code>: If the dWallet is not in active state
- Various validation errors for unsupported/paused algorithms


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
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a> = imported_key_dwallet_cap.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>;
    <b>let</b> is_imported_key_dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_approve_message">validate_approve_message</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>, signature_algorithm, hash_scheme);
    <b>assert</b>!(is_imported_key_dwallet, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ENotImportedKeyDWallet">ENotImportedKeyDWallet</a>);
    <b>let</b> approval = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyMessageApproval">ImportedKeyMessageApproval</a> {
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
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


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_approve_message">validate_approve_message</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, signature_algorithm: u32, hash_scheme: u32): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_approve_message">validate_approve_message</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: ID,
    signature_algorithm: u32,
    hash_scheme: u32,
): bool {
    <b>let</b> (dwallet, _) = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output">get_active_dwallet_and_public_output</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>);
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_curve_and_signature_algorithm_and_hash_scheme">validate_curve_and_signature_algorithm_and_hash_scheme</a>(dwallet.curve, signature_algorithm, hash_scheme);
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_network_encryption_key_supports_curve">validate_network_encryption_key_supports_curve</a>(dwallet.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>, dwallet.curve);
    dwallet.is_imported_key_dwallet
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_update_last_user_initiated_session_to_complete_in_current_epoch"></a>

## Function `update_last_user_initiated_session_to_complete_in_current_epoch`

Updates the last session sequence number that should complete in the current epoch.

Implements session flow control by limiting the number of active sessions per epoch.
This ensures validators don't become overloaded and can complete sessions before
epoch transitions.


<a name="@Algorithm_149"></a>

##### Algorithm

1. Skip update if session management is locked (during epoch transition)
2. Calculate target: completed sessions + max buffer, capped by latest session
3. Only update if the new target is higher (prevents regression)


<a name="@Parameters_150"></a>

##### Parameters

- <code>self</code>: Mutable reference to the coordinator


<a name="@Effects_151"></a>

##### Effects

- Updates <code>last_user_initiated_session_to_complete_in_current_epoch</code> if appropriate
- Maintains session flow control within the configured buffer limits
- Ensures session completion targets only increase, never decrease


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_update_last_user_initiated_session_to_complete_in_current_epoch">update_last_user_initiated_session_to_complete_in_current_epoch</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_update_last_user_initiated_session_to_complete_in_current_epoch">update_last_user_initiated_session_to_complete_in_current_epoch</a>(self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>) {
    // Don't update during epoch transitions when session management is locked
    <b>if</b> (self.session_management.locked_last_user_initiated_session_to_complete_in_current_epoch) {
        <b>return</b>
    };
    // Calculate new target: completed + buffer, but don't exceed latest session
    <b>let</b> new_last_user_initiated_session_to_complete_in_current_epoch = (
        self.session_management.number_of_completed_user_initiated_sessions + self.session_management.max_active_sessions_buffer
    ).min(
        self.session_management.next_session_sequence_number - 1
    );
    // Sanity check: Only update <b>if</b> the new target is higher (prevent regression)
    <b>if</b> (self.session_management.last_user_initiated_session_to_complete_in_current_epoch &gt;= new_last_user_initiated_session_to_complete_in_current_epoch) {
        <b>return</b>
    };
    self.session_management.last_user_initiated_session_to_complete_in_current_epoch = new_last_user_initiated_session_to_complete_in_current_epoch;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_all_current_epoch_sessions_completed"></a>

## Function `all_current_epoch_sessions_completed`

Validates that all required sessions for the current epoch have completed.

This function performs a comprehensive check to ensure the system is ready
for epoch advancement by verifying that all scheduled sessions have finished.


<a name="@Parameters_152"></a>

##### Parameters

- <code>self</code>: Reference to the coordinator


<a name="@Returns_153"></a>

##### Returns

<code><b>true</b></code> if all required sessions are completed and epoch can advance, <code><b>false</b></code> otherwise


<a name="@Validation_Criteria_154"></a>

##### Validation Criteria

1. **Session Management Locked**: <code>last_user_initiated_session_to_complete_in_current_epoch</code> must be locked
2. **User Sessions Complete**: All user-initiated sessions up to the target sequence number must be completed
3. **System Sessions Complete**: All started system sessions must be completed


<a name="@Why_This_Matters_155"></a>

##### Why This Matters

- Prevents epoch transitions with incomplete operations
- Ensures validator consensus on session completion
- Maintains system consistency across epoch boundaries
- Prevents resource leaks from abandoned sessions


<a name="@Session_Types_156"></a>

##### Session Types

- **User Sessions**: Have sequence numbers for multi-epoch scheduling
- **System Sessions**: Must complete within their creation epoch


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_all_current_epoch_sessions_completed">all_current_epoch_sessions_completed</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_all_current_epoch_sessions_completed">all_current_epoch_sessions_completed</a>(self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>): bool {
    <b>return</b> (self.session_management.locked_last_user_initiated_session_to_complete_in_current_epoch &&
        (self.session_management.number_of_completed_user_initiated_sessions == self.session_management.last_user_initiated_session_to_complete_in_current_epoch) &&
        (self.session_management.completed_system_sessions_count == self.session_management.started_system_sessions_count))
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_user_initiated_session_and_charge"></a>

## Function `remove_user_initiated_session_and_charge`

Completes a user-initiated session and processes its associated fees.

This function handles the critical session completion workflow, including fee
distribution, state cleanup, and session accounting. It's called when the
validator network has finished processing a user's MPC request.


<a name="@Parameters_157"></a>

##### Parameters

- <code>self</code>: Mutable reference to the coordinator
- <code>session_sequence_number</code>: Sequential number of the session to complete


<a name="@Returns_158"></a>

##### Returns

Gas reimbursement balance to be distributed to the user


<a name="@Session_Completion_Process_159"></a>

##### Session Completion Process

1. **Session Accounting**: Increments completed session counter
2. **Buffer Management**: Updates session completion target based on new buffer availability
3. **Fee Distribution**: Distributes collected fees to appropriate recipients
4. **Resource Cleanup**: Removes session objects and events from storage
5. **Network Key Updates**: Credits computation fees to the network encryption key


<a name="@Fee_Distribution_160"></a>

##### Fee Distribution

- **Computation Fees (IKA)**: Transferred to network encryption key for validator rewards
- **Consensus Validation Fees (IKA)**: Added to coordinator's fee pool for consensus rewards
- **Gas Reimbursement (SUI)**: Returned to caller for user refund


<a name="@Security_Properties_161"></a>

##### Security Properties

- Only called for successful session completions
- Fees are distributed atomically to prevent partial distributions
- Session sequence numbers ensure proper ordering
- Resource cleanup prevents memory leaks


<a name="@System_Sessions_162"></a>

##### System Sessions

This function is never called for system sessions, which handle their own
completion workflow without user fee management.


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_user_initiated_session_and_charge">remove_user_initiated_session_and_charge</a>&lt;E: <b>copy</b>, drop, store, Success: <b>copy</b>, drop, store, Rejected: <b>copy</b>, drop, store&gt;(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, session_sequence_number: u64, status: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSessionStatusEvent">dwallet_2pc_mpc_coordinator_inner::DWalletSessionStatusEvent</a>&lt;Success, Rejected&gt;): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_user_initiated_session_and_charge">remove_user_initiated_session_and_charge</a>&lt;E: <b>copy</b> + drop + store, Success: <b>copy</b> + drop + store, Rejected: <b>copy</b> + drop + store&gt;(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    session_sequence_number: u64,
    status: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSessionStatusEvent">DWalletSessionStatusEvent</a>&lt;Success, Rejected&gt;,
): Balance&lt;SUI&gt; {
    self.session_management.number_of_completed_user_initiated_sessions = self.session_management.number_of_completed_user_initiated_sessions + 1;
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_update_last_user_initiated_session_to_complete_in_current_epoch">update_last_user_initiated_session_to_complete_in_current_epoch</a>();
    <b>let</b> session = self.session_management.sessions.remove(session_sequence_number);
    // Unpack and delete the `<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSession">DWalletSession</a>` object.
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSession">DWalletSession</a> {
        session_identifier,
        computation_fee_charged_ika,
        gas_fee_reimbursement_sui,
        consensus_validation_fee_charged_ika,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
        id,
        ..
    } = session;
    // Remove the corresponding event.
    <b>let</b> dwallet_network_encryption_key = self.dwallet_network_encryption_keys.borrow_mut(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>);
    <b>let</b> _: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSessionEvent">DWalletSessionEvent</a>&lt;E&gt; = self.session_management.session_events.remove(id.to_inner());
    id.delete();
    // Unpack and delete the corresponding session identifier object.
    // This assures it cannot be reused <b>for</b> another session.
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">SessionIdentifier</a> {
        id,
        identifier_preimage: session_identifier_preimage,
    } = session_identifier;
    id.delete();
    dwallet_network_encryption_key.computation_fee_charged_ika.join(computation_fee_charged_ika);
    self.pricing_and_fee_management.consensus_validation_fee_charged_ika.join(consensus_validation_fee_charged_ika);
    event::emit(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletSessionResultEvent">DWalletSessionResultEvent</a> {
        session_identifier_preimage,
        status,
    });
    gas_fee_reimbursement_sui
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_dkg_first_round"></a>

## Function `request_dwallet_dkg_first_round`

Starts the first round of Distributed Key Generation (DKG) for a new dWallet.

Creates a new dWallet in the DKG requested state and initiates the first round
of the DKG protocol through the validator network. Returns a capability that
grants control over the newly created dWallet.


<a name="@Parameters_163"></a>

##### Parameters

- <code>self</code>: Mutable reference to the DWallet coordinator
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a></code>: ID of the network encryption key to use
- <code>curve</code>: Elliptic curve to use for the dWallet
- <code>payment_ika</code>: IKA payment for computation fees
- <code>payment_sui</code>: SUI payment for gas reimbursement
- <code>ctx</code>: Transaction context


<a name="@Returns_164"></a>

##### Returns

A new <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">DWalletCap</a></code> object granting control over the created dWallet


<a name="@Effects_165"></a>

##### Effects

- Creates a new <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a></code> object in DKG requested state
- Creates and returns a <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">DWalletCap</a></code> for the new dWallet
- Charges fees and creates a session for the DKG process
- Emits a <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGFirstRoundRequestEvent">DWalletDKGFirstRoundRequestEvent</a></code> to start the protocol


<a name="@Aborts_166"></a>

##### Aborts

- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidCurve">EInvalidCurve</a></code>: If the curve is not supported or is paused
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkEncryptionKeyNotExist">EDWalletNetworkEncryptionKeyNotExist</a></code>: If the network encryption key doesn't exist
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMissingProtocolPricing">EMissingProtocolPricing</a></code>: If pricing is not configured for DKG first round
- Various payment-related errors if insufficient funds provided


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_dkg_first_round">request_dwallet_dkg_first_round</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, curve: u32, session_identifier: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">dwallet_2pc_mpc_coordinator_inner::SessionIdentifier</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">dwallet_2pc_mpc_coordinator_inner::DWalletCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_dkg_first_round">request_dwallet_dkg_first_round</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: ID,
    curve: u32,
    session_identifier: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">SessionIdentifier</a>,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">DWalletCap</a> {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_curve">validate_curve</a>(curve);
    <b>let</b> <b>mut</b> pricing_value = self.pricing_and_fee_management.default.try_get_dwallet_pricing_value(curve, option::none(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DKG_FIRST_ROUND_PROTOCOL_FLAG">DKG_FIRST_ROUND_PROTOCOL_FLAG</a>);
    <b>assert</b>!(pricing_value.is_some(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMissingProtocolPricing">EMissingProtocolPricing</a>);
    // TODO(@Omer): check the state of the dWallet (i.e., not waiting <b>for</b> dkg.)
    // TODO(@Omer): I believe the best thing would be to always <b>use</b> the latest key. I'm not sure why the user should even supply the id.
    <b>assert</b>!(self.dwallet_network_encryption_keys.contains(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkEncryptionKeyNotExist">EDWalletNetworkEncryptionKeyNotExist</a>);
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_network_encryption_key_supports_curve">validate_network_encryption_key_supports_curve</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>, curve);
    // Create a new `<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">DWalletCap</a>` object.
    <b>let</b> id = object::new(ctx);
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a> = id.to_inner();
    <b>let</b> dwallet_cap = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">DWalletCap</a> {
        id: object::new(ctx),
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
    };
    <b>let</b> dwallet_cap_id = object::id(&dwallet_cap);
    // Create a new `<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a>` object,
    // link it to the `dwallet_cap` we just created by id,
    // and insert it into the `dwallets` map.
    self.dwallets.add(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a> {
        id,
        created_at_epoch: self.current_epoch,
        curve,
        public_user_secret_key_share: option::none(),
        dwallet_cap_id,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
        is_imported_key_dwallet: <b>false</b>,
        encrypted_user_secret_key_shares: object_table::new(ctx),
        sign_sessions: object_table::new(ctx),
        state: DWalletState::DKGRequested,
    });
    // Emit an event to request the Ika network to start DKG <b>for</b> this dWallet.
    event::emit(self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
        session_identifier,
                <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
        pricing_value.extract(),
        payment_ika,
        payment_sui,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGFirstRoundRequestEvent">DWalletDKGFirstRoundRequestEvent</a> {
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
            dwallet_cap_id,
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
            curve,
        },
        ctx,
    ));
    dwallet_cap
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_dkg_first_round"></a>

## Function `respond_dwallet_dkg_first_round`

Processes validator network response to dWallet DKG first round.

This function handles the validator network's response to a user's DKG first round
request, advancing the dWallet through its initialization lifecycle. It represents
the completion of the first phase of distributed cryptographic key generation.


<a name="@Parameters_167"></a>

##### Parameters

- <code>self</code>: Mutable reference to the coordinator
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a></code>: ID of the dWallet undergoing DKG
- <code>first_round_output</code>: Cryptographic output from validators' first round computation
- <code>rejected</code>: Whether the validator network rejected the DKG request
- <code>session_sequence_number</code>: Session identifier for fee processing


<a name="@Returns_168"></a>

##### Returns

Gas reimbursement balance for user refund


<a name="@DKG_First_Round_Process_169"></a>

##### DKG First Round Process

1. **Session Completion**: Processes session fees and cleanup
2. **State Validation**: Ensures dWallet is in correct state for first round completion
3. **Output Processing**: Handles validator output or rejection appropriately
4. **Event Emission**: Notifies ecosystem of DKG progress or failure
5. **State Transition**: Updates dWallet to next appropriate state


<a name="@Success_Path_170"></a>

##### Success Path

- **Input**: Valid first round output from validator network
- **State Transition**: <code>DKGRequested</code> → <code>AwaitingUserDKGVerificationInitiation</code>
- **Event**: <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletDKGFirstRoundEvent">CompletedDWalletDKGFirstRoundEvent</a></code> with cryptographic output
- **Next Step**: User must verify output and initiate second round


<a name="@Rejection_Path_171"></a>

##### Rejection Path

- **Input**: Network rejection signal (computational or consensus failure)
- **State Transition**: <code>DKGRequested</code> → <code>NetworkRejectedDKGRequest</code>
- **Event**: <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletDKGFirstRoundEvent">RejectedDWalletDKGFirstRoundEvent</a></code> signaling failure
- **Next Step**: User must create new dWallet or retry operation


<a name="@Security_Properties_172"></a>

##### Security Properties

- Only processes sessions in correct DKG state
- Validator consensus ensures output authenticity
- State transitions are atomic and irreversible
- Fees are processed regardless of success/failure


<a name="@Network_Integration_173"></a>

##### Network Integration

This function is exclusively called by the Ika validator network as part
of the consensus protocol, never directly by users.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_dkg_first_round">respond_dwallet_dkg_first_round</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, first_round_output: vector&lt;u8&gt;, rejected: bool, session_sequence_number: u64): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_dkg_first_round">respond_dwallet_dkg_first_round</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: ID,
    first_round_output: vector&lt;u8&gt;,
    rejected: bool,
    session_sequence_number: u64,
): Balance&lt;SUI&gt; {
    <b>let</b> status = <b>if</b> (rejected) {
        DWalletSessionStatusEvent::Rejected(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletDKGFirstRoundEvent">RejectedDWalletDKGFirstRoundEvent</a> {
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
        })
    } <b>else</b> {
        DWalletSessionStatusEvent::Success(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletDKGFirstRoundEvent">CompletedDWalletDKGFirstRoundEvent</a> {
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
            first_round_output,
        })
    };
    <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_user_initiated_session_and_charge">remove_user_initiated_session_and_charge</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGFirstRoundRequestEvent">DWalletDKGFirstRoundRequestEvent</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletDKGFirstRoundEvent">CompletedDWalletDKGFirstRoundEvent</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletDKGFirstRoundEvent">RejectedDWalletDKGFirstRoundEvent</a>&gt;(session_sequence_number, status);
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut">get_dwallet_mut</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>);
    dwallet.state = match (dwallet.state) {
        DWalletState::DKGRequested =&gt; {
            <b>if</b> (rejected) {
                DWalletState::NetworkRejectedDKGRequest
            } <b>else</b> {
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

Initiates the second round of Distributed Key Generation (DKG) with encrypted user shares.

This function represents the user's contribution to the DKG second round, where they
provide their encrypted secret share and request validator network verification.
It creates the encrypted share object and transitions the dWallet to network verification.


<a name="@Parameters_174"></a>

##### Parameters

- <code>self</code>: Mutable reference to the coordinator
- <code>dwallet_cap</code>: User's capability proving dWallet ownership
- <code>centralized_public_key_share_and_proof</code>: User's public key contribution with ZK proof
- <code>encrypted_centralized_secret_share_and_proof</code>: User's encrypted secret share with proof
- <code>encryption_key_address</code>: Address of the encryption key for securing the share
- <code>user_public_output</code>: User's contribution to the final public key
- <code>signer_public_key</code>: Ed25519 key for signature verification
- <code>payment_ika</code>: User's IKA payment for computation
- <code>payment_sui</code>: User's SUI payment for gas reimbursement
- <code>ctx</code>: Transaction context


<a name="@DKG_Second_Round_Process_175"></a>

##### DKG Second Round Process

1. **Validation**: Verifies encryption key compatibility and dWallet state
2. **Share Creation**: Creates <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a></code> with verification pending
3. **Payment Processing**: Charges user for validator computation and consensus
4. **Event Emission**: Requests validator network to verify encrypted share
5. **State Transition**: Updates dWallet to <code>AwaitingNetworkDKGVerification</code>


<a name="@Cryptographic_Security_176"></a>

##### Cryptographic Security

- **Zero-Knowledge Proofs**: User provides proofs of correct share encryption
- **Encryption Key Validation**: Ensures proper key curve compatibility
- **Share Verification**: Network will validate encrypted share correctness
- **Threshold Security**: Maintains distributed key generation properties


<a name="@Network_Integration_177"></a>

##### Network Integration

Emits <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGSecondRoundRequestEvent">DWalletDKGSecondRoundRequestEvent</a></code> for validator processing,
triggering network verification of the encrypted share.


<a name="@Aborts_178"></a>

##### Aborts

- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EImportedKeyDWallet">EImportedKeyDWallet</a></code>: If called on imported key dWallet
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMismatchCurve">EMismatchCurve</a></code>: If encryption key curve doesn't match dWallet curve
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongState">EWrongState</a></code>: If dWallet not in correct state for second round
- Various validation and payment errors


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_dwallet_dkg_second_round">request_dwallet_dkg_second_round</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, dwallet_cap: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">dwallet_2pc_mpc_coordinator_inner::DWalletCap</a>, centralized_public_key_share_and_proof: vector&lt;u8&gt;, encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;, encryption_key_address: <b>address</b>, user_public_output: vector&lt;u8&gt;, signer_public_key: vector&lt;u8&gt;, session_identifier: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">dwallet_2pc_mpc_coordinator_inner::SessionIdentifier</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
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
    session_identifier: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">SessionIdentifier</a>,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> encryption_key = self.encryption_keys.borrow(encryption_key_address);
    <b>let</b> encryption_key_curve = encryption_key.curve;
    <b>let</b> encryption_key_id = encryption_key.id.to_inner();
    <b>let</b> encryption_key = encryption_key.encryption_key;
    <b>let</b> created_at_epoch: u64 = self.current_epoch;
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a> = dwallet_cap.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>;
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet">get_dwallet</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>);
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
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a> = dwallet.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>;
    <b>let</b> encrypted_user_share = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a> {
        id: object::new(ctx),
        created_at_epoch,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
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
        session_identifier,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
        pricing_value.extract(),
        payment_ika,
        payment_sui,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGSecondRoundRequestEvent">DWalletDKGSecondRoundRequestEvent</a> {
            encrypted_user_secret_key_share_id,
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
            first_round_output,
            centralized_public_key_share_and_proof,
            dwallet_cap_id: object::id(dwallet_cap),
            encrypted_centralized_secret_share_and_proof,
            encryption_key,
            encryption_key_id,
            encryption_key_address,
            user_public_output,
            signer_public_key,
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
            curve,
        },
        ctx,
    );
    event::emit(emit_event);
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut">get_dwallet_mut</a>(dwallet_cap.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>);
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


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_dkg_second_round">respond_dwallet_dkg_second_round</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, public_output: vector&lt;u8&gt;, encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, rejected: bool, session_sequence_number: u64): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_dkg_second_round">respond_dwallet_dkg_second_round</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: ID,
    public_output: vector&lt;u8&gt;,
    encrypted_user_secret_key_share_id: ID,
    rejected: bool,
    session_sequence_number: u64,
): Balance&lt;SUI&gt; {
    <b>let</b> status = <b>if</b> (rejected) {
        DWalletSessionStatusEvent::Rejected(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletDKGSecondRoundEvent">RejectedDWalletDKGSecondRoundEvent</a> {
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
            public_output,
        })
    } <b>else</b> {
        DWalletSessionStatusEvent::Success(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletDKGSecondRoundEvent">CompletedDWalletDKGSecondRoundEvent</a> {
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
            public_output,
            encrypted_user_secret_key_share_id,
        })
    };
    <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_user_initiated_session_and_charge">remove_user_initiated_session_and_charge</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletDKGSecondRoundRequestEvent">DWalletDKGSecondRoundRequestEvent</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletDKGSecondRoundEvent">CompletedDWalletDKGSecondRoundEvent</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletDKGSecondRoundEvent">RejectedDWalletDKGSecondRoundEvent</a>&gt;(session_sequence_number, status);
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut">get_dwallet_mut</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>);
    dwallet.state = match (&dwallet.state) {
        DWalletState::AwaitingNetworkDKGVerification =&gt; {
            <b>if</b> (rejected) {
                DWalletState::NetworkRejectedDKGVerification
            } <b>else</b> {
                <b>let</b> encrypted_user_share = dwallet.encrypted_user_secret_key_shares.borrow_mut(encrypted_user_secret_key_share_id);
                encrypted_user_share.state = EncryptedUserSecretKeyShareState::NetworkVerificationCompleted;
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


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_re_encrypt_user_share_for">request_re_encrypt_user_share_for</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, destination_encryption_key_address: <b>address</b>, encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;, source_encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, session_identifier: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">dwallet_2pc_mpc_coordinator_inner::SessionIdentifier</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_re_encrypt_user_share_for">request_re_encrypt_user_share_for</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: ID,
    destination_encryption_key_address: <b>address</b>,
    encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;,
    source_encrypted_user_secret_key_share_id: ID,
    session_identifier: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">SessionIdentifier</a>,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> created_at_epoch = self.current_epoch;
    <b>let</b> destination_encryption_key = self.encryption_keys.borrow(destination_encryption_key_address);
    <b>let</b> destination_encryption_key_id = destination_encryption_key.id.to_inner();
    <b>let</b> destination_encryption_key = destination_encryption_key.encryption_key;
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut">get_dwallet_mut</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>);
    <b>let</b> public_output = *dwallet.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_active_and_get_public_output">validate_active_and_get_public_output</a>();
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a> = dwallet.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>;
    <b>let</b> curve = dwallet.curve;
    <b>assert</b>!(dwallet.encrypted_user_secret_key_shares.contains(source_encrypted_user_secret_key_share_id), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidSource">EInvalidSource</a>);
    <b>let</b> encrypted_user_share = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a> {
        id: object::new(ctx),
        created_at_epoch,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
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
            session_identifier,
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
            pricing_value.extract(),
            payment_ika,
            payment_sui,
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedShareVerificationRequestEvent">EncryptedShareVerificationRequestEvent</a> {
                encrypted_centralized_secret_share_and_proof,
                public_output,
                <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
                encryption_key: destination_encryption_key,
                encryption_key_id: destination_encryption_key_id,
                encrypted_user_secret_key_share_id,
                source_encrypted_user_secret_key_share_id,
                <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
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


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_re_encrypt_user_share_for">respond_re_encrypt_user_share_for</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, rejected: bool, session_sequence_number: u64): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_re_encrypt_user_share_for">respond_re_encrypt_user_share_for</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: ID,
    encrypted_user_secret_key_share_id: ID,
    rejected: bool,
    session_sequence_number: u64
): Balance&lt;SUI&gt; {
    <b>let</b> status = <b>if</b> (rejected) {
        DWalletSessionStatusEvent::Rejected(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedEncryptedShareVerificationEvent">RejectedEncryptedShareVerificationEvent</a> {
            encrypted_user_secret_key_share_id,
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
        })
    } <b>else</b> {
        DWalletSessionStatusEvent::Success(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedEncryptedShareVerificationEvent">CompletedEncryptedShareVerificationEvent</a> {
            encrypted_user_secret_key_share_id,
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
        })
    };
    <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_user_initiated_session_and_charge">remove_user_initiated_session_and_charge</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedShareVerificationRequestEvent">EncryptedShareVerificationRequestEvent</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedEncryptedShareVerificationEvent">CompletedEncryptedShareVerificationEvent</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedEncryptedShareVerificationEvent">RejectedEncryptedShareVerificationEvent</a>&gt;(session_sequence_number, status);
    <b>let</b> (dwallet, _) = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output_mut">get_active_dwallet_and_public_output_mut</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>);
    <b>let</b> encrypted_user_secret_key_share = dwallet.encrypted_user_secret_key_shares.borrow_mut(encrypted_user_secret_key_share_id);
    encrypted_user_secret_key_share.state = match(encrypted_user_secret_key_share.state) {
        EncryptedUserSecretKeyShareState::AwaitingNetworkVerification =&gt; {
            <b>if</b>(rejected) {
                EncryptedUserSecretKeyShareState::NetworkVerificationRejected
            } <b>else</b> {
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


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_accept_encrypted_user_share">accept_encrypted_user_share</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, user_output_signature: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_accept_encrypted_user_share">accept_encrypted_user_share</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: ID,
    encrypted_user_secret_key_share_id: ID,
    user_output_signature: vector&lt;u8&gt;,
) {
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut">get_dwallet_mut</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>);
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
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut">get_dwallet_mut</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>);
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
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
            user_output_signature,
            encryption_key_id,
            encryption_key_address,
        }
    );
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_imported_key_dwallet_verification"></a>

## Function `request_imported_key_dwallet_verification`

Request verification of the imported key dWallet from the Ika network.


<a name="@Parameters_179"></a>

##### Parameters

- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a></code>: The ID of the network encryption key to use for the dWallet.
- <code>curve</code>: The curve of the dWallet.
- <code>centralized_party_message</code>: The message from the centralized party.
- <code>encrypted_centralized_secret_share_and_proof</code>: The encrypted centralized secret share and proof.
- <code>encryption_key_address</code>: The address of the encryption key.
- <code>user_public_output</code>: The public output of the user.
- <code>signer_public_key</code>: The public key of the signer.
- <code>session_identifier_preimage</code>: The session identifier.
- <code>payment_ika</code>: The IKA payment for the operation.
- <code>payment_sui</code>: The SUI payment for the operation.
- <code>ctx</code>: The transaction context.


<a name="@Returns_180"></a>

##### Returns

- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyDWalletCap">ImportedKeyDWalletCap</a></code>: The cap of the imported key dWallet.


<a name="@Aborts_181"></a>

##### Aborts

- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkEncryptionKeyNotExist">EDWalletNetworkEncryptionKeyNotExist</a></code>: If the network encryption key does not exist.
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMismatchCurve">EMismatchCurve</a></code>: If the curve does not match the dWallet curve.
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidEncryptionKeySignature">EInvalidEncryptionKeySignature</a></code>: If the encryption key signature is invalid.
- <code>EInvalidUserPublicOutput</code>: If the user public output is invalid.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_imported_key_dwallet_verification">request_imported_key_dwallet_verification</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, curve: u32, centralized_party_message: vector&lt;u8&gt;, encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;, encryption_key_address: <b>address</b>, user_public_output: vector&lt;u8&gt;, signer_public_key: vector&lt;u8&gt;, session_identifier: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">dwallet_2pc_mpc_coordinator_inner::SessionIdentifier</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyDWalletCap">dwallet_2pc_mpc_coordinator_inner::ImportedKeyDWalletCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_imported_key_dwallet_verification">request_imported_key_dwallet_verification</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: ID,
    curve: u32,
    centralized_party_message: vector&lt;u8&gt;,
    encrypted_centralized_secret_share_and_proof: vector&lt;u8&gt;,
    encryption_key_address: <b>address</b>,
    user_public_output: vector&lt;u8&gt;,
    signer_public_key: vector&lt;u8&gt;,
    session_identifier: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">SessionIdentifier</a>,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyDWalletCap">ImportedKeyDWalletCap</a> {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_curve">validate_curve</a>(curve);
    <b>assert</b>!(self.dwallet_network_encryption_keys.contains(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkEncryptionKeyNotExist">EDWalletNetworkEncryptionKeyNotExist</a>);
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_network_encryption_key_supports_curve">validate_network_encryption_key_supports_curve</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>, curve);
    <b>let</b> encryption_key = self.encryption_keys.borrow(encryption_key_address);
    <b>let</b> encryption_key_id = encryption_key.id.to_inner();
    <b>let</b> encryption_key = encryption_key.encryption_key;
    <b>let</b> created_at_epoch: u64 = self.current_epoch;
    <b>let</b> id = object::new(ctx);
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a> = id.to_inner();
    <b>let</b> encrypted_user_share = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EncryptedUserSecretKeyShare">EncryptedUserSecretKeyShare</a> {
        id: object::new(ctx),
        created_at_epoch,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
        encrypted_centralized_secret_share_and_proof,
        encryption_key_id,
        encryption_key_address,
        source_encrypted_user_secret_key_share_id: option::none(),
        state: EncryptedUserSecretKeyShareState::AwaitingNetworkVerification
    };
    <b>let</b> encrypted_user_secret_key_share_id = object::id(&encrypted_user_share);
    <b>let</b> dwallet_cap = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyDWalletCap">ImportedKeyDWalletCap</a> {
        id: object::new(ctx),
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
    };
    <b>let</b> dwallet_cap_id = object::id(&dwallet_cap);
    <b>let</b> <b>mut</b> encrypted_user_secret_key_shares = object_table::new(ctx);
    encrypted_user_secret_key_shares.add(encrypted_user_secret_key_share_id, encrypted_user_share);
    self.dwallets.add(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWallet">DWallet</a> {
        id,
        created_at_epoch: self.current_epoch,
        curve,
        public_user_secret_key_share: option::none(),
        dwallet_cap_id,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
        is_imported_key_dwallet: <b>true</b>,
        encrypted_user_secret_key_shares,
        sign_sessions: object_table::new(ctx),
        state: DWalletState::AwaitingNetworkImportedKeyVerification,
    });
    <b>let</b> <b>mut</b> pricing_value = self.pricing_and_fee_management.current.try_get_dwallet_pricing_value(curve, option::none(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_IMPORTED_KEY_DWALLET_VERIFICATION_PROTOCOL_FLAG">IMPORTED_KEY_DWALLET_VERIFICATION_PROTOCOL_FLAG</a>);
    <b>assert</b>!(pricing_value.is_some(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMissingProtocolPricing">EMissingProtocolPricing</a>);
    <b>let</b> emit_event = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
        session_identifier,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
        pricing_value.extract(),
        payment_ika,
        payment_sui,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletImportedKeyVerificationRequestEvent">DWalletImportedKeyVerificationRequestEvent</a> {
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
            encrypted_user_secret_key_share_id,
            centralized_party_message,
            dwallet_cap_id,
            encrypted_centralized_secret_share_and_proof,
            encryption_key,
            encryption_key_id,
            encryption_key_address,
            user_public_output,
            signer_public_key,
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
            curve,
        },
        ctx,
    );
    event::emit(emit_event);
    dwallet_cap
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


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_imported_key_dwallet_verification">respond_imported_key_dwallet_verification</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, public_output: vector&lt;u8&gt;, encrypted_user_secret_key_share_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, rejected: bool, session_sequence_number: u64): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_imported_key_dwallet_verification">respond_imported_key_dwallet_verification</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: ID,
    public_output: vector&lt;u8&gt;,
    encrypted_user_secret_key_share_id: ID,
    rejected: bool,
    session_sequence_number: u64,
): Balance&lt;SUI&gt; {
    <b>let</b> status = <b>if</b> (rejected) {
        DWalletSessionStatusEvent::Rejected(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletImportedKeyVerificationEvent">RejectedDWalletImportedKeyVerificationEvent</a> {
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
        })
    } <b>else</b> {
        DWalletSessionStatusEvent::Success(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletImportedKeyVerificationEvent">CompletedDWalletImportedKeyVerificationEvent</a> {
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
            public_output,
            encrypted_user_secret_key_share_id,
        })
    };
    <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_user_initiated_session_and_charge">remove_user_initiated_session_and_charge</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletImportedKeyVerificationRequestEvent">DWalletImportedKeyVerificationRequestEvent</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedDWalletImportedKeyVerificationEvent">CompletedDWalletImportedKeyVerificationEvent</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedDWalletImportedKeyVerificationEvent">RejectedDWalletImportedKeyVerificationEvent</a>&gt;(session_sequence_number, status);
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut">get_dwallet_mut</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>);
    dwallet.state = match (&dwallet.state) {
        DWalletState::AwaitingNetworkImportedKeyVerification =&gt; {
            <b>if</b> (rejected) {
                DWalletState::NetworkRejectedImportedKeyVerification
            } <b>else</b> {
                <b>let</b> encrypted_user_share = dwallet.encrypted_user_secret_key_shares.borrow_mut(encrypted_user_secret_key_share_id);
                encrypted_user_share.state = EncryptedUserSecretKeyShareState::NetworkVerificationCompleted;
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


<a name="@Parameters_182"></a>

##### Parameters

- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a></code>: The ID of the dWallet to make the user secret key shares public.
- <code>public_user_secret_key_share</code>: The public user secret key shares to be made public.
- <code>payment_ika</code>: The IKA payment for the operation.
- <code>payment_sui</code>: The SUI payment for the operation.
- <code>ctx</code>: The transaction context.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_make_dwallet_user_secret_key_share_public">request_make_dwallet_user_secret_key_share_public</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, public_user_secret_key_share: vector&lt;u8&gt;, session_identifier: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">dwallet_2pc_mpc_coordinator_inner::SessionIdentifier</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_make_dwallet_user_secret_key_share_public">request_make_dwallet_user_secret_key_share_public</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: ID,
    public_user_secret_key_share: vector&lt;u8&gt;,
    session_identifier: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">SessionIdentifier</a>,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> (dwallet, public_output) = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output">get_active_dwallet_and_public_output</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>);
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a> = dwallet.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>;
    <b>let</b> curve = dwallet.curve;
    <b>assert</b>!(dwallet.public_user_secret_key_share.is_none(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletUserSecretKeySharesAlreadyPublic">EDWalletUserSecretKeySharesAlreadyPublic</a>);
    <b>let</b> <b>mut</b> pricing_value = self.pricing_and_fee_management.current.try_get_dwallet_pricing_value(curve, option::none(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MAKE_DWALLET_USER_SECRET_KEY_SHARE_PUBLIC_PROTOCOL_FLAG">MAKE_DWALLET_USER_SECRET_KEY_SHARE_PUBLIC_PROTOCOL_FLAG</a>);
    <b>assert</b>!(pricing_value.is_some(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMissingProtocolPricing">EMissingProtocolPricing</a>);
    event::emit(
        self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
            session_identifier,
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
            pricing_value.extract(),
            payment_ika,
            payment_sui,
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MakeDWalletUserSecretKeySharePublicRequestEvent">MakeDWalletUserSecretKeySharePublicRequestEvent</a> {
                public_user_secret_key_share,
                public_output,
                curve,
                <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
                <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
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


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_make_dwallet_user_secret_key_share_public">respond_make_dwallet_user_secret_key_share_public</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, public_user_secret_key_share: vector&lt;u8&gt;, rejected: bool, session_sequence_number: u64): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_make_dwallet_user_secret_key_share_public">respond_make_dwallet_user_secret_key_share_public</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: ID,
    public_user_secret_key_share: vector&lt;u8&gt;,
    rejected: bool,
    session_sequence_number: u64,
): Balance&lt;SUI&gt; {
    <b>let</b> status = <b>if</b> (rejected) {
        DWalletSessionStatusEvent::Rejected(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedMakeDWalletUserSecretKeySharePublicEvent">RejectedMakeDWalletUserSecretKeySharePublicEvent</a> {
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
        })
    } <b>else</b> {
        DWalletSessionStatusEvent::Success(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedMakeDWalletUserSecretKeySharePublicEvent">CompletedMakeDWalletUserSecretKeySharePublicEvent</a> {
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
            public_user_secret_key_share,
        })
    };
    <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_user_initiated_session_and_charge">remove_user_initiated_session_and_charge</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MakeDWalletUserSecretKeySharePublicRequestEvent">MakeDWalletUserSecretKeySharePublicRequestEvent</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedMakeDWalletUserSecretKeySharePublicEvent">CompletedMakeDWalletUserSecretKeySharePublicEvent</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedMakeDWalletUserSecretKeySharePublicEvent">RejectedMakeDWalletUserSecretKeySharePublicEvent</a>&gt;(session_sequence_number, status);
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut">get_dwallet_mut</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>);
    <b>if</b> (!rejected) {
        dwallet.public_user_secret_key_share.fill(public_user_secret_key_share);
    };
    gas_fee_reimbursement_sui
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_presign"></a>

## Function `request_presign`

Requests generation of a dWallet-specific presign for accelerated signing.

Presigns are precomputed cryptographic material that dramatically reduce online
signing latency from seconds to milliseconds. This function creates a dWallet-specific
presign that can only be used with the specified dWallet.


<a name="@Parameters_183"></a>

##### Parameters

- <code>self</code>: Mutable reference to the coordinator
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a></code>: Target dWallet for the presign generation
- <code>signature_algorithm</code>: Algorithm requiring presign material (e.g., ECDSA)
- <code>payment_ika</code>: User's IKA payment for computation
- <code>payment_sui</code>: User's SUI payment for gas reimbursement
- <code>ctx</code>: Transaction context


<a name="@Returns_184"></a>

##### Returns

<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">UnverifiedPresignCap</a></code> that must be verified before use in signing



<a name="@Security_Properties_185"></a>

##### Security Properties

- **Single Use**: Each presign can only be consumed once
- **Cryptographic Binding**: Tied to specific dWallet public key
- **Validator Consensus**: Generated through secure MPC protocol
- **Expiration**: Presigns have limited validity period


<a name="@Next_Steps_186"></a>

##### Next Steps

1. Wait for validator network to process the presign request
2. Call <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_is_presign_valid">is_presign_valid</a>()</code> to check completion status
3. Use <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_verify_presign_cap">verify_presign_cap</a>()</code> to convert to verified capability
4. Combine with message approval for actual signing


<a name="@Aborts_187"></a>

##### Aborts

- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidSignatureAlgorithm">EInvalidSignatureAlgorithm</a></code>: If the signature algorithm is not allowed for dWallet-specific presigns
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidCurve">EInvalidCurve</a></code>: If the curve is not supported
- <code>EInvalidNetworkEncryptionKey</code>: If the network encryption key is not supported
- <code>EInsufficientFunds</code>: If the user does not have enough funds to pay for the presign


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_presign">request_presign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, signature_algorithm: u32, session_identifier: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">dwallet_2pc_mpc_coordinator_inner::SessionIdentifier</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">dwallet_2pc_mpc_coordinator_inner::UnverifiedPresignCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_presign">request_presign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: ID,
    signature_algorithm: u32,
    session_identifier: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">SessionIdentifier</a>,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">UnverifiedPresignCap</a> {
    <b>let</b> created_at_epoch = self.current_epoch;
    <b>let</b> (dwallet, public_output) = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output">get_active_dwallet_and_public_output</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>);
    <b>let</b> curve = dwallet.curve;
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_curve_and_signature_algorithm">validate_curve_and_signature_algorithm</a>(curve, signature_algorithm);
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_network_encryption_key_supports_curve">validate_network_encryption_key_supports_curve</a>(dwallet.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>, curve);
    <b>assert</b>!(!self.support_config.signature_algorithms_allowed_global_presign.contains(&signature_algorithm), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidSignatureAlgorithm">EInvalidSignatureAlgorithm</a>);
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a> = dwallet.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>;
    <b>let</b> id = object::new(ctx);
    <b>let</b> presign_id = id.to_inner();
    <b>let</b> cap = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">UnverifiedPresignCap</a> {
        id: object::new(ctx),
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: option::some(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>),
        presign_id,
    };
    self.presign_sessions.add(presign_id, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignSession">PresignSession</a> {
        id,
        created_at_epoch,
        signature_algorithm,
        curve,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: option::some(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>),
        cap_id: object::id(&cap),
        state: PresignState::Requested,
    });
    <b>let</b> <b>mut</b> pricing_value = self.pricing_and_fee_management.current.try_get_dwallet_pricing_value(curve, option::some(signature_algorithm), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PRESIGN_PROTOCOL_FLAG">PRESIGN_PROTOCOL_FLAG</a>);
    <b>assert</b>!(pricing_value.is_some(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMissingProtocolPricing">EMissingProtocolPricing</a>);
    event::emit(
        self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
            session_identifier,
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
            pricing_value.extract(),
            payment_ika,
            payment_sui,
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignRequestEvent">PresignRequestEvent</a> {
                <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: option::some(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>),
                presign_id,
                dwallet_public_output: option::some(public_output),
                <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
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

Requests generation of a global presign for flexible cross-dWallet use.

Global presigns provide computational efficiency by creating precomputed material
that can be used with any compatible dWallet under the same network encryption key.
This enables better resource utilization and batch processing optimization.


<a name="@Parameters_188"></a>

##### Parameters

- <code>self</code>: Mutable reference to the coordinator
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a></code>: Network encryption key for presign compatibility
- <code>curve</code>: Cryptographic curve for presign generation
- <code>signature_algorithm</code>: Algorithm requiring presign material
- <code>payment_ika</code>: User's IKA payment for computation
- <code>payment_sui</code>: User's SUI payment for gas reimbursement
- <code>ctx</code>: Transaction context


<a name="@Returns_189"></a>

##### Returns

<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">UnverifiedPresignCap</a></code> that can be used with any compatible dWallet


<a name="@Security_Considerations_190"></a>

##### Security Considerations

- Global presigns maintain cryptographic security properties
- Network encryption key provides isolation between key epochs
- Validator consensus ensures presign authenticity
- Single-use property prevents replay attacks


<a name="@Next_Steps_191"></a>

##### Next Steps

1. Wait for validator network to process the global presign request
2. Verify presign completion using <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_is_presign_valid">is_presign_valid</a>()</code>
3. Convert to <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">VerifiedPresignCap</a></code> with <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_verify_presign_cap">verify_presign_cap</a>()</code>
4. Use with any compatible dWallet for signing operations


<a name="@Aborts_192"></a>

##### Aborts

- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidSignatureAlgorithm">EInvalidSignatureAlgorithm</a></code>: If the signature algorithm is not allowed for global presigns
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidCurve">EInvalidCurve</a></code>: If the curve is not supported
- <code>EInvalidNetworkEncryptionKey</code>: If the network encryption key is not supported
- <code>EInsufficientFunds</code>: If the user does not have enough funds to pay for the presign


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_global_presign">request_global_presign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, curve: u32, signature_algorithm: u32, session_identifier: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">dwallet_2pc_mpc_coordinator_inner::SessionIdentifier</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">dwallet_2pc_mpc_coordinator_inner::UnverifiedPresignCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_global_presign">request_global_presign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: ID,
    curve: u32,
    signature_algorithm: u32,
    session_identifier: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">SessionIdentifier</a>,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">UnverifiedPresignCap</a> {
    <b>let</b> created_at_epoch = self.current_epoch;
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_curve_and_signature_algorithm">validate_curve_and_signature_algorithm</a>(curve, signature_algorithm);
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_network_encryption_key_supports_curve">validate_network_encryption_key_supports_curve</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>, curve);
    <b>assert</b>!(self.support_config.signature_algorithms_allowed_global_presign.contains(&signature_algorithm), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EInvalidSignatureAlgorithm">EInvalidSignatureAlgorithm</a>);
    <b>let</b> id = object::new(ctx);
    <b>let</b> presign_id = id.to_inner();
    <b>let</b> cap = <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPresignCap">UnverifiedPresignCap</a> {
        id: object::new(ctx),
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: option::none(),
        presign_id,
    };
    self.presign_sessions.add(presign_id, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignSession">PresignSession</a> {
        id,
        created_at_epoch,
        signature_algorithm,
        curve,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: option::none(),
        cap_id: object::id(&cap),
        state: PresignState::Requested,
    });
    <b>let</b> <b>mut</b> pricing_value = self.pricing_and_fee_management.current.try_get_dwallet_pricing_value(curve, option::some(signature_algorithm), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PRESIGN_PROTOCOL_FLAG">PRESIGN_PROTOCOL_FLAG</a>);
    <b>assert</b>!(pricing_value.is_some(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMissingProtocolPricing">EMissingProtocolPricing</a>);
    event::emit(
        self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
            session_identifier,
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
            pricing_value.extract(),
            payment_ika,
            payment_sui,
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignRequestEvent">PresignRequestEvent</a> {
                <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: option::none(),
                presign_id,
                dwallet_public_output: option::none(),
                <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
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

Processes validator network response to presign generation request.

This function handles the completion or rejection of presign generation by the
validator network, updating the presign session state and emitting appropriate events.


<a name="@Parameters_193"></a>

##### Parameters

- <code>self</code>: Mutable reference to the coordinator
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a></code>: Target dWallet ID for dWallet-specific presigns (None for global)
- <code>presign_id</code>: Unique identifier of the presign session
- <code>session_id</code>: MPC session ID that processed the presign
- <code>presign</code>: Generated cryptographic presign material (if successful)
- <code>rejected</code>: Whether the validator network rejected the presign request
- <code>session_sequence_number</code>: Session sequence for fee processing


<a name="@Returns_194"></a>

##### Returns

Gas reimbursement balance for user refund


<a name="@Success_Path_195"></a>

##### Success Path

- **State Transition**: <code>Requested</code> → <code>Completed</code>
- **Presign Storage**: Cryptographic material is stored in session
- **Event**: <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedPresignEvent">CompletedPresignEvent</a></code> with presign data
- **Capability**: Associated capability can now be verified and used


<a name="@Rejection_Path_196"></a>

##### Rejection Path

- **State Transition**: <code>Requested</code> → <code>NetworkRejected</code>
- **Event**: <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedPresignEvent">RejectedPresignEvent</a></code> indicating failure
- **Capability**: Associated capability becomes unusable
- **Common Causes**: Insufficient validator participation, computation errors



<a name="@Security_Properties_197"></a>

##### Security Properties

- Presign material is cryptographically secure and verifiable
- Single-use property enforced through session consumption
- Validator consensus ensures authenticity of generated material
- Rejection handling prevents use of incomplete presigns


<a name="@Network_Integration_198"></a>

##### Network Integration

This function is exclusively called by the Ika validator network as part
of the distributed presign generation protocol.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_presign">respond_presign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../sui/object.md#sui_object_ID">sui::object::ID</a>&gt;, presign_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, presign: vector&lt;u8&gt;, rejected: bool, session_sequence_number: u64): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_presign">respond_presign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: Option&lt;ID&gt;,
    presign_id: ID,
    presign: vector&lt;u8&gt;,
    rejected: bool,
    session_sequence_number: u64
): Balance&lt;SUI&gt; {
    <b>let</b> status = <b>if</b> (rejected) {
        DWalletSessionStatusEvent::Rejected(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedPresignEvent">RejectedPresignEvent</a> {
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
            presign_id,
        })
    } <b>else</b> {
        DWalletSessionStatusEvent::Success(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedPresignEvent">CompletedPresignEvent</a> {
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
            presign_id,
            presign,
        })
    };
    <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_user_initiated_session_and_charge">remove_user_initiated_session_and_charge</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignRequestEvent">PresignRequestEvent</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedPresignEvent">CompletedPresignEvent</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedPresignEvent">RejectedPresignEvent</a>&gt;(session_sequence_number, status);
    <b>let</b> presign_obj = self.presign_sessions.borrow_mut(presign_id);
    presign_obj.state = match(presign_obj.state) {
        PresignState::Requested =&gt; {
            <b>if</b>(rejected) {
                PresignState::NetworkRejected
            } <b>else</b> {
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

Validates that a presign capability corresponds to a completed presign session.

Checks both the completion state and capability ID matching to ensure
the capability is authentic and the presign is ready for use.


<a name="@Parameters_199"></a>

##### Parameters

- <code>self</code>: Reference to the coordinator
- <code>cap</code>: Unverified presign capability to validate


<a name="@Returns_200"></a>

##### Returns

<code><b>true</b></code> if the presign is completed and the capability is valid, <code><b>false</b></code> otherwise


<a name="@Validation_Criteria_201"></a>

##### Validation Criteria

- Presign session must be in <code>Completed</code> state
- Capability ID must match the session's recorded capability ID
- Presign session must exist in the coordinator


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
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
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
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
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


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_and_initiate_sign">validate_and_initiate_sign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, pricing_value: (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricingValue">dwallet_pricing::DWalletPricingValue</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, session_identifier: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">dwallet_2pc_mpc_coordinator_inner::SessionIdentifier</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, signature_algorithm: u32, hash_scheme: u32, message: vector&lt;u8&gt;, presign_cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">dwallet_2pc_mpc_coordinator_inner::VerifiedPresignCap</a>, message_centralized_signature: vector&lt;u8&gt;, is_future_sign: bool, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_and_initiate_sign">validate_and_initiate_sign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    pricing_value: DWalletPricingValue,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    session_identifier: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">SessionIdentifier</a>,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: ID,
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
    <b>let</b> (dwallet, dwallet_public_output) = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output_mut">get_active_dwallet_and_public_output_mut</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>);
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">VerifiedPresignCap</a> {
        id,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: presign_cap_dwallet_id,
        presign_id: presign_cap_presign_id,
    } = presign_cap;
    <b>let</b> presign_cap_id = id.to_inner();
    id.delete();
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PresignSession">PresignSession</a> {
        id,
        created_at_epoch: _,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: presign_dwallet_id,
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
    <b>assert</b>!(presign_dwallet_id.is_none() || presign_dwallet_id.is_some_and!(|id| id == <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMessageApprovalMismatch">EMessageApprovalMismatch</a>);
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
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a> = dwallet.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>;
    <b>let</b> emit_event = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
        session_identifier,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
        pricing_value,
        payment_ika,
        payment_sui,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignRequestEvent">SignRequestEvent</a> {
            sign_id,
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
            dwallet_public_output,
            curve,
            signature_algorithm,
            hash_scheme,
            message,
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
            presign_id,
            presign,
            message_centralized_signature,
            is_future_sign,
        },
        ctx,
    );
    // Create a `<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignSession">SignSession</a>` object and register it in `sign_sessions`.
    <b>let</b> dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_dwallet_mut">get_dwallet_mut</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>);
    dwallet.sign_sessions.add(sign_id, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignSession">SignSession</a> {
        id,
        created_at_epoch,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
        state: SignState::Requested,
    });
    <b>let</b> is_imported_key_dwallet = dwallet.is_imported_key_dwallet;
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_curve_and_signature_algorithm_and_hash_scheme">validate_curve_and_signature_algorithm_and_hash_scheme</a>(curve, signature_algorithm, hash_scheme);
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_network_encryption_key_supports_curve">validate_network_encryption_key_supports_curve</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>, curve);
    event::emit(emit_event);
    is_imported_key_dwallet
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_sign"></a>

## Function `request_sign`

Initiates the Sign protocol for this dWallet.
Requires a <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">MessageApproval</a></code>, which approves a message for signing and is unpacked and deleted to ensure it is never used twice.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_sign">request_sign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, message_approval: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">dwallet_2pc_mpc_coordinator_inner::MessageApproval</a>, presign_cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">dwallet_2pc_mpc_coordinator_inner::VerifiedPresignCap</a>, message_centralized_signature: vector&lt;u8&gt;, session_identifier: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">dwallet_2pc_mpc_coordinator_inner::SessionIdentifier</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_sign">request_sign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    message_approval: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">MessageApproval</a>,
    presign_cap: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">VerifiedPresignCap</a>,
    message_centralized_signature: vector&lt;u8&gt;,
    session_identifier: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">SessionIdentifier</a>,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">MessageApproval</a> {
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
        signature_algorithm,
        hash_scheme,
        message
    } = message_approval;
    <b>let</b> (dwallet, _) = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output">get_active_dwallet_and_public_output</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>);
    <b>let</b> curve = dwallet.curve;
    <b>let</b> <b>mut</b> pricing_value = self.pricing_and_fee_management.current.try_get_dwallet_pricing_value(curve, option::some(signature_algorithm), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SIGN_PROTOCOL_FLAG">SIGN_PROTOCOL_FLAG</a>);
    <b>assert</b>!(pricing_value.is_some(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMissingProtocolPricing">EMissingProtocolPricing</a>);
    <b>let</b> is_imported_key_dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_and_initiate_sign">validate_and_initiate_sign</a>(
        pricing_value.extract(),
        payment_ika,
        payment_sui,
        session_identifier,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
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


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_imported_key_sign">request_imported_key_sign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, message_approval: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyMessageApproval">dwallet_2pc_mpc_coordinator_inner::ImportedKeyMessageApproval</a>, presign_cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">dwallet_2pc_mpc_coordinator_inner::VerifiedPresignCap</a>, message_centralized_signature: vector&lt;u8&gt;, session_identifier: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">dwallet_2pc_mpc_coordinator_inner::SessionIdentifier</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_imported_key_sign">request_imported_key_sign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    message_approval: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyMessageApproval">ImportedKeyMessageApproval</a>,
    presign_cap: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">VerifiedPresignCap</a>,
    message_centralized_signature: vector&lt;u8&gt;,
    session_identifier: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">SessionIdentifier</a>,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyMessageApproval">ImportedKeyMessageApproval</a> {
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
        signature_algorithm,
        hash_scheme,
        message
    } = message_approval;
    <b>let</b> (dwallet, _) = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output">get_active_dwallet_and_public_output</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>);
    <b>let</b> curve = dwallet.curve;
    <b>let</b> <b>mut</b> pricing_value = self.pricing_and_fee_management.current.try_get_dwallet_pricing_value(curve, option::some(signature_algorithm), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SIGN_PROTOCOL_FLAG">SIGN_PROTOCOL_FLAG</a>);
    <b>assert</b>!(pricing_value.is_some(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMissingProtocolPricing">EMissingProtocolPricing</a>);
    <b>let</b> is_imported_key_dwallet = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_and_initiate_sign">validate_and_initiate_sign</a>(
        pricing_value.extract(),
        payment_ika,
        payment_sui,
        session_identifier,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
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


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_future_sign">request_future_sign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, presign_cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">dwallet_2pc_mpc_coordinator_inner::VerifiedPresignCap</a>, message: vector&lt;u8&gt;, hash_scheme: u32, message_centralized_signature: vector&lt;u8&gt;, session_identifier: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">dwallet_2pc_mpc_coordinator_inner::SessionIdentifier</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPartialUserSignatureCap">dwallet_2pc_mpc_coordinator_inner::UnverifiedPartialUserSignatureCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_future_sign">request_future_sign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: ID,
    presign_cap: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPresignCap">VerifiedPresignCap</a>,
    message: vector&lt;u8&gt;,
    hash_scheme: u32,
    message_centralized_signature: vector&lt;u8&gt;,
    session_identifier: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">SessionIdentifier</a>,
    payment_ika: &<b>mut</b> Coin&lt;IKA&gt;,
    payment_sui: &<b>mut</b> Coin&lt;SUI&gt;,
    ctx: &<b>mut</b> TxContext
): <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_UnverifiedPartialUserSignatureCap">UnverifiedPartialUserSignatureCap</a> {
    // Check that the presign is global, or that it belongs to this dWallet.
    <b>assert</b>!(presign_cap.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>.is_none() || presign_cap.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>.is_some_and!(|id| id == <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMessageApprovalMismatch">EMessageApprovalMismatch</a>);
    <b>let</b> (dwallet, dwallet_public_output) = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output_mut">get_active_dwallet_and_public_output_mut</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>);
    <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a> = dwallet.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>;
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
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_validate_network_encryption_key_supports_curve">validate_network_encryption_key_supports_curve</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>, curve);
    <b>let</b> <b>mut</b> pricing_value = self.pricing_and_fee_management.current.try_get_dwallet_pricing_value(curve, option::some(signature_algorithm), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_FUTURE_SIGN_PROTOCOL_FLAG">FUTURE_SIGN_PROTOCOL_FLAG</a>);
    <b>assert</b>!(pricing_value.is_some(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMissingProtocolPricing">EMissingProtocolPricing</a>);
    <b>let</b> emit_event = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_charge_and_create_current_epoch_dwallet_event">charge_and_create_current_epoch_dwallet_event</a>(
        session_identifier,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
        pricing_value.extract(),
        payment_ika,
        payment_sui,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_FutureSignRequestEvent">FutureSignRequestEvent</a> {
                <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
                partial_centralized_signed_message_id,
                message,
                presign: presign,
                dwallet_public_output,
                curve,
                signature_algorithm,
                hash_scheme,
                message_centralized_signature,
                <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>,
        },
        ctx,
    );
    // Create a new `<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignature">PartialUserSignature</a>` that wraps around `presign_cap` to ensure it can't be used twice.
    self.partial_centralized_signed_messages.add(partial_centralized_signed_message_id, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PartialUserSignature">PartialUserSignature</a> {
        id: id,
        created_at_epoch: self.current_epoch,
        presign_cap,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
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


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_future_sign">respond_future_sign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, partial_centralized_signed_message_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, rejected: bool, session_sequence_number: u64): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_future_sign">respond_future_sign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: ID,
    partial_centralized_signed_message_id: ID,
    rejected: bool,
    session_sequence_number: u64
): Balance&lt;SUI&gt; {
    <b>let</b> status = <b>if</b> (rejected) {
        DWalletSessionStatusEvent::Rejected(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedFutureSignEvent">RejectedFutureSignEvent</a> {
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
            partial_centralized_signed_message_id,
        })
    } <b>else</b> {
        DWalletSessionStatusEvent::Success(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedFutureSignEvent">CompletedFutureSignEvent</a> {
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
            partial_centralized_signed_message_id,
        })
    };
    <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_user_initiated_session_and_charge">remove_user_initiated_session_and_charge</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_FutureSignRequestEvent">FutureSignRequestEvent</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedFutureSignEvent">CompletedFutureSignEvent</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedFutureSignEvent">RejectedFutureSignEvent</a>&gt;(session_sequence_number, status);
    <b>let</b> partial_centralized_signed_message = self.partial_centralized_signed_messages.borrow_mut(partial_centralized_signed_message_id);
    // Check that the presign is global, or that it belongs to this dWallet.
    <b>assert</b>!(partial_centralized_signed_message.presign_cap.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>.is_none() || partial_centralized_signed_message.presign_cap.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>.is_some_and!(|id| id == <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletMismatch">EDWalletMismatch</a>);
    partial_centralized_signed_message.state = match(partial_centralized_signed_message.state) {
        PartialUserSignatureState::AwaitingNetworkVerification =&gt; {
            <b>if</b>(rejected) {
                PartialUserSignatureState::NetworkVerificationRejected
            } <b>else</b> {
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

Checks that the partial user signature corresponding to <code>cap</code> is valid, by assuring it is in the <code>NetworkVerificationCompleted</code> state.


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


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_sign_with_partial_user_signature">request_sign_with_partial_user_signature</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, partial_user_signature_cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap">dwallet_2pc_mpc_coordinator_inner::VerifiedPartialUserSignatureCap</a>, message_approval: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">dwallet_2pc_mpc_coordinator_inner::MessageApproval</a>, session_identifier: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">dwallet_2pc_mpc_coordinator_inner::SessionIdentifier</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_sign_with_partial_user_signature">request_sign_with_partial_user_signature</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    partial_user_signature_cap: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap">VerifiedPartialUserSignatureCap</a>,
    message_approval: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MessageApproval">MessageApproval</a>,
    session_identifier: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">SessionIdentifier</a>,
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
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: _,
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
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
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
        session_identifier,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
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


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_imported_key_sign_with_partial_user_signature">request_imported_key_sign_with_partial_user_signature</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, partial_user_signature_cap: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap">dwallet_2pc_mpc_coordinator_inner::VerifiedPartialUserSignatureCap</a>, message_approval: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyMessageApproval">dwallet_2pc_mpc_coordinator_inner::ImportedKeyMessageApproval</a>, session_identifier: (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">dwallet_2pc_mpc_coordinator_inner::SessionIdentifier</a>, payment_ika: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;(ika=0x0)::ika::IKA&gt;, payment_sui: &<b>mut</b> <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_request_imported_key_sign_with_partial_user_signature">request_imported_key_sign_with_partial_user_signature</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    partial_user_signature_cap: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_VerifiedPartialUserSignatureCap">VerifiedPartialUserSignatureCap</a>,
    message_approval: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyMessageApproval">ImportedKeyMessageApproval</a>,
    session_identifier: <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SessionIdentifier">SessionIdentifier</a>,
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
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: _,
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
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
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
        session_identifier,
        <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
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
    partial_signature.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a> == message_approval.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a> &&
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
    partial_signature.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a> == message_approval.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a> &&
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


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_sign">respond_sign</a>(self: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, sign_id: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>, signature: vector&lt;u8&gt;, is_future_sign: bool, rejected: bool, session_sequence_number: u64): <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;<a href="../sui/sui.md#sui_sui_SUI">sui::sui::SUI</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_sign">respond_sign</a>(
    self: &<b>mut</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>: ID,
    sign_id: ID,
    signature: vector&lt;u8&gt;,
    is_future_sign: bool,
    rejected: bool,
    session_sequence_number: u64
): Balance&lt;SUI&gt; {
    <b>let</b> status = <b>if</b> (rejected) {
        DWalletSessionStatusEvent::Rejected(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedSignEvent">RejectedSignEvent</a> {
            sign_id,
            is_future_sign,
        })
    } <b>else</b> {
        DWalletSessionStatusEvent::Success(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedSignEvent">CompletedSignEvent</a> {
            sign_id,
            signature,
            is_future_sign,
        })
    };
    <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_remove_user_initiated_session_and_charge">remove_user_initiated_session_and_charge</a>&lt;<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SignRequestEvent">SignRequestEvent</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_CompletedSignEvent">CompletedSignEvent</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RejectedSignEvent">RejectedSignEvent</a>&gt;(session_sequence_number, status);
    <b>let</b> (dwallet, _) = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_active_dwallet_and_public_output_mut">get_active_dwallet_and_public_output_mut</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>);
    <b>let</b> sign = dwallet.sign_sessions.borrow_mut(sign_id);
    sign.state = match(sign.state) {
        SignState::Requested =&gt; {
            <b>if</b>(rejected) {
                SignState::NetworkRejected
            } <b>else</b> {
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

Processes a checkpoint message that has been signed by a validator quorum.

Verifies the BLS multi-signature from the active validator committee before
processing the checkpoint contents. This ensures only valid, consensus-approved
checkpoints are processed.


<a name="@Parameters_202"></a>

##### Parameters

- <code>self</code>: Mutable reference to the coordinator
- <code>signature</code>: BLS multi-signature from validators
- <code>signers_bitmap</code>: Bitmap indicating which validators signed
- <code>message</code>: Checkpoint message content to process
- <code>ctx</code>: Transaction context for coin creation


<a name="@Returns_203"></a>

##### Returns

SUI coin containing gas fee reimbursements from processed operations


<a name="@Effects_204"></a>

##### Effects

- Verifies the signature against the active committee
- Processes all operations contained in the checkpoint
- Updates session states and emits relevant events
- Collects and returns gas fee reimbursements


<a name="@Aborts_205"></a>

##### Aborts

- BLS verification errors if signature is invalid
- Various operation-specific errors during checkpoint processing


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
    <b>assert</b>!(self.last_processed_checkpoint_sequence_number + 1 == sequence_number, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EWrongCheckpointSequenceNumber">EWrongCheckpointSequenceNumber</a>);
    self.last_processed_checkpoint_sequence_number = sequence_number;
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
        <b>let</b> message_data_enum_tag = bcs_body.peel_enum_tag();
        // Parses checkpoint BCS bytes directly.
        match (message_data_enum_tag) {
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_DKG_FIRST_ROUND_OUTPUT_MESSAGE_TYPE">RESPOND_DWALLET_DKG_FIRST_ROUND_OUTPUT_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a> = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> first_round_output = bcs_body.peel_vec_u8();
                <b>let</b> rejected = bcs_body.peel_bool();
                <b>let</b> session_sequence_number = bcs_body.peel_u64();
                <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_dkg_first_round">respond_dwallet_dkg_first_round</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>, first_round_output, rejected, session_sequence_number);
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            },
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_DKG_SECOND_ROUND_OUTPUT_MESSAGE_TYPE">RESPOND_DWALLET_DKG_SECOND_ROUND_OUTPUT_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a> = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> encrypted_user_secret_key_share_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> public_output = bcs_body.peel_vec_u8();
                <b>let</b> rejected = bcs_body.peel_bool();
                <b>let</b> session_sequence_number = bcs_body.peel_u64();
                <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_dkg_second_round">respond_dwallet_dkg_second_round</a>(
                    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
                    public_output,
                    encrypted_user_secret_key_share_id,
                    rejected,
                    session_sequence_number,
                );
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            },
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_ENCRYPTED_USER_SHARE_MESSAGE_TYPE">RESPOND_DWALLET_ENCRYPTED_USER_SHARE_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a> = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> encrypted_user_secret_key_share_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> rejected = bcs_body.peel_bool();
                <b>let</b> session_sequence_number = bcs_body.peel_u64();
                <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_re_encrypt_user_share_for">respond_re_encrypt_user_share_for</a>(
                    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
                    encrypted_user_secret_key_share_id,
                    rejected,
                    session_sequence_number,
                );
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            },
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_MAKE_DWALLET_USER_SECRET_KEY_SHARES_PUBLIC_MESSAGE_TYPE">RESPOND_MAKE_DWALLET_USER_SECRET_KEY_SHARES_PUBLIC_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a> = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> public_user_secret_key_shares = bcs_body.peel_vec_u8();
                <b>let</b> rejected = bcs_body.peel_bool();
                <b>let</b> session_sequence_number = bcs_body.peel_u64();
                <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_make_dwallet_user_secret_key_share_public">respond_make_dwallet_user_secret_key_share_public</a>(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>, public_user_secret_key_shares, rejected, session_sequence_number);
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            },
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_IMPORTED_KEY_VERIFICATION_OUTPUT_MESSAGE_TYPE">RESPOND_DWALLET_IMPORTED_KEY_VERIFICATION_OUTPUT_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a> = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> public_output = bcs_body.peel_vec_u8();
                <b>let</b> encrypted_user_secret_key_share_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> rejected = bcs_body.peel_bool();
                <b>let</b> session_sequence_number = bcs_body.peel_u64();
                <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_imported_key_dwallet_verification">respond_imported_key_dwallet_verification</a>(
                    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
                    public_output,
                    encrypted_user_secret_key_share_id,
                    rejected,
                    session_sequence_number
                );
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            },
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_PRESIGN_MESSAGE_TYPE">RESPOND_DWALLET_PRESIGN_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a> = bcs_body.peel_option!(|bcs_option| object::id_from_bytes(bcs_option.peel_vec_u8()));
                <b>let</b> presign_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> presign = bcs_body.peel_vec_u8();
                <b>let</b> rejected = bcs_body.peel_bool();
                <b>let</b> session_sequence_number = bcs_body.peel_u64();
                <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_presign">respond_presign</a>(
                    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
                    presign_id,
                    presign,
                    rejected,
                    session_sequence_number);
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            },
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_SIGN_MESSAGE_TYPE">RESPOND_DWALLET_SIGN_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a> = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> sign_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> signature = bcs_body.peel_vec_u8();
                <b>let</b> is_future_sign = bcs_body.peel_bool();
                <b>let</b> rejected = bcs_body.peel_bool();
                <b>let</b> session_sequence_number = bcs_body.peel_u64();
                <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_sign">respond_sign</a>(
                    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
                    sign_id,
                    signature,
                    is_future_sign,
                    rejected,
                    session_sequence_number
                );
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            },
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_PARTIAL_SIGNATURE_VERIFICATION_OUTPUT_MESSAGE_TYPE">RESPOND_DWALLET_PARTIAL_SIGNATURE_VERIFICATION_OUTPUT_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a> = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> partial_centralized_signed_message_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> rejected = bcs_body.peel_bool();
                <b>let</b> session_sequence_number = bcs_body.peel_u64();
                <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_future_sign">respond_future_sign</a>(
                    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>,
                    partial_centralized_signed_message_id,
                    rejected,
                    session_sequence_number
                );
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            },
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_MPC_NETWORK_DKG_OUTPUT_MESSAGE_TYPE">RESPOND_DWALLET_MPC_NETWORK_DKG_OUTPUT_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> session_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a> = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> public_output = bcs_body.peel_vec_u8();
                <b>let</b> supported_curves = bcs_body.peel_vec_u32();
                <b>let</b> is_last = bcs_body.peel_bool();
                <b>let</b> rejected = bcs_body.peel_bool();
                <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_network_encryption_key_dkg">respond_dwallet_network_encryption_key_dkg</a>(session_id, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>, public_output, supported_curves,is_last, rejected, ctx);
                total_gas_fee_reimbursement_sui.join(gas_fee_reimbursement_sui);
            },
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RESPOND_DWALLET_MPC_NETWORK_RECONFIGURATION_OUTPUT_MESSAGE_TYPE">RESPOND_DWALLET_MPC_NETWORK_RECONFIGURATION_OUTPUT_MESSAGE_TYPE</a> =&gt; {
                <b>let</b> session_id = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a> = object::id_from_bytes(bcs_body.peel_vec_u8());
                <b>let</b> public_output = bcs_body.peel_vec_u8();
                <b>let</b> supported_curves = bcs_body.peel_vec_u32();
                <b>let</b> is_last = bcs_body.peel_bool();
                <b>let</b> rejected = bcs_body.peel_bool();
                <b>let</b> gas_fee_reimbursement_sui = self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_respond_dwallet_network_encryption_key_reconfiguration">respond_dwallet_network_encryption_key_reconfiguration</a>(session_id, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>, public_output, supported_curves, is_last, rejected, ctx);
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
            <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_END_OF_EPOCH_MESSAGE_TYPE">END_OF_EPOCH_MESSAGE_TYPE</a> =&gt; {
                self.received_end_of_publish = <b>true</b>;
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

Sets the supported curves, signature algorithms and hash schemes, and the default pricing.

This function is used to set the supported curves, signature algorithms and hash schemes, and the default pricing.
Default pricing is used to set the pricing for a protocol or curve if pricing is missing for a protocol or curve
and it has to contain the default pricing for all protocols and curves as set in the <code>supported_curves_to_signature_algorithms_to_hash_schemes</code> parameter.


<a name="@Parameters_206"></a>

##### Parameters

- **<code>default_pricing</code>**: The default pricing to use if pricing is missing for a protocol or curve.
- **<code>supported_curves_to_signature_algorithms_to_hash_schemes</code>**: A map of curves to signature algorithms to hash schemes.


<a name="@Errors_207"></a>

##### Errors

- **<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMissingProtocolPricing">EMissingProtocolPricing</a></code>**: If pricing is missing for any protocol or curve.


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


<a name="@Parameters_208"></a>

##### Parameters

- **<code>supported_curves_to_signature_algorithms_to_hash_schemes</code>**: A map of curves to signature algorithms to hash schemes.
- **<code>default_pricing</code>**: The default pricing to use if pricing is missing for a protocol or curve.


<a name="@Errors_209"></a>

##### Errors

- **<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMissingProtocolPricing">EMissingProtocolPricing</a></code>**: If pricing is missing for any protocol or curve.


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_verify_pricing_exists_for_all_protocols">verify_pricing_exists_for_all_protocols</a>(supported_curves_to_signature_algorithms_to_hash_schemes: &<a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u32, <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u32, vector&lt;u32&gt;&gt;&gt;, default_pricing: &(ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing">dwallet_pricing::DWalletPricing</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_verify_pricing_exists_for_all_protocols">verify_pricing_exists_for_all_protocols</a>(supported_curves_to_signature_algorithms_to_hash_schemes: &VecMap&lt;u32, VecMap&lt;u32, vector&lt;u32&gt;&gt;&gt;, default_pricing: &DWalletPricing) {
    <b>let</b> curves = supported_curves_to_signature_algorithms_to_hash_schemes.keys();
    curves.do_ref!(|curve| {
        <b>let</b> <b>mut</b> is_missing_pricing = <b>false</b>;
        <b>let</b> signature_algorithms = &supported_curves_to_signature_algorithms_to_hash_schemes[curve];
        <b>let</b> signature_algorithms = signature_algorithms.keys();
        is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(*curve, option::none(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DKG_FIRST_ROUND_PROTOCOL_FLAG">DKG_FIRST_ROUND_PROTOCOL_FLAG</a>).is_none();
        is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(*curve, option::none(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DKG_SECOND_ROUND_PROTOCOL_FLAG">DKG_SECOND_ROUND_PROTOCOL_FLAG</a>).is_none();
        is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(*curve, option::none(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_RE_ENCRYPT_USER_SHARE_PROTOCOL_FLAG">RE_ENCRYPT_USER_SHARE_PROTOCOL_FLAG</a>).is_none();
        is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(*curve, option::none(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_MAKE_DWALLET_USER_SECRET_KEY_SHARE_PUBLIC_PROTOCOL_FLAG">MAKE_DWALLET_USER_SECRET_KEY_SHARE_PUBLIC_PROTOCOL_FLAG</a>).is_none();
        is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(*curve, option::none(), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_IMPORTED_KEY_DWALLET_VERIFICATION_PROTOCOL_FLAG">IMPORTED_KEY_DWALLET_VERIFICATION_PROTOCOL_FLAG</a>).is_none();
        // Add here pricing validation <b>for</b> new protocols per curve.
        signature_algorithms.do_ref!(|signature_algorithm| {
            is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(*curve, option::some(*signature_algorithm), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_PRESIGN_PROTOCOL_FLAG">PRESIGN_PROTOCOL_FLAG</a>).is_none();
            is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(*curve, option::some(*signature_algorithm), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SIGN_PROTOCOL_FLAG">SIGN_PROTOCOL_FLAG</a>).is_none();
            is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(*curve, option::some(*signature_algorithm), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_FUTURE_SIGN_PROTOCOL_FLAG">FUTURE_SIGN_PROTOCOL_FLAG</a>).is_none();
            is_missing_pricing = is_missing_pricing || default_pricing.try_get_dwallet_pricing_value(*curve, option::some(*signature_algorithm), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_SIGN_WITH_PARTIAL_USER_SIGNATURE_PROTOCOL_FLAG">SIGN_WITH_PARTIAL_USER_SIGNATURE_PROTOCOL_FLAG</a>).is_none();
            // Add here pricing validation <b>for</b> new protocols per curve per signature algorithm.
        });
        <b>assert</b>!(!is_missing_pricing, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EMissingProtocolPricing">EMissingProtocolPricing</a>);
    });
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_set_paused_curves_and_signature_algorithms"></a>

## Function `set_paused_curves_and_signature_algorithms`

Sets the paused curves, signature algorithms and hash schemes.

This function is used to set the paused curves, signature algorithms and hash schemes.


<a name="@Parameters_210"></a>

##### Parameters

- **<code>paused_curves</code>**: The curves to pause.
- **<code>paused_signature_algorithms</code>**: The signature algorithms to pause.
- **<code>paused_hash_schemes</code>**: The hash schemes to pause.


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

Sets the pricing vote for a validator.

This function is used to set the pricing vote for a validator.
Cannot be called during the votes calculation.


<a name="@Parameters_211"></a>

##### Parameters

- **<code>validator_id</code>**: The ID of the validator.
- **<code>pricing_vote</code>**: The pricing vote for the validator.


<a name="@Errors_212"></a>

##### Errors

- **<code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ECannotSetDuringVotesCalculation">ECannotSetDuringVotesCalculation</a></code>**: If the pricing vote is set during the votes calculation.


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

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id"></a>

## Function `dwallet_network_encryption_key_id`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyCap">dwallet_2pc_mpc_coordinator_inner::DWalletNetworkEncryptionKeyCap</a>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>(self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletNetworkEncryptionKeyCap">DWalletNetworkEncryptionKeyCap</a>): ID {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_current_pricing"></a>

## Function `current_pricing`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_current_pricing">current_pricing</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>): (ika_system=0x0)::<a href="../ika_system/dwallet_pricing.md#(ika_system=0x0)_dwallet_pricing_DWalletPricing">dwallet_pricing::DWalletPricing</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_current_pricing">current_pricing</a>(self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>): DWalletPricing {
    self.pricing_and_fee_management.current
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_network_encryption_key_supported_curves"></a>

## Function `get_network_encryption_key_supported_curves`

Get the supported curves for a network encryption key.


<a name="@Parameters_213"></a>

##### Parameters

- <code>self</code>: Reference to the coordinator
- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a></code>: ID of the network encryption key


<a name="@Returns_214"></a>

##### Returns

Vector of supported curve identifiers


<a name="@Aborts_215"></a>

##### Aborts

- <code><a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkEncryptionKeyNotExist">EDWalletNetworkEncryptionKeyNotExist</a></code>: If the network encryption key doesn't exist


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_network_encryption_key_supported_curves">get_network_encryption_key_supported_curves</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">dwallet_2pc_mpc_coordinator_inner::DWalletCoordinatorInner</a>, <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>): vector&lt;u32&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_get_network_encryption_key_supported_curves">get_network_encryption_key_supported_curves</a>(
    self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCoordinatorInner">DWalletCoordinatorInner</a>,
    <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>: ID,
): vector&lt;u32&gt; {
    <b>assert</b>!(self.dwallet_network_encryption_keys.contains(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>), <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_EDWalletNetworkEncryptionKeyNotExist">EDWalletNetworkEncryptionKeyNotExist</a>);
    <b>let</b> dwallet_network_encryption_key = self.dwallet_network_encryption_keys.borrow(<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_network_encryption_key_id">dwallet_network_encryption_key_id</a>);
    dwallet_network_encryption_key.supported_curves
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id"></a>

## Function `dwallet_id`

=== Public Functions ===


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">dwallet_2pc_mpc_coordinator_inner::DWalletCap</a>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>(self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_DWalletCap">DWalletCap</a>): ID {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_imported_key_dwallet_id"></a>

## Function `imported_key_dwallet_id`



<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_imported_key_dwallet_id">imported_key_dwallet_id</a>(self: &(ika_system=0x0)::<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyDWalletCap">dwallet_2pc_mpc_coordinator_inner::ImportedKeyDWalletCap</a>): <a href="../sui/object.md#sui_object_ID">sui::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_imported_key_dwallet_id">imported_key_dwallet_id</a>(self: &<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_ImportedKeyDWalletCap">ImportedKeyDWalletCap</a>): ID {
    self.<a href="../ika_system/dwallet_2pc_mpc_coordinator_inner.md#(ika_system=0x0)_dwallet_2pc_mpc_coordinator_inner_dwallet_id">dwallet_id</a>
}
</code></pre>



</details>
