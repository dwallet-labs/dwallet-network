// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { type DWalletClient } from '@dwallet-network/dwallet.js/client';
import { TransactionObjectArgument } from '@dwallet-network/dwallet.js/transactions';

import { BaseRulePackageIds } from '../constants';

export * from './kiosk';
export * from './transfer-policy';

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
	client: DWalletClient;
	network: Network;
	packageIds?: BaseRulePackageIds;
};
