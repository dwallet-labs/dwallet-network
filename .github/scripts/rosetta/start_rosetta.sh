#!/bin/bash
# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: BSD-3-Clause-Clear

echo "Start Rosetta online server"
ika-rosetta start-online-server --data-path ./data &

echo "Start Rosetta offline server"
ika-rosetta start-offline-server &
