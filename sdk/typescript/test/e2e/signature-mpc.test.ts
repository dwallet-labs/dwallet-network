// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { beforeAll, describe, it } from 'vitest';

import {
	approveAndSign,
	createActiveEncryptionKeysTable,
	createDWallet,
	createPartialUserSignedMessages,
} from '../../src/signature-mpc';
import { getOrCreateEncryptionKey } from '../../src/signature-mpc/encrypt_user_share';
import { presignWithDWalletID } from '../../src/signature-mpc/sign';
import { setup, TestToolbox } from './utils/setup';

describe('Test signature mpc', () => {
	let toolbox: TestToolbox;
	let activeEncryptionKeysTableID: string;

	beforeAll(async () => {
		toolbox = await setup();
		const encryptionKeysHolder = await createActiveEncryptionKeysTable(
			toolbox.client,
			toolbox.keypair,
		);
		activeEncryptionKeysTableID = encryptionKeysHolder.objectId;
	});

	it('the signature mpc create dwallet', async () => {
		const encryptionKeyObj = await getOrCreateEncryptionKey(
			toolbox.keypair,
			toolbox.client,
			activeEncryptionKeysTableID,
		);

		const dkg = await createDWallet(
			toolbox.keypair,
			toolbox.client,
			encryptionKeyObj.encryptionKey,
			encryptionKeyObj.objectID,
		);

		const bytes: Uint8Array = new TextEncoder().encode('Sign it!!!');

		const signMessagesIdSHA256 = await createPartialUserSignedMessages(
			dkg?.dwalletID!,
			dkg?.decentralizedDKGOutput!,
			new Uint8Array(dkg?.secretKeyShare!),
			[bytes],
			'SHA256',
			toolbox.keypair,
			toolbox.client,
		);
		const sigSHA256 = await approveAndSign(
			dkg?.dwalletCapID!,
			signMessagesIdSHA256!,
			[bytes],
			dkg?.dwalletID!,
			'SHA256',
			toolbox.keypair,
			toolbox.client,
		);

		console.log('sigSHA256:');
		console.log(sigSHA256);

		const signMessagesIdKECCAK256 = await createPartialUserSignedMessages(
			dkg?.dwalletID!,
			dkg?.decentralizedDKGOutput!,
			new Uint8Array(dkg?.secretKeyShare!),
			[bytes],
			'KECCAK256',
			toolbox.keypair,
			toolbox.client,
		);
		const sigKECCAK256 = await approveAndSign(
			dkg?.dwalletCapID!,
			signMessagesIdKECCAK256!,
			[bytes],
			dkg?.dwalletID!,
			'KECCAK256',
			toolbox.keypair,
			toolbox.client,
		);

		console.log('sigKECCAK256:');
		console.log(sigKECCAK256);
	});

	it('should sign a message with a dwallet by dwallet ID', async () => {
		let encryptionKeyObj = await getOrCreateEncryptionKey(
			toolbox.keypair,
			toolbox.client,
			activeEncryptionKeysTableID,
		);
		const dwallet = await createDWallet(
			toolbox.keypair,
			toolbox.client,
			encryptionKeyObj.encryptionKey,
			encryptionKeyObj.objectID,
		);
		const message: Uint8Array = new TextEncoder().encode('Sign it!!!');
		let presignObjID = await presignWithDWalletID(
			toolbox.client,
			toolbox.keypair,
			dwallet?.dwalletID!,
			message,
			'SHA256',
			activeEncryptionKeysTableID,
		);
		let signatures = await approveAndSign(
			dwallet?.dwalletCapID!,
			presignObjID!,
			[message],
			dwallet?.dwalletID!,
			'SHA256',
			toolbox.keypair,
			toolbox.client,
		);
		console.log({ signatures });
	});
});
