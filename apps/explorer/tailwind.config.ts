// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import preset from '@mysten/core/tailwind.config';
import { type Config } from 'tailwindcss';

const config: Config = {
	content: ['./src/**/*.{js,jsx,ts,tsx}', './node_modules/@mysten/ui/src/**/*.{js,jsx,ts,tsx}'],
	presets: [preset],
	theme: {
		extend: {
			width: {
				'112': '28rem',
				'128': '32rem',
			},
		},
	},
} satisfies Partial<Config>;

export default config;
