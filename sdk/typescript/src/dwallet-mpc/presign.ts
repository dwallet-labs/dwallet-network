// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
// noinspection ES6PreferShortImport

import { bcs } from '@mysten/bcs';

import { Transaction } from '../transactions/index.js';
import type { Config } from './globals.js';
import {
	dWallet2PCMPCECDSAK1ModuleName,
	dWalletPackageID,
	fetchCompletedEvent,
} from './globals.js';

const launchPresignFirstRoundMoveFunc = `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::launch_batched_presign`;
export const presignMoveType = `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::Presign`;
const completedPresignMoveEvent = `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::CompletedBatchedPresignEvent`;

interface StartBatchedPresignEvent {
	session_id: string;
	batch_size: number;
	initiator: string;
}

export interface Presign {
	id: { id: string };
	session_id: string;
	first_round_session_id: string;
	dwallet_id: string;
	presign: number[];
}

interface CompletedBatchedPresignEvent {
	initiator: string;
	dwallet_id: string;
	session_id: string;
	presign_ids: string[];
	first_round_session_ids: string[];
	presigns: number[][];
}

export function isCompletedBatchedPresignEvent(obj: any): obj is CompletedBatchedPresignEvent {
	return (
		obj &&
		'session_id' in obj &&
		'initiator' in obj &&
		'dwallet_id' in obj &&
		'presign_ids' in obj &&
		'first_round_session_ids' in obj &&
		'presigns' in obj
	);
}

export async function presign(
	c: Config,
	dwalletID: string,
	batch_size: number,
): Promise<CompletedBatchedPresignEvent> {
	let sessionID = await launchPresignFirstRound(dwalletID, batch_size, c);
	return await fetchCompletedEvent<CompletedBatchedPresignEvent>(
		c,
		sessionID,
		completedPresignMoveEvent,
		isCompletedBatchedPresignEvent,
	);
}

async function launchPresignFirstRound(
	dwalletID: string,
	batch_size: number,
	c: Config,
): Promise<string> {
	const tx = new Transaction();

	tx.moveCall({
		target: launchPresignFirstRoundMoveFunc,
		arguments: [tx.object(dwalletID), tx.pure(bcs.u64().serialize(batch_size))],
	});

	const res = await c.client.signAndExecuteTransaction({
		signer: c.keypair,
		transaction: tx,
		options: {
			showEffects: true,
			showEvents: true,
		},
	});

	const event = isStartBatchPresignFirstRoundEvent(res.events?.at(0)?.parsedJson)
		? (res.events?.at(0)?.parsedJson as StartBatchedPresignEvent)
		: null;

	if (!event) {
		throw new Error(`${launchPresignFirstRoundMoveFunc} failed: ${res.errors}`);
	}
	return event.session_id;
}

function isStartBatchPresignFirstRoundEvent(obj: any): obj is StartBatchedPresignEvent {
	return obj && `session_id` in obj && `initiator` in obj && `batch_size` in obj;
}

export function isPresign(obj: any): obj is Presign {
	return (
		obj && 'id' in obj && 'first_round_session_id' in obj && 'dwallet_id' in obj && 'presign' in obj
	);
}
