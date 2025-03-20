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
# In a testnet use the faucet public key.
ROOT_ADDR=""
# The file containing the validators (separator: newline).
export VALIDATORS_FILE=""
# Validator Docker image name.
export IMAGE_NAME="471930101563.dkr.ecr.us-east-1.amazonaws.com/sui-fork:ika-testnet-v1.16.2-10"
# SUI Faucet URL.
export SUI_FAUCET_URL="http://127.0.0.1:9123/gas"
# Default sui epoch duration time.
export EPOCH_DURATION_TIME=86400000

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
    echo "  --epoch-duration-time <time>        Set the epoch duration time. Default: $EPOCH_DURATION_TIME"
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


# Add Validator Candidate.
# Create the validator.yaml file.
for entry in "${VALIDATORS_ARRAY[@]}"; do
      # Split the tuple "validatorName:validatorHostname" into variables.
      IFS=":" read -r VALIDATOR_NAME VALIDATOR_HOSTNAME <<< "$entry"
      VALIDATOR_DIR="${VALIDATOR_HOSTNAME}"
      ACCOUNT_ADDRESS=$(yq e '.account_address' "${VALIDATOR_DIR}/validator.info")
      P2P_ADDR=$(yq e '.p2p_address' "${VALIDATOR_DIR}/validator.info")
      cp ../validator.template.yaml "$VALIDATOR_DIR"/validator.yaml
      yq e ".p2p-config.external-address = \"$P2P_ADDR\"" -i "$VALIDATOR_DIR"/validator.yaml
      # --- Request tokens from the faucet ---
      # Use curl to post a FixedAmountRequest to the faucet.
      curl -X POST --location "${SUI_FAUCET_URL}" \
           -H "Content-Type: application/json" \
           -d '{
                "FixedAmountRequest": {
                  "recipient": "'"${ACCOUNT_ADDRESS}"'"
                }
              }' | jq
done
