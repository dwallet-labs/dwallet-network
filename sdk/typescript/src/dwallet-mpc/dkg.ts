// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
// noinspection ES6PreferShortImport

import { create_dkg_centralized_output } from '@dwallet-network/dwallet-mpc-wasm';

import { bcs } from '../bcs/index.js';
import type { MoveValue, PeraClient, PeraParsedData } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';
import { Transaction } from '../transactions/index.js';

export const dWalletPackageID = '0x3';
export const dWallet2PCMPCECDSAK1ModuleName = 'dwallet_2pc_mpc_ecdsa_k1';
const dWalletModuleName = 'dwallet';
const dkgFirstRoundOutputMoveType = `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::DKGFirstRoundOutput`;
const dwalletSecp256K1MoveType = `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::Secp256K1`;
const dWalletMoveType = `${dWalletPackageID}::${dWalletModuleName}::DWallet<${dwalletSecp256K1MoveType}>`;
const moveObjectDataType = 'moveObject';

interface DKGFirstRoundOutputFields {
	id: { id: string };
	session_id: string;
	dwallet_cap_id: string;
	output: number[];

	[key: string]: MoveValue;
}

export type DKGFirstRoundOutputMoveObj = {
	dataType: typeof moveObjectDataType;
	type: typeof dkgFirstRoundOutputMoveType;
	fields: DKGFirstRoundOutputFields;
	hasPublicTransfer: boolean;
};

interface CompletedDKGSecondRoundEvent {
	session_id: string;
	sender: string;
	dwallet_cap_id: string;
	dwallet_id: string;
	value: Uint8Array;
}

interface DWallet {
	id: { id: string };
	session_id: string;
	dwallet_cap_id: string;
	output: number[];

	[key: string]: MoveValue;
}

function isDKGFirstRoundOutputMoveObj(o?: PeraParsedData | null): o is DKGFirstRoundOutputMoveObj {
	return (
		o?.dataType === moveObjectDataType &&
		o?.type === dkgFirstRoundOutputMoveType &&
		'id' in o.fields &&
		'session_id' in o.fields &&
		'output' in o.fields &&
		'dwallet_cap_id' in o.fields
	);
}

interface Config {
	keypair: Keypair;
	client: PeraClient;
}

/**
 * Starts the first round of the DKG protocol to create a new dWallet.
 * The output of this function is being used to generate the input for the second round.
 */
async function launchDKGFirstRound(c: Config) {
	const tx = new Transaction();
	tx.moveCall({
		target: `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::launch_dkg_first_round`,
		arguments: [],
	});
	const result = await c.client.signAndExecuteTransaction({
		signer: c.keypair,
		transaction: tx,
		options: {
			showEffects: true,
		},
	});
	const createdDwalletCapRef = result.effects?.created?.[0].reference;
	if (!createdDwalletCapRef) {
		throw new Error('CreateDwallet error: Failed to create dWallet capability');
	}
	let obj = await dkgFirstRoundOutputObject(createdDwalletCapRef.objectId, c);
	return obj.fields;
}

async function dWalletFromEvent(
	c: Config,
	firstRound: DKGFirstRoundOutputFields,
	timeout: number = 10 * 60 * 1000,
): Promise<DWallet> {
	let cursor = null;
	const startTime = Date.now();

	while (Date.now() - startTime < timeout) {
		// Wait for 5 seconds between queries
		await new Promise((resolve) => setTimeout(resolve, 5000));

		// Query events with the current cursor.
		const {
			data: events,
			nextCursor,
			hasNextPage,
		} = await c.client.queryEvents({
			cursor,
			query: {
				MoveEventType: `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::CompletedDKGSecondRoundEvent`,
			},
		});

		for (const eventData of events) {
			const event = eventData.parsedJson as CompletedDKGSecondRoundEvent;
			if (event.dwallet_cap_id !== firstRound.dwallet_cap_id) {
				continue;
			}

			// Fetch the dWallet object using the event's dwallet_id.
			const dwalletResponse = await c.client.getObject({
				id: event.dwallet_id,
				options: { showContent: true },
			});

			// Check if the object is of the expected data type and cast it to DWallet.
			if (
				dwalletResponse.data?.content?.dataType === moveObjectDataType &&
				// todo: validate this.
				dwalletResponse.data?.content?.type === dWalletMoveType
			) {
				return dwalletResponse.data?.content?.fields as DWallet;
			}
		}

		// Update cursor for pagination
		cursor = hasNextPage ? nextCursor : null;
	}
	throw new Error(`timeout reached: Unable to create dWallet within ${timeout / 1000} seconds.`);
}

async function launchDKGSecondRound(
	c: Config,
	firstRound: DKGFirstRoundOutputFields,
	publicKeyShareAndProof: Uint8Array,
) {
	const tx = new Transaction();
	tx.moveCall({
		target: `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::launch_dkg_second_round`,
		arguments: [
			tx.object(firstRound.dwallet_cap_id),
			tx.pure(bcs.vector(bcs.u8()).serialize(publicKeyShareAndProof)),
			tx.pure(bcs.vector(bcs.u8()).serialize(firstRound.output)),
			tx.pure.id(firstRound.session_id),
		],
	});

	await c.client.signAndExecuteTransaction({
		signer: c.keypair,
		transaction: tx,
		options: {
			showEffects: true,
		},
	});
	return await dWalletFromEvent(c, firstRound);
}

/**
 * Fetches the DKGFirstRoundOutput object from the client.
 * Since the object might not be available immediately,
 * this function polls the client until the object is found, or a timeout is reached.
 */
async function dkgFirstRoundOutputObject(
	dwalletCapID: string,
	c: Config,
	timeout: number = 10 * 60 * 1000,
): Promise<DKGFirstRoundOutputMoveObj> {
	let cursor = null;
	const startTime = Date.now();

	while (Date.now() - startTime < timeout) {
		// Wait for a bit before polling again, objects might not be available immediately.
		await new Promise((r) => setTimeout(r, 1000));
		const {
			data: ownedObjects,
			hasNextPage,
			nextCursor,
		} = await c.client.getOwnedObjects({
			owner: c.keypair.toPeraAddress(),
			cursor,
		});
		const objectIds = ownedObjects.map((o) => o.data?.objectId).filter(Boolean) as string[];

		if (objectIds.length === 0) {
			await new Promise((r) => setTimeout(r, 500));
			continue;
		}

		const objectsContent = await c.client.multiGetObjects({
			ids: objectIds,
			options: { showContent: true },
		});

		// Find the first matching object with `moveObject` dataType and correct `dwalletCapID`
		const match = objectsContent
			.map((o) => o.data?.content)
			.find(
				(o): o is DKGFirstRoundOutputMoveObj =>
					isDKGFirstRoundOutputMoveObj(o) && o.fields.dwallet_cap_id === dwalletCapID,
			);
		if (match) return match;
		if (hasNextPage) {
			cursor = nextCursor;
		}
	}

	throw new Error(
		`timeout reached: Unable to find DKGFirstRoundOutputObject within ${timeout / 1000} seconds.`,
	);
}

export type CreatedDwallet = {
	dwalletID: string;
	centralizedDKGOutput: number[];
	decentralizedDKGOutput: number[];
	dwalletCapID: string;
};

export async function createDWallet(keypair: Keypair, client: PeraClient): Promise<CreatedDwallet> {
	let config: Config = {
		keypair: keypair,
		client: client,
	};

	const firstDKGOutput = await launchDKGFirstRound(config);
	let [publicKeyShareAndProof, centralizedOutput] = create_dkg_centralized_output(
		Uint8Array.from(firstDKGOutput.output),
		// Remove the 0x prefix.
		firstDKGOutput.session_id.slice(2),
	);
	let dwallet = await launchDKGSecondRound(config, firstDKGOutput, publicKeyShareAndProof);

	return {
		dwalletID: dwallet.id.id,
		centralizedDKGOutput: centralizedOutput,
		decentralizedDKGOutput: dwallet.output,
		dwalletCapID: dwallet.dwallet_cap_id,
	};
}
