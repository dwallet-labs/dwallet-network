// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

module.exports = {
	stories: [
		{
			directory: '../src/ui/stories',
			titlePrefix: 'UI',
			files: '**/*.stories.*',
		},
	],
	addons: ['@storybook/addon-a11y', '@storybook/addon-essentials'],
	framework: '@storybook/react-vite',
	staticDirs: ['../public'],
};
