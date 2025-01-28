// noinspection ES6PreferShortImport

// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import type { SerializedBcs } from '@mysten/bcs';

import { bcs } from '../bcs/index.js';
import type { TransactionArgument } from '../transactions/index.js';
import { Transaction } from '../transactions/index.js';
import type { Config, DWallet, DWalletWithSecretKeyShare } from './globals.js';
import { dWalletModuleName, fetchCompletedEvent, packageId } from './globals.js';

const signMoveFunc = `${packageId}::${dWalletModuleName}::sign`;
const prepareFutureSignMoveFunc = `${packageId}::${dWalletModuleName}::prepare_future_sign`;
const completeFutureSignMoveFunc = `${packageId}::${dWalletModuleName}::sign_with_partial_centralized_message_signatures`;
const approveMessagesMoveFunc = `${packageId}::${dWalletModuleName}::approve_messages`;
const completedSignMoveEvent = `${packageId}::${dWalletModuleName}::CompletedSignEvent`;

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
	output_object_id: Array<Array<number>>;
}

export function isCompletedSignEvent(obj: any): obj is CompletedSignEvent {
	return obj && 'session_id' in obj && 'output_object_id' in obj;
}

export async function signMessageTransactionCall(
	c: Config,
	dWallet: DWallet | DWalletWithSecretKeyShare,
	messages: Uint8Array[],
	hash: Hash,
	createSignDataArgs: (TransactionArgument | SerializedBcs<any>)[],
	createSignDataMoveFuncName: string,
	dWalletCurveMoveType: string,
	signDataMoveType: string,
): Promise<CompletedSignEvent> {
	const tx = new Transaction();

	const [messageApprovals] = tx.moveCall({
		target: approveMessagesMoveFunc,
		arguments: [
			tx.object(dWallet.dwallet_cap_id),
			tx.pure(bcs.u8().serialize(hash.valueOf())),
			tx.pure(bcs.vector(bcs.vector(bcs.u8())).serialize(messages)),
		],
	});

	const [signData] = tx.moveCall({
		target: createSignDataMoveFuncName,
		arguments: createSignDataArgs,
	});

	tx.moveCall({
		target: signMoveFunc,
		arguments: [messageApprovals, tx.object(dWallet.id.id), signData],
		typeArguments: [dWalletCurveMoveType, signDataMoveType],
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
	messages: Uint8Array[],
	dWalletID: string,
	createSignDataArgs: (TransactionArgument | SerializedBcs<any>)[],
	creatreSignDataMoveFuncName: string,
	dWalletMoveType: string,
	signDataMoveType: string,
) {
	const tx = new Transaction();

	const [signData] = tx.moveCall({
		target: creatreSignDataMoveFuncName,
		arguments: createSignDataArgs,
	});

	tx.moveCall({
		target: prepareFutureSignMoveFunc,
		arguments: [
			tx.pure(bcs.vector(bcs.vector(bcs.u8())).serialize(messages)),
			signData,
			tx.object(dWalletID),
		],
		typeArguments: [dWalletMoveType, signDataMoveType],
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
		throw new Error(`${prepareFutureSignMoveFunc} failed: ${res.errors}`);
	}

	return createdPartiallySignedMessagesEvent;
}

export async function completeFutureSignTransactionCall(
	c: Config,
	messages: Uint8Array[],
	hash: Hash,
	dWalletCapID: string,
	partialSignaturesObjectID: string,
	signDataMoveType: string,
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
		target: completeFutureSignMoveFunc,
		arguments: [tx.object(partialSignaturesObjectID), messageApprovals],
		typeArguments: [signDataMoveType],
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
