// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
// noinspection ES6PreferShortImport
import { create_dkg_centralized_output } from '@dwallet-network/dwallet-mpc-wasm';

import { bcs } from '../bcs/index.js';
import type { PeraClient } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';
import { Transaction } from '../transactions/index.js';
import {Config, dWalletModuleName, dWalletPackageID} from './globals.js';

const packageId = '0x3';
const dWallet2PCMPCECDSAK1ModuleName = 'dwallet_2pc_mpc_ecdsa_k1';
const dkgFirstRoundOutputMoveType = `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::DKGFirstRoundOutput`;
const dwalletSecp256K1MoveType = `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::Secp256K1`;
export const dWalletMoveType = `${dWalletPackageID}::${dWalletModuleName}::DWallet<${dwalletSecp256K1MoveType}>`;
const completedDKGSecondRoundEventMoveType = `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::CompletedDKGSecondRoundEvent`;

interface DKGFirstRoundOutput {
	id: { id: string };
	session_id: string;
	dwallet_cap_id: string;
	output: number[];
}

interface CompletedDKGSecondRoundEvent {
	session_id: string;
	initiator: string;
	dwallet_cap_id: string;
	dwallet_id: string;
	value: number[];
}

// The Move type.
export interface DWallet {
	id: { id: string };
	session_id: string;
	dwallet_cap_id: string;
	output: number[];
}

export interface CreatedDwallet {
	id: string;
	centralizedDKGOutput: number[];
	decentralizedDKGOutput: number[];
	dwalletCapID: string;
}

export async function createDWallet(conf: Config): Promise<CreatedDwallet> {
	const dkgFirstRoundOutput: DKGFirstRoundOutput = await launchDKGFirstRound(conf);
	let [publicKeyShareAndProof, centralizedOutput] = create_dkg_centralized_output(
		Uint8Array.from(dkgFirstRoundOutput.output),
		// Remove the 0x prefix.
		dkgFirstRoundOutput.session_id.slice(2),
	);
	let dwallet = await launchDKGSecondRound(conf, dkgFirstRoundOutput, publicKeyShareAndProof);

	return {
		id: dwallet.id.id,
		centralizedDKGOutput: centralizedOutput,
		decentralizedDKGOutput: dwallet.output,
		dwalletCapID: dwallet.dwallet_cap_id,
	};
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
	return await dkgFirstRoundOutputObject(createdDwalletCapRef.objectId, c);
}

function isDKGFirstRoundOutput(o: any): o is DKGFirstRoundOutput {
	return 'id' in o && 'session_id' in o && 'output' in o && 'dwallet_cap_id' in o;
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
): Promise<DKGFirstRoundOutput> {
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

		const objects = await c.client.multiGetObjects({
			ids: objectIds,
			options: { showContent: true },
		});

		for (const o of objects) {
			const content = o.data?.content;
			if (
				content?.dataType === 'moveObject' &&
				content.type === dkgFirstRoundOutputMoveType &&
				isDKGFirstRoundOutput(content.fields) &&
				content.fields.dwallet_cap_id === dwalletCapID
			) {
				return content.fields as DKGFirstRoundOutput;
			}
		}
		if (hasNextPage) {
			cursor = nextCursor;
		}
	}
	throw new Error(
		`timeout reached: Unable to find ${dkgFirstRoundOutputMoveType} within ${timeout / 1000} seconds.`,
	);
}

export async function launchDKGSecondRound(
	keypair: Keypair,
	client: PeraClient,
	publicKeyShareAndProof: Uint8Array,
	firstRoundOutput: Uint8Array,
	dwalletCapId: string,
	firstRoundSessionId: string,
) {
	const tx = new Transaction();
	tx.moveCall({
		target: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::launch_dkg_second_round`,
		arguments: [
			tx.object(dwalletCapId),
			tx.pure(bcs.vector(bcs.u8()).serialize(publicKeyShareAndProof)),
			tx.pure(bcs.vector(bcs.u8()).serialize(firstRoundOutput)),
			tx.pure.id(firstRoundSessionId),
		],
	});

	await client.signAndExecuteTransaction({
		signer: keypair,
		transaction: tx,
		options: {
			showEffects: true,
		},
	});

	const timeout = 5 * 60 * 1000; // 5 minutes in milliseconds
	const startTime = Date.now();

	for (;;) {
		if (Date.now() - startTime > timeout) {
			throw new Error('Timeout: Unable to fetch object within 5 minutes');
		}
		await new Promise((resolve) => setTimeout(resolve, 5000));
		let newEvents = await client.queryEvents({
			query: {
				MoveEventType: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::CompletedDKGSecondRoundEvent`,
			},
		});

		if (newEvents.data.length > 0) {
			let event = newEvents.data[0].parsedJson as { dwallet_cap_id: string; dwallet_id: string };
			if (event.dwallet_cap_id === dwalletCapId) {
				let dwallet = await client.getObject({
					id: event.dwallet_id,
					options: { showContent: true },
				});

				return dwallet.data?.content?.dataType === 'moveObject'
					? (dwallet.data?.content?.fields as {
							id: { id: string };
							dwallet_cap_id: string;
							output: number[];
						})
					: null;
			}
		}
	}
}

export async function fetchObjectByCapID(
	capId: string,
	type: string,
	keypair: Keypair,
	client: PeraClient,
) {
	let cursor = null;
	const timeout = 5 * 60 * 1000; // 5 minutes in milliseconds
	const startTime = Date.now();

	for (;;) {
		if (Date.now() - startTime > timeout) {
			throw new Error('Timeout: Unable to fetch object within 5 minutes');
		}

		const objects = await client.getOwnedObjects({
			owner: keypair.toPeraAddress(),
			cursor: cursor,
		});
		const objectsContent = await client.multiGetObjects({
			ids: objects.data.map((o) => o.data?.objectId!),
			options: { showContent: true },
		});

		const objectsFiltered = objectsContent
			.map((o) => o.data?.content)
			.filter((o) => {
				return (
					// @ts-ignore
					o?.dataType === 'moveObject' && o?.type === type && o.fields['dwallet_cap_id'] === capId
				);
			});
		if (objectsFiltered.length > 0) {
			return objectsFiltered[0];
		} else if (objects.hasNextPage) {
			cursor = objects.nextCursor;
		} else {
			cursor = null;
		}
		await new Promise((r) => setTimeout(r, 500));
	}
}

export type CreatedDwallet = {
	dwalletID: string;
	centralizedDKGOutput: number[];
	decentralizedDKGOutput: number[];
	dwalletCapID: string;
};

export const approveMessages = async (
	client: PeraClient,
	keypair: Keypair,
	dwalletCapId: string,
	messages: Uint8Array[],
) => {
	const tx = new Transaction();
	const [messageApprovals] = tx.moveCall({
		target: `${packageId}::${dWalletModuleName}::approve_messages`,
		arguments: [
			tx.object(dwalletCapId),
			tx.pure(bcs.vector(bcs.vector(bcs.u8())).serialize(messages)),
		],
	});
	tx.transferObjects([messageApprovals], keypair.toPeraAddress());
	return await client.signAndExecuteTransaction({
		signer: keypair,
		transaction: tx,
		options: {
			showEffects: true,
		},
	});
};
