// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import tsconfigPaths from 'vite-tsconfig-paths';
import { configDefaults, defineConfig } from 'vitest/config';

export default defineConfig({
	plugins: [tsconfigPaths()],
	test: {
		exclude: [...configDefaults.exclude, 'tests/**'],
		// TODO: Create custom extension environment.
		environment: 'happy-dom',
		minThreads: 1,
		setupFiles: ['./testSetup.ts'],
		restoreMocks: true,
	},
});
