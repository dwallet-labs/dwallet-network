import path from 'path';
import { getFullnodeUrl, SuiClient } from '@mysten/sui/client';
import { getFaucetHost, requestSuiFromFaucetV1 } from '@mysten/sui/faucet';
import { Ed25519Keypair } from '@mysten/sui/keypairs/ed25519';
import { beforeEach, describe, it } from 'vitest';

import { checkpointCreationTime, Config, delay } from '../../src/dwallet-mpc/globals';

const fiveMinutes = 5 * 60 * 1000;

async function generateConfig(dWalletSeed: Uint8Array, suiSeed: string): Promise<Config> {
	const sourceKeypair = Ed25519Keypair.deriveKeypairFromSeed(suiSeed);
	const encryptedSecretShareSigningKeypair = Ed25519Keypair.deriveKeypairFromSeed(
		Buffer.from(dWalletSeed).toString('hex'),
	);
	const source = sourceKeypair.getPublicKey().toSuiAddress();
	const sourceSuiClient = new SuiClient({ url: getFullnodeUrl('localnet') });
	await requestSuiFromFaucetV1({
		host: getFaucetHost('localnet'),
		recipient: source,
	});

	return {
		suiClientKeypair: sourceKeypair,
		client: sourceSuiClient,
		timeout: fiveMinutes,
		ikaConfig: require(path.resolve(process.cwd(), '../../ika_config.json')),
		dWalletSeed: dWalletSeed,
		encryptedSecretShareSigningKeypair,
	};
}

describe('Test dWallet MPC', () => {
	let sourceConf: Config;
	let destConf: Config;

	beforeEach(async () => {
		sourceConf = await generateConfig(new Uint8Array(32).fill(8), '0x1');
		destConf = await generateConfig(new Uint8Array(32).fill(7), '0x2');
		await delay(checkpointCreationTime);
	});

	it('encrypts a secret share for a given public key and transfers it', async () => {

	})

});
