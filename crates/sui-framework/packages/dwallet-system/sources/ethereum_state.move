/*
 * ethereum_state module provides functionality for managing and verifying the state of Ethereum within
 * the dwallet_system. It includes structures to represent Ethereum state, functions to initialize and update state,
 * and native functions to verify state updates and create initial state data.
*/
module dwallet_system::ethereum_state {
    use std::vector;
    use dwallet::object::{Self, UID, ID};
    use dwallet::transfer;
    use dwallet::tx_context::TxContext;

    friend dwallet_system::eth_dwallet;
    /// Ethereum state object.
    struct EthereumState has key {
        id: UID,
        /// serialised ConsensusStateManager
        data: vector<u8>,
        time_slot: u64,
        state_root: vector<u8>,
    }

    /// Latest Ethereum state object.
    /// Holds the ID of the latest Ethereum state object that is verified by the dWallet network.
    /// This object should have exactly one instance per `network`.
    struct LatestEthereumState has key, store {
        id: UID,
        eth_state_id: ID,
        last_slot: u64,
        eth_smart_contract_address: vector<u8>,
        eth_smart_contract_slot: u64,
        network: vector<u8>,
    }

    /// Initializes the first Ethereum state with the given checkpoint.
    /// Creates an EthereumState object, shares a LatestEthereumState object pointing to it,
    /// and freezes the EthereumState object.
    /// NOTE: this function performs no verification on the `checkpoint`, 
    /// and it serves as an initial "trusted" state which users should verify externally (once) before using.
    public fun init_state(
        checkpoint: vector<u8>,
        network: vector<u8>,
        eth_smart_contract_address: vector<u8>,
        eth_smart_contract_slot: u64,
        ctx: &mut TxContext
    ) {
        let state_data = create_initial_eth_state_data(checkpoint, network);
        let state = EthereumState {
            id: object::new(ctx),
            data: state_data,
            time_slot: 0u64,
            state_root: vector::empty(),
        };

        transfer::share_object(LatestEthereumState {
            id: object::new(ctx),
            eth_state_id: object::id(&state),
            last_slot: state.time_slot,
            eth_smart_contract_address,
            eth_smart_contract_slot,
            network,
        });
        transfer::freeze_object(state);
    }

    /// Verifies the new Ethereum state according to the provided updates, and updates the LatestEthereumState object
    /// if the new state is valid and has a newer time slot.
    public fun verify_new_state(
        updates_bytes: vector<u8>,
        finality_update_bytes: vector<u8>,
        optimistic_update_bytes: vector<u8>,
        state_bytes: vector<u8>,
        latest_ethereum_state: &mut LatestEthereumState,
        ctx: &mut TxContext,
    ) {
        let (data, time_slot, state_root) = verify_eth_state(
            updates_bytes,
            finality_update_bytes,
            optimistic_update_bytes,
            state_bytes
        );

        if (time_slot > latest_ethereum_state.last_slot) {
            let new_state = EthereumState {
                id: object::new(ctx),
                data,
                time_slot,
                state_root
            };

            latest_ethereum_state.eth_state_id = object::id(&new_state);
            latest_ethereum_state.last_slot = new_state.time_slot;
            transfer::freeze_object(new_state);
        };
    }

    public(friend) fun get_ethereum_state_id_from_latest(
        latest_ethereum_state: &LatestEthereumState
    ): ID {
        latest_ethereum_state.eth_state_id
    }

    public(friend) fun get_ethereum_state_network(
        latest_ethereum_state: &LatestEthereumState
    ): vector<u8> {
        latest_ethereum_state.network
    }

    public(friend) fun get_ethereum_state_root(
        state: &EthereumState
    ): vector<u8> {
        state.state_root
    }

    public(friend) fun get_contract_address(
        latest_ethereum_state: &LatestEthereumState
    ): vector<u8> {
        latest_ethereum_state.eth_smart_contract_address
    }

    public(friend) fun get_contract_approved_transactions_slot(
        latest_ethereum_state: &LatestEthereumState
    ): u64 {
        latest_ethereum_state.eth_smart_contract_slot
    }

    /// Native function.
    /// Verifies the Ethereum state according to the provided updates.
    native fun verify_eth_state(
        updates: vector<u8>,
        finality_update: vector<u8>,
        optimistic_update: vector<u8>,
        eth_state: vector<u8>
    ): (vector<u8>, u64, vector<u8>);

    /// Native function.
    /// Creates the initial Ethereum state data with the given checkpoint.
    native fun create_initial_eth_state_data(
        checkpoint: vector<u8>,
        network: vector<u8>,
    ): vector<u8>;
}