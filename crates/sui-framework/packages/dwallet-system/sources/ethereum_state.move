//
// `ethereum_state` module provides functionality for managing and verifying the state of Ethereum within
// the dwallet_system.
// It includes structures to represent Ethereum state, functions to initialize and update state,
// and native functions to verify state updates and create initial state data.
//

module dwallet_system::ethereum_state {
    use dwallet::object::{Self, ID, UID};
    use dwallet::transfer;
    use dwallet::tx_context::TxContext;

    friend dwallet_system::eth_dwallet;

    const EStateObjectMismatch: u64 = 1;
    const ENetworkMismatch: u64 = 2;
    const EUpdateIrrelevant: u64 = 3;

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
    struct EthereumState has key {
        id: UID,
        /// serialised ConsensusStateManager
        data: vector<u8>,
        time_slot: u64,
        latest_ethereum_state_id: ID,
        state_root: vector<u8>,
        block_number: u64,
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
    /// Creates an EthereumState object, shares a `LatestEthereumState` object pointing to it,
    /// and freezes the EthereumState object.
    /// NOTE: this function performs no verification on the `checkpoint`,
    /// and it serves as an initial "trusted" state which users should verify externally (once) before using.
    public fun init_state(
        state_bytes: vector<u8>,
        network: vector<u8>,
        eth_smart_contract_address: vector<u8>,
        eth_smart_contract_slot: u64,
        updates_vec_arg: vector<u8>,
        finality_update_arg: vector<u8>,
        optimistic_update_arg: vector<u8>,
        beacon_block: vector<u8>,
        beacon_block_body: vector<u8>,
        beacon_block_execution_payload: vector<u8>,
        beacon_block_type: vector<u8>,
        ctx: &mut TxContext
    ) {
        let response = create_initial_eth_state_data(state_bytes, network, updates_vec_arg, finality_update_arg, optimistic_update_arg, beacon_block, beacon_block_body, beacon_block_execution_payload, beacon_block_type);
        let state = EthereumState {
            id: object::new(ctx),
            data: response.data,
            time_slot: response.time_slot,
            latest_ethereum_state_id: object::id_from_address(@0x0),
            state_root: response.state_root,
            block_number: response.block_number,
        };

        let latest_ethereum_state = LatestEthereumState {
            id: object::new(ctx),
            eth_state_id: object::id(&state),
            last_slot: state.time_slot,
            eth_smart_contract_address,
            eth_smart_contract_slot,
            network,
        };

        state.latest_ethereum_state_id = object::uid_to_inner(&latest_ethereum_state.id);
        transfer::freeze_object(state);
        transfer::share_object(latest_ethereum_state);
    }

    /// Verifies the new Ethereum state according to the provided updates,
    /// and updates the `LatestEthereumState` object if the new state is valid and has a newer time slot.
    public fun verify_new_state(
        updates_bytes: vector<u8>,
        finality_update_bytes: vector<u8>,
        optimistic_update_bytes: vector<u8>,
        latest_ethereum_state: &mut LatestEthereumState,
        eth_state: &EthereumState,
        beacon_block: vector<u8>,
        beacon_block_body: vector<u8>,
        beacon_block_execution_payload: vector<u8>,
        beacon_block_type: vector<u8>,
        ctx: &mut TxContext,
    ) {
        // Verify that the state is the latest state
        let eth_state_id = object::id(eth_state);
        assert!(eth_state_id == latest_ethereum_state.eth_state_id, EStateObjectMismatch);

        // Verify that the state is based on the latest state
        let latest_ethereum_state_id = get_ethereum_state_latest_state_id(eth_state);
        assert!(latest_ethereum_state_id == object::id(latest_ethereum_state), EStateObjectMismatch);

        let eth_state_bytes = get_ethereum_state_data(eth_state);
        let response = verify_eth_state(
            updates_bytes,
            finality_update_bytes,
            optimistic_update_bytes,
            eth_state_bytes,
            beacon_block,
            beacon_block_body,
            beacon_block_execution_payload,
            beacon_block_type,
        );

        assert!(response.network == latest_ethereum_state.network, ENetworkMismatch);
        assert!(response.time_slot >= latest_ethereum_state.last_slot, EUpdateIrrelevant);

        if(response.time_slot == latest_ethereum_state.last_slot) {
            return
        };

        let new_state = EthereumState {
            id: object::new(ctx),
            data: response.data,
            time_slot: response.time_slot,
            latest_ethereum_state_id: object::id(latest_ethereum_state),
            state_root: response.state_root,
            block_number: response.block_number,
        };

        latest_ethereum_state.eth_state_id = object::id(&new_state);
        latest_ethereum_state.last_slot = new_state.time_slot;
        transfer::freeze_object(new_state);
    }

    public(friend) fun get_ethereum_state_id_from_latest(
        latest_ethereum_state: &LatestEthereumState
    ): ID {
        latest_ethereum_state.eth_state_id
    }

    public(friend) fun get_ethereum_state_state_root(
        state: &EthereumState
    ): vector<u8> {
        state.state_root
    }

    public(friend) fun get_latest_ethereum_state_network(
        latest_ethereum_state: &LatestEthereumState
    ): vector<u8> {
        latest_ethereum_state.network
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

    public(friend) fun get_ethereum_state_latest_state_id(
        state: &EthereumState
    ): ID {
        state.latest_ethereum_state_id
    }

    public(friend) fun get_ethereum_state_data(
        state: &EthereumState
    ): vector<u8> {
        state.data
    }

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
