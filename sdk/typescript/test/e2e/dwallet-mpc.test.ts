// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { create_sign_centralized_output } from '@dwallet-network/dwallet-mpc-wasm';
import { bcs } from '@mysten/bcs';
import { beforeEach, describe, expect, it } from 'vitest';

import { MoveStruct, PeraClient } from '../../src/client';
import { createDWallet } from '../../src/dwallet-mpc/dkg';
import { Config } from '../../src/dwallet-mpc/globals';
import { launchNetworkDKG } from '../../src/dwallet-mpc/network-dkg';
import { presign } from '../../src/dwallet-mpc/presign';
import { Hash, signMessageTransactionCall } from '../../src/dwallet-mpc/sign';
import { Ed25519Keypair } from '../../src/keypairs/ed25519';
import { mockCreateDwallet, mockCreatePresign } from './utils/dwallet';
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

	it('should run Presign', async () => {
		let conf: Config = {
			keypair: toolbox.keypair,
			client: toolbox.client,
			timeout: 10 * 60 * 1000,
		};
		const dWallet = await mockCreateDwallet(conf);
		expect(dWallet).toBeDefined();
		console.log({ dWallet });
		const presignOutput = await presign(conf, dWallet.id);
		expect(presignOutput).toBeDefined();
		console.log({ presignOutput });
	});

	it('should run DKG+Presign', async () => {
		let conf: Config = {
			keypair: toolbox.keypair,
			client: toolbox.client,
			timeout: 10 * 60 * 1000,
		};
		const dWallet = await createDWallet(conf);
		expect(dWallet).toBeDefined();
		console.log({ dWallet });
		const presignOutput = await presign(conf, dWallet.id);
		expect(presignOutput).toBeDefined();
		console.log({ presignOutput });
	});

	it('should run Sign', async () => {
		let conf: Config = {
			keypair: toolbox.keypair,
			client: toolbox.client,
			timeout: 10 * 60 * 1000,
		};
		const dWallet = await mockCreateDwallet(conf);
		expect(dWallet).toBeDefined();
		console.log({ dWallet });
		const presignOutput = await mockCreatePresign(conf, dWallet);
		expect(presignOutput).toBeDefined();
		console.log({ presignOutput });
		let serializedMsgs = bcs
			.vector(bcs.vector(bcs.u8()))
			.serialize([Uint8Array.from([1, 2, 3, 4, 5]), Uint8Array.from([6, 7, 8, 9, 10])])
			.toBytes();
		const [centralizedSignMsg, hashedMsg] = create_sign_centralized_output(
			Uint8Array.from(dWallet.centralizedDKGOutput),
			Uint8Array.from(presignOutput.first_round_output),
			Uint8Array.from(presignOutput.second_round_output),
			serializedMsgs,
			Hash.SHA256,
			presignOutput.first_round_session_id.slice(2),
		);
		console.log('Signing message');
		let signOutput = await signMessageTransactionCall(
			conf,
			dWallet.dwalletCapID,
			hashedMsg,
			dWallet.id,
			presignOutput.id.id,
			centralizedSignMsg,
			presignOutput.first_round_session_id,
		);
		expect(signOutput).toBeDefined();
		console.log({ signOutput });
	});

	it(
		'Full flow: DKG, Presign, Sign',
		async () => {
			let conf: Config = {
				keypair: toolbox.keypair,
				client: toolbox.client,
				timeout: 10 * 60 * 1000,
			};
			const dWallet = await createDWallet(conf);
			expect(dWallet).toBeDefined();
			const presignOutput = await presign(conf, dWallet.id);
			expect(presignOutput).toBeDefined();

			let serializedMsgs = bcs
				.vector(bcs.vector(bcs.u8()))
				.serialize([Uint8Array.from([1, 2, 3, 4, 5]), Uint8Array.from([6, 7, 8, 9, 10])])
				.toBytes();
			const [centralizedSignMsg, hashedMsg] = create_sign_centralized_output(
				Uint8Array.from(dWallet.centralizedDKGOutput),
				Uint8Array.from(presignOutput.firstRoundOutput),
				Uint8Array.from(presignOutput.secondRoundOutput),
				serializedMsgs,
				Hash.SHA256,
				presignOutput.firstRoundSessionID.slice(2),
			);

			console.log('Signing messages');
			let signOutput = await signMessageTransactionCall(
				conf,
				dWallet.dwalletCapID,
				hashedMsg,
				dWallet.id,
				presignOutput.secondRoundOutputID,
				centralizedSignMsg,
				presignOutput.firstRoundSessionID,
			);
			expect(signOutput).toBeDefined();
			console.log({ signOutput });
		},
		1000 * 60 * 20,
	);

	it('should run network dkg', async () => {
		const pollRef = { value: true };
		void printOwnedObjects(toolbox.keypair, toolbox.client, pollRef);
		let conf: Config = {
			keypair: toolbox.keypair,
			client: toolbox.client,
			timeout: 5 * 60 * 1000,
		};
		const dWallet = await launchNetworkDKG(conf);
		expect(dWallet).toBeDefined();
		pollRef.value = false;
		console.log({ dWallet });
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
