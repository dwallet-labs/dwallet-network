#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 2 ]]; then
  echo "Usage: $0 <GCP_PROJECT> <SECRET_NAME>" >&2
  exit 1
fi

GCP_PROJECT="$1"
SECRET_NAME="$2"

# The key files we expect to find here
KEY_FILES=("consensus.key" "network.key" "protocol.key")

# Make sure they all exist
for f in "${KEY_FILES[@]}"; do
  if [[ ! -f "$f" ]]; then
    echo "Error: Key file '$f' not found in $(pwd)" >&2
    exit 1
  fi
done

# Build a JSON object of the form { "consensus.key": "<contents>", ... }
SECRETS_JSON="{}"
for f in "${KEY_FILES[@]}"; do
  CONTENT=$(<"$f")
  SECRETS_JSON=$(jq --arg key "$f" --arg value "$CONTENT" \
    '. + {($key): $value}' <<< "$SECRETS_JSON")
done

# Write it out and push a new version
echo "$SECRETS_JSON" > secrets.json
gcloud secrets versions add "$SECRET_NAME" \
  --data-file=secrets.json \
  --project="$GCP_PROJECT"

echo "✅ Added new version to secret: $SECRET_NAME"
