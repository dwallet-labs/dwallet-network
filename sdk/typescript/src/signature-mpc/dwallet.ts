// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { serialized_pubkeys_from_decentralized_dkg_output } from '@dwallet-network/signature-mpc-wasm';

import { bcs } from '../bcs/index.js';
import { TransactionBlock } from '../builder/index.js';
import type { DWalletClient } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';
import type { SuiObjectRef } from '../types/index.js';
import { Dwallet } from './dwallet_2pc_mpc_ecdsa_k1_module';
import {DWalletToTransfer, EncryptedUserShare} from './encrypt_user_share';

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

	await client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: tx,
		options: {
			showEffects: true,
		},
	});
	return await waitForSignOutput(client);
}

export interface SignOutputEventData {
	signatures: Uint8Array[];
}

const waitForSignOutput = async (client: DWalletClient) => {
	return new Promise((resolve) => {
		client.subscribeEvent({
			filter: {
				MoveEventType: `${packageId}::${dWalletModuleName}::SignOutputEvent`,
			},
			onMessage: (event) => {
				let eventData = event?.parsedJson! as SignOutputEventData;
				resolve(eventData.signatures);
			},
		});
	});
};

export const storeEncryptionKey = async (
	encryptionKey: Uint8Array,
	encryptionKeyScheme: EncryptionKeyScheme,
	keypair: Keypair,
	client: DWalletClient,
): Promise<SuiObjectRef> => {
	let signedEncryptionKey = await keypair.sign(new Uint8Array(encryptionKey));
	const tx = new TransactionBlock();
	let purePubKey = tx.pure(bcs.vector(bcs.u8()).serialize(encryptionKey));
	let pureSignedPubKey = tx.pure(bcs.vector(bcs.u8()).serialize(signedEncryptionKey));
	let pureSuiPubKey = tx.pure(bcs.vector(bcs.u8()).serialize(keypair.getPublicKey().toRawBytes()));

	tx.moveCall({
		target: `${packageId}::${dWalletModuleName}::register_encryption_key`,
		arguments: [
			purePubKey,
			pureSignedPubKey,
			pureSuiPubKey,
			tx.pure(bcs.u8().serialize(encryptionKeyScheme)),
		],
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
					encryption_key_signature: Uint8Array;
			  })
			: null;

	return objectFields
		? {
				encryptionKey: objectFields?.encryption_key,
				signedEncryptionKey: objectFields?.encryption_key_signature,
				keyOwnerAddress: objectFields?.key_owner_address,
		  }
		: null;
};

export const getEncryptedUserShareByObjectId = async (
	client: DWalletClient,
	objID: string,
): Promise<EncryptedUserShare | null> => {
	const response = await client.getObject({
		id: objID,
		options: { showContent: true },
	});

	const objectFields =
		response.data?.content?.dataType === 'moveObject'
			? (response.data?.content?.fields as unknown as {
					dwallet_id: string;
					encrypted_secret_share_and_proof: number[];
					encryption_key_id: string;
					signed_dwallet_pubkeys: number[];
					sender_pubkey: number[];
			  })
			: null;

	return objectFields
		? {
				dwalletId: objectFields?.dwallet_id,
				encryptedUserShareAndProof: objectFields?.encrypted_secret_share_and_proof,
				encryptionKeyObjID: objectFields?.encryption_key_id,
				signedDWalletPubkeys: objectFields.signed_dwallet_pubkeys,
				senderPubKey: objectFields.sender_pubkey,
		  }
		: null;
};

export const getActiveEncryptionKeyObjID = async (
	client: DWalletClient,
	keyOwnerAddress: string,
	encryptionKeysHolderID: string,
): Promise<string> => {
	const tx = new TransactionBlock();
	const encryptionKeysHolder = tx.object(encryptionKeysHolderID);

	console.log(keyOwnerAddress);

	tx.moveCall({
		target: `${packageId}::${dWalletModuleName}::get_active_encryption_key`,
		arguments: [encryptionKeysHolder, tx.pure(keyOwnerAddress)],
	});

	let res = await client.devInspectTransactionBlock({
		sender: keyOwnerAddress,
		transactionBlock: tx,
	});

	return Buffer.from(
		new Uint8Array(res.results?.at(0)?.returnValues?.at(0)?.at(0)! as number[]),
	).toString('hex');
};

export const setActiveEncryptionKey = async (
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

export const createActiveEncryptionKeysTable = async (client: DWalletClient, keypair: Keypair) => {
	const tx = new TransactionBlock();
	tx.moveCall({
		target: `${packageId}::${dWalletModuleName}::create_active_encryption_keys`,
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

export const transferEncryptedUserShare = async (
	client: DWalletClient,
	keypair: Keypair,
	encryptedUserShareAndProof: number[],
	encryptionKeyObjID: string,
	dwallet: DWalletToTransfer,
) => {
	const tx = new TransactionBlock();
	const encryptionKey = tx.object(encryptionKeyObjID);
	const dwalletObj = tx.object(dwallet.dwalletId);
	let signedDWalletPubKeys = await keypair.sign(
		serialized_pubkeys_from_decentralized_dkg_output(
			new Uint8Array(dwallet.decentralizedDKGOutput!),
		),
	);
	let pureSuiPubKey = tx.pure(bcs.vector(bcs.u8()).serialize(keypair.getPublicKey().toRawBytes()));

	tx.moveCall({
		target: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::encrypt_user_share`,
		typeArguments: [],
		arguments: [
			dwalletObj,
			encryptionKey,
			tx.pure(encryptedUserShareAndProof),
			tx.pure([...signedDWalletPubKeys]),
			pureSuiPubKey,
		],
	});

	const res =  await client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: tx,
		options: {
			showEffects: true,
		},
	});

	return res.effects?.created?.at(0)?.reference;
};

export const createEncryptedUserSharesHolder = async (client: DWalletClient, keypair: Keypair) => {
	const tx = new TransactionBlock();
	tx.moveCall({
		target: `${packageId}::${dWalletModuleName}::create_encrypted_user_shares`,
		arguments: [],
	});

	let result = await client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: tx,
		options: {
			showEffects: true,
		},
	});

	return result.effects?.created?.at(0)?.reference;
}

export const saveEncryptedUserShare = async (
	client: DWalletClient,
	keypair: Keypair,
	encryptionKeyID: string,
	encryptedUserShareId: string,
	encrptedUserSharesID: string,
) => {
	const tx = new TransactionBlock();
	const encKey = tx.object(encryptionKeyID);
	const encryptedUserShare = tx.object(encryptedUserShareId);
	const encryptedUserShares = tx.object(encrptedUserSharesID);

	tx.moveCall({
		target: `${packageId}::${dWalletModuleName}::save_encrypted_user_share`,
		arguments: [encryptedUserShares, encryptedUserShare, encKey],
	});

	return await client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: tx,
		options: {
			showEffects: true,
		},
	});
};

export const getEncryptedUserShare = async (
	client: DWalletClient,
	keypair: Keypair,
	encrptedUserSharesObjID: string,
	dwalletID: string,
) => {
	const tx = new TransactionBlock();
	const encrptedUserSharesObj = tx.object(encrptedUserSharesObjID);

	tx.moveCall({
		target: `${packageId}::${dWalletModuleName}::get_encrypted_user_share_by_dwallet_id`,
		arguments: [encrptedUserSharesObj, tx.pure(dwalletID)],
	});

	let res = await client.devInspectTransactionBlock({
		sender: keypair.toSuiAddress(),
		transactionBlock: tx,
	});

	return res.results?.at(0)?.returnValues?.at(0)?.at(0);
};
