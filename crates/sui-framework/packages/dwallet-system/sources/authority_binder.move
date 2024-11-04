// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module dwallet_system::authority_binder {
	use std::string::String;
	use dwallet::transfer;
	use dwallet::object::{Self, ID, UID};
	use dwallet::tx_context::{ Self, TxContext };
	use dwallet_system::dwallet;
	use dwallet_system::dwallet::{MessageApproval, PublicUserShare};
   friend dwallet_system::ethereum_authority;

	// <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<

	const EInvalidDWalletCap: u64 = 100;

	// <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<
	
	// <<<<<<<<<<<<<<<<<<<<<<<< Constants <<<<<<<<<<<<<<<<<<<<<<<<

    #[allow(unused_const)]
    const SMART_CONTRACT: u8 = 0;
    #[allow(unused_const)]
    const MODULE: u8 = 1;

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
		// 0 = Smart Contract, 1 = Module.
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
		public_user_share_obj_id: ID,
	}
	
	public fun set_authority_public_user_share_obj_id<C: key + store, T: drop>(
		authority: &mut Authority<C>,
		dwallet: &dwallet::DWallet<T>,
		public_user_share_obj_id: &PublicUserShare
		) {
			assert!(
				object::id(&authority.authority_owner_dwallet_cap) == dwallet::get_dwallet_cap_id(dwallet),
				EInvalidDWalletCap
				);
			assert!(
				object::id(dwallet) == dwallet::get_dwallet_id_from_public_user_share(public_user_share_obj_id),
				EInvalidDWalletCap
				);

			authority.public_user_share_obj_id = object::id(public_user_share_obj_id);
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
		public_user_share: &PublicUserShare,
		ctx: &mut TxContext,
	) {
		let authority = Authority {
			id: object::new(ctx),
			name,
			unique_identifier,
			latest,
			config,
			authority_owner_dwallet_cap,
			public_user_share_obj_id: object::id(public_user_share),
		};
		transfer::share_object(authority);
	}
	
	public fun create_bind_to_authority<C: key + store>(
		authority: &Authority<C>,
		owner: vector<u8>,
		owner_type: u8,
		ctx: &mut TxContext,
	) {
		let bind = BindToAuthority {
			id: object::new(ctx),
			nonce: 0,
			authority_id: object::id(authority),
			owner,
			owner_type,
		};
		transfer::transfer(bind, tx_context::sender(ctx));
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

	/// Create a transaction hash for the authority acknowledgment.
	/// This is used to acknowledge the authority that the `DWalletBinder` is bound to it.
	public fun create_authority_ack_transaction_hash(
		binder: &DWalletBinder,
		virgin_bound: bool,
		chain_identifier: vector<u8>,
		// 0 = Number, 1 = Hex String
		chain_id_type: u8,
		domain_name: vector<u8>,
		domain_version: vector<u8>,
		): vector<u8> {
			create_authority_ack_transaction(
					object::id_bytes(binder),
					object::id_bytes(&binder.dwallet_cap),
					object::id_bytes(&binder.bind_to_authority),
					binder.bind_to_authority.nonce,
					virgin_bound,
					chain_identifier,
					domain_name,
					domain_version,
					binder.bind_to_authority.owner,
					chain_id_type,
					)
	}

	public entry fun approve_messages<C: key + store, T: drop>(
		authority: &Authority<C>,
		dwallet: &dwallet::DWallet<T>,
		messages: vector<vector<u8>>,
	): vector<MessageApproval> {
		let binder_dwallet_cap_id = object::id(&authority.authority_owner_dwallet_cap);
		let dwallet_cap_id = dwallet::get_dwallet_cap_id(dwallet);

		assert!(binder_dwallet_cap_id == dwallet_cap_id, EInvalidDWalletCap);

		dwallet::approve_messages(&authority.authority_owner_dwallet_cap, messages)
	}

	// <<<<<<<<<<<<<<<<<<<<<<<< Native functions <<<<<<<<<<<<<<<<<<<<<<<<

	#[allow(unused_function)]
    /// Create a transaction hash that will be signed later, to acknowledge the
    /// authority that the `DWalletBinder` is bound to it.
	native fun create_authority_ack_transaction(
		binder_id: vector<u8>,
		dwallet_cap_id: vector<u8>,
		bind_to_authority: vector<u8>,
		bind_to_authority_nonce: u64,
		virgin_bound: bool,
		chain_id: vector<u8>,
		domain_name: vector<u8>,
		domain_version: vector<u8>,
		contract_address: vector<u8>,
		chain_id_type: u8,
	): vector<u8>;

	// <<<<<<<<<<<<<<<<<<<<<<<< Native functions <<<<<<<<<<<<<<<<<<<<<<<<
}