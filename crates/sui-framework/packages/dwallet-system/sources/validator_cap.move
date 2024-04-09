// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module dwallet_system::validator_cap {
    use dwallet::object::{Self, ID, UID};
    use dwallet::transfer;
    use dwallet::tx_context::{Self, TxContext};
    friend dwallet_system::dwallet_system_state_inner;
    friend dwallet_system::validator;
    friend dwallet_system::validator_set;


    // use dwallet::object::{Self, UID};
    // use dwallet_system::dwallet::{Self, DWalletCap, MessageApproval};
    // // use dwallet::tx_context::{TxContext};
    // // use dwallet::transfer;
    // use dwallet::bcs;
    // use dwallet::vec_map::{Self, VecMap};

    #[test_only]
    friend dwallet_system::dwallet_system_tests;
    #[test_only]
    friend dwallet_system::rewards_distribution_tests;

    /// The capability object is created when creating a new `Validator` or when the
    /// validator explicitly creates a new capability object for rotation/revocation.
    /// The holder address of this object can perform some validator operations on behalf of
    /// the authorizer validator. Thus, if a validator wants to separate the keys for operation
    /// (such as reference gas price setting or tallying rule reporting) from fund/staking, it
    /// could transfer this capability object to another address.

    /// To facilitate rotating/revocation, `Validator` stores the ID of currently valid
    /// `UnverifiedValidatorOperationCap`. Thus, before converting `UnverifiedValidatorOperationCap`
    /// to `ValidatorOperationCap`, verification needs to be done to make sure
    /// the cap object is still valid.
    struct UnverifiedValidatorOperationCap has key, store {
        id: UID,
        authorizer_validator_address: address,
    }
    
    // struct EpochCommittee has key, store {
    //     id: UID,
    //     // config_id: ID,
    //     epoch: u64,
    //     committee: vector<u8>,
    // }

    /// Privileged operations require `ValidatorOperationCap` for permission check.
    /// This is only constructed after successful verification.
    struct ValidatorOperationCap has drop {
        authorizer_validator_address: address,
    }

    public(friend) fun unverified_operation_cap_address(cap: &UnverifiedValidatorOperationCap): &address {
        &cap.authorizer_validator_address
    }

    public(friend) fun verified_operation_cap_address(cap: &ValidatorOperationCap): &address {
        &cap.authorizer_validator_address
    }

    /// Should be only called by the friend modules when adding a `Validator`
    /// or rotating an existing validaotr's `operation_cap_id`.
    public(friend) fun new_unverified_validator_operation_cap_and_transfer(
        validator_address: address,
        ctx: &mut TxContext,
    ): ID {
        // This function needs to be called only by the validator itself, except
        // 1. in genesis where all valdiators are created by @0x0
        // 2. in tests where @0x0 could be used to simplify the setup
        let sender_address = tx_context::sender(ctx);
        assert!(sender_address == @0x0 || sender_address == validator_address, 0);

        let operation_cap = UnverifiedValidatorOperationCap {
            id: object::new(ctx),
            authorizer_validator_address: validator_address,
        };
        let operation_cap_id = object::id(&operation_cap);
        transfer::public_transfer(operation_cap, validator_address);
        operation_cap_id
    }

    /// Convert an `UnverifiedValidatorOperationCap` to `ValidatorOperationCap`.
    /// Should only be called by `validator_set` module AFTER verification.
    public(friend) fun new_from_unverified(
        cap: &UnverifiedValidatorOperationCap,
    ): ValidatorOperationCap {
        ValidatorOperationCap {
            authorizer_validator_address: cap.authorizer_validator_address
        }
    }

    


    


    // struct SuiModuleConfig has key, store {
    //     id: UID,
        
    // }

    // struct Registry has key, store {
    //     id: UID,
    //     mapping: VecMap<u64, address>, // mapping for the object id to 
    //     highest_epoch: u64,
    //     package_id: vector<u8>,
    //     event_type_layout: vector<u8>,
    //     message_field_name: vector<u8>,
    // }

    // struct CapWrapper has key, store {
    //     id: UID,
    //     cap: DWalletCap,
    // }


    // // TODO add wittness here so it can be initialized only once??
    // public fun init_module(init_committee: vector<u8>, package_id: vector<u8>, event_type_layout: vector<u8>, message_field_name: vector<u8>, ctx: &mut TxContext) {
        
    //     let registry = Registry {
    //         id: object::new(ctx),
    //         mapping: vec_map::empty(),
    //         highest_epoch: 0,
    //         package_id,
    //         event_type_layout,
    //         message_field_name,
    //     };
    //     transfer::share_object(registry);
        
    //     let first_committee = EpochCommittee {
    //         id: object::new(ctx),
    //         epoch: 0,
    //         committee: init_committee,
    //     };
    //     transfer::freeze_object(first_committee);
    // }


    // public fun create(dwallet_cap: DWalletCap, ctx: &mut TxContext){
    //     let wrapper = CapWrapper {
    //         id: object::new(ctx),
    //         cap: dwallet_cap,
    //     };

    //     transfer::share_object(wrapper);
    // }

    // native fun sui_state_proof_verify_committee(prev_committee: vector<u8>, checkpoint_summary: vector<u8>): vector<u8>;


    // public fun submit_new_state_committee(
    //     prev_epoch: &EpochCommittee,
    //     new_checkpoint_summary: vector<u8>,
    //     ctx: &mut TxContext,
    // ) {
    //     //validate that the old committe signed off on the new committee
    //     let committee_new = sui_state_proof_verify_committee(prev_epoch.committee, new_checkpoint_summary);

    //     let committee_new = EpochCommittee {
    //                         id: object::new(ctx),
    //                         // config_id: prev_epoch.config_id,
    //                         epoch: prev_epoch.epoch + 1,
    //                         committee: committee_new,
    //                         };
        
    //     transfer::freeze_object(committee_new);
    // }



    // // basically checks for events that the dWallet signed on SUI
    // native fun sui_state_proof_verify_transaction(committee: vector<u8>, checkpoint_summary: vector<u8>, checkpoint_contents: vector<u8>, transaction: vector<u8>, package_id_sui_event: vector<u8>, event_type_layout: vector<u8>,  message_field_name: vector<u8>): (vector<u8>);


    // public fun transaction_state_proof(
    //     registry: &Registry,
    //     cap_wrapper: &CapWrapper,
    //     committee: &EpochCommittee,
    //     checkpoint_summary: vector<u8>,
    //     checkpoint_contents: vector<u8>,
    //     transaction: vector<u8>, // serialised tx data
    //     ): vector<MessageApproval>{
    //     // TODO integrity check that committee matches the respective registry address

    //    let messages_serialised = sui_state_proof_verify_transaction(committee.committee, checkpoint_summary, checkpoint_contents, transaction, registry.package_id, registry.event_type_layout, registry.message_field_name);
        
    //     let messages = bcs::peel_vec_vec_u8(&mut bcs::new(messages_serialised));
    //     dwallet::approve_messages(&cap_wrapper.cap, messages)
    // }

}
