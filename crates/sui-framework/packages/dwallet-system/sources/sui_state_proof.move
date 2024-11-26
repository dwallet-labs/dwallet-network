#[allow(unused_field)]
module dwallet_system::sui_state_proof {
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
    const EStateProofNoMessagesToApprove: u64 = 2;

    // todo(yuval): doc all structs and functions
    struct StateProofRegistry has key, store {
        id: UID,
        highest_epoch: u64,
    }

    // todo(yuval): change name of structure `StateProofConfig` in move.
    struct StateProofConfig has key, store {
        id: UID,
        registry_id: ID,
        dwallet_package_id_in_sui_network: vector<u8>,
        init_cap_event_type_layout: vector<u8>,
        approve_event_type_layout: vector<u8>,
    }

    struct EpochCommittee has key, store {
        id: UID,
        committee: vector<u8>,
    }

    struct EpochCommitteeSubmitted has copy, drop {
        epoch: u64,
        registry_id: ID,
        epoch_committee_id: ID,
    }

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

    // todo(yuval): this is init function
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
        let registry = StateProofRegistry {
            id: object::new(ctx),
            highest_epoch: epoch_id_committee,
        };

        let config = StateProofConfig {
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

    public fun submit_new_state_committee(
        registry: &mut StateProofRegistry,
        prev_committee: &EpochCommittee,
        new_checkpoint_summary: vector<u8>,
        ctx: &mut TxContext,
    ) {
        // Yuval: This function updates the StateProofRegistry, which is pointed to by the `Authority`,
        // So this also updates the authority state.
        // Used for syncing new state.
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

    public fun verify_dwallet_cap_and_sui_cap_match(
        authority: &Authority<StateProofConfig>,
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

        assert!(object::id_from_address(dwallet_cap_id_address) == authority_binder::dwallet_cap_id(binder),
            EWrongDWalletCapId
        );

        authority_binder::set_foreign_cap_id(
            binder,
            object::id_from_address(sui_cap_id_address)
        );
    }

    public fun transaction_state_proof(
        binder: &mut DWalletBinder,
        authority: &Authority<StateProofConfig>,
        committee: &EpochCommittee,
        checkpoint_summary: vector<u8>,
        checkpoint_contents: vector<u8>,
        transaction: vector<u8>,
        dwallet: &DWallet<Secp256K1>,
    ): vector<MessageApproval> {
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

        let messages = bcs::peel_vec_vec_u8(
            &mut bcs::new(messages_serialised_bytes)
        );
        let cap_ids = bcs::peel_vec_address(
            &mut bcs::new(cap_ids_serialised_bytes)
        );

        let sui_dwallet_cap_id = get_foreign_cap_id(binder);
        // only messages are approved for the cap id that is represented by the cap wrapper
        let messages_to_approve = vector::empty<vector<u8>>();
        let i = 0;
        while (i < vector::length(&cap_ids)) {
            let cap_id_address = *vector::borrow(&cap_ids, i);
            if (object::id_from_address(cap_id_address) == sui_dwallet_cap_id) {
                vector::push_back(
                    &mut messages_to_approve,
                    *vector::borrow(&messages, i)
                );
            };
            i = i + 1;
        };

        assert!(
            vector::length(&messages_to_approve) > 0,
            EStateProofNoMessagesToApprove
        );

        authority_binder::approve_messages(binder, dwallet, messages_to_approve)
    }

    public fun approve_messages_with_authority(
        authority: &Authority<StateProofConfig>,
        messages: vector<vector<u8>>,
    ) : vector<MessageApproval> {
        // todo(yuval): this function should call `approve_message_with_authority` (need to implement) from authority_binder
        dwallet::approve_messages(
            authority_binder::borrow_dwallet_cap(authority),
            messages
        )
    }
}
