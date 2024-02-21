// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

const references = [
	{
		type: 'doc',
		label: 'References',
		id: 'references',
	},
	{
		type: 'link',
		label: 'Sui Framework (GitHub)',
		href: 'https://github.com/MystenLabs/sui/tree/main/crates/sui-framework/docs',
	},
	{
		type: 'category',
		label: 'Sui API',
		link: {
			type: 'doc',
			id: 'references/sui-api',
		},
		items: [
			'references/sui-api/beta-graph-ql',
			{
				type: 'link',
				label: 'API Reference',
				href: '/sui-api-ref',
			},
			'references/sui-api/rpc-best-practices',
		],
	},
	{
		type: 'category',
		label: 'Sui CLI',
		link: {
			type: 'doc',
			id: 'references/cli',
		},
		items: [
			'references/cli/client',
			'references/cli/console',
			'references/cli/keytool',
			'references/cli/move',
			'references/cli/validator',
		],
	},
	{
		type: 'category',
		label: 'Sui SDKs',
		link: {
			type: 'doc',
			id: 'references/sui-sdks',
		},
		items: [
			{
				type: 'link',
				label: 'Sui TypeScript SDK Site',
				href: 'https://sdk.mystenlabs.com/typescript',
			},
			'references/rust-sdk',
		],
	},
	{
		type: 'link',
		label: 'dApp Kit Site',
		href: 'https://sdk.mystenlabs.com/dapp-kit',
	},
	{
		type: 'category',
		label: 'Move',
		link: {
			type: 'doc',
			id: 'references/sui-move',
		},
		items: [
			'references/move/move-toml',
			'references/move/move-lock',
			{
				type: 'link',
				label: 'Move Language (GitHub)',
				href: 'https://github.com/move-language/move/blob/main/language/documentation/book/src/introduction.md',
			},
		],
	},
	'references/research-papers',
	'references/sui-glossary',
	{
		type: 'category',
		label: 'Contribute',
		link: {
			type: 'doc',
			id: 'references/contribute/contribution-process',
		},
		items: [
			'references/contribute/contribution-process',
			'references/contribute/contribute-to-sui-repos',
			{
				type: 'link',
				label: 'Submit a SIP',
				href: 'https://sips.sui.io',
			},
			'references/contribute/localize-sui-docs',
			'references/contribute/code-of-conduct',
			'references/contribute/style-guide',
		],
	},
];

module.exports = references;
