// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module.exports = {
	printWidth: 100,
	semi: true,
	singleQuote: true,
	tabWidth: 2,
	trailingComma: 'all',
	useTabs: true,
	plugins: ['@ianvs/prettier-plugin-sort-imports'],
	importOrder: [
		'<BUILT_IN_MODULES>',
		'<THIRD_PARTY_MODULES>',
		'',
		'^@/(.*)$',
		'^~/(.*)$',
		'',
		'^[.]',
	],
	overrides: [
		{
			files: 'apps/explorer/**/*',
			options: {
				plugins: ['prettier-plugin-tailwindcss'],
				tailwindConfig: './apps/explorer/tailwind.config.ts',
			},
		},
		{
			files: 'sdk/**/*',
			options: {
				proseWrap: 'always',
			},
		},
	],
};
