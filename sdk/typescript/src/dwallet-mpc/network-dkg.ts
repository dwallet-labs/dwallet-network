// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
// noinspection ES6PreferShortImport

import type { NetworkDecryptionKeyShares } from '../client/index.js';
import type { Config, MPCKeyScheme } from './globals.js';
import { delay } from './globals.js';

/**
 * Fetch the protocol public parameters from the network.
 * @param c
 * @param keyScheme
 * @param keyVersionNum
 */
export async function fetchProtocolPublicParameters(
	c: Config,
	keyScheme: MPCKeyScheme,
	keyVersionNum?: number | null,
): Promise<any> {
	const startTime = Date.now();

	while (Date.now() - startTime <= c.timeout) {
		const systemStateSummary = await c.client.getLatestPeraSystemState();
		const decryptionKeyShares = convertToMap(systemStateSummary.networkMpcKeys);

		if (!decryptionKeyShares.has(keyScheme)) {
			await delay(5000);
			continue;
		}
		const keyVersionsByScheme = decryptionKeyShares.get(keyScheme);
		if (!keyVersionsByScheme) {
			continue;
		}
		keyVersionNum = keyVersionNum ?? keyVersionsByScheme.length - 1;
		if (keyVersionsByScheme.length > keyVersionNum) {
			const keyAtVersion = keyVersionsByScheme[keyVersionNum];
			const protocolPublicParameters = keyAtVersion.protocol_public_parameters;
			if (protocolPublicParameters) {
				return protocolPublicParameters;
			}
		}
	}

	const seconds = ((Date.now() - startTime) / 1000).toFixed(2);
	throw new Error(
		`timeout: unable to fetch network DKG output within ${
			c.timeout / (60 * 1000)
		} minutes (${seconds} seconds passed).`,
	);
}

function convertToMap(
	input: [MPCKeyScheme, NetworkDecryptionKeyShares][],
): Map<MPCKeyScheme, NetworkDecryptionKeyShares[]> {
	const resultMap = new Map<MPCKeyScheme, NetworkDecryptionKeyShares[]>();

	input.forEach(([key, value]) => {
		if (!resultMap.has(key)) {
			resultMap.set(key, []);
		}
		resultMap.get(key)?.push(value);
	});

	return resultMap;
}
