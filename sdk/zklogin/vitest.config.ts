// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { defineConfig } from 'vitest/config';

export default defineConfig({
	resolve: {
		alias: {
			'@mysten/bcs': new URL('../bcs/src', import.meta.url).toString(),
			'@dwallet-network/dwallet.js': new URL('../typescript/src', import.meta.url).toString(),
		},
	},
});
