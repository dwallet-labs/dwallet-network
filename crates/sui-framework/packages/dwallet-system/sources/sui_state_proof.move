/// The `sui_state_proof` module provides mechanisms to verify and link
/// dWallet capabilities (`DWalletCap`) within to the Sui network.
/// It includes structures and functions for state proofs, committee
/// verification, and transaction-based proofing in a trustless manner
/// using Sui's Light Client data.

#[allow(unused_field)]
module dwallet_system::sui_state_proof {

    use dwallet::object::{Self, ID, UID};
    use dwallet_system::dwallet::{Self, DWalletCap, MessageApproval};
    use dwallet::tx_context::{TxContext};
    use dwallet::transfer;
    use dwallet::bcs;
    use dwallet::event;
    use std::vector;
    use std::debug;

    /// Error code for incorrect epoch submission.
    const EWrongEpochSubmitted: u64 = 0;

    /// Error code for mismatched DWalletCap ID.
    const EWrongDWalletCapId: u64 = 1;

    /// Error code for incorrect number of DWalletCaps.
    const EWrongAmountOfDWalletCaps: u64 = 2;


    /// Struct that keeps track of the highest verified epoch for a state proof.
    struct StateProofRegistry has key, store {
        /// Unique identifier for the registry.
        id: UID,

        /// Highest verified epoch in the registry.
        highest_epoch: u64,
    }

    /// Configuration for state proof validation, containing references to event type layouts
    /// and IDs needed for transaction-based proofs.
    struct StateProofConfig has key, store {
        /// Unique identifier for the config.
        id: UID,

        /// ID linking this config to the registry.
        registry_id: ID,

        /// ID of the package on the Sui network.
        package_id: vector<u8>,

        /// Layout for init cap event type.
        init_cap_event_type_layout: vector<u8>,

        /// Layout for approval event type.
        approve_event_type_layout: vector<u8>,
    }

    /// Holds information about a specific epoch committee.
    struct EpochCommittee has key, store {
        /// Unique identifier for the committee.
        id: UID,

        /// Serialized data for the committee.
        committee: vector<u8>,
    }

    /// Event emitted when a new committee is submitted and verified.
    struct EpochCommitteeSubmitted has copy, drop {
        /// Epoch number for the submission.
        epoch: u64,

        /// ID linking to the StateProofRegistry.
        registry_id: ID,

        /// ID of the newly submitted committee.
        epoch_committee_id: ID,
    }

    /// A wrapper for `DWalletCap` that links it with an associated Sui capability ID.
    struct CapWrapper has key, store {
        /// Unique identifier for the CapWrapper.
        id: UID,

        /// ID of the Sui capability.
        cap_id_sui: ID,

        /// The dWallet capability itself.
        cap: DWalletCap,
    }

    /// Native function for verifying the committee state proof based on a previous committee.
    native fun sui_state_proof_verify_committee(
        prev_committee: vector<u8>,
        checkpoint_summary: vector<u8>
    ): (vector<u8>, u64);

    /// Native function for linking a Sui capability to a dWallet capability,
    /// based on the committee and event type layout.
    native fun sui_state_proof_verify_link_cap(
        committee: vector<u8>,
        checkpoint_summary: vector<u8>,
        checkpoint_contents: vector<u8>,
        transaction: vector<u8>,
        event_type_layout: vector<u8>,
        package_id: vector<u8>
    ): (vector<u8>, vector<u8>);

    /// Native function for verifying a transaction as proof for dWallet
    /// state, based on committee and layout configuration.
    native fun sui_state_proof_verify_transaction(
        committee: vector<u8>,
        checkpoint_summary: vector<u8>,
        checkpoint_contents: vector<u8>,
        transaction: vector<u8>,
        event_type_layout: vector<u8>,
        package_id: vector<u8>
    ): (vector<u8>, vector<u8>);

    /// Initializes the module by creating a new `StateProofRegistry` and `StateProofConfig`,
    /// setting the initial committee for state verification. Emits an event to record the
    /// initial committee state, freezes necessary objects, and sets up the module context.
    ///
    /// # Parameters
    /// - `init_committee`: Initial committee data as a vector of bytes.
    /// - `package_id`: Unique package identifier.
    /// - `init_cap_event_type_layout`: Type layout for initial capability event.
    /// - `approve_event_type_layout`: Type layout for approval event.
    /// - `epoch_id_committee`: Identifier for the initial epoch.
    /// - `ctx`: Transaction context to create new objects.
    public fun init_module(
        init_committee: vector<u8>,
        package_id: vector<u8>,
        init_cap_event_type_layout: vector<u8>,
        approve_event_type_layout: vector<u8>,
        epoch_id_committee: u64,
        ctx: &mut TxContext) {
        let registry = StateProofRegistry {
            id: object::new(ctx),
            highest_epoch: epoch_id_committee,
        };

        let config = StateProofConfig {
            id: object::new(ctx),
            registry_id: object::id(&registry),
            package_id,
            init_cap_event_type_layout,
            approve_event_type_layout,
        };

        let first_committee = EpochCommittee {
            id: object::new(ctx),
            committee: init_committee,
        };

        event::emit(EpochCommitteeSubmitted {
            epoch: epoch_id_committee,
            registry_id: object::uid_to_inner(&registry.id),
            epoch_committee_id: object::id(&first_committee),
        });
        debug::print(&epoch_id_committee);

        transfer::share_object(registry);
        transfer::freeze_object(config);
        transfer::freeze_object(first_committee);
    }

    /// Submits a new state committee by verifying the previous committee data and incrementing
    /// the epoch in `StateProofRegistry`. Emits an event to record the updated committee status.
    ///
    /// # Parameters
    /// - `registry`: Mutable reference to `StateProofRegistry`.
    /// - `prev_committee`: Previous `EpochCommittee` to verify.
    /// - `new_checkpoint_summary`: Summary data of the new checkpoint.
    /// - `ctx`: Transaction context to create a new committee object.
    public fun submit_new_state_committee(
        registry: &mut StateProofRegistry,
        prev_committee: &EpochCommittee,
        new_checkpoint_summary: vector<u8>,
        ctx: &mut TxContext,
    ) {
        let (new_committee_verified_bytes, current_committee_epoch) = sui_state_proof_verify_committee(
            prev_committee.committee,
            new_checkpoint_summary
        );

        assert!(current_committee_epoch - 1 == registry.highest_epoch, EWrongEpochSubmitted);

        let committee_new = EpochCommittee {
            id: object::new(ctx),
            committee: new_committee_verified_bytes,
        };

        registry.highest_epoch = registry.highest_epoch + 1;

        event::emit(EpochCommitteeSubmitted {
            epoch: registry.highest_epoch,
            registry_id: object::uid_to_inner(&registry.id),
            epoch_committee_id: object::id(&committee_new),
        });

        transfer::freeze_object(committee_new);
    }

    /// Creates a `CapWrapper` that links a `DWalletCap` to a Sui capability, verifying the linkage
    /// through the current committee data and checkpoint summaries. Ensures the correct cap ID is used.
    ///
    /// # Parameters
    /// - `config`: Configuration data from `StateProofConfig`.
    /// - `dwallet_cap`: The `DWalletCap` to wrap.
    /// - `committee`: Reference to the current `EpochCommittee`.
    /// - `checkpoint_summary`: Summary data for the checkpoint.
    /// - `checkpoint_contents`: Contents of the checkpoint.
    /// - `transaction`: Transaction data associated with the checkpoint.
    /// - `ctx`: Transaction context for object creation.
    public fun create_dwallet_wrapper(
        config: &StateProofConfig,
        dwallet_cap: DWalletCap,
        committee: &EpochCommittee,
        checkpoint_summary: vector<u8>,
        checkpoint_contents: vector<u8>,
        transaction: vector<u8>,
        ctx: &mut TxContext
    ) {
        let (sui_cap_ids_bytes, dwallet_cap_ids_bytes) = sui_state_proof_verify_link_cap(
            committee.committee,
            checkpoint_summary,
            checkpoint_contents,
            transaction,
            config.init_cap_event_type_layout,
            config.package_id
        );

        // Check if the cap ID used on SUI is the same as the ID of `dwallet_cap`.
        let sui_cap_id_address_vec = bcs::peel_vec_address(&mut bcs::new(sui_cap_ids_bytes));
        let dwallet_cap_id_address_vec = bcs::peel_vec_address(&mut bcs::new(dwallet_cap_ids_bytes));

        let dwallet_cap_id = object::id(&dwallet_cap);
        let (is_valid, idx) = vector::index_of(&dwallet_cap_id_address_vec, &object::id_to_address(&dwallet_cap_id));
        assert!(is_valid, EWrongDWalletCapId);

        let sui_cap_id_address = vector::remove(&mut sui_cap_id_address_vec, idx);

        let wrapper = CapWrapper {
            id: object::new(ctx),
            cap_id_sui: object::id_from_address(sui_cap_id_address),
            cap: dwallet_cap,
        };

        transfer::share_object(wrapper);
    }

    /// Verifies a transaction against the current committee and approved event type layout,
    /// ensuring that each `DWalletCap` message receives an approval through a sequence of verifications.
    /// Returns a vector of approved messages for each `DWalletCap`.
    ///
    /// # Parameters
    /// - `config`: Configuration data from `StateProofConfig`.
    /// - `cap_wrapper`: Reference to `CapWrapper` containing a wrapped `DWalletCap`.
    /// - `committee`: Current committee data for verification.
    /// - `checkpoint_summary`: Summary of the checkpoint to verify against.
    /// - `checkpoint_contents`: Contents of the checkpoint.
    /// - `transaction`: Transaction details to be verified.
    ///
    /// # Returns
    /// - A vector of vectors, each containing `MessageApproval` objects representing approved messages.
    public fun transaction_state_proof(
        config: &StateProofConfig,
        cap_wrapper: &CapWrapper,
        committee: &EpochCommittee,
        checkpoint_summary: vector<u8>,
        checkpoint_contents: vector<u8>,
        transaction: vector<u8>,
    ): vector<vector<MessageApproval>> {
        let (cap_ids_serialised_bytes, messages_serialised_bytes) = sui_state_proof_verify_transaction(
            committee.committee,
            checkpoint_summary,
            checkpoint_contents,
            transaction,
            config.approve_event_type_layout,
            config.package_id
        );

        let cap_ids = bcs::peel_vec_address(&mut bcs::new(cap_ids_serialised_bytes));

        // Basically this is peel_vec_vec_vec<u8>.
        let bcs = bcs::new(messages_serialised_bytes);
        let len = bcs::peel_vec_length(&mut bcs);
        let i = 0;
        let messages = vector::empty<vector<vector<u8>>>();
        while (i < len) {
            vector::push_back(&mut messages, bcs::peel_vec_vec_u8(&mut bcs));
            i = i + 1;
        };

        assert!(vector::length(&cap_ids) == vector::length(&messages), EWrongAmountOfDWalletCaps);

        let result = vector::empty<vector<MessageApproval>>();
        let i = 0;
        while (i < vector::length(&cap_ids)) {
            let cap_id_address = *vector::borrow(&cap_ids, i);
            if (object::id_from_address(cap_id_address) == cap_wrapper.cap_id_sui) {
                let messages_to_approve = *vector::borrow(&messages, i);
                let approvals = dwallet::approve_messages(&cap_wrapper.cap, messages_to_approve);
                vector::push_back(&mut result, approvals);
            };
            i = i + 1;
        };
        result
    }
}
