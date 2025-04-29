# dWallet MPC

This module contains the core logic of the MPC protocol for the dWallet.

## Development Notes

### Running the Network DKG Protocol

The dWallet "admin" is the address that can initiate the network DKG.
To run the network DKG protocol,
you must configure your address as the dWallet admin address and execute the start transaction.

#### **OUTDATED** Configuring the dWallet "admin" Address

There are two ways to configure the admin address:

1. Run `pera genesis-ceremony init` and modify the admin address.
2. Update it directly in the code at `crates/pera-config/src/genesis.rs:L463`.

#### **OUTDATED** Executing the Start Transaction

Run the following commands:

```bash
ika client faucet
ika client call --package 0x3 --module pera_system --function request_start_network_dkg --args 1 0x5 --gas-budget 1000000000
```

### Mocks for Development

The MPC includes complex logic, some of which takes a significant amount of time to run in `dev` mode.
To simplify development, you can use the following mocks:

#### Mocking the Class-Groups Key Generation

Each validator must generate a Class Groups key pair and proof of validity,
publish the public data (encryption key + proof), and store the decryption key.  
To bypass this process during development, you can mock the key generation by enabling the `mock-class-groups` feature
on `ika-swarm-config` crate.

#### Mocking the Network DKG Protocol

The MPC session requires the network DKG protocol to calculate the network MPC key.
However, running the network DKG protocol currently takes a significant amount of time (approximately 20 minutes in
`release` mode).  
To bypass this process during development, you can mock the network DKG protocol by disabling the `with-network-dkg`
feature on the `ika-core` crate.

### **OUTDATED** Generating the RPC API for TypeScript

To generate the RPC schema, follow these steps:

1. Run the `main` function from `crates/pera-open-rpc/src/generate_json_rpc_spec.rs`.
2. Then, run `pnpm tsx scripts/generate.ts` from the `sdk/typescript` directory.

## Running a local blockchain from the IDE

1. **Create a Debug Configuration**:
    - Open "Run/Debug Configurations" in your IDE (IntelliK/RustRover) and create a new configuration.

2. **Set Command**:

   ```bash
   run --package ika --bin ika -- start --force-reinitiation
   ```

3. **Enable Options**:
    - Check:
        - **Automatically add required features if possible**
        - **Emulate terminal in output console**

4. **Environment Variables**:
   ```bash
   RUST_LOG=off,ika_node=info,ika_core=info,sui_node=info;RUST_MIN_STACK=16777216
   ```

5. **Working Directory**:
   ```plaintext
   /<PATH_TO_PROJECT-DIR>
   ```

6. **Run Sui Locally**:  
   From your terminal, run the following command:
   ```bash
    RUST_LOG="off,sui_node=info" sui start --with-faucet --force-regenesis --epoch-duration-ms 18000000
   ```

7. **Run**: Select the configuration and click **Run** or **Debug**.

## Running a local blockchain from the CLI

**Run Sui Locally:**  
From your terminal, run the following command:

```bash
RUST_LOG="off,sui_node=info" sui start --with-faucet --force-regenesis --epoch-duration-ms 18000000
```

**Run Ika:**

```bash
RUST_LOG="off,pera_node=info,pera_core=error" RUST_MIN_STACK=16777216 cargo run --bin ika -- start --force-reinitiation
```

## **OUTDATED** Testing the State Sync Mechanism Manually

To test the state sync feature, uncomment the code in the `start` function located in
`crates/pera/src/pera_commands.rs`.
This code restarts a validator node 10 seconds after the chain starts.

## Create Mock Data for the MPC Protocols

Clone the `cryptography-private` repository and check out the `ika-print-tests` branch.  
This branch contains code to store the Secp256K1 network DKG output into files.  
The files are generated while running the `dkg::tests::generates_distributed_key_secp256k1` test.

Before running the test, make sure to switch to `release` mode to avoid long execution times.  
The current test configuration is set to run with **four parties** and a **threshold of three**, with each party having
a voting power of **one**.

### Running the Test

To run the test from IntelliJ IDE, use the following debug configuration:

- **Command:**
  ```sh
  test --all-features --package class_groups --lib dkg::tests::generates_distributed_key_secp256k1 -- --nocapture
  ```
- **Environment variables:**
  ```sh
  RUST_MIN_STACK=16777216
  ```

### Output Files

After running the test, the generated files will be stored in the `cryptography-private/class_groups` directory:

- **`class-groups-keypair`**: Contains the key pair for class groups, used by all parties in the network DKG for
  encryption and decryption of secret shares.
- **`decryption_key_share_public_parameters.txt`**: Stores the decryption public parameters used in the signing
  protocol.
- **`decryption_shares.txt`**: Contains the decryption shares for all parties participating in the network DKG.
- **`encryption_scheme_public_parameters.txt`**: Includes the encryption scheme's public parameters used for all MPC
  protocols.
- **`public_output.txt`**: Stores the public output of the network DKG.

### Updating the Project with Mock Data

Replace the mock values of the following constants with the corresponding file contents:

- **`DECRYPTION_KEY_SHARE_PUBLIC_PARAMETERS`** → `decryption_key_share_public_parameters.txt`
- **`NETWORK_DKG_OUTPUT`** → `public_output.txt`
- **`DECRYPTION_SHARES`** → `decryption_shares.txt`

These constants are located in the `crates/shared-wasm-class-groups/src/constants.rs` file.

Next, replace the contents of `class-groups-keys-mock-files/class-groups-mock-key-full` with the content of
`class-groups-keypair`.

Finally, update the `mockedProtocolPublicParameters` in `sdk/typescript/src/dwallet-mpc/globals.ts` with the content of
`encryption_scheme_public_parameters.txt`.  
