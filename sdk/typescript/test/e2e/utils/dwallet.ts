// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
// noinspection ES6PreferShortImport

import { Buffer } from 'buffer';
import { create_sign_centralized_output } from '@dwallet-network/dwallet-mpc-wasm';
import { expect } from 'vitest';

import { bcs } from '../../../src/bcs/index.js';
import { createDWallet } from '../../../src/dwallet-mpc/dkg.js';
import type { Config, CreatedDwallet, DWallet } from '../../../src/dwallet-mpc/globals.js';
import {
	dWallet2PCMPCECDSAK1ModuleName,
	dWalletMoveType,
	isDWallet,
	MPCKeyScheme,
	packageId,
} from '../../../src/dwallet-mpc/globals.js';
import type { Presign } from '../../../src/dwallet-mpc/presign.js';
import { isPresign, presign, presignMoveType } from '../../../src/dwallet-mpc/presign.js';
import { Hash, signMessageTransactionCall } from '../../../src/dwallet-mpc/sign.js';
import { Transaction } from '../../../src/transactions/index.js';
import {
	DKGCentralizedPrivateOutput,
	DKGCentralizedPublicOutput,
	mockedDWallet,
	mockedPresign,
} from './dwallet_mocks.js';

export async function mockCreateDwallet(c: Config): Promise<CreatedDwallet> {
	console.log('Creating dWallet Mock');

	// Initiate the transaction
	const tx = new Transaction();
	tx.moveCall({
		target: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::create_mock_dwallet`,
		arguments: [
			tx.pure(bcs.vector(bcs.u8()).serialize(mockedDWallet.decentralizedDKGOutput)),
			tx.pure(bcs.vector(bcs.u8()).serialize(mockedDWallet.centralizedDKGPublicOutput)),
		],
	});

	// Execute the transaction
	const res = await c.client.signAndExecuteTransaction({
		signer: c.keypair,
		transaction: tx,
		options: {
			showEffects: true,
		},
	});

	// Validate the created objects
	const createdObjects = res.effects?.created;
	if (!createdObjects || createdObjects.length !== 2) {
		throw new Error(
			`mockCreateDwallet error: Unexpected number of objects created. Expected 2, got ${
				createdObjects?.length || 0
			}`,
		);
	}
	await new Promise((resolve) => setTimeout(resolve, 2000));
	for (const obj of createdObjects) {
		const objectData = await c.client.getObject({
			id: obj.reference.objectId,
			options: { showContent: true },
		});
		const dwalletData =
			objectData.data?.content?.dataType === 'moveObject' &&
			objectData.data?.content.type === dWalletMoveType &&
			isDWallet(objectData.data.content.fields)
				? (objectData.data.content.fields as DWallet)
				: null;

		if (dwalletData) {
			return {
				id: dwalletData.id.id,
				centralizedDKGPublicOutput: Array.from(Buffer.from(DKGCentralizedPublicOutput, 'base64')),
				centralizedDKGPrivateOutput: Array.from(Buffer.from(DKGCentralizedPrivateOutput, 'base64')),
				decentralizedDKGOutput: dwalletData.decentralized_output,
				dwalletCapID: dwalletData.dwallet_cap_id,
				dwalletMPCNetworkKeyVersion: dwalletData.dwallet_mpc_network_key_version,
			};
		}
	}
	throw new Error(`mockCreateDwallet error: failed to create an object of type ${dWalletMoveType}`);
}

export async function mockCreatePresign(c: Config, dwallet: CreatedDwallet): Promise<Presign> {
	console.log('Creating Presign Mock');
	const tx = new Transaction();
	const [presign] = tx.moveCall({
		target: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::create_mock_presign`,
		arguments: [
			tx.pure.id(dwallet.id),
			tx.pure(bcs.vector(bcs.u8()).serialize(mockedPresign.presign)),
			tx.pure.id(mockedPresign.firstRoundSessionID),
		],
	});
	tx.transferObjects([presign], c.keypair.toPeraAddress());
	let res = await c.client.signAndExecuteTransaction({
		signer: c.keypair,
		transaction: tx,
		options: {
			showEffects: true,
		},
	});
	const presignID = res.effects?.created?.at(0)?.reference.objectId;
	if (!presignID) {
		throw new Error('create_mock_presign error: Failed to create presign');
	}
	await new Promise((resolve) => setTimeout(resolve, 2000));
	const obj = await c.client.getObject({
		id: presignID,
		options: { showContent: true },
	});
	const preSignObj =
		obj.data?.content?.dataType === 'moveObject' &&
		obj.data?.content.type === presignMoveType &&
		isPresign(obj.data.content.fields)
			? (obj.data.content.fields as Presign)
			: null;

	if (!preSignObj) {
		throw new Error(
			`invalid object of type ${dWalletMoveType}, got: ${JSON.stringify(obj.data?.content)}`,
		);
	}

	return preSignObj;
}

/**
 * Run the Full MPC User Sessions
 */
export async function fullMPCUserSessions(
	conf: Config,
	protocolPublicParameters: Uint8Array,
	activeEncryptionKeysTableID: string,
) {
	const dWallet = await createDWallet(conf, protocolPublicParameters, activeEncryptionKeysTableID);
	console.log({ dWallet });
	expect(dWallet).toBeDefined();
	const presignCompletionEvent = await presign(conf, dWallet.id, 2);
	console.log({ presignCompletionEvent });
	expect(presignCompletionEvent).toBeDefined();
	let serializedMsgs = bcs
		.vector(bcs.vector(bcs.u8()))
		.serialize([Uint8Array.from([1, 2, 3, 4, 5]), Uint8Array.from([6, 7, 8, 9, 10])])
		.toBytes();
	let serializedPresigns = bcs
		.vector(bcs.vector(bcs.u8()))
		.serialize(presignCompletionEvent.presigns)
		.toBytes();
	let serializedPresignFirstRoundSessionIds = bcs
		.vector(bcs.string())
		.serialize(
			presignCompletionEvent.first_round_session_ids.map((session_id) => session_id.slice(2)),
		)
		.toBytes();
	const [centralizedSignedMsg, hashedMsgs] = create_sign_centralized_output(
		protocolPublicParameters,
		MPCKeyScheme.Secp256k1,
		Uint8Array.from(dWallet.centralizedDKGPublicOutput),
		Uint8Array.from(dWallet.centralizedDKGPrivateOutput),
		serializedPresigns,
		serializedMsgs,
		Hash.SHA256,
		serializedPresignFirstRoundSessionIds,
	);

	console.log('Signing messages');
	let signOutput = await signMessageTransactionCall(
		conf,
		dWallet.dwalletCapID,
		hashedMsgs,
		dWallet.id,
		presignCompletionEvent.presign_ids,
		centralizedSignedMsg,
	);
	expect(signOutput).toBeDefined();
	console.log({ signOutput });
}
