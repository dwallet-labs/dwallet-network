// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { beforeAll, describe, expect, it } from 'vitest';

import { MoveStruct, PeraClient } from '../../src/client';
import { createDWallet } from '../../src/dwallet-mpc/dkg';
import { Ed25519Keypair } from '../../src/keypairs/ed25519';
import { setup, TestToolbox } from './utils/setup';

describe('Test dwallet mpc', () => {
	let toolbox: TestToolbox;

	beforeAll(async () => {
		toolbox = await setup();
	});

	it('should create DWallet', async () => {
		console.log('Address', toolbox.keypair.toPeraAddress());

		const pollRef = { value: true };
		void printOwnedObjects(toolbox.keypair, toolbox.client, pollRef);

		const result = await createDWallet(toolbox.keypair, toolbox.client);
		expect(result).toBeDefined();
		pollRef.value = false;
	});
});

type MoveObjectContent = {
	dataType: 'moveObject';
	fields: MoveStruct;
	hasPublicTransfer: boolean;
	type: string;
};

async function printOwnedObjects(
	keypair: Ed25519Keypair,
	client: PeraClient,
	poll: { value: boolean },
) {
	let cursor = null;

	while (poll.value) {
		await new Promise((r) => setTimeout(r, 3000));
		const {
			data: ownedObjects,
			hasNextPage,
			nextCursor,
		} = await client.getOwnedObjects({
			owner: keypair.toPeraAddress(),
			cursor,
		});
		const objectIds = ownedObjects.map((o) => o.data?.objectId).filter(Boolean) as string[];

		if (objectIds.length === 0) {
			continue;
		}

		const objectsContent = await client.multiGetObjects({
			ids: objectIds,
			options: { showContent: true },
		});

		objectsContent.forEach((o) => {
			if ((o.data?.content as MoveObjectContent)?.type !== '0x2::coin::Coin<0x2::pera::PERA>') {
				console.log(o);
			}
		});
		if (hasNextPage) {
			cursor = nextCursor;
		}
	}

	console.log('Stopped polling');
}
