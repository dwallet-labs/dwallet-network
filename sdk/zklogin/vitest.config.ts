// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { defineConfig } from 'vitest/config';

export default defineConfig({
	resolve: {
		alias: {
			'@mysten/bcs': new URL('../bcs/src', import.meta.url).pathname,
			'@pera-io/pera': new URL('../typescript/src', import.meta.url).pathname,
		},
	},
});
