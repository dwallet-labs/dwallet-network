// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { IkaClient } from '@ika-io/ika/client';
import type { TransactionObjectArgument } from '@ika-io/ika/transactions';

import type { BaseRulePackageIds } from '../constants.js';

export * from './kiosk.js';
export * from './transfer-policy.js';

/**
 * A valid argument for any of the Kiosk functions.
 */
export type ObjectArgument = string | TransactionObjectArgument;

/**
 * A Network selector.
 * Kiosk SDK supports mainnet & testnet.
 * Pass `custom` for any other network (devnet, localnet).
 */
export enum Network {
	MAINNET = 'mainnet',
	TESTNET = 'testnet',
	CUSTOM = 'custom',
}

/**
 * The Client Options for Both KioskClient & TransferPolicyManager.
 */
export type KioskClientOptions = {
	client: IkaClient;
	network: Network;
	packageIds?: BaseRulePackageIds;
};
