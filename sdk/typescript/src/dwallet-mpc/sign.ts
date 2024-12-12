// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
// noinspection ES6PreferShortImport

import { bcs } from '../bcs/index.js';
import { Transaction } from '../transactions/index.js';
import type { Config } from './globals.js';
import { dWallet2PCMPCECDSAK1ModuleName, packageId } from './globals.js';

const signMoveFunc = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::sign`;
// const singOutputMoveType = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::SignOutput`;
const approveMessagesMoveFunc = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::approve_messages`;
const completedSignMoveEvent = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::CompletedSignEvent`;

export enum Hash {
	KECCAK256 = 0,
	SHA256 = 1,
}

export interface StartBatchedSignEvent {
	// Hexadecimal string representing the session ID (ID).
	session_id: string;
	// 2D array representing the list of hashed messages.
	hashed_messages: number[][];
	// Address of the user who initiated the process.
	initiating_user: string;
}

export interface CompletedSignEvent {
	session_id: string;
	signed_messages: Array<Array<number>>;
}

export function isCompletedSignEvent(obj: any): obj is CompletedSignEvent {
	return obj && 'session_id' in obj && 'signed_messages' in obj;
}

export async function signMessageTransactionCall(
	c: Config,
	dwalletCapID: string,
	hashedMessages: Uint8Array[],
	dWalletID: string,
	presignID: string,
	centralizedSignedMessages: Uint8Array[],
) {
	const tx = new Transaction();

	const [messageApprovals] = tx.moveCall({
		target: approveMessagesMoveFunc,
		arguments: [
			tx.object(dwalletCapID),
			tx.pure(bcs.vector(bcs.vector(bcs.u8())).serialize(hashedMessages)),
		],
	});

	tx.moveCall({
		target: signMoveFunc,
		arguments: [
			messageApprovals,
			tx.pure(bcs.vector(bcs.vector(bcs.u8())).serialize(hashedMessages)),
			tx.object(presignID),
			tx.object(dWalletID),
			tx.pure(bcs.vector(bcs.vector(bcs.u8())).serialize(centralizedSignedMessages)),
		],
	});

	let res = await c.client.signAndExecuteTransaction({
		signer: c.keypair,
		transaction: tx,
		options: {
			showEvents: true,
		},
	});

	const startBatchSignEvent = isStartBatchedSignEvent(res.events?.at(0)?.parsedJson)
		? (res.events?.at(0)?.parsedJson as StartBatchedSignEvent)
		: null;

	if (!startBatchSignEvent) {
		throw new Error(`${signMoveFunc} failed: ${res.errors}`);
	}

	return await fetchCompleteSignEvent(c, startBatchSignEvent.session_id);
}

export function isStartBatchedSignEvent(obj: any): obj is StartBatchedSignEvent {
	return obj && 'session_id' in obj && 'hashed_messages' in obj && 'initiating_user' in obj;
}

// function isSignOutput(obj: any): obj is SignOutput {
// 	return obj && obj.id && obj.session_id && obj.output && obj.dwallet_id;
// }

async function fetchCompleteSignEvent(c: Config, sessionID: string): Promise<CompletedSignEvent> {
	const startTime = Date.now();
	let cursor = null;

	while (Date.now() - startTime <= c.timeout) {
		// Wait for a bit before polling again, objects might not be available immediately.
		await new Promise((resolve) => setTimeout(resolve, 5_000));

		const { data, nextCursor, hasNextPage } = await c.client.queryEvents({
			query: {
				TimeRange: {
					startTime: (Date.now() - c.timeout).toString(),
					endTime: Date.now().toString(),
				},
			},
			cursor,
		});

		const match = data.find(
			(event) =>
				event.type === completedSignMoveEvent &&
				isCompletedSignEvent(event.parsedJson) &&
				event.parsedJson.session_id === sessionID,
		);
		if (match) {
			return match.parsedJson as CompletedSignEvent;
		}
		if (hasNextPage) {
			cursor = nextCursor;
		}
	}

	const seconds = ((Date.now() - startTime) / 1000).toFixed(2);
	throw new Error(
		`timeout: unable to fetch an event of type ${completedSignMoveEvent} within ${
			c.timeout / (60 * 1000)
		} minutes (${seconds} seconds passed).`,
	);
}
