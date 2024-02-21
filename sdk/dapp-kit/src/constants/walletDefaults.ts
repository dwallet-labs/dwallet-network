// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { WalletWithRequiredFeatures } from '@mysten/wallet-standard';

import { createInMemoryStore } from '../utils/stateStorage.js';

export const SUI_WALLET_NAME = 'Sui Wallet';

export const DEFAULT_STORAGE =
	typeof window !== 'undefined' && window.localStorage ? localStorage : createInMemoryStore();

export const DEFAULT_STORAGE_KEY = 'sui-dapp-kit:wallet-connection-info';

export const DEFAULT_REQUIRED_FEATURES: (keyof WalletWithRequiredFeatures['features'])[] = [
	'sui:signTransactionBlock',
];
