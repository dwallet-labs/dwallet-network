# Bitcoin MultiSig in Sui Move

The following example implements a MultiSig mechanism for Bitcoin txs using Sui Move smart contract.

```move
module sui_move_btc_multisig::multisig {
    use dwallet_network::dwallet_cap::{Self, DWalletCap};
    use sui::object::{id_from_address};

    const ENoPermission: u64 = 0;
    const EAlreadyVoted: u64 = 1;

    public struct MultiSig has key {
        id: UID,
        signers: vector<address>,
        cap: DWalletCap,
        treshold: u64,
    }

    public struct MessageApprovalProposal has key {
        id: UID,
        multisig_id: ID,
        signers: vector<address>,
        message: vector<u8>,
        approved: bool,
    }

    public struct ReplaceSingersProposal has key {
        id: UID,
        multisig_id: ID,
        signers: vector<address>,
        new_signers: vector<address>,
        approved: bool,
    }

    public fun create_multisig(dwallet_network_cap_address: address, treshold: u64, ctx: &mut TxContext) {
        let dwallet_network_cap_id = id_from_address(dwallet_network_cap_address);
        let cap = dwallet_cap::create_cap(dwallet_network_cap_id, ctx);

        let multisig = MultiSig {
            id: object::new(ctx),
            signers: vector[tx_context::sender(ctx)],
            cap,
            treshold
        };

        transfer::share_object(multisig);
    }

    fun find_signer(signers: &vector<address>, sender: address): bool {
        let length = vector::length(signers);
        let mut i = 0;
        while (i < length) {
            let addr = *vector::borrow(signers, i);
            if (addr == sender) {
                return true
            };
            i = i + 1;
        };
        false
    }

    public fun propose_message_approval(multisig: &MultiSig, message: vector<u8>, ctx: &mut TxContext) {
        let sender = tx_context::sender(ctx);
        let has_permission = find_signer(&multisig.signers, sender);
        assert!(has_permission, ENoPermission);
        let proposal = MessageApprovalProposal {
            id: object::new(ctx),
            multisig_id: object::id(multisig),
            signers: vector[],
            message,
            approved: false,
        };

        transfer::share_object(proposal);
    }

    public fun sign_message_approval(multisig: &MultiSig, proposal: &mut MessageApprovalProposal, ctx: &mut TxContext) {
        let sender = tx_context::sender(ctx);

        assert!(!proposal.approved, EAlreadyVoted);

        let has_permission = find_signer(&multisig.signers, sender);
        assert!(has_permission, ENoPermission);
        
        let already_voted = find_signer(&proposal.signers, sender);
        assert!(already_voted, EAlreadyVoted);

        vector::push_back(&mut proposal.signers, sender);

        let length = vector::length(&proposal.signers);
        if (length == multisig.treshold) {
            dwallet_cap::approve_message(&multisig.cap, proposal.message);
            proposal.approved = true;
        }
    }


    public fun propose_replace_signers(multisig: &MultiSig, new_signers: vector<address>, ctx: &mut TxContext) {
        let sender = tx_context::sender(ctx);
        let has_permission = find_signer(&multisig.signers, sender);
        assert!(has_permission, ENoPermission);
        let proposal = ReplaceSingersProposal {
            id: object::new(ctx),
            multisig_id: object::id(multisig),
            signers: vector[],
            new_signers,
            approved: false,
        };

        transfer::share_object(proposal);
    }

    public fun sign_replace_signers(multisig: &mut MultiSig, proposal: &mut ReplaceSingersProposal, ctx: &mut TxContext) {
        let sender = tx_context::sender(ctx);

        assert!(!proposal.approved, EAlreadyVoted);

        let has_permission = find_signer(&multisig.signers, sender);
        assert!(has_permission, ENoPermission);
        
        let already_voted = find_signer(&proposal.signers, sender);
        assert!(already_voted, EAlreadyVoted);

        vector::push_back(&mut proposal.signers, sender);

        let length = vector::length(&proposal.signers);
        if (length == multisig.treshold) {
            vector::push_back(&mut proposal.signers, sender);
            multisig.signers = proposal.new_signers;
            proposal.approved = true;
        }
    }

}
```
