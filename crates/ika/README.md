# Ika Validator CLI

## Overview

The `ika validator` CLI tool is designed for validators and validator candidates to manage their participation in the
Ika network. It provides commands to create validator information, become a candidate, join the committee, stake tokens,
and leave the committee.

## Usage

The CLI follows the structure:

```sh
ika validator <COMMAND> [OPTIONS]
```

### Available Commands

#### `make-validator-info`

Creates a validator information file containing necessary details to become a validator.

##### Arguments:

- `name` (String) - Validator name
- `description` (String) - Description of the validator
- `image_url` (String) - URL to the validator's image
- `project_url` (String) - Project website URL
- `host_name` (String) - Validator's host name
- `gas_price` (u64) - Gas price for transactions
- `sender_sui_address` (SuiAddress) - Address of the sender

##### Example:

```sh
ika validator make-validator-info --name "My Validator" --description "Secure and fast" --image_url "https://example.com/image.png" --project_url "https://example.com" --host_name "x.x.x.x" --gas_price 1000000 --sender_sui_address 0x1234...
```

#### `become-candidate`

Registers a validator candidate using a validator info file.

##### Arguments:

- `--validator-info-path` (PathBuf) - Path to the validator information file
- `--gas-budget` (Optional) - Gas budget for the transaction
- `--ika-sui-config` (Optional) - Path to the Ika system package network file

##### Example:

```sh
ika validator become-candidate --validator-info-path validator.info --gas-budget 200000000
```

#### `join-committee`

Requests to join the validator committee.

##### Arguments:

- `--gas-budget` (Optional) - Gas budget for the transaction
- `--ika-sui-config` (Optional) - Path to the Ika system network configuration file
- `--validator-cap-id` (ObjectID) - ID of the validator capability

##### Example:

```sh
ika validator join-committee --validator-cap-id 0x5678
```

#### `stake-validator`

Stake IKA tokens to a validator.

##### Arguments:

- `--gas-budget` (Optional) - Gas budget for the transaction
- `--ika-sui-config` (Optional) - Path to the Ika system network configuration file
- `--validator-id` (ObjectID) - Validator ID to stake to
- `--ika-coin-id` (ObjectID) - ID of the IKA coin being staked
- `--stake-amount` (u64) - Amount of IKA tokens to stake

##### Example:

```sh
ika validator stake-validator --validator-id 0x1234 --ika-coin-id 0x5678 --stake-amount 1000000
```

#### `leave-committee`

Requests to leave the validator committee.

##### Arguments:

- `--gas-budget` (Optional) - Gas budget for the transaction
- `--validator-cap-id` (ObjectID) - ID of the validator capability
- `--ika-sui-config` (Optional) - Path to the Ika system network configuration file

##### Example:

```sh
ika validator leave-committee --validator-cap-id 0x5678
```
