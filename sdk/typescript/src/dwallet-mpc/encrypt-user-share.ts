import {
	encrypt_secret_share,
	generate_secp_cg_keypair_from_seed,
} from '@dwallet-network/dwallet-mpc-wasm';
import { bcs, toHex } from '@mysten/bcs';
import type { PublicKey } from '@mysten/sui/cryptography';
import { Transaction } from '@mysten/sui/transactions';

import type { Config } from './globals.js';
import {
	delay,
	DWALLET_ECDSAK1_MOVE_MODULE_NAME,
	fetchObjectWithType,
	getDWalletSecpState,
	getEncryptionKeyMoveType,
	getObjectWithType,
	isMoveObject,
	SUI_PACKAGE_ID,
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
	if (activeEncryptionKeyObjID) {
		const activeEncryptionKeyObj = await fetchObjectWithType<EncryptionKey>(
			conf,
			getEncryptionKeyMoveType(conf.ikaConfig.ika_system_package_id),
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
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSAK1_MOVE_MODULE_NAME}::get_active_encryption_key`,
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
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSAK1_MOVE_MODULE_NAME}::register_encryption_key`,
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
export async function encryptUserShareForPublicKey(
	sourceConf: Config,
	destSuiPublicKey: PublicKey,
	dWalletSecretShare: Uint8Array,
) {
	const destActiveEncryptionKeyObjID = await getActiveEncryptionKeyObjID(
		sourceConf,
		destSuiPublicKey.toSuiAddress(),
	);
	if (!destActiveEncryptionKeyObjID) {
		throw new Error('the dest key pair does not have an active encryption key');
	}
	const destActiveEncryptionKeyObj = await getObjectWithType<EncryptionKey>(
		sourceConf,
		destActiveEncryptionKeyObjID,
		isEncryptionKey,
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
	return encrypt_secret_share(
		// Centralized Secret Key Share.
		dWalletSecretShare,
		// Encryption Key.
		new Uint8Array(destActiveEncryptionKeyObj.encryption_key),
	);
}

interface CreatedEncryptedSecretShareEvent {
	encrypted_user_secret_key_share_id: string;
	dwallet_id: string;
}

function isCreatedEncryptedSecretShareEvent(obj: any): obj is CreatedEncryptedSecretShareEvent {
	return 'encrypted_user_secret_key_share_id' in obj && 'dwallet_id' in obj;
}

//
// public fun request_re_encrypt_user_share_for(
// 	self: &mut DWallet2PcMpcSecp256K1,
// 	dwallet_id: ID,
// 	destination_encryption_key_address: address,
// 	encrypted_centralized_secret_share_and_proof: vector<u8>,
// 	source_encrypted_user_secret_key_share_id: ID,
// 	payment_ika: &mut Coin<IKA>,
// 	payment_sui: &mut Coin<SUI>,
// 	ctx: &mut TxContext,
// ) {

export async function transferEncryptedSecretShare(
	sourceConf: Config,
	destSuiPublicKey: PublicKey,
	encryptedUserKeyShareAndProofOfEncryption: Uint8Array,
	dwalletID: string,
	source_encrypted_user_secret_key_share_id: string,
) {
	const tx = new Transaction();
	const dwalletSecpState = await getDWalletSecpState(sourceConf);
	const dwalletStateArg = tx.sharedObjectRef({
		objectId: dwalletSecpState.object_id,
		initialSharedVersion: dwalletSecpState.initial_shared_version,
		mutable: true,
	});
	const dwalletIDArg = tx.pure.id(dwalletID);
	const destinationEncryptionKeyAddress = destSuiPublicKey.toSuiAddress();
	const destinationEncryptionKeyAddressArg = tx.pure.address(destinationEncryptionKeyAddress);
	const encryptedCentralizedSecretShareAndProofArg = tx.pure(
		bcs.vector(bcs.u8()).serialize(encryptedUserKeyShareAndProofOfEncryption),
	);
	const sourceEncryptedUserSecretKeyShareIDArg = tx.pure.id(
		source_encrypted_user_secret_key_share_id,
	);
	const emptyIKACoin = tx.moveCall({
		target: `${SUI_PACKAGE_ID}::coin::zero`,
		arguments: [],
		typeArguments: [`${sourceConf.ikaConfig.ika_package_id}::ika::IKA`],
	});

	tx.moveCall({
		target: `${sourceConf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSAK1_MOVE_MODULE_NAME}::request_re_encrypt_user_share_for`,
		arguments: [
			dwalletStateArg,
			dwalletIDArg,
			destinationEncryptionKeyAddressArg,
			encryptedCentralizedSecretShareAndProofArg,
			sourceEncryptedUserSecretKeyShareIDArg,
			emptyIKACoin,
			tx.gas,
		],
	});

	tx.moveCall({
		target: `${SUI_PACKAGE_ID}::coin::destroy_zero`,
		arguments: [emptyIKACoin],
		typeArguments: [`${sourceConf.ikaConfig.ika_package_id}::ika::IKA`],
	});

	const result = await sourceConf.client.signAndExecuteTransaction({
		signer: sourceConf.suiClientKeypair,
		transaction: tx,
		options: {
			showEvents: true,
		},
	});
	const startVerificationEvent = result.events?.at(0)?.parsedJson;
	if (!isStartEncryptedShareVerificationEvent(startVerificationEvent)) {
		throw new Error('invalid start DKG first round event');
	}
	await waitForChainVerification(
		sourceConf,
		startVerificationEvent.event_data.encrypted_user_secret_key_share_id,
	);
}

interface StartEncryptedShareVerificationEvent {
	event_data: {
		encrypted_user_secret_key_share_id: string;
	};
	session_id: string;
}

function isStartEncryptedShareVerificationEvent(
	obj: any,
): obj is StartEncryptedShareVerificationEvent {
	return !!obj?.session_id && !!obj?.event_data?.encrypted_user_secret_key_share_id;
}

function isEncryptedUserSecretKeyShare(obj: any): obj is EncryptedUserSecretKeyShare {
	return (
		'id' in obj &&
		'dwallet_id' in obj &&
		'encrypted_centralized_secret_share_and_proof' in obj &&
		'encryption_key_id' in obj &&
		'encryption_key_address' in obj
	);
}

interface EncryptedUserSecretKeyShare {
	id: { id: string };
	dwallet_id: string;
	encrypted_centralized_secret_share_and_proof: Uint8Array;
	encryption_key_id: string;
	encryption_key_address: string;
}

interface VerifiedEncryptedUserSecretKeyShare {
	state: any;
}

function isVerifiedEncryptedUserSecretKeyShare(
	obj: any,
): obj is VerifiedEncryptedUserSecretKeyShare {
	return obj.state.variant === 'NetworkVerificationCompleted';
}

async function waitForChainVerification(conf: Config, encryptedSecretShareObjID: string) {
	const startTime = Date.now();

	while (Date.now() - startTime <= conf.timeout) {
		// Wait for a bit before polling again, objects might not be available immediately.
		await delay(5_000);
		const dwallet = await conf.client.getObject({
			id: encryptedSecretShareObjID,
			options: {
				showContent: true,
			},
		});
		if (isMoveObject(dwallet?.data?.content)) {
			const dwalletMoveObject = dwallet?.data?.content?.fields;
			if (isVerifiedEncryptedUserSecretKeyShare(dwalletMoveObject)) {
				return;
			}
		}
	}
	const seconds = ((Date.now() - startTime) / 1000).toFixed(2);
	throw new Error(
		`timeout: unable to fetch the VerifiedEncryptedUserSecretKeyShare object within ${
			conf.timeout / (60 * 1000)
		} minutes (${seconds} seconds passed).`,
	);
}
