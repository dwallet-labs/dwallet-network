import type { DwalletMPCNetworkKey } from '../client/index.js';
import type { Config, MPCKeyScheme } from './globals.js';

export async function fetchProtocolPublicParameters(
	conf: Config,
	keyScheme: MPCKeyScheme,
	keyVersion: number | null | undefined,
): Promise<any> {
	const startTime = Date.now();

	while (Date.now() - startTime <= conf.timeout) {
		// Wait for 5 seconds between queries
		const systemStateSummary = await conf.client.getLatestPeraSystemState();
		const decryptionKeyShares = convertToMap(systemStateSummary.decryptionKeyShares);

		if (decryptionKeyShares.has(keyScheme)) {
			const versions_by_scheme = decryptionKeyShares.get(keyScheme);
			if (versions_by_scheme) {
				if (keyVersion === null || keyVersion === undefined) {
					keyVersion = versions_by_scheme.length - 1;
				}
				if (versions_by_scheme.length > keyVersion) {
					const latest_version = versions_by_scheme[keyVersion];
					if (latest_version && latest_version.length > keyVersion) {
						const protocolPublicParameters = latest_version[keyVersion]?.protocol_public_parameters;
						if (protocolPublicParameters) {
							return protocolPublicParameters;
						}
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
