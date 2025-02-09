import {
	decrypt_user_share,
	encrypt_secret_share,
	verify_user_share,
} from '@dwallet-network/dwallet-mpc-wasm';
import { beforeAll, describe, expect, it } from 'vitest';

import { createDWallet } from '../../src/dwallet-mpc/dkg';
import {
	createActiveEncryptionKeysTable,
	EncryptedUserShare,
	generateCGKeyPairFromSuiKeyPair,
} from '../../src/dwallet-mpc/encrypt-user-share';
import { Config, delay, mockedProtocolPublicParameters } from '../../src/dwallet-mpc/globals';
import { signWithEncryptedDWallet } from '../../src/dwallet-mpc/sign_ecdsa_k1';
import { Ed25519Keypair } from '../../src/keypairs/ed25519';
import {
	DKGCentralizedPrivateOutput,
	DKGDecentralizedPublicOutput,
	mockCreateDwallet,
} from './utils/dwallet_mocks';
import { setup, TestToolbox } from './utils/setup';

const checkpointCreationTime = 2000;

describe('encrypt user share', () => {
	let sourceClient: TestToolbox;
	let destClient: TestToolbox;
	let activeEncryptionKeysTableID: string;
	const fiveMinutes = 5 * 60 * 1000;

	beforeAll(async () => {
		sourceClient = await setup();
		destClient = await setup();
		activeEncryptionKeysTableID = (
			await createActiveEncryptionKeysTable({
				keypair: sourceClient.keypair,
				client: sourceClient.client,
				timeout: fiveMinutes,
			})
		).objectId;
		await delay(checkpointCreationTime);
	});

	it('encrypts a secret share for a given public key and transfers it', async () => {
		const encryptedUserShare = new EncryptedUserShare(sourceClient.client, fiveMinutes);

		const sourceDwallet = await mockCreateDwallet(
			encryptedUserShare.toConfig(sourceClient.keypair),
		);

		// Create Destination Class Groups Keypair & Store it on the chain.
		await encryptedUserShare.getOrCreateClassGroupsKeyPair(
			destClient.keypair,
			activeEncryptionKeysTableID,
		);
		await delay(checkpointCreationTime);

		const { destActiveEncryptionKeyObjID, encryptedUserKeyShareAndProofOfEncryption } =
			await encryptedUserShare.encryptUserShareForPublicKey(
				sourceClient.keypair,
				destClient.keypair.getPublicKey(),
				sourceDwallet,
				activeEncryptionKeysTableID,
			);

		const encryptedSecretShare = await encryptedUserShare.transferEncryptedUserSecretShare(
			sourceClient.keypair,
			encryptedUserKeyShareAndProofOfEncryption,
			destActiveEncryptionKeyObjID,
			sourceDwallet,
		);
		expect(encryptedSecretShare).toBeDefined();
		console.log({ encryptedSecretShare });

		// Verifies that the received dWallet is valid and encrypt it to myself.
		const createdEncryptedSecretShareEvent = await encryptedUserShare.acceptUserShare(
			activeEncryptionKeysTableID,
			encryptedSecretShare,
			sourceClient.keypair.toPeraAddress(),
			destClient.keypair,
		);
		expect(createdEncryptedSecretShareEvent).toBeDefined();
		console.log({ createdEncryptedSecretShareEvent });
	});

	it('creates an encryption key and stores it in the active encryption keys table', async () => {
		const encryptedUserShare = new EncryptedUserShare(sourceClient.client, fiveMinutes);

		const senderEncryptionKeyObj = await encryptedUserShare.getOrCreateClassGroupsKeyPair(
			sourceClient.keypair,
			activeEncryptionKeysTableID,
		);

		// Sleep for 5 seconds, so the getOrCreateEncryptionKey inner transactions effects have time to
		// get written to the chain.
		await delay(5000);

		const activeEncryptionKeyAddress = await encryptedUserShare.getActiveEncryptionKeyObjID(
			sourceClient.keypair.getPublicKey(),
			activeEncryptionKeysTableID,
		);

		expect(`0x${activeEncryptionKeyAddress}`).toEqual(senderEncryptionKeyObj.objectID);
	});

	it('signs with an encrypted secret share', async () => {
		console.log(sourceClient.keypair.toPeraAddress());
		const conf: Config = {
			keypair: sourceClient.keypair,
			client: sourceClient.client,
			timeout: fiveMinutes,
		};
		const dwallet = await createDWallet(
			conf,
			mockedProtocolPublicParameters,
			activeEncryptionKeysTableID,
		);
		expect(dwallet).toBeDefined();
		console.log({ dwallet });
		const messages = [Uint8Array.from([1, 2, 3, 4, 5]), Uint8Array.from([6, 7, 8, 9, 10])];
		const mockNetworkKey = true;
		const completion = await signWithEncryptedDWallet(
			conf,
			dwallet.id.id,
			activeEncryptionKeysTableID,
			messages,
			mockNetworkKey,
		);
		expect(completion).toBeDefined();
		console.log({ completion });
	});
});

// tests that can run without the blockchain running.
describe('encrypt user share — offline', () => {
	it("successfully encrypts a secret share, decrypt it, and verify the decrypted share is matching the dWallets' public share", () => {
		const keypair = Ed25519Keypair.generate();
		const [encryptionKey, decryptionKey] = generateCGKeyPairFromSuiKeyPair(keypair);
		const dwalletSecretKeyShare = Array.from(Buffer.from(DKGCentralizedPrivateOutput, 'base64'));
		const encryptedUserKeyShareAndProofOfEncryption = encrypt_secret_share(
			new Uint8Array(dwalletSecretKeyShare),
			encryptionKey,
		);
		const decrypted = decrypt_user_share(
			encryptionKey,
			decryptionKey,
			encryptedUserKeyShareAndProofOfEncryption,
		);
		expect(decrypted).toEqual(dwalletSecretKeyShare);
		const is_valid = verify_user_share(
			decrypted,
			new Uint8Array(Array.from(Buffer.from(DKGDecentralizedPublicOutput, 'base64'))),
		);
		expect(is_valid).toBeTruthy();
	});
});
