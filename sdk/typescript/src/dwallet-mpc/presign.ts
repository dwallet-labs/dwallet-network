// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
// noinspection ES6PreferShortImport

import { Transaction } from '../transactions/index.js';
import type { Config } from './globals.js';
import { dWallet2PCMPCECDSAK1ModuleName, fetchObjectFromEvent, packageId } from './globals.js';

const launchPresignFirstRoundMoveFunc = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::launch_presign_first_round`;
const completedPresignEventMoveType = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::CompletedPresignEvent`;
export const presignMoveType = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::Presign`;

interface StartPresignFirstRoundEvent {
	session_id: string;
	initiator: string;
	dwallet_id: string;
	dwallet_cap_id: string;
	dkg_output: number[];
}

interface CompletedPresignEvent {
	initiator: string;
	dwallet_id: string;
	presign_id: string;
}

export interface Presign {
	id: { id: string };
	session_id: string;
	first_round_session_id: string;
	dwallet_id: string;
	dwallet_cap_id: string;
	first_round_output: number[];
	second_round_output: number[];
}

interface PresignOutput {
	// todo(zeev): remove this?
	secondRoundOutputID: string;
	/**
	 * The encrypted mask and masked key share from the first round.
	 */
	firstRoundOutput: number[];
	/**
	 * Nonce public share and encryption of the masked nonce
	 * from the second round.
	 */
	secondRoundOutput: number[];
	/**
	 * Identifier for the first-round session of the presign process.
	 */
	firstRoundSessionID: string;
}

export async function presign(c: Config, dwalletID: string): Promise<PresignOutput> {
	await launchPresignFirstRound(dwalletID, c);
	let secondRoundOutput = await presignFromEvent(c, dwalletID);

	return {
		secondRoundOutputID: secondRoundOutput.id.id,
		firstRoundOutput: secondRoundOutput.first_round_output,
		secondRoundOutput: secondRoundOutput.second_round_output,
		firstRoundSessionID: secondRoundOutput.first_round_session_id,
	};
}

async function launchPresignFirstRound(dwalletID: string, c: Config) {
	const tx = new Transaction();

	tx.moveCall({
		target: launchPresignFirstRoundMoveFunc,
		arguments: [tx.object(dwalletID)],
	});

	const res = await c.client.signAndExecuteTransaction({
		signer: c.keypair,
		transaction: tx,
		options: {
			showEffects: true,
			showEvents: true,
		},
	});

	const event = isStartPresignFirstRoundEvent(res.events?.at(0)?.parsedJson)
		? (res.events?.at(0)?.parsedJson as StartPresignFirstRoundEvent)
		: null;

	if (!event) {
		throw new Error(`${launchPresignFirstRoundMoveFunc} failed: ${res.errors}`);
	}
}

function isStartPresignFirstRoundEvent(obj: any): obj is StartPresignFirstRoundEvent {
	return (
		obj && obj.session_id && obj.initiator && obj.dwallet_id && obj.dwallet_cap_id && obj.dkg_output
	);
}

export function isPresign(obj: any): obj is Presign {
	return (
		obj &&
		'id' in obj &&
		'session_id' in obj &&
		'first_round_session_id' in obj &&
		'dwallet_id' in obj &&
		'dwallet_cap_id' in obj &&
		'first_round_output' in obj &&
		'second_round_output' in obj
	);
}

async function presignFromEvent(conf: Config, dwalletID: string): Promise<Presign> {
	function isCompletedPresignEvent(event: any): event is CompletedPresignEvent {
		return event && `initiator` in event && `dwallet_id` in event && `presign_id` in event;
	}

	return fetchObjectFromEvent<CompletedPresignEvent, Presign>({
		conf,
		eventType: completedPresignEventMoveType,
		objectType: presignMoveType,
		isEvent: isCompletedPresignEvent,
		isObject: isPresign,
		filterEvent: (event) =>
			event.dwallet_id === dwalletID && event.initiator === conf.keypair.toPeraAddress(),
		getObjectId: (event) => event.presign_id,
	});
}
