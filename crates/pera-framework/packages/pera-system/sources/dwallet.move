// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

/// This module defines the core data structures and functions for
/// working with dWallets in the pera system.
///
/// ## Overview
///
/// - A **dWallet** (`DWallet`) represents a wallet that is created after the Distributed Key Generation (DKG) process.
///   It encapsulates the session ID, capability ID, and the output of the DKG's second round.
/// - A **dWallet capability** (`DWalletCap`) represents a capability that grants
///   ownership and control over a corresponding `DWallet`.
///
/// ## Key Concepts
///
/// - **DWallet**: A generic wallet structure with a phantom type `T`.
/// - **DWalletCap**: A capability object granting control over a specific dWallet.
/// - **Session ID**: A unique identifier for the DKG session.
module pera_system::dwallet {

    #[allow(unused_field)]
    /// `DWallet` represents a wallet that is created after the DKG process.
    ///
    /// ### Fields
    /// - `id`: The unique identifier for the dWallet object.
    /// - `session_id`: The ID of the session that generated this dWallet.
    /// - `dwallet_cap_id`: The ID of the dWallet capability associated with this wallet.
    /// - `output`: The output of the second DKG round, represented as a `vector<u8>`.
    public struct DWallet<phantom T> has key, store {
        id: UID,
        session_id: ID,
        dwallet_cap_id: ID,
        // The output of the second DKG round.
        output: vector<u8>,
    }

    /// `DWalletCap` holder controls a corresponding `DWallet`.
    ///
    /// ### Fields
    /// - `id`: The unique identifier for the dWallet capability object.
    public struct DWalletCap has key, store {
        id: UID,
    }

    /// A generic function to create a new [`DWallet`] object of type `T`.
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
    /// The holder of the `DWalletCap` has control and ownership over
    /// the associated `DWallet`.
    /// This function creates the capability object and transfers it to the sender's address.
    public(package) fun create_dwallet_cap(ctx: &mut TxContext): ID {
        let cap = DWalletCap {
            id: object::new(ctx),
        };
        let id = object::id(&cap);
        transfer::transfer(cap, ctx.sender());
        id
    }
}
