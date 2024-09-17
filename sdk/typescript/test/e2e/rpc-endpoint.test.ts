// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { beforeAll, describe, expect, it } from 'vitest';

import { PeraGasData } from '../../src/client';
import { setup, TestToolbox } from './utils/setup';

describe('Invoke any RPC endpoint', () => {
	let toolbox: TestToolbox;

	beforeAll(async () => {
		toolbox = await setup();
	});

	it('perax_getOwnedObjects', async () => {
		const gasObjectsExpected = await toolbox.client.getOwnedObjects({
			owner: toolbox.address(),
		});
		const gasObjects = await toolbox.client.call<{ data: PeraGasData }>('perax_getOwnedObjects', [
			toolbox.address(),
		]);
		expect(gasObjects.data).toStrictEqual(gasObjectsExpected.data);
	});

	it('pera_getObjectOwnedByAddress Error', async () => {
		expect(toolbox.client.call('perax_getOwnedObjects', [])).rejects.toThrowError();
	});

	it('perax_getCommitteeInfo', async () => {
		const committeeInfoExpected = await toolbox.client.getCommitteeInfo();

		const committeeInfo = await toolbox.client.call('perax_getCommitteeInfo', []);

		expect(committeeInfo).toStrictEqual(committeeInfoExpected);
	});
});
