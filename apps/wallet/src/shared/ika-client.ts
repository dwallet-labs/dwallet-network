// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import networkEnv from '_src/background/NetworkEnv';
import { API_ENV, ENV_TO_API, type NetworkEnvType } from '_src/shared/api-env';
import { SentryHttpTransport } from '@mysten/core';
import { IkaClient, IkaHTTPTransport } from '@ika-io/ika/client';

const ikaClientPerNetwork = new Map<string, IkaClient>();
const SENTRY_MONITORED_ENVS = [API_ENV.mainnet];

export function getIkaClient({ env, customRpcUrl }: NetworkEnvType): IkaClient {
	const key = `${env}_${customRpcUrl}`;
	if (!ikaClientPerNetwork.has(key)) {
		const connection = customRpcUrl ? customRpcUrl : ENV_TO_API[env];
		if (!connection) {
			throw new Error(`API url not found for network env ${env} ${customRpcUrl}`);
		}
		ikaClientPerNetwork.set(
			key,
			new IkaClient({
				transport:
					!customRpcUrl && SENTRY_MONITORED_ENVS.includes(env)
						? new SentryHttpTransport(connection)
						: new IkaHTTPTransport({ url: connection }),
			}),
		);
	}
	return ikaClientPerNetwork.get(key)!;
}

export async function getActiveNetworkIkaClient(): Promise<IkaClient> {
	return getIkaClient(await networkEnv.getActiveNetwork());
}
