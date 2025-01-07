// noinspection ES6PreferShortImport

import { generate_secp_cg_keypair_from_seed } from '@dwallet-network/dwallet-mpc-wasm';
import { toHEX } from '@mysten/bcs';

import { bcs } from '../bcs/index.js';
import type { PeraClient, PeraObjectRef } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';
import { decodePeraPrivateKey } from '../cryptography/index.js';
import type { Ed25519Keypair } from '../keypairs/ed25519/index.js';
import { Transaction } from '../transactions/index.js';
import type { Config } from './globals.js';
import { dWalletModuleName, fetchObjectWithType, packageId } from './globals.js';

/**
 * A class groups key pair.
 */
interface ClassGroupsSecpKeyPair {
	encryptionKey: Uint8Array;
	decryptionKey: Uint8Array;
	objectID: string;
}

/**
 * A class groups Move encryption key object.
 */
interface EncryptionKey {
	encryptionKey: Uint8Array;
	key_owner_address: string;
	encryption_key_signature: Uint8Array;
}

export enum EncryptionKeyScheme {
	ClassGroups = 0,
}

const encryptionKeyMoveType = `${packageId}::${dWalletModuleName}::EncryptionKey`;

/**
 * Creates a table that maps users` Sui addresses to Class Group encryption keys.
 */
export async function createActiveEncryptionKeysTable(client: PeraClient, keypair: Keypair) {
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

	const activeEncryptionKeysObj = result.effects?.created?.filter(
		(o) =>
			typeof o.owner === 'object' &&
			'Shared' in o.owner &&
			o.owner.Shared.initial_shared_version !== undefined,
	)[0].reference;
	if (!activeEncryptionKeysObj) {
		throw new Error('Failed to create the active encryption keys table');
	}

	return activeEncryptionKeysObj;
}

/**
 * Retrieves the active encryption key object ID for the given Sui address, if it exists. Throws an error otherwise.
 */
export const getActiveEncryptionKeyObjID = async (
	c: Config,
	encryptionKeysHolderID: string,
): Promise<string> => {
	let keyOwnerAddress = c.keypair.toPeraAddress();
	let client = c.client;
	const tx = new Transaction();
	const encryptionKeysHolder = tx.object(encryptionKeysHolderID);

	tx.moveCall({
		target: `${packageId}::${dWalletModuleName}::get_active_encryption_key`,
		arguments: [encryptionKeysHolder, tx.pure.address(keyOwnerAddress)],
	});
	// Safe to use this function as it is has been used here: https://github.com/dwallet-labs/dwallet-network/blob/29929ded135f05578b6ce33b52e6ff5e894d0487/sdk/deepbook-v3/src/client.ts#L84
	// in late 2024 (can be seen with git blame).
	let res = await client.devInspectTransactionBlock({
		sender: keyOwnerAddress,
		transactionBlock: tx,
	});

	const objIDArray = new Uint8Array(res.results?.at(0)?.returnValues?.at(0)?.at(0)! as number[]);
	return toHEX(objIDArray);
};

const isEncryptionKey = (obj: any): obj is EncryptionKey => {
	return 'encryptionKey' in obj && 'key_owner_address' in obj && 'encryption_key_signature' in obj;
};

export const getOrCreateEncryptionKey = async (
	c: Config,
	activeEncryptionKeysTableID: string,
): Promise<ClassGroupsSecpKeyPair> => {
	let [encryptionKey, decryptionKey] = generateClassGroupKeyPairFromSuiKeyPair(
		c.keypair as Ed25519Keypair,
	);
	const activeEncryptionKeyObjID = await getActiveEncryptionKeyObjID(
		c,
		activeEncryptionKeysTableID,
	);
	if (activeEncryptionKeyObjID) {
		let encryptionKeyObj = await fetchObjectWithType<EncryptionKey>(
			c,
			encryptionKeyMoveType,
			isEncryptionKey,
			activeEncryptionKeyObjID,
		);
		if (isEqual(encryptionKeyObj?.encryptionKey, encryptionKey)) {
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
		c,
	);

	// Sleep for 5 seconds, so the storeEncryptionKey transaction effects have time to
	// get written to the blockchain.
	await new Promise((r) => setTimeout(r, 5000));
	await upsertActiveEncryptionKey(encryptionKeyRef?.objectId, activeEncryptionKeysTableID, c);
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
const upsertActiveEncryptionKey = async (
	encryptionKeyObjID: string,
	encryptionKeysHolderID: string,
	c: Config,
) => {
	const tx = new Transaction();
	const EncKeyObj = tx.object(encryptionKeyObjID);
	const encryptionKeysHolder = tx.object(encryptionKeysHolderID);

	tx.moveCall({
		target: `${packageId}::${dWalletModuleName}::upsert_active_encryption_key`,
		arguments: [encryptionKeysHolder, EncKeyObj],
	});

	return await c.client.signAndExecuteTransaction({
		signer: c.keypair,
		transaction: tx,
		options: {
			showEffects: true,
		},
	});
};

/**
 * Store the given Class Groups encryption key in the blockchain.
 */
const storeEncryptionKey = async (
	encryptionKey: Uint8Array,
	encryptionKeyScheme: EncryptionKeyScheme,
	c: Config,
): Promise<PeraObjectRef> => {
	let signedEncryptionKey = await c.keypair.sign(new Uint8Array(encryptionKey));
	const tx = new Transaction();
	let purePubKey = tx.pure(bcs.vector(bcs.u8()).serialize(encryptionKey));
	let pureSignedPubKey = tx.pure(bcs.vector(bcs.u8()).serialize(signedEncryptionKey));
	let pureSuiPubKey = tx.pure(
		bcs.vector(bcs.u8()).serialize(c.keypair.getPublicKey().toRawBytes()),
	);

	tx.moveCall({
		target: `${packageId}::${dWalletModuleName}::register_encryption_key`,
		arguments: [
			purePubKey,
			pureSignedPubKey,
			pureSuiPubKey,
			tx.pure(bcs.u8().serialize(encryptionKeyScheme)),
		],
	});
	let result = await c.client.signAndExecuteTransaction({
		signer: c.keypair,
		transaction: tx,
		options: {
			showEffects: true,
		},
	});
	const encKeyRef = result.effects?.created?.filter((o) => o.owner === 'Immutable')[0].reference;
	if (!encKeyRef) {
		throw new Error('Failed to store the encryption key');
	}
	return encKeyRef;
};

function isEqual(arr1: Uint8Array, arr2: Uint8Array): boolean {
	if (arr1.length !== arr2.length) {
		return false;
	}

	return arr1.every((value, index) => value === arr2[index]);
}

const generateClassGroupKeyPairFromSuiKeyPair = (keypair: Ed25519Keypair): Uint8Array[] => {
	let secretKey = keypair.getSecretKey();
	let decoded = decodePeraPrivateKey(secretKey);
	return generate_secp_cg_keypair_from_seed(decoded.secretKey);
};
