// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { DEFAULT_API_ENV } from '_app/ApiProvider';
import { API_ENV, type NetworkEnvType } from '_src/shared/api-env';
import { isValidUrl } from '_src/shared/utils';
import mitt from 'mitt';
import Browser from 'webextension-polyfill';

class NetworkEnv {
	#events = mitt<{ changed: NetworkEnvType }>();

	async getActiveNetwork(): Promise<NetworkEnvType> {
		const { pera_Env, pera_Env_RPC } = await Browser.storage.local.get({
			pera_Env: DEFAULT_API_ENV,
			pera_Env_RPC: null,
		});
		const adjCustomUrl = pera_Env === API_ENV.customRPC ? pera_Env_RPC : null;
		return { env: pera_Env, customRpcUrl: adjCustomUrl };
	}

	async setActiveNetwork(network: NetworkEnvType) {
		const { env, customRpcUrl } = network;
		if (env === API_ENV.customRPC && !isValidUrl(customRpcUrl)) {
			throw new Error(`Invalid custom RPC url ${customRpcUrl}`);
		}
		await Browser.storage.local.set({
			pera_Env: env,
			pera_Env_RPC: customRpcUrl,
		});
		this.#events.emit('changed', network);
	}

	on = this.#events.on;

	off = this.#events.off;
}

const networkEnv = new NetworkEnv();
export default networkEnv;
