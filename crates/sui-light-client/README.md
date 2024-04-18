This crate contains a Command Line Interface to manage the SUI/ dWallet network lightclient.

## Init

```
$ cargo run -- --config example_config/light_client.yaml init
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
