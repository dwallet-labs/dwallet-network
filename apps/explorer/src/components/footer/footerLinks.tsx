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
	{ title: 'Blog', href: 'https://dwallet.io/blog' },
	{
		title: 'Docs',
		href: 'https://docs.dwallet.io/',
	},
	{
		title: 'GitHub',
		href: 'https://github.com/dwallet-labs/dwallet-network',
	},
];

export const socialLinks = [
	{
		children: (
			<FooterIcon>
				<SocialDiscord24 />
			</FooterIcon>
		),
		href: 'https://discord.gg/dwallet',
	},
	{
		children: (
			<FooterIcon>
				<SocialTwitter24 />
			</FooterIcon>
		),
		href: 'https://twitter.com/dWalletNetwork',
	},
	{
		children: (
			<FooterIcon>
				<SocialLinkedin24 />
			</FooterIcon>
		),
		href: 'https://www.linkedin.com/company/dwalletnetwork/',
	},
];

export const legalLinks = [
	{
		title: 'Privacy Policy',
		href: 'https://www.dwallet.io/privacy-policy',
	},
];
