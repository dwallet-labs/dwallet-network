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
			collapsed: true,
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
			label: 'Developers Guide',
			collapsed: true,
			items: [
				{
					type: 'category',
					label: 'Getting Started',
					collapsed: false,
					items: [
						{
							type: 'doc',
							label: 'dWallet Environment Setup',
							id: 'developers-guide/getting-started/dwallet-network-environment',
						},
						{
							type: 'doc',
							label: 'Install dWallet',
							id: 'developers-guide/getting-started/install-dwallet',
						},
						{
							type: 'doc',
							label: 'Get DWLT Address',
							id: 'developers-guide/getting-started/get-address',
						},
						{
							type: 'doc',
							label: 'Connect to a dWallet Network',
							id: 'developers-guide/getting-started/connect',
						},
						{
							type: 'doc',
							label: 'Connect to a Local Network',
							id: 'developers-guide/getting-started/local-network',
						},
						{
							type: 'doc',
							label: 'Get DWLT Tokens',
							id: 'developers-guide/getting-started/get-tokens',
						},
						{
							type: 'doc',
							label: 'Your First dWallet',
							id: 'developers-guide/getting-started/your-first-dwallet',
						},
					],
				},
			],
		},
		{
			type: 'doc',
			label: 'Operators Guide - Coming Soon',
			id: 'operators-guide',
		},
		{
			type: 'doc',
			label: 'Community',
			id: 'community',
		},
	],
};

export default sidebars;
