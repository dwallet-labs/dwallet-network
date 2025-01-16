// noinspection ES6PreferShortImport

import { Buffer } from 'buffer';

import { bcs } from '../../../src/bcs/index.js';
import type { Config, CreatedDwallet, DWallet } from '../../../src/dwallet-mpc/globals.js';
import {
	dWallet2PCMPCECDSAK1ModuleName,
	dWalletMoveType,
	isDWallet,
	packageId,
} from '../../../src/dwallet-mpc/globals.js';
import type { Presign } from '../../../src/dwallet-mpc/presign.js';
import { isPresign, presignMoveType } from '../../../src/dwallet-mpc/presign.js';
import { Transaction } from '../../../src/transactions/index.js';

export const DKGCentralizedPrivateOutput = 'JzXRzjOf/iAd6JWn5r0488W8nKqdWv2VMtmrQBzSBWc=';
export const DKGDecentralizedOutput =
	'IQNfkncdbv5x8NUvwEdnBB+1woAtHP8zEziSmhvd/1PlxiECa6POqM2GdrHoJmAoB108grDtdaLPEQiOBydAk598E/4AF+NZYWmueaw8EegU67EZL7DhT2n1tpY969v4bS8+AtaShjQ3mp3ao7R0k7nEdZAVekLouVPmOizLRsVQTLVU1MCn5VXk0yvh8iIwRsRcx5u+wi68wQmd1kpzP59S8LyhlnTUbwYZRH78ZDhOv5uwHz5JrLoAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACZV1otImIlpGXEueECQ3VKkMDeyiI/eYM0I2uMEFQXt5sFZbBPK1KwvNfIB4Ic2owCXEeG3wq/jBXmPDyv1xuCTO+yg5mrHJF+cZBNrVgjmVkofzdNi5E+MvvmMapqgqVpEJvBhwnjlv0GrrMnbwqTCg9ahgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAADniVLoCuy/3gGlskIJBsFKZqXHWuvDEyVRGsPyRiWR8UVXfV4+1FHci2xJnLWhArHRLDKkU8p5CGY0lBBCqkXQyum4bT/ZzXfqWARUIe5Ccyk4qQpqg0UbKcs/xBs6iQrgWxJZ5+JqXeTjEPpOAwcZTk0fuAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACeP3W73Olr5TriQTzomRHPYa3prgRbqzf4GtI7vKUD9qNVn54DffBcoY5Cy+8OQ2dGGguCITc8YPVoGciyK/UakecQYwYUG9qlzt08PubumloVvOmzAs3S0MHHF/zxDI0hdMkQn5DuJew3DWbjG2J4MO2KIAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACECZVoAsVkS9J2SEuQZ2lqGzcb1eTaS8GIq/k+jrGfoz/I=';
export const DKGCentralizedPublicOutput =
	'IQJlWgCxWRL0nZIS5BnaWobNxvV5NpLwYir+T6OsZ+jP8iECa6POqM2GdrHoJmAoB108grDtdaLPEQiOBydAk598E/4hA1+Sdx1u/nHw1S/AR2cEH7XCgC0c/zMTOJKaG93/U+XG';
const MockedPresign =
	'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';
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
