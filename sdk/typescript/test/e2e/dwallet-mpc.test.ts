// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { create_sign_centralized_output } from '@dwallet-network/dwallet-mpc-wasm';
import { beforeAll, describe, it } from 'vitest';

import { createDWallet } from '../../src/dwallet-mpc/dkg';
import { presign } from '../../src/dwallet-mpc/presign';
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
		const presignOutput = await presign(toolbox.keypair, toolbox.client, dwallet!.dwalletID);
		const res = create_sign_centralized_output(
			Uint8Array.from(dwallet?.centralizedDKGOutput),
			Uint8Array.from(presignOutput!.encryptionOfMaskAndMaskedKeyShare),
			Uint8Array.from(presignOutput!.noncePublicShareAndEncryptionOfMaskedNonce),
			Uint8Array.from([1, 2, 3, 4, 5]),
			0,
			dwallet?.dwalletID.slice(2)!,
		);
		console.log(res);
	});
});
