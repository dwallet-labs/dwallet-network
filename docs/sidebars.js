// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// @ts-check

/** @type {import('@docusaurus/plugin-content-docs').SidebarsConfig} */

// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

const sidebars = {
	dwalletSidebar: [
		{
			type: 'doc',
			label: 'Overview',
			id: 'overview',
		},
		{
			type: 'doc',
			label: 'Alpha Testnet',
			id: 'alpha-testnet',
		},
		{
			type: 'category',
			label: 'Core Concepts',
			collapsed: false,
			items: [
				'core-concepts/multi-chain-vs-cross-chain',
				'core-concepts/composable-modular-networks',
				'core-concepts/noncollusive-and-decentralized',
				'core-concepts/dwallets',
				{
					type: 'category',
					label: 'Cryptography',
					collapsed: false,
					items: ['core-concepts/cryptography/mpc', 'core-concepts/cryptography/2pc-mpc'],
				},
				'core-concepts/state-proofs',
			],
		},
		{
			type: 'category',
			label: 'Getting Started',
			collapsed: false,
			items: [
				{
					type: 'doc',
					label: 'dWallet Environment Setup',
					id: 'getting-started/guides/developers-guide/dwallet-network-environment',
				},
				{
					type: 'doc',
					label: 'Install dWallet',
					id: 'getting-started/guides/developers-guide/install-dwallet',
				},
				{
					type: 'doc',
					label: 'Connect to a dWallet Network',
					id: 'getting-started/guides/developers-guide/connect',
				},
				{
					type: 'doc',
					label: 'Connect to a Local Network',
					id: 'getting-started/guides/developers-guide/local-network',
				},
				{
					type: 'doc',
					label: 'Your First dWallet',
					id: 'getting-started/guides/developers-guide/your-first-dwallet',
				},
				{
					type: 'category',
					label: 'Guides - Coming Soon',
					collapsed: false,
					items: [
						{
							type: 'doc',
							label: 'Developers Guides',
							id: 'getting-started/guides/developers-guide',
						},
						{
							type: 'doc',
							label: 'Operators Guides',
							id: 'getting-started/guides/operators-guide',
						},
					],
				},
			],
		},
		{
			type: 'doc',
			label: 'Community',
			id: 'community',
		},
	],
};

export default sidebars;
