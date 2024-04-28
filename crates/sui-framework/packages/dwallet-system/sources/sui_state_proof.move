#[allow(unused_field)]
module dwallet_system::sui_state_proof {

    use dwallet::object::{Self, ID, UID};
    use dwallet_system::dwallet::{Self, DWalletCap, MessageApproval};
    use dwallet::tx_context::{TxContext};
    use dwallet::transfer;
    use dwallet::bcs;
    use dwallet::event;


    const EWrongEpochSubmitted: u64 = 0;


    struct StateProofRegistry has key, store {
            id: UID,
            highest_epoch: u64,
    }

    struct StateProofConfig has key, store{
        id: UID,
        registry_id: ID,
        package_id: vector<u8>,
        event_type_layout: vector<u8>,
    }

    struct EpochCommittee has key, store {
        id: UID,
        committee: vector<u8>,
    }


    struct EpochCommitteeSubmitted has copy, drop {
        epoch: u64,
        epoch_committee_id: ID,
    }

    struct CapWrapper has key, store {
        id: UID,
        cap_id_sui: ID,
        cap: DWalletCap,
    }

    struct Test has copy, drop, store {
        a: u64,
        b: u64,
    }

    native fun sui_state_proof_verify_committee(prev_committee: vector<u8>, checkpoint_summary: vector<u8>): (vector<u8>, u64);

    native fun sui_state_proof_verify_transaction(committee: vector<u8>, checkpoint_summary: vector<u8>, checkpoint_contents: vector<u8>, transaction: vector<u8>,  event_type_layout: vector<u8>,  package_id: vector<u8>): (vector<u8>, vector<u8>);



    public fun init_module(init_committee: vector<u8>, package_id: vector<u8>, event_type_layout: vector<u8>, epoch_id_committee: u64, ctx: &mut TxContext) {
        let registry = StateProofRegistry {
            id: object::new(ctx),
            highest_epoch: epoch_id_committee,
        };

        let config = StateProofConfig {
            id: object::new(ctx),
            registry_id: object::id(&registry),
            package_id: package_id,
            event_type_layout: event_type_layout,
        };

        let first_committee = EpochCommittee {
            id: object::new(ctx),
            committee: init_committee,
        };

        event::emit(EpochCommitteeSubmitted {
            epoch: epoch_id_committee,
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
        let (new_committee_verified_bytes, committee_epoch) = sui_state_proof_verify_committee(prev_committee.committee, new_checkpoint_summary);

        let committee_new = EpochCommittee {
                                    id: object::new(ctx),
                                    committee: new_committee_verified_bytes,
                                    };

        registry.highest_epoch = registry.highest_epoch + 1;

        assert!(committee_epoch == registry.highest_epoch, EWrongEpochSubmitted);


        event::emit(EpochCommitteeSubmitted {
            epoch: registry.highest_epoch,
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
        
        let (cap_id_bytes, _) = sui_state_proof_verify_transaction(committee.committee, checkpoint_summary, checkpoint_contents, transaction, config.event_type_layout, config.package_id );
        
        let cap_id_address = bcs::peel_address(&mut bcs::new(cap_id_bytes));
        let wrapper = CapWrapper {
            id: object::new(ctx),
            cap_id_sui: object::id_from_address(cap_id_address),
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
        ): vector<MessageApproval>{
        
        let (cap_id_bytes, messages_serialised_bytes) = sui_state_proof_verify_transaction(committee.committee, checkpoint_summary, checkpoint_contents, transaction, config.event_type_layout, config.package_id );


        let messages = bcs::peel_vec_vec_u8(&mut bcs::new(messages_serialised_bytes));
        let cap_id_address = bcs::peel_address(&mut bcs::new(cap_id_bytes));
        
        assert!(object::id_from_address(cap_id_address)  == cap_wrapper.cap_id_sui, 0);
        dwallet::approve_messages(&cap_wrapper.cap, messages)
    }
}