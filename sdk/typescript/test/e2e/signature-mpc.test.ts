// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { beforeAll, describe, it } from 'vitest';

import {
	approveAndSign,
	createActiveEncryptionKeysTable,
	createDWallet,
	createPartialUserSignedMessages,
	EncryptionKeyScheme,
	storeEncryptionKey,
} from '../../src/signature-mpc';
import { getOrCreateEncryptionKey } from '../../src/signature-mpc/encrypt_user_share';
import { presignWithDWalletID } from '../../src/signature-mpc/sign';
import { generatePaillierKeyPairFromSuiKeyPair } from '../../src/signature-mpc/utils';
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
		const [walletCreatorEncryptionKey, _] = generatePaillierKeyPairFromSuiKeyPair(toolbox.keypair);

		const pubKeyRef = await storeEncryptionKey(
			walletCreatorEncryptionKey,
			EncryptionKeyScheme.Paillier,
			toolbox.keypair,
			toolbox.client,
		);

		const dkg = await createDWallet(
			toolbox.keypair,
			toolbox.client,
			walletCreatorEncryptionKey,
			pubKeyRef.objectId,
		);

		const bytes: Uint8Array = new TextEncoder().encode('Sign it!!!');

		const signMessagesIdSHA256 = await createPartialUserSignedMessages(
			dkg?.dwalletId!,
			dkg?.decentralizedDKGOutput!,
			new Uint8Array(dkg?.secretKeyShare!),
			[bytes],
			'SHA256',
			toolbox.keypair,
			toolbox.client,
		);
		const sigSHA256 = await approveAndSign(
			dkg?.dwalletCapId!,
			signMessagesIdSHA256!,
			[bytes],
			dkg?.dwalletId!,
			'SHA256',
			toolbox.keypair,
			toolbox.client,
		);

		console.log('sigSHA256:');
		console.log(sigSHA256);

		const signMessagesIdKECCAK256 = await createPartialUserSignedMessages(
			dkg?.dwalletId!,
			dkg?.decentralizedDKGOutput!,
			new Uint8Array(dkg?.secretKeyShare!),
			[bytes],
			'KECCAK256',
			toolbox.keypair,
			toolbox.client,
		);
		const sigKECCAK256 = await approveAndSign(
			dkg?.dwalletCapId!,
			signMessagesIdKECCAK256!,
			[bytes],
			dkg?.dwalletId!,
			'KECCAK256',
			toolbox.keypair,
			toolbox.client,
		);

		console.log('sigKECCAK256:');
		console.log(sigKECCAK256);
	});

	it('should sign a message with a dwallet by dwallet id', async () => {
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
			dwallet?.dwalletId!,
			message,
			'SHA256',
			activeEncryptionKeysTableID,
		);
		let signatures = await approveAndSign(
			dwallet?.dwalletCapId!,
			presignObjID!,
			[message],
			dwallet?.dwalletId!,
			'SHA256',
			toolbox.keypair,
			toolbox.client,
		);
		console.log({ signatures });
	});
});
