// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { usePeraClient, usePeraClientContext } from '@mysten/dapp-kit';
import { KioskClient, Network } from '@mysten/kiosk';
import { createContext, ReactNode, useContext, useMemo } from 'react';

export const KioskClientContext = createContext<KioskClient | undefined>(undefined);

export function KioskClientProvider({ children }: { children: ReactNode }) {
	const peraClient = usePeraClient();
	const { network } = usePeraClientContext();
	const kioskClient = useMemo(
		() =>
			new KioskClient({
				client: peraClient,
				network: network as Network,
			}),
		[peraClient, network],
	);

	return <KioskClientContext.Provider value={kioskClient}>{children}</KioskClientContext.Provider>;
}

export function useKioskClient() {
	const kioskClient = useContext(KioskClientContext);
	if (!kioskClient) {
		throw new Error('kioskClient not setup properly.');
	}
	return kioskClient;
}
