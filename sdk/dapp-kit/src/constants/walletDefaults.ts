// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { IkaWalletFeatures, WalletWithRequiredFeatures } from '@mysten/wallet-standard';
import { STASHED_WALLET_NAME } from '@mysten/zksend';

import { createInMemoryStore } from '../utils/stateStorage.js';

export const IKA_WALLET_NAME = 'Ika Wallet';

export const DEFAULT_STORAGE =
	typeof window !== 'undefined' && window.localStorage ? localStorage : createInMemoryStore();

export const DEFAULT_STORAGE_KEY = 'ika-dapp-kit:wallet-connection-info';

const SIGN_FEATURES = [
	'ika:signTransaction',
	'ika:signTransactionBlock',
] satisfies (keyof IkaWalletFeatures)[];

export const DEFAULT_WALLET_FILTER = (wallet: WalletWithRequiredFeatures) =>
	SIGN_FEATURES.some((feature) => wallet.features[feature]);

export const DEFAULT_PREFERRED_WALLETS = [IKA_WALLET_NAME, STASHED_WALLET_NAME];
