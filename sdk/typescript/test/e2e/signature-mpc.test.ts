// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { beforeAll, describe, it } from 'vitest';

import {
	approveAndSign,
	createDWallet,
	createPartialUserSignedMessages,
	EncryptionKeyScheme,
	storeEncryptionKey,
} from '../../src/signature-mpc';
import { generatePaillierKeyPairFromSuiKeyPair } from '../../src/signature-mpc/utils';
import { setup, TestToolbox } from './utils/setup';

describe('Test signature mpc', () => {
	let toolbox: TestToolbox;

	beforeAll(async () => {
		toolbox = await setup();
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
			dkg?.centralizedDKGOutput!,
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
			dkg?.centralizedDKGOutput!,
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
