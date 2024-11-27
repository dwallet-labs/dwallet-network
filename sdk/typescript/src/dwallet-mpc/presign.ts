// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { PeraClient } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';
import { Transaction } from '../transactions/index.js';
import { dWallet2PCMPCECDSAK1ModuleName, packageId } from './globals.js';

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

	await client.signAndExecuteTransaction({
		signer: keypair,
		transaction: tx,
		options: {
			showEffects: true,
			showEvents: true,
		},
	});

	const timeout = 8 * 60 * 1000; // 5 minutes in milliseconds
	const startTime = Date.now();

	for (;;) {
		if (Date.now() - startTime > timeout) {
			throw new Error('Timeout: Unable to fetch object, reached timeout');
		}

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
								first_round_session_id: string;
								first_round_output: number[];
								second_round_output: number[];
								session_id: string;
							})
						: null;

				return {
					id: secondRoundOutputData!.id.id,
					firstRoundOutput: secondRoundOutputData!.first_round_output,
					secondRoundOutput: secondRoundOutputData!.second_round_output,
					sessionId: secondRoundOutputData!.first_round_session_id,
				};
			}
		}
	}
}

export type PresignOutput = {
	id: string;
	firstRoundOutput: number[];
	secondRoundOutput: number[];
	sessionId: string;
};
