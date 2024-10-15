// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import {
	serialized_pubkeys_from_decentralized_dkg_output,
	verify_signatures,
} from '@dwallet-network/signature-mpc-wasm';

import { bcs } from '../bcs/index.js';
import { TransactionBlock } from '../builder/index.js';
import type { DWalletClient } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';
import type { SuiObjectRef } from '../types/index.js';
import type { DWallet } from './dwallet_2pc_mpc_ecdsa_k1_module.js';
import type { DWalletToTransfer, EncryptedUserShare } from './encrypt_user_share.js';
import { fetchOwnedObjectByType } from './utils.js';

const packageId = '0x3';
const dWalletModuleName = 'dwallet';
const dWallet2PCMPCECDSAK1ModuleName = 'dwallet_2pc_mpc_ecdsa_k1';

export enum EncryptionKeyScheme {
	Paillier = 0,
}

export const getDwalletByObjID = async (
	client: DWalletClient,
	dwalletObjID: string,
): Promise<DWallet | null> => {
	const dwalletObject = await client.getObject({
		id: dwalletObjID,
		options: { showContent: true },
	});

	const dwalletObjectFields =
		dwalletObject.data?.content?.dataType === 'moveObject'
			? (dwalletObject.data?.content?.fields as {
					dwallet_cap_id: string;
					output: number[];
			  })
			: null;

	return dwalletObjectFields
		? {
				dwalletID: dwalletObjID,
				decentralizedDKGOutput: dwalletObjectFields.output,
				dwalletCapID: dwalletObjectFields.dwallet_cap_id,
		  }
		: null;
};

export function hashToNumber(hash: 'KECCAK256' | 'SHA256') {
	if (hash === 'KECCAK256') {
		return 0;
	} else {
		return 1;
	}
}

export async function approveAndSign(
	dwalletCapId: string,
	signMessagesId: string,
	messages: Uint8Array[],
	dwalletID: string,
	hash: 'KECCAK256' | 'SHA256',
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
			`${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::CreatedSignDataEvent`,
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
	let signatures = await waitForSignOutput(client);
	let encryptedUserShareObjId = await getEncryptedUserShare(client, keypair, dwalletID);
	let encryptedUserShareObj = await getEncryptedUserShareByObjectID(
		client,
		encryptedUserShareObjId!,
	);
	let dwallet = await getDwalletByObjID(client, dwalletID);
	let serializedPubkeys = serialized_pubkeys_from_decentralized_dkg_output(
		new Uint8Array(dwallet?.decentralizedDKGOutput!),
	);
	if (
		!(await keypair
			.getPublicKey()
			.verify(serializedPubkeys, new Uint8Array(encryptedUserShareObj?.signedDWalletPubKeys!)))
	) {
		throw new Error('The DWallet public keys has not been signed by the desired Sui address');
	}
	if (
		!verify_signatures(
			bcs.vector(bcs.vector(bcs.u8())).serialize(messages).toBytes(),
			hashToNumber(hash),
			new Uint8Array(dwallet?.decentralizedDKGOutput!),
			bcs.vector(bcs.vector(bcs.u8())).serialize(signatures).toBytes(),
		)
	) {
		throw new Error('Returned signatures are not valid');
	}
	return signatures;
}

export interface SignOutputEventData {
	signatures: Uint8Array[];
	dwallet_id: string;
}

export const waitForSignOutput = async (client: DWalletClient): Promise<Uint8Array[]> => {
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

/**
 * Store the given Paillier encryption key in the blockchain.
 */
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

/**
 * Fetches an {@link EncryptedUserShare} object from the blockchain by the given object ID.
 */
export const getEncryptedUserShareByObjectID = async (
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
				dwalletID: objectFields?.dwallet_id,
				encryptedUserShareAndProof: objectFields?.encrypted_secret_share_and_proof,
				encryptionKeyObjID: objectFields?.encryption_key_id,
				signedDWalletPubKeys: objectFields.signed_dwallet_pubkeys,
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

	const array = new Uint8Array(res.results?.at(0)?.returnValues?.at(0)?.at(0)! as number[]);
	const hexString = Array.from(array)
		.map((byte) => byte.toString(16).padStart(2, '0'))
		.join('');
	return hexString;
};

/**
 * Sets the given encryption key as the active encryption key for the given keypair Sui
 * address & encryption keys holder table.
 */
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

/**
 * Creates the table that maps a Sui address to the Paillier encryption
 * key is derived from the Sui address secret.
 */
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
	signedDWalletPubKeys: Uint8Array,
) => {
	const tx = new TransactionBlock();
	const encryptionKey = tx.object(encryptionKeyObjID);
	const dwalletObj = tx.object(dwallet.dwalletID);
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

	const res = await client.signAndExecuteTransactionBlock({
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
};

export const getEncryptedUserSharesObjID = async (client: DWalletClient, keypair: Keypair) => {
	const table = await fetchOwnedObjectByType(
		`${packageId}::${dWalletModuleName}::EncryptedUserShares`,
		keypair,
		client,
	);
	const tableFields =
		table?.dataType === 'moveObject'
			? (table.fields as {
					id: { id: string };
			  })
			: null;

	if (table === null) {
		const newTable = await createEncryptedUserSharesHolder(client, keypair);
		return newTable?.objectId;
	}
	return tableFields?.id.id;
};

export const saveEncryptedUserShare = async (
	client: DWalletClient,
	keypair: Keypair,
	encryptionKeyID: string,
	encryptedUserShareId: string,
) => {
	const tx = new TransactionBlock();
	const encKey = tx.object(encryptionKeyID);
	const encryptedUserShare = tx.object(encryptedUserShareId);
	const encryptedUserSharesId = await getEncryptedUserSharesObjID(client, keypair);

	tx.moveCall({
		target: `${packageId}::${dWalletModuleName}::save_encrypted_user_share`,
		arguments: [tx.object(encryptedUserSharesId!), encryptedUserShare, encKey],
	});

	return await client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: tx,
		options: {
			showEffects: true,
		},
	});
};

/**
 * Retrieve the encrypted user share that the user encrypted to itself from the dwallet ID.
 * First fetches the user's `EncryptedUserShares` table, and from it fetches the encrypted user share.
 */
export const getEncryptedUserShare = async (
	client: DWalletClient,
	keypair: Keypair,
	dwalletID: string,
): Promise<string> => {
	let encryptedUserSharesObjID = await getEncryptedUserSharesObjID(client, keypair);
	const tx = new TransactionBlock();
	const encryptedUserSharesObj = tx.object(encryptedUserSharesObjID!);

	tx.moveCall({
		target: `${packageId}::${dWalletModuleName}::get_encrypted_user_share_by_dwallet_id`,
		arguments: [encryptedUserSharesObj, tx.pure(dwalletID)],
	});

	let res = await client.devInspectTransactionBlock({
		sender: keypair.toSuiAddress(),
		transactionBlock: tx,
	});
	const array = new Uint8Array(res.results?.at(0)?.returnValues?.at(0)?.at(0)! as number[]);
	return Array.from(array)
		.map((byte) => byte.toString(16).padStart(2, '0'))
		.join('');
};
