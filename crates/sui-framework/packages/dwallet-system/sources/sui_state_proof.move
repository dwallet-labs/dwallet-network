#[allow(unused_field)]
module dwallet_system::sui_state_proof {

    use dwallet::object::{Self, ID, UID};
    use dwallet_system::dwallet::{Self, DWalletCap, MessageApproval};
    use dwallet::tx_context::{TxContext};
    use dwallet::transfer;
    use dwallet::bcs;
    use dwallet::event;
    use std::vector;


    const EWrongEpochSubmitted: u64 = 0;
    const EWrongDWalletCapId: u64 = 1;
    const EWrongAmountOfDWalletCaps: u64 = 2;


    struct StateProofRegistry has key, store {
        id: UID,
        highest_epoch: u64,
    }

    struct StateProofConfig has key, store{
        id: UID,
        registry_id: ID,
        package_id: vector<u8>,
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


    native fun sui_state_proof_verify_committee(prev_committee: vector<u8>, checkpoint_summary: vector<u8>): (vector<u8>, u64);

    native fun sui_state_proof_verify_link_cap(committee: vector<u8>, checkpoint_summary: vector<u8>, checkpoint_contents: vector<u8>, transaction: vector<u8>,  event_type_layout: vector<u8>,  package_id: vector<u8>): (vector<u8>, vector<u8>);

    native fun sui_state_proof_verify_transaction(committee: vector<u8>, checkpoint_summary: vector<u8>, checkpoint_contents: vector<u8>, transaction: vector<u8>,  event_type_layout: vector<u8>,  package_id: vector<u8>): (vector<u8>, vector<u8>);



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
            package_id: package_id,
            init_cap_event_type_layout: init_cap_event_type_layout,
            approve_event_type_layout: approve_event_type_layout,
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
        
        transfer::share_object(registry);
        transfer::freeze_object(config);
        transfer::freeze_object(first_committee);
    }

    
    


    public fun submit_new_state_committee(
        registry: &mut StateProofRegistry,
        prev_committee: &EpochCommittee,
        new_checkpoint_summary: vector<u8>,
        ctx: &mut TxContext,
    ) {
        let (new_committee_verified_bytes, new_committee_epoch) = sui_state_proof_verify_committee(prev_committee.committee, new_checkpoint_summary);

        assert!(new_committee_epoch - 1 == registry.highest_epoch, EWrongEpochSubmitted);


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


    public fun create_dwallet_wrapper(
        config: &StateProofConfig,
        dwallet_cap: DWalletCap, 
        committee: &EpochCommittee,
        checkpoint_summary: vector<u8>,
        checkpoint_contents: vector<u8>,
        transaction: vector<u8>,
        ctx: &mut TxContext
    ){        
        
        let (sui_cap_ids_bytes, dwallet_cap_ids_bytes) = sui_state_proof_verify_link_cap(committee.committee, checkpoint_summary, checkpoint_contents, transaction, config.init_cap_event_type_layout, config.package_id );
        

        // check if the cap id used on SUI is the same as the id of dwallet_cap
        let sui_cap_id_address_vec = bcs::peel_vec_address(&mut bcs::new(sui_cap_ids_bytes));
        let dwallet_cap_id_address_vec = bcs::peel_vec_address(&mut bcs::new(dwallet_cap_ids_bytes));

        // assert!(vector::length(&sui_cap_id_address_vec) == vector::length(&dwallet_caps), EWrongAmountOfDWalletCaps);
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



    public fun transaction_state_proof(
        config: &StateProofConfig,
        cap_wrapper: &CapWrapper,
        committee: &EpochCommittee,
        checkpoint_summary: vector<u8>,
        checkpoint_contents: vector<u8>,
        transaction: vector<u8>,
        ): vector<vector<MessageApproval>>{
        
        let (cap_ids_serialised_bytes, messages_serialised_bytes) = sui_state_proof_verify_transaction(committee.committee, checkpoint_summary, checkpoint_contents, transaction, config.approve_event_type_layout, config.package_id );

        let cap_ids = bcs::peel_vec_address(&mut bcs::new(cap_ids_serialised_bytes));

        // basically this is peel_vec_vec_vec<u8>,
        let bcs = bcs::new(messages_serialised_bytes);
        let (len, i, messages) = (bcs::peel_vec_length(&mut bcs), 0, vector::empty<vector<vector<u8>>>());
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