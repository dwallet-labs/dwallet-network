// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
// noinspection ES6PreferShortImport
import { Buffer } from 'buffer';
import { create_sign_centralized_output } from '@dwallet-network/dwallet-mpc-wasm';

import { bcs } from '../bcs/index.js';
import { Transaction } from '../transactions/index.js';
import { PERA_SYSTEM_STATE_OBJECT_ID } from '../utils/index.js';
import { decryptAndVerifyUserShare } from './encrypt-user-share.js';
import type { Config, DWallet } from './globals.js';
import {
	dWallet2PCMPCECDSAK1ModuleName,
	dWalletMoveType,
	dWalletPackageID,
	fetchCompletedEvent,
	fetchObjectWithType,
	isDWallet,
	MPCKeyScheme,
	packageId,
} from './globals.js';
import { fetchProtocolPublicParameters } from './network-dkg.js';
import { presign } from './presign.js';

const signMoveFunc = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::sign`;
const partiallySignMoveFunc = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::publish_partially_signed_messages`;
const futureSignMoveFunc = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::future_sign`;
const approveMessagesMoveFunc = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::approve_messages`;
const completedSignMoveEvent = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::CompletedSignEvent`;

const MockedProtocolPublicParameters =
	'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';
export const mockedProtocolPublicParameters = Uint8Array.from(
	Buffer.from(MockedProtocolPublicParameters, 'base64'),
);

export enum Hash {
	KECCAK256 = 0,
	SHA256 = 1,
}

export interface EncryptedUserShare {
	dwallet_id: string;
	encrypted_secret_share_and_proof: Uint8Array;
	encryption_key_id: string;
	signed_public_share: Uint8Array;
	encryptor_ed25519_pubkey: Uint8Array;
	encryptor_address: string;
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
	dWalletID: string,
	presignIDs: string[],
	centralizedSignedMessages: Uint8Array[],
): Promise<CompletedSignEvent> {
	const tx = new Transaction();

	const [messageApprovals] = tx.moveCall({
		target: approveMessagesMoveFunc,
		arguments: [
			tx.object(dwalletCapID),
			tx.pure(bcs.vector(bcs.vector(bcs.u8())).serialize(hashedMessages)),
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
			tx.sharedObjectRef({
				objectId: PERA_SYSTEM_STATE_OBJECT_ID,
				initialSharedVersion: 1,
				mutable: false,
			}),
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
	hashedMessages: Uint8Array[],
	dWalletCapID: string,
	partialSignaturesObjectID: string,
): Promise<CompletedSignEvent> {
	const tx = new Transaction();
	const [messageApprovals] = tx.moveCall({
		target: approveMessagesMoveFunc,
		arguments: [
			tx.object(dWalletCapID),
			tx.pure(bcs.vector(bcs.vector(bcs.u8())).serialize(hashedMessages)),
		],
	});
	tx.moveCall({
		target: futureSignMoveFunc,
		arguments: [
			tx.object(partialSignaturesObjectID),
			messageApprovals,
			tx.sharedObjectRef({
				objectId: PERA_SYSTEM_STATE_OBJECT_ID,
				initialSharedVersion: 1,
				mutable: false,
			}),
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

/**
 * Signs a message with a dWallet on-chain encrypted secret share
 * Can be called with any dWallet, as the encrypted secret share is automatically created
 * upon dWallet creation.
 *
 * @param conf The Pera config to run the TXs with
 * @param dwalletID The ID of the dWallet to sign with
 * @param activeEncryptionKeysTableID The ID of the active encryption keys table, that holds the config's client encryption key
 * @param messages The messages to sign
 * @param mockNetworkKey A boolean indicating whether to use a mocked chain MPC network key for testing purposes, or to use the real one.
 * defaults to false, a.k.a. to use the real one.
 */
export async function signWithEncryptedDWallet(
	conf: Config,
	dwalletID: string,
	activeEncryptionKeysTableID: string,
	messages: Uint8Array[],
	mockNetworkKey: boolean = false,
): Promise<CompletedSignEvent> {
	let dWallet = await fetchObjectWithType<DWallet>(conf, dWalletMoveType, isDWallet, dwalletID);
	let encryptedShare = await fetchEncryptedShare(conf, dwalletID);
	let decrypted_share = await decryptAndVerifyUserShare(
		conf,
		activeEncryptionKeysTableID,
		encryptedShare,
		conf.keypair.toPeraAddress(),
		dWallet,
	);
	const presignCompletionEvent = await presign(conf, dWallet.id.id, messages.length);
	let serializedMsgs = bcs.vector(bcs.vector(bcs.u8())).serialize(messages).toBytes();
	let serializedPresigns = bcs
		.vector(bcs.vector(bcs.u8()))
		.serialize(presignCompletionEvent.presigns)
		.toBytes();
	let serializedPresignFirstRoundSessionIds = bcs
		.vector(bcs.string())
		.serialize(
			presignCompletionEvent.first_round_session_ids.map((session_id) => session_id.slice(2)),
		)
		.toBytes();
	let protocolPublicParameters = mockNetworkKey
		? mockedProtocolPublicParameters
		: await fetchProtocolPublicParameters(
				conf,
				MPCKeyScheme.Secp256k1,
				dWallet.dwallet_mpc_network_key_version,
			);
	const [centralizedSignedMsg, hashedMsgs] = create_sign_centralized_output(
		protocolPublicParameters,
		MPCKeyScheme.Secp256k1,
		Uint8Array.from(dWallet.centralized_output),
		decrypted_share,
		serializedPresigns,
		serializedMsgs,
		Hash.SHA256,
		serializedPresignFirstRoundSessionIds,
	);

	console.log('Signing messages');
	return await signMessageTransactionCall(
		conf,
		dWallet.dwallet_cap_id,
		hashedMsgs,
		dWallet.id.id,
		presignCompletionEvent.presign_ids,
		centralizedSignedMsg,
	);
}

const encryptedSecretShareMoveType = `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::EncryptedUserShare`;

function isEncryptedUserShare(obj: any): obj is EncryptedUserShare {
	return (
		obj &&
		'id' in obj &&
		'dwallet_id' in obj &&
		'encrypted_secret_share_and_proof' in obj &&
		'encryption_key_id' in obj &&
		'signed_public_share' in obj &&
		'encryptor_ed25519_pubkey' in obj &&
		'encryptor_address' in obj
	);
}

async function fetchEncryptedShare(conf: Config, dwalletID: string): Promise<EncryptedUserShare> {
	let ownedEncryptedShares = await conf.client.getOwnedObjects({
		owner: conf.keypair.toPeraAddress(),
		options: {
			showContent: true,
			showType: true,
		},
		filter: {
			StructType: encryptedSecretShareMoveType,
		},
	});
	let encryptedShare = ownedEncryptedShares.data.find(
		(share) =>
			share &&
			share.data &&
			share.data.content &&
			'fields' in share.data.content &&
			isEncryptedUserShare(share.data?.content?.fields) &&
			share.data.content.fields.dwallet_id === dwalletID,
	);

	if (
		!(
			encryptedShare &&
			encryptedShare.data &&
			encryptedShare.data.content &&
			'fields' in encryptedShare.data.content &&
			isEncryptedUserShare(encryptedShare.data?.content?.fields)
		)
	) {
		throw new Error(`no encrypted share found for dwallet ${dwalletID}`);
	}

	return encryptedShare.data.content.fields;
}
