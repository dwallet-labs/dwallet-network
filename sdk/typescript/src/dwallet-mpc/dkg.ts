// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
// noinspection ES6PreferShortImport

import { create_dkg_centralized_output } from '@dwallet-network/dwallet-mpc-wasm';

import { bcs } from '../bcs/index.js';
import { Transaction } from '../transactions/index.js';
import type { Config } from './globals.js';
import {
	dWallet2PCMPCECDSAK1ModuleName,
	dWalletModuleName,
	dWalletPackageID,
	fetchObjectFromEvent,
} from './globals.js';

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

function isDKGFirstRoundOutput(o: any): o is DKGFirstRoundOutput {
	return 'id' in o && 'session_id' in o && 'output' in o && 'dwallet_cap_id' in o;
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

async function launchDKGSecondRound(
	c: Config,
	firstRound: DKGFirstRoundOutput,
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

export function isDWallet(obj: any): obj is DWallet {
	return obj && 'id' in obj && 'session_id' in obj && 'dwallet_cap_id' in obj && 'output' in obj;
}

async function dWalletFromEvent(conf: Config, firstRound: DKGFirstRoundOutput): Promise<DWallet> {
	function isCompletedDKGSecondRoundEvent(event: any): event is CompletedDKGSecondRoundEvent {
		return (
			event &&
			event.session_id &&
			event.initiator &&
			event.dwallet_cap_id &&
			event.dwallet_id &&
			Array.isArray(event.value)
		);
	}

	return fetchObjectFromEvent<CompletedDKGSecondRoundEvent, DWallet>({
		conf,
		eventType: completedDKGSecondRoundEventMoveType,
		objectType: dWalletMoveType,
		isEvent: isCompletedDKGSecondRoundEvent,
		isObject: isDWallet,
		filterEvent: (event) => event.dwallet_cap_id === firstRound.dwallet_cap_id,
		getObjectId: (event) => event.dwallet_id,
	});
}

