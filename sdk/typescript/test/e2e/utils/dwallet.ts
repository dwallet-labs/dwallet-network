// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
// noinspection ES6PreferShortImport

import { Buffer } from 'buffer';
import { create_sign_centralized_output } from '@dwallet-network/dwallet-mpc-wasm';
import { expect } from 'vitest';

import { bcs } from '../../../src/bcs/index.js';
import type { CreatedDwallet, DWallet } from '../../../src/dwallet-mpc/dkg.js';
import { createDWallet, dWalletMoveType, isDWallet } from '../../../src/dwallet-mpc/dkg.js';
import {
	dWallet2PCMPCECDSAK1ModuleName,
	MPCKeyScheme,
	packageId,
} from '../../../src/dwallet-mpc/globals.js';
import type { Config } from '../../../src/dwallet-mpc/globals.js';
import { isPresign, presign, presignMoveType } from '../../../src/dwallet-mpc/presign.js';
import type { Presign } from '../../../src/dwallet-mpc/presign.js';
import { Hash, signMessageTransactionCall } from '../../../src/dwallet-mpc/sign.js';
import { Transaction } from '../../../src/transactions/index.js';

const DKGCentralizedPrivateOutput = 'JzXRzjOf/iAd6JWn5r0488W8nKqdWv2VMtmrQBzSBWc=';
const DKGDecentralizedOutput =
	'IQNfkncdbv5x8NUvwEdnBB+1woAtHP8zEziSmhvd/1PlxiECa6POqM2GdrHoJmAoB108grDtdaLPEQiOBydAk598E/4AF+NZYWmueaw8EegU67EZL7DhT2n1tpY969v4bS8+AtaShjQ3mp3ao7R0k7nEdZAVekLouVPmOizLRsVQTLVU1MCn5VXk0yvh8iIwRsRcx5u+wi68wQmd1kpzP59S8LyhlnTUbwYZRH78ZDhOv5uwHz5JrLoAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACZV1otImIlpGXEueECQ3VKkMDeyiI/eYM0I2uMEFQXt5sFZbBPK1KwvNfIB4Ic2owCXEeG3wq/jBXmPDyv1xuCTO+yg5mrHJF+cZBNrVgjmVkofzdNi5E+MvvmMapqgqVpEJvBhwnjlv0GrrMnbwqTCg9ahgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAADniVLoCuy/3gGlskIJBsFKZqXHWuvDEyVRGsPyRiWR8UVXfV4+1FHci2xJnLWhArHRLDKkU8p5CGY0lBBCqkXQyum4bT/ZzXfqWARUIe5Ccyk4qQpqg0UbKcs/xBs6iQrgWxJZ5+JqXeTjEPpOAwcZTk0fuAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACeP3W73Olr5TriQTzomRHPYa3prgRbqzf4GtI7vKUD9qNVn54DffBcoY5Cy+8OQ2dGGguCITc8YPVoGciyK/UakecQYwYUG9qlzt08PubumloVvOmzAs3S0MHHF/zxDI0hdMkQn5DuJew3DWbjG2J4MO2KIAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACECZVoAsVkS9J2SEuQZ2lqGzcb1eTaS8GIq/k+jrGfoz/I=';
const DKGCentralizedPublicOutput =
	'IQJlWgCxWRL0nZIS5BnaWobNxvV5NpLwYir+T6OsZ+jP8iECa6POqM2GdrHoJmAoB108grDtdaLPEQiOBydAk598E/4hA1+Sdx1u/nHw1S/AR2cEH7XCgC0c/zMTOJKaG93/U+XG';
const MockedPresign =
	'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';
const MockedProtocolPublicParameters =
	'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';

export const mockedProtocolPublicParameters = Uint8Array.from(
	Buffer.from(MockedProtocolPublicParameters, 'base64'),
);

export const mockedDWallet = {
	centralizedPrivateDKGOutput: Uint8Array.from(Buffer.from(DKGCentralizedPrivateOutput, 'base64')),
	decentralizedDKGOutput: Uint8Array.from(Buffer.from(DKGDecentralizedOutput, 'base64')),
	centralizedDKGPublicOutput: Uint8Array.from(Buffer.from(DKGCentralizedPublicOutput, 'base64')),
};

export const mockedPresign = {
	presign: Uint8Array.from(Buffer.from(MockedPresign, 'base64')),
	firstRoundSessionID: '0x94cb1bae7e282ad800f78e176f08ae12ddc3784c2810232c7a20ea8ab41244b8',
};

export async function mockCreateDwallet(c: Config): Promise<CreatedDwallet> {
	console.log('Creating dWallet Mock');

	// Initiate the transaction
	const tx = new Transaction();
	const [dwallet] = tx.moveCall({
		target: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::create_mock_dwallet`,
		arguments: [tx.pure(bcs.vector(bcs.u8()).serialize(mockedDWallet.decentralizedDKGOutput))],
	});
	tx.transferObjects([dwallet], c.keypair.toPeraAddress());

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
				decentralizedDKGOutput: dwalletData.output,
				dwalletCapID: dwalletData.dwallet_cap_id,
				dwalletMPCNetworkKeyVersion: dwalletData.dwallet_mpc_network_key_version,
			};
		}
	}
	throw new Error(`mockCreateDwallet error: failed to create object of type ${dWalletMoveType}`);
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

export async function fullMPCUserSessions(conf: Config, protocolPublicParameters: Uint8Array) {
	const dWallet = await createDWallet(conf, protocolPublicParameters);
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
		// Todo (#382): Change to real value.
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
