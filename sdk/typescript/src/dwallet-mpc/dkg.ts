// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { create_dkg_centralized_output } from '@dwallet-network/dwallet-mpc-wasm';

import { bcs } from '../bcs/index.js';
import type { PeraClient } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';
import { Transaction } from '../transactions/index.js';

const packageId = '0x3';
const dWallet2PCMPCECDSAK1ModuleName = 'dwallet_2pc_mpc_ecdsa_k1';

/**
 * Starts the first round of the DKG protocol to create a new dWallet.
 * The output of this function is being used to generate the input for the second round.
 */
export async function launchDKGFirstRound(keypair: Keypair, client: PeraClient) {
	const tx = new Transaction();
	tx.moveCall({
		target: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::launch_dkg_first_round`,
		arguments: [],
	});
	const result = await client.signAndExecuteTransaction({
		signer: keypair,
		transaction: tx,
		options: {
			showEffects: true,
		},
	});
	const capRef = result.effects?.created?.[0].reference!;
	await new Promise((resolve) => setTimeout(resolve, 5000));
	let firstRoundOutputObject = await fetchObjectByCapID(
		capRef.objectId,
		`${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::DKGFirstRoundOutput`,
		keypair,
		client,
	);

	return firstRoundOutputObject?.dataType === 'moveObject'
		? (firstRoundOutputObject.fields as {
				output: number[];
				dwallet_cap_id: string;
				session_id: string;
			})
		: null;
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

	for (;;) {
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
	for (;;) {
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

export async function createDWallet(
	keypair: Keypair,
	client: PeraClient,
): Promise<CreatedDwallet | null> {
	const firstDKGOutput = await launchDKGFirstRound(keypair, client);
	let [publicKeyShareAndProof, centralizedOutput] = create_dkg_centralized_output(
		Uint8Array.from(firstDKGOutput!.output),
		firstDKGOutput?.session_id!.slice(2)!,
	);
	let dwallet = await launchDKGSecondRound(
		keypair,
		client,
		publicKeyShareAndProof,
		Uint8Array.from(firstDKGOutput!.output),
		firstDKGOutput?.dwallet_cap_id!,
		firstDKGOutput?.session_id!,
	);

	return {
		dwalletID: dwallet?.id!.id!,
		centralizedDKGOutput: centralizedOutput,
		decentralizedDKGOutput: dwallet?.output!,
		dwalletCapID: dwallet?.dwallet_cap_id!,
	};
}
