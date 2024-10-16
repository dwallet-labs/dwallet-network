// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module dwallet_system::authority_binder {
	use std::string::String;
	use dwallet::transfer;
	use dwallet::object::{Self, ID, UID};
	use dwallet::tx_context::TxContext;
	use dwallet_system::dwallet;

	friend dwallet_system::dwallet_2pc_mpc_ecdsa_k1;

	// <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<

	// <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<

	// <<<<<<<<<<<<<<<<<<<<<<<< Constants <<<<<<<<<<<<<<<<<<<<<<<<

	// <<<<<<<<<<<<<<<<<<<<<<<< Constants <<<<<<<<<<<<<<<<<<<<<<<<

	// <<<<<<<<<<<<<<<<<<<<<<<< Events <<<<<<<<<<<<<<<<<<<<<<<<

	// <<<<<<<<<<<<<<<<<<<<<<<< Events <<<<<<<<<<<<<<<<<<<<<<<<

	#[allow(unused_field)]
	/// Bind a `dwallet::DWalletCap` to an authority.
	struct DWalletBinder has key {
		id: UID,
		dwallet_cap: dwallet::DWalletCap,
		bind_to_authority: BindToAuthority,
		virgin_bound: bool,
	}

	/// Point to the bound authority that should enforce the login for the `dwallet::DWalletCap`.
	struct BindToAuthority has key, store {
		id: UID,
		nonce: u64,
		authority_id: ID,
		owner: vector<u8>,
		owner_type: u8,
	}

	#[allow(unused_field)]
	/// Represents an external authority that enforce the policy for a `dwallet::DWalletCap`.
	struct Authority<C: key + store, L: key + store> has key {
		id: UID,
		name: String,
		unique_identifier: vector<u8>,
		latest: L,
		config: C,
		authority_owner_dwallet_cap: dwallet::DWalletCap,
	}

	/// Create an Authority object.
	/// The `latest` object represents the latest state of the authority.
	/// The `config` object represents a configuration of the authority.
	public fun create_authority<C: key + store, L: key + store>(
		name: String,
		unique_identifier: vector<u8>,
		latest: L,
		config: C,
		authority_owner_dwallet_cap: dwallet::DWalletCap,
		ctx: &mut TxContext,
	) {
		let authority = Authority {
			id: object::new(ctx),
			name,
			unique_identifier,
			latest,
			config,
			authority_owner_dwallet_cap,
		};
		transfer::share_object(authority);
	}

	fun create_bind_to_authority<C: key + store, L: key + store>(
		authority: &Authority<C, L>,
		owner: vector<u8>,
		owner_type: u8,
		ctx: &mut TxContext,
	): BindToAuthority {
		BindToAuthority {
			id: object::new(ctx),
			nonce: 0,
			authority_id: object::id(authority),
			owner,
			owner_type,
		}
	}


	/// Create a `DWalletBinder` object.
	public fun create_binder<C: key + store, L: key + store>(
		dwallet_cap: dwallet::DWalletCap,
		authority: &Authority<C, L>,
		owner: vector<u8>,
		owner_type: u8,
		virgin_bound: bool,
		ctx: &mut TxContext,
	) {
		let bind_to_authority = create_bind_to_authority(
			authority,
			owner,
			owner_type,
			ctx
			);
		let binder = DWalletBinder {
			id: object::new(ctx),
			dwallet_cap,
			bind_to_authority,
			virgin_bound,
		};
		transfer::share_object(binder);
	}

	/// Bind a new authority to an existing `DWalletBinder`.
	/// This actually changes the authority that enforces the login policy for the `dwallet::DWalletCap`.
	public entry fun set_bind_to_authority<C: key + store, L: key + store>(
		binder: &mut DWalletBinder,
		authority: &Authority<C, L>,
        owner: vector<u8>,
		owner_type: u8,
	) {
		binder.bind_to_authority.nonce ;
		binder.bind_to_authority.authority_id = object::id(authority);
		binder.bind_to_authority.owner = owner;
		binder.bind_to_authority.owner_type = owner_type;
		// `virgin_bound` must be false after first changing.
		binder.virgin_bound = false;
	}

	/// Create a transaction hash for the authority acknowledgment.
	/// This is used to acknowledge the authority that the `DWalletBinder` is bound to it.
	public fun create_authority_ack_transaction_hash(
		binder: &DWalletBinder,
		virgin_bound: bool,
		chain_identifier: u64,
		domain_name: vector<u8>,
		domain_version: vector<u8>,
		): vector<u8> {
			// let bind_to_authority_nonce = binder.bind_to_authority.nonce;
			// let contract_address = binder.bind_to_authority.owner;

			// todo(yuval): we might want to use the `chain_identifier`, name, version from Authority
			create_authority_ack_transaction(
				object::id_bytes(binder),
				object::id_bytes(&binder.dwallet_cap),
				object::id_bytes(&binder.bind_to_authority),
				binder.bind_to_authority.nonce,
				virgin_bound,
				chain_identifier,
				domain_name,
				domain_version,
				binder.bind_to_authority.owner
				)
	}

	#[allow(unused_function)]
	native fun create_authority_ack_transaction(
		binder_id: vector<u8>,
		dwallet_cap_id: vector<u8>,
		bind_to_authority: vector<u8>,
		bind_to_authority_nonce: u64,
		virgin_bound: bool,
		chain_id: u64,
		domain_name: vector<u8>,
		domain_version: vector<u8>,
		contract_address: vector<u8>
	): vector<u8>;
}