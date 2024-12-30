// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
// noinspection ES6PreferShortImport

import { Buffer } from 'buffer';

import { bcs } from '../../../src/bcs/index.js';
import type { CreatedDwallet, DWallet } from '../../../src/dwallet-mpc/dkg.js';
import { dWalletMoveType, isDWallet } from '../../../src/dwallet-mpc/dkg.js';
import { dWallet2PCMPCECDSAK1ModuleName, packageId } from '../../../src/dwallet-mpc/globals.js';
import type { Config } from '../../../src/dwallet-mpc/globals.js';
import { isPresign, presignMoveType } from '../../../src/dwallet-mpc/presign.js';
import type { Presign } from '../../../src/dwallet-mpc/presign.js';
import { Transaction } from '../../../src/transactions/index.js';

const DKGCentralizedOutput =
	'IEOqWWYa3+D5sKS6TGrvLgi2M1DnRYHMkH0KYNoMgxkhAnOsvrxxbiLcoaiOXXlB5tpGagpOICQD+C2oO21jqVrTIQKobfzUEJP6XFRtlIIRGQs9Y2tEcq9tSdMYpVXLudJ1aiEDlZdJuZ7RTlfjDprLAZMDJ434cKJnMHJvqhr2vRNjnRo=';
const DKGDecentralizedOutput =
	'IQOVl0m5ntFOV+MOmssBkwMnjfhwomcwcm+qGva9E2OdGiECqG381BCT+lxUbZSCERkLPWNrRHKvbUnTGKVVy7nSdWoAm3ftNeuiMHQgd3FHNNf5cI4/e3//Mo6aqAyqWSvLemNWtE28fHi+qF6F/F7p556WHijVAzDJEbyZ3hURnO6azzkwZvJhfLd4rNB7KJSIHePyPN5ZvVvqSjxNdTvz0GnrZx+22O0JibJy4j3pvaQdgoX9iYwBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABXgJlPshIY25EmI/nlvOIcu2q/XYE1KLRUP2Jh+3xgmfbd6r3e4pI42ZPFbgT12eCncY5nNqTuaVCCuQjoDKbUqG/UCL7I7JZWtx1xO0htItssqBGffGi35NnMAzUu+LFQeWs0lPpeSP2i3l5OLu6l+Z5rTAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAkUuJ7/mTLFMjlwoQSutOm5Jr8NbiUyQWIJM6Q8wcsc0g8U3X/u9quguPyFdmt2mLhl0icEAzvf8jbYnHovkqjcVknZmn1G2+6YZYqPxKgPZ7mRYlfCYSa9UyzjZWprDpqzdzu/pcY42oJDhbeRALm32+/xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABb8GBTH1CDHxtZTkS18NKyiaBQurdQrcxBiMitu5qdrsCgs6J0nFOLgS6GqwqTNztkIYvd28o6Y+dUPOnXvHRdc6Qaxb7OUC+bJD5RbUqB57UqPs8LmgSGlnXg9NqRp7qF8MkSx3EDHO/tbb2Xh8lqG20CcAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACECc6y+vHFuItyhqI5deUHm2kZqCk4gJAP4Lag7bWOpWtM=';
const MockedPresign =
	'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';

export const mockedDWallet = {
	centralizedDKGOutput: Uint8Array.from(Buffer.from(DKGCentralizedOutput, 'base64')),
	decentralizedDKGOutput: Uint8Array.from(Buffer.from(DKGDecentralizedOutput, 'base64')),
};

export const mockedPresign = {
	presign: Uint8Array.from(Buffer.from(MockedPresign, 'base64')),
	firstRoundSessionID: '0x5d696d70bc5428c62f0f943adc8d04e9dfc83751af8aa5a314f96b04fa9d2c9d',
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
				centralizedDKGPublicOutput: Array.from(Buffer.from(DKGCentralizedOutput, 'base64')),
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
