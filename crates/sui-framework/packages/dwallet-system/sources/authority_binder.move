// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module dwallet_system::authority_binder {
	use std::hash::sha2_256;
	use std::vector;
	use std::string;
	use std::string::String;
	use dwallet::transfer;
	use dwallet::object::{Self, ID, UID};
	use dwallet::tx_context::{ Self, TxContext };
	use dwallet_system::dwallet;
	use dwallet_system::dwallet::{MessageApproval};
	use dwallet::ecdsa_k1;

	// const EHashMismatch: u64 = 0;
	const EInvalidSignature: u64 = 1;
	
	friend dwallet_system::ethereum_authority;
	friend dwallet_system::dwallet_2pc_mpc_ecdsa_k1;
	// todo(yuval): change name to `sui_authority`
	friend dwallet_system::sui_state_proof;
	// <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<

	const EInvalidDWalletCap: u64 = 100;

	// <<<<<<<<<<<<<<<<<<<<<<<< Error codes <<<<<<<<<<<<<<<<<<<<<<<<

	// <<<<<<<<<<<<<<<<<<<<<<<< Constants <<<<<<<<<<<<<<<<<<<<<<<<

	// const _SMART_CONTRACT: u8 = 0;
	// const _MODULE: u8 = 1;

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
		foreign_cap_id: ID,
		with_foreign_cap_id: bool,
	}

    /// Get the `BindToAuthority` object from the `DWalletBinder`.
	public(friend) fun get_bind_to_authority(binder: &DWalletBinder): &BindToAuthority {
		&binder.bind_to_authority
	}
	
	public(friend) fun dwallet_cap_id(binder: &DWalletBinder): ID {
		object::id(&binder.dwallet_cap)
	}

	public(friend) fun get_foreign_cap_id(binder: &DWalletBinder): ID {
		binder.foreign_cap_id
	}

	public(friend) fun set_foreign_cap_id(binder: &mut DWalletBinder, foreign_cap_id: ID) {
		binder.foreign_cap_id = foreign_cap_id;
		binder.with_foreign_cap_id = true;
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

    /// Get the `Authority`'s owner address from the `BindToAuthority`.
	public(friend) fun get_authority_owner_address(bind_to_authority: &BindToAuthority): vector<u8> {
		bind_to_authority.owner
	}

    /// Pointer to the latest state object.
	struct LatestState has store {
		id: ID,
	}

    /// Create a new `LatestState` object.
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
	}

	public(friend) fun borrow_dwallet_cap<C: key + store>(authority: &Authority<C>): &dwallet::DWalletCap {
		&authority.authority_owner_dwallet_cap
	}

	 /// Get the latest state object from the `Authority`.
   /// Set the latest state object to be pointed by the `Authority`.
	public(friend) fun set_latest_id<C: key + store>(authority: &mut Authority<C>, latest_id: ID) {
		authority.latest.id = latest_id;
	}

    /// Borrow the `config` object from the `Authority`.
	public(friend) fun borrow_config<C: key + store>(authority: &Authority<C>): &C {
		&authority.config
	}

	/// Create an Authority object.
	/// The `config` object represents a configuration of the authority.
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
		transfer::transfer(bind, tx_context::sender(ctx))
}

	/// Create a `DWalletBinder` object.
	public fun create_binder(
		dwallet_cap: dwallet::DWalletCap,
		bind_to_authority: BindToAuthority,
		virgin_bound: bool,
		foreign_cap_id: ID,
		with_foreign_cap_id: bool,
		ctx: &mut TxContext,
	) {
		let binder = DWalletBinder {
			id: object::new(ctx),
			dwallet_cap,
			bind_to_authority,
			virgin_bound,
			foreign_cap_id,
			with_foreign_cap_id,
		};
		transfer::share_object(binder);
	}

	/// Bind a new authority to an existing `DWalletBinder`.
	/// This actually changes the authority that enforces the login policy for the `dwallet::DWalletCap`.
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
		domain_name: vector<u8>,
		domain_version: vector<u8>,
		chain_type: vector<u8>,
		): vector<u8> {
			// todo(yuval): update to make sui compatible
			// let current = object::id_to_bytes(&object::id_from_address(@dwallet_system));
			let chain_type_string = string::utf8(chain_type);
			if (chain_type_string == string::utf8(b"Sui")) {
				let vec_info = vector::empty();
				vector::append(&mut vec_info, object::id_bytes(binder));
				vector::append(&mut vec_info, object::id_bytes(&binder.dwallet_cap));
				vector::append(&mut vec_info, object::id_bytes(&binder.bind_to_authority));
				vector::append(&mut vec_info,  std::bcs::to_bytes(&binder.bind_to_authority.nonce));
				vector::append(&mut vec_info,  std::bcs::to_bytes(&binder.virgin_bound));
				
				return sha2_256(vec_info)
			};

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
				chain_type
				)
		}

	/// Approve messages using the `DWalletBinder`.
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

	// todo(yuval): this function should go to the sui dwallet_cap module
	public fun bind_dwallet_to_authority(
		dwallet_binder: &DWalletBinder,
		binder_id: ID,
		dwallet_cap_id: ID,
		bind_to_authority_id: ID,
		nonce: u64,
		virgin_bound: bool,
		hash: vector<u8>,
		signature: vector<u8>,
		pk: vector<u8>,
	) {
		let info_as_vec = vector::empty();
		assert!(object::id(dwallet_binder) == binder_id, 10);
		vector::append(
			&mut info_as_vec,
			object::id_to_bytes(&binder_id)
		);
		assert!(object::id(&dwallet_binder.dwallet_cap) == dwallet_cap_id, 11);
		vector::append(
			&mut info_as_vec,
			object::id_to_bytes(&dwallet_cap_id)
		);
		assert!(object::id(&dwallet_binder.bind_to_authority) == bind_to_authority_id, 12);
		vector::append(
			&mut info_as_vec,
			object::id_to_bytes(&bind_to_authority_id)
		);
		assert!(std::bcs::to_bytes(&nonce) == std::bcs::to_bytes(&dwallet_binder.bind_to_authority.nonce), 13);
		vector::append(
			&mut info_as_vec,
			std::bcs::to_bytes(&nonce)
		);
		assert!(std::bcs::to_bytes(&virgin_bound) == std::bcs::to_bytes(&dwallet_binder.virgin_bound), 14);
		vector::append(
			&mut info_as_vec,
			std::bcs::to_bytes(&virgin_bound)
		);

		let constructed_hash = sha2_256(info_as_vec);
		let constructed_hash_bcs = dwallet::bcs::to_bytes(&constructed_hash);

		// let deserialized_message = dwallet::bcs::peel_vec_u8(&mut dwallet::bcs::new(hash));
		// let deserialized_signature = dwallet::bcs::peel_vec_u8(&mut dwallet::bcs::new(signature));
		// let deserialized_pk = dwallet::bcs::peel_vec_u8(&mut dwallet::bcs::new(pk));

		let constructed_hash_len: u64 = vector::length(&constructed_hash_bcs);
		let hash_len: u64 = vector::length(&hash); // todo(yuval): change hash_* names to message_* or message_hash_*
		assert!(constructed_hash_len == hash_len, 15);

		// create a clone of the message because the comparison will pop the bytes, then it cannot be
		let hash_clone = vector<u8>[];
		vector::append(&mut hash_clone, hash);

		// compare constructed hash with the hash
		let i: u64 = 0;
		while (i < constructed_hash_len) {
			let constructed_byte = vector::pop_back(&mut constructed_hash_bcs);
			let byte = vector::pop_back(&mut hash_clone);
			assert!(byte == constructed_byte, 16);
			i = i + 1;
		};

		// The last param 1 represents the hash function used is SHA256, the default hash function used when signing in CLI.
		// let recovered = ecdsa_k1::secp256k1_verify(&deserialized_signature, &deserialized_pk, &hash, 1);
		
		// remove first byte from each vector - this is length of vector, not part of the signature
		// vector::remove(&mut signature, 0);
		// vector::remove(&mut pk, 0);
		// vector::remove(&mut hash, 0);
		
		let recovered = ecdsa_k1::secp256k1_ecrecover(&signature, &hash, 1);
		let recovered_len: u64 = vector::length(&recovered);

		let j: u64 = 0;
		while (j < recovered_len) {
			let recovered_byte = vector::pop_back(&mut recovered);
			let byte = vector::pop_back(&mut pk);
			assert!(byte == recovered_byte, 17);
			j = j + 1;
		};

		assert!(recovered == pk, EInvalidSignature);
	}

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
		// todo(yuval): update PR to have chain_type instead of chain_id_type 
		chain_type: vector<u8>,
	): vector<u8>;
}