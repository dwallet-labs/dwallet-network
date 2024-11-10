// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { PeraClient } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';
import { Transaction } from '../transactions/index.js';
import { dWallet2PCMPCECDSAK1ModuleName, fetchObjectBySessionId, packageId } from './globals.js';

export async function presign(
	keypair: Keypair,
	client: PeraClient,
	dwalletId: string,
): Promise<PresignOutput | null> {
	const tx = new Transaction();
	tx.moveCall({
		target: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::launch_presign_first_round`,
		arguments: [tx.object(dwalletId)],
	});

	const res = await client.signAndExecuteTransaction({
		signer: keypair,
		transaction: tx,
		options: {
			showEffects: true,
			showEvents: true,
		},
	});

	const initEvent = res.events?.at(0)?.parsedJson as { session_id: string };

	await new Promise((resolve) => setTimeout(resolve, 5000));
	let firstRoundOutputObject = await fetchObjectBySessionId(
		initEvent.session_id,
		`${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::PresignSessionOutput`,
		keypair,
		client,
	);

	let firstRoundOutputData =
		firstRoundOutputObject?.dataType === 'moveObject'
			? (firstRoundOutputObject.fields as {
					id: { id: string };
					output: number[];
					session_id: string;
				})
			: null;

	for (;;) {
		await new Promise((resolve) => setTimeout(resolve, 5000));
		let newEvents = await client.queryEvents({
			query: {
				MoveEventType: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::CompletedPresignEvent`,
			},
		});

		if (newEvents.data.length > 0) {
			let event = newEvents.data[0].parsedJson as {
				dwallet_id: string;
				sender: string;
				presign_id: string;
			};
			if (event.dwallet_id === dwalletId && event.sender === keypair.toPeraAddress()) {
				let outputObject = await client.getObject({
					id: event.presign_id,
					options: { showContent: true },
				});

				let secondRoundOutputData =
					outputObject?.data?.content?.dataType === 'moveObject'
						? (outputObject.data?.content?.fields as {
								id: { id: string };
								presigns: number[];
								session_id: string;
							})
						: null;

				return {
					presignFirstRoundOutputId: firstRoundOutputData!.id.id,
					encryptionOfMaskAndMaskedKeyShare: firstRoundOutputData!.output,
					presignSecondRoundOutputId: secondRoundOutputData!.id.id,
					noncePublicShareAndEncryptionOfMaskedNonce: secondRoundOutputData!.presigns,
				};
			}
		}
	}
}

export type PresignOutput = {
	presignFirstRoundOutputId: string;
	encryptionOfMaskAndMaskedKeyShare: number[];
	presignSecondRoundOutputId: string;
	noncePublicShareAndEncryptionOfMaskedNonce: number[];
};
