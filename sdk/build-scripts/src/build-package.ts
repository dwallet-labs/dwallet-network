#! /usr/bin/env tsx
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { buildPackage } from './utils/buildPackage';

buildPackage().catch((error) => {
	console.error(error);
	process.exit(1);
});
