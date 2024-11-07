// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module pera_system::dwallet {

    #[allow(unused_field)]
    /// `DWallet` represents a wallet that is created after the DKG process.
    public struct DWallet<phantom T> has key, store {
        id: UID,
        session_id: ID,
        dwallet_cap_id: ID,
        // The output of the second DKG round.
        output: vector<u8>,
    }

    /// `DWalletCap` holder controls a corresponding `Dwallet`.
    public struct DWalletCap has key, store {
        id: UID,
    }

    /// A generic function to create a new [`DWallet`] object of type T.
    public(package) fun create_dwallet<T: drop>(
        session_id: ID,
        dwallet_cap_id: ID,
        output: vector<u8>,
        ctx: &mut TxContext
    ): DWallet<T> {
        DWallet<T> {
            id: object::new(ctx),
            session_id,
            dwallet_cap_id,
            output,
        }
    }

    /// Create a new [`DWalletCap`] object.
    /// The holder of this capability owns the `DWallet`.
    public(package) fun create_dwallet_cap(ctx: &mut TxContext): ID {
        let cap = DWalletCap {
            id: object::new(ctx),
        };
    let id = object::id(&cap);
        transfer::transfer(cap, ctx.sender());
        id
    }

    public(package) fun get_dwallet_cap_id<T: drop>(dwallet: &DWallet<T>): ID {
        dwallet.dwallet_cap_id
    }

    public(package) fun get_dwallet_output<T: drop>(dwallet: &DWallet<T>): vector<u8> {
        dwallet.output
    }
}
