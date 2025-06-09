# Ika Network Genesis and Validator Management

This guide explains how to set up and manage an Ika network with multiple validators using Docker containers. Ika is a decentralized network that runs on top of the Sui blockchain, providing additional functionality and services.

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) installed and running
- [Sui full node](https://docs.sui.io/build/fullnode) running for your target environment (local, devnet, testnet, or mainnet)
- Access to the Ika node Docker image
- Sufficient system resources (CPU, RAM, and disk space) for running multiple validators
- Basic understanding of Docker and container management

## Initial Setup

### 1. Build the Docker Image
```bash
# From the project root directory
./docker/ika-node/build.sh
```

The build script will create a Docker image containing the Ika node software. This image will be used to run all nodes.

### 2. Configure Genesis
1. Update `create-ika-gensis-mac.sh` with your environment settings:
   - Set the appropriate domain constants for your environment:
     - `local`: For local development and testing
     - `devnet`: For development network testing
     - `testnet`: For pre-production testing
     - `mainnet`: For production deployment
   - Configure `VALIDATOR_NUM_TO_CREATE`: Total number of validators to configure and stake
   - Configure `VALIDATOR_NUM_TO_JOIN_COMMITTEE`: Total number of validators to join the committee

2. Run the genesis script:
```bash
./create-ika-gensis-mac.sh
```

This script will:
- Generate validator configurations
- Create necessary directories and files
- Set up initial network parameters

### 3. Start the Network
From within the SUBDOMAIN folder, run:
```bash
docker-compose up
```
This will start 4 Ika nodes (val1-4) and the Ika fullnode. The `docker-compose.yml` file defines the network configuration and dependencies between services.

## Managing Validators

### Adding Validators to Committee
After the initial setup is complete, you can add more validators to the committee.

#### Prerequisites for Joining
- Validator must be properly configured and staked
- Sufficient stake amount must be available
- Network must be in a stable state
- Validator must have proper network connectivity

#### Steps to Join Committee
1. Set the environment variables:
   - Set the appropriate domain constants for your environment:
     - `local`: For local development and testing
     - `devnet`: For development network testing
     - `testnet`: For pre-production testing
     - `mainnet`: For production deployment
   - Configure `VALIDATOR_NUM`: Total number of validators to add to the committee
   - Configure `FIRST_VALIDATOR_IN_SET`: First validator number in the set (should be VALIDATOR_NUM_TO_JOIN_COMMITTEE + 1)

2. Run the join script:
```bash
./join-committee.sh
```


### Removing Validators from Committee
To remove validators from the committee

#### Prerequisites for Leaving
- Validator must be active in the committee
- Network must be in a stable state

#### Steps to Leave Committee
1. Set the environment variables:
   - Set the appropriate domain constants for your environment:
     - `local`: For local development and testing
     - `devnet`: For development network testing
     - `testnet`: For pre-production testing
     - `mainnet`: For production deployment
   - Configure `VALIDATOR_NUM`: Total number of validators to remove
   - Configure `FIRST_VALIDATOR_IN_SET`: First validator number to remove from the set

2. Run the leave script:
```bash
./leave-committee.sh
```

## Running Individual Validator Nodes

To run a single validator node, use the following command from within the subdomain folder:

```bash
docker run -it \
  --name <validator_name>.localhost \
  --env RUST_BACKTRACE=1 \
  --env RUST_LOG=warn,ika_node=info,ika_core=info,ika_network::state_sync=error \
  --env RUST_MIN_STACK=16777216 \
  --label org.label-schema.name="<validator_name>" \
  --network localhost_default \
  -v ./<validator_name>.localhost/validator.yaml:/opt/ika/config/validator.yaml:ro \
  -v ./<validator_name>.localhost/key-pairs:/opt/ika/key-pairs/:ro \
  -v ./dbs/<validator_name>.<validator_name>.localhost/opt/ika/db:/opt/ika/db:rw \
  -v ./<validator_name>.localhost/sui_backup/sui_config/:/root/.sui/sui_config \
  -v ./ika_sui_config.json:/opt/ika/ika_sui_config.json:ro \
  us-docker.pkg.dev/common-449616/ika-common-containers/ika-node:devnet-v0.0.7-arm64 \
  /opt/ika/bin/ika-node --config-path /opt/ika/config/validator.yaml
```

Replace `<validator_name>` with your desired validator name (e.g., val6).

## Notes
- Make sure your Sui full node is running before starting the Ika network
- All paths and configurations should be adjusted according to your environment
- The Docker image version may need to be updated based on your requirements
- Monitor validator performance and logs for any issues
- Keep your validator keys secure and backed up
- Regular maintenance and updates are recommended

