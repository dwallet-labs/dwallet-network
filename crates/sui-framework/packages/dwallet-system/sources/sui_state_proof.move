

#[allow(unused_field)]
module dwallet_system::sui_state_proof {

    use dwallet::object::{Self, ID, UID};
    use dwallet_system::dwallet::{Self, DWalletCap, MessageApproval};
    use dwallet::tx_context::{TxContext};
    use dwallet::transfer;
    use dwallet::bcs;


    struct EpochCommittee has key, store {
        id: UID,
        config_id: ID,
        epoch: u64,
        committee: vector<u8>,
    }


    struct SuiModuleConfig has key, store {
        id: UID,
        package_id: vector<u8>,
        event_type_layout: vector<u8>,
        message_field_name: vector<u8>,
    }

    struct CapWrapper has key, store {
        id: UID,
        cap: DWalletCap,
    }

    public fun create_state_proof_config(
        package_id: vector<u8>,
        event_type_layout: vector<u8>,
        message_field_name: vector<u8>,
        ctx: &mut TxContext,
    ) {
        let config = SuiModuleConfig {
            id: object::new(ctx),
            package_id,
            event_type_layout,
            message_field_name,
        };

        transfer::freeze_object(config);
    }

    public fun init_module(init_committee: vector<u8>, config: &SuiModuleConfig, ctx: &mut TxContext) {
        let first_committee = EpochCommittee {
            id: object::new(ctx),
            config_id: object::uid_to_inner(&config.id),
            epoch: 0,
            committee: init_committee,
        };

        transfer::freeze_object(first_committee);
    }


    public fun create(dwallet_cap: DWalletCap, ctx: &mut TxContext){
        let wrapper = CapWrapper {
            id: object::new(ctx),
            cap: dwallet_cap,
        };

        transfer::share_object(wrapper);
    }

    native fun sui_state_proof_verify_committee(prev_committee: vector<u8>, checkpoint_summary: vector<u8>): vector<u8>;


    public fun submit_new_state_committee(
        epoch: &EpochCommittee,
        checkpoint_summary: vector<u8>,
        ctx: &mut TxContext,
    ) {
        //validate that the old committe signed off on the new committee
        let committee_new = sui_state_proof_verify_committee(epoch.committee, checkpoint_summary);

        let committee_new = EpochCommittee {
                            id: object::new(ctx),
                            config_id: epoch.config_id,
                            epoch: epoch.epoch + 1,
                            committee: committee_new,
                            };
        
        transfer::freeze_object(committee_new);
    }



    // basically checks for events that the dWallet signed on SUI
    native fun sui_state_proof_verify_transaction(committee: vector<u8>, checkpoint_summary: vector<u8>, checkpoint_contents: vector<u8>, transaction: vector<u8>, package_id_sui_event: vector<u8>, event_type_layout: vector<u8>,  message_field_name: vector<u8>): (vector<u8>);


    public fun transaction_state_proof(
        cap_wrapper: &CapWrapper,
        committee: &EpochCommittee,
        config: &SuiModuleConfig,
        checkpoint_summary: vector<u8>,
        checkpoint_contents: vector<u8>,
        transaction: vector<u8>, // serialised tx data
        ): vector<MessageApproval>{

       let messages_serialised = sui_state_proof_verify_transaction(committee.committee, checkpoint_summary, checkpoint_contents, transaction, config.package_id, config.event_type_layout, config.message_field_name);
        
        let messages = bcs::peel_vec_vec_u8(&mut bcs::new(messages_serialised));
        dwallet::approve_messages(&cap_wrapper.cap, messages)
    }

}