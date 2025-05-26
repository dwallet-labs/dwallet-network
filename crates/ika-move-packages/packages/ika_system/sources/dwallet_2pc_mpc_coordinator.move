// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module ika_system::dwallet_2pc_mpc_coordinator;

use ika::ika::IKA;
use sui::sui::SUI;
use sui::coin::{Coin};
use sui::dynamic_field;
use ika_system::dwallet_2pc_mpc_coordinator_inner::{
    Self,
    DWalletCoordinatorInner,
    DWalletNetworkEncryptionKeyCap,
    DWalletCap,
    ImportedKeyDWalletCap,
    UnverifiedPresignCap,
    VerifiedPresignCap,
    MessageApproval,
    ImportedKeyMessageApproval,
    UnverifiedPartialUserSignatureCap,
    VerifiedPartialUserSignatureCap
};
use ika_system::bls_committee::BlsCommittee;
use ika_system::dwallet_pricing::DWalletPricing;
use sui::vec_map::VecMap;

public struct DWalletCoordinator has key {
    id: UID,
    version: u64,
    package_id: ID,
    new_package_id: Option<ID>,
}

const EWrongInnerVersion: u64 = 0;
const EInvalidMigration: u64 = 1;

/// Flag to indicate the version of the ika system.
const VERSION: u64 = 1;

// ==== functions that can only be called by init ====

/// Create a new System object and make it shared.
/// This function will be called only once in init.
public(package) fun create_dwallet_coordinator(
    package_id: ID,
    epoch: u64,
    active_committee: BlsCommittee,
    pricing: DWalletPricing,
    supported_curves_to_signature_algorithms_to_hash_schemes: VecMap<u32, VecMap<u32, vector<u32>>>,
    ctx: &mut TxContext
): DWalletCoordinator {
    let dwallet_coordinator_inner = dwallet_2pc_mpc_coordinator_inner::create_dwallet_coordinator_inner(
        epoch,
        active_committee,
        pricing,
        supported_curves_to_signature_algorithms_to_hash_schemes,
        ctx,
    );
    let mut self = DWalletCoordinator {
        id: object::new(ctx),
        version: VERSION,
        package_id,
        new_package_id: option::none(),
    };
    dynamic_field::add(&mut self.id, VERSION, dwallet_coordinator_inner);
    self
}

public(package) fun share_dwallet_coordinator(
    dwallet_coordinator: DWalletCoordinator,
) {
    transfer::share_object(dwallet_coordinator);
}

/// Being called by the Ika network to store outputs of completed MPC sessions to Sui.
public fun process_checkpoint_message_by_quorum(
    dwallet_2pc_mpc_coordinator: &mut DWalletCoordinator,
    signature: vector<u8>,
    signers_bitmap: vector<u8>,
    mut message: vector<u8>,
    message2: vector<u8>,
    message3: vector<u8>,
    message4: vector<u8>,
    ctx: &mut TxContext,
): Coin<SUI> {
    message.append(message2);
    message.append(message3);
    message.append(message4);
    let dwallet_inner = dwallet_2pc_mpc_coordinator.inner_mut();
    dwallet_inner.process_checkpoint_message_by_quorum(signature, signers_bitmap, message, ctx)
}

public(package) fun request_dwallet_network_encryption_key_dkg(
    self: &mut DWalletCoordinator,
    ctx: &mut TxContext
): DWalletNetworkEncryptionKeyCap {
    self.inner_mut().request_dwallet_network_encryption_key_dkg(ctx)
}

public fun get_active_encryption_key(
    self: &DWalletCoordinator,
    address: address,
): ID {
    self.inner().get_active_encryption_key(address)
}

public fun register_encryption_key(
    self: &mut DWalletCoordinator,
    curve: u32,
    encryption_key: vector<u8>,
    encryption_key_signature: vector<u8>,
    signer_public_key: vector<u8>,
    ctx: &mut TxContext
) {
    self.inner_mut().register_encryption_key(
        curve,
        encryption_key,
        encryption_key_signature,
        signer_public_key,
        ctx
    )
}

public fun approve_message(
    self: &mut DWalletCoordinator,
    dwallet_cap: &DWalletCap,
    signature_algorithm: u32,
    hash_scheme: u32,
    message: vector<u8>
): MessageApproval {
    self.inner().approve_message(
        dwallet_cap,
        signature_algorithm,
        hash_scheme,
        message,
    )
}

public fun approve_imported_key_message(
    self: &mut DWalletCoordinator,
    imported_key_dwallet_cap: &ImportedKeyDWalletCap,
    signature_algorithm: u32,
    hash_scheme: u32,
    message: vector<u8>
): ImportedKeyMessageApproval {
    self.inner().approve_imported_key_message(
        imported_key_dwallet_cap,
        signature_algorithm,
        hash_scheme,
        message,
    )
}

public fun request_dwallet_dkg_first_round(
    self: &mut DWalletCoordinator,
    dwallet_network_encryption_key_id: ID,
    curve: u32,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
): DWalletCap {
    self.inner_mut().request_dwallet_dkg_first_round(
        dwallet_network_encryption_key_id,
        curve,
        payment_ika,
        payment_sui,
        ctx
    )
}

public fun request_dwallet_dkg_second_round(
    self: &mut DWalletCoordinator,
    dwallet_cap: &DWalletCap,
    centralized_public_key_share_and_proof: vector<u8>,
    encrypted_centralized_secret_share_and_proof: vector<u8>,
    encryption_key_address: address,
    user_public_output: vector<u8>,
    singer_public_key: vector<u8>,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
) {
    self.inner_mut().request_dwallet_dkg_second_round(
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

public fun new_imported_key_dwallet(
    self: &mut DWalletCoordinator,
    dwallet_network_encryption_key_id: ID,
    curve: u32,
    ctx: &mut TxContext
): ImportedKeyDWalletCap {
    self.inner_mut().new_imported_key_dwallet(
        dwallet_network_encryption_key_id,
        curve,
        ctx,
    )
}

public fun request_imported_key_dwallet_verification(
    self: &mut DWalletCoordinator,
    dwallet_cap: &ImportedKeyDWalletCap,
    centralized_party_message: vector<u8>,
    encrypted_centralized_secret_share_and_proof: vector<u8>,
    encryption_key_address: address,
    user_public_output: vector<u8>,
    signer_public_key: vector<u8>,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
) {
    self.inner_mut().request_imported_key_dwallet_verification(
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

public fun request_make_dwallet_user_secret_key_shares_public(
    self: &mut DWalletCoordinator,
    dwallet_id: ID,
    public_user_secret_key_shares: vector<u8>,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext,
) {
    self.inner_mut().request_make_dwallet_user_secret_key_share_public(
        dwallet_id,
        public_user_secret_key_shares,
        payment_ika,
        payment_sui,
        ctx,
    )
}

public fun request_re_encrypt_user_share_for(
    self: &mut DWalletCoordinator,
    dwallet_id: ID,
    destination_encryption_key_address: address,
    encrypted_centralized_secret_share_and_proof: vector<u8>,
    source_encrypted_user_secret_key_share_id: ID,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext,
) {
    self.inner_mut().request_re_encrypt_user_share_for(
        dwallet_id,
        destination_encryption_key_address,
        encrypted_centralized_secret_share_and_proof,
        source_encrypted_user_secret_key_share_id,
        payment_ika,
        payment_sui,
        ctx,
    )
}

public fun accept_encrypted_user_share(
    self: &mut DWalletCoordinator,
    dwallet_id: ID,
    encrypted_user_secret_key_share_id: ID,
    user_output_signature: vector<u8>,
) {
    self.inner_mut().accept_encrypted_user_share(
        dwallet_id,
        encrypted_user_secret_key_share_id,
        user_output_signature,
    )
}

public fun request_presign(
    self: &mut DWalletCoordinator,
    dwallet_id: ID,
    signature_algorithm: u32,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
): UnverifiedPresignCap {
    self.inner_mut().request_presign(
        dwallet_id,
        signature_algorithm,
        payment_ika,
        payment_sui,
        ctx,
    )
}

public fun request_global_presign(
    self: &mut DWalletCoordinator,
    dwallet_network_encryption_key_id: ID,
    curve: u32,
    signature_algorithm: u32,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
): UnverifiedPresignCap {
    self.inner_mut().request_global_presign(
        dwallet_network_encryption_key_id,
        curve,
        signature_algorithm,
        payment_ika,
        payment_sui,
        ctx,
    )
}

public fun is_presign_valid(
    self: &DWalletCoordinator,
    presign_cap: &UnverifiedPresignCap,
): bool {
    self.inner().is_presign_valid(
        presign_cap,
    )
}

public fun verify_presign_cap(
    self: &mut DWalletCoordinator,
    cap: UnverifiedPresignCap,
    ctx: &mut TxContext
): VerifiedPresignCap {
    self.inner_mut().verify_presign_cap(cap, ctx)
}

public fun request_sign(
    self: &mut DWalletCoordinator,
    presign_cap: VerifiedPresignCap,
    message_approval: MessageApproval,
    message_centralized_signature: vector<u8>,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
) {
    self.inner_mut().request_sign(
        message_approval,
        presign_cap,
        message_centralized_signature,
        payment_ika,
        payment_sui,
        ctx
    )
}

public fun request_imported_key_sign(
    self: &mut DWalletCoordinator,
    presign_cap: VerifiedPresignCap,
    message_approval: ImportedKeyMessageApproval,
    message_centralized_signature: vector<u8>,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
) {
    self.inner_mut().request_imported_key_sign(
        message_approval,
        presign_cap,
        message_centralized_signature,
        payment_ika,
        payment_sui,
        ctx
    )
}

public fun request_future_sign(
    self: &mut DWalletCoordinator,
    dwallet_id: ID,
    presign_cap: VerifiedPresignCap,
    message: vector<u8>,
    hash_scheme: u32,
    message_centralized_signature: vector<u8>,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
): UnverifiedPartialUserSignatureCap {
    self.inner_mut().request_future_sign(
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

public fun is_partial_user_signature_valid(
    self: &DWalletCoordinator,
    cap: &UnverifiedPartialUserSignatureCap,
): bool {
    self.inner().is_partial_user_signature_valid(cap)
}

public fun verify_partial_user_signature_cap(
    self: &mut DWalletCoordinator,
    cap: UnverifiedPartialUserSignatureCap,
    ctx: &mut TxContext
): VerifiedPartialUserSignatureCap {
    self.inner_mut().verify_partial_user_signature_cap(
        cap,
        ctx,
    )
}

public fun request_sign_with_partial_user_signature(
    self: &mut DWalletCoordinator,
    partial_user_signature_cap: VerifiedPartialUserSignatureCap,
    message_approval: MessageApproval,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
) {
    self.inner_mut().request_sign_with_partial_user_signature(
        partial_user_signature_cap,
        message_approval,
        payment_ika,
        payment_sui,
        ctx,
    )
}


public fun request_imported_key_sign_with_partial_user_signature(
    self: &mut DWalletCoordinator,
    partial_user_signature_cap: VerifiedPartialUserSignatureCap,
    message_approval: ImportedKeyMessageApproval,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
) {
    self.inner_mut().request_imported_key_sign_with_partial_user_signature(
        partial_user_signature_cap,
        message_approval,
        payment_ika,
        payment_sui,
        ctx,
    )
}

public fun match_partial_user_signature_with_message_approval(
    self: &DWalletCoordinator,
    partial_user_signature_cap: &VerifiedPartialUserSignatureCap,
    message_approval: &MessageApproval,
): bool {
    self.inner().match_partial_user_signature_with_message_approval(
        partial_user_signature_cap,
        message_approval,
    )
}

public fun match_partial_user_signature_with_imported_key_message_approval(
    self: &DWalletCoordinator,
    partial_user_signature_cap: &VerifiedPartialUserSignatureCap,
    message_approval: &ImportedKeyMessageApproval,
): bool {
    self.inner().match_partial_user_signature_with_imported_key_message_approval(
        partial_user_signature_cap,
        message_approval,
    )
}

/// Migrate the dwallet_2pc_mpc_coordinator object to the new package id.
///
/// This function sets the new package id and version and can be modified in future versions
/// to migrate changes in the `dwallet_2pc_mpc_coordinator_inner` object if needed.
public fun migrate(
        self: &mut DWalletCoordinator,
) {
    assert!(self.version < VERSION, EInvalidMigration);

    // Move the old system state inner to the new version.
    let dwallet_2pc_mpc_coordinator_inner: DWalletCoordinatorInner = dynamic_field::remove(&mut self.id, self.version);
    dynamic_field::add(&mut self.id, VERSION, dwallet_2pc_mpc_coordinator_inner);
    self.version = VERSION;

    // Set the new package id.
    assert!(self.new_package_id.is_some(), EInvalidMigration);
    self.package_id = self.new_package_id.extract();
}

// === Internals ===

/// Get a mutable reference to `DWalletCoordinatorInnerVX` from the `DWalletCoordinator`.
public(package) fun inner_mut(self: &mut DWalletCoordinator): &mut DWalletCoordinatorInner {
    assert!(self.version == VERSION, EWrongInnerVersion);
    dynamic_field::borrow_mut(&mut self.id, VERSION)
}

/// Get an immutable reference to `DWalletCoordinatorVX` from the `DWalletCoordinator`.
public(package) fun inner(self: &DWalletCoordinator): &DWalletCoordinatorInner {
    assert!(self.version == VERSION, EWrongInnerVersion);
    dynamic_field::borrow(&self.id, VERSION)
}