// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { getFullnodeUrl, PeraClient } from '@pera-io/pera/client';
import { createContext, ReactNode, useContext } from 'react';

export type Network = 'mainnet' | 'testnet' | 'devnet' | 'localnet';

type DryRunContextType = {
	network: Network;
	client: PeraClient;
};

const DryRunContext = createContext<DryRunContextType | null>(null);

export const DryRunProvider = ({
	children,
	network,
}: {
	children: ReactNode;
	network: Network;
}) => {
	return (
		<DryRunContext.Provider
			value={{ network, client: new PeraClient({ url: getFullnodeUrl(network) }) }}
		>
			{children}
		</DryRunContext.Provider>
	);
};

export const useDryRunContext = () => {
	const context = useContext(DryRunContext);
	if (!context) {
		throw new Error('useDryRunContext must be used within the DryRunProvider');
	}
	return context;
};
