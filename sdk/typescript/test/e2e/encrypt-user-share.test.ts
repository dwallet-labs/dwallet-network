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
		const encryptionKeysHolder = await createActiveEncryptionKeysTable(
			dwalletSenderToolbox.client,
			dwalletSenderToolbox.keypair,
		);
		activeEncryptionKeysTableID = encryptionKeysHolder.objectId;
	});

	it('creates an encryption key & stores it in the active encryption keys table', async () => {
		let conf: Config = {
			keypair: dwalletSenderToolbox.keypair,
			client: dwalletSenderToolbox.client,
			timeout: 5 * 60 * 1000,
		};
		let senderEncryptionKeyObj = await getOrCreateEncryptionKey(
			conf,
			activeEncryptionKeysTableID,
		);

		// Sleep for 5 seconds so the getOrCreateEncryptionKey inner transactions effects has time to
		// get written to the chain.
		await new Promise((r) => setTimeout(r, 5000));

		const activeEncryptionKeyAddress = await getActiveEncryptionKeyObjID(
			conf,
			activeEncryptionKeysTableID,
		);

		expect(`0x${activeEncryptionKeyAddress}`).toEqual(senderEncryptionKeyObj.objectID!);
	});
});
