module ika_system::dwallet_2pc_mpc_secp256k1;

use ika::ika::IKA;
use sui::sui::SUI;
use sui::coin::{Coin};
use sui::dynamic_field;
use ika_system::dwallet_pricing::{DWalletPricing2PcMpcSecp256K1};
use ika_system::dwallet_2pc_mpc_secp256k1_inner::{
    Self,
    DWallet2PcMpcSecp256K1InnerV1,
    DWalletNetworkrkDecryptionKeyCap,
    EncryptionKey,
    DWalletCap,
    MessageApproval,
    UnverifiedECDSAPartialUserSignatureCap,
    VerifiedECDSAPartialUserSignatureCap
};


public struct DWallet2PcMpcSecp256K1 has key {
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
public(package) fun create(
    package_id: ID,
    epoch: u64,
    pricing: DWalletPricing2PcMpcSecp256K1,
    ctx: &mut TxContext
): (ID, DWalletNetworkrkDecryptionKeyCap) {
    let mut dwallet_2pc_mpc_secp256k1 = dwallet_2pc_mpc_secp256k1_inner::create(
        epoch,
        pricing,
        ctx,
    );
        // TODO: remove this code!
    let cap = dwallet_2pc_mpc_secp256k1.create_dwallet_network_decryption_key(ctx);
    let mut self = DWallet2PcMpcSecp256K1 {
        id: object::new(ctx),
        version: VERSION,
        package_id,
        new_package_id: option::none(),
    };
    let self_id = object::id(&self);
    dynamic_field::add(&mut self.id, VERSION, dwallet_2pc_mpc_secp256k1);
    transfer::share_object(self);
    (self_id, cap)
}

public fun get_active_encryption_key(
    self: &DWallet2PcMpcSecp256K1,
    address: address,
): &EncryptionKey {
    self.inner().get_active_encryption_key(address)
}

public fun register_encryption_key(
    self: &mut DWallet2PcMpcSecp256K1,
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

public fun request_dkg_first_round(
    self: &mut DWallet2PcMpcSecp256K1,
    dwallet_network_decryption_key_id: ID,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
): DWalletCap {
    self.inner_mut().request_dkg_first_round(
        dwallet_network_decryption_key_id,
        payment_ika,
        payment_sui,
        ctx
    )
}

public fun request_dkg_second_round(
    self: &mut DWallet2PcMpcSecp256K1,
    dwallet_cap: &DWalletCap,
    centralized_public_key_share_and_proof: vector<u8>,
    encrypted_centralized_secret_share_and_proof: vector<u8>,
    encryption_key_address: address,
    user_public_output: vector<u8>,
    singer_public_key: vector<u8>,
    ctx: &mut TxContext
) {
    self.inner_mut().request_dkg_second_round(
        dwallet_cap,
        centralized_public_key_share_and_proof,
        encrypted_centralized_secret_share_and_proof,
        encryption_key_address,
        user_public_output,
        singer_public_key,
        ctx
    )
}

public fun request_re_encrypt_user_share_for(
    self: &mut DWallet2PcMpcSecp256K1,
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
    self: &mut DWallet2PcMpcSecp256K1,
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
    self: &mut DWallet2PcMpcSecp256K1,
    dwallet_id: ID,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
) {
    self.inner_mut().request_ecdsa_presign(
        dwallet_id,
        payment_ika,
        payment_sui,
        ctx,
    )
}

public fun request_ecdsa_sign(
    self: &mut DWallet2PcMpcSecp256K1,
    dwallet_id: ID,
    message_approval: MessageApproval,
    presign_id: ID,
    message_centralized_signature: vector<u8>,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
) {
    self.inner_mut().request_ecdsa_sign(
        dwallet_id,
        message_approval,
        presign_id,
        message_centralized_signature,
        payment_ika,
        payment_sui,
        ctx
    )
}

public fun request_ecdsa_future_sign(
    self: &mut DWallet2PcMpcSecp256K1,
    dwallet_id: ID,
    message: vector<u8>,
    presign_id: ID,
    hash_scheme: u8,
    message_centralized_signature: vector<u8>,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
): UnverifiedECDSAPartialUserSignatureCap {
    self.inner_mut().request_ecdsa_future_sign(
        dwallet_id,
        message,
        presign_id,
        hash_scheme,
        message_centralized_signature,
        payment_ika,
        payment_sui,
        ctx,
    )
}

public fun verifiy_ecdsa_partial_user_signature_cap(
    self: &mut DWallet2PcMpcSecp256K1,
    cap: UnverifiedECDSAPartialUserSignatureCap,
    ctx: &mut TxContext
): VerifiedECDSAPartialUserSignatureCap {
    self.inner_mut().verifiy_ecdsa_partial_user_signature_cap(
        cap,
        ctx,
    )
}

public fun request_ecdsa_sign_with_partial_user_signatures(
    self: &mut DWallet2PcMpcSecp256K1,
    dwallet_id: ID,
    partial_user_signature_cap: VerifiedECDSAPartialUserSignatureCap,
    message_approval: MessageApproval,
    payment_ika: &mut Coin<IKA>,
    payment_sui: &mut Coin<SUI>,
    ctx: &mut TxContext
) {
    self.inner_mut().request_ecdsa_sign_with_partial_user_signatures(
        dwallet_id,
        partial_user_signature_cap,
        message_approval,
        payment_ika,
        payment_sui,
        ctx,
    )
}

public fun compare_ecdsa_partial_user_signatures_with_approvals(
    self: &DWallet2PcMpcSecp256K1,
    partial_user_signature_cap: &VerifiedECDSAPartialUserSignatureCap,
    message_approval: &MessageApproval,
) {
    self.inner().compare_ecdsa_partial_user_signatures_with_approvals(
        partial_user_signature_cap,
        message_approval,
    )
}

public(package) fun process_checkpoint_message_by_quorum(
    self: &mut DWallet2PcMpcSecp256K1,
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
        self: &mut DWallet2PcMpcSecp256K1,
) {
    assert!(self.version < VERSION, EInvalidMigration);

    // Move the old system state inner to the new version.
    let dwallet_2pc_mpc_secp256k1_inner: DWallet2PcMpcSecp256K1InnerV1 = dynamic_field::remove(&mut self.id, self.version);
    dynamic_field::add(&mut self.id, VERSION, dwallet_2pc_mpc_secp256k1_inner);
    self.version = VERSION;

    // Set the new package id.
    assert!(self.new_package_id.is_some(), EInvalidMigration);
    self.package_id = self.new_package_id.extract();
}

// === Internals ===

/// Get a mutable reference to `DWallet2PcMpcSecp256K1InnerVX` from the `DWallet2PcMpcSecp256K1`.
fun inner_mut(self: &mut DWallet2PcMpcSecp256K1): &mut DWallet2PcMpcSecp256K1InnerV1 {
    assert!(self.version == VERSION, EWrongInnerVersion);
    dynamic_field::borrow_mut(&mut self.id, VERSION)
}

/// Get an immutable reference to `DWallet2PcMpcSecp256K1VX` from the `DWallet2PcMpcSecp256K1`.
fun inner(self: &DWallet2PcMpcSecp256K1): &DWallet2PcMpcSecp256K1InnerV1 {
    assert!(self.version == VERSION, EWrongInnerVersion);
    dynamic_field::borrow(&self.id, VERSION)
}
