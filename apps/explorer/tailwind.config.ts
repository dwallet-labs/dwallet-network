// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import preset from '@mysten/core/tailwind.config';
import { type Config } from 'tailwindcss';

export default {
	content: ['./src/**/*.{js,jsx,ts,tsx}', './node_modules/@mysten/ui/src/**/*.{js,jsx,ts,tsx}'],
	presets: [preset],
} satisfies Partial<Config>;
