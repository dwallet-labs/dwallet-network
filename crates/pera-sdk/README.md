This crate provides the Pera Rust SDK, containing APIs to interact with the Pera network. Auto-generated documentation for this crate is [here](https://mystenlabs.github.io/pera/pera_sdk/index.html).

## Getting started

Add the `pera-sdk` dependency as following:

```toml
pera_sdk = { git = "https://github.com/mystenlabs/sui", package = "pera-sdk"}
tokio = { version = "1.2", features = ["full"] }
anyhow = "1.0"
```

The main building block for the Pera Rust SDK is the `PeraClientBuilder`, which provides a simple and straightforward way of connecting to a Pera network and having access to the different available APIs.

In the following example, the application connects to the Pera `testnet` and `devnet` networks and prints out their respective RPC API versions.

```rust
use pera_sdk::PeraClientBuilder;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Pera testnet -- https://fullnode.testnet.pera.io:443
    let pera_testnet = PeraClientBuilder::default().build_testnet().await?;
    println!("Pera testnet version: {}", pera_testnet.api_version());

     // Pera devnet -- https://fullnode.devnet.pera.io:443
    let pera_devnet = PeraClientBuilder::default().build_devnet().await?;
    println!("Pera devnet version: {}", pera_devnet.api_version());

    Ok(())
}

```

## Documentation for pera-sdk crate

[GitHub Pages](https://mystenlabs.github.io/pera/pera_sdk/index.html) hosts the generated documentation for all Rust crates in the Pera repository.

### Building documentation locally

You can also build the documentation locally. To do so,

1. Clone the `pera` repo locally. Open a Terminal or Console and go to the `pera/crates/pera-sdk` directory.

1. Run `cargo doc` to build the documentation into the `pera/target` directory. Take note of location of the generated file from the last line of the output, for example `Generated /Users/foo/pera/target/doc/pera_sdk/index.html`.

1. Use a web browser, like Chrome, to open the `.../target/doc/pera_sdk/index.html` file at the location your console reported in the previous step.

## Rust SDK examples

The [examples](https://github.com/MystenLabs/sui/tree/main/crates/pera-sdk/examples) folder provides both basic and advanced examples.

There are serveral files ending in `_api.rs` which provide code examples of the corresponding APIs and their methods. These showcase how to use the Pera Rust SDK, and can be run against the Pera testnet. Below are instructions on the prerequisites and how to run these examples.

### Prerequisites

Unless otherwise specified, most of these examples assume `Rust` and `cargo` are installed, and that there is an available internet connection. The examples connect to the Pera testnet (`https://fullnode.testnet.pera.io:443`) and execute different APIs using the active address from the local wallet. If there is no local wallet, it will create one, generate two addresses, set one of them to be active, and it will request 1 PERA from the testnet faucet for the active address.

### Running the existing examples

In the root folder of the `pera` repository (or in the `pera-sdk` crate folder), you can individually run examples using the command  `cargo run --example filename` (without `.rs` extension). For example:
* `cargo run --example pera_client` -- this one requires a local Pera network running (see [here](#Connecting to Pera Network
)). If you do not have a local Pera network running, please skip this example.
* `cargo run --example coin_read_api`
* `cargo run --example event_api` -- note that this will subscribe to a stream and thus the program will not terminate unless forced (Ctrl+C)
* `cargo run --example governance_api`
* `cargo run --example read_api`
* `cargo run --example programmable_transactions_api`
* `cargo run --example sign_tx_guide`

### Basic Examples

#### Connecting to Pera Network
The `PeraClientBuilder` struct provides a connection to the JSON-RPC server that you use for all read-only operations. The default URLs to connect to the Pera network are:

- Local: http://127.0.0.1:9000
- Devnet: https://fullnode.devnet.pera.io:443
- Testnet: https://fullnode.testnet.pera.io:443
- Mainnet: https://fullnode.mainnet.pera.io:443

For all available servers, see [here](https://pera.io/networkinfo).

For running a local Pera network, please follow [this guide](https://docs.pera.io/build/pera-local-network) for installing Pera and [this guide](https://docs.pera.io/build/pera-local-network#start-the-local-network) for starting the local Pera network.


```rust
use pera_sdk::PeraClientBuilder;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let pera = PeraClientBuilder::default()
        .build("http://127.0.0.1:9000") // local network address
        .await?;
    println!("Pera local network version: {}", pera.api_version());

    // local Pera network, like the above one but using the dedicated function
    let pera_local = PeraClientBuilder::default().build_localnet().await?;
    println!("Pera local network version: {}", pera_local.api_version());

    // Pera devnet -- https://fullnode.devnet.pera.io:443
    let pera_devnet = PeraClientBuilder::default().build_devnet().await?;
    println!("Pera devnet version: {}", pera_devnet.api_version());

    // Pera testnet -- https://fullnode.testnet.pera.io:443
    let pera_testnet = PeraClientBuilder::default().build_testnet().await?;
    println!("Pera testnet version: {}", pera_testnet.api_version());

    Ok(())
}
```

#### Read the total coin balance for each coin type owned by this address
```rust
use std::str::FromStr;
use pera_sdk::types::base_types::PeraAddress;
use pera_sdk::{ PeraClientBuilder};
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {

   let pera_local = PeraClientBuilder::default().build_localnet().await?;
   println!("Pera local network version: {}", pera_local.api_version());

   let active_address = PeraAddress::from_str("<YOUR PERA ADDRESS>")?; // change to your Pera address

   let total_balance = pera_local
      .coin_read_api()
      .get_all_balances(active_address)
      .await?;
   println!("The balances for all coins owned by address: {active_address} are {}", total_balance);
   Ok(())
}
```

## Advanced examples

See the programmable transactions [example](https://github.com/MystenLabs/sui/blob/main/crates/pera-sdk/examples/programmable_transactions_api.rs).

## Games examples

### Tic Tac Toe quick start

1. Prepare the environment
   1. Install `pera` binary following the [Pera installation](https://github.com/MystenLabs/sui/blob/main/docs/content/guides/developer/getting-started/pera-install.mdx) docs.
   1. [Connect to Pera Devnet](https://github.com/MystenLabs/sui/blob/main/docs/content/guides/developer/getting-started/connect.mdx).
   1. [Make sure you have two addresses with gas](https://github.com/MystenLabs/sui/blob/main/docs/content/guides/developer/getting-started/get-address.mdx) by using the `new-address` command to create new addresses:
      ```shell
      pera client new-address ed25519
      ```
      You must specify the key scheme, one of `ed25519` or `secp256k1` or `secp256r1`.
      You can skip this step if you are going to play with a friend. :)
   1. [Request Pera tokens](https://github.com/MystenLabs/sui/blob/main/docs/content/guides/developer/getting-started/get-coins.mdx) for all addresses that will be used to join the game.

2. Publish the move contract
   1. [Download the Pera source code](https://github.com/MystenLabs/sui/blob/main/docs/content/guides/developer/getting-started/pera-install.mdx).
   1. Publish the [`tic-tac-toe` package](https://github.com/MystenLabs/sui/tree/main/examples/tic-tac-toe/move)
      using the Pera client:
      ```shell
      pera client publish --path /path-to-pera-source-code/examples/tic-tac-toe/move
      ```
   1. Record the package object ID.

3. Create a new tic-tac-toe game
   1. Run the following command in the [`tic-tac-toe/cli` directory](https://github.com/MystenLabs/sui/tree/main/examples/tic-tac-toe/cli) to start a new game, replacing the game package objects ID with the one you recorded:
      ```shell
      cargo run -- new --package-id <<tic-tac-toe package object ID>> <<player O address>>
      ```
      This will create a game between the active address in the keystore, and the specified Player O.
   1. Copy the game ID and pass it to your friend to join the game.

4. Making a move

   Run the following command in the [`tic-tac-toe/cli` directory](https://github.com/MystenLabs/sui/tree/main/examples/tic-tac-toe/cli) to make a move in an existing game, as the active address in the CLI, replacing the game ID and address accordingly:
   ```shell
   cargo run -- move --package-id <<tic-tac-toe package object ID>> --row $R --col $C <<game ID>>
   ```

## License

[SPDX-License-Identifier: BSD-3-Clause-Clear](https://github.com/MystenLabs/sui/blob/main/LICENSE)
