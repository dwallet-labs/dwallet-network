// This module is deployed on the Sui network.
// It is responsible for managing the dWallet capabilities wrapper and approving messages.

module dwallet_network::dwallet_cap {
    use std::vector;
    use sui::object::{Self, ID, UID};
    use sui::event;
    use sui::tx_context::TxContext;

    /// Represents the primary dWallet capability.
    ///
    /// This struct wraps an instance of the dWallet capability, identified by a unique
    /// ID (`UID`) and a `dwallet_network_cap_id` that links it to the dWallet network.
    struct DWalletCap has key, store {
        /// Unique identifier for this dWallet capability instance.
        id: UID,
        /// Identifier linking this capability to the dWallet network.
        dwallet_network_cap_id: ID,
    }

    /// Event emitted when a new dWallet capability is initialized.
    ///
    /// This event helps track the initialization of `DWalletCap` instances on the network.
    struct DWalletNetworkInitCapRequest has copy, drop {
        /// The unique identifier of the initialized capability.
        cap_id: ID,
        /// The identifier linking this capability to the dWallet network.
        dwallet_network_cap_id: ID,
    }

    /// Event emitted when messages are approved for a dWallet capability.
    ///
    /// This struct captures the ID of the `DWalletCap` and the messages
    /// associated with the approval request.
    struct DWalletNetworkApproveRequest has copy, drop {
        /// The unique identifier of the capability for which messages are being approved.
        cap_id: ID,
        /// A vector of messages to be approved, where each message is a vector of bytes.
        messages: vector<vector<u8>>,
    }

    /// Creates a new dWallet capability (`DWalletCap`) and emits an initialization event.
    ///
    /// # Arguments
    ///
    /// * `dwallet_network_cap_id` - The identifier linking this capability to the dWallet network.
    /// * `ctx` - A mutable reference to the transaction context (`TxContext`), used to create the new capability.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `DWalletCap`.
    public fun create_cap(dwallet_network_cap_id: ID, ctx: &mut TxContext): DWalletCap {
        let cap = DWalletCap {
            id: object::new(ctx),
            dwallet_network_cap_id
        };

        // Emit an event to notify the initialization of a new dWallet capability.
        event::emit(DWalletNetworkInitCapRequest {
            // The object ID of the newly created `DWalletCap` object.
            cap_id: object::id(&cap),
            // The object ID of the dWallet capability on the dWallet Network that you wish to control.
            dwallet_network_cap_id,
        });

        cap
    }

    /// Approves a set of messages for a given dWallet capability.
    ///
    /// This function emits an `DWalletNetworkApproveRequest` event that contains
    /// the identifier of the dWallet capability and the messages to be approved.
    ///
    /// # Arguments
    ///
    /// * `cap` - A reference to the `DWalletCap` for which messages are being approved.
    /// * `messages` - A vector of messages, each represented as a vector of bytes.
    public fun approve_message(cap: &DWalletCap, messages: vector<vector<u8>>) {
        // Emit an event to notify that messages have been approved for the specified dWallet capability.
        event::emit(DWalletNetworkApproveRequest {
            cap_id: object::id(cap),
            messages,
        });
    }
}
