// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { verify_user_share } from '@dwallet-network/signature-mpc-wasm';
import { beforeAll, describe, expect, it } from 'vitest';

import {
	approveAndSign,
	createActiveEncryptionKeysTable,
	createDWallet,
	createPartialUserSignedMessages,
	decrypt_user_share,
	EncryptionKeyScheme,
	encryptUserShare,
	generate_keypair,
	generate_proof,
	getActiveEncryptionKey,
	getEncryptionKeyByObjectId,
	setActiveEncryptionKey,
	storeEncryptionKey,
} from '../../src/signature-mpc';
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
			dkg?.centralizedDKGOutput,
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
			dkg?.centralizedDKGOutput,
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

		const encryptionKeysHolder = await createActiveEncryptionKeysTable(toolbox.client, toolbox.keypair);

		await setActiveEncryptionKey(
			toolbox.client,
			toolbox.keypair,
			pubKeyRef?.objectId!,
			encryptionKeysHolder.objectId,
		);

		const activeEncryptionKey = await getActiveEncryptionKey(
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
		const dwallet = await createDWallet(toolbox.keypair, toolbox.client);
		const dwalletID = dwallet?.dwalletId!;
		const secretShare = dwallet?.secretKeyShare!;
		const encryptedUserShareAndProof = generate_proof(secretShare, recipientData?.encryptionKey!);

		await encryptUserShare(
			toolbox.client,
			toolbox.keypair,
			encryptedUserShareAndProof,
			publicKeyID,
			dwalletID,
		);

		const decryptedKeyShare = decrypt_user_share(
			encryptionKey,
			decryptionKey,
			encryptedUserShareAndProof,
		);

		let secretUserShare = new Uint8Array(256);
		secretUserShare.set(secretShare.reverse());
		expect(decryptedKeyShare).toEqual(secretUserShare);

		expect(
			verify_user_share(
				// Take the first 32 bytes, the only ones that are non-zero, and reverse them to convert them
				// from little-endian encoding to big-endian.
				// This is because of BCS and PlaintextSpaceGroupElement serialization.
				// PlaintextSpaceGroupElement is U2048 and has 32LIMBS of 64 bits each.
				new Uint8Array(decryptedKeyShare.slice(0, 32).reverse()),
				new Uint8Array(dwallet?.decentralizedDKGOutput!),
			),
		).toBeTruthy();
	});
});
