# dWallet MPC

This module contains the core logic of the MPC protocol for the dWallet.

## Development Notes

### Mocks for Development
The MPC includes complex logic, some of which takes a significant amount of time to run in `dev` mode. To simplify development, you can use the following mocks:

#### Mocking the Class-Groups Key Generation
Each validator must generate a class groups key pair and proof, publish the public data, and store the decryption key.  
To bypass this process during development, you can mock the key generation by enabling the `mock-class-groups` feature on the `pera-core` crate.

#### Mocking the Network DKG Protocol
The MPC session requires the network DKG protocol to calculate the network MPC key. However, running the network DKG protocol currently takes a substantial amount of time (approximately 20 minutes in release mode).  
To bypass this process during development, you can mock the network DKG protocol by disabling the `with-network-dkg` feature on the `pera-core` crate.

### Generating the RPC API for TypeScript
To generate the RPC schema, follow these steps:
1. Run the main function from `crates/pera-open-rpc/src/generate_json_rpc_spec.rs`.
2. Then, run `pnpm tsx scripts/generate.ts` from the `sdk/typescript` directory.
