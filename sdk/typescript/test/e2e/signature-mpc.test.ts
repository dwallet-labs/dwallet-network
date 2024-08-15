// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { beforeAll, describe, expect, it } from "vitest";

import {
	approveAndSign,
	createDWallet,
	createEncryptionKeysHolder,
	createPartialUserSignedMessages,
	decrypt_user_share,
	EncryptionKeyScheme,
	generate_keypair,
	generate_proof, getDwalletActiveEncryptionKey,
	getEncryptionKeyByObjectId,
	setDwalletPrimaryEncryptionKey,
	storeEncryptionKey,
	transferDwallet,
} from "../../src/signature-mpc";
import { setup, TestToolbox } from './utils/setup';

describe('Test signature mpc', () => {
	let toolbox: TestToolbox;

	beforeAll(async () => {
		toolbox = await setup();
	});

	it('the signature mpc create dwallet', async () => {
		console.log(toolbox.keypair.toSuiAddress());
		const dkg = await createDWallet(toolbox.keypair, toolbox.client);

		const bytes: Uint8Array = new TextEncoder().encode('Sign it!!!');

		const signMessagesIdSHA256 = await createPartialUserSignedMessages(
			dkg?.dwalletId!,
			dkg?.dkgOutput,
			[bytes],
			'SHA256',
			toolbox.keypair,
			toolbox.client,
		);
		const sigSHA256 = await approveAndSign(
			dkg?.dwalletCapId!,
			signMessagesIdSHA256!,
			[bytes],
			toolbox.keypair,
			toolbox.client,
		);

		console.log('sigSHA256:');
		console.log(sigSHA256);

		const signMessagesIdKECCAK256 = await createPartialUserSignedMessages(
			dkg?.dwalletId!,
			dkg?.dkgOutput,
			[bytes],
			'KECCAK256',
			toolbox.keypair,
			toolbox.client,
		);
		const sigKECCAK256 = await approveAndSign(
			dkg?.dwalletCapId!,
			signMessagesIdKECCAK256!,
			[bytes],
			toolbox.keypair,
			toolbox.client,
		);

		console.log('sigKECCAK256:');
		console.log(sigKECCAK256);
	});
});

describe('Create public key', () => {
	let toolbox: TestToolbox;

	beforeAll(async () => {
		toolbox = await setup();
	});

	it('the signature mpc create dwallet', async () => {
		const [encryptionKey, _] = generate_keypair();
		const pubKeyRef = await storeEncryptionKey(
			encryptionKey,
			EncryptionKeyScheme.Paillier,
			toolbox.keypair,
			toolbox.client,
		);
		console.log({ pubKeyRef });

		const encryptionKeysHolder = await createEncryptionKeysHolder(toolbox.client, toolbox.keypair);

		await setDwalletPrimaryEncryptionKey(
			toolbox.client,
			toolbox.keypair,
			pubKeyRef?.objectId!,
			encryptionKeysHolder.objectId,
		);

		const activeEncryptionKey = await getDwalletActiveEncryptionKey(
			toolbox.client,
			toolbox.keypair,
			encryptionKeysHolder.objectId,
		);

		const activeKeyHex = Buffer.from(new Uint8Array(activeEncryptionKey)).toString('hex');
		expect(`0x${activeKeyHex}`).toEqual(pubKeyRef?.objectId!);
	});
});

describe('Test key share transfer', () => {
	let toolbox: TestToolbox;

	beforeAll(async () => {
		toolbox = await setup();
	});

	it('should encrypt and transfer a dwallet to a newly generated public key', async () => {
		const [encryptionKey, decryptionKey] = generate_keypair();
		const pubKeyRef = await storeEncryptionKey(
			encryptionKey,
			EncryptionKeyScheme.Paillier,
			toolbox.keypair,
			toolbox.client,
		);
		const publicKeyID = pubKeyRef?.objectId;
		const recipientData = await getEncryptionKeyByObjectId(toolbox.client, publicKeyID);

		// Before running this test, you need to create a dwallet and out its object ID and secret share here.
		const secretKeyshare = '<SECRET_KEYSHARE>';
		const dwalletID = '<DWALLET_OBJECT_ID>';

		let parsedKeyshare = Uint8Array.from(Buffer.from(secretKeyshare, 'hex'));

		const encryptedUserShareAndProof = generate_proof(
			parsedKeyshare,
			recipientData?.encryptionKey!,
		);

		await transferDwallet(
			toolbox.client,
			toolbox.keypair,
			encryptedUserShareAndProof,
			publicKeyID,
			dwalletID,
		);

		const decryptedKeyshare = decrypt_user_share(
			encryptionKey,
			decryptionKey,
			encryptedUserShareAndProof,
		);

		let secretUserShare = new Uint8Array(256);
		secretUserShare.set(Uint8Array.from(Buffer.from(secretKeyshare, 'hex')).reverse());
		expect(decryptedKeyshare).toEqual(secretUserShare);
	});
});
