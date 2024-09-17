// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { getFullnodeUrl, PeraClient } from '@pera-io/pera/client';
import type { IdentifierRecord, ReadonlyWalletAccount } from '@mysten/wallet-standard';
import { getWallets } from '@mysten/wallet-standard';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import type { ComponentProps } from 'react';

import { WalletProvider } from '../src/components/WalletProvider.js';
import { PeraClientProvider } from '../src/index.js';
import { createMockAccount } from './mocks/mockAccount.js';
import { MockWallet } from './mocks/mockWallet.js';

export function createPeraClientContextWrapper(client: PeraClient) {
	return function PeraClientContextWrapper({ children }: { children: React.ReactNode }) {
		return <PeraClientProvider networks={{ test: client }}>{children}</PeraClientProvider>;
	};
}

export function createWalletProviderContextWrapper(
	providerProps: Omit<ComponentProps<typeof WalletProvider>, 'children'> = {},
	peraClient: PeraClient = new PeraClient({ url: getFullnodeUrl('localnet') }),
) {
	const queryClient = new QueryClient();
	return function WalletProviderContextWrapper({ children }: { children: React.ReactNode }) {
		return (
			<PeraClientProvider networks={{ test: peraClient }}>
				<QueryClientProvider client={queryClient}>
					<WalletProvider {...providerProps}>{children}</WalletProvider>;
				</QueryClientProvider>
			</PeraClientProvider>
		);
	};
}

export function registerMockWallet({
	id,
	walletName,
	accounts = [createMockAccount()],
	features = {},
}: {
	id?: string | null;
	walletName: string;
	accounts?: ReadonlyWalletAccount[];
	features?: IdentifierRecord<unknown>;
}) {
	const walletsApi = getWallets();
	const mockWallet = new MockWallet(id ?? crypto.randomUUID(), walletName, accounts, features);

	return {
		unregister: walletsApi.register(mockWallet),
		mockWallet,
	};
}
