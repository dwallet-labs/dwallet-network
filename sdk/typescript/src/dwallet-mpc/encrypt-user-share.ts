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
import type { Config, CreatedDwallet, DWallet } from './globals.js';
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
	packageId,
} from './globals.js';

const startEncryptedShareVerificationMoveType = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::StartEncryptedShareVerificationEvent`;
const createdEncryptedSecretShareEventMoveType = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::CreatedEncryptedSecretShareEvent`;
const startEncryptionKeyVerificationEventMoveType = `${packageId}::${dWalletModuleName}::StartEncryptionKeyVerificationEvent`;
const encryptedUserSecretKeyShareMoveType = `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::EncryptedUserSecretKeyShare`;
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

// todo(zeev): fix the docs, explain what is each part.
interface EncryptedUserSecretKeyShare {
	dwallet_id: string;
	encrypted_secret_share_and_proof: Uint8Array;
	encryption_key_id: string;
	// todo(zeev): is it really a share or the whole output?
	// todo(zeev): should be renamed to centralized output.
	signed_public_share: Uint8Array;
	// todo(zeev): rename to source, encryptor is not clear.
	// todo(zeev): In here it's the one that called encryption func.
	encryptor_ed25519_pubkey: Uint8Array;
	encryptor_address: string;
}

function isEncryptedUserSecretKeyShare(obj: any): obj is EncryptedUserSecretKeyShare {
	return (
		obj &&
		'id' in obj &&
		'dwallet_id' in obj &&
		'encrypted_secret_share_and_proof' in obj &&
		'encryption_key_id' in obj &&
		'signed_public_share' in obj &&
		'encryptor_ed25519_pubkey' in obj &&
		'encryptor_address' in obj
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

		const encryptionKeyCreationEvent = await this.registerEncryptionKey(
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
			target: `${packageId}::${dWalletModuleName}::get_active_encryption_key`,
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
		const signedEncryptionKey = await keyPair.sign(new Uint8Array(encryptionKey));
		const tx = new Transaction();
		tx.moveCall({
			target: `${packageId}::${dWalletModuleName}::register_encryption_key`,
			arguments: [
				tx.pure(bcs.vector(bcs.u8()).serialize(encryptionKey)),
				tx.pure(bcs.vector(bcs.u8()).serialize(signedEncryptionKey)),
				tx.pure(bcs.vector(bcs.u8()).serialize(keyPair.getPublicKey().toRawBytes())),
				tx.pure(bcs.u8().serialize(encryptionKeyScheme)),
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
			`${packageId}::${dWalletModuleName}::CreatedEncryptionKeyEvent`,
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
		sourceDwallet: CreatedDwallet,
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
			new Uint8Array(sourceDwallet.centralizedDKGPrivateOutput),
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
			target: `${packageId}::${dWalletModuleName}::upsert_active_encryption_key`,
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
		const dwalletToSend: CreatedDwallet = {
			id: dwalletID,
			centralizedDKGPrivateOutput: [...decryptedKeyShare],
			decentralizedDKGOutput: sourceDWallet.decentralized_output,
			dwalletCapID: sourceDWallet.dwallet_cap_id,
			dwalletMPCNetworkKeyVersion: sourceDWallet.dwallet_mpc_network_key_version,
			centralizedDKGPublicOutput: sourceDWallet.centralized_output,
		};

		// Encrypt it to self, so that in the future we'd know that we already
		// verified everything and only need to verify our signature.
		// Need to verify the signature to not trust the blockchain to provide this data.
		// todo(zeev): add more info.
		const { destActiveEncryptionKeyObjID, encryptedUserKeyShareAndProofOfEncryption } =
			await this.encryptUserShareForPublicKey(
				destKeyPair,
				destKeyPair.getPublicKey(),
				dwalletToSend,
				activeEncryptionKeysTableID,
			);

		return this.transferEncryptedUserSecretShare(
			destKeyPair,
			encryptedUserKeyShareAndProofOfEncryption,
			destActiveEncryptionKeyObjID,
			dwalletToSend,
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
				new Uint8Array(dwallet.centralized_output),
				// todo(zeev): rename to centralized_output
				new Uint8Array(encryptedUserSecretKeyShare.signed_public_share),
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
			encryptedUserSecretKeyShare.encrypted_secret_share_and_proof,
		);
		// Before validating this centralized output,
		// we are making sure it was signed by us.
		const isValid = verify_user_share(
			decryptedSecretShare,
			new Uint8Array(dwallet.centralized_output),
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
		sourceDwallet: CreatedDwallet,
	): Promise<CreatedEncryptedSecretShareEvent> {
		const tx = new Transaction();
		// Sign the DKG Public output to self, in order for the destination party to verify it later.
		const sourceSignedCentralizedPublicOutput = await sourceKeyPair.sign(
			new Uint8Array(sourceDwallet.centralizedDKGPublicOutput),
		);
		// todo(zeev): this should transfer the encrypted share to the destination.
		tx.moveCall({
			target: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::transfer_encrypted_user_share`,
			typeArguments: [],
			arguments: [
				tx.object(sourceDwallet.id),
				tx.object(destEncryptionKeyObjID),
				tx.pure(bcs.vector(bcs.u8()).serialize(encryptedUserKeyShareAndProofOfEncryption)),
				tx.pure(bcs.vector(bcs.u8()).serialize(sourceSignedCentralizedPublicOutput)),
				tx.pure(bcs.vector(bcs.u8()).serialize(sourceKeyPair.getPublicKey().toRawBytes())),
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
		target: `${packageId}::${dWalletModuleName}::create_active_encryption_keys`,
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
