import {
	encrypt_secret_share,
	generate_secp_cg_keypair_from_seed,
} from '@dwallet-network/dwallet-mpc-wasm';

import { bcs } from '../bcs/index.js';
import type { PeraClient, PeraObjectRef } from '../client/index.js';
import type { Keypair, PublicKey } from '../cryptography/index.js';
import { decodePeraPrivateKey } from '../cryptography/index.js';
import type { Ed25519Keypair } from '../keypairs/ed25519/index.js';
import { Transaction } from '../transactions/index.js';
import type { CreatedDwallet } from './dkg.js';
import type { Config } from './globals.js';
import {
	dWallet2PCMPCECDSAK1ModuleName,
	dWalletModuleName,
	fetchObjectWithType,
	packageId,
} from './globals.js';

export const encryptedSecretShareMoveType = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::EncryptedUserShare`;

/**
 * A class groups key pair.
 */
interface CGSecpKeyPair {
	encryptionKey: Uint8Array;
	decryptionKey: Uint8Array;
	objectID: string;
}

/**
 * A class groups Move encryption key object.
 */

interface EncryptionKey {
	encryption_key: Uint8Array;
	key_owner_address: string;
	encryption_key_signature: Uint8Array;
}

/**
 * The Move encrypted user share object.
 */
export interface EncryptedUserShare {
	id: string;
	dwallet_id: string;
	encrypted_secret_share_and_proof: Uint8Array;
	encryption_key_id: string;
}

export enum EncryptionKeyScheme {
	ClassGroups = 0,
}

export const isEncryptedUserShare = (obj: any): obj is EncryptedUserShare => {
	return (
		'id' in obj &&
		'dwallet_id' in obj &&
		'encrypted_secret_share_and_proof' in obj &&
		'encryption_key_id' in obj
	);
};

/**
 * Encrypts and sends the given secret user share to the given destination public key.
 *
 * @param c The DWallet client.
 * @param dwallet The dWallet that we want to send the secret user share of.
 * @param destinationPublicKey The ed2551 public key of the destination Sui address.
 * @param activeEncryptionKeysTableID The ID of the table that holds the active encryption keys.
 */
export const sendUserShareToSuiPubKey = async (
	c: Config,
	dwallet: CreatedDwallet,
	destinationPublicKey: PublicKey,
	activeEncryptionKeysTableID: string,
): Promise<string> => {
	const activeEncryptionKeyObjID = await getActiveEncryptionKeyObjID(
		c,
		destinationPublicKey.toPeraAddress(),
		activeEncryptionKeysTableID,
	);
	if (!activeEncryptionKeyObjID) {
		throw new Error('The destination public key does not have an active encryption key');
	}
	const recipientData = await fetchObjectWithType<EncryptionKey>(
		c,
		encryptionKeyMoveType,
		isEncryptionKey,
		activeEncryptionKeyObjID,
	);
	let isValidEncryptionKey = await destinationPublicKey.verify(
		new Uint8Array(recipientData.encryption_key),
		new Uint8Array(recipientData.encryption_key_signature),
	);
	if (!isValidEncryptionKey) {
		throw new Error(
			'The destination public key has not been signed by the desired destination Sui address',
		);
	}
	let encryptedUserShareAndProof = encrypt_secret_share(
		new Uint8Array(dwallet.centralizedDKGPrivateOutput),
		new Uint8Array(recipientData.encryption_key),
	);

	return await transferEncryptedUserShare(
		c,
		encryptedUserShareAndProof,
		activeEncryptionKeyObjID,
		dwallet,
	);
};

/**
 * Creates the table that maps a Sui address to the Class Groups encryption
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

/**
 * Retrieves the active encryption key object ID for the given Sui address, if it exists. Throws an error otherwise.
 */
export const getActiveEncryptionKeyObjID = async (
	c: Config,
	keyOwnerAddress: string,
	encryptionKeysHolderID: string,
): Promise<string> => {
	let client = c.client;
	const tx = new Transaction();
	const encryptionKeysHolder = tx.object(encryptionKeysHolderID);

	tx.moveCall({
		target: `${packageId}::${dWalletModuleName}::get_active_encryption_key`,
		arguments: [encryptionKeysHolder, tx.pure.address(keyOwnerAddress)],
	});

	let res = await client.devInspectTransactionBlock({
		sender: keyOwnerAddress,
		transactionBlock: tx,
	});

	const objIDArray = new Uint8Array(res.results?.at(0)?.returnValues?.at(0)?.at(0)! as number[]);
	return Array.from(objIDArray)
		.map((byte) => byte.toString(16).padStart(2, '0'))
		.join('');
};

export const getOrCreateEncryptionKey = async (
	c: Config,
	activeEncryptionKeysTableID: string,
): Promise<CGSecpKeyPair> => {
	let [encryptionKey, decryptionKey] = generateCGKeyPairFromSuiKeyPair(c.keypair as Ed25519Keypair);
	const activeEncryptionKeyObjID = await getActiveEncryptionKeyObjID(
		c,
		c.keypair.toPeraAddress(),
		activeEncryptionKeysTableID,
	);
	if (activeEncryptionKeyObjID) {
		let encryptionKeyObj = await fetchObjectWithType<EncryptionKey>(
			c,
			encryptionKeyMoveType,
			isEncryptionKey,
			activeEncryptionKeyObjID,
		);
		if (isEqual(encryptionKeyObj?.encryption_key!, encryptionKey)) {
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

	// Sleep for 5 seconds so the storeEncryptionKey transaction effects has time to
	// get written to the blockchain.
	await new Promise((r) => setTimeout(r, 5000));
	await setActiveEncryptionKey(encryptionKeyRef?.objectId!, activeEncryptionKeysTableID, c);
	return {
		decryptionKey,
		encryptionKey,
		objectID: encryptionKeyRef.objectId,
	};
};

export const generateCGKeyPairFromSuiKeyPair = (keypair: Ed25519Keypair): Uint8Array[] => {
	let secretKey = keypair.getSecretKey();
	let decoded = decodePeraPrivateKey(secretKey);
	return generate_secp_cg_keypair_from_seed(decoded.secretKey);
};

const transferEncryptedUserShare = async (
	conf: Config,
	encryptedUserShareAndProof: Uint8Array,
	encryptionKeyObjID: string,
	dwallet: CreatedDwallet,
): Promise<string> => {
	const tx = new Transaction();
	const encryptionKey = tx.object(encryptionKeyObjID);
	const dwalletObj = tx.object(dwallet.id);

	tx.moveCall({
		target: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::encrypt_user_share`,
		typeArguments: [],
		arguments: [
			dwalletObj,
			encryptionKey,
			tx.pure(bcs.vector(bcs.u8()).serialize(encryptedUserShareAndProof)),
		],
	});

	const res = await conf.client.signAndExecuteTransaction({
		signer: conf.keypair,
		transaction: tx,
		options: {
			showEffects: true,
		},
	});

	return res.effects?.created?.at(0)?.reference.objectId!;
};
const isEncryptionKey = (obj: any): obj is EncryptionKey => {
	return 'encryption_key' in obj && 'key_owner_address' in obj && 'encryption_key_signature' in obj;
};

let encryptionKeyMoveType = `${packageId}::${dWalletModuleName}::EncryptionKey`;

/**
 * Sets the given encryption key as the active encryption key for the given keypair Sui
 * address & encryption keys holder table.
 */
const setActiveEncryptionKey = async (
	encryptionKeyObjID: string,
	encryptionKeysHolderID: string,
	c: Config,
) => {
	const tx = new Transaction();
	const EncKeyObj = tx.object(encryptionKeyObjID);
	const encryptionKeysHolder = tx.object(encryptionKeysHolderID);

	tx.moveCall({
		target: `${packageId}::${dWalletModuleName}::set_active_encryption_key`,
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
	return result.effects?.created?.filter((o) => o.owner === 'Immutable')[0].reference!;
};

function isEqual(arr1: Uint8Array, arr2: Uint8Array): boolean {
	if (arr1.length !== arr2.length) {
		return false;
	}

	return arr1.every((value, index) => value === arr2[index]);
}
