// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
// noinspection ES6PreferShortImport

import { bcs } from '../bcs/index.js';
import { Transaction } from '../transactions/index.js';
import type { Config } from './globals.js';
import { dWallet2PCMPCECDSAK1ModuleName, fetchObjectBySessionId, packageId } from './globals.js';

const signMoveFunc = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::sign`;
const singOutputMoveType = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::SignOutput`;

export enum Hash {
	KECCAK256 = 0,
	SHA256 = 1,
}

export interface StartSignEvent {
	// Hex string representing the session ID
	session_id: string;

	// Hex string representing the presign session ID
	presign_session_id: string;

	// Address of the user who initiated the signing process
	initiator: string;

	// Hex string representing the dWallet ID
	dwallet_id: string;

	// Hex string representing the dWallet capability ID
	dwallet_cap_id: string;

	// Vector of unsigned 8-bit integers
	dkg_output: number[];

	// Vector of unsigned 8-bit integers
	hashed_message: number[];

	// Vector of unsigned 8-bit integers
	presign_first_round_output: number[];

	// Vector of unsigned 8-bit integers
	presign_second_round_output: number[];

	// Vector of unsigned 8-bit integers
	centralized_signed_message: number[];
}

interface SignOutput {
	id: { id: string };
	session_id: string;
	dwallet_id: string;
	output: number[];
}

export async function signMessageTransactionCall(
	c: Config,
	dwalletCapID: string,
	hashedMessage: Uint8Array,
	dwalletID: string,
	presignID: string,
	centralizedSignedMessage: Uint8Array,
	presignSessionID: string,
) {
	const tx = new Transaction();
	tx.moveCall({
		target: signMoveFunc,
		arguments: [
			tx.object(dwalletCapID),
			tx.pure(bcs.vector(bcs.u8()).serialize(hashedMessage)),
			tx.object(dwalletID),
			tx.object(presignID),
			tx.pure(bcs.vector(bcs.u8()).serialize(centralizedSignedMessage)),
			tx.object(presignSessionID),
		],
	});

	let res = await c.client.signAndExecuteTransaction({
		signer: c.keypair,
		transaction: tx,
		options: {
			showEvents: true,
		},
	});

	const startSignEvent = isStartSignEvent(res.events?.at(0)?.parsedJson)
		? (res.events?.at(0)?.parsedJson as StartSignEvent)
		: null;

	if (!startSignEvent) {
		throw new Error(`${signMoveFunc} failed: ${res.errors}`);
	}
	let obj = await fetchObjectBySessionId(startSignEvent.session_id, singOutputMoveType, c);

	const signOutput =
		obj?.dataType === 'moveObject' && isSignOutput(obj.fields) ? (obj.fields as SignOutput) : null;

	if (!signOutput) {
		throw new Error(`wrong object of type ${singOutputMoveType}, got: ${obj}`);
	}

	return signOutput;
}

function isStartSignEvent(obj: any): obj is StartSignEvent {
	return (
		obj &&
		'session_id' in obj &&
		'presign_session_id' in obj &&
		'initiator' in obj &&
		'dwallet_id' in obj &&
		'dwallet_cap_id' in obj &&
		'dkg_output' in obj &&
		'hashed_message' in obj &&
		'presign_first_round_output' in obj &&
		'presign_second_round_output' in obj &&
		'centralized_signed_message' in obj
	);
}

function isSignOutput(obj: any): obj is SignOutput {
	return obj && obj.id && obj.session_id && obj.output && obj.dwallet_id;
}
