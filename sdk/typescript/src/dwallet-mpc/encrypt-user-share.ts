// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
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
import { Ed25519Keypair, Ed25519PublicKey } from '../keypairs/ed25519/index.js';
import { Transaction } from '../transactions/index.js';
import { PERA_SYSTEM_STATE_OBJECT_ID } from '../utils/index.js';
import type { Config, DWallet, DWalletWithSecretKeyShare } from './globals.js';
import {
	checkpointCreationTime,
	delay,
	dWallet2PCMPCECDSAK1ModuleName,
	dWalletModuleName,
	dWalletMoveType,
	dWalletPackageID,
	fetchCompletedEvent,
	fetchObjectWithType,
	isDWallet,
	isEqual,
} from './globals.js';

const startEncryptedShareVerificationMoveType = `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::StartEncryptedShareVerificationEvent`;
const createdEncryptedSecretShareEventMoveType = `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::CreatedEncryptedSecretShareEvent`;
const startEncryptionKeyVerificationEventMoveType = `${dWalletPackageID}::${dWalletModuleName}::StartEncryptionKeyVerificationEvent`;
const encryptedUserSecretKeyShareMoveType = `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::EncryptedUserSecretKeyShare`;
const encryptionKeyMoveType = `${dWalletPackageID}::${dWalletModuleName}::EncryptionKey`;

/**
 * Event emitted by the blockchain when an
 * `EncryptionKey` Move object is created.
 */
interface CreatedEncryptionKeyEvent {
	session_id: string;
	encryption_key_id: string;
}

function isCreatedEncryptionKeyEvent(obj: any): obj is CreatedEncryptionKeyEvent {
	return obj && typeof obj.session_id === 'string' && typeof obj.encryption_key_id === 'string';
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
 * TS representation of an event to start an MPC session.
 * Usually the only thing needed from this event is the `session_id`,
 * which is used to fetch the
 * completion event.
 */
interface StartSessionEvent {
	session_id: string;
}

function isStartSessionEvent(obj: any): obj is StartSessionEvent {
	return 'session_id' in obj;
}

/**
 * A class groups Move encryption key object.
 */
interface EncryptionKey {
	encryption_key: Uint8Array;
	key_owner_address: string;
	encryption_key_signature: Uint8Array;
}

const isEncryptionKey = (obj: any): obj is EncryptionKey => {
	return 'encryption_key' in obj && 'key_owner_address' in obj && 'encryption_key_signature' in obj;
};

export enum EncryptionKeyScheme {
	ClassGroups = 0,
}

/**
 * A verified encrypted dWallet centralized secret key share.
 *
 * This represents an encrypted centralized secret key share tied to
 * a specific dWallet (`DWallet`).
 * It includes cryptographic proof that the encryption is valid and securely linked
 * to the associated `DWallet`.
 */
interface EncryptedUserSecretKeyShare {
	dwallet_id: string;
	encrypted_centralized_secret_share_and_proof: Uint8Array;
	encryption_key_id: string;
	centralized_public_output_signature: Uint8Array;
	encryptor_ed25519_pubkey: Uint8Array;
	encryptor_address: string;
}

function isEncryptedUserSecretKeyShare(obj: any): obj is EncryptedUserSecretKeyShare {
	return (
		obj &&
		typeof obj.dwallet_id === 'string' &&
		obj.encrypted_centralized_secret_share_and_proof instanceof Uint8Array &&
		typeof obj.encryption_key_id === 'string' &&
		obj.centralized_public_output_signature instanceof Uint8Array &&
		obj.encryptor_ed25519_pubkey instanceof Uint8Array &&
		typeof obj.encryptor_address === 'string'
	);
}

/**
 * TS representation of the Move event `CreatedEncryptedSecretShareEvent`.
 * Emitted when an encrypted share is created by the system transaction.
 */
interface CreatedEncryptedSecretShareEvent {
	// A unique identifier for the session related to this operation.
	session_id: string;

	// The ID of the `EncryptedUserSecretKeyShare` Move object.
	encrypted_share_obj_id: string;

	// The ID of the dWallet associated with this encrypted secret share.
	dwallet_id: string;

	// The encrypted centralized secret key share along with a cryptographic proof
	// that the encryption corresponds to the dWallet's secret key share.
	encrypted_centralized_secret_share_and_proof: Uint8Array;

	// The `EncryptionKey` Move object ID that was used to encrypt the secret key share.
	encryption_key_id: string;

	// The address of the entity that performed the encryption operation of this secret key share.
	encryptor_address: string;

	// The public key of the entity that performed the encryption operation
	// (with some encryption key — depends on the context)
	// and signed the `centralized_public_output`.
	// Used for verifications.
	encryptor_ed25519_pubkey: Uint8Array;

	// Signed dWallet public centralized output (signed by the `encryptor` entity).
	centralized_public_output_signature: Uint8Array;
}

function isCreatedEncryptedSecretShareEvent(obj: any): obj is CreatedEncryptedSecretShareEvent {
	return (
		obj &&
		typeof obj.session_id === 'string' &&
		typeof obj.encrypted_share_obj_id === 'string' &&
		typeof obj.dwallet_id === 'string' &&
		obj.encrypted_centralized_secret_share_and_proof instanceof Uint8Array &&
		typeof obj.encryption_key_id === 'string' &&
		typeof obj.encryptor_address === 'string' &&
		obj.encryptor_ed25519_pubkey instanceof Uint8Array &&
		obj.centralized_public_output_signature instanceof Uint8Array
	);
}

export class EncryptedUserShare {
	client: PeraClient;
	timeout: number;

	constructor(client: PeraClient, timeout: number) {
		this.client = client;
		this.timeout = timeout;
	}

	static fromConfig(config: Config): EncryptedUserShare {
		return new EncryptedUserShare(config.client, config.timeout);
	}

	public toConfig(keypair: Keypair): Config {
		return {
			keypair,
			client: this.client,
			timeout: this.timeout,
		};
	}

	async getOrCreateClassGroupsKeyPair(
		keyPair: Keypair,
		activeEncryptionKeysTableID: string,
	): Promise<ClassGroupsSecpKeyPair> {
		const [expectedEncryptionKey, decryptionKey] = generateCGKeyPairFromSuiKeyPair(keyPair);
		const activeEncryptionKeyObjID = await this.getActiveEncryptionKeyObjID(
			keyPair.getPublicKey(),
			activeEncryptionKeysTableID,
		);
		if (activeEncryptionKeyObjID) {
			const activeEncryptionKeyObj = await fetchObjectWithType<EncryptionKey>(
				this.toConfig(keyPair),
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

		const encryptionKeyCreationEvent: CreatedEncryptionKeyEvent = await this.registerEncryptionKey(
			keyPair,
			expectedEncryptionKey,
			EncryptionKeyScheme.ClassGroups,
		);
		await delay(checkpointCreationTime);

		await this.upsertActiveEncryptionKey(
			keyPair,
			encryptionKeyCreationEvent.encryption_key_id,
			activeEncryptionKeysTableID,
		);
		await delay(checkpointCreationTime);

		return {
			decryptionKey,
			encryptionKey: expectedEncryptionKey,
			objectID: encryptionKeyCreationEvent.encryption_key_id,
		};
	}

	/**
	 * Retrieves the active encryption key object ID
	 * from the active encryption keys table *activeEncryptionKeysTableID*
	 * for the given address — derived from the public key.
	 * Throws an error otherwise.
	 */
	async getActiveEncryptionKeyObjID(
		publicKey: PublicKey,
		activeEncryptionKeysTableID: string,
	): Promise<string> {
		const tx = new Transaction();
		const address = publicKey.toPeraAddress();
		tx.moveCall({
			target: `${dWalletPackageID}::${dWalletModuleName}::get_active_encryption_key`,
			arguments: [tx.object(activeEncryptionKeysTableID), tx.pure.address(address)],
		});

		// Safe to use this function as it has been used here:
		// https://github.com/dwallet-labs/dwallet-network/blob/29929ded135f05578b6ce33b52e6ff5e894d0487/sdk/deepbook-v3/src/client.ts#L84
		// in late 2024 (can be seen with git blame).
		// Note that regular `getObject()` is not working because of dynamic fields.
		const res = await this.client.devInspectTransactionBlock({
			sender: address,
			transactionBlock: tx,
		});

		const objIDArray = new Uint8Array(res.results?.at(0)?.returnValues?.at(0)?.at(0) as number[]);
		return toHEX(objIDArray);
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
	async registerEncryptionKey(
		keyPair: Keypair,
		encryptionKey: Uint8Array,
		encryptionKeyScheme: EncryptionKeyScheme,
	): Promise<CreatedEncryptionKeyEvent> {
		// Sign the encryption key with the key pair.
		const encryptionKeySignature = await keyPair.sign(new Uint8Array(encryptionKey));
		const tx = new Transaction();
		tx.moveCall({
			target: `${dWalletPackageID}::${dWalletModuleName}::register_encryption_key`,
			arguments: [
				tx.pure(bcs.vector(bcs.u8()).serialize(encryptionKey)),
				tx.pure(bcs.vector(bcs.u8()).serialize(encryptionKeySignature)),
				tx.pure(bcs.vector(bcs.u8()).serialize(keyPair.getPublicKey().toRawBytes())),
				tx.pure(bcs.u8().serialize(encryptionKeyScheme)),
				tx.sharedObjectRef({
					objectId: PERA_SYSTEM_STATE_OBJECT_ID,
					initialSharedVersion: 1,
					mutable: false,
				}),
			],
		});
		const res = await this.client.signAndExecuteTransaction({
			signer: keyPair,
			transaction: tx,
			options: {
				showEvents: true,
			},
		});

		const sessionID = (
			res.events?.find(
				(event) =>
					event.type === startEncryptionKeyVerificationEventMoveType &&
					isStartSessionEvent(event.parsedJson),
			)?.parsedJson as StartSessionEvent
		).session_id;

		return await fetchCompletedEvent<CreatedEncryptionKeyEvent>(
			this.toConfig(keyPair),
			sessionID,
			`${dWalletPackageID}::${dWalletModuleName}::CreatedEncryptionKeyEvent`,
			isCreatedEncryptionKeyEvent,
		);
	}

	/**
	 * Encrypts the given dWallet secret user key share for a given destination public key.
	 * This is needed to ensure that the destination entity can
	 * later verify and decrypt the encrypted dWallet secret key share.
	 *
	 * The function ensures that the destination public key has an active encryption key and
	 * verifies its authenticity by checking its signature against the public key.
	 * If the encryption key is valid,
	 * the function encrypts the dWallet secret key share and returns the
	 * encrypted key share along with a proof of encryption.
	 *
	 * @param sourceKeyPair - The key pair that currently owns the sourceDwallet that will
	 * be encrypted for the destination.
	 * @param destSuiPublicKey - The public key of the destination entity, used to encrypt the secret user key share.
	 * @param sourceDwallet - The dWallet containing the secret user key share to encrypt.
	 * @param activeEncryptionKeysTableID - The ID of the table holding the active encryption keys.
	 * @returns An object containing the encrypted user key share and proof of encryption,
	 *          along with the destination encryption key object ID.
	 * @throws Will throw an error if the destination public key does not have an active encryption key
	 *         or if the encryption key is not valid (not signed by the destination's public key).
	 */
	// todo(scaly): Maybe this func needs to receive an address instead of public key for dest?
	// todo(scaly): we don't know the public key, but we know the address.
	async encryptUserShareForPublicKey(
		sourceKeyPair: Keypair,
		destSuiPublicKey: PublicKey,
		sourceDwallet: DWalletWithSecretKeyShare,
		activeEncryptionKeysTableID: string,
	) {
		const destActiveEncryptionKeyObjID = await this.getActiveEncryptionKeyObjID(
			destSuiPublicKey,
			activeEncryptionKeysTableID,
		);
		if (!destActiveEncryptionKeyObjID) {
			throw new Error('the set key pair does not have an active encryption key');
		}
		const destActiveEncryptionKeyObj = await fetchObjectWithType<EncryptionKey>(
			this.toConfig(sourceKeyPair),
			encryptionKeyMoveType,
			isEncryptionKey,
			destActiveEncryptionKeyObjID,
		);

		// Make sure that the active signed encryption key is
		// valid by verifying that the destination public key has signed it.
		const isValidDestEncryptionKey = await destSuiPublicKey.verify(
			new Uint8Array(destActiveEncryptionKeyObj.encryption_key),
			new Uint8Array(destActiveEncryptionKeyObj.encryption_key_signature),
		);
		if (!isValidDestEncryptionKey) {
			throw new Error(
				'the destination encryption key has not been signed by the destination public key',
			);
		}

		// Encrypt the centralized secret key share with the destination active encryption key.
		const encryptedUserKeyShareAndProofOfEncryption = encrypt_secret_share(
			// Centralized Secret Key Share.
			new Uint8Array(sourceDwallet.centralizedSecretKeyShare),
			// Encryption Key.
			new Uint8Array(destActiveEncryptionKeyObj.encryption_key),
		);

		return {
			encryptedUserKeyShareAndProofOfEncryption,
			destActiveEncryptionKeyObjID,
		};
	}

	/**
	 * Updates or inserts the specified encryption key as the active encryption key
	 * for the given address (derived from keypair),
	 * the key is stored inside the active encryption keys table.
	 */
	async upsertActiveEncryptionKey(
		keyPair: Keypair,
		encryptionKeyObjID: string,
		activeEncryptionKeysTableID: string,
	) {
		const tx = new Transaction();
		tx.moveCall({
			target: `${dWalletPackageID}::${dWalletModuleName}::upsert_active_encryption_key`,
			arguments: [tx.object(activeEncryptionKeysTableID), tx.object(encryptionKeyObjID)],
		});

		return await this.client.signAndExecuteTransaction({
			signer: keyPair,
			transaction: tx,
			options: {
				showEffects: true,
			},
		});
	}

	/**
	 * todo(zeev): doc.
	 * Validates the provided `sourceEncryptedUserSecretShare` and re-encrypts it for the caller's keypair.
	 *
	 * This process ensures that users can later retrieve all secret shares ever encrypted for them,
	 * verify their validity, and confirm they are signed by the original source.
	 */
	async acceptUserShare(
		activeEncryptionKeysTableID: string,
		// todo(zeev): why do we pass the event?
		sourceEncryptedUserSecretShare: CreatedEncryptedSecretShareEvent,
		srcAddress: string,
		destKeyPair: Keypair,
	) {
		// Get the dWallet bound to the encrypted user share.
		const dwalletID = sourceEncryptedUserSecretShare.dwallet_id;
		const sourceDWallet = await fetchObjectWithType<DWallet>(
			this.toConfig(destKeyPair),
			dWalletMoveType,
			isDWallet,
			dwalletID,
		);

		const decryptedKeyShare = await this.decryptAndVerifyUserShare(
			activeEncryptionKeysTableID,
			sourceEncryptedUserSecretShare,
			sourceDWallet,
			srcAddress,
			destKeyPair,
		);

		// todo(zeev): rename.
		const dWalletToAccept: DWalletWithSecretKeyShare = {
			...sourceDWallet,
			centralizedSecretKeyShare: [...decryptedKeyShare],
		};

		// Encrypt it to self, so that in the future we'd know that we already
		// verified everything and only need to verify our signature.
		// Need to verify the signature to not trust the blockchain to provide this data.
		// todo(zeev): add more info.
		const { destActiveEncryptionKeyObjID, encryptedUserKeyShareAndProofOfEncryption } =
			await this.encryptUserShareForPublicKey(
				destKeyPair,
				destKeyPair.getPublicKey(),
				dWalletToAccept,
				activeEncryptionKeysTableID,
			);

		return this.transferEncryptedUserSecretShare(
			destKeyPair,
			encryptedUserKeyShareAndProofOfEncryption,
			destActiveEncryptionKeyObjID,
			dWalletToAccept,
		);
	}

	/**
	 * Decrypts and verifies a user's encrypted key share, ensuring that the source
	 * entity signed the dWallet public key share and that the decrypted key share
	 * matches the expected public key share.
	 */
	// todo(zeev): check this doc.
	// This function also verifies that the dkg output
	// has been signed by the source public key.
	async decryptAndVerifyUserShare(
		activeEncryptionKeysTableID: string,
		encryptedUserSecretKeyShare: EncryptedUserSecretKeyShare,
		dwallet: DWallet,
		srcIkaAddr: string,
		destIkaKeyPair: Keypair,
	): Promise<Uint8Array> {
		// The public key of the entity that sent
		// this encrypted key share (not the key that was used to encrypt it).
		const srcIkaPublicKey = new Ed25519PublicKey(
			encryptedUserSecretKeyShare.encryptor_ed25519_pubkey,
		);
		if (srcIkaPublicKey.toPeraAddress() !== srcIkaAddr) {
			throw new Error(
				'the source address does not match the address derived from the public key that was stored on the blockchain',
			);
		}
		// Make sure that the source entity signed the dWallet public key share.
		// We do it to make sure this is the key that was stored by us on the chain.
		if (
			!(await srcIkaPublicKey.verify(
				new Uint8Array(dwallet.centralized_public_output),
				// todo(zeev): rename to public_output
				new Uint8Array(encryptedUserSecretKeyShare.centralized_public_output_signature),
			))
		) {
			throw new Error('the desired address did not sign the dWallet public key share');
		}
		const destinationCGKeypair = await this.getOrCreateClassGroupsKeyPair(
			destIkaKeyPair,
			activeEncryptionKeysTableID,
		);
		const decryptedSecretShare = decrypt_user_share(
			destinationCGKeypair.encryptionKey,
			destinationCGKeypair.decryptionKey,
			encryptedUserSecretKeyShare.encrypted_centralized_secret_share_and_proof,
		);
		// Before validating this centralized output,
		// we are making sure it was signed by us.
		const isValid = verify_user_share(
			decryptedSecretShare,
			new Uint8Array(dwallet.centralized_public_output),
		);
		if (!isValid) {
			throw new Error('the decrypted key share does not match the dWallet public key share');
		}
		return decryptedSecretShare;
	}

	/**
	 * Transfers an encrypted dWallet user secret key share from a source entity to destination entity.
	 * This function emits an event with the encrypted user secret key share,
	 * along with its cryptographic proof, to the blockchain.
	 * The chain verifies that the encrypted data matches the expected secret key share
	 * associated with the dWallet before creating an `EncryptedUserSecretKeyShare` object.
	 */
	async transferEncryptedUserSecretShare(
		sourceKeyPair: Keypair,
		encryptedUserKeyShareAndProofOfEncryption: Uint8Array,
		destEncryptionKeyObjID: string,
		sourceDwallet: DWalletWithSecretKeyShare,
	): Promise<CreatedEncryptedSecretShareEvent> {
		const tx = new Transaction();
		// Sign the DKG Centralized Public output,
		// in order for the destination party to verify it later.
		const sourceSignedCentralizedPublicOutput = await sourceKeyPair.sign(
			new Uint8Array(sourceDwallet.centralized_public_output),
		);
		// todo(zeev): this should transfer the encrypted share to the destination.
		tx.moveCall({
			target: `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::transfer_encrypted_user_share`,
			typeArguments: [],
			arguments: [
				tx.object(sourceDwallet.id.id),
				tx.object(destEncryptionKeyObjID),
				tx.pure(bcs.vector(bcs.u8()).serialize(encryptedUserKeyShareAndProofOfEncryption)),
				tx.pure(bcs.vector(bcs.u8()).serialize(sourceSignedCentralizedPublicOutput)),
				tx.pure(bcs.vector(bcs.u8()).serialize(sourceKeyPair.getPublicKey().toRawBytes())),
				tx.sharedObjectRef({
					objectId: PERA_SYSTEM_STATE_OBJECT_ID,
					initialSharedVersion: 1,
					mutable: false,
				}),
			],
		});

		const result = await this.client.signAndExecuteTransaction({
			signer: sourceKeyPair,
			transaction: tx,
			options: {
				showEffects: true,
				showEvents: true,
			},
		});

		const sessionData = result.events?.find(
			(event) =>
				event.type === startEncryptedShareVerificationMoveType &&
				isStartSessionEvent(event.parsedJson),
		)?.parsedJson as StartSessionEvent;

		return await fetchCompletedEvent<CreatedEncryptedSecretShareEvent>(
			this.toConfig(sourceKeyPair),
			sessionData.session_id,
			createdEncryptedSecretShareEventMoveType,
			isCreatedEncryptedSecretShareEvent,
		);
	}
}

/**
 * Creates an object the holds table that maps chain addresses to Class Group encryption keys.
 * The Active encryption keys object is a Shared object.
 */
export async function createActiveEncryptionKeysTable(c: Config) {
	const tx = new Transaction();
	tx.moveCall({
		target: `${dWalletPackageID}::${dWalletModuleName}::create_active_encryption_keys`,
		arguments: [],
	});

	const result = await c.client.signAndExecuteTransaction({
		signer: c.keypair,
		transaction: tx,
		options: { showEffects: true },
	});

	const activeEncryptionKeysObj = result.effects?.created?.find(
		(o) =>
			typeof o.owner === 'object' &&
			'Shared' in o.owner &&
			o.owner.Shared.initial_shared_version !== undefined,
	)?.reference;
	if (!activeEncryptionKeysObj) {
		throw new Error('failed to create the active encryption keys object');
	}
	return activeEncryptionKeysObj;
}

/**
 * Derives a Secp256k1 Class Groups (CG) key pair using the secret key
 * from a provided Ed25519 key pair.
 *
 * This function extracts the secret key from the provided Ed25519 key pair,
 * decodes it using a specific decoding method (`decodePeraPrivateKey`), and
 * uses the decoded secret to derive a Secp256k1 Class Groups key pair.
 * The generated key pair is suitable for cryptographic operations requiring
 * Secp256k1 Class Groups keys.
 */
export function generateCGKeyPairFromSuiKeyPair(keyPair: Keypair): Uint8Array[] {
	if (!(keyPair instanceof Ed25519Keypair)) {
		throw new Error('key pair must be an instance of Ed25519Keypair');
	}
	const secretKey = keyPair.getSecretKey();
	const decodedKeyPair = decodePeraPrivateKey(secretKey);
	return generate_secp_cg_keypair_from_seed(decodedKeyPair.secretKey);
}

export async function fetchEncryptedUserSecretShare(
	conf: Config,
	dwalletID: string,
): Promise<EncryptedUserSecretKeyShare> {
	let cursor = null;
	const startTime = Date.now();

	while (Date.now() - startTime <= conf.timeout) {
		// Wait for 5 seconds between queries
		await delay(5000);

		const {
			data: ownedEncryptedShares,
			nextCursor,
			hasNextPage,
		} = await conf.client.getOwnedObjects({
			owner: conf.keypair.toPeraAddress(),
			options: { showContent: true, showType: true },
			filter: { StructType: encryptedUserSecretKeyShareMoveType },
			cursor,
		});

		for (const share of ownedEncryptedShares) {
			// Validate and parse the encrypted share
			const content = share?.data?.content;
			if (
				content?.dataType === 'moveObject' &&
				content.fields &&
				isEncryptedUserSecretKeyShare(content.fields) &&
				content.fields.dwallet_id === dwalletID
			) {
				return content.fields;
			}
		}

		cursor = hasNextPage ? nextCursor : null;

		// Break loop if there are no more pages
		if (!cursor) {
			break;
		}
	}

	const seconds = ((Date.now() - startTime) / 1000).toFixed(2);
	throw new Error(
		`Timeout: Unable to fetch encrypted share for dwallet ${dwalletID} within ${
			conf.timeout / (60 * 1000)
		} minutes (${seconds} seconds passed).`,
	);
}
