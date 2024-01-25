// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { useWalletStore } from './useWalletStore.js';

/**
 * Retrieves a list of registered wallets available to the dApp sorted by preference.
 */
export function useWallets() {
	return useWalletStore((state) => state.wallets);
}
