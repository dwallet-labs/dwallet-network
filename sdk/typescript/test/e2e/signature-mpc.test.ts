// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { bcs } from '@mysten/bcs';
import { beforeAll, describe, expect, it } from 'vitest';

import {
	approveAndSign,
	createDWallet,
	createPartialUserSignedMessages,
	decrypt_user_share,
	EncryptionKeyScheme,
	generate_keypair,
	generate_proof,
	getEncryptionKeyByObjectId,
	storeEncryptionKey,
	transferDwallet,
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
		const [encryption_key, _] = generate_keypair();
		const pubKeyRef = await storeEncryptionKey(
			encryption_key,
			EncryptionKeyScheme.Paillier,
			toolbox.keypair,
			toolbox.client,
		);
		console.log({ pubKeyRef });
	});
});

function bytesToHex(bytes: number[]): string {
	return bytes.map((byte) => byte.toString(16).padStart(2, '0')).join('');
}

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
		let DKGOutput = bcs.struct('Output', {
			secret_key_share: bcs.vector(bcs.u8()),
		});
		const dwallet = await createDWallet(toolbox.keypair, toolbox.client);
		// let parsed_output = DKGOutput.parse(new Uint8Array(dwallet?.dkgOutput));
		// Before running this test, you need to create a dwallet and out its object ID and secret share here.
		const dwalletID = dwallet?.dwalletId!;

		const encryptedUserShareAndProof = generate_proof(
			dwallet?.secret_key_share!,
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

		expect(decryptedKeyshare).toEqual(dwallet?.secret_key_share!);
	});
});
