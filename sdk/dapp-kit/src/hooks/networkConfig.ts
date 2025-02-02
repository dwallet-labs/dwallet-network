// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { IkaClientOptions } from '@ika-io/ika/client';

import { useIkaClientContext } from './useIkaClient.js';

export type NetworkConfig<T extends object = object> = IkaClientOptions & {
	variables?: T;
};

export function createNetworkConfig<
	const T extends Record<string, Config>,
	Config extends NetworkConfig<Variables> = T[keyof T],
	Variables extends object = NonNullable<Config['variables']>,
>(networkConfig: T) {
	function useNetworkConfig(): Config {
		const { config } = useIkaClientContext();

		if (!config) {
			throw new Error('No network config found');
		}

		return config as T[keyof T];
	}

	function useNetworkVariables(): Variables {
		const { variables } = useNetworkConfig();

		return (variables ?? {}) as Variables;
	}

	function useNetworkVariable<K extends keyof Variables>(name: K): Variables[K] {
		const variables = useNetworkVariables();

		return variables[name];
	}

	return {
		networkConfig,
		useNetworkConfig,
		useNetworkVariables,
		useNetworkVariable,
	};
}
