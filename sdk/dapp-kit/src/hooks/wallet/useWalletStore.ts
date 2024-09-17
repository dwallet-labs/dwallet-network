// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { useContext } from 'react';
import { useStore } from 'zustand';

import { WalletContext } from '../../contexts/walletContext.js';
import type { StoreState } from '../../walletStore.js';

export function useWalletStore<T>(selector: (state: StoreState) => T): T {
	const store = useContext(WalletContext);
	if (!store) {
		throw new Error(
			'Could not find WalletContext. Ensure that you have set up the WalletProvider.',
		);
	}
	return useStore(store, selector);
}
