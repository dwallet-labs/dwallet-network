// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { hello_wasm } from '@dwallet-network/signature-mpc-wasm';
import { beforeAll, describe, it } from 'vitest';

import { launchDKGSession, launchProofMPSession } from '../../src/signature-mpc/proof';
import { setup, TestToolbox } from './utils/setup';

describe('Test signature mpc', () => {
	let toolbox: TestToolbox;

	beforeAll(async () => {
		toolbox = await setup();
	});

	// it('should create proof MPC Event', async () => {
	// 	await launchProofMPSession(toolbox.keypair, toolbox.client);
	// 	console.log(toolbox.keypair.toPeraAddress());
	// });

	it('should create dkg MPC Event', async () => {
		// await launchDKGSession(toolbox.keypair, toolbox.client);
		// console.log(toolbox.keypair.toPeraAddress());
		let a = hello_wasm();
		console.log({ a });
	});
});
