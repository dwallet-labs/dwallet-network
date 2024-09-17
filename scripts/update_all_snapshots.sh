#!/bin/bash
# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: BSD-3-Clause-Clear
#
# Automatically update all snapshots. This is needed when the framework is changed or when protocol config is changed.

set -x
set -e

SCRIPT_PATH=$(realpath "$0")
SCRIPT_DIR=$(dirname "$SCRIPT_PATH")
ROOT="$SCRIPT_DIR/.."

cd "$ROOT/crates/pera-protocol-config" && cargo insta test --review
cd "$ROOT/crates/pera-swarm-config" && cargo insta test --review
cd "$ROOT/crates/pera-open-rpc" && cargo run --example generate-json-rpc-spec -- record
cd "$ROOT/crates/pera-core" && cargo -q run --example generate-format -- print > tests/staged/pera.yaml
UPDATE=1 cargo test -p pera-framework --test build-system-packages
UPDATE=1 cargo test -p pera-rest-api
