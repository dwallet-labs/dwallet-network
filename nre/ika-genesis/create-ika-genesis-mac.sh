#!/bin/bash

set -ex

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
export VALIDATOR_NUM=4
# The number of staked tokens for each validator.
export VALIDATOR_STAKED_TOKENS_NUM=40000000000000000
# The subdomain for the network.
export SUBDOMAIN="devnet.ika.cloud"
# The binary name to use.
export BINARY_NAME="ika"
# The directory to store the key pairs.
export KEY_PAIRS_DIR="key-pairs"
# The root address for the genesis account (to hold all the tokens).
# in a testnet use the faucet public key.
ROOT_ADDR=""
# The file containing the validators (separator: newline).
VALIDATORS_FILE=""
# Validator Docker image name.
IMAGE_NAME="471930101563.dkr.ecr.us-east-1.amazonaws.com/sui-fork:ika-testnet-v1.16.2-10"
# Faucet Image.
SUI_IMAGE_NAME="471930101563.dkr.ecr.us-east-1.amazonaws.com/sui-fork:ika-testnet-faucet-v1.16.2"
# Default sui epoch duration time.
EPOCH_DURATION_TIME=86400000

# Function to display help message
show_help() {
    echo "Usage: $0 [options]"
    echo ""
    echo "This script sets up a genesis and config with given options."
    echo ""
    echo "Options:"
    echo "  --validator-prefix <prefix>       Set the prefix for validators. Default: $VALIDATOR_PREFIX"
    echo "  --validator-num <number>          Set the number of validators. Default: $VALIDATOR_NUM"
    echo "  --validator-staked-tokens-num <number> Set the number of staked tokens. Default: $VALIDATOR_STAKED_TOKENS_NUM"
    echo "  --subdomain <subdomain>           Set the subdomain for validators. Default: $SUBDOMAIN"
    echo "  --binary-name <path>              Set the binary name path. Default: $PWD/ika"
    echo "  --key-pairs-dir <directory>       Set the directory for key pairs. Default: key-pairs"
    echo "  --root-addr <address>             Set the root address. Default: 0x3e..."
    echo "  --validators-file <file>          Specify a file with validators."
    echo "  --image-name <file>               Specify the Docker image name. Default: $IMAGE_NAME"
    echo "  --faucet-image-name <file>        Specify the Faucet Docker image name. Default: $SUI_IMAGE_NAME"
    echo "  --epoch-duration-time <time>      Set the epoch duration time. Default: $EPOCH_DURATION_TIME"
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
        --faucet-image-name) SUI_IMAGE_NAME="$2"; shift ;;
        --epoch-duration-time) EPOCH_DURATION_TIME="$2"; shift ;;
        -h|--help) show_help; exit 0 ;;
        *) echo "Unknown parameter passed: $1"; exit 1 ;;
    esac
    shift
done


BINARY_NAME="$(pwd)/$BINARY_NAME"

VALIDATORS_ARRAY=()

# Check if --validators-file is provided and process it.
if [[ -n "$VALIDATORS_FILE" ]]; then
    echo "Creating validators from file: $VALIDATORS_FILE"

    if [[ ! -f "$VALIDATORS_FILE" ]]; then
        echo "Error: File '$VALIDATORS_FILE' not found." >&2
        exit 1
    fi

    while IFS= read -r line || [[ -n "$line" ]]; do
        # Skip empty lines.
        [[ -z "$line" ]] && continue

        # Split the line into two parts: validator name and hostname.
        read -r name hostname <<< "$line"

        # Trim any extra whitespace.
        name=$(echo "$name" | xargs)
        hostname=$(echo "$hostname" | xargs)

        # Append the tuple as "name:hostname" to the array.
        VALIDATORS_ARRAY+=("$name:$hostname")
    done < "$VALIDATORS_FILE"

    # Debugging: Print the array content to verify.
    for entry in "${VALIDATORS_ARRAY[@]}"; do
        IFS=":" read -r v_name v_hostname <<< "$entry"
        echo "Processed validator: Name = $v_name, Hostname = $v_hostname"
    done

    VALIDATOR_NUM=${#VALIDATORS_ARRAY[@]}
else
    echo "Creating validators from prefix '$VALIDATOR_PREFIX' and number '$VALIDATOR_NUM'"

    for ((i=1; i<=VALIDATOR_NUM; i++)); do
        VALIDATOR_NAME="${VALIDATOR_PREFIX}${i}"
        # For enumerated list, compute the hostname as: name.SUBDOMAIN
        VALIDATOR_HOSTNAME="${VALIDATOR_NAME}.${SUBDOMAIN}"
        echo "Generated validator: Name = $VALIDATOR_NAME, Hostname = $VALIDATOR_HOSTNAME"
        VALIDATORS_ARRAY+=("$VALIDATOR_NAME:$VALIDATOR_HOSTNAME")
    done
fi

##
## Create a dir for this deployment.
##
rm -rf "$SUBDOMAIN"
mkdir -p "$SUBDOMAIN"
pushd "$SUBDOMAIN"

##
## Create Validators
##
SUI_BACKUP_DIR="sui_backup"

for entry in "${VALIDATORS_ARRAY[@]}"; do
    # Split the tuple "name:hostname" into VALIDATOR_NAME and VALIDATOR_HOSTNAME.
    IFS=":" read -r VALIDATOR_NAME VALIDATOR_HOSTNAME <<< "$entry"

    # Use the VALIDATOR_HOSTNAME as the directory name.
    VALIDATOR_DIR="${VALIDATOR_HOSTNAME}"
    echo "Creating directory structure for validator '$VALIDATOR_NAME' with hostname '$VALIDATOR_HOSTNAME'"

    # Create validator directory and backup directory.
    mkdir -p "$VALIDATOR_DIR/$SUI_BACKUP_DIR"
    SUI_CONFIG_PATH=~/.sui/sui_config

    # Recreate the sui config for each validator.
    rm -rf $SUI_CONFIG_PATH
    mkdir -p $SUI_CONFIG_PATH

    VALIDATOR_ACCOUNT_KEY_FILE=${VALIDATOR_HOSTNAME}.account.json
    SUI_TEMPLATE_DIR=../sui-template
    SUI_CLIENT_YAML_FILE=client.yaml
    SUI_KEYSTORE_FILE=sui.keystore
    SUI_ALIASES_FILE=sui.aliases
    cp $SUI_TEMPLATE_DIR/sui.keystore.template "$SUI_CONFIG_PATH/$SUI_KEYSTORE_FILE"
    cp $SUI_TEMPLATE_DIR/client.template.yaml "$SUI_CONFIG_PATH/$SUI_CLIENT_YAML_FILE"
    cp $SUI_TEMPLATE_DIR/sui.aliases.template.json "$SUI_CONFIG_PATH/$SUI_ALIASES_FILE"
    pushd $SUI_CONFIG_PATH
    sui keytool generate ed25519 "m/44'/784'/0'/0'/0'" word24 --json > "$VALIDATOR_ACCOUNT_KEY_FILE"
    SUI_ADDR=$(jq -r '.suiAddress' "$VALIDATOR_ACCOUNT_KEY_FILE")
    MNEMONIC=$(jq -r '.mnemonic' "$VALIDATOR_ACCOUNT_KEY_FILE")
    sui keytool import "$MNEMONIC" ed25519 "m/44'/784'/0'/0'/0'"

    # Fetch the alias and change it (the --alias option is not working currently)
    SUI_CURRENT_ALIAS=$(jq -r '.[].alias' sui.aliases)
    sui keytool update-alias  "$SUI_CURRENT_ALIAS" "$VALIDATOR_NAME"
    yq e -i ".envs[].alias = \"$SUBDOMAIN\"" "$SUI_CLIENT_YAML_FILE"
    yq e -i ".envs[].rpc = \"https://fullnode.$SUBDOMAIN:443\"" "$SUI_CLIENT_YAML_FILE"
    yq e -i ".active_address = \"$SUI_ADDR\"" "$SUI_CLIENT_YAML_FILE"
    yq e -i ".active_env = \"$SUBDOMAIN\"" "$SUI_CLIENT_YAML_FILE"
    popd
    cp -r $SUI_CONFIG_PATH "$VALIDATOR_DIR/$SUI_BACKUP_DIR"
    SENDER_SUI_ADDR=$SUI_ADDR

    # Create Validator info.
    pushd "$VALIDATOR_DIR" > /dev/null
    # todo(zeev): remove this later
    cp ../../class-groups.key .

    # Usage: {binary_name} validator make-validator-info <NAME> <DESCRIPTION> <IMAGE_URL> <PROJECT_URL> <HOST_NAME> <GAS_PRICE> <sender_sui_address>
    $BINARY_NAME validator make-validator-info "$VALIDATOR_NAME" "$VALIDATOR_NAME" "" "" "$VALIDATOR_HOSTNAME" 0 "$SENDER_SUI_ADDR"

    mkdir -p "$KEY_PAIRS_DIR"
    mv protocol.key network.key "$KEY_PAIRS_DIR"/
    popd > /dev/null
    sui keytool list
done


#declare -a ACCOUNT_TOKENS_DISTRIBUTIONS
#declare -a ACCOUNT_ADDRESSES
#declare -a P2P_ADDRESSES
#declare -a NETWORK_ADDRESSES
#
## Add Validators to genesis.
## Update each validator.info and committee/validator.info file.
## Create the validator.yaml file.
#for VALIDATOR_NAME in "${VALIDATORS_ARRAY[@]}"; do
#      VALIDATOR_DIR="${VALIDATOR_NAME}.${SUBDOMAIN}"
#
#      NETWORK_ADDR=$(yq e '.info["network-address"]' "${VALIDATOR_DIR}"/validator.info)
#      # P2P_ADDR=$(yq eval '.info.p2p-address' "$INFO_FILE")
#      P2P_ADDR=$(yq e '.info["p2p-address"]' "${VALIDATOR_DIR}"/validator.info)
#      NARWHAL_PRIMARY_ADDR=$(yq e '.info["narwhal-primary-address"]' "${VALIDATOR_DIR}"/validator.info)
#      NARWHAL_WORKER_ADDR=$(yq e '.info["narwhal-worker-address"]' "${VALIDATOR_DIR}"/validator.info)
#
#      # Add Validator to genesis
#      $BINARY_NAME genesis-ceremony add-validator --name "$VALIDATOR_NAME" --validator-key-file "$VALIDATOR_DIR/$KEY_PAIRS_DIR/protocol.key" --worker-key-file "$VALIDATOR_DIR/$KEY_PAIRS_DIR/worker.key" --account-key-file "$VALIDATOR_DIR/$KEY_PAIRS_DIR/account.key" --network-key-file "$VALIDATOR_DIR/$KEY_PAIRS_DIR/network.key" --network-address "$NETWORK_ADDR" --p2p-address "$P2P_ADDR" --narwhal-primary-address "$NARWHAL_PRIMARY_ADDR" --narwhal-worker-address "$NARWHAL_WORKER_ADDR" --description "$VALIDATOR_NAME" --image-url "" --project-url ""
#
#      INFO_FILE="committee/$VALIDATOR_NAME"
#      ACCOUNT_ADDRESS=$(yq eval '.info.account-address' "$INFO_FILE")
#      ACCOUNT_TOKENS_DISTRIBUTIONS+=("$ROOT_ADDR,$VALIDATOR_STAKED_TOKENS_NUM,$ACCOUNT_ADDRESS")
#      ACCOUNT_ADDRESSES+=("$ACCOUNT_ADDRESS")
#      P2P_ADDRESSES+=("$P2P_ADDR")
#      NETWORK_ADDRESSES+=("$(yq eval '.info.network-key' "$INFO_FILE")")
#      cp ../validator.template.yaml "$VALIDATOR_DIR"/validator.yaml
#       # Update the validator.yaml file.
#      yq e ".p2p-config.external-address = \"$P2P_ADDR\"" -i "$VALIDATOR_DIR"/validator.yaml
#done
#
## List the validators.
#$BINARY_NAME genesis-ceremony list-validators
#
## Example command to modify the epoch_duration_ms in a YAML file using yq
#yq eval ".epoch_duration_ms = $EPOCH_DURATION_TIME" -i parameters
#
## Build the default token-distribution-schedule.
#$BINARY_NAME genesis-ceremony build-unsigned-checkpoint
#
## Copy the token-distribution-schedule.
#mv token-distribution-schedule token-distribution-schedule-backup
#
#GENESIS_ACCOUNT_UNSTAKED_TOKENS=$(echo "10000000000000000000 - $VALIDATOR_NUM * $VALIDATOR_STAKED_TOKENS_NUM" | bc)
#
## Create a custom `token-distribution-schedule`.
#{
#  echo "recipient-address,amount-mist,staked-with-validator"
#  for address in "${ACCOUNT_TOKENS_DISTRIBUTIONS[@]}"; do
#    echo "$address"
#  done
#  echo "$ROOT_ADDR,$GENESIS_ACCOUNT_UNSTAKED_TOKENS,"
#  echo "0x0000000000000000000000000000000000000000000000000000000000000000,0,"
#} > token-distribution-schedule
#
## Remove the old unsigned genesis
#rm unsigned-genesis
#
## Create the new unsigned genesis
#$BINARY_NAME genesis-ceremony build-unsigned-checkpoint
#
## Sign for each validator
#for VALIDATOR_NAME in "${VALIDATORS_ARRAY[@]}"; do
#  VALIDATOR_DIR="${VALIDATOR_NAME}.${SUBDOMAIN}"
#  $BINARY_NAME genesis-ceremony verify-and-sign --key-file ./"$VALIDATOR_DIR"/"$KEY_PAIRS_DIR"/protocol.key
#done
#
#$BINARY_NAME genesis-ceremony finalize
#
## Copy genesis blob to each validator
#for VALIDATOR_NAME in "${VALIDATORS_ARRAY[@]}"; do
#  VALIDATOR_DIR="${VALIDATOR_NAME}.${SUBDOMAIN}"
#  cp genesis.blob "$VALIDATOR_DIR"
#done
#
## Create the `fullnode.yaml` file
#export FULL_NODE_DIR=fullnode
#mkdir -p "$FULL_NODE_DIR"
#cp genesis.blob "$FULL_NODE_DIR"
#cp ../fullnode.template.yaml fullnode.yaml
#
#mv fullnode.yaml "$FULL_NODE_DIR"
#
#### Gather all the protocol keys from the validators in order to create the the MPC keys.
#PROTOCOL_PRIVATE_KEYS_FILE="protocol-private-keys.txt"
#PROTOCOL_PUBLIC_KEYS_FILE="protocol-public-keys.txt"
#
#{
#  for VALIDATOR_NAME in "${VALIDATORS_ARRAY[@]}"; do
#  VALIDATOR_DIR="${VALIDATOR_NAME}.${SUBDOMAIN}"
#  full_hostname="${VALIDATOR_NAME}.${SUBDOMAIN}"
#  echo "$full_hostname $(cat "$VALIDATOR_DIR"/"$KEY_PAIRS_DIR"/protocol.key)"
#done
#} > $PROTOCOL_PRIVATE_KEYS_FILE
#
#echo "Protocol private keys have been written to $PROTOCOL_PRIVATE_KEYS_FILE."
#
#{
#  for VALIDATOR_NAME in "${VALIDATORS_ARRAY[@]}"; do
#  VALIDATOR_DIR="${VALIDATOR_NAME}.${SUBDOMAIN}"
#  full_hostname="${VALIDATOR_NAME}.${SUBDOMAIN}"
#
#  # Construct the path to the validator.info file
#  INFO_FILE="$VALIDATOR_DIR/validator.info"
#
#  # Check if the validator.info file exists
#  if [[ -f "$INFO_FILE" ]]; then
#      # Extract the protocol-key from the file. Adjust the grep command as necessary.
#      # This assumes the line contains "protocol-key: <key_value>" and extracts <key_value>.
#      PROTOCOL_KEY=$(yq e '.info."protocol-key"' "$INFO_FILE")
#
#      # Append the item name and protocol-key to the output file
#      echo "$full_hostname $PROTOCOL_KEY"
#  else
#      echo "Warning: File $INFO_FILE not found for item $VALIDATOR_DIR."
#      exit 1
#  fi
#done
#} > $PROTOCOL_PUBLIC_KEYS_FILE
#
#echo "Protocol public keys have been written to $PROTOCOL_PUBLIC_KEYS_FILE."
#
## Collect Validator Info files to dir a dir.
#mkdir info_files_for_mpc
#for VALIDATOR_NAME in "${VALIDATORS_ARRAY[@]}"; do
#  VALIDATOR_DIR="${VALIDATOR_NAME}.${SUBDOMAIN}"
#  full_hostname="${VALIDATOR_NAME}.${SUBDOMAIN}"
#  # Construct the path to the validator.info file
#  INFO_FILE="$VALIDATOR_DIR/validator.info"
#  cp "$INFO_FILE" "info_files_for_mpc/$VALIDATOR_DIR-validator.info"
#done
#
## Go back to the root directory.
#popd
#
## Run the MPC generation.
## These are commented out because the MPC generation is handled later (see README)
##cargo run -- --from-ready-private-key-file "$SUBDOMAIN/$PROTOCOL_PRIVATE_KEYS_FILE" --to-files "$SUBDOMAIN"
##cargo run -- --from-ready-public-key-file "$SUBDOMAIN/$PROTOCOL_PUBLIC_KEYS_FILE" --to-files "$SUBDOMAIN"
#
## Prepare a docker-compose file.
#DOCKER_COMPOSE="docker-compose.yaml"
#DOCKER_COMPOSE_PATH="$SUBDOMAIN/$DOCKER_COMPOSE"
#cp docker-compose.template.yaml "$DOCKER_COMPOSE_PATH"
#
## Replace DOMAIN_NAME_HERE with the provided domain name.
#yq e -i ".services.*.container_name |= sub(\"DOMAIN_NAME_HERE\"; \"$SUBDOMAIN\")" "$DOCKER_COMPOSE_PATH"
#
## Replace DOMAIN_NAME_HERE with the provided domain name in volume paths.
#yq e -i "(.services.*.volumes[] | select(test(\".*DOMAIN_NAME_HERE.*\"))) |= sub(\"DOMAIN_NAME_HERE\"; \"$SUBDOMAIN\")" "$DOCKER_COMPOSE_PATH"
#
## Replace IMAGE_NAME with the provided image name.
#yq e -i ".services.*.image = \"$IMAGE_NAME\"" "$DOCKER_COMPOSE_PATH"
#
## Replace SUI_IMAGE_NAME with the provided faucet image name
#yq e -i "(.services.faucet.image) = \"$SUI_IMAGE_NAME\"" "$DOCKER_COMPOSE_PATH"
#
#echo "$DOCKER_COMPOSE file has been created successfully."
#
