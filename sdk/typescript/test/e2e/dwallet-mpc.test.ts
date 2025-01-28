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
	delay,
	mockedProtocolPublicParameters,
	MPCKeyScheme,
} from '../../src/dwallet-mpc/globals';
import { fetchProtocolPublicParameters } from '../../src/dwallet-mpc/network-dkg';
import { presign } from '../../src/dwallet-mpc/presign';
import {
	completeFutureSignTransactionCall,
	Hash,
	partiallySignMessageTransactionCall,
	signMessageTransactionCall,
} from '../../src/dwallet-mpc/sign';
import {
	createSignDataMoveArgs,
	createSignDataMoveFunc,
	dWalletCurveMoveType,
	signDataMoveType,
} from '../../src/dwallet-mpc/sign_with_ecdsa_k1';
import { Ed25519Keypair } from '../../src/keypairs/ed25519';
import { fullMPCUserSessionsECDSAK1 } from './utils/dwallet';
import { mockCreateDwallet, mockCreatePresign } from './utils/dwallet_mocks';
import { setup, TestToolbox } from './utils/setup';

const fiveMinutes = 5 * 60 * 1000;
describe('Test dWallet MPC', () => {
	let toolbox: TestToolbox;
	let activeEncryptionKeysTableID: string;

	beforeEach(async () => {
		toolbox = await setup();
		console.log('Current Address', toolbox.keypair.toPeraAddress());
		activeEncryptionKeysTableID = (
			await createActiveEncryptionKeysTable({
				keypair: toolbox.keypair,
				client: toolbox.client,
				timeout: fiveMinutes,
			})
		).objectId;
		await delay(2000);
	});

	it('should create a dWallet (DKG)', async () => {
		const pollRef = { value: true };
		let conf: Config = {
			keypair: toolbox.keypair,
			client: toolbox.client,
			timeout: fiveMinutes,
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
			timeout: fiveMinutes * 2,
		};
		const dWallet = await mockCreateDwallet(conf);
		expect(dWallet).toBeDefined();
		console.log({ dWallet });
		const batchSize = 1;
		const presignOutput = await presign(conf, dWallet.id.id, batchSize);
		expect(presignOutput).toBeDefined();
		console.log({ presignOutput });
	});

	it('should run DKG+Presign', async () => {
		let conf: Config = {
			keypair: toolbox.keypair,
			client: toolbox.client,
			timeout: fiveMinutes * 2,
		};
		const dWallet = await createDWallet(
			conf,
			mockedProtocolPublicParameters,
			activeEncryptionKeysTableID,
		);
		expect(dWallet).toBeDefined();
		console.log({ dWallet });
		const presignOutput = await presign(conf, dWallet.id.id, 1);
		expect(presignOutput).toBeDefined();
		console.log({ presignOutput });
	});

	it('should run Sign (with ECDSA K1)', async () => {
		let conf: Config = {
			keypair: toolbox.keypair,
			client: toolbox.client,
			timeout: fiveMinutes * 2,
		};
		const dWallet = await mockCreateDwallet(conf);
		expect(dWallet).toBeDefined();
		console.log({ dWallet });
		const presignOutput1 = await mockCreatePresign(conf, dWallet);
		expect(presignOutput1).toBeDefined();
		const presignOutput2 = await mockCreatePresign(conf, dWallet);
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
		const [centralizedSignMsg] = create_sign_centralized_output(
			mockedProtocolPublicParameters,
			MPCKeyScheme.Secp256k1,
			Uint8Array.from(dWallet.centralized_public_output),
			Uint8Array.from(dWallet.centralizedSecretKeyShare),
			serializedPresigns,
			serializedMsgs,
			Hash.SHA256,
			serializedPresignSessionIds,
		);

		let signDataArgs = createSignDataMoveArgs(
			[presignOutput1.id.id, presignOutput2.id.id],
			centralizedSignMsg,
			dWallet,
		);

		console.log('Signing message');
		let signOutput = await signMessageTransactionCall(
			conf,
			dWallet,
			messages,
			Hash.SHA256,
			signDataArgs,
			createSignDataMoveFunc,
			dWalletCurveMoveType,
			signDataMoveType,
		);
		expect(signOutput).toBeDefined();
		console.log({ signOutput });
	});

	it('Full user-side triggered flow: DKG, Presign, Sign (with ECDSA K1)', async () => {
		let conf: Config = {
			keypair: toolbox.keypair,
			client: toolbox.client,
			timeout: fiveMinutes * 2,
		};
		await fullMPCUserSessionsECDSAK1(
			conf,
			mockedProtocolPublicParameters,
			activeEncryptionKeysTableID,
		);
	});

	it('Full flow: Network DKG, DKG, Presign, Sign (with ECDSA K1)', async () => {
		let conf: Config = {
			keypair: toolbox.keypair,
			client: toolbox.client,
			timeout: fiveMinutes * 2,
		};
		// Todo (#472): Start the network DKG flow from the test.
		let protocolPublicParams = await fetchProtocolPublicParameters(
			conf,
			MPCKeyScheme.Secp256k1,
			null,
		);
		await fullMPCUserSessionsECDSAK1(conf, protocolPublicParams, activeEncryptionKeysTableID);
	});

	it('should run future sign (with ECDSA K1)', async () => {
		let c: Config = {
			keypair: toolbox.keypair,
			client: toolbox.client,
			timeout: fiveMinutes * 2,
		};
		const dWallet = await mockCreateDwallet(c);
		expect(dWallet).toBeDefined();
		console.log({ dWallet });
		const presignOutput1 = await mockCreatePresign(c, dWallet);
		const presignOutput2 = await mockCreatePresign(c, dWallet);
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
		const [centralizedSignMsg] = create_sign_centralized_output(
			// Todo (#382): Change to real value.
			mockedProtocolPublicParameters,
			MPCKeyScheme.Secp256k1,
			Uint8Array.from(dWallet.centralized_public_output),
			Uint8Array.from(dWallet.centralizedSecretKeyShare),
			serializedPresigns,
			serializedMsgs,
			Hash.SHA256,
			serializedPresignSessionIds,
		);

		let signDataArgs = createSignDataMoveArgs(
			[presignOutput1.id.id, presignOutput2.id.id],
			centralizedSignMsg,
			dWallet,
		);

		let partiallySignedMessages = await partiallySignMessageTransactionCall(
			c,
			messages,
			dWallet.id.id,
			signDataArgs,
			createSignDataMoveFunc,
			dWalletCurveMoveType,
			signDataMoveType,
		);
		expect(partiallySignedMessages).toBeDefined();
		console.log({ partiallySignedMessages });
		await delay(5000);
		let completedSignEvent = await completeFutureSignTransactionCall(
			c,
			messages,
			Hash.SHA256,
			dWallet.dwallet_cap_id,
			partiallySignedMessages.partial_signatures_object_id,
			signDataMoveType,
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
			timeout: fiveMinutes,
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
		await delay(3000);
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
