// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { create_sign_centralized_output } from '@dwallet-network/dwallet-mpc-wasm';
import { beforeAll, describe, it } from 'vitest';

import { createDWallet } from '../../src/dwallet-mpc/dkg';
import { mockedDWallet, mockedPresign } from '../../src/dwallet-mpc/mock';
import { presign } from '../../src/dwallet-mpc/presign';
import { Hash, signMessageTransactionCall, signMockCall } from '../../src/dwallet-mpc/sign';
import { setup, TestToolbox } from './utils/setup';

describe('Test dwallet mpc', () => {
	let toolbox: TestToolbox;

	beforeAll(async () => {
		toolbox = await setup();
	});

	it('should create DWallet', async () => {
		console.log(toolbox.keypair.toPeraAddress());
		const dwallet = await createDWallet(toolbox.keypair, toolbox.client);
		console.log(dwallet);
	});

	it('should create presign', async () => {
		console.log(toolbox.keypair.toPeraAddress());
		const dwallet = await createDWallet(toolbox.keypair, toolbox.client);
		console.log({ dwallet });
		const presignOutput = await presign(toolbox.keypair, toolbox.client, dwallet!.dwalletID);
		console.log({ presignOutput });
	});

	it('should sign message', async () => {
		console.log(toolbox.keypair.toPeraAddress());
		const dwallet = await createDWallet(toolbox.keypair, toolbox.client);
		console.log({ dwallet });
		const presignOutput = await presign(toolbox.keypair, toolbox.client, dwallet!.dwalletID);
		console.log({ presignOutput });

		const [sign_msg, _, hash_msg] = create_sign_centralized_output(
			Uint8Array.from(dwallet?.centralizedDKGOutput!),
			Uint8Array.from(presignOutput?.encryptionOfMaskAndMaskedKeyShare!),
			Uint8Array.from(presignOutput?.noncePublicShareAndEncryptionOfMaskedNonce!),
			Uint8Array.from([1, 2, 3, 4, 5]),
			Hash.SHA256,
			presignOutput?.presignFirstRoundOutputId.slice(2)!,
		);

		let res = await signMessageTransactionCall(
			toolbox.keypair,
			toolbox.client,
			dwallet?.dwalletCapID!,
			hash_msg,
			dwallet?.dwalletID!,
			presignOutput?.presignFirstRoundOutputId!,
			presignOutput?.presignSecondRoundOutputId!,
			sign_msg,
			presignOutput?.presignFirstRoundSessionId!,
		);

		console.log(res);
	});

	it('should sign a message successfully with mock ', async () => {
		console.log(toolbox.keypair.toPeraAddress());
		const [sign_msg, _, hash_msg] = create_sign_centralized_output(
			Uint8Array.from(mockedDWallet.centralizedDKGOutput),
			Uint8Array.from(mockedPresign.firstRoundOutput),
			Uint8Array.from(mockedPresign.secondRoundOutput),
			Uint8Array.from([1, 2, 3, 4, 5]),
			Hash.SHA256,
			mockedPresign.firstRoundSessionID.slice(2)!,
		);

		let res = await signMockCall(
			toolbox.keypair,
			toolbox.client,
			hash_msg,
			mockedPresign.firstRoundOutput,
			mockedPresign.secondRoundOutput,
			mockedDWallet.decentralizedDKGOutput,
			sign_msg,
			mockedPresign.firstRoundSessionID,
		);

		console.log(res);
	});
});
