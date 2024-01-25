#!/bin/bash
# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: BSD-3-Clause-Clear

echo "Start Rosetta online server"
sui-rosetta start-online-server --data-path ./data &

echo "Start Rosetta offline server"
sui-rosetta start-offline-server &
