// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { expect, test } from './fixtures';
import { createWallet } from './utils/auth';

test('Assets tab', async ({ page, extensionUrl }) => {
	await createWallet(page, extensionUrl);
	await page.getByRole('navigation').getByRole('link', { name: 'Assets' }).click();

	await expect(page.getByRole('main').getByRole('heading')).toHaveText(/Assets/);
});

test('Apps tab', async ({ page, extensionUrl }) => {
	await createWallet(page, extensionUrl);
	await page.getByRole('navigation').getByRole('link', { name: 'Apps' }).click();

	await expect(page.getByRole('main')).toHaveText(
		/Apps below are actively curated but do not indicate any endorsement or relationship with dWallet Wallet. Please DYOR./i,
	);
});

test('Activity tab', async ({ page, extensionUrl }) => {
	await createWallet(page, extensionUrl);
	await page.getByRole('navigation').getByRole('link', { name: 'Activity' }).click();

	await expect(page.getByRole('main').getByRole('heading')).toHaveText(/Your Activity/);
});
