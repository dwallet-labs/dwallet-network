import path from 'path';
import { getFullnodeUrl, SuiClient } from '@mysten/sui/client';
import { getFaucetHost, requestSuiFromFaucetV1 } from '@mysten/sui/faucet';
import { Ed25519Keypair } from '@mysten/sui/keypairs/ed25519';
import { Config } from '../../../src/dwallet-mpc/globals'; // Adjusted path for Config

const fiveMinutes = 5 * 60 * 1000;

export async function generateConfig(dWalletSeed: Uint8Array, suiSeed: string): Promise<Config> {
	const keypair = Ed25519Keypair.deriveKeypairFromSeed(suiSeed);
	const encryptedSecretShareSigningKeypair = Ed25519Keypair.deriveKeypairFromSeed(
		Buffer.from(dWalletSeed).toString('hex'),
	);
	const address = keypair.getPublicKey().toSuiAddress();
	const suiClient = new SuiClient({ url: getFullnodeUrl('localnet') });
	await requestSuiFromFaucetV1({
		host: getFaucetHost('localnet'),
		recipient: address,
	});

	return {
		suiClientKeypair: keypair,
		client: suiClient,
		timeout: fiveMinutes,
		// Adjusted path for ika_config.json
		ikaConfig: require(path.resolve(process.cwd(), '../../../ika_config.json')),
		dWalletSeed: dWalletSeed,
		encryptedSecretShareSigningKeypair,
	};
}
