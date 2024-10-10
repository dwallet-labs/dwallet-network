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
	struct Authority<C, L> has key {
		id: UID,
		name: String,
		unique_identifier: vector<u8>,
		latest: L,
		config: C,
		authority_owner_dwallet_cap: dwallet::DWalletCap,
	}

	public(friend) fun create_authority<C: store, L: store>(
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
	
	public(friend) fun create_bind_to_authority(
		authority_id: ID,
		owner: vector<u8>,
		owner_type: u8,
		ctx: &mut TxContext,
	): BindToAuthority {
		BindToAuthority {
			id: object::new(ctx),
			nonce: 0,
			authority_id,
			owner,
			owner_type,
		}
	}

	public(friend) fun create_binder(
		dwallet_cap: dwallet::DWalletCap,
		bind_to_authority: BindToAuthority,
		virgin_bound: bool,
		ctx: &mut TxContext,
	) {
		let binder = DWalletBinder {
			id: object::new(ctx),
			dwallet_cap,
			bind_to_authority,
			virgin_bound,
		};
		transfer::share_object(binder);
	}

	public(friend) fun set_bind_to_authority(
		binder: &mut DWalletBinder,
		authority_id: ID,
		owner: vector<u8>,
		owner_type: u8,
	) {
		binder.bind_to_authority.nonce ;
		binder.bind_to_authority.authority_id = authority_id;
		binder.bind_to_authority.owner = owner;
		binder.bind_to_authority.owner_type = owner_type;
		// `virgin_bound` must be false after first changing.
		binder.virgin_bound = false;
	}

	public entry fun create_authority_ack_transaction_hash(
		binder: &DWalletBinder,
		authority_dwallet_cap: &dwallet::DWalletCap,
		bind_to_authority: &BindToAuthority,
		virgin_bound: bool,
		chain_identifier: u64,
		domain_name: vector<u8>,
		domain_version: u64,
		contract_address: vector<u8> 
		): vector<u8> {
			create_authority_ack_transaction(
				object::id_bytes(binder),
				object::id_bytes(authority_dwallet_cap),
				object::id_bytes(bind_to_authority),
				virgin_bound,
				chain_identifier,
				domain_name,
				domain_version,
				contract_address
				)
	}

   #[allow(unused_function)]
	public native fun create_authority_ack_transaction(
		binder_id: vector<u8>, 
		dwallet_cap_id: vector<u8>, 
		bind_to_authority_id: vector<u8>, 
		virgin_bound: bool, chain_id: u64, 
		domain_name: vector<u8>, 
		domain_version: u64, 
		contract_address: vector<u8> 
	): vector<u8>;
}