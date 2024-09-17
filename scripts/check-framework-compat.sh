#!/bin/bash
# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: BSD-3-Clause-Clear
#
# Check whether the version of framework in the repo is compatible
# with the version on chain, as reported by the currently active
# environment, using the binary in environment variable $PERA.

set -e

PERA=${PERA:-pera}
REPO=$(git rev-parse --show-toplevel)

for PACKAGE in "$REPO"/crates/pera-framework/packages/*; do
    $PERA client verify-source "$PACKAGE"
done

