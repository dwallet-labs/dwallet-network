#!/bin/bash
# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0
#
# Automatically update all snapshots. This is needed when the framework is changed or when protocol config is changed.

set -x
set -e

SCRIPT_PATH=$(realpath "$0")
SCRIPT_DIR=$(dirname "$SCRIPT_PATH")
ROOT="$SCRIPT_DIR/.."

cd "$ROOT/crates/ika-protocol-config" && cargo insta test --review
cd "$ROOT/crates/ika-swarm-config" && cargo insta test --review
cd "$ROOT/crates/ika-open-rpc" && cargo run --example generate-json-rpc-spec -- record
cd "$ROOT/crates/ika-core" && cargo -q run --example generate-format -- print > tests/staged/ika.yaml
UPDATE=1 cargo test -p ika-framework --test build-system-packages
UPDATE=1 cargo test -p ika-rest-api
