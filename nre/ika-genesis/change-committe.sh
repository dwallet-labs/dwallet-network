#!/bin/bash

# f = 38
# max vote = 10% of Total ~= 12
# q(threshold) = 2f+1 = 77
# n(total) = 3f+1 = 115
# Configuration variables (change these values as needed)
TOTAL_VOTING_POWER=115
# TOTAL_VOTING_POWER*2/3 + 1
QUORUM_THRESHOLD=77
# Cap voting power of an individual validator at 10%.
MAX_VOTING_POWER=12
# Validity threshold for the committee.
# f+1
VALIDITY_THRESHOLD=39

pushd ../../

# File paths.
MOVE_FILE="crates/ika-move-packages/packages/ika_system/sources/system_v1/bls_committee.move"
RUST_FILE="crates/ika-types/src/committee.rs"

# Check if files exist.
if [ ! -f "$MOVE_FILE" ]; then
    echo "Error: Move file not found at $MOVE_FILE"
    exit 1
fi

if [ ! -f "$RUST_FILE" ]; then
    echo "Error: Rust file not found at $RUST_FILE"
    exit 1
fi

echo "Starting parameter update..."

# Update Move file
echo "Updating Move file at $MOVE_FILE"

# Replace values in the Move file using sed.
sed -i '' \
    -e "s/const TOTAL_VOTING_POWER: u64 = [0-9]*;/const TOTAL_VOTING_POWER: u64 = $TOTAL_VOTING_POWER;/" \
    -e "s/const QUORUM_THRESHOLD: u64 = [0-9]*;/const QUORUM_THRESHOLD: u64 = $QUORUM_THRESHOLD;/" \
    -e "s/const MAX_VOTING_POWER: u64 = [0-9]*;/const MAX_VOTING_POWER: u64 = $MAX_VOTING_POWER;/" \
    "$MOVE_FILE"

# Update Rust file.
echo "Updating Rust file at $RUST_FILE"

# Replace values in the Rust file.
sed -i '' \
    -e "s/pub const TOTAL_VOTING_POWER: StakeUnit = [0-9]*;/pub const TOTAL_VOTING_POWER: StakeUnit = $TOTAL_VOTING_POWER;/" \
    -e "s/pub const QUORUM_THRESHOLD: StakeUnit = .*$/pub const QUORUM_THRESHOLD: StakeUnit = $QUORUM_THRESHOLD;/" \
    -e "s/pub const VALIDITY_THRESHOLD: StakeUnit = [0-9]*;/pub const VALIDITY_THRESHOLD: StakeUnit = $VALIDITY_THRESHOLD;/" \
    "$RUST_FILE"

# Verify the changes.
echo "Verifying changes..."

# Check Move file changes.
echo "Move file changes:"
grep -n "TOTAL_VOTING_POWER\|QUORUM_THRESHOLD\|MAX_VOTING_POWER" "$MOVE_FILE"

# Check Rust file changes.
echo "Rust file changes:"
grep -n "TOTAL_VOTING_POWER\|QUORUM_THRESHOLD\|VALIDITY_THRESHOLD" "$RUST_FILE"

make snapshot
