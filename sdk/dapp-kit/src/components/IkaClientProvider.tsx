// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { getFullnodeUrl, isIkaClient, IkaClient } from '@ika-io/ika/client';
import type { IkaClientOptions } from '@ika-io/ika/client';
import { createContext, useMemo, useState } from 'react';

import type { NetworkConfig } from '../hooks/networkConfig.js';

type NetworkConfigs<T extends NetworkConfig | IkaClient = NetworkConfig | IkaClient> = Record<
	string,
	T
>;

export interface IkaClientProviderContext {
	client: IkaClient;
	networks: NetworkConfigs;
	network: string;
	config: NetworkConfig | null;
	selectNetwork: (network: string) => void;
}

export const IkaClientContext = createContext<IkaClientProviderContext | null>(null);

export type IkaClientProviderProps<T extends NetworkConfigs> = {
	createClient?: (name: keyof T, config: T[keyof T]) => IkaClient;
	children: React.ReactNode;
	networks?: T;
	onNetworkChange?: (network: keyof T & string) => void;
} & (
	| {
			defaultNetwork?: keyof T & string;
			network?: never;
	  }
	| {
			defaultNetwork?: never;
			network?: keyof T & string;
	  }
);

const DEFAULT_NETWORKS = {
	localnet: { url: getFullnodeUrl('localnet') },
};

const DEFAULT_CREATE_CLIENT = function createClient(
	_name: string,
	config: NetworkConfig | IkaClient,
) {
	if (isIkaClient(config)) {
		return config;
	}

	return new IkaClient(config);
};

export function IkaClientProvider<T extends NetworkConfigs>(props: IkaClientProviderProps<T>) {
	const { onNetworkChange, network, children } = props;
	const networks = (props.networks ?? DEFAULT_NETWORKS) as T;
	const createClient =
		(props.createClient as typeof DEFAULT_CREATE_CLIENT) ?? DEFAULT_CREATE_CLIENT;

	const [selectedNetwork, setSelectedNetwork] = useState<keyof T & string>(
		props.network ?? props.defaultNetwork ?? (Object.keys(networks)[0] as keyof T & string),
	);

	const currentNetwork = props.network ?? selectedNetwork;

	const client = useMemo(() => {
		return createClient(currentNetwork, networks[currentNetwork]);
	}, [createClient, currentNetwork, networks]);

	const ctx = useMemo((): IkaClientProviderContext => {
		return {
			client,
			networks,
			network: currentNetwork,
			config:
				networks[currentNetwork] instanceof IkaClient
					? null
					: (networks[currentNetwork] as IkaClientOptions),
			selectNetwork: (newNetwork) => {
				if (currentNetwork === newNetwork) {
					return;
				}

				if (!network && newNetwork !== selectedNetwork) {
					setSelectedNetwork(newNetwork);
				}

				onNetworkChange?.(newNetwork);
			},
		};
	}, [client, networks, selectedNetwork, currentNetwork, network, onNetworkChange]);

	return <IkaClientContext.Provider value={ctx}>{children}</IkaClientContext.Provider>;
}
