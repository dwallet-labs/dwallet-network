// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';
import tsconfigPaths from 'vite-tsconfig-paths';

export default defineConfig({
	plugins: [react(), tsconfigPaths({ root: '../../' })],
	resolve: {
		alias: {
			'@mysten/bcs': new URL('../../../../sdk/bcs/src', import.meta.url).pathname,
		},
	},
});
