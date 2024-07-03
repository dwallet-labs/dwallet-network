## MPC Notes

- Aggregation function is a part of the MPC code, not related to 2PCMPC.
- User -> BC -> MPC -> BC -> User
- DKG:
- Presign:
    - User
    - Blockchain
    - Output
    - User takes output for Sign
    - Blockchain finishes presign
    - User sends sign to blockchain with all outputs.

## Packages

- SUI-Framework is the Move modules.
- [dwallet.move](crates/sui-framework/packages/dwallet-system/sources/dwallet.move)
- [dwallet_2pc_mpc_ecdsa_k1.move](crates/sui-framework/packages/dwallet-system/sources/dwallet_2pc_mpc_ecdsa_k1.move)
- Inside `signature_mpc` dir the file `aggregate.rs` is used for inner aggregation.
- Each Protocol (DKG, Presign, Sign) has its own module, but it calls aggregation function to do inner block
  aggregation (part of the MPC requirements)
- Currently, the MPC requires all validators, no threshold.

## Reliable Broadcast

- Reliable broadcast is a protocol that ensures that all honest nodes receive the same message.
- We know the parties
- We can guarantee some time.
- We can identify attacks and malicious nodes.

## Sui Dirs

- Narwhal+Bullshark = old consensus.
- Consensus = Mysticeti
- [external-crates–](external-crates)-SUI MOVE CODE (The Engine)–Omer added code that blocks publish.
- [sdk–](sdk)-Typescript SDK, Omer added a typescript code that calls WASM code.
- sui-execution - Allows running Move engine – this one executes the move code.
- [sui-core](crates/sui-core) - Transactions, Checkpoints, Communication with Consensus.
    - Inside this the `signature_mpc` resides.
    - authority == validator code.
    - sui-framework = move modules
    - sui-node = node execution code (creates the process)
    - sui-swarm + sui-swarm-config = local run of several Validators (has a CLI) – This crate contains a collection of
      utilities for managing complete Sui
      networks.
      The intended use for these utilities is for performing end-to-end testing and benchmarking.
- [sui-protocol-config](./crates/sui-protocol-config) - the SUI current Protocol configuration, when upgrade happens,
  the protocol version and config are changed (if needed).
  Note that the protocol can be different for each network, to test features, for example.
  The Validators need to reach consensus to upgrade, and to enable/disable features.
  Like everything else this change can happen every epoch.
- [sui-framework](./crates/sui-framework) - The SUI Move Modules, dwallet-system == sui-system, dwallet-framework ==
  sui-framework.
    - [dwallet-framework](./crates/sui-framework/packages/dwallet-framework) - The Move modules for SUI development.
    - [deepbook](./crates/sui-framework/packages/deepbook) - not needed
    - [dwallet-system](./crates/sui-framework/packages/dwallet-system) - The internal blockchain logic—genesis,
      validator control, staking, storage, etc... all in Move.

- In Every Epoch, several things that can be changed:
    - Protocol Version.
    - Binary Version.
    - Committee.
    - Move STDLib Version.
    - SUI Execution Version.
