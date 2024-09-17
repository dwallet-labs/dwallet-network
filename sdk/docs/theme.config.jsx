// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { useRouter } from 'next/router';

const config = {
	logo: <span>Pera TypeScript Docs</span>,
	project: {
		link: 'https://github.com/MystenLabs/sui/tree/main/sdk/',
	},
	chat: {
		link: 'https://discord.com/invite/Pera',
	},
	docsRepositoryBase: 'https://github.com/MystenLabs/sui/tree/main/sdk/docs',
	footer: {
		text: `Copyright © ${new Date().getFullYear()}, Mysten Labs, Inc.`,
	},
	head: (
		<>
			<meta name="google-site-verification" content="T-2HWJAKh8s63o9KFxCFXg5MON_NGLJG76KJzr_Hp0A" />
			<meta httpEquiv="Content-Language" content="en" />
		</>
	),
	banner: {
		key: '1.0-release',
		dismissible: false,
		text: (
			<a href="/typescript/migrations/pera-1.0">
				🎉 @pera-io/pera 1.0 has been released - Read the full migration guide here!
			</a>
		),
	},
	useNextSeoProps() {
		const { asPath } = useRouter();

		return {
			titleTemplate: asPath !== '/' ? '%s | Pera TypeScript Docs' : 'Pera TypeScript Docs',
			description:
				'Pera TypeScript Documentation. Discover the power of Pera through examples, guides, and concepts.',
			openGraph: {
				title: 'Pera TypeScript Docs',
				description:
					'Pera TypeScript Documentation. Discover the power of Pera through examples, guides, and concepts.',
				site_name: 'Pera TypeScript Docs',
			},
			additionalMetaTags: [{ content: 'Pera TypeScript Docs', name: 'apple-mobile-web-app-title' }],
			twitter: {
				card: 'summary_large_image',
				site: '@Mysten_Labs',
			},
		};
	},
};

export default config;
