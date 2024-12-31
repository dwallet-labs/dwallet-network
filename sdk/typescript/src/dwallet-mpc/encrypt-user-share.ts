import { generate_secp_cg_keypair_from_seed } from '@dwallet-network/dwallet-mpc-wasm';

import { bcs } from '../bcs/index.js';
import type { PeraClient, PeraObjectRef } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';
import { decodePeraPrivateKey } from '../cryptography/index.js';
import type { Ed25519Keypair } from '../keypairs/ed25519/index.js';
import { Transaction } from '../transactions/index.js';
import { dWalletModuleName, packageId } from './globals.js';

type EncryptionKeyPair = {
	encryptionKey: Uint8Array;
	decryptionKey: Uint8Array;
	objectID: string;
};

export enum EncryptionKeyScheme {
	ClassGroups = 0,
}

/**
 * Creates the table that maps a Sui address to the Paillier encryption
 * key is derived from the Sui address secret.
 */
export const createActiveEncryptionKeysTable = async (client: PeraClient, keypair: Keypair) => {
	const tx = new Transaction();
	tx.moveCall({
		target: `${packageId}::${dWalletModuleName}::create_active_encryption_keys`,
		arguments: [],
	});

	let result = await client.signAndExecuteTransaction({
		signer: keypair,
		transaction: tx,
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

export const getActiveEncryptionKeyObjID = async (
	client: PeraClient,
	keyOwnerAddress: string,
	encryptionKeysHolderID: string,
): Promise<string> => {
	const tx = new Transaction();
	const encryptionKeysHolder = tx.object(encryptionKeysHolderID);

	console.log(keyOwnerAddress);

	tx.moveCall({
		target: `${packageId}::${dWalletModuleName}::get_active_encryption_key`,
		arguments: [encryptionKeysHolder, tx.pure.address(keyOwnerAddress)],
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

export const getOrCreateEncryptionKey = async (
	keypair: Ed25519Keypair,
	client: PeraClient,
	activeEncryptionKeysTableID: string,
): Promise<EncryptionKeyPair> => {
	let [encryptionKey, decryptionKey] = generatePaillierKeyPairFromSuiKeyPair(keypair);
	const activeEncryptionKeyObjID = await getActiveEncryptionKeyObjID(
		client,
		keypair.toPeraAddress(),
		activeEncryptionKeysTableID,
	);
	if (activeEncryptionKeyObjID) {
		let encryptionKeyObj = await getEncryptionKeyByObjectId(client, activeEncryptionKeyObjID);
		if (isEqual(encryptionKeyObj?.encryptionKey!, encryptionKey)) {
			return {
				encryptionKey,
				decryptionKey,
				objectID: activeEncryptionKeyObjID,
			};
		}
		throw new Error(
			'Encryption key derived from Sui secret does not match the one in the active encryption keys table',
		);
	}

	const encryptionKeyRef = await storeEncryptionKey(
		encryptionKey,
		EncryptionKeyScheme.ClassGroups,
		keypair,
		client,
	);

	// Sleep for 5 seconds so the storeEncryptionKey transaction effects has time to
	// get written to the blockchain.
	await new Promise((r) => setTimeout(r, 5000));
	await setActiveEncryptionKey(
		client,
		keypair,
		encryptionKeyRef?.objectId!,
		activeEncryptionKeysTableID,
	);
	return {
		decryptionKey,
		encryptionKey,
		objectID: encryptionKeyRef.objectId,
	};
};

/**
 * Sets the given encryption key as the active encryption key for the given keypair Sui
 * address & encryption keys holder table.
 */
const setActiveEncryptionKey = async (
	client: PeraClient,
	keypair: Keypair,
	encryptionKeyObjID: string,
	encryptionKeysHolderID: string,
) => {
	const tx = new Transaction();
	const EncKeyObj = tx.object(encryptionKeyObjID);
	const encryptionKeysHolder = tx.object(encryptionKeysHolderID);

	tx.moveCall({
		target: `${packageId}::${dWalletModuleName}::set_active_encryption_key`,
		arguments: [encryptionKeysHolder, EncKeyObj],
	});

	return await client.signAndExecuteTransaction({
		signer: keypair,
		transaction: tx,
		options: {
			showEffects: true,
		},
	});
};

/**
 * Store the given Paillier encryption key in the blockchain.
 */
const storeEncryptionKey = async (
	encryptionKey: Uint8Array,
	encryptionKeyScheme: EncryptionKeyScheme,
	keypair: Keypair,
	client: PeraClient,
): Promise<PeraObjectRef> => {
	let signedEncryptionKey = await keypair.sign(new Uint8Array(encryptionKey));
	const tx = new Transaction();
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
	let result = await client.signAndExecuteTransaction({
		signer: keypair,
		transaction: tx,
		options: {
			showEffects: true,
		},
	});
	return result.effects?.created?.filter((o) => o.owner === 'Immutable')[0].reference!;
};

function isEqual(arr1: Uint8Array, arr2: Uint8Array): boolean {
	if (arr1.length !== arr2.length) {
		return false;
	}

	return arr1.every((value, index) => value === arr2[index]);
}

const getEncryptionKeyByObjectId = async (client: PeraClient, encryptionKeyObjID: string) => {
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

const generatePaillierKeyPairFromSuiKeyPair = (keypair: Ed25519Keypair): Uint8Array[] => {
	let secretKey = keypair.getSecretKey();
	let decoded = decodePeraPrivateKey(secretKey);
	return generate_secp_cg_keypair_from_seed(decoded.secretKey);
};
