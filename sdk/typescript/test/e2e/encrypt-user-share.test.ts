import { beforeAll, describe, expect, it } from 'vitest';

import {
	createActiveEncryptionKeysTable,
	getActiveEncryptionKeyObjID,
	getOrCreateEncryptionKey,
} from '../../src/dwallet-mpc/encrypt-user-share';
import { Config } from '../../src/dwallet-mpc/globals';
import { setup, TestToolbox } from './utils/setup';

describe('encrypt user share', () => {
	let dwalletSenderToolbox: TestToolbox;
	let activeEncryptionKeysTableID: string;

	beforeAll(async () => {
		dwalletSenderToolbox = await setup();
		const encryptionKeysRef = await createActiveEncryptionKeysTable(
			dwalletSenderToolbox.client,
			dwalletSenderToolbox.keypair,
		);
		activeEncryptionKeysTableID = encryptionKeysRef.objectId;
	});

	it('creates an encryption key and stores it in the active encryption keys table', async () => {
		const conf: Config = {
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
			activeEncryptionKeysTableID,
		);

		expect(`0x${activeEncryptionKeyAddress}`).toEqual(senderEncryptionKeyObj.objectID);
	});
});
