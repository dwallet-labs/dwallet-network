// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
// noinspection ES6PreferShortImport

import type { SerializedBcs } from '@mysten/bcs';

import { bcs } from '../bcs/index.js';
import type { TransactionArgument } from '../transactions/index.js';
import { Transaction } from '../transactions/index.js';
import { PERA_SYSTEM_STATE_OBJECT_ID } from '../utils/index.js';
import type { Config, DWallet, DWalletWithSecretKeyShare, StartSessionEvent } from './globals.js';
import {
	dWallet2PCMPCECDSAK1ModuleName,
	dWalletModuleName,
	dWalletPackageID,
	fetchCompletedEvent,
	isStartSessionEvent,
} from './globals.js';

const signMoveFunc = `${dWalletPackageID}::${dWalletModuleName}::sign`;
const requestFutureSignMoveFunc = `${dWalletPackageID}::${dWalletModuleName}::request_future_sign`;
const completeFutureSignMoveFunc = `${dWalletPackageID}::${dWalletModuleName}::sign_with_partial_centralized_message_signatures`;
const approveMessagesMoveFunc = `${dWalletPackageID}::${dWalletModuleName}::approve_messages`;
const completedSignMoveEvent = `${dWalletPackageID}::${dWalletModuleName}::CompletedSignEvent`;

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

export interface CreatedPartialCentralizedSignedMessagesEvent {
	session_id: string;
	partial_signatures_object_id: string;
}

export interface CompletedSignEvent {
	session_id: string;
	output_object_id: Array<Array<number>>;
	signatures: Array<Array<number>>;
	is_future_sign: boolean;
}

export function isCompletedSignEvent(obj: any): obj is CompletedSignEvent {
	return (
		obj &&
		'session_id' in obj &&
		'output_object_id' in obj &&
		'signatures' in obj &&
		'is_future_sign' in obj
	);
}

export async function signMessageTransactionCall(
	c: Config,
	tx: Transaction,
	dWallet: DWallet | DWalletWithSecretKeyShare,
	messages: Uint8Array[],
	hash: Hash,
	createSignDataArgs: (TransactionArgument | SerializedBcs<any>)[],
	createSignDataMoveFuncName: string,
	dWalletCurveMoveType: string,
	signDataMoveType: string,
): Promise<CompletedSignEvent> {
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
		arguments: [
			tx.object(dWallet.id.id),
			messageApprovals,
			signData,
			tx.sharedObjectRef({
				objectId: PERA_SYSTEM_STATE_OBJECT_ID,
				initialSharedVersion: 1,
				mutable: false,
			}),
		],
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
): obj is CreatedPartialCentralizedSignedMessagesEvent {
	return obj && 'partial_signatures_object_id' in obj && 'session_id' in obj;
}

let startPartialSignatureVerificationEventMoveType = `${dWalletPackageID}::${dWalletModuleName}::StartPartialSignaturesVerificationEvent<${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::SignData>`;

export async function partiallySignMessageTransactionCall(
	c: Config,
	tx: Transaction,
	messages: Uint8Array[],
	dWalletID: string,
	signatureAlgorithmData: (TransactionArgument | SerializedBcs<any>)[],
	createSignatureAlgorithmDataMoveFunc: string,
	dWalletMoveType: string,
	signatureDataMoveType: string,
	hash: Hash,
): Promise<CreatedPartialCentralizedSignedMessagesEvent> {
	const [signData] = tx.moveCall({
		target: createSignatureAlgorithmDataMoveFunc,
		arguments: signatureAlgorithmData,
	});

	tx.moveCall({
		target: requestFutureSignMoveFunc,
		arguments: [
			tx.object(dWalletID),
			tx.pure(bcs.vector(bcs.vector(bcs.u8())).serialize(messages)),
			signData,
			tx.pure(bcs.u8().serialize(hash.valueOf())),
			tx.sharedObjectRef({
				objectId: PERA_SYSTEM_STATE_OBJECT_ID,
				initialSharedVersion: 1,
				mutable: false,
			}),
		],
		typeArguments: [dWalletMoveType, signatureDataMoveType],
	});

	let res = await c.client.signAndExecuteTransaction({
		signer: c.keypair,
		transaction: tx,
		options: {
			showEvents: true,
		},
	});

	const sessionID = (
		res.events?.find(
			(event) =>
				event.type === startPartialSignatureVerificationEventMoveType &&
				isStartSessionEvent(event.parsedJson),
		)?.parsedJson as StartSessionEvent
	).session_id;

	return await fetchCompletedEvent<CreatedPartialCentralizedSignedMessagesEvent>(
		c,
		sessionID,
		`${dWalletPackageID}::${dWalletModuleName}::CreatedPartialCentralizedSignedMessagesEvent`,
		isCreatedPartiallySignedMessagesEvent,
	);
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
		arguments: [
			tx.object(partialSignaturesObjectID),
			messageApprovals,
			tx.sharedObjectRef({
				objectId: PERA_SYSTEM_STATE_OBJECT_ID,
				initialSharedVersion: 1,
				mutable: false,
			}),
		],
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
