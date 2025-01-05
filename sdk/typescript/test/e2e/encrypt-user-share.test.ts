import { beforeAll, describe, expect, it } from 'vitest';

import {
	createActiveEncryptionKeysTable,
	encryptedSecretShareMoveType,
	EncryptedUserShare,
	getActiveEncryptionKeyObjID,
	getOrCreateEncryptionKey,
	isEncryptedUserShare,
	sendUserShareToSuiPubKey,
} from '../../src/dwallet-mpc/encrypt-user-share';
import { Config, fetchObjectWithType } from '../../src/dwallet-mpc/globals';
import { mockCreateDwallet } from './utils/dwallet';
import { setup, TestToolbox } from './utils/setup';

describe('encrypt user share', () => {
	let dwalletSenderToolbox: TestToolbox;
	let dwalletReceiverToolbox: TestToolbox;
	let activeEncryptionKeysTableID: string;

	beforeAll(async () => {
		dwalletSenderToolbox = await setup();
		dwalletReceiverToolbox = await setup();
		const encryptionKeysHolder = await createActiveEncryptionKeysTable(
			dwalletSenderToolbox.client,
			dwalletSenderToolbox.keypair,
		);
		activeEncryptionKeysTableID = encryptionKeysHolder.objectId;
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

		// ======================= Create Destination Class Groups Keypair =======================
		await getOrCreateEncryptionKey(receiverConf, activeEncryptionKeysTableID);
		await new Promise((r) => setTimeout(r, checkpointCreationTime));

		// ======================= Send DWallet Secret Share To Destination Keypair  =======================
		let encryptedUserShareObjID = await sendUserShareToSuiPubKey(
			senderConf,
			createdDwallet,
			dwalletReceiverToolbox.keypair.getPublicKey(),
			activeEncryptionKeysTableID,
		);

		// ======================= Fetch the received DWallet =======================
		await new Promise((r) => setTimeout(r, checkpointCreationTime));
		let encryptedUserShare = await fetchObjectWithType<EncryptedUserShare>(
			receiverConf,
			encryptedSecretShareMoveType,
			isEncryptedUserShare,
			encryptedUserShareObjID,
		);
		// TODO (#467): Decrypt the encrypted user share and verify it is valid.
		expect(encryptedUserShare).toBeDefined();
	});

	it('creates an encryption key & stores it in the active encryption keys table', async () => {
		let conf: Config = {
			keypair: dwalletSenderToolbox.keypair,
			client: dwalletSenderToolbox.client,
			timeout: 5 * 60 * 1000,
		};
		let senderEncryptionKeyObj = await getOrCreateEncryptionKey(conf, activeEncryptionKeysTableID);

		// Sleep for 5 seconds so the getOrCreateEncryptionKey inner transactions effects has time to
		// get written to the chain.
		await new Promise((r) => setTimeout(r, 5000));

		const activeEncryptionKeyAddress = await getActiveEncryptionKeyObjID(
			conf,
			conf.keypair.toPeraAddress(),
			activeEncryptionKeysTableID,
		);

		expect(`0x${activeEncryptionKeyAddress}`).toEqual(senderEncryptionKeyObj.objectID!);
	});
});
