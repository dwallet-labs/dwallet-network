// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

const withNextra = require('nextra')({
	theme: 'nextra-theme-docs',
	themeConfig: './theme.config.jsx',
});

module.exports = withNextra({
	redirects: () => {
		return [
			{
				source: '/',
				destination: '/typescript',
				statusCode: 302,
			},
		];
	},
});
