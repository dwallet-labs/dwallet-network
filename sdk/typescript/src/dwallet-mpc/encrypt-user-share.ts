// noinspection ES6PreferShortImport

import {
	decrypt_user_share,
	encrypt_secret_share,
	generate_secp_cg_keypair_from_seed,
	verify_user_share,
} from '@dwallet-network/dwallet-mpc-wasm';
import { toHEX } from '@mysten/bcs';

import { bcs } from '../bcs/index.js';
import type { PeraClient } from '../client/index.js';
import type { Keypair, PublicKey } from '../cryptography/index.js';
import { decodePeraPrivateKey } from '../cryptography/index.js';
import type { Ed25519Keypair } from '../keypairs/ed25519/index.js';
import { Ed25519PublicKey } from '../keypairs/ed25519/index.js';
import { Transaction } from '../transactions/index.js';
import type { Config, CreatedDwallet, DWallet } from './globals.js';
import {
	checkpointCreationTime,
	dWallet2PCMPCECDSAK1ModuleName,
	dWalletModuleName,
	dWalletMoveType,
	fetchCompletedEvent,
	fetchObjectWithType,
	isDWallet,
	packageId,
} from './globals.js';
import type { EncryptedUserShare } from './sign.js';
import {PERA_SYSTEM_STATE_OBJECT_ID} from "../utils";

const startEncryptedShareVerificationMoveType = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::StartEncryptedShareVerificationEvent`;
const createdEncryptedSecretShareEventMoveType = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::CreatedEncryptedSecretShareEvent`;
const startEncryptionKeyVerificationEventMoveType = `${packageId}::${dWalletModuleName}::StartEncryptionKeyVerificationEvent`;
const encryptionKeyMoveType = `${packageId}::${dWalletModuleName}::EncryptionKey`;

interface CreatedEncryptionKeyEvent {
	scheme: number;
	encryption_key: Uint8Array;
	key_owner_address: string;
	encryption_key_signature: Uint8Array;
	key_owner_pubkey: Uint8Array;
	session_id: string;
	encryption_key_id: string;
}

/**
 * A class groups key pair.
 */
interface ClassGroupsSecpKeyPair {
	encryptionKey: Uint8Array;
	decryptionKey: Uint8Array;
	objectID: string;
}

/**
 * TS representation of the Move CreatedEncryptedSecretShareEvent.
 */
interface CreatedEncryptedSecretShareEvent {
	encrypted_share_obj_id: string;
	dwallet_id: string;
	encrypted_secret_share_and_proof: Uint8Array;
	encryption_key_id: string;
	session_id: string;
	encryptor_address: string;
	encryptor_ed25519_pubkey: Uint8Array;
	signed_public_share: Uint8Array;
}

/**
 * TS representation of an event to start an MPC session.
 * Usually the only thing needed from this event is the `session_id`, which is used to fetch the
 * completion event.
 */
interface StartSessionEvent {
	session_id: string;
}

/**
 * A class groups Move encryption key object.
 */
interface EncryptionKey {
	encryption_key: Uint8Array;
	key_owner_address: string;
	encryption_key_signature: Uint8Array;
}

export enum EncryptionKeyScheme {
	ClassGroups = 0,
}

/**
 * Encrypts and sends the given secret user share to the given destination public key.
 *
 * @param c The DWallet client.
 * @param dwallet The dWallet that we want to send the secret user share of.
 * @param destinationPublicKey The ed2551 public key of the destination Sui address.
 * @param activeEncryptionKeysTableID The ID of the table that holds the active encryption keys.
 */
export async function encryptUserShareWithSuiPubKey(
	c: Config,
	dwallet: CreatedDwallet,
	destinationPublicKey: PublicKey,
	activeEncryptionKeysTableID: string,
): Promise<CreatedEncryptedSecretShareEvent> {
	const destinationEncryptionKeyObjID = await getActiveEncryptionKeyObjID(
		c,
		destinationPublicKey.toPeraAddress(),
		activeEncryptionKeysTableID,
	);
	if (!destinationEncryptionKeyObjID) {
		throw new Error('The destination public key does not have an active encryption key');
	}
	const recipientData = await fetchObjectWithType<EncryptionKey>(
		c,
		encryptionKeyMoveType,
		isEncryptionKey,
		destinationEncryptionKeyObjID,
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
		destinationEncryptionKeyObjID,
		dwallet,
	);
}

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

	const activeEncryptionKeysObj = result.effects?.created?.find(
		(o) =>
			typeof o.owner === 'object' &&
			'Shared' in o.owner &&
			o.owner.Shared.initial_shared_version !== undefined,
	)?.reference;
	if (!activeEncryptionKeysObj) {
		throw new Error('Failed to create the active encryption keys table');
	}

	await new Promise((r) => setTimeout(r, checkpointCreationTime));

	return activeEncryptionKeysObj;
}

/**
 * Retrieves the active encryption key object ID
 * for the given Sui address if it exists.
 * Throws an error otherwise.
 */
export async function getActiveEncryptionKeyObjID(
	c: Config,
	keyOwnerAddress: string,
	activeEncryptionKeysTableID: string,
): Promise<string> {
	const tx = new Transaction();

	tx.moveCall({
		target: `${packageId}::${dWalletModuleName}::get_active_encryption_key`,
		arguments: [tx.object(activeEncryptionKeysTableID), tx.pure.address(keyOwnerAddress)],
	});

	// Safe to use this function as it has been used here:
	// https://github.com/dwallet-labs/dwallet-network/blob/29929ded135f05578b6ce33b52e6ff5e894d0487/sdk/deepbook-v3/src/client.ts#L84
	// in late 2024 (can be seen with git blame).
	let res = await c.client.devInspectTransactionBlock({
		sender: keyOwnerAddress,
		transactionBlock: tx,
	});

	const objIDArray = new Uint8Array(res.results?.at(0)?.returnValues?.at(0)?.at(0) as number[]);
	return toHEX(objIDArray);
}

export async function getOrCreateEncryptionKey(
	c: Config,
	activeEncryptionKeysTableID: string,
): Promise<ClassGroupsSecpKeyPair> {
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
		if (isEqual(encryptionKeyObj?.encryption_key, encryptionKey)) {
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

	const encryptionKeyCreationEvent = await storeEncryptionKey(
		encryptionKey,
		EncryptionKeyScheme.ClassGroups,
		c,
	);

	// Sleep for 2 seconds, so the storeEncryptionKey transaction effects have time to
	// get written to the blockchain.
	await new Promise((r) => setTimeout(r, checkpointCreationTime));
	await upsertActiveEncryptionKey(
		encryptionKeyCreationEvent.encryption_key_id,
		activeEncryptionKeysTableID,
		c,
	);
	await new Promise((r) => setTimeout(r, checkpointCreationTime));
	return {
		decryptionKey,
		encryptionKey,
		objectID: encryptionKeyCreationEvent.encryption_key_id,
	};
}

export function generateCGKeyPairFromSuiKeyPair(keypair: Ed25519Keypair): Uint8Array[] {
	let secretKey = keypair.getSecretKey();
	let decoded = decodePeraPrivateKey(secretKey);
	return generate_secp_cg_keypair_from_seed(decoded.secretKey);
}

/**
 * Validates the provided `encryptedUserShare` and re-encrypts it for the caller's keypair.
 *
 * This process ensures that users can later retrieve all secret shares ever encrypted for them,
 * verify their validity, and confirm they are signed by the original source.
 */
export async function acceptUserShare(
	encryptedUserShare: CreatedEncryptedSecretShareEvent,
	expectedSourceSuiAddress: string,
	encryptionKeysHolderObjID: string,
	conf: Config,
) {
	let dwalletID = encryptedUserShare.dwallet_id;
	let dwallet = await fetchObjectWithType<DWallet>(conf, dWalletMoveType, isDWallet, dwalletID);

	// This function also verifies that the dkg output has been signed by the source public key.
	const decryptedKeyShare = await decryptAndVerifyUserShare(
		conf,
		encryptionKeysHolderObjID,
		encryptedUserShare,
		expectedSourceSuiAddress,
		dwallet,
	);

	let dwalletToSend: CreatedDwallet = {
		id: dwalletID,
		centralizedDKGPrivateOutput: [...decryptedKeyShare],
		decentralizedDKGOutput: dwallet.decentralized_output,
		dwalletCapID: dwallet.dwallet_cap_id,
		dwalletMPCNetworkKeyVersion: dwallet.dwallet_mpc_network_key_version,
		centralizedDKGPublicOutput: dwallet.centralized_output,
	};

	// Encrypt it to self, so that in the future we'd know that we already
	// verified everything and only need to verify our signature.
	// Need to verify the signature to not trust the blockchain to provide this data.
	await encryptUserShareWithSuiPubKey(
		conf,
		dwalletToSend,
		conf.keypair.getPublicKey(),
		encryptionKeysHolderObjID,
	);
}

const transferEncryptedUserShare = async (
	conf: Config,
	encryptedUserShareAndProof: Uint8Array,
	encryptionKeyObjID: string,
	dwallet: CreatedDwallet,
): Promise<CreatedEncryptedSecretShareEvent> => {
	const tx = new Transaction();
	const encryptionKey = tx.object(encryptionKeyObjID);
	const dwalletObj = tx.object(dwallet.id);
	let signedCentralizedOutput = await conf.keypair.sign(
		new Uint8Array(dwallet.centralizedDKGPublicOutput),
	);
	tx.moveCall({
		target: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::publish_encrypted_user_share`,
		typeArguments: [],
		arguments: [
			dwalletObj,
			encryptionKey,
			tx.pure(bcs.vector(bcs.u8()).serialize(encryptedUserShareAndProof)),
			tx.pure(bcs.vector(bcs.u8()).serialize(signedCentralizedOutput)),
			tx.pure(bcs.vector(bcs.u8()).serialize(conf.keypair.getPublicKey().toRawBytes())),
		],
	});

	const result = await conf.client.signAndExecuteTransaction({
		signer: conf.keypair,
		transaction: tx,
		options: {
			showEffects: true,
			showEvents: true,
		},
	});

	let sessionData = result.events?.find(
		(event) =>
			event.type === startEncryptedShareVerificationMoveType &&
			isStartSessionEvent(event.parsedJson),
	)?.parsedJson as StartSessionEvent;

	return await fetchCompletedEvent<CreatedEncryptedSecretShareEvent>(
		conf,
		sessionData.session_id,
		createdEncryptedSecretShareEventMoveType,
		isCreatedEncryptedSecretShareEvent,
	);
};

function isCreatedEncryptedSecretShareEvent(obj: any): obj is CreatedEncryptedSecretShareEvent {
	return (
		'encrypted_share_obj_id' in obj &&
		'dwallet_id' in obj &&
		'encrypted_secret_share_and_proof' in obj &&
		'encryption_key_id' in obj &&
		'session_id' in obj &&
		'encryptor_address' in obj &&
		'encryptor_ed25519_pubkey' in obj &&
		'signed_public_share' in obj
	);
}

function isStartSessionEvent(obj: any): obj is StartSessionEvent {
	return 'session_id' in obj;
}

const isEncryptionKey = (obj: any): obj is EncryptionKey => {
	return 'encryption_key' in obj && 'key_owner_address' in obj && 'encryption_key_signature' in obj;
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
): Promise<CreatedEncryptionKeyEvent> => {
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
			tx.sharedObjectRef({
				objectId: PERA_SYSTEM_STATE_OBJECT_ID,
				initialSharedVersion: 1,
				mutable: true,
			})
		],
	});
	let result = await c.client.signAndExecuteTransaction({
		signer: c.keypair,
		transaction: tx,
		options: {
			showEvents: true,
		},
	});

	let sessionID = (
		result.events?.find(
			(event) =>
				event.type === startEncryptionKeyVerificationEventMoveType &&
				isStartSessionEvent(event.parsedJson),
		)?.parsedJson as StartSessionEvent
	).session_id;
	return await fetchCompletedEvent<CreatedEncryptionKeyEvent>(
		c,
		sessionID,
		`${packageId}::${dWalletModuleName}::CreatedEncryptionKeyEvent`,
		isCreatedEncryptionKeyEvent,
	);
};

function isCreatedEncryptionKeyEvent(obj: any): obj is CreatedEncryptionKeyEvent {
	return (
		'scheme' in obj &&
		'encryption_key' in obj &&
		'key_owner_address' in obj &&
		'encryption_key_signature' in obj &&
		'key_owner_pubkey' in obj &&
		'session_id' in obj &&
		'encryption_key_id' in obj
	);
}

function isEqual(arr1: Uint8Array, arr2: Uint8Array): boolean {
	return arr1.length === arr2.length && arr1.every((value, index) => value === arr2[index]);
}

export async function decryptAndVerifyUserShare(
	conf: Config,
	activeEncryptionKeysTableID: string,
	encryptedUserShare: EncryptedUserShare,
	expectedSourceSuiAddress: string,
	encryptedDWallet: DWallet,
): Promise<Uint8Array> {
	let encryptorPubkey = new Ed25519PublicKey(encryptedUserShare.encryptor_ed25519_pubkey);
	let encryptor_address = encryptorPubkey.toPeraAddress();
	if (encryptor_address !== expectedSourceSuiAddress) {
		throw new Error('The source public key does not match the expected Sui address');
	}
	if (
		!(await encryptorPubkey.verify(
			new Uint8Array(encryptedDWallet.centralized_output),
			new Uint8Array(encryptedUserShare.signed_public_share),
		))
	) {
		throw new Error('the dWallet public key share has not been signed by the desired Sui address');
	}
	let destination_cg_keypair = await getOrCreateEncryptionKey(conf, activeEncryptionKeysTableID);
	let decrypted_share = decrypt_user_share(
		destination_cg_keypair.encryptionKey,
		destination_cg_keypair.decryptionKey,
		encryptedUserShare.encrypted_secret_share_and_proof,
	);
	let is_valid = verify_user_share(
		decrypted_share,
		new Uint8Array(encryptedDWallet.centralized_output),
	);
	if (!is_valid) {
		throw new Error("the decrypted key share doesn't match the dwallet's public key share");
	}
	return decrypted_share;
}
