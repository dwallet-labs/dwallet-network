// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { usePeraClientContext } from '@mysten/dapp-kit';
import { KioskClient, Network } from '@mysten/kiosk';
import { createContext, useMemo, type ReactNode } from 'react';

export const KioskClientContext = createContext<KioskClient | null>(null);

const peraToKioskNetwork: Record<string, Network> = {
	mainnet: Network.MAINNET,
	testnet: Network.TESTNET,
};

export type KioskClientProviderProps = {
	children: ReactNode;
};

export function KioskClientProvider({ children }: KioskClientProviderProps) {
	const { client, network } = usePeraClientContext();
	const kioskNetwork = peraToKioskNetwork[network.toLowerCase()] || Network.CUSTOM;
	const kioskClient = useMemo(
		() => new KioskClient({ client, network: kioskNetwork }),
		[client, kioskNetwork],
	);
	return <KioskClientContext.Provider value={kioskClient}>{children}</KioskClientContext.Provider>;
}
