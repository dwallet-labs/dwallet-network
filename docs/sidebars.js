/**
 * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */

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
				items: [
					'core-concepts/cryptography/mpc',
					'core-concepts/cryptography/2pc-mpc',
				],
			},
			'core-concepts/state-proofs',
		],
	},
	{
		type: 'category',
		label: 'Getting Started - Coming Soon',
    collapsed: false,
    items: [
			{
				type: 'doc',
				label: 'Installation',
				id: 'getting-started/installation',
			},
			{
				type: 'doc',
				label: 'Your First dWallet',
				id: 'getting-started/your-first-dwallet',
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
