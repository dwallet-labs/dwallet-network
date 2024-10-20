// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// `ethereum_authority` module provides functionality for managing and verifying the state of Ethereum within
// the dwallet_system.
// It includes structures to represent Ethereum state, functions to initialize and update state,
// and native functions to verify state updates and create initial state data.
//

module dwallet_system::ethereum_authority {
	use std::string::String;
    use dwallet::object::{Self, UID};
    use dwallet::tx_context::TxContext;
    use dwallet_system::dwallet::{DWallet,DWalletCap, MessageApproval};
    use dwallet_system::authority_binder;
    use dwallet_system::authority_binder::{ Authority };
    use dwallet_system::authority_binder::{ create_authority };
	use dwallet_system::dwallet_2pc_mpc_ecdsa_k1::{Secp256K1};
    use dwallet::tx_context;
    use dwallet::transfer;

    const ENetworkMismatch: u64 = 100;
    const EUpdateIrrelevant: u64 = 101;
    const EInvalidStateProof: u64 = 102;

    /// State verification response.
    struct StateVerificationResponse has copy, drop {
        data: vector<u8>,
        time_slot: u64,
        network: vector<u8>,
        state_root: vector<u8>,
        block_number: u64,
    }

    /// Initial state creation response.
    struct InitStateResponse has copy, drop {
        data: vector<u8>,
        time_slot: u64,
        state_root: vector<u8>,
        block_number: u64,
    }

    /// Ethereum state object.
    struct EthereumState has key, store {
        id: UID,
        /// serialised ConsensusStateManager
        data: vector<u8>,
        time_slot: u64,
        state_root: vector<u8>,
        block_number: u64,
        network: vector<u8>,
    }

    /// Ethereum Smart Contract Config.
	struct EthereumSmartContractConfig has store, key {
		id: UID,
		approved_tx_slot: u64,
        network: vector<u8>,
	}

    /// Creates an Ethereum Smart Contract Config.
	public fun create_ethereum_smart_contract_config(approved_tx_slot: u64, network: vector<u8>, ctx: &mut TxContext) {
		let config = EthereumSmartContractConfig {
			id: object::new(ctx),
			approved_tx_slot,
            network,
		};
		transfer::transfer(config, tx_context::sender(ctx))
	}

    /// Get the approved transactions mapping slot of the Ethereum Smart Contract.
    public fun get_contract_approved_tx_slot(config: &EthereumSmartContractConfig): u64 {
        config.approved_tx_slot
    }

    /// Get the network of the Ethereum Smart Contract.
    public fun get_network(config: &EthereumSmartContractConfig): vector<u8> {
        config.network
    }

    /// Creates an `Authority<EthereumSmartContractConfig>`.
    /// Initializes the Ethereum state with the provided state bytes and updates.
    /// The state is then pointed to by the `Authority`.
    public fun create_ethereum_authority(
        name: String,
        unique_identifier: vector<u8>,
        config: EthereumSmartContractConfig,
        authority_owner_dwallet_cap: DWalletCap,
        state_bytes: vector<u8>,
        updates_vec: vector<u8>,
        finality_update: vector<u8>,
        optimistic_update: vector<u8>,
        beacon_block: vector<u8>,
        beacon_block_body: vector<u8>,
        beacon_block_execution_payload: vector<u8>,
        beacon_block_type: vector<u8>,
        ctx: &mut TxContext,
    ) {
        let initial_state = create_initial_state(
            state_bytes,
            get_network(&config),
            updates_vec,
            finality_update,
            optimistic_update,
            beacon_block,
            beacon_block_body,
            beacon_block_execution_payload,
            beacon_block_type,
            ctx,
            );

        let latest_state = authority_binder::create_latest_state(
            object::id(&initial_state)
            );

        transfer::freeze_object(initial_state);

        create_authority(
            name,
            unique_identifier,
            latest_state,
            config,
            authority_owner_dwallet_cap,
            ctx,
        );
    }

    /// Initializes the first Ethereum state with the given checkpoint.
    /// Creates an EthereumState object, and transfers it to the sender.
    /// NOTE: this function performs no verification on the `checkpoint`,
    /// and it serves as an initial "trusted" state which users should verify
    /// externally (once) before using.
    fun create_initial_state(
        state_bytes: vector<u8>,
        network: vector<u8>,
        updates_vec: vector<u8>,
        finality_update: vector<u8>,
        optimistic_update: vector<u8>,
        beacon_block: vector<u8>,
        beacon_block_body: vector<u8>,
        beacon_block_execution_payload: vector<u8>,
        beacon_block_type: vector<u8>,
        ctx: &mut TxContext
        ): EthereumState {
            let response = create_initial_eth_state_data(
                state_bytes,
                network,
                updates_vec,
                finality_update,
                optimistic_update,
                beacon_block,
                beacon_block_body,
                beacon_block_execution_payload,
                beacon_block_type
                );

            let state = EthereumState {
                id: object::new(ctx),
                data: response.data,
                time_slot: response.time_slot,
                state_root: response.state_root,
                block_number: response.block_number,
                network,
            };

            state
    }

    /// Verifies the new Ethereum state according to the provided updates,
    /// and updates the `Authority` object if the new state is valid and has a newer time slot.
    public fun update_authority_state(
        authority: &mut Authority<EthereumSmartContractConfig>,
        current_state_from_authority: &EthereumState,
        updates_bytes: vector<u8>,
        finality_update_bytes: vector<u8>,
        optimistic_update_bytes: vector<u8>,
        beacon_block: vector<u8>,
        beacon_block_body: vector<u8>,
        beacon_block_execution_payload: vector<u8>,
        beacon_block_type: vector<u8>,
        ctx: &mut TxContext,
    ) {
        // Verifies and creates the new state.
        let updated_state = update_state(
            updates_bytes,
            finality_update_bytes,
            optimistic_update_bytes,
            current_state_from_authority,
            beacon_block,
            beacon_block_body,
            beacon_block_execution_payload,
            beacon_block_type,
            ctx,
        );

        // Update state in Authority.
        set_verified_authority_state(authority, current_state_from_authority, &updated_state);
        transfer::freeze_object(updated_state);
    }

    /// Updates the Ethereum state with the provided updates.
    /// Verifies the new state and transfers it to the sender.
    fun update_state(
        updates_vec: vector<u8>,
        finality_update: vector<u8>,
        optimistic_update: vector<u8>,
        eth_state: &EthereumState,
        beacon_block: vector<u8>,
        beacon_block_body: vector<u8>,
        beacon_block_execution_payload: vector<u8>,
        beacon_block_type: vector<u8>,
        ctx: &mut TxContext
        ): EthereumState {
            let EthereumState {
                id: _,
                data,
                time_slot: _,
                state_root: _,
                block_number: _,
                network: _,
            } = eth_state;

            let response = verify_eth_state(
                updates_vec,
                finality_update,
                optimistic_update,
                *data,
                beacon_block,
                beacon_block_body,
                beacon_block_execution_payload,
                beacon_block_type
                );

            let new_state = EthereumState {
                id: object::new(ctx),
                data: response.data,
                time_slot: response.time_slot,
                state_root: response.state_root,
                block_number: response.block_number,
                network: response.network,
            };

            new_state
        }

    /// Sets the latest state in the `Authority` object if the new state is
    /// valid and has a newer time slot.
    fun set_verified_authority_state(
        authority: &mut Authority<EthereumSmartContractConfig>,
        current_state: &EthereumState,
        updated_state: &EthereumState,
    ) {
        assert!(current_state.time_slot <= updated_state.time_slot, EUpdateIrrelevant);
        assert!(current_state.network == updated_state.network, ENetworkMismatch);

        if (current_state.time_slot == updated_state.time_slot) {
            return
        };

        authority_binder::set_latest_id(authority, object::id(updated_state));
    }

    /// Verifies the provided proof using the `verify_message_proof` native function.
    /// If the proof is valid, the message is approved.
    public fun approve_message(
        authority: &Authority<EthereumSmartContractConfig>,
        binder: &authority_binder::DWalletBinder,
        state: &EthereumState,
        message: vector<u8>,
        dwallet: &DWallet<Secp256K1>,
        proof: vector<u8>,
    ): vector<MessageApproval> {
        let bind_to_authority = authority_binder::borrow_bind_to_authority(binder);
        let contract_address = authority_binder::get_authority_owner_address(bind_to_authority);
        let config = authority_binder::borrow_config(authority);

        let is_valid = verify_message_proof(
            proof,
            message,
            object::id_to_bytes(&object::id(dwallet)),
            config.approved_tx_slot,
            contract_address,
            state.state_root,
        );
        assert!(is_valid, EInvalidStateProof);

        authority_binder::approve_messages(binder, dwallet, vector[message])
    }

    /// Verify the Message inside the Storage Merkle Root.
    native fun verify_message_proof(
        proof: vector<u8>,
        message: vector<u8>,
        dwallet_id: vector<u8>,
        data_slot: u64,
        contract_address: vector<u8>,
        eth_state_state_root: vector<u8>,
    ): bool;

    /// Native function.
    /// Verifies the Ethereum state according to the provided updates.
    native fun verify_eth_state(
        updates: vector<u8>,
        finality_update: vector<u8>,
        optimistic_update: vector<u8>,
        eth_state: vector<u8>,
        beacon_block: vector<u8>,
        beacon_block_body: vector<u8>,
        beacon_block_execution_payload: vector<u8>,
        beacon_block_type: vector<u8>,
    ): StateVerificationResponse;

    /// Native function.
    /// Creates the initial Ethereum state data with the given checkpoint.
    native fun create_initial_eth_state_data(
        state_bytes: vector<u8>,
        network: vector<u8>,
        updates_vec_arg: vector<u8>,
        finality_update_arg: vector<u8>,
        optimistic_update_arg: vector<u8>,
        beacon_block: vector<u8>,
        beacon_block_body: vector<u8>,
        beacon_block_execution_payload: vector<u8>,
        beacon_block_type: vector<u8>,
    ): InitStateResponse;
}