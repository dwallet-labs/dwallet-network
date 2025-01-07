// noinspection ES6PreferShortImport

// noinspection ES6PreferShortImport
// noinspection ES6PreferShortImport
import type { DwalletMPCNetworkKey } from '../client/index.js';
import type { Config, MPCKeyScheme } from './globals.js';

/**
 * Fetch the protocol public parameters from the network.
 * @param conf
 * @param keyScheme
 * @param keyVersionNum
 */
export async function fetchProtocolPublicParameters(
	conf: Config,
	keyScheme: MPCKeyScheme,
	keyVersionNum: number | null | undefined,
): Promise<any> {
	const startTime = Date.now();

	while (Date.now() - startTime <= conf.timeout) {
		const systemStateSummary = await conf.client.getLatestPeraSystemState();
		const decryptionKeyShares = convertToMap(systemStateSummary.decryptionKeyShares);

		if (decryptionKeyShares.has(keyScheme)) {
			const keyVersionsByScheme = decryptionKeyShares.get(keyScheme);
			if (!keyVersionsByScheme) {
				continue;
			}
			keyVersionNum = keyVersionNum ?? keyVersionsByScheme.length - 1;
			if (keyVersionsByScheme.length > keyVersionNum) {
				const keyAtVersion = keyVersionsByScheme[keyVersionNum];
				if (keyAtVersion && keyAtVersion.length > keyVersionNum) {
					const protocolPublicParameters = keyAtVersion[keyVersionNum]?.protocol_public_parameters;
					if (protocolPublicParameters) {
						return protocolPublicParameters;
					}
				}
			}
		}

		// Wait for 5 seconds before the next attempt
		await new Promise((resolve) => setTimeout(resolve, 5000));
	}

	const seconds = ((Date.now() - startTime) / 1000).toFixed(2);
	throw new Error(
		`timeout: unable to fetch network DKG output within ${
			conf.timeout / (60 * 1000)
		} minutes (${seconds} seconds passed).`,
	);
}

function convertToMap(
	input: [number, DwalletMPCNetworkKey[]][],
): Map<number, DwalletMPCNetworkKey[][]> {
	const resultMap = new Map<number, DwalletMPCNetworkKey[][]>();

	input.forEach(([key, value]) => {
		if (!resultMap.has(key)) {
			resultMap.set(key, []);
		}
		resultMap.get(key)!.push(value);
	});

	return resultMap;
}
