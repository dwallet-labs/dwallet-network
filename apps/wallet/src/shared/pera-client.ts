// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import networkEnv from '_src/background/NetworkEnv';
import { API_ENV, ENV_TO_API, type NetworkEnvType } from '_src/shared/api-env';
import { SentryHttpTransport } from '@mysten/core';
import { PeraClient, PeraHTTPTransport } from '@pera-io/pera/client';

const peraClientPerNetwork = new Map<string, PeraClient>();
const SENTRY_MONITORED_ENVS = [API_ENV.mainnet];

export function getPeraClient({ env, customRpcUrl }: NetworkEnvType): PeraClient {
	const key = `${env}_${customRpcUrl}`;
	if (!peraClientPerNetwork.has(key)) {
		const connection = customRpcUrl ? customRpcUrl : ENV_TO_API[env];
		if (!connection) {
			throw new Error(`API url not found for network env ${env} ${customRpcUrl}`);
		}
		peraClientPerNetwork.set(
			key,
			new PeraClient({
				transport:
					!customRpcUrl && SENTRY_MONITORED_ENVS.includes(env)
						? new SentryHttpTransport(connection)
						: new PeraHTTPTransport({ url: connection }),
			}),
		);
	}
	return peraClientPerNetwork.get(key)!;
}

export async function getActiveNetworkPeraClient(): Promise<PeraClient> {
	return getPeraClient(await networkEnv.getActiveNetwork());
}
