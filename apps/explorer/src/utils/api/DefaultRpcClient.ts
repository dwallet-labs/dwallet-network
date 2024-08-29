// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { SentryHttpTransport } from '@mysten/core';
import { DWalletClient, SuiHTTPTransport, getFullnodeUrl } from '@dwallet-network/dwallet.js/client';

export enum Network {
	LOCAL = 'LOCAL',
	DEVNET = 'DEVNET',
	TESTNET = 'TESTNET',
	MAINNET = 'MAINNET',
}

export const NetworkConfigs: Record<Network, { url: string }> = {
	[Network.LOCAL]: { url: getFullnodeUrl('localnet') },
	[Network.DEVNET]: { url: getFullnodeUrl('devnet') },
	[Network.TESTNET]: { url: getFullnodeUrl('testnet') },
	[Network.MAINNET]: { url: 'localnet' },
};

const defaultClientMap: Map<Network | string, DWalletClient> = new Map();

// NOTE: This class should not be used directly in React components, prefer to use the useSuiClient() hook instead
export const createSuiClient = (network: Network | string) => {
	const existingClient = defaultClientMap.get(network);
	if (existingClient) return existingClient;

	const networkUrl = network in Network ? NetworkConfigs[network as Network].url : network;

	const client = new DWalletClient({
		transport:
			network in Network && network === Network.MAINNET
				? new SentryHttpTransport(networkUrl)
				: new SuiHTTPTransport({ url: networkUrl }),
	});
	defaultClientMap.set(network, client);
	return client;
};
