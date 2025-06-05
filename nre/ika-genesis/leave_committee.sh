#!/bin/bash

set -e

command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check if jq is installed
if ! command_exists jq; then
    echo "jq is not installed, installing..."
    brew install jq
else
    echo "jq is already installed."
fi

# Check if yq is installed
if ! command_exists yq; then
    echo "yq is not installed, installing..."
    brew install yq
else
    echo "yq is already installed."
fi

# Default values.
# The prefix for the validator names (e.g. val1.devnet.ika.cloud, val2.devnet.ika.cloud, etc...).
export VALIDATOR_PREFIX="val"
# The number of validators to create.
export VALIDATOR_NUM=3
export FIRST_VALIDATOR_IN_SET=5
# The number of staked tokens for each validator.
export VALIDATOR_STAKED_TOKENS_NUM=40000000000000000
# The subdomain for Ika the network.
export SUBDOMAIN="localhost"
# export SUBDOMAIN="beta.devnet.ika-network.net"
# The binary name to use.
export BINARY_NAME="ika"
# The directory to store the key pairs.
export KEY_PAIRS_DIR="key-pairs"
# The root address for the genesis account (to hold all the tokens).
# In a testnet use the faucet public key.
ROOT_ADDR=""
# The file containing the validators (separator: newline).
export VALIDATORS_FILE=""
# Validator Docker image name.
export IMAGE_NAME="us-docker.pkg.dev/common-449616/ika-common-containers/ika-node:devnet-v0.0.7-arm64"
# SUI fullnode URL.
# export SUI_FULLNODE_RPC_URL="https://fullnode.sui.beta.devnet.ika-network.net"
export SUI_FULLNODE_RPC_URL="http://localhost:9000"
# Sui Docker URL (only needed if you run Ika on Docker against localhost on non-linux).
# If it's not against localhost, set it to the remote sui RPC.
export SUI_DOCKER_URL="http://docker.for.mac.localhost:9000"
# export SUI_DOCKER_URL="https://fullnode.sui.beta.devnet.ika-network.net"
# SUI Faucet URL.
# export SUI_FAUCET_URL="https://faucet.sui.beta.devnet.ika-network.net/gas"
export SUI_FAUCET_URL="http://localhost:9123/gas"
# Default Ika epoch duration time.
# export EPOCH_DURATION_TIME_MS=86400000
export EPOCH_DURATION_TIME_MS=2400000
# Sui chain identifier.
export SUI_CHAIN_IDENTIFIER="custom"
SUI_CONFIG_PATH=~/.sui/sui_config


# Function to display help message
show_help() {
    echo "Usage: $0 [options]"
    echo ""
    echo "This script sets up a genesis and config with given options."
    echo ""
    echo "Options:"
    echo "  --validator-prefix <prefix>         Set the prefix for validators. Default: $VALIDATOR_PREFIX"
    echo "  --validator-num <number>            Set the number of validators. Default: $VALIDATOR_NUM"
    echo "  --validator-staked-tokens-num <num>   Set the number of staked tokens. Default: $VALIDATOR_STAKED_TOKENS_NUM"
    echo "  --subdomain <subdomain>             Set the subdomain for validators. Default: $SUBDOMAIN"
    echo "  --binary-name <path>                Set the binary name path. Default: $PWD/ika"
    echo "  --key-pairs-dir <directory>         Set the directory for key pairs. Default: key-pairs"
    echo "  --root-addr <address>               Set the root address. Default: 0x3e..."
    echo "  --validators-file <file>            Specify a file with validators."
    echo "  --image-name <image>                Specify the Docker image name. Default: $IMAGE_NAME"
    echo "  --sui-faucet-url <url>              Set the SUI faucet URL. Default: $SUI_FAUCET_URL"
    echo "  --epoch-duration-time <time>        Set the epoch duration time. Default: $EPOCH_DURATION_TIME_MS"
    echo "  -h, --help                        Display this help message and exit."
    echo ""
    echo "Note: --validators-file overrides --validator-prefix and --validator-num."
}

# Parse named arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --validator-prefix) VALIDATOR_PREFIX="$2"; shift ;;
        --validator-num) VALIDATOR_NUM="$2"; shift ;;
        --validator-staked-tokens-num) VALIDATOR_STAKED_TOKENS_NUM="$2"; shift ;;
        --subdomain) SUBDOMAIN="$2"; shift ;;
        --binary-name) BINARY_NAME="$2"; shift ;;
        --key-pairs-dir) KEY_PAIRS_DIR="$2"; shift ;;
        --root-addr) ROOT_ADDR="$2"; shift ;;
        --validators-file) VALIDATORS_FILE="$2"; shift ;;
        --image-name) IMAGE_NAME="$2"; shift ;;
        --sui-faucet-url) SUI_FAUCET_URL="$2"; shift ;;
        --epoch-duration-time) EPOCH_DURATION_TIME_MS="$2"; shift ;;
        -h|--help) show_help; exit 0 ;;
        *) echo "Unknown parameter passed: $1"; exit 1 ;;
    esac
    shift
done


RUST_MIN_STACK=16777216

RUST_MIN_STACK=$RUST_MIN_STACK cargo build --release --bin "$BINARY_NAME"
cp ../../target/release/"$BINARY_NAME" .
BINARY_NAME="$(pwd)/$BINARY_NAME"

VALIDATORS_ARRAY=()

echo "Creating validators from prefix '$VALIDATOR_PREFIX' and number '$VALIDATOR_NUM'"

for ((i=FIRST_VALIDATOR_IN_SET; i<VALIDATOR_NUM + FIRST_VALIDATOR_IN_SET; i++)); do

    VALIDATOR_NAME="${VALIDATOR_PREFIX}${i}"
    # For enumerated list, compute the hostname as: name.SUBDOMAIN
    VALIDATOR_HOSTNAME="${VALIDATOR_NAME}.${SUBDOMAIN}"
    echo "Generated validator: Name = $VALIDATOR_NAME, Hostname = $VALIDATOR_HOSTNAME"
    VALIDATORS_ARRAY+=("$VALIDATOR_NAME:$VALIDATOR_HOSTNAME")
done

pushd "$SUBDOMAIN"

SUI_BACKUP_DIR="sui_backup"

export PUBLISHER_DIR=publisher

PUBLISHER_CONFIG_FILE="$PUBLISHER_DIR/ika_config.json"

IKA_PACKAGE_ID=$(jq -r '.ika_package_id' "$PUBLISHER_CONFIG_FILE")
IKA_SYSTEM_PACKAGE_ID=$(jq -r '.ika_system_package_id' "$PUBLISHER_CONFIG_FILE")
IKA_SYSTEM_OBJECT_ID=$(jq -r '.ika_system_object_id' "$PUBLISHER_CONFIG_FILE")

# Print the values for verification.
echo "IKA Package ID: $IKA_PACKAGE_ID"
echo "IKA System Package ID: $IKA_SYSTEM_PACKAGE_ID"
echo "System ID: $IKA_SYSTEM_OBJECT_ID"

# Array to store validator tuples
VALIDATOR_TUPLES=()
TMP_OUTPUT_DIR="/tmp/become_candidate_outputs"
TUPLES_FILE="$TMP_OUTPUT_DIR/tuples.txt"

# Read tuples file after all jobs complete
if [[ -f "$TUPLES_FILE" ]]; then
    while IFS= read -r tuple; do
        VALIDATOR_TUPLES+=("$tuple")
    done < "$TUPLES_FILE"
else
    echo "[ERROR] Tuples file not found: $TUPLES_FILE"
fi

# Summary
echo
echo "✅ All validator tuples:"
for tup in "${VALIDATOR_TUPLES[@]}"; do
    echo "  $tup"
done

############################
# Leave Committee
############################

for tuple in "${VALIDATOR_TUPLES[@]}"; do
    IFS=":" read -r VALIDATOR_NAME VALIDATOR_ID VALIDATOR_CAP_ID <<< "$tuple"

    # Find the validator's hostname based on its name
    # Just the first validator for now.
    for entry in "${VALIDATORS_ARRAY[@]0:1}"; do
        IFS=":" read -r NAME HOSTNAME <<< "$entry"
        if [[ "$NAME" == "$VALIDATOR_NAME" ]]; then
            echo "Debug: Processing validator: $VALIDATOR_NAME with ID: $VALIDATOR_ID and Cap ID: $VALIDATOR_CAP_ID"

            VALIDATOR_HOSTNAME="$HOSTNAME"
            echo "Debug: Found hostname: $VALIDATOR_HOSTNAME"

            # Copy sui_config and run leave-committee
            VALIDATOR_DIR="$VALIDATOR_HOSTNAME"
            rm -rf "$SUI_CONFIG_PATH"
            echo "Debug: Removing $SUI_CONFIG_PATH"
            mkdir -p "$SUI_CONFIG_PATH"
            echo "Debug: Creating $SUI_CONFIG_PATH"
            echo "Debug: Copying sui_config from $VALIDATOR_DIR/$SUI_BACKUP_DIR/sui_config/ to $SUI_CONFIG_PATH"
            cp -r "$VALIDATOR_DIR/$SUI_BACKUP_DIR/sui_config/"* "$SUI_CONFIG_PATH"

            echo "Leaving committee for Validator '$VALIDATOR_NAME' (Cap ID: $VALIDATOR_CAP_ID)"

            $BINARY_NAME validator leave-committee \
                --validator-cap-id "$VALIDATOR_CAP_ID"
            break
        fi
    done
done
