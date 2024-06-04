// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import networkEnv from '_src/background/NetworkEnv';
import { API_ENV, ENV_TO_API, type NetworkEnvType } from '_src/shared/api-env';
import { SentryHttpTransport } from '@mysten/core';
import { DWalletClient, SuiHTTPTransport } from '@dwallet-network/dwallet.js/client';

const suiClientPerNetwork = new Map<string, DWalletClient>();
const SENTRY_MONITORED_ENVS = [API_ENV.mainnet];

export function getSuiClient({ env, customRpcUrl }: NetworkEnvType): DWalletClient {
	const key = `${env}_${customRpcUrl}`;
	if (!suiClientPerNetwork.has(key)) {
		const connection = customRpcUrl ? customRpcUrl : ENV_TO_API[env];
		if (!connection) {
			throw new Error(`API url not found for network env ${env} ${customRpcUrl}`);
		}
		suiClientPerNetwork.set(
			key,
			new DWalletClient({
				transport:
					!customRpcUrl && SENTRY_MONITORED_ENVS.includes(env)
						? new SentryHttpTransport(connection)
						: new SuiHTTPTransport({ url: connection }),
			}),
		);
	}
	return suiClientPerNetwork.get(key)!;
}

export async function getActiveNetworkSuiClient(): Promise<DWalletClient> {
	return getSuiClient(await networkEnv.getActiveNetwork());
}
