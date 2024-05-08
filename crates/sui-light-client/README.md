This crate contains a Command Line Interface to manage the SUI light client for dWallet network.

## Init

Before init, it is required to set the correct `package_id` of the deployed `sui_light_client.move` package in
[light_client.yaml](example_config/light_client.yaml) -> `sui_deployed_state_proof_package` field.

Use checkpoint id to bootstrap the init process for the module - `0` syncs from Genesis.

```bash
$ cargo run --bin light-client -- --config example_config/light_client.yaml init --ckp-id 702225
```

## Sync

Every day, new checkpoints must be downloaded by synchronizing through the following process:

```bash
$ cargo run -- --config example_config/light_client.yaml sync
```

## Prove Tx

> This should be done with the Typescript SDK.

```bash
cargo run -- --config example_config/light_client.yaml transaction -t 7DefdfmiEvb9de6LSKdD99xY7syZGJ3RzkP7XxHxcgc
```
