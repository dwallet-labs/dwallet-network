// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { beforeAll, describe, it } from 'vitest';

import { launchProofMPCEvent } from '../../src/signature-mpc/proof';
import { setup, TestToolbox } from './utils/setup';

describe('Test signature mpc', () => {
	let toolbox: TestToolbox;

	beforeAll(async () => {
		toolbox = await setup();
	});

	it('A simple test to launch the Proof MPC flow', async () => {
		await launchProofMPCEvent(toolbox.keypair, toolbox.client);
	});
});
