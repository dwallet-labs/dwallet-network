// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
// noinspection ES6PreferShortImport

import { Transaction } from '../transactions/index.js';
import type { Config } from './globals.js';
import {
	dWallet2PCMPCECDSAK1ModuleName,
	fetchObjectBySessionId,
	fetchObjectFromEvent,
	packageId,
} from './globals.js';

const launchPresignFirstRoundMoveFunc = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::launch_presign_first_round`;
const presignSessionOutputMoveType = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::PresignSessionOutput`;
const completedPresignEventMoveType = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::CompletedPresignEvent`;
const presignMoveType = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::Presign`;

interface StartPresignFirstRoundEvent {
	session_id: string;
	initiator: string;
	dwallet_id: string;
	dwallet_cap_id: string;
	dkg_output: number[];
}

// todo(zeev): First Round Session Output should be deleted
interface PresignSessionOutput {
	id: { id: string };
	session_id: string;
	dwallet_id: string;
	dwallet_cap_id: string;
	output: number[];
}

interface CompletedPresignEvent {
	initiator: string;
	dwallet_id: string;
	presign_id: string;
}

interface PresignObjFields {
	id: { id: string };
	session_id: string;
	dwallet_id: string;
	dwallet_cap_id: string;
	presigns: number[];
}

interface PresignOutput {
	/**
	 * Identifier for the first-round output of the presign session.
	 */
	presignFirstRoundOutputId: string;
	/**
	 * The encrypted mask and masked key share from the first round.
	 */
	presignFirstRoundOutputData: number[];
	/**
	 * Identifier for the second-round output of the presign session.
	 */
	presignSecondRoundOutputId: string;
	/**
	 * Nonce public share and encryption of the masked nonce
	 * from the second round.
	 */
	presignSecondRoundOutputData: number[];
	/**
	 * Identifier for the first-round session of the presign process.
	 */
	presignSessionId: string;
}

export async function presign(c: Config, dwalletID: string): Promise<PresignOutput> {
	let firstRoundOutput = await launchPresignFirstRound(dwalletID, c);
	let secondRoundOutput = await presignFromEvent(c, dwalletID);

	return {
		presignFirstRoundOutputId: firstRoundOutput.id.id,
		presignFirstRoundOutputData: firstRoundOutput.output,
		presignSecondRoundOutputId: secondRoundOutput.id.id,
		presignSecondRoundOutputData: secondRoundOutput.presigns,
		presignSessionId: firstRoundOutput.session_id,
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

	let obj = await fetchObjectBySessionId(event.session_id, presignSessionOutputMoveType, c);

	let firstRoundOutput =
		obj?.dataType === 'moveObject' && isPresignSessionOutput(obj.fields)
			? (obj.fields as PresignSessionOutput)
			: null;

	if (!firstRoundOutput) {
		throw new Error(`wrong object of type ${presignSessionOutputMoveType}, got: ${obj}`);
	}
	return firstRoundOutput;
}

function isStartPresignFirstRoundEvent(obj: any): obj is StartPresignFirstRoundEvent {
	return (
		obj && obj.session_id && obj.initiator && obj.dwallet_id && obj.dwallet_cap_id && obj.dkg_output
	);
}

function isPresignSessionOutput(obj: any): obj is PresignSessionOutput {
	return (
		obj &&
		obj.id &&
		obj.session_id &&
		obj.dwallet_id &&
		obj.dwallet_cap_id &&
		Array.isArray(obj.output)
	);
}

async function presignFromEvent(conf: Config, dwalletID: string): Promise<PresignObjFields> {
	function isCompletedPresignEvent(event: any): event is CompletedPresignEvent {
		return event && event.initiator && event.dwallet_id && event.presign_id;
	}

	function isPresignObj(obj: any): obj is PresignObjFields {
		return obj && obj.id && obj.session_id && obj.dwallet_id && obj.dwallet_cap_id && obj.presigns;
	}

	return fetchObjectFromEvent<CompletedPresignEvent, PresignObjFields>({
		conf,
		eventType: completedPresignEventMoveType,
		objectType: presignMoveType,
		isEvent: isCompletedPresignEvent,
		isObject: isPresignObj,
		filterEvent: (event) =>
			event.dwallet_id === dwalletID && event.initiator === conf.keypair.toPeraAddress(),
		getObjectId: (event) => event.presign_id,
	});
}
