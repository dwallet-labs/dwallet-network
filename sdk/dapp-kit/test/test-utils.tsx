// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { getFullnodeUrl, IkaClient } from '@ika-io/ika/client';
import type { IdentifierRecord, ReadonlyWalletAccount } from '@mysten/wallet-standard';
import { getWallets } from '@mysten/wallet-standard';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import type { ComponentProps } from 'react';

import { WalletProvider } from '../src/components/WalletProvider.js';
import { IkaClientProvider } from '../src/index.js';
import { createMockAccount } from './mocks/mockAccount.js';
import { MockWallet } from './mocks/mockWallet.js';

export function createIkaClientContextWrapper(client: IkaClient) {
	return function IkaClientContextWrapper({ children }: { children: React.ReactNode }) {
		return <IkaClientProvider networks={{ test: client }}>{children}</IkaClientProvider>;
	};
}

export function createWalletProviderContextWrapper(
	providerProps: Omit<ComponentProps<typeof WalletProvider>, 'children'> = {},
	ikaClient: IkaClient = new IkaClient({ url: getFullnodeUrl('localnet') }),
) {
	const queryClient = new QueryClient();
	return function WalletProviderContextWrapper({ children }: { children: React.ReactNode }) {
		return (
			<IkaClientProvider networks={{ test: ikaClient }}>
				<QueryClientProvider client={queryClient}>
					<WalletProvider {...providerProps}>{children}</WalletProvider>;
				</QueryClientProvider>
			</IkaClientProvider>
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
