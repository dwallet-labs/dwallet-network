import {
	decrypt_user_share,
	encrypt_secret_share,
	verify_user_share,
} from '@dwallet-network/dwallet-mpc-wasm';
import { beforeAll, describe, expect, it } from 'vitest';

import {
	acceptUserShare,
	createActiveEncryptionKeysTable,
	generateCGKeyPairFromSuiKeyPair,
	getActiveEncryptionKeyObjID,
	getOrCreateEncryptionKey,
	encryptUserShareWithSuiPubKey,
} from '../../src/dwallet-mpc/encrypt-user-share';
import { Config } from '../../src/dwallet-mpc/globals';
import { Ed25519Keypair } from '../../src/keypairs/ed25519';
import {
	DKGCentralizedPrivateOutput,
	DKGDecentralizedOutput,
	mockCreateDwallet,
} from './utils/dwallet';
import { setup, TestToolbox } from './utils/setup';

describe('encrypt user share', () => {
	let dwalletSenderToolbox: TestToolbox;
	let dwalletReceiverToolbox: TestToolbox;
	let activeEncryptionKeysTableID: string;

	beforeAll(async () => {
		dwalletSenderToolbox = await setup();
		dwalletReceiverToolbox = await setup();
		const encryptionKeysRef = await createActiveEncryptionKeysTable(
			dwalletSenderToolbox.client,
			dwalletSenderToolbox.keypair,
		);
		activeEncryptionKeysTableID = encryptionKeysRef.objectId;
	});

	it('encrypts a secret share to a given Sui public key', async () => {
		const checkpointCreationTime = 2000;
		let senderConf: Config = {
			keypair: dwalletSenderToolbox.keypair,
			client: dwalletSenderToolbox.client,
			timeout: 5 * 60 * 1000,
		};

		let receiverConf: Config = {
			keypair: dwalletReceiverToolbox.keypair,
			client: dwalletReceiverToolbox.client,
			timeout: 5 * 60 * 1000,
		};
		await new Promise((r) => setTimeout(r, checkpointCreationTime));

		// ======================= Create Source DWallet =======================
		const createdDwallet = await mockCreateDwallet(senderConf);

		// ======================= Create Destination Class Groups Keypair & Store it on chain =======================
		await getOrCreateEncryptionKey(receiverConf, activeEncryptionKeysTableID);
		await new Promise((r) => setTimeout(r, checkpointCreationTime));

		// ======================= Send DWallet Secret Share To Destination Keypair  =======================
		let encryptedSecretShare = await encryptUserShareWithSuiPubKey(
			senderConf,
			createdDwallet,
			dwalletReceiverToolbox.keypair.getPublicKey(),
			activeEncryptionKeysTableID,
		);

		// ======================= Verify Received DWallet is Valid & Encrypt it to Myself =======================
		await acceptUserShare(
			encryptedSecretShare,
			senderConf.keypair.toPeraAddress(),
			activeEncryptionKeysTableID,
			receiverConf,
		);
	});

	it('creates an encryption key & stores it in the active encryption keys table', async () => {
		let conf: Config = {
			keypair: dwalletSenderToolbox.keypair,
			client: dwalletSenderToolbox.client,
			timeout: 5 * 60 * 1000,
		};
		const senderEncryptionKeyObj = await getOrCreateEncryptionKey(
			conf,
			activeEncryptionKeysTableID,
		);

		// Sleep for 5 seconds, so the getOrCreateEncryptionKey inner transactions effects have time to
		// get written to the chain.
		await new Promise((r) => setTimeout(r, 5000));

		const activeEncryptionKeyAddress = await getActiveEncryptionKeyObjID(
			conf,
			conf.keypair.toPeraAddress(),
			activeEncryptionKeysTableID,
		);

		expect(`0x${activeEncryptionKeyAddress}`).toEqual(senderEncryptionKeyObj.objectID);
	});
});

// tests that can run without the blockchain running.
describe('encrypt user share — offline', () => {
	it("successfully encrypts a secret share, decrypt it, and verify the decrypted share is matching the DWallet's public share", () => {
		let keypair = Ed25519Keypair.generate();
		let [encryptionKey, decryptionKey] = generateCGKeyPairFromSuiKeyPair(keypair);
		let dwallet_secret_key_share = Array.from(Buffer.from(DKGCentralizedPrivateOutput, 'base64'));
		let encryptedUserShareAndProof = encrypt_secret_share(
			new Uint8Array(dwallet_secret_key_share),
			encryptionKey,
		);
		let decrypted = decrypt_user_share(encryptionKey, decryptionKey, encryptedUserShareAndProof);
		expect(decrypted).toEqual(dwallet_secret_key_share);
		let is_valid = verify_user_share(
			decrypted,
			new Uint8Array(Array.from(Buffer.from(DKGDecentralizedOutput, 'base64'))),
		);
		expect(is_valid).toBeTruthy();
	});
});
