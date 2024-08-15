// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { bcs } from '../bcs/index.js';
import { TransactionBlock } from '../builder/index.js';
import {DWalletClient, ObjectOwner} from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';
import type { SuiObjectRef } from '../types/index.js';
import { fetchObjectBySessionId } from './utils.js';

const packageId = '0x3';
const dWalletModuleName = 'dwallet';
const dWallet2PCMPCECDSAK1ModuleName = 'dwallet_2pc_mpc_ecdsa_k1';

export enum EncryptionKeyScheme {
	Paillier = 0,
}

export async function approveAndSign(
	dwalletCapId: string,
	signMessagesId: string,
	messages: Uint8Array[],
	keypair: Keypair,
	client: DWalletClient,
) {
	const tx = new TransactionBlock();
	const [messageApprovals] = tx.moveCall({
		target: `${packageId}::${dWalletModuleName}::approve_messages`,
		arguments: [
			tx.object(dwalletCapId),
			tx.pure(bcs.vector(bcs.vector(bcs.u8())).serialize(messages)),
		],
	});
	tx.moveCall({
		target: `${packageId}::${dWalletModuleName}::sign`,
		typeArguments: [
			`${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::SignData`,
			`${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::NewSignDataEvent`,
		],
		arguments: [tx.object(signMessagesId), messageApprovals],
	});
	const result = await client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: tx,
		options: {
			showEffects: true,
		},
	});

	const signSessionRef = result.effects?.created?.filter((o) => o.owner === 'Immutable')[0]
		.reference!;

	const signOutput = await fetchObjectBySessionId(
		signSessionRef.objectId,
		`${packageId}::${dWalletModuleName}::SignOutput`,
		keypair,
		client,
	);

	const fields =
		signOutput?.dataType === 'moveObject'
			? (signOutput.fields as {
					id: { id: string };
					signatures: number[][];
			  })
			: null;

	return fields
		? {
				signOutputId: fields.id.id,
				signatures: fields.signatures,
		  }
		: null;
}

export const storeEncryptionKey = async (
	encryptionKey: Uint8Array,
	encryptionKeyScheme: EncryptionKeyScheme,
	keypair: Keypair,
	client: DWalletClient,
): Promise<SuiObjectRef> => {
	const tx = new TransactionBlock();
	let purePubKey = tx.pure(bcs.vector(bcs.u8()).serialize(encryptionKey));
	tx.moveCall({
		target: `${packageId}::${dWalletModuleName}::register_encryption_key`,
		arguments: [purePubKey, tx.pure(bcs.u8().serialize(encryptionKeyScheme))],
	});
	let result = await client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: tx,
		options: {
			showEffects: true,
		},
	});
	return result.effects?.created?.filter((o) => o.owner === 'Immutable')[0].reference!;
};

export const getEncryptionKeyByObjectId = async (
	client: DWalletClient,
	encryptionKeyObjID: string,
) => {
	const response = await client.getObject({
		id: encryptionKeyObjID,
		options: { showContent: true },
	});

	const objectFields =
		response.data?.content?.dataType === 'moveObject'
			? (response.data?.content?.fields as unknown as {
					encryption_key: Uint8Array;
					key_owner_address: string;
			  })
			: null;

	return objectFields
		? {
				encryptionKey: objectFields?.encryption_key,
				keyOwnerAddress: objectFields?.key_owner_address,
		  }
		: null;
};

export const getDwalletActiveEncryptionKey = async (
	// client: DWalletClient,
	keypair: Keypair,
	encryptionKeysHolderID: string,
) => {
	const tx = new TransactionBlock();
	const encryptionKeysHolder = tx.object(encryptionKeysHolderID);

	console.log(keypair.toSuiAddress());

	let [active_encryption_key_id] = tx.moveCall({
		target: `${packageId}::${dWalletModuleName}::get_encryption_key`,
		arguments: [encryptionKeysHolder, tx.pure(keypair.toSuiAddress())],
	});

	// console.log(b);

	// let a = await client.signAndExecuteTransactionBlock({
	// 	signer: keypair,
	// 	transactionBlock: tx,
	// 	options: {
	// 		showEffects: true,
	// 	},
	// });

	// console.log(a);
	return active_encryption_key_id;
};

export const setDwalletPrimaryEncryptionKey = async (
	client: DWalletClient,
	keypair: Keypair,
	encryptionKeyObjID: string,
	encryptionKeysHolderID: string,
) => {
	const tx = new TransactionBlock();
	const EncKeyObj = tx.object(encryptionKeyObjID);
	const encryptionKeysHolder = tx.object(encryptionKeysHolderID);

	tx.moveCall({
		target: `${packageId}::${dWalletModuleName}::set_active_encryption_key`,
		arguments: [encryptionKeysHolder, EncKeyObj],
	});

	return await client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: tx,
		options: {
			showEffects: true,
		},
	});
};

export const createEncryptionKeysHolder = async (client: DWalletClient, keypair: Keypair) => {
	const tx = new TransactionBlock();
	tx.moveCall({
		target: `${packageId}::${dWalletModuleName}::create_encryption_keys_holder`,
		arguments: [],
	});

	let result = await client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: tx,
		options: {
			showEffects: true,
		},
	});

	return result.effects?.created?.filter(
		(o) =>
			typeof o.owner === 'object' &&
			'Shared' in o.owner &&
			o.owner.Shared.initial_shared_version !== undefined,
	)[0].reference!;
};

export const transferDwallet = async (
	client: DWalletClient,
	keypair: Keypair,
	encryptedUserShareAndProof: Uint8Array,
	encryptionKeyObjID: string,
	dwalletID: string,
) => {
	const tx = new TransactionBlock();
	const pub_key_obj = tx.object(encryptionKeyObjID);
	const dwallet = tx.object(dwalletID);

	tx.moveCall({
		target: `${packageId}::${dWalletModuleName}::encrypt_user_share`,
		arguments: [dwallet, pub_key_obj, tx.pure(encryptedUserShareAndProof)],
	});

	return await client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: tx,
		options: {
			showEffects: true,
		},
	});
};