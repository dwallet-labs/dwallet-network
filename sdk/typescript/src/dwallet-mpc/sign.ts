// noinspection ES6PreferShortImport

// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { create_sign_centralized_output } from '@dwallet-network/dwallet-mpc-wasm';

import { bcs } from '../bcs/index.js';
import { Transaction } from '../transactions/index.js';
import { EncryptedUserShare, fetchEncryptedUserSecretShare } from './encrypt-user-share.js';
import type { Config, DWallet, DWalletWithSecretKeyShare } from './globals.js';
import {
	dWallet2PCMPCECDSAK1ModuleName,
	dWalletModuleName,
	dWalletMoveType,
	fetchCompletedEvent,
	fetchObjectWithType,
	isDWallet,
	mockedProtocolPublicParameters,
	MPCKeyScheme,
	packageId,
} from './globals.js';
import { fetchProtocolPublicParameters } from './network-dkg.js';
import { presign } from './presign.js';

const signMoveFunc = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::sign`;
const partiallySignMoveFunc = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::publish_partially_signed_messages`;
const futureSignMoveFunc = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::future_sign`;
const approveMessagesMoveFunc = `${packageId}::${dWalletModuleName}::approve_messages`;
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
	dWallet: DWallet | DWalletWithSecretKeyShare,
	messages: Uint8Array[],
	hash: Hash,
	presignIDs: string[],
	centralizedSignedMessages: Uint8Array[],
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

	tx.moveCall({
		target: signMoveFunc,
		arguments: [
			messageApprovals,
			tx.pure(bcs.vector(bcs.vector(bcs.u8())).serialize(messages)),
			tx.makeMoveVec({ elements: presignIDs.map((presignID) => tx.object(presignID)) }),
			tx.object(dWallet.id.id),
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
	messages: Uint8Array[],
	dWalletID: string,
	presignIDs: string[],
	centralizedSignedMessages: Uint8Array[],
) {
	const tx = new Transaction();

	tx.moveCall({
		target: partiallySignMoveFunc,
		arguments: [
			tx.pure(bcs.vector(bcs.vector(bcs.u8())).serialize(centralizedSignedMessages)),
			tx.pure(bcs.vector(bcs.vector(bcs.u8())).serialize(messages)),
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

/**
 * Presigns and Signs a message with the dWallets' on-chain encrypted secret share.
 * Can be called with any dWallet, as the encrypted secret share is automatically created
 * upon dWallet creation.
 *
 * @param conf The Pera config to run the TXs with.
 * @param dwalletID The ID of the dWallet to sign with.
 * @param activeEncryptionKeysTableID The ID of the active encryption keys table that holds the client encryption key.
 * @param messages The messages to sign.
 * @param mockNetworkKey A boolean indicating whether to use a mocked chain MPC network key
 * for testing purposes or to use the real one.
 * defaults to false, a.k.a. to use the real one.
 */
export async function signWithEncryptedDWallet(
	conf: Config,
	dwalletID: string,
	activeEncryptionKeysTableID: string,
	messages: Uint8Array[],
	mockNetworkKey: boolean = false,
): Promise<CompletedSignEvent> {
	const dWallet = await fetchObjectWithType<DWallet>(conf, dWalletMoveType, isDWallet, dwalletID);
	const encryptedSecretShare = await fetchEncryptedUserSecretShare(conf, dwalletID);
	const userShare = EncryptedUserShare.fromConfig(conf);
	// The share is encrypted to myself, this is why the source and dest are the same.
	const decryptedShare = await userShare.decryptAndVerifyUserShare(
		activeEncryptionKeysTableID,
		encryptedSecretShare,
		dWallet,
		conf.keypair.toPeraAddress(),
		conf.keypair,
	);

	const presignCompletionEvent = await presign(conf, dWallet.id.id, messages.length);
	const serializedMsgs = bcs.vector(bcs.vector(bcs.u8())).serialize(messages).toBytes();
	const serializedPresigns = bcs
		.vector(bcs.vector(bcs.u8()))
		.serialize(presignCompletionEvent.presigns)
		.toBytes();
	const serializedPresignFirstRoundSessionIds = bcs
		.vector(bcs.string())
		.serialize(
			presignCompletionEvent.first_round_session_ids.map((session_id) => session_id.slice(2)),
		)
		.toBytes();
	const protocolPublicParameters = mockNetworkKey
		? mockedProtocolPublicParameters
		: await fetchProtocolPublicParameters(
				conf,
				MPCKeyScheme.Secp256k1,
				dWallet.dwallet_mpc_network_decryption_key_version,
			);
	const [centralizedSignedMsg] = create_sign_centralized_output(
		protocolPublicParameters,
		MPCKeyScheme.Secp256k1,
		Uint8Array.from(dWallet.centralized_public_output),
		decryptedShare,
		serializedPresigns,
		serializedMsgs,
		Hash.SHA256,
		serializedPresignFirstRoundSessionIds,
	);

	return await signMessageTransactionCall(
		conf,
		dWallet,
		messages,
		Hash.SHA256,
		presignCompletionEvent.presign_ids,
		centralizedSignedMsg,
	);
}
