#!/bin/bash
# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: BSD-3-Clause-Clear

set -e

# Check for modified or untracked files after CI has run
diff="$(git diff)"
echo "${diff}"
[[ -z "${diff}" ]]

changed_files="$(git status --porcelain)"
echo "${changed_files}"
[[ -z "${changed_files}" ]]
