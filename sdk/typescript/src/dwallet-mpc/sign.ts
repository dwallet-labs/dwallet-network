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

interface StartSignEvent {
	session_id: string;
	presign_session_id: string;
	initiator: string;
	dwallet_id: string;
	dwallet_cap_id: string;
	dkg_output: Uint8Array;
	hashed_message: Uint8Array;
	presign: Uint8Array;
	centralized_signed_message: Uint8Array;
}

interface SignOutput {
	id: { id: string };
	session_id: string;
	output: number[];
}

export async function signMessageTransactionCall(
	c: Config,
	hashedMessage: Uint8Array,
	presign: Uint8Array,
	dkgOutput: Uint8Array,
	centralizedSignedMessage: Uint8Array,
	presignFirstRoundSessionId: string,
) {
	const tx = new Transaction();

	tx.moveCall({
		target: signMoveFunc,
		arguments: [
			tx.pure(bcs.vector(bcs.u8()).serialize(hashedMessage)),
			tx.pure(bcs.vector(bcs.u8()).serialize(presign)),
			tx.pure(bcs.vector(bcs.u8()).serialize(dkgOutput)),
			tx.pure(bcs.vector(bcs.u8()).serialize(centralizedSignedMessage)),
			tx.pure.id(presignFirstRoundSessionId),
		],
	});

	let res = await c.client.signAndExecuteTransaction({
		signer: c.keypair,
		transaction: tx,
		options: {
			showEvents: true,
		},
	});

	const event = isStartSignEvent(res.events?.at(0)?.parsedJson)
		? (res.events?.at(0)?.parsedJson as StartSignEvent)
		: null;

	if (!event) {
		throw new Error(`${signMoveFunc} failed: ${res.errors}`);
	}

	let obj = await fetchObjectBySessionId(event.session_id, singOutputMoveType, c);

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
		obj.session_id &&
		obj.presign_session_id &&
		obj.initiator &&
		obj.dwallet_id &&
		obj.dwallet_cap_id &&
		obj.dkg_output &&
		obj.hashed_message &&
		obj.presign &&
		obj.centralized_signed_message
	);
}

function isSignOutput(obj: any): obj is SignOutput {
	return obj && obj.id && obj.session_id && obj.output;
}
