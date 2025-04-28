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
pera client faucet
pera client call --package 0x3 --module pera_system --function request_start_network_dkg --args 1 0x5 --gas-budget 1000000000
```

### Mocks for Development

The MPC includes complex logic, some of which takes a significant amount of time to run in `dev` mode.
To simplify development, you can use the following mocks:

#### Mocking the Class-Groups Key Generation

Each validator must generate a Class Groups key pair and proof of validity,
publish the public data (encryption key + proof), and store the decryption key.  
To bypass this process during development, you can mock the key generation by enabling the `mock-class-groups` feature
on `ika-swarm-config` crate.

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
