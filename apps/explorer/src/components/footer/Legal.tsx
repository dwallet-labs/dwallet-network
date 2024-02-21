// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { useProductAnalyticsConfig } from '@mysten/core';
import { Text } from '@mysten/ui';

import { legalLinks } from './footerLinks';
import { Link } from '~/ui/Link';

export function LegalText() {
	return (
		<div className="flex justify-center md:justify-start">
			<Text color="steel-dark" variant="pSubtitleSmall/medium">
				&copy;
				{`${new Date().getFullYear()} dWallet Labs. All
  rights reserved.`}
			</Text>
		</div>
	);
}

export function LegalLinks() {
	const { data: productAnalyticsConfig } = useProductAnalyticsConfig();

	return (
		<ul className="flex flex-col gap-3 md:flex-row md:gap-8">
			{legalLinks.map(({ title, href }) => (
				<li className="flex items-center justify-center" key={href}>
					<Link variant="text" href={href}>
						<Text variant="subtitleSmall/medium" color="steel-dark">
							{title}
						</Text>
					</Link>
				</li>
			))}
			{productAnalyticsConfig?.mustProvideCookieConsent && (
				<li className="flex items-center justify-center">
					<Link variant="text" data-cc="c-settings">
						<Text variant="subtitleSmall/medium" color="steel-dark">
							Manage Cookies
						</Text>
					</Link>
				</li>
			)}
		</ul>
	);
}
