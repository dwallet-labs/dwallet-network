// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { vanillaExtractPlugin } from '@vanilla-extract/vite-plugin';
import { defineConfig } from 'vite';

process.env.VITE_VERCEL_ENV = process.env.VERCEL_ENV || 'development';

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [vanillaExtractPlugin()],
});
