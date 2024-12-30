The User initiates a dWallet DKG first round by calling the `launch_dkg_first_round()` Move function from the
[dwallet_2pc_mpc_ecdsa_k1.move](crates/pera-framework/packages/pera-system/sources/dwallet_2pc_mpc_ecdsa_k1.move)
module.

The Move function emits a `StartDKGFirstRoundEvent` event, and creates a transaction with the event as an effect.

This transaction is being sequenced in the consensus, and every validator process it, along with the entire consensus
output, in the `ConsensusHandler::handle_consensus_output_internal()` function.

When the validator executes the transaction in the `AuthorityState::commit_certificate()` function, it looks for any
`StartDKGFirstRoundEvent` events in the `AuthorityState::handle_dwallet_mpc_events()` function.
The `session_info_from_event()` function does that filtering, and it returns `Option::Some(data)` only if the event is a
DWallet MPC related event.

If it finds such an event, it sends it, using the `AuthorityPerEpochStore`’s dwallet_mpc_sender, to the
DWalletMPCManager.
We send the messages to the manager using this channel so it will be able to run the heavy MPC
cryptography on a different thread, without blocking the chain from processing other transactions in the meantime, such
as faucet requests or token
transfers.

The DWalletMPCManager creates a new DwalletMPCSession with the given session_id and sender from the event, or push the
event to the pending creation instances queue if it reached the limit of DwalletMPCManager::max_active_mpc_sessions. The
limit is being set in the Validator’s configuration file.
