// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { PeraWalletFeatures, WalletWithRequiredFeatures } from '@mysten/wallet-standard';
import { STASHED_WALLET_NAME } from '@mysten/zksend';

import { createInMemoryStore } from '../utils/stateStorage.js';

export const PERA_WALLET_NAME = 'Pera Wallet';

export const DEFAULT_STORAGE =
	typeof window !== 'undefined' && window.localStorage ? localStorage : createInMemoryStore();

export const DEFAULT_STORAGE_KEY = 'pera-dapp-kit:wallet-connection-info';

const SIGN_FEATURES = [
	'pera:signTransaction',
	'pera:signTransactionBlock',
] satisfies (keyof PeraWalletFeatures)[];

export const DEFAULT_WALLET_FILTER = (wallet: WalletWithRequiredFeatures) =>
	SIGN_FEATURES.some((feature) => wallet.features[feature]);

export const DEFAULT_PREFERRED_WALLETS = [PERA_WALLET_NAME, STASHED_WALLET_NAME];
