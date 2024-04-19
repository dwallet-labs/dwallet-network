This crate contains a Command Line Interface to manage the SUI/ dWallet network lightclient.

## Init

Before init, needed to set the correct package_id of the deployed sui_light_client.move package in example_config/light_client.yaml -> sui_deployed_state_proof_package

Use checkpoint id from where to init the module, 0 syncs from Genesis.

```
$ cargo run --bin light-client -- --config example_config/light_client.yaml init --ckp-id 702225
```

## Sync

Every day there is a need to download new checkpoints through sync by doing:

```
$ cargo run -- --config example_config/light_client.yaml sync
```

## Prove Tx

This should be done over the TS SDK.

```
cargo run -- --config example_config/light_client.yaml transaction -t 7DefdfmiEvb9de6LSKdD99xY7syZGJ3RzkP7XxHxcgc
```
