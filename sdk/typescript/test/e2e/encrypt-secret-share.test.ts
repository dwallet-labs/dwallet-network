import path from 'path';
import { getFullnodeUrl, SuiClient } from '@mysten/sui/client';
import { getFaucetHost, requestSuiFromFaucetV1 } from '@mysten/sui/faucet';
import { Ed25519Keypair } from '@mysten/sui/keypairs/ed25519';
import { beforeEach, describe, it } from 'vitest';

import { acceptEncryptedUserShare, createDWallet } from '../../src/dwallet-mpc/dkg';
import {
	encryptUserShareForPublicKey,
	getOrCreateClassGroupsKeyPair,
	transferEncryptedSecretShare,
} from '../../src/dwallet-mpc/encrypt-user-share';
import {
	checkpointCreationTime,
	Config,
	delay,
	mockedNetworkDecryptionKeyPublicOutput,
} from '../../src/dwallet-mpc/globals';

const fiveMinutes = 5 * 60 * 1000;

describe('Test dWallet MPC', () => {
	let sourceConf: Config;
	let destConf: Config;

	beforeEach(async () => {
		sourceConf = await generateConfig(new Uint8Array(32).fill(8), '0x1');
		destConf = await generateConfig(new Uint8Array(32).fill(7), '0x2');
		await delay(checkpointCreationTime);
	});

	it('encrypts a secret share for a given public key and transfers it', async () => {
		const sourceDwallet = await createDWallet(sourceConf, mockedNetworkDecryptionKeyPublicOutput);
		// Create Destination Class Groups Keypair & Store it on the chain.
		await getOrCreateClassGroupsKeyPair(destConf);
		await delay(checkpointCreationTime);
		const encryptedUserKeyShareAndProofOfEncryption = await encryptUserShareForPublicKey(
			sourceConf,
			destConf.encryptedSecretShareSigningKeypair.toSuiAddress(),
			sourceDwallet.secret_share,
		);
		console.log(
			`encryptedUserKeyShareAndProofOfEncryption: ${encryptedUserKeyShareAndProofOfEncryption}`,
		);
		const encryptedShareID = await transferEncryptedSecretShare(
			sourceConf,
			destConf.encryptedSecretShareSigningKeypair.getPublicKey(),
			encryptedUserKeyShareAndProofOfEncryption,
			sourceDwallet.dwalletID,
			sourceDwallet.encrypted_secret_share_id,
		);
		await acceptEncryptedUserShare(destConf, {
			dwallet_id: sourceDwallet.dwalletID,
			encrypted_user_secret_key_share_id: encryptedShareID,
			public_output: sourceDwallet.output,
		});
		console.log(`Secret share has been transferred successfully ${encryptedShareID}`);
	});
});

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
