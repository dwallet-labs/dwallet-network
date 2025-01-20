// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { create_sign_centralized_output } from '@dwallet-network/dwallet-mpc-wasm';
import { bcs } from '@mysten/bcs';
import { beforeEach, describe, expect, it } from 'vitest';

import { MoveStruct, PeraClient } from '../../src/client';
import { createDWallet } from '../../src/dwallet-mpc/dkg';
import { createActiveEncryptionKeysTable } from '../../src/dwallet-mpc/encrypt-user-share';
import {
	Config,
	mockedProtocolPublicParameters,
	MPCKeyScheme,
} from '../../src/dwallet-mpc/globals';
import { fetchProtocolPublicParameters } from '../../src/dwallet-mpc/network-dkg';
import { presign } from '../../src/dwallet-mpc/presign';
import {
	futureSignTransactionCall,
	Hash,
	partiallySignMessageTransactionCall,
	signMessageTransactionCall,
} from '../../src/dwallet-mpc/sign';
import { Ed25519Keypair } from '../../src/keypairs/ed25519';
import { fullMPCUserSessions } from './utils/dwallet';
import { mockCreateDwallet, mockCreatePresign } from './utils/dwallet_mocks';
import { setup, TestToolbox } from './utils/setup';

describe('Test dWallet MPC', () => {
	let toolbox: TestToolbox;
	let activeEncryptionKeysTableID: string;

	const timeout = 5 * 60 * 1000;
	beforeEach(async () => {
		toolbox = await setup();
		console.log('Current Address', toolbox.keypair.toPeraAddress());
		activeEncryptionKeysTableID = (
			await createActiveEncryptionKeysTable({
				keypair: toolbox.keypair,
				client: toolbox.client,
				timeout: timeout,
			})
		).objectId;
	});

	it('should create a dWallet (DKG)', async () => {
		const pollRef = { value: true };
		let conf: Config = {
			keypair: toolbox.keypair,
			client: toolbox.client,
			timeout: timeout,
		};
		const dWallet = await createDWallet(
			conf,
			mockedProtocolPublicParameters,
			activeEncryptionKeysTableID,
		);

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
		const presignOutput = await presign(conf, dWallet.id, 1);
		expect(presignOutput).toBeDefined();
		console.log({ presignOutput });
	});

	it('should run DKG+Presign', async () => {
		let conf: Config = {
			keypair: toolbox.keypair,
			client: toolbox.client,
			timeout: 10 * 60 * 1000,
		};
		const dWallet = await createDWallet(
			conf,
			mockedProtocolPublicParameters,
			activeEncryptionKeysTableID,
		);
		expect(dWallet).toBeDefined();
		console.log({ dWallet });
		const presignOutput = await presign(conf, dWallet.id, 1);
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
		const presignOutput1 = await mockCreatePresign(conf, dWallet);
		const presignOutput2 = await mockCreatePresign(conf, dWallet);
		expect(presignOutput1).toBeDefined();
		expect(presignOutput2).toBeDefined();
		console.log({ presignOutput1, presignOutput2 });
		let messages = [Uint8Array.from([1, 2, 3, 4, 5]), Uint8Array.from([6, 7, 8, 9, 10])];
		let serializedMsgs = bcs.vector(bcs.vector(bcs.u8())).serialize(messages).toBytes();
		let serializedPresigns = bcs
			.vector(bcs.vector(bcs.u8()))
			.serialize([presignOutput1.presign, presignOutput2.presign])
			.toBytes();
		let serializedPresignSessionIds = bcs
			.vector(bcs.string())
			.serialize([
				presignOutput1.first_round_session_id.slice(2),
				presignOutput2.first_round_session_id.slice(2),
			])
			.toBytes();
		const [centralizedSignMsg, hashedMsg] = create_sign_centralized_output(
			mockedProtocolPublicParameters,
			MPCKeyScheme.Secp256k1,
			Uint8Array.from(dWallet.centralizedDKGPublicOutput),
			Uint8Array.from(dWallet.centralizedDKGPrivateOutput),
			serializedPresigns,
			serializedMsgs,
			Hash.SHA256,
			serializedPresignSessionIds,
		);
		console.log('Signing message');
		let signOutput = await signMessageTransactionCall(
			conf,
			dWallet.dwalletCapID,
			hashedMsg,
			messages,
			Hash.SHA256,
			dWallet.id,
			[presignOutput1.id.id, presignOutput2.id.id],
			centralizedSignMsg,
		);
		expect(signOutput).toBeDefined();
		console.log({ signOutput });
	});

	it('Full user-side triggered flow: DKG, Presign, Sign', async () => {
		let conf: Config = {
			keypair: toolbox.keypair,
			client: toolbox.client,
			timeout: 10 * 60 * 1000,
		};
		await fullMPCUserSessions(conf, mockedProtocolPublicParameters, activeEncryptionKeysTableID);
	});

	it('Full flow: Network DKG, DKG, Presign, Sign', async () => {
		let conf: Config = {
			keypair: toolbox.keypair,
			client: toolbox.client,
			timeout: 30 * 60 * 1000,
		};
		// Todo (#472): Start the network DKG flow from the test.
		let protocolPublicParams = await fetchProtocolPublicParameters(
			conf,
			MPCKeyScheme.Secp256k1,
			null,
		);
		conf.timeout = 10 * 60 * 1000;
		await fullMPCUserSessions(conf, protocolPublicParams, activeEncryptionKeysTableID);
	});

	it('should run future sign', async () => {
		let conf: Config = {
			keypair: toolbox.keypair,
			client: toolbox.client,
			timeout: 10 * 60 * 1000,
		};
		const dWallet = await mockCreateDwallet(conf);
		expect(dWallet).toBeDefined();
		console.log({ dWallet });
		const presignOutput1 = await mockCreatePresign(conf, dWallet);
		const presignOutput2 = await mockCreatePresign(conf, dWallet);
		expect(presignOutput1).toBeDefined();
		expect(presignOutput2).toBeDefined();
		console.log({ presignOutput1, presignOutput2 });
		let messages = [Uint8Array.from([1, 2, 3, 4, 5]), Uint8Array.from([6, 7, 8, 9, 10])];
		let serializedMsgs = bcs.vector(bcs.vector(bcs.u8())).serialize(messages).toBytes();
		let serializedPresigns = bcs
			.vector(bcs.vector(bcs.u8()))
			.serialize([presignOutput1.presign, presignOutput2.presign])
			.toBytes();
		let serializedPresignSessionIds = bcs
			.vector(bcs.string())
			.serialize([
				presignOutput1.first_round_session_id.slice(2),
				presignOutput2.first_round_session_id.slice(2),
			])
			.toBytes();
		const [centralizedSignMsg, hashedMsgs] = create_sign_centralized_output(
			// Todo (#382): Change to real value.
			mockedProtocolPublicParameters,
			MPCKeyScheme.Secp256k1,
			Uint8Array.from(dWallet.centralizedDKGPublicOutput),
			Uint8Array.from(dWallet.centralizedDKGPrivateOutput),
			serializedPresigns,
			serializedMsgs,
			Hash.SHA256,
			serializedPresignSessionIds,
		);
		let partiallySignedMessages = await partiallySignMessageTransactionCall(
			conf,
			hashedMsgs,
			dWallet.id,
			[presignOutput1.id.id, presignOutput2.id.id],
			centralizedSignMsg,
		);
		expect(partiallySignedMessages).toBeDefined();
		console.log({ partiallySignedMessages });
		// Sleep for 5 seconds for a checkpoint to be created, so the new object can be used.
		await new Promise((r) => setTimeout(r, 5000));
		let completedSignEvent = await futureSignTransactionCall(
			conf,
			messages,
			Hash.SHA256,
			dWallet.dwalletCapID,
			partiallySignedMessages.partial_signatures_object_id,
		);
		expect(completedSignEvent).toBeDefined();
		console.log({ completedSignEvent });
	});

	it('should run network dkg', async () => {
		const pollRef = { value: true };
		void printOwnedObjects(toolbox.keypair, toolbox.client, pollRef);
		let conf: Config = {
			keypair: toolbox.keypair,
			client: toolbox.client,
			timeout: timeout,
		};

		const keyVersionNum = 0;
		console.log(fetchProtocolPublicParameters(conf, MPCKeyScheme.Secp256k1, keyVersionNum));

		pollRef.value = false;
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
