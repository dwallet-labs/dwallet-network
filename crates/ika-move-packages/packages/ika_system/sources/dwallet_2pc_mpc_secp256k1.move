module ika_system::dwallet_2pc_mpc_secp256k1;

use ika::ika::IKA;
use sui::balance::Balance;
use sui::sui::SUI;
use sui::coin::{Coin};
use sui::dynamic_field;
use ika_system::dwallet_pricing::{DWalletPricing2PcMpcSecp256K1};
use ika_system::dwallet_2pc_mpc_secp256k1_inner::{
    Self,
    DWalletCoordinatorInner,
    DWalletNetworkDecryptionKeyCap,
    DWalletCap,
    ECDSAPresignCap,
    MessageApproval,
    UnverifiedECDSAPartialUserSignatureCap,
    VerifiedECDSAPartialUserSignatureCap
};
use ika_system::bls_committee::BlsCommittee;

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
    pricing: DWalletPricing2PcMpcSecp256K1,
    ctx: &mut TxContext
): ID {
    let dwallet_coordinator_inner = dwallet_2pc_mpc_secp256k1_inner::create_dwallet_coordinator_inner(
        epoch,
        active_committee,
        pricing,
        ctx,
    );
        // TODO: remove this code!
    let mut self = DWalletCoordinator {
        id: object::new(ctx),
        version: VERSION,
        package_id,
        new_package_id: option::none(),
    };
    let self_id = object::id(&self);
    dynamic_field::add(&mut self.id, VERSION, dwallet_coordinator_inner);
    transfer::share_object(self);
    self_id
}


public(package) fun request_dwallet_network_decryption_key_dkg(
    self: &mut DWalletCoordinator,
    ctx: &mut TxContext
): DWalletNetworkDecryptionKeyCap {
    self.inner_mut().request_dwallet_network_decryption_key_dkg(ctx)
}

public(package) fun advance_epoch(
    self: &mut DWalletCoordinator,
    committee: BlsCommittee,
): Balance<IKA> {
    self.inner_mut().advance_epoch(committee)
}

public fun get_active_encryption_key(
    self: &DWalletCoordinator,
    address: address,
): ID {
    self.inner().get_active_encryption_key(address)
}

public fun register_encryption_key(
    self: &mut DWalletCoordinator,
    encryption_key: vector<u8>,
    encryption_key_signature: vector<u8>,
    signer_public_key: vector<u8>,
    ctx: &mut TxContext
) {
    self.inner_mut().register_encryption_key(
        encryption_key,
        encryption_key_signature,
        signer_public_key,
        ctx
    )
}

public fun request_dwallet_dkg_first_round(
    self: &mut DWalletCoordinator,
    dwallet_network_decryption_key_id: ID,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
): DWalletCap {
    self.inner_mut().request_dwallet_dkg_first_round(
        dwallet_network_decryption_key_id,
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

// TODO (#493): Remove mock functions
public fun create_first_round_dwallet_mock(self: &mut DWalletCoordinator, first_round_output: vector<u8>, dwallet_network_decryption_key_id: ID, ctx: &mut TxContext): DWalletCap {
    self.inner_mut().create_first_round_dwallet_mock(first_round_output, dwallet_network_decryption_key_id, ctx)
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

public fun request_ecdsa_presign(
    self: &mut DWalletCoordinator,
    dwallet_id: ID,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
): ECDSAPresignCap {
    self.inner_mut().request_ecdsa_presign(
        dwallet_id,
        payment_ika,
        payment_sui,
        ctx,
    )
}

public fun is_ecdsa_presign_valid(
    self: &DWalletCoordinator,
    presign_cap: &ECDSAPresignCap,
): bool {
    self.inner().is_ecdsa_presign_valid(
        presign_cap,
    )
}

public fun request_ecdsa_sign(
    self: &mut DWalletCoordinator,
    presign_cap: ECDSAPresignCap,
    message_approval: MessageApproval,
    message_centralized_signature: vector<u8>,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
) {
    self.inner_mut().request_ecdsa_sign(
        message_approval,
        presign_cap,
        message_centralized_signature,
        payment_ika,
        payment_sui,
        ctx
    )
}

public fun request_ecdsa_future_sign(
    self: &mut DWalletCoordinator,
    presign_cap: ECDSAPresignCap,
    message: vector<u8>,
    hash_scheme: u8,
    message_centralized_signature: vector<u8>,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
): UnverifiedECDSAPartialUserSignatureCap {
    self.inner_mut().request_ecdsa_future_sign(
        presign_cap,
        message,
        hash_scheme,
        message_centralized_signature,
        payment_ika,
        payment_sui,
        ctx,
    )
}

public fun verify_ecdsa_partial_user_signature_cap(
    self: &mut DWalletCoordinator,
    cap: UnverifiedECDSAPartialUserSignatureCap,
    ctx: &mut TxContext
): VerifiedECDSAPartialUserSignatureCap {
    self.inner_mut().verify_ecdsa_partial_user_signature_cap(
        cap,
        ctx,
    )
}

public fun request_ecdsa_sign_with_partial_user_signatures(
    self: &mut DWalletCoordinator,
    partial_user_signature_cap: VerifiedECDSAPartialUserSignatureCap,
    message_approval: MessageApproval,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
) {
    self.inner_mut().request_ecdsa_sign_with_partial_user_signatures(
        partial_user_signature_cap,
        message_approval,
        payment_ika,
        payment_sui,
        ctx,
    )
}

public fun compare_ecdsa_partial_user_signatures_with_approvals(
    self: &DWalletCoordinator,
    partial_user_signature_cap: &VerifiedECDSAPartialUserSignatureCap,
    message_approval: &MessageApproval,
) {
    self.inner().compare_ecdsa_partial_user_signatures_with_approvals(
        partial_user_signature_cap,
        message_approval,
    )
}

#[allow(unused_function)]
public(package) fun process_checkpoint_message_by_quorum(
    self: &mut DWalletCoordinator,
    signature: vector<u8>,
    signers_bitmap: vector<u8>,
    message: vector<u8>,
    ctx: &mut TxContext,
) {
    let self = self.inner_mut();
    self.process_checkpoint_message_by_quorum(signature, signers_bitmap, message, ctx);
}

/// Migrate the dwallet_2pc_mpc_secp256k1 object to the new package id.
///
/// This function sets the new package id and version and can be modified in future versions
/// to migrate changes in the `dwallet_2pc_mpc_secp256k1_inner` object if needed.
public fun migrate(
        self: &mut DWalletCoordinator,
) {
    assert!(self.version < VERSION, EInvalidMigration);

    // Move the old system state inner to the new version.
    let dwallet_2pc_mpc_secp256k1_inner: DWalletCoordinatorInner = dynamic_field::remove(&mut self.id, self.version);
    dynamic_field::add(&mut self.id, VERSION, dwallet_2pc_mpc_secp256k1_inner);
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
