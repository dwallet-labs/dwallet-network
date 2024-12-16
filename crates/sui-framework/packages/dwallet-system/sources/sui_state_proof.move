#[allow(unused_field)]
module dwallet_system::sui_state_proof {
    use std::hash::sha2_256;
    use std::string::String;
    use dwallet::object::{Self, ID, UID};
    use dwallet_system::dwallet::{
        Self,
        DWallet,
        DWalletCap,
        MessageApproval
    };

    use dwallet_system::dwallet_2pc_mpc_ecdsa_k1::{ Secp256K1 };
    use dwallet_system::authority_binder;
    use dwallet_system::authority_binder::{
        Authority,
        DWalletBinder,
        create_authority,
        create_latest_state,
        get_foreign_cap_id,
    };

    use dwallet::tx_context::{ TxContext };
    use dwallet::transfer;
    use dwallet::bcs;
    use dwallet::event;
    use std::vector;

    const EWrongEpochSubmitted: u64 = 0;
    const EWrongDWalletCapId: u64 = 1;
    // const EStateProofNoMessagesToApprove: u64 = 2;
    const EWrongAmountOfDWalletCaps: u64 = 3;

    /// Represents a registry for state proofs.
    struct SuiStateProofRegistry has key, store {
        id: UID,
        highest_epoch: u64,
    }

    /// Holds the configuration details for state proofs.
    struct SuiStateProofConfig has key, store {
        id: UID,
        registry_id: ID,
        /// The package ID of the DWallet in the Sui network.
        dwallet_package_id_in_sui_network: vector<u8>,
        /// Layout of the event type for initializing capability.
        init_cap_event_type_layout: vector<u8>,
        /// Layout of the event type for approving messages.
        approve_event_type_layout: vector<u8>,
    }

    /// Represents a committee for a specific epoch.
    struct EpochCommittee has key, store {
        id: UID,
        /// Serialized committee information.
        committee: vector<u8>,
    }

    /// Structure representing the submission of an epoch committee, including the epoch number and registry identifiers.
    struct EpochCommitteeSubmitted has copy, drop {
        epoch: u64,
        registry_id: ID,
        epoch_committee_id: ID,
    }

    /// A wrapper structure to hold dWallet linking information.
    struct CapWrapper has key, store {
        id: UID,
        cap_id_sui: ID,
        cap: DWalletCap,
    }

    native fun sui_state_proof_verify_committee(
        prev_committee: vector<u8>,
        checkpoint_summary: vector<u8>
    ): (vector<u8>, u64);

    native fun sui_state_proof_verify_link_cap(
        committee: vector<u8>,
        checkpoint_summary: vector<u8>,
        checkpoint_contents: vector<u8>,
        transaction: vector<u8>,
        event_type_layout: vector<u8>,
        package_id: vector<u8>
    ): (vector<u8>, vector<u8>);

    native fun sui_state_proof_verify_transaction(
        committee: vector<u8>,
        checkpoint_summary: vector<u8>,
        checkpoint_contents: vector<u8>,
        transaction: vector<u8>,
        event_type_layout: vector<u8>,
        package_id: vector<u8>
    ): (vector<u8>, vector<u8>);

    /// Initializes a new authority with state proof capabilities.
    ///
    /// This function creates a new SuiStateProofRegistry, SuiStateProofConfig, and EpochCommittee. It sets up the initial state
    /// of the authority by registering the initial committee and configuring the state proof parameters.
    /// The function emits an event to record the first committee submission, shares the registry, and freezes the committee
    /// object to prevent further modifications.
    /// Finally, it creates the authority using the provided configuration.
    public fun create_sui_authority(
        init_committee: vector<u8>,
        dwallet_package_id_in_sui_network: vector<u8>,
        init_cap_event_type_layout: vector<u8>,
        approve_event_type_layout: vector<u8>,
        epoch_id_committee: u64,
        authority_owner_dwallet_cap: dwallet::DWalletCap,
        authority_name: String,
        authority_unique_identifier: vector<u8>,
        ctx: &mut TxContext
    ) {
        let registry = SuiStateProofRegistry {
            id: object::new(ctx),
            highest_epoch: epoch_id_committee,
        };

        let config = SuiStateProofConfig {
            id: object::new(ctx),
            registry_id: object::id(&registry),
            dwallet_package_id_in_sui_network: dwallet_package_id_in_sui_network,
            init_cap_event_type_layout: init_cap_event_type_layout,
            approve_event_type_layout: approve_event_type_layout,
        };

        let first_committee = EpochCommittee {
            id: object::new(ctx),
            committee: init_committee,
        };

        event::emit(
            EpochCommitteeSubmitted {
                epoch: epoch_id_committee,
                registry_id: object::uid_to_inner(&registry.id),
                epoch_committee_id: object::id(&first_committee),
            }
        );

        let latest = create_latest_state(object::id(&registry));

        transfer::share_object(registry);
        transfer::freeze_object(first_committee);

        create_authority(
            authority_name,
            authority_unique_identifier,
            latest,
            config,
            authority_owner_dwallet_cap,
            ctx,
        );
    }

    /// Updates the SuiStateProofRegistry with a new committee.
    ///
    /// This function is used to sync the authority state with the new committee by
    /// verifying the submitted checkpoint summary.
    /// It creates a new EpochCommittee with the verified information and updates the
    /// registry's highest epoch. Then it emits an event to record the submission of
    /// the new committee and freezes the committee object to prevent further modifications.
    /// This function updates the SuiStateProofRegistry, which is pointed to by the `Authority`,
    /// So this also updates the authority state.
    public fun submit_new_state_committee(
        registry: &mut SuiStateProofRegistry,
        prev_committee: &EpochCommittee,
        new_checkpoint_summary: vector<u8>,
        ctx: &mut TxContext,
    ) {
        let (
            new_committee_verified_bytes,
            new_committee_epoch
        ) = sui_state_proof_verify_committee(
            prev_committee.committee,
            new_checkpoint_summary
        );

        assert!(
            new_committee_epoch - 1 == registry.highest_epoch,
            EWrongEpochSubmitted
        );

        let committee_new = EpochCommittee {
            id: object::new(ctx),
            committee: new_committee_verified_bytes,
        };

        registry.highest_epoch = registry.highest_epoch + 1;

        event::emit(
            EpochCommitteeSubmitted {
                epoch: registry.highest_epoch,
                registry_id: object::uid_to_inner(&registry.id),
                epoch_committee_id: object::id(&committee_new),
            }
        );

        transfer::freeze_object(committee_new);
    }

    /// This function ensures that the dWallet capability ID matches the capability ID
    /// used in the SUI network.
    /// It does so by extracting the capability IDs from the transaction's events and
    /// comparing them. If the IDs match, it updates the foreign capability ID in the binder
    /// with the SUI capability ID. Otherwise, it throws an error.
    public fun verify_dwallet_cap_and_sui_cap_match(
        authority: &Authority<SuiStateProofConfig>,
        binder: &mut DWalletBinder,
        committee: &EpochCommittee,
        checkpoint_summary: vector<u8>,
        checkpoint_contents: vector<u8>,
        transaction: vector<u8>,
    ) {
        let config = authority_binder::borrow_config(authority);
        // Get the cap id of the dwallet cap and the cap id of the cap used on SUI
        // by iterating the transaction events
        let (
            sui_cap_id_bytes,
            dwallet_cap_id_bytes
        ) = sui_state_proof_verify_link_cap(
            committee.committee,
            checkpoint_summary,
            checkpoint_contents,
            transaction,
            config.init_cap_event_type_layout,
            config.dwallet_package_id_in_sui_network
        );

        // check if the cap id used on SUI is the same as the id of dwallet_cap
        let sui_cap_id_addresses = bcs::peel_vec_address(&mut bcs::new(sui_cap_id_bytes));
        let sui_cap_id_address = *vector::borrow(&sui_cap_id_addresses, 0);

        let dwallet_cap_id_addresses = bcs::peel_vec_address(
            &mut bcs::new(dwallet_cap_id_bytes)
        );
        let dwallet_cap_id_address = *vector::borrow(&dwallet_cap_id_addresses, 0);

        assert!(
            object::id_from_address(dwallet_cap_id_address) == authority_binder::dwallet_cap_id(
                binder
            ),
            EWrongDWalletCapId
        );

        authority_binder::set_foreign_cap_id(
            binder,
            object::id_from_address(sui_cap_id_address)
        );
    }

    /// Approves messages based on state proof verification.
    ///
    /// This function verifies the transaction against the current state by extracting
    /// capability IDs and message bytes.
    /// It only approves messages associated with the capability ID represented by the cap wrapper. It iterates through the
    /// extracted capabilities and selects messages that match the SUI DWallet capability ID. The function then calls the
    /// `approve_messages` function to approve these messages on behalf of the provided DWallet.
    /// Returns a vector of `MessageApproval`s.
    public fun transaction_state_proof(
        binder: &mut DWalletBinder,
        authority: &Authority<SuiStateProofConfig>,
        committee: &EpochCommittee,
        checkpoint_summary: vector<u8>,
        checkpoint_contents: vector<u8>,
        transaction: vector<u8>,
        dwallet: &DWallet<Secp256K1>,
    ): (
        vector<vector<MessageApproval>>,
        vector<vector<u8>>
    ) {
        let config = authority_binder::borrow_config(authority);

        let (
            cap_ids_serialised_bytes,
            messages_serialised_bytes
        ) = sui_state_proof_verify_transaction(
            committee.committee,
            checkpoint_summary,
            checkpoint_contents,
            transaction,
            config.approve_event_type_layout,
            config.dwallet_package_id_in_sui_network
        );

        let cap_ids = bcs::peel_vec_address(
            &mut bcs::new(cap_ids_serialised_bytes)
        );

        let sui_dwallet_cap_id = get_foreign_cap_id(binder);
        // Basically this is peel_vec_vec_vec<u8>.
        let bcs = bcs::new(messages_serialised_bytes);
        let len = bcs::peel_vec_length(&mut bcs);
        let i = 0;
        let messages = vector::empty<vector<vector<u8>>>();
        while (i < len) {
            vector::push_back(
                &mut messages,
                bcs::peel_vec_vec_u8(&mut bcs)
            );
            i = i + 1;
        };

        assert!(
            vector::length(&cap_ids) == vector::length(&messages),
            EWrongAmountOfDWalletCaps
        );
        let messages_for_verification = vector::empty<vector<u8>>();
        let result = vector::empty<vector<MessageApproval>>();
        let i = 0;
        while (i < vector::length(&cap_ids)) {
            let cap_id_address = *vector::borrow(&cap_ids, i);
            if (object::id_from_address(cap_id_address) == sui_dwallet_cap_id) {
                let messages_to_approve = *vector::borrow(&messages, i);
                let j = 0;
                while (
                    j < vector::length(&messages_to_approve)
                ) {
                    vector::push_back(
                        &mut messages_for_verification,
                        *vector::borrow(&messages_to_approve, j)
                    );
                    j = j + 1;
                };
                let approvals = authority_binder::approve_messages(
                    binder, dwallet, messages_to_approve
                );
                vector::push_back(&mut result, approvals);
            };
            i = i + 1;
        };
        (result, messages_for_verification)
    }

    /// Approves an ACK message with authority.
    ///
    /// This function verifies the ACK message by constructing a message from the provided
    /// information and comparing it to the received message.
    /// If the messages match, the function approves the message using the `approve_messages` function.
    /// Returns a vector of `MessageApproval`s.
    public fun approve_ack_message_with_authority(
        authority: &Authority<SuiStateProofConfig>,
        message: vector<u8>,
        binder_id: ID,
        dwallet_cap_id: ID,
        bind_to_authority_id: ID,
        nonce: u64,
        virgin_bound: bool,
    ): vector<MessageApproval> {
        // ) : (vector<u8>, vector<u8>, bool) {
        let info_as_vec = vector::empty();
        vector::append(
            &mut info_as_vec,
            object::id_to_bytes(&binder_id)
        );
        vector::append(
            &mut info_as_vec,
            object::id_to_bytes(&dwallet_cap_id)
        );
        vector::append(
            &mut info_as_vec,
            object::id_to_bytes(&bind_to_authority_id)
        );
        vector::append(
            &mut info_as_vec,
            std::bcs::to_bytes(&nonce)
        );
        vector::append(
            &mut info_as_vec,
            std::bcs::to_bytes(&virgin_bound)
        );

        let constructed_message = sha2_256(info_as_vec);
        let constructed_message_bcs = dwallet::bcs::to_bytes(&constructed_message);
        let constructed_message_len: u64 = vector::length(&constructed_message_bcs);
        let message_len: u64 = vector::length(&message);

        assert!(
            constructed_message_len == message_len,
            15
        );
        assert!(
            constructed_message_bcs == message,
            16
        );
        // (constructed_message_bcs, message, constructed_message_len == message_len)

        dwallet::approve_messages(
            authority_binder::borrow_dwallet_cap(authority),
            vector[message]
        )
    }
}
