// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { bcs } from '../bcs/index.js';
import { TransactionBlock } from '../builder/index.js';
import type { DWalletClient } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';
import { SuiObjectRef } from '../types';
import { fetchObjectBySessionId } from './utils.js';

const packageId = '0x3';
const dWalletModuleName = 'dwallet';
const dWallet2PCMPCECDSAK1ModuleName = 'dwallet_2pc_mpc_ecdsa_k1';
const dWalletTransferModuleName = 'dwallet_transfer';

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

export const storePublicKey = async (
	public_key: Uint8Array,
	keypair: Keypair,
	client: DWalletClient,
): Promise<SuiObjectRef> => {
	const tx = new TransactionBlock();
	let purePubKey = tx.pure(bcs.vector(bcs.u8()).serialize(public_key));
	tx.moveCall({
		target: `${packageId}::${dWalletTransferModuleName}::store_public_key`,
		arguments: [purePubKey],
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

export const getPublicKeyByObjectId = async (client: DWalletClient, publicKeyObjID: string) => {
	const response = await client.getObject({
		id: publicKeyObjID,
		options: { showContent: true },
	});

	const objectFields =
		response.data?.content?.dataType === 'moveObject'
			? (response.data?.content?.fields as {
					public_key: Uint8Array;
					key_owner_address: string;
			  })
			: null;

	return objectFields
		? {
				publicKey: objectFields?.public_key,
				keyOwnerAddress: objectFields?.key_owner_address,
		  }
		: null;
};

export const transferDwallet = async (
	client: DWalletClient,
	keypair: Keypair,
	proof: Uint8Array,
	encrypted_secret_share: Uint8Array,
	range_commitment: Uint8Array,
	publicKeyObjID: string,
	dwalletID: string,
	recipient_address: string,
) => {
	const tx = new TransactionBlock();
	const pub_key_obj = tx.object(publicKeyObjID);
	const dwallet = tx.object(dwalletID);

	tx.moveCall({
		target: `${packageId}::${dWalletTransferModuleName}::encrypt_user_share`,
		arguments: [
			dwallet,
			pub_key_obj,
			tx.pure(proof),
			parseArg(range_commitment, tx),
			parseArg(encrypted_secret_share, tx),
			tx.pure(recipient_address),
		],
	});

	return await client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: tx,
		options: {
			showEffects: true,
		},
	});
};

const parseArg = (arg, tx) => tx.pure(bcs.vector(bcs.u8()).serialize(arg));
