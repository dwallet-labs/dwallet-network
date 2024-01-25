// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { useRouter } from 'next/router';

const config = {
	logo: <span>dWallet TypeScript Docs</span>,
	project: {
		link: 'https://github.com/MystenLabs/sui/tree/main/sdk/',
	},
	chat: {
		link: 'https://discord.com/invite/Sui',
	},
	docsRepositoryBase: 'https://github.com/MystenLabs/sui/tree/main/sdk/docs/pages',
	footer: {
		text: 'Copyright © 2023, Mysten Labs, Inc.',
	},
	head: (
		<>
			<meta name="google-site-verification" content="T-2HWJAKh8s63o9KFxCFXg5MON_NGLJG76KJzr_Hp0A" />
			<meta httpEquiv="Content-Language" content="en" />
		</>
	),
	useNextSeoProps() {
		const { asPath } = useRouter();

		return {
			titleTemplate: asPath !== '/' ? '%s | dWallet TypeScript Docs' : 'dWallet TypeScript Docs',
			description:
				'dWallet TypeScript Documentation. Discover the power of dWallet through examples, guides, and concepts.',
			openGraph: {
				title: 'dWallet TypeScript Docs',
				description:
					'dWallet TypeScript Documentation. Discover the power of dWallet through examples, guides, and concepts.',
				site_name: 'dWallet TypeScript Docs',
			},
			additionalMetaTags: [{ content: 'Sui TypeScript Docs', name: 'apple-mobile-web-app-title' }],
			twitter: {
				card: 'summary_large_image',
				site: '@Mysten_Labs',
			},
		};
	},
};

export default config;
