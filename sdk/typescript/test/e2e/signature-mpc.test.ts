// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { beforeAll, describe, it } from 'vitest';

import {
	approveAndSign,
	createDWallet,
	createPartialUserSignedMessages,
	generate_keypair,
	generate_proof,
	getEncryptionKeyByObjectId,
	storeEncryptionKey,
	transferDwallet,
	EncryptionKeyScheme,
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

describe('Test key share transfer', () => {
	let toolbox: TestToolbox;

	beforeAll(async () => {
		toolbox = await setup();
	});

	it('should encrypt and transfer a dwallet to a newly generated public key', async () => {
		const [pub_key, _] = generate_keypair();
		const pubKeyRef = await storeEncryptionKey(
			pub_key,
			EncryptionKeyScheme.Paillier,
			toolbox.keypair,
			toolbox.client,
		);
		const publicKeyID = pubKeyRef?.objectId;
		const recipientData = await getEncryptionKeyByObjectId(toolbox.client, publicKeyID);

		// Before running this test, you need to create a dwallet and out its object ID and secret share here.
		const secretKeyshare = '3D753F12D01268B1433482A11E5ADC7BB409C7DB135C48969E911D1E954D7751';
		const dwalletID = '0x5a94ad49edfbae341afec2d2169751e56e1366d8bfb0aba7cd7b54fde98ea5b9';

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
			recipientData?.keyOwnerAddress!,
		);
	});
});
