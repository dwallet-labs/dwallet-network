// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
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

export async function launchDKGSession(keypair: Keypair, client: PeraClient) {
	const tx = new Transaction();
	tx.moveCall({
		target: `${packageId}::${dwalletModuleName}::launch_initiate_dkg_session`,
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

	// sleep five seconds
	await new Promise((resolve) => setTimeout(resolve, 5000));
	let firstRoundOutput = await fetchObjectBySessionId(
		sessionRef.objectId,
		`${packageId}::${dwalletModuleName}::CompletedFirstDKGRoundData`,
		keypair,
		client,
	);
	console.log({ firstRoundOutput });
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
