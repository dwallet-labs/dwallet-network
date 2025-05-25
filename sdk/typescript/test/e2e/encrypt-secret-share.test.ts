import { beforeEach, describe, it } from 'vitest';

// Removed Ed25519Keypair, SuiClient, getFullnodeUrl, getFaucetHost, requestSuiFromFaucetV1, path
// Config will be imported if still needed for explicit typing, otherwise removed too.
import { acceptEncryptedUserShare, createDWallet } from '../../src/dwallet-mpc/dkg';
import {
	decryptAndVerifyReceivedUserShare,
	encryptUserShareForPublicKey,
	getOrCreateClassGroupsKeyPair,
	transferEncryptedSecretShare,
} from '../../src/dwallet-mpc/encrypt-user-share';
import {
	checkpointCreationTime,
	Config, // Keep Config if sourceConf/destConf are explicitly typed
	delay,
	getNetworkDecryptionKeyPublicOutput,
} from '../../src/dwallet-mpc/globals';
import { generateConfig } from '../utils/test-utils'; // Import from the new location

// const fiveMinutes = 5 * 60 * 1000; // This is defined in test-utils.ts now

describe('Test dWallet MPC', () => {
	let sourceConf: Config;
	let destConf: Config;

	beforeEach(async () => {
		sourceConf = await generateConfig(new Uint8Array(32).fill(8), '0x1');
		destConf = await generateConfig(new Uint8Array(32).fill(7), '0x2');
		await delay(checkpointCreationTime);
	});

	it('encrypt a secret share for a given Sui address, decrypt it, verify it & publish signed dWallet output on chain ', async () => {
		const networkDecryptionKeyPublicOutput = await getNetworkDecryptionKeyPublicOutput(sourceConf);
		const sourceDwallet = await createDWallet(sourceConf, networkDecryptionKeyPublicOutput);
		// Create Destination Class Groups Keypair & Store it on the chain.
		await getOrCreateClassGroupsKeyPair(destConf);
		await delay(checkpointCreationTime);
		const encryptedUserKeyShareAndProofOfEncryption = await encryptUserShareForPublicKey(
			sourceConf,
			destConf.encryptedSecretShareSigningKeypair.toSuiAddress(),
			sourceDwallet.secret_share,
			networkDecryptionKeyPublicOutput,
		);
		console.log(
			`encryptedUserKeyShareAndProofOfEncryption: ${encryptedUserKeyShareAndProofOfEncryption}`,
		);
		const encryptedShareObjID = await transferEncryptedSecretShare(
			sourceConf,
			destConf.encryptedSecretShareSigningKeypair.toSuiAddress(),
			encryptedUserKeyShareAndProofOfEncryption,
			sourceDwallet.dwalletID,
			sourceDwallet.encrypted_secret_share_id,
		);
		const encryptedDWalletData = {
			dwallet_id: sourceDwallet.dwalletID,
			encrypted_user_secret_key_share_id: encryptedShareObjID,
		};
		const decryptedSecretShare = await decryptAndVerifyReceivedUserShare(
			destConf,
			encryptedDWalletData,
			sourceConf.encryptedSecretShareSigningKeypair.toSuiAddress(),
			networkDecryptionKeyPublicOutput,
		);
		console.log(`decryptedSecretShare: ${decryptedSecretShare}`);
		await acceptEncryptedUserShare(destConf, encryptedDWalletData);
		console.log(`Secret share has been transferred successfully ${encryptedShareObjID}`);
	});
});
