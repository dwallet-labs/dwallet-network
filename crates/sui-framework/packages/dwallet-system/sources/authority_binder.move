// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module dwallet_system::authority_binder {
	use std::string::String;
	use dwallet::transfer;
	use dwallet::object::{Self, ID, UID};
	use dwallet::tx_context::TxContext;
	use dwallet_system::dwallet;
	use dwallet_system::dwallet::{MessageApproval};

   friend dwallet_system::ethereum_authority;

	// <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<

	const EInvalidDWalletCap: u64 = 100;

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

	public(friend) fun get_bind_to_authority(binder: &DWalletBinder): &BindToAuthority {
		&binder.bind_to_authority
	}
	
	public(friend) fun borrow_dwallet_cap(binder: &DWalletBinder): &dwallet::DWalletCap {
		&binder.dwallet_cap
	}

	/// Point to the bound authority that should enforce the login for the `dwallet::DWalletCap`.
	struct BindToAuthority has key, store {
		id: UID,
		nonce: u64,
		authority_id: ID,
		owner: vector<u8>,
		owner_type: u8,
	}

	public(friend) fun get_authority_owner_address(bind_to_authority: &BindToAuthority): vector<u8> {
		bind_to_authority.owner
	}

	struct LatestState has store {
		id: ID,
	}

	public(friend) fun create_latest_state(id:ID): LatestState {
		LatestState {
			id,
		}
	}

	#[allow(unused_field)]
	/// Represents an external authority that enforce the policy for a `dwallet::DWalletCap`.
	struct Authority<C: key + store> has key {
		id: UID,
		name: String,
		unique_identifier: vector<u8>,
		latest: LatestState,
		config: C,
		authority_owner_dwallet_cap: dwallet::DWalletCap,
		// unencrypted user share of the authority's private key - used for 2PC MPC
	}

	public(friend) fun set_latest_id<C: key + store>(authority: &mut Authority<C>, latest_id: ID) {
		authority.latest.id = latest_id;
	}

	public(friend) fun borrow_config<C: key + store>(authority: &Authority<C>): &C {
		&authority.config
	}

	public(friend) fun create_authority<C: key + store>(
		name: String,
		unique_identifier: vector<u8>,
		latest: LatestState,
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
	
	public(friend) fun create_bind_to_authority<C: key + store>(
		authority: &Authority<C>,
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

	public fun create_binder(
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

	public entry fun set_bind_to_authority<C: key + store>(
		binder: &mut DWalletBinder,
		authority: &Authority<C>,
		owner: vector<u8>,
		owner_type: u8,
	) {
		binder.bind_to_authority.nonce;
		binder.bind_to_authority.authority_id = object::id(authority);
		binder.bind_to_authority.owner = owner;
		binder.bind_to_authority.owner_type = owner_type;
		// `virgin_bound` must be false after first changing.
		binder.virgin_bound = false;
	}

	public fun create_authority_ack_transaction_hash(
		binder: &DWalletBinder,
		virgin_bound: bool,
		chain_identifier: u64,
		domain_name: vector<u8>,
		domain_version: vector<u8>,
		): vector<u8> {
			// todo(yuval): use the `chain_identifier`, name, version from Authority			
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

	public(friend) fun approve_messages<T: drop>(
		binder: &DWalletBinder,
		dwallet: &dwallet::DWallet<T>,
		messages: vector<vector<u8>>,
	): vector<MessageApproval> {
		let binder_dwallet_cap_id = object::id(&binder.dwallet_cap);
		let dwallet_cap_id = dwallet::get_dwallet_cap_id(dwallet);

		assert!(binder_dwallet_cap_id == dwallet_cap_id, EInvalidDWalletCap);

		dwallet::approve_messages(&binder.dwallet_cap, messages)
	}

	// <<<<<<<<<<<<<<<<<<<<<<<< Native functions <<<<<<<<<<<<<<<<<<<<<<<<

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

	// <<<<<<<<<<<<<<<<<<<<<<<< Native functions <<<<<<<<<<<<<<<<<<<<<<<<
}