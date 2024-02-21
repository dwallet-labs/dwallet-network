// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { SocialDiscord24, SocialLinkedin24, SocialTwitter24 } from '@mysten/icons';
import { type ReactNode } from 'react';

type FooterItem = {
	category: string;
	items: { title: string; children: ReactNode; href: string }[];
};
export type FooterItems = FooterItem[];

function FooterIcon({ children }: { children: ReactNode }) {
	return <div className="flex items-center text-steel-darker">{children}</div>;
}

export const footerLinks = [
	{ title: 'FAQ', href: 'https://docs.mystenlabs.com/explorer/faq' },
	{ title: 'Blog', href: 'https://medium.com/mysten-labs' },
	{
		title: 'Whitepaper',
		href: 'https://github.com/MystenLabs/sui/blob/main/doc/paper/sui.pdf',
	},
	{
		title: 'Docs',
		href: 'https://docs.mystenlabs.com/explorer',
	},
	{
		title: 'GitHub',
		href: 'https://github.com/MystenLabs',
	},
	{ title: 'Press', href: 'https://mystenlabs.com/#community' },
];

export const socialLinks = [
	{
		children: (
			<FooterIcon>
				<SocialDiscord24 />
			</FooterIcon>
		),
		href: 'https://discord.gg/BK6WFhud',
	},
	{
		children: (
			<FooterIcon>
				<SocialTwitter24 />
			</FooterIcon>
		),
		href: 'https://twitter.com/Mysten_Labs',
	},
	{
		children: (
			<FooterIcon>
				<SocialLinkedin24 />
			</FooterIcon>
		),
		href: 'https://www.linkedin.com/company/mysten-labs/',
	},
];

export const legalLinks = [
	{
		title: 'Terms & Conditions',
		href: 'https://mystenlabs.com/legal#termsofservice',
	},
	{
		title: 'Privacy Policy',
		href: 'https://mystenlabs.com/legal#privacypolicy',
	},
];
