#!/bin/sh
# Copyright (c) The Move Contributors
# SPDX-License-Identifier: BSD-3-Clause-Clear

TYPE="$(echo "$1" | sed s/^--resolve-move-//)"
PACKAGE="$2"

cat <<EOF
Broken response (not a lock file) from resolver for $TYPE of $PACKAGE.
EOF
