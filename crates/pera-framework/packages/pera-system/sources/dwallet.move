// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module pera_system::dwallet {
    #[allow(unused_field)]
    /// `DWallet` represents a wallet that is created after the DKG process.
    public struct DWallet<phantom T> has key, store {
        id: UID,
        session_id: ID,
        dwallet_cap_id: ID,
        // `output` of the DKG decentralized process.
        output: vector<u8>,
    }

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

    /// `DWalletCap` holder controls a corresponding `Dwallet`.
    public struct DWalletCap has key, store {
        id: UID,
    }

    /// Create a new `DWalletCap`
    /// The holder of this capability owns the `DWallet`.
    public(package) fun create_dwallet_cap(ctx: &mut TxContext): DWalletCap {
        DWalletCap {
            id: object::new(ctx),
        }
    }
}
