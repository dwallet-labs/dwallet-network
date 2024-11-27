// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { create_sign_centralized_output } from '@dwallet-network/dwallet-mpc-wasm';
import { beforeEach, describe, expect, it } from 'vitest';

import { MoveStruct, PeraClient } from '../../src/client';
import { createDWallet } from '../../src/dwallet-mpc/dkg';
import { Config } from '../../src/dwallet-mpc/globals';
import { presign } from '../../src/dwallet-mpc/presign';
import { Hash, signMessageTransactionCall } from '../../src/dwallet-mpc/sign';
import { Ed25519Keypair } from '../../src/keypairs/ed25519';
import { mockedDWallet, mockedPresign } from './utils/dwallet';
import { setup, TestToolbox } from './utils/setup';

describe('Test dWallet MPC', () => {
	let toolbox: TestToolbox;

	beforeEach(async () => {
		toolbox = await setup();
		console.log('Address', toolbox.keypair.toPeraAddress());
	});

	it('should create a dWallet (DKG)', async () => {
		const pollRef = { value: true };
		void printOwnedObjects(toolbox.keypair, toolbox.client, pollRef);
		let conf: Config = {
			keypair: toolbox.keypair,
			client: toolbox.client,
			timeout: 5 * 60 * 1000,
		};
		const dWallet = await createDWallet(conf);
		expect(dWallet).toBeDefined();
		pollRef.value = false;
		console.log({ dWallet });
	});

	it('should run a presign', async () => {
		let conf: Config = {
			keypair: toolbox.keypair,
			client: toolbox.client,
			timeout: 5 * 60 * 1000,
		};
		const dWallet = await createDWallet(conf);
		expect(dWallet).toBeDefined();
		console.log({ dWallet });
		const presignOutput = await presign(conf, dWallet.dwalletID);
		expect(presignOutput).toBeDefined();
		console.log({ presignOutput });
	});

	it('should sign a message successfully ', async () => {
		const message = Uint8Array.from([1, 2, 3, 4, 5]);
		const [sign_msg, _, fullPresigns, msg_hash] = create_sign_centralized_output(
			Uint8Array.from(mockedDWallet.centralizedDKGOutput),
			Uint8Array.from(mockedPresign.firstRoundOutput),
			Uint8Array.from(mockedPresign.secondRoundOutput),
			message,
			Hash.SHA256,
			// slice(2) Removes the 0x prefix.
			mockedPresign.firstRoundSessionID.slice(2)!,
		);
		let conf: Config = {
			keypair: toolbox.keypair,
			client: toolbox.client,
			timeout: 5 * 60 * 1000,
		};
		let signOutput = await signMessageTransactionCall(
			conf,
			msg_hash,
			fullPresigns,
			mockedDWallet.decentralizedDKGOutput,
			sign_msg,
			mockedPresign.firstRoundSessionID,
		);
		expect(signOutput).toBeDefined();
		console.log({ signOutput });
	});
});

async function printOwnedObjects(
	keypair: Ed25519Keypair,
	client: PeraClient,
	poll: { value: boolean },
) {
	type MoveObjectContent = {
		dataType: 'moveObject';
		fields: MoveStruct;
		hasPublicTransfer: boolean;
		type: string;
	};

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
