import { generate_secp_cg_keypair_from_seed } from '@dwallet-network/dwallet-mpc-wasm';
import { bcs, toHex } from '@mysten/bcs';
import { Transaction } from '@mysten/sui/transactions';

import type { Config } from './globals.js';
import {
	DWALLET_ECDSA_K1_INNER_MOVE_MODULE_NAME,
	DWALLET_ECDSA_K1_MOVE_MODULE_NAME,
	fetchObjectWithType,
	getDWalletSecpState,
} from './globals.js';

/**
 * A class groups key pair.
 */
export interface ClassGroupsSecpKeyPair {
	encryptionKey: Uint8Array;
	decryptionKey: Uint8Array;
	objectID: string;
}

/**
 * Event emitted by the blockchain when an
 * `EncryptionKey` Move object is created.
 */
interface CreatedEncryptionKeyEvent {
	session_id: string;
	encryption_key_id: string;
}

/**
 * A class groups Move encryption key object.
 */
interface EncryptionKey {
	encryption_key: Uint8Array;
	signer_address: string;
	encryption_key_signature: Uint8Array;
}

interface CreatedEncryptionKeyEvent {
	encryption_key_id: string;
	signer_address: string;
}

function isEncryptionKey(obj: any): obj is EncryptionKey {
	return 'encryption_key' in obj && 'signer_address' in obj && 'encryption_key_signature' in obj;
}

export async function getOrCreateClassGroupsKeyPair(conf: Config): Promise<ClassGroupsSecpKeyPair> {
	const [expectedEncryptionKey, decryptionKey] = generate_secp_cg_keypair_from_seed(
		conf.dWalletSeed,
	);
	const activeEncryptionKeyObjID = await getActiveEncryptionKeyObjID(
		conf,
		conf.encryptedSecretShareSigningKeypair.toSuiAddress(),
	);
	const encryptionKeyMoveType = `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSA_K1_INNER_MOVE_MODULE_NAME}::EncryptionKey`;

	if (activeEncryptionKeyObjID) {
		const activeEncryptionKeyObj = await fetchObjectWithType<EncryptionKey>(
			conf,
			encryptionKeyMoveType,
			isEncryptionKey,
			activeEncryptionKeyObjID,
		);
		if (isEqual(activeEncryptionKeyObj?.encryption_key, expectedEncryptionKey)) {
			return {
				encryptionKey: expectedEncryptionKey,
				decryptionKey,
				objectID: activeEncryptionKeyObjID,
			};
		}
		throw new Error(
			'encryption key derived from the key pair does not match the one in the active encryption keys table',
		);
	}

	const encryptionKeyCreationEvent: CreatedEncryptionKeyEvent = await registerEncryptionKey(
		conf,
		expectedEncryptionKey,
	);

	return {
		decryptionKey,
		encryptionKey: expectedEncryptionKey,
		objectID: encryptionKeyCreationEvent.encryption_key_id,
	};
}

function isEqual(arr1: Uint8Array, arr2: Uint8Array): boolean {
	return arr1.length === arr2.length && arr1.every((value, index) => value === arr2[index]);
}

/**
 * Retrieves the active encryption key object ID
 * from the active encryption keys table *activeEncryptionKeysTableID*
 * for the given address — derived from the public key.
 * Throws an error otherwise.
 */
async function getActiveEncryptionKeyObjID(conf: Config, address: string): Promise<string> {
	const tx = new Transaction();
	const dwalletState = await getDWalletSecpState(conf);
	tx.moveCall({
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSA_K1_MOVE_MODULE_NAME}::get_active_encryption_key`,
		arguments: [
			tx.sharedObjectRef({
				objectId: dwalletState.object_id,
				initialSharedVersion: dwalletState.initial_shared_version,
				mutable: false,
			}),
			tx.pure.address(address),
		],
	});

	// Safe to use this function as it has been used here:
	// https://github.com/dwallet-labs/dwallet-network/blob/29929ded135f05578b6ce33b52e6ff5e894d0487/sdk/deepbook-v3/src/client.ts#L84
	// in late 2024 (can be seen with git blame).
	// Note that regular `getObject()` is not working because of dynamic fields.
	const res = await conf.client.devInspectTransactionBlock({
		sender: address,
		transactionBlock: tx,
	});

	const objIDArray = new Uint8Array(res.results?.at(0)?.returnValues?.at(0)?.at(0) as number[]);
	return toHex(objIDArray);
}

/**
 * Registers (stores) the given encryption key in the blockchain.
 *
 * This function facilitates the storage of an encryption key as an immutable object
 * on the blockchain.
 * The key is signed with the provided key pair to ensure
 * cryptographic integrity, and validate it by the blockchain.
 * Currently, only Class Groups encryption keys are supported.
 *
 * ### Parameters
 * — `keyPair`: A `Keypair` object used to sign the encryption key.
 * — `encryptionKey`: The encryption key to be registered.
 * — `encryptionKeyScheme`: The scheme of the encryption key (e.g., Class Groups).
 */
async function registerEncryptionKey(
	conf: Config,
	encryptionKey: Uint8Array,
): Promise<CreatedEncryptionKeyEvent> {
	// Sign the encryption key with the key pair.
	const encryptionKeySignature = await conf.encryptedSecretShareSigningKeypair.sign(
		new Uint8Array(encryptionKey),
	);
	const tx = new Transaction();

	const dwalletState = await getDWalletSecpState(conf);
	tx.moveCall({
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSA_K1_MOVE_MODULE_NAME}::register_encryption_key`,
		arguments: [
			tx.sharedObjectRef({
				objectId: dwalletState.object_id,
				initialSharedVersion: dwalletState.initial_shared_version,
				mutable: true,
			}),
			tx.pure(bcs.vector(bcs.u8()).serialize(encryptionKey)),
			tx.pure(bcs.vector(bcs.u8()).serialize(encryptionKeySignature)),
			tx.pure(
				bcs
					.vector(bcs.u8())
					.serialize(conf.encryptedSecretShareSigningKeypair.getPublicKey().toRawBytes()),
			),
		],
	});
	const res = await conf.client.signAndExecuteTransaction({
		signer: conf.suiClientKeypair,
		transaction: tx,
		options: {
			showEvents: true,
		},
	});
	const createdEncryptionKeyEvent = res.events?.find((event) =>
		isCreatedEncryptionKeyEvent(event.parsedJson),
	);
	if (!createdEncryptionKeyEvent) {
		throw new Error('Encryption key registration failed');
	}
	return createdEncryptionKeyEvent.parsedJson as CreatedEncryptionKeyEvent;
}

function isCreatedEncryptionKeyEvent(obj: any): obj is CreatedEncryptionKeyEvent {
	return 'encryption_key_id' in obj && 'signer_address' in obj;
}
