// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { defineConfig } from 'vitest/config';

export default defineConfig({
	test: {
		minThreads: 1,
		maxThreads: 8,
		hookTimeout: 1000000,
		testTimeout: 1000000,
	},
	resolve: {
		alias: {
			'@mysten/bcs': new URL('../bcs/src', import.meta.url).toString(),
			'@dwallet/dwallet.js': new URL('../typescript/src', import.meta.url).toString(),
		},
	},
});
