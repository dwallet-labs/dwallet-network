// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

const guides = [
	{
		type: 'doc',
		label: 'Guides',
		id: 'guides',
	},
	{
		type: 'category',
		label: 'Developer Guides',
		link: {
			type: 'doc',
			id: 'guides/developer',
		},
		items: [
			{
				type: 'category',
				label: 'Getting Started',
				link: {
					type: 'doc',
					id: 'guides/developer/getting-started',
				},
				items: [
					'guides/developer/getting-started/sui-environment',
					'guides/developer/getting-started/sui-install',
					'guides/developer/getting-started/connect',
					'guides/developer/getting-started/local-network',
					'guides/developer/getting-started/get-address',
					'guides/developer/getting-started/get-coins',
				],
			},
			{
				type: 'category',
				label: 'Your First Sui dApp',
				link: {
					type: 'doc',
					id: 'guides/developer/first-app',
				},
				items: [
					'guides/developer/first-app/write-package',
					'guides/developer/first-app/build-test',
					'guides/developer/first-app/publish',
					'guides/developer/first-app/debug',
					'guides/developer/first-app/client-tssdk',
				],
			},
			{
				type: 'category',
				label: 'Sui 101',
				link: {
					type: 'doc',
					id: 'guides/developer/sui-101',
				},
				items: [
					'guides/developer/sui-101/shared-owned',
					'guides/developer/sui-101/create-coin',
					'guides/developer/sui-101/create-nft',
					'guides/developer/sui-101/using-events',
					'guides/developer/sui-101/access-time',
					'guides/developer/sui-101/sign-and-send-txn',
					'guides/developer/sui-101/sponsor-txn',
					{
						type: 'category',
						label: 'Working with PTBs',
						items: [
							'guides/developer/sui-101/simulating-refs',
							'guides/developer/sui-101/coin-mgt',
							'guides/developer/sui-101/building-ptb',
						],
					},
				],
			},
			{
				type: 'category',
				label: 'Cryptography',
				link: {
					type: 'doc',
					id: 'guides/developer/cryptography',
				},
				items: [
					'guides/developer/cryptography/signing',
					'guides/developer/cryptography/groth16',
					'guides/developer/cryptography/hashing',
					'guides/developer/cryptography/ecvrf',
				],
			},
			{
				type: 'category',
				label: 'Advanced Topics',
				link: {
					type: 'doc',
					id: 'guides/developer/advanced',
				},
				items: [
					{
						type: 'category',
						label: 'Efficient Smart Contracts',
						link: {
							type: 'doc',
							id: 'guides/developer/advanced/efficient-smart-contracts',
						},
						items: ['guides/developer/advanced/min-gas-fees'],
					},
					'guides/developer/advanced/wallet-integrations',
					'guides/developer/advanced/security-best-practices',
					'guides/developer/advanced/maximize-reach',
					'guides/developer/advanced/asset-tokenization',
				],
			},
			{
				type: 'category',
				label: 'App Examples',
				link: {
					type: 'doc',
					id: 'guides/developer/app-examples',
				},
				items: [
					'guides/developer/app-examples/e2e-counter',
					'guides/developer/app-examples/auction',
					'guides/developer/app-examples/escrow',
					'guides/developer/app-examples/trusted-swap',
					'guides/developer/app-examples/tic-tac-toe',
					'guides/developer/app-examples/recaptcha',
					'guides/developer/app-examples/turnip-town',
					{
						type: 'category',
						label: 'Oracles',
						link: {
							type: 'doc',
							id: 'guides/developer/app-examples/oracle',
						},
						items: [
							'guides/developer/app-examples/weather-oracle',
							'guides/developer/app-examples/meta-pricing-oracle',
						],
					},
					'guides/developer/app-examples/coin-flip',
					'guides/developer/app-examples/blackjack',
				],
			},
			'guides/developer/starter-templates',
			'guides/developer/zklogin-onboarding',
			'guides/developer/dev-cheat-sheet',
		],
	},
	{
		type: 'category',
		label: 'Operator Guides',
		link: {
			type: 'doc',
			id: 'guides/operator',
		},
		items: [
			'guides/operator/sui-full-node',
			'guides/operator/validator-config',
			'guides/operator/data-management',
			'guides/operator/snapshots',
			'guides/operator/archives',
			'guides/operator/genesis',
			'guides/operator/validator-committee',
			'guides/operator/validator-tasks',
			'guides/operator/node-tools',
			'guides/operator/formal-snapshot',
			'guides/operator/exchange-integration',
		],
	},
];
module.exports = guides;
