// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { bcs } from '../bcs';
import type { PeraClient } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';
import { Transaction } from '../transactions/index.js';

const packageId = '0x3';
const dWalletProofMPCModuleName = 'proof';

/**
 * Launches a proof MPC session by calling the `launch_proof_mpc_flow` function in the `proof` module.
 */
export async function launchProofMPSession(keypair: Keypair, client: PeraClient) {
	const tx = new Transaction();
	tx.moveCall({
		target: `${packageId}::${dWalletProofMPCModuleName}::launch_proof_mpc_flow`,
		arguments: [],
	});

	await client.signAndExecuteTransaction({
		signer: keypair,
		transaction: tx,
		options: {
			showEffects: true,
		},
	});
}

let dwalletModuleName = 'dwallet';
const dWallet2PCMPCECDSAK1ModuleName = 'dwallet_2pc_mpc_ecdsa_k1';

export async function startFirstDKGSession(keypair: Keypair, client: PeraClient) {
	const tx = new Transaction();
	tx.moveCall({
		target: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::start_first_dkg_session`,
		arguments: [],
	});

	const result = await client.signAndExecuteTransaction({
		signer: keypair,
		transaction: tx,
		options: {
			showEffects: true,
		},
	});
	const sessionRef = result.effects?.created?.filter((o) => o.owner === 'Immutable')[0].reference!;

	await new Promise((resolve) => setTimeout(resolve, 5000));
	let firstRoundOutputObject = await fetchObjectBySessionId(
		sessionRef.objectId,
		`${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::CompletedFirstDKGRoundData`,
		keypair,
		client,
	);

	let firstRoundOutput =
		firstRoundOutputObject?.dataType === 'moveObject'
			? (firstRoundOutputObject.fields as {
					value: number[];
				})
			: null;

	return firstRoundOutput?.value;
}

export async function launchDKGSecondRound(
	keypair: Keypair,
	client: PeraClient,
	publicKeyShareAndProof: Uint8Array,
	firstRoundOutput: Uint8Array,
) {
	const tx = new Transaction();
	tx.moveCall({
		target: `${packageId}::${dwalletModuleName}::launch_dkg_second_round`,
		arguments: [
			tx.pure(bcs.vector(bcs.u8()).serialize(publicKeyShareAndProof)),
			tx.pure(bcs.vector(bcs.u8()).serialize(firstRoundOutput)),
		],
	});

	await client.signAndExecuteTransaction({
		signer: keypair,
		transaction: tx,
		options: {
			showEffects: true,
		},
	});
}

export async function fetchObjectBySessionId(
	sessionId: string,
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
					o?.dataType === 'moveObject' && o?.type === type && o.fields['session_id'] === sessionId
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
