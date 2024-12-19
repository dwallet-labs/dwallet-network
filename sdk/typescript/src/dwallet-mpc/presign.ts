// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
// noinspection ES6PreferShortImport

import { bcs } from '@mysten/bcs';

import { Transaction } from '../transactions/index.js';
import type { Config } from './globals.js';
import { dWallet2PCMPCECDSAK1ModuleName, fetchCompletedEvent, packageId } from './globals.js';
import { completedPresignMoveEvent } from './sign.js';

const launchPresignFirstRoundMoveFunc = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::launch_batched_presign`;
export const presignMoveType = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::Presign`;

interface StartPresignFirstRoundEvent {
	session_id: string;
	initiator: string;
	dwallet_id: string;
	dkg_output: number[];
}

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

export interface CompletedPresignEvent {
	session_id: string;
	initiator: string;
	dwallet_id: string;
	presign_ids: string[];
	first_round_session_ids: string[];
	presigns: number[][];
}

export function isCompletedPresignEvent(obj: any): obj is CompletedPresignEvent {
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
): Promise<CompletedPresignEvent> {
	let sessionID = await launchPresignFirstRound(dwalletID, batch_size, c);
	return await fetchCompletedEvent<CompletedPresignEvent>(
		c,
		sessionID,
		completedPresignMoveEvent,
		isCompletedPresignEvent,
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

function isStartBatchPresignFirstRoundEvent(obj: any): obj is StartPresignFirstRoundEvent {
	return obj && obj.session_id && obj.initiator && obj.batch_size;
}

export function isPresign(obj: any): obj is Presign {
	return (
		obj && 'id' in obj && 'first_round_session_id' in obj && 'dwallet_id' in obj && 'presign' in obj
	);
}
