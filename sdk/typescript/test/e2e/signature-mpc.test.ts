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

	it('should create proof MPC Event', async () => {
		await launchProofMPCEvent(toolbox.keypair, toolbox.client);
		//sleep for 5 seconds
		await new Promise((r) => setTimeout(r, 15000));
		const objects = await toolbox.client.getOwnedObjects({
			owner: toolbox.keypair.toPeraAddress(),
			cursor: null,
		});

		const obj = await toolbox.client.getObject({
			id: objects.data[0].data?.objectId!,
			options: { showContent: true },
		});
		console.log(obj);
	});
});
