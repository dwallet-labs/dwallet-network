// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { expect, test } from './fixtures';
import { createWallet } from './utils/auth';

const TEST_TIMEOUT = 45 * 1000;
const STAKE_AMOUNT = 100;

test('staking', async ({ page, extensionUrl }) => {
	test.setTimeout(4 * TEST_TIMEOUT);

	await createWallet(page, extensionUrl);

	await page.getByTestId('faucet-request-button').click();
	await expect(page.getByTestId('coin-balance')).not.toHaveText('0DWLT');

	await page.getByText(/Stake and Earn DWLT/).click();
	await page.getByTestId('validator-list-item').first().click();
	await page.getByTestId('select-validator-cta').click();
	await page.getByTestId('stake-amount-input').fill(STAKE_AMOUNT.toString());
	await page.getByRole('button', { name: 'Stake Now' }).click();
	await expect(page.getByTestId('loading-indicator')).not.toBeVisible({
		timeout: TEST_TIMEOUT,
	});
	await expect(page.getByTestId('overlay-title')).toHaveText('Transaction');
	await expect(page.getByTestId('transaction-status')).toHaveText('Transaction Success');

	await page.getByTestId('close-icon').click();

	await expect(page.getByTestId(`stake-button-${STAKE_AMOUNT}-DWLT`)).toBeVisible({
		timeout: TEST_TIMEOUT,
	});
	await page.getByTestId(`stake-button-${STAKE_AMOUNT}-DWLT`).click();

	await expect(page.getByTestId('stake-card')).toBeVisible({ timeout: 3 * TEST_TIMEOUT });
	await page.getByTestId('stake-card').click();
	await page.getByTestId('unstake-button').click();
	await page.getByRole('button', { name: 'Unstake Now' }).click();
	await expect(page.getByTestId('loading-indicator')).not.toBeVisible({
		timeout: TEST_TIMEOUT,
	});
	await expect(page.getByTestId('overlay-title')).toHaveText('Transaction');
	await expect(page.getByTestId('transaction-status')).toHaveText('Transaction Success');
});
