// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
// noinspection ES6PreferShortImport

import { bcs } from '../bcs/index.js';
import { Transaction } from '../transactions/index.js';
import type { Config } from './globals.js';
import { dWallet2PCMPCECDSAK1ModuleName, fetchCompletedEvent, packageId } from './globals.js';

const signMoveFunc = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::sign`;
const partiallySignMoveFunc = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::publish_partially_signed_messages`;
const futureSignMoveFunc = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::future_sign`;
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

export interface CreatedPartiallySignedMessagesEvent {
	partial_signatures_object_id: string;
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
	messages: Uint8Array[],
	hash: Hash,
	dWalletID: string,
	presignIDs: string[],
	centralizedSignedMessages: Uint8Array[],
): Promise<CompletedSignEvent> {
	const tx = new Transaction();

	const [messageApprovals] = tx.moveCall({
		target: approveMessagesMoveFunc,
		arguments: [
			tx.object(dwalletCapID),
			tx.pure(bcs.u8().serialize(hash.valueOf())),
			tx.pure(bcs.vector(bcs.vector(bcs.u8())).serialize(messages)),
		],
	});

	tx.moveCall({
		target: signMoveFunc,
		arguments: [
			messageApprovals,
			tx.pure(bcs.vector(bcs.vector(bcs.u8())).serialize(hashedMessages)),
			tx.makeMoveVec({ elements: presignIDs.map((presignID) => tx.object(presignID)) }),
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
	return await fetchCompletedEvent<CompletedSignEvent>(
		c,
		startBatchSignEvent.session_id,
		completedSignMoveEvent,
		isCompletedSignEvent,
	);
}

export function isStartBatchedSignEvent(obj: any): obj is StartBatchedSignEvent {
	return obj && 'session_id' in obj && 'hashed_messages' in obj && 'initiator' in obj;
}

export function isCreatedPartiallySignedMessagesEvent(
	obj: any,
): obj is CreatedPartiallySignedMessagesEvent {
	return obj && 'partial_signatures_object_id' in obj;
}

export async function partiallySignMessageTransactionCall(
	c: Config,
	hashedMessages: Uint8Array[],
	dWalletID: string,
	presignIDs: string[],
	centralizedSignedMessages: Uint8Array[],
) {
	const tx = new Transaction();

	tx.moveCall({
		target: partiallySignMoveFunc,
		arguments: [
			tx.pure(bcs.vector(bcs.vector(bcs.u8())).serialize(centralizedSignedMessages)),
			tx.pure(bcs.vector(bcs.vector(bcs.u8())).serialize(hashedMessages)),
			tx.makeMoveVec({ elements: presignIDs.map((presignID) => tx.object(presignID)) }),
			tx.object(dWalletID),
		],
	});

	let res = await c.client.signAndExecuteTransaction({
		signer: c.keypair,
		transaction: tx,
		options: {
			showEvents: true,
		},
	});

	const createdPartiallySignedMessagesEvent = isCreatedPartiallySignedMessagesEvent(
		res.events?.at(0)?.parsedJson,
	)
		? (res.events?.at(0)?.parsedJson as CreatedPartiallySignedMessagesEvent)
		: null;

	if (!createdPartiallySignedMessagesEvent) {
		throw new Error(`${partiallySignMoveFunc} failed: ${res.errors}`);
	}

	return createdPartiallySignedMessagesEvent;
}

export async function futureSignTransactionCall(
	c: Config,
	messages: Uint8Array[],
	hash: Hash,
	dWalletCapID: string,
	partialSignaturesObjectID: string,
): Promise<CompletedSignEvent> {
	const tx = new Transaction();
	const [messageApprovals] = tx.moveCall({
		target: approveMessagesMoveFunc,
		arguments: [
			tx.object(dWalletCapID),
			tx.pure(bcs.u8().serialize(hash.valueOf())),
			tx.pure(bcs.vector(bcs.vector(bcs.u8())).serialize(messages)),
		],
	});
	tx.moveCall({
		target: futureSignMoveFunc,
		arguments: [tx.object(partialSignaturesObjectID), messageApprovals],
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
	return await fetchCompletedEvent<CompletedSignEvent>(
		c,
		startBatchSignEvent.session_id,
		completedSignMoveEvent,
		isCompletedSignEvent,
	);
}
