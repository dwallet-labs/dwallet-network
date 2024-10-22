// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { beforeAll, describe, it } from 'vitest';

import { launchProofMPSession } from '../../src/signature-mpc/proof';
import { setup, TestToolbox } from './utils/setup';

describe('Test MPC Proof', () => {
	let toolbox: TestToolbox;

	beforeAll(async () => {
		toolbox = await setup();
	});

	it('should create proof MPC Event', async () => {
		await launchProofMPSession(toolbox.keypair, toolbox.client);
		console.log(toolbox.keypair.toPeraAddress());
	});
});
