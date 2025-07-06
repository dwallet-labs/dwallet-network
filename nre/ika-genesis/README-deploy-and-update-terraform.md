# Deploy and Update Terraform Script

This script (`deploy-and-update-terraform.sh`) is designed to run after `create-ika-genesis-mac.sh` has completed successfully. It combines the functionality of both `gcp-upsert-validator-keys.sh` and `gcp-upsert-fullnode-notifier-secret.sh`, and additionally updates the Terraform configuration with the generated chain configuration.

## Features

1. **Deploy Validator Keys**: Uploads validator keys to GCP secrets (same as `gcp-upsert-validator-keys.sh`)
2. **Deploy Fullnode Keys**: Uploads fullnode keys to GCP secrets (same as `gcp-upsert-fullnode-notifier-secret.sh`)
3. **Update Terraform Configuration**: Updates the `ika_chain_config` block in the Terraform workspaces file with the generated configuration
4. **Copy Configuration Files**: Copies `class-groups.key` and `seed_peers.yaml` to the Terraform modules directory

## Prerequisites

- `create-ika-genesis-mac.sh` must have been run successfully
- `gcloud` CLI must be installed and configured
- `jq` must be installed
- The target Terraform workspaces file must exist
- GCP secrets must already exist (usually created by Terraform)

## Usage

### Basic Usage
```bash
./deploy-and-update-terraform.sh <SUBDOMAIN> <GCP_PROJECT> <ENV_PREFIX> <TF_NETWORK_NAME>
```

### With Options
```bash
./deploy-and-update-terraform.sh <SUBDOMAIN> <GCP_PROJECT> <ENV_PREFIX> <TF_NETWORK_NAME> --validator-num 120 --tf-path /path/to/workspaces.tf
```

### Parameters

#### Required Parameters
- `SUBDOMAIN`: The subdomain used in genesis creation (e.g., `beta.devnet2.ika-network.net`)
- `GCP_PROJECT`: The GCP project ID (e.g., `devnet-449616`)
- `ENV_PREFIX`: The environment prefix for secret names (e.g., `ika-devnet`)
- `TF_NETWORK_NAME`: The network name in Terraform config to update (e.g., `new-devnet2`)

#### Optional Parameters
- `--validator-num <number>`: Set the number of validators (default: 115)
- `--tf-path <path>`: Path to workspaces.tf file (default: `../infra/tf-gcp/workspaces.tf`)
- `--help`: Display help message

## Examples

### Example 1: Basic deployment
```bash
./deploy-and-update-terraform.sh beta.devnet2.ika-network.net devnet-449616 ika-devnet new-devnet2
```

### Example 2: With custom validator count
```bash
./deploy-and-update-terraform.sh beta.devnet2.ika-network.net devnet-449616 ika-devnet new-devnet2 --validator-num 120
```

### Example 3: With custom Terraform file path
```bash
./deploy-and-update-terraform.sh beta.devnet2.ika-network.net devnet-449616 ika-devnet new-devnet2 --tf-path /custom/path/workspaces.tf
```

## What the Script Does

### 1. Validation
- Validates all required parameters are provided
- Checks that the subdomain directory exists
- Verifies that `locals.tf` exists in the subdomain directory
- Validates that the Terraform workspaces file exists

### 2. Deploy Validator Keys
- Processes each validator directory (val1, val2, val3, etc.)
- Combines `consensus.key`, `network.key`, and `protocol.key` into a JSON object
- Uploads the JSON to GCP secrets with naming pattern: `{ENV_PREFIX}-ika-val-{i}-keys`
- Processes up to 10 validators in parallel for efficiency

### 3. Deploy Fullnode Keys
- Locates the `publisher.key` file in the `publisher/sui_config` directory
- Uploads it to GCP secrets with naming pattern: `{ENV_PREFIX}-ika-fullnode-1-keys`

### 4. Update Terraform Configuration
- Extracts the `ika_chain_config` block from `locals.tf`
- Creates a backup of the original Terraform file
- Updates the specified network's `ika_chain_config` block in the workspaces file
- Verifies the update was successful

### 5. Copy Configuration Files
- Extracts subdomain prefix from the full subdomain (e.g., `beta.devnet2` from `beta.devnet2.ika-network.net`)
- Creates the destination directory structure: `{terraform_dir}/modules/ika-chains/files/{subdomain_prefix}/ika/`
- Copies `class-groups.key` and `seed_peers.yaml` from the subdomain directory to the modules directory
- Verifies each file copy operation

## Output

The script provides colored output with emojis to make it easy to follow:
- 🚀 Process start
- 🔐 Key deployment phases
- 🔧 Terraform update phase
- ✅ Success messages
- ⚠️  Warning messages
- 🎉 Completion summary

## Error Handling

The script includes comprehensive error handling:
- Validates all inputs before processing
- Checks for required files and directories
- Creates backups before making changes
- Provides clear error messages with guidance
- Exits cleanly on errors

## Files Created/Modified

- **GCP Secrets**: Updates versions of existing secrets
- **Terraform Backup**: Creates timestamped backup of workspaces.tf
- **Terraform File**: Updates the ika_chain_config block for the specified network
- **Configuration Files**: Copies `class-groups.key` and `seed_peers.yaml` to `{terraform_dir}/modules/ika-chains/files/{subdomain_prefix}/ika/`

## Relationship to Existing Scripts

This script **does not replace** the existing scripts:
- `gcp-upsert-validator-keys.sh` - Still available for standalone validator key deployment
- `gcp-upsert-fullnode-notifier-secret.sh` - Still available for standalone fullnode key deployment

Instead, it provides a combined workflow that includes the additional Terraform update functionality.

## Next Steps After Running

1. Review the updated Terraform configuration
2. Run `terraform plan` to validate the changes
3. Apply the changes with `terraform apply`
4. Verify the deployment in GCP and Kubernetes 