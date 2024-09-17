// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { defineConfig } from 'vitest/config';

export default defineConfig({
	test: {
		minWorkers: 1,
		maxWorkers: 4,
		hookTimeout: 1000000,
		testTimeout: 1000000,
		env: {
			NODE_ENV: 'test',
		},
	},
	resolve: {
		alias: {
			'@mysten/bcs': new URL('../bcs/src', import.meta.url).pathname,
			'@pera-io/pera/transactions': new URL('../typescript/src/transactions', import.meta.url)
				.pathname,
			'@pera-io/pera': new URL('../typescript/src', import.meta.url).pathname,
		},
	},
});
